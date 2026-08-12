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

//#region 🔖️Mutation
/// @emoji 📦️ Centralized snapshot mutation — one `apply` per technology. Moved from
/// `crate::os_store::MutationDiff` verbatim (only the parameter name `snapshot` → `base`).
pub trait MutationDiff<P>: Clone + Default + serde::Serialize + serde::de::DeserializeOwned {
    fn apply(&self, base: &P) -> P;
    /// @emoji ➕️ Composes `self` (base→mid) with `other` (mid→after) into base→after, in place.
    /// Normative absorb contract (`.claude/plans/the-current-schemas-are-scalable-journal.md`
    /// `## Absorb`): **structural** (operates on the diff's own key/index/field shape, never on
    /// applied snapshot values), **total** (defined for every pair of diffs over the same
    /// artifact, including out-of-range/no-op cases — never panics), **base-free** (no snapshot
    /// parameter; the two diffs alone determine the result), and **sequential-coalesce only**
    /// (this composes two diffs known to have been applied in sequence by the same actor;
    /// concurrent-edit merging is `protocol_crdt::merge_concurrent_diffs`'s job, never this
    /// method's). LAW: `absorb(d1, d2).apply(base) == d2.apply(&d1.apply(base))`, associative
    /// over further absorbs of the same artifact's diff vocabulary.
    fn absorb(&mut self, other: Self);
}

/// @emoji 🧮️ Diff-level algebra for a technology's [`MutationDiff`] type: inverse, state-delta
/// construction, and emptiness. Deliberately a SEPARATE trait from `MutationDiff` (not new
/// methods added to it) — `MutationDiff` already has 51+ repo-wide implementors, so a breaking
/// method addition there would break all of them at once. Follows this crate's own `DiffCodec`
/// precedent below: land the trait standalone in a spine wave, adopt it per-type in later waves
/// via a seeded shrink-only policy allowlist (`POLICY_DIFF_ALGEBRA`), never as a hard bound on
/// `MutationDiff` itself until every implementor is covered.
/// LAWS: `d.inverse(base).apply(&d.apply(base)) == *base`; `Self::between(a, b).apply(a) == *b`;
/// `Self::between(a, a).is_empty()`.
pub trait DiffAlgebra<P>: Sized {
    /// 🔁️ Diff-level undo: the diff that, applied after `self`, restores `base`.
    fn inverse(&self, base: &P) -> Self;
    /// 🧭️ State delta: the diff that, applied to `base`, yields `other`.
    fn between(base: &P, other: &P) -> Self;
    /// 🕳️ Whether this diff changes nothing relative to whatever base it was built against.
    fn is_empty(&self) -> bool;
}

/// @emoji 🔁️ Stored operation: emits a diff and computes inverse from pre-state. Moved from
/// `crate::os_store::Mutation` verbatim except: `mutation_id`/`dependencies`/`author_id` now return the
/// `protocol_core` id newtypes (were bare `String`) and `base_version` now returns
/// `Option<crate::os_spr::ids::ArtifactVersion>` (was a bare `u64` defaulting to `0`, which conflated
/// "no base" with "based on version 0" — `None` fixes that); `conflict_rule`/`state_class` are new
/// defaulted methods so every existing `impl` recompiles unchanged; `reconcile` becomes an instance
/// method (`&self`) returning this crate's own `ReconcileReport` instead of `crate::os_store::SpaceConflict`,
/// so `vcs` maps `ReconcileReport -> SpaceConflict` at its own edge instead of this crate knowing
/// about space types.
pub trait Mutation<P>: Clone + serde::Serialize + serde::de::DeserializeOwned {
    type Diff: MutationDiff<P>;

    fn diff(&self, base: &P) -> Self::Diff;
    fn inverse(&self, base: &P) -> Vec<Self>;

    fn mutation_id(&self) -> Option<crate::os_spr::ids::MutationId> {
        None
    }
    fn dependencies(&self) -> Vec<crate::os_spr::ids::MutationId> {
        Vec::new()
    }
    fn base_version(&self) -> Option<crate::os_spr::ids::ArtifactVersion> {
        None
    }
    fn author_id(&self) -> Option<crate::os_spr::ids::ActorId> {
        None
    }
    fn timestamp(&self) -> Option<crate::os_spr::ids::HybridLogicalTimestamp> {
        None
    }
    fn undo_policy(&self) -> crate::os_spr::UndoPolicy {
        crate::os_spr::UndoPolicy::ExactBaseOnly
    }
    fn merge_strategy(&self) -> crate::os_spr::MergeStrategyKind {
        crate::os_spr::MergeStrategyKind::LwwRegister
    }
    /// @emoji ⚖️ Per-operation conflict declaration; defaults to `Merge(self.merge_strategy())` so a
    /// technology that only overrode `merge_strategy` keeps its exact prior collapse-to-merge shape.
    fn conflict_rule(&self) -> crate::os_spr::ConflictRule {
        crate::os_spr::ConflictRule::Merge(self.merge_strategy())
    }
    /// @emoji 🗂️ Which durability/visibility class this operation's diffs belong to.
    fn state_class(&self) -> crate::os_spr::StateClass {
        crate::os_spr::StateClass::Persistent
    }
    /// @emoji 🤝️ Post-materialization reconciliation pass (e.g. cross-document studio graph checks).
    /// Defaults to a no-op so every existing document kind keeps its exact prior behavior.
    fn reconcile(&self, snapshot: P) -> (P, Vec<ReconcileReport>) {
        (snapshot, Vec::new())
    }
    /// @emoji 🛂️ Pre-apply validation against the current snapshot. Defaults to `Ok`.
    fn validate(&self, _snapshot: &P) -> Result<(), String> {
        Ok(())
    }
}

/// @emoji 📋️ One `reconcile` finding.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReconcileReport {
    pub id: String,
    pub message: String,
    pub severity: ReconcileSeverity,
}

/// @emoji 🚦️ How serious a `ReconcileReport` is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReconcileSeverity {
    Info,
    Warning,
    Blocking,
}
//#endregion 🔖️Mutation

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
    pub fn new(paths: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { paths: paths.into_iter().map(Into::into).collect() }
    }

    fn segments(path: &str) -> Vec<&str> {
        path.split('/').filter(|segment| !segment.is_empty()).collect()
    }

    /// 🔀️ Whether any of `self.paths` shares an ancestor/descendant relationship with `prefix`.
    pub fn intersects_prefix(&self, prefix: &str) -> bool {
        let target = Self::segments(prefix);
        self.paths.iter().any(|path| {
            let own = Self::segments(path);
            let shared = own.len().min(target.len());
            own[..shared] == target[..shared]
        })
    }

    /// 🔀️ Whether any of `prefixes` intersects `self` (see [`intersects_prefix`](Self::intersects_prefix)).
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
/// @emoji 📗️ The closed imperative-verb vocabulary every triad-dir slug and dispatch-enum variant
/// must draw its leading verb from (paired with its past-tense record-name form). Grown from the
/// golden reference schema's naming (`compose/client/schema/graphql/schema.golden.graphql`:
/// `rename`/`changeDescription`/`flatten`/`drag`/`fix`/…) plus the closed structural-transform set
/// (`group`/`ungroup`, `flatten`/`unflatten`, `split`/`merge`, `extract`/`inline`) needed to retire
/// `Set*`/`SetSnapshot`. `set` stays approved for a genuinely single-field setter on an addressed
/// target (`set-layer-visible`) — only whole-document `set-snapshot` is banned, by
/// `policySemanticVocabularyBreaches` in `📜️script.ts`, not by this list. Adding a verb is a spine
/// change here plus the mirrored `POLICY_APPROVED_MUTATION_VERBS` table in `📜️script.ts`.
pub const APPROVED_VERBS: &[(&str, &str)] = &[
    ("add", "Added"),
    ("apply", "Applied"),
    ("bind", "Bound"),
    ("change", "Changed"),
    ("clear", "Cleared"),
    ("connect", "Connected"),
    ("create", "Created"),
    ("delete", "Deleted"),
    ("disconnect", "Disconnected"),
    ("drag", "Dragged"),
    ("duplicate", "Duplicated"),
    ("edit", "Edited"),
    ("extract", "Extracted"),
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
    ("rotate", "Rotated"),
    ("scale", "Scaled"),
    ("set", "Set"),
    ("split", "Split"),
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

/// @emoji 🦠️ One handcrafted mutation kind — implemented once by the payload struct declared in a
/// `🧬️mutations/<kind>/🦠️mutation/🦀️component.rs` leaf. `Op` is the artifact's dispatch enum
/// (`Op: Mutation<P>`); `inverse` returns `Vec<Op>` (not `Vec<Self>`) because a kind's true inverse
/// is frequently a DIFFERENT kind (`create` ↔ `delete`, `group` ↔ `ungroup`). `diff`/`inverse`
/// bodies are required to delegate to the sibling `🔺️diff`/`↩️inverse` leaves — enforced by
/// `policyMutationImplPresenceBreaches`, not by the type system.
pub trait MutationKind<P, Op>: Clone + serde::Serialize + serde::de::DeserializeOwned
where
    Op: Mutation<P>,
{
    const SEMANTICS: SemanticDescriptor;

    fn diff(&self, base: &P) -> <Op as Mutation<P>>::Diff;
    /// Missing/already-absent target ⇒ `Vec::new()` (the semantic replacement for the old
    /// `NoMutation` sentinel variant — there is no "no-op mutation", only an inverse with nothing
    /// to undo).
    fn inverse(&self, base: &P) -> Vec<Op>;
    /// @emoji 🏷️ Human undo/history label, e.g. `Rename piece "a" to "b"`.
    fn label(&self) -> String;
    /// @emoji 🎯️ Structured address of the target inside the artifact (outermost segment first);
    /// empty means whole-artifact scope.
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
    fn validate(&self, _base: &P) -> Result<(), String> {
        Ok(())
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

//#region 🔖️OpText
/// @emoji ⚡️ Handcrafted ONE-LINE textual representation of an operation, implemented once per
/// technology next to its `Mutation` enum. Moved verbatim from `crate::os_store::OpText` (method order
/// flipped to match the frozen contract; behavior unchanged). LAWS: `print_op` output never
/// contains `\n`; `Op::parse_op` recovers an equal operation from `op.print_op()`.
pub trait OpText: Sized {
    fn print_op(&self) -> String;
    fn parse_op(line: &str) -> Result<Self, crate::os_dsl::TextError>;
}
//#endregion 🔖️OpText

//#region 🔖️OpBinary
/// @emoji 🎞️ Binary twin of [`OpText`]: the maximum-token-efficient one-line grammar and this
/// byte encoding are two renderings of the same operation, implemented per technology next to its
/// `Mutation` enum (in practice emitted by `#[derive(crate::os_dsl::DslOps)]` through `crate::os_dsl::op_rt`, the
/// exact mirror of the `ArtifactDsl`/`ArtifactPack` pairing). Layout (owned by the runtime, not
/// by implementors): `format u8 (=1) | variant ordinal varint | record body`. LAWS:
/// `Op::decode_op(op.encode_op()) == op == Op::parse_op(op.print_op())`, and encoding is
/// deterministic — byte-identical output for equal operations.
pub trait OpBinary: Sized {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError>;
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError>;
}
//#endregion 🔖️OpBinary

//#region 🔖️DiffCodec
/// @emoji 🧬️ Grammared twin of [`OpText`]/[`OpBinary`], but for a technology's `MutationDiff::Diff`
/// value rather than its `Mutation`: the W1 foundation of the `handcrafted-grammar-for-every-artifact`
/// program's diff track (design ruling B-R4 at `.claude/plans/the-final-goal-for-jolly-spindle.md`) —
/// today every `*Diff` type is serde-only, this trait promotes a diff to a first-class grammared value
/// exactly like `OpText`/`OpBinary` already did for operations. In practice emitted by
/// `#[derive(crate::os_dsl::DslDiff)]` through the same `RecordSpec`-generation machinery `DslRecord`/
/// `DslArtifact` already use (a diff is structurally just another record). Schema id convention:
/// `"<doc-schema>#diff"`. Deliberately NOT (yet) a supertrait bound of [`MutationDiff`] — W1 only
/// proves the mechanism on a handful of real diff types (tracked in `script.ts`'s
/// `POLICY_DIFF_COMPLETENESS_ALLOWLIST`); wiring it as a hard bound across all diff types is deferred
/// to wave 6 (`## Master wave plan` `W6 — Lane C (B5)`), once every type is covered.
/// LAWS: `Diff::parse_diff(&d.print_diff()) == d`, `Diff::decode_diff(&d.encode_diff()?)? == d`,
/// `print_diff` output never contains `\n`, and `encode_diff` is deterministic.
pub trait DiffCodec: Sized {
    fn print_diff(&self) -> String;
    fn parse_diff(line: &str) -> Result<Self, crate::os_dsl::TextError>;
    fn encode_diff(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError>;
    fn decode_diff(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError>;
}
//#endregion 🔖️DiffCodec

//#region 🔖️Meta
/// @emoji 🧾️ Per-operation causal/undo metadata attached to one `Edit` slot. Moved from
/// `crate::os_store::MutationMeta` (was `vcs/rs/lib.rs` L59) with the id-flavored fields upgraded from bare
/// `String`/`Option<String>` to the `protocol_core` newtypes and `timestamp` upgraded from
/// `Option<HybridLogicalTimestamp>` to a required field (an edit's op always has a tick by the time
/// it is durable).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MutationMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation_id: Option<crate::os_spr::ids::MutationId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<crate::os_spr::ids::MutationId>,
    pub base_version: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<crate::os_spr::ids::ActorId>,
    pub timestamp: crate::os_spr::ids::HybridLogicalTimestamp,
    pub undo_policy: crate::os_spr::UndoPolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_hash: Option<crate::os_spr::ids::PayloadHash>,
    /// @emoji 🗣️ `"<doc-schema>#<kind>"` once the authoring mutation implements [`SemanticMutation`]
    /// — additive, `None` for mutations still on generic vocabulary. Populated by callers (store
    /// replay tightens this once a store's `Mutation: SemanticMutation<P>`, at the final ratchet);
    /// this crate never derives it implicitly to avoid a premature trait-bound change here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_kind: Option<crate::os_spr::ids::SchemaId>,
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
    /// `📡️spr/📜️history/🦀️component.rs`) so it survives persistence and sync.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
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
//#endregion 🔖️Meta

//#region 🔖️Collection
/// 🧬️ Collection identity/patch/diff/ops — single source of truth in VCS (`crate::os_vcs`).
pub use crate::os_vcs::{
    apply_collection_mutation, collection_diff_from_mutation, inverse_collection_mutation, CollectionDiff,
    CollectionMutation, Identified, ItemPatch, Patchable,
};

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

/// @emoji ▶️ Applies a [`NamedTripleDiff`] to an id-keyed `Vec` in place: removals, then patches
/// (in whatever pre-removal/pre-add state remains — patches never target a removed or just-added
/// id by construction), then appends (added items pushed at the end, in order).
pub fn named_apply<K, V, Patch>(items: &mut Vec<V>, diff: &NamedTripleDiff<K, V, Patch>)
where
    K: PartialEq,
    V: Clone + crate::os_vcs::Identified<K> + crate::os_vcs::Patchable<Patch>,
{
    items.retain(|item| !diff.removed.iter().any(|id| item.id() == id));
    for item_patch in &diff.modified {
        if let Some(item) = items.iter_mut().find(|item| item.id() == &item_patch.id) {
            item.apply_patch(&item_patch.patch);
        }
    }
    items.extend(diff.added.iter().cloned());
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

/// @emoji ▶️ Applies an [`IndexedTripleDiff`] in place: BASE-state `modified` patches first, then
/// BASE-state `removed` (descending, so earlier removals don't shift later indices), then
/// FINAL-state `added` (ascending, clamped to the growing length).
pub fn indexed_apply<V, Patch>(items: &mut Vec<V>, diff: &IndexedTripleDiff<V, Patch>)
where
    V: Clone + crate::os_vcs::Patchable<Patch>,
{
    for (index, patch) in &diff.modified {
        if let Some(item) = items.get_mut(*index) {
            item.apply_patch(patch);
        }
    }
    let mut removed: Vec<usize> = diff.removed.clone();
    removed.sort_unstable_by(|a, b| b.cmp(a));
    for index in removed {
        if index < items.len() {
            items.remove(index);
        }
    }
    let mut added: Vec<&(usize, V)> = diff.added.iter().collect();
    added.sort_unstable_by_key(|(index, _)| *index);
    for (index, value) in added {
        let at = (*index).min(items.len());
        items.insert(at, value.clone());
    }
}
//#endregion 🔖️DiffKit

//#region 🔖️Descriptor
/// @emoji 🪪️ A registered operation kind's runtime descriptor: schema identity/version, its
/// `StateClass`/`ConflictRule`, and a content-addressed `fingerprint` over those four fields.
#[derive(Clone, Debug, PartialEq)]
pub struct MutationDescriptor {
    pub id: crate::os_spr::ids::SchemaId,
    pub schema_version: crate::os_spr::ids::SchemaVersion,
    pub state_class: crate::os_spr::StateClass,
    pub conflict_rule: crate::os_spr::ConflictRule,
    pub fingerprint: [u8; 32],
    /// @emoji 🗣️ Semantic identity, present once a mutation kind moves off generic vocabulary —
    /// additive fields, `None` for descriptors registered before the semantic-mutations overhaul.
    /// Not part of `fingerprint` (kept golden-pin stable): the fingerprint identifies the schema
    /// contract, not its human-facing naming.
    pub verb: Option<&'static str>,
    pub entity: Option<&'static str>,
    pub record: Option<&'static str>,
}

impl MutationDescriptor {
    /// @emoji 🏗️ Constructs a descriptor, computing `fingerprint` deterministically from the other
    /// four fields. The contract fixes the struct's shape but not how `fingerprint` is derived; our
    /// choice is a canonical-JSON encoding of `(id, schema_version, state_class, conflict_rule)`
    /// hashed with blake3 — stable across process runs and platforms, pinned by a golden test below.
    pub fn new(id: crate::os_spr::ids::SchemaId, schema_version: crate::os_spr::ids::SchemaVersion, state_class: crate::os_spr::StateClass, conflict_rule: crate::os_spr::ConflictRule) -> Self {
        let fingerprint = descriptor_fingerprint(&id, schema_version, state_class, conflict_rule);
        Self { id, schema_version, state_class, conflict_rule, fingerprint, verb: None, entity: None, record: None }
    }

    /// @emoji 🗣️ Attaches semantic identity (`SemanticDescriptor`'s fields) to an already-built
    /// descriptor — used by `#[derive(Mutations)]`'s generated `register_*_descriptors` calls.
    pub fn with_semantics(mut self, semantics: &SemanticDescriptor) -> Self {
        self.verb = Some(semantics.verb);
        self.entity = Some(semantics.entity);
        self.record = Some(semantics.record);
        self
    }
}

fn descriptor_fingerprint(id: &crate::os_spr::ids::SchemaId, schema_version: crate::os_spr::ids::SchemaVersion, state_class: crate::os_spr::StateClass, conflict_rule: crate::os_spr::ConflictRule) -> [u8; 32] {
    #[derive(serde::Serialize)]
    struct Canonical<'a> {
        id: &'a str,
        schema_version: u32,
        state_class: crate::os_spr::StateClass,
        conflict_rule: crate::os_spr::ConflictRule,
    }
    let canonical = Canonical { id: &id.0, schema_version: schema_version.0, state_class, conflict_rule };
    let bytes = serde_json::to_vec(&canonical).expect("descriptor canonical encoding never fails");
    *blake3::hash(&bytes).as_bytes()
}

static MUTATION_DESCRIPTOR_REGISTRY: std::sync::OnceLock<std::sync::RwLock<std::collections::HashMap<String, MutationDescriptor>>> = std::sync::OnceLock::new();

fn mutation_descriptor_registry() -> &'static std::sync::RwLock<std::collections::HashMap<String, MutationDescriptor>> {
    MUTATION_DESCRIPTOR_REGISTRY.get_or_init(|| std::sync::RwLock::new(std::collections::HashMap::new()))
}

/// @emoji 📝️ Registers (or overwrites) a descriptor by `descriptor.id`. Mirrors
/// `crate::os_store::CodecRegistry`'s `OnceLock<RwLock<HashMap>>` pattern; idempotent, safe to call repeatedly.
pub fn register_mutation_descriptor(descriptor: MutationDescriptor) {
    let mut registry = mutation_descriptor_registry().write().unwrap_or_else(|poisoned| poisoned.into_inner());
    registry.insert(descriptor.id.0.clone(), descriptor);
}

/// @emoji 🔎️ Looks up the descriptor registered for `schema`, if any.
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧸️Fixtures
    // Dummy (P=i64, Op=AddOp) pair: the smallest possible Mutation/MutationDiff/OpText impl,
    // used across every law test below instead of a real technology's op set.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct AddDiff {
        delta: i64,
    }
    impl MutationDiff<i64> for AddDiff {
        fn apply(&self, base: &i64) -> i64 {
            base + self.delta
        }
        fn absorb(&mut self, other: Self) {
            self.delta += other.delta;
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct AddOp {
        delta: i64,
    }
    impl Mutation<i64> for AddOp {
        type Diff = AddDiff;
        fn diff(&self, _base: &i64) -> AddDiff {
            AddDiff { delta: self.delta }
        }
        fn inverse(&self, _base: &i64) -> Vec<Self> {
            vec![AddOp { delta: -self.delta }]
        }
    }
    impl OpText for AddOp {
        fn print_op(&self) -> String {
            format!("add {}", self.delta)
        }
        fn parse_op(line: &str) -> Result<Self, crate::os_dsl::TextError> {
            let rest = line.strip_prefix("add ").ok_or_else(|| crate::os_dsl::TextError::new("expected 'add <n>'", crate::os_dsl::TextSpan::at(1, 1)))?;
            let delta: i64 = rest.trim().parse().map_err(|_| crate::os_dsl::TextError::new("invalid integer", crate::os_dsl::TextSpan::at(1, 1)))?;
            Ok(AddOp { delta })
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    struct Item {
        id: String,
        value: i64,
    }
    impl Identified<String> for Item {
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
        let op = AddOp { delta: 5 };
        let forward = op.diff(&base).apply(&base);
        assert_eq!(forward, 15);
        let [undo] = <[AddOp; 1]>::try_from(op.inverse(&base)).unwrap();
        let restored = undo.diff(&forward).apply(&forward);
        assert_eq!(restored, base);
    }

    #[test]
    fn operation_diff_absorb_accumulates() {
        let mut a = AddDiff { delta: 3 };
        a.absorb(AddDiff { delta: 4 });
        assert_eq!(a.delta, 7);
    }

    #[test]
    fn operation_defaults_are_stable() {
        let op = AddOp { delta: 1 };
        assert_eq!(op.mutation_id(), None);
        assert!(op.dependencies().is_empty());
        assert_eq!(op.base_version(), None);
        assert_eq!(op.author_id(), None);
        assert_eq!(op.timestamp(), None);
        assert_eq!(op.undo_policy(), crate::os_spr::UndoPolicy::ExactBaseOnly);
        assert_eq!(op.merge_strategy(), crate::os_spr::MergeStrategyKind::LwwRegister);
        assert_eq!(op.conflict_rule(), crate::os_spr::ConflictRule::Merge(crate::os_spr::MergeStrategyKind::LwwRegister));
        assert_eq!(op.state_class(), crate::os_spr::StateClass::Persistent);
        assert!(op.validate(&0).is_ok());
        let (snapshot, reports) = op.reconcile(42);
        assert_eq!(snapshot, 42);
        assert!(reports.is_empty());
    }
    //#endregion 🧪️MutationLaws

    //#region 🧪️OpTextLaws
    #[test]
    fn op_text_round_trip() {
        let op = AddOp { delta: -7 };
        let line = op.print_op();
        assert!(!line.contains('\n'));
        let parsed = AddOp::parse_op(&line).expect("round trip parse");
        assert_eq!(parsed, op);
    }

    #[test]
    fn op_text_parse_error_carries_message() {
        let error = AddOp::parse_op("nope").unwrap_err();
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
        let edit = Edit::<AddOp> {
            id: "edit-1".into(),
            actor: Some("actor-1".into()),
            forwards: vec![AddOp { delta: 1 }, AddOp { delta: 2 }],
            inverse: vec![AddOp { delta: -2 }, AddOp { delta: -1 }],
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
            }],
            description: Some("two adds".into()),
            coalesce_key: None,
            sequence_number: 1,
            started_at: "2026-07-27T00:00:00Z".into(),
            finished_at: None,
        };
        let json = serde_json::to_string(&edit).expect("serialize");
        let round_tripped: Edit<AddOp> = serde_json::from_str(&json).expect("deserialize");
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
        named_apply(&mut items, &diff);
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
        indexed_apply(&mut items, &diff);
        assert_eq!(items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["z", "a", "y", "b"]);
        assert_eq!(items[1].value, 101, "modified applies to BASE-state index 0 (item a) before removal/insertion shift it");
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
    fn mutation_descriptor_with_semantics_attaches_without_changing_fingerprint() {
        let semantics = SemanticDescriptor { verb: "rename", entity: "widget", kind: "rename-widget", record: "RenamedWidget" };
        let base = MutationDescriptor::new(crate::os_spr::ids::SchemaId("demo.rename-widget".into()), crate::os_spr::ids::SchemaVersion(1), crate::os_spr::StateClass::Persistent, crate::os_spr::ConflictRule::Merge(crate::os_spr::MergeStrategyKind::LwwRegister));
        let fingerprint_before = base.fingerprint;
        let with_semantics = base.with_semantics(&semantics);
        assert_eq!(with_semantics.fingerprint, fingerprint_before, "attaching semantics must not change the fingerprint");
        assert_eq!(with_semantics.verb, Some("rename"));
        assert_eq!(with_semantics.entity, Some("widget"));
        assert_eq!(with_semantics.record, Some("RenamedWidget"));
    }
    //#endregion 🧪️SemanticsLaws

    //#region 🧪️MutationsDeriveLaws
    // Smallest possible end-to-end proof that `#[derive(dsl_derive::Mutations)]` (📖
    // `🗣️dsl/✨️derive/🦀️component.rs`'s `🔖️Mutations` region) actually wires a triad leaf's
    // `MutationKind` impl into a working `Mutation`/`SemanticMutation` dispatch enum — real
    // artifacts follow this exact shape (payload struct in a `🦠️mutation` leaf, dispatch enum with
    // `#[mutations(...)]`), just with more variants and real leaf files instead of a nested module.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct MiniDoc {
        name: String,
    }

    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct MiniDiff {
        name: Option<String>,
    }
    impl MutationDiff<MiniDoc> for MiniDiff {
        fn apply(&self, base: &MiniDoc) -> MiniDoc {
            MiniDoc { name: self.name.clone().unwrap_or_else(|| base.name.clone()) }
        }
        fn absorb(&mut self, other: Self) {
            if other.name.is_some() {
                self.name = other.name;
            }
        }
    }

    mod rename_mini {
        use super::*;

        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct RenameMini {
            pub new_name: String,
        }
        impl MutationKind<MiniDoc, MiniMutation> for RenameMini {
            const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "mini", kind: "rename-mini", record: "RenamedMini" };
            fn diff(&self, _base: &MiniDoc) -> MiniDiff {
                MiniDiff { name: Some(self.new_name.clone()) }
            }
            fn inverse(&self, base: &MiniDoc) -> Vec<MiniMutation> {
                vec![MiniMutation::RenameMini(RenameMini { new_name: base.name.clone() })]
            }
            fn label(&self) -> String {
                format!("Rename mini to \"{}\"", self.new_name)
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl_derive::Mutations)]
    #[mutations(snapshot = MiniDoc, diff = MiniDiff, schema = "mini.doc")]
    enum MiniMutation {
        RenameMini(rename_mini::RenameMini),
    }

    #[test]
    fn derive_mutations_wires_mutation_and_semantic_mutation() {
        let base = MiniDoc { name: "a".into() };
        let mutation = MiniMutation::RenameMini(rename_mini::RenameMini { new_name: "b".into() });

        let after = mutation.diff(&base).apply(&base);
        assert_eq!(after.name, "b");

        let inverse = mutation.inverse(&base);
        assert_eq!(inverse.len(), 1, "inverse of a single rename is a single rename back");
        let MiniMutation::RenameMini(undo) = &inverse[0];
        assert_eq!(undo.diff(&after).apply(&after), base, "inverse computed from base must restore base");

        assert_eq!(MiniMutation::kinds().len(), 1);
        assert_eq!(MiniMutation::kinds()[0].kind, "rename-mini");
        assert_eq!(mutation.semantics().kind, "rename-mini");
        assert_eq!(mutation.semantics().record, "RenamedMini");
        assert_eq!(mutation.label(), "Rename mini to \"b\"");
        assert!(mutation.target().is_empty(), "MutationKind::target defaults to empty (whole-artifact scope)");

        register_mini_mutation_descriptors();
        let descriptor = mutation_descriptor("mini.doc#rename-mini").expect("derive-generated register fn must register the descriptor");
        assert_eq!(descriptor.verb, Some("rename"));
        assert_eq!(descriptor.entity, Some("mini"));
        assert_eq!(descriptor.record, Some("RenamedMini"));
    }
    //#endregion 🧪️MutationsDeriveLaws

    //#region 🧪️DescriptorLaws
    #[test]
    fn operation_descriptor_fingerprint_is_golden_pinned() {
        let descriptor =
            MutationDescriptor::new(crate::os_spr::ids::SchemaId("note.append".into()), crate::os_spr::ids::SchemaVersion(1), crate::os_spr::StateClass::Persistent, crate::os_spr::ConflictRule::Merge(crate::os_spr::MergeStrategyKind::TextSequence));
        let hex: String = descriptor.fingerprint.iter().map(|b| format!("{b:02x}")).collect();
        // Golden pin computed once from `descriptor_fingerprint`'s canonical-JSON+blake3 encoding;
        // any change to that encoding (or to serde's field order/derives on the id/enum types it
        // hashes) is a breaking change to every persisted `MutationDescriptor` and must update this.
        assert_eq!(hex, "8c6c0b22512540811343d8caa8d147f48e936728105861c4360bb202f626d517");
    }
    //#endregion 🧪️DescriptorLaws

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
    // "abs_value" from an i64 snapshot, reusing the same AddDiff/AddOp pair as the Mutation laws
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
    impl DiffRegions for AddDiff {
        fn touches(&self) -> TouchedPaths {
            if self.delta == 0 {
                TouchedPaths::default()
            } else {
                TouchedPaths::new(["value"])
            }
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
        let noop = AddDiff { delta: 0 };
        assert!(!noop.touches().intersects_any(AddInference::fields()[0].reads));
        assert_eq!(AddInference::infer(&noop.apply(&base)), AddInference::infer(&base));

        let real = AddDiff { delta: 1 };
        assert!(real.touches().intersects_any(AddInference::fields()[0].reads));
        assert_ne!(AddInference::infer(&real.apply(&base)), AddInference::infer(&base));
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
        let outcome: CommandOutcome<AddDiff> = CommandOutcome::default();
        assert!(outcome.persistent.is_empty());
        assert!(outcome.shared_ui.is_empty());
        assert!(outcome.local_ui.is_empty());
        assert!(outcome.preview.is_empty());
        assert!(outcome.effects.is_empty());
    }

    #[test]
    fn operation_event_serde_round_trip() {
        let event = MutationEvent { mutation_id: crate::os_spr::ids::MutationId("op-1".into()), state_class: crate::os_spr::StateClass::Effect, payload: serde_json::json!({ "kind": "toast", "text": "saved" }) };
        let json = serde_json::to_string(&event).expect("serialize");
        let round_tripped: MutationEvent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round_tripped, event);
    }
    //#endregion 🧪️OutcomeLaws
}
//#endregion 🧪️Tests
