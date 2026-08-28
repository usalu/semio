//! 🎞️ Protocol command/collaboration-semantics layer: the `Mutation`/`MutationDiff`/`OpText`
//! trait family, `MutationMeta`/`Edit`, generic collection-operation plumbing, the runtime
//! `MutationDescriptor` registry, the upcast seam, and the five-channel `CommandOutcome`. Moved
//! (with two defaulted trait methods and a `reconcile` return-type change, both called out inline)
//! from `vcs/rs/lib.rs`'s `🔖️Mutation`/`🔖️CollectionDiff`/`🔖️CollectionMutation` regions and
//! `framework/core`. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` `## Amendment` §`protocol_command`.
//!
//! Op payloads stay schema-opaque here exactly like `protocol_history`: this crate never parses or
//! interprets an `Op`'s fields, only threads it through the trait seams a technology implements.

//#region 🔖️Contract
/// 🎞️ The mutation contract itself lives in `protocol::mutation` (framework module
/// `📡️replication`); this authoring layer builds on it and re-exports it so every historical
/// `command::Mutation`/`Edit`/`MutationMessage` path keeps resolving.
pub use protocol::mutation::*;
//#endregion 🔖️Contract

//#region 🔖️Inference
/// @emoji 💡️ All information inferable from a snapshot — the fourth schema family alongside
/// `Snapshot`/`Diff`/`Mutation` (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
/// LAWS: pure (reads only `snapshot`), deterministic (equal snapshots ⇒ byte-equal canonical
/// serializations of the result), total (never panics, never fails — an inference with error
/// states models them as data, not as a `Result`). `infer` is THE single semantics source: every
/// cache path in `crate::os_inference` must be observationally identical to calling this directly.
pub trait Inference<P>: Clone + Default + serde::Serialize + serde::de::DeserializeOwned {
    fn infer(snapshot: &P) -> Self;
}

/// @emoji 🗺️ Region vocabulary shared by [`DiffRegions::touches`] and an [`InferenceFieldSpec`]'s
/// `reads`. Paths are `/`-joined segments (e.g. `"objects/o1/vortices"`); two paths "intersect" when
/// one's segments are a prefix of the other's (either direction), matching how a coarse write region
/// (`"objects"`) covers every finer read region beneath it (`"objects/o1/vortices"`) and vice versa.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TouchedPaths {
    pub paths: Vec<String>,
}

impl TouchedPaths {
    /// 🏗️ Builds a `TouchedPaths` from plain `&str` path segments.
    // 🚫️async: E1 pure accessor consumed inside a sync std Iterator closure — see R9
    pub fn new(paths: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { paths: paths.into_iter().map(Into::into).collect() }
    }

    // 🚫️async: E1 pure accessor consumed inside a sync std Iterator closure — see R9
    fn segments(path: &str) -> Vec<&str> {
        path.split('/').filter(|segment| !segment.is_empty()).collect()
    }

    /// 🔀️ Whether any of `self.paths` shares an ancestor/descendant relationship with `prefix`.
    // 🚫️async: E1 pure accessor consumed inside a sync std Iterator closure — see R9
    pub fn intersects_prefix(&self, prefix: &str) -> bool {
        let target = Self::segments(prefix);
        self.paths.iter().any(|path| {
            let own = Self::segments(path);
            let shared = own.len().min(target.len());
            own[..shared] == target[..shared]
        })
    }

    /// 🔀️ Whether any of `prefixes` intersects `self` (see [`intersects_prefix`](Self::intersects_prefix)).
    // 🚫️async: E1 pure accessor consumed inside a sync std Iterator closure — see R9
    pub fn intersects_any(&self, prefixes: &[&str]) -> bool {
        prefixes.iter().any(|prefix| self.intersects_prefix(prefix))
    }
}

/// @emoji 🗺️ Write-region coverage of a diff — the diff→invalidation bridge for inference's tier-1
/// gate. Deliberately a SEPARATE trait from [`MutationDiff`] (not a new method on it), following the
/// same seeded-shrink-only rollout as [`DiffAlgebra`] above: land standalone here, adopt per-type via
/// a `POLICY_DIFF_REGIONS` allowlist, never as a hard bound on `MutationDiff` until every implementor
/// is covered.
/// LAW (coverage soundness): any snapshot region whose value differs between `base` and
/// `self.apply(base)` is covered by some path in `touches()`. Over-approximation is legal (costs an
/// extra recompute), under-approximation is a correctness bug — a stale cached inference value.
pub trait DiffRegions {
    fn touches(&self) -> TouchedPaths;
}

/// @emoji 🕸️ One named inferred field family and its declared snapshot read-set (the tier-1 gate
/// [`DiffRegions::touches`] is checked against).
#[derive(Clone, Copy, Debug)]
pub struct InferenceFieldSpec {
    pub id: &'static str,
    pub reads: &'static [&'static str],
}

/// @emoji 🧬️ Registrable metadata for an artifact's inference family — twin of `ProjectionClass`'s
/// `id`/`schema_version`/`reads` trio (`crate::os_db::projection`), but static: one impl per `XInference`
/// type, declared next to its `Inference` impl.
pub trait InferenceSpec<P>: Inference<P> {
    fn inference_schema_id() -> &'static str;
    /// 🔢️ Salts every cache key derived from this spec's fields — bump when the derivation
    /// algorithm changes so a warm cache never serves a value computed under the old algorithm.
    fn schema_version() -> u32;
    fn fields() -> &'static [InferenceFieldSpec];
}
//#endregion 🔖️Inference

//#region 🔖️Semantics
/// 📗️ Closed imperative verbs and past-tense records for concrete mutation identities.
/// History operations commit checkpoints, switch alternatives, and restore prior selections.
/// Generic snapshot replacement and absence sentinels remain outside this vocabulary.
pub const APPROVED_VERBS: &[(&str, &str)] = &[
    ("add", "Added"),
    ("append", "Appended"),
    ("apply", "Applied"),
    ("bind", "Bound"),
    ("change", "Changed"),
    ("clear", "Cleared"),
    ("commit", "Committed"),
    ("connect", "Connected"),
    ("create", "Created"),
    ("delete", "Deleted"),
    ("disconnect", "Disconnected"),
    ("drag", "Dragged"),
    ("duplicate", "Duplicated"),
    ("edit", "Edited"),
    ("extract", "Extracted"),
    ("finish", "Finished"),
    ("fix", "Fixed"),
    ("flatten", "Flattened"),
    ("group", "Grouped"),
    ("inline", "Inlined"),
    ("insert", "Inserted"),
    ("merge", "Merged"),
    ("move", "Moved"),
    ("remove", "Removed"),
    ("rename", "Renamed"),
    ("reorder", "Reordered"),
    ("replace", "Replaced"),
    ("resize", "Resized"),
    ("restore", "Restored"),
    ("rotate", "Rotated"),
    ("scale", "Scaled"),
    ("seal", "Sealed"),
    ("set", "Set"),
    ("split", "Split"),
    ("start", "Started"),
    ("switch", "Switched"),
    ("toggle", "Toggled"),
    ("unbind", "Unbound"),
    ("unflatten", "Unflattened"),
    ("ungroup", "Ungrouped"),
    ("update", "Updated"),
];

/// @emoji 🔤️ `const`-context string equality (stable `&str: PartialEq` isn't usable in a `const`
/// assertion) — used by `#[derive(Mutations)]`'s generated `SEMANTICS.kind == kebab(variant)` check.
pub const fn str_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// @emoji ✅️ `const`-context membership check against [`APPROVED_VERBS`] — used by
/// `#[derive(Mutations)]`'s generated compile-time assertion so an unapproved verb is a build
/// error, not a policy-scan finding discovered later.
pub const fn is_approved_verb(verb: &str) -> bool {
    let mut i = 0;
    while i < APPROVED_VERBS.len() {
        if str_eq(APPROVED_VERBS[i].0, verb) {
            return true;
        }
        i += 1;
    }
    false
}

/// @emoji 🪧️ Compile-time semantic identity of one handcrafted mutation kind — one `&'static`
/// instance per `🧬️mutations/<kind>/` triad dir, declared as `MutationKind::SEMANTICS` on the
/// kind's payload struct. `kind` MUST equal the triad dir stem (emoji stripped) and the kebab of
/// the dispatch-enum variant wrapping this payload (`#[derive(Mutations)]` asserts this at
/// compile time); `record` is the past-tense name a future operation-log/GraphQL layer exposes
/// (golden schema precedent: `RenamedPiece`, `FlattenedDesign`) — imperative naming is the only
/// naming system in Rust code, past tense lives here and nowhere else.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub struct SemanticDescriptor {
    pub verb: &'static str,
    pub entity: &'static str,
    pub kind: &'static str,
    pub record: &'static str,
}

//#region 🪪️MutationLeafDescriptor
/// 🪞️ Reexports the lower mutation metadata contract through the public OS command façade.
pub use protocol::mutation::{
    validate_mutation_leaf_descriptor, validate_mutation_leaf_descriptor_roster, validate_mutation_leaf_source, MutationComposition, MutationDiffParticipation, MutationInvertibility, MutationLanguageSurface, MutationLeaf,
    MutationLeafDescriptor, MutationLeafDescriptorRosterValidationError, MutationLeafDescriptorValidationError, MutationLeafSourceScope, MutationLeafSourceValidationError, MutationOutcomeClass, MutationSourceProvenance,
};
//#endregion 🪪️MutationLeafDescriptor

/// 🦠️ One direct mutation leaf with mandatory source-derived metadata and handcrafted behavior.
/// `Op` wraps the owner's concrete leaves; inverses may select a different leaf from that roster.
pub trait MutationKind<P, Op>: MutationLeaf + Clone + serde::Serialize + serde::de::DeserializeOwned
where
    Op: Mutation<P>,
{
    const SEMANTICS: SemanticDescriptor;

    fn diff(&self, base: &P) -> MutationOutcome<<Op as Mutation<P>>::Diff>;
    /// Missing/already-absent target ⇒ `Vec::new()` (the semantic replacement for the old
    /// `NoMutation` sentinel variant — there is no "no-op mutation", only an inverse with nothing
    /// to undo).
    fn inverse(&self, base: &P) -> Vec<Op>;
    /// @emoji 🏷️ Human undo/history label, e.g. `Rename piece "a" to "b"`.
    fn label(&self) -> String;
    /// ⏱️ Returns the authored clock, or absence when this leaf does not carry one.
    fn timestamp(&self) -> Option<protocol::ids::HybridLogicalTimestamp> {
        None
    }
    /// @emoji 🎯️ Structured address of the target inside the artifact (outermost segment first);
    /// empty means whole-artifact scope.
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
    /// @emoji 🌐️ Whether this kind can ever emit cross-artifact transaction steps.
    fn may_emit_foreign_steps(&self) -> bool {
        false
    }
    /// @emoji 🌐️ Foreign steps this kind additionally dispatches to OTHER artifacts. Defaults to
    /// `Vec::new()` so no existing handcrafted `impl MutationKind` breaks; `#[derive(Mutations)]`
    /// gains a per-variant delegating arm (see `🗣️dsl/✨️derive/🦀️component.rs` `🔖️Mutations`).
    fn foreign_steps(&self, _base: &P) -> Vec<ForeignStep> {
        Vec::new()
    }
}

/// @emoji 🗣️ Refinement of [`Mutation`] for an enum whose every variant is a [`MutationKind`].
/// Implemented only by `#[derive(Mutations)]`, never by hand. End-state (final ratchet, once every
/// artifact's dispatch enum implements it): `ArtifactApp`/`ArtifactStore` bounds tighten from
/// `Mutation` to `SemanticMutation`, making semantic vocabulary the only expressible one at
/// compile time — see `.claude/plans/the-mutations-are-extremely-compiled-pumpkin.md`.
pub trait SemanticMutation<P>: Mutation<P> {
    /// This artifact's full kind table, one row per variant — registration/introspection source.
    fn kinds() -> &'static [SemanticDescriptor];
    fn semantics(&self) -> &'static SemanticDescriptor;
    fn label(&self) -> String;
    fn target(&self) -> Vec<String>;
}
//#endregion 🔖️Semantics

//#region 🔖️Collection
/// 🧬️ Collection identity/patch/diff/ops — single source of truth in VCS (`crate::os_vcs`).
pub use crate::os_vcs::{apply_collection_mutation, collection_diff_from_mutation, inverse_collection_mutation, CollectionDiff, CollectionMutation, Identified, ItemPatch, Patchable};

//#endregion 🔖️Collection

//#region 🔖️DiffKit
/// @emoji 🗃️ Id-keyed collection delta shape — the shared type behind a technology's per-collection
/// diff fragment, replacing hand-copied `NamedTripleDiff`-shaped structs (6 copies pre-overhaul).
/// `K` = item id, `V` = full item, `Patch` = per-item sparse patch (`Patchable<Patch>`). Deliberately
/// provides only [`named_apply`] here — `absorb`/`inverse`/`between` stay handcrafted per artifact
/// (via [`DiffAlgebra`]/[`MutationDiff`]) because their correct semantics depend on how that
/// artifact's `Patch` type composes, which this crate cannot know generically.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedTripleDiff<K, V, Patch> {
    pub removed: Vec<K>,
    pub modified: Vec<ItemPatch<K, Patch>>,
    pub added: Vec<V>,
}

impl<K, V, Patch> Default for NamedTripleDiff<K, V, Patch> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

impl<K, V, Patch> NamedTripleDiff<K, V, Patch> {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// @emoji ▶️ Validates and applies a [`NamedTripleDiff`] to an id-keyed `Vec` in place:
/// removals, then patches, then appends. Validation is completed before the first write, so a
/// missing/duplicate/contradictory persisted target rejects the whole diff atomically.
pub fn named_apply<K, V, Patch>(items: &mut Vec<V>, diff: &NamedTripleDiff<K, V, Patch>) -> Result<(), MutationApplyError>
where
    K: PartialEq,
    V: Clone + Identified<K> + Patchable<Patch>,
{
    for (index, id) in diff.removed.iter().enumerate() {
        if !items.iter().any(|item| item.id() == id) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "removed item does not exist").at(["removed".to_string(), index.to_string()]));
        }
        if diff.removed[..index].iter().any(|previous| previous == id) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "item is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
    }
    for (index, item_patch) in diff.modified.iter().enumerate() {
        if !items.iter().any(|item| item.id() == &item_patch.id) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "modified item does not exist").at(["modified".to_string(), index.to_string()]));
        }
        if diff.removed.iter().any(|id| id == &item_patch.id) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "item cannot be removed and modified by the same diff").at(["modified".to_string(), index.to_string()]));
        }
        if diff.modified[..index].iter().any(|previous| previous.id == item_patch.id) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "item is modified more than once").at(["modified".to_string(), index.to_string()]));
        }
    }
    for (index, added) in diff.added.iter().enumerate() {
        if items.iter().any(|item| item.id() == added.id()) || diff.added[..index].iter().any(|previous| previous.id() == added.id()) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "added item identity already exists").at(["added".to_string(), index.to_string()]));
        }
    }
    let mut candidate = items.clone();
    candidate.retain(|item| !diff.removed.iter().any(|id| item.id() == id));
    for item_patch in &diff.modified {
        let item = match candidate.iter_mut().find(|item| item.id() == &item_patch.id) {
            Some(item) => item,
            None => {
                return Err(MutationApplyError::new("mutation.apply.conflicting-target", "an earlier patch changed a later target's identity").at(["modified"]));
            }
        };
        item.apply_patch(&item_patch.patch);
    }
    candidate.extend(diff.added.iter().cloned());
    *items = candidate;
    Ok(())
}

/// @emoji 🗃️ Index-keyed ordered-collection delta shape — the shared type behind an intrinsically
/// ordered, id-less collection's diff fragment (pptx slides/shapes, paragraphs, table rows),
/// replacing hand-copied `IndexedTripleDiff`-shaped structs (4 copies pre-overhaul). Index
/// convention (owned by the artifact's handcrafted `MutationKind::diff`, [`indexed_apply`] just
/// enforces it): `removed`/`modified` indices are BASE-state; `added` indices are FINAL-state.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexedTripleDiff<V, Patch> {
    pub removed: Vec<usize>,
    pub modified: Vec<(usize, Patch)>,
    pub added: Vec<(usize, V)>,
}

impl<V, Patch> Default for IndexedTripleDiff<V, Patch> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

impl<V, Patch> IndexedTripleDiff<V, Patch> {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// @emoji ▶️ Validates and applies an [`IndexedTripleDiff`] in place: BASE-state `modified`
/// patches first, BASE-state `removed` descending, then FINAL-state `added` ascending. Every
/// index is exact; out-of-range and duplicate indices reject atomically instead of clamping.
pub fn indexed_apply<V, Patch>(items: &mut Vec<V>, diff: &IndexedTripleDiff<V, Patch>) -> Result<(), MutationApplyError>
where
    V: Clone + Patchable<Patch>,
{
    for (position, (index, _)) in diff.modified.iter().enumerate() {
        if *index >= items.len() {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", format!("modified base index {index} is out of range for length {}", items.len())).at(["modified".to_string(), position.to_string()]));
        }
        if diff.modified[..position].iter().any(|(previous, _)| previous == index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", format!("base index {index} is modified more than once")).at(["modified".to_string(), position.to_string()]));
        }
        if diff.removed.contains(index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", format!("base index {index} cannot be removed and modified by the same diff")).at(["modified".to_string(), position.to_string()]));
        }
    }
    for (position, index) in diff.removed.iter().enumerate() {
        if *index >= items.len() {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", format!("removed base index {index} is out of range for length {}", items.len())).at(["removed".to_string(), position.to_string()]));
        }
        if diff.removed[..position].contains(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", format!("base index {index} is removed more than once")).at(["removed".to_string(), position.to_string()]));
        }
    }
    let mut added: Vec<(usize, &(usize, V))> = diff.added.iter().enumerate().collect();
    added.sort_unstable_by_key(|(_, (index, _))| *index);
    let mut next_len = items.len() - diff.removed.len();
    for (ordinal, (position, (index, _))) in added.iter().enumerate() {
        if ordinal > 0 && added[ordinal - 1].1 .0 == *index {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", format!("final index {index} is added more than once")).at(["added".to_string(), position.to_string()]));
        }
        if *index > next_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", format!("added final index {index} is out of range for length {next_len}")).at(["added".to_string(), position.to_string()]));
        }
        next_len += 1;
    }
    for (index, patch) in &diff.modified {
        items[*index].apply_patch(patch);
    }
    let mut removed: Vec<usize> = diff.removed.clone();
    removed.sort_unstable_by(|a, b| b.cmp(a));
    for index in removed {
        items.remove(index);
    }
    for (_, (index, value)) in added {
        items.insert(*index, value.clone());
    }
    Ok(())
}
//#endregion 🔖️DiffKit

//#region 🔖️Descriptor
/// 🪪️ Immutable schema, state and complete leaf identity registered for one mutation kind.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationDescriptor {
    id: crate::os_spr::ids::SchemaId,
    schema_version: crate::os_spr::ids::SchemaVersion,
    state_class: crate::os_spr::StateClass,
    leaf: MutationLeafDescriptor,
    semantics: SemanticDescriptor,
    fingerprint: [u8; 32],
}

impl MutationDescriptor {
    /// 🏗️ Validates all required metadata and fingerprints the complete immutable identity.
    pub fn new(
        id: crate::os_spr::ids::SchemaId,
        schema_version: crate::os_spr::ids::SchemaVersion,
        state_class: crate::os_spr::StateClass,
        leaf: MutationLeafDescriptor,
        semantics: SemanticDescriptor,
    ) -> Result<Self, MutationDescriptorError> {
        if id.0.trim().is_empty() {
            return Err(MutationDescriptorError::InvalidField { field: "id", requirement: "must be nonblank" });
        }
        if schema_version.0 == 0 {
            return Err(MutationDescriptorError::InvalidField { field: "schemaVersion", requirement: "must be positive" });
        }
        leaf.validate().map_err(|error| MutationDescriptorError::InvalidField { field: error.field, requirement: error.requirement })?;
        if !is_approved_verb(semantics.verb) {
            return Err(MutationDescriptorError::InvalidField { field: "semantics.verb", requirement: "must be an approved imperative verb" });
        }
        if semantics.entity.trim().is_empty() || semantics.record.trim().is_empty() {
            return Err(MutationDescriptorError::InvalidField { field: "semantics", requirement: "entity and record must be nonblank" });
        }
        if leaf.semantic_kind != semantics.kind {
            return Err(MutationDescriptorError::InvalidField { field: "semantics.kind", requirement: "must equal the leaf semantic kind" });
        }
        let fingerprint = descriptor_fingerprint(&id, schema_version, state_class, &leaf, &semantics);
        Ok(Self { id, schema_version, state_class, leaf, semantics, fingerprint })
    }

    pub fn id(&self) -> &crate::os_spr::ids::SchemaId { &self.id }
    pub fn schema_version(&self) -> crate::os_spr::ids::SchemaVersion { self.schema_version }
    pub fn state_class(&self) -> crate::os_spr::StateClass { self.state_class }
    pub fn leaf(&self) -> &MutationLeafDescriptor { &self.leaf }
    pub fn semantics(&self) -> &SemanticDescriptor { &self.semantics }
    pub fn fingerprint(&self) -> &[u8; 32] { &self.fingerprint }
}

fn descriptor_fingerprint(
    id: &crate::os_spr::ids::SchemaId,
    schema_version: crate::os_spr::ids::SchemaVersion,
    state_class: crate::os_spr::StateClass,
    leaf: &MutationLeafDescriptor,
    semantics: &SemanticDescriptor,
) -> [u8; 32] {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Canonical<'a> {
        id: &'a str,
        schema_version: u32,
        state_class: crate::os_spr::StateClass,
        leaf: &'a MutationLeafDescriptor,
        semantics: &'a SemanticDescriptor,
    }
    let canonical = Canonical { id: &id.0, schema_version: schema_version.0, state_class, leaf, semantics };
    let mut bytes = b"semio.mutation-descriptor/v1\0".to_vec();
    bytes.extend(serde_json::to_vec(&canonical).expect("primitive descriptor identity is JSON serializable"));
    semio_framework_hash::Sha256::digest(&bytes)
}

/// 🚫️ Invalid metadata or a conflicting immutable identity; registration never overwrites.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MutationDescriptorError {
    InvalidField { field: &'static str, requirement: &'static str },
    Conflict { id: String, existing_fingerprint: [u8; 32], incoming_fingerprint: [u8; 32] },
}

impl std::fmt::Display for MutationDescriptorError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidField { field, requirement } => write!(formatter, "invalid mutation descriptor {field}: {requirement}"),
            Self::Conflict { id, .. } => write!(formatter, "conflicting mutation descriptor for {id}"),
        }
    }
}

impl std::error::Error for MutationDescriptorError {}

/// 🗂️ One owned registry with equality-based idempotence and atomic batch admission.
#[derive(Debug, Default)]
pub struct MutationDescriptorRegistry {
    entries: std::collections::HashMap<String, MutationDescriptor>,
}

impl MutationDescriptorRegistry {
    pub fn new() -> Self { Self::default() }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn get(&self, id: &str) -> Option<&MutationDescriptor> { self.entries.get(id) }

    pub fn register(&mut self, descriptor: MutationDescriptor) -> Result<(), MutationDescriptorError> {
        self.register_all([descriptor])
    }

    /// 🧷️ Checks every candidate before inserting any, including duplicates within the batch.
    pub fn register_all(&mut self, descriptors: impl IntoIterator<Item = MutationDescriptor>) -> Result<(), MutationDescriptorError> {
        let mut pending: std::collections::HashMap<String, MutationDescriptor> = std::collections::HashMap::new();
        for descriptor in descriptors {
            if let Some(existing) = self.entries.get(&descriptor.id.0).or_else(|| pending.get(&descriptor.id.0)) {
                if existing != &descriptor {
                    return Err(MutationDescriptorError::Conflict { id: descriptor.id.0.clone(), existing_fingerprint: existing.fingerprint, incoming_fingerprint: descriptor.fingerprint });
                }
            } else {
                pending.insert(descriptor.id.0.clone(), descriptor);
            }
        }
        self.entries.extend(pending);
        Ok(())
    }
}

static MUTATION_DESCRIPTOR_REGISTRY: std::sync::OnceLock<std::sync::RwLock<MutationDescriptorRegistry>> = std::sync::OnceLock::new();

fn mutation_descriptor_registry() -> &'static std::sync::RwLock<MutationDescriptorRegistry> {
    MUTATION_DESCRIPTOR_REGISTRY.get_or_init(|| std::sync::RwLock::new(MutationDescriptorRegistry::new()))
}

/// 📝️ Registers an equal identity idempotently, rejecting a conflicting same-id value.
pub fn register_mutation_descriptor(descriptor: MutationDescriptor) -> Result<(), MutationDescriptorError> {
    register_mutation_descriptors([descriptor])
}

/// 📝️ Atomically registers a complete roster in the process-wide registry.
pub fn register_mutation_descriptors(descriptors: impl IntoIterator<Item = MutationDescriptor>) -> Result<(), MutationDescriptorError> {
    let mut registry = mutation_descriptor_registry().write().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.register_all(descriptors)
}

/// 🔎️ Looks up a complete immutable descriptor registered for a schema id.
pub fn mutation_descriptor(schema: &str) -> Option<MutationDescriptor> {
    let registry = mutation_descriptor_registry().read().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.get(schema).cloned()
}
//#endregion 🔖️Descriptor

//#region 🔖️Upcast
/// @emoji ⬆️ Rewrites an operation authored at an older schema version into today's shape.
/// LAW: `upcast(upcast(x)) == upcast(x)` — idempotence at the target version.
pub trait MutationUpcaster<Op> {
    fn upcast(&self, from_version: crate::os_spr::ids::SchemaVersion, op: Op) -> Op;
}
//#endregion 🔖️Upcast

//#region 🔖️Events
/// @emoji 📡️ One side-effect-channel event emitted alongside a persistent/UI diff.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MutationEvent {
    pub mutation_id: crate::os_spr::ids::MutationId,
    pub state_class: crate::os_spr::StateClass,
    pub payload: serde_json::Value,
}
//#endregion 🔖️Events

//#region 🔖️Outcome
/// @emoji 🗂️ The five-channel separation `framework/core`'s `InvocationResult` later maps onto:
/// durable diffs, two UI-visibility tiers, a speculative preview tier, and side-effect events.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CommandOutcome<Diff> {
    pub persistent: Vec<Diff>,
    pub shared_ui: Vec<Diff>,
    pub local_ui: Vec<Diff>,
    pub preview: Vec<Diff>,
    pub effects: Vec<MutationEvent>,
}
//#endregion 🔖️Outcome

//#region 🔖️Composite

/// @emoji 🪆️ One step of a [`Planner`]'s plan: either a concrete `Op` applied to the composite's
/// own snapshot, or a [`ForeignStep`] dispatched elsewhere.
#[derive(Clone, Debug, PartialEq)]
pub enum PlanStep<Op> {
    Local(Op),
    Foreign(ForeignStep),
}

/// @emoji 🛑️ A composite plan's recursion ceiling — mirrors `MAX_TXN_DEPTH` in the transaction
/// protocol (§5 of the contract freeze), since a `Planner`'s foreign-step chain is exactly what a
/// transaction later replays hop by hop.
pub const MAX_PLAN_DEPTH: u8 = 8;

/// @emoji 🚧️ Typed failure of composite-mutation planning — never a panic, per the purity law on
/// [`CompositeMutationKind::plan`].
#[derive(Clone, Debug, PartialEq)]
pub enum PlanError {
    DepthExceeded(u8),
    Cycle(String),
    StepRejected(String),
    Apply(MutationApplyError),
    Invalid(String),
}

impl std::fmt::Display for PlanError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DepthExceeded(depth) => write!(formatter, "plan depth {depth} exceeds MAX_PLAN_DEPTH"),
            Self::Cycle(id) => write!(formatter, "plan cycle on {id}"),
            Self::StepRejected(detail) => write!(formatter, "step rejected: {detail}"),
            Self::Apply(error) => write!(formatter, "step diff could not be applied: {error}"),
            Self::Invalid(detail) => formatter.write_str(detail),
        }
    }
}

impl std::error::Error for PlanError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Apply(error) => Some(error),
            _ => None,
        }
    }
}

impl From<MutationApplyError> for PlanError {
    fn from(error: MutationApplyError) -> Self {
        Self::Apply(error)
    }
}

/// @emoji 🧮️ Accumulates a [`CompositeMutationKind::plan`]'s steps against a snapshot that starts
/// at `base` and advances by every `call`ed local op — so a later step's `validate`/`diff` sees the
/// snapshot as it would exist after every step planned before it, exactly like sequential apply.
/// Foreign-hop depth/cycle bookkeeping lives here (`call_foreign`), keyed on `(mutation_id,
/// blake3(payload))` — the same pair a replayed `TransactionPrepare` (§5 of the contract freeze)
/// would use to detect a step it has already prepared.
pub struct Planner<P, Op: Mutation<P>> {
    base: P,
    steps: Vec<PlanStep<Op>>,
    pre_states: Vec<Option<P>>,
    depth: u8,
    seen: Vec<(String, [u8; 32])>,
    messages: Vec<MutationMessage>,
}

/// 🎯️ Prefixes `message.target` with `prefix` as its new outermost segment — how [`Planner::call`]
/// attributes each local step's messages back to "the step path" (§C4).
// 🚫️async: R9 pure accessor — only consumer is `Iterator::map`'s sync closure below; no
// suspension point exists in the body either.
fn prefix_message(mut message: MutationMessage, prefix: &str) -> MutationMessage {
    message.target.insert(0, prefix.to_string());
    message
}

impl<P: Clone, Op: Mutation<P>> Planner<P, Op> {
    pub fn new(base: &P) -> Self {
        Self { base: base.clone(), steps: Vec::new(), pre_states: Vec::new(), depth: 0, seen: Vec::new(), messages: Vec::new() }
    }

    /// 🪞️ The snapshot as it stands after every step `call`ed so far.
    pub fn base(&self) -> &P {
        &self.base
    }

    /// 📨️ Every message folded in so far, across every `call`ed step (including the failing one, if
    /// any — a caller reading `Err(PlanError::StepRejected(..))` still finds the `Fatal` message(s)
    /// that caused it here).
    pub fn messages(&self) -> &[MutationMessage] {
        &self.messages
    }

    /// ▶️ Computes `op.diff(base)`, folds its [`MutationMessage`]s (target-prefixed by this step's
    /// index) into `self.messages`, and — unless any message is `Fatal` — advances `base` by the
    /// diff and records `op` as a [`PlanStep::Local`], so the NEXT `call`/`call_foreign` sees the
    /// post-state, matching what sequential application of the resulting plan would do. A `Fatal`
    /// message stops the plan outright (`Err(PlanError::StepRejected)`) without advancing `base`;
    /// a merely `Error`/`Warning`/`Info` message still advances `base` and continues planning — only
    /// [`fold_plan_diff`]'s all-or-nothing fold treats those as poisoning the composite's own diff.
    pub fn call(&mut self, op: Op) -> Result<(), PlanError> {
        let step_index = self.steps.len();
        let (diff, messages) = op.diff(&self.base).into_parts();
        let is_fatal = messages.iter().any(|message| message.level == crate::os_dsl::Severity::Fatal);
        let reason = messages.iter().filter(|message| message.level == crate::os_dsl::Severity::Fatal).map(|message| message.message.clone()).collect::<Vec<_>>().join("; ");
        let prefix = format!("step-{step_index}");
        self.messages.extend(messages.into_iter().map(|message| prefix_message(message, &prefix)));
        if is_fatal {
            return Err(PlanError::StepRejected(reason));
        }
        let pre_state = self.base.clone();
        self.base = diff.apply(&self.base)?;
        self.steps.push(PlanStep::Local(op));
        self.pre_states.push(Some(pre_state));
        Ok(())
    }

    /// 🪜️ Records a hop to another artifact, enforcing [`MAX_PLAN_DEPTH`] and rejecting a repeated
    /// `(mutation_id, payload hash)` pair as a cycle — both typed [`PlanError`]s, never a panic.
    /// Deliberately does NOT advance `base` (a foreign step's effect is on a DIFFERENT snapshot).
    pub fn call_foreign(&mut self, step: ForeignStep) -> Result<(), PlanError> {
        let next_depth = self.depth.checked_add(1).filter(|depth| *depth <= MAX_PLAN_DEPTH).ok_or(PlanError::DepthExceeded(MAX_PLAN_DEPTH))?;
        let key = (step.mutation_id.0.clone(), *blake3::hash(&step.payload).as_bytes());
        if self.seen.contains(&key) {
            return Err(PlanError::Cycle(step.target.artifact_id));
        }
        self.depth = next_depth;
        self.seen.push(key);
        self.steps.push(PlanStep::Foreign(step));
        self.pre_states.push(None);
        Ok(())
    }

    pub fn steps(&self) -> &[PlanStep<Op>] {
        &self.steps
    }

    pub fn into_steps(self) -> Vec<PlanStep<Op>> {
        self.steps
    }

    /// 🧭️ Consumes the plan with each local step's already-validated pre-state.
    fn into_steps_with_pre_states(self) -> (Vec<PlanStep<Op>>, Vec<Option<P>>) {
        (self.steps, self.pre_states)
    }

    /// ➡️ Consumes `self` into its raw `(steps, messages)` parts — [`fold_plan_diff`]'s primitive.
    pub fn into_parts(self) -> (Vec<PlanStep<Op>>, Vec<MutationMessage>) {
        (self.steps, self.messages)
    }
}

/// @emoji 🕸️ A mutation kind whose effect is a PLAN over one-or-more concrete `Op`s (this
/// artifact's own) and/or [`ForeignStep`]s (other artifacts') rather than a single direct diff.
/// Implemented once per composite kind, exactly like [`MutationKind`] is implemented once per
/// handcrafted kind — `#[derive(CompositeMutation)]` (`🗣️dsl/✨️derive/🦀️component.rs`
/// `🔖️CompositeMutation`) wires the delegating `MutationKind` impl from it via the free helpers
/// below.
///
/// LAW (purity): `plan` reads only `base` (never mutates outside state) and drives `planner`
/// exclusively through `call`/`call_foreign` — the free helpers below are the ONLY supported way to
/// fold a plan into a diff/inverse/foreign-step list, so every composite kind observes identical
/// semantics regardless of who calls it.
pub trait CompositeMutationKind<P, Op: Mutation<P>>: MutationLeaf + Clone + serde::Serialize + serde::de::DeserializeOwned {
    const SEMANTICS: SemanticDescriptor;
    fn plan(&self, base: &P, planner: &mut Planner<P, Op>) -> Result<(), PlanError>;
    fn label(&self) -> String;
    /// ⏱️ Returns only the clock explicitly carried by this composite payload.
    fn timestamp(&self) -> Option<protocol::ids::HybridLogicalTimestamp> {
        None
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

/// @emoji 🏗️ Runs `kind.plan` against a fresh [`Planner`] seeded at `base`. NOT a blanket
/// `impl<T: CompositeMutationKind> MutationKind for T` — coherence rejects that against the ~200
/// concrete `impl MutationKind` in the tree — so every other free helper here, and the
/// `#[derive(CompositeMutation)]` delegation, is built on top of this one instead.
pub fn plan_of<P: Clone, Op: Mutation<P>, K: CompositeMutationKind<P, Op>>(kind: &K, base: &P) -> Result<Vec<PlanStep<Op>>, PlanError> {
    let mut planner = Planner::new(base);
    kind.plan(base, &mut planner)?;
    Ok(planner.into_steps())
}

/// @emoji 🧬️ Folds a composite's LOCAL steps into one [`MutationOutcome`] via
/// [`MutationDiff::absorb`], applying each step against the snapshot as it stood right before that
/// step (matching [`Planner::call`]'s own advance-as-you-go semantics) — so a successful
/// `fold_plan_diff(k, b).diff().apply(&b)` equals sequential application of the plan's local steps.
/// Foreign steps never contribute to the folded diff (LAW 5 of the contract freeze). **All-or-
/// nothing** (§C4): if planning itself fails (`PlanError`) or any step's messages reach `Error` or
/// worse, the returned diff is empty (`Default::default()`) — but every message collected along the
/// way is still kept, so a caller sees exactly why. A `PlanError` additionally contributes one
/// `Fatal` `"mutation.invariant"` message. Never panics, matching this fn's frozen non-`Result`
/// signature.
pub fn fold_plan_diff<P: Clone, Op: Mutation<P>, K: CompositeMutationKind<P, Op>>(kind: &K, base: &P) -> MutationOutcome<<Op as Mutation<P>>::Diff> {
    let mut planner = Planner::new(base);
    let plan_result = kind.plan(base, &mut planner);
    let (steps, mut messages) = planner.into_parts();
    if let Err(error) = &plan_result {
        messages.push(MutationMessage::fatal("mutation.invariant", error.to_string()));
    }
    let rejected = plan_result.is_err() || matches!(worst_level(&messages), Some(level) if level >= crate::os_dsl::Severity::Error);
    if rejected {
        return MutationOutcome::new(<Op as Mutation<P>>::Diff::default()).absorb_messages(messages);
    }

    let mut current = base.clone();
    let mut folded = <Op as Mutation<P>>::Diff::default();
    for step in steps {
        if let PlanStep::Local(op) = step {
            let diff = op.diff(&current).into_parts().0;
            match diff.apply(&current) {
                Ok(next) => current = next,
                Err(error) => {
                    messages.push(MutationMessage::fatal("mutation.invariant", error.to_string()).at(error.target));
                    return MutationOutcome::new(<Op as Mutation<P>>::Diff::default()).absorb_messages(messages);
                }
            }
            folded.absorb(diff);
        }
    }
    MutationOutcome::new(folded).absorb_messages(messages)
}

/// ↩️ Stores each local step's inverse against its own pre-state in forward local-step order.
/// The returned flat vector uses the same storage order as [`Mutation::inverse`]; Store reverses
/// that entire vector once when applying it. Preserving both the group order and each group's
/// stored order is required for checked or otherwise noncommutative steps. A planning failure
/// folds to an empty vector.
pub fn fold_plan_inverse<P: Clone, Op: Mutation<P>, K: CompositeMutationKind<P, Op>>(kind: &K, base: &P) -> Vec<Op> {
    let mut planner = Planner::new(base);
    if kind.plan(base, &mut planner).is_err() {
        return Vec::new();
    }
    let (steps, pre_states) = planner.into_steps_with_pre_states();
    let mut local_steps: Vec<(Op, P)> = Vec::new();
    for (step, pre_state) in steps.into_iter().zip(pre_states) {
        if let (PlanStep::Local(op), Some(pre_state)) = (step, pre_state) {
            local_steps.push((op, pre_state));
        }
    }
    let mut inverses = Vec::new();
    for (op, pre_state) in local_steps.into_iter() {
        inverses.extend(op.inverse(&pre_state));
    }
    inverses
}

/// @emoji 🌐️ The [`ForeignStep`]s of a composite's plan, in discovery order — what
/// `#[derive(CompositeMutation)]`'s generated `MutationKind::foreign_steps` delegates to. A
/// planning failure folds to `Vec::new()`, never a panic.
pub fn plan_foreign_steps<P: Clone, Op: Mutation<P>, K: CompositeMutationKind<P, Op>>(kind: &K, base: &P) -> Vec<ForeignStep> {
    let Ok(steps) = plan_of(kind, base) else {
        return Vec::new();
    };
    steps
        .into_iter()
        .filter_map(|step| match step {
            PlanStep::Foreign(foreign) => Some(foreign),
            PlanStep::Local(_) => None,
        })
        .collect()
}
//#endregion 🔖️Composite

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🧬️registry/🦀️.rs"]
mod registry_fixture;

#[cfg(test)]
#[path = "🧪️tests/🧬️mutation-laws/🦀️.rs"]
mod mutation_laws_fixture;

#[cfg(test)]
mod tests {
    use super::*;
    use super::mutation_laws_fixture::{AddCounter, AddCounterTwice, AddCounterFourTimes, AddCounterThenNotifyForeign, CounterDiff, CounterMutation, foreign_step_fixture};

    //#region 🧪️ApplyErrorContract
    #[test]
    fn mutation_apply_error_json_round_trip_matches_typescript_parity_vector() {
        let error = MutationApplyError::new("mutation.apply.invalid-index", "index 4 exceeds length 2").at(["slides", "4"]);
        let json = serde_json::to_string(&error).expect("serialize apply error");
        assert_eq!(json, r#"{"code":"mutation.apply.invalid-index","message":"index 4 exceeds length 2","target":["slides","4"]}"#);
        assert_eq!(serde_json::from_str::<MutationApplyError>(&json).expect("deserialize apply error"), error);
    }

    #[test]
    fn mutation_apply_error_under_prefixes_without_losing_inner_target() {
        let error = MutationApplyError::new("mutation.apply.missing-target", "vertex missing").at(["vertices", "v1"]).under(["objects", "o1"]);
        assert_eq!(error.target, vec!["objects", "o1", "vertices", "v1"]);
    }
    //#endregion 🧪️ApplyErrorContract

    //#region 🧸️Fixtures
    #[derive(Clone, Debug, PartialEq)]
    struct Item {
        id: String,
        value: i64,
    }
    impl Identified<String> for Item {
        // 🚫️async: E1 pure accessor — Identified::id must stay sync, see the trait's own tag.
        fn id(&self) -> &String {
            &self.id
        }
    }
    impl Patchable<i64> for Item {
        fn apply_patch(&mut self, patch: &i64) {
            self.value += patch;
        }
        fn diff_patch(&self, other: &Self) -> Option<i64> {
            let delta = other.value - self.value;
            if delta == 0 {
                None
            } else {
                Some(delta)
            }
        }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️MutationLaws
    #[test]
    fn operation_diff_apply_matches_backwards_inverse() {
        let base: i64 = 10;
        let op = CounterMutation::AddCounter(AddCounter { delta: 5 });
        let forward = op.diff(&base).diff().apply(&base).expect("valid forward diff");
        assert_eq!(forward, 15);
        let [undo] = <[CounterMutation; 1]>::try_from(op.inverse(&base)).unwrap();
        let restored = undo.diff(&forward).diff().apply(&forward).expect("valid inverse diff");
        assert_eq!(restored, base);
    }

    #[test]
    fn operation_diff_absorb_accumulates() {
        let mut a = CounterDiff { deltas: vec![3] };
        a.absorb(CounterDiff { deltas: vec![4] });
        assert_eq!(a.deltas, vec![3, 4]);
        assert_eq!(a.apply(&0), Ok(7));
    }

    #[test]
    fn operation_defaults_are_stable() {
        let op = CounterMutation::AddCounter(AddCounter { delta: 1 });
        assert_eq!(op.mutation_id(), None);
        assert!(op.dependencies().is_empty());
        assert_eq!(op.base_version(), None);
        assert_eq!(op.author_id(), None);
        assert_eq!(op.timestamp(), None);
        assert_eq!(op.undo_policy(), crate::os_spr::UndoPolicy::ExactBaseOnly);
        assert_eq!(op.state_class(), crate::os_spr::StateClass::Artifact);
        assert!(op.foreign_steps(&0).is_empty());
    }
    //#endregion 🧪️MutationLaws

    //#region 🧪️OpTextLaws
    #[test]
    fn op_text_round_trip() {
        let op = CounterMutation::AddCounter(AddCounter { delta: -7 });
        let line = op.print_op();
        assert!(!line.contains('\n'));
        let parsed = CounterMutation::parse_op(&line).expect("round trip parse");
        assert_eq!(parsed, op);
    }

    #[test]
    fn op_text_parse_error_carries_message() {
        let error = CounterMutation::parse_op("nope").unwrap_err();
        assert!(!error.message.is_empty());
    }
    //#endregion 🧪️OpTextLaws

    //#region 🧪️MetaSerde
    #[test]
    fn operation_meta_serde_round_trip() {
        let meta = MutationMeta {
            mutation_id: Some(crate::os_spr::ids::MutationId("op-1".into())),
            dependencies: vec![crate::os_spr::ids::MutationId("op-0".into())],
            base_version: 3,
            author_id: Some(crate::os_spr::ids::ActorId("actor-1".into())),
            timestamp: crate::os_spr::ids::HybridLogicalTimestamp::new(1, 1000),
            undo_policy: crate::os_spr::UndoPolicy::TransformAgainstConcurrent,
            payload_hash: Some(crate::os_spr::ids::PayloadHash([7u8; 32])),
            semantic_kind: None,
            label: None,
            group_id: Some("invocation-1".to_string()),
            origin: MutationOrigin::Owner,
        };
        let json = serde_json::to_string(&meta).expect("serialize");
        assert!(json.contains("\"group_id\":\"invocation-1\""), "group_id must serialize under its own field name (MutationMeta has no rename_all), got {json}");
        let round_tripped: MutationMeta = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round_tripped, meta, "group_id must round-trip through serde exactly like semantic_kind/label");

        let solitary = MutationMeta { group_id: None, ..meta };
        let solitary_json = serde_json::to_string(&solitary).expect("serialize");
        assert!(!solitary_json.contains("group_id"), "a solitary edit's None group_id must be omitted, matching skip_serializing_if on the sibling optional fields");
        let solitary_round_tripped: MutationMeta = serde_json::from_str(&solitary_json).expect("deserialize");
        assert_eq!(solitary_round_tripped, solitary);
    }

    #[test]
    fn edit_serde_round_trip() {
        let edit = Edit::<CounterMutation> {
            id: "edit-1".into(),
            actor: Some("actor-1".into()),
            forwards: vec![CounterMutation::AddCounter(AddCounter { delta: 1 }), CounterMutation::AddCounter(AddCounter { delta: 2 })],
            inverse: vec![CounterMutation::AddCounter(AddCounter { delta: -1 }), CounterMutation::AddCounter(AddCounter { delta: -2 })],
            mutation_meta: vec![MutationMeta {
                mutation_id: None,
                dependencies: Vec::new(),
                base_version: 0,
                author_id: None,
                timestamp: crate::os_spr::ids::HybridLogicalTimestamp::new(1, 0),
                undo_policy: crate::os_spr::UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: None,
                origin: MutationOrigin::Owner,
            }],
            description: Some("two adds".into()),
            coalesce_key: None,
            sequence_number: 1,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        let json = serde_json::to_string(&edit).expect("serialize");
        let round_tripped: Edit<CounterMutation> = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round_tripped, edit);
    }
    //#endregion 🧪️MetaSerde

    //#region 🧪️CollectionLaws
    #[test]
    fn apply_add_remove_move_patch() {
        let mut items = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }];

        apply_collection_mutation(&mut items, &CollectionMutation::Add { index: 1, item: Item { id: "c".into(), value: 3 } });
        assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["a", "c", "b"]);

        apply_collection_mutation(&mut items, &CollectionMutation::Move { id: "c".into(), to_index: 2 });
        assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["a", "b", "c"]);

        apply_collection_mutation::<String, Item, i64>(&mut items, &CollectionMutation::Patch { id: "b".into(), patch: 10 });
        assert_eq!(items.iter().find(|i| i.id == "b").unwrap().value, 12);

        apply_collection_mutation(&mut items, &CollectionMutation::Remove { id: "a".into() });
        assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["b", "c"]);
    }

    #[test]
    fn invert_collection_operation_round_trips_every_kind() {
        let original = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }];

        let add = CollectionMutation::Add { index: 2, item: Item { id: "c".into(), value: 3 } };
        let mut items = original.clone();
        apply_collection_mutation(&mut items, &add);
        let inverse = inverse_collection_mutation(&original, &add);
        apply_collection_mutation(&mut items, &inverse);
        assert_eq!(items, original);

        let mov = CollectionMutation::<String, Item, i64>::Move { id: "b".into(), to_index: 0 };
        let mut items = original.clone();
        apply_collection_mutation(&mut items, &mov);
        let inverse = inverse_collection_mutation(&original, &mov);
        apply_collection_mutation(&mut items, &inverse);
        assert_eq!(items, original);

        let patch = CollectionMutation::Patch { id: "a".into(), patch: 9i64 };
        let mut items = original.clone();
        apply_collection_mutation(&mut items, &patch);
        let inverse = inverse_collection_mutation(&original, &patch);
        apply_collection_mutation(&mut items, &inverse);
        assert_eq!(items, original);

        let remove = CollectionMutation::<String, Item, i64>::Remove { id: "a".into() };
        let mut items = original.clone();
        apply_collection_mutation(&mut items, &remove);
        let inverse = inverse_collection_mutation(&original, &remove);
        apply_collection_mutation(&mut items, &inverse);
        assert_eq!(items, original);
    }

    #[test]
    fn collection_diff_from_operation_projects_each_kind() {
        let items = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }];

        let add = CollectionMutation::<String, Item, i64>::Add { index: 0, item: Item { id: "c".into(), value: 3 } };
        let diff = collection_diff_from_mutation(&items, &add);
        assert_eq!(diff.added, vec![Item { id: "c".into(), value: 3 }]);
        assert!(diff.removed.is_empty() && diff.modified.is_empty());

        let remove = CollectionMutation::<String, Item, i64>::Remove { id: "a".into() };
        let diff = collection_diff_from_mutation(&items, &remove);
        assert_eq!(diff.removed, vec!["a".to_string()]);

        let patch = CollectionMutation::Patch { id: "b".into(), patch: 5i64 };
        let diff = collection_diff_from_mutation(&items, &patch);
        assert_eq!(diff.modified, vec![ItemPatch { id: "b".into(), patch: 5i64 }]);

        let mov = CollectionMutation::<String, Item, i64>::Move { id: "a".into(), to_index: 1 };
        let diff = collection_diff_from_mutation(&items, &mov);
        assert_eq!(diff.removed, vec!["a".to_string()]);
        assert_eq!(diff.added, vec![Item { id: "a".into(), value: 1 }]);
    }
    //#endregion 🧪️CollectionLaws

    //#region 🧪️DiffKitLaws
    #[test]
    fn named_apply_removes_patches_then_adds() {
        let mut items = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }, Item { id: "c".into(), value: 3 }];
        let diff = NamedTripleDiff::<String, Item, i64> { removed: vec!["a".into()], modified: vec![ItemPatch { id: "b".into(), patch: 10 }], added: vec![Item { id: "d".into(), value: 4 }] };
        named_apply(&mut items, &diff).expect("valid named diff");
        assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["b", "c", "d"]);
        assert_eq!(items.iter().find(|i| i.id == "b").unwrap().value, 12);
    }

    #[test]
    fn named_triple_diff_is_empty_holds() {
        let empty: NamedTripleDiff<String, Item, i64> = NamedTripleDiff::default();
        assert!(empty.is_empty());
        let nonempty = NamedTripleDiff::<String, Item, i64> { added: vec![Item { id: "a".into(), value: 1 }], ..Default::default() };
        assert!(!nonempty.is_empty());
    }

    #[test]
    fn indexed_apply_modifies_removes_descending_then_inserts_ascending() {
        let mut items = vec![Item { id: "a".into(), value: 1 }, Item { id: "b".into(), value: 2 }, Item { id: "c".into(), value: 3 }];
        // BASE state: [a, b, c]. modified targets base index 0 (a). removed targets base index 2 (c).
        // added targets FINAL indices 0 and 2 in the post-remove/pre-add state [a', b].
        let diff = IndexedTripleDiff::<Item, i64> { removed: vec![2], modified: vec![(0, 100)], added: vec![(0, Item { id: "z".into(), value: 9 }), (2, Item { id: "y".into(), value: 8 })] };
        indexed_apply(&mut items, &diff).expect("valid indexed diff");
        assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["z", "a", "y", "b"]);
        assert_eq!(items[1].value, 101, "modified applies to BASE-state index 0 (item a) before removal/insertion shift it");
    }

    #[test]
    fn indexed_apply_rejects_duplicate_added_indices_without_mutating() {
        let original = vec![Item { id: "a".into(), value: 1 }];
        let mut items = original.clone();
        let diff = IndexedTripleDiff::<Item, i64> { added: vec![(1, Item { id: "b".into(), value: 2 }), (1, Item { id: "c".into(), value: 3 })], ..Default::default() };
        let error = indexed_apply(&mut items, &diff).expect_err("duplicate final indices must reject");
        assert_eq!(error.code, "mutation.apply.duplicate-target");
        assert_eq!(items, original, "rejected indexed diff must be atomic");
    }

    #[test]
    fn indexed_triple_diff_is_empty_holds() {
        let empty: IndexedTripleDiff<Item, i64> = IndexedTripleDiff::default();
        assert!(empty.is_empty());
        let nonempty = IndexedTripleDiff::<Item, i64> { removed: vec![0], ..Default::default() };
        assert!(!nonempty.is_empty());
    }
    //#endregion 🧪️DiffKitLaws

    //#region 🧪️SemanticsLaws
    #[test]
    fn str_eq_matches_std_partial_eq() {
        assert!(str_eq("rename-piece", "rename-piece"));
        assert!(!str_eq("rename-piece", "rename-part"));
        assert!(!str_eq("short", "shorter"));
    }

    #[test]
    fn is_approved_verb_matches_the_table() {
        assert!(is_approved_verb("rename"));
        assert!(is_approved_verb("flatten"));
        assert!(!is_approved_verb("set-snapshot"));
        assert!(!is_approved_verb("modify"));
    }

    #[test]
    fn approved_verbs_are_unique_and_lowercase() {
        let mut seen = std::collections::HashSet::new();
        for (verb, record) in APPROVED_VERBS {
            assert_eq!(*verb, verb.to_lowercase(), "verb {verb:?} must be lowercase");
            assert!(seen.insert(*verb), "duplicate verb {verb:?} in APPROVED_VERBS");
            assert!(!record.is_empty(), "verb {verb:?} must have a non-empty past-tense record form");
        }
    }

    #[test]
    fn space_history_verbs_match_the_language_neutral_contract() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🧬️verb-vocabulary/🔣️cases.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let verb = case["verb"].as_str().unwrap();
            let expected = case["record"].as_str();
            assert_eq!(APPROVED_VERBS.iter().find(|entry| entry.0 == verb).map(|entry| entry.1), expected);
            assert_eq!(is_approved_verb(verb), expected.is_some());
        }
    }

    #[test]
    fn mutation_descriptor_semantics_participate_in_immutable_identity() {
        use super::registry_fixture::{MiniDoc, MiniMutation, RenameMini};
        let semantics = <RenameMini as MutationKind<MiniDoc, MiniMutation>>::SEMANTICS;
        let construct = |semantics| MutationDescriptor::new(crate::os_spr::SchemaId("mini.doc#rename-mini".into()), crate::os_spr::SchemaVersion(1), crate::os_spr::StateClass::Artifact, RenameMini::DESCRIPTOR, semantics).unwrap();
        let base = construct(semantics);
        let changed = construct(SemanticDescriptor { record: "RenamedMiniLabel", ..semantics });
        assert_ne!(base.fingerprint(), changed.fingerprint());
        assert_eq!(base.semantics(), &semantics);
        assert_eq!(base.leaf(), &RenameMini::DESCRIPTOR);
    }
    //#endregion 🧪️SemanticsLaws

    //#region 🧪️MutationsDeriveLaws
    #[test]
    fn derive_mutations_wires_complete_leaf_and_atomic_registration() {
        use super::registry_fixture::*;
        let base = MiniDoc { name: "a".into() };
        let mutation: MiniMutation = RenameMini { new_name: "b".into() }.into();
        let after = mutation.diff(&base).diff().apply(&base).expect("valid forward diff");
        assert_eq!(after.name, "b");
        let inverse = mutation.inverse(&base);
        assert_eq!(inverse.len(), 1);
        assert_eq!(inverse[0].diff(&after).diff().apply(&after), Ok(base));
        assert_eq!(MiniMutation::DESCRIPTORS, &[RenameMini::DESCRIPTOR]);
        assert_eq!(mutation.descriptor(), &RenameMini::DESCRIPTOR);
        assert_eq!(MiniMutation::kinds(), &[<RenameMini as MutationKind<MiniDoc, MiniMutation>>::SEMANTICS]);
        assert_eq!(mutation.semantics().record, "RenamedMini");
        assert_eq!(mutation.label(), "Rename mini to \"b\"");
        assert!(mutation.target().is_empty());
        register_mini_mutation_descriptors(crate::os_spr::StateClass::Artifact).unwrap();
        register_mini_mutation_descriptors(crate::os_spr::StateClass::Artifact).unwrap();
        let descriptor = mutation_descriptor("mini.doc#rename-mini").unwrap();
        assert_eq!(descriptor.semantics(), mutation.semantics());
        assert_eq!(descriptor.leaf(), mutation.descriptor());
        let declared: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🧬️registry/🧬️mutations/📛️rename-mini/🔣️.json")).unwrap();
        assert_eq!(serde_json::to_value(descriptor.leaf()).unwrap(), declared);
        assert!(register_mini_mutation_descriptors(crate::os_spr::StateClass::Config).is_err());
        assert_eq!(mutation_descriptor("mini.doc#rename-mini"), Some(descriptor));
    }
    //#endregion 🧪️MutationsDeriveLaws

    //#region 🧪️DescriptorLaws
    #[test]
    fn descriptor_registry_rejects_conflicts_without_partial_publication() {
        use super::registry_fixture::{MiniDoc, MiniMutation, RenameMini};
        let build = |id: &str, state| MutationDescriptor::new(crate::os_spr::SchemaId(id.into()), crate::os_spr::SchemaVersion(1), state, RenameMini::DESCRIPTOR, <RenameMini as MutationKind<MiniDoc, MiniMutation>>::SEMANTICS).unwrap();
        let first = build("mini.first", crate::os_spr::StateClass::Artifact);
        let conflict = build("mini.first", crate::os_spr::StateClass::Config);
        let second = build("mini.second", crate::os_spr::StateClass::Artifact);
        assert_ne!(first.fingerprint(), conflict.fingerprint());
        let mut registry = MutationDescriptorRegistry::new();
        assert!(registry.register_all([first.clone(), conflict.clone()]).is_err());
        assert!(registry.is_empty());
        registry.register(first.clone()).unwrap();
        assert!(registry.register_all([second.clone(), conflict]).is_err());
        assert_eq!(registry.len(), 1);
        assert_eq!(registry.get("mini.first"), Some(&first));
        assert!(registry.get("mini.second").is_none());
        registry.register_all([first.clone(), second.clone(), second]).unwrap();
        assert_eq!(registry.len(), 2);
        assert_eq!(registry.get("mini.first"), Some(&first));
    }
    //#endregion 🧪️DescriptorLaws

    //#region 🧪️MutationLeafDescriptorLaws
    fn mutation_leaf_descriptor_fixture() -> MutationLeafDescriptor {
        MutationLeafDescriptor {
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
        }
    }

    fn mutation_leaf_descriptor_fixture_json() -> serde_json::Value {
        serde_json::from_str(include_str!("🧪️tests/🧬️mutation-leaf-descriptor/🧫️fixtures/🔣️.json")).expect("valid neutral descriptor fixture")
    }

    static MUTATION_LEAF_DESCRIPTOR_DUPLICATE_OUTCOMES: [MutationOutcomeClass; 2] = [MutationOutcomeClass::Applied, MutationOutcomeClass::Applied];
    static MUTATION_LEAF_DESCRIPTOR_NON_RUST_SURFACES: [MutationLanguageSurface; 1] = [MutationLanguageSurface::Text];
    static MUTATION_LEAF_DESCRIPTOR_DUPLICATE_SURFACES: [MutationLanguageSurface; 2] = [MutationLanguageSurface::Rust, MutationLanguageSurface::Rust];
    static MUTATION_LEAF_DESCRIPTOR_OWNER_BOUNDARIES: [(&str, &str, bool); 5] = [
        ("unicode-line-separator", "prefix\u{2028}/🧬️mutations/➕️insert-page", false),
        ("unicode-paragraph-separator", "prefix\u{2029}/🧬️mutations/➕️insert-page", false),
        ("multiple-markers-later-valid", "/🧬️mutations/first/🧬️mutations/second", true),
        ("multiple-markers-prefixed", "prefix/🧬️mutations/second/🧬️mutations/third", true),
        ("marker-without-suffix", "prefix/🧬️mutations/", false),
    ];
    static MUTATION_LEAF_DESCRIPTOR_OUTCOMES: [MutationOutcomeClass; 1] = [MutationOutcomeClass::Applied];
    static MUTATION_LEAF_DESCRIPTOR_SURFACES: [MutationLanguageSurface; 1] = [MutationLanguageSurface::Rust];
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_ROOT: &str = "✏️s/🔌️plugins/🧪️probe/🧬️mutations";
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT: MutationLeafDescriptor = MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➕️insert-page", semantic_kind: "insert-page", display_name: "Insert Page", emoji: "➕️", aggregate_variant: "InsertPage", payload_schema: "🦀️.rs#InsertPage", text_opcode: Some("insert-page"), binary_tag: Some(1), invertibility: MutationInvertibility::ExplicitMutation, diff_participation: MutationDiffParticipation::ApplyOnly, outcome_classes: &MUTATION_LEAF_DESCRIPTOR_OUTCOMES, composition: MutationComposition::Atomic, required_language_surfaces: &MUTATION_LEAF_DESCRIPTOR_SURFACES };
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE: MutationLeafDescriptor = MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➖️remove-page", semantic_kind: "remove-page", display_name: "Remove Page", emoji: "➖️", aggregate_variant: "RemovePage", payload_schema: "🦀️.rs#RemovePage", text_opcode: Some("remove-page"), binary_tag: Some(2), invertibility: MutationInvertibility::ExplicitMutation, diff_participation: MutationDiffParticipation::ApplyOnly, outcome_classes: &MUTATION_LEAF_DESCRIPTOR_OUTCOMES, composition: MutationComposition::Atomic, required_language_surfaces: &MUTATION_LEAF_DESCRIPTOR_SURFACES };
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_NULLABLE: MutationLeafDescriptor = MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/♻️replace-page", semantic_kind: "replace-page", display_name: "Replace Page", emoji: "♻️", aggregate_variant: "ReplacePage", payload_schema: "🦀️.rs#ReplacePage", text_opcode: None, binary_tag: None, invertibility: MutationInvertibility::ExplicitMutation, diff_participation: MutationDiffParticipation::ApplyOnly, outcome_classes: &MUTATION_LEAF_DESCRIPTOR_OUTCOMES, composition: MutationComposition::Atomic, required_language_surfaces: &MUTATION_LEAF_DESCRIPTOR_SURFACES };
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_OTHER_OWNER: MutationLeafDescriptor = MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧪️other/🧬️mutations/➕️insert-page", semantic_kind: "insert-page", display_name: "Insert Page", emoji: "➕️", aggregate_variant: "InsertPage", payload_schema: "🦀️.rs#InsertPage", text_opcode: Some("insert-page"), binary_tag: Some(1), invertibility: MutationInvertibility::ExplicitMutation, diff_participation: MutationDiffParticipation::ApplyOnly, outcome_classes: &MUTATION_LEAF_DESCRIPTOR_OUTCOMES, composition: MutationComposition::Atomic, required_language_surfaces: &MUTATION_LEAF_DESCRIPTOR_SURFACES };
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_UNIQUE: [MutationLeafDescriptor; 2] = [MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT, MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE];
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_SEMANTIC: [MutationLeafDescriptor; 2] = [MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT, MutationLeafDescriptor { semantic_kind: "insert-page", ..MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE }];
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_OPCODE: [MutationLeafDescriptor; 2] = [MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT, MutationLeafDescriptor { text_opcode: Some("insert-page"), ..MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE }];
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_TAG: [MutationLeafDescriptor; 2] = [MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT, MutationLeafDescriptor { binary_tag: Some(1), ..MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE }];
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_NULLABLE_REPEAT: [MutationLeafDescriptor; 2] = [MUTATION_LEAF_DESCRIPTOR_ROSTER_NULLABLE, MutationLeafDescriptor { owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/🗄️archive-page", semantic_kind: "archive-page", display_name: "Archive Page", emoji: "🗄️", aggregate_variant: "ArchivePage", payload_schema: "🦀️.rs#ArchivePage", ..MUTATION_LEAF_DESCRIPTOR_ROSTER_NULLABLE }];
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_OWNER_MISMATCH: [MutationLeafDescriptor; 2] = [MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT, MUTATION_LEAF_DESCRIPTOR_ROSTER_OTHER_OWNER];
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_OWNER: [MutationLeafDescriptor; 2] = [MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT, MutationLeafDescriptor { semantic_kind: "restore-page", display_name: "Restore Page", emoji: "↩️", aggregate_variant: "RestorePage", payload_schema: "🦀️.rs#RestorePage", text_opcode: Some("restore-page"), binary_tag: Some(3), ..MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT }];
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_NESTED_OWNER: [MutationLeafDescriptor; 1] = [MutationLeafDescriptor { owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➖️remove-page/🧪️tests", ..MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE }];
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_PARENT_CHILD: [MutationLeafDescriptor; 1] = [MutationLeafDescriptor { owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/..", ..MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE }];
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_BACKSLASH_CHILD: [MutationLeafDescriptor; 1] = [MutationLeafDescriptor { owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➖️remove-page\\🧪️tests", ..MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE }];
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_NUL_CHILD: [MutationLeafDescriptor; 1] = [MutationLeafDescriptor { owner: "✏️s/🔌️plugins/🧪️probe/🧬️mutations/➖️remove-page\0", ..MUTATION_LEAF_DESCRIPTOR_ROSTER_REMOVE }];
    static MUTATION_LEAF_DESCRIPTOR_ROSTER_OTHER_OWNER_SINGLE: [MutationLeafDescriptor; 1] = [MUTATION_LEAF_DESCRIPTOR_ROSTER_OTHER_OWNER];
    const MUTATION_LEAF_DESCRIPTOR_CONST_VALID: Result<(), MutationLeafDescriptorValidationError> = validate_mutation_leaf_descriptor(&MUTATION_LEAF_DESCRIPTOR_ROSTER_INSERT);
    const MUTATION_LEAF_DESCRIPTOR_CONST_ROSTER_VALID: Result<(), MutationLeafDescriptorRosterValidationError> = validate_mutation_leaf_descriptor_roster(MUTATION_LEAF_DESCRIPTOR_ROSTER_ROOT, &MUTATION_LEAF_DESCRIPTOR_ROSTER_UNIQUE);

    #[test]
    fn mutation_leaf_descriptor_serializes_all_schema_fields() {
        let descriptor = mutation_leaf_descriptor_fixture();
        let fixture = mutation_leaf_descriptor_fixture_json();
        assert_eq!(validate_mutation_leaf_descriptor(&descriptor), Ok(()));
        let serialized = serde_json::to_value(descriptor).expect("serialize descriptor");
        assert_eq!(serialized, fixture["descriptor"]);
        assert_eq!(serialized.as_object().expect("descriptor object").len(), 14);
        assert!(serialized.get("textOpcode").expect("required nullable text opcode").is_null());
        assert!(serialized.get("binaryTag").expect("required nullable binary tag").is_null());
    }

    #[test]
    fn mutation_leaf_descriptor_enum_wires_match_neutral_fixture() {
        let fixture = mutation_leaf_descriptor_fixture_json();
        let wires = serde_json::json!({
            "invertibility": [MutationInvertibility::SelfInvertible, MutationInvertibility::ExplicitMutation, MutationInvertibility::Plan, MutationInvertibility::NonInvertible],
            "diffParticipation": [MutationDiffParticipation::Detect, MutationDiffParticipation::ApplyOnly, MutationDiffParticipation::Plan, MutationDiffParticipation::None],
            "outcomeClasses": [MutationOutcomeClass::Applied, MutationOutcomeClass::Info, MutationOutcomeClass::Warning, MutationOutcomeClass::Error, MutationOutcomeClass::Fatal],
            "composition": [MutationComposition::Atomic, MutationComposition::Composite],
            "requiredLanguageSurfaces": [MutationLanguageSurface::Rust, MutationLanguageSurface::Typescript, MutationLanguageSurface::Graphql, MutationLanguageSurface::Protobuf, MutationLanguageSurface::JsonSchema, MutationLanguageSurface::Text, MutationLanguageSurface::Binary],
        });
        assert_eq!(wires, fixture["enumWireValues"]);
    }

    #[test]
    fn mutation_leaf_descriptor_validates_static_schema_boundaries() {
        let fixture = mutation_leaf_descriptor_fixture_json();
        for vector in fixture["binaryTagVectors"].as_array().expect("vector array") {
            let expected = vector["expected"].as_bool().expect("expected boolean");
            let actual = match vector.get("value") {
                Some(serde_json::Value::Null) => Some(None),
                Some(serde_json::Value::Number(value)) => value.as_u64().and_then(|value| u32::try_from(value).ok()).map(Some),
                _ => None,
            };
            assert_eq!(actual.is_some(), expected, "{}", vector["name"]);
            if let Some(binary_tag) = actual {
                let mut descriptor = mutation_leaf_descriptor_fixture();
                descriptor.binary_tag = binary_tag;
                assert_eq!(descriptor.validate(), Ok(()), "{}", vector["name"]);
            }
        }
        let descriptor = mutation_leaf_descriptor_fixture();
        assert!(serde_json::to_value(descriptor).expect("serialize descriptor").get("binaryTag").is_some(), "binaryTag cannot be omitted from the static descriptor shape");

        let mut invalid = descriptor;
        invalid.schema_version = 2;
        assert_eq!(invalid.validate().expect_err("schema version must be one").field, "schemaVersion");
        invalid = descriptor;
        invalid.owner = "compose/🧬️mutations/➕️insert-page";
        assert_eq!(invalid.validate().expect_err("compose owner is excluded").field, "owner");
        invalid = descriptor;
        invalid.owner = "owner-without-mutation-root";
        assert_eq!(invalid.validate().expect_err("owner must name a direct mutation leaf").field, "owner");
        invalid = descriptor;
        invalid.semantic_kind = "insert";
        assert_eq!(invalid.validate().expect_err("semantic kind needs two kebab segments").field, "semanticKind");
        invalid = descriptor;
        invalid.display_name = "";
        assert_eq!(invalid.validate().expect_err("display name is required").field, "displayName");
        invalid = descriptor;
        invalid.emoji = "";
        assert_eq!(invalid.validate().expect_err("emoji is required").field, "emoji");
        invalid = descriptor;
        invalid.aggregate_variant = "insert_page";
        assert_eq!(invalid.validate().expect_err("aggregate variant is Pascal identifier").field, "aggregateVariant");
        invalid = descriptor;
        invalid.payload_schema = "";
        assert_eq!(invalid.validate().expect_err("payload schema is required").field, "payloadSchema");
        invalid = descriptor;
        invalid.text_opcode = Some("insert");
        assert_eq!(invalid.validate().expect_err("text opcode is kebab semantic kind").field, "textOpcode");
        invalid = descriptor;
        invalid.outcome_classes = &[];
        assert_eq!(invalid.validate().expect_err("outcome classes are required").field, "outcomeClasses");
        invalid.outcome_classes = &MUTATION_LEAF_DESCRIPTOR_DUPLICATE_OUTCOMES;
        assert_eq!(invalid.validate().expect_err("outcome classes are unique").field, "outcomeClasses");
        invalid = descriptor;
        invalid.required_language_surfaces = &[];
        assert_eq!(invalid.validate().expect_err("language surfaces are required").field, "requiredLanguageSurfaces");
        invalid.required_language_surfaces = &MUTATION_LEAF_DESCRIPTOR_NON_RUST_SURFACES;
        assert_eq!(invalid.validate().expect_err("rust surface is required").field, "requiredLanguageSurfaces");
        invalid.required_language_surfaces = &MUTATION_LEAF_DESCRIPTOR_DUPLICATE_SURFACES;
        assert_eq!(invalid.validate().expect_err("language surfaces are unique").field, "requiredLanguageSurfaces");
    }

    #[test]
    fn mutation_leaf_descriptor_owner_boundaries_match_neutral_vectors() {
        let fixture = mutation_leaf_descriptor_fixture_json();
        let neutral: Vec<(&str, &str, bool)> = fixture["ownerBoundaryVectors"].as_array().expect("owner boundary vectors").iter().map(|vector| (vector["name"].as_str().expect("name"), vector["owner"].as_str().expect("owner"), vector["expected"].as_bool().expect("expected"))).collect();
        assert_eq!(neutral, MUTATION_LEAF_DESCRIPTOR_OWNER_BOUNDARIES);
        for (name, owner, expected) in MUTATION_LEAF_DESCRIPTOR_OWNER_BOUNDARIES {
            let mut descriptor = mutation_leaf_descriptor_fixture();
            descriptor.owner = owner;
            assert_eq!(descriptor.validate().is_ok(), expected, "{name}");
        }
    }

    #[test]
    fn mutation_leaf_descriptor_const_roster_boundaries_match_neutral_vectors() {
        assert_eq!(MUTATION_LEAF_DESCRIPTOR_CONST_VALID, Ok(()));
        assert_eq!(MUTATION_LEAF_DESCRIPTOR_CONST_ROSTER_VALID, Ok(()));
        let fixture = mutation_leaf_descriptor_fixture_json();
        let names: Vec<(&str, bool)> = fixture["rosterVectors"].as_array().expect("roster vectors").iter().map(|vector| (vector["name"].as_str().expect("name"), vector["expected"].as_bool().expect("expected"))).collect();
        assert_eq!(names, [("same-owner-unique", true), ("duplicate-semantic-kind", false), ("duplicate-text-opcode", false), ("duplicate-binary-tag", false), ("nullable-identities-repeat", true), ("unrelated-owner", false), ("duplicate-owner", false), ("nested-child", false), ("parent-child", false), ("backslash-child", false), ("absolute-root", false), ("windows-root", false), ("windows-slash-drive-root", false), ("windows-relative-drive-root", false), ("empty-segment-root", false), ("dot-root", false), ("parent-root", false), ("nul-root", false), ("nul-child", false), ("distinct-owner-same-identities", true)]);
        let root = MUTATION_LEAF_DESCRIPTOR_ROSTER_ROOT;
        assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_UNIQUE), Ok(()));
        assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_SEMANTIC).expect_err("duplicate semantic kind").field, "semanticKind");
        assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_OPCODE).expect_err("duplicate text opcode").field, "textOpcode");
        assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_TAG).expect_err("duplicate binary tag").field, "binaryTag");
        assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_NULLABLE_REPEAT), Ok(()));
        assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_OWNER_MISMATCH).expect_err("unrelated owner").field, "owner");
        assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_DUPLICATE_OWNER).expect_err("duplicate owner").field, "owner");
        assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_NESTED_OWNER).expect_err("nested child").field, "owner");
        assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_PARENT_CHILD).expect_err("parent child").field, "owner");
        assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_BACKSLASH_CHILD).expect_err("backslash child").field, "owner");
        assert_eq!(validate_mutation_leaf_descriptor_roster(root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_NUL_CHILD).expect_err("nul child").field, "owner");
        for unsafe_root in ["/✏️s/🔌️plugins/🧪️probe/🧬️mutations", "C:\\✏️s\\🔌️plugins\\🧪️probe\\🧬️mutations", "C:/✏️s/🔌️plugins/🧪️probe/🧬️mutations", "C:✏️s/🔌️plugins/🧪️probe/🧬️mutations", "✏️s//🔌️plugins/🧪️probe/🧬️mutations", "./✏️s/🔌️plugins/🧪️probe/🧬️mutations", "../✏️s/🔌️plugins/🧪️probe/🧬️mutations", "✏️s/\0🔌️plugins/🧪️probe/🧬️mutations"] {
            assert_eq!(validate_mutation_leaf_descriptor_roster(unsafe_root, &MUTATION_LEAF_DESCRIPTOR_ROSTER_UNIQUE).expect_err("unsafe root").field, "owner");
        }
        assert_eq!(validate_mutation_leaf_descriptor_roster("✏️s/🔌️plugins/🧪️other/🧬️mutations", &MUTATION_LEAF_DESCRIPTOR_ROSTER_OTHER_OWNER_SINGLE), Ok(()));
    }
    //#endregion 🧪️MutationLeafDescriptorLaws

    //#region 🧪️UpcastLaws
    // Clamp-to-floor is the simplest genuinely idempotent upcaster: `upcast(upcast(x)) ==
    // upcast(x)` holds because `max(max(x, 10), 10) == max(x, 10)` for every `x`.
    struct ClampToFloor;
    impl MutationUpcaster<i64> for ClampToFloor {
        fn upcast(&self, _from_version: crate::os_spr::ids::SchemaVersion, op: i64) -> i64 {
            op.max(10)
        }
    }

    #[test]
    fn upcaster_is_idempotent_at_target_version() {
        let upcaster = ClampToFloor;
        let version = crate::os_spr::ids::SchemaVersion(1);
        for start in [0i64, 3, 7, 10, 40] {
            let once = upcaster.upcast(version, start);
            let twice = upcaster.upcast(version, once);
            assert_eq!(once, twice);
        }
    }
    //#endregion 🧪️UpcastLaws

    //#region 🧪️InferenceLaws
    // Smallest possible (P=i64) Inference/InferenceSpec/DiffRegions fixture: infers "is_even" and
    // "abs_value" from an i64 snapshot, reusing the same CounterDiff/CounterMutation pair as the Mutation laws
    // above so this proves the inference traits interoperate with the existing diff/mutation shape.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct AddInference {
        is_even: bool,
        abs_value: i64,
    }
    // 🎯️ Hand-written (not derived) `Default`: it must equal `infer(&i64::default())` for the
    // default law to hold, and `0.is_even() == true` disagrees with `bool::default() == false`.
    impl Default for AddInference {
        fn default() -> Self {
            AddInference { is_even: true, abs_value: 0 }
        }
    }
    impl Inference<i64> for AddInference {
        fn infer(snapshot: &i64) -> Self {
            AddInference { is_even: snapshot % 2 == 0, abs_value: snapshot.abs() }
        }
    }
    impl InferenceSpec<i64> for AddInference {
        fn inference_schema_id() -> &'static str {
            "s.wave3.synthetic.inference"
        }
        fn schema_version() -> u32 {
            1
        }
        fn fields() -> &'static [InferenceFieldSpec] {
            &[InferenceFieldSpec { id: "isEven", reads: &["value"] }, InferenceFieldSpec { id: "absValue", reads: &["value"] }]
        }
    }


    #[test]
    fn inference_determinism_law() {
        let base: i64 = 42;
        assert_eq!(AddInference::infer(&base), AddInference::infer(&base));
        let json_a = serde_json::to_string(&AddInference::infer(&base)).unwrap();
        let json_b = serde_json::to_string(&AddInference::infer(&42)).unwrap();
        assert_eq!(json_a, json_b, "equal snapshots must infer byte-equal canonical serializations");
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(AddInference::infer(&i64::default()), AddInference::default());
    }

    #[test]
    fn inference_diff_consistency_law() {
        let base: i64 = 10;
        let noop = CounterDiff { deltas: vec![0] };
        assert!(!noop.touches().intersects_any(AddInference::fields()[0].reads));
        assert_eq!(AddInference::infer(&noop.apply(&base).expect("valid no-op diff")), AddInference::infer(&base));

        let real = CounterDiff { deltas: vec![1] };
        assert!(real.touches().intersects_any(AddInference::fields()[0].reads));
        assert_ne!(AddInference::infer(&real.apply(&base).expect("valid real diff")), AddInference::infer(&base));
    }

    #[test]
    fn inference_spec_carries_schema_identity() {
        assert_eq!(AddInference::inference_schema_id(), "s.wave3.synthetic.inference");
        assert_eq!(AddInference::schema_version(), 1);
        assert_eq!(AddInference::fields().len(), 2);
    }

    #[test]
    fn touched_paths_intersects_ancestor_and_descendant_prefixes() {
        let coarse = TouchedPaths::new(["objects"]);
        assert!(coarse.intersects_prefix("objects/o1/vortices"), "a coarse write region must cover a finer read region beneath it");

        let fine = TouchedPaths::new(["objects/o1/vortices"]);
        assert!(fine.intersects_prefix("objects"), "a finer write region must still be caught by a coarser read region above it");

        let unrelated = TouchedPaths::new(["objects/o1"]);
        assert!(!unrelated.intersects_prefix("attractions"), "disjoint subtrees must not intersect");
    }

    #[test]
    fn touched_paths_intersects_any_matches_first_hit() {
        let touched = TouchedPaths::new(["attractions/a1"]);
        assert!(touched.intersects_any(&["objects", "attractions"]));
        assert!(!touched.intersects_any(&["objects", "vortices"]));
    }

    #[test]
    fn touched_paths_default_is_empty_and_intersects_nothing() {
        let empty = TouchedPaths::default();
        assert!(empty.paths.is_empty());
        assert!(!empty.intersects_prefix("anything"));
    }
    //#endregion 🧪️InferenceLaws

    //#region 🧪️OutcomeLaws
    #[test]
    fn command_outcome_default_is_empty() {
        let outcome: CommandOutcome<CounterDiff> = CommandOutcome::default();
        assert!(outcome.persistent.is_empty());
        assert!(outcome.shared_ui.is_empty());
        assert!(outcome.local_ui.is_empty());
        assert!(outcome.preview.is_empty());
        assert!(outcome.effects.is_empty());
    }

    #[test]
    fn operation_event_serde_round_trip() {
        let event = MutationEvent { mutation_id: crate::os_spr::ids::MutationId("op-1".into()), state_class: crate::os_spr::StateClass::Transient, payload: serde_json::json!({ "kind": "toast", "text": "saved" }) };
        let json = serde_json::to_string(&event).expect("serialize");
        let round_tripped: MutationEvent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round_tripped, event);
    }
    //#endregion 🧪️OutcomeLaws

    //#region 🧪️CompositeLaws
    #[test]
    fn fold_plan_diff_equals_sequential_apply() {
        let base: i64 = 10;
        let kind = AddCounterTwice { delta: 3 };
        let diff = fold_plan_diff(&kind, &base);
        assert_eq!(diff.diff().apply(&base), Ok(16));

        let steps = plan_of(&kind, &base).expect("plan succeeds");
        let mut sequential = base;
        for step in &steps {
            if let PlanStep::Local(op) = step {
                sequential = op.diff(&sequential).diff().apply(&sequential).expect("valid planned diff");
            }
        }
        assert_eq!(diff.diff().apply(&base), Ok(sequential), "fold_plan_diff must equal sequential application of the plan's local steps");
    }

    #[test]
    fn fold_plan_inverse_restores_base() {
        let base: i64 = 10;
        let kind = AddCounterTwice { delta: 3 };
        let forward = fold_plan_diff(&kind, &base).diff().apply(&base).expect("valid folded diff");
        assert_ne!(forward, base);
        let inverses = fold_plan_inverse(&kind, &base);
        let mut restored = forward;
        for op in inverses.iter().rev() {
            restored = op.diff(&restored).diff().apply(&restored).expect("valid inverse diff");
        }
        assert_eq!(restored, base, "fold_plan_inverse applied after the composite must restore base");
    }

    #[test]
    fn composite_of_composite_nests_and_folds_identically_to_flattened_plan() {
        let base: i64 = 0;
        let quad = AddCounterFourTimes { delta: 2 };
        let diff = fold_plan_diff(&quad, &base);
        assert_eq!(diff.diff().apply(&base), Ok(8), "two nested AddCounterTwice{{delta:2}} embeds must fold to +8");

        let steps = plan_of(&quad, &base).expect("plan succeeds");
        let local_deltas: Vec<i64> = steps
            .iter()
            .filter_map(|step| match step {
                PlanStep::Local(CounterMutation::AddCounter(op)) => Some(op.delta),
                _ => None,
            })
            .collect();
        assert_eq!(local_deltas, vec![2, 2, 2, 2], "nesting must flatten to four local steps, identical to the un-nested plan");

        let inverses = fold_plan_inverse(&quad, &base);
        let mut restored = diff.diff().apply(&base).expect("valid folded diff");
        for op in inverses.iter().rev() {
            restored = op.diff(&restored).diff().apply(&restored).expect("valid inverse diff");
        }
        assert_eq!(restored, base);
    }

    #[test]
    fn plan_depth_beyond_max_is_typed_error_never_panics() {
        let base: i64 = 0;
        let kind = AddCounterThenNotifyForeign { delta: 1, foreign_count: MAX_PLAN_DEPTH + 1 };
        let error = plan_of(&kind, &base).expect_err("a plan with more foreign hops than MAX_PLAN_DEPTH must be rejected, not panic");
        assert_eq!(error, PlanError::DepthExceeded(MAX_PLAN_DEPTH));
    }

    #[test]
    fn plan_cycle_is_typed_error_never_panics() {
        let base: i64 = 0;
        let mut planner: Planner<i64, CounterMutation> = Planner::new(&base);
        planner.call_foreign(foreign_step_fixture(0)).expect("first hop to a fresh target succeeds");
        let error = planner.call_foreign(foreign_step_fixture(0)).expect_err("repeating the identical (mutation_id, payload) pair must be rejected as a cycle, not panic");
        assert_eq!(error, PlanError::Cycle("artifact-0".to_string()));
    }

    #[test]
    fn foreign_steps_are_excluded_from_fold_plan_diff() {
        let base: i64 = 5;
        let kind = AddCounterThenNotifyForeign { delta: 4, foreign_count: 2 };
        let diff = fold_plan_diff(&kind, &base);
        assert_eq!(diff.diff().apply(&base), Ok(9), "only the local AddCounter{{delta:4}} may contribute to the folded diff");

        let foreign = plan_foreign_steps(&kind, &base);
        assert_eq!(foreign.len(), 2);
        assert_eq!(foreign[0].target.artifact_id, "artifact-0");
        assert_eq!(foreign[1].target.artifact_id, "artifact-1");
    }

    #[test]
    fn derive_composite_mutation_wires_delegating_mutation_kind() {
        let base: i64 = 1;
        let kind = AddCounterTwice { delta: 5 };
        let diff = MutationKind::<i64, CounterMutation>::diff(&kind, &base);
        assert_eq!(diff.diff().apply(&base), Ok(11));
        let inverse = MutationKind::<i64, CounterMutation>::inverse(&kind, &base);
        let mut restored = diff.diff().apply(&base).expect("valid folded diff");
        for op in inverse.iter().rev() {
            restored = op.diff(&restored).diff().apply(&restored).expect("valid inverse diff");
        }
        assert_eq!(restored, base);
        assert_eq!(<AddCounterTwice as MutationKind<i64, CounterMutation>>::SEMANTICS.kind, "add-counter-twice");
        assert!(MutationKind::<i64, CounterMutation>::foreign_steps(&kind, &base).is_empty());
    }
    //#endregion 🧪️CompositeLaws
}
//#endregion 🧪️Tests
