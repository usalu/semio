//! 📥️ Items of `operation` that belong to `os-store-engine-owners` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`.
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_os_store_member_open::{MemberOpenAdmissionError, MemberOpenDiagnostic, MemberOpenProgress, MemberOpenRequest};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_job::{StepContext,OriginalTurnFailure};
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
    /// 🎟️ Concrete constructor and returned owner header moves precede any decoder work.
    fn begin_copy_byte_demand(request: &MemberOpenRequest) -> Result<usize, MemberOpenDiagnostic> where Self:Sized { request.admitted_expected().map(|_|std::mem::size_of::<Self>().saturating_mul(2)) }
    fn begin(request: MemberOpenRequest) -> Result<Self, MemberOpenAdmissionError>
    where
        Self: Sized;
    fn step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<MemberSnapshotOpenStep,OriginalTurnFailure>;
    fn take_ready(&mut self, cx: &mut StepContext<'_>) -> Result<Option<(Self::Snapshot, MemberOpenRequest)>,OriginalTurnFailure>;
}
