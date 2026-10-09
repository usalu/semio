//! 📋️ Original replay reports retain every outcome identity, diagnostic and unused vector backing.
semio_framework_value::artifact_retire_struct!(crate::MutationReplayOutcome { mutation_id, edit_id, op_index, worst, messages, superseded, withdrawn });
semio_framework_value::artifact_retire_struct!(crate::ReplayReport { from_position, outcomes, worst });
