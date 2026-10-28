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

impl DurableOwnedThreeMemberDecisionV1 {
    pub fn recover_store_owned<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        &self,
        parent_store: &ArtifactStore<ParentP, ParentMutation>,
        drawing_store: &ArtifactStore<DrawingP, DrawingMutation>,
        value_store: &ArtifactStore<ValueP, ValueMutation>,
    ) -> Result<DurableOwnedThreeStoreRecoveryV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, DurableOwnedGroupDecisionError>
    where
        ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + ValueToValue,
        ParentMutation: StoreMutation<ParentP> + ValueToValue + ValueFromValue,
        DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + ValueToValue,
        DrawingMutation: StoreMutation<DrawingP> + ValueToValue + ValueFromValue,
        ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + ValueToValue,
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
    ParentP: ArtifactPack + ArtifactDsl + BorrowedDslRecord + FieldProjectionSource + semio_framework_os_store_text_io::ArtifactStoreSnapshot + ValueToValue + Send + Sync + 'static,
    ParentMutation: StoreMutation<ParentP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    DrawingP: ArtifactPack + ArtifactDsl + BorrowedDslRecord + FieldProjectionSource + semio_framework_os_store_text_io::ArtifactStoreSnapshot + ValueToValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
    ValueP: ArtifactPack + ArtifactDsl + BorrowedDslRecord + FieldProjectionSource + semio_framework_os_store_text_io::ArtifactStoreSnapshot + ValueToValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + ArtifactCanonicalJsonTree + ValueToValue + ValueFromValue + semio_framework_os_spr::OpBinary + semio_framework_os_spr::OpText + Send + 'static,
{
    /// 🧵️ Plans every member before transferring one original candidate on a separately funded turn.
    pub fn advance_binding(&mut self, parent: &ArtifactStore<ParentP, ParentMutation>, drawing: &ArtifactStore<DrawingP, DrawingMutation>, value: &ArtifactStore<ValueP, ValueMutation>, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant, native: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<Option<semio_framework_value::RetainedCloneProgress>, DurableOwnedGroupDecisionError> {
        use semio_framework_value::{RetirementDemand, retained_clone::{retained_clone_grant_admits, RetainedCloneProgress}};
        let grant = grant.retained_grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(None); }
        let error = DurableOwnedGroupDecisionError::Retirement;
        if !self.issuers_captured {
            let copy = 3 * (size_of::<Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<ParentMutation>>>() + size_of::<Arc<dyn semio_framework_value::SnapshotRetirementFactory<ParentP>>>());
            let demand = RetirementDemand { copy_bytes: copy, depth: 1, ..Default::default() };
            if !retained_clone_grant_admits(grant, demand) { return Ok(None); }
            if parent.owners.mutation_retirement_factory.is_none() || parent.owners.snapshot_retirement_factory.is_none() || drawing.owners.mutation_retirement_factory.is_none() || drawing.owners.snapshot_retirement_factory.is_none() || value.owners.mutation_retirement_factory.is_none() || value.owners.snapshot_retirement_factory.is_none() { return Err(DurableOwnedGroupDecisionError::InvalidOutcome); }
            let mut receipt = RetainedCloneProgress { copied_items: 1, ..Default::default() };
            for progress in [self.parent.capture_issuers(&parent.parts_ref(), grant)?, self.drawing.capture_issuers(&drawing.parts_ref(), grant)?, self.value.capture_issuers(&value.parts_ref(), grant)?] {
                let progress = progress.ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                receipt.copied_bytes += progress.copied_bytes;
                receipt.retained_capacity_bytes += progress.retained_capacity_bytes;
                receipt.released_bytes += progress.released_bytes;
            }
            self.issuers_captured = true;
            return Ok(Some(receipt));
        }
        if self.binding_step < 8 {
            for valid in [self.parent.prepared().is_some_and(|original| original.authority().generation().0 == parent.cursor.generation && original.authority().base_revision() == parent.cursor.content_revision), self.drawing.prepared().is_some_and(|original| original.authority().generation().0 == drawing.cursor.generation && original.authority().base_revision() == drawing.cursor.content_revision), self.value.prepared().is_some_and(|original| original.authority().generation().0 == value.cursor.generation && original.authority().base_revision() == value.cursor.content_revision)] { if !valid { return Err(DurableOwnedGroupDecisionError::InvalidFrontier); } }
        }
        let progress = match self.binding_step {
            0 => { let step = self.parent.advance_unbound(PARENT_RECOVERY_SCHEMA, grant, native)?; if self.parent.unbound_ready() { self.binding_step = 1; } step },
            1 => { let step = self.drawing.advance_unbound(DRAWING_RECOVERY_SCHEMA, grant, native)?; if self.drawing.unbound_ready() { self.binding_step = 2; } step },
            2 => { let step = self.value.advance_unbound(VALUE_RECOVERY_SCHEMA, grant, native)?; if self.value.unbound_ready() { self.binding_step = 3; } step },
            3 if self.decision.is_none() => {
                let bytes = size_of::<DurableOwnedThreeMemberDecisionV1>();
                let demand = RetirementDemand { copy_bytes: bytes, capacity_bytes: bytes, depth: 1, ..Default::default() };
                if !retained_clone_grant_admits(grant, demand) { return Ok(None); }
                native.checkpoint().map_err(error)?;
                native.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(error)?;
                native.charge(bytes).map_err(error)?;
                self.decision = Some(Box::new(DurableOwnedThreeMemberDecisionV1 { schema: String::new(), anchor: DurableOwnedGroupAnchorV1 { schema: String::new(), parent: semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } }, shape: String::new() }, anchor_sha256: String::new(), decision_sha256: String::new(), parent: empty_binding_member(), drawing: empty_binding_member(), value: empty_binding_member() }));
                Some(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: bytes, ..Default::default() })
            },
            3 if self.decision_plan_phase < 7 => {
                let envelope = &parent.log.envelope;
                let dialect = envelope.dialect.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidIdentity)?;
                let original = match self.decision_plan_phase { 0 => DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1, 1 => DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1, 2 => DURABLE_OWNED_GROUP_SHAPE_V1, 3 => &envelope.id, 4 => &dialect.artifact_kind, 5 => &dialect.standard, _ => &dialect.subset };
                let demand = RetirementDemand { copy_bytes: original.len() + size_of::<String>(), capacity_bytes: original.len(), depth: 1, ..Default::default() };
                if !retained_clone_grant_admits(grant, demand) { return Ok(None); }
                native.checkpoint().map_err(error)?;
                native.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(error)?;
                let text = native.copy_text(original).map_err(error)?;
                let decision = self.decision.as_mut().unwrap();
                match self.decision_plan_phase { 0 => decision.schema = text, 1 => decision.anchor.schema = text, 2 => decision.anchor.shape = text, 3 => decision.anchor.parent.artifact_id = text, 4 => decision.anchor.parent.dialect.artifact_kind = text, 5 => decision.anchor.parent.dialect.standard = text, _ => decision.anchor.parent.dialect.subset = text }
                self.decision_plan_phase += 1;
                Some(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() })
            },
            3 => {
                let next = match self.decision_plan_phase { 7 => self.parent.prepared_member_from_store(PARENT_ROLE, &parent.parts_ref(), grant, native)?, 8 => self.drawing.prepared_member_from_store(DRAWING_ROLE, &drawing.parts_ref(), grant, native)?, _ => self.value.prepared_member_from_store(VALUE_ROLE, &value.parts_ref(), grant, native)? };
                let Some((member, receipt)) = next else { return Ok(None); };
                let decision = self.decision.as_mut().unwrap();
                match self.decision_plan_phase { 7 => decision.parent = member, 8 => decision.drawing = member, _ => decision.value = member }
                self.decision_plan_phase += 1;
                if self.decision_plan_phase == 10 { self.decision_identity.initialize(); self.binding_step = 4; self.decision_plan_phase = 0; }
                Some(receipt)
            },
            4 => { let receipt = self.decision_identity.advance(self.decision.as_mut().unwrap().as_mut(), grant, native).map_err(error)?; if self.decision_identity.ready() { self.binding_step = 5; } (receipt != Default::default()).then_some(receipt) },
            5 => { let step = self.parent.advance_bound(&parent.parts_ref(), &self.decision.as_ref().unwrap().decision_sha256, grant, native)?; if self.parent.plan_ready() { self.binding_step = 6; } step },
            6 => { let step = self.drawing.advance_bound(&drawing.parts_ref(), &self.decision.as_ref().unwrap().decision_sha256, grant, native)?; if self.drawing.plan_ready() { self.binding_step = 7; } step },
            7 => { let step = self.value.advance_bound(&value.parts_ref(), &self.decision.as_ref().unwrap().decision_sha256, grant, native)?; if self.value.plan_ready() { self.binding_step = 8; } step },
            8 => { let step = self.parent.transfer_bound(grant)?; if self.parent.bound().is_some() { self.binding_step = 9; } step },
            9 => { let step = self.drawing.transfer_bound(grant)?; if self.drawing.bound().is_some() { self.binding_step = 10; } step },
            10 => { let step = self.value.transfer_bound(grant)?; if self.value.bound().is_some() { self.binding_step = 11; } step },
            11 => { let step = advance_binding_member(self.parent.bound().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?, &mut self.decision.as_mut().unwrap().parent, grant, native)?; if step.0 { self.binding_step = 12; } step.1 },
            12 => { let step = advance_binding_member(self.drawing.bound().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?, &mut self.decision.as_mut().unwrap().drawing, grant, native)?; if step.0 { self.binding_step = 13; } step.1 },
            13 if self.decision_plan_phase == 0 => { let step = advance_binding_member(self.value.bound().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?, &mut self.decision.as_mut().unwrap().value, grant, native)?; if step.0 { self.decision_plan_phase = 1; } step.1 },
            13 => {
                if !self.decision_encoding.ready() { let receipt = self.decision_encoding.advance(self.decision.as_ref().unwrap().as_ref(), DurableOwnedThreeMemberDecisionV1::envelope_id(), DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, grant, native).map_err(error)?; (receipt != Default::default()).then_some(receipt) }
                else { let Some((pack, receipt)) = self.decision_encoding.take(grant) else { return Ok(None); }; self.decision_pack = Some(pack); self.binding_step = 14; self.decision_plan_phase=0; Some(receipt) }
            },
            14 => {
                let (done,receipt)=match self.decision_plan_phase {
                    0=>self.parent.close_binding_residue(grant)?,
                    1=>self.drawing.close_binding_residue(grant)?,
                    2=>self.value.close_binding_residue(grant)?,
                    3=>{let receipt=self.decision_identity.close(grant).map_err(error)?;(self.decision_identity.ready(),receipt)},
                    _=>{let receipt=self.decision_encoding.close(grant).map_err(error)?;(self.decision_encoding.terminal_is_empty(),receipt)},
                };
                if done{self.decision_plan_phase+=1;if self.decision_plan_phase==5{self.binding_step=15;}}
                (receipt!=Default::default()).then_some(receipt)
            },
            _ => None,
        };
        Ok(progress)
    }

    /// 📦️ Transfers the already encoded fixed-three result only after all original member plans complete.
    pub fn take_bound(&mut self, grant: semio_framework_value::RetainedCloneGrant) -> Result<Option<(DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>, semio_framework_value::RetainedCloneProgress)>, DurableOwnedGroupDecisionError> {
        let copy = size_of::<DurableOwnedThreeStoreBoundV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>>();
        if self.binding_step != 15 || grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < copy { return Ok(None); }
        let bound = DurableOwnedThreeStoreBoundV1 { decision: self.decision.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?, parent: self.parent.take_bound().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?, drawing: self.drawing.take_bound().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?, value: self.value.take_bound().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?, decision_pack: self.decision_pack.take() };
        Ok(Some((bound, semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() })))
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

fn empty_binding_member() -> DurableOwnedGroupMemberV1 {
    DurableOwnedGroupMemberV1 { role: String::new(), reference: semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } }, owner: None, expected_generation: 0, expected_revision: [0;32], recovery_schema: String::new(), recovery_pack: Vec::new(), recovery_pack_sha256: String::new(), unbound_outcome_sha256: String::new(), post_generation: 0, post_revision: [0;32] }
}

fn advance_binding_member<P, M>(bound: &DurableStoreBoundOutcomeV1<P, M>, member: &mut DurableOwnedGroupMemberV1, grant: semio_framework_value::RetainedCloneGrant, native: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(bool, Option<semio_framework_value::RetainedCloneProgress>), DurableOwnedGroupDecisionError> {
    use semio_framework_value::{RetirementDemand, RetainedCloneProgress, retained_clone::retained_clone_grant_admits};
    let error = DurableOwnedGroupDecisionError::Retirement;
    let demand = if member.recovery_pack.capacity() == 0 && !bound.recovery_pack.is_empty() { RetirementDemand { copy_bytes: size_of::<Vec<u8>>(), capacity_bytes: bound.recovery_pack.len(), depth: 1, ..Default::default() } } else if member.recovery_pack.len() < bound.recovery_pack.len() { RetirementDemand { copy_bytes: 1 + size_of::<usize>(), depth: 1, ..Default::default() } } else if member.recovery_pack_sha256.is_empty() { RetirementDemand { copy_bytes: bound.recovery_pack_sha256.len() + size_of::<u64>() + 32, capacity_bytes: bound.recovery_pack_sha256.len(), depth: 1, ..Default::default() } } else { return Ok((true, None)); };
    if !retained_clone_grant_admits(grant, demand) { return Ok((false, None)); }
    native.checkpoint().map_err(error)?;
    if demand.capacity_bytes != 0 { native.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(error)?; }
    if member.recovery_pack.capacity() == 0 && !bound.recovery_pack.is_empty() { member.recovery_pack = native.allocate_vec(bound.recovery_pack.len()).map_err(error)?; }
    else if member.recovery_pack.len() < bound.recovery_pack.len() { member.recovery_pack.push(bound.recovery_pack[member.recovery_pack.len()]); }
    else { member.recovery_pack_sha256 = native.copy_text(&bound.recovery_pack_sha256).map_err(error)?; member.post_generation = bound.post_generation; member.post_revision = bound.post_revision; }
    Ok((false, Some(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() })))
}
