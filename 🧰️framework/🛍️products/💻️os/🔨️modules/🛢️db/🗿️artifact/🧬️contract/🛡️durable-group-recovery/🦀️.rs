//! 🛡️ Durable group recovery: the committed recovery operations, the decision from a transaction and the recovery target, driver and owner. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️Receipt
impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: pack::snapshot::ArtifactPack + store::os_store::serialization::ArtifactStoreSnapshot + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static + store::ColdMutation<ParentP> + store::OpBinary + store::OpText,
    DrawingP: pack::snapshot::ArtifactPack + store::os_store::serialization::ArtifactStoreSnapshot + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static + store::ColdMutation<DrawingP> + store::OpBinary + store::OpText,
    ValueP: pack::snapshot::ArtifactPack + store::os_store::serialization::ArtifactStoreSnapshot + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static + store::ColdMutation<ValueP> + store::OpBinary + store::OpText,
{
    pub fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> ArtifactCommittedDurableGroupRecoveryAdvanceV1 {
        let Some(state) = self.state.take() else {
            return ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(store::durable_group::DurableOwnedGroupDecisionError::InvalidOutcome);
        };
        match state {
            ArtifactCommittedDurableGroupRecoveryStateV1::Admitting { witness, admission, .. } => {
                let document = witness.document.clone();
                let receipt = witness.receipt();
                match admission.restore_untrusted_committed_record(&witness.record, receipt.clone()) {
                    Ok(store::durable_group::DurableOwnedMapRecoveryStartV1::Apply(host)) => {
                        let Some(phase) = host.phase() else {
                            *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                            return ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(store::durable_group::DurableOwnedGroupDecisionError::InvalidOutcome);
                        };
                        *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                        ArtifactCommittedDurableGroupRecoveryAdvanceV1::Progress(phase)
                    }
                    Ok(store::durable_group::DurableOwnedMapRecoveryStartV1::AlreadyApplied(owners)) => {
                        *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Complete { document, receipt, already_applied: true, owners });
                        ArtifactCommittedDurableGroupRecoveryAdvanceV1::Complete
                    }
                    Err(rejected) => {
                        let error = rejected.error.clone();
                        let admission = store::durable_group::DurableOwnedMapRecoveryAdmissionV1::new(rejected.parent, rejected.drawing, rejected.value);
                        *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Admitting { witness, admission, last_error: Some(error.clone()) });
                        ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(error)
                    }
                }
            }
            ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, mut host } => match host.advance(grant) {
                store::durable_group::DurableOwnedMapRecoveryAdvanceV1::Progress(phase) => {
                    *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                    ArtifactCommittedDurableGroupRecoveryAdvanceV1::Progress(phase)
                                }
                store::durable_group::DurableOwnedMapRecoveryAdvanceV1::DiagnosticProgress(phase, progress) => {
                    *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                    ArtifactCommittedDurableGroupRecoveryAdvanceV1::DiagnosticProgress(phase, progress)
}
                store::durable_group::DurableOwnedMapRecoveryAdvanceV1::Blocked => {
                    *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                    ArtifactCommittedDurableGroupRecoveryAdvanceV1::Blocked
                }
                store::durable_group::DurableOwnedMapRecoveryAdvanceV1::Fault(error) => {
                    *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                    ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(error)
                }
                store::durable_group::DurableOwnedMapRecoveryAdvanceV1::AwaitingAcknowledgement => {
                    if !host.acknowledge_restoration(&receipt) {
                        *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                        return ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(store::durable_group::DurableOwnedGroupDecisionError::InvalidHash);
                    }
                    let Some(phase) = host.phase() else {
                        *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                        return ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(store::durable_group::DurableOwnedGroupDecisionError::InvalidOutcome);
                    };
                    *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                    ArtifactCommittedDurableGroupRecoveryAdvanceV1::Progress(phase)
                }
                store::durable_group::DurableOwnedMapRecoveryAdvanceV1::Complete => {
                    let Some(owners) = host.take_terminal_owners() else {
                        *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Recovering { document, receipt, host });
                        return ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(store::durable_group::DurableOwnedGroupDecisionError::InvalidOutcome);
                    };
                    *self.state = Some(ArtifactCommittedDurableGroupRecoveryStateV1::Complete { document, receipt, already_applied: false, owners });
                    ArtifactCommittedDurableGroupRecoveryAdvanceV1::Complete
                }
            },
            complete @ ArtifactCommittedDurableGroupRecoveryStateV1::Complete { .. } => {
                *self.state = Some(complete);
                ArtifactCommittedDurableGroupRecoveryAdvanceV1::Complete
            }
        }
    }

    pub(crate) fn take_terminal(&mut self) -> Option<ArtifactCommittedDurableGroupRecoveryTerminalV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        let state = self.state.take()?;
        match state {
            ArtifactCommittedDurableGroupRecoveryStateV1::Complete { document, receipt, already_applied, owners } => Some(ArtifactCommittedDurableGroupRecoveryTerminalV1 { document, receipt, already_applied, owners }),
            state => {
                *self.state = Some(state);
                None
            }
        }
    }

    pub fn take_rejected_terminal(&mut self) -> Option<ArtifactCommittedDurableGroupRecoveryRejectedTerminalV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        let state = self.state.take()?;
        match state {
            ArtifactCommittedDurableGroupRecoveryStateV1::Admitting { witness, admission, last_error: Some(error) } => {
                let document = witness.document.clone();
                let receipt = witness.receipt();
                drop(witness);
                Some(ArtifactCommittedDurableGroupRecoveryRejectedTerminalV1 { document, receipt, error, owners: admission.into_owners() })
            }
            state => {
                *self.state = Some(state);
                None
            }
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.is_none()
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ColdMutation<ParentP>,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ColdMutation<DrawingP>,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ColdMutation<ValueP>,
{
    fn drop(&mut self) {
        assert!(self.state.is_none(), "committed WAL recovery reached Drop before exact three-Store terminal handoff");
    }
}

pub async fn committed_durable_group_decision_from_transaction<S: WalStorage>(document: &ArtifactId, mut transaction: db_wal_log::WalCommittedTransaction<'_, '_, S>) -> Result<Option<ArtifactCommittedDurableGroupDecisionV1>, DbError> {
    let transaction_id = transaction.transaction_id();
    let segment_index = transaction.segment_index();
    let record_count = transaction.record_count();
    let mut event = None;
    let mut mixed = false;
    loop {
        match transaction.next_record_step()? {
            db_wal_log::WalCommittedRecordStep::Record(db_wal_log::WalRecord::Event(bytes)) => {
                if event.is_some() {
                    mixed = true;
                } else {
                    let mut canonical_pack = Vec::with_capacity(bytes.len());
                    for fragment in bytes.fragments() {
                        canonical_pack.extend_from_slice(fragment);
                    }
                    event = Some(canonical_pack);
                }
            }
            db_wal_log::WalCommittedRecordStep::Record(_) => mixed = event.is_some() || record_count > 1,
            db_wal_log::WalCommittedRecordStep::Yield => {
                transaction.replenish(std::time::Instant::now() + std::time::Duration::from_secs(2), 65_536)?;
                semio_framework_async::yield_once().await;
                continue;
            }
            db_wal_log::WalCommittedRecordStep::Done => break,
        }
        while transaction.close_record_step()? {
            transaction.replenish(std::time::Instant::now() + std::time::Duration::from_secs(2), 65_536)?;
            semio_framework_async::yield_once().await;
        }
    }
    transaction.finish()?;
    let Some(canonical_pack) = event else { return Ok(None) };
    if mixed || record_count != 1 {
        return Err(DbError::Corrupt("durable group decision must be the sole committed transaction body".to_string()));
    }
    let record = store::durable_group::DurableOwnedGroupJournalRecordV1::admit_canonical(canonical_pack).map_err(|error| DbError::Corrupt(format!("committed durable group decision is invalid: {error}")))?;
    if record.document().artifact_id != document.0 {
        return Err(DbError::Corrupt("committed durable group decision targets a different replay document".to_string()));
    }
    Ok(Some(ArtifactCommittedDurableGroupDecisionV1 { document: document.clone(), transaction_id, segment_index, record }))
}

enum ArtifactDurableGroupRecoveryTargetStateV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ColdMutation<ParentP>,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ColdMutation<DrawingP>,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ColdMutation<ValueP>,
{
    Admitting(store::durable_group::DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>),
    Recovering { owner: ArtifactCommittedDurableGroupRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, checkpoint: ArtifactDurableGroupRecoveredCheckpointV1 },
    Complete(store::durable_group::DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>),
    Empty,
}

struct ArtifactDurableGroupRecoveryTargetSharedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ColdMutation<ParentP>,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ColdMutation<DrawingP>,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ColdMutation<ValueP>,
{
    pub document: ArtifactId,
    pub state: std::sync::Mutex<ArtifactDurableGroupRecoveryTargetStateV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>,
    pub scan_complete: std::sync::atomic::AtomicBool,
    pub(super) recovered_checkpoint: std::sync::Mutex<Option<ArtifactDurableGroupRecoveredCheckpointV1>>,
}

pub trait ArtifactDurableGroupRecoveryDriverCoreV1: Send {
    fn document(&self) -> &ArtifactId;
    fn drive_recovery_step(&mut self) -> Result<bool, DbError>;
    fn accept(&mut self, witness: ArtifactCommittedDurableGroupDecisionV1) -> Result<(), DbError>;
    fn finish_scan(&mut self) -> Result<(), DbError>;
}

/// 🧭 Opaque actor-mailbox owner for one exact-three Store recovery scan.
pub struct ArtifactDurableGroupRecoveryDriverV1 {
    pub owner: Box<dyn ArtifactDurableGroupRecoveryDriverCoreV1>,
}

struct ArtifactDurableGroupRecoveryTargetDriverV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ColdMutation<ParentP>,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ColdMutation<DrawingP>,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ColdMutation<ValueP>,
{
    pub shared: Arc<ArtifactDurableGroupRecoveryTargetSharedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>,
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> ArtifactDurableGroupRecoveryDriverCoreV1 for ArtifactDurableGroupRecoveryTargetDriverV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: pack::snapshot::ArtifactPack + store::os_store::serialization::ArtifactStoreSnapshot + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static + store::ColdMutation<ParentP> + store::OpBinary + store::OpText,
    DrawingP: pack::snapshot::ArtifactPack + store::os_store::serialization::ArtifactStoreSnapshot + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static + store::ColdMutation<DrawingP> + store::OpBinary + store::OpText,
    ValueP: pack::snapshot::ArtifactPack + store::os_store::serialization::ArtifactStoreSnapshot + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static + store::ColdMutation<ValueP> + store::OpBinary + store::OpText,
{
    fn document(&self) -> &ArtifactId {
        &self.shared.document
    }

    fn drive_recovery_step(&mut self) -> Result<bool, DbError> {
        let mut state = self.shared.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let current = std::mem::replace(&mut *state, ArtifactDurableGroupRecoveryTargetStateV1::Empty);
        let ArtifactDurableGroupRecoveryTargetStateV1::Recovering { owner: mut recovery, checkpoint } = current else {
            *state = current;
            return Ok(true);
        };
        match recovery.advance(store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, maximum_capacity_bytes: store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, maximum_release_bytes: store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, maximum_depth: 64 }) {
            ArtifactCommittedDurableGroupRecoveryAdvanceV1::Progress(_) | ArtifactCommittedDurableGroupRecoveryAdvanceV1::DiagnosticProgress(_, _) | ArtifactCommittedDurableGroupRecoveryAdvanceV1::Blocked => {
                *state = ArtifactDurableGroupRecoveryTargetStateV1::Recovering { owner: recovery, checkpoint };
                Ok(false)
            }
            ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(error) => {
                *state = ArtifactDurableGroupRecoveryTargetStateV1::Recovering { owner: recovery, checkpoint };
                Err(DbError::Unavailable(format!("committed durable group recovery retained a fault: {error:?}")))
            }
            ArtifactCommittedDurableGroupRecoveryAdvanceV1::Complete => {
                let Some(terminal) = recovery.take_terminal() else {
                    *state = ArtifactDurableGroupRecoveryTargetStateV1::Recovering { owner: recovery, checkpoint };
                    return Err(DbError::Internal("committed durable group recovery completed without exact Store owners".to_string()));
                };
                drop(recovery);
                if terminal.document != self.shared.document {
                    *state = ArtifactDurableGroupRecoveryTargetStateV1::Admitting(store::durable_group::DurableOwnedMapRecoveryAdmissionV1::new(terminal.owners.parent, terminal.owners.drawing, terminal.owners.value));
                    return Err(DbError::Corrupt("committed durable group recovery returned a different document".to_string()));
                }
                if terminal.receipt != *checkpoint.receipt() {
                    *state = ArtifactDurableGroupRecoveryTargetStateV1::Admitting(store::durable_group::DurableOwnedMapRecoveryAdmissionV1::new(terminal.owners.parent, terminal.owners.drawing, terminal.owners.value));
                    return Err(DbError::Corrupt("committed durable group recovery returned a different receipt".to_string()));
                }
                let mut checkpoint = checkpoint;
                checkpoint.already_applied = terminal.already_applied;
                *self.shared.recovered_checkpoint.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(checkpoint);
                *state = ArtifactDurableGroupRecoveryTargetStateV1::Admitting(store::durable_group::DurableOwnedMapRecoveryAdmissionV1::new(terminal.owners.parent, terminal.owners.drawing, terminal.owners.value));
                Ok(true)
            }
        }
    }

    fn accept(&mut self, witness: ArtifactCommittedDurableGroupDecisionV1) -> Result<(), DbError> {
        if witness.document() != &self.shared.document {
            return Err(DbError::Corrupt("committed durable group recovery witness targets a different document".to_string()));
        }
        let mut state = self.shared.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let current = std::mem::replace(&mut *state, ArtifactDurableGroupRecoveryTargetStateV1::Empty);
        let ArtifactDurableGroupRecoveryTargetStateV1::Admitting(admission) = current else {
            *state = current;
            return Err(DbError::Conflict("committed durable group recovery target is not ready for another witness".to_string()));
        };
        let checkpoint = witness.recovered_checkpoint();
        *state = ArtifactDurableGroupRecoveryTargetStateV1::Recovering { owner: witness.into_store_owned_recovery(admission), checkpoint };
        Ok(())
    }

    fn finish_scan(&mut self) -> Result<(), DbError> {
        let mut state = self.shared.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let current = std::mem::replace(&mut *state, ArtifactDurableGroupRecoveryTargetStateV1::Empty);
        match current {
            ArtifactDurableGroupRecoveryTargetStateV1::Admitting(admission) => {
                *state = ArtifactDurableGroupRecoveryTargetStateV1::Complete(admission.into_owners());
                self.shared.scan_complete.store(true, std::sync::atomic::Ordering::Release);
                Ok(())
            }
            current @ ArtifactDurableGroupRecoveryTargetStateV1::Complete(_) => {
                *state = current;
                Ok(())
            }
            current => {
                *state = current;
                Err(DbError::Conflict("committed durable group recovery scan ended with an active witness".to_string()))
            }
        }
    }
}

/// 🧰 Retains one actor scan and the exact three Stores across caller cancellation and retry.
#[must_use = "durable group recovery must return its exact three Store owners"]
pub struct ArtifactDurableGroupRecoveryOwnerV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ColdMutation<ParentP>,
    DrawingP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ColdMutation<DrawingP>,
    ValueP: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue,
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ColdMutation<ValueP>,
{
    pub shared: Arc<ArtifactDurableGroupRecoveryTargetSharedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>,
    pub request: Option<Pin<Box<db_actor::AskFuture<ArtifactMessage, Result<(), DbError>>>>>,
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> ArtifactDurableGroupRecoveryOwnerV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: pack::snapshot::ArtifactPack + store::os_store::serialization::ArtifactStoreSnapshot + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    ParentMutation: store::Mutation<ParentP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static + store::ColdMutation<ParentP> + store::OpBinary + store::OpText,
    DrawingP: pack::snapshot::ArtifactPack + store::os_store::serialization::ArtifactStoreSnapshot + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    DrawingMutation: store::Mutation<DrawingP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static + store::ColdMutation<DrawingP> + store::OpBinary + store::OpText,
    ValueP: pack::snapshot::ArtifactPack + store::os_store::serialization::ArtifactStoreSnapshot + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
    ValueMutation: store::Mutation<ValueP> + store::os_store::serialization::ArtifactCanonicalJsonTree + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static + store::ColdMutation<ValueP> + store::OpBinary + store::OpText,
{
    pub fn new(document: ArtifactId, admission: store::durable_group::DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>) -> (Self, ArtifactDurableGroupRecoveryDriverV1) {
        let shared = Arc::new(ArtifactDurableGroupRecoveryTargetSharedV1 {
            document,
            state: std::sync::Mutex::new(ArtifactDurableGroupRecoveryTargetStateV1::Admitting(admission)),
            scan_complete: std::sync::atomic::AtomicBool::new(false),
            recovered_checkpoint: std::sync::Mutex::new(None),
        });
        let driver = ArtifactDurableGroupRecoveryDriverV1 { owner: Box::new(ArtifactDurableGroupRecoveryTargetDriverV1 { shared: shared.clone() }) };
        (Self { shared, request: None }, driver)
    }

    pub fn driver(&self) -> ArtifactDurableGroupRecoveryDriverV1 {
        ArtifactDurableGroupRecoveryDriverV1 { owner: Box::new(ArtifactDurableGroupRecoveryTargetDriverV1 { shared: self.shared.clone() }) }
    }

    pub async fn advance(&mut self) -> Result<bool, DbError> {
        let Some(request) = self.request.as_mut() else {
            return Ok(self.shared.scan_complete.load(std::sync::atomic::Ordering::Acquire));
        };
        let result = request.as_mut().await;
        self.request = None;
        result??;
        Ok(self.shared.scan_complete.load(std::sync::atomic::Ordering::Acquire))
    }

    pub fn take_terminal_owners(&mut self) -> Option<store::durable_group::DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        if self.request.is_some() || !self.shared.scan_complete.load(std::sync::atomic::Ordering::Acquire) {
            return None;
        }
        let mut state = self.shared.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let current = std::mem::replace(&mut *state, ArtifactDurableGroupRecoveryTargetStateV1::Empty);
        match current {
            ArtifactDurableGroupRecoveryTargetStateV1::Complete(owners) => Some(owners),
            current => {
                *state = current;
                None
            }
        }
    }

    /// 🧹 Returns exact Stores only when a failed scan has no accepted committed witness in
    /// flight. Once a witness is accepted, recovery is non-cancellable and must be resumed.
    pub fn take_idle_owners(&mut self) -> Option<store::durable_group::DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        if self.request.is_some() {
            return None;
        }
        let mut state = self.shared.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let current = std::mem::replace(&mut *state, ArtifactDurableGroupRecoveryTargetStateV1::Empty);
        match current {
            ArtifactDurableGroupRecoveryTargetStateV1::Admitting(admission) => Some(admission.into_owners()),
            current => {
                *state = current;
                None
            }
        }
    }

    pub fn scan_is_active(&self) -> bool {
        self.request.is_some()
    }

    /// 📍️ Returns the last fully restored committed Event projection without exposing its WAL witness.
    pub fn recovered_checkpoint(&self) -> Option<ArtifactDurableGroupRecoveredCheckpointV1> {
        self.shared.recovered_checkpoint.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    pub fn terminal_is_empty(&self) -> bool {
        matches!(&*self.shared.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner), ArtifactDurableGroupRecoveryTargetStateV1::Empty)
    }
}
//#endregion 🔖️Receipt
//#endregion 🔖️Ids
