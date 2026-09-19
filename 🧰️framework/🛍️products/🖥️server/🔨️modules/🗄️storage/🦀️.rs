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
//! reference in-memory backends never do, a real disk/network backend will). Backends
//! (embedded file storage, a server-grade engine later) implement these traits behind
//! [`StorageProfile`], which the instance opens them from.
//!
//! **Four contracts, zero implementations.** This module declares the four roles and nothing that
//! fulfils them: a [`ServerInstance`](crate::gateway::ServerInstance) names the concrete backend for
//! each role as an associated type, so the set of backends is closed where the instance is defined
//! and never here. An instance that really does carry several backends of one role closes them
//! into one enum in its own crate — the site that closes the set. The reference in-memory semantics every durable backend
//! must reproduce now live with the test instance that uses them
//! (`🧪️tests/🧩️instance/🦀️.rs`), because a reference implementation is a test fixture, not a
//! product surface.

use crate::contract::{ActorKey, CommandReceipt, DeviceId, EventRecord, IdempotencyKey, Principal, Revision, SessionId};
use protocol::codec::ids::ContentHash;
use protocol::crypto::RecordHasher;
use protocol::format::Blake3Hasher;
use serde::{Deserialize, Serialize};
use semio_framework_dispatch_macros::dyn_enum;
use std::future::Future;

//#region 🔖️Profile
/// @emoji 🏗️ Which deployment shape the storage backends are opened in — the one instruction the
/// framework gives an instance's [`ServerInstance::open`](crate::gateway::ServerInstance::open),
/// and the only thing it says about durability.
///
/// Two variants, because two shapes genuinely exist and are both implemented: a process that keeps
/// nothing past its own lifetime, and a process that owns one directory. What a backend *does* with
/// either is the instance's decision — [`Ephemeral`](Self::Ephemeral) is not "no storage", it is
/// "no storage that outlives me", and an instance is free to open the same four roles over RAM for
/// it. Clustered and hosted shapes are added when a backend implements them, never as a speculative
/// option flag on these.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum StorageProfile {
    /// @emoji 🫧️ Nothing survives the process: the shape a test, a scratch instance or a
    /// short-lived edge replica opens its roles in.
    Ephemeral,
    /// @emoji 🏠️ Everything lives under one directory owned by one process: the deployment profile
    /// for hub, zentrale and a developer's laptop alike.
    Embedded { data_dir: String },
}

impl StorageProfile {
    /// @emoji 📁️ The directory this profile owns, or `None` when it owns none. A backend that needs
    /// a path asks here instead of matching, so adding a shape later is not a breaking match.
    pub fn data_dir(&self) -> Option<&str> {
        match self {
            Self::Ephemeral => None,
            Self::Embedded { data_dir } => Some(data_dir),
        }
    }

    /// @emoji 💾️ Whether state opened in this profile is expected to outlive the process.
    pub fn is_durable(&self) -> bool {
        self.data_dir().is_some()
    }
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
pub trait AuthorityStore: Send + Sync {
    /// @emoji 🔎️ The receipt already recorded for `key`, if this command was seen before. A retry
    /// answers from here instead of re-executing.
    fn receipt(&self, key: &IdempotencyKey) -> impl Future<Output = Result<Option<CommandReceipt>, StorageError>> + Send;

    /// @emoji 🧾️ Binds `key` to `receipt`. Recording the identical receipt again succeeds silently;
    /// binding a key to a *different* receipt is a [`StorageError::Conflict`].
    fn record_receipt(&mut self, key: &IdempotencyKey, receipt: &CommandReceipt) -> impl Future<Output = Result<(), StorageError>> + Send;

    /// @emoji ➕️ Appends `events` to `actor`'s stream and returns the new last sequence. Every event
    /// must carry `actor` as its stream and a sequence exactly one past its predecessor, starting at
    /// `last_seq + 1`; anything else is a [`StorageError::SequenceGap`] and nothing is written.
    fn append_events(&mut self, actor: &ActorKey, events: &[EventRecord], outbox: &[OutboxEntry]) -> impl Future<Output = Result<u64, StorageError>> + Send;

    /// @emoji 📜️ Every event of `actor` with a sequence strictly greater than `since`, in order.
    fn events_since(&self, actor: &ActorKey, since: u64) -> impl Future<Output = Result<Vec<EventRecord>, StorageError>> + Send;

    /// @emoji 🔚️ The highest sequence written for `actor`; `0` for an actor with no history.
    fn last_seq(&self, actor: &ActorKey) -> impl Future<Output = Result<u64, StorageError>> + Send;

    /// @emoji 📸️ Replaces `actor`'s replay accelerator. A snapshot older than the stored one is a
    /// [`StorageError::Conflict`] — snapshots only ever move forward.
    fn put_snapshot(&mut self, actor: &ActorKey, revision: Revision, bytes: Vec<u8>) -> impl Future<Output = Result<(), StorageError>> + Send;

    /// @emoji 🖼️ The stored snapshot of `actor` and the revision it was taken at, if any.
    fn snapshot(&self, actor: &ActorKey) -> impl Future<Output = Result<Option<(Revision, Vec<u8>)>, StorageError>> + Send;

    /// @emoji 📤️ Queues `entries` for publication, stamping each with the next queue id and marking
    /// it undelivered; the caller's `id` and `delivered` fields are ignored.
    fn enqueue_outbox(&mut self, entries: Vec<OutboxEntry>) -> impl Future<Output = Result<(), StorageError>> + Send;

    /// @emoji 📥️ Up to `limit` undelivered entries in queue order.
    fn pending_outbox(&self, limit: usize) -> impl Future<Output = Result<Vec<OutboxEntry>, StorageError>> + Send;

    /// @emoji 📬️ Acknowledges delivery of `ids`. Re-acknowledging is idempotent; an unknown id is a
    /// [`StorageError::NotFound`] and nothing is marked.
    fn mark_outbox_delivered(&mut self, ids: &[u64]) -> impl Future<Output = Result<(), StorageError>> + Send;

    /// @emoji 🤝️ Takes ownership of `actor` for `holder`. Re-acquiring as the current holder renews
    /// at the same epoch; taking it from a different holder bumps the epoch, which fences the
    /// previous holder out for good.
    fn acquire_lease(&mut self, actor: &ActorKey, holder: &str) -> impl Future<Output = Result<Lease, StorageError>> + Send;

    /// @emoji 🛡️ Whether `lease` is still the live lease on `actor`. A stale epoch answers `false`,
    /// and the caller must abandon its turn with [`StorageError::LeaseLost`].
    fn validate_lease(&self, actor: &ActorKey, lease: &Lease) -> impl Future<Output = bool> + Send;
}

//#endregion 🔖️Authority

//#region 🔖️Projection
/// @emoji 🔭️ Rebuildable read models, addressed by projection name and key. Nothing here is a source
/// of truth: [`ProjectionStore::clear`] plus a replay from sequence zero must reproduce it exactly,
/// which is what makes a schema change a rebuild rather than a migration.
///
/// **Every write answers.** `put`, `set_checkpoint` and `clear` return
/// `Result<(), `[`StorageError`]`>` rather than `()`, because a backend that journals to disk can
/// fail at the sink, and a read model whose memory silently ran ahead of its journal reports state
/// a restart will not reproduce. Rebuildability is the *repair*, never a licence to swallow the
/// fault: a refused write leaves the read models exactly as they were, and the caller decides
/// whether to retry the fold, hold the checkpoint back or rebuild. Reads keep their plain return
/// types — the fold is in memory, so there is nothing for a read to fail at.
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
pub trait ProjectionStore: Send + Sync {
    /// @emoji ✍️ Writes `value` at `key` inside `projection`, replacing any previous value. A
    /// backend that could not durably record the write answers [`StorageError::Backend`] and leaves
    /// the read models exactly as they were — see the trait note on write outcomes.
    fn put(&mut self, projection: &str, key: &str, value: Vec<u8>) -> impl Future<Output = Result<(), StorageError>> + Send;

    /// @emoji 📖️ The value stored at `key`, if the projection has one.
    fn get(&self, projection: &str, key: &str) -> impl Future<Output = Option<Vec<u8>>> + Send;

    /// @emoji 📋️ Every entry of `projection` whose key starts with `prefix`, ascending by key —
    /// ordering is part of the contract so a paged query is stable across backends.
    fn list(&self, projection: &str, prefix: &str) -> impl Future<Output = Vec<(String, Vec<u8>)>> + Send;

    /// @emoji 🚩️ The last event sequence folded into `projection`; `0` when it has never been built.
    fn checkpoint(&self, projection: &str) -> impl Future<Output = u64> + Send;

    /// @emoji 🏁️ Records that `projection` now reflects everything up to `seq`.
    fn set_checkpoint(&mut self, projection: &str, seq: u64) -> impl Future<Output = Result<(), StorageError>> + Send;

    /// @emoji 🧹️ Drops every entry of `projection` and resets its checkpoint to zero, so the next
    /// fold rebuilds it from the beginning.
    fn clear(&mut self, projection: &str) -> impl Future<Output = Result<(), StorageError>> + Send;
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
pub trait BlobStore: Send + Sync {
    /// @emoji 💾️ Stores `bytes` under `hash`. Storing identical content again succeeds silently;
    /// binding a hash to different bytes is a [`StorageError::Conflict`].
    fn put(&mut self, hash: ContentHash, bytes: &[u8]) -> impl Future<Output = Result<(), StorageError>> + Send;

    /// @emoji 📦️ The bytes stored under `hash`, if any.
    fn get(&self, hash: &ContentHash) -> impl Future<Output = Option<Vec<u8>>> + Send;

    /// @emoji ❓️ Whether `hash` is already stored — the cheap half of an upload negotiation.
    fn has(&self, hash: &ContentHash) -> impl Future<Output = bool> + Send;
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
///
/// **Every write answers, and here it is a security property.** `create`, `delete` and
/// `revoke_principal` return a [`StorageError`] on failure rather than `()`/`usize`, because a
/// revocation the backend could not carry out leaves a key that still opens the door. A caller
/// told "signed out everywhere" when the store failed to remove the records has been told
/// something false; it must see the fault and refuse the sign-out instead. Reads stay plain — a
/// live session set is answered from memory.
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
pub trait SessionStore: Send + Sync {
    /// @emoji 🆕️ Stores `session`, replacing any record with the same id. A backend that could not
    /// record it answers [`StorageError::Backend`] and stores nothing, so a caller never hands out
    /// a session id the store does not hold.
    fn create(&mut self, session: SessionRecord) -> impl Future<Output = Result<(), StorageError>> + Send;

    /// @emoji 🔑️ The session with `id`, if it is still live.
    fn get(&self, id: &SessionId) -> impl Future<Output = Option<SessionRecord>> + Send;

    /// @emoji 🗑️ Removes `id`; a no-op if it is already gone. A removal the backend refused is a
    /// [`StorageError::Backend`], never a silent success — the session still opens the door.
    fn delete(&mut self, id: &SessionId) -> impl Future<Output = Result<(), StorageError>> + Send;

    /// @emoji 🚪️ Removes every session of `principal` and returns how many were removed — the
    /// "sign out everywhere" primitive. Refusing on the first record it cannot remove is
    /// deliberate: a partial sign-out reported as a number is indistinguishable from a complete one.
    fn revoke_principal(&mut self, principal: &Principal) -> impl Future<Output = Result<usize, StorageError>> + Send;
}

//#endregion 🔖️Session

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
