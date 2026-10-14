//! ♻ Command-owned original retirement.
use crate::{ArtifactCommand, HistoryLane, OpenToolTransaction, SupersedeInput};
use semio_framework_value::artifact_retire_struct;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor, sequence};
artifact_retire_struct!(OpenToolTransaction { transaction, edit_id });
semio_framework_value::artifact_retire_leaf!(HistoryLane);

impl<Mutation: RetireOwned> RetireOwned for SupersedeInput<Mutation> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        sequence(vec![semio_framework_value::retirement::deferred(self.target), semio_framework_value::retirement::deferred(self.replacement)])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.target), semio_framework_value::retirement::deferred_birth_bytes_for(&self.replacement)])
    }
    fn controlled_retirement_supported() -> bool { Mutation::controlled_retirement_supported() }
}

macro_rules! retire_command_fields {
    ($($variant:ident { $($field:ident),* }),* $(,)?) => {
        impl<Mutation: RetireOwned> RetireOwned for ArtifactCommand<Mutation> {
            fn retirement(self) -> Box<dyn RetirementCursor> {
                match self {
                    $(Self::$variant { $($field),* } => sequence(vec![$(semio_framework_value::retirement::deferred($field)),*]),)*
                    Self::Undo | Self::Redo | Self::PruneDrafts => sequence(Vec::new()),
                }
            }
            fn retirement_birth_bytes(&self) -> Option<usize> {
                match self {
                    $(Self::$variant { $($field),* } => semio_framework_value::retirement::sequence_birth_bytes(&[$(semio_framework_value::retirement::deferred_birth_bytes_for($field)),*]),)*
                    Self::Undo | Self::Redo | Self::PruneDrafts => semio_framework_value::retirement::sequence_birth_bytes(&[]),
                }
            }
            fn controlled_retirement_supported() -> bool { Mutation::controlled_retirement_supported() }
        }
    };
}

retire_command_fields!(
    Apply { mutations, transaction },
    UndoWithPolicy { policy, semantic_command },
    ApplyInLane { mutations, lane, transaction },
    UndoInLane { lane },
    RedoInLane { lane },
    CommitCheckpoint { message, authors },
    CreateAlternative { name },
    SwitchAlternative { alternative_id },
    CheckoutCheckpoint { checkpoint_id },
    IngestRemote { envelope },
    SetMergePolicy { policy },
    ResolveConflict { conflict_id, resolution },
    Supersede { scope, inputs },
    CreateAlternativeWithSupersede { name, inputs },
    AppendTransaction { mutations, transaction },
    CommitTransaction { transaction_id },
    AbortTransaction { transaction_id },
);

