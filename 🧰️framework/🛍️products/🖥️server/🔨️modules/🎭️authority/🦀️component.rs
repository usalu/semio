//! 🎭️ Authority: the actor turn protocol — the middle of three layers that never merge.
//!
//! The **CQRS dual bus** ([`CommandEnvelope`]/`QueryEnvelope`) carries application intent; the
//! **actor turn protocol** in this file serializes that intent through exactly one consistency
//! boundary; the **replication protocol** (`protocol` crate) moves the resulting causal state
//! between replicas. Collapsing any two of them produces a system that cannot be reasoned about:
//! a bus would start sequencing storage, a turn would start speaking a transport, replication
//! would start deciding. This file owns the middle layer and nothing else — admission,
//! deduplication, placement, revision fencing, decision, commit, evolution, receipt.
//!
//! Two laws govern everything here. **Exactly-once**: a resubmitted [`IdempotencyKey`] returns the
//! byte-identical [`CommandReceipt`] and appends no second event. **Never trust the client**: the
//! optimistic replica runs the very same [`Decider`] and may produce events, but the authority
//! re-derives and re-stamps every [`EventRecord`] it commits.
//!
//! No transport, no runtime, no clock: callers pass the [`HybridLogicalClock`] reading in.

use std::collections::HashMap;

use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
use thiserror::Error;

use crate::contract::{
    ActorKey, CommandEnvelope, CommandOutcome, CommandReceipt, EventRecord, HybridLogicalClock,
    IdempotencyKey, Notice, PolicyDecision, Principal, ProcessId, Rejection, Revision, Scope,
};
use crate::storage::{AuthorityStore, Lease, OutboxEntry};

//#region 🔖️Error
/// @emoji 💥️ What can go wrong inside a turn that is not a domain [`Rejection`]. A rejection is an
/// answer the caller asked for; an [`AuthorityError`] is the authority failing to answer at all.
#[derive(Debug, Error)]
pub enum AuthorityError {
    /// 🔓️ The activation's fencing lease is no longer held — another epoch owns this actor.
    #[error("lease lost for the requested actor")]
    LeaseLost,
    /// 👻️ No [`Decider`] is registered for this actor kind on this instance.
    #[error("unknown actor kind: {0}")]
    UnknownActorKind(String),
    /// 🗄️ The durable store refused or failed the turn.
    #[error("storage failure: {0}")]
    Storage(String),
}
//#endregion 🔖️Error

//#region 🔖️Turn
/// @emoji 🧠️ One actor's private state as the turn protocol sees it: a fenced [`Revision`] plus an
/// opaque domain payload. The protocol owns `revision` and never parses `bytes`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorState {
    pub revision: Revision,
    pub bytes: Vec<u8>,
}

impl Default for ActorState {
    /// 🌱️ A never-yet-activated actor: revision zero, empty domain state.
    fn default() -> Self {
        Self { revision: Revision(0), bytes: Vec::new() }
    }
}

/// @emoji 🧭️ Everything a decision may read that is not the actor's own state. Deliberately tiny:
/// anything absent here is ambient input a pure decision is forbidden to reach for.
#[derive(Clone, Debug, PartialEq)]
pub struct DecisionContext {
    pub now: HybridLogicalClock,
    pub principal: Principal,
    pub scope: Scope,
}

/// @emoji 📣️ A side effect a decision requests: mail, webhook, blob transcode, push. Committed to
/// the outbox in the same write as the events, dispatched later, never inside the turn.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Effect {
    pub kind: String,
    pub payload: Vec<u8>,
}

/// @emoji 🎯️ The whole result of one decision. There is no fourth branch on purpose: a turn either
/// produces facts, refuses, or hands the work to a long-running process.
#[derive(Clone, Debug, PartialEq)]
pub enum Decision {
    Emit { events: Vec<EventRecord>, effects: Vec<Effect> },
    Reject(Rejection),
    Defer(ProcessId),
}

/// @emoji ⚖️ The deterministic core that the optimistic client replica and the authority both run,
/// unchanged, against the same [`ActorState`] and [`CommandEnvelope`].
///
/// `decide` **MUST be pure**: same state, command and context yield the same [`Decision`], always.
/// It may not read a clock, a random source, the filesystem, the network or any global — the only
/// admissible non-state input is [`DecisionContext`]. Impurity here silently breaks convergence,
/// because the client's speculative apply and the authority's canonical turn would diverge with no
/// error anywhere to observe.
///
/// `evolve` folds a committed [`EventRecord`] into `state.bytes` and **must not touch**
/// `state.revision`: the turn protocol owns the revision so that optimistic concurrency stays a
/// property of the protocol rather than of every domain implementation.
///
/// The authority never trusts a client-produced [`EventRecord`]. A replica may run `decide` and
/// apply the result locally, but the authority re-runs `decide` itself and re-stamps `stream`,
/// `seq` and `hlc` on every event before committing; only `kind` and `payload` survive.
#[dyn_enum]
pub trait Decider: Send + Sync {
    /// 🏷️ The actor kind this decider serves, matched against [`ActorKey::kind`].
    async fn actor_kind(&self) -> &str;
    /// 🎲️ Decide one command against one state. Pure — see the trait documentation.
    async fn decide(&self, state: &ActorState, command: &CommandEnvelope, context: &DecisionContext) -> Decision;
    /// 🌀️ Fold one committed event into the domain state. Never mutates the revision.
    async fn evolve(&self, state: &mut ActorState, event: &EventRecord);
}

/// 🔢️ The actor kind [`CounterDecider`] serves.
pub const COUNTER: &str = "counter";

/// 🧮️ Fold [`CounterDecider`]'s little-endian counter bytes back to a `u64`, defaulting to zero for
/// an unstarted or malformed state.
fn read_counter(bytes: &[u8]) -> u64 {
    <[u8; 8]>::try_from(bytes).map(u64::from_le_bytes).unwrap_or(0)
}

/// 🧮️ The framework's reference [`Decider`]: a little-endian counter over one command kind. Kept in
/// production scope (not only in tests) so [`Deciders`] closes over a real variant and every
/// `CommandBus` example in this crate's own tests exercises the genuine enum-dispatch path rather
/// than a mock. A product built on this framework adds its own deciders as further `Deciders`
/// variants alongside this one (O1 — closed-set dyn removal, see `dyn_enum_close!` below).
pub struct CounterDecider;

impl Decider for CounterDecider {
    async fn actor_kind(&self) -> &str {
        COUNTER
    }

    async fn decide(&self, state: &ActorState, command: &CommandEnvelope, _context: &DecisionContext) -> Decision {
        match command.kind.as_str() {
            "counter.increment" => Decision::Emit {
                events: vec![EventRecord {
                    stream: command.target.clone(),
                    seq: 0,
                    hlc: HybridLogicalClock::default(),
                    kind: "counter.incremented".into(),
                    payload: command.payload.clone(),
                }],
                effects: vec![],
            },
            "counter.audit" => Decision::Emit {
                events: vec![],
                effects: vec![Effect { kind: "counter.audited".into(), payload: state.bytes.clone() }],
            },
            "counter.forbid" => Decision::Reject(Rejection::Invalid { detail: "counter refuses".into() }),
            "counter.rebuild" => Decision::Defer(ProcessId("rebuild-1".into())),
            other => Decision::Reject(Rejection::UnknownCommandKind { command_kind: other.into() }),
        }
    }

    async fn evolve(&self, state: &mut ActorState, event: &EventRecord) {
        let step = u64::from(event.payload.first().copied().unwrap_or(0));
        let current = read_counter(&state.bytes);
        state.bytes = (current + step).to_le_bytes().to_vec();
    }
}

dyn_enum_close! {
    pub enum Deciders: Decider {
        Counter(CounterDecider),
    }
}
//#endregion 🔖️Turn

//#region 🔖️Directory
/// @emoji 📇️ One actor kind bound to the implementation that serves it.
pub struct ActorRegistration {
    pub actor_kind: String,
    pub decider: Deciders,
}

impl ActorRegistration {
    /// 🔗️ Bind a decider under the actor kind it declares.
    pub async fn new(decider: Deciders) -> Self {
        Self { actor_kind: decider.actor_kind().await.to_string(), decider }
    }
}

/// @emoji 🔥️ One live actor: its fencing lease, its state, how far its mailbox has been consumed
/// and which snapshot version that state was last persisted at.
#[derive(Clone, Debug)]
pub struct Activation {
    pub lease: Lease,
    pub state: ActorState,
    pub mailbox_seq: u64,
    pub snapshot_version: u64,
}

/// @emoji 🗺️ Where actors live and under whose lease.
///
/// The implementation is a single-process [`HashMap`], but the **contract is not**: placement,
/// activation epochs, fencing leases, mailbox sequence numbers, receipt-based deduplication and
/// passivation are all exposed here precisely because a fenced distributed placement service must
/// be droppable in behind this same surface without any caller changing. A caller that reaches
/// past [`activate`](Self::activate) into the map would be writing single-process assumptions into
/// the turn protocol, so the map stays private and every access is fenced by an epoch.
///
/// The lease minted here is the in-process stand-in for `AuthorityStore::acquire_lease`; the
/// distributed implementation swaps the source of the epoch, not the shape of the call.
#[derive(Default)]
pub struct AuthorityDirectory {
    activations: HashMap<ActorKey, Activation>,
    next_epoch: u64,
}

impl AuthorityDirectory {
    /// 🆕️ An empty directory with no actor placed.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🔥️ Place `key` under `holder`, minting a fenced activation epoch on first placement and
    /// returning the already-live activation otherwise.
    pub fn activate(&mut self, key: ActorKey, holder: &str) -> Result<&mut Activation, AuthorityError> {
        if !self.activations.contains_key(&key) {
            let epoch = self.next_epoch;
            self.next_epoch += 1;
            let lease = Lease { epoch, holder: holder.to_string() };
            let activation = Activation {
                lease,
                state: ActorState::default(),
                mailbox_seq: 0,
                snapshot_version: 0,
            };
            self.activations.insert(key.clone(), activation);
        }
        self.activations.get_mut(&key).ok_or(AuthorityError::LeaseLost)
    }

    /// 💤️ Drop the activation, releasing its lease. The next [`activate`](Self::activate) mints a
    /// strictly higher epoch, so any in-flight holder of the old epoch is fenced out.
    pub fn passivate(&mut self, key: &ActorKey) {
        self.activations.remove(key);
    }

    /// 👀️ Whether this actor is currently placed here.
    pub fn is_active(&self, key: &ActorKey) -> bool {
        self.activations.contains_key(key)
    }

    /// 🔢️ The fencing epoch of the current activation, if any.
    pub fn activation_epoch(&self, key: &ActorKey) -> Option<u64> {
        self.activations.get(key).map(|activation| activation.lease.epoch)
    }

    /// 🧾️ Read-only view of one activation.
    pub fn activation(&self, key: &ActorKey) -> Option<&Activation> {
        self.activations.get(key)
    }
}
//#endregion 🔖️Directory

//#region 🔖️Bus
/// @emoji 🚦️ The policy admission callback. A boxed closure rather than a concrete engine so the
/// turn protocol depends on the *decision*, never on how the decision was reached.
pub type PolicyHook = Box<dyn Fn(&CommandEnvelope) -> PolicyDecision + Send + Sync>;

/// @emoji 🏛️ The command side of the dual bus: it runs exactly one turn per submitted command,
/// against exactly one actor, in a fixed and non-negotiable order.
pub struct CommandBus<S: AuthorityStore> {
    directory: AuthorityDirectory,
    store: S,
    policy_hook: PolicyHook,
    registrations: HashMap<String, ActorRegistration>,
    holder: String,
}

impl<S: AuthorityStore> CommandBus<S> {
    /// 🆕️ Build a bus over a placement directory, a durable store and a policy admission hook.
    pub fn new(directory: AuthorityDirectory, store: S, policy_hook: PolicyHook) -> Self {
        Self { directory, store, policy_hook, registrations: HashMap::new(), holder: HOLDER.to_string() }
    }

    /// 📇️ Register a decider under the actor kind it declares, replacing any previous one.
    pub async fn register(&mut self, decider: Deciders) {
        let registration = ActorRegistration::new(decider).await;
        self.registrations.insert(registration.actor_kind.clone(), registration);
    }

    /// 🗺️ The placement directory this bus runs turns against.
    pub fn directory(&self) -> &AuthorityDirectory {
        &self.directory
    }

    /// 🗄️ The durable store this bus commits into.
    pub fn store(&self) -> &S {
        &self.store
    }

    /// 🗄️ Mutable access to the store, for the outbox drain and projection catch-up.
    pub fn store_mut(&mut self) -> &mut S {
        &mut self.store
    }

    /// @emoji 📨️ Run one turn.
    ///
    /// The order below is the protocol and may not be rearranged:
    ///
    /// 1. **Admit** — a command is servable only if a [`Decider`] is registered for its target
    ///    actor kind; otherwise [`Rejection::UnknownCommandKind`]. Cheapest check first, so an
    ///    unroutable command costs no storage read.
    /// 2. **Deduplicate** — if the [`IdempotencyKey`] already carries a [`CommandReceipt`], return
    ///    that same receipt as [`CommandOutcome::Accepted`] with no events. This runs *before*
    ///    policy and *before* placement: a retry must be answered identically even if the caller's
    ///    grants or the actor's placement changed since the original turn. This is the
    ///    exactly-once law, and it is the only reason a client may retry a timed-out submission.
    /// 3. **Authorize** — [`PolicyPoint::CommandAdmission`](crate::contract::PolicyPoint); a deny
    ///    becomes [`Rejection::Unauthorized`] and the actor is never even placed.
    /// 4. **Place** — acquire the activation and its fencing lease.
    /// 5. **Fence the revision** — a stated `expected_revision` that disagrees with the activation
    ///    yields [`Rejection::RevisionConflict`] carrying the actual revision, so the client can
    ///    rebase rather than guess.
    /// 6. **Decide** — the pure core runs. Nothing before this point consulted the domain.
    /// 7. **Commit** — events are re-stamped by the authority and written together with their
    ///    outbox entries in one atomic store call, so an effect can never escape without its
    ///    causing event, nor an event be visible without its pending effect.
    /// 8. **Evolve** — the committed events are folded into the in-memory state and the revision
    ///    is advanced to the last committed sequence number.
    /// 9. **Record the receipt** — only after the commit, so a crash between the two replays the
    ///    turn rather than acknowledging work that never landed.
    /// 10. **Answer** — [`Decision::Reject`] became [`CommandOutcome::Rejected`],
    ///     [`Decision::Defer`] became [`CommandOutcome::Pending`], everything else is accepted.
    pub async fn submit(&mut self, envelope: CommandEnvelope, now: HybridLogicalClock) -> CommandOutcome {
        //#region 🔖️Admit
        let Some(registration) = self.registrations.get(&envelope.target.kind) else {
            let reason = Rejection::UnknownCommandKind { command_kind: envelope.kind.clone() };
            return refuse(&envelope, Revision(0), now, reason);
        };
        //#endregion 🔖️Admit

        //#region 🔖️Deduplicate
        if let Some(key) = &envelope.idempotency_key {
            match self.store.receipt(key).await {
                Ok(Some(receipt)) => return CommandOutcome::Accepted { receipt, events: Vec::new(), frontier: None },
                Ok(None) => {}
                Err(error) => return unavailable(&envelope, Revision(0), now, &error.to_string()),
            }
        }
        //#endregion 🔖️Deduplicate

        //#region 🔖️Authorize
        if let PolicyDecision::Deny { reason } = (self.policy_hook)(&envelope) {
            return refuse(&envelope, Revision(0), now, Rejection::Unauthorized { detail: reason });
        }
        //#endregion 🔖️Authorize

        //#region 🔖️Place
        let activation = match self.directory.activate(envelope.target.clone(), &self.holder) {
            Ok(activation) => activation,
            Err(error) => return unavailable(&envelope, Revision(0), now, &error.to_string()),
        };
        //#endregion 🔖️Place

        //#region 🔖️Fence
        if let Some(expected) = envelope.expected_revision {
            let actual = activation.state.revision;
            if expected != actual {
                return refuse(&envelope, actual, now, Rejection::RevisionConflict { expected, actual });
            }
        }
        //#endregion 🔖️Fence

        //#region 🔖️Decide
        let context = DecisionContext {
            now,
            principal: envelope.principal.clone(),
            scope: envelope.scope.clone(),
        };
        let (events, effects) = match registration.decider.decide(&activation.state, &envelope, &context).await {
            Decision::Emit { events, effects } => (events, effects),
            Decision::Reject(reason) => return refuse(&envelope, activation.state.revision, now, reason),
            Decision::Defer(process) => {
                let receipt = acknowledge(&envelope, activation.state.revision, now);
                return CommandOutcome::Pending { receipt, process };
            }
        };
        //#endregion 🔖️Decide

        //#region 🔖️Commit
        let committed = seal(&envelope.target, activation.mailbox_seq, now, events);
        let outbox = dispatchable(&envelope.target, &committed, effects);
        if let Err(error) = self.store.append_events(&envelope.target, &committed, &outbox).await {
            return unavailable(&envelope, activation.state.revision, now, &error.to_string());
        }
        //#endregion 🔖️Commit

        //#region 🔖️Evolve
        for event in &committed {
            registration.decider.evolve(&mut activation.state, event).await;
        }
        if let Some(last) = committed.last() {
            activation.mailbox_seq = last.seq;
            activation.state.revision = Revision(last.seq);
        }
        let revision = activation.state.revision;
        //#endregion 🔖️Evolve

        //#region 🔖️Receipt
        let receipt = CommandReceipt {
            command_id: envelope.command_id.clone(),
            actor: envelope.target.clone(),
            revision,
            accepted_at: now,
        };
        if let Some(key) = &envelope.idempotency_key {
            if let Err(error) = self.store.record_receipt(key, &receipt).await {
                return unavailable(&envelope, revision, now, &error.to_string());
            }
        }
        //#endregion 🔖️Receipt

        //#region 🔖️Answer
        CommandOutcome::Accepted { receipt, events: committed, frontier: None }
        //#endregion 🔖️Answer
    }
}

/// 🏷️ The holder name a single-process authority leases actors under.
const HOLDER: &str = "authority";

/// 🧾️ The receipt describing where this command left the actor.
fn acknowledge(envelope: &CommandEnvelope, revision: Revision, now: HybridLogicalClock) -> CommandReceipt {
    CommandReceipt {
        command_id: envelope.command_id.clone(),
        actor: envelope.target.clone(),
        revision,
        accepted_at: now,
    }
}

/// 🚫️ A refusal carrying the receipt the caller still needs to correlate the answer.
fn refuse(envelope: &CommandEnvelope, revision: Revision, now: HybridLogicalClock, reason: Rejection) -> CommandOutcome {
    CommandOutcome::Rejected { receipt: acknowledge(envelope, revision, now), reason, notices: Vec::new() }
}

/// 🗄️ A storage or placement failure surfaced as a retryable refusal rather than a panic.
fn unavailable(envelope: &CommandEnvelope, revision: Revision, now: HybridLogicalClock, detail: &str) -> CommandOutcome {
    let notices = vec![Notice { code: "authority.retryable".into(), message: detail.to_string() }];
    let reason = Rejection::ActorUnavailable { detail: detail.to_string() };
    CommandOutcome::Rejected { receipt: acknowledge(envelope, revision, now), reason, notices }
}

/// 🔏️ Re-stamp decided events with authority-assigned stream, sequence and clock, keeping only the
/// domain-owned `kind` and `payload`, so nothing a client proposed can reach the log unverified.
fn seal(actor: &ActorKey, from_seq: u64, now: HybridLogicalClock, events: Vec<EventRecord>) -> Vec<EventRecord> {
    events
        .into_iter()
        .enumerate()
        .map(|(offset, event)| EventRecord {
            stream: actor.clone(),
            seq: from_seq + offset as u64 + 1,
            hlc: now,
            kind: event.kind,
            payload: event.payload,
        })
        .collect()
}

/// 📤️ The outbox rows for one turn: one per committed event for saga fan-out, one per requested
/// effect for the dispatcher. Ids are assigned by the store at append time.
fn dispatchable(actor: &ActorKey, events: &[EventRecord], effects: Vec<Effect>) -> Vec<OutboxEntry> {
    let from_events = events.iter().map(|event| OutboxEntry {
        id: 0,
        actor: actor.clone(),
        kind: event.kind.clone(),
        payload: event.payload.clone(),
        event: Some(event.clone()),
        delivered: false,
    });
    let from_effects = effects.into_iter().map(|effect| OutboxEntry {
        id: 0,
        actor: actor.clone(),
        kind: effect.kind,
        payload: effect.payload,
        event: None,
        delivered: false,
    });
    from_events.chain(from_effects).collect()
}
//#endregion 🔖️Bus

//#region 🔖️Saga
/// @emoji 🧵️ A cross-actor workflow reacting to committed facts.
///
/// A saga answers one committed [`EventRecord`] with further [`CommandEnvelope`]s, each of which
/// runs as its own independent turn against its own actor. There is deliberately no way to express
/// a transaction spanning two actors: consistency across actors is reached by **compensating
/// commands** — an action that must be undone is undone by a new command that undoes it, recorded
/// as a fact like any other. Two-phase commit, distributed locks and cross-actor rollback are all
/// absent by design, because each of them re-couples the availability of one actor to another.
#[dyn_enum]
pub trait Saga: Send + Sync {
    /// 🎬️ The commands this workflow issues in response to one committed event.
    async fn on_event(&self, event: &EventRecord) -> Vec<CommandEnvelope>;
}

/// 🔁️ The framework's reference [`Saga`]: re-issues the triggering event's command verbatim. Kept
/// in production scope (not only in tests) so [`Sagas`] closes over a real variant; a product's own
/// workflows are added as further `Sagas` variants alongside it.
pub struct EchoSaga;

impl Saga for EchoSaga {
    /// 🎬️ Re-issues a `counter.increment` at the triggering event's own stream — the framework's
    /// minimal reference workflow, exercised end-to-end by this crate's own outbox-draining test.
    async fn on_event(&self, event: &EventRecord) -> Vec<CommandEnvelope> {
        vec![CommandEnvelope {
            command_id: crate::contract::CommandId("cmd-counter.increment".into()),
            kind: "counter.increment".into(),
            version: 1,
            target: event.stream.clone(),
            scope: Scope("space-1".into()),
            principal: Principal::User { id: "alice".into() },
            session: Some(crate::contract::SessionId("s1".into())),
            device: Some(crate::contract::DeviceId("d1".into())),
            payload: vec![3],
            causal_frontier: None,
            client_hlc: HybridLogicalClock { millis: 1, counter: 0 },
            expected_revision: None,
            idempotency_key: Some(IdempotencyKey("echo".into())),
            capability_proof: None,
            trace: crate::contract::TraceContext::default(),
        }]
    }
}

dyn_enum_close! {
    pub enum Sagas: Saga {
        Echo(EchoSaga),
    }
}

/// @emoji 🔁️ Turns committed outbox rows into follow-up commands. A seam: it decides *what* to
/// issue, never *when* to run — the caller owns the scheduling.
#[derive(Default)]
pub struct SagaRunner {
    pub sagas: Vec<Sagas>,
}

impl SagaRunner {
    /// 🆕️ A runner with no workflow registered.
    pub fn new() -> Self {
        Self::default()
    }

    /// 📇️ Register one workflow.
    pub fn register(&mut self, saga: Sagas) {
        self.sagas.push(saga);
    }

    /// @emoji 🚰️ Read up to `limit` pending outbox rows, map them through every saga, mark the rows
    /// delivered and return the follow-up commands.
    ///
    /// Delivery is at-least-once: a crash between mapping and marking replays the rows, which is
    /// safe precisely because every command the sagas emit carries an [`IdempotencyKey`] and is
    /// deduplicated by [`CommandBus::submit`]. Rows without an event are pure effects and belong to
    /// the effect dispatcher, not to a saga; they are drained here so one cursor advances.
    pub async fn drain_outbox<S: AuthorityStore + ?Sized>(&mut self, store: &mut S, limit: usize) -> Vec<CommandEnvelope> {
        let entries = store.pending_outbox(limit).await.unwrap_or_default();
        let mut commands: Vec<CommandEnvelope> = Vec::new();
        for entry in &entries {
            let Some(event) = entry.event.as_ref() else { continue };
            for saga in &self.sagas {
                commands.extend(saga.on_event(event).await);
            }
        }
        let delivered: Vec<u64> = entries.iter().map(|entry| entry.id).collect();
        store.mark_outbox_delivered(&delivered).await.ok();
        commands
    }
}
//#endregion 🔖️Saga

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{CommandId, DeviceId, SessionId, TenantId, TraceContext};
    use crate::storage::MemoryAuthorityStore;

    fn key(kind: &str) -> ActorKey {
        ActorKey { tenant: TenantId("t1".into()), kind: kind.into(), id: "c1".into() }
    }

    fn command(target: &ActorKey, kind: &str, idempotency: Option<&str>) -> CommandEnvelope {
        CommandEnvelope {
            command_id: CommandId(format!("cmd-{kind}")),
            kind: kind.into(),
            version: 1,
            target: target.clone(),
            scope: Scope("space-1".into()),
            principal: Principal::User { id: "alice".into() },
            session: Some(SessionId("s1".into())),
            device: Some(DeviceId("d1".into())),
            payload: vec![3],
            causal_frontier: None,
            client_hlc: HybridLogicalClock { millis: 1, counter: 0 },
            expected_revision: None,
            idempotency_key: idempotency.map(|value| IdempotencyKey(value.into())),
            capability_proof: None,
            trace: TraceContext::default(),
        }
    }

    async fn bus() -> CommandBus<MemoryAuthorityStore> {
        allowing(Box::new(|_| PolicyDecision::Allow)).await
    }

    async fn allowing(hook: PolicyHook) -> CommandBus<MemoryAuthorityStore> {
        let mut bus = CommandBus::new(AuthorityDirectory::new(), MemoryAuthorityStore::default(), hook);
        bus.register(Deciders::Counter(CounterDecider)).await;
        bus
    }

    fn tick(millis: u64) -> HybridLogicalClock {
        HybridLogicalClock { millis, counter: 0 }
    }

    //#region 🔖️Admit
    #[semio_framework_async_macros::async_test]
    async fn a_command_for_an_unserved_actor_kind_is_rejected_as_unknown() {
        let mut bus = bus().await;
        let outcome = bus.submit(command(&key("ghost"), "ghost.poke", None), tick(1)).await;
        match outcome {
            CommandOutcome::Rejected { reason: Rejection::UnknownCommandKind { command_kind }, .. } => {
                assert_eq!(command_kind, "ghost.poke");
            }
            other => panic!("expected unknown command kind, got {other:?}"),
        }
        assert!(!bus.directory().is_active(&key("ghost")));
    }
    //#endregion 🔖️Admit

    //#region 🔖️Deduplicate
    #[semio_framework_async_macros::async_test]
    async fn an_accepted_turn_emits_events_and_bumps_the_revision() {
        let mut bus = bus().await;
        let outcome = bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(1)).await;
        match outcome {
            CommandOutcome::Accepted { receipt, events, .. } => {
                assert_eq!(events.len(), 1);
                assert_eq!(events[0].seq, 1);
                assert_eq!(events[0].stream, key(COUNTER));
                assert_eq!(events[0].hlc, tick(1));
                assert_eq!(receipt.revision, Revision(1));
            }
            other => panic!("expected acceptance, got {other:?}"),
        }
        let activation = bus.directory().activation(&key(COUNTER)).expect("placed");
        assert_eq!(read_counter(&activation.state.bytes), 3);
        assert_eq!(activation.mailbox_seq, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn resubmitting_one_idempotency_key_returns_the_identical_receipt_and_no_second_event() {
        let mut bus = bus().await;
        let first = bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(1)).await;
        let second = bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(9)).await;

        let CommandOutcome::Accepted { receipt: original, events: appended, .. } = first else {
            panic!("first submission must be accepted");
        };
        let CommandOutcome::Accepted { receipt: replayed, events: none, .. } = second else {
            panic!("retry must be accepted");
        };
        assert_eq!(appended.len(), 1);
        assert!(none.is_empty());
        assert_eq!(replayed, original);

        let activation = bus.directory().activation(&key(COUNTER)).expect("placed");
        assert_eq!(activation.mailbox_seq, 1);
        assert_eq!(read_counter(&activation.state.bytes), 3);
        assert_eq!(bus.store().last_seq(&key(COUNTER)).await.unwrap(), 1);
    }
    //#endregion 🔖️Deduplicate

    //#region 🔖️Authorize
    #[semio_framework_async_macros::async_test]
    async fn a_policy_denial_rejects_before_the_actor_is_placed() {
        let mut bus = allowing(Box::new(|_| PolicyDecision::Deny { reason: "no grant".into() })).await;
        let outcome = bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(1)).await;
        match outcome {
            CommandOutcome::Rejected { reason: Rejection::Unauthorized { detail }, .. } => {
                assert_eq!(detail, "no grant");
            }
            other => panic!("expected unauthorized, got {other:?}"),
        }
        assert!(!bus.directory().is_active(&key(COUNTER)));
    }
    //#endregion 🔖️Authorize

    //#region 🔖️Fence
    #[semio_framework_async_macros::async_test]
    async fn a_stale_expected_revision_conflicts_and_reports_the_actual_one() {
        let mut bus = bus().await;
        bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(1)).await;

        let mut stale = command(&key(COUNTER), "counter.increment", Some("k2"));
        stale.expected_revision = Some(Revision(0));
        match bus.submit(stale, tick(2)).await {
            CommandOutcome::Rejected { reason: Rejection::RevisionConflict { expected, actual }, .. } => {
                assert_eq!(expected, Revision(0));
                assert_eq!(actual, Revision(1));
            }
            other => panic!("expected revision conflict, got {other:?}"),
        }
        assert_eq!(bus.store().last_seq(&key(COUNTER)).await.unwrap(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn a_matching_expected_revision_passes_the_fence() {
        let mut bus = bus().await;
        bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(1)).await;

        let mut fresh = command(&key(COUNTER), "counter.increment", Some("k2"));
        fresh.expected_revision = Some(Revision(1));
        assert!(matches!(bus.submit(fresh, tick(2)).await, CommandOutcome::Accepted { .. }));
        assert_eq!(read_counter(&bus.directory().activation(&key(COUNTER)).unwrap().state.bytes), 6);
    }
    //#endregion 🔖️Fence

    //#region 🔖️Decide
    #[semio_framework_async_macros::async_test]
    async fn a_decider_rejection_becomes_a_rejected_outcome() {
        let mut bus = bus().await;
        match bus.submit(command(&key(COUNTER), "counter.forbid", Some("k1")), tick(1)).await {
            CommandOutcome::Rejected { reason: Rejection::Invalid { detail }, .. } => {
                assert_eq!(detail, "counter refuses");
            }
            other => panic!("expected invalid rejection, got {other:?}"),
        }
        assert_eq!(bus.store().last_seq(&key(COUNTER)).await.unwrap(), 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn a_decider_deferral_becomes_a_pending_outcome() {
        let mut bus = bus().await;
        match bus.submit(command(&key(COUNTER), "counter.rebuild", Some("k1")), tick(1)).await {
            CommandOutcome::Pending { receipt, process } => {
                assert_eq!(process, ProcessId("rebuild-1".into()));
                assert_eq!(receipt.revision, Revision(0));
            }
            other => panic!("expected pending, got {other:?}"),
        }
        assert_eq!(bus.store().last_seq(&key(COUNTER)).await.unwrap(), 0);
    }
    //#endregion 🔖️Decide

    //#region 🔖️Saga
    #[semio_framework_async_macros::async_test]
    async fn the_outbox_drains_exactly_once_across_two_calls() {
        let mut bus = bus().await;
        bus.submit(command(&key(COUNTER), "counter.increment", Some("k1")), tick(1)).await;
        bus.submit(command(&key(COUNTER), "counter.increment", Some("k2")), tick(2)).await;

        let mut runner = SagaRunner::new();
        runner.register(Sagas::Echo(EchoSaga));

        let first = runner.drain_outbox(bus.store_mut(), 64).await;
        assert_eq!(first.len(), 2);
        assert!(first.iter().all(|follow_up| follow_up.kind == "counter.increment"));

        let second = runner.drain_outbox(bus.store_mut(), 64).await;
        assert!(second.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn effects_reach_the_outbox_without_an_event() {
        let mut bus = bus().await;
        bus.submit(command(&key(COUNTER), "counter.audit", Some("k1")), tick(1)).await;
        let pending = bus.store().pending_outbox(64).await.unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].kind, "counter.audited");
        assert!(pending[0].event.is_none());
    }
    //#endregion 🔖️Saga

    //#region 🔖️Directory
    #[semio_framework_async_macros::async_test]
    async fn passivation_fences_the_next_activation_with_a_higher_epoch() {
        let mut directory = AuthorityDirectory::new();
        directory.activate(key(COUNTER), "authority").unwrap();
        let first = directory.activation_epoch(&key(COUNTER)).unwrap();
        directory.passivate(&key(COUNTER));
        assert!(!directory.is_active(&key(COUNTER)));
        directory.activate(key(COUNTER), "authority").unwrap();
        assert!(directory.activation_epoch(&key(COUNTER)).unwrap() > first);
    }
    //#endregion 🔖️Directory
}
