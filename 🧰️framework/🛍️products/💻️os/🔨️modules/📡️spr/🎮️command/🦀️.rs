//! 🎞️ Protocol command/collaboration-semantics layer: the `Mutation`/`MutationDiff`/`OpText`
//! trait family, `MutationMeta`/`Edit`, generic collection-operation plumbing, the runtime
//! `MutationDescriptor` registry, the upcast seam, and the five-channel `CommandOutcome`. Moved
//! (with two defaulted trait methods and a `reconcile` return-type change, both called out inline)
//! from `vcs/rs/lib.rs`'s `🔖️Mutation`/`🔖️CollectionDiff`/`🔖️CollectionMutation` regions and
//! `framework/core`. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` `## Amendment` §`protocol_command`.
//!
//! Op payloads stay schema-opaque here exactly like `protocol_history`: this crate never parses or
//! interprets an `Op`'s fields, only threads it through the trait seams a technology implements.

//#region 🔖️Contract
/// 🎞️ The mutation contract itself lives in `protocol::mutation` (framework module
/// `📡️replication`); this authoring layer builds on it and re-exports it so every historical
/// `command::Mutation`/`Edit`/`MutationMessage` path keeps resolving.
pub use protocol::mutation::*;

use semio_framework_value_derive::{FromValue, ToValue};
//#endregion 🔖️Contract

//#region 🔖️Inference
/// @emoji 💡️ All information inferable from a snapshot — the fourth schema family alongside
/// `Snapshot`/`Diff`/`Mutation` (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
/// LAWS: pure (reads only `snapshot`), deterministic (equal snapshots ⇒ byte-equal canonical
/// serializations of the result), total (never panics, never fails — an inference with error
/// states models them as data, not as a `Result`). `infer` is THE single semantics source: every
/// cache path in `crate::os_inference` must be observationally identical to calling this directly.
/// 🌱️ Bound on [`protocol::value::ToValue`]/[`protocol::value::FromValue`], not `serde::Serialize`/
/// `serde::de::DeserializeOwned` — the same move [`CompositeMutationKind`] below and
/// `protocol::Mutation` itself already made. Every `#[derive(ToValue, FromValue)]` inference type in
/// the plugin tree implements these and no longer implements serde's, so the serde bound left every
/// one of them failing to satisfy this trait.
pub trait Inference<P>: Clone + Default + protocol::value::ToValue + protocol::value::FromValue {
    fn infer(snapshot: &P) -> Self;
}

/// @emoji 🗺️ Region vocabulary shared by [`DiffRegions::touches`] and an [`InferenceFieldSpec`]'s
/// `reads`. Paths are `/`-joined segments (e.g. `"objects/o1/vortices"`); two paths "intersect" when
/// one's segments are a prefix of the other's (either direction), matching how a coarse write region
/// (`"objects"`) covers every finer read region beneath it (`"objects/o1/vortices"`) and vice versa.
#[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
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
    ("discard", "Discarded"),
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
    ("stage", "Staged"),
    ("start", "Started"),
    ("switch", "Switched"),
    ("toggle", "Toggled"),
    ("unbind", "Unbound"),
    ("unflatten", "Unflattened"),
    ("ungroup", "Ungrouped"),
    ("update", "Updated"),
    ("verify", "Verified"),
];

/// @emoji 🔤️ `const`-context string equality (stable `&str: PartialEq` isn't usable in a `const`
/// assertion) — used by `#[derive(Mutations)]`'s generated `SEMANTICS.kind == kebab(variant)` check.
pub const fn str_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = a.len();
    while i > 0 {
        i -= 1;
        if a[i] != b[i] {
            return false;
        }
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue)]
pub struct SemanticDescriptor {
    pub verb: &'static str,
    pub entity: &'static str,
    pub kind: &'static str,
    pub record: &'static str,
}

//#region 🪪️MutationLeafDescriptor
/// 🪞️ Reexports the lower mutation metadata contract through the public OS command façade.
pub use protocol::mutation::{
    validate_mutation_leaf_descriptor, validate_mutation_leaf_descriptor_roster, validate_mutation_leaf_descriptor_roster_uniqueness, validate_mutation_leaf_source, MutationComposition, MutationDiffParticipation, MutationDomainOperation,
    MutationInvertibility, MutationLanguageSurface, MutationLeaf, MutationLeafDescriptor, MutationLeafDescriptorRosterValidationError, MutationLeafDescriptorValidationError, MutationLeafSourceScope, ValidatedMutationLeafSourceScope, MutationLeafSourceValidationError,
    MutationOutcomeClass, MutationOwnerLayout, MutationSourceProvenance,
};
//#endregion 🪪️MutationLeafDescriptor

/// 🦠️ One direct mutation leaf with mandatory source-derived metadata and handcrafted behavior.
/// `Op` wraps the owner's concrete leaves; inverses may select a different leaf from that roster.
/// 🌱️ Bound on [`protocol::value::ToValue`]/[`protocol::value::FromValue`], not serde's — the same
/// move [`Inference`] above, [`CompositeMutationKind`] below and `protocol::Mutation` itself already
/// made. Every mutation-leaf payload derives `ToValue`/`FromValue` and no longer derives serde's.
pub trait MutationKind<P, Op>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue
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
    /// gains a per-variant delegating arm (see `🗣️dsl/✨️derive/🦀️.rs` `🔖️Mutations`).
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
// 🎞️ Was declined in an earlier pass (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/📓️directory-spr-serde-
// removal.md`, decline #3): `modified: Vec<ItemPatch<K, Patch>>` needed `ItemPatch<K, Patch>:
// ToValue + FromValue`, which `crate::os_vcs::ItemPatch` did not yet have. It does now — converted.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Eq, ToValue)]
#[value(rename_all = "camelCase")]
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
    pub fn new(id: crate::os_spr::ids::SchemaId, schema_version: crate::os_spr::ids::SchemaVersion, state_class: crate::os_spr::StateClass, leaf: MutationLeafDescriptor, semantics: SemanticDescriptor) -> Result<Self, MutationDescriptorError> {
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

    pub fn id(&self) -> &crate::os_spr::ids::SchemaId {
        &self.id
    }
    pub fn schema_version(&self) -> crate::os_spr::ids::SchemaVersion {
        self.schema_version
    }
    pub fn state_class(&self) -> crate::os_spr::StateClass {
        self.state_class
    }
    pub fn leaf(&self) -> &MutationLeafDescriptor {
        &self.leaf
    }
    pub fn semantics(&self) -> &SemanticDescriptor {
        &self.semantics
    }
    pub fn fingerprint(&self) -> &[u8; 32] {
        &self.fingerprint
    }
}

fn descriptor_fingerprint(id: &crate::os_spr::ids::SchemaId, schema_version: crate::os_spr::ids::SchemaVersion, state_class: crate::os_spr::StateClass, leaf: &MutationLeafDescriptor, semantics: &SemanticDescriptor) -> [u8; 32] {
    #[derive(ToValue)]
    #[value(rename_all = "camelCase")]
    struct Canonical<'a> {
        id: &'a str,
        schema_version: u32,
        state_class: crate::os_spr::StateClass,
        leaf: MutationLeafDescriptor,
        semantics: SemanticDescriptor,
    }
    let canonical = Canonical { id: &id.0, schema_version: schema_version.0, state_class, leaf: *leaf, semantics: *semantics };
    let mut bytes = b"semio.mutation-descriptor/v1\0".to_vec();
    bytes.extend(crate::os_pack::json::to_json_string(&canonical).into_bytes());
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
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn get(&self, id: &str) -> Option<&MutationDescriptor> {
        self.entries.get(id)
    }

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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
pub struct MutationEvent {
    pub mutation_id: crate::os_spr::ids::MutationId,
    pub state_class: crate::os_spr::StateClass,
    pub payload: protocol::value::DslValue,
}
//#endregion 🔖️Events

//#region 🔖️Outcome
/// @emoji 🗂️ The five-channel separation `framework/core`'s `InvocationResult` later maps onto:
/// durable diffs, two UI-visibility tiers, a speculative preview tier, and side-effect events.
#[derive(Clone, Debug, Default, ToValue, FromValue)]
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
        let key = (step.mutation_id.0.clone(), *semio_framework_hash::hash(&step.payload).as_bytes());
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
/// handcrafted kind — `#[derive(CompositeMutation)]` (`🗣️dsl/✨️derive/🦀️.rs`
/// `🔖️CompositeMutation`) wires the delegating `MutationKind` impl from it via the free helpers
/// below.
///
/// LAW (purity): `plan` reads only `base` (never mutates outside state) and drives `planner`
/// exclusively through `call`/`call_foreign` — the free helpers below are the ONLY supported way to
/// fold a plan into a diff/inverse/foreign-step list, so every composite kind observes identical
/// semantics regardless of who calls it.
///
/// Bound on [`protocol::value::ToValue`]/[`protocol::value::FromValue`], not `serde::Serialize`/
/// `serde::de::DeserializeOwned` — mirrors `MutationDiff`'s own supertrait migration (see
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
/// 🔍️research/`), so a plugin/extension implementing this trait never needs `serde` just to
/// satisfy it.
pub trait CompositeMutationKind<P, Op: Mutation<P>>: MutationLeaf + Clone + protocol::value::ToValue + protocol::value::FromValue {
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
#[path = "🧪️tests/📔️registry/🦀️.rs"]
mod registry_fixture;

#[cfg(test)]
#[path = "🧪️tests/🧬️mutation-laws/🦀️.rs"]
mod mutation_laws_fixture;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
