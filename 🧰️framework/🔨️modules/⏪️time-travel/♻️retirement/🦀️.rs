//! ⏪️ Native ephemeral session fields retire through their original owned-value authorities.
semio_framework_value::artifact_retire_leaf!(crate::TimeTravelBase, crate::TimeTravelStage, crate::TimeTravelProgress);
semio_framework_value::artifact_retire_struct!(crate::TimeTravelTarget { mutation, position });
semio_framework_value::artifact_retire_struct!(crate::TimeTravelDraft { target, replacement });
semio_framework_value::artifact_retire_struct!(crate::TimeTravelPending { target, original, replacement, return_stage });
semio_framework_value::artifact_retire_struct!(crate::TimeTravelSession { id, generation, base, stage, accepted, pending, report, progress, fault });
macro_rules! native_event_retirement {
    ($type:ty { $($variant:ident { $($field:ident),+ }),+ $(,)? }; $($unit:ident),+ $(,)?) => {
        impl semio_framework_value::retirement::RetireOwned for $type {
            fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
                use semio_framework_value::retirement::{deferred,sequence};
                match self { $(Self::$variant { $($field),+ } => sequence(vec![$(deferred($field)),+]),)+ $(Self::$unit => sequence(Vec::new()),)+ }
            }
            fn retirement_birth_bytes(&self) -> Option<usize> {
                use semio_framework_value::retirement::{deferred_birth_bytes_for,sequence_birth_bytes};
                match self { $(Self::$variant { $($field),+ } => sequence_birth_bytes(&[$(deferred_birth_bytes_for($field)),+]),)+ $(Self::$unit => sequence_birth_bytes(&[]),)+ }
            }
            fn controlled_retirement_supported() -> bool { true }
        }
    };
}
native_event_retirement!(crate::TimeTravelChoice { Alternative { name } }; Overwrite);
native_event_retirement!(crate::TimeTravelEvent {
    Begin { target, original }, BeginWithdrawn { target, original }, Draft { generation, replacement },
    Withdraw { generation }, Accept { generation }, Discard { generation }, Restore { generation, target },
    ReplayProgressed { generation, done, total }, ReplayCompleted { generation, report }, ReplayCancelled { generation },
    ReplayFaulted { generation, code }, Rerun { generation }, RequestFinalize { generation }, Choose { generation, choice },
    Back { generation }, Finalized { generation }, FinalizeFaulted { generation, code }, BaseMoved { base, positions }
}; Exit);
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
