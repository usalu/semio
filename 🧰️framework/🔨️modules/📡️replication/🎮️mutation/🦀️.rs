//! 🎞️ Mutation contract: the `Mutation`/`MutationDiff` trait family, per-operation
//! diagnostics (`MutationMessage`/`MutationOutcome`), and semantic transformation laws.
//! Product-neutral — both the optimistic client replica and the authority
//! rerun the same deciders against these types. Op payloads stay schema-opaque: this module
//! never parses an `Op`, it only threads it through the trait seams a technology implements.
//!
//! Frozen contract: `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md`
//! `## Amendment` §`protocol_command`.

/// 🗂️ Shared keyed-map mutation algebra and wire contract.
#[path = "🗂️map/🦀️.rs"]
pub mod map;
pub use map::{MapDelta, MapDeltaTarget, MapEntryDelta, MapEntryOperation, MapPresence};

#[path="❄️reference/🦀️.rs"]
mod cold_reference;
pub use cold_reference::ColdMutation;
/// 📏️ Portable visible diagnostic envelope independent of machine pointer width.
pub const MUTATION_MESSAGE_ENTRY_BYTES: usize = 4096;
/// 🧳️ Normative owned diagnostic row charge shared by bounded history ledgers.
pub const MUTATION_MESSAGE_OWNER_BYTES: usize = 96;
/// 🧭️ Every target segment consumes normative owner credit, including empty text.
pub const MUTATION_MESSAGE_TARGET_BYTES: usize = 24;
//#region 🔖️Mutation
pub use semio_framework_mutation_diff::{MutationApplyError,MutationApplyResult,ApplyCapability,apply_diff_cold,apply_diff_cold_into,PreparedField,MutationDiff,DiffAlgebra,ColdMutationDiff,ColdDiffAlgebra,with_retained_apply_capability};

/// 🔁️ Stored operation metadata and native delta vocabulary; retained preparation owns decision, diagnostics and inverse production.
pub trait Mutation<P>: Sized + Sync + semio_framework_value::ToValue + semio_framework_value::FromValue {
    type Diff: MutationDiff<P>;
    /// 🧷️ Every direct mutation leaf's static metadata, in aggregate-variant order.
    const DESCRIPTORS: &'static [MutationLeafDescriptor];

    /// 🧷️ This value's own direct mutation leaf descriptor.
    fn descriptor(&self) -> &'static MutationLeafDescriptor;

    /// 🧊️ Explicit cold disposal of one owned operation, for the same reason [`MutationDiff::retire_cold`]
    /// exists: a row that carries a domain value with a fail-closed root cannot be dropped.
    fn retire_cold(self)
    where
        Self: Sized,
    {
    }

    fn mutation_id(&self) -> Option<semio_framework_replication_ids::MutationId> {
        None
    }
    fn dependencies(&self) -> Vec<semio_framework_replication_ids::MutationId> {
        Vec::new()
    }
    /// 🎯️ The structured address this operation writes (outermost segment first; empty: the
    /// whole artifact), carried as `MutationEnvelope::target` so the hub can tell concurrent writes
    /// to disjoint parts of a document it cannot read from writes to the same part.
    /// `#[derive(Mutations)]` forwards each leaf's declared `MutationKind::target`.
    fn conflict_target(&self) -> Vec<String> {
        Vec::new()
    }
    fn base_version(&self) -> Option<semio_framework_replication_ids::ArtifactVersion> {
        None
    }
    fn author_id(&self) -> Option<semio_framework_replication_ids::ActorId> {
        None
    }
    fn timestamp(&self) -> Option<semio_framework_replication_ids::HybridLogicalTimestamp> {
        None
    }
    fn undo_policy(&self) -> semio_framework_replication_wire_core::UndoPolicy {
        semio_framework_replication_wire_core::UndoPolicy::ExactBaseOnly
    }
    /// 🗂️ Which durability/visibility class this operation's diffs belong to.
    fn state_class(&self) -> semio_framework_schema_state::StateClass {
        semio_framework_schema_state::StateClass::Artifact
    }
    /// 🌐️ Conservative base-independent foreign-step capability used to avoid replaying
    /// ordinary local operations solely for transaction-proposal discovery.
    fn may_emit_foreign_steps(&self) -> bool {
        true
    }
    /// 🧾️ The upper bound of the rows [`ColdMutation::inverse`] yields for this operation on any base: what a one-item Store preparation
    /// declares as its fold footprint (`ArtifactStoreOneItemFootprint::for_leaf`). `#[derive(Mutations)]` answers the wrapped
    /// leaf's schema-declared [`MutationLeaf::inverse_rows`] (design §20.5); a hand-written aggregate without leaf schemas is
    /// point-invertible.
    fn inverse_rows(&self) -> usize {
        1
    }
    /// 🌐️ Borrows one original foreign output; discovery never constructs a plan or payload.
    fn foreign_step_source<'a>(&'a self, _base: &'a P, _index: usize) -> Result<Option<ForeignStepSource<'a>>, semio_framework_value::ValueError> {
        Ok(None)
    }
    /// 🧬️ Per [`Self::DESCRIPTORS`] row, that leaf's payload JSON Schema (`MutationLeaf::PAYLOAD_SCHEMA`) — the static
    /// roster of editable inputs a plugin publishes without an operation at hand. `#[derive(Mutations)]` fills it; a
    /// hand-written projection bridge forwards its source aggregate's.
    const INPUT_SCHEMAS: &'static [&'static str] = &[];
    /// 🔗️ Per [`Self::INPUT_SCHEMAS`] row, the documents that leaf's payload schema references (`MutationLeaf::PAYLOAD_SCHEMA_DOCUMENTS`)
    /// — what the runtime publishes beside the schemas so every `$ref` of every editable input resolves. Row-aligned like
    /// [`Self::INPUT_SCHEMAS`]; `#[derive(Mutations)]` fills it, a hand-written bridge forwards its source aggregate's.
    const INPUT_SCHEMA_DOCUMENTS: &'static [&'static [&'static str]] = &[];
    /// 🧬️ The payload JSON Schema of this operation's own leaf, or `None` when its inputs are not editable.
    /// `#[derive(Mutations)]` answers the wrapped leaf's `MutationLeaf::PAYLOAD_SCHEMA`.
    fn input_schema(&self) -> Option<&'static str> {
        None
    }
    /// 🧬️ The value [`Self::input_schema`] describes: the leaf payload without any aggregate tag or wrapper, so a
    /// generic editor never depends on the aggregate's wire layout. The default is the whole value.
    fn payload_value(&self) -> semio_framework_value::DslValue {
        self.to_value()
    }
    /// 🧬️ Rebuilds this operation's own kind from an edited [`Self::payload_value`]; a value that does not decode as
    /// that leaf is refused, never reinterpreted as another kind. The default decodes the whole value.
    fn with_payload_value(&self, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError>
    where
        Self: Sized,
    {
        Self::from_value(value)
    }
    /// 🧬️ Builds the operation of semantic kind `kind` (its leaf descriptor's `semantic_kind`) from its editable payload
    /// ([`Self::payload_value`]) alone — the constructor a wire witness such as a `🥒️.feature` row decodes through.
    /// `#[derive(Mutations)]` answers every leaf kind of the aggregate; the default refuses.
    fn from_payload_value(kind: &str, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError>
    where
        Self: Sized,
    {
        let _ = value;
        Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!("{kind} has no payload constructor")))
    }
}
//#endregion 🔖️Mutation

#[path = "📋️mutation-metadata-enums/🦀️.rs"]
mod mutation_metadata_enums;
pub use mutation_metadata_enums::*;

#[path = "🪪️leaf-descriptor-validation/🦀️.rs"]
mod leaf_descriptor_validation;
pub use leaf_descriptor_validation::*;

#[path = "🔎️leaf-source-scope/🦀️.rs"]
mod leaf_source_scope;
pub use leaf_source_scope::*;

#[path = "🚦️outcome-and-message/🦀️.rs"]
mod outcome_and_message;
pub use outcome_and_message::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-leaf-metadata/🦀️.rs"]
mod mutation_leaf_metadata_tests;







//#region 🔖️Foreign
#[path = "🌐️foreign/🦀️.rs"]
mod foreign_output;
pub use foreign_output::{ForeignTarget,ForeignStep,ForeignStepSource,ForeignStepCopy,ForeignStepRetirement,ForeignStepsOwner,ForeignStepsPreparation};
//#endregion 🔖️Foreign





/// 🎯️ The one store step from an operation to its next projection: computes the op-stamped diff, applies it through
/// [`protocol::apply_diff`] and retires the diff cold. Every store lane, replay, merge and backbone fold routes through
/// here; the apply refusal stays structured for callers that must fail instead of record.
pub fn apply_operation<P, Mutation>(state: &P, operation: &Mutation, op_index: u32) -> (semio_framework_mutation_diff::MutationApplyResult<P>, Vec<crate::MutationMessage>)
where
    P:Clone,
    Mutation: self::Mutation<P>+ColdMutation<P>,
{
    let (diff, messages) = operation.diff(state).stamp_op_index(op_index).into_parts();
    let applied = semio_framework_mutation_diff::apply_diff_cold(&diff, state);
    <Mutation::ReferenceDiff as MutationDiff<P>>::retire_cold(diff);
    (applied, messages)
}






#[cfg(test)]
#[path = "🧪️tests/🧪️outcome-code/🦀️.rs"]
mod outcome_code_tests;
#[cfg(test)]
#[path = "🧪️tests/🧳️native-contract/🧪️tests/🦀️.rs"]
mod native_diff_contract_tests;
#[cfg(test)]
#[path = "🧪️tests/📦️unpublished-owner/🦀️.rs"]
mod unpublished_owner_tests;

mod owned_retirement {
    use super::*;
    semio_framework_value::artifact_retire_struct!(MutationMessage { level, code, message, target, op_index });
}

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;
