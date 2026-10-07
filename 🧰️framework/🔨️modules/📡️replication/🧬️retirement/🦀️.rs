//! 📡️ Replication-owned delta retirement implements the neutral contract.
use semio_framework_value::retirement::{RetireOwned, RetirementCursor, sequence, leaf};
impl<V: RetireOwned> RetireOwned for crate::MapDelta<V> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        self.into_entries().retirement()
    }
}
impl<V: RetireOwned> RetireOwned for crate::MapEntryDelta<V> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let (precondition, operation) = self.into_parts();
        sequence(vec![leaf(precondition), operation.retirement()])
    }
}
impl<V: RetireOwned> RetireOwned for crate::MapEntryOperation<V> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            crate::MapEntryOperation::Set(value) => sequence(vec![leaf(0u8), value.retirement()]),
            crate::MapEntryOperation::Remove => leaf(1u8),
            crate::MapEntryOperation::Reject => leaf(2u8),
        }
    }
}

semio_framework_value::artifact_retire_struct!(crate::TransactionRef { id, tool });
semio_framework_value::artifact_retire_struct!(crate::HistoryFold { applied, redo, refused, checkpoint, alternative, trunk, changes, checkpoints, alternatives, supersessions });
semio_framework_value::artifact_retire_struct!(crate::EffectiveSupersession { transition_id, actor, timestamp, scope, replacement });
semio_framework_value::artifact_retire_leaf!(crate::HybridLogicalTimestamp);
semio_framework_value::artifact_retire_struct!(crate::FoldChange { id, edit_ids, description, saved_at });
semio_framework_value::artifact_retire_struct!(crate::FoldCheckpoint { id, change_ids, parent_id, authors, message, timestamp, pins });
semio_framework_value::artifact_retire_struct!(crate::FoldAlternative { id, name, checkpoint_ids });
semio_framework_value::artifact_retire_struct!(crate::TransitionAuthor { id, name, avatar });
semio_framework_value::artifact_retire_struct!(crate::TransitionPin { child_uri, checkpoint_id });
semio_framework_value::artifact_retire_struct!(crate::MutationMessage { level, code, message, target, op_index });
semio_framework_value::artifact_retire_struct!(crate::EditMessages { edit_id, messages });
impl semio_framework_value::retirement::RetireOwned for crate::MutationId {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.0)
    }
}
impl semio_framework_value::retirement::RetireOwned for crate::InputReplacement {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::Input { schema, payload } => semio_framework_value::artifact_retirement_sequence![schema, payload],
            Self::Withdrawn => semio_framework_value::retirement::sequence(Vec::new()),
        }
    }
}
impl semio_framework_value::retirement::RetireOwned for crate::MutationOrigin {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::Owner => semio_framework_value::retirement::sequence(Vec::new()),
            Self::Contributed { plugin_id, mutation_id, payload_hash } => semio_framework_value::artifact_retirement_sequence![plugin_id, mutation_id.0, payload_hash.0],
            Self::Transaction { initiator } => semio_framework_value::artifact_retirement_sequence![initiator.artifact_id, initiator.artifact_kind, initiator.dialect],
        }
    }
}

semio_framework_value::artifact_retire_struct!(crate::FoldEdit { id, actor, timestamp, mutation_ids, line });
semio_framework_value::artifact_retire_struct!(crate::TransitionCheckpoint { checkpoint_id, parent_id, change_id, mutation_ids, description, saved_at, authors, message, timestamp, line_id });
semio_framework_value::artifact_retire_struct!(crate::TransitionSupersede { scope, inputs });
semio_framework_value::artifact_retire_struct!(crate::SupersededInput { target, replacement });
impl semio_framework_value::retirement::RetireOwned for crate::HistoryTransition {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        use semio_framework_value::retirement::RetireOwned;
        match self {
            Self::Revert { mutation_ids } | Self::Reinstate { mutation_ids } => mutation_ids.retirement(),
            Self::Commit(checkpoint) => checkpoint.retirement(),
            Self::Branch { alternative_id, name, checkpoint_id } => semio_framework_value::artifact_retirement_sequence![alternative_id, name, checkpoint_id],
            Self::Checkout { checkpoint_id, alternative_id } => semio_framework_value::artifact_retirement_sequence![checkpoint_id, alternative_id],
            Self::Repin { checkpoint_id, pinned_checkpoint_id, pins } => semio_framework_value::artifact_retirement_sequence![checkpoint_id, pinned_checkpoint_id, pins],
            Self::Supersede(supersede) => supersede.retirement(),
        }
    }
}

semio_framework_value::artifact_retire_struct!(crate::MutationEnvelope { mutation_id, document_id, actor, dependencies, observed, target, diff, inverse, timestamp, transaction, verb, line });
semio_framework_value::artifact_retire_struct!(crate::ArtifactDiff { schema, payload });
semio_framework_value::artifact_retire_struct!(crate::InverseMutation { schema, payload });
impl semio_framework_value::retirement::RetireOwned for crate::ActorId { fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { semio_framework_value::retirement::RetireOwned::retirement(self.0) } }
impl semio_framework_value::retirement::RetireOwned for crate::ArtifactId { fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { semio_framework_value::retirement::RetireOwned::retirement(self.0) } }
impl semio_framework_value::retirement::RetireOwned for crate::SchemaId { fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { semio_framework_value::retirement::RetireOwned::retirement(self.0) } }

semio_framework_value::artifact_retire_struct!(crate::ViewerHead { line_id, checkpoint_id });

semio_framework_value::artifact_retire_struct!(crate::Conflict { id, kind, status, messages, actors, timestamp });
semio_framework_value::artifact_retire_leaf!(crate::ConflictStatus);
impl RetireOwned for crate::ConflictId { fn retirement(self) -> Box<dyn RetirementCursor> { self.0.retirement() } }
impl RetireOwned for crate::ConflictKind {
    fn retirement(self) -> Box<dyn RetirementCursor> { match self { Self::Quarantined { envelopes } => envelopes.retirement(), Self::Degraded { edit_ids } => edit_ids.retirement() } }
}
