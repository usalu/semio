//! 🛠️ Map assembly recovery completion: the map assembly operations that complete the recovery, retire the owners and report the phases. See [`super`].

use super::*;

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapCommitHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ParentMutation: ArtifactCanonicalJsonTree + StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + semio_framework_os_spr::ColdMutation<ParentP> + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: ArtifactCanonicalJsonTree + StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + semio_framework_os_spr::ColdMutation<DrawingP> + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: ArtifactCanonicalJsonTree + StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + semio_framework_os_spr::ColdMutation<ValueP> + 'static,
{
    pub fn phase(&self) -> Option<DurableOwnedThreeStoreCommitPhaseV1> {
        self.operation.as_ref().and_then(DurableOwnedMapCommitOperationV1::phase)
    }

    pub fn capture_snapshot(&self) -> Result<DurableOwnedThreeStoreSnapshotV1<ParentP, DrawingP, ValueP>, DurableOwnedGroupDecisionError> {
        self.operation.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?.capture_snapshot()
    }

    pub fn cancel(&mut self) -> bool {
        self.operation.as_mut().is_some_and(DurableOwnedMapCommitOperationV1::cancel)
    }

    pub fn acknowledge(&mut self) -> bool {
        self.operation.as_mut().is_some_and(DurableOwnedMapCommitOperationV1::acknowledge_retained)
    }

    pub fn advance(&mut self, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedGroupDecisionError> {
        self.operation.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?.advance(grant)
    }

    pub fn take_terminal_owners(&mut self) -> Option<DurableOwnedMapCommitOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        let owners = self.operation.as_mut()?.take_terminal_owners()?;
        drop(self.operation.take().expect("terminal Map host retains its empty operation slot"));
        Some(owners)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.operation.is_none()
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapRecoveryOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ParentMutation: ArtifactCanonicalJsonTree + StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + semio_framework_os_spr::ColdMutation<ParentP> + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: ArtifactCanonicalJsonTree + StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + semio_framework_os_spr::ColdMutation<DrawingP> + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: ArtifactCanonicalJsonTree + StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + semio_framework_os_spr::ColdMutation<ValueP> + 'static,
{
    pub fn phase(&self) -> Option<DurableOwnedThreeStoreCommitPhaseV1> {
        self.coordinator.as_ref().map(DurableOwnedThreeStoreCommitV1::phase)
    }

    pub fn capture_snapshot(&self) -> Result<DurableOwnedThreeStoreSnapshotV1<ParentP, DrawingP, ValueP>, DurableOwnedGroupDecisionError> {
        capture_store_owned_three_snapshot(
            self.parent.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            self.drawing.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            self.value.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
        )
    }

    pub fn acknowledge(&mut self) -> bool {
        let Some(receipt) = self.coordinator.as_ref().and_then(|coordinator| coordinator.receipt.clone()) else { return false };
        self.coordinator.as_mut().is_some_and(|coordinator| coordinator.acknowledge(&receipt))
    }

    pub fn advance(&mut self, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedGroupDecisionError> {
        self.coordinator.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?.advance_recovery(
            self.parent.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            self.drawing.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            self.value.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?,
            grant,
        )
    }

    pub fn take_terminal_owners(&mut self) -> Option<DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        if !self.coordinator.as_ref().is_some_and(DurableOwnedThreeStoreCommitV1::terminal_is_empty) || self.parent.is_none() || self.drawing.is_none() || self.value.is_none() {
            return None;
        }
        drop(self.coordinator.take().expect("validated terminal Map recovery coordinator remains owned"));
        Some(DurableOwnedMapRecoveryOwnersV1 {
            parent: self.parent.take().expect("validated terminal recovered parent Store remains owned"),
            drawing: self.drawing.take().expect("validated terminal recovered drawing Store remains owned"),
            value: self.value.take().expect("validated terminal recovered value Store remains owned"),
        })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.parent.is_none() && self.drawing.is_none() && self.value.is_none() && self.coordinator.is_none()
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedMapRecoveryOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ValueToValue + Clone + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ParentP>,
    DrawingP: ValueToValue + Clone + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<DrawingP>,
    ValueP: ValueToValue + Clone + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ValueP>,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.parent.is_none() && self.drawing.is_none() && self.value.is_none() && self.coordinator.is_none()), "retained committed Map recovery reached Drop before exact terminal owner handoff");
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapRecoveryHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ParentMutation: ArtifactCanonicalJsonTree + StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + semio_framework_os_spr::ColdMutation<ParentP> + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: ArtifactCanonicalJsonTree + StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + semio_framework_os_spr::ColdMutation<DrawingP> + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: ArtifactCanonicalJsonTree + StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + semio_framework_os_spr::ColdMutation<ValueP> + 'static,
{
    pub fn phase(&self) -> Option<DurableOwnedThreeStoreCommitPhaseV1> {
        self.operation.as_ref().and_then(DurableOwnedMapRecoveryOperationV1::phase)
    }

    pub fn capture_snapshot(&self) -> Option<DurableOwnedThreeStoreSnapshotV1<ParentP, DrawingP, ValueP>> {
        self.operation.as_ref()?.capture_snapshot().ok()
    }

    #[doc(hidden)]
    pub fn acknowledge_restoration(&mut self, expected: &DurableOwnedGroupJournalReceiptV1) -> bool {
        let Some(operation) = self.operation.as_mut() else { return false };
        let Some(actual) = operation.coordinator.as_ref().and_then(|coordinator| coordinator.receipt.as_ref()) else { return false };
        if actual != expected {
            return false;
        }
        operation.acknowledge()
    }

    pub fn advance(&mut self, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> DurableOwnedMapRecoveryAdvanceV1 {
        let Some(operation) = self.operation.as_mut() else { return DurableOwnedMapRecoveryAdvanceV1::Fault(DurableOwnedGroupDecisionError::InvalidOutcome) };
        match operation.advance(grant) {
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(phase)) => DurableOwnedMapRecoveryAdvanceV1::Progress(phase),
                        Ok(DurableOwnedThreeStoreCommitAdvanceV1::DiagnosticProgress(phase, progress)) => DurableOwnedMapRecoveryAdvanceV1::DiagnosticProgress(phase, progress),
Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked) => DurableOwnedMapRecoveryAdvanceV1::Blocked,
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(_)) => DurableOwnedMapRecoveryAdvanceV1::AwaitingAcknowledgement,
            Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete) => DurableOwnedMapRecoveryAdvanceV1::Complete,
            Err(error) => DurableOwnedMapRecoveryAdvanceV1::Fault(error),
        }
    }

    pub fn take_terminal_owners(&mut self) -> Option<DurableOwnedMapRecoveryOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        let owners = self.operation.as_mut()?.take_terminal_owners()?;
        drop(self.operation.take().expect("terminal Map recovery host retains its empty operation slot"));
        Some(owners)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.operation.is_none()
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedMapRecoveryHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ValueToValue + Clone + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ParentP>,
    DrawingP: ValueToValue + Clone + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<DrawingP>,
    ValueP: ValueToValue + Clone + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ValueP>,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.operation.is_none()), "committed Map recovery host reached Drop before terminal owner handoff");
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedMapCommitHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ValueToValue + Clone + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ParentP>,
    DrawingP: ValueToValue + Clone + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<DrawingP>,
    ValueP: ValueToValue + Clone + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ValueP>,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.operation.is_none()), "retained Map host reached Drop before exact terminal owner handoff");
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    ParentMutation: ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ParentP>,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    DrawingMutation: ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<DrawingP>,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    ValueMutation: ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ValueP>,
{
    pub fn from_store_prepared(
        parent: ArtifactStoreOneItemPrepared<ParentP, ParentMutation>,
        drawing: ArtifactStoreOneItemPrepared<DrawingP, DrawingMutation>,
        value: ArtifactStoreOneItemPrepared<ValueP, ValueMutation>,
    ) -> Result<Self, DurableOwnedGroupDecisionError> {
        Ok(Self::from_owners(DurableStorePreparedOwnerV1::original(parent), DurableStorePreparedOwnerV1::original(drawing), DurableStorePreparedOwnerV1::original(value)))
    }

    /// 🧷️ Gathers the already sealed original owners into one binding ladder.
    pub fn from_owners(parent: DurableStorePreparedOwnerV1<ParentP, ParentMutation>, drawing: DurableStorePreparedOwnerV1<DrawingP, DrawingMutation>, value: DurableStorePreparedOwnerV1<ValueP, ValueMutation>) -> Self {
        Self { parent, drawing, value, decision: None, decision_pack: None, binding_step: 0, decision_plan_phase: 0, decision_identity: Default::default(), issuers_captured:false, decision_retirement:None,decision_pack_retirement:None, retirement_progress:Default::default(), decision_encoding: Default::default()  }
    }
}

/// ♻️ Verifies every bound outcome of a decision against the three stores and answers its recovery frontier.
pub fn recover_decision_store_owned<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
    decision: &DurableOwnedThreeMemberDecisionV1,
    parent_store: &ArtifactStore<ParentP, ParentMutation>,
    drawing_store: &ArtifactStore<DrawingP, DrawingMutation>,
    value_store: &ArtifactStore<ValueP, ValueMutation>,
) -> Result<DurableOwnedThreeStoreRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, DurableOwnedGroupDecisionError>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    ParentMutation: ArtifactCanonicalJsonTree + StoreMutation<ParentP> + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ParentP>,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    DrawingMutation: ArtifactCanonicalJsonTree + StoreMutation<DrawingP> + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<DrawingP>,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    ValueMutation: ArtifactCanonicalJsonTree + StoreMutation<ValueP> + ValueToValue + ValueFromValue + Clone + semio_framework_os_spr::ColdMutation<ValueP>,
{
    decision.validate()?;
    let identities = [store_member_identity(&parent_store.parts_ref())?, store_member_identity(&drawing_store.parts_ref())?, store_member_identity(&value_store.parts_ref())?];
    for ((reference, owner), member) in identities.iter().zip([&decision.parent, &decision.drawing, &decision.value]) {
        if reference != &member.reference || owner != &member.owner {
            return Err(DurableOwnedGroupDecisionError::InvalidOwner);
        }
    }
    let frontiers = [recovery_frontier(&parent_store.parts_ref(), &decision.parent, &decision.decision_sha256)?, recovery_frontier(&drawing_store.parts_ref(), &decision.drawing, &decision.decision_sha256)?, recovery_frontier(&value_store.parts_ref(), &decision.value, &decision.decision_sha256)?];
    if frontiers.iter().all(|frontier| *frontier == DurableMemberRecoveryFrontier::Post) {
        return Ok(DurableOwnedThreeStoreRecoveryV1::AlreadyApplied);
    }
    if !frontiers.iter().all(|frontier| *frontier == DurableMemberRecoveryFrontier::Base) {
        return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
    }
    Ok(DurableOwnedThreeStoreRecoveryV1::Apply(DurableOwnedThreeStoreBoundV1 {
        parent: DurableStoreBoundOutcomeV1::recover(&parent_store.parts_ref(), &decision.parent, &decision.decision_sha256)?,
        drawing: DurableStoreBoundOutcomeV1::recover(&drawing_store.parts_ref(), &decision.drawing, &decision.decision_sha256)?,
        value: DurableStoreBoundOutcomeV1::recover(&value_store.parts_ref(), &decision.value, &decision.decision_sha256)?,
        decision: Box::new(decision.clone()),
        decision_pack: None,
    }))
}

