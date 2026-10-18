//! 🚢️ Map assembly commit and publication steps: the map assembly operations that commit, publish and verify the three member stores. See [`super`].

use super::*;

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    DrawingP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
    ValueP: ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Clone + ValueToValue + ValueFromValue + Send + Sync + 'static,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue + Send + 'static,
{
    pub fn phase(&self) -> DurableOwnedThreeStoreCommitPhaseV1 {
        self.phase
    }

    pub fn cancel(&mut self) -> bool {
        if self.recovery_committed {
            return false;
        }
        if matches!(
            self.phase,
            DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease
                | DurableOwnedThreeStoreCommitPhaseV1::PublishingDrawingLease
                | DurableOwnedThreeStoreCommitPhaseV1::PublishingValueLease
                | DurableOwnedThreeStoreCommitPhaseV1::AdoptingParent
                | DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing
                | DurableOwnedThreeStoreCommitPhaseV1::AdoptingValue
                | DurableOwnedThreeStoreCommitPhaseV1::ClearingParent
                | DurableOwnedThreeStoreCommitPhaseV1::ClearingDrawing
                | DurableOwnedThreeStoreCommitPhaseV1::ClearingValue
                | DurableOwnedThreeStoreCommitPhaseV1::AwaitingAck
                | DurableOwnedThreeStoreCommitPhaseV1::ClosingJournal
                | DurableOwnedThreeStoreCommitPhaseV1::Complete
        ) {
            return false;
        }
        self.cancel_requested = true;
        true
    }

    fn begin_abort(&mut self) -> Result<(), DurableOwnedGroupDecisionError> {
        if !self.visibility_owner.as_mut().is_some_and(semio_framework_os_vcs::ArtifactGroupVisibilityOwner::abort) {
            return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
        }
        self.phase = DurableOwnedThreeStoreCommitPhaseV1::AbortingValue;
        Ok(())
    }

    pub fn visibility(&self) -> Result<Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, DurableOwnedGroupDecisionError> {
        self.visibility.as_ref().map(Arc::clone).ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)
    }

    fn start_journal_close(&mut self) {
        if self.journal_close_started {
            return;
        }
        if let Some(journal) = self.journal.as_mut() {
            journal.begin_close();
            self.journal_close_started = true;
        }
    }

    pub fn acknowledge(&mut self, receipt: &DurableOwnedGroupJournalReceiptV1) -> bool {
        if self.phase != DurableOwnedThreeStoreCommitPhaseV1::AwaitingAck || self.receipt.as_ref() != Some(receipt) {
            return false;
        }
        self.start_journal_close();
        self.phase = DurableOwnedThreeStoreCommitPhaseV1::ClosingJournal;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == DurableOwnedThreeStoreCommitPhaseV1::Complete
            && self.parent.is_none()
            && self.drawing.is_none()
            && self.value.is_none()
            && self.decision.is_none()
            && self.decision_pack.is_none()
            && self.decision_sha256.is_empty()
            && self.visibility_owner.is_none()
            && self.visibility.is_none()
            && self.journal.is_none()
            && self.receipt.is_none()
    }

    pub fn mount_map(
        self,
        parent: ArtifactStore<ParentP, ParentMutation>,
        drawing: ArtifactStore<DrawingP, DrawingMutation>,
        value: ArtifactStore<ValueP, ValueMutation>,
        sink: Box<dyn DurableOwnedGroupJournalSinkV1>,
    ) -> DurableOwnedMapCommitHostV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
        DurableOwnedMapCommitHostV1 {
            operation: std::mem::ManuallyDrop::new(Some(DurableOwnedMapCommitOperationV1 {
                parent: std::mem::ManuallyDrop::new(Some(parent)),
                drawing: std::mem::ManuallyDrop::new(Some(drawing)),
                value: std::mem::ManuallyDrop::new(Some(value)),
                coordinator: std::mem::ManuallyDrop::new(Some(self)),
                sink: std::mem::ManuallyDrop::new(Some(sink)),
            })),
        }
    }

    pub fn advance(
        &mut self,
        parent_store: &mut ArtifactStore<ParentP, ParentMutation>,
        drawing_store: &mut ArtifactStore<DrawingP, DrawingMutation>,
        value_store: &mut ArtifactStore<ValueP, ValueMutation>,
        sink: &mut dyn DurableOwnedGroupJournalSinkV1,
        grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant,
    ) -> Result<DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedGroupDecisionError> {
        self.advance_with_sink(parent_store, drawing_store, value_store, Some(sink), grant)
    }

    pub(super) fn advance_recovery(
        &mut self,
        parent_store: &mut ArtifactStore<ParentP, ParentMutation>,
        drawing_store: &mut ArtifactStore<DrawingP, DrawingMutation>,
        value_store: &mut ArtifactStore<ValueP, ValueMutation>,
        grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant,
    ) -> Result<DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedGroupDecisionError> {
        self.advance_with_sink(parent_store, drawing_store, value_store, None, grant)
    }

    fn advance_with_sink(
        &mut self,
        parent_store: &mut ArtifactStore<ParentP, ParentMutation>,
        drawing_store: &mut ArtifactStore<DrawingP, DrawingMutation>,
        value_store: &mut ArtifactStore<ValueP, ValueMutation>,
        sink: Option<&mut dyn DurableOwnedGroupJournalSinkV1>,
        grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant,
    ) -> Result<DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedGroupDecisionError> {
        if self.phase == DurableOwnedThreeStoreCommitPhaseV1::Complete {
            return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete);
        }
        if !grant.permits_one() {
            return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
        }
        if self.cancel_requested
            && matches!(self.phase, DurableOwnedThreeStoreCommitPhaseV1::StagingParent | DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing | DurableOwnedThreeStoreCommitPhaseV1::StagingValue | DurableOwnedThreeStoreCommitPhaseV1::StartingJournal)
        {
            self.begin_abort()?;
            return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(self.phase));
        }
        match self.phase {
            DurableOwnedThreeStoreCommitPhaseV1::StagingParent => {
                let outcome = self.parent.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match stage_store_member(&mut parent_store.parts(), outcome, &self.visibility()?, grant) {
                    Ok(()) => self.phase = DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing,
                    Err((error, outcome)) => {
                        self.parent = Some(outcome);
                        if !self.recovery_committed {
                            self.begin_abort()?;
                        }
                        return Err(error);
                    }
                }
            }
            DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing => {
                let outcome = self.drawing.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match stage_store_member(&mut drawing_store.parts(), outcome, &self.visibility()?, grant) {
                    Ok(()) => self.phase = DurableOwnedThreeStoreCommitPhaseV1::StagingValue,
                    Err((error, outcome)) => {
                        self.drawing = Some(outcome);
                        if !self.recovery_committed {
                            self.begin_abort()?;
                        }
                        return Err(error);
                    }
                }
            }
            DurableOwnedThreeStoreCommitPhaseV1::StagingValue => {
                let outcome = self.value.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                match stage_store_member(&mut value_store.parts(), outcome, &self.visibility()?, grant) {
                    Ok(()) => self.phase = DurableOwnedThreeStoreCommitPhaseV1::StartingJournal,
                    Err((error, outcome)) => {
                        self.value = Some(outcome);
                        if !self.recovery_committed {
                            self.begin_abort()?;
                        }
                        return Err(error);
                    }
                }
            }
            DurableOwnedThreeStoreCommitPhaseV1::StartingJournal => {
                if self.recovery_committed {
                    let receipt = self.receipt.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                    if receipt.decision_sha256 != self.decision_sha256 || self.decision.as_ref().is_none_or(|decision| receipt.anchor_sha256 != decision.anchor_sha256) {
                        return Err(DurableOwnedGroupDecisionError::InvalidHash);
                    }
                    if !self.visibility_owner.as_mut().is_some_and(semio_framework_os_vcs::ArtifactGroupVisibilityOwner::commit) {
                        return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
                    }
                    self.phase = DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease;
                } else {
                    let decision_bytes = self.decision_pack.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?.len();
                    if grant.maximum_copy_bytes < decision_bytes {
                        return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
                    }
                    let decision_pack = self.decision_pack.take().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                    self.journal = Some(sink.ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?.begin_commit(decision_pack, self.decision_sha256.clone()));
                    self.phase = DurableOwnedThreeStoreCommitPhaseV1::Journal;
                }
            }
            DurableOwnedThreeStoreCommitPhaseV1::Journal => {
                let journal = self.journal.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
                if self.cancel_requested && !self.cancel_forwarded {
                    journal.cancel();
                    self.cancel_forwarded = true;
                }
                match journal.advance(semio_framework_os_store_one_item::ArtifactStoreOneItemGrant { maximum_items: 1, ..grant }).map_err(DurableOwnedGroupDecisionError::Codec)? {
                    DurableOwnedGroupJournalAdvanceV1::Pending => return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(self.phase)),
                    DurableOwnedGroupJournalAdvanceV1::Absent => {
                        self.begin_abort()?;
                    }
                    DurableOwnedGroupJournalAdvanceV1::Rejected(reason) => {
                        self.begin_abort()?;
                        return Err(DurableOwnedGroupDecisionError::Codec(reason));
                    }
                    DurableOwnedGroupJournalAdvanceV1::Committed(receipt) => {
                        if receipt.decision_sha256 != self.decision_sha256 || self.decision.as_ref().is_none_or(|decision| receipt.anchor_sha256 != decision.anchor_sha256) {
                            return Err(DurableOwnedGroupDecisionError::InvalidHash);
                        }
                        if !self.visibility_owner.as_mut().is_some_and(semio_framework_os_vcs::ArtifactGroupVisibilityOwner::commit) {
                            return Err(DurableOwnedGroupDecisionError::InvalidFrontier);
                        }
                        self.receipt = Some(receipt);
                        self.phase = DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease;
                    }
                }
            }
            DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease => {
                let root = parent_store.wire.durable_group_root.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
                if !parent_store.owners.snapshot_read_leases.publish_authority(root.generation, root.content_revision) {
                    return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
                }
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::PublishingDrawingLease;
            }
            DurableOwnedThreeStoreCommitPhaseV1::PublishingDrawingLease => {
                let root = drawing_store.wire.durable_group_root.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
                if !drawing_store.owners.snapshot_read_leases.publish_authority(root.generation, root.content_revision) {
                    return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
                }
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::PublishingValueLease;
            }
            DurableOwnedThreeStoreCommitPhaseV1::PublishingValueLease => {
                let root = value_store.wire.durable_group_root.as_ref().ok_or(DurableOwnedGroupDecisionError::InvalidFrontier)?;
                if !value_store.owners.snapshot_read_leases.publish_authority(root.generation, root.content_revision) {
                    return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
                }
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AdoptingParent;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AdoptingParent => {
                adopt_staged_store_member(&mut parent_store.parts(), &self.visibility()?)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing => {
                adopt_staged_store_member(&mut drawing_store.parts(), &self.visibility()?)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AdoptingValue;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AdoptingValue => {
                adopt_staged_store_member(&mut value_store.parts(), &self.visibility()?)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::ClearingParent;
            }
            DurableOwnedThreeStoreCommitPhaseV1::ClearingParent => {
                validate_adopted_store_member(&parent_store.parts_ref(), &self.visibility()?)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::ClearingDrawing;
            }
            DurableOwnedThreeStoreCommitPhaseV1::ClearingDrawing => {
                validate_adopted_store_member(&drawing_store.parts_ref(), &self.visibility()?)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::ClearingValue;
            }
            DurableOwnedThreeStoreCommitPhaseV1::ClearingValue => {
                let visibility = self.visibility()?;
                validate_adopted_store_member(&value_store.parts_ref(), &visibility)?;
                clear_adopted_store_member(&mut parent_store.parts(), &visibility)?;
                clear_adopted_store_member(&mut drawing_store.parts(), &visibility)?;
                clear_adopted_store_member(&mut value_store.parts(), &visibility)?;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AwaitingAck;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AwaitingAck => {
                return Ok(DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(self.receipt.clone().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?));
            }
            DurableOwnedThreeStoreCommitPhaseV1::AbortingValue => {
                if self.value.is_some() {
                    retire_unstaged_store_member(&mut value_store.parts(), &mut self.value, grant)?;
                } else {
                    abort_staged_store_member(&mut value_store.parts(), &self.visibility()?)?;
                }
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AbortingDrawing;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AbortingDrawing => {
                if self.drawing.is_some() {
                    retire_unstaged_store_member(&mut drawing_store.parts(), &mut self.drawing, grant)?;
                } else {
                    abort_staged_store_member(&mut drawing_store.parts(), &self.visibility()?)?;
                }
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::AbortingParent;
            }
            DurableOwnedThreeStoreCommitPhaseV1::AbortingParent => {
                if self.parent.is_some() {
                    retire_unstaged_store_member(&mut parent_store.parts(), &mut self.parent, grant)?;
                } else {
                    abort_staged_store_member(&mut parent_store.parts(), &self.visibility()?)?;
                }
                self.start_journal_close();
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::ClosingJournal;
            }
            DurableOwnedThreeStoreCommitPhaseV1::ClosingJournal => {
                if self.journal.is_some() {
                    if grant.maximum_depth == 0 { return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked); }
                    let child = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant.retained_grant() };
                    match close_journal_owner(&mut self.journal, child).map_err(DurableOwnedGroupDecisionError::Retirement)? {
                        semio_framework_value::retained_clone::RetainedCloneStep::Complete(_) => {},
                        semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) if progress == Default::default() => return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Blocked),
                        semio_framework_value::retained_clone::RetainedCloneStep::Progress(_) => return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(self.phase)),
                    }
                }
                self.decision = None;
                self.decision_pack = None;
                self.decision_sha256.clear();
                self.visibility_owner = None;
                self.visibility = None;
                self.receipt = None;
                self.phase = DurableOwnedThreeStoreCommitPhaseV1::Complete;
            }
            DurableOwnedThreeStoreCommitPhaseV1::Complete => return Ok(DurableOwnedThreeStoreCommitAdvanceV1::Complete),
        }
        Ok(DurableOwnedThreeStoreCommitAdvanceV1::Progress(self.phase))
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedThreeStoreCommitV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> {
    fn drop(&mut self) {
        assert!(
            std::thread::panicking()
                || self.phase == DurableOwnedThreeStoreCommitPhaseV1::Complete
                && self.parent.is_none()
                && self.drawing.is_none()
                && self.value.is_none()
                && self.decision.is_none()
                && self.decision_pack.is_none()
                && self.decision_sha256.is_empty()
                && self.visibility_owner.is_none()
                && self.visibility.is_none()
                && self.journal.is_none()
                && self.receipt.is_none(),
            "durable owned group coordinator reached Drop before exact terminal emptiness"
        );
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapCommitOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
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

    pub fn cancel(&mut self) -> bool {
        self.coordinator.as_mut().is_some_and(DurableOwnedThreeStoreCommitV1::cancel)
    }

    pub(super) fn acknowledge_retained(&mut self) -> bool {
        let Some(receipt) = self.coordinator.as_ref().and_then(|coordinator| coordinator.receipt.clone()) else { return false };
        self.coordinator.as_mut().is_some_and(|coordinator| coordinator.acknowledge(&receipt))
    }

    pub fn advance(&mut self, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedGroupDecisionError> {
        let coordinator = self.coordinator.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let parent = self.parent.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let drawing = self.drawing.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let value = self.value.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        let sink = self.sink.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
        coordinator.advance(parent, drawing, value, sink.as_mut(), grant)
    }

    pub fn take_terminal_owners(&mut self) -> Option<DurableOwnedMapCommitOwnersV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>> {
        if !self.coordinator.as_ref().is_some_and(DurableOwnedThreeStoreCommitV1::terminal_is_empty) || self.parent.is_none() || self.drawing.is_none() || self.value.is_none() || self.sink.is_none() {
            return None;
        }
        drop(self.coordinator.take().expect("validated terminal Map coordinator remains owned"));
        Some(DurableOwnedMapCommitOwnersV1 {
            parent: self.parent.take().expect("validated terminal parent Store remains owned"),
            drawing: self.drawing.take().expect("validated terminal drawing Store remains owned"),
            value: self.value.take().expect("validated terminal value Store remains owned"),
            sink: self.sink.take().expect("validated terminal journal sink remains owned"),
        })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.parent.is_none() && self.drawing.is_none() && self.value.is_none() && self.coordinator.is_none() && self.sink.is_none()
    }
}

impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedMapCommitOperationV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
where
    ParentP: Clone + ValueToValue + ValueFromValue,
    ParentMutation: StoreMutation<ParentP> + Clone + ValueToValue + ValueFromValue,
    DrawingP: Clone + ValueToValue + ValueFromValue,
    DrawingMutation: StoreMutation<DrawingP> + Clone + ValueToValue + ValueFromValue,
    ValueP: Clone + ValueToValue + ValueFromValue,
    ValueMutation: StoreMutation<ValueP> + Clone + ValueToValue + ValueFromValue,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.parent.is_none() && self.drawing.is_none() && self.value.is_none() && self.coordinator.is_none() && self.sink.is_none()), "retained Map commit operation reached Drop before exact terminal owner handoff");
    }
}
