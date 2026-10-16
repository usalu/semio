//! 📥️ Items of `snapshot_read` that belong to `os-store-replay` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🔗️read/🦀️.rs`.
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::ValueError;
use semio_framework_os_store_ledger::lifted_replay::EffectiveSupersessions;
use semio_framework_value_snapshot_read::*;
use semio_framework_os_store_ledger::lifted_snapshot_read::ArtifactStoreStringRetirement;
/// 🧳️ Retains the original supersession index under independent item, capacity, release and depth grants.
pub struct ArtifactStoreSupersessionRetirement {
    pub owner: semio_framework_value::retirement::controlled::ControlledRetirement<EffectiveSupersessions>,
}

impl ArtifactStoreSupersessionRetirement {
    pub(crate) fn new(supersessions: EffectiveSupersessions) -> Self {
        let owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(supersessions).unwrap_or_else(|_|unreachable!("supersession index declares controlled retirement"));
        Self { owner }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreSupersessionRetirement {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.owner.step(grant)}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(copy)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}
    fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}
    fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
}

pub struct ArtifactStoreStringVectorRetirement {
    pub values: std::mem::ManuallyDrop<Option<semio_framework_os_vcs::HistoryPageStack<String>>>,
    pub active: std::mem::ManuallyDrop<Option<ArtifactStoreStringRetirement>>,
}

impl ArtifactStoreStringVectorRetirement {
    pub(crate) fn new(values: impl Into<semio_framework_os_vcs::HistoryPageStack<String>>) -> Self {
        Self { values: std::mem::ManuallyDrop::new(Some(values.into())), active: std::mem::ManuallyDrop::new(None) }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreStringVectorRetirement {
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {
        self.active.as_ref().map_or(Ok(0), ErasedSnapshotRetirement::next_copy_byte_demand)
    }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> {
        self.active.as_ref().map_or(Ok(0), |active| active.next_capacity_byte_demand(copy))
    }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        if let Some(active) = self.active.as_ref() { return active.next_release_byte_demand(); }
        self.values.as_ref().filter(|values| values.is_empty()).map_or(Ok(0), |values| values.next_empty_page_release_byte_demand().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "string history backing extent is unavailable")))
    }
    fn next_depth_demand(&self) -> Result<usize, ValueError> {
        if let Some(active) = self.active.as_ref() {
            return active.next_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "string history retirement depth overflow"));
        }
        Ok(usize::from(self.values.is_some()))
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_close};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth < self.next_depth_demand()? { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "string history retirement exceeds admitted depth")); }
        if let Some(active) = self.active.as_mut() {
            if active.terminal_is_empty() {
                drop(self.active.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            let child_grant = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = active.close_step(child_grant)?;
            admit_retained_clone_close(child_grant, step, active.terminal_is_empty(), "Store string history child")?;
            return Ok(match step { RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress) => RetainedCloneStep::Progress(progress) });
        }
        let values = self.values.as_mut().expect("live string history owner");
        if let Some(value) = values.pop() {
            *self.active = Some(ArtifactStoreStringRetirement::new(value));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        let release = values.next_empty_page_release_byte_demand().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "string history retains owners in an empty page"))?;
        if release != 0 {
            if grant.maximum_release_bytes < release { return Ok(RetainedCloneStep::Progress(empty)); }
            if !values.release_empty_page() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "string history page changed after exact release demand")); }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: release, ..empty }));
        }
        drop(self.values.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.values.is_none() && self.active.is_none()
    }
}

impl Drop for ArtifactStoreStringVectorRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "artifact store string-vector retirement reached Drop before its exact terminal-empty witness");
    }
}

pub const ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS: usize = 1 << 20;
