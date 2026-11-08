//! 📥️ Trait impls of `♻️retirement` whose trait or type belongs to `os-store-child` (orphan rule, PLAN-OS-KERNEL-CRATES.md 7.7): lifted out of `🏪️store/♻️retirement/🦀️.rs`.
use semio_framework_value::retirement::RetireOwned;

type ChildOwned = (String, semio_framework_artifact_reference::ArtifactRef, Option<crate::artifact_child_retained_clone::ArtifactChildLocalOwnerAlias>);

impl<S: Send + 'static> RetireOwned for crate::ArtifactChild<S> {
    type RetirementCursor = <ChildOwned as RetireOwned>::RetirementCursor;
    fn retirement_seed(self) -> Self::RetirementCursor { (self.child_id, self.target, self.local_owner.map(crate::artifact_child_retained_clone::ArtifactChildLocalOwnerAlias)).retirement_seed() }
    fn controlled_retirement_supported() -> bool { true }
}
