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
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationApplyError {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<String>,
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
pub trait MutationDiff<P>: Clone + Default + serde::Serialize + serde::de::DeserializeOwned {
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
pub trait Mutation<P>: Clone + serde::Serialize + serde::de::DeserializeOwned {
    type Diff: MutationDiff<P>;

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
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationOutcome<D> {
    diff: D,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    messages: Vec<MutationMessage>,
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
    /// `📡️spr/📜️history/🦀️component.rs`) so it survives persistence and sync.
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
//#endregion 🔖️Origin
