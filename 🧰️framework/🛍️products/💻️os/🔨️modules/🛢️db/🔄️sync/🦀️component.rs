//! 🗄️ `db_sync` — server side of `protocol_wire`: frontier exchange, missing-command transfer,
//! snapshot bootstrap, and resume tokens for a document replica ((re)connecting to the semio_hub over
//! `protocol::{ClientFrame, ServerFrame}`). Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`); wire types frozen in `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/
//! contract.md` `## Amendment` §`protocol_wire`.
//!
//! 🎯️ Design choice (dependency scope): per the contract's dep table this crate depends on
//! `db_core`, `db_wal`, `db_storage`, `protocol` only — no `db_artifact` (still a stub as of this
//! wave, and per the contract's hard dependency rule this crate never interprets operation
//! semantics anyway). Every function below therefore derives a document's sync state directly
//! from its WAL via `db_wal::replay_document` rather than consulting a live document actor — the
//! authoritative source once `db_artifact` lands. This crate's replay-derived `Frontier` is a
//! faithful stand-in (see `//#region 🔖️ReplicaState`'s doc for exactly which fields are derived
//! vs. placeholder), not a shortcut: every WAL record this crate touches is decoded and verified
//! the same way `db_wal`'s own recovery path does.
//!
//! 🎯️ Design choice (`ResumeToken` receive path): `ResumeToken` exposes `encode(&Frontier)
//! -> ResumeToken` and `ResumeToken::decode(&self) -> Frontier`, but no public constructor from an
//! arbitrary wire string (its inner field is private to `db_core`) — so this crate cannot
//! reconstruct a `ResumeToken` from `protocol::ClientFrame::Hello.resume_token: Option<String>` to
//! call its type-safe `decode`. Rather than duplicating `db_core`'s private parsing logic (a
//! frozen, un-editable crate this wave), this crate uses `Hello.frontier:
//! Option<protocol::RuntimeFrontierSummary>` — a separate, always-decodable field on the very same
//! frame — as the authoritative source of "where is the replica" on the receive path. The
//! `resume_token` this crate ISSUES (`issue_resume_token`, on the send path, `Welcome.resume_token`)
//! is fully real: `ResumeToken::encode` is public and exercised end to end.
use crate::db_durability::Frontier;
use crate::*;
/// @emoji ✉️ This crate's own convention for `db_wal::WalRecord::Command`'s payload bytes:
/// `protocol_causal::encode_envelope`'s binary record — the same primitive codec `protocol_wire`
/// uses for `ClientFrame::Commands`/`ServerFrame::Commands`, so a WAL command's bytes are
/// byte-identical to its on-wire form (M-C's "communication AND storage both binary"). `db_wal`
/// itself never interprets these bytes (per the contract, no crate below `db_artifact` does);
/// this crate is the first one that needs to read a command's bytes back out semantically (to
/// relay it as a typed `protocol::MutationEnvelope` in a `ServerFrame::Commands`), so it is the
/// natural place to fix this convention. Once `db_artifact` lands it becomes the writer of these
/// bytes; this codec is the seam it should reuse rather than inventing a second one.
pub async fn encode_command_envelope(envelope: &protocol::MutationEnvelope) -> Vec<u8> {
    let mut out = Vec::new();
    protocol::encode_envelope(envelope, &mut out);
    out
}

/// @emoji 📖️ Inverse of `encode_command_envelope`. Validates the byte length against
/// `DbLimits::default().max_command_bytes` BEFORE decoding anything sized by it (mirrors
/// `pack_core`'s stated invariant), then maps a decode failure to `DbError::Corrupt` rather than
/// leaking `protocol::ProtocolError`.
#[cfg(test)]
async fn decode_command_envelope(bytes: &[u8]) -> Result<protocol::MutationEnvelope, DbError> {
    check_len(bytes.len() as u64, DbLimits::default().max_command_bytes, "wal_command_envelope")?;
    let mut pos = 0usize;
    let envelope = protocol::decode_envelope(bytes, &mut pos).map_err(|error| DbError::Corrupt(format!("malformed wal command envelope: {error}")))?;
    Ok(envelope)
}

async fn decode_retained_command_envelope(bytes: &db_wal::WalBytes, control: &mut db_wal::WalCursorControl) -> Result<protocol::MutationEnvelope, DbError> {
    let mut cursor = bytes.cursor();
    let mutation_id = protocol::MutationId(cursor.text(4_096, control)?);
    let document_id = protocol::ArtifactId(cursor.text(4_096, control)?);
    let actor = protocol::ActorId(cursor.text(4_096, control)?);
    let count = cursor.varint(control)?;
    check_len(count, 65_536, "sync wal envelope dependencies")?;
    let mut dependencies = Vec::with_capacity(count as usize);
    for _ in 0..count {
        dependencies.push(protocol::MutationId(cursor.text(4_096, control)?));
        semio_framework_async::yield_once().await;
    }
    let diff_schema = protocol::SchemaId(cursor.text(4_096, control)?);
    let diff_payload = decode_protocol_field(&mut cursor, 256 * 1024 * 1024, control).await?;
    let inverse_schema = protocol::SchemaId(cursor.text(4_096, control)?);
    let inverse_payload = decode_protocol_field(&mut cursor, 256 * 1024 * 1024, control).await?;
    let timestamp = protocol::HybridLogicalTimestamp { actor: cursor.varint(control)?, physical_ms: cursor.varint(control)?, logical: cursor.varint(control)? };
    if cursor.remaining() != 0 {
        return Err(DbError::Corrupt("sync wal command has trailing bytes".to_string()));
    }
    Ok(protocol::MutationEnvelope {
        mutation_id,
        document_id,
        actor,
        dependencies,
        diff: protocol::ArtifactDiff { schema: diff_schema, payload: diff_payload },
        inverse: protocol::InverseMutation { schema: inverse_schema, payload: inverse_payload },
        timestamp,
    })
}

async fn decode_protocol_field(cursor: &mut db_wal::WalBytesCursor<'_>, maximum: u64, control: &mut db_wal::WalCursorControl) -> Result<Vec<u8>, DbError> {
    let mut remaining = cursor.begin_field(maximum, control)?;
    let mut output = Vec::with_capacity(remaining);
    let mut fragment = [0u8; 4096];
    while remaining != 0 {
        let copied = cursor.read_field_fragment(&mut remaining, &mut fragment, control)?;
        output.extend_from_slice(&fragment[..copied]);
        semio_framework_async::yield_once().await;
    }
    Ok(output)
}
//#endregion 🔖️Codec

//#region 🔖️ReplicaState
/// @emoji 🧾️ One document's currently-retained sync state, replayed fresh from its WAL — the
/// shared input every negotiation function below works from.
///
/// 🎯️ Design choice (`Frontier` field derivation, since `db_artifact` doesn't exist yet to supply
/// an authoritative one): `head_seq` = count of `WAL_COMMAND` records replayed (genesis = 0);
/// `commit_seq` = count of `WAL_TX_COMMIT` records replayed; `chain_hash` = a replay-derived
/// content chain, `blake3(digest_1 || .. || digest_k)` where `digest_i = blake3(command_i's raw WAL
/// bytes)` — the same shape `protocol::verify_slice`'s `slice_content_chain` uses, chosen because
/// `db_wal` does not expose a public accessor for a segment's real commit `chain_hash` (see
/// `db_wal`'s own `SegmentWriter::tip_chain_hash`, which is private); `epoch` is always `0` here —
/// cluster fencing epochs are `db_cluster`'s concern, unreachable without a `CatalogStorage` scoped
/// to this specific document's shard, which this crate's inputs don't carry.
#[derive(Clone, Debug, PartialEq)]
pub struct ArtifactSyncState {
    pub frontier: Frontier,
    pub commands: Vec<protocol::MutationEnvelope>,
    /// @emoji 🚧️ The lowest `head_seq` this crate can still serve via tail (missing-command)
    /// transfer — the `head_seq` of the most recent `WAL_SNAPSHOT_PUB` record replayed, or `0` if
    /// none (nothing has ever been compacted away). A replica behind this floor needs
    /// `decide_bootstrap`'s snapshot path instead.
    pub floor_head_seq: u64,
}

/// @emoji 🔁️ Replays `document`'s entire currently-retained WAL via `db_wal::replay_document` and
/// derives its `ArtifactSyncState` — see the struct's doc for exactly how each field is derived.
pub async fn replay_sync_state(storage: &impl db_storage::WalStorage, document: ArtifactId) -> Result<ArtifactSyncState, DbError> {
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = db_wal::WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
    let mut records = db_wal::replay_document(storage, &document, control).await?;
    let mut decode_control = db_wal::WalCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
    let mut commands = Vec::new();
    let mut chain = blake3::Hasher::new();
    let mut commit_seq = 0u64;
    let mut floor_head_seq = 0u64;
    loop {
        let mut record = match records.next_step().await? {
            db_wal::WalReplayStep::Record(record) => record,
            db_wal::WalReplayStep::Yield => continue,
            db_wal::WalReplayStep::Done => break,
        };
        match &mut record {
            db_wal::WalRecord::Command(bytes) => {
                commands.push(decode_retained_command_envelope(bytes, &mut decode_control).await?);
                chain.update(&bytes.hash());
            }
            db_wal::WalRecord::TxCommit { .. } => commit_seq += 1,
            // 🎯️ Overwritten on every occurrence rather than max()'d: `WalRecord`s replay in
            // on-disk (chronological) order, so the last one seen is always the most recent.
            db_wal::WalRecord::SnapshotPub { frontier, .. } => floor_head_seq = frontier.head_seq,
            _ => {}
        }
        let _ = record.close_step()?;
        drop(record);
        semio_framework_async::yield_once().await;
    }
    let _ = records.close_step().await?;
    drop(records);
    let head_seq = commands.len() as u64;
    let chain_hash = if commands.is_empty() { [0; 32] } else { *chain.finalize().as_bytes() };
    let frontier = Frontier { document, head_seq, commit_seq, chain_hash, epoch: 0 };
    Ok(ArtifactSyncState { frontier, commands, floor_head_seq })
}

//#endregion 🔖️ReplicaState

//#region 🔖️Frontier
/// @emoji ➖️ `FrontierDelta::between`, re-exposed under this crate's own name for
/// discoverability — frontier-delta computation is this crate's stated responsibility, so
/// `db_sync::frontier_delta` is the expected first stop even though the primitive itself lives in
/// `db_core`.
pub async fn frontier_delta(from: &Frontier, to: &Frontier) -> Result<FrontierDelta, DbError> {
    FrontierDelta::between(from, to).await
}

/// @emoji 🌉️ `Frontier` -> `protocol::RuntimeFrontierSummary` (the wire-frame shape
/// `ServerFrame::{Welcome, Commands, Ack}.*frontier` fields carry). `head_edit_id` has no
/// `Frontier` counterpart (see `ArtifactSyncState`'s doc); callers pass whatever they
/// consider the frontier's tip identity (`state_frontier_summary` below supplies the natural
/// choice: the last replayed command's `mutation_id`).
pub async fn to_frontier_summary(frontier: &Frontier, head_edit_id: String) -> protocol::RuntimeFrontierSummary {
    protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(frontier.document.0.clone()), head_edit_ordinal: frontier.head_seq, head_edit_id, last_commit_seq: frontier.commit_seq, chain_hash: frontier.chain_hash }
}

/// @emoji 🌉️ Inverse bridge direction: `protocol::RuntimeFrontierSummary` -> `Frontier`,
/// the primitive `handle_hello`/`handle_frontier_advertise` use to turn a replica's advertised
/// wire frontier into something `missing_commands`/`decide_bootstrap` can compare against a
/// `ArtifactSyncState`. `epoch` is always `0` (see `ArtifactSyncState`'s doc: `RuntimeFrontierSummary`
/// carries no cluster-fencing epoch at all).
// 🚫️async: E1 pure accessor consumed by a sync Option::map — see R9
pub fn from_frontier_summary(summary: &protocol::RuntimeFrontierSummary) -> Frontier {
    Frontier { document: ArtifactId(summary.document_id.0.clone()), head_seq: summary.head_edit_ordinal, commit_seq: summary.last_commit_seq, chain_hash: summary.chain_hash, epoch: 0 }
}

/// @emoji 🌉️ `state`'s own frontier as a `RuntimeFrontierSummary`, with `head_edit_id` filled from
/// the last replayed command's `mutation_id` (empty string for a genesis document with no
/// commands yet).
pub async fn state_frontier_summary(state: &ArtifactSyncState) -> protocol::RuntimeFrontierSummary {
    let head_edit_id = state.commands.last().map(|envelope| envelope.mutation_id.0.clone()).unwrap_or_default();
    to_frontier_summary(&state.frontier, head_edit_id).await
}
//#endregion 🔖️Frontier

//#region 🔖️MissingCommands
/// @emoji 📦️ The missing-command-transfer primitive: every command `state` holds strictly after
/// `replica`'s `head_seq`, in replay order — what `db_sync` ships a reconnecting/catching-up
/// replica via `ServerFrame::Commands`.
///
/// 🎯️ Design choice (why not `protocol::extract_range`/`RecordSlice`): that primitive walks a
/// `.spr` stream for `protocol::wire::REC_EDIT`-kind frames — the shape `protocol_history`'s
/// history-log format uses. `db_wal`'s WAL segments are also `.spr` containers but frame commands
/// under the family's own `WAL_COMMAND` (`0x44`) record kind in the `0x40..=0x4F` extension range
/// (see `db_wal`'s `//#region 🔖️RecordKinds`), never `REC_EDIT` — so `extract_range` structurally
/// cannot find them. This function is this crate's `WAL_COMMAND`-shaped analog, built the same
/// way (a linear ordinal-indexed slice) but over `ArtifactSyncState::commands`, which is already
/// the fully-decoded, ordinal-indexed sequence `replay_sync_state` produced.
pub async fn missing_commands(state: &ArtifactSyncState, replica: &Frontier) -> Result<Vec<protocol::MutationEnvelope>, DbError> {
    if replica.document != state.frontier.document {
        return Err(DbError::InvalidArgument(format!("frontier document mismatch: replica {} vs server {}", replica.document, state.frontier.document)));
    }
    if replica.head_seq > state.frontier.head_seq {
        return Err(DbError::InvalidArgument(format!("replica frontier is ahead of the server: replica head_seq {} > server head_seq {}", replica.head_seq, state.frontier.head_seq)));
    }
    if replica.head_seq < state.floor_head_seq {
        return Err(DbError::Unavailable(format!("replica head_seq {} is behind the retained WAL floor {}; snapshot bootstrap is required", replica.head_seq, state.floor_head_seq)));
    }
    Ok(state.commands[replica.head_seq as usize..].to_vec())
}

/// @emoji 📨️ Wraps `envelopes` (typically `missing_commands`' result) as a `ServerFrame::Commands`
/// stamped with `state`'s current frontier — `origin` is the relaying actor identity the caller
/// (the semio_hub session layer, which owns its own actor identity) supplies; this crate has no opinion
/// on it beyond passing it through.
pub async fn commands_server_frame(state: &ArtifactSyncState, envelopes: Vec<protocol::MutationEnvelope>, origin: protocol::ActorId) -> protocol::ServerFrame {
    protocol::ServerFrame::Commands { envelopes, origin, frontier: state_frontier_summary(state).await }
}
//#endregion 🔖️MissingCommands

//#region 🔖️Bootstrap
/// @emoji 🚀️ How a (re)connecting replica should be caught up, decided by `decide_bootstrap` —
/// the pre-wire-encoding twin of `protocol::Bootstrap` (kept separate so this crate's core
/// decision logic stays testable without constructing full `ServerFrame`s; `build_welcome` below
/// lowers it to the wire shape).
#[derive(Debug, PartialEq)]
pub enum BootstrapPlan {
    /// @emoji ✅️ The replica is already fully caught up — nothing to send.
    None,
    /// @emoji 🚚️ The replica is within the retained WAL floor: ship it the missing commands
    /// directly, no snapshot needed.
    Tail { envelopes: Vec<protocol::MutationEnvelope> },
    /// @emoji 📸️ The replica is behind the retained WAL floor (or brand new against a compacted
    /// document): ship it a whole snapshot generation first.
    Snapshot { generation: u64, pages: db_storage::DbIoPages, pack_hash: [u8; 32] },
}

/// @emoji 🧭️ Decides `BootstrapPlan` for `replica` (`None` meaning a totally fresh replica with no
/// prior frontier at all) against `state`, consulting `snapshots` only when the replica's
/// `head_seq` has fallen behind `state.floor_head_seq`.
pub async fn decide_bootstrap(state: &ArtifactSyncState, snapshots: &impl db_storage::SnapshotStorage, replica: Option<&Frontier>) -> Result<BootstrapPlan, DbError> {
    let replica_head_seq = replica.map_or(0, |frontier| frontier.head_seq);
    if replica_head_seq >= state.floor_head_seq {
        let missing = match replica {
            Some(frontier) => missing_commands(state, frontier).await?,
            None => state.commands.clone(),
        };
        return Ok(if missing.is_empty() { BootstrapPlan::None } else { BootstrapPlan::Tail { envelopes: missing } });
    }
    let generation = snapshots
        .latest_generation(&state.frontier.document)
        .await?
        .ok_or_else(|| DbError::Unavailable(format!("replica head_seq {replica_head_seq} is behind the retained WAL floor {} and no snapshot generation is available", state.floor_head_seq)))?;
    let pages = snapshots.read_generation(&state.frontier.document, generation).await?;
    let pack_hash = db_storage::db_io_hash_pages(&pages).await.0;
    Ok(BootstrapPlan::Snapshot { generation, pages, pack_hash })
}
//#endregion 🔖️Bootstrap

//#region 🔖️ResumeToken
/// @emoji 🎫️ Issues a fresh resume token for `frontier` — the send-path half of resume tokens (see
/// module doc for why the receive path uses `Hello.frontier` instead). `Welcome.resume_token` is
/// always populated from this.
pub async fn issue_resume_token(frontier: &Frontier) -> Result<String, DbError> {
    Ok(ResumeToken::encode(frontier)?.as_str().to_string())
}
//#endregion 🔖️ResumeToken

//#region 🔖️Hello
//#region 👋️RetainedHello
const DATABASE_SYNC_HELLO_SLOTS: usize = 8;
const DATABASE_SYNC_HELLO_MAX_ITEMS: usize = 65_536;
const DATABASE_SYNC_HELLO_MAX_BYTES: usize = 256 * 1024 * 1024;
const DATABASE_SYNC_HELLO_RETRY_LIMIT: u8 = 8;
const DATABASE_SYNC_HELLO_DEADLINE_MS: u64 = 30_000;
const DATABASE_SYNC_HELLO_TURN_MS: u64 = 8;
const DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES: usize = 4 * 1024;
const DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS: usize = db_storage::DB_IO_OPERATION_PAGES;
const DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_BYTES: usize = DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS * db_storage::DB_IO_PAGE_BYTES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DatabaseSyncHelloProgress {
    Admitted,
    Replay,
    Decode,
    Bootstrap,
    Welcome,
    Streaming,
    Closing,
    Completed,
    Cancelled,
    Fault,
}

impl DatabaseSyncHelloProgress {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Admitted,
            1 => Self::Replay,
            2 => Self::Decode,
            3 => Self::Bootstrap,
            4 => Self::Welcome,
            5 => Self::Streaming,
            6 => Self::Closing,
            7 => Self::Completed,
            8 => Self::Cancelled,
            _ => Self::Fault,
        }
    }
}

#[derive(Clone, Copy)]
struct DatabaseSyncHelloAdmissionSlot {
    generation: u64,
    items: usize,
    bytes: usize,
    occupied: bool,
}

const EMPTY_DATABASE_SYNC_HELLO_SLOT: DatabaseSyncHelloAdmissionSlot = DatabaseSyncHelloAdmissionSlot { generation: 0, items: 0, bytes: 0, occupied: false };

struct DatabaseSyncHelloAdmissionState {
    slots: [DatabaseSyncHelloAdmissionSlot; DATABASE_SYNC_HELLO_SLOTS],
    items: usize,
    bytes: usize,
    next_generation: u64,
}

static DATABASE_SYNC_HELLO_ADMISSION: std::sync::Mutex<DatabaseSyncHelloAdmissionState> =
    std::sync::Mutex::new(DatabaseSyncHelloAdmissionState { slots: [EMPTY_DATABASE_SYNC_HELLO_SLOT; DATABASE_SYNC_HELLO_SLOTS], items: 0, bytes: 0, next_generation: 1 });

struct DatabaseSyncHelloAdmission {
    slot: usize,
    generation: u64,
    items: usize,
    bytes: usize,
}

impl DatabaseSyncHelloAdmission {
    fn try_claim(input_items: usize, input_bytes: usize) -> Result<Self, DbError> {
        if input_items == 0 || input_items > DATABASE_SYNC_HELLO_MAX_ITEMS || input_bytes > DATABASE_SYNC_HELLO_MAX_BYTES {
            return Err(DbError::LimitExceeded("database sync hello input credit"));
        }
        let mut state = DATABASE_SYNC_HELLO_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(slot) = state.slots.iter().position(|entry| !entry.occupied) else {
            return Err(DbError::Unavailable("database sync hello admission saturated".to_string()));
        };
        let items = DATABASE_SYNC_HELLO_MAX_ITEMS;
        let bytes = DATABASE_SYNC_HELLO_MAX_BYTES;
        let next_items = state.items.checked_add(items).ok_or(DbError::LimitExceeded("database sync hello total items"))?;
        let next_bytes = state.bytes.checked_add(bytes).ok_or(DbError::LimitExceeded("database sync hello total bytes"))?;
        if next_items > DATABASE_SYNC_HELLO_SLOTS * DATABASE_SYNC_HELLO_MAX_ITEMS || next_bytes > DATABASE_SYNC_HELLO_SLOTS * DATABASE_SYNC_HELLO_MAX_BYTES {
            return Err(DbError::Unavailable("database sync hello global credit saturated".to_string()));
        }
        let generation = state.next_generation;
        state.next_generation = generation.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello generation"))?;
        state.slots[slot] = DatabaseSyncHelloAdmissionSlot { generation, items, bytes, occupied: true };
        state.items = next_items;
        state.bytes = next_bytes;
        Ok(Self { slot, generation, items, bytes })
    }

    fn is_current(&self) -> bool {
        let state = DATABASE_SYNC_HELLO_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.slots.get(self.slot).is_some_and(|entry| entry.occupied && entry.generation == self.generation && entry.items == self.items && entry.bytes == self.bytes)
    }
}

impl Drop for DatabaseSyncHelloAdmission {
    fn drop(&mut self) {
        let mut state = DATABASE_SYNC_HELLO_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(entry) = state.slots.get_mut(self.slot) else { return };
        if !entry.occupied || entry.generation != self.generation || entry.items != self.items || entry.bytes != self.bytes {
            return;
        }
        *entry = EMPTY_DATABASE_SYNC_HELLO_SLOT;
        state.items = state.items.saturating_sub(self.items);
        state.bytes = state.bytes.saturating_sub(self.bytes);
    }
}

#[derive(Default)]
struct DatabaseSyncHelloBackingLedger {
    items: usize,
    bytes: usize,
}

impl DatabaseSyncHelloBackingLedger {
    fn observe(&mut self, items: usize, bytes: usize, label: &'static str) -> Result<(), DbError> {
        let next_items = self.items.checked_add(items).ok_or(DbError::LimitExceeded(label))?;
        let next_bytes = self.bytes.checked_add(bytes).ok_or(DbError::LimitExceeded(label))?;
        if next_items > DATABASE_SYNC_HELLO_MAX_ITEMS || next_bytes > DATABASE_SYNC_HELLO_MAX_BYTES {
            return Err(DbError::LimitExceeded(label));
        }
        self.items = next_items;
        self.bytes = next_bytes;
        Ok(())
    }

    fn release(&mut self, items: usize, bytes: usize) -> Result<(), DbError> {
        self.items = self.items.checked_sub(items).ok_or(DbError::Internal("database sync hello item credit underflow".to_string()))?;
        self.bytes = self.bytes.checked_sub(bytes).ok_or(DbError::Internal("database sync hello byte credit underflow".to_string()))?;
        Ok(())
    }

    fn reserve_allocation(&mut self, items: usize, requested: usize, label: &'static str) -> Result<usize, DbError> {
        let reserved = DATABASE_SYNC_HELLO_MAX_BYTES.checked_sub(self.bytes).ok_or(DbError::LimitExceeded(label))?;
        if requested > reserved {
            return Err(DbError::LimitExceeded(label));
        }
        self.observe(items, reserved, label)?;
        Ok(reserved)
    }

    fn settle_allocation(&mut self, reserved: usize, actual: usize, label: &'static str) -> Result<(), DbError> {
        if actual > reserved {
            return Err(DbError::LimitExceeded(label));
        }
        self.release(0, reserved - actual)
    }

    fn terminal_is_empty(&self) -> bool {
        self.items == 0 && self.bytes == 0
    }

    fn close_one_credit(&mut self) -> bool {
        if self.bytes != 0 {
            self.bytes -= self.bytes.min(db_storage::DB_IO_PAGE_BYTES);
            return true;
        }
        if self.items != 0 {
            self.items -= 1;
            return true;
        }
        false
    }
}

fn database_sync_hello_allocate_vec<T>(ledger: &mut DatabaseSyncHelloBackingLedger, count: usize, label: &'static str) -> Result<Vec<T>, DbError> {
    if count == 0 {
        return Ok(Vec::new());
    }
    let requested = count.checked_mul(std::mem::size_of::<T>()).ok_or(DbError::LimitExceeded(label))?;
    let reserved = ledger.reserve_allocation(1, requested, label)?;
    let mut owner = Vec::new();
    if owner.try_reserve_exact(count).is_err() {
        ledger.release(1, reserved)?;
        return Err(DbError::LimitExceeded(label));
    }
    let actual = owner.capacity().checked_mul(std::mem::size_of::<T>()).ok_or(DbError::LimitExceeded(label))?;
    if actual > reserved {
        drop(owner);
        ledger.release(1, reserved)?;
        return Err(DbError::LimitExceeded(label));
    }
    ledger.settle_allocation(reserved, actual, label)?;
    Ok(owner)
}

struct DatabaseSyncHelloSnapshotBackingReservation {
    bytes: usize,
}

fn database_sync_hello_reserve_snapshot_chunk(ledger: &mut DatabaseSyncHelloBackingLedger, logical_len: usize) -> Result<DatabaseSyncHelloSnapshotBackingReservation, DbError> {
    if logical_len == 0 || logical_len > DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES {
        return Err(DbError::LimitExceeded("database sync hello fixed snapshot chunk length"));
    }
    ledger.observe(1, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES, "database sync hello fixed snapshot chunk backing")?;
    Ok(DatabaseSyncHelloSnapshotBackingReservation { bytes: DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES })
}

impl DatabaseSyncHelloSnapshotBackingReservation {
    fn allocate(self, ledger: &mut DatabaseSyncHelloBackingLedger) -> Result<protocol::SnapshotChunkBytes, DbError> {
        let mut owner = protocol::SnapshotChunkBytes::allocate_fixed();
        let actual = owner.backing_bytes();
        if actual != self.bytes {
            owner.close_one();
            ledger.release(1, self.bytes)?;
            return Err(DbError::LimitExceeded("database sync hello fixed snapshot chunk observed backing"));
        }
        Ok(owner)
    }
}

#[derive(Clone, Copy)]
struct DatabaseSyncHelloSnapshotPageReservation {
    items: usize,
    bytes: usize,
}

fn database_sync_hello_reserve_snapshot_pages(ledger: &mut DatabaseSyncHelloBackingLedger) -> Result<DatabaseSyncHelloSnapshotPageReservation, DbError> {
    ledger.observe(DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS, DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_BYTES, "database sync hello fixed snapshot page backing")?;
    Ok(DatabaseSyncHelloSnapshotPageReservation { items: DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS, bytes: DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_BYTES })
}

impl DatabaseSyncHelloSnapshotPageReservation {
    fn observed(self, pages: &db_storage::DbIoPages) -> Result<(usize, usize), DbError> {
        let items = usize::from(pages.page_count());
        let bytes = items.checked_mul(db_storage::DB_IO_PAGE_BYTES).ok_or(DbError::LimitExceeded("database sync hello snapshot observed page backing"))?;
        if items > self.items || bytes > self.bytes || pages.len() > bytes {
            return Err(DbError::LimitExceeded("database sync hello snapshot observed page backing"));
        }
        Ok((items, bytes))
    }

    fn settle(self, ledger: &mut DatabaseSyncHelloBackingLedger, items: usize, bytes: usize) -> Result<(), DbError> {
        ledger.release(self.items - items, self.bytes - bytes)
    }

    fn release(self, ledger: &mut DatabaseSyncHelloBackingLedger) -> Result<(), DbError> {
        ledger.release(self.items, self.bytes)
    }
}

fn database_sync_hello_clone_string(source: &str, ledger: &mut DatabaseSyncHelloBackingLedger, label: &'static str) -> Result<String, DbError> {
    let mut owner = database_sync_hello_allocate_vec::<u8>(ledger, source.len(), label)?;
    owner.extend_from_slice(source.as_bytes());
    String::from_utf8(owner).map_err(|error| {
        let mut owner = error.into_bytes();
        let capacity = owner.capacity();
        drop(std::mem::take(&mut owner));
        let _ = ledger.release(1, capacity);
        DbError::Corrupt("database sync hello admitted text lost utf-8 validity".to_string())
    })
}

struct DatabaseSyncHelloOwners {
    storage: Option<std::sync::Arc<db_storage::DbBackend>>,
    document: ArtifactId,
    hello_frontier: Option<protocol::RuntimeFrontierSummary>,
    session_id: String,
    origin: protocol::ActorId,
    snapshot_chunk_bytes: usize,
}

fn database_sync_hello_retire_string(owner: &mut String) -> bool {
    if owner.capacity() == 0 {
        return false;
    }
    drop(std::mem::take(owner));
    true
}

fn database_sync_hello_retire_bytes(owner: &mut Vec<u8>) -> bool {
    if owner.capacity() == 0 {
        return false;
    }
    drop(std::mem::take(owner));
    true
}

struct DatabaseSyncHelloEnvelopeClose {
    owner: Option<protocol::MutationEnvelope>,
}

impl DatabaseSyncHelloEnvelopeClose {
    fn close_one(&mut self) -> bool {
        let Some(owner) = self.owner.as_mut() else { return false };
        if let Some(dependency) = owner.dependencies.pop() {
            drop(dependency);
            return true;
        }
        if owner.dependencies.capacity() != 0 {
            drop(std::mem::take(&mut owner.dependencies));
            return true;
        }
        if database_sync_hello_retire_string(&mut owner.mutation_id.0)
            || database_sync_hello_retire_string(&mut owner.document_id.0)
            || database_sync_hello_retire_string(&mut owner.actor.0)
            || database_sync_hello_retire_string(&mut owner.diff.schema.0)
            || database_sync_hello_retire_bytes(&mut owner.diff.payload)
            || database_sync_hello_retire_string(&mut owner.inverse.schema.0)
            || database_sync_hello_retire_bytes(&mut owner.inverse.payload)
        {
            return true;
        }
        self.owner = None;
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none()
    }
}

struct DatabaseSyncHelloFrameClose {
    owner: Option<protocol::ServerFrame>,
    envelope: Option<DatabaseSyncHelloEnvelopeClose>,
}

impl DatabaseSyncHelloFrameClose {
    fn close_one(&mut self) -> bool {
        if let Some(cursor) = self.envelope.as_mut() {
            let pending = cursor.close_one();
            if cursor.terminal_is_empty() {
                self.envelope = None;
            }
            return pending;
        }
        let Some(owner) = self.owner.as_mut() else { return false };
        match owner {
            protocol::ServerFrame::Welcome { session_id, resume_token, server_frontier, bootstrap } => {
                if database_sync_hello_retire_string(session_id)
                    || database_sync_hello_retire_string(resume_token)
                    || database_sync_hello_retire_string(&mut server_frontier.document_id.0)
                    || database_sync_hello_retire_string(&mut server_frontier.head_edit_id)
                {
                    return true;
                }
                if let protocol::Bootstrap::Snapshot { inline, .. } = bootstrap {
                    if let Some(bytes) = inline.as_mut() {
                        if database_sync_hello_retire_bytes(bytes) {
                            return true;
                        }
                        *inline = None;
                        return true;
                    }
                }
            }
            protocol::ServerFrame::Commands { envelopes, origin, frontier } => {
                if let Some(envelope) = envelopes.pop() {
                    self.envelope = Some(DatabaseSyncHelloEnvelopeClose { owner: Some(envelope) });
                    return true;
                }
                if envelopes.capacity() != 0 {
                    drop(std::mem::take(envelopes));
                    return true;
                }
                if database_sync_hello_retire_string(&mut origin.0) || database_sync_hello_retire_string(&mut frontier.document_id.0) || database_sync_hello_retire_string(&mut frontier.head_edit_id) {
                    return true;
                }
            }
            protocol::ServerFrame::SnapshotChunk { bytes, .. } => {
                if bytes.close_one() {
                    return true;
                }
            }
            _ => {}
        }
        self.owner = None;
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none() && self.envelope.is_none()
    }
}

impl DatabaseSyncHelloOwners {
    fn storage(&self) -> Result<&db_storage::DbBackend, DbError> {
        self.storage.as_deref().ok_or_else(|| DbError::Internal("database sync hello storage owner missing".to_string()))
    }

    fn close_one(&mut self) -> bool {
        if let Some(frontier) = self.hello_frontier.as_mut() {
            if database_sync_hello_retire_string(&mut frontier.document_id.0) || database_sync_hello_retire_string(&mut frontier.head_edit_id) {
                return true;
            }
            self.hello_frontier = None;
            return true;
        }
        if database_sync_hello_retire_string(&mut self.session_id) || database_sync_hello_retire_string(&mut self.origin.0) || database_sync_hello_retire_string(&mut self.document.0) {
            return true;
        }
        self.storage.take().is_some()
    }

    fn terminal_is_empty(&self) -> bool {
        self.storage.is_none() && self.hello_frontier.is_none() && self.session_id.capacity() == 0 && self.origin.0.capacity() == 0 && self.document.0.capacity() == 0
    }
}

fn database_sync_hello_input_credit(owners: &DatabaseSyncHelloOwners) -> Result<(usize, usize), DbError> {
    let frontier_items = owners.hello_frontier.as_ref().map_or(0, |_| 3);
    let items = 4usize.checked_add(frontier_items).ok_or(DbError::LimitExceeded("database sync hello input items"))?;
    let frontier_bytes = owners.hello_frontier.as_ref().map_or(0, |frontier| frontier.document_id.0.capacity().saturating_add(frontier.head_edit_id.capacity()).saturating_add(std::mem::size_of::<protocol::RuntimeFrontierSummary>()));
    let bytes = owners
        .document
        .0
        .capacity()
        .checked_add(owners.session_id.capacity())
        .and_then(|value| value.checked_add(owners.origin.0.capacity()))
        .and_then(|value| value.checked_add(frontier_bytes))
        .ok_or(DbError::LimitExceeded("database sync hello input bytes"))?;
    Ok((items, bytes))
}

async fn database_sync_hello_opportunity(cancelled: &std::sync::atomic::AtomicBool, expired: &std::sync::atomic::AtomicBool) -> Result<(), DbError> {
    database_sync_hello_control(cancelled, expired)?;
    semio_framework_async::yield_once().await;
    database_sync_hello_control(cancelled, expired)
}

fn database_sync_hello_control(cancelled: &std::sync::atomic::AtomicBool, expired: &std::sync::atomic::AtomicBool) -> Result<(), DbError> {
    if expired.load(std::sync::atomic::Ordering::Acquire) {
        return Err(DbError::Timeout("database sync hello deadline"));
    }
    if cancelled.load(std::sync::atomic::Ordering::Acquire) {
        return Err(DbError::Closed);
    }
    Ok(())
}

fn database_sync_hello_allocate_envelope_vec<T>(ledger: &mut DatabaseSyncHelloBackingLedger, count: usize) -> Result<Vec<T>, DbError> {
    if count == 0 {
        return Ok(Vec::new());
    }
    let label = "database sync hello cumulative envelope backing";
    let requested = count.checked_mul(std::mem::size_of::<T>()).ok_or(DbError::LimitExceeded(label))?;
    let reserved = DATABASE_SYNC_HELLO_MAX_BYTES.checked_sub(ledger.bytes).ok_or(DbError::LimitExceeded(label))?;
    if requested > reserved {
        return Err(DbError::LimitExceeded(label));
    }
    ledger.observe(1, reserved, "database sync hello cumulative envelope backing")?;
    let mut owner = Vec::new();
    if owner.try_reserve_exact(count).is_err() {
        ledger.release(1, reserved)?;
        return Err(DbError::LimitExceeded(label));
    }
    let actual = owner.capacity().checked_mul(std::mem::size_of::<T>()).ok_or(DbError::LimitExceeded(label))?;
    if actual > reserved {
        drop(owner);
        ledger.release(1, reserved)?;
        return Err(DbError::LimitExceeded(label));
    }
    ledger.settle_allocation(reserved, actual, label)?;
    Ok(owner)
}

fn database_sync_hello_retire_vec<T>(owner: &mut Vec<T>, ledger: &mut DatabaseSyncHelloBackingLedger) -> Result<(), DbError> {
    let capacity = owner.capacity().checked_mul(std::mem::size_of::<T>()).ok_or(DbError::LimitExceeded("database sync hello retirement capacity"))?;
    if capacity == 0 {
        return Ok(());
    }
    drop(std::mem::take(owner));
    ledger.release(1, capacity)
}

#[derive(Default)]
struct DatabaseSyncHelloEnvelopeBuilder {
    mutation_id: String,
    document_id: String,
    actor: String,
    dependencies: Vec<protocol::MutationId>,
    diff_schema: String,
    diff_payload: Vec<u8>,
    inverse_schema: String,
    inverse_payload: Vec<u8>,
}

impl DatabaseSyncHelloEnvelopeBuilder {
    fn close_one(&mut self, ledger: &mut DatabaseSyncHelloBackingLedger) -> Result<bool, DbError> {
        if let Some(mut owner) = self.dependencies.pop() {
            let capacity = owner.0.capacity();
            drop(std::mem::take(&mut owner.0));
            ledger.release(usize::from(capacity != 0), capacity)?;
            return Ok(true);
        }
        if self.dependencies.capacity() != 0 {
            database_sync_hello_retire_vec(&mut self.dependencies, ledger)?;
            return Ok(true);
        }
        for owner in [&mut self.mutation_id, &mut self.document_id, &mut self.actor, &mut self.diff_schema, &mut self.inverse_schema] {
            if owner.capacity() != 0 {
                let capacity = owner.capacity();
                drop(std::mem::take(owner));
                ledger.release(1, capacity)?;
                return Ok(true);
            }
        }
        if self.diff_payload.capacity() != 0 {
            database_sync_hello_retire_vec(&mut self.diff_payload, ledger)?;
            return Ok(true);
        }
        if self.inverse_payload.capacity() != 0 {
            database_sync_hello_retire_vec(&mut self.inverse_payload, ledger)?;
            return Ok(true);
        }
        Ok(false)
    }

    fn finish(self, timestamp: protocol::HybridLogicalTimestamp) -> protocol::MutationEnvelope {
        protocol::MutationEnvelope {
            mutation_id: protocol::MutationId(self.mutation_id),
            document_id: protocol::ArtifactId(self.document_id),
            actor: protocol::ActorId(self.actor),
            dependencies: self.dependencies,
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId(self.diff_schema), payload: self.diff_payload },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId(self.inverse_schema), payload: self.inverse_payload },
            timestamp,
        }
    }
}

async fn database_sync_hello_decode_text(
    cursor: &mut db_wal::WalBytesCursor<'_>,
    control: &mut db_wal::WalCursorControl,
    ledger: &mut DatabaseSyncHelloBackingLedger,
    cancelled: &std::sync::atomic::AtomicBool,
    expired: &std::sync::atomic::AtomicBool,
) -> Result<String, DbError> {
    let mut remaining = cursor.begin_field(4_096, control)?;
    let mut output = database_sync_hello_allocate_envelope_vec::<u8>(ledger, remaining)?;
    let mut fragment = [0u8; 1_024];
    while remaining != 0 {
        let copied = match cursor.read_field_fragment(&mut remaining, &mut fragment, control) {
            Ok(copied) => copied,
            Err(error) => {
                database_sync_hello_retire_vec(&mut output, ledger)?;
                return Err(error);
            }
        };
        output.extend_from_slice(&fragment[..copied]);
        if let Err(error) = database_sync_hello_opportunity(cancelled, expired).await {
            database_sync_hello_retire_vec(&mut output, ledger)?;
            return Err(error);
        }
    }
    match String::from_utf8(output) {
        Ok(owner) => Ok(owner),
        Err(error) => {
            let mut owner = error.into_bytes();
            database_sync_hello_retire_vec(&mut owner, ledger)?;
            Err(DbError::Corrupt("database sync hello WAL text is not valid utf-8".to_string()))
        }
    }
}

async fn database_sync_hello_decode_payload(
    cursor: &mut db_wal::WalBytesCursor<'_>,
    control: &mut db_wal::WalCursorControl,
    ledger: &mut DatabaseSyncHelloBackingLedger,
    cancelled: &std::sync::atomic::AtomicBool,
    expired: &std::sync::atomic::AtomicBool,
) -> Result<Vec<u8>, DbError> {
    let mut remaining = cursor.begin_field(DATABASE_SYNC_HELLO_MAX_BYTES as u64, control)?;
    let mut output = database_sync_hello_allocate_envelope_vec::<u8>(ledger, remaining)?;
    let mut fragment = [0u8; 4_096];
    while remaining != 0 {
        let copied = match cursor.read_field_fragment(&mut remaining, &mut fragment, control) {
            Ok(copied) => copied,
            Err(error) => {
                database_sync_hello_retire_vec(&mut output, ledger)?;
                return Err(error);
            }
        };
        output.extend_from_slice(&fragment[..copied]);
        if let Err(error) = database_sync_hello_opportunity(cancelled, expired).await {
            database_sync_hello_retire_vec(&mut output, ledger)?;
            return Err(error);
        }
    }
    Ok(output)
}

async fn database_sync_hello_decode_envelope(
    bytes: &db_wal::WalBytes,
    control: &mut db_wal::WalCursorControl,
    ledger: &mut DatabaseSyncHelloBackingLedger,
    cancelled: &std::sync::atomic::AtomicBool,
    expired: &std::sync::atomic::AtomicBool,
) -> Result<protocol::MutationEnvelope, DbError> {
    let mut cursor = bytes.cursor();
    let mut owner = DatabaseSyncHelloEnvelopeBuilder::default();
    let decoded = async {
        owner.mutation_id = database_sync_hello_decode_text(&mut cursor, control, ledger, cancelled, expired).await?;
        owner.document_id = database_sync_hello_decode_text(&mut cursor, control, ledger, cancelled, expired).await?;
        owner.actor = database_sync_hello_decode_text(&mut cursor, control, ledger, cancelled, expired).await?;
        let count = cursor.varint(control)?;
        check_len(count, DATABASE_SYNC_HELLO_MAX_ITEMS as u64, "database sync hello WAL dependencies")?;
        owner.dependencies = database_sync_hello_allocate_envelope_vec::<protocol::MutationId>(ledger, count as usize)?;
        for _ in 0..count {
            let dependency = database_sync_hello_decode_text(&mut cursor, control, ledger, cancelled, expired).await?;
            owner.dependencies.push(protocol::MutationId(dependency));
        }
        owner.diff_schema = database_sync_hello_decode_text(&mut cursor, control, ledger, cancelled, expired).await?;
        owner.diff_payload = database_sync_hello_decode_payload(&mut cursor, control, ledger, cancelled, expired).await?;
        owner.inverse_schema = database_sync_hello_decode_text(&mut cursor, control, ledger, cancelled, expired).await?;
        owner.inverse_payload = database_sync_hello_decode_payload(&mut cursor, control, ledger, cancelled, expired).await?;
        let timestamp = protocol::HybridLogicalTimestamp { actor: cursor.varint(control)?, physical_ms: cursor.varint(control)?, logical: cursor.varint(control)? };
        if cursor.remaining() != 0 {
            return Err(DbError::Corrupt("database sync hello WAL command has trailing bytes".to_string()));
        }
        Ok(timestamp)
    }
    .await;
    match decoded {
        Ok(timestamp) => Ok(owner.finish(timestamp)),
        Err(error) => {
            let mut control_error = None;
            while owner.close_one(ledger)? {
                semio_framework_async::yield_once().await;
                if control_error.is_none() {
                    control_error = database_sync_hello_control(cancelled, expired).err();
                }
            }
            Err(control_error.unwrap_or(error))
        }
    }
}

fn database_sync_hello_envelope_credit(envelope: &protocol::MutationEnvelope) -> Result<(usize, usize), DbError> {
    let items = 7usize.checked_add(envelope.dependencies.len()).ok_or(DbError::LimitExceeded("database sync hello envelope items"))?;
    let dependency_shell = envelope.dependencies.capacity().checked_mul(std::mem::size_of::<protocol::MutationId>()).ok_or(DbError::LimitExceeded("database sync hello dependency shell"))?;
    let mut bytes = envelope.mutation_id.0.capacity();
    for backing in [envelope.document_id.0.capacity(), envelope.actor.0.capacity(), dependency_shell, envelope.diff.schema.0.capacity(), envelope.diff.payload.capacity(), envelope.inverse.schema.0.capacity(), envelope.inverse.payload.capacity()] {
        bytes = bytes.checked_add(backing).ok_or(DbError::LimitExceeded("database sync hello envelope bytes"))?;
    }
    for dependency in &envelope.dependencies {
        bytes = bytes.checked_add(dependency.0.capacity()).ok_or(DbError::LimitExceeded("database sync hello dependency bytes"))?;
    }
    Ok((items, bytes))
}

async fn database_sync_hello_close_pages(pages: &mut db_storage::DbIoPages, cancelled: &std::sync::atomic::AtomicBool, expired: &std::sync::atomic::AtomicBool) -> Result<(), DbError> {
    let mut control_error = None;
    while pages.close_step()?.is_some() {
        semio_framework_async::yield_once().await;
        if control_error.is_none() {
            control_error = database_sync_hello_control(cancelled, expired).err();
        }
    }
    control_error.map_or(Ok(()), Err)
}

async fn replay_sync_state_retained(
    storage: &db_storage::DbBackend,
    document: ArtifactId,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    expired: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ledger: &mut DatabaseSyncHelloBackingLedger,
    progress: &std::sync::atomic::AtomicU8,
) -> Result<ArtifactSyncState, DbError> {
    progress.store(DatabaseSyncHelloProgress::Replay as u8, std::sync::atomic::Ordering::Release);
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS);
    let control = db_wal::WalCursorControl::new(cancelled.clone(), deadline, DATABASE_SYNC_HELLO_MAX_ITEMS)?;
    database_sync_hello_control(&cancelled, &expired)?;
    let wal = storage.wal().await;
    database_sync_hello_control(&cancelled, &expired)?;
    let mut records = db_wal::replay_document(&wal, &document, control).await?;
    let mut decode_control = db_wal::WalCursorControl::new(cancelled.clone(), deadline, DATABASE_SYNC_HELLO_MAX_ITEMS)?;
    let mut commands = database_sync_hello_allocate_vec::<protocol::MutationEnvelope>(ledger, DATABASE_SYNC_HELLO_MAX_ITEMS, "database sync hello command shell")?;
    let mut chain = blake3::Hasher::new();
    let mut commit_seq = 0u64;
    let mut floor_head_seq = 0u64;
    let replay = async {
        loop {
            let deadline = std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS);
            records.replenish(deadline, DATABASE_SYNC_HELLO_MAX_ITEMS)?;
            decode_control.replenish(deadline, DATABASE_SYNC_HELLO_MAX_ITEMS)?;
            database_sync_hello_control(&cancelled, &expired)?;
            let mut record = match records.next_step().await? {
                db_wal::WalReplayStep::Record(record) => record,
                db_wal::WalReplayStep::Yield => {
                    database_sync_hello_opportunity(&cancelled, &expired).await?;
                    continue;
                }
                db_wal::WalReplayStep::Done => break,
            };
            match &mut record {
                db_wal::WalRecord::Command(bytes) => {
                    progress.store(DatabaseSyncHelloProgress::Decode as u8, std::sync::atomic::Ordering::Release);
                    let envelope = database_sync_hello_decode_envelope(bytes, &mut decode_control, ledger, &cancelled, &expired).await?;
                    commands.push(envelope);
                    chain.update(&bytes.hash());
                }
                db_wal::WalRecord::TxCommit { .. } => commit_seq = commit_seq.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello commit sequence"))?,
                db_wal::WalRecord::SnapshotPub { frontier, .. } => floor_head_seq = frontier.head_seq,
                _ => {}
            }
            let _ = record.close_step()?;
            database_sync_hello_opportunity(&cancelled, &expired).await?;
        }
        records.replenish(std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS), DATABASE_SYNC_HELLO_MAX_ITEMS)?;
        database_sync_hello_control(&cancelled, &expired)?;
        while records.close_step().await? {
            records.replenish(std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS), DATABASE_SYNC_HELLO_MAX_ITEMS)?;
            database_sync_hello_opportunity(&cancelled, &expired).await?;
            database_sync_hello_control(&cancelled, &expired)?;
        }
        Ok::<(), DbError>(())
    }
    .await;
    if let Err(error) = replay {
        let mut control_error = None;
        while let Some(envelope) = commands.pop() {
            let mut close = DatabaseSyncHelloEnvelopeClose { owner: Some(envelope) };
            while close.close_one() {
                semio_framework_async::yield_once().await;
                if control_error.is_none() {
                    control_error = database_sync_hello_control(&cancelled, &expired).err();
                }
            }
        }
        database_sync_hello_retire_vec(&mut commands, ledger)?;
        return Err(control_error.unwrap_or(error));
    }
    let head_seq = u64::try_from(commands.len()).map_err(|_| DbError::LimitExceeded("database sync hello head sequence"))?;
    let chain_hash = if commands.is_empty() { [0; 32] } else { *chain.finalize().as_bytes() };
    Ok(ArtifactSyncState { frontier: Frontier { document, head_seq, commit_seq, chain_hash, epoch: 0 }, commands, floor_head_seq })
}

enum DatabaseSyncHelloFollowUp {
    None,
    Tail { envelopes: Option<Vec<protocol::MutationEnvelope>>, closing: Option<DatabaseSyncHelloEnvelopeClose>, origin: Option<protocol::ActorId>, frontier: Option<protocol::RuntimeFrontierSummary> },
    Snapshot { pages: db_storage::DbIoPages, chunk_bytes: usize, offset: usize, page: u8, page_offset: usize, seq: u32, chunk: Option<protocol::SnapshotChunkBytes>, done: bool },
}

struct DatabaseSyncHelloGrant {
    deadline: std::time::Instant,
    checks: u8,
    forced_expiry_check: u8,
}

impl DatabaseSyncHelloGrant {
    fn fresh() -> Result<Self, DbError> {
        let deadline = std::time::Instant::now().checked_add(std::time::Duration::from_millis(DATABASE_SYNC_HELLO_TURN_MS)).ok_or(DbError::LimitExceeded("database sync hello grant deadline"))?;
        Ok(Self { deadline, checks: 0, forced_expiry_check: 0 })
    }

    #[cfg(test)]
    fn expiring_at(check: u8) -> Self {
        Self { deadline: std::time::Instant::now() + std::time::Duration::from_secs(1), checks: 0, forced_expiry_check: check }
    }

    fn check(&mut self, cancelled: &std::sync::atomic::AtomicBool, expired: &std::sync::atomic::AtomicBool) -> Result<(), DbError> {
        database_sync_hello_control(cancelled, expired)?;
        self.checks = self.checks.saturating_add(1);
        if self.forced_expiry_check != 0 && self.checks >= self.forced_expiry_check || std::time::Instant::now() >= self.deadline {
            return Err(DbError::Timeout("database sync hello 8 ms grant"));
        }
        Ok(())
    }
}

impl DatabaseSyncHelloFollowUp {
    fn drive_one(&mut self, ledger: &mut DatabaseSyncHelloBackingLedger, cancelled: &std::sync::atomic::AtomicBool, expired: &std::sync::atomic::AtomicBool) -> Result<Option<Option<protocol::ServerFrame>>, DbError> {
        let mut grant = DatabaseSyncHelloGrant::fresh()?;
        self.drive_one_with_grant(ledger, cancelled, expired, &mut grant)
    }

    fn drive_one_with_grant(
        &mut self,
        ledger: &mut DatabaseSyncHelloBackingLedger,
        cancelled: &std::sync::atomic::AtomicBool,
        expired: &std::sync::atomic::AtomicBool,
        grant: &mut DatabaseSyncHelloGrant,
    ) -> Result<Option<Option<protocol::ServerFrame>>, DbError> {
        grant.check(cancelled, expired)?;
        match self {
            Self::None => Ok(Some(None)),
            Self::Tail { envelopes, closing, origin, frontier } => {
                if closing.is_some() {
                    return Err(DbError::Closed);
                }
                grant.check(cancelled, expired)?;
                let Some(envelopes) = envelopes.take() else { return Ok(Some(None)) };
                let origin = origin.take().ok_or_else(|| DbError::Internal("database sync hello tail origin missing".to_string()))?;
                let frontier = frontier.take().ok_or_else(|| DbError::Internal("database sync hello tail frontier missing".to_string()))?;
                Ok(Some(Some(protocol::ServerFrame::Commands { envelopes, origin, frontier })))
            }
            Self::Snapshot { pages, chunk_bytes, offset, page, page_offset, seq, chunk, done } => {
                if *done {
                    return Ok(Some(None));
                }
                if *offset == pages.len() {
                    grant.check(cancelled, expired)?;
                    *done = true;
                    return Ok(Some(Some(protocol::ServerFrame::SnapshotDone { seq_count: *seq })));
                }
                let unit_bytes = (*chunk_bytes).min(DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES);
                if chunk.is_none() {
                    grant.check(cancelled, expired)?;
                    let len = unit_bytes.min(pages.len() - *offset);
                    let reservation = database_sync_hello_reserve_snapshot_chunk(ledger, len)?;
                    let owner = reservation.allocate(ledger)?;
                    *chunk = Some(owner);
                }
                grant.check(cancelled, expired)?;
                let fragment = pages.page(*page).ok_or_else(|| DbError::Corrupt("database sync hello snapshot page missing".to_string()))?;
                let target = chunk.as_mut().ok_or_else(|| DbError::Internal("database sync hello snapshot chunk owner missing".to_string()))?;
                let remaining = unit_bytes.saturating_sub(target.len()).min(pages.len() - *offset);
                let copied = remaining.min(fragment.len().saturating_sub(*page_offset));
                if copied == 0 {
                    return Err(DbError::Corrupt("database sync hello snapshot cursor stalled".to_string()));
                }
                grant.check(cancelled, expired)?;
                if !target.try_extend_from_slice(&fragment[*page_offset..*page_offset + copied]) {
                    return Err(DbError::LimitExceeded("database sync hello fixed snapshot chunk copy"));
                }
                *offset += copied;
                *page_offset += copied;
                if *page_offset == fragment.len() {
                    *page = page.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello snapshot page cursor"))?;
                    *page_offset = 0;
                }
                if target.len() == unit_bytes || *offset == pages.len() {
                    grant.check(cancelled, expired)?;
                    let next_seq = seq.checked_add(1).ok_or(DbError::LimitExceeded("database sync hello snapshot sequence"))?;
                    let bytes = chunk.take().ok_or_else(|| DbError::Internal("database sync hello completed chunk missing".to_string()))?;
                    let frame = protocol::ServerFrame::SnapshotChunk { seq: *seq, bytes };
                    *seq = next_seq;
                    return Ok(Some(Some(frame)));
                }
                Ok(None)
            }
        }
    }

    fn close_one(&mut self) -> Result<bool, DbError> {
        match self {
            Self::None => Ok(false),
            Self::Tail { envelopes, closing, origin, frontier } => {
                if let Some(cursor) = closing.as_mut() {
                    let pending = cursor.close_one();
                    if cursor.terminal_is_empty() {
                        *closing = None;
                    }
                    return Ok(pending);
                }
                if let Some(envelopes) = envelopes.as_mut() {
                    if let Some(envelope) = envelopes.pop() {
                        *closing = Some(DatabaseSyncHelloEnvelopeClose { owner: Some(envelope) });
                        return Ok(true);
                    }
                    if envelopes.capacity() != 0 {
                        drop(std::mem::take(envelopes));
                        return Ok(true);
                    }
                    *envelopes = Vec::new();
                    return Ok(true);
                }
                if let Some(owner) = origin.as_mut() {
                    if database_sync_hello_retire_string(&mut owner.0) {
                        return Ok(true);
                    }
                    *origin = None;
                    return Ok(true);
                }
                if let Some(owner) = frontier.as_mut() {
                    if database_sync_hello_retire_string(&mut owner.document_id.0) || database_sync_hello_retire_string(&mut owner.head_edit_id) {
                        return Ok(true);
                    }
                    *frontier = None;
                    return Ok(true);
                }
                Ok(false)
            }
            Self::Snapshot { pages, chunk, .. } => {
                if let Some(owner) = chunk.as_mut() {
                    if owner.close_one() {
                        return Ok(true);
                    }
                    *chunk = None;
                    return Ok(true);
                }
                Ok(pages.close_step()?.is_some())
            }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        match self {
            Self::None => true,
            Self::Tail { envelopes, closing, origin, frontier } => envelopes.as_ref().is_none_or(|owners| owners.is_empty() && owners.capacity() == 0) && closing.is_none() && origin.is_none() && frontier.is_none(),
            Self::Snapshot { pages, chunk, .. } => pages.terminal_is_empty() && chunk.is_none(),
        }
    }
}

struct DatabaseSyncHelloPrepared {
    welcome: Option<protocol::ServerFrame>,
    welcome_close: Option<DatabaseSyncHelloFrameClose>,
    follow_up: DatabaseSyncHelloFollowUp,
}

struct DatabaseSyncHelloExecution {
    owners: Option<DatabaseSyncHelloOwners>,
    prepared: Result<DatabaseSyncHelloPrepared, DbError>,
    ledger: Option<DatabaseSyncHelloBackingLedger>,
    close_fault: Option<DatabaseSyncHelloFollowUpCloseFault>,
}

async fn database_sync_hello_execute(
    mut owners: DatabaseSyncHelloOwners,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    expired: std::sync::Arc<std::sync::atomic::AtomicBool>,
    progress: std::sync::Arc<std::sync::atomic::AtomicU8>,
) -> DatabaseSyncHelloExecution {
    let mut ledger = DatabaseSyncHelloBackingLedger::default();
    let run = async {
        if owners.snapshot_chunk_bytes == 0 {
            return Err(DbError::InvalidArgument("snapshot_chunk_bytes must be non-zero".to_string()));
        }
        let (input_items, input_bytes) = database_sync_hello_input_credit(&owners)?;
        ledger.observe(input_items, input_bytes, "database sync hello cumulative input backing")?;
        database_sync_hello_opportunity(&cancelled, &expired).await?;
        let document = ArtifactId(std::mem::take(&mut owners.document.0));
        database_sync_hello_control(&cancelled, &expired)?;
        let mut state = replay_sync_state_retained(owners.storage()?, document, cancelled.clone(), expired.clone(), &mut ledger, &progress).await?;
        database_sync_hello_opportunity(&cancelled, &expired).await?;
        let replica = owners.hello_frontier.as_ref();
        if let Some(replica) = replica.as_ref() {
            if replica.document_id.0 != state.frontier.document.0 {
                return Err(DbError::InvalidArgument("database sync hello frontier document mismatch".to_string()));
            }
            if replica.head_edit_ordinal > state.frontier.head_seq {
                return Err(DbError::InvalidArgument("database sync hello frontier ahead of server".to_string()));
            }
        }
        progress.store(DatabaseSyncHelloProgress::Bootstrap as u8, std::sync::atomic::Ordering::Release);
        let replica_head = replica.as_ref().map_or(0, |frontier| frontier.head_edit_ordinal);
        let head_edit_id = match state.commands.last() {
            Some(envelope) => database_sync_hello_clone_string(&envelope.mutation_id.0, &mut ledger, "database sync hello server frontier head backing")?,
            None => String::new(),
        };
        let (bootstrap, follow_up) = if replica_head >= state.floor_head_seq {
            let start = usize::try_from(replica_head).map_err(|_| DbError::LimitExceeded("database sync hello tail cursor"))?;
            let missing = state.commands.len().saturating_sub(start);
            let mut envelopes = database_sync_hello_allocate_vec::<protocol::MutationEnvelope>(&mut ledger, missing, "database sync hello tail shell backing")?;
            for (index, envelope) in std::mem::take(&mut state.commands).into_iter().enumerate() {
                if index >= start {
                    envelopes.push(envelope);
                } else {
                    let mut close = DatabaseSyncHelloEnvelopeClose { owner: Some(envelope) };
                    while close.close_one() {
                        semio_framework_async::yield_once().await;
                        database_sync_hello_control(&cancelled, &expired)?;
                    }
                }
                database_sync_hello_opportunity(&cancelled, &expired).await?;
            }
            if envelopes.is_empty() {
                (protocol::Bootstrap::None, DatabaseSyncHelloFollowUp::None)
            } else {
                let frontier_document = database_sync_hello_clone_string(&state.frontier.document.0, &mut ledger, "database sync hello tail frontier document backing")?;
                let frontier_head = database_sync_hello_clone_string(&head_edit_id, &mut ledger, "database sync hello tail frontier head backing")?;
                let frontier = protocol::RuntimeFrontierSummary {
                    document_id: protocol::ArtifactId(frontier_document),
                    head_edit_ordinal: state.frontier.head_seq,
                    head_edit_id: frontier_head,
                    last_commit_seq: state.frontier.commit_seq,
                    chain_hash: state.frontier.chain_hash,
                };
                let origin = protocol::ActorId(std::mem::take(&mut owners.origin.0));
                (protocol::Bootstrap::Tail, DatabaseSyncHelloFollowUp::Tail { envelopes: Some(envelopes), closing: None, origin: Some(origin), frontier: Some(frontier) })
            }
        } else {
            database_sync_hello_control(&cancelled, &expired)?;
            let snapshots = owners.storage()?.snapshot().await;
            database_sync_hello_control(&cancelled, &expired)?;
            let generation = snapshots.latest_generation(&state.frontier.document).await?.ok_or_else(|| DbError::Unavailable("database sync hello snapshot generation unavailable".to_string()))?;
            database_sync_hello_control(&cancelled, &expired)?;
            let page_reservation = database_sync_hello_reserve_snapshot_pages(&mut ledger)?;
            let mut pages = match snapshots.read_generation(&state.frontier.document, generation).await {
                Ok(pages) => pages,
                Err(error) => {
                    page_reservation.release(&mut ledger)?;
                    return Err(error);
                }
            };
            if let Err(error) = database_sync_hello_control(&cancelled, &expired) {
                database_sync_hello_close_pages(&mut pages, &cancelled, &expired).await?;
                page_reservation.release(&mut ledger)?;
                return Err(error);
            }
            let (page_items, page_bytes) = match page_reservation.observed(&pages) {
                Ok(observed) => observed,
                Err(error) => {
                    database_sync_hello_close_pages(&mut pages, &cancelled, &expired).await?;
                    page_reservation.release(&mut ledger)?;
                    return Err(error);
                }
            };
            page_reservation.settle(&mut ledger, page_items, page_bytes)?;
            let mut hash = blake3::Hasher::new();
            for page in 0..pages.page_count() {
                let Some(fragment) = pages.page(page) else {
                    database_sync_hello_close_pages(&mut pages, &cancelled, &expired).await?;
                    ledger.release(page_items, page_bytes)?;
                    return Err(DbError::Corrupt("database sync hello snapshot hash page missing".to_string()));
                };
                hash.update(fragment);
                if let Err(error) = database_sync_hello_opportunity(&cancelled, &expired).await {
                    database_sync_hello_close_pages(&mut pages, &cancelled, &expired).await?;
                    ledger.release(page_items, page_bytes)?;
                    return Err(error);
                }
            }
            database_sync_hello_control(&cancelled, &expired)?;
            let observed_generation = match snapshots.latest_generation(&state.frontier.document).await {
                Ok(generation) => generation,
                Err(error) => {
                    database_sync_hello_close_pages(&mut pages, &cancelled, &expired).await?;
                    ledger.release(page_items, page_bytes)?;
                    return Err(error);
                }
            };
            if observed_generation != Some(generation) {
                database_sync_hello_close_pages(&mut pages, &cancelled, &expired).await?;
                ledger.release(page_items, page_bytes)?;
                return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(generation), actual: crate::db_ids::GenerationId(observed_generation.unwrap_or(0)) });
            }
            let pack_hash = *hash.finalize().as_bytes();
            (protocol::Bootstrap::Snapshot { pack_hash, inline: None }, DatabaseSyncHelloFollowUp::Snapshot { pages, chunk_bytes: owners.snapshot_chunk_bytes, offset: 0, page: 0, page_offset: 0, seq: 0, chunk: None, done: false })
        };
        progress.store(DatabaseSyncHelloProgress::Welcome as u8, std::sync::atomic::Ordering::Release);
        database_sync_hello_control(&cancelled, &expired)?;
        let resume_reservation = ledger.reserve_allocation(1, 0, "database sync hello resume token backing")?;
        let mut resume_token = match issue_resume_token(&state.frontier).await {
            Ok(owner) => owner,
            Err(error) => {
                ledger.release(1, resume_reservation)?;
                return Err(error);
            }
        };
        if resume_token.capacity() > resume_reservation {
            drop(std::mem::take(&mut resume_token));
            ledger.release(1, resume_reservation)?;
            return Err(DbError::LimitExceeded("database sync hello resume token observed backing"));
        }
        ledger.settle_allocation(resume_reservation, resume_token.capacity(), "database sync hello resume token backing")?;
        let server_frontier = protocol::RuntimeFrontierSummary {
            document_id: protocol::ArtifactId(std::mem::take(&mut state.frontier.document.0)),
            head_edit_ordinal: state.frontier.head_seq,
            head_edit_id,
            last_commit_seq: state.frontier.commit_seq,
            chain_hash: state.frontier.chain_hash,
        };
        let session_id = std::mem::take(&mut owners.session_id);
        let welcome = protocol::ServerFrame::Welcome { session_id, resume_token, server_frontier, bootstrap };
        Ok(DatabaseSyncHelloPrepared { welcome: Some(welcome), welcome_close: None, follow_up })
    }
    .await;
    let terminal = match &run {
        Ok(_) => DatabaseSyncHelloProgress::Streaming,
        Err(DbError::Closed) | Err(DbError::Timeout(_)) => DatabaseSyncHelloProgress::Cancelled,
        Err(_) => DatabaseSyncHelloProgress::Fault,
    };
    progress.store(terminal as u8, std::sync::atomic::Ordering::Release);
    DatabaseSyncHelloExecution { owners: Some(owners), prepared: run, ledger: Some(ledger), close_fault: None }
}

type DatabaseSyncHelloExecutionFuture = std::pin::Pin<Box<dyn std::future::Future<Output = DatabaseSyncHelloExecution> + Send + 'static>>;

struct DatabaseSyncHelloFollowUpCloseFault {
    error: Option<DbError>,
    attempts: u8,
}

impl DatabaseSyncHelloFollowUpCloseFault {
    fn retain(&mut self, error: DbError) {
        if self.error.is_none() {
            self.error = Some(error);
        }
        self.attempts = self.attempts.saturating_add(1);
    }

    fn close_one(&mut self) -> bool {
        self.error.take().is_some()
    }

    fn terminal_is_empty(&self) -> bool {
        self.error.is_none()
    }
}

fn database_sync_hello_apply_follow_up_close_result(fault: &mut Option<DatabaseSyncHelloFollowUpCloseFault>, terminal_is_empty: bool, result: Result<bool, DbError>) -> bool {
    match result {
        Ok(pending) => pending || !terminal_is_empty,
        Err(error) => {
            let mut retained = DatabaseSyncHelloFollowUpCloseFault { error: None, attempts: 0 };
            retained.retain(error);
            *fault = Some(retained);
            true
        }
    }
}

struct DatabaseSyncHelloQuarantineClose {
    future: Option<DatabaseSyncHelloExecutionFuture>,
    items: usize,
    bytes: usize,
}

struct DatabaseSyncHelloReturnedFrameLease {
    generation: u64,
    items: usize,
    bytes: usize,
    close: Option<DatabaseSyncHelloFrameClose>,
}

impl DatabaseSyncHelloQuarantineClose {
    fn close_one(&mut self) -> bool {
        if let Some(future) = self.future.take() {
            drop(future);
            self.items = self.items.saturating_sub(1);
            self.bytes = 0;
            return true;
        }
        false
    }

    fn terminal_is_empty(&self) -> bool {
        self.future.is_none() && self.items == 0 && self.bytes == 0
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum DatabaseSyncHelloDriverAuthority {
    Idle,
    Queued,
    Driving,
    Retry,
}

struct DatabaseSyncHelloCore {
    future: Option<DatabaseSyncHelloExecutionFuture>,
    execution: Option<DatabaseSyncHelloExecution>,
    frame: Option<Result<Option<protocol::ServerFrame>, DbError>>,
    frame_close: Option<DatabaseSyncHelloFrameClose>,
    returned_frame: Option<DatabaseSyncHelloReturnedFrameLease>,
    returned_fallback: Option<DatabaseSyncHelloReturnedFrameLease>,
    quarantined: Option<DatabaseSyncHelloQuarantineClose>,
}

struct DatabaseSyncHelloState {
    pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    slot: usize,
    generation: u64,
    admission: std::sync::Mutex<Option<DatabaseSyncHelloAdmission>>,
    core: std::sync::Mutex<DatabaseSyncHelloCore>,
    driver: std::sync::atomic::AtomicU8,
    retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    expired: std::sync::Arc<std::sync::atomic::AtomicBool>,
    deadline_ms: u64,
    progress: std::sync::Arc<std::sync::atomic::AtomicU8>,
    wake_requested: std::sync::atomic::AtomicBool,
    demand: std::sync::atomic::AtomicBool,
    abandoned: std::sync::atomic::AtomicBool,
    close_armed: std::sync::atomic::AtomicBool,
    close_requested: std::sync::atomic::AtomicBool,
    waker: std::sync::Mutex<Option<std::task::Waker>>,
    returned_generation: std::sync::atomic::AtomicU64,
}

fn database_sync_hello_registry() -> &'static std::sync::Mutex<[Option<std::sync::Arc<DatabaseSyncHelloState>>; DATABASE_SYNC_HELLO_SLOTS]> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<[Option<std::sync::Arc<DatabaseSyncHelloState>>; DATABASE_SYNC_HELLO_SLOTS]>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::array::from_fn(|_| None)))
}

impl std::task::Wake for DatabaseSyncHelloState {
    fn wake(self: std::sync::Arc<Self>) {
        self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        self.schedule();
    }

    fn wake_by_ref(self: &std::sync::Arc<Self>) {
        self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        self.schedule();
    }
}

impl DatabaseSyncHelloState {
    fn current(&self) -> bool {
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseSyncHelloAdmission::is_current)
    }

    fn schedule(self: &std::sync::Arc<Self>) {
        if self.driver.compare_exchange(DatabaseSyncHelloDriverAuthority::Idle as u8, DatabaseSyncHelloDriverAuthority::Queued as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.submit_exact(Box::new(move || state.drive_one()), 0);
    }

    fn submit_exact(self: &std::sync::Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(semio_framework_async::Lane::Io, job) {
            Ok(()) => {}
            Err(error) => {
                let next = attempt.saturating_add(1).min(DATABASE_SYNC_HELLO_RETRY_LIMIT);
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), next));
                if self.driver.compare_exchange(DatabaseSyncHelloDriverAuthority::Queued as u8, DatabaseSyncHelloDriverAuthority::Retry as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
                    let state = self.clone();
                    self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry());
                }
            }
        }
    }

    fn retry(self: std::sync::Arc<Self>) {
        let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return };
        if self.pool.now_ms() >= self.deadline_ms || attempt >= DATABASE_SYNC_HELLO_RETRY_LIMIT {
            self.expired.store(true, std::sync::atomic::Ordering::Release);
            self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        }
        if self.driver.compare_exchange(DatabaseSyncHelloDriverAuthority::Retry as u8, DatabaseSyncHelloDriverAuthority::Queued as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
            self.submit_exact(job, attempt);
        } else {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
        }
    }

    fn drive_one(self: std::sync::Arc<Self>) {
        if self.driver.compare_exchange(DatabaseSyncHelloDriverAuthority::Queued as u8, DatabaseSyncHelloDriverAuthority::Driving as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        let returned = self.close_returned_frame_one();
        let closing = self.close_requested.load(std::sync::atomic::Ordering::Acquire) && self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).future.is_none();
        let pending = returned.unwrap_or_else(|| if closing { self.close_one_claimed() } else { self.poll_one() });
        let wake = self.wake_requested.swap(false, std::sync::atomic::Ordering::AcqRel);
        self.driver.store(DatabaseSyncHelloDriverAuthority::Idle as u8, std::sync::atomic::Ordering::Release);
        if pending || wake || self.demand.load(std::sync::atomic::Ordering::Acquire) {
            self.schedule();
        }
    }

    fn close_returned_frame_one(&self) -> Option<bool> {
        let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let fallback = core.returned_fallback.is_some();
        let lease = if fallback { core.returned_fallback.as_mut()? } else { core.returned_frame.as_mut()? };
        let close = lease.close.as_mut()?;
        let pending = close.close_one();
        if !close.terminal_is_empty() {
            return Some(pending);
        }
        let items = lease.items;
        let bytes = lease.bytes;
        let generation = lease.generation;
        let Some(execution) = core.execution.as_mut() else {
            return Some(true);
        };
        let Some(ledger) = execution.ledger.as_mut() else {
            return Some(true);
        };
        if ledger.release(items, bytes).is_err() {
            return Some(true);
        }
        if fallback {
            if core.returned_fallback.as_ref().is_some_and(|owner| owner.generation == generation) {
                core.returned_fallback.take();
            }
        } else if core.returned_frame.as_ref().is_some_and(|owner| owner.generation == generation) {
            core.returned_frame.take();
        }
        drop(core);
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
        Some(false)
    }

    fn poll_one(self: &std::sync::Arc<Self>) -> bool {
        if !self.current() {
            self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        }
        let future = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).future.take();
        if let Some(mut future) = future {
            let waker = std::task::Waker::from(self.clone());
            let mut context = std::task::Context::from_waker(&waker);
            return match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| future.as_mut().poll(&mut context))) {
                Ok(std::task::Poll::Pending) => {
                    let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    core.future = Some(future);
                    true
                }
                Ok(std::task::Poll::Ready(execution)) => {
                    let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    if self.current() {
                        core.execution = Some(execution);
                    } else {
                        core.execution = Some(DatabaseSyncHelloExecution { owners: execution.owners, prepared: Err(DbError::Closed), ledger: execution.ledger, close_fault: execution.close_fault });
                    }
                    drop(core);
                    if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
                        self.arm_close();
                    }
                    if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        waker.wake();
                    }
                    false
                }
                Err(_) => {
                    let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    let bytes = std::mem::size_of_val(&*future);
                    core.quarantined = Some(DatabaseSyncHelloQuarantineClose { future: Some(future), items: 1, bytes });
                    core.frame = Some(Err(DbError::Internal("database sync hello worker panic".to_string())));
                    self.progress.store(DatabaseSyncHelloProgress::Fault as u8, std::sync::atomic::Ordering::Release);
                    drop(core);
                    self.arm_close();
                    false
                }
            };
        }
        if !self.demand.swap(false, std::sync::atomic::Ordering::AcqRel) {
            return false;
        }
        if let Err(error) = database_sync_hello_control(&self.cancelled, &self.expired) {
            let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            core.frame = Some(Err(error));
            drop(core);
            self.arm_close();
            if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                waker.wake();
            }
            return false;
        }
        let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if core.frame.is_some() || core.returned_frame.is_some() {
            return false;
        }
        let Some(execution) = core.execution.as_mut() else {
            core.frame = Some(Err(DbError::Closed));
            return false;
        };
        let frame = match (&mut execution.prepared, &mut execution.ledger) {
            (Ok(prepared), Some(ledger)) => prepared.follow_up.drive_one(ledger, &self.cancelled, &self.expired),
            (Ok(_), None) => Err(DbError::Internal("database sync hello backing ledger missing".to_string())),
            (Err(_), _) => Ok(Some(None)),
        };
        match frame {
            Ok(Some(frame)) => {
                let terminal = frame.is_none();
                core.frame = Some(Ok(frame));
                if terminal {
                    self.progress.store(DatabaseSyncHelloProgress::Completed as u8, std::sync::atomic::Ordering::Release);
                }
            }
            Ok(None) => self.demand.store(true, std::sync::atomic::Ordering::Release),
            Err(error) => core.frame = Some(Err(error)),
        }
        drop(core);
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
        self.demand.load(std::sync::atomic::Ordering::Acquire)
    }

    fn arm_close(self: &std::sync::Arc<Self>) {
        if self.close_armed.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        self.progress.store(DatabaseSyncHelloProgress::Closing as u8, std::sync::atomic::Ordering::Release);
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || {
            state.close_armed.store(false, std::sync::atomic::Ordering::Release);
            state.close_requested.store(true, std::sync::atomic::Ordering::Release);
            state.schedule();
        });
    }

    fn close_one_claimed(&self) -> bool {
        let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if core.returned_frame.is_some() {
            return false;
        }
        let mut pending = false;
        if let Some(cursor) = core.frame_close.as_mut() {
            pending = cursor.close_one();
            if cursor.terminal_is_empty() {
                core.frame_close = None;
            }
        } else if let Some(frame) = core.frame.take() {
            if let Ok(Some(frame)) = frame {
                core.frame_close = Some(DatabaseSyncHelloFrameClose { owner: Some(frame), envelope: None });
            }
            pending = true;
        }
        if let Some(execution) = core.execution.as_mut() {
            if let Ok(prepared) = execution.prepared.as_mut() {
                if pending {
                } else if let Some(cursor) = prepared.welcome_close.as_mut() {
                    pending = cursor.close_one();
                    if cursor.terminal_is_empty() {
                        prepared.welcome_close = None;
                    }
                } else if let Some(welcome) = prepared.welcome.take() {
                    prepared.welcome_close = Some(DatabaseSyncHelloFrameClose { owner: Some(welcome), envelope: None });
                    pending = true;
                } else if let Some(fault) = execution.close_fault.as_mut() {
                    pending = fault.close_one();
                    if fault.terminal_is_empty() {
                        execution.close_fault = None;
                    }
                } else {
                    let result = prepared.follow_up.close_one();
                    pending = database_sync_hello_apply_follow_up_close_result(&mut execution.close_fault, prepared.follow_up.terminal_is_empty(), result);
                }
            }
            if !pending {
                if let Some(owners) = execution.owners.as_mut() {
                    pending = owners.close_one();
                    if owners.terminal_is_empty() {
                        execution.owners = None;
                    }
                }
            }
            if !pending {
                if let Some(ledger) = execution.ledger.as_mut() {
                    pending = ledger.close_one_credit();
                    if ledger.terminal_is_empty() {
                        execution.ledger = None;
                    }
                }
            }
            if !pending {
                let prepared_terminal = execution.prepared.as_ref().is_err() || execution.prepared.as_ref().is_ok_and(|prepared| prepared.welcome.is_none() && prepared.welcome_close.is_none() && prepared.follow_up.terminal_is_empty());
                pending = !prepared_terminal || execution.owners.is_some() || execution.ledger.is_some() || execution.close_fault.is_some();
            }
        }
        if !pending {
            if let Some(quarantine) = core.quarantined.as_mut() {
                pending = quarantine.close_one();
                if quarantine.terminal_is_empty() {
                    let _ = core.quarantined.take();
                }
            }
        }
        if !pending {
            core.execution = None;
            core.frame = None;
            core.frame_close = None;
        }
        drop(core);
        if !pending {
            let mut registry = database_sync_hello_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if registry.get(self.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.generation) {
                registry[self.slot] = None;
            }
            drop(registry);
            self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.close_requested.store(false, std::sync::atomic::Ordering::Release);
        }
        pending
    }

    fn deadline_callback(self: &std::sync::Arc<Self>) {
        if self.current() && !matches!(self.progress(), DatabaseSyncHelloProgress::Completed | DatabaseSyncHelloProgress::Cancelled | DatabaseSyncHelloProgress::Fault) {
            self.expired.store(true, std::sync::atomic::Ordering::Release);
            self.cancelled.store(true, std::sync::atomic::Ordering::Release);
            self.schedule();
        }
    }

    fn progress(&self) -> DatabaseSyncHelloProgress {
        DatabaseSyncHelloProgress::from_u8(self.progress.load(std::sync::atomic::Ordering::Acquire))
    }
}

/// 👋️ Retained generation-qualified sync-hello future driven only by the shared I/O lane.
pub struct DatabaseSyncHelloFuture {
    state: Option<std::sync::Arc<DatabaseSyncHelloState>>,
    completed: bool,
}

impl std::fmt::Debug for DatabaseSyncHelloFuture {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseSyncHelloFuture").field("generation", &self.state.as_ref().map(|state| state.generation)).field("completed", &self.completed).finish()
    }
}

impl DatabaseSyncHelloFuture {
    pub fn try_submit(
        pool: std::sync::Arc<semio_framework_async::WorkerPool>,
        storage: std::sync::Arc<db_storage::DbBackend>,
        document: ArtifactId,
        hello_frontier: Option<protocol::RuntimeFrontierSummary>,
        session_id: String,
        origin: protocol::ActorId,
        snapshot_chunk_bytes: usize,
    ) -> Result<Self, DatabaseSyncHelloRejected> {
        let owners = DatabaseSyncHelloOwners { storage: Some(storage), document, hello_frontier, session_id, origin, snapshot_chunk_bytes };
        let (input_items, input_bytes) = match database_sync_hello_input_credit(&owners) {
            Ok(credit) => credit,
            Err(error) => return Err(DatabaseSyncHelloRejected::new(pool, error, owners)),
        };
        let admission = match DatabaseSyncHelloAdmission::try_claim(input_items, input_bytes) {
            Ok(admission) => admission,
            Err(error) => return Err(DatabaseSyncHelloRejected::new(pool, error, owners)),
        };
        let slot = admission.slot;
        let generation = admission.generation;
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let expired = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let progress = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(DatabaseSyncHelloProgress::Admitted as u8));
        let future = Box::pin(database_sync_hello_execute(owners, cancelled.clone(), expired.clone(), progress.clone()));
        let deadline_ms = pool.now_ms().saturating_add(DATABASE_SYNC_HELLO_DEADLINE_MS);
        let state = std::sync::Arc::new(DatabaseSyncHelloState {
            pool: pool.clone(),
            slot,
            generation,
            admission: std::sync::Mutex::new(Some(admission)),
            core: std::sync::Mutex::new(DatabaseSyncHelloCore { future: Some(future), execution: None, frame: None, frame_close: None, returned_frame: None, returned_fallback: None, quarantined: None }),
            driver: std::sync::atomic::AtomicU8::new(DatabaseSyncHelloDriverAuthority::Idle as u8),
            retry_job: std::sync::Mutex::new(None),
            cancelled,
            expired,
            deadline_ms,
            progress,
            wake_requested: std::sync::atomic::AtomicBool::new(false),
            demand: std::sync::atomic::AtomicBool::new(false),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            close_armed: std::sync::atomic::AtomicBool::new(false),
            close_requested: std::sync::atomic::AtomicBool::new(false),
            waker: std::sync::Mutex::new(None),
            returned_generation: std::sync::atomic::AtomicU64::new(1),
        });
        database_sync_hello_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot] = Some(state.clone());
        let deadline = state.clone();
        pool.callback_at(deadline_ms, move || deadline.deadline_callback());
        state.schedule();
        Ok(Self { state: Some(state), completed: false })
    }

    pub fn cancel(&self) {
        if let Some(state) = self.state.as_ref() {
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.schedule();
        }
    }

    pub fn progress(&self) -> DatabaseSyncHelloProgress {
        self.state.as_ref().map_or(DatabaseSyncHelloProgress::Fault, |state| state.progress())
    }
}

impl std::future::Future for DatabaseSyncHelloFuture {
    type Output = Result<DatabaseSyncHelloResult, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.completed {
            return std::task::Poll::Ready(Err(DbError::Closed));
        }
        let Some(state) = self.state.as_ref().cloned() else { return std::task::Poll::Ready(Err(DbError::Closed)) };
        if !state.current() {
            self.completed = true;
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.arm_close();
            return std::task::Poll::Ready(Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(state.generation), actual: crate::db_ids::GenerationId(0) }));
        }
        if state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).execution.is_some() {
            self.completed = true;
            self.state.take();
            return std::task::Poll::Ready(Ok(DatabaseSyncHelloResult { state: Some(state) }));
        }
        *state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        if state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).execution.is_some() {
            self.completed = true;
            self.state.take();
            return std::task::Poll::Ready(Ok(DatabaseSyncHelloResult { state: Some(state) }));
        }
        state.schedule();
        std::task::Poll::Pending
    }
}

impl Drop for DatabaseSyncHelloFuture {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        if let Some(state) = self.state.take() {
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.schedule();
        }
    }
}

/// 🧾️ Terminal hello witness converted to a retained frame session or closed.
pub struct DatabaseSyncHelloResult {
    state: Option<std::sync::Arc<DatabaseSyncHelloState>>,
}

impl DatabaseSyncHelloResult {
    pub fn close_and_take_session(mut self) -> Result<DatabaseSyncHelloSession, DbError> {
        let Some(state) = self.state.take() else { return Err(DbError::Closed) };
        let error = {
            let mut core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(execution) = core.execution.as_mut() else { return Err(DbError::Closed) };
            if execution.prepared.is_err() {
                match std::mem::replace(&mut execution.prepared, Err(DbError::Closed)) {
                    Err(error) => Some(error),
                    Ok(prepared) => {
                        execution.prepared = Ok(prepared);
                        None
                    }
                }
            } else {
                None
            }
        };
        if let Some(error) = error {
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            state.arm_close();
            return Err(error);
        }
        Ok(DatabaseSyncHelloSession { state: Some(state), welcome_taken: false })
    }
}

impl Drop for DatabaseSyncHelloResult {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.arm_close();
        }
    }
}

fn database_sync_hello_returned_frame_credit(frame: &protocol::ServerFrame) -> Result<(usize, usize), DbError> {
    match frame {
        protocol::ServerFrame::SnapshotChunk { bytes, .. } => Ok((usize::from(bytes.backing_bytes() != 0), bytes.backing_bytes())),
        _ => Ok((0, 0)),
    }
}

fn database_sync_hello_lease_frame(state: &std::sync::Arc<DatabaseSyncHelloState>, core: &mut DatabaseSyncHelloCore, frame: protocol::ServerFrame) -> Result<DatabaseSyncHelloReturnedFrame, DbError> {
    if core.returned_frame.is_some() {
        let (items, bytes) = database_sync_hello_returned_frame_credit(&frame)?;
        if core.returned_fallback.is_none() {
            core.returned_fallback = Some(DatabaseSyncHelloReturnedFrameLease { generation: 0, items, bytes, close: Some(DatabaseSyncHelloFrameClose { owner: Some(frame), envelope: None }) });
            state.schedule();
        }
        return Err(DbError::Unavailable("database sync hello returned frame lease occupied".to_string()));
    }
    let generation = match state.returned_generation.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| generation.checked_add(1).filter(|next| *next != 0)) {
        Ok(generation) => generation,
        Err(generation) => {
            let (items, bytes) = database_sync_hello_returned_frame_credit(&frame)?;
            core.returned_frame = Some(DatabaseSyncHelloReturnedFrameLease { generation, items, bytes, close: Some(DatabaseSyncHelloFrameClose { owner: Some(frame), envelope: None }) });
            state.schedule();
            return Err(DbError::LimitExceeded("database sync hello returned frame generation"));
        }
    };
    let (items, bytes) = database_sync_hello_returned_frame_credit(&frame)?;
    core.returned_frame = Some(DatabaseSyncHelloReturnedFrameLease { generation, items, bytes, close: None });
    Ok(DatabaseSyncHelloReturnedFrame { state: Some(state.clone()), generation, frame: Some(frame) })
}

/// 📤️ Generation-qualified returned frame whose explicit acknowledgement mounts exact close.
pub struct DatabaseSyncHelloReturnedFrame {
    state: Option<std::sync::Arc<DatabaseSyncHelloState>>,
    generation: u64,
    frame: Option<protocol::ServerFrame>,
}

impl DatabaseSyncHelloReturnedFrame {
    pub fn frame(&self) -> Result<&protocol::ServerFrame, DbError> {
        self.frame.as_ref().ok_or(DbError::Closed)
    }

    fn mount_close(&mut self) -> Result<(), DbError> {
        let Some(frame) = self.frame.take() else { return Err(DbError::Closed) };
        let Some(state) = self.state.as_ref() else {
            self.frame = Some(frame);
            return Err(DbError::Closed);
        };
        let mut core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(lease) = core.returned_frame.as_mut() else {
            let (items, bytes) = database_sync_hello_returned_frame_credit(&frame)?;
            core.returned_frame = Some(DatabaseSyncHelloReturnedFrameLease { generation: self.generation, items, bytes, close: Some(DatabaseSyncHelloFrameClose { owner: Some(frame), envelope: None }) });
            drop(core);
            state.schedule();
            return Err(DbError::Closed);
        };
        if lease.generation != self.generation || lease.close.is_some() {
            let actual = lease.generation;
            let (items, bytes) = database_sync_hello_returned_frame_credit(&frame)?;
            if core.returned_fallback.is_some() {
                self.frame = Some(frame);
                return Err(DbError::Unavailable("database sync hello returned frame fallback occupied".to_string()));
            }
            core.returned_fallback = Some(DatabaseSyncHelloReturnedFrameLease { generation: self.generation, items, bytes, close: Some(DatabaseSyncHelloFrameClose { owner: Some(frame), envelope: None }) });
            drop(core);
            state.schedule();
            return Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(self.generation), actual: crate::db_ids::GenerationId(actual) });
        }
        lease.close = Some(DatabaseSyncHelloFrameClose { owner: Some(frame), envelope: None });
        drop(core);
        state.schedule();
        Ok(())
    }

    pub fn acknowledge(mut self) -> Result<(), DbError> {
        self.mount_close()
    }
}

impl Drop for DatabaseSyncHelloReturnedFrame {
    fn drop(&mut self) {
        if self.frame.is_some() {
            match self.mount_close() {
                Ok(()) | Err(DbError::StaleGeneration { .. }) => {}
                Err(_) => {
                    if let Some(state) = self.state.as_ref() {
                        state.cancelled.store(true, std::sync::atomic::Ordering::Release);
                        state.arm_close();
                    }
                }
            }
        }
    }
}

/// 📡️ Backpressured hello output with one bounded I/O-lane opportunity per frame request.
pub struct DatabaseSyncHelloSession {
    state: Option<std::sync::Arc<DatabaseSyncHelloState>>,
    welcome_taken: bool,
}

impl DatabaseSyncHelloSession {
    pub fn take_welcome(&mut self) -> Result<DatabaseSyncHelloReturnedFrame, DbError> {
        if self.welcome_taken {
            return Err(DbError::Closed);
        }
        let Some(state) = self.state.as_ref() else { return Err(DbError::Closed) };
        database_sync_hello_control(&state.cancelled, &state.expired)?;
        let mut core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let welcome = core.execution.as_mut().and_then(|execution| execution.prepared.as_mut().ok()).and_then(|prepared| prepared.welcome.take()).ok_or_else(|| DbError::Internal("database sync hello welcome owner missing".to_string()))?;
        let welcome = database_sync_hello_lease_frame(state, &mut core, welcome)?;
        self.welcome_taken = true;
        Ok(welcome)
    }

    pub fn next_frame(&self) -> DatabaseSyncHelloNextFuture {
        DatabaseSyncHelloNextFuture { state: self.state.clone(), completed: false }
    }

    pub fn cancel(&self) {
        if let Some(state) = self.state.as_ref() {
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.arm_close();
        }
    }

    pub fn progress(&self) -> DatabaseSyncHelloProgress {
        self.state.as_ref().map_or(DatabaseSyncHelloProgress::Fault, |state| state.progress())
    }
}

impl Drop for DatabaseSyncHelloSession {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.arm_close();
        }
    }
}

pub struct DatabaseSyncHelloNextFuture {
    state: Option<std::sync::Arc<DatabaseSyncHelloState>>,
    completed: bool,
}

impl std::future::Future for DatabaseSyncHelloNextFuture {
    type Output = Result<Option<DatabaseSyncHelloReturnedFrame>, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.completed {
            return std::task::Poll::Ready(Err(DbError::Closed));
        }
        let Some(state) = self.state.as_ref().cloned() else { return std::task::Poll::Ready(Err(DbError::Closed)) };
        if let Err(error) = database_sync_hello_control(&state.cancelled, &state.expired) {
            self.completed = true;
            state.arm_close();
            return std::task::Poll::Ready(Err(error));
        }
        let mut core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if core.returned_frame.is_none() {
            if let Some(frame) = core.frame.take() {
                self.completed = true;
                return match frame {
                    Ok(Some(frame)) => std::task::Poll::Ready(database_sync_hello_lease_frame(&state, &mut core, frame).map(Some)),
                    Ok(None) => {
                        drop(core);
                        state.arm_close();
                        std::task::Poll::Ready(Ok(None))
                    }
                    Err(error) => std::task::Poll::Ready(Err(error)),
                };
            }
        }
        drop(core);
        *state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        let mut core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if core.returned_frame.is_none() {
            if let Some(frame) = core.frame.take() {
                self.completed = true;
                return match frame {
                    Ok(Some(frame)) => std::task::Poll::Ready(database_sync_hello_lease_frame(&state, &mut core, frame).map(Some)),
                    Ok(None) => {
                        drop(core);
                        state.arm_close();
                        std::task::Poll::Ready(Ok(None))
                    }
                    Err(error) => std::task::Poll::Ready(Err(error)),
                };
            }
        }
        let lease_closing = core.returned_frame.as_ref().is_some_and(|lease| lease.close.is_some());
        drop(core);
        if lease_closing {
            state.schedule();
        } else if state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).returned_frame.is_none() {
            state.demand.store(true, std::sync::atomic::Ordering::Release);
            state.schedule();
        }
        std::task::Poll::Pending
    }
}

impl Drop for DatabaseSyncHelloNextFuture {
    fn drop(&mut self) {
        if !self.completed {
            if let Some(state) = self.state.take() {
                state.cancelled.store(true, std::sync::atomic::Ordering::Release);
                state.arm_close();
            }
        }
    }
}
struct DatabaseSyncHelloRejectedClose {
    pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    owners: std::sync::Mutex<Option<DatabaseSyncHelloOwners>>,
    queued: std::sync::atomic::AtomicBool,
    retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    cancelled: std::sync::atomic::AtomicBool,
    deadline_ms: u64,
    terminal: std::sync::atomic::AtomicU8,
    terminal_callback: std::sync::atomic::AtomicBool,
    submissions: std::sync::atomic::AtomicU8,
    registry_generation: std::sync::atomic::AtomicU64,
    registry_next: std::sync::Mutex<Option<std::sync::Arc<DatabaseSyncHelloRejectedClose>>>,
    registry_released: std::sync::atomic::AtomicBool,
}

fn database_sync_hello_rejected_registry() -> &'static std::sync::Mutex<Option<std::sync::Arc<DatabaseSyncHelloRejectedClose>>> {
    static REGISTRY: std::sync::Mutex<Option<std::sync::Arc<DatabaseSyncHelloRejectedClose>>> = std::sync::Mutex::new(None);
    &REGISTRY
}

fn database_sync_hello_install_rejected_registry(owner: &std::sync::Arc<DatabaseSyncHelloRejectedClose>) -> Result<u64, DbError> {
    static GENERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let generation = GENERATION
        .fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| generation.checked_add(1).filter(|generation| *generation != 0))
        .map_err(|_| DbError::LimitExceeded("database sync hello rejection generation"))?;
    owner.registry_generation.store(generation, std::sync::atomic::Ordering::Release);
    let mut registry = database_sync_hello_rejected_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    *owner.registry_next.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = registry.take();
    *registry = Some(owner.clone());
    Ok(generation)
}

fn database_sync_hello_release_rejected_registry(owner: &std::sync::Arc<DatabaseSyncHelloRejectedClose>) {
    owner.registry_released.store(true, std::sync::atomic::Ordering::Release);
    let mut registry = database_sync_hello_rejected_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(root) = registry.as_ref() else { return };
    if !std::sync::Arc::ptr_eq(root, owner) || !root.registry_released.load(std::sync::atomic::Ordering::Acquire) {
        return;
    }
    let next = root.registry_next.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
    *registry = next;
    let cleanup = registry.as_ref().filter(|next| next.registry_released.load(std::sync::atomic::Ordering::Acquire)).cloned();
    drop(registry);
    if let Some(next) = cleanup {
        let pool = next.pool.clone();
        pool.callback_at(pool.now_ms().saturating_add(1), move || database_sync_hello_release_rejected_registry(&next));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DatabaseSyncHelloRejectedTerminalWitness {
    pub generation: u64,
    pub terminal: u8,
    pub job_retained: bool,
    pub owners_retained: bool,
    pub submissions: u8,
}

pub fn database_sync_hello_rejected_terminal_witness(generation: u64) -> Option<DatabaseSyncHelloRejectedTerminalWitness> {
    let registry = database_sync_hello_rejected_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut owner = registry.clone();
    drop(registry);
    while let Some(current) = owner {
        if current.registry_generation.load(std::sync::atomic::Ordering::Acquire) == generation {
            return Some(DatabaseSyncHelloRejectedTerminalWitness {
                generation,
                terminal: current.terminal.load(std::sync::atomic::Ordering::Acquire),
                job_retained: current.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
                owners_retained: current.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
                submissions: current.submissions.load(std::sync::atomic::Ordering::Acquire),
            });
        }
        owner = current.registry_next.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
    }
    None
}

impl DatabaseSyncHelloRejectedClose {
    fn claim_submission(&self) -> bool {
        self.submissions.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |submissions| submissions.checked_add(1).filter(|next| *next <= DATABASE_SYNC_HELLO_RETRY_LIMIT)).is_ok()
    }

    fn publish_terminal_recovery(self: &std::sync::Arc<Self>, terminal: u8) {
        self.terminal.store(terminal, std::sync::atomic::Ordering::Release);
        if self.terminal_callback.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        let close = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || close.retry_terminal_once());
    }

    fn retry_terminal_once(self: std::sync::Arc<Self>) {
        self.terminal_callback.store(false, std::sync::atomic::Ordering::Release);
        let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return };
        if !self.claim_submission() {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            self.terminal.store(3, std::sync::atomic::Ordering::Release);
            return;
        }
        self.queued.store(true, std::sync::atomic::Ordering::Release);
        match self.pool.try_submit(semio_framework_async::Lane::Io, job) {
            Ok(()) => self.terminal.store(0, std::sync::atomic::Ordering::Release),
            Err(error) => {
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), attempt));
                self.queued.store(false, std::sync::atomic::Ordering::Release);
            }
        }
    }

    fn terminal_witness(&self, attempt: u8) -> u8 {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return 1;
        }
        if self.pool.now_ms() >= self.deadline_ms {
            return 2;
        }
        if attempt >= DATABASE_SYNC_HELLO_RETRY_LIMIT.saturating_sub(1) {
            return 3;
        }
        0
    }

    fn retain_retry(self: &std::sync::Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
        self.queued.store(false, std::sync::atomic::Ordering::Release);
        let terminal = self.terminal_witness(attempt);
        if terminal != 0 {
            self.publish_terminal_recovery(terminal);
            return;
        }
        let close = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || close.retry());
    }

    fn schedule(self: &std::sync::Arc<Self>) {
        if self.terminal.load(std::sync::atomic::Ordering::Acquire) != 0 {
            return;
        }
        if self.queued.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        if !self.claim_submission() {
            self.queued.store(false, std::sync::atomic::Ordering::Release);
            self.publish_terminal_recovery(3);
            return;
        }
        let close = self.clone();
        match self.pool.try_submit(semio_framework_async::Lane::Io, Box::new(move || close.drive_one())) {
            Ok(()) => {}
            Err(error) => self.retain_retry(error.into_job(), 1),
        }
    }

    fn retry(self: std::sync::Arc<Self>) {
        let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return };
        let terminal = self.terminal_witness(attempt);
        if terminal != 0 {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            self.publish_terminal_recovery(terminal);
            return;
        }
        if !self.claim_submission() {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            self.publish_terminal_recovery(3);
            return;
        }
        self.queued.store(true, std::sync::atomic::Ordering::Release);
        match self.pool.try_submit(semio_framework_async::Lane::Io, job) {
            Ok(()) => {}
            Err(error) => self.retain_retry(error.into_job(), attempt.saturating_add(1)),
        }
    }

    fn drive_one(self: std::sync::Arc<Self>) {
        self.queued.store(false, std::sync::atomic::Ordering::Release);
        let mut owners = self.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(owner) = owners.as_mut() else { return };
        if owner.close_one() {
            let terminal = owner.terminal_is_empty();
            if terminal {
                owners.take();
                self.terminal.store(4, std::sync::atomic::Ordering::Release);
                database_sync_hello_release_rejected_registry(&self);
            }
            drop(owners);
            if !terminal {
                self.schedule();
            }
            return;
        }
        owners.take();
        self.terminal.store(4, std::sync::atomic::Ordering::Release);
        database_sync_hello_release_rejected_registry(&self);
    }
}

/// ⛔️ Lossless pre-admission refusal retaining every exact hello input owner.
pub struct DatabaseSyncHelloRejected {
    error: Option<DbError>,
    close: std::sync::Arc<DatabaseSyncHelloRejectedClose>,
}

impl DatabaseSyncHelloRejected {
    fn new(pool: std::sync::Arc<semio_framework_async::WorkerPool>, error: DbError, owners: DatabaseSyncHelloOwners) -> Self {
        let deadline_ms = pool.now_ms().saturating_add(DATABASE_SYNC_HELLO_DEADLINE_MS);
        let close = std::sync::Arc::new(DatabaseSyncHelloRejectedClose {
            pool,
            owners: std::sync::Mutex::new(Some(owners)),
            queued: std::sync::atomic::AtomicBool::new(false),
            retry_job: std::sync::Mutex::new(None),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            deadline_ms,
            terminal: std::sync::atomic::AtomicU8::new(0),
            terminal_callback: std::sync::atomic::AtomicBool::new(false),
            submissions: std::sync::atomic::AtomicU8::new(0),
            registry_generation: std::sync::atomic::AtomicU64::new(0),
            registry_next: std::sync::Mutex::new(None),
            registry_released: std::sync::atomic::AtomicBool::new(false),
        });
        let error = match database_sync_hello_install_rejected_registry(&close) {
            Ok(_) => error,
            Err(registry_error) => {
                let mut registry = database_sync_hello_rejected_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                *close.registry_next.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = registry.take();
                *registry = Some(close.clone());
                registry_error
            }
        };
        Self { error: Some(error), close }
    }

    pub fn retirement_generation(&self) -> u64 {
        self.close.registry_generation.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn cancel_retirement(&self) {
        self.close.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }

    pub fn close_and_take_error(mut self) -> DbError {
        let error = self.error.take().unwrap_or(DbError::Closed);
        self.close.schedule();
        error
    }
}

impl std::fmt::Debug for DatabaseSyncHelloRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseSyncHelloRejected").field("error", &self.error).finish()
    }
}

impl Drop for DatabaseSyncHelloRejected {
    fn drop(&mut self) {
        self.close.schedule();
    }
}
//#endregion 👋️RetainedHello

/// @emoji 🚀️ What `handle_hello` produces: the `Welcome` frame itself, plus whatever follow-up
/// frames the chosen bootstrap needs (a single `Commands` frame for `Tail`; one `SnapshotChunk`
/// per chunk plus a trailing `SnapshotDone` for a non-inlined `Snapshot`; none for `None` or an
/// inlined `Snapshot`, whose bytes already travel inside `Welcome.bootstrap`).
#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub struct WelcomeResponse {
    pub welcome: protocol::ServerFrame,
    pub follow_up: Vec<protocol::ServerFrame>,
}

#[cfg(test)]
struct BootstrapPageRangeCopy<'a> {
    pages: &'a db_storage::DbIoPages,
    skip: usize,
    remaining: usize,
    page: u8,
    output: Option<Vec<u8>>,
}

#[cfg(test)]
impl std::future::Future for BootstrapPageRangeCopy<'_> {
    type Output = Result<Vec<u8>, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        if owner.remaining == 0 {
            return std::task::Poll::Ready(Ok(owner.output.take().expect("bootstrap page range output consumed once")));
        }
        let Some(fragment) = owner.pages.page(owner.page) else {
            return std::task::Poll::Ready(Err(DbError::Corrupt("bootstrap page range exceeded its retained owner".to_string())));
        };
        if owner.skip >= fragment.len() {
            owner.skip -= fragment.len();
            owner.page += 1;
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        let available = &fragment[owner.skip..];
        let written = available.len().min(owner.remaining);
        owner.output.as_mut().expect("bootstrap page range output retained").extend_from_slice(&available[..written]);
        owner.skip = 0;
        owner.remaining -= written;
        owner.page += 1;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

#[cfg(test)]
fn bootstrap_page_range_copy(pages: &db_storage::DbIoPages, start: usize, len: usize) -> Result<BootstrapPageRangeCopy<'_>, DbError> {
    let end = start.checked_add(len).ok_or(DbError::LimitExceeded("bootstrap page range"))?;
    if end > pages.len() {
        return Err(DbError::InvalidArgument("bootstrap page range exceeds its retained owner".to_string()));
    }
    let mut output = Vec::new();
    output.try_reserve_exact(len).map_err(|_| DbError::Unavailable("bootstrap protocol result capacity exhausted".to_string()))?;
    Ok(BootstrapPageRangeCopy { pages, skip: start, remaining: len, page: 0, output: Some(output) })
}

/// @emoji 🏗️ Lowers a `BootstrapPlan` to the wire `protocol::Bootstrap` shape plus its follow-up
/// frames. A `Snapshot` whose bytes fit within `snapshot_chunk_bytes` is inlined directly into
/// `Bootstrap::Snapshot.inline` (no follow-up frames); a larger one is chunked instead — this
/// crate's own choice of threshold behavior, since the contract fixes `Bootstrap::Snapshot`'s two
/// shapes but not when to prefer one over the other.
#[cfg(test)]
async fn lower_bootstrap_plan(plan: &BootstrapPlan, state: &ArtifactSyncState, origin: &protocol::ActorId, snapshot_chunk_bytes: usize) -> Result<(protocol::Bootstrap, Vec<protocol::ServerFrame>), DbError> {
    match plan {
        BootstrapPlan::None => Ok((protocol::Bootstrap::None, Vec::new())),
        BootstrapPlan::Tail { envelopes } => Ok((protocol::Bootstrap::Tail, vec![commands_server_frame(state, envelopes.clone(), origin.clone()).await])),
        BootstrapPlan::Snapshot { pages, pack_hash, .. } => {
            if pages.len() <= snapshot_chunk_bytes {
                let bytes = bootstrap_page_range_copy(pages, 0, pages.len())?.await?;
                Ok((protocol::Bootstrap::Snapshot { pack_hash: *pack_hash, inline: Some(bytes) }, Vec::new()))
            } else {
                let chunks = pages.len().div_ceil(snapshot_chunk_bytes);
                if chunks > db_storage::DB_IO_OPERATION_ITEMS {
                    return Err(DbError::LimitExceeded("bootstrap protocol chunk items"));
                }
                let mut follow_up = Vec::new();
                follow_up.try_reserve_exact(chunks + 1).map_err(|_| DbError::Unavailable("bootstrap protocol item capacity exhausted".to_string()))?;
                for seq in 0..chunks {
                    let start = seq * snapshot_chunk_bytes;
                    let len = snapshot_chunk_bytes.min(pages.len() - start);
                    let bytes = bootstrap_page_range_copy(pages, start, len)?.await?;
                    let fixed = protocol::SnapshotChunkBytes::try_from_slice(&bytes).ok_or(DbError::LimitExceeded("bootstrap protocol fixed snapshot chunk"))?;
                    follow_up.push(protocol::ServerFrame::SnapshotChunk { seq: seq as u32, bytes: fixed });
                }
                follow_up.push(protocol::ServerFrame::SnapshotDone { seq_count: chunks as u32 });
                Ok((protocol::Bootstrap::Snapshot { pack_hash: *pack_hash, inline: None }, follow_up))
            }
        }
    }
}

/// @emoji 🏗️ Builds the full `WelcomeResponse` for `plan` against `state`. `snapshot_chunk_bytes`
/// must be non-zero (validated before `lower_bootstrap_plan` could otherwise divide the snapshot
/// into a runaway number of zero-progress chunks).
#[cfg(test)]
pub async fn build_welcome(state: &ArtifactSyncState, plan: &BootstrapPlan, session_id: String, origin: &protocol::ActorId, snapshot_chunk_bytes: usize) -> Result<WelcomeResponse, DbError> {
    if snapshot_chunk_bytes == 0 {
        return Err(DbError::InvalidArgument("snapshot_chunk_bytes must be non-zero".to_string()));
    }
    let resume_token = issue_resume_token(&state.frontier).await?;
    let (bootstrap, follow_up) = lower_bootstrap_plan(plan, state, origin, snapshot_chunk_bytes).await?;
    let welcome = protocol::ServerFrame::Welcome { session_id, resume_token, server_frontier: state_frontier_summary(state).await, bootstrap };
    Ok(WelcomeResponse { welcome, follow_up })
}

/// @emoji 👋️ The top-level entry point for a `protocol::ClientFrame::Hello`: replays `document`'s
/// current sync state, decides a bootstrap plan against `hello_frontier` (the replica's advertised
/// `RuntimeFrontierSummary`, `None` for a totally fresh replica — see module doc for why this
/// crate reads `Hello.frontier` rather than decoding `Hello.resume_token`), and lowers it to a
/// `WelcomeResponse`.
#[cfg(test)]
pub async fn handle_hello(
    pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    storage: std::sync::Arc<db_storage::DbBackend>,
    document: ArtifactId,
    hello_frontier: Option<protocol::RuntimeFrontierSummary>,
    session_id: String,
    origin: protocol::ActorId,
    snapshot_chunk_bytes: usize,
) -> Result<WelcomeResponse, DbError> {
    let future = DatabaseSyncHelloFuture::try_submit(pool, storage, document, hello_frontier, session_id, origin, snapshot_chunk_bytes).map_err(DatabaseSyncHelloRejected::close_and_take_error)?;
    let mut session = future.await?.close_and_take_session()?;
    let welcome_owner = session.take_welcome()?;
    let welcome = welcome_owner.frame()?.clone();
    welcome_owner.acknowledge()?;
    let mut follow_up = Vec::new();
    while let Some(frame_owner) = session.next_frame().await? {
        follow_up.push(frame_owner.frame()?.clone());
        frame_owner.acknowledge()?;
    }
    Ok(WelcomeResponse { welcome, follow_up })
}

/// @emoji 📡️ Mid-session catch-up: a connected replica sends `ClientFrame::FrontierAdvertise`
/// (e.g. after a period of being caught up passively via broadcast, to confirm its position) and
/// the semio_hub replies with whatever commands it's still missing, or `None` if it's already current.
pub async fn handle_frontier_advertise(storage: &impl db_storage::WalStorage, document: ArtifactId, advertised: &protocol::RuntimeFrontierSummary, origin: protocol::ActorId) -> Result<Option<protocol::ServerFrame>, DbError> {
    let state = replay_sync_state(storage, document).await?;
    let replica = from_frontier_summary(advertised);
    let missing = missing_commands(&state, &replica).await?;
    Ok(if missing.is_empty() { None } else { Some(commands_server_frame(&state, missing, origin).await) })
}
//#endregion 🔖️Hello
//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use db_storage::MemoryStorage;
    use db_wal::{ArtifactWal, GroupCommitPolicy, WalRecord};
    use ArtifactId;

    //#region 🧸️Fixtures
    async fn sample_envelope(id: &str, seq: u64) -> protocol::MutationEnvelope {
        protocol::MutationEnvelope {
            mutation_id: protocol::MutationId(id.to_string()),
            document_id: protocol::ArtifactId("doc-1".to_string()),
            actor: protocol::ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId("diff.v1".to_string()), payload: seq.to_le_bytes().to_vec() },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId("diff.v1".to_string()), payload: Vec::new() },
            timestamp: protocol::HybridLogicalTimestamp::new(1, seq),
        }
    }

    async fn submit_record(storage: &MemoryStorage, wal: &mut ArtifactWal, record: WalRecord, now_ms: u64) {
        let mut records = db_wal::WalRecordBatch::new();
        assert!(records.push(record).is_ok());
        wal.submit(storage, &records, DurabilityClass::Fsync, now_ms).await.unwrap();
        while records.close_step().unwrap() {}
    }

    async fn command_record(envelope: &protocol::MutationEnvelope) -> WalRecord {
        let mut control = db_wal::WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
        let bytes = db_wal::WalBytes::try_admit(encode_command_envelope(envelope).await, 1024 * 1024, &mut control).await.unwrap();
        WalRecord::Command(bytes)
    }

    /// @emoji 🧸️ Creates `document`'s WAL in `storage` and submits `count` sample commands
    /// (ids `"op-0".."op-{count-1}"`), each `Fsync`-durable so replay sees them immediately.
    async fn seed_wal(storage: &MemoryStorage, document: &ArtifactId, count: u64) {
        let mut wal = db_actor::block_on(ArtifactWal::create(storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
        for i in 0..count {
            let envelope = sample_envelope(&format!("op-{i}"), i).await;
            submit_record(storage, &mut wal, command_record(&envelope).await, i).await;
        }
    }

    /// @emoji 🧸️ Reopens `document`'s WAL and appends one `SnapshotPub` marker covering `frontier`.
    async fn publish_snapshot_marker(storage: &MemoryStorage, document: &ArtifactId, generation: u64, frontier: Frontier) {
        let (mut wal, _report) = db_actor::block_on(ArtifactWal::open(storage, document.clone(), GroupCommitPolicy::default(), 1000)).unwrap();
        submit_record(storage, &mut wal, WalRecord::SnapshotPub { generation, frontier }, 1000).await;
    }
    //#endregion 🧸️Fixtures

    //#region 🔖️Codec
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trips_through_encode_decode() {
        let envelope = sample_envelope("op-1", 7).await;
        let bytes = encode_command_envelope(&envelope).await;
        assert_eq!(decode_command_envelope(&bytes).await.unwrap(), envelope);
    }

    #[semio_framework_async_macros::async_test]
    async fn decode_command_envelope_rejects_malformed_bytes_without_panicking() {
        assert!(matches!(decode_command_envelope(b"not json").await, Err(DbError::Corrupt(_))));
    }
    //#endregion 🔖️Codec

    //#region 🔖️ReplicaState
    #[semio_framework_async_macros::async_test]
    async fn replay_sync_state_derives_frontier_and_ordered_commands() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 3).await;

        let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();
        assert_eq!(state.frontier.head_seq, 3);
        assert_eq!(state.frontier.commit_seq, 3);
        assert_eq!(state.floor_head_seq, 0);
        assert_eq!(state.commands.len(), 3);
        assert_eq!(state.commands[0].mutation_id.0, "op-0");
        assert_eq!(state.commands[2].mutation_id.0, "op-2");
    }

    #[semio_framework_async_macros::async_test]
    async fn replay_sync_state_on_empty_document_is_genesis() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 0).await;

        let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();
        assert_eq!(state.frontier.head_seq, 0);
        assert_eq!(state.frontier.chain_hash, [0u8; 32]);
        assert!(state.commands.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn replay_sync_state_tracks_the_latest_snapshot_pub_as_the_floor() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 5).await;
        let floor_frontier = Frontier { document: document.clone(), head_seq: 2, commit_seq: 2, chain_hash: [1u8; 32], epoch: 0 };
        publish_snapshot_marker(&storage, &document, 1, floor_frontier).await;

        let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();
        assert_eq!(state.floor_head_seq, 2);
        assert_eq!(state.frontier.head_seq, 5, "the marker itself carries no commands");
    }
    //#endregion 🔖️ReplicaState

    //#region 🔖️Frontier
    #[semio_framework_async_macros::async_test]
    async fn frontier_delta_reports_the_command_gap_and_rejects_backwards() {
        let document: ArtifactId = "doc-1".into();
        let from = Frontier { document: document.clone(), head_seq: 2, commit_seq: 2, chain_hash: [0u8; 32], epoch: 0 };
        let to = Frontier { document, head_seq: 5, commit_seq: 5, chain_hash: [9u8; 32], epoch: 0 };

        let delta = frontier_delta(&from, &to).await.unwrap();
        assert_eq!(delta.commands, 3);
        assert!(!delta.is_empty().await);
        assert!(frontier_delta(&to, &from).await.is_err(), "a delta only ever moves forward");
    }

    #[semio_framework_async_macros::async_test]
    async fn frontier_summary_bridges_round_trip() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 2).await;
        let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();

        let summary = state_frontier_summary(&state).await;
        assert_eq!(summary.head_edit_ordinal, 2);
        assert_eq!(summary.head_edit_id, "op-1");
        assert_eq!(summary.last_commit_seq, state.frontier.commit_seq);
        assert_eq!(summary.chain_hash, state.frontier.chain_hash);

        let bridged_back = from_frontier_summary(&summary);
        assert_eq!(bridged_back.head_seq, state.frontier.head_seq);
        assert_eq!(bridged_back.commit_seq, state.frontier.commit_seq);
        assert_eq!(bridged_back.chain_hash, state.frontier.chain_hash);
        assert_eq!(bridged_back.document, state.frontier.document);
    }
    //#endregion 🔖️Frontier

    //#region 🔖️MissingCommands
    #[semio_framework_async_macros::async_test]
    async fn missing_commands_transfer_round_trip_from_genesis() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 4).await;
        let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();

        let replica_frontier = Frontier::genesis(document);
        let missing = missing_commands(&state, &replica_frontier).await.unwrap();
        assert_eq!(missing, state.commands, "a genesis replica is missing every command");

        // "Applying" the transfer catches the replica up to the server's frontier exactly.
        let caught_up = state.frontier.clone();
        assert!(missing_commands(&state, &caught_up).await.unwrap().is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_commands_transfer_round_trip_for_a_partially_caught_up_replica() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 3).await;
        let first_state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();
        let replica_frontier = first_state.frontier;

        // More commands land on the server after the replica already caught up once.
        {
            let (mut wal, _report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 100)).unwrap();
            for i in 3..6u64 {
                let envelope = sample_envelope(&format!("op-{i}"), i).await;
                submit_record(&storage, &mut wal, command_record(&envelope).await, i).await;
            }
        }

        let second_state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();
        let missing = missing_commands(&second_state, &replica_frontier).await.unwrap();
        assert_eq!(missing.len(), 3);
        assert_eq!(missing[0].mutation_id.0, "op-3");
        assert_eq!(missing[2].mutation_id.0, "op-5");
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_commands_rejects_document_mismatch_and_a_replica_ahead_of_server() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 2).await;
        let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();

        let other_document = Frontier::genesis("doc-2".into());
        assert!(matches!(missing_commands(&state, &other_document).await, Err(DbError::InvalidArgument(_))));

        let ahead = Frontier { head_seq: 99, ..state.frontier.clone() };
        assert!(matches!(missing_commands(&state, &ahead).await, Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_commands_rejects_a_replica_behind_the_retained_floor() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 5).await;
        let floor_frontier = Frontier { document: document.clone(), head_seq: 3, commit_seq: 3, chain_hash: [2u8; 32], epoch: 0 };
        publish_snapshot_marker(&storage, &document, 1, floor_frontier).await;
        let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();

        let too_far_behind = Frontier { document, head_seq: 1, commit_seq: 1, chain_hash: [0u8; 32], epoch: 0 };
        assert!(matches!(missing_commands(&state, &too_far_behind).await, Err(DbError::Unavailable(_))));
    }
    //#endregion 🔖️MissingCommands

    //#region 🔖️Bootstrap
    #[semio_framework_async_macros::async_test]
    async fn decide_bootstrap_serves_tail_for_a_fresh_replica_within_the_floor() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 3).await;
        let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();

        let plan = db_actor::block_on(decide_bootstrap(&state, &storage, None)).unwrap();
        assert_eq!(plan, BootstrapPlan::Tail { envelopes: state.commands });
    }

    #[semio_framework_async_macros::async_test]
    async fn decide_bootstrap_reports_none_for_an_already_caught_up_replica() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 3).await;
        let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();

        let plan = db_actor::block_on(decide_bootstrap(&state, &storage, Some(&state.frontier))).unwrap();
        assert_eq!(plan, BootstrapPlan::None);
    }

    #[semio_framework_async_macros::async_test]
    async fn decide_bootstrap_serves_snapshot_when_a_generation_is_available_below_the_floor() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 5).await;
        let floor_frontier = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [3u8; 32], epoch: 0 };
        publish_snapshot_marker(&storage, &document, 7, floor_frontier).await;
        db_actor::block_on(db_storage::SnapshotStorage::write_generation(&storage, &document, 7, b"snapshot-bytes")).unwrap();
        let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();

        let stale_replica = Frontier { document, head_seq: 0, commit_seq: 0, chain_hash: [0u8; 32], epoch: 0 };
        let plan = db_actor::block_on(decide_bootstrap(&state, &storage, Some(&stale_replica))).unwrap();
        match plan {
            BootstrapPlan::Snapshot { generation, pages, pack_hash } => {
                assert_eq!(generation, 7);
                assert_eq!(pages, b"snapshot-bytes");
                assert_eq!(pack_hash, *blake3::hash(b"snapshot-bytes").as_bytes());
            }
            other => panic!("expected a Snapshot plan, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn decide_bootstrap_reports_unavailable_when_below_floor_with_no_snapshot() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 5).await;
        let floor_frontier = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [3u8; 32], epoch: 0 };
        publish_snapshot_marker(&storage, &document, 7, floor_frontier).await;
        let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();

        let stale_replica = Frontier { document, head_seq: 0, commit_seq: 0, chain_hash: [0u8; 32], epoch: 0 };
        assert!(matches!(db_actor::block_on(decide_bootstrap(&state, &storage, Some(&stale_replica))), Err(DbError::Unavailable(_))));
    }
    //#endregion 🔖️Bootstrap

    //#region 🔖️ResumeToken
    #[semio_framework_async_macros::async_test]
    async fn issue_resume_token_produces_the_documented_v1_wire_format() {
        let document: ArtifactId = "doc-1".into();
        let frontier = Frontier { document, head_seq: 4, commit_seq: 4, chain_hash: [5u8; 32], epoch: 0 };
        let token = issue_resume_token(&frontier).await.unwrap();
        assert!(token.starts_with("v1|doc-1|4|4|0|"));
    }
    //#endregion 🔖️ResumeToken

    //#region 🔖️Hello
    #[semio_framework_async_macros::async_test]
    async fn handle_hello_bootstraps_a_fresh_replica_via_tail_and_issues_a_resume_token() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 3).await;
        let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(storage));

        let response = db_actor::block_on(handle_hello(crate::db_storage::db_io_test_pool(), storage, document, None, "session-1".to_string(), protocol::ActorId("semio_hub".to_string()), 64 * 1024)).unwrap();
        let protocol::ServerFrame::Welcome { bootstrap, server_frontier, resume_token, .. } = &response.welcome else {
            panic!("expected a Welcome frame");
        };
        assert_eq!(*bootstrap, protocol::Bootstrap::Tail);
        assert_eq!(server_frontier.head_edit_ordinal, 3);
        assert!(!resume_token.is_empty());
        assert_eq!(response.follow_up.len(), 1);
        match &response.follow_up[0] {
            protocol::ServerFrame::Commands { envelopes, .. } => assert_eq!(envelopes.len(), 3),
            other => panic!("expected a Commands follow-up frame, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn handle_hello_reports_no_follow_up_for_an_already_caught_up_replica() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 2).await;
        let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();
        let hello_frontier = state_frontier_summary(&state).await;
        let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(storage));

        let response = db_actor::block_on(handle_hello(crate::db_storage::db_io_test_pool(), storage, document, Some(hello_frontier), "session-2".to_string(), protocol::ActorId("semio_hub".to_string()), 64 * 1024)).unwrap();
        let protocol::ServerFrame::Welcome { bootstrap, .. } = &response.welcome else {
            panic!("expected a Welcome frame");
        };
        assert_eq!(*bootstrap, protocol::Bootstrap::None);
        assert!(response.follow_up.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn handle_hello_chunks_a_snapshot_larger_than_the_requested_chunk_size() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 4).await;
        let floor_frontier = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [1u8; 32], epoch: 0 };
        publish_snapshot_marker(&storage, &document, 9, floor_frontier).await;
        let big_snapshot = vec![7u8; 10];
        db_actor::block_on(db_storage::SnapshotStorage::write_generation(&storage, &document, 9, &big_snapshot)).unwrap();
        let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(storage));

        let stale_hello_frontier = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0u8; 32] };
        let response = db_actor::block_on(handle_hello(crate::db_storage::db_io_test_pool(), storage, document, Some(stale_hello_frontier), "session-3".to_string(), protocol::ActorId("semio_hub".to_string()), 4)).unwrap();

        let protocol::ServerFrame::Welcome { bootstrap, .. } = &response.welcome else {
            panic!("expected a Welcome frame");
        };
        assert!(matches!(bootstrap, protocol::Bootstrap::Snapshot { inline: None, .. }));
        // 10 bytes chunked at 4 bytes/chunk -> 3 chunks (4, 4, 2), plus one SnapshotDone.
        assert_eq!(response.follow_up.len(), 4);
        assert!(matches!(response.follow_up[3], protocol::ServerFrame::SnapshotDone { seq_count: 3 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn handle_hello_rejects_zero_snapshot_chunk_bytes() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 1).await;
        let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(storage));
        assert!(matches!(db_actor::block_on(handle_hello(crate::db_storage::db_io_test_pool(), storage, document, None, "s".to_string(), protocol::ActorId("semio_hub".to_string()), 0)), Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn handle_frontier_advertise_relays_missing_commands_and_none_when_caught_up() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 2).await;
        let first_state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();
        let replica_summary = state_frontier_summary(&first_state).await;

        {
            let (mut wal, _report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 100)).unwrap();
            let envelope = sample_envelope("op-2", 2).await;
            submit_record(&storage, &mut wal, command_record(&envelope).await, 100).await;
        }

        let frame = db_actor::block_on(handle_frontier_advertise(&storage, document.clone(), &replica_summary, protocol::ActorId("semio_hub".to_string()))).unwrap();
        match frame {
            Some(protocol::ServerFrame::Commands { envelopes, .. }) => {
                assert_eq!(envelopes.len(), 1);
                assert_eq!(envelopes[0].mutation_id.0, "op-2");
            }
            other => panic!("expected a Commands frame, got {other:?}"),
        }

        let up_to_date_state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();
        let up_to_date_summary = state_frontier_summary(&up_to_date_state).await;
        assert!(db_actor::block_on(handle_frontier_advertise(&storage, document, &up_to_date_summary, protocol::ActorId("semio_hub".to_string()))).unwrap().is_none());
    }

    fn held_sync_hello_pool() -> (std::sync::Arc<semio_framework_async::WorkerPool>, std::sync::Arc<std::sync::atomic::AtomicBool>) {
        let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let entered = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let held = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        let worker_entered = entered.clone();
        let worker_held = held.clone();
        pool.try_submit(
            semio_framework_async::Lane::Maintenance,
            Box::new(move || {
                worker_entered.store(true, std::sync::atomic::Ordering::Release);
                while worker_held.load(std::sync::atomic::Ordering::Acquire) {
                    std::thread::yield_now();
                }
            }),
        )
        .unwrap();
        while !entered.load(std::sync::atomic::Ordering::Acquire) {
            std::thread::yield_now();
        }
        (pool, held)
    }

    fn replenishing_sync_hello_io_job(pool: std::sync::Arc<semio_framework_async::WorkerPool>, active: std::sync::Arc<std::sync::atomic::AtomicBool>) -> semio_framework_async::Job {
        Box::new(move || {
            if active.load(std::sync::atomic::Ordering::Acquire) {
                let next = replenishing_sync_hello_io_job(pool.clone(), active.clone());
                if let Err(error) = pool.try_submit(semio_framework_async::Lane::Io, next) {
                    drop(error.into_job());
                }
            }
        })
    }

    fn replenishing_held_sync_hello_pool() -> (std::sync::Arc<semio_framework_async::WorkerPool>, std::sync::Arc<std::sync::atomic::AtomicBool>, std::sync::Arc<std::sync::atomic::AtomicBool>) {
        let (pool, held) = held_sync_hello_pool();
        let active = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        loop {
            let job = replenishing_sync_hello_io_job(pool.clone(), active.clone());
            if let Err(error) = pool.try_submit(semio_framework_async::Lane::Io, job) {
                drop(error.into_job());
                break;
            }
        }
        (pool, held, active)
    }

    async fn retained_sync_hello_storage() -> std::sync::Arc<db_storage::DbBackend> {
        std::sync::Arc::new(db_storage::DbBackend::Memory(MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()))
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_sync_hello_handoff_first_poll_cancel_preserves_exact_owner_and_io_lane() {
        let (pool, held) = held_sync_hello_pool();
        let storage = retained_sync_hello_storage().await;
        let storage_identity = std::sync::Arc::as_ptr(&storage);
        let document = ArtifactId(String::from("p1z-handoff-cancel"));
        let document_identity = document.0.as_ptr();
        let future = DatabaseSyncHelloFuture::try_submit(pool, storage, document, None, String::from("p1z-session"), protocol::ActorId(String::from("p1z-origin")), 4096).unwrap();
        let state = future.state.as_ref().unwrap().clone();
        assert_eq!(state.driver.load(std::sync::atomic::Ordering::Acquire), DatabaseSyncHelloDriverAuthority::Queued as u8);
        future.cancel();
        held.store(false, std::sync::atomic::Ordering::Release);
        let result = future.await.unwrap();
        let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owners = core.execution.as_ref().unwrap().owners.as_ref().unwrap();
        assert_eq!(std::sync::Arc::as_ptr(owners.storage.as_ref().unwrap()), storage_identity);
        assert_eq!(owners.document.0.as_ptr(), document_identity);
        assert!(matches!(core.execution.as_ref().unwrap().prepared, Err(DbError::Closed)));
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_sync_hello_max_plus_one_refusal_keeps_storage_document_frontier_session_origin_identity() {
        let (pool, held) = held_sync_hello_pool();
        let storage = retained_sync_hello_storage().await;
        let mut futures = Vec::new();
        for index in 0..DATABASE_SYNC_HELLO_SLOTS {
            futures.push(DatabaseSyncHelloFuture::try_submit(pool.clone(), storage.clone(), ArtifactId(format!("p1z-max-{index}")), None, format!("session-{index}"), protocol::ActorId(format!("origin-{index}")), 4096).unwrap());
        }
        let document = ArtifactId(String::from("p1z-max-plus-one"));
        let identity = document.0.as_ptr();
        let rejected = DatabaseSyncHelloFuture::try_submit(pool, storage, document, None, String::from("retained-session"), protocol::ActorId(String::from("retained-origin")), 4096).unwrap_err();
        let owners = rejected.close.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(owners.as_ref().unwrap().document.0.as_ptr(), identity);
        assert_eq!(owners.as_ref().unwrap().session_id, "retained-session");
        assert_eq!(owners.as_ref().unwrap().origin.0, "retained-origin");
        drop(owners);
        held.store(false, std::sync::atomic::Ordering::Release);
        for future in futures {
            future.cancel();
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_sync_hello_tail_stream_publishes_welcome_then_one_backpressured_frame() {
        let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
        let memory = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = ArtifactId(String::from("p1z-tail-stream"));
        seed_wal(&memory, &document, 3).await;
        let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
        let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, None, String::from("p1z-tail"), protocol::ActorId(String::from("p1z-origin")), 4096).unwrap().await.unwrap();
        let mut session = result.close_and_take_session().unwrap();
        let welcome = session.take_welcome().unwrap();
        assert!(matches!(welcome.frame().unwrap(), protocol::ServerFrame::Welcome { bootstrap: protocol::Bootstrap::Tail, .. }));
        welcome.acknowledge().unwrap();
        let frame = session.next_frame().await.unwrap().unwrap();
        assert!(matches!(frame.frame().unwrap(), protocol::ServerFrame::Commands { envelopes, .. } if envelopes.len() == 3));
        frame.acknowledge().unwrap();
        assert!(session.next_frame().await.unwrap().is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_sync_hello_snapshot_cursor_copies_at_most_one_page_fragment_per_driver_opportunity() {
        let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
        let memory = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = ArtifactId(String::from("p1z-snapshot-stream"));
        seed_wal(&memory, &document, 4).await;
        let floor = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [1; 32], epoch: 0 };
        publish_snapshot_marker(&memory, &document, 9, floor).await;
        let pages = db_storage::db_io_copy_pages(b"0123456789").unwrap().await.unwrap();
        db_storage::SnapshotStorage::write_generation(&memory, &document, 9, pages).await.unwrap();
        let stale = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0; 32] };
        let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
        let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, Some(stale), String::from("p1z-snapshot"), protocol::ActorId(String::from("p1z-origin")), 4).unwrap().await.unwrap();
        let mut session = result.close_and_take_session().unwrap();
        let welcome = session.take_welcome().unwrap();
        assert!(matches!(welcome.frame().unwrap(), protocol::ServerFrame::Welcome { bootstrap: protocol::Bootstrap::Snapshot { inline: None, .. }, .. }));
        welcome.acknowledge().unwrap();
        for (seq, bytes) in [(0, b"0123".as_slice()), (1, b"4567".as_slice()), (2, b"89".as_slice())] {
            let frame = session.next_frame().await.unwrap().unwrap();
            assert!(matches!(frame.frame().unwrap(), protocol::ServerFrame::SnapshotChunk { seq: actual, bytes: actual_bytes } if *actual == seq && actual_bytes.as_slice() == bytes));
            frame.acknowledge().unwrap();
        }
        let done = session.next_frame().await.unwrap().unwrap();
        assert!(matches!(done.frame().unwrap(), protocol::ServerFrame::SnapshotDone { seq_count: 3 }));
        done.acknowledge().unwrap();
        assert!(session.next_frame().await.unwrap().is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_sync_hello_returned_snapshot_credit_waits_for_exact_generation_ack() {
        let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
        let memory = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = ArtifactId(String::from("p1z-returned-credit"));
        seed_wal(&memory, &document, 2).await;
        let floor = Frontier { document: document.clone(), head_seq: 2, commit_seq: 2, chain_hash: [2; 32], epoch: 0 };
        publish_snapshot_marker(&memory, &document, 7, floor).await;
        let pages = db_storage::db_io_copy_pages(b"abcdefgh").unwrap().await.unwrap();
        db_storage::SnapshotStorage::write_generation(&memory, &document, 7, pages).await.unwrap();
        let stale = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0; 32] };
        let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
        let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, Some(stale), String::from("p1z-returned-credit-session"), protocol::ActorId(String::from("p1z-returned-credit-origin")), 4).unwrap().await.unwrap();
        let mut session = result.close_and_take_session().unwrap();
        session.take_welcome().unwrap().acknowledge().unwrap();
        let first = session.next_frame().await.unwrap().unwrap();
        let state = session.state.as_ref().unwrap().clone();
        let (generation, items, bytes, ledger_before) = {
            let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let lease = core.returned_frame.as_ref().unwrap();
            let ledger = core.execution.as_ref().unwrap().ledger.as_ref().unwrap();
            (lease.generation, lease.items, lease.bytes, (ledger.items, ledger.bytes))
        };
        assert!(generation != 0 && items == 1 && bytes >= 4);
        let mut next = Box::pin(session.next_frame());
        let waker = std::task::Waker::noop();
        let mut context = std::task::Context::from_waker(waker);
        assert!(matches!(std::future::Future::poll(next.as_mut(), &mut context), std::task::Poll::Pending));
        {
            let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let lease = core.returned_frame.as_ref().unwrap();
            let ledger = core.execution.as_ref().unwrap().ledger.as_ref().unwrap();
            assert_eq!(lease.generation, generation);
            assert_eq!((ledger.items, ledger.bytes), ledger_before);
            assert!(core.frame.is_none());
        }
        first.acknowledge().unwrap();
        let second = next.await.unwrap().unwrap();
        assert!(matches!(second.frame().unwrap(), protocol::ServerFrame::SnapshotChunk { seq: 1, .. }));
        second.acknowledge().unwrap();
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_sync_hello_maximum_snapshot_request_stays_page_unit_bounded() {
        let mut page_ledger = DatabaseSyncHelloBackingLedger::default();
        let page_reservation = database_sync_hello_reserve_snapshot_pages(&mut page_ledger).unwrap();
        assert_eq!((page_ledger.items, page_ledger.bytes), (DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_ITEMS, DATABASE_SYNC_HELLO_SNAPSHOT_PAGE_BYTES));
        let mut page_owner = db_storage::db_io_copy_pages(b"fixed-page-owner").unwrap().await.unwrap();
        let (page_items, page_bytes) = page_reservation.observed(&page_owner).unwrap();
        assert_eq!((page_items, page_bytes), (1, db_storage::DB_IO_PAGE_BYTES));
        page_reservation.settle(&mut page_ledger, page_items, page_bytes).unwrap();
        while page_owner.close_step().unwrap().is_some() {}
        page_ledger.release(page_items, page_bytes).unwrap();
        assert!(page_owner.terminal_is_empty() && page_ledger.terminal_is_empty());

        let mut fixed_ledger = DatabaseSyncHelloBackingLedger::default();
        let fixed_reservation = database_sync_hello_reserve_snapshot_chunk(&mut fixed_ledger, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES).unwrap();
        assert_eq!((fixed_ledger.items, fixed_ledger.bytes), (1, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES));
        assert_eq!(fixed_reservation.bytes, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES);
        let fixed = fixed_reservation.allocate(&mut fixed_ledger).unwrap();
        assert_eq!(fixed.backing_bytes(), DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES);
        let fixed_frame = protocol::ServerFrame::SnapshotChunk { seq: 0, bytes: fixed };
        let (fixed_items, fixed_bytes) = database_sync_hello_returned_frame_credit(&fixed_frame).unwrap();
        let mut fixed_lease = DatabaseSyncHelloReturnedFrameLease { generation: 1, items: fixed_items, bytes: fixed_bytes, close: Some(DatabaseSyncHelloFrameClose { owner: Some(fixed_frame), envelope: None }) };
        assert_eq!((fixed_lease.items, fixed_lease.bytes), (1, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES));
        assert!(fixed_lease.close.as_mut().unwrap().close_one());
        assert!(fixed_lease.close.as_mut().unwrap().close_one());
        assert!(fixed_lease.close.as_ref().unwrap().terminal_is_empty());
        fixed_ledger.release(fixed_lease.items, fixed_lease.bytes).unwrap();
        assert!(fixed_ledger.terminal_is_empty());

        let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
        let memory = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = ArtifactId(String::from("p1z-maximum-frame-unit"));
        seed_wal(&memory, &document, 2).await;
        let floor = Frontier { document: document.clone(), head_seq: 2, commit_seq: 2, chain_hash: [3; 32], epoch: 0 };
        publish_snapshot_marker(&memory, &document, 11, floor).await;
        let source = vec![b'x'; DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES * 2 + 1];
        let pages = db_storage::db_io_copy_pages(&source).unwrap().await.unwrap();
        db_storage::SnapshotStorage::write_generation(&memory, &document, 11, pages).await.unwrap();
        let stale = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0; 32] };
        let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
        let result =
            DatabaseSyncHelloFuture::try_submit(pool, storage, document, Some(stale), String::from("p1z-maximum-frame-session"), protocol::ActorId(String::from("p1z-maximum-frame-origin")), DATABASE_SYNC_HELLO_MAX_BYTES).unwrap().await.unwrap();
        let mut session = result.close_and_take_session().unwrap();
        session.take_welcome().unwrap().acknowledge().unwrap();
        for (seq, expected) in [(0, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES), (1, DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES), (2, 1)] {
            let frame = session.next_frame().await.unwrap().unwrap();
            assert!(matches!(frame.frame().unwrap(), protocol::ServerFrame::SnapshotChunk { seq: actual, bytes } if *actual == seq && bytes.len() == expected && bytes.backing_bytes() == DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES));
            frame.acknowledge().unwrap();
        }
        let done = session.next_frame().await.unwrap().unwrap();
        assert!(matches!(done.frame().unwrap(), protocol::ServerFrame::SnapshotDone { seq_count: 3 }));
        done.acknowledge().unwrap();
        assert!(session.next_frame().await.unwrap().is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_sync_hello_grant_deadline_between_allocation_copy_and_publication_retains_credit() {
        let cancelled = std::sync::atomic::AtomicBool::new(false);
        let expired = std::sync::atomic::AtomicBool::new(false);
        let document = ArtifactId(String::from("p1z-grant-deadline"));
        let source = vec![b'g'; DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES];
        let pages = db_storage::db_io_copy_pages(&source).unwrap().await.unwrap();
        let mut follow_up = DatabaseSyncHelloFollowUp::Snapshot { pages, chunk_bytes: DATABASE_SYNC_HELLO_MAX_BYTES, offset: 0, page: 0, page_offset: 0, seq: 0, chunk: None, done: false };
        let mut ledger = DatabaseSyncHelloBackingLedger::default();
        let mut before_copy = DatabaseSyncHelloGrant::expiring_at(3);
        assert!(matches!(follow_up.drive_one_with_grant(&mut ledger, &cancelled, &expired, &mut before_copy), Err(DbError::Timeout("database sync hello 8 ms grant"))));
        assert!(matches!(&follow_up, DatabaseSyncHelloFollowUp::Snapshot { chunk: Some(chunk), offset: 0, seq: 0, .. } if chunk.is_empty() && chunk.backing_bytes() == DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES));
        assert_eq!(ledger.items, 1);
        assert!(ledger.bytes <= DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES);
        while follow_up.close_one().unwrap() {}
        while ledger.close_one_credit() {}
        assert!(follow_up.terminal_is_empty() && ledger.terminal_is_empty());

        let pages = db_storage::db_io_copy_pages(&source).unwrap().await.unwrap();
        let mut follow_up = DatabaseSyncHelloFollowUp::Snapshot { pages, chunk_bytes: DATABASE_SYNC_HELLO_MAX_BYTES, offset: 0, page: 0, page_offset: 0, seq: 0, chunk: None, done: false };
        let mut ledger = DatabaseSyncHelloBackingLedger::default();
        let mut before_publication = DatabaseSyncHelloGrant::expiring_at(5);
        assert!(matches!(follow_up.drive_one_with_grant(&mut ledger, &cancelled, &expired, &mut before_publication), Err(DbError::Timeout("database sync hello 8 ms grant"))));
        assert!(matches!(&follow_up, DatabaseSyncHelloFollowUp::Snapshot { chunk: Some(chunk), offset, seq: 0, .. } if chunk.len() == DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES && *offset == DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES));
        assert_eq!(ledger.items, 1);
        assert!(ledger.bytes <= DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES);
        while follow_up.close_one().unwrap() {}
        while ledger.close_one_credit() {}
        assert!(follow_up.terminal_is_empty() && ledger.terminal_is_empty());

        let independently_expired = std::sync::atomic::AtomicBool::new(true);
        assert!(matches!(database_sync_hello_control(&cancelled, &independently_expired), Err(DbError::Timeout("database sync hello deadline"))));
        assert_eq!(document.0, "p1z-grant-deadline");
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_sync_hello_cancel_before_stream_demand_publishes_no_new_frame() {
        let pool = std::sync::Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
        let memory = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = ArtifactId(String::from("p1z-cancel-before-stream"));
        seed_wal(&memory, &document, 2).await;
        let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(memory));
        let result = DatabaseSyncHelloFuture::try_submit(pool, storage, document, None, String::from("p1z-cancel-stream-session"), protocol::ActorId(String::from("p1z-cancel-stream-origin")), 4_096).unwrap().await.unwrap();
        let mut session = result.close_and_take_session().unwrap();
        session.take_welcome().unwrap().acknowledge().unwrap();
        let state = session.state.as_ref().unwrap().clone();
        session.cancel();
        assert!(matches!(session.next_frame().await, Err(DbError::Closed)));
        assert!(!state.demand.load(std::sync::atomic::Ordering::Acquire));
        let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(!matches!(core.frame.as_ref(), Some(Ok(Some(_)))));
    }

    #[test]
    fn retained_sync_hello_cumulative_actual_backing_rejects_max_plus_one_without_mutation() {
        let mut ledger = DatabaseSyncHelloBackingLedger::default();
        ledger.observe(DATABASE_SYNC_HELLO_MAX_ITEMS, DATABASE_SYNC_HELLO_MAX_BYTES, "maximum").unwrap();
        let before = (ledger.items, ledger.bytes);
        assert!(ledger.observe(1, 0, "cumulative max plus one").is_err());
        assert_eq!((ledger.items, ledger.bytes), before);
        assert!(ledger.observe(0, 1, "cumulative max plus one").is_err());
        assert_eq!((ledger.items, ledger.bytes), before);
    }

    #[test]
    fn retained_sync_hello_predebits_envelope_clone_and_overallocation_before_owner_construction() {
        let mut ledger = DatabaseSyncHelloBackingLedger::default();
        let mut bytes = database_sync_hello_allocate_envelope_vec::<u8>(&mut ledger, 4_096).unwrap();
        assert_eq!(ledger.items, 1);
        assert_eq!(ledger.bytes, bytes.capacity());
        database_sync_hello_retire_vec(&mut bytes, &mut ledger).unwrap();
        let source = String::from("p1z-predebit");
        let owner = database_sync_hello_clone_string(&source, &mut ledger, "database sync hello law clone backing").unwrap();
        assert_ne!(source.as_ptr(), owner.as_ptr());
        assert_eq!(ledger.bytes, owner.capacity());
        let before = (ledger.items, ledger.bytes);
        assert!(database_sync_hello_allocate_envelope_vec::<u8>(&mut ledger, DATABASE_SYNC_HELLO_MAX_BYTES).is_err());
        assert_eq!((ledger.items, ledger.bytes), before);
    }

    #[test]
    fn retained_sync_hello_cancel_between_yield_and_resume_prevents_next_wal_backend_operation() {
        let cancelled = std::sync::atomic::AtomicBool::new(false);
        let expired = std::sync::atomic::AtomicBool::new(false);
        let mut opportunity = std::pin::pin!(database_sync_hello_opportunity(&cancelled, &expired));
        let waker = std::task::Waker::noop();
        let mut context = std::task::Context::from_waker(waker);
        assert!(matches!(std::future::Future::poll(opportunity.as_mut(), &mut context), std::task::Poll::Pending));
        cancelled.store(true, std::sync::atomic::Ordering::Release);
        assert!(matches!(std::future::Future::poll(opportunity.as_mut(), &mut context), std::task::Poll::Ready(Err(DbError::Closed))));
        assert!(database_sync_hello_control(&cancelled, &expired).is_err());
    }

    #[test]
    fn retained_sync_hello_quarantine_cursor_and_byte_item_ledger_reach_zero_before_release() {
        let future: DatabaseSyncHelloExecutionFuture = Box::pin(std::future::pending());
        let bytes = std::mem::size_of_val(&*future);
        let mut quarantine = DatabaseSyncHelloQuarantineClose { future: Some(future), items: 1, bytes };
        assert!(quarantine.close_one());
        assert!(quarantine.terminal_is_empty());
        let mut ledger = DatabaseSyncHelloBackingLedger::default();
        ledger.observe(2, db_storage::DB_IO_PAGE_BYTES + 1, "database sync hello quarantine law backing").unwrap();
        assert!(ledger.close_one_credit());
        assert_eq!(ledger.bytes, 1);
        while ledger.close_one_credit() {}
        assert!(ledger.terminal_is_empty());
    }

    #[test]
    fn retained_sync_hello_page_close_error_is_typed_and_blocks_terminal_release() {
        let mut fault = None;
        let pending = database_sync_hello_apply_follow_up_close_result(&mut fault, false, Err(DbError::Internal("p1z retained page-close fault".to_string())));
        assert!(pending);
        let retained = fault.as_ref().unwrap();
        assert_eq!(retained.attempts, 1);
        assert!(matches!(retained.error.as_ref(), Some(DbError::Internal(detail)) if detail == "p1z retained page-close fault"));
        assert!(!retained.terminal_is_empty());
        assert!(fault.as_mut().unwrap().close_one());
        assert!(fault.as_ref().unwrap().terminal_is_empty());
    }

    #[test]
    fn retained_sync_hello_refusal_retry_saturation_is_bounded_and_retains_terminal_job_owner() {
        let (pool, held, active) = replenishing_held_sync_hello_pool();
        let rejected = DatabaseSyncHelloRejected::new(
            pool,
            DbError::Unavailable("p1z refusal saturation law".to_string()),
            DatabaseSyncHelloOwners {
                storage: None,
                document: ArtifactId(String::from("p1z-refusal-bounded")),
                hello_frontier: None,
                session_id: String::from("p1z-refusal-session"),
                origin: protocol::ActorId(String::from("p1z-refusal-origin")),
                snapshot_chunk_bytes: 4_096,
            },
        );
        let close = rejected.close.clone();
        let generation = rejected.retirement_generation();
        assert!(matches!(rejected.close_and_take_error(), DbError::Unavailable(_)));
        let witness = database_sync_hello_rejected_terminal_witness(generation).unwrap();
        assert!(witness.owners_retained && witness.job_retained && witness.submissions == 1);
        assert_eq!(close.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().unwrap().document.0, "p1z-refusal-bounded");
        assert!(close.submissions.load(std::sync::atomic::Ordering::Acquire) <= DATABASE_SYNC_HELLO_RETRY_LIMIT);
        active.store(false, std::sync::atomic::Ordering::Release);
        held.store(false, std::sync::atomic::Ordering::Release);
    }

    #[test]
    fn retained_sync_hello_forever_stuck_sole_worker_guarantees_discoverable_ownership_only() {
        let (pool, held) = held_sync_hello_pool();
        loop {
            if let Err(error) = pool.try_submit(semio_framework_async::Lane::Io, Box::new(|| {})) {
                drop(error.into_job());
                break;
            }
        }
        let rejected = DatabaseSyncHelloRejected::new(
            pool,
            DbError::Unavailable("p1z forever-stuck sole worker".to_string()),
            DatabaseSyncHelloOwners {
                storage: None,
                document: ArtifactId(String::from("p1z-discoverable-only")),
                hello_frontier: None,
                session_id: String::from("p1z-discoverable-session"),
                origin: protocol::ActorId(String::from("p1z-discoverable-origin")),
                snapshot_chunk_bytes: 4_096,
            },
        );
        let generation = rejected.retirement_generation();
        rejected.close_and_take_error();
        let witness = database_sync_hello_rejected_terminal_witness(generation).unwrap();
        assert!(witness.owners_retained && witness.job_retained);
        assert!(witness.submissions <= DATABASE_SYNC_HELLO_RETRY_LIMIT);
        held.store(false, std::sync::atomic::Ordering::Release);
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_sync_hello_deadline_retry_drop_close_retains_registry_until_worker_service() {
        let (pool, held) = held_sync_hello_pool();
        let storage = retained_sync_hello_storage().await;
        let future = DatabaseSyncHelloFuture::try_submit(pool, storage, ArtifactId(String::from("p1z-deadline")), None, String::from("deadline-session"), protocol::ActorId(String::from("deadline-origin")), 4096).unwrap();
        let state = future.state.as_ref().unwrap().clone();
        state.deadline_callback();
        drop(future);
        assert!(state.expired.load(std::sync::atomic::Ordering::Acquire));
        assert!(state.cancelled.load(std::sync::atomic::Ordering::Acquire));
        assert!(matches!(database_sync_hello_control(&state.cancelled, &state.expired), Err(DbError::Timeout("database sync hello deadline"))));
        assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
        assert!(database_sync_hello_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[state.slot].is_some());
        held.store(false, std::sync::atomic::Ordering::Release);
    }

    #[test]
    fn retained_sync_hello_ready_pending_panic_and_repeat_poll_have_typed_terminal_states() {
        let source = include_str!("🦀️component.rs");
        assert!(source.contains("Ok(std::task::Poll::Pending)"));
        assert!(source.contains("Ok(std::task::Poll::Ready(execution))"));
        assert!(source.contains("core.quarantined = Some(DatabaseSyncHelloQuarantineClose"));
        assert!(source.contains("if self.completed"));
        assert!(source.contains("std::task::Poll::Ready(Err(DbError::Closed))"));
    }

    #[test]
    fn retained_sync_hello_production_census_has_zero_blocking_waits_and_no_eager_follow_up() {
        let engine = include_str!("../../⚙️engine/🦀️component.rs");
        let production = engine.split("//#region 🧪️Tests").next().unwrap();
        assert!(!production.contains("db_actor::block_on(db_sync::handle_hello"));
        assert!(production.contains("DatabaseSyncHelloFuture::try_submit"));
        let source = include_str!("🦀️component.rs");
        assert!(source.contains("#[cfg(test)]\npub async fn handle_hello"));
        assert!(source.contains("DatabaseSyncHelloNextFuture"));
    }
    //#endregion 🔖️Hello
}
//#endregion 🧪️Tests
