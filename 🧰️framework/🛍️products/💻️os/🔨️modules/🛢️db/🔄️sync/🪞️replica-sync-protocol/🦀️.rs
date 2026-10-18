//! 🪞️ Replica sync protocol: the command envelope and wal command codec, the derived replica sync state with its replay, the frontier delta and summaries, the missing commands, the bootstrap plan and the resume token. See [`super`].

use super::*;

/// ✉️ This crate's own convention for `db_wal::WalRecord::Command`'s payload bytes:
/// `protocol_io::binary::causal::encode_envelope`'s binary record — the same primitive codec `protocol_wire`
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

/// 📖️ Inverse of `encode_command_envelope`. Validates the byte length against
/// `DbLimits::default().max_command_bytes` BEFORE decoding anything sized by it (mirrors
/// `pack_core`'s stated invariant), then maps a decode failure to `DbError::Corrupt` rather than
/// leaking `protocol::ProtocolError`.
#[cfg(any(test, feature = "testkit"))]
pub async fn decode_command_envelope(bytes: &[u8]) -> Result<protocol::MutationEnvelope, DbError> {
    check_len(bytes.len() as u64, DbLimits::default().max_command_bytes, "wal_command_envelope")?;
    let mut pos = 0usize;
    let envelope = protocol::decode_envelope(bytes, &mut pos).map_err(|error| DbError::Corrupt(format!("malformed wal command envelope: {error}")))?;
    Ok(envelope)
}

/// 🧷️ Hands `read` one WAL command record as a contiguous slice: a record held in one page is read
/// in place, a longer one is gathered once (a record spans at most one operation's pages).
fn with_wal_command<T>(bytes: &db_wal_log::WalBytes, read: impl FnOnce(&[u8]) -> T) -> T {
    let mut fragments = bytes.fragments();
    match (fragments.next(), fragments.next()) {
        (Some(only), None) => read(only),
        (None, _) => read(&[]),
        _ => {
            let mut gathered = Vec::with_capacity(bytes.len());
            for fragment in bytes.fragments() {
                gathered.extend_from_slice(fragment);
            }
            read(&gathered)
        }
    }
}

/// 📜️ Inverse of `encode_command_envelope` over a WAL-held command record, in one bounded step: the
/// record is already in memory, so a decode never yields — replay and hello walks batch whole records
/// into their cooperative turns (`db_wal::WalTurn`). The layout is `protocol::decode_envelope`'s
/// alone; this is the one decoder of WAL commands (document open, the ledger tail, every sync hello).
pub fn decode_wal_command(bytes: &db_wal_log::WalBytes, control: &mut db_wal_log::WalCursorControl) -> Result<protocol::MutationEnvelope, DbError> {
    control.grant()?;
    with_wal_command(bytes, |record| {
        let mut pos = 0usize;
        let envelope = protocol::decode_envelope(record, &mut pos).map_err(|error| DbError::Corrupt(format!("wal command envelope: {error}")))?;
        if pos != record.len() {
            return Err(DbError::Corrupt("wal command envelope has trailing bytes".to_string()));
        }
        Ok(envelope)
    })
}

/// 🆔️ The mutation id of a WAL command record (the layout's first field) without decoding the rest.
pub fn wal_command_id(bytes: &db_wal_log::WalBytes, control: &mut db_wal_log::WalCursorControl) -> Result<String, DbError> {
    control.grant()?;
    with_wal_command(bytes, |record| protocol::read_str(record, &mut 0).map_err(|error| DbError::Corrupt(format!("wal command id: {error}"))))
}

//#region 🔖️ReplicaState
/// 🧾️ One document's currently-retained sync state, replayed fresh from its WAL — the
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
    /// 🚧️ The lowest `head_seq` this crate can still serve via tail (missing-command)
    /// transfer — the `head_seq` of the most recent `WAL_SNAPSHOT_PUB` record replayed, or `0` if
    /// none (nothing has ever been compacted away). A replica behind this floor needs
    /// `decide_bootstrap`'s snapshot path instead.
    pub floor_head_seq: u64,
}

/// 🔁️ Replays committed transactions via `db_wal::replay_committed_document` and
/// derives its `ArtifactSyncState` — see the struct's doc for exactly how each field is derived.
pub async fn replay_sync_state(storage: &impl db_io_core::WalStorage, document: ArtifactId) -> Result<ArtifactSyncState, DbError> {
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = db_wal_log::WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
    let mut records = db_wal_log::replay_committed_document(storage, &document, control).await?;
    let mut decode_control = db_wal_log::WalCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
    let mut commands = Vec::new();
    let mut chain = semio_framework_hash::Hasher::new();
    let mut commit_seq = 0u64;
    let mut floor_head_seq = 0u64;
    let mut turn = db_wal_log::WalTurn::start();
    let replay = async {
        loop {
            records.renew_step()?;
            decode_control.replenish(std::time::Instant::now() + db_wal_log::WAL_REPLAY_STEP_STALL_BOUND, db_wal_log::WAL_REPLAY_STEP_FUEL)?;
            let mut transaction = match records.next_transaction_step().await? {
                db_wal_log::WalCommittedStep::Transaction(transaction) => transaction,
                db_wal_log::WalCommittedStep::Yield => {
                    turn.step().await;
                    continue;
                }
                db_wal_log::WalCommittedStep::Done => break,
            };
            loop {
                match transaction.next_record_step()? {
                    db_wal_log::WalCommittedRecordStep::Record(db_wal_log::WalRecord::Command(bytes)) => {
                        commands.push(decode_wal_command(bytes, &mut decode_control)?);
                        chain.update(&bytes.hash());
                    }
                    db_wal_log::WalCommittedRecordStep::Record(db_wal_log::WalRecord::SnapshotPub { frontier, .. }) => floor_head_seq = frontier.head_seq,
                    db_wal_log::WalCommittedRecordStep::Record(_) => {}
                    db_wal_log::WalCommittedRecordStep::Yield => {
                        turn.step().await;
                        continue;
                    }
                    db_wal_log::WalCommittedRecordStep::Done => break,
                }
                while transaction.close_record_step()? {
                    turn.step().await;
                }
                turn.step().await;
            }
            transaction.finish()?;
            commit_seq = commit_seq.checked_add(1).ok_or(DbError::LimitExceeded("sync replay commit sequence"))?;
        }
        Ok::<(), DbError>(())
    }
    .await;
    while records.close_owner_step()? {
        semio_framework_async::yield_once().await;
    }
    replay?;
    drop(records);
    let head_seq = commands.len() as u64;
    let chain_hash = if commands.is_empty() { [0; 32] } else { *chain.finalize().as_bytes() };
    let frontier = Frontier { document, head_seq, commit_seq, chain_hash, epoch: 0 };
    Ok(ArtifactSyncState { frontier, commands, floor_head_seq })
}
//#endregion 🔖️ReplicaState

//#region 🔖️Frontier
/// ➖️ `FrontierDelta::between`, re-exposed under this crate's own name for
/// discoverability — frontier-delta computation is this crate's stated responsibility, so
/// `db_sync::frontier_delta` is the expected first stop even though the primitive itself lives in
/// `db_core`.
pub async fn frontier_delta(from: &Frontier, to: &Frontier) -> Result<FrontierDelta, DbError> {
    FrontierDelta::between(from, to).await
}

/// 🌉️ `Frontier` -> `protocol::RuntimeFrontierSummary` (the wire-frame shape
/// `ServerFrame::{Welcome, Commands, Ack}.*frontier` fields carry). `head_edit_id` has no
/// `Frontier` counterpart (see `ArtifactSyncState`'s doc); callers pass whatever they
/// consider the frontier's tip identity (`state_frontier_summary` below supplies the natural
/// choice: the last replayed command's `mutation_id`).
pub async fn to_frontier_summary(frontier: &Frontier, head_edit_id: String) -> protocol::RuntimeFrontierSummary {
    protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(frontier.document.0.clone()), head_edit_ordinal: frontier.head_seq, head_edit_id, last_commit_seq: frontier.commit_seq, chain_hash: frontier.chain_hash }
}

/// 🌉️ Inverse bridge direction: `protocol::RuntimeFrontierSummary` -> `Frontier`,
/// the primitive `handle_hello`/`handle_frontier_advertise` use to turn a replica's advertised
/// wire frontier into something `missing_commands`/`decide_bootstrap` can compare against a
/// `ArtifactSyncState`. `epoch` is always `0` (see `ArtifactSyncState`'s doc: `RuntimeFrontierSummary`
/// carries no cluster-fencing epoch at all).
// 🚫️async: E1 pure accessor consumed by a sync Option::map — see R9
pub fn from_frontier_summary(summary: &protocol::RuntimeFrontierSummary) -> Frontier {
    Frontier { document: ArtifactId(summary.document_id.0.clone()), head_seq: summary.head_edit_ordinal, commit_seq: summary.last_commit_seq, chain_hash: summary.chain_hash, epoch: 0 }
}

/// 🌉️ `state`'s own frontier as a `RuntimeFrontierSummary`, with `head_edit_id` filled from
/// the last replayed command's `mutation_id` (empty string for a genesis document with no
/// commands yet).
pub async fn state_frontier_summary(state: &ArtifactSyncState) -> protocol::RuntimeFrontierSummary {
    let head_edit_id = state.commands.last().map(|envelope| envelope.mutation_id.0.clone()).unwrap_or_default();
    to_frontier_summary(&state.frontier, head_edit_id).await
}
//#endregion 🔖️Frontier

//#region 🔖️MissingCommands
/// 📦️ The missing-command-transfer primitive: every command `state` holds strictly after
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

/// 📨️ Wraps `envelopes` (typically `missing_commands`' result) as a `ServerFrame::Commands`
/// stamped with `state`'s current frontier — `origin` is the relaying actor identity the caller
/// (the semio_hub session layer, which owns its own actor identity) supplies; this crate has no opinion
/// on it beyond passing it through.
pub async fn commands_server_frame(state: &ArtifactSyncState, envelopes: Vec<protocol::MutationEnvelope>, origin: protocol::ActorId) -> protocol::ServerFrame {
    protocol::ServerFrame::Commands { envelopes, origin, frontier: state_frontier_summary(state).await }
}
//#endregion 🔖️MissingCommands

//#region 🔖️Bootstrap
/// 🚀️ How a (re)connecting replica should be caught up, decided by `decide_bootstrap` —
/// the pre-wire-encoding twin of `protocol::Bootstrap` (kept separate so this crate's core
/// decision logic stays testable without constructing full `ServerFrame`s; `build_welcome` below
/// lowers it to the wire shape).
#[derive(Debug, PartialEq)]
pub enum BootstrapPlan {
    /// ✅️ The replica is already fully caught up — nothing to send.
    None,
    /// 🚚️ The replica is within the retained WAL floor: ship it the missing commands
    /// directly, no snapshot needed.
    Tail { envelopes: Vec<protocol::MutationEnvelope> },
    /// 📸️ The replica is behind the retained WAL floor (or brand new against a compacted
    /// document): ship it a whole snapshot generation first.
    Snapshot { generation: u64, pages: db_io_core::DbIoPages, pack_hash: [u8; 32] },
}

/// 🧭️ Decides `BootstrapPlan` for `replica` (`None` meaning a totally fresh replica with no
/// prior frontier at all) against `state`, consulting `snapshots` only when the replica's
/// `head_seq` has fallen behind `state.floor_head_seq`.
pub async fn decide_bootstrap(state: &ArtifactSyncState, snapshots: &impl SnapshotStorage, replica: Option<&Frontier>) -> Result<BootstrapPlan, DbError> {
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
    let pack_hash = db_io_registry::db_io_hash_pages(&pages).await.0;
    Ok(BootstrapPlan::Snapshot { generation, pages, pack_hash })
}
//#endregion 🔖️Bootstrap

//#region 🔖️ResumeToken
/// 🎫️ Issues a fresh resume token for `frontier` — the send-path half of resume tokens (see
/// module doc for why the receive path uses `SocketHelloV1.frontier` instead). `Welcome.resume_token` is
/// always populated from this.
pub async fn issue_resume_token(frontier: &Frontier) -> Result<String, DbError> {
    Ok(ResumeToken::encode(frontier)?.as_str().to_string())
}
//#endregion 🔖️ResumeToken
