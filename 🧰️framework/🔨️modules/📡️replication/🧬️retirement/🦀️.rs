//! 📡️ Replication-owned delta retirement implements the neutral contract.
use semio_framework_value::retirement::{RetireOwned, RetirementCursor, sequence, leaf, deferred, deferred_birth_bytes_for, sequence_birth_bytes};
impl<V: RetireOwned> RetireOwned for crate::MapDelta<V> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        self.into_entries().retirement()
    }
}
impl<V: RetireOwned> RetireOwned for crate::MapEntryDelta<V> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let (precondition, operation) = self.into_parts();
        sequence(vec![deferred(precondition), deferred(operation)])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { sequence_birth_bytes(&[deferred_birth_bytes_for(&self.precondition()), deferred_birth_bytes_for(self.operation())]) }
    fn controlled_retirement_supported() -> bool { V::controlled_retirement_supported() }
}
impl<V: RetireOwned> RetireOwned for crate::MapEntryOperation<V> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            crate::MapEntryOperation::Set(value) => sequence(vec![deferred(0u8), deferred(value)]),
            crate::MapEntryOperation::Remove => leaf(1u8),
            crate::MapEntryOperation::Reject => leaf(2u8),
        }
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { match self { Self::Set(value) => sequence_birth_bytes(&[deferred_birth_bytes_for(&0u8), deferred_birth_bytes_for(value)]), Self::Remove | Self::Reject => Some(semio_framework_value::retirement::leaf_birth_bytes::<u8>()) } }
    fn controlled_retirement_supported() -> bool { V::controlled_retirement_supported() }
}

semio_framework_value::artifact_retire_leaf!(crate::MapPresence);
semio_framework_value::artifact_retire_leaf!(crate::MergePolicy, crate::ConflictResolution);
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
impl RetireOwned for crate::MutationId {
    fn retirement(self)->Box<dyn RetirementCursor>{self.0.retirement()}
    fn retirement_birth_bytes(&self)->Option<usize>{self.0.retirement_birth_bytes()}
    fn controlled_retirement_supported()->bool{String::controlled_retirement_supported()}
}
impl semio_framework_value::retirement::RetireOwned for crate::InputReplacement {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::Input { schema, payload } => sequence(vec![deferred(schema), deferred(payload)]),
            Self::Withdrawn => semio_framework_value::retirement::sequence(Vec::new()),
        }
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { match self { Self::Input { schema, payload } => sequence_birth_bytes(&[deferred_birth_bytes_for(schema), deferred_birth_bytes_for(payload)]), Self::Withdrawn => sequence_birth_bytes(&[]) } }
    fn controlled_retirement_supported() -> bool { true }
}
impl semio_framework_value::retirement::RetireOwned for crate::MutationOrigin {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::Owner => semio_framework_value::retirement::sequence(Vec::new()),
            Self::Contributed { plugin_id, mutation_id, payload_hash } => sequence(vec![deferred(plugin_id), deferred(mutation_id.0), deferred(payload_hash.0)]),
            Self::Transaction { initiator } => sequence(vec![deferred(initiator.artifact_id), deferred(initiator.artifact_kind), deferred(initiator.dialect)]),
        }
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { match self { Self::Owner => sequence_birth_bytes(&[]), Self::Contributed { plugin_id, mutation_id, payload_hash } => sequence_birth_bytes(&[deferred_birth_bytes_for(plugin_id), deferred_birth_bytes_for(&mutation_id.0), deferred_birth_bytes_for(&payload_hash.0)]), Self::Transaction { initiator } => sequence_birth_bytes(&[deferred_birth_bytes_for(&initiator.artifact_id), deferred_birth_bytes_for(&initiator.artifact_kind), deferred_birth_bytes_for(&initiator.dialect)]) } }
    fn controlled_retirement_supported() -> bool { true }
}

semio_framework_value::artifact_retire_struct!(crate::FoldEdit { id, actor, timestamp, mutation_ids, line });
semio_framework_value::artifact_retire_struct!(crate::TransitionCheckpoint { checkpoint_id, parent_id, change_id, mutation_ids, description, saved_at, authors, message, timestamp, line_id });
semio_framework_value::artifact_retire_struct!(crate::TransitionSupersede { scope, inputs });
semio_framework_value::artifact_retire_struct!(crate::SupersededInput { target, replacement });
impl RetireOwned for crate::HistoryTransition {
    fn retirement(self)->Box<dyn RetirementCursor>{
        match self{
            Self::Revert{mutation_ids}|Self::Reinstate{mutation_ids}=>mutation_ids.retirement(),
            Self::Commit(checkpoint)=>checkpoint.retirement(),
            Self::Branch{alternative_id,name,checkpoint_id}=>sequence(vec![deferred(alternative_id),deferred(name),deferred(checkpoint_id)]),
            Self::Checkout{checkpoint_id,alternative_id}=>sequence(vec![deferred(checkpoint_id),deferred(alternative_id)]),
            Self::Repin{checkpoint_id,pinned_checkpoint_id,pins}=>sequence(vec![deferred(checkpoint_id),deferred(pinned_checkpoint_id),deferred(pins)]),
            Self::Supersede(supersede)=>supersede.retirement(),
        }
    }
    fn retirement_birth_bytes(&self)->Option<usize>{
        match self{
            Self::Revert{mutation_ids}|Self::Reinstate{mutation_ids}=>mutation_ids.retirement_birth_bytes(),
            Self::Commit(checkpoint)=>checkpoint.retirement_birth_bytes(),
            Self::Branch{alternative_id,name,checkpoint_id}=>sequence_birth_bytes(&[deferred_birth_bytes_for(alternative_id),deferred_birth_bytes_for(name),deferred_birth_bytes_for(checkpoint_id)]),
            Self::Checkout{checkpoint_id,alternative_id}=>sequence_birth_bytes(&[deferred_birth_bytes_for(checkpoint_id),deferred_birth_bytes_for(alternative_id)]),
            Self::Repin{checkpoint_id,pinned_checkpoint_id,pins}=>sequence_birth_bytes(&[deferred_birth_bytes_for(checkpoint_id),deferred_birth_bytes_for(pinned_checkpoint_id),deferred_birth_bytes_for(pins)]),
            Self::Supersede(supersede)=>supersede.retirement_birth_bytes(),
        }
    }
    fn controlled_retirement_supported()->bool{true}
}

semio_framework_value::artifact_retire_struct!(crate::MutationEnvelope { mutation_id, document_id, actor, dependencies, observed, target, diff, inverse, timestamp, transaction, verb, line });
semio_framework_value::artifact_retire_struct!(crate::ArtifactDiff { schema, payload });
semio_framework_value::artifact_retire_struct!(crate::InverseMutation { schema, payload });
impl RetireOwned for crate::ActorId { fn retirement(self)->Box<dyn RetirementCursor>{self.0.retirement()} fn retirement_birth_bytes(&self)->Option<usize>{self.0.retirement_birth_bytes()} fn controlled_retirement_supported()->bool{semio_framework_value::SharedUtf8::controlled_retirement_supported()} }
impl RetireOwned for crate::ArtifactId { fn retirement(self)->Box<dyn RetirementCursor>{self.0.retirement()} fn retirement_birth_bytes(&self)->Option<usize>{self.0.retirement_birth_bytes()} fn controlled_retirement_supported()->bool{String::controlled_retirement_supported()} }
impl RetireOwned for crate::SchemaId { fn retirement(self)->Box<dyn RetirementCursor>{self.0.retirement()} fn retirement_birth_bytes(&self)->Option<usize>{self.0.retirement_birth_bytes()} fn controlled_retirement_supported()->bool{String::controlled_retirement_supported()} }

semio_framework_value::artifact_retire_struct!(crate::ViewerHead { line_id, checkpoint_id });

semio_framework_value::artifact_retire_struct!(crate::Conflict { id, kind, status, messages, actors, timestamp });
semio_framework_value::artifact_retire_leaf!(crate::ConflictStatus);
impl RetireOwned for crate::ConflictId { fn retirement(self) -> Box<dyn RetirementCursor> { self.0.retirement() } fn retirement_birth_bytes(&self) -> Option<usize> { self.0.retirement_birth_bytes() } fn controlled_retirement_supported() -> bool { String::controlled_retirement_supported() } }
impl RetireOwned for crate::ConflictKind {
    fn retirement(self) -> Box<dyn RetirementCursor> { match self { Self::Quarantined { envelopes } => envelopes.retirement(), Self::Degraded { edit_ids } => edit_ids.retirement() } }
    fn retirement_birth_bytes(&self) -> Option<usize> { match self { Self::Quarantined { envelopes } => envelopes.retirement_birth_bytes(), Self::Degraded { edit_ids } => edit_ids.retirement_birth_bytes() } }
    fn controlled_retirement_supported() -> bool { true }
}
