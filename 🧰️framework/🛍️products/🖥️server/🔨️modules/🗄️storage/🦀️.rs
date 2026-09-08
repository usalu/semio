//! 🗄️ Server storage roles — the four durability contracts an authority needs, kept as four
//! separate traits instead of one `Database` interface.
//!
//! **Why four.** Each role has a different truth, a different lifetime and a different recovery
//! story, so collapsing them into one trait would force every backend to be good at all four:
//!
//! - [`AuthorityStore`] holds the only authoritative state: the command inbox (receipts keyed by
//!   [`IdempotencyKey`], so a retry after a timeout produces one effect and one answer), the
//!   per-actor event streams that *are* the state, the snapshots that merely accelerate replay,
//!   the transactional outbox that makes "state changed" and "the world was told" one atomic fact,
//!   and the actor leases that fence a stale owner out after a failover. Everything here is
//!   append-only and irreplaceable — losing it loses history.
//! - [`ProjectionStore`] holds read models. Every byte in it is *derived* and can be dropped and
//!   rebuilt from the event streams, which is exactly why it must not share a transaction or a
//!   durability budget with the authority: a projection may lag, be wiped, or be rebuilt on a
//!   different schema while the authority keeps accepting commands.
//! - [`BlobStore`] holds content-addressed bytes. Immutable, deduplicated by [`ContentHash`] and
//!   never rewritten, so it wants object storage semantics, not transactional ones.
//! - [`SessionStore`] holds authentication state. Revocation must be immediate and cheap, and the
//!   data is intentionally *not* event-sourced — a revoked session must leave no replayable trace.
//!
//! **No clock, no driver.** Every timestamp is passed in by the caller, so a decider and its store
//! can be replayed deterministically in a test even though every method is `async` (O1: the literal
//! `async` keyword is universal here regardless of whether a given backend actually suspends — the
//! in-memory reference backends below never do, a real disk/network backend will). Backends
//! (embedded file storage in Wave 2, a server-grade engine later) implement these traits behind
//! [`StorageProfile`]; the in-memory implementations here are the reference semantics every
//! backend must reproduce. Each trait is `#[dyn_enum]`'d and closed over its one reference
//! implementation (`AuthorityStores`/`ProjectionStores`/`BlobStores`/`SessionStores`) rather than
//! boxed as `dyn` (O1 — drop dyn dispatch); a real second backend adds a variant here, not a `Box`.

use crate::contract::{ActorKey, CommandReceipt, DeviceId, EventRecord, IdempotencyKey, Principal, Revision, SessionId};
use protocol::codec::ids::ContentHash;
use protocol::crypto::RecordHasher;
use protocol::format::Blake3Hasher;
use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

//#region 🔖️Profile
/// @emoji 🏗️ Which deployment shape the storage backends are opened in. Wave 2 ships exactly one
/// profile on purpose: a single-process authority owning a local data directory. Clustered and
/// hosted profiles are added as further variants when a second backend actually exists, never as a
/// speculative option flag on this one.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum StorageProfile {
    /// @emoji 🏠️ Everything lives under one directory owned by one process: the Wave-2 deployment
    /// profile for hub, zentrale and a developer's laptop alike.
    Embedded { data_dir: String },
}
//#endregion 🔖️Profile

//#region 🔖️Error
/// @emoji 💥️ Every way a storage role can refuse. Deliberately small: a backend translates its own
/// driver errors into these, so an authority never matches on a driver type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StorageError {
    /// @emoji 🕳️ The addressed entry does not exist.
    NotFound,
    /// @emoji 🪜️ An append was not contiguous with the stream's current head — the writer is
    /// working from a stale `last_seq` and must re-read before retrying.
    SequenceGap { expected: u64, got: u64 },
    /// @emoji 🎟️ The caller's [`Lease`] was fenced out by a newer epoch; its writes must be
    /// abandoned, not retried.
    LeaseLost,
    /// @emoji ⚔️ The write contradicts what is already stored (a re-bound idempotency key, a
    /// backwards snapshot, a hash bound to different bytes).
    Conflict(String),
    /// @emoji 🔌️ The backend itself failed — disk, permissions, corruption.
    Backend(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => formatter.write_str("storage entry not found"),
            Self::SequenceGap { expected, got } => write!(formatter, "sequence gap: expected {expected}, got {got}"),
            Self::LeaseLost => formatter.write_str("actor lease lost"),
            Self::Conflict(detail) => write!(formatter, "storage conflict: {detail}"),
            Self::Backend(detail) => write!(formatter, "storage backend failure: {detail}"),
        }
    }
}

impl std::error::Error for StorageError {}
//#endregion 🔖️Error

//#region 🔖️Authority
/// @emoji 🔐️ Ownership of one actor, fenced by a monotonically increasing epoch. A holder that
/// loses the lease keeps its old epoch, which is how the store recognizes and rejects it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lease {
    /// @emoji 🔢️ Bumped every time ownership moves to a different holder; never reused.
    pub epoch: u64,
    /// @emoji 🙋️ Opaque identity of the node or worker holding the actor.
    pub holder: String,
}

/// @emoji 📮️ One event queued for publication in the same write as the event itself, so a crash can
/// never leave state advanced but the world uninformed. Delivery is at-least-once on the wire and
/// exactly-once against this queue: an entry leaves `pending` only once it is marked delivered.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutboxEntry {
    /// @emoji 🆔️ Store-assigned, monotonically increasing queue position; also the delivery order.
    pub id: u64,
    /// @emoji 🎭️ The actor whose turn produced the event.
    pub actor: ActorKey,
    /// @emoji 🏷️ Routing tag — the event's kind for a saga row, the effect's kind for an effect row.
    pub kind: String,
    /// @emoji 📦️ The opaque body a publisher hands on.
    pub payload: Vec<u8>,
    /// @emoji 📚️ The durable fact, present for event-derived rows and absent for pure effects.
    pub event: Option<EventRecord>,
    /// @emoji ✅️ Whether a publisher has acknowledged this entry.
    pub delivered: bool,
}

impl OutboxEntry {
    /// @emoji 🌱️ An undelivered entry with a placeholder id — [`AuthorityStore::enqueue_outbox`]
    /// stamps the real one.
    pub fn pending(actor: ActorKey, event: EventRecord) -> Self {
        Self { id: 0, actor, kind: event.kind.clone(), payload: event.payload.clone(), event: Some(event), delivered: false }
    }
}

/// @emoji 🏛️ The authoritative state of a server: command inbox, per-actor event streams, snapshots,
/// transactional outbox and actor leases. The one role whose data cannot be regenerated.
#[dyn_enum]
pub trait AuthorityStore: Send + Sync {
    /// @emoji 🔎️ The receipt already recorded for `key`, if this command was seen before. A retry
    /// answers from here instead of re-executing.
    async fn receipt(&self, key: &IdempotencyKey) -> Result<Option<CommandReceipt>, StorageError>;

    /// @emoji 🧾️ Binds `key` to `receipt`. Recording the identical receipt again succeeds silently;
    /// binding a key to a *different* receipt is a [`StorageError::Conflict`].
    async fn record_receipt(&mut self, key: &IdempotencyKey, receipt: &CommandReceipt) -> Result<(), StorageError>;

    /// @emoji ➕️ Appends `events` to `actor`'s stream and returns the new last sequence. Every event
    /// must carry `actor` as its stream and a sequence exactly one past its predecessor, starting at
    /// `last_seq + 1`; anything else is a [`StorageError::SequenceGap`] and nothing is written.
    async fn append_events(&mut self, actor: &ActorKey, events: &[EventRecord], outbox: &[OutboxEntry]) -> Result<u64, StorageError>;

    /// @emoji 📜️ Every event of `actor` with a sequence strictly greater than `since`, in order.
    async fn events_since(&self, actor: &ActorKey, since: u64) -> Result<Vec<EventRecord>, StorageError>;

    /// @emoji 🔚️ The highest sequence written for `actor`; `0` for an actor with no history.
    async fn last_seq(&self, actor: &ActorKey) -> Result<u64, StorageError>;

    /// @emoji 📸️ Replaces `actor`'s replay accelerator. A snapshot older than the stored one is a
    /// [`StorageError::Conflict`] — snapshots only ever move forward.
    async fn put_snapshot(&mut self, actor: &ActorKey, revision: Revision, bytes: Vec<u8>) -> Result<(), StorageError>;

    /// @emoji 🖼️ The stored snapshot of `actor` and the revision it was taken at, if any.
    async fn snapshot(&self, actor: &ActorKey) -> Result<Option<(Revision, Vec<u8>)>, StorageError>;

    /// @emoji 📤️ Queues `entries` for publication, stamping each with the next queue id and marking
    /// it undelivered; the caller's `id` and `delivered` fields are ignored.
    async fn enqueue_outbox(&mut self, entries: Vec<OutboxEntry>) -> Result<(), StorageError>;

    /// @emoji 📥️ Up to `limit` undelivered entries in queue order.
    async fn pending_outbox(&self, limit: usize) -> Result<Vec<OutboxEntry>, StorageError>;

    /// @emoji 📬️ Acknowledges delivery of `ids`. Re-acknowledging is idempotent; an unknown id is a
    /// [`StorageError::NotFound`] and nothing is marked.
    async fn mark_outbox_delivered(&mut self, ids: &[u64]) -> Result<(), StorageError>;

    /// @emoji 🤝️ Takes ownership of `actor` for `holder`. Re-acquiring as the current holder renews
    /// at the same epoch; taking it from a different holder bumps the epoch, which fences the
    /// previous holder out for good.
    async fn acquire_lease(&mut self, actor: &ActorKey, holder: &str) -> Result<Lease, StorageError>;

    /// @emoji 🛡️ Whether `lease` is still the live lease on `actor`. A stale epoch answers `false`,
    /// and the caller must abandon its turn with [`StorageError::LeaseLost`].
    async fn validate_lease(&self, actor: &ActorKey, lease: &Lease) -> bool;
}

/// @emoji 🧠️ Reference in-memory [`AuthorityStore`]: the semantics every durable backend must match,
/// and the store a deterministic decider test runs against.
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
    /// @emoji 🐣️ An empty authority store.
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

dyn_enum_close! {
    pub enum AuthorityStores: AuthorityStore {
        Memory(MemoryAuthorityStore),
    }
}
//#endregion 🔖️Authority

//#region 🔖️Projection
/// @emoji 🔭️ Rebuildable read models, addressed by projection name and key. Nothing here is a source
/// of truth: [`ProjectionStore::clear`] plus a replay from sequence zero must reproduce it exactly,
/// which is what makes a schema change a rebuild rather than a migration.
#[dyn_enum]
pub trait ProjectionStore: Send + Sync {
    /// @emoji ✍️ Writes `value` at `key` inside `projection`, replacing any previous value.
    async fn put(&mut self, projection: &str, key: &str, value: Vec<u8>);

    /// @emoji 📖️ The value stored at `key`, if the projection has one.
    async fn get(&self, projection: &str, key: &str) -> Option<Vec<u8>>;

    /// @emoji 📋️ Every entry of `projection` whose key starts with `prefix`, ascending by key —
    /// ordering is part of the contract so a paged query is stable across backends.
    async fn list(&self, projection: &str, prefix: &str) -> Vec<(String, Vec<u8>)>;

    /// @emoji 🚩️ The last event sequence folded into `projection`; `0` when it has never been built.
    async fn checkpoint(&self, projection: &str) -> u64;

    /// @emoji 🏁️ Records that `projection` now reflects everything up to `seq`.
    async fn set_checkpoint(&mut self, projection: &str, seq: u64);

    /// @emoji 🧹️ Drops every entry of `projection` and resets its checkpoint to zero, so the next
    /// fold rebuilds it from the beginning.
    async fn clear(&mut self, projection: &str);
}

/// @emoji 🗂️ Reference in-memory [`ProjectionStore`], ordered by key so `list` is deterministic.
#[derive(Debug, Default)]
pub struct MemoryProjectionStore {
    projections: BTreeMap<String, BTreeMap<String, Vec<u8>>>,
    checkpoints: BTreeMap<String, u64>,
}

impl MemoryProjectionStore {
    /// @emoji 🥚️ An empty projection store.
    pub fn new() -> Self {
        Self::default()
    }
}

impl ProjectionStore for MemoryProjectionStore {
    async fn put(&mut self, projection: &str, key: &str, value: Vec<u8>) {
        self.projections.entry(projection.to_owned()).or_default().insert(key.to_owned(), value);
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

    async fn set_checkpoint(&mut self, projection: &str, seq: u64) {
        self.checkpoints.insert(projection.to_owned(), seq);
    }

    async fn clear(&mut self, projection: &str) {
        self.projections.remove(projection);
        self.checkpoints.remove(projection);
    }
}

dyn_enum_close! {
    pub enum ProjectionStores: ProjectionStore {
        Memory(MemoryProjectionStore),
    }
}
//#endregion 🔖️Projection

//#region 🔖️Blob
/// @emoji #️⃣ The canonical content hash of `bytes`, computed with the same `blake3` primitive the
/// replication format commits with — offered so a caller can address a blob without picking its own
/// hash function, and so this crate needs no hashing dependency of its own.
pub fn content_hash(bytes: &[u8]) -> ContentHash {
    ContentHash(Blake3Hasher.hash(bytes))
}

/// @emoji 🧱️ Immutable, content-addressed bytes. The caller supplies the hash (see [`content_hash`])
/// because the address is minted where the content is produced — an upload is verified once, at the
/// edge, and every later reference is by hash alone. Identical content is stored once.
#[dyn_enum]
pub trait BlobStore: Send + Sync {
    /// @emoji 💾️ Stores `bytes` under `hash`. Storing identical content again succeeds silently;
    /// binding a hash to different bytes is a [`StorageError::Conflict`].
    async fn put(&mut self, hash: ContentHash, bytes: &[u8]) -> Result<(), StorageError>;

    /// @emoji 📦️ The bytes stored under `hash`, if any.
    async fn get(&self, hash: &ContentHash) -> Option<Vec<u8>>;

    /// @emoji ❓️ Whether `hash` is already stored — the cheap half of an upload negotiation.
    async fn has(&self, hash: &ContentHash) -> bool;
}

/// @emoji 🎒️ Reference in-memory [`BlobStore`], deduplicating by hash.
#[derive(Debug, Default)]
pub struct MemoryBlobStore {
    blobs: HashMap<ContentHash, Vec<u8>>,
}

impl MemoryBlobStore {
    /// @emoji 🐤️ An empty blob store.
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

dyn_enum_close! {
    pub enum BlobStores: BlobStore {
        Memory(MemoryBlobStore),
    }
}
//#endregion 🔖️Blob

//#region 🔖️Session
/// @emoji 🪪️ One live authenticated session. Timestamps are supplied by the caller — this crate owns
/// no clock, so expiry is decided by whoever reads the record, never inside the store.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionRecord {
    /// @emoji 🏷️ The session's identity, as presented by the client on every request.
    pub id: SessionId,
    /// @emoji 👤️ Who the session authenticates as.
    pub principal: Principal,
    /// @emoji 📱️ The device the session was issued to, when the client identified one.
    pub device: Option<DeviceId>,
    /// @emoji ⏱️ Wall-clock issue time in milliseconds, recorded by the caller.
    pub issued_at_millis: u64,
}

/// @emoji 🎫️ Live authentication state. Deliberately not event-sourced: a revoked session must
/// vanish rather than survive as a replayable fact, and revocation must be immediate.
#[dyn_enum]
pub trait SessionStore: Send + Sync {
    /// @emoji 🆕️ Stores `session`, replacing any record with the same id.
    async fn create(&mut self, session: SessionRecord);

    /// @emoji 🔑️ The session with `id`, if it is still live.
    async fn get(&self, id: &SessionId) -> Option<SessionRecord>;

    /// @emoji 🗑️ Removes `id`; a no-op if it is already gone.
    async fn delete(&mut self, id: &SessionId);

    /// @emoji 🚪️ Removes every session of `principal` and returns how many were removed — the
    /// "sign out everywhere" primitive.
    async fn revoke_principal(&mut self, principal: &Principal) -> usize;
}

/// @emoji 🗝️ Reference in-memory [`SessionStore`].
#[derive(Debug, Default)]
pub struct MemorySessionStore {
    sessions: HashMap<SessionId, SessionRecord>,
}

impl MemorySessionStore {
    /// @emoji 🪺️ An empty session store.
    pub fn new() -> Self {
        Self::default()
    }
}

impl SessionStore for MemorySessionStore {
    async fn create(&mut self, session: SessionRecord) {
        self.sessions.insert(session.id.clone(), session);
    }

    async fn get(&self, id: &SessionId) -> Option<SessionRecord> {
        self.sessions.get(id).cloned()
    }

    async fn delete(&mut self, id: &SessionId) {
        self.sessions.remove(id);
    }

    async fn revoke_principal(&mut self, principal: &Principal) -> usize {
        let before = self.sessions.len();
        self.sessions.retain(|_, session| &session.principal != principal);
        before - self.sessions.len()
    }
}

dyn_enum_close! {
    pub enum SessionStores: SessionStore {
        Memory(MemorySessionStore),
    }
}
//#endregion 🔖️Session

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
