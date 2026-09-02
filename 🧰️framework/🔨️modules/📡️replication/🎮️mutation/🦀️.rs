//! 🎞️ Mutation contract: the `Mutation`/`MutationDiff` trait family, per-operation
//! diagnostics (`MutationMessage`/`MutationOutcome`), and the `OpText`/`OpBinary`/`DiffCodec`
//! grammar seams. Product-neutral — both the optimistic client replica and the authority
//! rerun the same deciders against these types. Op payloads stay schema-opaque: this module
//! never parses an `Op`, it only threads it through the trait seams a technology implements.
//!
//! Frozen contract: `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md`
//! `## Amendment` §`protocol_command`.

//#region 🔖️Mutation
/// @emoji 🚫️ Structured rejection of a diff that cannot be applied to its supplied base.
/// The shape is protocol-owned and wire-safe: callers never need a technology crate's error type
/// to preserve the stable machine code, human diagnostic, and outermost-first target address.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MutationApplyError {
    pub code: String,
    pub message: String,
    pub target: Vec<String>,
}

/// 🌱️ Hand-written, not derived — same DAG reason `MutationMessage`'s hand-written twin below
/// documents (this crate sits below `os-kernel`). Mirrors the pre-existing `#[serde(default,
/// skip_serializing_if = "Vec::is_empty")]` sparse-emission on `target` byte-for-byte.
impl crate::value::ToValue for MutationApplyError {
    fn to_value(&self) -> crate::value::DslValue {
        let mut entries = vec![("code".to_string(), crate::value::ToValue::to_value(&self.code)), ("message".to_string(), crate::value::ToValue::to_value(&self.message))];
        if !self.target.is_empty() {
            entries.push(("target".to_string(), crate::value::ToValue::to_value(&self.target)));
        }
        crate::value::DslValue::object(entries)
    }
}
impl crate::value::FromValue for MutationApplyError {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for MutationApplyError, found {value:?}")));
        };
        let mut code = None;
        let mut message = None;
        let mut target = Vec::new();
        for (key, entry) in fields {
            match key.as_str() {
                "code" => code = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("code"))?),
                "message" => message = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("message"))?),
                "target" => target = <Vec<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("target"))?,
                _ => {}
            }
        }
        Ok(MutationApplyError {
            code: code.ok_or_else(|| crate::value::ValueError::new("MutationApplyError missing code"))?,
            message: message.ok_or_else(|| crate::value::ValueError::new("MutationApplyError missing message"))?,
            target,
        })
    }
}

impl std::fmt::Display for MutationApplyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for MutationApplyError {}

impl MutationApplyError {
    /// 🏗️ Builds an untargeted typed rejection.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self { code: code.into(), message: message.into(), target: Vec::new() }
    }

    /// 🎯️ Attaches the rejected diff address, outermost segment first.
    pub fn at(mut self, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.target = target.into_iter().map(Into::into).collect();
        self
    }

    /// 🪆️ Prefixes outer address segments while retaining the inner error's exact target.
    pub fn under(mut self, prefix: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let mut target: Vec<String> = prefix.into_iter().map(Into::into).collect();
        target.append(&mut self.target);
        self.target = target;
        self
    }
}

/// @emoji 🛡️ Crate-owned result of applying a diff to a snapshot.
pub type MutationApplyResult<P> = Result<P, MutationApplyError>;

/// @emoji 📦️ Centralized snapshot mutation — one fallible `apply` per technology. A
/// malformed or base-incompatible persisted diff must return [`MutationApplyError`]; it must never
/// clamp an index, ignore a missing target, or return the unchanged base as implicit success.
///
/// Bound on [`crate::value::ToValue`]/[`crate::value::FromValue`], not `serde::Serialize`/
/// `serde::de::DeserializeOwned` — every plugin technology implementing this trait used to be
/// forced onto `serde` by this supertrait alone; see
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
/// 🔍️research/📓️serde-replacement-surface.md`.
pub trait MutationDiff<P>: Clone + Default + crate::value::ToValue + crate::value::FromValue {
    fn apply(&self, base: &P) -> MutationApplyResult<P>;
    /// @emoji ➕️ Composes `self` (base→mid) with `other` (mid→after) into base→after, in place.
    /// Normative absorb contract (`.claude/plans/the-current-schemas-are-scalable-journal.md`
    /// `## Absorb`): **structural** (operates on the diff's own key/index/field shape, never on
    /// applied snapshot values), **total** (defined for every pair of diffs over the same
    /// artifact, including out-of-range/no-op cases — never panics), **base-free** (no snapshot
    /// parameter; the two diffs alone determine the result), and **sequential-coalesce only**
    /// (this composes two diffs known to have been applied in sequence by the same actor;
    /// concurrent-edit merging is an authority's `MergePolicy`/`📡️spr/⚔️conflict` job, never this
    /// method's — the CRDT-era concurrent-diff merge helper this docstring used to point at is
    /// deleted, see `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`).
    /// LAW: whenever sequential application succeeds,
    /// `absorb(d1, d2).await.apply(base).await == d1.apply(base).await.and_then(|mid| d2.apply(&mid).await)`, associative
    /// over further absorbs of the same artifact's diff vocabulary. A rejection remains a
    /// rejection; absorb must not manufacture an implicit success path.
    fn absorb(&mut self, other: Self);
}

/// @emoji 🧮️ Diff-level algebra for a technology's [`MutationDiff`] type: inverse, state-delta
/// construction, and emptiness. Deliberately a SEPARATE trait from `MutationDiff` (not new
/// methods added to it) — `MutationDiff` already has 51+ repo-wide implementors, so a breaking
/// method addition there would break all of them at once. Follows this crate's own `DiffCodec`
/// precedent below: land the trait standalone in a spine wave, adopt it per-type in later waves
/// via a seeded shrink-only policy allowlist (`POLICY_DIFF_ALGEBRA`), never as a hard bound on
/// `MutationDiff` itself until every implementor is covered.
/// LAWS (for valid diffs): `d.inverse(base).await.apply(&d.apply(base).await?).await == Ok(*base)`;
/// `Self::between(a, b).await.apply(a).await == Ok(*b)`; `Self::between(a, a).await.is_empty().await`.
pub trait DiffAlgebra<P>: Sized {
    /// 🔁️ Diff-level undo: the diff that, applied after `self`, restores `base`.
    fn inverse(&self, base: &P) -> Self;
    /// 🧭️ State delta: the diff that, applied to `base`, yields `other`.
    fn between(base: &P, other: &P) -> Self;
    /// 🕳️ Whether this diff changes nothing relative to whatever base it was built against.
    fn is_empty(&self) -> bool;
}

/// @emoji 🔁️ Stored operation: emits a [`MutationOutcome`] (diff plus messages) and computes inverse
/// from pre-state. Moved from `os_store::Mutation` verbatim except: `mutation_id`/
/// `dependencies`/`author_id` now return the `protocol_core` id newtypes (were bare `String`) and
/// `base_version` now returns `Option<crate::ids::ArtifactVersion>` (was a bare `u64`
/// defaulting to `0`, which conflated "no base" with "based on version 0" — `None` fixes that);
/// `state_class` is a new defaulted method so every existing `impl` recompiles unchanged.
/// `validate` and its CRDT-era merge/reconcile hooks are GONE (ticket
/// `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C4): every rejection or
/// merge-policy concern a technology used to express through those hooks now travels as a
/// [`MutationMessage`] on `diff`'s own [`MutationOutcome`].
pub trait Mutation<P>: Clone + crate::value::ToValue + crate::value::FromValue {
    type Diff: MutationDiff<P>;
    /// 🧷️ Every direct mutation leaf's static metadata, in aggregate-variant order.
    const DESCRIPTORS: &'static [MutationLeafDescriptor];

    /// 🧷️ This value's own direct mutation leaf descriptor.
    fn descriptor(&self) -> &'static MutationLeafDescriptor;
    fn diff(&self, base: &P) -> MutationOutcome<Self::Diff>;
    fn inverse(&self, base: &P) -> Vec<Self>;

    fn mutation_id(&self) -> Option<crate::ids::MutationId> {
        None
    }
    fn dependencies(&self) -> Vec<crate::ids::MutationId> {
        Vec::new()
    }
    fn base_version(&self) -> Option<crate::ids::ArtifactVersion> {
        None
    }
    fn author_id(&self) -> Option<crate::ids::ActorId> {
        None
    }
    fn timestamp(&self) -> Option<crate::ids::HybridLogicalTimestamp> {
        None
    }
    fn undo_policy(&self) -> crate::UndoPolicy {
        crate::UndoPolicy::ExactBaseOnly
    }
    /// @emoji 🗂️ Which durability/visibility class this operation's diffs belong to.
    fn state_class(&self) -> crate::StateClass {
        crate::StateClass::Artifact
    }
    /// @emoji 🌐️ Conservative base-independent foreign-step capability used to avoid replaying
    /// ordinary local operations solely for transaction-proposal discovery.
    fn may_emit_foreign_steps(&self) -> bool {
        true
    }
    /// @emoji 🌐️ Foreign steps this operation additionally dispatches to OTHER artifacts — empty
    /// for every ordinary single-artifact operation. Defaults to `Vec::new().await` so no existing
    /// `impl Mutation` breaks; only a composite mutation's delegating `MutationKind::foreign_steps`
    /// (see `🔖️Composite` below, `plan_foreign_steps`) ever returns anything here.
    fn foreign_steps(&self, _base: &P) -> Vec<ForeignStep> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation

//#region 🪪️MutationLeafDescriptor
/// 🧷️ Schema vocabulary for one direct mutation's inversion behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum MutationInvertibility {
    #[serde(rename = "self")]
    SelfInvertible,
    ExplicitMutation,
    Plan,
    NonInvertible,
}

/// 🌱️ Hand-written, not derived — same DAG reason `MutationMessage`'s hand-written twin below
/// documents. `Serialize`-only in the original (no `Deserialize`: these are static roster metadata,
/// never hydrated from the wire), so only `ToValue` is mirrored — capability parity, not more.
impl crate::value::ToValue for MutationInvertibility {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::String(
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum MutationDiffParticipation {
    Detect,
    ApplyOnly,
    Plan,
    None,
}

/// 🌱️ Hand-written, not derived — same reason as `MutationInvertibility` above.
impl crate::value::ToValue for MutationDiffParticipation {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::String(
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

/// 🧷️ Schema vocabulary for one direct mutation's observable outcomes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum MutationOutcomeClass {
    Applied,
    Info,
    Warning,
    Error,
    Fatal,
}

/// 🌱️ Hand-written, not derived — same reason as `MutationInvertibility` above.
impl crate::value::ToValue for MutationOutcomeClass {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::String(
            match self {
                MutationOutcomeClass::Applied => "applied",
                MutationOutcomeClass::Info => "info",
                MutationOutcomeClass::Warning => "warning",
                MutationOutcomeClass::Error => "error",
                MutationOutcomeClass::Fatal => "fatal",
            }
            .to_string(),
        )
    }
}

/// 🧷️ Schema vocabulary for one direct mutation's composition form.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum MutationComposition {
    Atomic,
    Composite,
}

/// 🌱️ Hand-written, not derived — same reason as `MutationInvertibility` above.
impl crate::value::ToValue for MutationComposition {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::String(match self { MutationComposition::Atomic => "atomic", MutationComposition::Composite => "composite" }.to_string())
    }
}

/// 🧷️ Schema vocabulary for a direct mutation's required language surfaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
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
impl crate::value::ToValue for MutationLanguageSurface {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::String(
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
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
impl crate::value::ToValue for MutationLeafDescriptor {
    fn to_value(&self) -> crate::value::DslValue {
        let entries = vec![
            ("schemaVersion".to_string(), crate::value::ToValue::to_value(&self.schema_version)),
            ("owner".to_string(), crate::value::DslValue::String(self.owner.to_string())),
            ("semanticKind".to_string(), crate::value::DslValue::String(self.semantic_kind.to_string())),
            ("displayName".to_string(), crate::value::DslValue::String(self.display_name.to_string())),
            ("emoji".to_string(), crate::value::DslValue::String(self.emoji.to_string())),
            ("aggregateVariant".to_string(), crate::value::DslValue::String(self.aggregate_variant.to_string())),
            ("payloadSchema".to_string(), crate::value::DslValue::String(self.payload_schema.to_string())),
            ("textOpcode".to_string(), match self.text_opcode { Some(s) => crate::value::DslValue::String(s.to_string()), None => crate::value::DslValue::Null }),
            ("binaryTag".to_string(), crate::value::ToValue::to_value(&self.binary_tag)),
            ("invertibility".to_string(), crate::value::ToValue::to_value(&self.invertibility)),
            ("diffParticipation".to_string(), crate::value::ToValue::to_value(&self.diff_participation)),
            ("outcomeClasses".to_string(), crate::value::DslValue::Array(self.outcome_classes.iter().map(crate::value::ToValue::to_value).collect())),
            ("composition".to_string(), crate::value::ToValue::to_value(&self.composition)),
            ("requiredLanguageSurfaces".to_string(), crate::value::DslValue::Array(self.required_language_surfaces.iter().map(crate::value::ToValue::to_value).collect())),
        ];
        crate::value::DslValue::object(entries)
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
pub const fn validate_mutation_leaf_descriptor_roster(mutation_root: &'static str, descriptors: &'static [MutationLeafDescriptor]) -> Result<(), MutationLeafDescriptorRosterValidationError> {
    if !mutation_leaf_descriptor_root(mutation_root) {
        return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "owner", first_index: 0, index: 0 });
    }
    let mut index = 0;
    while index < descriptors.len() {
        let descriptor = &descriptors[index];
        if let Err(error) = validate_mutation_leaf_descriptor(descriptor) {
            return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: error.field, first_index: index, index });
        }
        if !mutation_leaf_descriptor_direct_child(mutation_root, descriptor.owner) {
            return Err(MutationLeafDescriptorRosterValidationError { owner: mutation_root, field: "owner", first_index: index, index });
        }
        index += 1;
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
    mutation_leaf_descriptor_bytes_at(left, 0, right)
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

/// 🧭️ Aggregate-owned source facts used to validate one direct mutation leaf.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MutationLeafSourceScope {
    pub workspace_token: [u8; 32],
    pub mutation_root: &'static str,
    pub taxonomy_path: &'static str,
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
    if !mutation_leaf_descriptor_root(scope.mutation_root) || !mutation_leaf_source_path(scope.mutation_root) {
        return Err(MutationLeafSourceValidationError { field: "mutationRoot", requirement: "must be a safe normalized mutation root" });
    }
    if !mutation_leaf_source_path(scope.taxonomy_path) {
        return Err(MutationLeafSourceValidationError { field: "taxonomyPath", requirement: "must be a safe normalized portable path" });
    }
    if !mutation_leaf_source_filename(scope.source_filename) {
        return Err(MutationLeafSourceValidationError { field: "sourceFilename", requirement: "must be a safe normalized portable filename" });
    }
    if !mutation_leaf_source_filename(scope.descriptor_filename) {
        return Err(MutationLeafSourceValidationError { field: "descriptorFilename", requirement: "must be a safe normalized portable filename" });
    }
    if !mutation_leaf_source_path(descriptor.owner) {
        return Err(MutationLeafSourceValidationError { field: "owner", requirement: "must be a safe normalized portable path" });
    }
    if !mutation_leaf_descriptor_direct_child(scope.mutation_root, descriptor.owner) {
        return Err(MutationLeafSourceValidationError { field: "owner", requirement: "must be an immediate child of mutationRoot" });
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
    if !mutation_leaf_source_path_matches(descriptor.owner, scope.source_filename, provenance.source_path) {
        return Err(MutationLeafSourceValidationError { field: "sourcePath", requirement: "must equal owner plus the canonical source filename" });
    }
    if !mutation_leaf_source_path_matches(descriptor.owner, scope.descriptor_filename, provenance.descriptor_path) {
        return Err(MutationLeafSourceValidationError { field: "descriptorPath", requirement: "must equal owner plus the canonical descriptor filename" });
    }
    Ok(())
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

/// 🪪️ Metadata-only ownership contract for a direct mutation leaf.
pub trait MutationLeaf {
    const DESCRIPTOR: MutationLeafDescriptor;
    const PROVENANCE: MutationSourceProvenance;
}
//#endregion 🪪️MutationLeaf

#[cfg(test)]
mod mutation_leaf_metadata_tests {
    use super::*;

    const LEAF_DESCRIPTOR: MutationLeafDescriptor = MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page",
        semantic_kind: "insert-page",
        display_name: "Insert Page",
        emoji: "➕️",
        aggregate_variant: "InsertPage",
        payload_schema: "🦀️.rs#InsertPage",
        text_opcode: None,
        binary_tag: None,
        invertibility: MutationInvertibility::ExplicitMutation,
        diff_participation: MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[MutationOutcomeClass::Applied],
        composition: MutationComposition::Atomic,
        required_language_surfaces: &[MutationLanguageSurface::Rust],
    };
    const LEAF_PROVENANCE: MutationSourceProvenance = MutationSourceProvenance {
        workspace_token: [0x2a; 32],
        mutation_root: "✏️s/🔌️plugins/🧪️probe/🧬️mutations",
        owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page",
        source_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/🦀️.rs",
        descriptor_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/🔣️.json",
        taxonomy_path: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
    };
    const LEAF_SOURCE_SCOPE: MutationLeafSourceScope = MutationLeafSourceScope {
        workspace_token: [0x2a; 32],
        mutation_root: "✏️s/🔌️plugins/🧪️probe/🧬️mutations",
        taxonomy_path: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
        source_filename: "🦀️.rs",
        descriptor_filename: "🔣️.json",
    };
    const LEAF_PROVENANCE_TOKEN_MISMATCH: MutationSourceProvenance = MutationSourceProvenance { workspace_token: [0x2b; 32], ..LEAF_PROVENANCE };
    const LEAF_SOURCE_VALID: Result<(), MutationLeafSourceValidationError> = validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &LEAF_PROVENANCE, &LEAF_SOURCE_SCOPE);
    const LEAF_SOURCE_TOKEN_REJECTED: Result<(), MutationLeafSourceValidationError> = validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &LEAF_PROVENANCE_TOKEN_MISMATCH, &LEAF_SOURCE_SCOPE);
    const _: () = match LEAF_SOURCE_VALID { Ok(()) => (), Err(_) => panic!("canonical leaf source must validate") };
    const _: () = match LEAF_SOURCE_TOKEN_REJECTED { Err(_) => (), Ok(()) => panic!("workspace-token mismatch must reject") };
    struct BorrowedLeaf<'a, T>(&'a T);
    impl<'a, T> MutationLeaf for BorrowedLeaf<'a, T> {
        const DESCRIPTOR: MutationLeafDescriptor = LEAF_DESCRIPTOR;
        const PROVENANCE: MutationSourceProvenance = LEAF_PROVENANCE;
    }
    fn metadata_from<T: MutationLeaf>(_: &T) -> (MutationLeafDescriptor, MutationSourceProvenance) {
        (T::DESCRIPTOR, T::PROVENANCE)
    }

    #[test]
    fn borrowed_generic_leaf_infers_static_metadata() {
        let local = 42_u32;
        let borrowed = &local;
        let leaf = BorrowedLeaf(&borrowed);
        let (descriptor, provenance) = metadata_from(&leaf);
        assert_eq!(**leaf.0, 42);
        assert_eq!(descriptor, LEAF_DESCRIPTOR);
        assert_eq!(provenance, LEAF_PROVENANCE);
        assert_eq!(provenance.owner, descriptor.owner);
    }

    #[test]
    fn compiler_contract_vectors_have_complete_expected_outcomes() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🧬️mutation-leaf-contract/🧫️fixtures/🔣️.json")).expect("valid lower mutation leaf contract fixture");
        let cases = fixture["cases"].as_array().expect("compiler cases");
        assert_eq!(cases.len(), 3);
        assert!(cases.iter().any(|case| case["borrowedGeneric"] == true && case["expectedCompile"] == true));
        for case in cases.iter().filter(|case| case["expectedCompile"] == false) {
            assert_eq!(case["errorCode"], "E0046", "{}", case["name"]);
        }
    }

    #[test]
    fn source_contract_rejects_every_workspace_token_byte_and_path_decoy() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🧬️mutation-leaf-source-contract/🧫️fixtures/🔣️.json")).expect("valid lower mutation leaf source fixture");
        let bytes = fixture["workspaceTokenMismatchBytes"].as_array().expect("workspace token byte vectors");
        assert_eq!(bytes.len(), 32);
        for byte in bytes {
            let index = byte.as_u64().expect("workspace token index") as usize;
            let mut provenance = LEAF_PROVENANCE;
            provenance.workspace_token[index] ^= 0xff;
            assert_eq!(validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &provenance, &LEAF_SOURCE_SCOPE), Err(MutationLeafSourceValidationError { field: "workspaceToken", requirement: "must equal the aggregate workspace token" }));
        }
        let nested = MutationLeafDescriptor { owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/🧪️tests", ..LEAF_DESCRIPTOR };
        assert_eq!(validate_mutation_leaf_source(&nested, &LEAF_PROVENANCE, &LEAF_SOURCE_SCOPE).unwrap_err().field, "owner");
        let historical = MutationSourceProvenance { source_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/component.rs", ..LEAF_PROVENANCE };
        assert_eq!(validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &historical, &LEAF_SOURCE_SCOPE).unwrap_err().field, "sourcePath");
        let alternate_scope = MutationLeafSourceScope { source_filename: "operation.rs", descriptor_filename: "metadata.json", ..LEAF_SOURCE_SCOPE };
        let alternate_provenance = MutationSourceProvenance { source_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/operation.rs", descriptor_path: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page/metadata.json", ..LEAF_PROVENANCE };
        assert_eq!(validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &alternate_provenance, &alternate_scope), Ok(()));
        let unsafe_taxonomy = MutationLeafSourceScope { taxonomy_path: "🧰️framework/../🔣️taxonomy.json", ..LEAF_SOURCE_SCOPE };
        assert_eq!(validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &LEAF_PROVENANCE, &unsafe_taxonomy).unwrap_err().field, "taxonomyPath");
        let compose_filename = MutationLeafSourceScope { source_filename: "Compose", ..LEAF_SOURCE_SCOPE };
        assert_eq!(validate_mutation_leaf_source(&LEAF_DESCRIPTOR, &LEAF_PROVENANCE, &compose_filename).unwrap_err().field, "sourceFilename");
        assert_eq!(fixture["cases"].as_array().expect("source cases").len(), 26);
    }
}

//#region 🔖️Message
/// @emoji 📨️ One outcome-carried diagnostic from a `Mutation`/`MutationKind::diff` — the level
/// vocabulary is [`crate::diagnostic::Severity`] (`Info < Warning < Error < Fatal`, that declaration
/// order IS the level order via `derive(Ord)`); `code` is one of the frozen seven `mutation.*`
/// codes (`.🦑️repo/🎫️tickets/26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/
/// 📋️contract-freeze.md` §C2 — closed set, no per-plugin codes, ever); `message` is English prose
/// (UI localizes by `code`, never by parsing `message`); `target` is the address of the offending
/// element (outermost segment first, matching [`MutationKind::target`]'s convention); `op_index` is
/// stamped by a batch replay ([`MutationOutcome::stamp_op_index`]) once this message's originating
/// op's position within the batch is known.
/// 🌱️ serde's derives are carried ALONGSIDE the hand-written `ToValue`/`FromValue` twin below —
/// the transitional state the serde-fanout playbook prescribes ("add alongside, do not blind-swap").
/// Attributes are restored verbatim from 67fb4216b2 so the serde wire shape stays byte-identical to
/// the twin. Drop them once every consumer in `🔗️causal`/`📡️wire`/`⚔️conflict` moves to `ToValue`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationMessage {
    pub level: crate::diagnostic::Severity,
    pub code: crate::diagnostic::FaultCode,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op_index: Option<u32>,
}

/// 🌉️ Hand-written, not derived: mirrors the `#[serde(default, skip_serializing_if = "…")]`
/// sparse-emission on `target`/`op_index` (only emitted when non-default), matching the
/// pre-existing serde wire shape byte-for-byte in field presence.
impl crate::value::ToValue for MutationMessage {
    fn to_value(&self) -> crate::value::DslValue {
        let mut entries = vec![
            ("level".to_string(), crate::value::ToValue::to_value(&self.level)),
            ("code".to_string(), crate::value::ToValue::to_value(&self.code)),
            ("message".to_string(), crate::value::ToValue::to_value(&self.message)),
        ];
        if !self.target.is_empty() {
            entries.push(("target".to_string(), crate::value::ToValue::to_value(&self.target)));
        }
        if self.op_index.is_some() {
            entries.push(("opIndex".to_string(), crate::value::ToValue::to_value(&self.op_index)));
        }
        crate::value::DslValue::object(entries)
    }
}
impl crate::value::FromValue for MutationMessage {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for MutationMessage, found {value:?}")));
        };
        let mut level = None;
        let mut code = None;
        let mut message = None;
        let mut target = Vec::new();
        let mut op_index = None;
        for (key, entry) in fields {
            match key.as_str() {
                "level" => level = Some(<crate::diagnostic::Severity as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("level"))?),
                "code" => code = Some(<crate::diagnostic::FaultCode as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("code"))?),
                "message" => message = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("message"))?),
                "target" => target = <Vec<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("target"))?,
                "opIndex" => op_index = <Option<u32> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("opIndex"))?,
                _ => {}
            }
        }
        Ok(MutationMessage {
            level: level.ok_or_else(|| crate::value::ValueError::new("MutationMessage missing level"))?,
            code: code.ok_or_else(|| crate::value::ValueError::new("MutationMessage missing code"))?,
            message: message.ok_or_else(|| crate::value::ValueError::new("MutationMessage missing message"))?,
            target,
            op_index,
        })
    }
}

impl MutationMessage {
    fn at_level(level: crate::diagnostic::Severity, code: impl Into<crate::diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        Self { level, code: code.into(), message: message.into(), target: Vec::new(), op_index: None }
    }

    pub fn info(code: impl Into<crate::diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        Self::at_level(crate::diagnostic::Severity::Info, code, message)
    }
    pub fn warn(code: impl Into<crate::diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        Self::at_level(crate::diagnostic::Severity::Warning, code, message)
    }
    pub fn error(code: impl Into<crate::diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        Self::at_level(crate::diagnostic::Severity::Error, code, message)
    }
    pub fn fatal(code: impl Into<crate::diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        Self::at_level(crate::diagnostic::Severity::Fatal, code, message)
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

/// @emoji 🚦️ The worst (highest) [`crate::diagnostic::Severity`] across `messages`, or `None` if empty.
pub fn worst_level(messages: &[MutationMessage]) -> Option<crate::diagnostic::Severity> {
    messages.iter().map(|message| message.level).max()
}

/// @emoji 🗂️ A `Mutation`/`MutationKind::diff`'s full result: the diff plus every
/// [`MutationMessage`] it raised. LAWS (`📋️contract-freeze.md` §C2): (1) a `Fatal` message ⇒
/// `diff == D::default()`; (2) an `Error` message ⇒ `diff` carries no change for the named target;
/// (3) deterministic — equal `(op, base)` ⇒ equal `messages`. Laws 1/2 are upheld by every `diff`
/// leaf's own construction (via [`MutationOutcome::fatal`]/[`MutationOutcome::error`], or by simply
/// never writing to a rejected target before calling [`MutationOutcome::info`]/
/// [`MutationOutcome::warn`]/[`MutationOutcome::absorb_messages`]), not enforced by this type
/// itself — see `🧪️testkit`'s `assert_fatal_never_applies`/`assert_missing_target_is_error`.
///
/// 🎯️ Naming note: the frozen contract's prose also lists chainable `.error(..).await`/`.fatal(..).await`
/// instance builders alongside the static `::error(code,msg,target).await`/`::fatal(code,msg,target).await`
/// whole-outcome shortcuts — those two forms cannot share a name on the same inherent type (Rust
/// E0592: an instance method and an associated function of the same name conflict across impl
/// blocks whenever their `D` bounds overlap). This crate keeps `fatal`/`error` as the static,
/// whole-outcome-rejecting shortcuts (the common case for a simple non-batch verb) and `info`/`warn`
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
impl<D: crate::value::ToValue> crate::value::ToValue for MutationOutcome<D> {
    fn to_value(&self) -> crate::value::DslValue {
        let mut entries = vec![("diff".to_string(), crate::value::ToValue::to_value(&self.diff))];
        if !self.messages.is_empty() {
            entries.push(("messages".to_string(), crate::value::ToValue::to_value(&self.messages)));
        }
        crate::value::DslValue::object(entries)
    }
}
impl<D: crate::value::FromValue> crate::value::FromValue for MutationOutcome<D> {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for MutationOutcome, found {value:?}")));
        };
        let mut diff = None;
        let mut messages = Vec::new();
        for (key, entry) in fields {
            match key.as_str() {
                "diff" => diff = Some(<D as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("diff"))?),
                "messages" => messages = <Vec<MutationMessage> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("messages"))?,
                _ => {}
            }
        }
        Ok(MutationOutcome { diff: diff.ok_or_else(|| crate::value::ValueError::new("MutationOutcome missing diff"))?, messages })
    }
}

impl<D: Default> MutationOutcome<D> {
    /// 🕳️ No change, no messages — the algebra's identity element.
    pub fn empty() -> Self {
        Self { diff: D::default(), messages: Vec::new() }
    }

    /// 🚨️ Forces `diff = D::default()` (LAW 1) with one `Fatal` message.
    pub fn fatal(code: impl Into<crate::diagnostic::FaultCode>, message: impl Into<String>, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { diff: D::default(), messages: vec![MutationMessage::fatal(code, message).at(target)] }
    }

    /// 🚫️ Empty diff with one `Error` message — the caller's `diff` leaf never touched `target`.
    pub fn error(code: impl Into<crate::diagnostic::FaultCode>, message: impl Into<String>, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { diff: D::default(), messages: vec![MutationMessage::error(code, message).at(target)] }
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

    /// 🛡️ Applies this outcome atomically and converts an apply rejection into a fatal outcome.
    pub fn apply_to<P>(self, snapshot: &mut P) -> Self
    where
        D: Default + MutationDiff<P>,
    {
        let (diff, mut messages) = self.into_parts();
        match diff.apply(snapshot) {
            Ok(next) => {
                *snapshot = next;
                Self { diff, messages }
            }
            Err(error) => {
                messages.push(MutationMessage::fatal(error.code, error.message).at(error.target));
                Self { diff: D::default(), messages }
            }
        }
    }

    /// ➕️ Appends one `Info`-level message (e.g. a cascade note).
    pub fn info(mut self, code: impl Into<crate::diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        self.messages.push(MutationMessage::info(code, message));
        self
    }

    /// ➕️ Appends one `Warning`-level message.
    pub fn warn(mut self, code: impl Into<crate::diagnostic::FaultCode>, message: impl Into<String>) -> Self {
        self.messages.push(MutationMessage::warn(code, message));
        self
    }

    /// ➕️ Appends every message in `messages`, in order — the general escape hatch for a targeted
    /// message that the 2-arg `info`/`warn` builders can't express (see the naming note above).
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
    pub fn worst_level(&self) -> Option<crate::diagnostic::Severity> {
        worst_level(&self.messages)
    }

    /// ✅️ Whether `policy` would accept this outcome (i.e. does NOT reject its worst level). An
    /// outcome with no messages is always applicable.
    pub fn is_applicable(&self, policy: crate::MergePolicy) -> bool {
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

//#region 🔖️OpText
/// @emoji ⚡️ Handcrafted ONE-LINE textual representation of an operation, implemented once per
/// technology next to its `Mutation` enum. Moved verbatim from `os_store::OpText` (method order
/// flipped to match the frozen contract; behavior unchanged). LAWS: `print_op` output never
/// contains `\n`; `Op::parse_op` recovers an equal operation from `op.print_op().await`.
pub trait OpText: Sized {
    fn print_op(&self) -> String;
    fn parse_op(line: &str) -> Result<Self, crate::diagnostic::TextError>;
}
//#endregion 🔖️OpText

//#region 🔖️OpBinary
/// @emoji 🎞️ Binary twin of [`OpText`]: the maximum-token-efficient one-line grammar and this
/// byte encoding are two renderings of the same operation, implemented per technology next to its
/// `Mutation` enum (in practice emitted by `#[derive(os_dsl::DslOps)]` through `os_dsl::op_rt`, the
/// exact mirror of the `ArtifactDsl`/`ArtifactPack` pairing). Layout (owned by the runtime, not
/// by implementors): `format u8 (=1) | variant ordinal varint | record body`. LAWS:
/// `Op::decode_op(op.encode_op().await).await == op == Op::parse_op(op.print_op().await).await`, and encoding is
/// deterministic — byte-identical output for equal operations.
pub trait OpBinary: Sized {
    /// 🎯️ Typed tool ids generated by `app_commands!`; non-command binary types keep one fallback key.
    const TOOL_JOB_IDS: &'static [&'static str] = &["typed-command"];
    fn encode_op(&self) -> Result<Vec<u8>, crate::ProtocolError>;
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::ProtocolError>;
}
//#endregion 🔖️OpBinary

//#region 🔖️DiffCodec
/// @emoji 🧬️ Grammared twin of [`OpText`]/[`OpBinary`], but for a technology's `MutationDiff::Diff`
/// value rather than its `Mutation`: the W1 foundation of the `handcrafted-grammar-for-every-artifact`
/// program's diff track (design ruling B-R4 at `.claude/plans/the-final-goal-for-jolly-spindle.md`) —
/// today every `*Diff` type is serde-only, this trait promotes a diff to a first-class grammared value
/// exactly like `OpText`/`OpBinary` already did for operations. In practice emitted by
/// `#[derive(os_dsl::DslDiff)]` through the same `RecordSpec`-generation machinery `DslRecord`/
/// `DslArtifact` already use (a diff is structurally just another record). Schema id convention:
/// `"<doc-schema>#diff"`. Deliberately NOT (yet) a supertrait bound of [`MutationDiff`] — W1 only
/// proves the mechanism on a handful of real diff types (tracked in `script.ts`'s
/// `POLICY_DIFF_COMPLETENESS_ALLOWLIST`); wiring it as a hard bound across all diff types is deferred
/// to wave 6 (`## Master wave plan` `W6 — Lane C (B5)`), once every type is covered.
/// LAWS: `Diff::parse_diff(&d.print_diff().await).await == d`, `Diff::decode_diff(&d.encode_diff().await?).await? == d`,
/// `print_diff` output never contains `\n`, and `encode_diff` is deterministic.
pub trait DiffCodec: Sized {
    fn print_diff(&self) -> String;
    fn parse_diff(line: &str) -> Result<Self, crate::diagnostic::TextError>;
    fn encode_diff(&self) -> Result<Vec<u8>, crate::ProtocolError>;
    fn decode_diff(bytes: &[u8]) -> Result<Self, crate::ProtocolError>;
}
//#endregion 🔖️DiffCodec

//#region 🔖️Foreign
/// @emoji 🌉️ A mutation step aimed at an artifact OTHER than the one being mutated. Cross-boundary
/// identity travels as plain strings, never `semio_framework::*`/`io::*` types — see the
/// dependency-edge law at `.🦑️repo/🎫️tickets/26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS/📋️contract-freeze.md`
/// §0.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeignTarget {
    pub artifact_id: String,
    pub artifact_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dialect: Option<String>,
}

/// 🌉️ Hand-written, not derived — same DAG reason as `HybridLogicalTimestamp`/`ids`/`UndoPolicy`
/// (this crate sits below `os-kernel`). Mirrors `#[serde(rename_all = "camelCase")]` field naming
/// and the `dialect` sparse-emission by hand.
impl crate::value::ToValue for ForeignTarget {
    fn to_value(&self) -> crate::value::DslValue {
        let mut entries = vec![
            ("artifactId".to_string(), crate::value::ToValue::to_value(&self.artifact_id)),
            ("artifactKind".to_string(), crate::value::ToValue::to_value(&self.artifact_kind)),
        ];
        if self.dialect.is_some() {
            entries.push(("dialect".to_string(), crate::value::ToValue::to_value(&self.dialect)));
        }
        crate::value::DslValue::object(entries)
    }
}
impl crate::value::FromValue for ForeignTarget {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for ForeignTarget, found {value:?}")));
        };
        let mut artifact_id = None;
        let mut artifact_kind = None;
        let mut dialect = None;
        for (key, entry) in fields {
            match key.as_str() {
                "artifactId" => artifact_id = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("artifactId"))?),
                "artifactKind" => artifact_kind = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("artifactKind"))?),
                "dialect" => dialect = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("dialect"))?,
                _ => {}
            }
        }
        Ok(ForeignTarget {
            artifact_id: artifact_id.ok_or_else(|| crate::value::ValueError::new("ForeignTarget missing artifactId"))?,
            artifact_kind: artifact_kind.ok_or_else(|| crate::value::ValueError::new("ForeignTarget missing artifactKind"))?,
            dialect,
        })
    }
}

/// @emoji 🪜️ One foreign hop of a [`Planner`]'s plan: the target artifact, the mutation/contributed
/// id it dispatches, its already-encoded payload, and a human label.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeignStep {
    pub target: ForeignTarget,
    pub mutation_id: crate::ids::SchemaId,
    pub payload: Vec<u8>,
    pub label: String,
}

/// 🌱️ Hand-written, not derived — same DAG reason `MutationMessage`'s hand-written twin above
/// documents.
impl crate::value::ToValue for ForeignStep {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("target".to_string(), crate::value::ToValue::to_value(&self.target)),
            ("mutationId".to_string(), crate::value::ToValue::to_value(&self.mutation_id)),
            ("payload".to_string(), crate::value::ToValue::to_value(&self.payload)),
            ("label".to_string(), crate::value::ToValue::to_value(&self.label)),
        ])
    }
}
impl crate::value::FromValue for ForeignStep {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for ForeignStep, found {value:?}")));
        };
        let mut target = None;
        let mut mutation_id = None;
        let mut payload = None;
        let mut label = None;
        for (key, entry) in fields {
            match key.as_str() {
                "target" => target = Some(<ForeignTarget as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("target"))?),
                "mutationId" => mutation_id = Some(<crate::ids::SchemaId as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("mutationId"))?),
                "payload" => payload = Some(<Vec<u8> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("payload"))?),
                "label" => label = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("label"))?),
                _ => {}
            }
        }
        Ok(ForeignStep {
            target: target.ok_or_else(|| crate::value::ValueError::new("ForeignStep missing target"))?,
            mutation_id: mutation_id.ok_or_else(|| crate::value::ValueError::new("ForeignStep missing mutationId"))?,
            payload: payload.ok_or_else(|| crate::value::ValueError::new("ForeignStep missing payload"))?,
            label: label.ok_or_else(|| crate::value::ValueError::new("ForeignStep missing label"))?,
        })
    }
}
//#endregion 🔖️Foreign

//#region 🔖️Meta

/// @emoji 🧾️ Per-operation causal/undo metadata attached to one `Edit` slot. Moved from
/// `crate::os_store::MutationMeta` (was `vcs/rs/lib.rs` L59) with the id-flavored fields upgraded from bare
/// `String`/`Option<String>` to the `protocol_core` newtypes and `timestamp` upgraded from
/// `Option<HybridLogicalTimestamp>` to a required field (an edit's op always has a tick by the time
/// it is durable).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MutationMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation_id: Option<crate::ids::MutationId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<crate::ids::MutationId>,
    pub base_version: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<crate::ids::ActorId>,
    pub timestamp: crate::ids::HybridLogicalTimestamp,
    pub undo_policy: crate::UndoPolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_hash: Option<crate::ids::PayloadHash>,
    /// @emoji 🗣️ `"<doc-schema>#<kind>"` once the authoring mutation implements [`SemanticMutation`]
    /// — additive, `None` for mutations still on generic vocabulary. Populated by callers (store
    /// replay tightens this once a store's `Mutation: SemanticMutation<P>`, at the final ratchet);
    /// this crate never derives it implicitly to avoid a premature trait-bound change here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_kind: Option<crate::ids::SchemaId>,
    /// @emoji 🏷️ Human undo/history label captured at authoring time (`MutationKind::label`/
    /// `SemanticMutation::label`), so history UI stops reverse-engineering one from `print_op`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// @emoji 🧑‍🤝‍🧑️ Composite-gesture stamp: `Some(id)` when this edit was authored as one member
    /// of a multi-document composite gesture (the future `CompositionCoordinator`'s atomic
    /// parent+child dispatch across several `ArtifactEnvelope`s), so group undo can find and
    /// reverse every sibling member together; `None` for a solitary, single-document edit.
    /// Additive, mirrors `semantic_kind`/`label` above. 🎞️ Wire representation is the bare id
    /// string `semio_framework::kernel::InvocationId` wraps, not that newtype itself: this crate
    /// (`semio-framework-os-kernel`) is a dependency `semio-framework` builds on (see
    /// `📦️packages/🦀️rust/Cargo.toml`), so importing `InvocationId` here would invert that edge —
    /// see `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave1-reports/b1-spr-vcs-report.md` for the
    /// decision record. Callers holding a real `InvocationId` pass `.0`; this field round-trips
    /// through `Edit.mutation_meta` into the `.spr` history log (`HistoryOpMeta.group_id`,
    /// `📡️spr/📜️history/🦀️.rs`) so it survives persistence and sync.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// @emoji 🔀️ Provenance of this operation: `Owner` (default, omitted from wire — the
    /// overwhelming common case, an edit authored by the artifact's own owner logic), `Contributed`
    /// (synthesized by a contributor plugin's `contributor.artifact-mutation-plan` response), or
    /// `Transaction` (applied as one member of a cross-artifact composite gesture initiated
    /// elsewhere). Additive, mirrors `group_id`/`semantic_kind`/`label` above — see
    /// `.🦑️repo/🎫️tickets/26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS/📋️contract-freeze.md`
    /// §1.
    #[serde(default, skip_serializing_if = "MutationOrigin::is_owner")]
    pub origin: MutationOrigin,
}

/// 🌉️ Hand-written, not derived — same DAG reason as the other replication-crate types above.
/// No `#[serde(rename_all = ...)]` on this struct, so field names stay snake_case (unlike
/// `ForeignTarget`/`MutationOrigin` above); mirrors every `skip_serializing_if` by hand.
impl crate::value::ToValue for MutationMeta {
    fn to_value(&self) -> crate::value::DslValue {
        let mut entries = Vec::new();
        if self.mutation_id.is_some() {
            entries.push(("mutation_id".to_string(), crate::value::ToValue::to_value(&self.mutation_id)));
        }
        if !self.dependencies.is_empty() {
            entries.push(("dependencies".to_string(), crate::value::ToValue::to_value(&self.dependencies)));
        }
        entries.push(("base_version".to_string(), crate::value::ToValue::to_value(&self.base_version)));
        if self.author_id.is_some() {
            entries.push(("author_id".to_string(), crate::value::ToValue::to_value(&self.author_id)));
        }
        entries.push(("timestamp".to_string(), crate::value::ToValue::to_value(&self.timestamp)));
        entries.push(("undo_policy".to_string(), crate::value::ToValue::to_value(&self.undo_policy)));
        if self.payload_hash.is_some() {
            entries.push(("payload_hash".to_string(), crate::value::ToValue::to_value(&self.payload_hash)));
        }
        if self.semantic_kind.is_some() {
            entries.push(("semantic_kind".to_string(), crate::value::ToValue::to_value(&self.semantic_kind)));
        }
        if self.label.is_some() {
            entries.push(("label".to_string(), crate::value::ToValue::to_value(&self.label)));
        }
        if self.group_id.is_some() {
            entries.push(("group_id".to_string(), crate::value::ToValue::to_value(&self.group_id)));
        }
        if !self.origin.is_owner() {
            entries.push(("origin".to_string(), crate::value::ToValue::to_value(&self.origin)));
        }
        crate::value::DslValue::object(entries)
    }
}
impl crate::value::FromValue for MutationMeta {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for MutationMeta, found {value:?}")));
        };
        let mut mutation_id = None;
        let mut dependencies = Vec::new();
        let mut base_version = None;
        let mut author_id = None;
        let mut timestamp = None;
        let mut undo_policy = None;
        let mut payload_hash = None;
        let mut semantic_kind = None;
        let mut label = None;
        let mut group_id = None;
        let mut origin = MutationOrigin::Owner;
        for (key, entry) in fields {
            match key.as_str() {
                "mutation_id" => mutation_id = <Option<crate::ids::MutationId> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("mutation_id"))?,
                "dependencies" => dependencies = <Vec<crate::ids::MutationId> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("dependencies"))?,
                "base_version" => base_version = Some(<u64 as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("base_version"))?),
                "author_id" => author_id = <Option<crate::ids::ActorId> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("author_id"))?,
                "timestamp" => timestamp = Some(<crate::ids::HybridLogicalTimestamp as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("timestamp"))?),
                "undo_policy" => undo_policy = Some(<crate::UndoPolicy as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("undo_policy"))?),
                "payload_hash" => payload_hash = <Option<crate::ids::PayloadHash> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("payload_hash"))?,
                "semantic_kind" => semantic_kind = <Option<crate::ids::SchemaId> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("semantic_kind"))?,
                "label" => label = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("label"))?,
                "group_id" => group_id = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("group_id"))?,
                "origin" => origin = <MutationOrigin as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("origin"))?,
                _ => {}
            }
        }
        Ok(MutationMeta {
            mutation_id,
            dependencies,
            base_version: base_version.ok_or_else(|| crate::value::ValueError::new("MutationMeta missing base_version"))?,
            author_id,
            timestamp: timestamp.ok_or_else(|| crate::value::ValueError::new("MutationMeta missing timestamp"))?,
            undo_policy: undo_policy.ok_or_else(|| crate::value::ValueError::new("MutationMeta missing undo_policy"))?,
            payload_hash,
            semantic_kind,
            label,
            group_id,
            origin,
        })
    }
}

/// @emoji 📝️ One coalesced batch of operations, forward and backward, plus their causal metadata.
/// Moved verbatim from `crate::os_store::Edit` (was `vcs/rs/lib.rs` L73).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edit<Op> {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    pub forwards: Vec<Op>,
    pub inverse: Vec<Op>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mutation_meta: Vec<MutationMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coalesce_key: Option<String>,
    pub sequence_number: i32,
    pub started_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
}

/// 🌉️ Hand-written, not derived — same DAG reason as the other replication-crate types above (and
/// the actual trigger of the os-kernel cascade: `CursorRevisionAccumulator::edit_digest`/
/// `reconcile_stack` already bound `Mutation: ToValue` and hash via `to_json_string`, which needs
/// `Edit<Mutation>: ToValue` itself, not just its generic parameter). Mirrors
/// `#[serde(rename_all = "camelCase")]` field naming and every `skip_serializing_if`; `Option<T>`
/// fields with no explicit `#[serde(default)]` (`description`/`finished_at`) still default to
/// `None` when absent — serde's built-in behavior for `Option<T>` fields, mirrored here too.
impl<Op: crate::value::ToValue> crate::value::ToValue for Edit<Op> {
    fn to_value(&self) -> crate::value::DslValue {
        let mut entries = vec![("id".to_string(), crate::value::ToValue::to_value(&self.id))];
        if self.actor.is_some() {
            entries.push(("actor".to_string(), crate::value::ToValue::to_value(&self.actor)));
        }
        entries.push(("forwards".to_string(), crate::value::ToValue::to_value(&self.forwards)));
        entries.push(("inverse".to_string(), crate::value::ToValue::to_value(&self.inverse)));
        if !self.mutation_meta.is_empty() {
            entries.push(("mutationMeta".to_string(), crate::value::ToValue::to_value(&self.mutation_meta)));
        }
        if self.description.is_some() {
            entries.push(("description".to_string(), crate::value::ToValue::to_value(&self.description)));
        }
        if self.coalesce_key.is_some() {
            entries.push(("coalesceKey".to_string(), crate::value::ToValue::to_value(&self.coalesce_key)));
        }
        entries.push(("sequenceNumber".to_string(), crate::value::ToValue::to_value(&self.sequence_number)));
        entries.push(("startedAt".to_string(), crate::value::ToValue::to_value(&self.started_at)));
        if self.finished_at.is_some() {
            entries.push(("finishedAt".to_string(), crate::value::ToValue::to_value(&self.finished_at)));
        }
        crate::value::DslValue::object(entries)
    }
}
impl<Op: crate::value::FromValue> crate::value::FromValue for Edit<Op> {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for Edit, found {value:?}")));
        };
        let mut id = None;
        let mut actor = None;
        let mut forwards = None;
        let mut inverse = None;
        let mut mutation_meta = Vec::new();
        let mut description = None;
        let mut coalesce_key = None;
        let mut sequence_number = None;
        let mut started_at = None;
        let mut finished_at = None;
        for (key, entry) in fields {
            match key.as_str() {
                "id" => id = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("id"))?),
                "actor" => actor = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("actor"))?,
                "forwards" => forwards = Some(<Vec<Op> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("forwards"))?),
                "inverse" => inverse = Some(<Vec<Op> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("inverse"))?),
                "mutationMeta" => mutation_meta = <Vec<MutationMeta> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("mutationMeta"))?,
                "description" => description = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("description"))?,
                "coalesceKey" => coalesce_key = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("coalesceKey"))?,
                "sequenceNumber" => sequence_number = Some(<i32 as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("sequenceNumber"))?),
                "startedAt" => started_at = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("startedAt"))?),
                "finishedAt" => finished_at = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("finishedAt"))?,
                _ => {}
            }
        }
        Ok(Edit {
            id: id.ok_or_else(|| crate::value::ValueError::new("Edit missing id"))?,
            actor,
            forwards: forwards.ok_or_else(|| crate::value::ValueError::new("Edit missing forwards"))?,
            inverse: inverse.ok_or_else(|| crate::value::ValueError::new("Edit missing inverse"))?,
            mutation_meta,
            description,
            coalesce_key,
            sequence_number: sequence_number.ok_or_else(|| crate::value::ValueError::new("Edit missing sequenceNumber"))?,
            started_at: started_at.ok_or_else(|| crate::value::ValueError::new("Edit missing startedAt"))?,
            finished_at,
        })
    }
}
//#endregion 🔖️Meta

//#region 🔖️Origin
/// @emoji 🔀️ Provenance of one [`MutationMeta`]-described operation. `Owner` is the default (and
/// the overwhelming common case), omitted from wire by `MutationMeta.origin`'s
/// `skip_serializing_if`.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum MutationOrigin {
    #[default]
    Owner,
    Contributed {
        plugin_id: String,
        mutation_id: crate::ids::SchemaId,
        payload_hash: crate::ids::PayloadHash,
    },
    Transaction {
        initiator: ForeignTarget,
    },
}
impl MutationOrigin {
    /// 🕳️ Whether this is the (wire-omitted) default owner origin.
    // 🚫️async: E1 — called by name from `#[serde(skip_serializing_if = "...")]`, whose generated
    // call site is sync.
    pub fn is_owner(&self) -> bool {
        matches!(self, Self::Owner)
    }
}

/// 🌉️ Hand-written, not derived — same DAG reason as the other replication-crate types above.
/// `#[serde(tag = "kind")]` internal tagging has no adjacently-tagged (`tag` + `content`) shape the
/// derive macro supports anyway, so this was always going to be hand-written; mirrors the wire
/// shape (`kind` sibling to each struct variant's own fields) exactly.
impl crate::value::ToValue for MutationOrigin {
    fn to_value(&self) -> crate::value::DslValue {
        match self {
            MutationOrigin::Owner => crate::value::DslValue::object([("kind".to_string(), crate::value::DslValue::String("owner".to_string()))]),
            MutationOrigin::Contributed { plugin_id, mutation_id, payload_hash } => crate::value::DslValue::object([
                ("kind".to_string(), crate::value::DslValue::String("contributed".to_string())),
                ("pluginId".to_string(), crate::value::ToValue::to_value(plugin_id)),
                ("mutationId".to_string(), crate::value::ToValue::to_value(mutation_id)),
                ("payloadHash".to_string(), crate::value::ToValue::to_value(payload_hash)),
            ]),
            MutationOrigin::Transaction { initiator } => crate::value::DslValue::object([
                ("kind".to_string(), crate::value::DslValue::String("transaction".to_string())),
                ("initiator".to_string(), crate::value::ToValue::to_value(initiator)),
            ]),
        }
    }
}
impl crate::value::FromValue for MutationOrigin {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for MutationOrigin, found {value:?}")));
        };
        let mut kind = None;
        let mut plugin_id = None;
        let mut mutation_id = None;
        let mut payload_hash = None;
        let mut initiator = None;
        for (key, entry) in fields {
            match key.as_str() {
                "kind" => kind = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("kind"))?),
                "pluginId" => plugin_id = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("pluginId"))?),
                "mutationId" => mutation_id = Some(<crate::ids::SchemaId as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("mutationId"))?),
                "payloadHash" => payload_hash = Some(<crate::ids::PayloadHash as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("payloadHash"))?),
                "initiator" => initiator = Some(<ForeignTarget as crate::value::FromValue>::from_value(entry).map_err(|error| error.under("initiator"))?),
                _ => {}
            }
        }
        match kind.as_deref() {
            Some("owner") => Ok(MutationOrigin::Owner),
            Some("contributed") => Ok(MutationOrigin::Contributed {
                plugin_id: plugin_id.ok_or_else(|| crate::value::ValueError::new("MutationOrigin::Contributed missing pluginId"))?,
                mutation_id: mutation_id.ok_or_else(|| crate::value::ValueError::new("MutationOrigin::Contributed missing mutationId"))?,
                payload_hash: payload_hash.ok_or_else(|| crate::value::ValueError::new("MutationOrigin::Contributed missing payloadHash"))?,
            }),
            Some("transaction") => Ok(MutationOrigin::Transaction {
                initiator: initiator.ok_or_else(|| crate::value::ValueError::new("MutationOrigin::Transaction missing initiator"))?,
            }),
            Some(other) => Err(crate::value::ValueError::new(format!("unknown MutationOrigin kind `{other}`"))),
            None => Err(crate::value::ValueError::new("MutationOrigin missing kind")),
        }
    }
}
//#endregion 🔖️Origin
