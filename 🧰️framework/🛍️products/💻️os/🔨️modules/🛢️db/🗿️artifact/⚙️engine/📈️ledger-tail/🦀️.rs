//! 📈️ Ledger tail: the ledger points and commits and `artifact_ledger_tail`. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️LedgerTail
/// 🧮️ Ceiling on the committed transactions one ledger tail may carry, equal to the hub
/// authority's own `AUTHORITY_MAX_OPERATIONS`.
pub const ARTIFACT_LEDGER_TAIL_MAX_COMMITS: usize = 16_384;

/// 📍️ One exact point of a document's committed ledger: the counters and content chain the
/// document authority publishes plus the edit id at its tip (`None` only for genesis).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactLedgerPoint {
    pub head_seq: u64,
    pub commit_seq: u64,
    pub chain_hash: [u8; 32],
    pub head_edit_id: Option<protocol::MutationId>,
}

impl ArtifactLedgerPoint {
    /// 🌱️ The point of a document nothing was ever committed to.
    pub const fn genesis() -> Self {
        Self { head_seq: 0, commit_seq: 0, chain_hash: [0; 32], head_edit_id: None }
    }

    /// 📸️ The point an actor-serialized snapshot names.
    pub fn of_snapshot(snapshot: &CheckpointPublicationSnapshot) -> Self {
        Self { head_seq: snapshot.frontier.head_seq, commit_seq: snapshot.frontier.commit_seq, chain_hash: snapshot.frontier.chain_hash, head_edit_id: snapshot.head_edit_id.clone() }
    }

    fn is_genesis(&self) -> bool {
        *self == Self::genesis()
    }
}

/// 📜️ One committed command transaction of the ledger tail: its envelopes in WAL order and the
/// point the document authority published for it.
#[derive(Clone, Debug, PartialEq)]
pub struct ArtifactLedgerCommit {
    pub point: ArtifactLedgerPoint,
    pub envelopes: Vec<protocol::MutationEnvelope>,
}

/// 📜️ The committed command transactions strictly after `after` through exactly `through`, read
/// from the same WAL and with the same point derivation `ArtifactEngine::open` replays: a command
/// transaction's point is its terminal `Frontier` record plus its last command's id, and a durable
/// group decision advances the point by one head and one commit to its parent post revision. Both
/// points must be ledger points exactly, `after` must precede `through`, and a durable group
/// decision inside the range is refused: its content is an approval's, published with its own
/// checkpoint, and never an envelope a replica folds. `cancelled` stops the walk at the next record.
///
/// See `🌎️hub/🗿️artifact-authority/📌️check-in` for the hub Check In that consumes it.
pub async fn artifact_ledger_tail(wal: &impl WalStorage, document: &ArtifactId, after: &ArtifactLedgerPoint, through: &ArtifactLedgerPoint, cancelled: Arc<std::sync::atomic::AtomicBool>) -> Result<Vec<ArtifactLedgerCommit>, DbError> {
    if through.head_seq <= after.head_seq || through.commit_seq <= after.commit_seq || through.head_edit_id.is_none() {
        return Err(DbError::InvalidArgument("ledger tail must end at an edited point after its start".to_string()));
    }
    let control = db_wal_log::WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + db_wal_log::WAL_REPLAY_STEP_STALL_BOUND, db_wal_log::WAL_REPLAY_STEP_FUEL)?;
    let mut records = db_wal_log::replay_committed_document(wal, document, control).await?;
    let mut decode_control = db_wal_log::WalCursorControl::stall_bounded(cancelled.clone(), db_wal_log::WAL_REPLAY_STEP_STALL_BOUND, db_wal_log::WAL_REPLAY_STEP_FUEL)?;
    let mut turn = db_wal_log::WalTurn::start();
    let result = async {
        let mut point = ArtifactLedgerPoint::genesis();
        let mut decisions: HashSet<String> = HashSet::new();
        let mut started = after.is_genesis();
        let mut commits = Vec::new();
        loop {
            if cancelled.load(std::sync::atomic::Ordering::Acquire) {
                return Err(DbError::Unavailable("ledger tail cancelled".to_string()));
            }
            records.renew_step()?;
            decode_control.renew_step()?;
            let mut transaction = match records.next_transaction_step().await? {
                db_wal_log::WalCommittedStep::Transaction(transaction) => transaction,
                db_wal_log::WalCommittedStep::Yield => {
                    turn.step().await;
                    continue;
                }
                db_wal_log::WalCommittedStep::Done => return Err(DbError::NotFound("ledger tail end is not a committed ledger point".to_string())),
            };
            let mut envelopes = Vec::new();
            let mut frontier = None;
            let mut decision = None;
            loop {
                let record = match transaction.next_record_step()? {
                    db_wal_log::WalCommittedRecordStep::Record(record) => record,
                    db_wal_log::WalCommittedRecordStep::Yield => {
                        turn.step().await;
                        continue;
                    }
                    db_wal_log::WalCommittedRecordStep::Done => break,
                };
                match record {
                    db_wal_log::WalRecord::Command(bytes) => {
                        let envelope = db_sync::decode_wal_command(bytes, &mut decode_control)?;
                        if envelope.document_id.0 != document.0 {
                            return Err(DbError::Corrupt("ledger envelope document differs".to_string()));
                        }
                        envelopes.push(envelope);
                    }
                    db_wal_log::WalRecord::Frontier(committed) => {
                        if committed.document != *document {
                            return Err(DbError::Corrupt("ledger frontier document differs".to_string()));
                        }
                        frontier = Some(committed.clone());
                    }
                    db_wal_log::WalRecord::Event(bytes) => {
                        let mut canonical_pack = Vec::with_capacity(bytes.len());
                        for fragment in bytes.fragments() {
                            canonical_pack.extend_from_slice(fragment);
                        }
                        decision = Some(store::durable_group::DurableOwnedGroupJournalRecordV1::admit_canonical(canonical_pack).map_err(|error| DbError::Corrupt(format!("committed durable group decision is invalid: {error}")))?);
                    }
                    _ => {}
                }
                while transaction.close_record_step()? {
                    turn.step().await;
                }
                turn.step().await;
            }
            transaction.finish()?;
            let next = match (decision, frontier, envelopes.last()) {
                (Some(record), None, None) => {
                    if !decisions.insert(record.decision_sha256().to_string()) {
                        continue;
                    }
                    if started {
                        return Err(DbError::Conflict("ledger tail contains a durable group decision".to_string()));
                    }
                    ArtifactLedgerPoint {
                        head_seq: point.head_seq.checked_add(1).ok_or(DbError::LimitExceeded("ledger tail head sequence"))?,
                        commit_seq: point.commit_seq.checked_add(1).ok_or(DbError::LimitExceeded("ledger tail commit sequence"))?,
                        chain_hash: record.parent_post_revision(),
                        head_edit_id: Some(protocol::MutationId(record.parent_edit_id().to_string())),
                    }
                }
                (None, Some(frontier), Some(last)) => ArtifactLedgerPoint { head_seq: frontier.head_seq, commit_seq: frontier.commit_seq, chain_hash: frontier.chain_hash, head_edit_id: Some(last.mutation_id.clone()) },
                (None, None, None) => continue,
                _ => return Err(DbError::Corrupt("ledger transaction is neither one command batch with its frontier nor one durable group decision".to_string())),
            };
            if next.head_seq <= point.head_seq || next.commit_seq <= point.commit_seq {
                return Err(DbError::Corrupt("ledger points do not advance".to_string()));
            }
            point = next;
            if !started {
                if point == *after {
                    started = true;
                } else if point.commit_seq >= after.commit_seq {
                    return Err(DbError::NotFound("ledger tail start is not a committed ledger point".to_string()));
                }
                continue;
            }
            if commits.len() == ARTIFACT_LEDGER_TAIL_MAX_COMMITS {
                return Err(DbError::LimitExceeded("ledger tail commits"));
            }
            let reached = point == *through;
            if !reached && point.commit_seq >= through.commit_seq {
                return Err(DbError::NotFound("ledger tail end is not a committed ledger point".to_string()));
            }
            commits.push(ArtifactLedgerCommit { point: point.clone(), envelopes });
            if reached {
                return Ok(commits);
            }
        }
    }
    .await;
    while records.close_owner_step()? {
        semio_framework_async::yield_once().await;
    }
    drop(records);
    result
}
//#endregion 🔖️LedgerTail
//#endregion 🔖️Ids
