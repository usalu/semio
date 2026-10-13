//! 🚪️ Artifact io capabilities: serializers, deserializers, composers, builders, decomposers, analyzers and derived artifacts.
use std::collections::HashMap;
use store::{os_store::serialization::ArtifactPack, Mutation, MutationDiff};
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
use {semio_framework_os_kernel::io::{Analysis, AnalyzeSource, ComposeError, ComposeFuture, ComposeSource, ComposedArtifact, ComposerEntry, Composition, ErasedComposeSource, IoPayload}, semio_framework_artifact_reference::{ArtifactDialect, Dialect, StandardId, SubsetId}};



// 🗣️ Shared two-axis (locale × terminology) label resolution — replaces the ~25x hand-rolled
// `struct XLabels { .. }` + `const X_LABELS_EN/DE` + `fn x_labels(view_state) -> &'static XLabels`
// pattern duplicated per app, AND the 4-crate hand-rolled `NATIVE_EN/NATIVE_DE/REUSE_EN/REUSE_DE` +
// non-exhaustive `match (terminology, is_de) { ..., (_, true) => ... }` terminology resolvers this
// SDK never covered. `app_labels!` now declares all four cells per field and resolves them via an
// exhaustive match on the generated `Locale`/`Terminology` enums — no catch-all arm is possible, so
// adding a locale or terminology to `🖱️ui/🎚️axes/🔣️.json` breaks every invocation until it supplies that
// cell. See ticket 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND.

/// 🧵️ Directed snapshot conversion out of this dialect into a foreign dialect. One unit
/// struct per `🚪️io/📤️export/🧵️serializers/…` leaf.
pub trait ArtifactSerializer {
    type From;
    type Into;
    const FROM: Dialect;
    const INTO: Dialect;
    /// 🌀️ `io-async-signatures`: async signature, behaviourally unchanged body — every
    /// existing leaf's `serialize` never truly awaits, so its future resolves the moment it
    /// is first polled (see `resolve_ready` in the framework io module).
    /// 🌊️ O1/S4 (sdk-dedyn): plain AFIT, NOT `-> impl Future<..> + Send` — the latter is exactly
    /// the double-future shape R1 bans (a future whose OWN output is a second future); every
    /// real leaf already implements the corrected single-future signature below.
    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError>;
}

/// 🧩️ Directed snapshot conversion from a foreign dialect into this dialect. One unit struct
/// per `🚪️io/📥️import/🧩️deserializers/…` leaf.
pub trait ArtifactDeserializer {
    type From;
    type Into;
    const FROM: Dialect;
    const INTO: Dialect;
    /// 🌀️ `io-async-signatures`: async signature, behaviourally unchanged body — see
    /// `ArtifactSerializer::serialize`'s doc comment for the ready-future guarantee.
    /// 🌊️ O1/S4 (sdk-dedyn): plain AFIT — see `ArtifactSerializer::serialize`'s doc for why.
    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError>;
}

/// 🎹️ Subset-level composer: analyze foreign/native sources, build one snapshot in `WRITES`.
/// Standard- and artifact-level (final) composers aggregate `ComposerEntry` rows value-level
/// (see `ComposerEntry::of` convention on the framework side) rather than via this trait.
pub trait ArtifactComposer: Sized {
    type Snapshot;
    const WRITES: Dialect;
    fn reads() -> &'static [Dialect];
    /// 🌀️ `io-async-signatures`: async signature, behaviourally unchanged body — see
    /// `ArtifactSerializer::serialize`'s doc comment for the ready-future guarantee.
    /// 🌊️ O1/S4 (sdk-dedyn): plain AFIT — see `ArtifactSerializer::serialize`'s doc for why.
    fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError>;
}

/// 🎹️ Erases a typed `ArtifactComposer` into a `ComposerEntry` row for `register_composer_entries`.
/// Lives here (not in `semio_framework_io`) because it needs the `ArtifactComposer` trait, which
/// the lower-layer io module can't see. Erasure round-trips the snapshot through the same
/// `store::os_store::serialization::ArtifactPack` binary codec `ArtifactBuilder::from_binary` already uses.
// 🚫️async: E1 pure struct-builder consumed by `std::sync::Once::call_once`'s fixed sync closure
// (the `subset!` macro's `__subset_registration::register`) — see R9. The erased `compose` field
// it installs stays a real async boundary; only the OUTER registration-row builder is sync.
/// 🚫️async: E4 fn-pointer slot — `ComposerEntry.compose: AsyncComposeFn` is a bare fn pointer;
/// an `async fn` item's pointer type is unnameable, so the erasure itself must be a plain `fn`
/// returning the already-boxed future.
pub fn composer_entry_of<C: ArtifactComposer>() -> ComposerEntry
where
    C::Snapshot: ArtifactPack,
{
    /// 🌉️ bridged via `resolve_ready` — see `composer_entry_of`'s doc: no real
    /// suspension exists in a codec body, and this fn-pointer thunk feeds the
    /// Send-bounded `ComposeFuture` erasure table (R1), which plain AFIT can't satisfy.
    fn erased_compose<C: ArtifactComposer>(sources: &[ErasedComposeSource]) -> ComposeFuture<'_>
    where
        C::Snapshot: ArtifactPack,
    {
        Box::pin(async move {
            let typed_sources: Vec<ComposeSource<'_>> = sources
                .iter()
                .map(|s| ComposeSource {
                    dialect: s.dialect,
                    payload: match &s.payload {
                        IoPayload::Text(t) => AnalyzeSource::Text(t.as_str()),
                        IoPayload::Binary(b) => AnalyzeSource::Binary(b.as_slice()),
                    },
                })
                .collect();
            let composed = C::compose(&typed_sources)?;
            let bytes = ArtifactPack::encode_pack(&composed.snapshot);
            Ok(ComposedArtifact { dialect: C::WRITES, payload: IoPayload::Binary(bytes), diagnostics: composed.diagnostics, confidence: composed.confidence })
        })
    }
    ComposerEntry { writes: C::WRITES, reads: C::reads(), compose: erased_compose::<C> }
}

/// 🧩️ Erases a typed `ArtifactDeserializer` into a `ComposerEntry` row, same shape as
/// `composer_entry_of` but single-read (exactly one source — unlike a composer's multi-source
/// union, a directed conversion has exactly one origin dialect) and via `Dialect`-typed
/// snapshots rather than raw text/binary: the source payload is decoded as `D::From` (its own
/// `store::os_store::serialization::ArtifactPack` codec), `D::deserialize` runs the typed conversion, and the result is
/// re-packed as `D::Into` the same way `composer_entry_of` re-packs its `Composition::snapshot`.
/// Registered through a subset composer's `register()` into the same `IoKey → ComposerEntry`
/// registry composer entries already use.
// 🚫️async: E1 pure struct-builder — see `composer_entry_of`'s doc for why this must be sync.
/// 🚫️async: E4 fn-pointer slot — see `composer_entry_of`'s inner `erased_compose` doc.
pub fn deserializer_entry_of<D: ArtifactDeserializer>() -> ComposerEntry
where
    D::From: ArtifactPack + Send,
    D::Into: ArtifactPack,
{
    /// 🌉️ bridged via `resolve_ready` — see `composer_entry_of`'s doc: no real
    /// suspension exists in a codec body, and this fn-pointer thunk feeds the
    /// Send-bounded `ComposeFuture` erasure table (R1), which plain AFIT can't satisfy.
    fn erased_compose<D: ArtifactDeserializer>(sources: &[ErasedComposeSource]) -> ComposeFuture<'_>
    where
        D::From: ArtifactPack + Send,
        D::Into: ArtifactPack,
    {
        Box::pin(async move {
            let source = match sources {
                [one] => one,
                other => {
                    return Err(ComposeError { message: format!("deserializer {}->{} needs exactly 1 source, got {}", D::FROM.artifact_kind, D::INTO.artifact_kind, other.len()), diagnostics: Vec::new() });
                }
            };
            let bytes = match &source.payload {
                IoPayload::Binary(b) => b.as_slice(),
                IoPayload::Text(_) => {
                    return Err(ComposeError { message: format!("deserializer {}->{} source must be Binary (ArtifactPack-encoded)", D::FROM.artifact_kind, D::INTO.artifact_kind), diagnostics: Vec::new() });
                }
            };
            let from = <D::From as ArtifactPack>::decode_pack(bytes).map_err(|e| ComposeError { message: format!("deserializer {}->{} failed to decode source: {e:?}", D::FROM.artifact_kind, D::INTO.artifact_kind), diagnostics: Vec::new() })?;
            let into =
                ::semio_framework_async::poll::resolve_ready(D::deserialize(&from)).map_err(|e| ComposeError { message: format!("deserializer {}->{} failed: {e:?}", D::FROM.artifact_kind, D::INTO.artifact_kind), diagnostics: Vec::new() })?;
            let bytes = <D::Into as ArtifactPack>::encode_pack(&into);
            Ok(ComposedArtifact { dialect: D::INTO, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: crate::io::Confidence::High })
        })
    }
    ComposerEntry { writes: D::INTO, reads: &[D::FROM], compose: erased_compose::<D> }
}

/// 🧵️ Erases a typed `ArtifactSerializer` into a `ComposerEntry` row — mirror image of
/// `deserializer_entry_of`: writes `S::INTO`, reads exactly `[S::FROM]`, decodes the single
/// source as `S::From`, runs `S::serialize`, re-packs the result as `S::Into`.
// 🚫️async: E1 pure struct-builder — see `composer_entry_of`'s doc for why this must be sync.
/// 🚫️async: E4 fn-pointer slot — see `composer_entry_of`'s inner `erased_compose` doc.
pub fn serializer_entry_of<S: ArtifactSerializer>() -> ComposerEntry
where
    S::From: ArtifactPack + Send,
    S::Into: ArtifactPack,
{
    /// 🌉️ bridged via `resolve_ready` — see `composer_entry_of`'s doc: no real
    /// suspension exists in a codec body, and this fn-pointer thunk feeds the
    /// Send-bounded `ComposeFuture` erasure table (R1), which plain AFIT can't satisfy.
    fn erased_compose<S: ArtifactSerializer>(sources: &[ErasedComposeSource]) -> ComposeFuture<'_>
    where
        S::From: ArtifactPack + Send,
        S::Into: ArtifactPack,
    {
        Box::pin(async move {
            let source = match sources {
                [one] => one,
                other => {
                    return Err(ComposeError { message: format!("serializer {}->{} needs exactly 1 source, got {}", S::FROM.artifact_kind, S::INTO.artifact_kind, other.len()), diagnostics: Vec::new() });
                }
            };
            let bytes = match &source.payload {
                IoPayload::Binary(b) => b.as_slice(),
                IoPayload::Text(_) => {
                    return Err(ComposeError { message: format!("serializer {}->{} source must be Binary (ArtifactPack-encoded)", S::FROM.artifact_kind, S::INTO.artifact_kind), diagnostics: Vec::new() });
                }
            };
            let from = <S::From as ArtifactPack>::decode_pack(bytes).map_err(|e| ComposeError { message: format!("serializer {}->{} failed to decode source: {e:?}", S::FROM.artifact_kind, S::INTO.artifact_kind), diagnostics: Vec::new() })?;
            let into = ::semio_framework_async::poll::resolve_ready(S::serialize(&from)).map_err(|e| ComposeError { message: format!("serializer {}->{} failed: {e:?}", S::FROM.artifact_kind, S::INTO.artifact_kind), diagnostics: Vec::new() })?;
            let bytes = <S::Into as ArtifactPack>::encode_pack(&into);
            Ok(ComposedArtifact { dialect: S::INTO, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: crate::io::Confidence::High })
        })
    }
    ComposerEntry { writes: S::INTO, reads: &[S::FROM], compose: erased_compose::<S> }
}

/// 🏗️ Incremental artifact materializer — snapshot/text/binary in, soft `Diagnostic`s out.
/// `mutate` returns the authoritative mutation outcome alongside the mutated builder. The
/// builder applies `outcome.diff()` once and returns that outcome intact, preserving every
/// diagnostic message without a second semantics computation. Persisted diffs cross the
/// fallible `absorb` boundary so malformed input cannot become an unchanged snapshot.
pub trait ArtifactBuilder: Sized {
    type Snapshot;
    type Mutation: Mutation<Self::Snapshot, Diff = Self::Diff>;
    type Diff;
    fn empty() -> Self;
    fn from_snapshot(snapshot: Self::Snapshot) -> Self;
    fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError>;
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError>;
    fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>);
    fn absorb(self, diff: Self::Diff) -> protocol::MutationApplyResult<Self>;
    fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>>;
}

/// 🎚 Soft confidence for partial decomposition success.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Confidence {
    High,
    Medium,
    Low,
}

/// 📥 One decomposition source blob.
#[derive(Clone, Debug)]
pub enum DecomposeSource<'a> {
    Text(&'a str),
    Binary(&'a [u8]),
}

/// 📦 Decomposition result carrying soft diagnostics (never `Fault` for partial success).
#[derive(Clone, Debug)]
pub struct Decomposition<T> {
    pub parts: T,
    pub confidence: Confidence,
    pub diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
}

/// 📑️ Splits heterogeneous sources into typed parts with soft diagnostics.
pub trait ArtifactDecomposer: Sized {
    type Snapshot;
    type Parts;
    async fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts>;
}

/// 🧐️ Standards/subsets successor to `ArtifactDecomposer` (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO
/// phase 2): read-only analysis that also reports which `Dialect` it recognized, so aggregate
/// composers can route a payload to the right standard (e.g. GIF87a vs GIF89a, `%PDF-1.x`).
/// New artifacts implement this directly; migrated artifacts keep `ArtifactDecomposer` until
/// their own migration wave swaps it -- both traits coexist until the global W16 strict flip.
pub trait ArtifactAnalyzer: Sized {
    type Parts;
    const DIALECT: Dialect;
    /// 👃️ Cheap recognizability probe -- no allocation, no full parse.
    fn sniff(source: &AnalyzeSource<'_>) -> crate::io::Confidence;
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts>;
}

/// 🧐️ Schema-owned analysis hook used by the derived analyzer.
pub trait ArtifactAnalysis: 'static {
    type Parts;

    const DIALECT: Dialect;

    fn sniff(source: &AnalyzeSource<'_>) -> crate::io::Confidence;

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts>;
}

/// 🎹️ IO-owned composition hook used by the derived composer.
pub trait ArtifactComposition: 'static {
    type Snapshot;

    const WRITES: Dialect;

    fn reads() -> &'static [Dialect];

    fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError>;
}

/// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1, Task 5): schema-owned child-composition hook —
/// the derived analogue of `ArtifactComposition` for an artifact whose snapshot is assembled
/// from OTHER artifacts' packs via declared `#[child(kind = "…")]` slots
/// (`semio_framework_schema_composition::ArtifactCompositionFields`), rather than only foreign-dialect IO
/// sources. `Snapshot` deliberately repeats `DerivedArtifactSpec::Snapshot` instead of being
/// read off it — a bare `type Children: ArtifactChildren` on `DerivedArtifactSpec` cannot itself
/// constrain `Children::Snapshot == Self::Snapshot` without a where-clause at every use site, so
/// `DerivedArtifactSpec::Children`'s own bound (below) restates the equality once, centrally.
pub trait ArtifactChildren {
    type Snapshot;
    /// 🪆️ This artifact's declared child slots — `&[]` for a leaf with no `#[child(...)]`
    /// fields (see `NoChildren`).
    fn slots() -> &'static [::semio_framework_schema_composition::ChildSlotSpec];
    /// 🏗️ Assembles `Self::Snapshot` from resolver-supplied child packs, one `(dialect, pack
    /// bytes)` pair per slot instance (`many` slots contribute more than one pair for the SAME
    /// `ChildSlotSpec.kind`) — the composition-side counterpart to `decompose_to_children`.
    fn compose_from_children(parts: &[(ArtifactDialect, Vec<u8>)]) -> Result<Self::Snapshot, ComposeError>;
    /// 📤️ Inverse of `compose_from_children`: the child packs `Self::Snapshot` currently owns,
    /// keyed by their own dialect — the seam `ArtifactChildren`'s own decompose side of the
    /// contract needs so a child's pack can be re-derived/re-exported without re-deriving the
    /// whole parent.
    fn decompose_to_children(snapshot: &Self::Snapshot) -> Vec<(ArtifactDialect, Vec<u8>)>;
}

/// 🍃️ `DerivedArtifactSpec::Children` default for a leaf artifact with zero composition slots —
/// `slots()` is empty, `decompose_to_children` is always empty, and `compose_from_children`
/// unconditionally errors (a leaf has nothing to compose FROM children; the derived composer
/// below never even calls it when `slots()` is empty — see `DerivedArtifactComposer::compose`).
pub struct NoChildren<S>(std::marker::PhantomData<S>);

impl<S> ArtifactChildren for NoChildren<S> {
    type Snapshot = S;

    fn slots() -> &'static [::semio_framework_schema_composition::ChildSlotSpec] {
        &[]
    }

    fn compose_from_children(_parts: &[(ArtifactDialect, Vec<u8>)]) -> Result<Self::Snapshot, ComposeError> {
        Err(ComposeError { message: "this artifact declares no #[child(...)] slots — NoChildren::compose_from_children is unreachable via the derived composer".into(), diagnostics: Vec::new() })
    }

    fn decompose_to_children(_snapshot: &Self::Snapshot) -> Vec<(ArtifactDialect, Vec<u8>)> {
        Vec::new()
    }
}

/// 🧬️ Hook bundle from which all public artifact lifecycle types are derived.
pub trait DerivedArtifactSpec: Sized + 'static {
    type Snapshot;
    type Mutation: Mutation<Self::Snapshot, Diff = Self::Diff>;
    type Diff: MutationDiff<Self::Snapshot>;
    type Construction: ArtifactBuilder<Snapshot = Self::Snapshot, Mutation = Self::Mutation, Diff = Self::Diff>;
    type Analysis: ArtifactAnalysis;
    type Composition: ArtifactComposition<Snapshot = Self::Snapshot>;
    /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1, Task 5): defaults to `NoChildren<Self::Snapshot>`
    /// for every spec built through `derive_artifact_facets!` without a `children: $ty` arm — see
    /// that macro's own doc comment. Stable Rust has no language-level default associated type, so
    /// the "default" lives at the MACRO level (it always emits a concrete `type Children = …;`),
    /// not here; a hand-written `impl DerivedArtifactSpec` must supply one explicitly.
    type Children: ArtifactChildren<Snapshot = Self::Snapshot>;
}

/// 📦️ Analyzer output derived from the artifact snapshot codec.
#[derive(Clone, Debug, Default)]
pub struct DerivedArtifactParts<Snapshot> {
    pub snapshot: Option<Snapshot>,
}

/// 🏗️ Generic materializer derived from a snapshot, semantic mutation, diff, and optional
/// subset validation hook.
pub struct DerivedArtifactBuilder<Spec: DerivedArtifactSpec> {
    construction: Spec::Construction,
}

impl<Spec: DerivedArtifactSpec> Clone for DerivedArtifactBuilder<Spec>
where
    Spec::Construction: Clone,
{
    fn clone(&self) -> Self {
        Self { construction: self.construction.clone() }
    }
}

impl<Spec: DerivedArtifactSpec> std::fmt::Debug for DerivedArtifactBuilder<Spec>
where
    Spec::Construction: std::fmt::Debug,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_tuple("DerivedArtifactBuilder").field(&self.construction).finish()
    }
}

impl<Spec: DerivedArtifactSpec> Default for DerivedArtifactBuilder<Spec> {
    /// 🚫️async: E1 impl of externally-declared `Default` — bridged via `resolve_ready`
    /// (`empty()`'s own chain has no real suspension; io-async-signatures).
    fn default() -> Self {
        Self::empty()
    }
}

impl<Spec: DerivedArtifactSpec> ArtifactBuilder for DerivedArtifactBuilder<Spec> {
    type Snapshot = Spec::Snapshot;
    type Mutation = Spec::Mutation;
    type Diff = Spec::Diff;

    fn empty() -> Self {
        Self { construction: Spec::Construction::empty() }
    }

    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { construction: Spec::Construction::from_snapshot(snapshot) }
    }

    fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Ok(Self { construction: Spec::Construction::from_text(text)? })
    }

    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self { construction: Spec::Construction::from_binary(bytes)? })
    }

    fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
        let (construction, outcome) = self.construction.mutate(mutation);
        (Self { construction }, outcome)
    }

    fn absorb(self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
        self.construction.absorb(diff).map(|construction| Self { construction })
    }

    fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
        self.construction.build()
    }
}

impl<Spec: DerivedArtifactSpec> std::ops::Deref for DerivedArtifactBuilder<Spec> {
    type Target = Spec::Construction;

    fn deref(&self) -> &Self::Target {
        &self.construction
    }
}

impl<Spec: DerivedArtifactSpec> std::ops::DerefMut for DerivedArtifactBuilder<Spec> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.construction
    }
}

/// 🧐️ Codec-backed analyzer derived from the artifact snapshot and dialect coordinate.
pub struct DerivedArtifactAnalyzer<Spec: DerivedArtifactSpec>(std::marker::PhantomData<Spec>);

impl<Spec: DerivedArtifactSpec> ArtifactAnalyzer for DerivedArtifactAnalyzer<Spec> {
    type Parts = <Spec::Analysis as ArtifactAnalysis>::Parts;
    const DIALECT: Dialect = <Spec::Analysis as ArtifactAnalysis>::DIALECT;

    fn sniff(source: &AnalyzeSource<'_>) -> crate::io::Confidence {
        <Spec::Analysis as ArtifactAnalysis>::sniff(source)
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        <Spec::Analysis as ArtifactAnalysis>::analyze(sources)
    }
}

/// 🎹️ Composer derived from native snapshot codecs plus directed foreign IO hooks — and, when
/// `Spec::Children` declares real slots (UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM C1, Task 5), from
/// this artifact's own composed children too. Living HERE (rather than duplicated per macro arm
/// in `derive_artifact_facets!`) means every `DerivedArtifactSpec` — hand-written or
/// macro-generated — gets child-slot-aware `reads()`/`compose()` automatically, for free, the
/// moment its `Children` associated type names real slots.
pub struct DerivedArtifactComposer<Spec: DerivedArtifactSpec>(std::marker::PhantomData<Spec>);

impl<Spec: DerivedArtifactSpec> ArtifactComposer for DerivedArtifactComposer<Spec> {
    type Snapshot = Spec::Snapshot;
    const WRITES: Dialect = <Spec::Composition as ArtifactComposition>::WRITES;

    /// 🪆️ `Spec::Composition`'s own native/foreign reads, UNION each child slot's kind as an
    /// unconstrained-standard/unconstrained-subset `Dialect` (`StandardId("*")`/`SubsetId::ANY`
    /// — a child slot names only a `kind`, per `ChildSlotSpec.kind: &'static str`, never a
    /// specific standard/subset, so "any standard, any subset of this kind" is the literal
    /// reading of "this slot accepts the kind's dialect family").
    ///
    /// `ArtifactComposer::reads()`'s signature is fixed to `&'static [Dialect]` by the
    /// pre-existing (non-composition) trait, while `Spec::Children::slots()` is only knowable at
    /// runtime (a trait associated fn call, not a `const`) — so the union must be computed once
    /// and leaked into a `'static` lifetime rather than built as a `const`.
    ///
    /// ⚠️ Memoized in a `TypeId`-keyed table, NOT in a function-local `static`: a `static`
    /// declared inside a generic function is NOT monomorphized per type parameter — Rust gives
    /// every instantiation of `reads()` the SAME storage. With a plain `OnceLock` there, the
    /// first artifact kind to call `reads()` anywhere in the process would win and hand its
    /// answer to every other artifact kind forever (a composing artifact silently reporting a
    /// leaf's empty reads, or vice versa, depending purely on call order).
    ///
    /// A leaf `Spec::Children = NoChildren<_>` (`slots()` = `&[]`) still degrades to exactly
    /// `Spec::Composition::reads()`.
    ///
    /// 🎯️ Leaked deliberately: one small `Vec` per artifact-kind monomorphization, minted
    /// once for the process lifetime — the `&'static [Dialect]` return type admits no
    /// other option, and the count is bounded by the number of artifact kinds.
    fn reads() -> &'static [Dialect] {
        static UNIONS: std::sync::OnceLock<std::sync::Mutex<HashMap<std::any::TypeId, &'static [Dialect]>>> = std::sync::OnceLock::new();
        let unions = UNIONS.get_or_init(|| std::sync::Mutex::new(HashMap::new()));
        let key = std::any::TypeId::of::<Spec>();
        let composition_reads = <Spec::Composition as ArtifactComposition>::reads();
        let child_slots = <Spec::Children as ArtifactChildren>::slots();
        let mut unions = unions.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        unions.entry(key).or_insert_with(|| {
            let mut reads: Vec<Dialect> = composition_reads.to_vec();
            for slot in child_slots {
                reads.push(Dialect { artifact_kind: slot.kind, standard: StandardId("*"), subset: SubsetId::ANY });
            }
            Box::leak(reads.into_boxed_slice()) as &'static [Dialect]
        })
    }

    /// 🪆️ Routes through `Spec::Children::compose_from_children` when `Spec::Children` declares
    /// real slots AND every source's dialect kind matches one of them (never for a leaf artifact
    /// — `slots()` empty short-circuits straight to the unchanged `Spec::Composition::compose`
    /// path, so nothing about a non-composing artifact's behavior changes); falls through to
    /// `Spec::Composition::compose` otherwise (a mixed/foreign-dialect source set, or no
    /// declared slots at all).
    /// 🌀️ `io-async-signatures`: async signature — behaviourally unchanged; the composition it
    /// delegates to (`ArtifactComposition::compose`, `ArtifactChildren::compose_from_children`)
    /// stays synchronous by design (see the framework io module's `resolve_ready` doc comment).
    fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let slots = <Spec::Children as ArtifactChildren>::slots();
        let all_child_sources = !slots.is_empty() && !sources.is_empty() && sources.iter().all(|source| slots.iter().any(|slot| slot.kind == source.dialect.artifact_kind));
        if all_child_sources {
            let parts: Vec<(ArtifactDialect, Vec<u8>)> = sources
                .iter()
                .map(|source| {
                    let bytes = match &source.payload {
                        AnalyzeSource::Binary(bytes) => bytes.to_vec(),
                        AnalyzeSource::Text(text) => text.as_bytes().to_vec(),
                    };
                    (ArtifactDialect::from(source.dialect), bytes)
                })
                .collect();
            let snapshot = <Spec::Children as ArtifactChildren>::compose_from_children(&parts)?;
            return Ok(Composition { snapshot, confidence: crate::io::Confidence::High, diagnostics: Vec::new() });
        }
        <Spec::Composition as ArtifactComposition>::compose(sources)
    }
}
