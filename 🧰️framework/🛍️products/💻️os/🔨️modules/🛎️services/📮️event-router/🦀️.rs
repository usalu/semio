//! 📮️ The indexed event router: per-subscriber mailboxes with latest-wins, lossless, coalesced and ring policies.

use semio_framework_actor::ActorId;
use semio_framework_async::ChannelPolicy;
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

//#region 📮️EventRouter
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Topic(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PublishOutcome {
    Delivered,
    Collapsed,
    RejectedFull { cap: u32 },
    RejectedInsufficientCredit,
    NoSuchSubscriber,
}

enum Mailbox {
    LatestWins { max_bytes: u64, pending: Option<Vec<u8>> },
    Coalesced { cap: u32, max_bytes: u64, used_bytes: u64, pending: HashMap<String, Vec<u8>>, order: VecDeque<String> },
    Ring { cap: u32, max_bytes: u64, used_bytes: u64, pending: VecDeque<Vec<u8>> },
    LosslessBounded { cap: u32, max_bytes: u64, used_bytes: u64, pending: VecDeque<Vec<u8>> },
    ByteCredit { remaining: u64 },
}

impl Mailbox {
    // 🚫️async: E1-adjacent — pure, in-memory state-machine construction, consumed (below) through
    // sync `Iterator::map`/`map_or` closures and a `std::sync::Mutex` guard held across the whole
    // call in `EventRouter::publish`/`drain`. See R9 and the same reasoning on `TokenBucket`/
    // `WheelCore` above.
    pub(super) fn new(policy: &ChannelPolicy) -> Mailbox {
        match policy {
            ChannelPolicy::LatestWins { max_bytes } => Mailbox::LatestWins { max_bytes: *max_bytes, pending: None },
            ChannelPolicy::Coalesced { key: _, max_items, max_bytes } => Mailbox::Coalesced { cap: *max_items, max_bytes: *max_bytes, used_bytes: 0, pending: HashMap::new(), order: VecDeque::new() },
            ChannelPolicy::Ring { max_items, max_bytes } => Mailbox::Ring { cap: *max_items, max_bytes: *max_bytes, used_bytes: 0, pending: VecDeque::new() },
            ChannelPolicy::LosslessBounded { max_items, max_bytes } => Mailbox::LosslessBounded { cap: *max_items, max_bytes: *max_bytes, used_bytes: 0, pending: VecDeque::new() },
            ChannelPolicy::ByteCredit { max_items: _, max_bytes } => Mailbox::ByteCredit { remaining: *max_bytes },
        }
    }

    /// 📮️ `mailbox_key` is only meaningful for a [`ChannelPolicy::Coalesced`] mailbox — an
    /// incoming message under a key already pending REPLACES it (collapse); a new key queues
    /// alongside the others.
    // 🚫️async: E1-adjacent — see `Mailbox::new`'s tag above (R9).
    pub(super) fn publish(&mut self, mailbox_key: Option<&str>, payload: Vec<u8>) -> PublishOutcome {
        match self {
            Mailbox::LatestWins { max_bytes, pending } => {
                if payload.len() as u64 > *max_bytes {
                    return PublishOutcome::RejectedInsufficientCredit;
                }
                let collapsed = pending.is_some();
                *pending = Some(payload);
                if collapsed {
                    PublishOutcome::Collapsed
                } else {
                    PublishOutcome::Delivered
                }
            }
            Mailbox::Coalesced { cap, max_bytes, used_bytes, pending, order } => {
                let key = mailbox_key.unwrap_or_default().to_string();
                let cost = payload.len() as u64;
                if *cap == 0 {
                    return PublishOutcome::RejectedFull { cap: *cap };
                }
                if cost > *max_bytes {
                    return PublishOutcome::RejectedInsufficientCredit;
                }
                let mut collapsed = false;
                if let Some(previous) = pending.remove(&key) {
                    *used_bytes = used_bytes.saturating_sub(previous.len() as u64);
                    order.retain(|queued| queued != &key);
                    collapsed = true;
                }
                while pending.len() as u32 >= *cap || used_bytes.saturating_add(cost) > *max_bytes {
                    let Some(oldest) = order.pop_front() else { break };
                    if let Some(previous) = pending.remove(&oldest) {
                        *used_bytes = used_bytes.saturating_sub(previous.len() as u64);
                        collapsed = true;
                    }
                }
                pending.insert(key.clone(), payload);
                order.push_back(key);
                *used_bytes = used_bytes.saturating_add(cost);
                if collapsed {
                    PublishOutcome::Collapsed
                } else {
                    PublishOutcome::Delivered
                }
            }
            Mailbox::Ring { cap, max_bytes, used_bytes, pending } => {
                let cost = payload.len() as u64;
                if *cap == 0 {
                    return PublishOutcome::RejectedFull { cap: *cap };
                }
                if cost > *max_bytes {
                    return PublishOutcome::RejectedInsufficientCredit;
                }
                let mut collapsed = false;
                while pending.len() as u32 >= *cap || used_bytes.saturating_add(cost) > *max_bytes {
                    let Some(previous) = pending.pop_front() else { break };
                    *used_bytes = used_bytes.saturating_sub(previous.len() as u64);
                    collapsed = true;
                }
                pending.push_back(payload);
                *used_bytes = used_bytes.saturating_add(cost);
                if collapsed {
                    PublishOutcome::Collapsed
                } else {
                    PublishOutcome::Delivered
                }
            }
            Mailbox::LosslessBounded { cap, max_bytes, used_bytes, pending } => {
                if pending.len() as u32 >= *cap {
                    PublishOutcome::RejectedFull { cap: *cap }
                } else if used_bytes.saturating_add(payload.len() as u64) > *max_bytes {
                    PublishOutcome::RejectedInsufficientCredit
                } else {
                    *used_bytes = used_bytes.saturating_add(payload.len() as u64);
                    pending.push_back(payload);
                    PublishOutcome::Delivered
                }
            }
            Mailbox::ByteCredit { remaining } => {
                let cost = payload.len() as u64;
                if cost > *remaining {
                    PublishOutcome::RejectedInsufficientCredit
                } else {
                    *remaining -= cost;
                    PublishOutcome::Delivered
                }
            }
        }
    }

    /// 💳️ `ByteCredit` tracks a spendable budget only, no queued payload, in this packet — a real
    /// byte-metered channel's actual delivery mechanism is later-packet wiring, so it drains empty.
    // 🚫️async: E1-adjacent — see `Mailbox::new`'s tag above (R9).
    pub(super) fn drain(&mut self) -> Vec<Vec<u8>> {
        match self {
            Mailbox::LatestWins { pending, .. } => pending.take().into_iter().collect(),
            Mailbox::Coalesced { used_bytes, pending, order, .. } => {
                let mut out = Vec::new();
                while let Some(key) = order.pop_front() {
                    if let Some(value) = pending.remove(&key) {
                        out.push(value);
                    }
                }
                *used_bytes = 0;
                out
            }
            Mailbox::Ring { used_bytes, pending, .. } | Mailbox::LosslessBounded { used_bytes, pending, .. } => {
                *used_bytes = 0;
                pending.drain(..).collect()
            }
            Mailbox::ByteCredit { .. } => Vec::new(),
        }
    }
}

#[derive(Clone)]
struct Subscriber {
    pub(super) actor: ActorId,
    policy: ChannelPolicy,
}

/// 📮️ Indexed topic router: [`EventRouter::subscribe`]/`unsubscribe` register a
/// `(ActorId, ChannelPolicy)` per topic; [`EventRouter::publish`]/`send_message` deliver into
/// a per-`(topic, actor)` [`Mailbox`] built from the subscriber's OWN declared policy —
/// `LatestWins`/`Coalesced`/`Ring` collapse, `LosslessBounded` rejects rather than growing past `cap`,
/// exactly the backpressure vocabulary `semio_framework_async::ChannelPolicy` declares. Draining a
/// mailbox into a real [`CompletionSink::complete`] call with a real actor generation is a later
/// packet's job — see the crate report's `## honest gaps`; this region's own contract (the
/// routing/backpressure DECISION) is tested directly via [`EventRouter::drain`].
pub struct EventRouter {
    subscribers: Mutex<HashMap<Topic, Vec<Subscriber>>>,
    mailboxes: Mutex<HashMap<(Topic, ActorId), Mailbox>>,
}

impl Default for EventRouter {
    fn default() -> EventRouter {
        EventRouter::new()
    }
}

impl EventRouter {
    // 🚫️async: E1 pure constructor consumed by `Default::default` (external trait, cannot be async)
    // — see R9. No suspension point exists (`Mutex::new`/`HashMap::new` are in-memory only).
    pub fn new() -> EventRouter {
        EventRouter { subscribers: Mutex::new(HashMap::new()), mailboxes: Mutex::new(HashMap::new()) }
    }

    pub fn subscribe(&self, topic: Topic, actor: ActorId, policy: ChannelPolicy) {
        let mailbox = Mailbox::new(&policy);
        self.mailboxes.lock().expect("EventRouter mailboxes mutex poisoned").insert((topic.clone(), actor), mailbox);
        self.subscribers.lock().expect("EventRouter subscribers mutex poisoned").entry(topic).or_default().push(Subscriber { actor, policy });
    }

    /// 🔍️ The `ChannelPolicy` `actor` declared for `topic` at [`EventRouter::subscribe`] time, if
    /// still subscribed — lets a caller (e.g. a diagnostics surface) inspect backpressure
    /// vocabulary without reaching into a `Mailbox`, which is private.
    pub async fn declared_policy(&self, topic: &Topic, actor: ActorId) -> Option<ChannelPolicy> {
        self.subscribers.lock().expect("EventRouter subscribers mutex poisoned").get(topic)?.iter().find(|subscriber| subscriber.actor == actor).map(|subscriber| subscriber.policy.clone())
    }

    pub async fn unsubscribe(&self, topic: &Topic, actor: ActorId) {
        if let Some(subscribers) = self.subscribers.lock().expect("EventRouter subscribers mutex poisoned").get_mut(topic) {
            subscribers.retain(|subscriber| subscriber.actor != actor);
        }
        self.mailboxes.lock().expect("EventRouter mailboxes mutex poisoned").remove(&(topic.clone(), actor));
    }

    /// 📮️ Delivers `payload` to every subscriber of `topic`, honouring each one's OWN policy
    /// independently — one bounded subscriber rejecting never affects another's delivery.
    pub async fn publish(&self, topic: &Topic, mailbox_key: Option<&str>, payload: &[u8]) -> Vec<(ActorId, PublishOutcome)> {
        let subscribers = self.subscribers.lock().expect("EventRouter subscribers mutex poisoned").get(topic).cloned().unwrap_or_default();
        let mut mailboxes = self.mailboxes.lock().expect("EventRouter mailboxes mutex poisoned");
        subscribers
            .into_iter()
            .map(|subscriber| {
                let outcome = mailboxes.get_mut(&(topic.clone(), subscriber.actor)).map_or(PublishOutcome::NoSuchSubscriber, |mailbox| mailbox.publish(mailbox_key, payload.to_vec()));
                (subscriber.actor, outcome)
            })
            .collect()
    }

    /// ✉️ Direct actor-to-actor send, bypassing topic subscription — `actor` must already have a
    /// mailbox for `topic` from a prior [`EventRouter::subscribe`] call.
    pub async fn send_message(&self, topic: &Topic, actor: ActorId, payload: Vec<u8>) -> PublishOutcome {
        match self.mailboxes.lock().expect("EventRouter mailboxes mutex poisoned").get_mut(&(topic.clone(), actor)) {
            Some(mailbox) => mailbox.publish(None, payload),
            None => PublishOutcome::NoSuchSubscriber,
        }
    }

    pub async fn drain(&self, topic: &Topic, actor: ActorId) -> Vec<Vec<u8>> {
        self.mailboxes.lock().expect("EventRouter mailboxes mutex poisoned").get_mut(&(topic.clone(), actor)).map(|mailbox| mailbox.drain()).unwrap_or_default()
    }
}
//#endregion 📮️EventRouter
