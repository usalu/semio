//! 🏪️ Store-specific retirement implementations over the neutral owner contract.
use semio_framework_value::artifact_retire_struct;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor, sequence};
use std::mem::ManuallyDrop;

artifact_retire_struct!(super::CursorRevisionRecord { ledger_key, id_digest, edit_digest, prefix_digest });
artifact_retire_struct!(super::EditDigestChains { forwards, inverse, meta, forwards_digest, inverse_digest, meta_digest });
artifact_retire_struct!(super::CursorRevisionAccumulator { identity_digest, applied, redo, applied_tail_chains, mutation_positions, indexed_edits, unit_flags });
artifact_retire_struct!(super::BlobRef { hash, size, media_type });
artifact_retire_struct!(super::ArtifactLink { target, pin, role });
impl<S: Send + 'static> RetireOwned for super::ArtifactChild<S> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        sequence(vec![semio_framework_value::retirement::deferred(self.child_id), semio_framework_value::retirement::deferred(self.target)])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.child_id), semio_framework_value::retirement::deferred_birth_bytes_for(&self.target)])
    }
    fn controlled_retirement_supported() -> bool { true }
}

artifact_retire_struct!(super::HistoryColumn { checkpoint_id, timestamp, labels, authors, parent_checkpoint_id, description, lane, alternative_ids });
artifact_retire_struct!(super::SpaceMemberPin { document_id, checkpoint_id, alternative_id });
artifact_retire_struct!(super::SpaceCheckpoint { id, parent_id, message, authors, timestamp, members });
artifact_retire_struct!(super::SpaceAlternative { id, name, checkpoint_ids });
artifact_retire_struct!(super::SpaceHistorySnapshot { checkpoints, alternatives, active_alternative_id });
artifact_retire_struct!(super::CommitSpaceCheckpoint { checkpoint });
artifact_retire_struct!(super::CreateSpaceAlternative { alternative });
artifact_retire_struct!(super::SwitchSpaceAlternative { alternative_id });
artifact_retire_struct!(super::RemoveSpaceCheckpoint { checkpoint_id });
artifact_retire_struct!(super::RemoveSpaceAlternative { alternative_id });
artifact_retire_struct!(super::SetActiveSpaceAlternative { alternative_id });
impl RetireOwned for super::SpaceHistoryMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::CommitSpaceCheckpoint(value) => value.retirement(),
            Self::CreateSpaceAlternative(value) => value.retirement(),
            Self::SwitchSpaceAlternative(value) => value.retirement(),
            Self::RemoveSpaceCheckpoint(value) => value.retirement(),
            Self::RemoveSpaceAlternative(value) => value.retirement(),
            Self::SetActiveSpaceAlternative(value) => value.retirement(),
        }
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        match self {
            Self::CommitSpaceCheckpoint(value) => value.retirement_birth_bytes(),
            Self::CreateSpaceAlternative(value) => value.retirement_birth_bytes(),
            Self::SwitchSpaceAlternative(value) => value.retirement_birth_bytes(),
            Self::RemoveSpaceCheckpoint(value) => value.retirement_birth_bytes(),
            Self::RemoveSpaceAlternative(value) => value.retirement_birth_bytes(),
            Self::SetActiveSpaceAlternative(value) => value.retirement_birth_bytes(),
        }
    }
    fn controlled_retirement_supported() -> bool { true }
}

#[cfg(test)]
#[path="../📬️command/♻️retirement/🧪️tests/🦀️.rs"]
mod command_custody_tests;
