//! 🛠️ Map assembly recovery completion: the map assembly operations that complete the recovery, retire the owners and report the phases. See [`super`].

use super::*;

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapCommitHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
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
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
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
    ParentP: ValueToValue,
    ParentMutation: StoreMutation<ParentP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
    DrawingP: ValueToValue,
    DrawingMutation: StoreMutation<DrawingP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
    ValueP: ValueToValue,
    ValueMutation: StoreMutation<ValueP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.parent.is_none() && self.drawing.is_none() && self.value.is_none() && self.coordinator.is_none()), "retained committed Map recovery reached Drop before exact terminal owner handoff");
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapRecoveryHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
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
    ParentP: ValueToValue,
    ParentMutation: StoreMutation<ParentP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
    DrawingP: ValueToValue,
    DrawingMutation: StoreMutation<DrawingP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
    ValueP: ValueToValue,
    ValueMutation: StoreMutation<ValueP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.operation.is_none()), "committed Map recovery host reached Drop before terminal owner handoff");
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedMapCommitHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ValueToValue,
    ParentMutation: StoreMutation<ParentP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
    DrawingP: ValueToValue,
    DrawingMutation: StoreMutation<DrawingP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
    ValueP: ValueToValue,
    ValueMutation: StoreMutation<ValueP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.operation.is_none()), "retained Map host reached Drop before exact terminal owner handoff");
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    ParentMutation: ValueToValue + ValueFromValue,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    DrawingMutation: ValueToValue + ValueFromValue,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot,
    ValueMutation: ValueToValue + ValueFromValue,
{
    pub fn from_store_prepared(
        parent: ArtifactStoreOneItemPrepared<ParentP, ParentMutation>,
        drawing: ArtifactStoreOneItemPrepared<DrawingP, DrawingMutation>,
        value: ArtifactStoreOneItemPrepared<ValueP, ValueMutation>,
    ) -> Result<Self, DurableOwnedGroupDecisionError> {
        Ok(Self { parent: DurableStorePreparedOwnerV1::original(parent), drawing: DurableStorePreparedOwnerV1::original(drawing), value: DurableStorePreparedOwnerV1::original(value), decision: None, decision_pack: None, binding_step: 0, decision_plan_phase: 0, decision_packet_hash: None, decision_identity: Default::default(), issuers_captured:false, decision_retirement:None,decision_pack_retirement:None, retirement_progress:Default::default(), #[cfg(feature = "deflate")] decision_encoding: Default::default() })
    }
}

impl DurableOwnedThreeMemberDecisionV1 {
    pub fn recover_store_owned<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        &self,
        parent_store: &ArtifactStore<ParentP, ParentMutation>,
        drawing_store: &ArtifactStore<DrawingP, DrawingMutation>,
        value_store: &ArtifactStore<ValueP, ValueMutation>,
    ) -> Result<DurableOwnedThreeStoreRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, DurableOwnedGroupDecisionError>
    where
        ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
        ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue,
        DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
        DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue,
        ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
        ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue,
    {
        self.validate()?;
        let identities = [store_member_identity(&parent_store.parts_ref())?, store_member_identity(&drawing_store.parts_ref())?, store_member_identity(&value_store.parts_ref())?];
        for ((reference, owner), member) in identities.iter().zip([&self.parent, &self.drawing, &self.value]) {
            if reference != &member.reference || owner != &member.owner {
                return Err(DurableOwnedGroupDecisionError::InvalidOwner);
            }
        }
        let frontiers = [recovery_frontier(&parent_store.parts_ref(), &self.parent, &self.decision_sha256)?, recovery_frontier(&drawing_store.parts_ref(), &self.drawing, &self.decision_sha256)?, recovery_frontier(&value_store.parts_ref(), &self.value, &self.decision_sha256)?];
        if frontiers.iter().all(|frontier| *frontier == DurableMemberRecoveryFrontier::Post) {
            return Ok(DurableOwnedThreeStoreRecoveryV1::AlreadyApplied);
        }
        if !frontiers.iter().all(|frontier| *frontier == DurableMemberRecoveryFrontier::Base) {
            return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
        }
        Ok(DurableOwnedThreeStoreRecoveryV1::Apply(DurableOwnedThreeStoreBoundV1 {
            parent: DurableStoreBoundOutcomeV1::recover(&parent_store.parts_ref(), &self.parent, &self.decision_sha256)?,
            drawing: DurableStoreBoundOutcomeV1::recover(&drawing_store.parts_ref(), &self.drawing, &self.decision_sha256)?,
            value: DurableStoreBoundOutcomeV1::recover(&value_store.parts_ref(), &self.value, &self.decision_sha256)?,
            decision: Box::new(self.clone()),
            decision_pack: None,
        }))
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStorePreparedV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + ValueToValue + ValueFromValue,
{
    pub fn bind_store_owned(
        self,
        parent_store: &ArtifactStore<ParentP, ParentMutation>,
        drawing_store: &ArtifactStore<DrawingP, DrawingMutation>,
        value_store: &ArtifactStore<ValueP, ValueMutation>,
    ) -> Result<DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, DurableOwnedGroupDecisionError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        let (parent_reference, parent_owner) = store_member_identity(&parent_store.parts_ref())?;
        let (drawing_reference, drawing_owner) = store_member_identity(&drawing_store.parts_ref())?;
        let (value_reference, value_owner) = store_member_identity(&value_store.parts_ref())?;
        if parent_reference == drawing_reference || parent_reference == value_reference || drawing_reference == value_reference {
            return Err(DurableOwnedGroupDecisionError::InvalidIdentity);
        }
        let anchor = DurableOwnedGroupAnchorV1 { schema: DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1.into(), parent: parent_reference.clone(), shape: DURABLE_OWNED_GROUP_SHAPE_V1.into() };
        let parent = prepared_member(PARENT_ROLE, parent_reference.clone(), parent_owner.clone(), &self.parent)?;
        let drawing = prepared_member(DRAWING_ROLE, drawing_reference.clone(), drawing_owner.clone(), &self.drawing)?;
        let value = prepared_member(VALUE_ROLE, value_reference.clone(), value_owner.clone(), &self.value)?;
        if !member_identity_matches(&parent, PARENT_ROLE, PARENT_RECOVERY_SCHEMA, &parent_reference.to_uri(), &anchor)
            || !member_identity_matches(&drawing, DRAWING_ROLE, DRAWING_RECOVERY_SCHEMA, "gismap-drawing!s.stdio.semio@v1/drawing", &anchor)
            || !member_identity_matches(&value, VALUE_ROLE, VALUE_RECOVERY_SCHEMA, "gismap-value!s.stdio.semio@v1/value", &anchor)
            || !prepared_base_matches_store(&parent_store.parts_ref(), &parent, &self.parent.prepared)
            || !prepared_base_matches_store(&drawing_store.parts_ref(), &drawing, &self.drawing.prepared)
            || !prepared_base_matches_store(&value_store.parts_ref(), &value, &self.value.prepared)
        {
            return Err(DurableOwnedGroupDecisionError::InvalidOwner);
        }
        let anchor_sha256 = semio_framework_hash::sha256_hex(semio_framework_pack_json::to_json_string(&anchor).as_bytes());
        let mut decision = DurableOwnedThreeMemberDecisionV1 { schema: DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1.into(), anchor, anchor_sha256, decision_sha256: String::new(), parent, drawing, value };
        decision.decision_sha256 = semio_framework_hash::sha256_hex(decision.canonical_unsigned_json().as_bytes());
        let parent = self.parent.bind(&parent_store.parts_ref(), &decision.decision_sha256)?;
        let drawing = self.drawing.bind(&drawing_store.parts_ref(), &decision.decision_sha256)?;
        let value = self.value.bind(&value_store.parts_ref(), &decision.decision_sha256)?;
        decision.parent = bound_member(PARENT_ROLE, parent_reference, parent_owner, &parent);
        decision.drawing = bound_member(DRAWING_ROLE, drawing_reference, drawing_owner, &drawing);
        decision.value = bound_member(VALUE_ROLE, value_reference, value_owner, &value);
        decision.validate()?;
        Ok(DurableOwnedThreeStoreBoundV1 { decision, parent, drawing, value })
    }
}
