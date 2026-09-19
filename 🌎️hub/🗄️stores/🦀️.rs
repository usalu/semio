//! 🗄️ Hub as **instance #1** of the generic server product: [`HubInstance`] names every port of
//! `server::gateway::ServerInstance`, and this module implements the four storage roles behind it
//! for both shapes `StorageProfile` offers.
//!
//! **Why hub owns the backends and the framework owns nothing.** The server product declares four
//! storage contracts and ships no implementation of any of them (`🖥️server/🔨️modules/🗄️storage`), so
//! the set of backends is closed here, at the deployment that owns them. That is the whole point of
//! the `ServerInstance` seam: before it, the product constructed its own in-memory stores and
//! persisted nothing *by construction*, and no downstream crate could change that without editing
//! the framework.
//!
//! **Event-sourced, never CRUD.** Nothing on disk is a mutable row. [`HubAuthorityStore`] and
//! [`HubProjectionStore`] each own one append-only journal of the facts that happened —
//! `receiptRecorded`, `eventsAppended`, `snapshotPut`, `outboxEnqueued`, `outboxDelivered`,
//! `leaseAcquired` — and their in-memory state is the fold of that journal, recomputed on open.
//! A write is journalled and fsynced *before* it is folded, so a process that dies between the two
//! replays the fact on the next open rather than losing it. Blobs are content-addressed immutable
//! files, which is not state to event-source but bytes to name. Sessions are deliberately the one
//! role that is **not** journalled: the storage contract requires that a revoked session leave no
//! replayable trace, so a session is one file that exists or does not.
//!
//! **Two profiles, one code path.** `StorageProfile::Ephemeral` gives every store a journal with no
//! sink: the same folds run, nothing is written, nothing survives. `StorageProfile::Embedded`
//! points them at one directory. There is no separate "memory backend" type to keep in sync with
//! the durable one — a second implementation of the same semantics is exactly the drift the
//! conformance suite exists to prevent.

use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

use protocol::codec::ids::ContentHash;
use server::contract::{ActorKey, CommandEnvelope, CommandReceipt, EventRecord, IdempotencyKey, ModuleManifest, Principal, Revision, SessionId};
use server::gateway::{InstanceStores, NoDocumentAuthority, NoQueryHandler, ServerInstance, ServerModule};
use server::policy::{Credential, PrincipalResolver, Resolved};
use server::storage::{AuthorityStore, BlobStore, Lease, OutboxEntry, ProjectionStore, SessionRecord, SessionStore, StorageError, StorageProfile};

//#region 🔖️Instance
/// 🪪️ The hub deployment of the server product.
///
/// Ten associated types, and every one of them is hub's own answer rather than a framework default.
/// Six are already the real thing — the four storage roles are implemented in this file and are
/// durable under `StorageProfile::Embedded`; `Queries` and `Documents` name the framework's explicit
/// "this instance hosts none" types, which is a statement, not a stub: `NoDocumentAuthority` is
/// uninhabited, so `/scopes/{scope}/document/ws` answers `notFound` by construction until hub's
/// replication engine is wired into that port.
///
/// The four remaining sets — [`HubModules`], [`HubDeciders`], [`HubSagas`], [`HubResolvers`] — are
/// uninhabited for the same reason and in the same shape: hub's 48 routes, its authorization
/// predicates and its subsystems have not moved behind the module/decider/resolver ports yet, and
/// naming an empty set says so precisely. Each becomes an enum over real variants as the subsystems
/// move, and `ServerInstance` is the only place that has to learn about it.
pub struct HubInstance;

impl ServerInstance for HubInstance {
    type Modules = HubModules;
    type Queries = NoQueryHandler;
    type Documents = NoDocumentAuthority;
    type Deciders = HubDeciders;
    type Sagas = HubSagas;
    type Resolvers = HubResolvers;
    type AuthorityStore = HubAuthorityStore;
    type ProjectionStore = HubProjectionStore;
    type BlobStore = HubBlobStore;
    type SessionStore = HubSessionStore;

    /// 🗄️ Open hub's four roles in the shape the profile asks for. `Embedded` creates the directory
    /// if it is missing and replays every journal under it, so a restart resumes at the frontier the
    /// previous process left; `Ephemeral` opens the identical stores with no sink.
    async fn open(profile: &StorageProfile) -> Result<InstanceStores<Self>, StorageError> {
        let Some(data_dir) = profile.data_dir() else {
            return Ok(InstanceStores { authority: HubAuthorityStore::ephemeral(), projections: HubProjectionStore::ephemeral(), blobs: HubBlobStore::ephemeral(), sessions: HubSessionStore::ephemeral() });
        };
        let root = PathBuf::from(data_dir);
        create_dir(&root).await?;
        Ok(InstanceStores {
            authority: HubAuthorityStore::open(&root.join("authority")).await?,
            projections: HubProjectionStore::open(&root.join("projections")).await?,
            blobs: HubBlobStore::open(&root.join("blobs")).await?,
            sessions: HubSessionStore::open(&root.join("sessions")).await?,
        })
    }
}

/// 🕳️ Hub's module set, while no subsystem has moved behind [`ServerModule`] yet. Uninhabited on
/// purpose: an instance with no modules registers none, and the day `🔐️auth` becomes the first one
/// this is the enum that gains its variant.
pub enum HubModules {}

impl ServerModule for HubModules {
    type Instance = HubInstance;

    async fn manifest(&self) -> ModuleManifest {
        match *self {}
    }
}

/// 🕳️ Hub's decider set, while its command handling still lives in `🏗️bootstrap`.
pub enum HubDeciders {}

impl server::authority::Decider for HubDeciders {
    async fn actor_kind(&self) -> &str {
        match *self {}
    }

    async fn decide(&self, _state: &server::authority::ActorState, _command: &CommandEnvelope, _context: &server::authority::DecisionContext) -> server::authority::Decision {
        match *self {}
    }

    async fn evolve(&self, _state: &mut server::authority::ActorState, _event: &EventRecord) {
        match *self {}
    }
}

/// 🕳️ Hub's workflow set, while nothing drains its outbox into further commands.
pub enum HubSagas {}

impl server::authority::Saga for HubSagas {
    async fn on_event(&self, _event: &EventRecord) -> Vec<CommandEnvelope> {
        match *self {}
    }
}

/// 🕳️ Hub's authentication ladder, while its credential handling still lives in `🏗️bootstrap`.
/// An empty ladder is not an open door: `ResolverChain` falls back to the anonymous principal, and
/// every policy decision is taken against that.
pub enum HubResolvers {}

impl PrincipalResolver for HubResolvers {
    async fn resolve(&self, _credential: &Credential) -> Option<Resolved> {
        match *self {}
    }

    async fn name(&self) -> &str {
        match *self {}
    }
}
//#endregion 🔖️Instance

//#region 🔖️Journal
/// 📜️ An append-only journal of one store's facts, one JSON record per line.
///
/// `sink` is `None` for [`StorageProfile::Ephemeral`], which is what makes the two profiles one code
/// path instead of two backends. Every [`append`](Self::append) fsyncs before returning, so the
/// caller may fold the record into memory knowing a crash cannot lose it.
struct Journal<R> {
    sink: Option<tokio::fs::File>,
    record: PhantomData<R>,
}

impl<R: Serialize + DeserializeOwned> Journal<R> {
    /// 🫧️ A journal that writes nowhere.
    fn ephemeral() -> Self {
        Self { sink: None, record: PhantomData }
    }

    /// ⏪️ Every record written to `path` so far, in order.
    ///
    /// A process killed mid-append leaves a torn final line. Replay stops at the first line that is
    /// not complete and parseable and the file is truncated back to the last good byte, which is the
    /// only recovery an append-only log needs: a record that was never fsynced is a fact that never
    /// happened.
    async fn replay(path: &Path) -> Result<Vec<R>, StorageError> {
        let bytes = match tokio::fs::read(path).await {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(backend(path, &error)),
        };
        let mut records = Vec::new();
        let mut intact = 0usize;
        for line in bytes.split_inclusive(|byte| *byte == b'\n') {
            if !line.ends_with(b"\n") {
                break;
            }
            match serde_json::from_slice::<R>(&line[..line.len() - 1]) {
                Ok(record) => {
                    records.push(record);
                    intact += line.len();
                }
                Err(_) => break,
            }
        }
        if intact < bytes.len() {
            let file = tokio::fs::OpenOptions::new().write(true).open(path).await.map_err(|error| backend(path, &error))?;
            file.set_len(intact as u64).await.map_err(|error| backend(path, &error))?;
            file.sync_all().await.map_err(|error| backend(path, &error))?;
        }
        Ok(records)
    }

    /// 🔌️ Open `path` for appending, creating it when it does not exist.
    async fn attach(path: &Path) -> Result<Self, StorageError> {
        let sink = tokio::fs::OpenOptions::new().create(true).append(true).open(path).await.map_err(|error| backend(path, &error))?;
        Ok(Self { sink: Some(sink), record: PhantomData })
    }

    /// 🧹️ Replace `path` with exactly `records` — the compaction a fold-and-overwrite store runs at
    /// open time so a rewritten key does not cost a line forever.
    async fn rewrite(path: &Path, records: &[R]) -> Result<(), StorageError> {
        let mut bytes = Vec::new();
        for record in records {
            bytes.extend_from_slice(&serde_json::to_vec(record).map_err(|error| StorageError::Backend(error.to_string()))?);
            bytes.push(b'\n');
        }
        tokio::fs::write(path, &bytes).await.map_err(|error| backend(path, &error))
    }

    /// ➕️ Append one record, flush it out of the writer's own buffer, and fsync it.
    ///
    /// The explicit [`flush`](tokio::io::AsyncWriteExt::flush) is load-bearing and was measured, not
    /// assumed: `tokio::fs::File` buffers a write and performs it on a blocking worker, so
    /// `write_all` returns `Ok` before the filesystem has seen anything, and `sync_data` completes
    /// the in-flight write **discarding its error** before fsyncing. A sink that refuses every byte
    /// therefore reported success through both calls. `flush` is the one call that surfaces the
    /// deferred write error, so it is what makes this `Result` mean anything at all.
    async fn append(&mut self, record: &R) -> Result<(), StorageError> {
        let Some(sink) = self.sink.as_mut() else {
            return Ok(());
        };
        let mut line = serde_json::to_vec(record).map_err(|error| StorageError::Backend(error.to_string()))?;
        line.push(b'\n');
        sink.write_all(&line).await.map_err(|error| StorageError::Backend(error.to_string()))?;
        sink.flush().await.map_err(|error| StorageError::Backend(error.to_string()))?;
        sink.sync_data().await.map_err(|error| StorageError::Backend(error.to_string()))
    }

    /// 💾️ Whether this journal has a sink at all.
    fn is_durable(&self) -> bool {
        self.sink.is_some()
    }
}

/// 💥️ One filesystem failure, named with the path it happened on.
fn backend(path: &Path, error: &std::io::Error) -> StorageError {
    StorageError::Backend(format!("{}: {error}", path.display()))
}

/// 📁️ Create `path` and every missing parent.
async fn create_dir(path: &Path) -> Result<(), StorageError> {
    tokio::fs::create_dir_all(path).await.map_err(|error| backend(path, &error))
}

/// 🔡️ Lowercase hex of `bytes` — how an opaque identity becomes a filename no platform argues with.
fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(DIGITS[usize::from(byte >> 4)] as char);
        encoded.push(DIGITS[usize::from(byte & 0x0f)] as char);
    }
    encoded
}
//#endregion 🔖️Journal

//#region 🔖️Authority
/// 📓️ One fact of the authority log. The journal is the authority; the maps below are its fold.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
enum AuthorityRecord {
    /// 🧾️ One idempotency key was bound to one receipt.
    ReceiptRecorded { key: IdempotencyKey, receipt: CommandReceipt },
    /// ➕️ One actor's stream grew, and the outbox rows that grew with it in the same write.
    EventsAppended { actor: ActorKey, events: Vec<EventRecord>, outbox: Vec<OutboxEntry> },
    /// 📸️ One actor's replay accelerator moved forward.
    SnapshotPut { actor: ActorKey, revision: Revision, bytes: Vec<u8> },
    /// 📤️ Rows were queued for publication outside an append.
    OutboxEnqueued { entries: Vec<OutboxEntry> },
    /// 📬️ A publisher acknowledged those queue positions.
    OutboxDelivered { ids: Vec<u64> },
    /// 🤝️ Ownership of one actor moved or was renewed.
    LeaseAcquired { actor: ActorKey, lease: Lease },
}

/// 🏛️ Hub's authoritative history: command inbox, per-actor event streams, snapshots, transactional
/// outbox and actor leases, folded from one append-only journal.
pub struct HubAuthorityStore {
    receipts: HashMap<IdempotencyKey, CommandReceipt>,
    streams: BTreeMap<ActorKey, Vec<EventRecord>>,
    snapshots: BTreeMap<ActorKey, (Revision, Vec<u8>)>,
    outbox: BTreeMap<u64, OutboxEntry>,
    next_outbox_id: u64,
    leases: BTreeMap<ActorKey, Lease>,
    journal: Journal<AuthorityRecord>,
}

impl HubAuthorityStore {
    /// 🫧️ An authority store that keeps its history only for this process.
    pub fn ephemeral() -> Self {
        Self::folded(Journal::ephemeral(), Vec::new())
    }

    /// 💾️ Open the durable authority under `dir`, replaying every fact ever journalled there.
    pub async fn open(dir: &Path) -> Result<Self, StorageError> {
        create_dir(dir).await?;
        let path = dir.join("log.jsonl");
        let records = Journal::replay(&path).await?;
        Ok(Self::folded(Journal::attach(&path).await?, records))
    }

    /// 🌍️ Whether this store's history outlives the process.
    pub fn is_durable(&self) -> bool {
        self.journal.is_durable()
    }

    /// ⏪️ Replay `records` into an empty store.
    fn folded(journal: Journal<AuthorityRecord>, records: Vec<AuthorityRecord>) -> Self {
        let mut store = Self { receipts: HashMap::new(), streams: BTreeMap::new(), snapshots: BTreeMap::new(), outbox: BTreeMap::new(), next_outbox_id: 0, leases: BTreeMap::new(), journal };
        for record in records {
            store.fold(record);
        }
        store
    }

    /// 🍰️ Apply one journalled fact to the in-memory state. Never validates — validation happened
    /// before the fact was written, and a replay must reproduce history, not re-judge it.
    fn fold(&mut self, record: AuthorityRecord) {
        match record {
            AuthorityRecord::ReceiptRecorded { key, receipt } => {
                self.receipts.insert(key, receipt);
            }
            AuthorityRecord::EventsAppended { actor, events, outbox } => {
                self.streams.entry(actor).or_default().extend(events);
                self.queue(outbox);
            }
            AuthorityRecord::SnapshotPut { actor, revision, bytes } => {
                self.snapshots.insert(actor, (revision, bytes));
            }
            AuthorityRecord::OutboxEnqueued { entries } => self.queue(entries),
            AuthorityRecord::OutboxDelivered { ids } => {
                for id in ids {
                    if let Some(entry) = self.outbox.get_mut(&id) {
                        entry.delivered = true;
                    }
                }
            }
            AuthorityRecord::LeaseAcquired { actor, lease } => {
                self.leases.insert(actor, lease);
            }
        }
    }

    /// 📥️ Insert already-stamped outbox rows and carry the id cursor past them.
    fn queue(&mut self, entries: Vec<OutboxEntry>) {
        for entry in entries {
            self.next_outbox_id = self.next_outbox_id.max(entry.id);
            self.outbox.insert(entry.id, entry);
        }
    }

    /// 🔢️ Assign the next queue ids to `entries`, so the journalled record and the replayed one
    /// carry the identical positions.
    fn stamped(&self, entries: &[OutboxEntry]) -> Vec<OutboxEntry> {
        let mut id = self.next_outbox_id;
        entries
            .iter()
            .map(|entry| {
                id += 1;
                OutboxEntry { id, delivered: false, ..entry.clone() }
            })
            .collect()
    }

    /// 🔚️ The highest sequence written for `actor`.
    fn head(&self, actor: &ActorKey) -> u64 {
        self.streams.get(actor).and_then(|stream| stream.last()).map_or(0, |record| record.seq)
    }

    /// 💾️ Journal then fold — never the other way round, so a crash between the two loses nothing.
    async fn commit(&mut self, record: AuthorityRecord) -> Result<(), StorageError> {
        self.journal.append(&record).await?;
        self.fold(record);
        Ok(())
    }
}

impl AuthorityStore for HubAuthorityStore {
    async fn receipt(&self, key: &IdempotencyKey) -> Result<Option<CommandReceipt>, StorageError> {
        Ok(self.receipts.get(key).cloned())
    }

    async fn record_receipt(&mut self, key: &IdempotencyKey, receipt: &CommandReceipt) -> Result<(), StorageError> {
        match self.receipts.get(key) {
            Some(existing) if existing == receipt => return Ok(()),
            Some(existing) => return Err(StorageError::Conflict(format!("idempotency key {} is already bound to command {}", key.0, existing.command_id.0))),
            None => {}
        }
        self.commit(AuthorityRecord::ReceiptRecorded { key: key.clone(), receipt: receipt.clone() }).await
    }

    async fn append_events(&mut self, actor: &ActorKey, events: &[EventRecord], outbox: &[OutboxEntry]) -> Result<u64, StorageError> {
        let mut expected = self.head(actor) + 1;
        for event in events {
            if &event.stream != actor {
                return Err(StorageError::Conflict(format!("event at seq {} belongs to stream {}/{}", event.seq, event.stream.kind, event.stream.id)));
            }
            if event.seq != expected {
                return Err(StorageError::SequenceGap { expected, got: event.seq });
            }
            expected += 1;
        }
        let queued = self.stamped(outbox);
        self.commit(AuthorityRecord::EventsAppended { actor: actor.clone(), events: events.to_vec(), outbox: queued }).await?;
        Ok(self.head(actor))
    }

    async fn events_since(&self, actor: &ActorKey, since: u64) -> Result<Vec<EventRecord>, StorageError> {
        Ok(self.streams.get(actor).into_iter().flatten().filter(|record| record.seq > since).cloned().collect())
    }

    async fn last_seq(&self, actor: &ActorKey) -> Result<u64, StorageError> {
        Ok(self.head(actor))
    }

    async fn put_snapshot(&mut self, actor: &ActorKey, revision: Revision, bytes: Vec<u8>) -> Result<(), StorageError> {
        if let Some((stored, _)) = self.snapshots.get(actor) {
            if revision < *stored {
                return Err(StorageError::Conflict(format!("snapshot revision {} is older than stored {}", revision.0, stored.0)));
            }
        }
        self.commit(AuthorityRecord::SnapshotPut { actor: actor.clone(), revision, bytes }).await
    }

    async fn snapshot(&self, actor: &ActorKey) -> Result<Option<(Revision, Vec<u8>)>, StorageError> {
        Ok(self.snapshots.get(actor).cloned())
    }

    async fn enqueue_outbox(&mut self, entries: Vec<OutboxEntry>) -> Result<(), StorageError> {
        let queued = self.stamped(&entries);
        self.commit(AuthorityRecord::OutboxEnqueued { entries: queued }).await
    }

    async fn pending_outbox(&self, limit: usize) -> Result<Vec<OutboxEntry>, StorageError> {
        Ok(self.outbox.values().filter(|entry| !entry.delivered).take(limit).cloned().collect())
    }

    async fn mark_outbox_delivered(&mut self, ids: &[u64]) -> Result<(), StorageError> {
        if ids.iter().any(|id| !self.outbox.contains_key(id)) {
            return Err(StorageError::NotFound);
        }
        self.commit(AuthorityRecord::OutboxDelivered { ids: ids.to_vec() }).await
    }

    async fn acquire_lease(&mut self, actor: &ActorKey, holder: &str) -> Result<Lease, StorageError> {
        let lease = match self.leases.get(actor) {
            Some(current) if current.holder == holder => current.clone(),
            Some(current) => Lease { epoch: current.epoch + 1, holder: holder.to_owned() },
            None => Lease { epoch: 1, holder: holder.to_owned() },
        };
        self.commit(AuthorityRecord::LeaseAcquired { actor: actor.clone(), lease: lease.clone() }).await?;
        Ok(lease)
    }

    async fn validate_lease(&self, actor: &ActorKey, lease: &Lease) -> bool {
        self.leases.get(actor).is_some_and(|current| current == lease)
    }
}
//#endregion 🔖️Authority

//#region 🔖️Projection
/// 📓️ One fact of a read model's journal.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
enum ProjectionRecord {
    /// ✍️ One read-model entry took a value.
    Put { projection: String, key: String, value: Vec<u8> },
    /// 🏁️ One read model reached a sequence.
    Checkpointed { projection: String, seq: u64 },
    /// 🧹️ One read model was dropped for rebuild.
    Cleared { projection: String },
}

/// 🔭️ Hub's read models. Persisted so a restart resumes folding at its checkpoint instead of
/// replaying every stream from zero — never as a source of truth: [`ProjectionStore::clear`] plus a
/// replay must reproduce every byte, which is why the journal is compacted to the current state at
/// open time rather than kept as history.
pub struct HubProjectionStore {
    projections: BTreeMap<String, BTreeMap<String, Vec<u8>>>,
    checkpoints: BTreeMap<String, u64>,
    journal: Journal<ProjectionRecord>,
}

impl HubProjectionStore {
    /// 🫧️ Read models that live only for this process.
    pub fn ephemeral() -> Self {
        Self::folded(Journal::ephemeral(), Vec::new())
    }

    /// 💾️ Open the durable read models under `dir` and compact their journal to what they now hold.
    pub async fn open(dir: &Path) -> Result<Self, StorageError> {
        create_dir(dir).await?;
        let path = dir.join("log.jsonl");
        let mut store = Self::folded(Journal::ephemeral(), Journal::replay(&path).await?);
        Journal::rewrite(&path, &store.compacted()).await?;
        store.journal = Journal::attach(&path).await?;
        Ok(store)
    }

    /// 🌍️ Whether these read models outlive the process.
    pub fn is_durable(&self) -> bool {
        self.journal.is_durable()
    }

    /// ⏪️ Replay `records` into empty read models.
    fn folded(journal: Journal<ProjectionRecord>, records: Vec<ProjectionRecord>) -> Self {
        let mut store = Self { projections: BTreeMap::new(), checkpoints: BTreeMap::new(), journal };
        for record in records {
            store.fold(record);
        }
        store
    }

    /// 🍰️ Apply one journalled fact to the in-memory read models.
    fn fold(&mut self, record: ProjectionRecord) {
        match record {
            ProjectionRecord::Put { projection, key, value } => {
                self.projections.entry(projection).or_default().insert(key, value);
            }
            ProjectionRecord::Checkpointed { projection, seq } => {
                self.checkpoints.insert(projection, seq);
            }
            ProjectionRecord::Cleared { projection } => {
                self.projections.remove(&projection);
                self.checkpoints.remove(&projection);
            }
        }
    }

    /// 🧹️ The shortest journal that folds to exactly what is held now.
    fn compacted(&self) -> Vec<ProjectionRecord> {
        let mut records: Vec<ProjectionRecord> = self.projections.iter().flat_map(|(projection, entries)| entries.iter().map(|(key, value)| ProjectionRecord::Put { projection: projection.clone(), key: key.clone(), value: value.clone() })).collect();
        records.extend(self.checkpoints.iter().map(|(projection, seq)| ProjectionRecord::Checkpointed { projection: projection.clone(), seq: *seq }));
        records
    }

    /// 💾️ Journal then fold, and never the other way round: a record the sink refused is a fact
    /// that did not happen, so it is not folded and the caller is told. This is what the
    /// `Result`-returning write contract bought — the previous shape counted the fault, dropped the
    /// sink and folded anyway, which left these read models answering queries from state the next
    /// open would not reproduce.
    async fn commit(&mut self, record: ProjectionRecord) -> Result<(), StorageError> {
        self.journal.append(&record).await?;
        self.fold(record);
        Ok(())
    }
}

impl ProjectionStore for HubProjectionStore {
    async fn put(&mut self, projection: &str, key: &str, value: Vec<u8>) -> Result<(), StorageError> {
        self.commit(ProjectionRecord::Put { projection: projection.to_owned(), key: key.to_owned(), value }).await
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
        self.commit(ProjectionRecord::Checkpointed { projection: projection.to_owned(), seq }).await
    }

    async fn clear(&mut self, projection: &str) -> Result<(), StorageError> {
        self.commit(ProjectionRecord::Cleared { projection: projection.to_owned() }).await
    }
}
//#endregion 🔖️Projection

//#region 🔖️Blob
/// 🧱️ Hub's content-addressed bytes: one immutable file per hash, named by the hash itself, so the
/// store needs no index and a restart discovers everything it holds by looking.
pub struct HubBlobStore {
    memory: HashMap<ContentHash, Vec<u8>>,
    dir: Option<PathBuf>,
}

impl HubBlobStore {
    /// 🫧️ Bytes that live only for this process.
    pub fn ephemeral() -> Self {
        Self { memory: HashMap::new(), dir: None }
    }

    /// 💾️ Open the durable content store under `dir`.
    pub async fn open(dir: &Path) -> Result<Self, StorageError> {
        create_dir(dir).await?;
        Ok(Self { memory: HashMap::new(), dir: Some(dir.to_path_buf()) })
    }

    /// 🌍️ Whether these bytes outlive the process.
    pub fn is_durable(&self) -> bool {
        self.dir.is_some()
    }

    /// 📁️ Where `hash` lives, when this store has a directory at all.
    fn path(&self, hash: &ContentHash) -> Option<PathBuf> {
        self.dir.as_ref().map(|dir| dir.join(hash.to_string()))
    }
}

impl BlobStore for HubBlobStore {
    async fn put(&mut self, hash: ContentHash, bytes: &[u8]) -> Result<(), StorageError> {
        let Some(path) = self.path(&hash) else {
            return match self.memory.get(&hash) {
                Some(existing) if existing.as_slice() == bytes => Ok(()),
                Some(_) => Err(StorageError::Conflict(format!("content hash {hash} already stores different bytes"))),
                None => {
                    self.memory.insert(hash, bytes.to_vec());
                    Ok(())
                }
            };
        };
        match tokio::fs::read(&path).await {
            Ok(existing) if existing == bytes => Ok(()),
            Ok(_) => Err(StorageError::Conflict(format!("content hash {hash} already stores different bytes"))),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => tokio::fs::write(&path, bytes).await.map_err(|error| backend(&path, &error)),
            Err(error) => Err(backend(&path, &error)),
        }
    }

    async fn get(&self, hash: &ContentHash) -> Option<Vec<u8>> {
        match self.path(hash) {
            Some(path) => tokio::fs::read(&path).await.ok(),
            None => self.memory.get(hash).cloned(),
        }
    }

    async fn has(&self, hash: &ContentHash) -> bool {
        match self.path(hash) {
            Some(path) => tokio::fs::metadata(&path).await.is_ok(),
            None => self.memory.contains_key(hash),
        }
    }
}
//#endregion 🔖️Blob

//#region 🔖️Session
/// 🎫️ Hub's live authentication state: one file per session, removed on revocation.
///
/// The only role here that is **not** event-sourced, and deliberately so — the storage contract
/// requires that a revoked session leave no replayable trace, which an append-only journal cannot
/// promise. Session files are the exception that proves the rule: they are not history, they are the
/// set of keys that currently open the door.
pub struct HubSessionStore {
    sessions: HashMap<SessionId, SessionRecord>,
    dir: Option<PathBuf>,
}

impl HubSessionStore {
    /// 🫧️ Sessions that die with the process.
    pub fn ephemeral() -> Self {
        Self { sessions: HashMap::new(), dir: None }
    }

    /// 💾️ Open the durable session set under `dir`, reading back every session still live.
    pub async fn open(dir: &Path) -> Result<Self, StorageError> {
        create_dir(dir).await?;
        let mut sessions = HashMap::new();
        let mut entries = tokio::fs::read_dir(dir).await.map_err(|error| backend(dir, &error))?;
        while let Some(entry) = entries.next_entry().await.map_err(|error| backend(dir, &error))? {
            let path = entry.path();
            let Ok(bytes) = tokio::fs::read(&path).await else {
                continue;
            };
            if let Ok(record) = serde_json::from_slice::<SessionRecord>(&bytes) {
                sessions.insert(record.id.clone(), record);
            }
        }
        Ok(Self { sessions, dir: Some(dir.to_path_buf()) })
    }

    /// 🌍️ Whether these sessions outlive the process.
    pub fn is_durable(&self) -> bool {
        self.dir.is_some()
    }

    /// 📁️ Where `id`'s record lives, when this store has a directory at all.
    fn path(&self, id: &SessionId) -> Option<PathBuf> {
        self.dir.as_ref().map(|dir| dir.join(format!("{}.json", hex(id.0.as_bytes()))))
    }

    /// 🗑️ Remove one session from memory and from disk, leaving no trace to replay. Disk first: a
    /// record this process forgot but could not delete is a key that still opens the door on the
    /// next open, so the caller learns about it instead of the memory silently diverging.
    async fn forget(&mut self, id: &SessionId) -> Result<(), StorageError> {
        if let Some(path) = self.path(id) {
            if let Err(error) = tokio::fs::remove_file(&path).await {
                if error.kind() != std::io::ErrorKind::NotFound {
                    return Err(backend(&path, &error));
                }
            }
        }
        self.sessions.remove(id);
        Ok(())
    }
}

impl SessionStore for HubSessionStore {
    async fn create(&mut self, session: SessionRecord) -> Result<(), StorageError> {
        if let Some(path) = self.path(&session.id) {
            let bytes = serde_json::to_vec(&session).map_err(|error| StorageError::Backend(error.to_string()))?;
            tokio::fs::write(&path, &bytes).await.map_err(|error| backend(&path, &error))?;
        }
        self.sessions.insert(session.id.clone(), session);
        Ok(())
    }

    async fn get(&self, id: &SessionId) -> Option<SessionRecord> {
        self.sessions.get(id).cloned()
    }

    async fn delete(&mut self, id: &SessionId) -> Result<(), StorageError> {
        self.forget(id).await
    }

    async fn revoke_principal(&mut self, principal: &Principal) -> Result<usize, StorageError> {
        let doomed: Vec<SessionId> = self.sessions.values().filter(|session| &session.principal == principal).map(|session| session.id.clone()).collect();
        for id in &doomed {
            self.forget(id).await?;
        }
        Ok(doomed.len())
    }
}
//#endregion 🔖️Session

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
