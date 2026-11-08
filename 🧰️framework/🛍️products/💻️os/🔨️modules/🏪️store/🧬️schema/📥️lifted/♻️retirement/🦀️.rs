//! 📥️ Items of `retirement_integration` that belong to `os-store-space-schema` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/♻️retirement/🦀️.rs`.
use semio_framework_value::artifact_retire_struct;
artifact_retire_struct!(crate::space_history_document::SpaceMemberPin { document_id, checkpoint_id, alternative_id });

artifact_retire_struct!(crate::space_history_document::SpaceCheckpoint { id, parent_id, message, authors, timestamp, members });

artifact_retire_struct!(crate::space_history_document::SpaceAlternative { id, name, checkpoint_ids });

artifact_retire_struct!(crate::space_history_document::SpaceHistorySnapshot { checkpoints, alternatives, active_alternative_id });

artifact_retire_struct!(crate::space_history_mutations::commit_space_checkpoint::CommitSpaceCheckpoint { checkpoint, index });

artifact_retire_struct!(crate::space_history_mutations::create_space_alternative::CreateSpaceAlternative { alternative });

artifact_retire_struct!(crate::space_history_mutations::switch_space_alternative::SwitchSpaceAlternative { alternative_id });

artifact_retire_struct!(crate::space_history_mutations::remove_space_checkpoint::RemoveSpaceCheckpoint { checkpoint_id });

artifact_retire_struct!(crate::space_history_mutations::remove_space_alternative::RemoveSpaceAlternative { alternative_id });

artifact_retire_struct!(crate::space_history_mutations::set_active_space_alternative::SetActiveSpaceAlternative { alternative_id });
