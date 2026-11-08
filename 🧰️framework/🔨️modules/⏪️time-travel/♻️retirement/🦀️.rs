//! ⏪️ Native ephemeral session fields retire through their original owned-value authorities.
semio_framework_value::artifact_retire_leaf!(crate::TimeTravelBase, crate::TimeTravelStage, crate::TimeTravelProgress);
semio_framework_value::artifact_retire_struct!(crate::TimeTravelTarget { mutation, position });
semio_framework_value::artifact_retire_struct!(crate::TimeTravelDraft { target, replacement });
semio_framework_value::artifact_retire_struct!(crate::TimeTravelPending { target, original, replacement, return_stage });
semio_framework_value::artifact_retire_struct!(crate::TimeTravelSession { id, generation, base, stage, accepted, pending, report, progress, fault });
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
