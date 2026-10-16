//! 📥️ Items of `retirement_integration` that belong to `os-store-space-schema` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/♻️retirement/🦀️.rs`.
use semio_framework_value::artifact_retire_struct;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};
artifact_retire_struct!(crate::space_history_document::SpaceMemberPin { document_id, checkpoint_id, alternative_id });

artifact_retire_struct!(crate::space_history_document::SpaceCheckpoint { id, parent_id, message, authors, timestamp, members });

artifact_retire_struct!(crate::space_history_document::SpaceAlternative { id, name, checkpoint_ids });

artifact_retire_struct!(crate::space_history_document::SpaceHistorySnapshot { checkpoints, alternatives, active_alternative_id });

artifact_retire_struct!(crate::space_history_mutations::commit_space_checkpoint::CommitSpaceCheckpoint { checkpoint });

artifact_retire_struct!(crate::space_history_mutations::create_space_alternative::CreateSpaceAlternative { alternative });

artifact_retire_struct!(crate::space_history_mutations::switch_space_alternative::SwitchSpaceAlternative { alternative_id });

artifact_retire_struct!(crate::space_history_mutations::remove_space_checkpoint::RemoveSpaceCheckpoint { checkpoint_id });

artifact_retire_struct!(crate::space_history_mutations::remove_space_alternative::RemoveSpaceAlternative { alternative_id });

artifact_retire_struct!(crate::space_history_mutations::set_active_space_alternative::SetActiveSpaceAlternative { alternative_id });

impl RetireOwned for crate::space_history_mutations::SpaceHistoryMutation {
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
