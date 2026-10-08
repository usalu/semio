//! 🏪️ Store-specific retirement implementations over the neutral owner contract.
use semio_framework_value::artifact_retire_struct;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor, sequence};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{RetirementStep, controlled::ControlledRetirement};
use std::mem::ManuallyDrop;

struct HistoryPageRetirement<T: RetireOwned> {
    original: ManuallyDrop<crate::os_vcs::HistoryPageStack<T>>,
    active: ManuallyDrop<Option<ControlledRetirement<T>>>,
}

impl<T: RetireOwned> RetirementCursor for HistoryPageRetirement<T> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        match self.next_depth_demand() {
            Err(error) => return RetirementStep::Failure(error),
            Ok(depth) if grant.maximum_depth < depth => return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::DepthLimit, "Store history page exceeds admitted depth")),
            _ => {}
        }
        if let Some(active) = self.active.as_mut() {
            if active.terminal_is_empty() {
                drop(self.active.take());
                return RetirementStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() });
            }
            return match active.step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant }) {
                Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => RetirementStep::Progress(progress),
                Err(error) => RetirementStep::Failure(error),
            };
        }
        if let Some(value) = self.original.pop() {
            *self.active = Some(ControlledRetirement::new(value).unwrap_or_else(|_| unreachable!("history entry declares controlled retirement")));
            return RetirementStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() });
        }
        let Some(release) = self.original.next_empty_page_release_byte_demand() else { return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated, "empty history page lost its release extent")); };
        if release == 0 { return RetirementStep::Complete; }
        if grant.maximum_release_bytes < release { return RetirementStep::BudgetExhausted; }
        if !self.original.release_empty_page() { return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated, "history page changed after exact release inquiry")); }
        RetirementStep::Bytes(release)
    }
    fn terminal_is_empty(&self) -> bool { self.original.is_empty() && self.original.capacity() == 0 && self.active.is_none() }
    fn next_work_byte_demand(&self) -> Result<usize, ValueError> { self.active.as_ref().map_or(Ok(0), ControlledRetirement::next_copy_byte_demand) }
    fn next_birth_bytes(&self, copy: usize) -> Option<usize> { self.active.as_ref().map_or(Some(0), |active| active.next_capacity_byte_demand(copy).ok()) }
    fn next_close_byte_demand(&self) -> Option<usize> { self.active.as_ref().map_or_else(|| if self.original.is_empty() { self.original.next_empty_page_release_byte_demand() } else { Some(0) }, |active| active.next_release_byte_demand().ok()) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.active.as_ref().map_or(Ok(usize::from(!self.terminal_is_empty())), |active| active.next_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "history entry depth overflow"))) }
    fn terminal_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(std::mem::size_of::<Self>()) }
}

impl<T: RetireOwned> Drop for HistoryPageRetirement<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "original Store history pages must reach physical terminal ownership");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.original); ManuallyDrop::drop(&mut self.active); } }
    }
}

impl<T: RetireOwned> RetireOwned for crate::os_vcs::HistoryPageStack<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> { Box::new(HistoryPageRetirement { original: ManuallyDrop::new(self), active: ManuallyDrop::new(None) }) }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(std::mem::size_of::<HistoryPageRetirement<T>>()) }
    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() }
}

artifact_retire_struct!(super::ArtifactCursorOwners { applied_edit_ids, redo_edit_ids, checkpoint_id });
artifact_retire_struct!(super::ArtifactCursorGroupRoot { visibility, owners });
artifact_retire_struct!(crate::os_vcs::ArtifactHistoryKey { index, generation });
artifact_retire_struct!(super::CursorRevisionRecord { ledger_key, id_digest, edit_digest, prefix_digest });
artifact_retire_struct!(super::EditDigestChains { forwards, inverse, meta, forwards_digest, inverse_digest, meta_digest });
artifact_retire_struct!(super::CursorRevisionAccumulator { identity_digest, applied, redo, applied_tail_chains, mutation_positions, indexed_edits, unit_flags });
struct GroupVisibilityRetirement {
    original: Option<crate::os_vcs::ArtifactGroupVisibility>,
    remaining: usize,
}

impl RetirementCursor for GroupVisibilityRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        if !self.terminal_is_empty() && grant.maximum_depth == 0 { return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::DepthLimit, "group visibility requires its original owner depth")); }
        if self.remaining != 0 {
            let bytes = self.remaining.min(grant.maximum_copy_bytes);
            if bytes == 0 { return RetirementStep::BudgetExhausted; }
            self.remaining -= bytes;
            return RetirementStep::ProcessedBytes(bytes);
        }
        drop(self.original.take());
        RetirementStep::Complete
    }
    fn terminal_is_empty(&self) -> bool { self.original.is_none() && self.remaining == 0 }
    fn next_work_byte_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(self.remaining != 0)) }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> { Some(0) }
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn terminal_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(std::mem::size_of::<Self>()) }
}

impl RetireOwned for crate::os_vcs::ArtifactGroupVisibility {
    fn retirement(self) -> Box<dyn RetirementCursor> { Box::new(GroupVisibilityRetirement { original: Some(self), remaining: std::mem::size_of::<Self>() }) }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(std::mem::size_of::<GroupVisibilityRetirement>()) }
    fn controlled_retirement_supported() -> bool { true }
    fn retirement_element_copy_bytes() -> usize { std::mem::size_of::<Self>() }
}
impl RetireOwned for super::ArtifactCursor {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let mut original = ManuallyDrop::new(self);
        let owners = unsafe { ManuallyDrop::take(&mut original.owners) };
        let group = unsafe { ManuallyDrop::take(&mut original.group) };
        semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(owners), semio_framework_value::retirement::deferred(group)])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&*self.owners), semio_framework_value::retirement::deferred_birth_bytes_for(&*self.group)]) }
    fn controlled_retirement_supported() -> bool { true }
}
artifact_retire_struct!(super::BlobRef { hash, size, media_type });
artifact_retire_struct!(super::ArtifactLink { target, pin, role });
artifact_retire_struct!(super::OwnerRef { parent, slot, child_id });
impl<S: Send + 'static> RetireOwned for super::ArtifactChild<S> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        semio_framework_value::artifact_retirement_sequence![self.child_id, self.target]
    }
}
impl RetireOwned for super::LinkPin {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::Head => sequence(Vec::new()),
            Self::Checkpoint { id } => id.retirement(),
            Self::Snapshot { blob } => blob.retirement(),
        }
    }
}

artifact_retire_struct!(super::ArtifactBackboneRef { uri });
artifact_retire_struct!(super::MigrationProvenance { document_id, dialect, checkpoint_id, migrated_at });
artifact_retire_struct!(super::OpenToolTransaction { transaction, edit_id });
impl RetireOwned for super::HistoryLane {
    fn retirement(self) -> Box<dyn RetirementCursor> { match self { Self::Document => 0u8, Self::Interaction => 1u8 }.retirement() }
    fn retirement_birth_bytes(&self) -> Option<usize> { 0u8.retirement_birth_bytes() }
    fn controlled_retirement_supported() -> bool { true }
}

semio_framework_value::artifact_retire_leaf!(super::HistoryLane);
