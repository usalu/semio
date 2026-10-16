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

/// 📏️ Portable visible diagnostic envelope independent of machine pointer width.
pub const MUTATION_MESSAGE_ENTRY_BYTES: usize = 4096;
/// 🧳️ Normative owned diagnostic row charge shared by bounded history ledgers.
pub const MUTATION_MESSAGE_OWNER_BYTES: usize = 96;
/// 🧭️ Every target segment consumes normative owner credit, including empty text.
pub const MUTATION_MESSAGE_TARGET_BYTES: usize = 24;

//#region 🔖️Mutation
pub use semio_framework_mutation_diff::{MutationApplyError,MutationApplyResult,ApplyCapability,apply_diff,MutationDiff,DiffAlgebra};

/// 🔁️ Stored operation: emits a [`MutationOutcome`] (diff plus messages) and computes inverse
/// from pre-state. Moved from `os_store::Mutation` verbatim except: `mutation_id`/
/// `dependencies`/`author_id` now return the `protocol_core` id newtypes (were bare `String`) and
/// `base_version` now returns `Option<crate::ids::ArtifactVersion>` (was a bare `u64`
/// defaulting to `0`, which conflated "no base" with "based on version 0" — `None` fixes that);
/// `state_class` is a new defaulted method so every existing `impl` recompiles unchanged.
/// `validate` and its CRDT-era merge/reconcile hooks are GONE (ticket
/// `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C4): every rejection or
/// merge-policy concern a technology used to express through those hooks now travels as a
/// [`MutationMessage`] on `diff`'s own [`MutationOutcome`].
pub trait Mutation<P>: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue {
    type Diff: MutationDiff<P>;
    /// 🧷️ Every direct mutation leaf's static metadata, in aggregate-variant order.
    const DESCRIPTORS: &'static [MutationLeafDescriptor];

    /// 🧷️ This value's own direct mutation leaf descriptor.
    fn descriptor(&self) -> &'static MutationLeafDescriptor;
    fn diff(&self, base: &P) -> MutationOutcome<Self::Diff>;
    fn inverse(&self, base: &P) -> Result<Vec<Self>, semio_framework_value::ValueError> ;

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
    /// 🧾️ The upper bound of the rows [`Self::inverse`] yields for this operation on any base: what a one-item Store preparation
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

//#region 🪪️MutationLeafDescriptor
/// 🧷️ Schema vocabulary for one direct mutation's inversion behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MutationInvertibility {
    SelfInvertible,
    ExplicitMutation,
    Plan,
    NonInvertible,
}

/// 🌱️ Hand-written, not derived — same DAG reason `MutationMessage`'s hand-written twin below
/// documents. `Serialize`-only in the original (no `Deserialize`: these are static roster metadata,
/// never hydrated from the wire), so only `ToValue` is mirrored — capability parity, not more.
impl semio_framework_value::ToValue for MutationInvertibility {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(
            match self {
                MutationInvertibility::SelfInvertible => "self",
                MutationInvertibility::ExplicitMutation => "explicit-mutation",
                MutationInvertibility::Plan => "plan",
                MutationInvertibility::NonInvertible => "non-invertible",
            }
            .to_string(),
        )
    }
}

/// 🧷️ Schema vocabulary for one direct mutation's diff participation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MutationDiffParticipation {
    Detect,
    ApplyOnly,
    Plan,
    None,
}

/// 🌱️ Hand-written, not derived — same reason as `MutationInvertibility` above.
impl semio_framework_value::ToValue for MutationDiffParticipation {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(
            match self {
                MutationDiffParticipation::Detect => "detect",
                MutationDiffParticipation::ApplyOnly => "apply-only",
                MutationDiffParticipation::Plan => "plan",
                MutationDiffParticipation::None => "none",
            }
            .to_string(),
        )
    }
}

/// 🧷️ The protocol outcome class one direct mutation can reach — the vocabulary fixtures, manifests and bridges
/// share verbatim, so no layer projects one record onto another.
///
/// @see 🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json — `$defs/MutationOutcomeClass`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MutationOutcomeClass {
    Applied,
    NoOp,
    Empty,
    Disjoint,
    Rejected,
}

impl MutationOutcomeClass {
    /// 🏷️ The wire spelling of this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            MutationOutcomeClass::Applied => "applied",
            MutationOutcomeClass::NoOp => "no-op",
            MutationOutcomeClass::Empty => "empty",
            MutationOutcomeClass::Disjoint => "disjoint",
            MutationOutcomeClass::Rejected => "rejected",
        }
    }
}

/// 🌱️ Hand-written, not derived — same reason as `MutationInvertibility` above.
impl semio_framework_value::ToValue for MutationOutcomeClass {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(self.as_str().to_string())
    }
}

/// 🧷️ Schema vocabulary for one direct mutation's composition form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MutationComposition {
    Atomic,
    Composite,
}

/// 🌱️ Hand-written, not derived — same reason as `MutationInvertibility` above.
impl semio_framework_value::ToValue for MutationComposition {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(match self { MutationComposition::Atomic => "atomic", MutationComposition::Composite => "composite" }.to_string())
    }
}

/// 🧷️ Schema vocabulary for a direct mutation's required language surfaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MutationLanguageSurface {
    Rust,
    Typescript,
    Graphql,
    Protobuf,
    JsonSchema,
    Text,
    Binary,
}

/// 🌱️ Hand-written, not derived — same reason as `MutationInvertibility` above.
impl semio_framework_value::ToValue for MutationLanguageSurface {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::String(
            match self {
                MutationLanguageSurface::Rust => "rust",
                MutationLanguageSurface::Typescript => "typescript",
                MutationLanguageSurface::Graphql => "graphql",
                MutationLanguageSurface::Protobuf => "protobuf",
                MutationLanguageSurface::JsonSchema => "json-schema",
                MutationLanguageSurface::Text => "text",
                MutationLanguageSurface::Binary => "binary",
            }
            .to_string(),
        )
    }
}

/// 🧷️ Exact fourteen-field static metadata contract for one direct mutation leaf.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MutationLeafDescriptor {
    pub schema_version: u32,
    pub owner: &'static str,
    pub semantic_kind: &'static str,
    pub display_name: &'static str,
    pub emoji: &'static str,
    pub aggregate_variant: &'static str,
    pub payload_schema: &'static str,
    pub text_opcode: Option<&'static str>,
    pub binary_tag: Option<u32>,
    pub invertibility: MutationInvertibility,
    pub diff_participation: MutationDiffParticipation,
    pub outcome_classes: &'static [MutationOutcomeClass],
    pub composition: MutationComposition,
    pub required_language_surfaces: &'static [MutationLanguageSurface],
}

/// 🌱️ Hand-written, not derived — same reason as `MutationInvertibility` above. `&'static str`/
/// `&'static [T]` have no blanket `ToValue` (unlike `String`/`Vec<T>`), so string/slice fields are
/// encoded inline rather than through a generic impl.
impl semio_framework_value::ToValue for MutationLeafDescriptor {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let entries = vec![
            ("schemaVersion".to_string(), semio_framework_value::ToValue::to_value(&self.schema_version)),
            ("owner".to_string(), semio_framework_value::DslValue::String(self.owner.to_string())),
            ("semanticKind".to_string(), semio_framework_value::DslValue::String(self.semantic_kind.to_string())),
            ("displayName".to_string(), semio_framework_value::DslValue::String(self.display_name.to_string())),
            ("emoji".to_string(), semio_framework_value::DslValue::String(self.emoji.to_string())),
            ("aggregateVariant".to_string(), semio_framework_value::DslValue::String(self.aggregate_variant.to_string())),
            ("payloadSchema".to_string(), semio_framework_value::DslValue::String(self.payload_schema.to_string())),
            ("textOpcode".to_string(), match self.text_opcode { Some(s) => semio_framework_value::DslValue::String(s.to_string()), None => semio_framework_value::DslValue::Null }),
            ("binaryTag".to_string(), semio_framework_value::ToValue::to_value(&self.binary_tag)),
            ("invertibility".to_string(), semio_framework_value::ToValue::to_value(&self.invertibility)),
            ("diffParticipation".to_string(), semio_framework_value::ToValue::to_value(&self.diff_participation)),
            ("outcomeClasses".to_string(), semio_framework_value::DslValue::Array(self.outcome_classes.iter().map(semio_framework_value::ToValue::to_value).collect())),
            ("composition".to_string(), semio_framework_value::ToValue::to_value(&self.composition)),
            ("requiredLanguageSurfaces".to_string(), semio_framework_value::DslValue::Array(self.required_language_surfaces.iter().map(semio_framework_value::ToValue::to_value).collect())),
        ];
        semio_framework_value::DslValue::object(entries)
    }
}

/// 🧷️ Identifies a static descriptor field that violates the language-neutral schema contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MutationLeafDescriptorValidationError {
    pub field: &'static str,
    pub requirement: &'static str,
}

/// 🧷️ Identifies a same-owner static roster violation and its duplicate positions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MutationLeafDescriptorRosterValidationError {
    pub owner: &'static str,
    pub field: &'static str,
    pub first_index: usize,
    pub index: usize,
}

impl MutationLeafDescriptor {
    /// 🧷️ Validates this exact static descriptor against the fourteen-field schema contract.
    pub const fn validate(&self) -> Result<(), MutationLeafDescriptorValidationError> {
        validate_mutation_leaf_descriptor(self)
    }
}

/// 🧷️ Validates one static descriptor without introducing defaults or partial metadata.
pub const fn validate_mutation_leaf_descriptor(descriptor: &MutationLeafDescriptor) -> Result<(), MutationLeafDescriptorValidationError> {
    if descriptor.schema_version != 1 {
        return Err(MutationLeafDescriptorValidationError { field: "schemaVersion", requirement: "must equal 1" });
    }
    if !mutation_leaf_descriptor_owner(descriptor.owner) {
        return Err(MutationLeafDescriptorValidationError { field: "owner", requirement: "must name a non-compose direct mutation leaf" });
    }
    if !mutation_leaf_descriptor_kebab(descriptor.semantic_kind) {
        return Err(MutationLeafDescriptorValidationError { field: "semanticKind", requirement: "must be a two-or-more-segment kebab identifier" });
    }
    if descriptor.display_name.is_empty() {
        return Err(MutationLeafDescriptorValidationError { field: "displayName", requirement: "must be non-empty" });
    }
    if descriptor.emoji.is_empty() {
        return Err(MutationLeafDescriptorValidationError { field: "emoji", requirement: "must be non-empty" });
    }
    if !mutation_leaf_descriptor_pascal(descriptor.aggregate_variant) {
        return Err(MutationLeafDescriptorValidationError { field: "aggregateVariant", requirement: "must be an ASCII Pascal identifier" });
    }
    if descriptor.payload_schema.is_empty() {
        return Err(MutationLeafDescriptorValidationError { field: "payloadSchema", requirement: "must be non-empty" });
    }
    if let Some(opcode) = descriptor.text_opcode {
        if !mutation_leaf_descriptor_kebab(opcode) {
            return Err(MutationLeafDescriptorValidationError { field: "textOpcode", requirement: "must be null or a two-or-more-segment kebab identifier" });
        }
    }
    if descriptor.outcome_classes.is_empty() || !mutation_leaf_descriptor_outcomes_unique(descriptor.outcome_classes) {
        return Err(MutationLeafDescriptorValidationError { field: "outcomeClasses", requirement: "must be a non-empty unique array" });
    }
    if descriptor.required_language_surfaces.is_empty() || !mutation_leaf_descriptor_surfaces_unique(descriptor.required_language_surfaces) || !mutation_leaf_descriptor_has_rust(descriptor.required_language_surfaces) {
        return Err(MutationLeafDescriptorValidationError { field: "requiredLanguageSurfaces", requirement: "must be a non-empty unique array containing rust" });
    }
    Ok(())
}

/// 🧷️ Validates exact descriptor uniqueness within one explicit owner roster.
pub const fn validate_mutation_leaf_descriptor_roster(mutation_root: &'static str, descriptors: &'static [MutationLeafDescriptor], owner_layout: MutationOwnerLayout) -> Result<(), MutationLeafDescriptorRosterValidationError> {
    if !mutation_leaf_descriptor_root(mutation_root) {
        return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "owner", first_index: 0, index: 0 });
    }
    if let MutationOwnerLayout::DomainOperations(owners) = owner_layout {
        if owners.is_empty() || owners.len() != descriptors.len() {
            return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "owner", first_index: 0, index: 0 });
        }
    }
    let layout_valid = mutation_owner_layout_valid(mutation_root, owner_layout);
    let mut index = 0;
    while index < descriptors.len() {
        let descriptor = &descriptors[index];
        if let Err(error) = validate_mutation_leaf_descriptor(descriptor) {
            return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: error.field, first_index: index, index });
        }
        if !layout_valid || !mutation_owner_layout_identity_matches(mutation_root, owner_layout, descriptor.owner, descriptor.semantic_kind) {
            return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "owner", first_index: index, index });
        }
        index += 1;
    }
    validate_mutation_leaf_descriptor_roster_uniqueness(mutation_root, descriptors, owner_layout)
}

/// 🧮️ Checks cross-leaf uniqueness and roster completeness after individual leaf source validation.
pub const fn validate_mutation_leaf_descriptor_roster_uniqueness(mutation_root: &'static str, descriptors: &'static [MutationLeafDescriptor], owner_layout: MutationOwnerLayout) -> Result<(), MutationLeafDescriptorRosterValidationError> {
    if let MutationOwnerLayout::DomainOperations(owners) = owner_layout {
        if owners.is_empty() || owners.len() != descriptors.len() {
            return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "owner", first_index: 0, index: 0 });
        }
    }
    let mut index = 0;
    while index < descriptors.len() {
        let descriptor = &descriptors[index];
        let mut duplicate = index + 1;
        while duplicate < descriptors.len() {
            let other = &descriptors[duplicate];
            if mutation_leaf_descriptor_str_eq(descriptor.semantic_kind, other.semantic_kind) {
                return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "semanticKind", first_index: index, index: duplicate });
            }
            if mutation_leaf_descriptor_str_eq(descriptor.owner, other.owner) {
                return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "owner", first_index: index, index: duplicate });
            }
            if let (Some(left), Some(right)) = (descriptor.text_opcode, other.text_opcode) {
                if mutation_leaf_descriptor_str_eq(left, right) {
                    return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "textOpcode", first_index: index, index: duplicate });
                }
            }
            if let (Some(left), Some(right)) = (descriptor.binary_tag, other.binary_tag) {
                if left == right {
                    return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "binaryTag", first_index: index, index: duplicate });
                }
            }
            duplicate += 1;
        }
        index += 1;
    }
    Ok(())
}

const MUTATION_ROOT_MARKER: &[u8] = "/🧬️mutations/".as_bytes();
const MUTATION_ROOT_SUFFIX: &[u8] = "/🧬️mutations".as_bytes();

const fn mutation_leaf_descriptor_owner(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() { return false; }
    let mut index = 0;
    let mut marker = false;
    while index < bytes.len() {
        if bytes[index] == b'\n' || bytes[index] == b'\r' || (index + 2 < bytes.len() && bytes[index] == 0xe2 && bytes[index + 1] == 0x80 && (bytes[index + 2] == 0xa8 || bytes[index + 2] == 0xa9)) { return false; }
        if (index == 0 || bytes[index - 1] == b'/') && mutation_leaf_descriptor_bytes_at(bytes, index, b"compose") && (index + 7 == bytes.len() || bytes[index + 7] == b'/') { return false; }
        if !marker && index > 0 && index + MUTATION_ROOT_MARKER.len() < bytes.len() && mutation_leaf_descriptor_bytes_at(bytes, index, MUTATION_ROOT_MARKER) { marker = true; }
        index += 1;
    }
    marker
}

const fn mutation_leaf_descriptor_root(value: &str) -> bool {
    let bytes = value.as_bytes();
    mutation_leaf_descriptor_relative_path(bytes) && mutation_leaf_descriptor_path_safe(bytes) && bytes.len() > MUTATION_ROOT_SUFFIX.len() && mutation_leaf_descriptor_bytes_at(bytes, bytes.len() - MUTATION_ROOT_SUFFIX.len(), MUTATION_ROOT_SUFFIX)
}

const fn mutation_leaf_descriptor_direct_child(root: &str, owner: &str) -> bool {
    let root = root.as_bytes();
    let owner = owner.as_bytes();
    if owner.len() <= root.len() + 1 || !mutation_leaf_descriptor_bytes_at(owner, 0, root) || owner[root.len()] != b'/' { return false; }
    let start = root.len() + 1;
    let mut index = start;
    while index < owner.len() {
        if owner[index] == b'/' || owner[index] == b'\\' || owner[index] == 0 { return false; }
        index += 1;
    }
    !(owner.len() == start + 1 && owner[start] == b'.') && !(owner.len() == start + 2 && owner[start] == b'.' && owner[start + 1] == b'.')
}

const fn mutation_owner_domain_child(root: &str, owner: &str) -> bool {
    if !mutation_leaf_source_path(owner) { return false; }
    let root = root.as_bytes();
    let owner = owner.as_bytes();
    if owner.len() <= root.len() + 1 || !mutation_leaf_descriptor_bytes_at(owner, 0, root) || owner[root.len()] != b'/' { return false; }
    let mut index = root.len() + 1;
    let mut separators = 0;
    while index < owner.len() {
        if owner[index] == b'/' { separators += 1; }
        index += 1;
    }
    separators == 1
}

const fn mutation_owner_layout_valid(root: &str, layout: MutationOwnerLayout) -> bool {
    if let MutationOwnerLayout::DomainOperations(owners) = layout {
        let mut index = 0;
        while index < owners.len() {
            let entry = &owners[index];
            if !mutation_owner_domain_child(root, entry.owner) || !mutation_leaf_descriptor_kebab(entry.semantic_kind) { return false; }
            index += 1;
        }
    }
    true
}

const fn mutation_owner_layout_identity_matches(root: &str, layout: MutationOwnerLayout, owner: &str, semantic_kind: &str) -> bool {
    match layout {
        MutationOwnerLayout::Flat => mutation_leaf_descriptor_direct_child(root, owner),
        MutationOwnerLayout::DomainOperations(owners) => {
            if !mutation_owner_domain_child(root, owner) { return false; }
            let mut index = 0;
            let mut matches = 0;
            while index < owners.len() {
                let entry = &owners[index];
                let same_owner = mutation_leaf_descriptor_str_eq(owner, entry.owner);
                let same_identity = mutation_leaf_descriptor_str_eq(semantic_kind, entry.semantic_kind);
                if same_owner != same_identity { return false; }
                if same_owner { matches += 1; }
                index += 1;
            }
            matches == 1
        }
    }
}

const fn mutation_leaf_descriptor_relative_path(bytes: &[u8]) -> bool {
    if bytes.is_empty() || bytes[0] == b'/' || bytes[0] == b'\\' { return false; }
    let mut start = 0;
    let mut index = 0;
    while index <= bytes.len() {
        if index == bytes.len() || bytes[index] == b'/' {
            let length = index - start;
            if length == 0 || (length == 1 && bytes[start] == b'.') || (length == 2 && bytes[start] == b'.' && bytes[start + 1] == b'.') { return false; }
            start = index + 1;
        } else if bytes[index] == b'\\' || bytes[index] == b':' || bytes[index] == 0 { return false; }
        index += 1;
    }
    true
}

const fn mutation_leaf_descriptor_path_safe(bytes: &[u8]) -> bool {
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\n' || bytes[index] == b'\r' || (index + 2 < bytes.len() && bytes[index] == 0xe2 && bytes[index + 1] == 0x80 && (bytes[index + 2] == 0xa8 || bytes[index + 2] == 0xa9)) { return false; }
        if (index == 0 || bytes[index - 1] == b'/') && mutation_leaf_descriptor_bytes_at(bytes, index, b"compose") && (index + 7 == bytes.len() || bytes[index + 7] == b'/') { return false; }
        index += 1;
    }
    true
}

const fn mutation_leaf_descriptor_kebab(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() < 3 || !mutation_leaf_descriptor_ascii_lower(bytes[0]) { return false; }
    let mut index = 1;
    let mut hyphen = false;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'-' {
            if index + 1 == bytes.len() || bytes[index + 1] == b'-' { return false; }
            hyphen = true;
        } else if !mutation_leaf_descriptor_ascii_lower(byte) && !mutation_leaf_descriptor_ascii_digit(byte) { return false; }
        index += 1;
    }
    hyphen
}

const fn mutation_leaf_descriptor_pascal(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() || !mutation_leaf_descriptor_ascii_upper(bytes[0]) { return false; }
    let mut index = 1;
    while index < bytes.len() {
        let byte = bytes[index];
        if !mutation_leaf_descriptor_ascii_upper(byte) && !mutation_leaf_descriptor_ascii_lower(byte) && !mutation_leaf_descriptor_ascii_digit(byte) { return false; }
        index += 1;
    }
    true
}

const fn mutation_leaf_descriptor_outcomes_unique(values: &[MutationOutcomeClass]) -> bool {
    let mut index = 0;
    while index < values.len() {
        let mut duplicate = index + 1;
        while duplicate < values.len() {
            if values[index] as u8 == values[duplicate] as u8 { return false; }
            duplicate += 1;
        }
        index += 1;
    }
    true
}

const fn mutation_leaf_descriptor_surfaces_unique(values: &[MutationLanguageSurface]) -> bool {
    let mut index = 0;
    while index < values.len() {
        let mut duplicate = index + 1;
        while duplicate < values.len() {
            if values[index] as u8 == values[duplicate] as u8 { return false; }
            duplicate += 1;
        }
        index += 1;
    }
    true
}

const fn mutation_leaf_descriptor_has_rust(values: &[MutationLanguageSurface]) -> bool {
    let mut index = 0;
    while index < values.len() {
        if values[index] as u8 == MutationLanguageSurface::Rust as u8 { return true; }
        index += 1;
    }
    false
}

const fn mutation_leaf_descriptor_bytes_at(bytes: &[u8], index: usize, needle: &[u8]) -> bool {
    if index + needle.len() > bytes.len() { return false; }
    let mut offset = 0;
    while offset < needle.len() {
        if bytes[index + offset] != needle[offset] { return false; }
        offset += 1;
    }
    true
}

const fn mutation_leaf_descriptor_str_eq(left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();
    if left.len() != right.len() { return false; }
    let mut index = left.len();
    while index > 0 {
        index -= 1;
        if left[index] != right[index] { return false; }
    }
    true
}

const fn mutation_leaf_descriptor_ascii_lower(byte: u8) -> bool { byte >= b'a' && byte <= b'z' }
const fn mutation_leaf_descriptor_ascii_upper(byte: u8) -> bool { byte >= b'A' && byte <= b'Z' }
const fn mutation_leaf_descriptor_ascii_digit(byte: u8) -> bool { byte >= b'0' && byte <= b'9' }
//#endregion 🪪️MutationLeafDescriptor

//#region 🪪️MutationLeaf
/// 🧭️ Compile-time source facts for one direct mutation leaf; excluded from descriptor wire identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MutationSourceProvenance {
    pub workspace_token: [u8; 32],
    pub mutation_root: &'static str,
    pub owner: &'static str,
    pub source_path: &'static str,
    pub descriptor_path: &'static str,
    pub taxonomy_path: &'static str,
}

/// 🏘️ One explicitly registered physical domain-operation owner and its unchanged semantic identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MutationDomainOperation {
    pub owner: &'static str,
    pub semantic_kind: &'static str,
}

/// 🗺️ Exact aggregate layout authority; domain owners never infer identities or accept arbitrary descendants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MutationOwnerLayout {
    Flat,
    DomainOperations(&'static [MutationDomainOperation]),
}

/// 🧭️ Aggregate-owned source facts used to validate one explicitly owned mutation leaf.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MutationLeafSourceScope {
    pub workspace_token: [u8; 32],
    pub mutation_root: &'static str,
    pub owner_layout: MutationOwnerLayout,
    pub taxonomy_path: &'static str,
    pub mutation_payload_facet: &'static str,
    pub source_filename: &'static str,
    pub descriptor_filename: &'static str,
}

/// 🚫️ Identifies one aggregate-source fact a mutation leaf does not prove.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MutationLeafSourceValidationError {
    pub field: &'static str,
    pub requirement: &'static str,
}

/// 🧭️ Validates a direct leaf's descriptor and provenance against aggregate-owned source facts.
pub const fn validate_mutation_leaf_source(
    descriptor: &MutationLeafDescriptor,
    provenance: &MutationSourceProvenance,
    scope: &MutationLeafSourceScope,
) -> Result<(), MutationLeafSourceValidationError> {
    if let Err(error) = validate_mutation_leaf_descriptor(descriptor) {
        return Err(MutationLeafSourceValidationError { field: error.field, requirement: error.requirement });
    }
    match scope.validate() {
        Ok(checked) => checked.validate_leaf(descriptor, provenance),
        Err(error) => Err(error),
    }
}

/// 🪪️ Aggregate authority whose root, filenames and physical layout were validated once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValidatedMutationLeafSourceScope {
    scope: MutationLeafSourceScope,
}

impl MutationLeafSourceScope {
    /// 🧭️ Checks shared aggregate facts before validating each leaf against the same immutable authority.
    pub const fn validate(&self) -> Result<ValidatedMutationLeafSourceScope, MutationLeafSourceValidationError> {
        let scope = self;
        if !mutation_leaf_descriptor_root(scope.mutation_root) || !mutation_leaf_source_path(scope.mutation_root) {
            return Err(MutationLeafSourceValidationError { field: "mutationRoot", requirement: "must be a safe normalized mutation root" });
        }
        if !mutation_leaf_source_path(scope.taxonomy_path) {
            return Err(MutationLeafSourceValidationError { field: "taxonomyPath", requirement: "must be a safe normalized portable path" });
        }
        if !mutation_leaf_source_filename(scope.mutation_payload_facet) {
            return Err(MutationLeafSourceValidationError { field: "mutationPayloadFacet", requirement: "must be one safe normalized portable segment" });
        }
        if !mutation_leaf_source_filename(scope.source_filename) {
            return Err(MutationLeafSourceValidationError { field: "sourceFilename", requirement: "must be a safe normalized portable filename" });
        }
        if !mutation_leaf_source_filename(scope.descriptor_filename) {
            return Err(MutationLeafSourceValidationError { field: "descriptorFilename", requirement: "must be a safe normalized portable filename" });
        }
        if !mutation_owner_layout_valid(scope.mutation_root, scope.owner_layout) {
            return Err(MutationLeafSourceValidationError { field: "owner", requirement: "must match the aggregate's exact physical owner and semantic identity layout" });
        }
        Ok(ValidatedMutationLeafSourceScope { scope: *scope })
    }
}

impl ValidatedMutationLeafSourceScope {
    /// 🔎️ Checks each descriptor and its exact source provenance against the validated aggregate.
    pub const fn validate_leaf(&self, descriptor: &MutationLeafDescriptor, provenance: &MutationSourceProvenance) -> Result<(), MutationLeafSourceValidationError> {
        let scope = &self.scope;
        if let Err(error) = validate_mutation_leaf_descriptor(descriptor) {
            return Err(MutationLeafSourceValidationError { field: error.field, requirement: error.requirement });
        }
        if !mutation_leaf_source_path(descriptor.owner) {
            return Err(MutationLeafSourceValidationError { field: "owner", requirement: "must be a safe normalized portable path" });
        }
        if !mutation_owner_layout_identity_matches(scope.mutation_root, scope.owner_layout, descriptor.owner, descriptor.semantic_kind) {
            return Err(MutationLeafSourceValidationError { field: "owner", requirement: "must match the aggregate's exact physical owner and semantic identity layout" });
        }
        if !mutation_leaf_source_tokens_match(&scope.workspace_token, &provenance.workspace_token) {
            return Err(MutationLeafSourceValidationError { field: "workspaceToken", requirement: "must equal the aggregate workspace token" });
        }
        if !mutation_leaf_descriptor_str_eq(provenance.mutation_root, scope.mutation_root) {
            return Err(MutationLeafSourceValidationError { field: "mutationRoot", requirement: "must equal the aggregate mutation root" });
        }
        if !mutation_leaf_descriptor_str_eq(provenance.owner, descriptor.owner) {
            return Err(MutationLeafSourceValidationError { field: "owner", requirement: "must equal the descriptor owner" });
        }
        if !mutation_leaf_descriptor_str_eq(provenance.taxonomy_path, scope.taxonomy_path) {
            return Err(MutationLeafSourceValidationError { field: "taxonomyPath", requirement: "must equal the aggregate taxonomy path" });
        }
        if !mutation_leaf_source_path_matches_direct_or_payload(descriptor.owner, scope.mutation_payload_facet, scope.source_filename, provenance.source_path) {
            return Err(MutationLeafSourceValidationError { field: "sourcePath", requirement: "must equal owner plus the canonical source filename, directly or under the canonical payload facet" });
        }
        if !mutation_leaf_source_path_matches(descriptor.owner, scope.descriptor_filename, provenance.descriptor_path) {
            return Err(MutationLeafSourceValidationError { field: "descriptorPath", requirement: "must equal owner plus the canonical descriptor filename" });
        }
        Ok(())
    }
}

const fn mutation_leaf_source_path(value: &str) -> bool {
    let bytes = value.as_bytes();
    mutation_leaf_descriptor_relative_path(bytes) && mutation_leaf_descriptor_path_safe(bytes) && mutation_leaf_source_no_compose_ascii_case(bytes)
}

const fn mutation_leaf_source_filename(value: &str) -> bool {
    let bytes = value.as_bytes();
    if !mutation_leaf_source_path(value) { return false; }
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'/' { return false; }
        index += 1;
    }
    true
}

const fn mutation_leaf_source_no_compose_ascii_case(bytes: &[u8]) -> bool {
    let mut index = 0;
    while index < bytes.len() {
        if (index == 0 || bytes[index - 1] == b'/') && mutation_leaf_source_bytes_at_ascii_case(bytes, index, b"compose") && (index + 7 == bytes.len() || bytes[index + 7] == b'/') { return false; }
        index += 1;
    }
    true
}

const fn mutation_leaf_source_bytes_at_ascii_case(bytes: &[u8], start: usize, expected: &[u8]) -> bool {
    if start + expected.len() > bytes.len() { return false; }
    let mut index = 0;
    while index < expected.len() {
        let byte = bytes[start + index];
        let folded = if byte >= b'A' && byte <= b'Z' { byte + (b'a' - b'A') } else { byte };
        if folded != expected[index] { return false; }
        index += 1;
    }
    true
}

const fn mutation_leaf_source_tokens_match(left: &[u8; 32], right: &[u8; 32]) -> bool {
    let mut index = 0;
    while index < 32 {
        if left[index] != right[index] { return false; }
        index += 1;
    }
    true
}

const fn mutation_leaf_source_path_matches(owner: &str, filename: &str, value: &str) -> bool {
    let owner = owner.as_bytes();
    let filename = filename.as_bytes();
    let value = value.as_bytes();
    value.len() == owner.len() + 1 + filename.len()
        && mutation_leaf_descriptor_bytes_at(value, 0, owner)
        && value[owner.len()] == b'/'
        && mutation_leaf_descriptor_bytes_at(value, owner.len() + 1, filename)
}

const fn mutation_leaf_source_path_matches_direct_or_payload(owner: &str, payload_facet: &str, filename: &str, value: &str) -> bool {
    if mutation_leaf_source_path_matches(owner, filename, value) { return true; }
    let owner = owner.as_bytes();
    let payload_facet = payload_facet.as_bytes();
    let filename = filename.as_bytes();
    let value = value.as_bytes();
    value.len() == owner.len() + 1 + payload_facet.len() + 1 + filename.len()
        && mutation_leaf_descriptor_bytes_at(value, 0, owner)
        && value[owner.len()] == b'/'
        && mutation_leaf_descriptor_bytes_at(value, owner.len() + 1, payload_facet)
        && value[owner.len() + 1 + payload_facet.len()] == b'/'
        && mutation_leaf_descriptor_bytes_at(value, owner.len() + 1 + payload_facet.len() + 1, filename)
}

/// 🪪️ Ownership contract for a direct mutation leaf, and the editable payload of one of its operations.
pub trait MutationLeaf {
    const DESCRIPTOR: MutationLeafDescriptor;
    const PROVENANCE: MutationSourceProvenance;
    /// 🧬️ The leaf's normative payload JSON Schema text (`DESCRIPTOR.payload_schema`, embedded at compile time by
    /// `#[derive(dsl::MutationLeaf)]`) — the source of its input descriptors (`manifest::mutation_input_defs`).
    const PAYLOAD_SCHEMA: &'static str;
    /// 🔗️ Every schema document [`Self::PAYLOAD_SCHEMA`] references by `$id`, transitively, that the leaf's own plugin (or
    /// framework module) tree holds — embedded by `#[derive(dsl::MutationLeaf)]` and published at runtime, so every input resolves
    /// whatever facet or artifact of the plugin it lives in (design §16.3).
    const PAYLOAD_SCHEMA_DOCUMENTS: &'static [&'static str] = &[];
    /// 🧬️ [`Self::PAYLOAD_SCHEMA`] when this operation is user-editable, else `None`. A leaf whose type wraps its payload
    /// in one variant of an enum (`#[mutation_leaf(payload = Apply)]`) is editable only in that variant; its other
    /// variants (an internal inverse such as `Restore`) are not.
    fn input_schema(&self) -> Option<&'static str> {
        Some(Self::PAYLOAD_SCHEMA)
    }
    /// 🧬️ The value [`Self::input_schema`] describes: the whole leaf value, or the content of the `payload` variant.
    fn input_value(&self) -> semio_framework_value::DslValue
    where
        Self: semio_framework_value::ToValue,
    {
        semio_framework_value::ToValue::to_value(self)
    }
    /// 🧬️ Rebuilds this operation from an edited [`Self::input_value`]; an operation that is not editable refuses.
    fn with_input_value(&self, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError>
    where
        Self: Sized + semio_framework_value::FromValue,
    {
        <Self as semio_framework_value::FromValue>::from_value(value)
    }
    /// 🧬️ A new editable operation of this leaf from its [`Self::input_value`] alone: the whole leaf value, or the content
    /// of the `payload` variant.
    fn from_input_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError>
    where
        Self: Sized + semio_framework_value::FromValue,
    {
        <Self as semio_framework_value::FromValue>::from_value(value)
    }
    /// 🧾️ The upper bound of the rows `Mutation::inverse` yields for this operation on any base, declared schema-first by the
    /// leaf payload schema's `x-semio-inverse-rows` (`fixed`, `perTarget`, `bounded`; absent = one row) and generated by
    /// `#[derive(dsl::MutationLeaf)]` (design §20.5).
    fn inverse_rows(&self) -> usize;
}
//#endregion 🪪️MutationLeaf

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-leaf-metadata/🦀️.rs"]
mod mutation_leaf_metadata_tests;

//#region 🔖️Message
/// 🪙️ One code of the frozen outcome vocabulary. The only code type [`MutationOutcome::refuse`] accepts, so a guard
/// that picks its code at runtime can never name a code outside the table or at a level the table does not fix.
/// Language-agnostic twin: `🧪️tests/🧫️fixtures/🧫️outcome-code/🔣️.json` (schema `🧬️schema/🔣️outcome-code`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OutcomeCode {
    TargetMissing,
    TargetReferenced,
    TargetMismatch,
    NoOp,
    Partial,
    Clamped,
    PreconditionDrifted,
    DuplicateId,
    Invariant,
    InverseRefused,
    Cascade,
}

impl OutcomeCode {
    /// 📚️ Every vocabulary code, in the document's order.
    pub const ALL: [OutcomeCode; 11] = [Self::TargetMissing, Self::TargetReferenced, Self::TargetMismatch, Self::NoOp, Self::Partial, Self::Clamped, Self::PreconditionDrifted, Self::DuplicateId, Self::Invariant, Self::InverseRefused, Self::Cascade];

    /// 🔤️ The wire spelling of the code.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetMissing => "mutation.target-missing",
            Self::TargetReferenced => "mutation.target-referenced",
            Self::TargetMismatch => "mutation.target-mismatch",
            Self::NoOp => "mutation.no-op",
            Self::Partial => "mutation.partial",
            Self::Clamped => "mutation.clamped",
            Self::PreconditionDrifted => "mutation.precondition-drifted",
            Self::DuplicateId => "mutation.duplicate-id",
            Self::Invariant => "mutation.invariant",
            Self::InverseRefused => "mutation.inverse-refused",
            Self::Cascade => "mutation.cascade",
        }
    }

    /// 🎚️ The one level the vocabulary fixes for the code.
    pub const fn level(self) -> semio_framework_diagnostic::Severity {
        match self {
            Self::TargetMissing | Self::TargetReferenced | Self::TargetMismatch => semio_framework_diagnostic::Severity::Error,
            Self::NoOp | Self::Partial | Self::Clamped | Self::PreconditionDrifted => semio_framework_diagnostic::Severity::Warning,
            Self::DuplicateId | Self::Invariant | Self::InverseRefused => semio_framework_diagnostic::Severity::Fatal,
            Self::Cascade => semio_framework_diagnostic::Severity::Info,
        }
    }
}

/// 🔡️ A vocabulary code converts to its wire spelling wherever a bare code string travels (validator errors, faults).
impl From<OutcomeCode> for &'static str {
    fn from(code: OutcomeCode) -> Self {
        code.as_str()
    }
}

/// 🎟️ A vocabulary code is a message code: any builder taking a code takes the typed one.
impl From<OutcomeCode> for semio_framework_diagnostic::FaultCode {
    fn from(code: OutcomeCode) -> Self {
        Self::from(code.as_str())
    }
}

/// 📖️ The frozen outcome-code vocabulary and the one level each code fixes, derived from [`OutcomeCode`] — the table
/// persistence validates against (`🏪️store` `expected_mutation_message_level`). Closed: no per-plugin codes.
pub const OUTCOME_CODES: [(&str, semio_framework_diagnostic::Severity); 11] = {
    let mut table = [("", semio_framework_diagnostic::Severity::Info); 11];
    let mut index = 0;
    while index < OutcomeCode::ALL.len() {
        table[index] = (OutcomeCode::ALL[index].as_str(), OutcomeCode::ALL[index].level());
        index += 1;
    }
    table
};

/// 🧱️ Prefix of the apply-time rejection family `mutation.apply.<kebab-detail>`, always `Fatal`.
pub const APPLY_OUTCOME_CODE_PREFIX: &str = "mutation.apply.";

/// ⚖️ The level the frozen vocabulary fixes for `code`; `None` for any code outside it.
pub fn outcome_code_level(code: &str) -> Option<semio_framework_diagnostic::Severity> {
    OUTCOME_CODES.iter().find(|(known, _)| *known == code).map(|(_, level)| *level).or_else(|| {
        code.strip_prefix(APPLY_OUTCOME_CODE_PREFIX)
            .filter(|detail| detail.split('-').all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())))
            .map(|_| semio_framework_diagnostic::Severity::Fatal)
    })
}

/// 📨️ One outcome-carried diagnostic from a `Mutation`/`MutationKind::diff` — the level
/// vocabulary is [`crate::diagnostic::Severity`] (`Info < Warning < Error < Fatal`, that declaration
/// order IS the level order via `derive(Ord)`); `code` is one of the frozen `mutation.*`
/// codes (`.🧬semio/🦑️repo/🎫️tickets/26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/
/// 📋️contract-freeze.md` §C2, extended 2026-09-30 by the state-dependent `Error` codes `mutation.target-referenced` and
/// `mutation.target-mismatch`, 2026-10-05 by the `Warning` `mutation.precondition-drifted` and the `Fatal`
/// `mutation.inverse-refused` — closed set, no per-plugin codes, ever); `message` is English prose
/// (UI localizes by `code`, never by parsing `message`); `target` is the address of the offending
/// element (outermost segment first, matching [`MutationKind::target`]'s convention); `op_index` is
/// stamped by a batch replay ([`MutationOutcome::stamp_op_index`]) once this message's originating
/// op's position within the batch is known.
/// 🌱️ serde's derives are carried ALONGSIDE the hand-written `ToValue`/`FromValue` twin below —
/// the transitional state the serde-fanout playbook prescribes ("add alongside, do not blind-swap").
/// Attributes are restored verbatim from 67fb4216b2 so the serde wire shape stays byte-identical to
/// the twin. Drop them once every consumer in `🔗️causal`/`📡️wire`/`⚔️conflict` moves to `ToValue`.
#[derive(Clone, Debug, PartialEq)]
pub struct MutationMessage {
    pub level: semio_framework_diagnostic::Severity,
    pub code: semio_framework_diagnostic::FaultCode,
    pub message: String,
    pub target: Vec<String>,
    pub op_index: Option<u32>,
}

/// 🌉️ Hand-written, not derived: mirrors the `#[serde(default, skip_serializing_if = "…")]`
/// sparse-emission on `target`/`op_index` (only emitted when non-default), matching the
/// pre-existing serde wire shape byte-for-byte in field presence.
impl semio_framework_value::ToValue for MutationMessage {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = vec![
            ("level".to_string(), semio_framework_value::ToValue::to_value(&self.level)),
            ("code".to_string(), semio_framework_value::ToValue::to_value(&self.code)),
            ("message".to_string(), semio_framework_value::ToValue::to_value(&self.message)),
        ];
        if !self.target.is_empty() {
            entries.push(("target".to_string(), semio_framework_value::ToValue::to_value(&self.target)));
        }
        if self.op_index.is_some() {
            entries.push(("opIndex".to_string(), semio_framework_value::ToValue::to_value(&self.op_index)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}
impl semio_framework_value::FromValue for MutationMessage {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for MutationMessage, found {value:?}")));
        };
        let mut level = None;
        let mut code = None;
        let mut message = None;
        let mut target = Vec::new();
        let mut op_index = None;
        for (key, entry) in fields {
            match key.as_str() {
                "level" => level = Some(<semio_framework_diagnostic::Severity as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("level"))?),
                "code" => code = Some(<semio_framework_diagnostic::FaultCode as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("code"))?),
                "message" => message = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("message"))?),
                "target" => target = <Vec<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("target"))?,
                "opIndex" => op_index = <Option<u32> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("opIndex"))?,
                _ => {}
            }
        }
        Ok(MutationMessage {
            level: level.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationMessage missing level"))?,
            code: code.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationMessage missing code"))?,
            message: message.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationMessage missing message"))?,
            target,
            op_index,
        })
    }
}

impl MutationMessage {
    fn at_level(level: semio_framework_diagnostic::Severity, code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        Self { level, code: code.into(), message: message.into(), target: Vec::new(), op_index: None }
    }

    pub fn info(code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        Self::at_level(semio_framework_diagnostic::Severity::Info, code, message)
    }
    pub fn warning(code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        Self::at_level(semio_framework_diagnostic::Severity::Warning, code, message)
    }
    pub fn error(code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        Self::at_level(semio_framework_diagnostic::Severity::Error, code, message)
    }
    pub fn fatal(code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        Self::at_level(semio_framework_diagnostic::Severity::Fatal, code, message)
    }

    /// 🎯️ Attaches the target address (outermost segment first).
    pub fn at(mut self, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.target = target.into_iter().map(Into::into).collect();
        self
    }

    /// 🔢️ Stamps which op (within a batch) produced this message.
    pub fn at_op(mut self, op_index: u32) -> Self {
        self.op_index = Some(op_index);
        self
    }
}

/// 🚦️ The worst (highest) [`crate::diagnostic::Severity`] across `messages`, or `None` if empty.
pub fn worst_level(messages: &[MutationMessage]) -> Option<semio_framework_diagnostic::Severity> {
    messages.iter().map(|message| message.level).max()
}

/// 🗂️ A `Mutation`/`MutationKind::diff`'s full result: the diff plus every
/// [`MutationMessage`] it raised. LAWS (`📋️contract-freeze.md` §C2): (1) a `Fatal` message ⇒
/// `diff == D::default()`; (2) an `Error` message ⇒ `diff` carries no change for the named target;
/// (3) deterministic — equal `(op, base)` ⇒ equal `messages`. Laws 1/2 are upheld by every `diff`
/// leaf's own construction (via [`MutationOutcome::fatal`]/[`MutationOutcome::error`], or by simply
/// never writing to a rejected target before calling [`MutationOutcome::info`]/
/// [`MutationOutcome::warning`]/[`MutationOutcome::absorb_messages`]), not enforced by this type
/// itself — see `🧪️testkit`'s `assert_fatal_never_applies`/`assert_missing_target_is_error`.
///
/// 🎯️ Naming note: the frozen contract's prose also lists chainable `.error(..).await`/`.fatal(..).await`
/// instance builders alongside the static `::error(code,msg,target).await`/`::fatal(code,msg,target).await`
/// whole-outcome shortcuts — those two forms cannot share a name on the same inherent type (Rust
/// E0592: an instance method and an associated function of the same name conflict across impl
/// blocks whenever their `D` bounds overlap). This crate keeps `fatal`/`error` as the static,
/// whole-outcome-rejecting shortcuts (the common case for a simple non-batch verb) and `info`/`warning`
/// as the 2-arg chainable instance builders (the fan-out recipe's own `.info("mutation.cascade", ..).await`
/// example); a leaf needing a TARGETED error/warning alongside a non-empty diff (the batch-verb case).await
/// builds the message directly (`MutationMessage::error(code, msg).await.at(target).await`) and attaches it via
/// [`MutationOutcome::absorb_messages`].
#[derive(Clone, Debug, PartialEq)]
pub struct MutationOutcome<D> {
    diff: D,
    messages: Vec<MutationMessage>,
}

/// 🌱️ Hand-written, not derived — same DAG reason `MutationMessage`'s hand-written twin above
/// documents. Mirrors `#[serde(rename_all = "camelCase", default, skip_serializing_if =
/// "Vec::is_empty")]` byte-for-byte.
impl<D: semio_framework_value::ToValue> semio_framework_value::ToValue for MutationOutcome<D> {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = vec![("diff".to_string(), semio_framework_value::ToValue::to_value(&self.diff))];
        if !self.messages.is_empty() {
            entries.push(("messages".to_string(), semio_framework_value::ToValue::to_value(&self.messages)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}
impl<D: semio_framework_value::FromValue> semio_framework_value::FromValue for MutationOutcome<D> {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for MutationOutcome, found {value:?}")));
        };
        let mut diff = None;
        let mut messages = Vec::new();
        for (key, entry) in fields {
            match key.as_str() {
                "diff" => diff = Some(<D as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("diff"))?),
                "messages" => messages = <Vec<MutationMessage> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("messages"))?,
                _ => {}
            }
        }
        Ok(MutationOutcome { diff: diff.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "MutationOutcome missing diff"))?, messages })
    }
}

impl<D: Default> MutationOutcome<D> {
    /// 🕳️ No change, no messages — the algebra's identity element.
    pub fn empty() -> Self {
        Self { diff: D::default(), messages: Vec::new() }
    }

    /// 🚨️ Forces `diff = D::default()` (LAW 1) with one `Fatal` message.
    pub fn fatal(code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { diff: D::default(), messages: vec![MutationMessage::fatal(code, message).at(target)] }
    }

    /// 🚫️ Empty diff with one `Error` message — the caller's `diff` leaf never touched `target`.
    pub fn error(code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { diff: D::default(), messages: vec![MutationMessage::error(code, message).at(target)] }
    }

    /// 🧭️ Empty diff with one message at the level the vocabulary fixes for `code` — for a guard that picks its
    /// vocabulary code at runtime; [`OutcomeCode`] admits no other code.
    pub fn refuse(code: OutcomeCode, message: impl Into<String>, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { diff: D::default(), messages: vec![MutationMessage::at_level(code.level(), code.as_str(), message).at(target)] }
    }
}

impl<D> MutationOutcome<D> {
    /// ✅️ A successful diff, no messages.
    pub fn new(diff: D) -> Self {
        Self { diff, messages: Vec::new() }
    }

    pub fn diff(&self) -> &D {
        &self.diff
    }

    pub fn messages(&self) -> &[MutationMessage] {
        &self.messages
    }

    /// ➡️ Consumes `self` into its raw `(diff, messages)` parts.
    pub fn into_parts(self) -> (D, Vec<MutationMessage>) {
        (self.diff, self.messages)
    }

    /// ➕️ Appends one `Info`-level message (e.g. a cascade note).
    pub fn info(mut self, code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        self.messages.push(MutationMessage::info(code, message));
        self
    }

    /// ➕️ Appends one `Warning`-level message.
    pub fn warning(mut self, code: impl Into<semio_framework_diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        self.messages.push(MutationMessage::warning(code, message));
        self
    }

    /// ➕️ Appends every message in `messages`, in order — the general escape hatch for a targeted
    /// message that the 2-arg `info`/`warning` builders can't express (see the naming note above).
    pub fn absorb_messages(mut self, messages: impl IntoIterator<Item = MutationMessage>) -> Self {
        self.messages.extend(messages);
        self
    }

    /// 🔢️ Stamps `op_index` onto every message currently carried — called once per op during a
    /// batch replay, right after that op's `diff` is computed.
    pub fn stamp_op_index(mut self, op_index: u32) -> Self {
        for message in &mut self.messages {
            message.op_index = Some(op_index);
        }
        self
    }

    /// 🚦️ The worst level across `self.messages`, or `None` if there are none.
    pub fn worst_level(&self) -> Option<semio_framework_diagnostic::Severity> {
        worst_level(&self.messages)
    }

    /// ✅️ Whether `policy` would accept this outcome (i.e. does NOT reject its worst level). An
    /// outcome with no messages is always applicable.
    pub fn is_applicable(&self, policy: semio_framework_replication_wire_core::MergePolicy) -> bool {
        match self.worst_level() {
            Some(level) => !policy.rejects(level),
            None => true,
        }
    }

    /// 🔀️ Maps the diff, keeping every message unchanged.
    pub fn map<D2>(self, f: impl FnOnce(D) -> D2) -> MutationOutcome<D2> {
        MutationOutcome { diff: f(self.diff), messages: self.messages }
    }
}
//#endregion 🔖️Message

//#region 🔖️Foreign
#[path = "🌐️foreign/🦀️.rs"]
mod foreign_output;
pub use foreign_output::{ForeignTarget,ForeignStep,ForeignStepSource,ForeignStepCopy,ForeignStepRetirement,ForeignStepsOwner,ForeignStepsPreparation};
//#endregion 🔖️Foreign

#[cfg(test)]
#[path = "🧪️tests/🧪️outcome-code/🦀️.rs"]
mod outcome_code_tests;

/// 🎯️ The one store step from an operation to its next projection: computes the op-stamped diff, applies it through
/// [`protocol::apply_diff`] and retires the diff cold. Every store lane, replay, merge and backbone fold routes through
/// here; the apply refusal stays structured for callers that must fail instead of record.
pub fn apply_operation<P, Mutation>(state: &P, operation: &Mutation, op_index: u32) -> (semio_framework_mutation_diff::MutationApplyResult<P>, Vec<crate::MutationMessage>)
where
    Mutation: self::Mutation<P>,
{
    let (diff, messages) = operation.diff(state).stamp_op_index(op_index).into_parts();
    let applied = semio_framework_mutation_diff::apply_diff(&diff, state);
    <Mutation::Diff as MutationDiff<P>>::retire_cold(diff);
    (applied, messages)
}

mod owned_retirement {
    use super::*;
    semio_framework_value::artifact_retire_struct!(MutationMessage { level, code, message, target, op_index });
}
