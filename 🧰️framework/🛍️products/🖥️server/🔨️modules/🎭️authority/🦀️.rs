//! 🎭️ Authority: the actor turn protocol — the middle of three layers that never merge.
//!
//! The **CQRS dual bus** ([`CommandEnvelope`]/`QueryEnvelope`) carries application intent; the
//! **actor turn protocol** in this file serializes that intent through exactly one consistency
//! boundary; the **replication protocol** (`protocol` crate) moves the resulting causal state
//! between replicas. Collapsing any two of them produces a system that cannot be reasoned about:
//! a bus would start sequencing storage, a turn would start speaking a transport, replication
//! would start deciding. This file owns the middle layer and nothing else — admission,
//! deduplication, placement, revision fencing, decision, evolution, and the one commit that makes a
//! batch of turns durable: events, outbox, receipts, snapshots.
//!
//! Three laws govern everything here. **Exactly-once**: a resubmitted [`IdempotencyKey`] returns
//! the byte-identical [`CommandReceipt`] to the principal that was answered with it and appends no
//! second event. **A key is its principal's own**: receipts are stored per principal
//! ([`receipt_key`]), so no caller can read or occupy a key another principal — a workflow's service
//! account included — uses. **Never trust the client**: the optimistic replica runs the very same
//! [`Decider`] and may produce events, but the authority re-derives and re-stamps every
//! [`EventRecord`] it commits.
//!
//! No transport, no runtime, no clock: callers pass the [`HybridLogicalClock`] reading in. The store
//! sits behind a shared read-write lock ([`SharedStore`]) so a reader of history never waits for a
//! turn.

use std::collections::HashMap;
use std::sync::Arc;

use crate::contract::{ActorKey, CommandEnvelope, CommandOutcome, CommandReceipt, EventRecord, HybridLogicalClock, IdempotencyKey, Notice, PolicyDecision, Principal, ProcessId, Rejection, Revision, Scope};
use crate::policy::principal_key;
use crate::storage::{AuthorityStore, Lease, OutboxEntry, StorageError, TurnCommit};
use semio_framework_dispatch_macros::dyn_enum;
use std::future::Future;
use tokio::sync::RwLock;

//#region 🔖️Error
/// 💥️ What can go wrong inside a turn that is not a domain [`Rejection`]. A rejection is an
/// answer the caller asked for; an [`AuthorityError`] is the authority failing to answer at all.
#[derive(Debug)]
pub enum AuthorityError {
    /// 🔓️ The activation's fencing lease is no longer held — another epoch owns this actor.
    LeaseLost,
    /// 👻️ No [`Decider`] is registered for this actor kind on this instance.
    UnknownActorKind(String),
    /// 🗄️ The durable store refused or failed the turn.
    Storage(String),
}

impl std::fmt::Display for AuthorityError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeaseLost => formatter.write_str("lease lost for the requested actor"),
            Self::UnknownActorKind(kind) => write!(formatter, "unknown actor kind: {kind}"),
            Self::Storage(detail) => write!(formatter, "storage failure: {detail}"),
        }
    }
}

impl std::error::Error for AuthorityError {}
//#endregion 🔖️Error

//#region 🔖️Turn
/// 🧠️ One actor's private state as the turn protocol sees it: a fenced [`Revision`] plus an
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

/// 🧭️ Everything a decision may read that is not the actor's own state. Deliberately tiny:
/// anything absent here is ambient input a pure decision is forbidden to reach for.
#[derive(Clone, Debug, PartialEq)]
pub struct DecisionContext {
    pub now: HybridLogicalClock,
    pub principal: Principal,
    pub scope: Scope,
}

/// 📣️ A side effect a decision requests: mail, webhook, blob transcode, push. Committed to
/// the outbox in the same write as the events, dispatched later, never inside the turn.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Effect {
    pub kind: String,
    pub payload: Vec<u8>,
}

/// 🎯️ The whole result of one decision. There is no fourth branch on purpose: a turn either
/// produces facts, refuses, or hands the work to a long-running process.
#[derive(Clone, Debug, PartialEq)]
pub enum Decision {
    Emit { events: Vec<EventRecord>, effects: Vec<Effect> },
    Reject(Rejection),
    Defer(ProcessId),
}

/// ⚖️ The deterministic core that the optimistic client replica and the authority both run,
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
/// **Send futures, declared not inferred.** Every method of this port returns
/// `impl Future<..> + Send` instead of being written `async fn`, and that is structural, not a
/// style choice: [`ServerState`](crate::gateway::ServerState) reaches this port behind an
/// instance's associated type, so the concrete future is opaque at the call site and axum's
/// handler and socket tasks — which are `Send` by construction — cannot otherwise prove it may
/// cross a thread. An `async fn` here compiles and then fails at every route that uses it. The
/// implementations stay ordinary `async fn`, which Rust accepts against this signature, and so does
/// the delegate `dyn_enum_close!` generates for a set of them: the macro emits `async fn .. -> T`
/// over the future's `Output`, because two match arms cannot unify two distinct opaque futures.
#[dyn_enum]
pub trait Decider: Send + Sync {
    /// 🏷️ The actor kind this decider serves, matched against [`ActorKey::kind`].
    fn actor_kind(&self) -> impl Future<Output = &str> + Send;
    /// 🎲️ Decide one command against one state. Pure — see the trait documentation.
    fn decide(&self, state: &ActorState, command: &CommandEnvelope, context: &DecisionContext) -> impl Future<Output = Decision> + Send;
    /// 🌀️ Fold one committed event into the domain state. Never mutates the revision.
    fn evolve(&self, state: &mut ActorState, event: &EventRecord) -> impl Future<Output = ()> + Send;
    /// 🧬️ The revision of this decider's fold and of the encoding of `state.bytes`. The bus stamps
    /// it on every snapshot it writes and ignores a stored snapshot of another revision, replaying
    /// the stream instead — so it MUST be raised whenever `evolve` or that encoding changes.
    fn state_format(&self) -> impl Future<Output = u32> + Send {
        async { 0 }
    }
}

//#endregion 🔖️Turn

//#region 🔖️Directory
/// 📇️ One actor kind bound to the implementation that serves it. `D` is the instance's
/// decider type — [`ServerInstance::Deciders`](crate::gateway::ServerInstance::Deciders).
pub struct ActorRegistration<D: Decider> {
    pub actor_kind: String,
    pub decider: D,
}

impl<D: Decider> ActorRegistration<D> {
    /// 🔗️ Bind a decider under the actor kind it declares.
    pub async fn new(decider: D) -> Self {
        Self { actor_kind: decider.actor_kind().await.to_string(), decider }
    }
}

/// 🔥️ One live actor: its fencing lease, its state, how far its mailbox has been consumed,
/// which snapshot version that state was last persisted at and when it last took a turn.
#[derive(Clone, Debug)]
pub struct Activation {
    pub lease: Lease,
    pub state: ActorState,
    pub mailbox_seq: u64,
    pub snapshot_version: u64,
    used: u64,
}

/// 🧮️ How many actors a directory keeps placed unless it is [`bounded`](AuthorityDirectory::bounded)
/// otherwise.
pub const DIRECTORY_CAPACITY: usize = 2048;

/// 🗺️ Where actors live and under whose lease.
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
///
/// **Bounded.** A directory keeps at most its capacity of actors placed: placing one more
/// passivates the actor that took its last turn longest ago. An activation is a cache of what the
/// store holds — its next placement rehydrates it — so the bound costs a replay, never a fact, and
/// the memory of a server no longer grows with the number of actors ever addressed.
pub struct AuthorityDirectory {
    activations: HashMap<ActorKey, Activation>,
    next_epoch: u64,
    capacity: usize,
    turns: u64,
}

impl Default for AuthorityDirectory {
    /// 🌱️ An empty directory of [`DIRECTORY_CAPACITY`].
    fn default() -> Self {
        Self::bounded(DIRECTORY_CAPACITY)
    }
}

impl AuthorityDirectory {
    /// 🆕️ An empty directory with no actor placed.
    pub fn new() -> Self {
        Self::default()
    }

    /// 📏️ An empty directory keeping at most `capacity` actors placed (at least one).
    pub fn bounded(capacity: usize) -> Self {
        Self { activations: HashMap::new(), next_epoch: 0, capacity: capacity.max(1), turns: 0 }
    }

    /// 🔥️ Place `key` under `holder`, minting a fenced activation epoch on first placement and
    /// returning the already-live activation otherwise. A placement beyond the capacity passivates
    /// the least recently used actor first.
    pub fn activate(&mut self, key: &ActorKey, holder: &str) -> Result<&mut Activation, AuthorityError> {
        self.turns += 1;
        if !self.activations.contains_key(key) {
            if self.activations.len() >= self.capacity {
                let coldest = self.activations.iter().min_by_key(|(_, activation)| activation.used).map(|(coldest, _)| coldest.clone());
                if let Some(coldest) = coldest {
                    self.activations.remove(&coldest);
                }
            }
            let epoch = self.next_epoch;
            self.next_epoch += 1;
            let lease = Lease { epoch, holder: holder.to_string() };
            let activation = Activation { lease, state: ActorState::default(), mailbox_seq: 0, snapshot_version: 0, used: 0 };
            self.activations.insert(key.clone(), activation);
        }
        let activation = self.activations.get_mut(key).ok_or(AuthorityError::LeaseLost)?;
        activation.used = self.turns;
        Ok(activation)
    }

    /// 🔢️ How many actors are currently placed here.
    pub fn placed(&self) -> usize {
        self.activations.len()
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
/// 🚦️ The policy admission callback. A boxed closure rather than a concrete engine so the
/// turn protocol depends on the *decision*, never on how the decision was reached.
pub type PolicyHook = Box<dyn Fn(&CommandEnvelope) -> PolicyDecision + Send + Sync>;

/// 🛃️ The shape admission callback, asked before anything is read or placed: whether this
/// envelope may address its target at all. An instance refuses a target id (or any other envelope
/// field) of a shape its actors never have here, so a malformed address costs neither a storage
/// read nor an activation. Pure and synchronous: it judges the envelope and nothing else.
pub type AdmissionHook = Box<dyn Fn(&CommandEnvelope) -> Result<(), Rejection> + Send + Sync>;

/// 🤝️ The authority store as the bus and every reader of history share it: a turn takes the write
/// half only around its own commits, so reading a stream waits for one store call at most and never
/// for the turns queued on the bus.
pub type SharedStore<S> = Arc<RwLock<S>>;

/// 📸️ How many committed events an actor may run ahead of its stored snapshot before the bus
/// writes the next one, unless the bus is told otherwise with [`CommandBus::snapshotting`].
pub const SNAPSHOT_INTERVAL: u64 = 64;

/// 🚫️ The `invalid` detail of a command whose idempotency key its principal already used for
/// another actor.
pub const IDEMPOTENCY_CONFLICT: &str = "idempotency-key-conflict";

/// 🚫️ The `unauthorized` detail of every policy denial. What was missing is the policy engine's
/// knowledge, not the caller's.
pub const FORBIDDEN: &str = "forbidden";

/// 🚫️ The `actorUnavailable` detail of every turn the authority itself failed. The cause goes to the
/// operator's log, never to the caller.
pub const UNAVAILABLE: &str = "unavailable";

/// 🗝️ The key a principal's idempotency key is stored under: the principal's own key space, so the
/// same client-chosen text presented by two principals names two receipts. The length prefix keeps
/// the pair unambiguous whatever characters either half carries.
pub fn receipt_key(principal: &Principal, key: &IdempotencyKey) -> IdempotencyKey {
    let owner = principal_key(principal);
    IdempotencyKey(format!("{}:{owner}:{}", owner.len(), key.0))
}

/// 🏛️ The command side of the dual bus: it runs exactly one turn per submitted command,
/// against exactly one actor, in a fixed and non-negotiable order.
pub struct CommandBus<S: AuthorityStore, D: Decider> {
    directory: AuthorityDirectory,
    store: SharedStore<S>,
    policy_hook: PolicyHook,
    admission_hook: AdmissionHook,
    registrations: HashMap<String, ActorRegistration<D>>,
    holder: String,
    snapshot_interval: u64,
    committed: Committed,
}

/// 🧮️ How much a bus has committed since it was built: events and outbox rows. Whoever relays the
/// outbox or folds read models compares these with what it last relayed and folded, and asks the
/// store only when they moved.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Committed {
    pub events: u64,
    pub outbox: u64,
}

impl<S: AuthorityStore, D: Decider> CommandBus<S, D> {
    /// 🆕️ Build a bus over a placement directory, a durable store and a policy admission hook. It
    /// admits every envelope shape and snapshots every [`SNAPSHOT_INTERVAL`] events until told
    /// otherwise.
    pub fn new(directory: AuthorityDirectory, store: S, policy_hook: PolicyHook) -> Self {
        Self { directory, store: Arc::new(RwLock::new(store)), policy_hook, admission_hook: Box::new(|_| Ok(())), registrations: HashMap::new(), holder: HOLDER.to_string(), snapshot_interval: SNAPSHOT_INTERVAL, committed: Committed::default() }
    }

    /// 🛃️ Ask `hook` about every envelope before anything else is done with it.
    pub fn admitting(mut self, hook: AdmissionHook) -> Self {
        self.admission_hook = hook;
        self
    }

    /// 📸️ Write an actor's snapshot once it is `interval` events ahead of the stored one; `0`
    /// writes none.
    pub fn snapshotting(mut self, interval: u64) -> Self {
        self.snapshot_interval = interval;
        self
    }

    /// 📇️ Register a decider under the actor kind it declares, replacing any previous one.
    pub async fn register(&mut self, decider: D) {
        let registration = ActorRegistration::new(decider).await;
        self.registrations.insert(registration.actor_kind.clone(), registration);
    }

    /// 🗺️ The placement directory this bus runs turns against.
    pub fn directory(&self) -> &AuthorityDirectory {
        &self.directory
    }

    /// 🗄️ The durable store this bus commits into, shared: a reader clones the handle once and
    /// never takes the bus again.
    pub fn store(&self) -> &SharedStore<S> {
        &self.store
    }

    /// 🧮️ How many events and outbox rows this bus has committed so far.
    pub fn committed(&self) -> Committed {
        self.committed
    }

    /// 📨️ Run one turn: a batch of one ([`submit_batch`](Self::submit_batch)).
    pub async fn submit(&mut self, envelope: CommandEnvelope, now: HybridLogicalClock) -> CommandOutcome {
        let lost = unavailable(&envelope, Revision(0), now, &"the bus answered no outcome");
        self.submit_batch(vec![(envelope, now)]).await.pop().unwrap_or(lost)
    }

    /// 📦️ Run a batch of turns in order and make them durable together, answering each command.
    ///
    /// A batch is how an authority under load keeps up: every turn is still decided one at a time,
    /// in order, against the state the turns before it left, but the batch's facts reach the store
    /// in one commit ([`AuthorityStore::commit`]) — one flush for a burst of commands instead of
    /// one or two per command. No command is answered before its facts are durable.
    ///
    /// The order below is the protocol and may not be rearranged. Per command:
    ///
    /// 1. **Admit** — a command is servable only if a [`Decider`] is registered for its target
    ///    actor kind ([`Rejection::UnknownCommandKind`] otherwise) and the [`AdmissionHook`] accepts
    ///    the envelope's shape. Cheapest checks first, so an unroutable or malformed command costs
    ///    no storage read and places no actor.
    /// 2. **Deduplicate** — if the caller's principal already holds a [`CommandReceipt`] for this
    ///    [`IdempotencyKey`] ([`receipt_key`]) — stored, or staged earlier in this very batch — and
    ///    it is a receipt of this very target, answer that same receipt as
    ///    [`CommandOutcome::Accepted`] with no events. A receipt of another actor is never handed
    ///    out: the command is refused with [`IDEMPOTENCY_CONFLICT`] and an answer built from the
    ///    caller's own envelope. This runs *before* policy and *before* placement: a retry must be
    ///    answered identically even if the caller's grants or the actor's placement changed since
    ///    the original turn. This is the exactly-once law, and it is the only reason a client may
    ///    retry a timed-out submission.
    /// 3. **Authorize** — [`PolicyPoint::CommandAdmission`](crate::contract::PolicyPoint); a deny
    ///    becomes [`Rejection::Unauthorized`] carrying [`FORBIDDEN`] and the actor is never even
    ///    placed.
    /// 4. **Place** — acquire the activation and its fencing lease; a fresh activation is first
    ///    rehydrated from its snapshot and durable stream, so a restarted authority decides
    ///    against the state its history implies and appends after the last committed sequence.
    /// 5. **Fence the revision** — a stated `expected_revision` that disagrees with the activation
    ///    yields [`Rejection::RevisionConflict`] carrying the actual revision, so the client can
    ///    rebase rather than guess.
    /// 6. **Decide** — the pure core runs. Nothing before this point consulted the domain.
    ///    [`Decision::Reject`] is answered [`CommandOutcome::Rejected`] and [`Decision::Defer`]
    ///    [`CommandOutcome::Pending`] at once; neither writes anything.
    /// 7. **Stage** — the decided events are re-stamped by the authority, their outbox rows and
    ///    the receipt are prepared, and the events are folded into the in-memory state so the next
    ///    command of the batch decides against them. Once the state is a whole snapshot interval
    ///    ahead of the stored snapshot, a snapshot stamped with the decider's
    ///    [`state_format`](Decider::state_format) is staged with it.
    /// 8. **Release** — an actor still at revision zero after its turn has no history to keep
    ///    placed and is passivated, so refused commands for ids that never existed leave nothing
    ///    behind.
    ///
    /// Then, once for the batch:
    ///
    /// 9. **Commit** — every staged turn's events, outbox rows, receipt and snapshot go to the
    ///    store in one call, so an effect can never escape without its causing event, nor an event
    ///    be visible without its pending effect, nor — on a backend that commits atomically — a
    ///    fact land without the receipt that makes its retry harmless.
    /// 10. **Answer** — a committed turn is [`CommandOutcome::Accepted`] with its events. A turn
    ///     the store refused is answered [`Rejection::ActorUnavailable`], and its actor is
    ///     passivated: what was decided ahead of the store is discarded, and the actor's next turn
    ///     rehydrates what is durable.
    pub async fn submit_batch(&mut self, commands: Vec<(CommandEnvelope, HybridLogicalClock)>) -> Vec<CommandOutcome> {
        let mut answers: Vec<Option<CommandOutcome>> = Vec::with_capacity(commands.len());
        let mut staged: Vec<Staged> = Vec::new();
        let mut turns = 0;
        for (envelope, now) in commands {
            if turns >= self.directory.capacity {
                self.commit(&mut staged, &mut answers).await;
                turns = 0;
            }
            turns += 1;
            let slot = answers.len();
            let answer = self.stage(envelope, now, slot, &mut staged).await;
            answers.push(answer);
        }
        self.commit(&mut staged, &mut answers).await;
        answers.into_iter().flatten().collect()
    }

    /// 🎬️ Steps one to eight for one command: its answer, or `None` when its turn is staged (or
    /// replays a turn staged in this batch) and is answered by the commit.
    async fn stage(&mut self, envelope: CommandEnvelope, now: HybridLogicalClock, slot: usize, staged: &mut Vec<Staged>) -> Option<CommandOutcome> {
        //#region 🔖️Admit
        let Some(registration) = self.registrations.get(&envelope.target.kind) else {
            let reason = Rejection::UnknownCommandKind { command_kind: envelope.kind.clone() };
            return Some(refuse(&envelope, Revision(0), now, reason));
        };
        if let Err(reason) = (self.admission_hook)(&envelope) {
            return Some(refuse(&envelope, Revision(0), now, reason));
        }
        //#endregion 🔖️Admit

        //#region 🔖️Deduplicate
        let key = envelope.idempotency_key.as_ref().map(|key| receipt_key(&envelope.principal, key));
        if let Some(key) = &key {
            if let Some(earlier) = staged.iter_mut().find(|earlier| earlier.key.as_ref() == Some(key)) {
                if earlier.envelope.target != envelope.target {
                    return Some(refuse(&envelope, Revision(0), now, Rejection::Invalid { detail: IDEMPOTENCY_CONFLICT.to_string() }));
                }
                earlier.replays.push(slot);
                return None;
            }
            let stored = self.store.read().await.receipt(key).await;
            match stored {
                Ok(Some(receipt)) if receipt.actor == envelope.target => return Some(CommandOutcome::Accepted { receipt, events: Vec::new(), frontier: None }),
                Ok(Some(_)) => return Some(refuse(&envelope, Revision(0), now, Rejection::Invalid { detail: IDEMPOTENCY_CONFLICT.to_string() })),
                Ok(None) => {}
                Err(error) => return Some(unavailable(&envelope, Revision(0), now, &error)),
            }
        }
        //#endregion 🔖️Deduplicate

        //#region 🔖️Authorize
        if let PolicyDecision::Deny { .. } = (self.policy_hook)(&envelope) {
            return Some(refuse(&envelope, Revision(0), now, Rejection::Unauthorized { detail: FORBIDDEN.to_string() }));
        }
        //#endregion 🔖️Authorize

        //#region 🔖️Place
        let placed = self.directory.is_active(&envelope.target);
        let activation = match self.directory.activate(&envelope.target, &self.holder) {
            Ok(activation) => activation,
            Err(error) => return Some(unavailable(&envelope, Revision(0), now, &error)),
        };
        if !placed {
            if let Err(error) = rehydrate(&self.store, &registration.decider, &envelope.target, activation).await {
                self.directory.passivate(&envelope.target);
                return Some(unavailable(&envelope, Revision(0), now, &error));
            }
        }
        //#endregion 🔖️Place

        let decided = decide(&registration.decider, activation, &envelope, now, self.snapshot_interval).await;

        //#region 🔖️Release
        if activation.state.revision == Revision(0) {
            self.directory.passivate(&envelope.target);
        }
        //#endregion 🔖️Release

        match decided {
            Ok(turn) => {
                staged.push(Staged { slot, envelope, now, key, replays: Vec::new(), turn });
                None
            }
            Err(answer) => Some(*answer),
        }
    }

    /// 🧾️ Steps nine and ten: commit every staged turn in one store call and answer each.
    async fn commit(&mut self, staged: &mut Vec<Staged>, answers: &mut [Option<CommandOutcome>]) {
        if staged.is_empty() {
            return;
        }
        //#region 🔖️Commit
        let turns: Vec<TurnCommit<'_>> = staged
            .iter()
            .map(|staged| TurnCommit {
                actor: &staged.envelope.target,
                events: &staged.turn.events,
                outbox: &staged.turn.outbox,
                receipt: staged.key.as_ref().map(|key| (key, &staged.turn.receipt)),
                snapshot: staged.turn.snapshot.as_deref().map(|bytes| (staged.turn.receipt.revision, bytes)),
            })
            .collect();
        let mut fates = self.store.write().await.commit(&turns).await.into_iter();
        drop(turns);
        //#endregion 🔖️Commit

        //#region 🔖️Answer
        for Staged { slot, envelope, now, replays, turn, .. } in staged.drain(..) {
            match fates.next().unwrap_or_else(|| Err(StorageError::Backend("the store answered no fate for a committed turn".to_string()))) {
                Ok(()) => {
                    self.committed.events += turn.events.len() as u64;
                    self.committed.outbox += turn.outbox.len() as u64;
                    for replay in replays {
                        answers[replay] = Some(CommandOutcome::Accepted { receipt: turn.receipt.clone(), events: Vec::new(), frontier: None });
                    }
                    answers[slot] = Some(CommandOutcome::Accepted { receipt: turn.receipt, events: turn.events, frontier: None });
                }
                Err(error) => {
                    self.directory.passivate(&envelope.target);
                    for refused in replays.into_iter().chain([slot]) {
                        answers[refused] = Some(unavailable(&envelope, turn.before, now, &error));
                    }
                }
            }
        }
        //#endregion 🔖️Answer
    }
}

/// 🏷️ The holder name a single-process authority leases actors under.
const HOLDER: &str = "authority";

/// 📝️ What one accepted turn commits: its re-stamped events, their outbox rows, the receipt, the
/// snapshot when one is due, and the revision the actor had before.
struct Turn {
    events: Vec<EventRecord>,
    outbox: Vec<OutboxEntry>,
    receipt: CommandReceipt,
    snapshot: Option<Vec<u8>>,
    before: Revision,
}

/// ⏳️ One decided turn waiting for its batch's commit: the answer slot of its command, the
/// envelope and clock the answer is built from, the key its receipt is stored under, and the slots
/// of later commands of the batch that replayed that key.
struct Staged {
    slot: usize,
    envelope: CommandEnvelope,
    now: HybridLogicalClock,
    key: Option<IdempotencyKey>,
    replays: Vec<usize>,
    turn: Turn,
}

/// ⚖️ Steps five to seven of [`CommandBus::submit_batch`]: one placed actor decides one admitted,
/// authorized, not-yet-seen command. `Err` is the command's answer when it emits nothing.
async fn decide<D: Decider>(decider: &D, activation: &mut Activation, envelope: &CommandEnvelope, now: HybridLogicalClock, snapshot_interval: u64) -> Result<Turn, Box<CommandOutcome>> {
    //#region 🔖️Fence
    let before = activation.state.revision;
    if let Some(expected) = envelope.expected_revision {
        if expected != before {
            return Err(Box::new(refuse(envelope, before, now, Rejection::RevisionConflict { expected, actual: before })));
        }
    }
    //#endregion 🔖️Fence

    //#region 🔖️Decide
    let context = DecisionContext { now, principal: envelope.principal.clone(), scope: envelope.scope.clone() };
    let (events, effects) = match decider.decide(&activation.state, envelope, &context).await {
        Decision::Emit { events, effects } => (events, effects),
        Decision::Reject(reason) => return Err(Box::new(refuse(envelope, before, now, reason))),
        Decision::Defer(process) => return Err(Box::new(CommandOutcome::Pending { receipt: acknowledge(envelope, before, now), process })),
    };
    //#endregion 🔖️Decide

    //#region 🔖️Stage
    let events = seal(&envelope.target, activation.mailbox_seq, now, events);
    let outbox = dispatchable(&envelope.target, &events, effects);
    for event in &events {
        decider.evolve(&mut activation.state, event).await;
    }
    if let Some(last) = events.last() {
        activation.mailbox_seq = last.seq;
        activation.state.revision = Revision(last.seq);
    }
    let revision = activation.state.revision;
    let due = snapshot_interval > 0 && revision.0 - activation.snapshot_version >= snapshot_interval;
    let snapshot = if due { Some(frame(decider.state_format().await, &activation.state.bytes)) } else { None };
    if due {
        activation.snapshot_version = revision.0;
    }
    Ok(Turn { events, outbox, receipt: acknowledge(envelope, revision, now), snapshot, before })
    //#endregion 🔖️Stage
}

/// 💧️ Rebuild a freshly placed activation from the durable store before its first decision: the
/// newest snapshot of the decider's current [`state_format`](Decider::state_format) (if any) seeds
/// the state at its revision, and every later event is folded through the decider in order. Without
/// it a restarted authority would decide against an empty state and append from sequence one onto a
/// stream that already holds history. The stream is read in one store call and folded after the
/// store is released.
async fn rehydrate<S: AuthorityStore, D: Decider>(store: &RwLock<S>, decider: &D, actor: &ActorKey, activation: &mut Activation) -> Result<(), StorageError> {
    let snapshot = store.read().await.snapshot(actor).await?;
    if let Some((revision, framed)) = snapshot {
        if let Some(bytes) = unframe(decider.state_format().await, &framed) {
            activation.state = ActorState { revision, bytes: bytes.to_vec() };
            activation.mailbox_seq = revision.0;
            activation.snapshot_version = revision.0;
        }
    }
    let events = store.read().await.events_since(actor, activation.mailbox_seq).await?;
    for event in events {
        decider.evolve(&mut activation.state, &event).await;
        activation.mailbox_seq = event.seq;
        activation.state.revision = Revision(event.seq);
    }
    Ok(())
}

/// 🖼️ A snapshot as the bus stores it: the decider's state format, little-endian, then the state.
fn frame(format: u32, state: &[u8]) -> Vec<u8> {
    let mut framed = Vec::with_capacity(4 + state.len());
    framed.extend_from_slice(&format.to_le_bytes());
    framed.extend_from_slice(state);
    framed
}

/// 🔓️ The state inside a stored snapshot, when it was written in `format`.
fn unframe(format: u32, framed: &[u8]) -> Option<&[u8]> {
    let (stamp, state) = framed.split_at_checked(4)?;
    (stamp == format.to_le_bytes()).then_some(state)
}

/// 🧾️ The receipt describing where this command left the actor.
fn acknowledge(envelope: &CommandEnvelope, revision: Revision, now: HybridLogicalClock) -> CommandReceipt {
    CommandReceipt { command_id: envelope.command_id.clone(), actor: envelope.target.clone(), revision, accepted_at: now }
}

/// 🚫️ A refusal carrying the receipt the caller still needs to correlate the answer.
fn refuse(envelope: &CommandEnvelope, revision: Revision, now: HybridLogicalClock, reason: Rejection) -> CommandOutcome {
    CommandOutcome::Rejected { receipt: acknowledge(envelope, revision, now), reason, notices: Vec::new() }
}

/// 🗄️ A storage or placement failure surfaced as a retryable refusal rather than a panic. The
/// cause is reported to the operator; the caller learns only that a retry may succeed.
fn unavailable(envelope: &CommandEnvelope, revision: Revision, now: HybridLogicalClock, cause: &dyn std::fmt::Display) -> CommandOutcome {
    crate::report("authority turn", cause);
    let notices = vec![Notice { code: "authority.retryable".into(), message: UNAVAILABLE.to_string() }];
    let reason = Rejection::ActorUnavailable { detail: UNAVAILABLE.to_string() };
    CommandOutcome::Rejected { receipt: acknowledge(envelope, revision, now), reason, notices }
}

/// 🔏️ Re-stamp decided events with authority-assigned stream, sequence and clock, keeping only the
/// domain-owned `kind` and `payload`, so nothing a client proposed can reach the log unverified.
fn seal(actor: &ActorKey, from_seq: u64, now: HybridLogicalClock, events: Vec<EventRecord>) -> Vec<EventRecord> {
    events.into_iter().enumerate().map(|(offset, event)| EventRecord { stream: actor.clone(), seq: from_seq + offset as u64 + 1, hlc: now, kind: event.kind, payload: event.payload }).collect()
}

/// 📤️ The outbox rows for one turn: one per committed event for saga fan-out, one per requested
/// effect for the dispatcher. Ids are assigned by the store at append time.
fn dispatchable(actor: &ActorKey, events: &[EventRecord], effects: Vec<Effect>) -> Vec<OutboxEntry> {
    let from_events = events.iter().map(|event| OutboxEntry { id: 0, actor: actor.clone(), kind: event.kind.clone(), payload: event.payload.clone(), event: Some(event.clone()), delivered: false });
    let from_effects = effects.into_iter().map(|effect| OutboxEntry { id: 0, actor: actor.clone(), kind: effect.kind, payload: effect.payload, event: None, delivered: false });
    from_events.chain(from_effects).collect()
}
//#endregion 🔖️Bus

//#region 🔖️Saga
/// 🧵️ A cross-actor workflow reacting to committed facts.
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

/// 🔁️ Turns committed outbox rows into follow-up commands. A seam: it decides *what* to
/// issue, never *when* to run — the caller owns the scheduling.
pub struct SagaRunner<W: Saga> {
    pub sagas: Vec<W>,
}

impl<W: Saga> Default for SagaRunner<W> {
    /// 🌿️ A runner with no workflow — `derive(Default)` would demand `W: Default`, which a
    /// workflow holding configuration never is.
    fn default() -> Self {
        Self { sagas: Vec::new() }
    }
}

impl<W: Saga> SagaRunner<W> {
    /// 🆕️ A runner with no workflow registered.
    pub fn new() -> Self {
        Self::default()
    }

    /// 📇️ Register one workflow.
    pub fn register(&mut self, saga: W) {
        self.sagas.push(saga);
    }

    /// 🚰️ Read up to `limit` pending outbox rows, map them through every saga, mark the rows
    /// delivered and return the follow-up commands.
    ///
    /// Delivery is at-least-once: a crash between mapping and marking replays the rows, which is
    /// safe precisely because every command the sagas emit carries an [`IdempotencyKey`](crate::contract::IdempotencyKey) and is
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
