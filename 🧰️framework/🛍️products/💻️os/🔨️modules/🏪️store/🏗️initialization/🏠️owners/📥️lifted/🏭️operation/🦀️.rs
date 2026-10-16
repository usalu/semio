//! 📥️ Items of `operation` that belong to `os-store-engine-owners` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`.
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_os_store_member_open::{MemberOpenAdmissionError, MemberOpenDiagnostic, MemberOpenProgress, MemberOpenRequest};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_job::StepContext;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemberSnapshotOpenProgress {
    pub opening: MemberOpenProgress,
    pub retained_progress: RetainedCloneProgress,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberSnapshotOpenStep {
    Pending(MemberSnapshotOpenProgress),
    Ready,
    Rejected(MemberOpenDiagnostic),
}

pub trait MemberSnapshotOpenOperation: ErasedSnapshotRetirement {
    type Snapshot;
    fn begin_birth_bytes(request: &MemberOpenRequest) -> Result<usize, MemberOpenDiagnostic>;
    fn begin(request: MemberOpenRequest) -> Result<Self, MemberOpenAdmissionError>
    where
        Self: Sized;
    fn step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> MemberSnapshotOpenStep;
    fn take_ready(&mut self, cx: &mut StepContext<'_>) -> Option<(Self::Snapshot, MemberOpenRequest)>;
}
