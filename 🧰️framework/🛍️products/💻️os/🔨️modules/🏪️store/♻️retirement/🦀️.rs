//! 🏪️ Store-specific retirement implementations over the neutral owner contract.
use semio_framework_value::retirement::{RetireOwned, RetirementCursor, leaf, sequence};
use semio_framework_value::artifact_retire_struct;
artifact_retire_struct!(crate::os_io::ArtifactDialect { artifact_kind, standard, subset });
artifact_retire_struct!(crate::os_io::ArtifactRef { artifact_id, dialect });
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
