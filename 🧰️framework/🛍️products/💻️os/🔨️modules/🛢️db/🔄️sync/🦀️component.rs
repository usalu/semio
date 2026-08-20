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
use crate::*;
use crate::db_durability::Frontier;
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
    protocol::encode_envelope(envelope, &mut out).await;
    out
}

/// @emoji 📖️ Inverse of `encode_command_envelope`. Validates the byte length against
/// `DbLimits::default().max_command_bytes` BEFORE decoding anything sized by it (mirrors
/// `pack_core`'s stated invariant), then maps a decode failure to `DbError::Corrupt` rather than
/// leaking `protocol::ProtocolError`.
pub async fn decode_command_envelope(bytes: &[u8]) -> Result<protocol::MutationEnvelope, DbError> {
    check_len(bytes.len() as u64, DbLimits::default().max_command_bytes, "wal_command_envelope")?;
    let mut pos = 0usize;
    let envelope = protocol::decode_envelope(bytes, &mut pos).await.map_err(|error| DbError::Corrupt(format!("malformed wal command envelope: {error}")))?;
    Ok(envelope)
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
    let records = db_wal::replay_document(storage, &document).await?;
    let mut commands = Vec::new();
    let mut command_digests: Vec<[u8; 32]> = Vec::new();
    let mut commit_seq = 0u64;
    let mut floor_head_seq = 0u64;
    for record in &records {
        match record {
            db_wal::WalRecord::Command(bytes) => {
                commands.push(decode_command_envelope(bytes).await?);
                command_digests.push(*blake3::hash(bytes).as_bytes());
            }
            db_wal::WalRecord::TxCommit { .. } => commit_seq += 1,
            // 🎯️ Overwritten on every occurrence rather than max()'d: `WalRecord`s replay in
            // on-disk (chronological) order, so the last one seen is always the most recent.
            db_wal::WalRecord::SnapshotPub { frontier, .. } => floor_head_seq = frontier.head_seq,
            _ => {}
        }
    }
    let head_seq = commands.len() as u64;
    let chain_hash = fold_content_chain(&command_digests);
    let frontier = Frontier { document, head_seq, commit_seq, chain_hash, epoch: 0 };
    Ok(ArtifactSyncState { frontier, commands, floor_head_seq })
}

/// @emoji 🔐️ Folds per-command digests into one combined digest — see `ArtifactSyncState`'s doc
/// for the derivation this implements. `[0u8; 32]` for an empty document, matching
/// `Frontier::genesis`'s all-zero `chain_hash`.
// 🚫️async: E1 pure accessor, always used inline in a struct-literal field position — see R9
fn fold_content_chain(digests: &[[u8; 32]]) -> [u8; 32] {
    if digests.is_empty() {
        return [0u8; 32];
    }
    let mut concat = Vec::with_capacity(digests.len() * 32);
    for digest in digests {
        concat.extend_from_slice(digest);
    }
    *blake3::hash(&concat).as_bytes()
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
#[derive(Clone, Debug, PartialEq)]
pub enum BootstrapPlan {
    /// @emoji ✅️ The replica is already fully caught up — nothing to send.
    None,
    /// @emoji 🚚️ The replica is within the retained WAL floor: ship it the missing commands
    /// directly, no snapshot needed.
    Tail { envelopes: Vec<protocol::MutationEnvelope> },
    /// @emoji 📸️ The replica is behind the retained WAL floor (or brand new against a compacted
    /// document): ship it a whole snapshot generation first.
    Snapshot { generation: u64, bytes: Vec<u8>, pack_hash: [u8; 32] },
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
        .latest_generation(&state.frontier.document).await?
        .ok_or_else(|| DbError::Unavailable(format!("replica head_seq {replica_head_seq} is behind the retained WAL floor {} and no snapshot generation is available", state.floor_head_seq)))?;
    let bytes = snapshots.read_generation(&state.frontier.document, generation).await?;
    let pack_hash = *blake3::hash(&bytes).as_bytes();
    Ok(BootstrapPlan::Snapshot { generation, bytes, pack_hash })
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
/// @emoji 🚀️ What `handle_hello` produces: the `Welcome` frame itself, plus whatever follow-up
/// frames the chosen bootstrap needs (a single `Commands` frame for `Tail`; one `SnapshotChunk`
/// per chunk plus a trailing `SnapshotDone` for a non-inlined `Snapshot`; none for `None` or an
/// inlined `Snapshot`, whose bytes already travel inside `Welcome.bootstrap`).
#[derive(Clone, Debug, PartialEq)]
pub struct WelcomeResponse {
    pub welcome: protocol::ServerFrame,
    pub follow_up: Vec<protocol::ServerFrame>,
}

/// @emoji 🏗️ Lowers a `BootstrapPlan` to the wire `protocol::Bootstrap` shape plus its follow-up
/// frames. A `Snapshot` whose bytes fit within `snapshot_chunk_bytes` is inlined directly into
/// `Bootstrap::Snapshot.inline` (no follow-up frames); a larger one is chunked instead — this
/// crate's own choice of threshold behavior, since the contract fixes `Bootstrap::Snapshot`'s two
/// shapes but not when to prefer one over the other.
async fn lower_bootstrap_plan(plan: &BootstrapPlan, state: &ArtifactSyncState, origin: &protocol::ActorId, snapshot_chunk_bytes: usize) -> (protocol::Bootstrap, Vec<protocol::ServerFrame>) {
    match plan {
        BootstrapPlan::None => (protocol::Bootstrap::None, Vec::new()),
        BootstrapPlan::Tail { envelopes } => (protocol::Bootstrap::Tail, vec![commands_server_frame(state, envelopes.clone(), origin.clone()).await]),
        BootstrapPlan::Snapshot { bytes, pack_hash, .. } => {
            if bytes.len() <= snapshot_chunk_bytes {
                (protocol::Bootstrap::Snapshot { pack_hash: *pack_hash, inline: Some(bytes.clone()) }, Vec::new())
            } else {
                let chunks: Vec<&[u8]> = bytes.chunks(snapshot_chunk_bytes).collect();
                let mut follow_up: Vec<protocol::ServerFrame> = chunks.iter().enumerate().map(|(seq, chunk)| protocol::ServerFrame::SnapshotChunk { seq: seq as u32, bytes: chunk.to_vec() }).collect();
                follow_up.push(protocol::ServerFrame::SnapshotDone { seq_count: chunks.len() as u32 });
                (protocol::Bootstrap::Snapshot { pack_hash: *pack_hash, inline: None }, follow_up)
            }
        }
    }
}

/// @emoji 🏗️ Builds the full `WelcomeResponse` for `plan` against `state`. `snapshot_chunk_bytes`
/// must be non-zero (validated before `lower_bootstrap_plan` could otherwise divide the snapshot
/// into a runaway number of zero-progress chunks).
pub async fn build_welcome(state: &ArtifactSyncState, plan: &BootstrapPlan, session_id: String, origin: &protocol::ActorId, snapshot_chunk_bytes: usize) -> Result<WelcomeResponse, DbError> {
    if snapshot_chunk_bytes == 0 {
        return Err(DbError::InvalidArgument("snapshot_chunk_bytes must be non-zero".to_string()));
    }
    let resume_token = issue_resume_token(&state.frontier).await?;
    let (bootstrap, follow_up) = lower_bootstrap_plan(plan, state, origin, snapshot_chunk_bytes).await;
    let welcome = protocol::ServerFrame::Welcome { session_id, resume_token, server_frontier: state_frontier_summary(state).await, bootstrap };
    Ok(WelcomeResponse { welcome, follow_up })
}

/// @emoji 👋️ The top-level entry point for a `protocol::ClientFrame::Hello`: replays `document`'s
/// current sync state, decides a bootstrap plan against `hello_frontier` (the replica's advertised
/// `RuntimeFrontierSummary`, `None` for a totally fresh replica — see module doc for why this
/// crate reads `Hello.frontier` rather than decoding `Hello.resume_token`), and lowers it to a
/// `WelcomeResponse`.
pub async fn handle_hello(
    storage: &db_storage::DbBackend,
    document: ArtifactId,
    hello_frontier: Option<&protocol::RuntimeFrontierSummary>,
    session_id: String,
    origin: &protocol::ActorId,
    snapshot_chunk_bytes: usize,
) -> Result<WelcomeResponse, DbError> {
    let state = replay_sync_state(&storage.wal().await, document).await?;
    let replica = hello_frontier.map(from_frontier_summary);
    let plan = decide_bootstrap(&state, &storage.snapshot().await, replica.as_ref()).await?;
    build_welcome(&state, &plan, session_id, origin, snapshot_chunk_bytes).await
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
    use ArtifactId;
    use db_storage::MemoryStorage;
    use db_wal::{ArtifactWal, GroupCommitPolicy, WalRecord};

    //#region 🧸️Fixtures
    async fn sample_envelope(id: &str, seq: u64) -> protocol::MutationEnvelope {
        protocol::MutationEnvelope {
            mutation_id: protocol::MutationId(id.to_string()),
            document_id: protocol::ArtifactId("doc-1".to_string()),
            actor: protocol::ActorId("actor-1".to_string()),
            dependencies: Vec::new(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId("diff.v1".to_string()), payload: seq.to_le_bytes().to_vec() },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId("diff.v1".to_string()), payload: Vec::new() },
            timestamp: protocol::HybridLogicalTimestamp::new(1, seq).await,
        }
    }

    /// @emoji 🧸️ Creates `document`'s WAL in `storage` and submits `count` sample commands
    /// (ids `"op-0".."op-{count-1}"`), each `Fsync`-durable so replay sees them immediately.
    async fn seed_wal(storage: &MemoryStorage, document: &ArtifactId, count: u64) {
        let mut wal = db_actor::block_on(ArtifactWal::create(storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
        for i in 0..count {
            let envelope = sample_envelope(&format!("op-{i}"), i).await;
            let bytes = encode_command_envelope(&envelope).await;
            db_actor::block_on(wal.submit(storage, &[WalRecord::Command(bytes)], DurabilityClass::Fsync, i)).unwrap();
        }
    }

    /// @emoji 🧸️ Reopens `document`'s WAL and appends one `SnapshotPub` marker covering `frontier`.
    async fn publish_snapshot_marker(storage: &MemoryStorage, document: &ArtifactId, generation: u64, frontier: Frontier) {
        let (mut wal, _report) = db_actor::block_on(ArtifactWal::open(storage, document.clone(), GroupCommitPolicy::default(), 1000)).unwrap();
        db_actor::block_on(wal.submit(storage, &[WalRecord::SnapshotPub { generation, frontier }], DurabilityClass::Fsync, 1000)).unwrap();
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
        let storage = MemoryStorage::new().await;
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
        let storage = MemoryStorage::new().await;
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 0).await;

        let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();
        assert_eq!(state.frontier.head_seq, 0);
        assert_eq!(state.frontier.chain_hash, [0u8; 32]);
        assert!(state.commands.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn replay_sync_state_tracks_the_latest_snapshot_pub_as_the_floor() {
        let storage = MemoryStorage::new().await;
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
        let storage = MemoryStorage::new().await;
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
        let storage = MemoryStorage::new().await;
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
        let storage = MemoryStorage::new().await;
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 3).await;
        let first_state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();
        let replica_frontier = first_state.frontier;

        // More commands land on the server after the replica already caught up once.
        {
            let (mut wal, _report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 100)).unwrap();
            for i in 3..6u64 {
                let envelope = sample_envelope(&format!("op-{i}"), i).await;
                db_actor::block_on(wal.submit(&storage, &[WalRecord::Command(encode_command_envelope(&envelope).await)], DurabilityClass::Fsync, i)).unwrap();
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
        let storage = MemoryStorage::new().await;
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
        let storage = MemoryStorage::new().await;
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
        let storage = MemoryStorage::new().await;
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 3).await;
        let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();

        let plan = db_actor::block_on(decide_bootstrap(&state, &storage, None)).unwrap();
        assert_eq!(plan, BootstrapPlan::Tail { envelopes: state.commands });
    }

    #[semio_framework_async_macros::async_test]
    async fn decide_bootstrap_reports_none_for_an_already_caught_up_replica() {
        let storage = MemoryStorage::new().await;
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 3).await;
        let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();

        let plan = db_actor::block_on(decide_bootstrap(&state, &storage, Some(&state.frontier))).unwrap();
        assert_eq!(plan, BootstrapPlan::None);
    }

    #[semio_framework_async_macros::async_test]
    async fn decide_bootstrap_serves_snapshot_when_a_generation_is_available_below_the_floor() {
        let storage = MemoryStorage::new().await;
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 5).await;
        let floor_frontier = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [3u8; 32], epoch: 0 };
        publish_snapshot_marker(&storage, &document, 7, floor_frontier).await;
        db_actor::block_on(db_storage::SnapshotStorage::write_generation(&storage, &document, 7, b"snapshot-bytes")).unwrap();
        let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();

        let stale_replica = Frontier { document, head_seq: 0, commit_seq: 0, chain_hash: [0u8; 32], epoch: 0 };
        let plan = db_actor::block_on(decide_bootstrap(&state, &storage, Some(&stale_replica))).unwrap();
        match plan {
            BootstrapPlan::Snapshot { generation, bytes, pack_hash } => {
                assert_eq!(generation, 7);
                assert_eq!(bytes, b"snapshot-bytes");
                assert_eq!(pack_hash, *blake3::hash(b"snapshot-bytes").as_bytes());
            }
            other => panic!("expected a Snapshot plan, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn decide_bootstrap_reports_unavailable_when_below_floor_with_no_snapshot() {
        let storage = MemoryStorage::new().await;
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
        let storage = MemoryStorage::new().await;
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 3).await;
        let storage: db_storage::DbBackend = db_storage::DbBackend::Memory(storage);

        let response = db_actor::block_on(handle_hello(&storage, document, None, "session-1".to_string(), &protocol::ActorId("semio_hub".to_string()), 64 * 1024)).unwrap();
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
        let storage = MemoryStorage::new().await;
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 2).await;
        let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();
        let hello_frontier = state_frontier_summary(&state).await;
        let storage: db_storage::DbBackend = db_storage::DbBackend::Memory(storage);

        let response = db_actor::block_on(handle_hello(&storage, document, Some(&hello_frontier), "session-2".to_string(), &protocol::ActorId("semio_hub".to_string()), 64 * 1024)).unwrap();
        let protocol::ServerFrame::Welcome { bootstrap, .. } = &response.welcome else {
            panic!("expected a Welcome frame");
        };
        assert_eq!(*bootstrap, protocol::Bootstrap::None);
        assert!(response.follow_up.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn handle_hello_chunks_a_snapshot_larger_than_the_requested_chunk_size() {
        let storage = MemoryStorage::new().await;
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 4).await;
        let floor_frontier = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [1u8; 32], epoch: 0 };
        publish_snapshot_marker(&storage, &document, 9, floor_frontier).await;
        let big_snapshot = vec![7u8; 10];
        db_actor::block_on(db_storage::SnapshotStorage::write_generation(&storage, &document, 9, &big_snapshot)).unwrap();
        let storage: db_storage::DbBackend = db_storage::DbBackend::Memory(storage);

        let stale_hello_frontier = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0u8; 32] };
        let response = db_actor::block_on(handle_hello(&storage, document, Some(&stale_hello_frontier), "session-3".to_string(), &protocol::ActorId("semio_hub".to_string()), 4)).unwrap();

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
        let storage = MemoryStorage::new().await;
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 1).await;
        let storage: db_storage::DbBackend = db_storage::DbBackend::Memory(storage);
        assert!(matches!(db_actor::block_on(handle_hello(&storage, document, None, "s".to_string(), &protocol::ActorId("semio_hub".to_string()), 0)), Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn handle_frontier_advertise_relays_missing_commands_and_none_when_caught_up() {
        let storage = MemoryStorage::new().await;
        let document: ArtifactId = "doc-1".into();
        seed_wal(&storage, &document, 2).await;
        let first_state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();
        let replica_summary = state_frontier_summary(&first_state).await;

        {
            let (mut wal, _report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 100)).unwrap();
            let envelope = sample_envelope("op-2", 2).await;
            db_actor::block_on(wal.submit(&storage, &[WalRecord::Command(encode_command_envelope(&envelope).await)], DurabilityClass::Fsync, 100)).unwrap();
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
    //#endregion 🔖️Hello
}
//#endregion 🧪️Tests
