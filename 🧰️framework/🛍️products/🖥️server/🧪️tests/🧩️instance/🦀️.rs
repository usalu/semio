//! 🧩️ The reference [`ServerInstance`] this crate tests itself with.
//!
//! Everything in this file used to sit in production scope, because the previous de-dyn pass closed
//! every port over a single demo variant inside the framework: the counting module, the counter
//! decider, the echo saga and the four in-memory stores were the *only* implementations the product
//! admitted, which is what made the product unusable from outside it. They are fixtures, so they
//! live with the fixtures now, and the framework ships none of them.
//!
//! [`TestInstance`] is also the worked example a real instance follows, and it shows both shapes a
//! port can take at a closing site. Eight ports name one concrete type directly — the honest shape
//! when a deployment has exactly one implementation of a role. [`TestDeciders`] and [`TestSagas`]
//! each close two implementations with `dyn_enum_close!`, one over a port declaring
//! `impl Future<..> + Send` and one over a port left as plain `async fn`, and both are closed from
//! a module that did not declare the trait — the position every downstream instance crate is in.
//! Either way the set is closed here, at the site that owns the implementations, and not inside the
//! framework.

use semio_framework_dispatch_macros::dyn_enum_close;
use std::collections::{BTreeMap, HashMap};

use crate::authority::{ActorState, Decider, Decision, DecisionContext, Effect, Saga};
use crate::contract::{
    ActorKey, CommandEnvelope, CommandId, CommandReceipt, DeviceId, EventRecord, HybridLogicalClock, IdempotencyKey, ModuleManifest, PolicyGrant, PolicyPoint, PolicyTemplate, Principal, ProcessId, Rejection, Revision, Scope,
    SessionId, TraceContext,
};
use crate::gateway::{GatewayRouter, InstanceStores, NoDocumentAuthority, NoQueryHandler, ServerInstance, ServerModule, ServerState};
use crate::policy::{Credential, PrincipalResolver, Resolved};
use crate::storage::{AuthorityStore, BlobStore, Lease, OutboxEntry, ProjectionStore, SessionRecord, SessionStore, StorageError, StorageProfile};
use protocol::codec::ids::ContentHash;

//#region 🔖️Instance
/// 🪪️ The deployment this crate's own tests run: every port in memory, nothing durable, one module.
pub struct TestInstance;

impl ServerInstance for TestInstance {
    type Modules = CountingModule;
    type Queries = NoQueryHandler;
    type Documents = NoDocumentAuthority;
    type Deciders = TestDeciders;
    type Sagas = TestSagas;
    type Resolvers = BearerTokenResolver;
    type AuthorityStore = MemoryAuthorityStore;
    type ProjectionStore = MemoryProjectionStore;
    type BlobStore = MemoryBlobStore;
    type SessionStore = MemorySessionStore;

    /// 🗄️ Opens four empty in-memory stores and ignores `data_dir`, which is this profile's whole
    /// point: the framework no longer decides that a server persists nothing — this instance does,
    /// and a durable instance decides otherwise without touching the framework.
    async fn open(_profile: &StorageProfile) -> Result<InstanceStores<Self>, StorageError> {
        Ok(InstanceStores { authority: MemoryAuthorityStore::new(), projections: MemoryProjectionStore::new(), blobs: MemoryBlobStore::new(), sessions: MemorySessionStore::new() })
    }
}
//#endregion 🔖️Instance

//#region 🔖️Module
/// 🧮️ Contributes one policy template and one health route, nothing else.
pub struct CountingModule;

impl ServerModule for CountingModule {
    type Instance = TestInstance;

    async fn manifest(&self) -> ModuleManifest {
        ModuleManifest { id: "counting".into(), policies: vec![PolicyTemplate { name: "author".into(), auto_apply: false, grants: vec![PolicyGrant { point: PolicyPoint::CommandAdmission, resource: "*".into(), action: "*".into() }] }], ..Default::default() }
    }

    async fn deciders(&self) -> Vec<TestDeciders> {
        vec![TestDeciders::Counter(CounterDecider)]
    }

    async fn sagas(&self) -> Vec<TestSagas> {
        vec![TestSagas::Echo(EchoSaga)]
    }

    async fn routes(&self, router: GatewayRouter<ServerState<TestInstance>>) -> GatewayRouter<ServerState<TestInstance>> {
        router.route("/counting/health", axum::routing::get(|| async { "ok" }))
    }
}
//#endregion 🔖️Module

//#region 🔖️Decider
/// 🔢️ The actor kind [`CounterDecider`] serves.
pub const COUNTER: &str = "counter";

/// 🪞️ The actor kind [`MirrorDecider`] serves.
pub const MIRROR: &str = "mirror";

/// 🧮️ Fold [`CounterDecider`]'s little-endian counter bytes back to a `u64`, defaulting to zero for
/// an unstarted or malformed state.
pub fn read_counter(bytes: &[u8]) -> u64 {
    <[u8; 8]>::try_from(bytes).map(u64::from_le_bytes).unwrap_or(0)
}

/// 🧮️ A little-endian counter over one command kind, exercising all four [`Decision`] branches.
pub struct CounterDecider;

impl Decider for CounterDecider {
    async fn actor_kind(&self) -> &str {
        COUNTER
    }

    async fn decide(&self, state: &ActorState, command: &CommandEnvelope, _context: &DecisionContext) -> Decision {
        match command.kind.as_str() {
            "counter.increment" => Decision::Emit { events: vec![EventRecord { stream: command.target.clone(), seq: 0, hlc: HybridLogicalClock::default(), kind: "counter.incremented".into(), payload: command.payload.clone() }], effects: vec![] },
            "counter.audit" => Decision::Emit { events: vec![], effects: vec![Effect { kind: "counter.audited".into(), payload: state.bytes.clone() }] },
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

/// 🪞️ A second decider, serving its own actor kind by echoing the command payload back as an event.
/// It exists so [`TestDeciders`] is a genuinely two-variant closed set: one variant would prove only
/// that a type alias compiles, not that a downstream `dyn_enum_close!` dispatches.
pub struct MirrorDecider;

impl Decider for MirrorDecider {
    async fn actor_kind(&self) -> &str {
        MIRROR
    }

    async fn decide(&self, _state: &ActorState, command: &CommandEnvelope, _context: &DecisionContext) -> Decision {
        match command.kind.as_str() {
            "mirror.reflect" => Decision::Emit { events: vec![EventRecord { stream: command.target.clone(), seq: 0, hlc: HybridLogicalClock::default(), kind: "mirror.reflected".into(), payload: command.payload.clone() }], effects: vec![] },
            other => Decision::Reject(Rejection::UnknownCommandKind { command_kind: other.into() }),
        }
    }

    async fn evolve(&self, state: &mut ActorState, event: &EventRecord) {
        state.bytes = event.payload.clone();
    }
}

dyn_enum_close! {
    /// ⚖️ The instance's decider set, closed from a module that is not the one declaring
    /// [`Decider`] — the position every downstream instance crate is in. [`Decider`] declares
    /// `impl Future<..> + Send` returns, so the macro emits each delegate as an `async fn` over the
    /// future's `Output` and awaits inside every arm: one `match` cannot unify two distinct opaque
    /// future types in one return position, and Rust accepts an `async fn` implementation against
    /// an `impl Future` declaration precisely because the concrete future of each variant is
    /// provably `Send`.
    pub enum TestDeciders: Decider {
        Counter(CounterDecider),
        Mirror(MirrorDecider),
    }
}
//#endregion 🔖️Decider

//#region 🔖️Saga
/// 🔁️ Re-issues a `counter.increment` at the triggering event's own stream — the minimal workflow
/// the outbox-draining test runs end to end.
pub struct EchoSaga;

impl Saga for EchoSaga {
    async fn on_event(&self, event: &EventRecord) -> Vec<CommandEnvelope> {
        vec![CommandEnvelope {
            command_id: CommandId("cmd-counter.increment".into()),
            kind: "counter.increment".into(),
            version: 1,
            target: event.stream.clone(),
            scope: Scope("space-1".into()),
            principal: Principal::User { id: "alice".into() },
            session: Some(SessionId("s1".into())),
            device: Some(DeviceId("d1".into())),
            payload: vec![3],
            causal_frontier: None,
            client_hlc: HybridLogicalClock { millis: 1, counter: 0 },
            expected_revision: None,
            idempotency_key: Some(IdempotencyKey("echo".into())),
            capability_proof: None,
            trace: TraceContext::default(),
        }]
    }
}
/// 🤐️ A workflow that answers every fact with silence, so [`TestSagas`] closes a genuinely
/// two-variant set and the generated delegation really has to pick an arm.
pub struct SilentSaga;

impl Saga for SilentSaga {
    async fn on_event(&self, _event: &EventRecord) -> Vec<CommandEnvelope> {
        Vec::new()
    }
}

dyn_enum_close! {
    pub enum TestSagas: Saga {
        Echo(EchoSaga),
        Silent(SilentSaga),
    }
}
//#endregion 🔖️Saga

//#region 🔖️Resolver
/// 🪜️ A ladder rung recognizing exactly one configured bearer token.
pub struct BearerTokenResolver {
    pub name: String,
    pub bearer: String,
    pub principal: Principal,
}

impl PrincipalResolver for BearerTokenResolver {
    async fn resolve(&self, credential: &Credential) -> Option<Resolved> {
        if credential.bearer.as_deref() != Some(self.bearer.as_str()) {
            return None;
        }
        Some(Resolved {
            principal: self.principal.clone(),
            session: Some(SessionId(format!("session-{}", self.name))),
            device: Some(DeviceId("d1".to_string())),
            via: self.name.clone(),
            actor: Some(format!("actor-{}", self.name)),
        })
    }

    async fn name(&self) -> &str {
        &self.name
    }
}
//#endregion 🔖️Resolver

//#region 🔖️Authority
/// 🧠️ Reference in-memory [`AuthorityStore`]: the semantics every durable backend must match, and
/// the store a deterministic decider test runs against.
#[derive(Debug, Default)]
pub struct MemoryAuthorityStore {
    receipts: HashMap<IdempotencyKey, CommandReceipt>,
    streams: BTreeMap<ActorKey, Vec<EventRecord>>,
    snapshots: BTreeMap<ActorKey, (Revision, Vec<u8>)>,
    outbox: BTreeMap<u64, OutboxEntry>,
    next_outbox_id: u64,
    leases: BTreeMap<ActorKey, Lease>,
}

impl MemoryAuthorityStore {
    /// 🐣️ An empty authority store.
    pub fn new() -> Self {
        Self::default()
    }
}

impl AuthorityStore for MemoryAuthorityStore {
    async fn receipt(&self, key: &IdempotencyKey) -> Result<Option<CommandReceipt>, StorageError> {
        Ok(self.receipts.get(key).cloned())
    }

    async fn record_receipt(&mut self, key: &IdempotencyKey, receipt: &CommandReceipt) -> Result<(), StorageError> {
        match self.receipts.get(key) {
            Some(existing) if existing == receipt => Ok(()),
            Some(existing) => Err(StorageError::Conflict(format!("idempotency key {} is already bound to command {}", key.0, existing.command_id.0))),
            None => {
                self.receipts.insert(key.clone(), receipt.clone());
                Ok(())
            }
        }
    }

    async fn append_events(&mut self, actor: &ActorKey, events: &[EventRecord], outbox: &[OutboxEntry]) -> Result<u64, StorageError> {
        let stream = self.streams.entry(actor.clone()).or_default();
        let mut expected = stream.last().map_or(0, |record| record.seq) + 1;
        for event in events {
            if &event.stream != actor {
                return Err(StorageError::Conflict(format!("event at seq {} belongs to stream {}/{}", event.seq, event.stream.kind, event.stream.id)));
            }
            if event.seq != expected {
                return Err(StorageError::SequenceGap { expected, got: event.seq });
            }
            expected += 1;
        }
        stream.extend_from_slice(events);
        let head = stream.last().map_or(0, |record| record.seq);
        for entry in outbox {
            self.next_outbox_id += 1;
            let mut queued = entry.clone();
            queued.id = self.next_outbox_id;
            queued.delivered = false;
            self.outbox.insert(queued.id, queued);
        }
        Ok(head)
    }

    async fn events_since(&self, actor: &ActorKey, since: u64) -> Result<Vec<EventRecord>, StorageError> {
        Ok(self.streams.get(actor).into_iter().flatten().filter(|record| record.seq > since).cloned().collect())
    }

    async fn last_seq(&self, actor: &ActorKey) -> Result<u64, StorageError> {
        Ok(self.streams.get(actor).and_then(|stream| stream.last()).map_or(0, |record| record.seq))
    }

    async fn put_snapshot(&mut self, actor: &ActorKey, revision: Revision, bytes: Vec<u8>) -> Result<(), StorageError> {
        if let Some((stored, _)) = self.snapshots.get(actor) {
            if revision < *stored {
                return Err(StorageError::Conflict(format!("snapshot revision {} is older than stored {}", revision.0, stored.0)));
            }
        }
        self.snapshots.insert(actor.clone(), (revision, bytes));
        Ok(())
    }

    async fn snapshot(&self, actor: &ActorKey) -> Result<Option<(Revision, Vec<u8>)>, StorageError> {
        Ok(self.snapshots.get(actor).cloned())
    }

    async fn enqueue_outbox(&mut self, entries: Vec<OutboxEntry>) -> Result<(), StorageError> {
        for mut entry in entries {
            self.next_outbox_id += 1;
            entry.id = self.next_outbox_id;
            entry.delivered = false;
            self.outbox.insert(entry.id, entry);
        }
        Ok(())
    }

    async fn pending_outbox(&self, limit: usize) -> Result<Vec<OutboxEntry>, StorageError> {
        Ok(self.outbox.values().filter(|entry| !entry.delivered).take(limit).cloned().collect())
    }

    async fn mark_outbox_delivered(&mut self, ids: &[u64]) -> Result<(), StorageError> {
        if ids.iter().any(|id| !self.outbox.contains_key(id)) {
            return Err(StorageError::NotFound);
        }
        for id in ids {
            if let Some(entry) = self.outbox.get_mut(id) {
                entry.delivered = true;
            }
        }
        Ok(())
    }

    async fn acquire_lease(&mut self, actor: &ActorKey, holder: &str) -> Result<Lease, StorageError> {
        let lease = match self.leases.get(actor) {
            Some(current) if current.holder == holder => current.clone(),
            Some(current) => Lease { epoch: current.epoch + 1, holder: holder.to_owned() },
            None => Lease { epoch: 1, holder: holder.to_owned() },
        };
        self.leases.insert(actor.clone(), lease.clone());
        Ok(lease)
    }

    async fn validate_lease(&self, actor: &ActorKey, lease: &Lease) -> bool {
        self.leases.get(actor).is_some_and(|current| current == lease)
    }
}
//#endregion 🔖️Authority

//#region 🔖️Projection
/// 🧨️ Whether a reference store's durable sink accepts writes.
///
/// A journalling backend has a real sink to fail at — a read-only file handle, a full disk, a
/// directory that vanished. An in-memory reference has none, so it carries this instead, and the
/// conformance suite's fault laws hold it to exactly the contract a durable backend is held to:
/// a refused write reports [`StorageError::Backend`] and changes nothing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ReferenceSink {
    /// ✅️ Writes land.
    #[default]
    Working,
    /// 💥️ Every write is refused, and nothing is folded into memory.
    Failing,
}

impl ReferenceSink {
    /// 🚧️ `Err` when this sink is failing, naming the role that refused.
    fn admit(self, role: &str) -> Result<(), StorageError> {
        match self {
            Self::Working => Ok(()),
            Self::Failing => Err(StorageError::Backend(format!("reference {role} sink is failing"))),
        }
    }
}

/// 🗂️ Reference in-memory [`ProjectionStore`], ordered by key so `list` is deterministic.
#[derive(Debug, Default)]
pub struct MemoryProjectionStore {
    projections: BTreeMap<String, BTreeMap<String, Vec<u8>>>,
    checkpoints: BTreeMap<String, u64>,
    sink: ReferenceSink,
}

impl MemoryProjectionStore {
    /// 🥚️ An empty projection store.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🧨️ Fail this store's sink from here on, the way a disk stops accepting writes under a
    /// store that already holds state.
    pub fn fail(&mut self) {
        self.sink = ReferenceSink::Failing;
    }
}

impl ProjectionStore for MemoryProjectionStore {
    async fn put(&mut self, projection: &str, key: &str, value: Vec<u8>) -> Result<(), StorageError> {
        self.sink.admit("projection")?;
        self.projections.entry(projection.to_owned()).or_default().insert(key.to_owned(), value);
        Ok(())
    }

    async fn get(&self, projection: &str, key: &str) -> Option<Vec<u8>> {
        self.projections.get(projection).and_then(|entries| entries.get(key)).cloned()
    }

    async fn list(&self, projection: &str, prefix: &str) -> Vec<(String, Vec<u8>)> {
        self.projections.get(projection).into_iter().flat_map(|entries| entries.range(prefix.to_owned()..).take_while(|(key, _)| key.starts_with(prefix))).map(|(key, value)| (key.clone(), value.clone())).collect()
    }

    async fn checkpoint(&self, projection: &str) -> u64 {
        self.checkpoints.get(projection).copied().unwrap_or(0)
    }

    async fn set_checkpoint(&mut self, projection: &str, seq: u64) -> Result<(), StorageError> {
        self.sink.admit("projection")?;
        self.checkpoints.insert(projection.to_owned(), seq);
        Ok(())
    }

    async fn clear(&mut self, projection: &str) -> Result<(), StorageError> {
        self.sink.admit("projection")?;
        self.projections.remove(projection);
        self.checkpoints.remove(projection);
        Ok(())
    }
}
//#endregion 🔖️Projection

//#region 🔖️Blob
/// 🎒️ Reference in-memory [`BlobStore`], deduplicating by hash.
#[derive(Debug, Default)]
pub struct MemoryBlobStore {
    blobs: HashMap<ContentHash, Vec<u8>>,
}

impl MemoryBlobStore {
    /// 🐤️ An empty blob store.
    pub fn new() -> Self {
        Self::default()
    }
}

impl BlobStore for MemoryBlobStore {
    async fn put(&mut self, hash: ContentHash, bytes: &[u8]) -> Result<(), StorageError> {
        match self.blobs.get(&hash) {
            Some(existing) if existing.as_slice() == bytes => Ok(()),
            Some(_) => Err(StorageError::Conflict(format!("content hash {hash} already stores different bytes"))),
            None => {
                self.blobs.insert(hash, bytes.to_vec());
                Ok(())
            }
        }
    }

    async fn get(&self, hash: &ContentHash) -> Option<Vec<u8>> {
        self.blobs.get(hash).cloned()
    }

    async fn has(&self, hash: &ContentHash) -> bool {
        self.blobs.contains_key(hash)
    }
}
//#endregion 🔖️Blob

//#region 🔖️Session
/// 🗝️ Reference in-memory [`SessionStore`].
#[derive(Debug, Default)]
pub struct MemorySessionStore {
    sessions: HashMap<SessionId, SessionRecord>,
    sink: ReferenceSink,
}

impl MemorySessionStore {
    /// 🪺️ An empty session store.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🧨️ Fail this store's sink from here on, the way a disk stops accepting writes under a
    /// store that already holds sessions.
    pub fn fail(&mut self) {
        self.sink = ReferenceSink::Failing;
    }
}

impl SessionStore for MemorySessionStore {
    async fn create(&mut self, session: SessionRecord) -> Result<(), StorageError> {
        self.sink.admit("session")?;
        self.sessions.insert(session.id.clone(), session);
        Ok(())
    }

    async fn get(&self, id: &SessionId) -> Option<SessionRecord> {
        self.sessions.get(id).cloned()
    }

    async fn delete(&mut self, id: &SessionId) -> Result<(), StorageError> {
        self.sink.admit("session")?;
        self.sessions.remove(id);
        Ok(())
    }

    async fn revoke_principal(&mut self, principal: &Principal) -> Result<usize, StorageError> {
        self.sink.admit("session")?;
        let before = self.sessions.len();
        self.sessions.retain(|_, session| &session.principal != principal);
        Ok(before - self.sessions.len())
    }
}
//#endregion 🔖️Session
