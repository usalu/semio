//! 📥️ Trait impls of `♻️retirement` whose trait or type belongs to `os-store-child` (orphan rule, PLAN-OS-KERNEL-CRATES.md 7.7): lifted out of `🏪️store/♻️retirement/🦀️.rs`.
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::retirement::RetirementCursor;
use semio_framework_value::retirement::sequence;

impl<S: Send + 'static> RetireOwned for crate::ArtifactChild<S> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        sequence(vec![semio_framework_value::retirement::deferred(self.child_id), semio_framework_value::retirement::deferred(self.target)])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.child_id), semio_framework_value::retirement::deferred_birth_bytes_for(&self.target)])
    }
    fn controlled_retirement_supported() -> bool { true }
}
