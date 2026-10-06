// #region io
//! 🚪️ Dialect vocabulary and typed artifact-to-artifact IO dispatch registry.
//! Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase 2 (standards/subsets). Lives beside
//! `🔺️mesh` (not `os`) so plugins and the OS product share one definition without an
//! inverted dependency — same reasoning as mesh's now-retired legacy format enum.

use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_diagnostic::Diagnostic;
use std::collections::{BTreeMap, HashMap};
use std::sync::RwLock;

//#region 🔖️Dialect
/// 🧬️ `StandardId`/`SubsetId`/`Dialect`/`ArtifactDialect` moved verbatim to
/// `🚪️io/🧬️schema/🦀️.rs` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-A
/// task 1) so the vocabulary — and `ArtifactDialect::to_coordinate`/`parse_coordinate`, the ONE
/// dialect-coordinate codec in the repo — has a single definition site regardless of which crate
/// mounts this file. Re-exported here unchanged so every existing reference in this file (and
/// every downstream `io::Dialect`/`io::ArtifactDialect` import) keeps resolving to the exact same
/// type.
pub use crate::io_schema::{ArtifactDialect, Dialect, StandardId, SubsetId};

pub use crate::sqlite_snapshot;
//#endregion 🔖️Dialect

//#region 🔖️ArtifactRef
/// 🧬️ `ArtifactKindId`/`is_canonical_artifact_kind`/`ArtifactRef` moved verbatim to
/// `🚪️io/🧬️schema/🦀️.rs` alongside `🔖️Dialect` above — see that region's doc comment.
pub use crate::io_schema::{is_canonical_artifact_kind, ArtifactKindId, ArtifactRef};
//#endregion 🔖️ArtifactRef

//#region 🔐️CodecContracts
//#region 🔒️Diagnostics
/// 📍️ A source-local byte range, with optional human-facing line and column coordinates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceSpan {
    pub resource: String,
    pub byte_start: u64,
    pub byte_end: u64,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

/// 🚫️ A source span that cannot identify one exact bounded source range.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SourceSpanError {
    MissingResource,
    ReversedBytes { start: u64, end: u64 },
    IncompleteCoordinate,
    ZeroCoordinate,
}

impl SourceSpan {
    /// 🔎️ Validates the range and its optional human-facing coordinate as one owned boundary.
    pub async fn validate(&self) -> Result<(), SourceSpanError> {
        if self.resource.trim().is_empty() {
            return Err(SourceSpanError::MissingResource);
        }
        if self.byte_start > self.byte_end {
            return Err(SourceSpanError::ReversedBytes { start: self.byte_start, end: self.byte_end });
        }
        match (self.line, self.column) {
            (None, None) => Ok(()),
            (Some(line), Some(column)) if line > 0 && column > 0 => Ok(()),
            (Some(_), Some(_)) => Err(SourceSpanError::ZeroCoordinate),
            _ => Err(SourceSpanError::IncompleteCoordinate),
        }
    }
}

/// 🪝️ Owned source syntax retained by a lossless artifact result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnchoredSyntax {
    pub anchor: String,
    pub span: SourceSpan,
    pub bytes: Vec<u8>,
}

impl AnchoredSyntax {
    /// 🔎️ Confirms an anchor retains exactly the source range it claims.
    pub async fn validate(&self) -> Result<(), CodecFailure> {
        if self.anchor.trim().is_empty() {
            return Err(CodecFailure::error("io.codec.empty-anchor", "source anchor is empty").await);
        }
        if let Err(error) = self.span.validate().await {
            return Err(CodecFailure::error("io.codec.invalid-source-span", format!("invalid anchor {:?}: {error:?}", self.anchor)).await);
        }
        let width = self.span.byte_end - self.span.byte_start;
        if width != self.bytes.len() as u64 {
            return Err(CodecFailure::error("io.codec.anchor-width", format!("anchor {:?} has {} bytes for source width {width}", self.anchor, self.bytes.len())).await);
        }
        Ok(())
    }
}

/// 🫥️ Owned unsupported syntax whose bytes must survive lossless processing unchanged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpaqueExtension {
    pub kind: String,
    pub source: AnchoredSyntax,
}

/// 🗿️ Codec-owned semantic data together with lossless lexical and opaque source records.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactCodecResult<T> {
    pub semantic: T,
    pub anchors: Vec<AnchoredSyntax>,
    pub opaque_extensions: Vec<OpaqueExtension>,
}

impl<T> ArtifactCodecResult<T> {
    /// 🔐️ Validates retained lossless records and their deterministic anchor ownership.
    pub async fn validate_lossless(&self) -> Result<(), CodecFailure> {
        let mut anchors = std::collections::BTreeSet::new();
        for anchor in &self.anchors {
            if !anchors.insert(anchor.anchor.clone()) {
                return Err(CodecFailure::error("io.codec.duplicate-anchor", format!("duplicate anchor {:?}", anchor.anchor)).await);
            }
            anchor.validate().await?;
        }
        let mut opaque_extensions = std::collections::BTreeSet::new();
        for extension in &self.opaque_extensions {
            if extension.kind.trim().is_empty() {
                return Err(CodecFailure::error("io.codec.empty-opaque-kind", "opaque extension kind is empty").await);
            }
            extension.source.validate().await?;
            let key = (extension.kind.as_str(), extension.source.anchor.as_str(), extension.source.span.byte_start, extension.source.span.byte_end);
            if !opaque_extensions.insert(key) {
                return Err(CodecFailure::error("io.codec.duplicate-opaque-extension", format!("duplicate opaque extension {:?} at {:?}", extension.kind, extension.source.anchor)).await);
            }
        }
        Ok(())
    }

    /// 📏️ Orders retained opaque records independently of insertion or registry order.
    pub async fn canonical_opaque_extensions(&self) -> Vec<&OpaqueExtension> {
        let mut extensions = self.opaque_extensions.iter().collect::<Vec<_>>();
        extensions.sort_by(|a, b| (a.kind.as_str(), a.source.anchor.as_str(), a.source.span.byte_start, a.source.span.byte_end).cmp(&(b.kind.as_str(), b.source.anchor.as_str(), b.source.span.byte_start, b.source.span.byte_end)));
        extensions
    }

    /// 🧭️ Applies the deterministic source-record order required by canonical codec output.
    pub async fn canonicalize(&mut self) {
        self.anchors.sort_by(|a, b| (a.anchor.as_str(), a.span.byte_start, a.span.byte_end).cmp(&(b.anchor.as_str(), b.span.byte_start, b.span.byte_end)));
        self.opaque_extensions.sort_by(|a, b| (a.kind.as_str(), a.source.anchor.as_str(), a.source.span.byte_start, a.source.span.byte_end).cmp(&(b.kind.as_str(), b.source.anchor.as_str(), b.source.span.byte_start, b.source.span.byte_end)));
    }

    /// 🧭️ Verifies the representation promise attached to a completed codec result.
    pub async fn validate_representation(&self, representation: CodecRepresentation) -> Result<(), CodecFailure> {
        self.validate_lossless().await?;
        if representation == CodecRepresentation::Canonical {
            let mut canonical = self.anchors.iter().collect::<Vec<_>>();
            canonical.sort_by_key(|anchor| (anchor.anchor.as_str(), anchor.span.byte_start, anchor.span.byte_end));
            if canonical != self.anchors.iter().collect::<Vec<_>>() {
                return Err(CodecFailure::error("io.codec.noncanonical-anchors", "canonical result anchors are not deterministically ordered").await);
            }
            if self.canonical_opaque_extensions().await != self.opaque_extensions.iter().collect::<Vec<_>>() {
                return Err(CodecFailure::error("io.codec.noncanonical-opaque-extensions", "canonical opaque extensions are not deterministically ordered").await);
            }
        }
        Ok(())
    }
}

/// 🚦️ Severity independent of any parser or UI implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodecSeverity {
    Error,
    Warning,
    Information,
}

/// 🧾️ Structured codec diagnostic with an exact source range whenever one exists.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodecDiagnostic {
    pub code: String,
    pub severity: CodecSeverity,
    pub message: String,
    pub primary_span: Option<SourceSpan>,
    pub related_spans: Vec<SourceSpan>,
}

/// ⚠️ A failed codec operation. Failures remain structured so hosts never need to parse text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodecFailure {
    pub diagnostics: Vec<CodecDiagnostic>,
}

impl CodecFailure {
    /// 🛑️ Builds one structured error without coupling this contract to a parser implementation.
    pub async fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self { diagnostics: vec![CodecDiagnostic { code: code.into(), severity: CodecSeverity::Error, message: message.into(), primary_span: None, related_spans: Vec::new() }] }
    }
}

/// 📦️ A successful value and every non-fatal diagnostic produced while obtaining it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodecOutput<T> {
    pub value: T,
    pub diagnostics: Vec<CodecDiagnostic>,
}

/// 🧩️ Common result boundary for every resource, payload, and artifact codec operation.
pub type CodecResult<T> = Result<CodecOutput<T>, CodecFailure>;
//#endregion 🔒️Diagnostics

//#region ⏱️Policies
/// 🎯️ The deterministic representation promise requested from a codec.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodecRepresentation {
    Canonical,
    Lossless,
}

/// 🧯️ Cross-thread cancellation owned by the caller, not a runtime or codec dependency.
#[derive(Clone, Debug, Default)]
pub struct CancellationToken(std::sync::Arc<std::sync::atomic::AtomicBool>);

impl CancellationToken {
    // 🚫️async: E1 pure accessor consumed by external-trait impls (`Default::default` on
    // `DecodePolicy`/`EncodePolicy`, whose signature is fixed by the `Default` trait) — see R9.
    /// 🌱️ Creates an active cancellation token.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🛑️ Cancels every context sharing this token.
    pub async fn cancel(&self) {
        self.0.store(true, std::sync::atomic::Ordering::Release);
    }

    /// 🔎️ Reads cancellation with acquire semantics.
    pub async fn is_cancelled(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Acquire)
    }
}

/// 📏️ Finite caller-owned resource ceilings for one codec invocation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodecLimits {
    pub max_read_bytes: u64,
    pub max_written_bytes: u64,
    pub max_work_units: u64,
    pub max_allocations: u64,
    pub max_recursion_depth: u32,
}

impl Default for CodecLimits {
    fn default() -> Self {
        Self { max_read_bytes: 64 * 1024 * 1024, max_written_bytes: 64 * 1024 * 1024, max_work_units: 20_000_000, max_allocations: 64 * 1024 * 1024, max_recursion_depth: 256 }
    }
}

/// 📥️ Decode policy, including the representation promise, bounded work, and cancellation.
#[derive(Clone, Debug)]
pub struct DecodePolicy {
    pub representation: CodecRepresentation,
    pub limits: CodecLimits,
    pub cancellation: CancellationToken,
}

impl Default for DecodePolicy {
    fn default() -> Self {
        Self { representation: CodecRepresentation::Canonical, limits: CodecLimits::default(), cancellation: CancellationToken::new() }
    }
}

/// 📤️ Encode policy, including the representation promise, bounded work, and cancellation.
#[derive(Clone, Debug)]
pub struct EncodePolicy {
    pub representation: CodecRepresentation,
    pub limits: CodecLimits,
    pub cancellation: CancellationToken,
}

impl Default for EncodePolicy {
    fn default() -> Self {
        Self { representation: CodecRepresentation::Canonical, limits: CodecLimits::default(), cancellation: CancellationToken::new() }
    }
}

/// 📊️ Consumption recorded by one codec invocation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CodecConsumption {
    pub read_bytes: u64,
    pub written_bytes: u64,
    pub work_units: u64,
    pub allocations: u64,
    pub recursion_depth: u32,
    pub peak_recursion_depth: u32,
}

/// 🧮️ Mutable budget guard supplied to codec implementations.
#[derive(Clone, Debug)]
pub struct CodecBudget {
    limits: CodecLimits,
    cancellation: CancellationToken,
    consumption: CodecConsumption,
}

impl CodecBudget {
    /// 🌱️ Creates a fresh counter from caller-owned limits and cancellation.
    pub async fn new(limits: CodecLimits, cancellation: CancellationToken) -> Self {
        Self { limits, cancellation, consumption: CodecConsumption::default() }
    }

    /// 🔎️ Returns work already consumed by this invocation.
    pub async fn consumption(&self) -> CodecConsumption {
        self.consumption
    }

    /// 🛑️ Fails immediately when the caller has cancelled this operation.
    pub async fn ensure_active(&self) -> Result<(), CodecFailure> {
        if self.cancellation.is_cancelled().await {
            return Err(CodecFailure::error("io.codec.cancelled", "codec operation cancelled").await);
        }
        Ok(())
    }

    /// 📥️ Charges streaming input bytes before retaining or processing them.
    pub async fn charge_read(&mut self, bytes: u64) -> Result<(), CodecFailure> {
        self.ensure_active().await?;
        Self::charge(&mut self.consumption.read_bytes, bytes, self.limits.max_read_bytes, "read-bytes").await
    }

    /// 📤️ Charges output bytes before a host sink accepts them.
    pub async fn charge_write(&mut self, bytes: u64) -> Result<(), CodecFailure> {
        self.ensure_active().await?;
        Self::charge(&mut self.consumption.written_bytes, bytes, self.limits.max_written_bytes, "written-bytes").await
    }

    /// ⚙️ Charges deterministic implementation work units.
    pub async fn charge_work(&mut self, units: u64) -> Result<(), CodecFailure> {
        self.ensure_active().await?;
        Self::charge(&mut self.consumption.work_units, units, self.limits.max_work_units, "work-units").await
    }

    /// 🧱️ Charges a logical allocation before allocating externally supplied data.
    pub async fn charge_allocation(&mut self, allocations: u64) -> Result<(), CodecFailure> {
        self.ensure_active().await?;
        Self::charge(&mut self.consumption.allocations, allocations, self.limits.max_allocations, "allocations").await
    }

    /// 🪆️ Enters a bounded parser or encoder recursion frame.
    pub async fn enter_recursion(&mut self) -> Result<(), CodecFailure> {
        self.ensure_active().await?;
        if self.consumption.recursion_depth >= self.limits.max_recursion_depth {
            return Err(CodecFailure::error("io.codec.recursion-exhausted", format!("recursion budget {} exhausted", self.limits.max_recursion_depth)).await);
        }
        self.consumption.recursion_depth += 1;
        self.consumption.peak_recursion_depth = self.consumption.peak_recursion_depth.max(self.consumption.recursion_depth);
        Ok(())
    }

    /// 🪆️ Leaves one parser or encoder recursion frame.
    pub async fn leave_recursion(&mut self) -> Result<(), CodecFailure> {
        self.ensure_active().await?;
        match self.consumption.recursion_depth.checked_sub(1) {
            Some(depth) => self.consumption.recursion_depth = depth,
            None => return Err(CodecFailure::error("io.codec.recursion-underflow", "codec left a recursion frame it did not enter").await),
        }
        Ok(())
    }

    async fn charge(used: &mut u64, increment: u64, limit: u64, resource: &str) -> Result<(), CodecFailure> {
        let next = match used.checked_add(increment) {
            Some(next) => next,
            None => return Err(CodecFailure::error("io.codec.budget-overflow", format!("{resource} counter overflow")).await),
        };
        if next > limit {
            return Err(CodecFailure::error("io.codec.budget-exhausted", format!("{resource} budget {limit} exhausted by request for {increment}")).await);
        }
        *used = next;
        Ok(())
    }
}

/// 🔍️ Decode invocation state; codecs charge it instead of owning hidden global limits.
/// 🧬️ Generic over the host-owned resolver `R` — `ResourceResolver` is an open host-extension
/// point with no closed implementor set, so `📌️important.md` R11 de-dyns its holder with a
/// generic parameter instead of `Arc<dyn ResourceResolver>`.
#[derive(Clone)]
pub struct DecodeContext<R: ResourceResolver> {
    pub policy: DecodePolicy,
    pub budget: CodecBudget,
    resolver: Option<std::sync::Arc<R>>,
}

impl<R: ResourceResolver> DecodeContext<R> {
    /// 🌱️ Starts one decode invocation.
    pub async fn new(policy: DecodePolicy) -> Self {
        let budget = CodecBudget::new(policy.limits.clone(), policy.cancellation.clone());
        Self { policy, budget: budget.await, resolver: None }
    }

    /// 🔗️ Starts one decode invocation with the host-owned external resource resolver.
    pub async fn with_resolver(policy: DecodePolicy, resolver: std::sync::Arc<R>) -> Self {
        let budget = CodecBudget::new(policy.limits.clone(), policy.cancellation.clone());
        Self { policy, budget: budget.await, resolver: Some(resolver) }
    }

    /// 🌊️ Creates the only codec-facing bounded view over a payload source. `S` is trivially
    /// generic (R11 part a) — no runtime-chosen implementor lives at this call site.
    pub async fn source<'source, S: PayloadSource>(&'source mut self, source: &'source mut S) -> CodecResult<BoundedPayloadSource<'source, S>> {
        if let Err(error) = source.span().await.validate().await {
            return Err(CodecFailure::error("io.codec.invalid-source-span", format!("invalid payload source span: {error:?}")).await);
        }
        self.budget.ensure_active().await?;
        Ok(CodecOutput { value: BoundedPayloadSource { source, budget: &mut self.budget, policy: &self.policy }, diagnostics: Vec::new() })
    }

    /// 🔗️ Resolves one external source exclusively through the host-owned resolver. The resolved
    /// source's concrete type is the resolver's own `Self::Source` associated type (R11 part b) —
    /// the openness lives at the resolver implementor, never boxed here.
    pub async fn resolve<'context>(&'context mut self, request: &ResourceRequest) -> CodecResult<ResolvedPayloadSource<'context, R::Source>> {
        self.budget.ensure_active().await?;
        let resolver = match self.resolver.clone() {
            Some(resolver) => resolver,
            None => return Err(CodecFailure::error("io.codec.resource-resolver-unavailable", "decode context has no resource resolver").await),
        };
        let resolved = resolver.resolve_decode(request).await?;
        if let Err(error) = resolved.value.span().await.validate().await {
            return Err(CodecFailure::error("io.codec.invalid-source-span", format!("invalid resolved payload source span: {error:?}")).await);
        }
        Ok(CodecOutput { value: ResolvedPayloadSource { source: resolved.value, budget: &mut self.budget, policy: &self.policy }, diagnostics: resolved.diagnostics })
    }

    /// ✅️ Finalizes a decode result only when its requested representation is valid.
    pub async fn finalize_result<T>(&mut self, mut result: ArtifactCodecResult<T>) -> CodecResult<ArtifactCodecResult<T>> {
        self.budget.charge_work(1).await?;
        if self.policy.representation == CodecRepresentation::Canonical {
            result.canonicalize().await;
        }
        result.validate_representation(self.policy.representation).await?;
        Ok(CodecOutput { value: result, diagnostics: Vec::new() })
    }
}

/// 🔍️ Encode invocation state; codecs charge it instead of owning hidden global limits.
/// 🧬️ Generic over the host-owned resolver `R` — see `DecodeContext`'s doc comment and
/// `📌️important.md` R11.
#[derive(Clone)]
pub struct EncodeContext<R: ResourceResolver> {
    pub policy: EncodePolicy,
    pub budget: CodecBudget,
    resolver: Option<std::sync::Arc<R>>,
}

impl<R: ResourceResolver> EncodeContext<R> {
    /// 🌱️ Starts one encode invocation.
    pub async fn new(policy: EncodePolicy) -> Self {
        let budget = CodecBudget::new(policy.limits.clone(), policy.cancellation.clone());
        Self { policy, budget: budget.await, resolver: None }
    }

    /// 🔗️ Starts one encode invocation with the host-owned external resource resolver.
    pub async fn with_resolver(policy: EncodePolicy, resolver: std::sync::Arc<R>) -> Self {
        let budget = CodecBudget::new(policy.limits.clone(), policy.cancellation.clone());
        Self { policy, budget: budget.await, resolver: Some(resolver) }
    }

    /// 🚰️ Creates the only codec-facing bounded view over a payload sink. `S` is trivially
    /// generic (R11 part a).
    pub async fn sink<'sink, S: PayloadSink>(&'sink mut self, sink: &'sink mut S) -> CodecResult<BoundedPayloadSink<'sink, S>> {
        self.budget.ensure_active().await?;
        Ok(CodecOutput { value: BoundedPayloadSink { sink, budget: &mut self.budget }, diagnostics: Vec::new() })
    }

    /// 🔗️ Resolves one host-owned encoded resource destination. The resolved sink's concrete
    /// type is the resolver's own `Self::Sink` associated type (R11 part b).
    pub async fn resolve<'context>(&'context mut self, request: &ResourceRequest) -> CodecResult<ResolvedPayloadSink<'context, R::Sink>> {
        self.budget.ensure_active().await?;
        let resolver = match self.resolver.clone() {
            Some(resolver) => resolver,
            None => return Err(CodecFailure::error("io.codec.resource-resolver-unavailable", "encode context has no resource resolver").await),
        };
        let resolved = resolver.resolve_encode(request).await?;
        Ok(CodecOutput { value: ResolvedPayloadSink { sink: resolved.value, budget: &mut self.budget }, diagnostics: resolved.diagnostics })
    }

    /// ✅️ Finalizes an encode result only when host policy has made it canonical or lossless-valid.
    pub async fn finalize_result<T>(&mut self, mut result: ArtifactCodecResult<T>) -> CodecResult<ArtifactCodecResult<T>> {
        self.budget.charge_work(1).await?;
        if self.policy.representation == CodecRepresentation::Canonical {
            result.canonicalize().await;
        }
        result.validate_representation(self.policy.representation).await?;
        Ok(CodecOutput { value: result, diagnostics: Vec::new() })
    }
}
//#endregion ⏱️Policies

//#region 🌊️Resources
/// 🔎️ Bounded type-identification result produced before full decode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PayloadSniff {
    pub media_type: Option<String>,
    pub confidence: Confidence,
    pub diagnostics: Vec<CodecDiagnostic>,
}

/// 🧭️ Optional random-access view over a streaming source.
pub trait RandomAccessPayload: Send + Sync {
    fn len(&self) -> impl std::future::Future<Output = CodecResult<u64>> + Send;
    fn is_empty(&self) -> impl std::future::Future<Output = CodecResult<bool>> + Send {
        async {
            let output = self.len().await?;
            Ok(CodecOutput { value: output.value == 0, diagnostics: output.diagnostics })
        }
    }
    fn read_at(&self, offset: u64, output: &mut [u8]) -> impl std::future::Future<Output = CodecResult<usize>> + Send;
}

/// 🌊️ A forward-only source which may additionally expose random access. `RandomAccess` is an
/// associated type rather than `Option<&dyn RandomAccessPayload>` (`📌️important.md` R11 part b —
/// the return position of a default trait method needs the associated type at the OWNING trait,
/// which `dyn_enum_close!` cannot annotate here). Implementors that never support random access
/// still declare it (any `RandomAccessPayload` works — the default body never constructs one).
pub trait PayloadSource: Send {
    type RandomAccess: RandomAccessPayload;

    fn span(&self) -> impl std::future::Future<Output = SourceSpan> + Send;
    fn read_chunk(&mut self, output: &mut [u8]) -> impl std::future::Future<Output = CodecResult<usize>> + Send;
    fn random_access(&self) -> impl std::future::Future<Output = Option<&Self::RandomAccess>> + Send {
        async { None }
    }
}

/// 🚰️ A streaming output sink.
pub trait PayloadSink: Send {
    fn write_chunk(&mut self, input: &[u8]) -> impl std::future::Future<Output = CodecResult<()>> + Send;
}

/// 🔗️ Resource request expressed without tying codecs to a filesystem, HTTP client, or host API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceRequest {
    pub locator: String,
    pub expected_media_type: Option<String>,
}

/// 🧭️ Resolves a resource into the common streaming/random-access source contract. `Source`/`Sink`
/// push the runtime choice to the implementor (`📌️important.md` R11 part b): a resolver that
/// genuinely needs to hand back several different source kinds declares its own closed enum over
/// them — generated with `dyn_enum_close!` if it wants to — and names that enum here. Nothing in
/// this trait is boxed or `dyn`.
pub trait ResourceResolver: Send + Sync {
    type Source: PayloadSource;
    type Sink: PayloadSink;

    fn resolve_decode(&self, request: &ResourceRequest) -> impl std::future::Future<Output = CodecResult<Self::Source>> + Send;
    fn resolve_encode(&self, request: &ResourceRequest) -> impl std::future::Future<Output = CodecResult<Self::Sink>> + Send;
}

/// 🔒️ Host-owned bounded random-access view exposed to codecs. Borrows the invocation's budget
/// and policy directly rather than the whole `DecodeContext`, so this stays non-generic over the
/// resolver `R` even though `DecodeContext<R>` is generic (`📌️important.md` R11) — only `A`, the
/// trivially-generic source type (R11 part a), needs naming here.
pub struct BoundedRandomAccessPayload<'a, A: RandomAccessPayload> {
    source: &'a A,
    budget: &'a mut CodecBudget,
    policy: &'a DecodePolicy,
}

impl<A: RandomAccessPayload> BoundedRandomAccessPayload<'_, A> {
    /// 📏️ Reads the declared resource length while checking cancellation.
    pub async fn len(&mut self) -> CodecResult<u64> {
        self.budget.ensure_active().await?;
        self.budget.charge_work(1).await?;
        self.source.len().await
    }

    /// 🎯️ Reads at one exact offset without letting codecs bypass read/work limits.
    pub async fn read_at(&mut self, offset: u64, output: &mut [u8]) -> CodecResult<usize> {
        let allowed = self.permitted(output.len()).await?;
        let result = self.source.read_at(offset, &mut output[..allowed]).await?;
        self.charge_result(result.value, allowed).await
    }

    async fn permitted(&mut self, requested: usize) -> Result<usize, CodecFailure> {
        self.budget.ensure_active().await?;
        self.budget.charge_work(1).await?;
        let remaining = self.policy.limits.max_read_bytes.saturating_sub(self.budget.consumption().await.read_bytes);
        let permitted = if remaining > usize::MAX as u64 { requested } else { requested.min(remaining as usize) };
        if requested > 0 && permitted == 0 {
            return Err(CodecFailure::error("io.codec.budget-exhausted", "read-bytes budget exhausted").await);
        }
        Ok(permitted)
    }

    async fn charge_result(&mut self, read: usize, permitted: usize) -> CodecResult<usize> {
        if read > permitted {
            return Err(CodecFailure::error("io.codec.source-overread", format!("payload source returned {read} bytes after being limited to {permitted}")).await);
        }
        self.budget.charge_read(read as u64).await?;
        Ok(CodecOutput { value: read, diagnostics: Vec::new() })
    }
}

/// 🔒️ Host-owned bounded streaming view exposed to codecs. Generic over `S: PayloadSource`
/// (R11 part a) and, like `BoundedRandomAccessPayload`, non-generic over the resolver `R`.
pub struct BoundedPayloadSource<'a, S: PayloadSource> {
    source: &'a mut S,
    budget: &'a mut CodecBudget,
    policy: &'a DecodePolicy,
}

impl<S: PayloadSource> BoundedPayloadSource<'_, S> {
    /// 📥️ Reads one bounded streaming chunk and charges its actual retained bytes.
    pub async fn read_chunk(&mut self, output: &mut [u8]) -> CodecResult<usize> {
        let allowed = self.permitted(output.len()).await?;
        let result = self.source.read_chunk(&mut output[..allowed]).await?;
        self.charge_result(result.value, allowed).await
    }

    /// 🎯️ Opens a bounded random-access view when the source supports it.
    pub async fn random_access(&mut self) -> Option<BoundedRandomAccessPayload<'_, S::RandomAccess>> {
        let source = self.source.random_access().await?;
        Some(BoundedRandomAccessPayload { source, budget: self.budget, policy: self.policy })
    }

    async fn permitted(&mut self, requested: usize) -> Result<usize, CodecFailure> {
        self.budget.ensure_active().await?;
        self.budget.charge_work(1).await?;
        let remaining = self.policy.limits.max_read_bytes.saturating_sub(self.budget.consumption().await.read_bytes);
        let permitted = if remaining > usize::MAX as u64 { requested } else { requested.min(remaining as usize) };
        if requested > 0 && permitted == 0 {
            return Err(CodecFailure::error("io.codec.budget-exhausted", "read-bytes budget exhausted").await);
        }
        Ok(permitted)
    }

    async fn charge_result(&mut self, read: usize, permitted: usize) -> CodecResult<usize> {
        if read > permitted {
            return Err(CodecFailure::error("io.codec.source-overread", format!("payload source returned {read} bytes after being limited to {permitted}")).await);
        }
        self.budget.charge_read(read as u64).await?;
        Ok(CodecOutput { value: read, diagnostics: Vec::new() })
    }
}

/// 🔒️ A resolver-owned source whose only codec-facing operations share this context's budget.
/// `S` is the resolver's own `ResourceResolver::Source` associated type — the runtime choice made
/// by the implementor, never a box (`📌️important.md` R11 part b).
pub struct ResolvedPayloadSource<'a, S: PayloadSource> {
    source: S,
    budget: &'a mut CodecBudget,
    policy: &'a DecodePolicy,
}

impl<S: PayloadSource> ResolvedPayloadSource<'_, S> {
    /// 📥️ Reads one bounded streaming chunk from a resolved source.
    pub async fn read_chunk(&mut self, output: &mut [u8]) -> CodecResult<usize> {
        let allowed = self.permitted(output.len()).await?;
        let result = self.source.read_chunk(&mut output[..allowed]).await?;
        self.charge_result(result.value, allowed).await
    }

    /// 🎯️ Opens bounded random access when this resolved source supports it.
    pub async fn random_access(&mut self) -> Option<BoundedRandomAccessPayload<'_, S::RandomAccess>> {
        let source = self.source.random_access().await?;
        Some(BoundedRandomAccessPayload { source, budget: self.budget, policy: self.policy })
    }

    async fn permitted(&mut self, requested: usize) -> Result<usize, CodecFailure> {
        self.budget.ensure_active().await?;
        self.budget.charge_work(1).await?;
        let remaining = self.policy.limits.max_read_bytes.saturating_sub(self.budget.consumption().await.read_bytes);
        let permitted = if remaining > usize::MAX as u64 { requested } else { requested.min(remaining as usize) };
        if requested > 0 && permitted == 0 {
            return Err(CodecFailure::error("io.codec.budget-exhausted", "read-bytes budget exhausted").await);
        }
        Ok(permitted)
    }

    async fn charge_result(&mut self, read: usize, permitted: usize) -> CodecResult<usize> {
        if read > permitted {
            return Err(CodecFailure::error("io.codec.source-overread", format!("payload source returned {read} bytes after being limited to {permitted}")).await);
        }
        self.budget.charge_read(read as u64).await?;
        Ok(CodecOutput { value: read, diagnostics: Vec::new() })
    }
}

/// 🔒️ Host-owned bounded streaming sink exposed to codecs. Generic over `S: PayloadSink`
/// (R11 part a); write_chunk never reads `policy`, so unlike the source side this only borrows
/// the budget.
pub struct BoundedPayloadSink<'a, S: PayloadSink> {
    sink: &'a mut S,
    budget: &'a mut CodecBudget,
}

impl<S: PayloadSink> BoundedPayloadSink<'_, S> {
    /// 📤️ Writes one bounded chunk while charging work, allocation, and output bytes first.
    pub async fn write_chunk(&mut self, input: &[u8]) -> CodecResult<()> {
        self.budget.charge_work(1).await?;
        self.budget.charge_allocation(input.len() as u64).await?;
        self.budget.charge_write(input.len() as u64).await?;
        self.sink.write_chunk(input).await
    }
}

/// 🔒️ A resolver-owned sink whose only codec-facing operation shares this context's budget.
/// `S` is the resolver's own `ResourceResolver::Sink` associated type (R11 part b).
pub struct ResolvedPayloadSink<'a, S: PayloadSink> {
    sink: S,
    budget: &'a mut CodecBudget,
}

impl<S: PayloadSink> ResolvedPayloadSink<'_, S> {
    /// 📤️ Writes one budgeted chunk to a resolved destination.
    pub async fn write_chunk(&mut self, input: &[u8]) -> CodecResult<()> {
        self.budget.charge_work(1).await?;
        self.budget.charge_allocation(input.len() as u64).await?;
        self.budget.charge_write(input.len() as u64).await?;
        self.sink.write_chunk(input).await
    }
}
//#endregion 🌊️Resources

//#region 🧬️Codecs
/// 📦️ Codec for a transport-level payload. It owns sniffing, bounded streaming, and output policy.
/// `Source`/`Sink` name the concrete `BoundedPayloadSource`/`BoundedPayloadSink` type parameters
/// this codec streams through (`📌️important.md` R11 part a — trivially generic, no boxing).
pub trait PayloadCodec: Send + Sync {
    type Payload;
    type Source: PayloadSource;
    type Sink: PayloadSink;

    fn sniff(&self, source: &mut BoundedPayloadSource<'_, Self::Source>) -> impl std::future::Future<Output = CodecResult<PayloadSniff>> + Send;
    fn decode_payload(&self, source: &mut BoundedPayloadSource<'_, Self::Source>) -> impl std::future::Future<Output = CodecResult<Self::Payload>> + Send;
    fn encode_payload(&self, payload: &Self::Payload, sink: &mut BoundedPayloadSink<'_, Self::Sink>) -> impl std::future::Future<Output = CodecResult<()>> + Send;
}

/// 🗿️ Semantic artifact codec layered over a transport `PayloadCodec` implementation.
/// `decode_artifact`/`encode_artifact` are themselves generic over the resolver `R` (rather than
/// the trait as a whole) — a codec body works with whichever resolver its caller supplies, it
/// never needs to know the resolver's concrete type ahead of time (`📌️important.md` R11 part b).
pub trait ArtifactCodec: PayloadCodec {
    type Artifact;

    fn dialect(&self) -> impl std::future::Future<Output = &ArtifactDialect> + Send;
    fn decode_artifact<R: ResourceResolver>(&self, payload: Self::Payload, context: &mut DecodeContext<R>) -> impl std::future::Future<Output = CodecResult<Self::Artifact>> + Send;
    fn encode_artifact<R: ResourceResolver>(&self, artifact: &Self::Artifact, context: &mut EncodeContext<R>) -> impl std::future::Future<Output = CodecResult<Self::Payload>> + Send;
}
//#endregion 🧬️Codecs
//#endregion 🔐️CodecContracts

//#region 🔖️ComposeTypes
/// 📥 One typed compose source: a foreign or native dialect plus its payload.
#[derive(Clone, Debug)]
pub enum AnalyzeSource<'a> {
    Text(&'a str),
    Binary(&'a [u8]),
}

/// 🎚 Soft confidence for partial analysis/composition success.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub enum Confidence {
    High,
    Medium,
    Low,
}

/// 📦 Analysis result carrying the dialect the analyzer determined it read, with soft diagnostics.
#[derive(Clone, Debug)]
pub struct Analysis<T> {
    pub parts: T,
    pub dialect: Dialect,
    pub confidence: Confidence,
    pub diagnostics: Vec<Diagnostic>,
}

/// 🎹 One typed source for composition: a foreign or native dialect plus its payload.
pub struct ComposeSource<'a> {
    pub dialect: Dialect,
    pub payload: AnalyzeSource<'a>,
}

/// 🎹 Composition result: one snapshot in the composer's `WRITES` dialect.
#[derive(Clone, Debug)]
pub struct Composition<T> {
    pub snapshot: T,
    pub confidence: Confidence,
    pub diagnostics: Vec<Diagnostic>,
}

/// ⚠️ Composition failed: no compatible source dialect, or every candidate errored.
#[derive(Clone, Debug, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub struct ComposeError {
    pub message: String,
    pub diagnostics: Vec<Diagnostic>,
}
//#endregion 🔖️ComposeTypes

//#region 🔖️ErasedRegistry
/// 🧾️ Erased payload crossing composer/registry boundaries (dispatch, UI, wire).
pub use crate::io_schema::IoPayload;

/// 🎹️ One erased compose source for the type-erased registry entry points.
pub struct ErasedComposeSource {
    pub dialect: Dialect,
    pub payload: IoPayload,
}

/// 📦️ Erased composition result.
pub struct ComposedArtifact {
    pub dialect: Dialect,
    pub payload: IoPayload,
    pub diagnostics: Vec<Diagnostic>,
    pub confidence: Confidence,
}

/// 🌀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (`io-async-signatures`): the boxed, pinned future
/// every erased compose/deserialize/serialize hop now returns — named once so none of the 163
/// `ComposerEntry` construction sites (nor `composer_entry_of`/`deserializer_entry_of`/
/// `serializer_entry_of`) has to repeat the raw `Pin<Box<dyn Future<..>>>` spelling. Borrows
/// `sources` for its own lifetime so a hop that never truly suspends still doesn't have to
/// allocate an owned copy just to satisfy the signature.
pub type ComposeFuture<'a> = std::pin::Pin<Box<dyn std::future::Future<Output = Result<ComposedArtifact, ComposeError>> + Send + 'a>>;

/// 🌀️ `ComposerEntry.compose`'s function-POINTER type (still a plain, non-capturing `fn` — every
/// existing `compose: some_fn` construction site keeps working unchanged) whose signature now
/// returns a future instead of the future's own eventual output.
pub type AsyncComposeFn = for<'a> fn(&'a [ErasedComposeSource]) -> ComposeFuture<'a>;

/// 🌀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (`io-thunks`, R1/R2 E4): wraps an `async fn(&[
/// ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError>` in a macro-generated, non-
/// capturing `fn` thunk that coerces to `AsyncComposeFn` — an `async fn` item's pointer type is
/// unnameable, so `compose: some_async_fn` can never coerce into a `ComposerEntry.compose` row
/// directly. The wrapped function itself keeps the literal `async` keyword the universal-async
/// decree (`📌️important.md` O1) requires; only the macro-generated `__thunk` is sync, and it is
/// invisible in source (R2 E4's "macro-generated" escape hatch — no per-site `// 🚫️async:` tag
/// needed). `$crate::` so this also works from every OTHER crate's `ComposerEntry` construction
/// site once `fleet-codemods` rolls it out fleet-wide (the 163 sites this ticket's `io-thunks`
/// packet does NOT touch — see `compose-thunk-rewrite.py` in the ticket folder).
#[macro_export]
macro_rules! compose_thunk {
    ($f:path) => {{
        // 🚫️async: E4 fn-pointer slot
        fn __thunk<'a>(s: &'a [$crate::ErasedComposeSource]) -> $crate::ComposeFuture<'a> {
            Box::pin($f(s))
        }
        __thunk
    }};
}

/// 🌀️ `compose_thunk!`'s twin for `io_mechanism::IoEntry.run` — wraps an
/// `async fn(&IoPayload) -> IoResult<IoPayload>` in a macro-generated E4 thunk that resolves it
/// synchronously via `resolve_ready` (unlike `compose_thunk!`, `IoEntry.run` is genuinely
/// non-future-returning: every codec body here runs to completion without suspending, see
/// `resolve_ready`'s own doc comment).
#[macro_export]
macro_rules! io_run_thunk {
    ($f:path) => {{
        // 🚫️async: E4 fn-pointer slot
        fn __thunk(payload: &$crate::io_schema::IoPayload) -> $crate::io_schema::IoResult<$crate::io_schema::IoPayload> {
            ::semio_framework_async::poll::resolve_ready($f(payload))
        }
        __thunk
    }};
}

/// 🌀️ `io_run_thunk!`'s twin for `io_mechanism::IoEntry.sniff` — wraps an
/// `async fn(&IoPayload) -> Confidence`.
#[macro_export]
macro_rules! io_sniff_thunk {
    ($f:path) => {{
        // 🚫️async: E4 fn-pointer slot
        fn __thunk(payload: &$crate::io_schema::IoPayload) -> $crate::io_schema::Confidence {
            ::semio_framework_async::poll::resolve_ready($f(payload))
        }
        __thunk
    }};
}



/// 🎹️ Type-erased composer vtable row. Built by a plugin's composer facet from its typed
/// `ArtifactComposer` impl (SDK trait lives in the plugin crate, this struct only carries the
/// erased shape so the registry never needs the plugin's concrete snapshot types).
pub struct ComposerEntry {
    pub writes: Dialect,
    pub reads: &'static [Dialect],
    pub compose: AsyncComposeFn,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub enum IoDirection {
    Import,
    Export,
}

/// 🗝️ Owned mirror of two dialects + direction — the registry key. Owned (not `&'static`) so it
/// can be built from runtime UI input (format kind strings) as well as static composer entries.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IoKey {
    pub artifact_kind: String,
    pub standard: String,
    pub subset: String,
    pub direction: IoDirection,
    pub format_kind: String,
    pub format_standard: String,
    pub format_subset: String,
}

impl IoKey {
    /// 🗝️ Build a key from an (owner, counterpart) pair already resolved to the right
    /// perspective by the caller -- see the two call sites in `register_composer_entries`.
    fn from_owner_counterpart(owner: Dialect, counterpart: Dialect, direction: IoDirection) -> Self {
        IoKey {
            artifact_kind: owner.artifact_kind.to_string(),
            standard: owner.standard.0.to_string(),
            subset: owner.subset.0.to_string(),
            direction,
            format_kind: counterpart.artifact_kind.to_string(),
            format_standard: counterpart.standard.0.to_string(),
            format_subset: counterpart.subset.0.to_string(),
        }
    }
}

static IO_REGISTRY: std::sync::OnceLock<RwLock<BTreeMap<IoKey, &'static ComposerEntry>>> = std::sync::OnceLock::new();

fn io_registry() -> &'static RwLock<BTreeMap<IoKey, &'static ComposerEntry>> {
    IO_REGISTRY.get_or_init(|| RwLock::new(BTreeMap::new()))
}

/// ⚠️ A deterministic IO-key ownership collision. The registry never replaces the first owner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IoRegistryConflict {
    pub key: IoKey,
}

/// 🚫️ A registry lock is unavailable after a failed writer panicked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IoRegistryUnavailable {
    pub registry: &'static str,
}

/// ⚠️ A composer registration either conflicts or cannot safely acquire its registry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IoRegistryRegistrationError {
    Conflict(Box<IoRegistryConflict>),
    Unavailable(IoRegistryUnavailable),
}

impl std::fmt::Display for IoRegistryRegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conflict(conflict) => write!(f, "io registry conflict for key {:?}", conflict.key),
            Self::Unavailable(unavail) => write!(f, "io registry unavailable: {}", unavail.registry),
        }
    }
}
impl std::error::Error for IoRegistryRegistrationError {}

fn same_composer_entry(left: &ComposerEntry, right: &ComposerEntry) -> bool {
    left.writes == right.writes && left.reads == right.reads && std::ptr::fn_addr_eq(left.compose, right.compose)
}

fn composer_entries_by_key<'entry>(entries: impl IntoIterator<Item = &'entry ComposerEntry>) -> Result<BTreeMap<IoKey, &'entry ComposerEntry>, IoRegistryRegistrationError> {
    let mut proposed: BTreeMap<IoKey, &'entry ComposerEntry> = BTreeMap::new();
    for entry in entries {
        for &source in entry.reads {
            let keys = [IoKey::from_owner_counterpart(entry.writes, source, IoDirection::Import), IoKey::from_owner_counterpart(source, entry.writes, IoDirection::Export)];
            for key in keys {
                if let Some(existing) = proposed.get(&key) {
                    if !same_composer_entry(existing, entry) {
                        return Err(IoRegistryRegistrationError::Conflict(Box::new(IoRegistryConflict { key })));
                    }
                } else {
                    proposed.insert(key, entry);
                }
            }
        }
    }
    Ok(proposed)
}

fn validate_composer_entries(registry: &BTreeMap<IoKey, &'static ComposerEntry>, proposed: &BTreeMap<IoKey, &'static ComposerEntry>) -> Result<(), IoRegistryRegistrationError> {
    for (key, entry) in proposed {
        if let Some(existing) = registry.get(key) {
            if !same_composer_entry(existing, entry) {
                return Err(IoRegistryRegistrationError::Conflict(Box::new(IoRegistryConflict { key: key.clone() })));
            }
        }
    }
    Ok(())
}

/// 🔬️ Verifies a static composer table against all established keys without mutating the registry.
pub async fn preflight_composer_entries(entries: &'static [ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
    preflight_composer_entry_refs(&entries.iter().collect::<Vec<_>>()).await
}

/// 🔬️ Verifies independently declared static composers as one atomic candidate set.
pub async fn preflight_composer_entry_refs(entries: &[&'static ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    preflight_composer_entry_refs_in_assembly(&assembly, entries).await
}

/// 🔬️ Verifies composers while one artifact assembly owns the shared publication barrier.
pub async fn preflight_composer_entry_refs_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, entries: &[&'static ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
    let proposed = composer_entries_by_key(entries.iter().copied())?;
    let registry = io_registry().read().map_err(|_| IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "io-composer" }))?;
    validate_composer_entries(&registry, &proposed)
}

/// 📌️ Registers one artifact's composer entries atomically. Re-registering the exact static entry
/// is idempotent; a different entry for any exact key fails and leaves the registry unchanged.
pub fn register_composer_entries(entries: &'static [ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
    register_composer_entry_refs(&entries.iter().collect::<Vec<_>>())
}

/// 📌️ Registers independently declared static composers as one all-or-nothing candidate set.
pub fn register_composer_entry_refs(entries: &[&'static ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    register_composer_entry_refs_in_assembly(&assembly, entries)
}

/// 📌️ Publishes preflighted composers while one artifact assembly owns the shared barrier.
pub fn register_composer_entry_refs_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, entries: &[&'static ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
    let proposed = composer_entries_by_key(entries.iter().copied())?;
    let mut reg = io_registry().write().map_err(|_| IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "io-composer" }))?;
    validate_composer_entries(&reg, &proposed)?;
    for (key, entry) in proposed {
        reg.entry(key).or_insert(entry);
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct IoResolveError {
    pub message: String,
    pub candidates: Vec<IoKey>,
    pub unavailable: Option<IoRegistryUnavailable>,
}

/// 🔎️ Look up the composer entry for one exact (artifact/standard/subset, direction,
/// format/standard/subset) coordinate. No silent defaulting — callers with a partially-specified
/// query (unknown standard/subset) must enumerate `dialects_for` first and choose explicitly.
pub async fn resolve(key: &IoKey) -> Result<&'static ComposerEntry, IoResolveError> {
    let reg = io_registry().read().map_err(|_| IoResolveError { message: "io composer registry unavailable".to_string(), candidates: Vec::new(), unavailable: Some(IoRegistryUnavailable { registry: "io-composer" }) })?;
    reg.get(key).copied().ok_or_else(|| IoResolveError {
        message: format!("no composer registered for {}/{}/{} {:?} {}/{}/{}", key.artifact_kind, key.standard, key.subset, key.direction, key.format_kind, key.format_standard, key.format_subset),
        candidates: reg.keys().filter(|k| k.artifact_kind == key.artifact_kind).cloned().collect(),
        unavailable: None,
    })
}

/// 📚️ Lists every dialect one artifact can move data through in a given direction.
pub async fn dialects_for(artifact_kind: &str, direction: IoDirection) -> Result<Vec<Dialect>, IoRegistryUnavailable> {
    let reg = io_registry().read().map_err(|_| IoRegistryUnavailable { registry: "io-composer" })?;
    let mut dialects: Vec<Dialect> = reg.iter().filter(|(k, _)| k.artifact_kind == artifact_kind && k.direction == direction).map(|(_, entry)| entry.writes).collect();
    dialects.sort_by_key(|dialect| ArtifactDialect::from(*dialect).to_coordinate());
    dialects.dedup();
    Ok(dialects)
}

/// 🗝️ Every registered `IoKey` for one artifact_kind + direction, WITH the owner's real
/// standard/subset (not a hardcoded default) -- callers that used to build a key by hand and
/// guess `standard: "1", subset: "*"` should enumerate this instead and pick explicitly, the same
/// "no silent defaulting" policy `resolve` already documents.
pub async fn io_keys_for(artifact_kind: &str, direction: IoDirection) -> Result<Vec<IoKey>, IoRegistryUnavailable> {
    let reg = io_registry().read().map_err(|_| IoRegistryUnavailable { registry: "io-composer" })?;
    Ok(reg.keys().filter(|key| key.artifact_kind == artifact_kind && key.direction == direction).cloned().collect())
}

/// 📇️ Every registered composer entry, erased to owned dialects -- the shape the WIT
/// `list-artifact-dialects` guest export mirrors verbatim (one row per distinct `writes` entry
/// registered locally, each carrying the full `reads` list).
pub async fn list_composer_entries() -> Result<Vec<(ArtifactDialect, Vec<ArtifactDialect>)>, IoRegistryUnavailable> {
    let reg = io_registry().read().map_err(|_| IoRegistryUnavailable { registry: "io-composer" })?;
    let mut seen: BTreeMap<String, &'static ComposerEntry> = BTreeMap::new();
    for entry in reg.values() {
        seen.entry(ArtifactDialect::from(entry.writes).to_coordinate()).or_insert(*entry);
    }
    Ok(seen.into_values().map(|entry| (ArtifactDialect::from(entry.writes), entry.reads.iter().map(|&d| ArtifactDialect::from(d)).collect())).collect())
}

//#region 🔖️Dispatch
/// 🌉️ The seam that makes cross-plugin compose real. `io_dispatch` ALWAYS tries the local
/// registry first (the fast, common, same-crate case every existing caller already used via
/// `resolve`+`compose`); on a local miss it falls through to a settable hook instead of failing
/// outright. Native shells install a router-backed hook (resolves through every loaded plugin,
/// see the plugin host's `IoRouter`); a wasm guest installs a hook that calls the `io-compose`
/// host import, which the host then routes to whichever OTHER plugin actually owns the key. Until
/// a hook is installed (or in a context with nothing to fall through to, e.g. a bare unit test)
/// this behaves exactly like `resolve`+`compose` did before -- existing single-crate callers are
/// unaffected by this seam's mere existence.
/// 🔌️ A host-owned erased fallback executable for cross-plugin IO dispatch. Async to match
/// `AsyncComposeFn` (io-async-signatures): `None` means "no opinion, fall through to the
/// `IoResolveError`"; `Some(future)` is the fallback's own compose future, exactly as if it were
/// a local `ComposerEntry.compose` call.
pub type IoFallback = dyn for<'a> Fn(&'a IoKey, &'a [ErasedComposeSource]) -> Option<ComposeFuture<'a>> + Send + Sync;

/// 🪪️ A fallback descriptor plus the exact executable allocation that owns it.
#[derive(Clone)]
pub struct IoFallbackDispatcher {
    pub identity: String,
    pub dispatch: std::sync::Arc<IoFallback>,
}

static IO_FALLBACK: std::sync::OnceLock<IoFallbackDispatcher> = std::sync::OnceLock::new();

/// ⚠️ A fallback identity was already registered with a different descriptor or executable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IoFallbackRegistrationError {
    pub established_identity: String,
    pub incoming_identity: String,
}

/// 🔌️ Install the fallback dispatcher. Call exactly once, before any `io_dispatch` call that
/// should reach it (host boot / the guest runtime ensure `plugin_exports!` generates). Re-registration is idempotent
/// only for the same descriptor and executable identity; every other race is a typed conflict.
pub async fn set_io_fallback_dispatcher(dispatcher: IoFallbackDispatcher) -> Result<(), IoFallbackRegistrationError> {
    match IO_FALLBACK.get() {
        Some(existing) if existing.identity == dispatcher.identity && std::sync::Arc::ptr_eq(&existing.dispatch, &dispatcher.dispatch) => Ok(()),
        Some(existing) => Err(IoFallbackRegistrationError { established_identity: existing.identity.clone(), incoming_identity: dispatcher.identity }),
        None => match IO_FALLBACK.set(dispatcher) {
            Ok(()) => Ok(()),
            Err(incoming) => match IO_FALLBACK.get() {
                Some(existing) => Err(IoFallbackRegistrationError { established_identity: existing.identity.clone(), incoming_identity: incoming.identity }),
                None => Err(IoFallbackRegistrationError { established_identity: incoming.identity.clone(), incoming_identity: incoming.identity }),
            },
        },
    }
}

/// 🎹️ Resolve `key` locally; on a local miss, ask the installed fallback (if any). Returns the
/// SAME `IoResolveError`-shaped message as a local-only `resolve` when nothing (local or
/// fallback) has the key, so existing error-message-matching callers don't need to change.
pub async fn io_dispatch(key: &IoKey, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    match resolve(key).await {
        Ok(entry) => validate_composed_subset((entry.compose)(sources).await?).await,
        Err(local_err) => match IO_FALLBACK.get().and_then(|dispatcher| (dispatcher.dispatch)(key, sources)) {
            Some(fallback) => validate_composed_subset(fallback.await?).await,
            None => Err(ComposeError { message: local_err.message, diagnostics: Vec::new() }),
        },
    }
}

async fn validate_composed_subset(mut composed: ComposedArtifact) -> Result<ComposedArtifact, ComposeError> {
    run_subset_validation(composed.dialect, &composed.payload, &mut composed.diagnostics).await.map_err(|error| ComposeError { message: format!("subset validation failed: {error:?}"), diagnostics: Vec::new() })?;
    Ok(composed)
}

/// 🌉️🌉️ Two-hop compose: resolve+compose `hub` from `sources` via `io_dispatch`, then feed that
/// hop's `ComposedArtifact` as the SINGLE source for resolving+composing `target`, also via
/// `io_dispatch` (so both hops get the fallback dispatcher and subset validation `io_dispatch`
/// already gives a single-hop compose, for free). Built for the domain-plugin hub-and-spoke shape
/// (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): a domain
/// artifact composes into a semio subset (the hub), then the semio subset composes into the real
/// target format — never more than 2 hops.
///
/// **Max-2-hops invariant**: `hub` MUST be resolvable directly from `sources` (hop 1), and `target`
/// MUST be resolvable from hub's OWN composed output alone (hop 2) — never from `sources` again and
/// never chained through a third key. This is a deliberate ceiling, not an oversight: an unbounded
/// transitive walk over the registry can cycle (A resolves via B resolves via A) or blow up
/// combinatorially as more dialects register, and neither failure mode is diagnosable from a single
/// stack frame the way a fixed 2-hop call is. Callers that need a longer chain compose it themselves
/// as repeated `io_compose_via`/`io_dispatch` calls, each one an explicit, auditable hop.
pub async fn io_compose_via(hub: &IoKey, target: &IoKey, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    let hub_composed = io_dispatch(hub, sources).await?;
    let hop_source = ErasedComposeSource { dialect: hub_composed.dialect, payload: hub_composed.payload };
    io_dispatch(target, std::slice::from_ref(&hop_source)).await
}
//#endregion 🔖️Dispatch

//#region 🔖️SubsetValidator
/// 🛡️ SDK trait (D5, ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION):
/// one subset's own conformance checker (e.g. PDF/A-2b's "no `/Encrypt`, no JS/Launch actions,
/// `OutputIntent` present, fonts embedded"). Unlike `ArtifactComposer`/`ArtifactAnalyzer` (which
/// live in the plugin crate because they need typed `Snapshot` visibility this generic/erased io
/// module doesn't have), `SubsetValidator` can live directly HERE: its signature is already
/// erased over `IoPayload` (mirroring `ComposerEntry.compose`'s own `fn(&[ErasedComposeSource])`
/// erasure) -- a concrete artifact implements it by decoding its own typed `Snapshot` out of the
/// payload internally (via `store::ArtifactPack`/`ArtifactDsl`, exactly like `ComposerEntry`'s
/// erasure already does one layer up), so this module never needs to know the concrete type.
pub trait SubsetValidator {
    const DIALECT: Dialect;
    fn validate(payload: &IoPayload) -> impl std::future::Future<Output = Vec<Diagnostic>> + Send;
}

/// 🧾️ Type-erased subset-validator vtable row -- the registry stores this, mirroring how
/// `ComposerEntry` stores a plain `fn` pointer rather than a trait object.
pub struct SubsetValidatorEntry {
    pub dialect: Dialect,
    pub validate: fn(&IoPayload) -> Vec<Diagnostic>,
}

/// 🎹️ Erases a typed `SubsetValidator` impl into a `SubsetValidatorEntry` row -- the
/// `ComposerEntry::of::<C>()`-style helper for this trait.
pub fn subset_validator_entry_of<V: SubsetValidator>() -> SubsetValidatorEntry {
    // 🚫️async: E4 fn-pointer slot — `SubsetValidatorEntry.validate` is a bare `fn` pointer;
    // `SubsetValidator::validate` stays `async fn` (a real trait method) and this thunk drives it
    // to completion synchronously via `resolve_ready`, same pattern as the `IoEntry` constructors.
    fn validate<V: SubsetValidator>(payload: &IoPayload) -> Vec<Diagnostic> {
        ::semio_framework_async::poll::resolve_ready(V::validate(payload))
    }
    SubsetValidatorEntry { dialect: V::DIALECT, validate: validate::<V> }
}

static SUBSET_VALIDATOR_REGISTRY: std::sync::OnceLock<RwLock<BTreeMap<ArtifactDialect, &'static SubsetValidatorEntry>>> = std::sync::OnceLock::new();

fn subset_validator_registry() -> &'static RwLock<BTreeMap<ArtifactDialect, &'static SubsetValidatorEntry>> {
    SUBSET_VALIDATOR_REGISTRY.get_or_init(|| RwLock::new(BTreeMap::new()))
}

/// ⚠️ A subset-validator dialect already has a different owner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubsetValidatorRegistryConflict {
    pub dialect: ArtifactDialect,
}

/// ⚠️ Subset-validator registration cannot replace an owner or use a poisoned registry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubsetValidatorRegistryError {
    Conflict(SubsetValidatorRegistryConflict),
    Unavailable(IoRegistryUnavailable),
}

impl std::fmt::Display for SubsetValidatorRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conflict(conflict) => write!(f, "subset validator conflict for dialect {:?}", conflict.dialect),
            Self::Unavailable(unavail) => write!(f, "subset validator registry unavailable: {}", unavail.registry),
        }
    }
}
impl std::error::Error for SubsetValidatorRegistryError {}

/// 🚫️ A subset validation cannot execute because its registry is unavailable or incomplete.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubsetValidationError {
    Missing { dialect: ArtifactDialect },
    Unavailable(IoRegistryUnavailable),
}

fn same_subset_validator_entry(left: &SubsetValidatorEntry, right: &SubsetValidatorEntry) -> bool {
    left.dialect == right.dialect && std::ptr::fn_addr_eq(left.validate, right.validate)
}

/// 📌️ Registers one subset validator without replacing an established dialect owner.
pub fn register_subset_validator(entry: &'static SubsetValidatorEntry) -> Result<(), SubsetValidatorRegistryError> {
    register_subset_validators(&[entry])
}

fn validate_subset_validators(registry: &BTreeMap<ArtifactDialect, &'static SubsetValidatorEntry>, entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {
    let mut proposed: BTreeMap<ArtifactDialect, &'static SubsetValidatorEntry> = BTreeMap::new();
    for entry in entries {
        let dialect = ArtifactDialect::from(entry.dialect);
        if let Some(existing) = proposed.get(&dialect) {
            if same_subset_validator_entry(existing, entry) {
                continue;
            }
            return Err(SubsetValidatorRegistryError::Conflict(SubsetValidatorRegistryConflict { dialect }));
        }
        proposed.insert(dialect, entry);
    }
    for (dialect, entry) in &proposed {
        if let Some(existing) = registry.get(dialect) {
            if !same_subset_validator_entry(existing, entry) {
                return Err(SubsetValidatorRegistryError::Conflict(SubsetValidatorRegistryConflict { dialect: dialect.clone() }));
            }
        }
    }
    Ok(())
}

/// 🔬️ Verifies subset-validator entries without changing their established owners.
pub async fn preflight_subset_validators(entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    preflight_subset_validators_in_assembly(&assembly, entries).await
}

/// 🔬️ Verifies subset validators while one artifact assembly owns the shared publication barrier.
pub async fn preflight_subset_validators_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {
    let registry = subset_validator_registry().read().map_err(|_| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" }))?;
    validate_subset_validators(&registry, entries)
}

/// 📌️ Registers subset-validator entries only when the entire candidate set is conflict-free.
pub fn register_subset_validators(entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    register_subset_validators_in_assembly(&assembly, entries)
}

/// 📌️ Publishes preflighted subset validators while one artifact assembly owns the shared barrier.
pub fn register_subset_validators_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {
    let mut reg = subset_validator_registry().write().map_err(|_| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" }))?;
    validate_subset_validators(&reg, entries)?;
    for entry in entries {
        let dialect = ArtifactDialect::from(entry.dialect);
        reg.entry(dialect).or_insert(entry);
    }
    Ok(())
}

/// 📚️ Every dialect key currently registered in `SUBSET_VALIDATOR_REGISTRY`.
pub async fn list_registered_subset_validator_dialects() -> Result<Vec<Dialect>, SubsetValidatorRegistryError> {
    let registry = subset_validator_registry().read().map_err(|_| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" }))?;
    Ok(registry.values().map(|entry| entry.dialect).collect())
}

/// 🛡️ The generic validate-on-build hook (D5): if `dialect.subset` is anything other than
/// `SubsetId::ANY` and a validator is registered for that EXACT dialect, run it and fold its
/// `Diagnostic`s onto `diagnostics`. Advisory only -- a validator that itself returns diagnostics
/// never fails composition; diagnostics are soft signals here exactly like `Composition<T>`/
/// `Analysis<T>` already carry elsewhere in this file (a subset composer wanting a HARD gate
/// enforces that itself, inside its own `compose`, before ever returning `Ok` -- see the PDF/A
/// pilot). Called from every generic compose-dispatch path in this module (`io_dispatch`,
/// `wire_artifact_compose`) so every future subset gets this for free the moment it registers a
/// validator -- no dispatch call site needs to change again. A poisoned registry lock is surfaced
/// as a typed dispatch failure.
///
/// `ANY` always short-circuits (nothing to validate against the unconstrained base subset). A
/// real (non-`ANY`) dialect with NO registered validator is, since ticket
/// 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES, treated as a defect rather than silence
/// -- every real subset is expected to register one (`policyStandardSubsetVocabularyBreaches`
/// checks this statically) -- and emits one `io.subset.validator-missing` Warning naming the
/// coordinate. Still never hard-fails here: the receiving side of a cross-plugin wire compose may
/// legitimately not host the owning plugin's validator locally, and a missing validator is exactly
/// the kind of thing a diagnostic (not a dispatch error) exists to surface.
async fn run_subset_validation(dialect: Dialect, payload: &IoPayload, diagnostics: &mut Vec<Diagnostic>) -> Result<(), SubsetValidationError> {
    if dialect.subset == SubsetId::ANY {
        return Ok(());
    }
    let reg = subset_validator_registry().read().map_err(|_| SubsetValidationError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" }))?;
    match reg.get(&ArtifactDialect::from(dialect)) {
        Some(entry) => diagnostics.extend((entry.validate)(payload)),
        None => return Err(SubsetValidationError::Missing { dialect: ArtifactDialect::from(dialect) }),
    }
    Ok(())
}
//#endregion 🔖️SubsetValidator

//#region 🔖️IoFidelity
/// ⚖️ Declared strongest IO fidelity a subset codec achieves.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub enum IoFidelityClass {
    Exact,
    Canonical,
    Semantic,
    Lossy,
}

impl IoFidelityClass {
    pub async fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Canonical => "canonical",
            Self::Semantic => "semantic",
            Self::Lossy => "lossy",
        }
    }

    pub async fn parse(s: &str) -> Result<Self, String> {
        match s {
            "exact" => Ok(Self::Exact),
            "canonical" => Ok(Self::Canonical),
            "semantic" => Ok(Self::Semantic),
            "lossy" => Ok(Self::Lossy),
            other => Err(format!("unknown io fidelity class {other:?}")),
        }
    }

    /// Ordered strength: Exact > Canonical > Semantic > Lossy
    pub async fn rank(self) -> u8 {
        match self {
            Self::Exact => 3,
            Self::Canonical => 2,
            Self::Semantic => 1,
            Self::Lossy => 0,
        }
    }
}

/// 📜 Manifest-facing IO fidelity declaration for a subset dialect.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct IoFidelityDeclaration {
    pub class: IoFidelityClass,
    /// Field paths dropped under Lossy codecs; must be empty for stronger classes.
    pub drops: Vec<String>,
}

impl IoFidelityDeclaration {
    pub async fn validate(&self) -> Result<(), String> {
        if self.class != IoFidelityClass::Lossy && !self.drops.is_empty() {
            return Err("drops must be empty unless fidelity is lossy".into());
        }
        if self.class == IoFidelityClass::Lossy && self.drops.is_empty() {
            return Err("lossy fidelity requires a non-empty minimal drops set".into());
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️io-fidelity/🦀️.rs"]
mod io_fidelity_tests;
//#endregion 🔖️IoFidelity

//#region 🔖️Wire
/// 🎹️ Wire twin of `ErasedComposeSource` for crossing a wasm component boundary: `Dialect` is
/// `&'static str`-based and can't be safely deserialized from arbitrary runtime bytes (would need
/// to leak memory per call), so the wire form always carries the owned `ArtifactDialect` and gets
/// resolved back to the real `&'static Dialect` locally by matching coordinate strings against the
/// receiving side's own `ComposerEntry.reads` — exactly like the native W15 dispatch sites in
/// `🧰️framework/🛍️products/💻️os/🦀️.rs` already do via `io_dialects_for(...).find(...)`.
#[derive(Clone, Debug, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct WireComposeSource {
    pub dialect: ArtifactDialect,
    pub payload: IoPayload,
}

/// 🎹️ Wire twin of `ComposedArtifact`.
#[derive(Clone, Debug, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct WireComposedArtifact {
    pub dialect: ArtifactDialect,
    pub payload: IoPayload,
    pub diagnostics: Vec<Diagnostic>,
    pub confidence: Confidence,
}

impl From<ComposedArtifact> for WireComposedArtifact {
    fn from(value: ComposedArtifact) -> Self {
        Self { dialect: ArtifactDialect::from(value.dialect), payload: value.payload, diagnostics: value.diagnostics, confidence: value.confidence }
    }
}

/// 🔒️ Process-local intern table: `ArtifactDialect` → a genuine `&'static Dialect`. Cross-plugin
/// compose results can name a dialect the RECEIVING plugin never registered a `&'static Dialect`
/// constant for (it belongs to whichever plugin actually produced it) — `Dialect`'s `&'static str`
/// fields can't be manufactured from an arbitrary runtime `String` without leaking memory, so this
/// interns each DISTINCT coordinate exactly once (a bounded, one-time leak per never-before-seen
/// dialect string for the lifetime of the process, the same tradeoff any string-interning table
/// makes) and reuses it for every subsequent occurrence of that coordinate.
///
/// Deliberately validates nothing against a subset vocabulary (ticket
/// 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES considered and rejected adding a check
/// here): this runs on the RECEIVING side of cross-plugin compose, where the local process by
/// design has no way to know the producing plugin's subset vocabulary — hard-failing on an
/// "unrecognized" subset here would break legitimate cross-plugin io for any dialect the receiver
/// simply hasn't loaded a manifest for. Authority already lives at the right boundaries instead:
/// `wire_artifact_compose` rejects source dialects the local `ComposerEntry.reads` doesn't declare,
/// `run_subset_validation` reports (never fails on) a missing validator, and static "does this
/// standard's on-disk subset vocabulary match its declared manifest" enforcement is
/// `policyStandardSubsetVocabularyBreaches`'s job in script.ts, not runtime intern's.
/// 🚫️ A typed wire boundary rejection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IoWireError {
    Decode { operation: &'static str, message: String },
    Encode { operation: &'static str, message: String },
    Limit { operation: &'static str, detail: String },
    Registry(IoRegistryUnavailable),
    Resolve(String),
    Subset(SubsetValidationError),
    InternUnavailable,
}

impl std::fmt::Display for IoWireError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode { operation, message } => write!(formatter, "{operation} decode failed: {message}"),
            Self::Encode { operation, message } => write!(formatter, "{operation} encode failed: {message}"),
            Self::Limit { operation, detail } => write!(formatter, "{operation} exceeds wire limit: {detail}"),
            Self::Registry(error) => write!(formatter, "registry unavailable: {}", error.registry),
            Self::Resolve(message) => formatter.write_str(message),
            Self::Subset(error) => write!(formatter, "subset validation failed: {error:?}"),
            Self::InternUnavailable => formatter.write_str("dialect intern registry unavailable"),
        }
    }
}

impl std::error::Error for IoWireError {}

const MAX_IO_WIRE_BYTES: usize = 4 * 1024 * 1024;
const MAX_IO_WIRE_SOURCES: usize = 256;
const MAX_IO_WIRE_DIALECT_COMPONENT_BYTES: usize = 192;
const MAX_IO_WIRE_INTERNED_DIALECTS: usize = 512;

fn ensure_wire_bytes(operation: &'static str, bytes: &[u8]) -> Result<(), IoWireError> {
    if bytes.len() > MAX_IO_WIRE_BYTES {
        return Err(IoWireError::Limit { operation, detail: format!("{} bytes exceeds {MAX_IO_WIRE_BYTES}", bytes.len()) });
    }
    Ok(())
}

async fn validate_wire_dialect(operation: &'static str, dialect: &ArtifactDialect) -> Result<(), IoWireError> {
    for (name, value) in [("artifact_kind", &dialect.artifact_kind), ("standard", &dialect.standard), ("subset", &dialect.subset)] {
        if value.is_empty() || value.len() > MAX_IO_WIRE_DIALECT_COMPONENT_BYTES || value.bytes().any(|byte| byte.is_ascii_whitespace() || byte.is_ascii_control() || matches!(byte, b'@' | b'/' | b'!')) {
            return Err(IoWireError::Limit { operation, detail: format!("invalid bounded dialect {name}") });
        }
    }
    Ok(())
}

async fn validate_wire_payload(operation: &'static str, payload: &IoPayload) -> Result<(), IoWireError> {
    let bytes = match payload {
        IoPayload::Text(text) => text.len(),
        IoPayload::Binary(bytes) => bytes.len(),
    };
    if bytes > MAX_IO_WIRE_BYTES {
        return Err(IoWireError::Limit { operation, detail: format!("payload {bytes} bytes exceeds {MAX_IO_WIRE_BYTES}") });
    }
    Ok(())
}

async fn validate_wire_key(key: &IoKey) -> Result<(), IoWireError> {
    for value in [&key.artifact_kind, &key.standard, &key.subset, &key.format_kind, &key.format_standard, &key.format_subset] {
        if value.is_empty() || value.len() > MAX_IO_WIRE_DIALECT_COMPONENT_BYTES || value.bytes().any(|byte| byte.is_ascii_whitespace() || byte.is_ascii_control()) {
            return Err(IoWireError::Limit { operation: "io-key", detail: "key component is empty or exceeds the bounded wire grammar".to_string() });
        }
    }
    Ok(())
}

async fn encode_wire_json<T: ToValue>(operation: &'static str, value: &T) -> Result<Vec<u8>, IoWireError> {
    let bytes = semio_framework_pack_json::to_json_string(value).into_bytes();
    ensure_wire_bytes(operation, &bytes)?;
    Ok(bytes)
}

/// 🌉️ `encode_wire_json`'s inverse: decodes bounded wire bytes (already UTF-8 JSON text, per the
/// `T: ToValue` half above) via `pack::json`/`FromValue` instead of `serde_json`.
async fn decode_wire_json<T: FromValue>(operation: &'static str, bytes: &[u8]) -> Result<T, IoWireError> {
    let text = std::str::from_utf8(bytes).map_err(|error| IoWireError::Decode { operation, message: error.to_string() })?;
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| IoWireError::Decode { operation, message: error.to_string() })
}

async fn intern_dialect(dialect: &ArtifactDialect) -> Result<Dialect, IoWireError> {
    validate_wire_dialect("dialect", dialect).await?;
    static INTERNED: std::sync::OnceLock<RwLock<HashMap<ArtifactDialect, Dialect>>> = std::sync::OnceLock::new();
    let table = INTERNED.get_or_init(|| RwLock::new(HashMap::new()));
    if let Some(found) = table.read().map_err(|_| IoWireError::InternUnavailable)?.get(dialect) {
        return Ok(*found);
    }
    let mut write = table.write().map_err(|_| IoWireError::InternUnavailable)?;
    if let Some(found) = write.get(dialect) {
        return Ok(*found);
    }
    if write.len() >= MAX_IO_WIRE_INTERNED_DIALECTS {
        return Err(IoWireError::Limit { operation: "dialect", detail: format!("intern table reached {MAX_IO_WIRE_INTERNED_DIALECTS} entries") });
    }
    let leaked = Dialect { artifact_kind: Box::leak(dialect.artifact_kind.clone().into_boxed_str()), standard: StandardId(Box::leak(dialect.standard.clone().into_boxed_str())), subset: SubsetId(Box::leak(dialect.subset.clone().into_boxed_str())) };
    write.insert(dialect.clone(), leaked);
    Ok(leaked)
}

/// 🌉️ Decodes a wire `WireComposedArtifact` (JSON bytes) into a native `ComposedArtifact`,
/// interning its dialect via `intern_dialect`. The receiving-side half of `wire_artifact_compose`
/// — used by a guest's `io_dispatch` fallback hook once `host.io-compose` returns.
pub async fn wire_decode_composed_artifact(bytes: &[u8]) -> Result<ComposedArtifact, IoWireError> {
    ensure_wire_bytes("composed-artifact", bytes)?;
    let wire: WireComposedArtifact = decode_wire_json("composed-artifact", bytes).await?;
    validate_wire_dialect("composed-artifact", &wire.dialect).await?;
    validate_wire_payload("composed-artifact", &wire.payload).await?;
    if wire.diagnostics.len() > MAX_IO_WIRE_SOURCES {
        return Err(IoWireError::Limit { operation: "composed-artifact", detail: "too many diagnostics".to_string() });
    }
    Ok(ComposedArtifact { dialect: intern_dialect(&wire.dialect).await?, payload: wire.payload, diagnostics: wire.diagnostics, confidence: wire.confidence })
}

/// 🌉️ Encodes this process's own composer roster (`list_composer_entries`) as JSON bytes — the
/// body of the WIT `list-artifact-dialects` guest export (see D3, ticket 26/08/10/
/// ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION). JSON (not `pack_rt::encode_wire_value`)
/// is a deliberate simplification for this first cut: the WIT signature is an opaque `list<u8>`
/// either way, so swapping the wire encoding later needs no ABI change, and this module has no
/// existing dependency on `store`/`dsl`'s pack machinery worth introducing just for this.
pub async fn wire_list_composer_entries() -> Result<Vec<u8>, IoWireError> {
    let entries = list_composer_entries().await.map_err(IoWireError::Registry)?;
    encode_wire_json("composer-entry-list", &entries).await
}

/// 🌉️ Decodes a wire `(IoKey, Vec<WireComposeSource>)` request and composes it against THIS
/// process's own local registry only — never the fallback hook. A guest receiving an incoming
/// `artifact-compose` call is, by construction, the plugin the host router already decided owns
/// the key; falling through again here would be a pointless extra hop at best and a reentrancy
/// risk at worst (see the host router's own one-hop guard). The body of the WIT
/// `artifact-compose` guest export. Errors are flattened to a message string, matching how every
/// other fallible call on this ABI surfaces errors (a `Fault`, not structured data) — see
/// `migrate-artifact`'s `plugin-error` for the existing precedent.
pub async fn wire_artifact_compose(key_bytes: &[u8], sources_bytes: &[u8]) -> Result<Vec<u8>, IoWireError> {
    ensure_wire_bytes("io-key", key_bytes)?;
    ensure_wire_bytes("compose-source", sources_bytes)?;
    let key: IoKey = decode_wire_json("io-key", key_bytes).await?;
    validate_wire_key(&key).await?;
    let wire_sources: Vec<WireComposeSource> = decode_wire_json("compose-source", sources_bytes).await?;
    if wire_sources.len() > MAX_IO_WIRE_SOURCES {
        return Err(IoWireError::Limit { operation: "compose-source", detail: format!("{} sources exceeds {MAX_IO_WIRE_SOURCES}", wire_sources.len()) });
    }
    let entry = resolve(&key).await.map_err(|error| error.unavailable.map_or_else(|| IoWireError::Resolve(error.message), IoWireError::Registry))?;
    let mut sources = Vec::with_capacity(wire_sources.len());
    for wire in wire_sources {
        validate_wire_dialect("compose-source", &wire.dialect).await?;
        validate_wire_payload("compose-source", &wire.payload).await?;
        let dialect = entry.reads.iter().copied().find(|&d| ArtifactDialect::from(d) == wire.dialect).ok_or_else(|| IoWireError::Resolve(format!("composer for {} does not read dialect {}", key.artifact_kind, wire.dialect.to_coordinate())))?;
        sources.push(ErasedComposeSource { dialect, payload: wire.payload });
    }
    match ::semio_framework_async::poll::resolve_ready((entry.compose)(&sources)) {
        Ok(mut composed) => {
            run_subset_validation(composed.dialect, &composed.payload, &mut composed.diagnostics).await.map_err(IoWireError::Subset)?;
            encode_wire_json("composed-artifact", &WireComposedArtifact::from(composed)).await
        }
        Err(error) => Err(IoWireError::Resolve(error.message)),
    }
}
//#endregion 🔖️Wire

//#region 🔖️FormatCatalog
/// 🗄️ One representation's plural MIME and extension claims with canonical identity metadata. Generic
/// successor to the closed, `🔺️mesh`-local `StdioFormatEntry`/`STDIO_FORMAT_CATALOG` (ticket
/// 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT wave 2): where `mesh`'s catalog is a single
/// hardcoded `const` slice only `stdio` can ever contribute to, this registry is additive and
/// string-keyed like `IO_REGISTRY` above it, so ANY plugin that owns formats (not just `stdio`)
/// can call `register_format_descriptors` from its own init. `mesh`'s catalog itself is untouched
/// here -- evicting it onto this registry is a LATER wave's job, once every producer/consumer of
/// `StdioFormatEntry` has migrated to `FormatDescriptor`.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct FormatDescriptor {
    pub kind_id: String,
    pub short_id: String,
    pub aliases: Vec<String>,
    pub mimes: Vec<String>,
    pub extensions: Vec<String>,
    pub name: String,
    pub full_name: String,
    pub neutral: bool,
    pub dir_name: String,
    pub is_binary: bool,
}

/// ⚠️ A format registry identity, extension, or MIME ownership collision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormatRegistryConflict {
    Identity { key: String, established_kind_id: String, conflicting_kind_id: String },
    Extension { extension: String, established_kind_id: String, conflicting_kind_id: String },
    Mime { mime: String, established_kind_id: String, conflicting_kind_id: String },
    Invalid { kind_id: String, detail: String },
}

impl std::fmt::Display for FormatRegistryConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identity { key, established_kind_id, conflicting_kind_id } => write!(f, "format conflict on key '{key}': established {established_kind_id}, conflicting {conflicting_kind_id}"),
            Self::Extension { extension, established_kind_id, conflicting_kind_id } => write!(f, "format conflict on extension '{extension}': established {established_kind_id}, conflicting {conflicting_kind_id}"),
            Self::Mime { mime, established_kind_id, conflicting_kind_id } => write!(f, "format conflict on mime '{mime}': established {established_kind_id}, conflicting {conflicting_kind_id}"),
            Self::Invalid { kind_id, detail } => write!(f, "invalid format descriptor {kind_id}: {detail}"),
        }
    }
}
impl std::error::Error for FormatRegistryConflict {}

/// 🚫️ A format registration or inspection could not acquire its authoritative catalog.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormatRegistryError {
    Conflict(FormatRegistryConflict),
    Unknown { input: String },
    Unavailable(IoRegistryUnavailable),
}

impl std::fmt::Display for FormatRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conflict(error) => error.fmt(formatter),
            Self::Unknown { input } => write!(formatter, "unknown format {input}"),
            Self::Unavailable(error) => write!(formatter, "format registry unavailable: {}", error.registry),
        }
    }
}

impl std::error::Error for FormatRegistryError {}

static FORMAT_CATALOG: std::sync::OnceLock<RwLock<BTreeMap<String, FormatDescriptor>>> = std::sync::OnceLock::new();

/// 🪶️ Framework-owned semantic SQLite file endpoint for every declared artifact snapshot.
pub fn sqlite_snapshot_format_descriptor() -> FormatDescriptor {
    FormatDescriptor { kind_id: crate::io_schema::SQLITE_SNAPSHOT.artifact_kind.to_string(), short_id: "sqlite".to_string(), aliases: vec![], mimes: vec!["application/vnd.sqlite3".to_string()], extensions: vec![".sqlite".to_string()], name: "SQLite".to_string(), full_name: "Semantic Artifact Snapshot SQLite".to_string(), neutral: true, dir_name: "sqlite".to_string(), is_binary: true }
}

fn format_catalog() -> &'static RwLock<BTreeMap<String, FormatDescriptor>> {
    FORMAT_CATALOG.get_or_init(|| {
        let descriptor = sqlite_snapshot_format_descriptor();
        let rows = format_descriptor_keys(&descriptor).map(|key| (key, descriptor.clone())).collect();
        RwLock::new(rows)
    })
}

/// 📌️ Registers format rows atomically. Identity, extension, and non-empty MIME claims are each
/// globally singular; equal duplicate rows are idempotent and never replace an established owner.
pub async fn register_format_descriptors(descriptors: impl IntoIterator<Item = FormatDescriptor>) -> Result<(), FormatRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    register_format_descriptors_in_assembly(&assembly, descriptors)
}

/// 📌️ Publishes preflighted format rows while one artifact assembly owns the shared barrier.
pub fn register_format_descriptors_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, descriptors: impl IntoIterator<Item = FormatDescriptor>) -> Result<(), FormatRegistryError> {
    let (proposed, proposed_by_kind) = index_format_descriptors(descriptors).map_err(FormatRegistryError::Conflict)?;
    let mut registry = format_catalog().write().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;
    validate_format_descriptors(&registry, &proposed, &proposed_by_kind).map_err(FormatRegistryError::Conflict)?;
    for (key, descriptor) in proposed {
        registry.entry(key).or_insert(descriptor);
    }
    Ok(())
}

/// 🔬️ Verifies format rows against the catalog without mutating their global ownership.
pub async fn preflight_format_descriptors(rows: &[FormatDescriptor]) -> Result<(), FormatRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    preflight_format_descriptors_in_assembly(&assembly, rows)
}

/// 🔬️ Verifies format rows while one artifact assembly owns the shared publication barrier.
pub fn preflight_format_descriptors_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, rows: &[FormatDescriptor]) -> Result<(), FormatRegistryError> {
    let (proposed, proposed_by_kind) = index_format_descriptors(rows.iter().cloned()).map_err(FormatRegistryError::Conflict)?;
    let registry = format_catalog().read().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;
    validate_format_descriptors(&registry, &proposed, &proposed_by_kind).map_err(FormatRegistryError::Conflict)
}

type FormatDescriptorIndexes = (BTreeMap<String, FormatDescriptor>, BTreeMap<String, FormatDescriptor>);

fn index_format_descriptors(descriptors: impl IntoIterator<Item = FormatDescriptor>) -> Result<FormatDescriptorIndexes, FormatRegistryConflict> {
    let mut proposed: BTreeMap<String, FormatDescriptor> = BTreeMap::new();
    let mut proposed_by_kind: BTreeMap<String, FormatDescriptor> = BTreeMap::new();
    for row in descriptors {
        let row = canonicalize_format_descriptor(row)?;
        for key in format_descriptor_keys(&row) {
            if let Some(existing) = proposed.get(&key) {
                if existing != &row {
                    return Err(FormatRegistryConflict::Identity { key, established_kind_id: existing.kind_id.clone(), conflicting_kind_id: row.kind_id.clone() });
                }
            } else {
                proposed.insert(key, row.clone());
            }
        }
        if let Some(existing) = proposed_by_kind.get(&row.kind_id) {
            if existing != &row {
                return Err(FormatRegistryConflict::Identity { key: row.kind_id.clone(), established_kind_id: existing.kind_id.clone(), conflicting_kind_id: row.kind_id.clone() });
            }
        } else {
            proposed_by_kind.insert(row.kind_id.clone(), row);
        }
    }
    Ok((proposed, proposed_by_kind))
}

fn canonicalize_format_descriptor(mut row: FormatDescriptor) -> Result<FormatDescriptor, FormatRegistryConflict> {
    row.kind_id = row.kind_id.trim().to_string();
    row.short_id = row.short_id.trim().to_string();
    if row.kind_id.is_empty() || row.short_id.is_empty() {
        return Err(FormatRegistryConflict::Invalid { kind_id: row.kind_id.clone(), detail: "kind_id and short_id must both be non-empty".to_string() });
    }
    for extension in &mut row.extensions {
        *extension = extension.trim().to_ascii_lowercase();
    }
    if row.extensions.iter().any(String::is_empty) || row.extensions.is_empty() {
        return Err(FormatRegistryConflict::Invalid { kind_id: row.kind_id.clone(), detail: "at least one non-empty extension claim is required".to_string() });
    }
    row.extensions.sort();
    if row.extensions.windows(2).any(|claims| claims[0] == claims[1]) {
        return Err(FormatRegistryConflict::Invalid { kind_id: row.kind_id.clone(), detail: "extension claims must be distinct".to_string() });
    }
    for mime in &mut row.mimes {
        *mime = mime.trim().to_ascii_lowercase();
    }
    if row.mimes.iter().any(String::is_empty) {
        return Err(FormatRegistryConflict::Invalid { kind_id: row.kind_id.clone(), detail: "MIME claims must be non-empty; omit unclaimed MIME values".to_string() });
    }
    row.mimes.sort();
    if row.mimes.windows(2).any(|claims| claims[0] == claims[1]) {
        return Err(FormatRegistryConflict::Invalid { kind_id: row.kind_id.clone(), detail: "MIME claims must be distinct".to_string() });
    }
    for alias in &mut row.aliases {
        *alias = alias.trim().to_string();
    }
    if row.aliases.iter().any(String::is_empty) || row.aliases.iter().any(|alias| alias == &row.kind_id || alias == &row.short_id) {
        return Err(FormatRegistryConflict::Invalid { kind_id: row.kind_id.clone(), detail: "aliases must be non-empty and distinct from kind_id and short_id".to_string() });
    }
    row.aliases.sort();
    if row.aliases.windows(2).any(|aliases| aliases[0] == aliases[1]) {
        return Err(FormatRegistryConflict::Invalid { kind_id: row.kind_id.clone(), detail: "aliases must be distinct".to_string() });
    }
    Ok(row)
}

fn format_mimes(row: &FormatDescriptor) -> impl Iterator<Item = &str> {
    row.mimes.iter().map(String::as_str)
}

fn validate_format_descriptors(registry: &BTreeMap<String, FormatDescriptor>, proposed: &BTreeMap<String, FormatDescriptor>, proposed_by_kind: &BTreeMap<String, FormatDescriptor>) -> Result<(), FormatRegistryConflict> {
    let mut established_by_kind: BTreeMap<String, &FormatDescriptor> = BTreeMap::new();
    for descriptor in registry.values() {
        established_by_kind.entry(descriptor.kind_id.clone()).or_insert(descriptor);
    }
    for row in proposed_by_kind.values() {
        for existing in established_by_kind.values().copied().chain(proposed_by_kind.values()) {
            if existing.kind_id == row.kind_id {
                continue;
            }
            for existing_ext in &existing.extensions {
                for row_ext in &row.extensions {
                    if existing_ext == row_ext {
                        return Err(FormatRegistryConflict::Extension { extension: row_ext.clone(), established_kind_id: existing.kind_id.clone(), conflicting_kind_id: row.kind_id.clone() });
                    }
                }
            }
            for existing_mime in format_mimes(existing) {
                for row_mime in format_mimes(row) {
                    if existing_mime == row_mime {
                        return Err(FormatRegistryConflict::Mime { mime: row_mime.to_string(), established_kind_id: existing.kind_id.clone(), conflicting_kind_id: row.kind_id.clone() });
                    }
                }
            }
        }
    }
    for (key, row) in proposed {
        if let Some(existing) = registry.get(key) {
            if existing != row {
                return Err(FormatRegistryConflict::Identity { key: key.clone(), established_kind_id: existing.kind_id.clone(), conflicting_kind_id: row.kind_id.clone() });
            }
        }
    }
    Ok(())
}

fn format_descriptor_keys(row: &FormatDescriptor) -> impl Iterator<Item = String> + '_ {
    std::iter::once(row.kind_id.clone()).chain(std::iter::once(row.short_id.clone())).chain(row.aliases.iter().cloned())
}

/// 🔎️ Resolves a format by its `kind_id`, `short_id`, or registered alias.
pub fn format_descriptor(kind_or_short_or_alias: &str) -> Result<Option<FormatDescriptor>, FormatRegistryError> {
    let registry = format_catalog().read().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;
    Ok(registry.get(kind_or_short_or_alias).cloned())
}

/// 🏷️ Normalize any recognized form (kind id, short id, alias) to the canonical `kind_id`.
pub fn normalize_format_kind(input: &str) -> Result<Option<String>, FormatRegistryError> {
    Ok(format_descriptor(input)?.map(|descriptor| descriptor.kind_id))
}

/// 🗂️ File-picker `accept` filter (comma-joined extensions) for a list of kind/short/alias
/// strings -- the generic successor to `mesh::stdio_accept_filter`.
pub fn format_accept_filter(kind_ids: &[&str]) -> Result<String, FormatRegistryError> {
    let registry = format_catalog().read().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;
    let mut extensions = Vec::new();
    for kind_id in kind_ids {
        let descriptor = registry.get(*kind_id).ok_or_else(|| FormatRegistryError::Unknown { input: (*kind_id).to_string() })?;
        extensions.extend(descriptor.extensions.iter().cloned());
    }
    Ok(extensions.join(","))
}

/// 🧷️ All IO and store rows a plugin must publish as one irreducible assembly unit.
#[derive(Default)]
pub struct ArtifactAssemblyRegistryPlan {
    pub composer_entries: Vec<&'static ComposerEntry>,
    pub subset_validators: Vec<&'static SubsetValidatorEntry>,
    pub format_descriptors: Vec<FormatDescriptor>,
    pub document_codecs: Vec<store::ArtifactCodec>,
    pub dialect_migrations: Vec<store::DialectMigration>,
    pub native_snapshots: Vec<io_mechanism::NativeSnapshotRegistration>,
}

impl ArtifactAssemblyRegistryPlan {
    /// 🌱️ Starts an empty plan; callers append every owned registry row before committing once.
    pub async fn new() -> Self {
        Self { composer_entries: Vec::new(), subset_validators: Vec::new(), format_descriptors: Vec::new(), document_codecs: Vec::new(), dialect_migrations: Vec::new(), native_snapshots: Vec::new() }
    }
}

/// 🚫️ An all-registry assembly cannot acquire its locks or pass preflight.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactAssemblyRegistryError {
    Transaction(semio_framework_schema_registry::assembly::Error),
    Composer(Box<IoRegistryRegistrationError>),
    SubsetValidator(SubsetValidatorRegistryError),
    Format(FormatRegistryError),
    Store(Box<store::ArtifactAssemblyStoreRegistryError>),
    NativeSnapshot(io_mechanism::IoRegistryError),
}

impl std::fmt::Display for ArtifactAssemblyRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transaction(error) => error.fmt(formatter),
            Self::Composer(error) => error.fmt(formatter),
            Self::SubsetValidator(error) => error.fmt(formatter),
            Self::Format(error) => error.fmt(formatter),
            Self::Store(error) => error.fmt(formatter),
            Self::NativeSnapshot(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ArtifactAssemblyRegistryError {}

/// 🧰️ Publishes an imperative native codec and its exact snapshot dialect atomically.
pub fn register_native_snapshot_codec(dialect: Dialect, codec: store::ArtifactCodec) -> Result<(), ArtifactAssemblyRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(ArtifactAssemblyRegistryError::Transaction)?;
    let plan = ArtifactAssemblyRegistryPlan { document_codecs: vec![codec.clone()], native_snapshots: vec![io_mechanism::NativeSnapshotRegistration { dialect: dialect.into(), codec }], ..Default::default() };
    commit_artifact_assembly_registry_plan(&assembly, plan)
}

/// 📦️ Publishes a native document codec and any actual owner-declared relational capability.
pub fn register_native_document_codec(dialect: Dialect, codec: store::ArtifactCodec) -> Result<(), ArtifactAssemblyRegistryError> {
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(ArtifactAssemblyRegistryError::Transaction)?;
    let plan = ArtifactAssemblyRegistryPlan { document_codecs: vec![codec.clone()], native_snapshots: io_mechanism::NativeSnapshotRegistration::from_capability(dialect.into(), codec).into_iter().collect(), ..Default::default() };
    commit_artifact_assembly_registry_plan(&assembly, plan)
}

/// 📌️ Acquires every affected write lock, preflights every candidate, then commits without any
/// fallible operation after the first registry mutation.
pub fn commit_artifact_assembly_registry_plan(assembly: &semio_framework_schema_registry::assembly::Transaction, plan: ArtifactAssemblyRegistryPlan) -> Result<(), ArtifactAssemblyRegistryError> {
    let mut store_guards = store::acquire_artifact_assembly_store_registry_guards(assembly).map_err(|error| ArtifactAssemblyRegistryError::Store(Box::new(error)))?;
    let mut composers = io_registry().write().map_err(|_| ArtifactAssemblyRegistryError::Composer(Box::new(IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "io-composer" }))))?;
    let mut subset_validators = subset_validator_registry().write().map_err(|_| ArtifactAssemblyRegistryError::SubsetValidator(SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" })))?;
    let mut formats = format_catalog().write().map_err(|_| ArtifactAssemblyRegistryError::Format(FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" })))?;
    let mut native_snapshots = io_mechanism::native_snapshot_registry().write().map_err(|_| ArtifactAssemblyRegistryError::NativeSnapshot(io_mechanism::IoRegistryError::Unavailable))?;
    let proposed_snapshots = io_mechanism::propose_native_snapshots(&native_snapshots, &plan.native_snapshots).map_err(ArtifactAssemblyRegistryError::NativeSnapshot)?;
    let proposed_composers = composer_entries_by_key(plan.composer_entries.iter().copied()).map_err(|error| ArtifactAssemblyRegistryError::Composer(Box::new(error)))?;
    validate_composer_entries(&composers, &proposed_composers).map_err(|error| ArtifactAssemblyRegistryError::Composer(Box::new(error)))?;
    validate_subset_validators(&subset_validators, &plan.subset_validators).map_err(ArtifactAssemblyRegistryError::SubsetValidator)?;
    let (proposed_formats, proposed_formats_by_kind) = index_format_descriptors(plan.format_descriptors.iter().cloned()).map_err(|error| ArtifactAssemblyRegistryError::Format(FormatRegistryError::Conflict(error)))?;
    validate_format_descriptors(&formats, &proposed_formats, &proposed_formats_by_kind).map_err(|error| ArtifactAssemblyRegistryError::Format(FormatRegistryError::Conflict(error)))?;
    store::preflight_artifact_assembly_store_registry_guards(&store_guards, &plan.document_codecs, &plan.dialect_migrations).map_err(|error| ArtifactAssemblyRegistryError::Store(Box::new(error)))?;
    for (key, entry) in proposed_composers {
        composers.entry(key).or_insert(entry);
    }
    for entry in plan.subset_validators {
        subset_validators.entry(ArtifactDialect::from(entry.dialect)).or_insert(entry);
    }
    for (key, descriptor) in proposed_formats {
        formats.entry(key).or_insert(descriptor);
    }
    store::commit_artifact_assembly_store_registry_guards(&mut store_guards, plan.document_codecs, plan.dialect_migrations);
    native_snapshots.extend(proposed_snapshots);
    Ok(())
}

/// 📋️ Serialize every distinct registered format as a `mimes.csv`-shaped body (header + one row
/// per distinct `kind_id`, sorted for determinism) -- the generic successor to
/// `mesh::stdio_mimes_csv`.
pub async fn formats_csv() -> Result<String, FormatRegistryError> {
    let reg = format_catalog().read().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;
    let mut seen: BTreeMap<&str, &FormatDescriptor> = BTreeMap::new();
    for row in reg.values() {
        seen.entry(row.kind_id.as_str()).or_insert(row);
    }
    let mut out = String::from("MIME,Extension,Name,FullName,Neutral,Dir,Kind\n");
    for row in seen.into_values() {
        let mimes = format_mimes(row).collect::<Vec<_>>();
        for extension in &row.extensions {
            for mime in mimes.iter().copied().chain(std::iter::once("").take(usize::from(mimes.is_empty()))) {
                out.push_str(mime);
                out.push(',');
                out.push_str(extension);
                out.push(',');
                out.push_str(&row.name);
                out.push(',');
                out.push_str(&row.full_name);
                out.push(',');
                out.push_str(if row.neutral { "true" } else { "false" });
                out.push(',');
                out.push_str(&row.dir_name);
                out.push(',');
                out.push_str(&row.kind_id);
                out.push('\n');
            }
        }
    }
    Ok(out)
}
//#endregion 🔖️FormatCatalog
//#endregion 🔖️ErasedRegistry

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

//#region 🔖️IoMechanism
/// 🚪️🆕️ The new io mechanism (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-A task
/// 2-4, design.md §3): `Serializer`/`Deserializer` typed traits, a type-erased `IoEntry` vtable
/// row, a `(from, into)`-keyed registry, breadth-bounded deterministic routing (`io_route`),
/// route execution (`io_run`), carrier-dialect identification (`io_identify`), and the SDK-facing
/// constructors plugins build entries from. Nested in its OWN module (not flattened into `io`'s
/// top level) so its `IoPayload`/`Confidence` — deliberately DIFFERENT, wider types than the OLD
/// file's `IoPayload`/`Confidence` a few regions up (this one adds `Confidence::None` and is not
/// used by anything above this region) — never collide names with them in the same scope. D2: the
/// OLD registry above (`ComposerEntry`/`IoKey`/`io_dispatch`/`SubsetValidator`/`FormatCatalog`)
/// is untouched and keeps working; this region is purely additive until W6 deletes the old one.
pub mod io_mechanism {
    use crate::io_schema::{ArtifactDialect, Confidence, Dialect, IoEntryDescriptor, IoError, IoFidelity, IoOutcome, IoPayload, IoResult, IoRoute, CARRIER_BINARY, CARRIER_TEXT, SQLITE_SNAPSHOT};
    use super::sqlite_snapshot::{SnapshotEncoding, SqliteDatabase, SqliteDatabaseLimits, SqliteRow, SqliteTable, SqliteValue, SqliteSnapshotProgress, SqliteSnapshotControl, SqliteSnapshotPhase, export_sqlite_database, import_sqlite_database, export_sqlite_database_controlled, import_sqlite_database_controlled};
    use semio_framework_diagnostic::Diagnostic;
    use semio_framework_value::{ValueError, ValueRefusalKind};
    use std::collections::{BTreeMap, BTreeSet};
    use std::sync::RwLock;

    //#region 🔖️Traits
    /// 🎹️ A typed native-value → foreign-payload encoder. `INTO`/`FIDELITY` are the foreign
    /// dialect and the strongest fidelity this serializer achieves. `children` are the owned members of a composed native (design
    /// §20.15: composed content is read on read, never through the parent's local owner) — the empty view for any other native.
    pub trait Serializer<S> {
        const INTO: Dialect;
        const FIDELITY: IoFidelity;
        fn serialize(from: &S, children: &ArchiveChildren) -> impl std::future::Future<Output = IoResult<IoPayload>> + Send;
    }

    /// 🎹️ A typed foreign-payload → native-value decoder. `FROM`/`FIDELITY` are the foreign
    /// dialect and the strongest fidelity this deserializer achieves. `CONFORMANCE` is D5's
    /// `SubsetValidator` replacement, run by `deserializer_entry`/`deserializer_entry_text` right
    /// after a successful `deserialize` — an addition beyond the ticket's literal trait sketch: an
    /// `IoEntry.run`'s bare-`fn`-pointer field cannot capture a runtime `conformance` CONSTRUCTOR
    /// PARAMETER (a closure that closes over any value, even a `Copy` fn pointer, cannot coerce to
    /// a bare `fn` pointer), so a compile-time-resolvable associated const is the only channel that
    /// keeps `IoEntry.run` a true, non-capturing function pointer.
    pub trait Deserializer<S> {
        const FROM: Dialect;
        const FIDELITY: IoFidelity;
        const CONFORMANCE: Option<fn(&S) -> Vec<Diagnostic>> = None;
        fn sniff(_payload: &IoPayload) -> impl std::future::Future<Output = Confidence> + Send {
            std::future::ready(Confidence::None)
        }
        fn deserialize(payload: &IoPayload) -> impl std::future::Future<Output = IoResult<S>> + Send;
    }
    //#endregion 🔖️Traits

    //#region 🔖️ArchiveChildren
    /// 🪆️ The read-only typed view of a composed artifact's owned children that a reader takes beside its parent (design §20.15:
    /// composed content is read on read, never through the parent's local owner): the members of the document's recursive archive
    /// (`store::channel::DocumentArchivePack`), addressed by slot and child id. A native without owned children gets the empty view.
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct ArchiveChildren {
        members: Vec<store::channel::OwnedDocumentMemberPackEntry>,
    }

    impl ArchiveChildren {
        /// 🈳️ The view of a document without owned children.
        pub const fn empty() -> Self {
            Self { members: Vec::new() }
        }

        /// 🗃️ The owned members of `archive`, nested ones included.
        pub fn from_archive(archive: &store::channel::DocumentArchivePack) -> Self {
            Self { members: archive.members.clone() }
        }

        /// 📋️ Every `(slot, child_id)` the view holds, in archive order.
        pub fn slots(&self) -> Vec<(String, String)> {
            self.members.iter().map(|member| (member.owner.slot.clone(), member.owner.child_id.clone())).collect()
        }

        /// 🎯️ The dialect the child at `slot`/`child_id` materializes as.
        pub fn dialect(&self, slot: &str, child_id: &str) -> Option<ArtifactDialect> {
            self.find(slot, child_id).map(|member| ArtifactDialect { artifact_kind: member.reference.artifact_kind.clone(), standard: member.reference.standard.clone(), subset: member.reference.subset.clone() })
        }

        /// 🧵️ The head snapshot of the child at `slot`/`child_id`: its archived envelope decoded and its history folded.
        pub async fn typed<S, M>(&self, slot: &str, child_id: &str) -> Result<S, IoError>
        where
            S: Clone + store::ArtifactPack,
            M: store::OpText + store::OpBinary + store::Mutation<S>,
        {
            let member = self.find(slot, child_id).ok_or_else(|| refusal(ValueRefusalKind::InvalidValue, format!("no owned child {slot}/{child_id} in the archive")))?;
            let (pack, spr) = store::decode_document_pack_bytes(&member.envelope_pack).await.map_err(|error| refusal(ValueRefusalKind::InvalidValue, format!("owned child {slot}/{child_id}: {error}")))?;
            store::parse_document_pack::<S, M>(&pack, &spr).await.map(store::ParsedDocumentText::into_snapshot).map_err(text_refusal)
        }

        /// 🔎️ The archived member at `slot`/`child_id`.
        fn find(&self, slot: &str, child_id: &str) -> Option<&store::channel::OwnedDocumentMemberPackEntry> {
            self.members.iter().find(|member| member.owner.slot == slot && member.owner.child_id == child_id)
        }
    }

    /// 📨️ A binary native payload split into the parent's native bytes and its owned children: a composed artifact's head carrier
    /// (`store::channel::encode_document_archive_bytes` of `{parent HEAD native, empty parent history, members}`, first byte
    /// [`store::channel::DOCUMENT_ARCHIVE_VERSION`]) yields its parent bytes and members; any other payload is the parent alone.
    fn native_carrier(bytes: &[u8]) -> Result<(std::borrow::Cow<'_, [u8]>, ArchiveChildren), IoError> {
        if bytes.first() != Some(&store::channel::DOCUMENT_ARCHIVE_VERSION) {
            return Ok((std::borrow::Cow::Borrowed(bytes), ArchiveChildren::empty()));
        }
        let archive = ::semio_framework_async::poll::resolve_ready(store::channel::decode_document_archive_bytes(bytes)).map_err(|error| refusal(ValueRefusalKind::InvalidValue, format!("composed carrier: {error}")))?;
        if !archive.parent_spr.is_empty() {
            return Err(refusal(ValueRefusalKind::InvalidValue, "composed carrier: a head carrier holds no parent history"));
        }
        Ok((std::borrow::Cow::Owned(archive.parent_pack), ArchiveChildren { members: archive.members }))
    }
    //#endregion 🔖️ArchiveChildren

    //#region 🔖️DslTxtCarrier
    /// 🗒️ Serialize any `ArtifactDsl` document as UTF-8 carrier text (stdio.txt body law).
    pub fn serialize_dsl_txt<S: store::ArtifactDsl>(from: &S) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(store::ArtifactDsl::print_dsl(from))))
    }

    /// 📃️ Parse carrier text (or UTF-8 binary) back into an `ArtifactDsl` document.
    pub fn deserialize_dsl_txt<S: store::ArtifactDsl>(payload: &IoPayload) -> IoResult<S> {
        let text = match payload {
            IoPayload::Text(text) => text.as_str(),
            IoPayload::Binary(bytes) => std::str::from_utf8(bytes).map_err(|error| IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, error.to_string())))?,
        };
        S::parse_dsl(text).map(IoOutcome::clean).map_err(text_refusal)
    }

    fn refusal(kind: ValueRefusalKind, message: impl Into<String>) -> IoError { IoError::from_value_error(ValueError::new(kind, message)) }

    fn text_refusal(error: semio_framework_diagnostic::TextError) -> IoError {
        let mut callback = |_| true;
        let mut control = semio_framework_value::NativeEncodeControl::new(isize::MAX as usize, &mut callback);
        IoError::from_text_error_controlled(error, &mut control).unwrap_or_else(IoError::from_value_error)
    }
    //#endregion 🔖️DslTxtCarrier

    //#region 🔖️Entry
    /// 🧲️ The side of an entry that is the registering artifact's native dialect, declared by the constructor and never inferred: an
    /// `Export` entry serializes out of it (`from` is native), an `Import` entry deserializes into it (`into` is native).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum IoEntryDirection {
        Export,
        Import,
    }

    /// 🛤️ One registered entry seen from its native side — the shape a package descriptor lists as an import/export row.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct IoNativeRoute {
        pub native: ArtifactDialect,
        pub foreign: ArtifactDialect,
        pub direction: IoEntryDirection,
    }

    /// 🧾️ Type-erased io vtable row: one directed hop `from -> into` at a declared `fidelity`, its native side `direction`, an
    /// optional `sniff` (carrier-dialect identification), and the erased `run` this hop executes.
    pub struct IoEntry {
        pub from: Dialect,
        pub into: Dialect,
        pub fidelity: IoFidelity,
        pub direction: IoEntryDirection,
        pub sniff: Option<fn(&IoPayload) -> Confidence>,
        pub run: fn(&IoPayload) -> IoResult<IoPayload>,
    }

    fn same_io_entry(left: &IoEntry, right: &IoEntry) -> bool {
        let same_sniff = match (left.sniff, right.sniff) {
            (Some(a), Some(b)) => std::ptr::fn_addr_eq(a, b),
            (None, None) => true,
            _ => false,
        };
        ArtifactDialect::from(left.from) == ArtifactDialect::from(right.from) && ArtifactDialect::from(left.into) == ArtifactDialect::from(right.into) && left.fidelity == right.fidelity && left.direction == right.direction && same_sniff && std::ptr::fn_addr_eq(left.run, right.run)
    }

    fn descriptor_of(entry: &IoEntry) -> IoEntryDescriptor {
        IoEntryDescriptor { from: ArtifactDialect::from(entry.from), into: ArtifactDialect::from(entry.into), fidelity: entry.fidelity, sniffs: entry.sniff.is_some() }
    }
    //#endregion 🔖️Entry

    //#region 🔖️Registry
    type EntryKey = (ArtifactDialect, ArtifactDialect);
    type EntryMap = BTreeMap<EntryKey, &'static IoEntry>;

    static IO_MECHANISM_REGISTRY: std::sync::OnceLock<RwLock<EntryMap>> = std::sync::OnceLock::new();

    fn io_mechanism_registry() -> &'static RwLock<EntryMap> {
        IO_MECHANISM_REGISTRY.get_or_init(|| RwLock::new(BTreeMap::new()))
    }

    /// 🧷️ Exact domain identity and semantic snapshot provider published by artifact assembly.
    #[derive(Clone)]
    pub struct NativeSnapshotRegistration {
        pub dialect: ArtifactDialect,
        pub codec: store::ArtifactCodec,
    }

    impl NativeSnapshotRegistration {
        /// 🪶️ Publishes a snapshot registration only for an owner-declared relational capability.
        pub fn from_capability(dialect: ArtifactDialect, codec: store::ArtifactCodec) -> Option<Self> {
            codec.snapshot_sqlite.as_ref()?;
            Some(Self { dialect, codec })
        }
    }

    pub(super) type NativeSnapshotMap = BTreeMap<ArtifactDialect, store::ArtifactCodec>;
    static NATIVE_SNAPSHOT_REGISTRY: std::sync::OnceLock<RwLock<NativeSnapshotMap>> = std::sync::OnceLock::new();

    pub(super) fn native_snapshot_registry() -> &'static RwLock<NativeSnapshotMap> {
        NATIVE_SNAPSHOT_REGISTRY.get_or_init(|| RwLock::new(BTreeMap::new()))
    }

    pub(super) fn propose_native_snapshots(existing: &NativeSnapshotMap, registrations: &[NativeSnapshotRegistration]) -> Result<NativeSnapshotMap, IoRegistryError> {
        let mut proposed = NativeSnapshotMap::new();
        for registration in registrations {
            let dialect = &registration.dialect;
            let coordinate = dialect.to_coordinate();
            if dialect == &ArtifactDialect::from(SQLITE_SNAPSHOT) || ArtifactDialect::parse_coordinate(&coordinate).as_ref() != Ok(dialect) || coordinate.chars().any(char::is_control) || registration.codec.snapshot_sqlite.is_none() {
                return Err(IoRegistryError::InvalidSnapshotDialect(dialect.clone()));
            }
            let provider = registration.codec.snapshot_sqlite.as_ref().expect("checked explicit SQLite provider");
            let schema = SqliteDatabase::from_schema(&provider.schema).map_err(|error| IoRegistryError::InvalidSnapshotSchema { dialect: dialect.clone(), message: error.to_string() })?;
            if schema.tables.is_empty() || schema.tables.iter().any(|table| table.name.eq_ignore_ascii_case("semio_snapshot")) {
                return Err(IoRegistryError::InvalidSnapshotSchema { dialect: dialect.clone(), message: "artifact SQLite schema must contain domain tables and cannot claim reserved snapshot metadata".to_string() });
            }
            for current in [existing.get(dialect), proposed.get(dialect)].into_iter().flatten() {
                if current.extension != registration.codec.extension || current.pack_schema_hash != registration.codec.pack_schema_hash || !current.snapshot_sqlite.as_ref().expect("registered relational provider").identical_to(&registration.codec.snapshot_sqlite.as_ref().expect("validated relational provider")) {
                    return Err(IoRegistryError::Duplicate { from: Box::new(dialect.clone()), into: Box::new(ArtifactDialect::from(SQLITE_SNAPSHOT)) });
                }
            }
            proposed.insert(dialect.clone(), registration.codec.clone());
        }
        Ok(proposed)
    }

    /// 🧭️ Publishes native snapshot identities under an already acquired assembly barrier.
    pub fn register_native_snapshots_in_assembly(_assembly: &semio_framework_schema_registry::assembly::Transaction, registrations: &[NativeSnapshotRegistration]) -> Result<(), IoRegistryError> {
        let mut registry = native_snapshot_registry().write().map_err(|_| IoRegistryError::Unavailable)?;
        let proposed = propose_native_snapshots(&registry, registrations)?;
        registry.extend(proposed);
        Ok(())
    }

    /// 🔎️ Checks snapshot registration conflicts before any assembly publication.
    pub fn preflight_native_snapshots(registrations: &[NativeSnapshotRegistration]) -> Result<(), IoRegistryError> {
        let registry = native_snapshot_registry().read().map_err(|_| IoRegistryError::Unavailable)?;
        propose_native_snapshots(&registry, registrations).map(|_| ())
    }

    /// 🏛️ Resolves the exact registered dialect's handwritten semantic SQL schema.
    pub fn native_snapshot_sqlite_schema(dialect: &ArtifactDialect) -> Result<String, IoRegistryError> {
        let registry = native_snapshot_registry().read().map_err(|_| IoRegistryError::Unavailable)?;
        let codec = registry.get(dialect).ok_or_else(|| IoRegistryError::InvalidSnapshotDialect(dialect.clone()))?;
        let provider = codec.snapshot_sqlite.as_ref().ok_or_else(|| IoRegistryError::InvalidSnapshotDialect(dialect.clone()))?;
        Ok(provider.schema.to_string())
    }

    fn snapshot_descriptor(from: ArtifactDialect, into: ArtifactDialect) -> IoEntryDescriptor {
        IoEntryDescriptor { from, into, fidelity: IoFidelity::Exact, sniffs: false }
    }

    fn supplemental_snapshot_entry(snapshots: &NativeSnapshotMap, from: &ArtifactDialect, into: &ArtifactDialect) -> Option<IoEntryDescriptor> {
        let sqlite = ArtifactDialect::from(SQLITE_SNAPSHOT);
        if (into == &sqlite && snapshots.contains_key(from)) || (from == &sqlite && snapshots.contains_key(into)) {
            Some(snapshot_descriptor(from.clone(), into.clone()))
        } else {
            None
        }
    }

    const SNAPSHOT_METADATA_SQL: &str = include_str!("🪶️sqlite-snapshot/🧬️schema/🗄️.sql");

    fn validate_snapshot_schema(database: &SqliteDatabase, schema: &str, phase: SqliteSnapshotPhase, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        crate::sqlite_snapshot::validate_sqlite_database_schema_controlled(database, schema, phase, control)
    }

    fn validate_snapshot_coordinate(kind: &str, standard: &str, subset: &str, control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<(), ValueError> {
        if kind.is_empty() || standard.is_empty() || subset.is_empty() { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "snapshot SQLite metadata dialect is invalid")); }
        for (index, text) in [kind, standard, subset].iter().enumerate() {
            let mut position = 0usize;
            let mut characters = 0usize;
            for character in text.chars() {
                if character.is_control() || index == 0 && character == '@' || index == 2 && character == '/' { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "snapshot SQLite metadata dialect is invalid")); }
                position += character.len_utf8();
                characters += 1;
                if characters % 256 == 0 { control.checkpoint(phase, position, text.len())?; }
            }
            control.checkpoint(phase, position, text.len())?;
        }
        Ok(())
    }

    fn validate_snapshot_metadata<'a>(table: &'a SqliteTable, control: &mut SqliteSnapshotControl<'_>) -> Result<(&'a str, &'a str, &'a str, SnapshotEncoding), ValueError> {
        crate::sqlite_snapshot::validate_sqlite_table_schema_controlled(table, SNAPSHOT_METADATA_SQL, SqliteSnapshotPhase::ReconstructSnapshot, control)?;
        if table.rows.len() != 1 || table.rows[0].rowid != 1 { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "snapshot SQLite metadata schema or row count is invalid")); }
        let [SqliteValue::Integer(1), SqliteValue::Text(kind), SqliteValue::Text(standard), SqliteValue::Text(subset), SqliteValue::Integer(1), SqliteValue::Text(encoding)] = table.rows[0].values.as_slice() else { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "snapshot SQLite metadata values are invalid")); };
        validate_snapshot_coordinate(kind, standard, subset, control, SqliteSnapshotPhase::ReconstructSnapshot)?;
        Ok((kind, standard, subset, SnapshotEncoding::parse(encoding)?))
    }

    /// 🪪️ Reads the decomposed coordinate using an operation-owned admission authority.
    pub fn sqlite_snapshot_metadata(database: &SqliteDatabase) -> Result<(ArtifactDialect, SnapshotEncoding), ValueError> {
        let mut callback = |_| true;
        let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default());
        let table = database.tables.iter().find(|table| table.name == "semio_snapshot").ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "SQLite file has no semantic snapshot metadata"))?;
        let (kind, standard, subset, encoding) = validate_snapshot_metadata(table, &mut control)?;
        let copy = crate::sqlite_snapshot::transfer::copy_text;
        Ok((ArtifactDialect { artifact_kind: copy(kind, SqliteSnapshotPhase::ReconstructSnapshot, &mut control)?, standard: copy(standard, SqliteSnapshotPhase::ReconstructSnapshot, &mut control)?, subset: copy(subset, SqliteSnapshotPhase::ReconstructSnapshot, &mut control)? }, encoding))
    }

    /// 🪪️ Admits the literal metadata cells and their actual backing before ownership.
    pub fn attach_sqlite_snapshot_metadata(database: &mut SqliteDatabase, dialect: &ArtifactDialect, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        use crate::sqlite_snapshot::transfer::{reserve, copy_text};
        let phase = SqliteSnapshotPhase::ProjectSnapshot;
        control.checkpoint(phase, 0, 0)?;
        validate_snapshot_coordinate(&dialect.artifact_kind, &dialect.standard, &dialect.subset, control, phase)?;
        if database.tables.iter().any(|table| table.name.eq_ignore_ascii_case("semio_snapshot")) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "domain schema cannot claim reserved snapshot metadata")); }
        let limits = control.limits();
        if database.tables.len().checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "snapshot table count overflow"))? > limits.max_tables || limits.max_columns < 6 { return Err(ValueError::new(ValueRefusalKind::WorkLimit, "snapshot metadata exceeds table or column limit")); }
        let mut rows = 1usize;
        let sql = SNAPSHOT_METADATA_SQL.trim().trim_end_matches(';');
        let mut bytes = 16usize.checked_add(dialect.artifact_kind.len()).and_then(|v| v.checked_add(dialect.standard.len())).and_then(|v| v.checked_add(dialect.subset.len())).and_then(|v| v.checked_add(encoding.as_str().len())).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "snapshot metadata byte count overflow"))?;
        let mut schema_bytes = sql.len().checked_add("semio_snapshot".len()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "snapshot schema byte count overflow"))?;
        for table in &database.tables {
            schema_bytes = schema_bytes.checked_add(table.sql.len()).and_then(|v| v.checked_add(table.name.len())).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "snapshot schema byte count overflow"))?;
            for row in &table.rows {
                rows = rows.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "snapshot row count overflow"))?;
                for value in &row.values { bytes = bytes.checked_add(match value { SqliteValue::Null => 0, SqliteValue::Integer(_) | SqliteValue::Real(_) => 8, SqliteValue::Text(text) => text.len(), SqliteValue::Blob(blob) => blob.len() }).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "snapshot value byte count overflow"))?; }
                if rows % 256 == 0 { control.checkpoint(phase, rows, 0)?; }
            }
        }
        control.check_rows(rows)?;
        control.check_value_bytes(bytes)?;
        if schema_bytes > limits.max_schema_bytes { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "snapshot metadata exceeds schema byte limit")); }
        let mut values = reserve(6, control)?;
        values.push(SqliteValue::Integer(1));
        for text in [&dialect.artifact_kind, &dialect.standard, &dialect.subset] { values.push(SqliteValue::Text(copy_text(text, phase, control)?)); }
        values.push(SqliteValue::Integer(1));
        values.push(SqliteValue::Text(copy_text(encoding.as_str(), phase, control)?));
        let mut metadata_rows = reserve(1, control)?;
        metadata_rows.push(SqliteRow { rowid: 1, values });
        let name = copy_text("semio_snapshot", phase, control)?;
        let sql = copy_text(sql, phase, control)?;
        if database.tables.len() == database.tables.capacity() {
            let mut tables = reserve(database.tables.len().checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "snapshot table count overflow"))?, control)?;
            let total = database.tables.len();
            for (index, table) in database.tables.drain(..).enumerate() { tables.push(table); if index % 256 == 0 { control.checkpoint(phase, index, total)?; } }
            database.tables = tables;
        }
        database.tables.push(SqliteTable { name, sql, rows: metadata_rows });
        control.checkpoint(phase, rows, rows)
    }

    /// 🪪️ Moves validated metadata out without cloning its literal coordinate fields.
    pub fn take_sqlite_snapshot_metadata_controlled(database: &mut SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<(ArtifactDialect, SnapshotEncoding), ValueError> {
        let index = database.tables.iter().position(|table| table.name == "semio_snapshot").ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "SQLite file has no semantic snapshot metadata"))?;
        let (_, _, _, encoding) = validate_snapshot_metadata(&database.tables[index], control)?;
        let count = database.tables.len();
        for at in index..count - 1 { database.tables.swap(at, at + 1); if at % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, at, count)?; } }
        let mut table = database.tables.pop().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "snapshot metadata disappeared"))?;
        let mut row = table.rows.pop().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "snapshot metadata row disappeared"))?;
        let mut values = row.values.drain(..);
        values.next();
        let kind = match values.next() { Some(SqliteValue::Text(v)) => v, _ => return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "snapshot metadata kind disappeared")) };
        let standard = match values.next() { Some(SqliteValue::Text(v)) => v, _ => return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "snapshot metadata standard disappeared")) };
        let subset = match values.next() { Some(SqliteValue::Text(v)) => v, _ => return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "snapshot metadata subset disappeared")) };
        Ok((ArtifactDialect { artifact_kind: kind, standard, subset }, encoding))
    }

    /// 🪪️ Extracts metadata under one operation-owned control.
    pub fn take_sqlite_snapshot_metadata(database: &mut SqliteDatabase) -> Result<(ArtifactDialect, SnapshotEncoding), ValueError> {
        take_sqlite_snapshot_metadata_controlled(database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default()))
    }
    // 🐛️ terra-io-thunks: every call site below awaits this before `.read()`/`.write()` — the
    // codemod that added `async` here left all five callers unfixed (the `no .await needed` case
    // insert-await.py handles cannot help: cargo never reached this file to emit the diagnostic,
    // see 📓️terra-io-thunks-report.md).

    /// ⚠️ A duplicate `(from, into)` registration for a genuinely different entry, or a poisoned
    /// registry lock.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum IoRegistryError {
        Duplicate { from: Box<ArtifactDialect>, into: Box<ArtifactDialect> },
        InvalidSnapshotDialect(ArtifactDialect),
        InvalidSnapshotSchema { dialect: ArtifactDialect, message: String },
        Unavailable,
    }

    impl std::fmt::Display for IoRegistryError {
        // 🐛️ terra-io-thunks: `Display::fmt` is E1 (fixed sync signature) so it cannot `.await`
        // the now-`async` `ArtifactDialect::to_coordinate` — `resolve_ready` bridges it exactly
        // like every other forced-sync call site in this file (`to_coordinate` never truly
        // suspends).
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Duplicate { from, into } => write!(f, "io entry already registered for {} -> {}", from.to_coordinate(), into.to_coordinate()),
                Self::InvalidSnapshotDialect(dialect) => write!(f, "invalid native snapshot dialect {}", dialect.to_coordinate()),
                Self::InvalidSnapshotSchema { dialect, message } => write!(f, "invalid semantic SQLite schema for {}: {message}", dialect.to_coordinate()),
                Self::Unavailable => f.write_str("io mechanism registry unavailable"),
            }
        }
    }
    impl std::error::Error for IoRegistryError {}

    // 🐛️ terra-io-thunks: was a plain `fn` calling the now-async `same_io_entry` from an
    // unawaitable match guard (E0308: expected bool, found future). `build_proposed` is a regular
    // helper, not a fn-pointer slot, so per R2's O1 it keeps the literal `async` keyword rather
    // than staying sync — restructured off the match guard so the `.await` has a plain expression
    // position to live in.
    fn build_proposed(entries: &[&'static IoEntry]) -> Result<EntryMap, IoRegistryError> {
        let mut proposed: EntryMap = BTreeMap::new();
        for entry in entries {
            let key = (ArtifactDialect::from(entry.from), ArtifactDialect::from(entry.into));
            if key.0 == ArtifactDialect::from(SQLITE_SNAPSHOT) || key.1 == ArtifactDialect::from(SQLITE_SNAPSHOT) {
                return Err(IoRegistryError::InvalidSnapshotDialect(ArtifactDialect::from(SQLITE_SNAPSHOT)));
            }
            if let Some(existing) = proposed.get(&key) {
                if same_io_entry(existing, entry) {
                    continue;
                }
                return Err(IoRegistryError::Duplicate { from: Box::new(key.0), into: Box::new(key.1) });
            }
            proposed.insert(key, entry);
        }
        Ok(proposed)
    }

    fn validate_against(existing: &EntryMap, proposed: &EntryMap) -> Result<(), IoRegistryError> {
        for (key, entry) in proposed {
            if let Some(current) = existing.get(key) {
                if !same_io_entry(current, entry) {
                    return Err(IoRegistryError::Duplicate { from: Box::new(key.0.clone()), into: Box::new(key.1.clone()) });
                }
            }
        }
        Ok(())
    }

    /// 📌️ Registers `entries` atomically under the SAME `semio_framework_schema_registry::assembly::begin()`
    /// publication barrier `register_composer_entries` (the OLD mechanism, above) uses, so a
    /// plugin's old- and new-mechanism registrations never interleave. A duplicate `(from, into)`
    /// key for a genuinely DIFFERENT entry is a typed error and leaves the registry unchanged
    /// (preflight via `build_proposed`+`validate_against`, THEN commit — all-or-nothing, proven by
    /// `registration_is_all_or_nothing` below); re-registering the identical static entry is
    /// idempotent, mirroring `register_composer_entries`.
    pub fn io_register(entries: &'static [IoEntry]) -> Result<(), IoRegistryError> {
        let _assembly = semio_framework_schema_registry::assembly::begin().map_err(|_| IoRegistryError::Unavailable)?;
        let proposed = build_proposed(&entries.iter().collect::<Vec<_>>())?;
        let mut registry = io_mechanism_registry().write().map_err(|_| IoRegistryError::Unavailable)?;
        validate_against(&registry, &proposed)?;
        for (key, entry) in proposed {
            registry.entry(key).or_insert(entry);
        }
        Ok(())
    }
    //#endregion 🔖️Registry

    //#region 🔖️Route
    // 🐛️ terra-io-thunks: recursive `async fn` needs its own recursive call boxed (E0733) — the
    // codemod added `async` to a self-recursive traversal without that step. `Box::pin(..).await`
    // is the standard idiom, not an E-class exception (nothing here is a fn-pointer slot).
    async fn walk_routes(registry: &EntryMap, current: &ArtifactDialect, into: &ArtifactDialect, remaining_hops: u8, path: &mut Vec<&'static IoEntry>, visited: &mut BTreeSet<ArtifactDialect>, candidates: &mut Vec<Vec<&'static IoEntry>>) {
        if remaining_hops == 0 {
            return;
        }
        for (key, entry) in registry.iter() {
            if &key.0 != current || visited.contains(&key.1) {
                continue;
            }
            let next = key.1.clone();
            path.push(entry);
            if &next == into {
                candidates.push(path.clone());
            } else {
                visited.insert(next.clone());
                Box::pin(walk_routes(registry, &next, into, remaining_hops - 1, path, visited, candidates)).await;
                visited.remove(&next);
            }
            path.pop();
        }
    }

    async fn route_rank(route: &[&'static IoEntry]) -> (std::cmp::Reverse<u8>, usize, String) {
        let min_fidelity = route.iter().map(|entry| entry.fidelity.rank()).min().unwrap_or(0);
        let joined = route.iter().map(|entry| ArtifactDialect::from(entry.into).to_coordinate()).collect::<Vec<_>>().join(",");
        (std::cmp::Reverse(min_fidelity), route.len(), joined)
    }

    async fn rank_to_fidelity(rank: u8) -> IoFidelity {
        match rank {
            3 => IoFidelity::Exact,
            2 => IoFidelity::Canonical,
            1 => IoFidelity::Semantic,
            _ => IoFidelity::Lossy,
        }
    }

    /// 🌉️ Breadth-bounded, cycle-free enumeration of every simple path `from -> into` up to
    /// `max_hops` (clamped to ≤3), picking the best by (1) highest MINIMUM fidelity along the
    /// route, (2) fewest hops, (3) lexicographic order of the joined `to_coordinate()` strings of
    /// every dialect visited after `from`. Sorting the FULL candidate set at the end (rather than
    /// short-circuiting on the first hit) plus the registry already being a `BTreeMap` (whose
    /// iteration order never depends on insertion order) together give `route_is_deterministic`.
    // 🐛️ terra-io-thunks: `Vec::sort_by`'s comparator is a plain synchronous `FnMut` (std's fixed
    // signature, not ours to make async) so it can never itself call the now-`async`
    // `route_rank`/`rank_to_fidelity`/`descriptor_of` — those stay `async fn` per O1 (they are
    // regular helpers, not fn-pointer slots) and this function awaits them into plain values FIRST,
    // then sorts/maps over the already-resolved data synchronously.
    async fn resolve_route(registry: &EntryMap, from: &ArtifactDialect, into: &ArtifactDialect, max_hops: u8) -> IoResult<IoRoute> {
        let max_hops = max_hops.min(3);
        if max_hops == 0 {
            return Err(refusal(ValueRefusalKind::InvalidValue, format!("io_route {} -> {}: max_hops clamped to 0", from.to_coordinate(), into.to_coordinate())));
        }
        let mut candidates: Vec<Vec<&'static IoEntry>> = Vec::new();
        let mut path: Vec<&'static IoEntry> = Vec::new();
        let mut visited: BTreeSet<ArtifactDialect> = BTreeSet::new();
        visited.insert(from.clone());
        walk_routes(registry, from, into, max_hops, &mut path, &mut visited, &mut candidates).await;
        if candidates.is_empty() {
            return Err(refusal(ValueRefusalKind::UnsupportedOwner, format!("no io route from {} to {} within {max_hops} hops", from.to_coordinate(), into.to_coordinate())));
        }
        let mut ranked = Vec::with_capacity(candidates.len());
        for route in candidates {
            let rank = route_rank(&route).await;
            ranked.push((rank, route));
        }
        ranked.sort_by(|a, b| a.0.cmp(&b.0));
        let best = ranked.into_iter().next().expect("candidates checked non-empty above").1;
        let fidelity = rank_to_fidelity(best.iter().map(|entry| entry.fidelity.rank()).min().expect("a route has at least one hop")).await;
        let mut hops = Vec::with_capacity(best.len());
        for entry in &best {
            hops.push(descriptor_of(entry));
        }
        Ok(IoOutcome::clean(IoRoute { hops, fidelity }))
    }

    /// 🌉️ `resolve_route` against the process-wide registry.
    pub async fn io_route(from: &ArtifactDialect, into: &ArtifactDialect, max_hops: u8) -> IoResult<IoRoute> {
        if max_hops > 0 {
            let snapshots = native_snapshot_registry().read().map_err(|_| refusal(ValueRefusalKind::InvariantViolated, "native snapshot registry unavailable"))?;
            if let Some(hop) = supplemental_snapshot_entry(&snapshots, from, into) {
                return Ok(IoOutcome::clean(IoRoute { hops: vec![hop], fidelity: IoFidelity::Exact }));
            }
        }
        let registry = io_mechanism_registry().read().map_err(|_| refusal(ValueRefusalKind::InvariantViolated, "io mechanism registry unavailable"))?.clone();
        resolve_route(&registry, from, into, max_hops).await
    }

    fn typed_snapshot_codec<P: store::ArtifactSqliteSnapshot + 'static>(dialect: &ArtifactDialect) -> Result<store::ArtifactCodec, IoError> {
        let codec = native_snapshot_registry().read().map_err(|_| refusal(ValueRefusalKind::InvariantViolated, "native snapshot registry unavailable"))?.get(dialect).cloned().ok_or_else(|| refusal(ValueRefusalKind::UnsupportedOwner, format!("unregistered typed snapshot dialect {}", dialect.to_coordinate())))?;
        let provider = codec.snapshot_sqlite.as_ref().ok_or_else(|| refusal(ValueRefusalKind::UnsupportedOwner, "artifact has no semantic SQLite provider"))?;
        if provider.snapshot_type != Some(std::any::TypeId::of::<P>()) || provider.schema != P::SQLITE_SCHEMA {
            return Err(refusal(ValueRefusalKind::InvalidValue, "typed snapshot does not own the requested exact dialect"));
        }
        Ok(codec)
    }

    fn validate_typed_snapshot_subset<P: store::ArtifactSqliteSnapshot>(dialect: &ArtifactDialect, snapshot: &P, database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<semio_framework_diagnostic::Diagnostic>, IoError> {
        store::validate_owned_sqlite_snapshot_subset(snapshot, dialect, database, control).map(|outcome| outcome.diagnostics)
    }

    /// 📤️ Exports the complete owned snapshot through its exact registered relational model.
    pub async fn io_export_sqlite_snapshot<P: store::ArtifactSqliteSnapshot + 'static>(dialect: &ArtifactDialect, snapshot: &P, encoding: SnapshotEncoding, limits: SqliteDatabaseLimits, progress: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> IoResult<Vec<u8>> {
        typed_snapshot_codec::<P>(dialect)?;
        let mut control = SqliteSnapshotControl::new(progress, limits);
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0).map_err(IoError::from_value_error)?;
        let mut database = snapshot.to_sqlite_database(&mut control).map_err(IoError::from_value_error)?;
        validate_snapshot_schema(&database, P::SQLITE_SCHEMA, SqliteSnapshotPhase::ProjectSnapshot, &mut control).map_err(IoError::from_value_error)?;
        let diagnostics = validate_typed_snapshot_subset(dialect, snapshot, &database, &mut control)?;
        attach_sqlite_snapshot_metadata(&mut database, dialect, encoding, &mut control).map_err(IoError::from_value_error)?;
        export_sqlite_database_controlled(&database, &mut control).map(|value| IoOutcome { value, diagnostics }).map_err(IoError::from_value_error)
    }

    /// 📥️ Reconstructs every typed snapshot field directly from its declared relational tables.
    pub async fn io_import_sqlite_snapshot<P: store::ArtifactSqliteSnapshot + 'static>(dialect: &ArtifactDialect, bytes: &[u8], limits: SqliteDatabaseLimits, progress: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> IoResult<P> {
        typed_snapshot_codec::<P>(dialect)?;
        let mut control = SqliteSnapshotControl::new(progress, limits);
        let mut database = import_sqlite_database_controlled(bytes, &mut control).map_err(IoError::from_value_error)?;
        let (actual, _) = take_sqlite_snapshot_metadata_controlled(&mut database, &mut control).map_err(IoError::from_value_error)?;
        if actual != *dialect { return Err(refusal(ValueRefusalKind::InvalidValue, "typed SQLite snapshot dialect does not match requested dialect")); }
        validate_snapshot_schema(&database, P::SQLITE_SCHEMA, SqliteSnapshotPhase::ReconstructSnapshot, &mut control).map_err(IoError::from_value_error)?;
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, 0).map_err(IoError::from_value_error)?;
        let value = P::from_sqlite_database(&database, &mut control).map_err(IoError::from_value_error)?;
        let diagnostics = validate_typed_snapshot_subset(dialect, &value, &database, &mut control)?;
        Ok(IoOutcome { value, diagnostics })
    }

    fn run_snapshot_hop(snapshots: &NativeSnapshotMap, hop: &IoEntryDescriptor, payload: IoPayload, limits: SqliteDatabaseLimits, progress: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> IoResult<IoPayload> {
        let sqlite = ArtifactDialect::from(SQLITE_SNAPSHOT);
        let mut control = SqliteSnapshotControl::new(progress, limits);
        control.checkpoint(SqliteSnapshotPhase::DecodeNative, 0, 1).map_err(IoError::from_value_error)?;
        if hop.into == sqlite {
            let size = match &payload { IoPayload::Binary(bytes) => bytes.len(), IoPayload::Text(text) => text.len() };
            if size > limits.max_file_bytes || hop.from.to_coordinate().len() > limits.max_value_bytes {
                return Err(refusal(ValueRefusalKind::OwnershipLimit, "native snapshot exceeds SQLite snapshot limits"));
            }
            let codec = snapshots.get(&hop.from).ok_or_else(|| refusal(ValueRefusalKind::UnsupportedOwner, "unregistered native snapshot dialect"))?;
            let provider = codec.snapshot_sqlite.as_ref().ok_or_else(|| refusal(ValueRefusalKind::UnsupportedOwner, "artifact has no handwritten semantic SQLite provider"))?;
            let projected = (provider.export)(&codec.schema, &hop.from, &payload, &mut control)?;
            let mut database = projected.value;
            validate_snapshot_schema(&database, &provider.schema, SqliteSnapshotPhase::ProjectSnapshot, &mut control).map_err(IoError::from_value_error)?;
            control.check_database(&database, SqliteSnapshotPhase::ProjectSnapshot).map_err(IoError::from_value_error)?;
            let diagnostics = projected.diagnostics;
            let encoding = match payload { IoPayload::Binary(_) => SnapshotEncoding::Binary, IoPayload::Text(_) => SnapshotEncoding::Text };
            attach_sqlite_snapshot_metadata(&mut database, &hop.from, encoding, &mut control).map_err(IoError::from_value_error)?;
            return export_sqlite_database_controlled(&database, &mut control).map(|bytes| IoOutcome { value: IoPayload::Binary(bytes), diagnostics }).map_err(IoError::from_value_error);
        }
        let IoPayload::Binary(bytes) = payload else {
            return Err(refusal(ValueRefusalKind::InvalidValue, "SQLite snapshot import requires a binary payload"));
        };
        let mut database = import_sqlite_database_controlled(&bytes, &mut control).map_err(IoError::from_value_error)?;
        let (dialect, encoding) = take_sqlite_snapshot_metadata_controlled(&mut database, &mut control).map_err(IoError::from_value_error)?;
        if dialect != hop.into {
            return Err(refusal(ValueRefusalKind::InvalidValue, "semantic SQLite snapshot dialect does not match requested dialect"));
        }
        let codec = snapshots.get(&hop.into).ok_or_else(|| refusal(ValueRefusalKind::UnsupportedOwner, "unregistered native snapshot dialect"))?;
        let provider = codec.snapshot_sqlite.as_ref().ok_or_else(|| refusal(ValueRefusalKind::UnsupportedOwner, "artifact has no handwritten semantic SQLite provider"))?;
        validate_snapshot_schema(&database, &provider.schema, SqliteSnapshotPhase::ReconstructSnapshot, &mut control).map_err(IoError::from_value_error)?;
        let reconstructed = (provider.import)(&codec.schema, &hop.into, database, encoding, &mut control)?;
        let payload = reconstructed.value;
        let size = match &payload { IoPayload::Binary(bytes) => bytes.len(), IoPayload::Text(text) => text.len() };
        if size > limits.max_file_bytes { return Err(refusal(ValueRefusalKind::OwnershipLimit, "reconstructed native snapshot exceeds SQLite snapshot limits")); }
        Ok(IoOutcome { value: payload, diagnostics: reconstructed.diagnostics })
    }

    async fn resolve_run(registry: &EntryMap, route: &IoRoute, payload: IoPayload) -> IoResult<IoPayload> {
        let mut current = payload;
        let mut diagnostics = Vec::new();
        for hop in &route.hops {
            let key = (hop.from.clone(), hop.into.clone());
            let entry = registry.get(&key).ok_or_else(|| refusal(ValueRefusalKind::UnsupportedOwner, format!("io_run: no entry registered for hop {} -> {}", hop.from.to_coordinate(), hop.into.to_coordinate())))?;
            let outcome = (entry.run)(&current).map_err(|error| IoError { cause: ValueError::new(error.cause.kind, format!("io_run: hop {} -> {} failed: {}", hop.from.to_coordinate(), hop.into.to_coordinate(), error.cause.message)), diagnostics: error.diagnostics })?;
            current = outcome.value;
            diagnostics.extend(outcome.diagnostics);
        }
        Ok(IoOutcome { value: current, diagnostics })
    }

    /// 🌉️ Folds `IoEntry::run` along every hop of `route`, accumulating diagnostics; on failure
    /// the `IoError.cause.message` names which hop (by dialect coordinates) failed.
    pub async fn io_run(route: &IoRoute, payload: IoPayload) -> IoResult<IoPayload> {
        io_run_with_snapshot_control(route, payload, SqliteDatabaseLimits::default(), &mut |_| true).await
    }

    /// 🛑️ Executes I/O with bounded SQLite allocation and page progress; returning false cancels.
    pub async fn io_run_with_snapshot_control(route: &IoRoute, payload: IoPayload, limits: SqliteDatabaseLimits, progress: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> IoResult<IoPayload> {
        if route.hops.is_empty() || route.hops.windows(2).any(|pair| pair[0].into != pair[1].from) {
            return Err(refusal(ValueRefusalKind::InvalidValue, "io_run: route must contain connected hops"));
        }
        let sqlite = ArtifactDialect::from(SQLITE_SNAPSHOT);
        if route.hops.iter().any(|hop| hop.from == sqlite || hop.into == sqlite) {
            if route.hops.len() != 1 || route.fidelity != IoFidelity::Exact {
                return Err(refusal(ValueRefusalKind::InvalidValue, "io_run: SQLite snapshots are exact endpoint transfers and cannot be intermediate hops"));
            }
            let hop = &route.hops[0];
            let native = if hop.from == sqlite { &hop.into } else { &hop.from };
            let codec = native_snapshot_registry().read().map_err(|_| refusal(ValueRefusalKind::InvariantViolated, "native snapshot registry unavailable"))?.get(native).cloned();
            let snapshots = codec.map(|codec| BTreeMap::from([(native.clone(), codec)])).unwrap_or_default();
            if supplemental_snapshot_entry(&snapshots, &hop.from, &hop.into).as_ref() != Some(hop) {
                return Err(refusal(ValueRefusalKind::InvalidValue, "io_run: inconsistent SQLite snapshot descriptor"));
            }
            return run_snapshot_hop(&snapshots, hop, payload, limits, progress);
        }
        let registry = io_mechanism_registry().read().map_err(|_| refusal(ValueRefusalKind::InvariantViolated, "io mechanism registry unavailable"))?.clone();
        resolve_run(&registry, route, payload).await
    }
    //#endregion 🔖️Route

    //#region 🔖️Identify
    async fn resolve_identify(registry: &EntryMap, payload: &IoPayload) -> Vec<(ArtifactDialect, Confidence)> {
        let carrier = ArtifactDialect::from(match payload {
            IoPayload::Binary(_) => CARRIER_BINARY,
            IoPayload::Text(_) => CARRIER_TEXT,
        });
        let mut found: Vec<(ArtifactDialect, Confidence)> = registry
            .values()
            .filter(|entry| ArtifactDialect::from(entry.from) == carrier)
            .filter_map(|entry| entry.sniff.map(|sniff| (ArtifactDialect::from(entry.into), sniff(payload))))
            .filter(|(_, confidence)| *confidence != Confidence::None)
            .collect();
        found.sort_by(|a, b| b.1.rank().cmp(&a.1.rank()).then_with(|| a.0.to_coordinate().cmp(&b.0.to_coordinate())));
        found
    }

    /// 🔍️ Runs `sniff` of every entry whose `from` is the carrier dialect matching `payload`'s own
    /// variant (`CARRIER_BINARY` for `Binary`, `CARRIER_TEXT` for `Text` — never both), drops
    /// `Confidence::None`, sorts by confidence (descending) then coordinate (ascending).
    pub async fn io_identify(payload: &IoPayload) -> Vec<(ArtifactDialect, Confidence)> {
        io_identify_with_snapshot_control(payload, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap_or_default()
    }

    /// 🛑️ Validates semantic SQLite identity under caller-controlled resource and cancellation bounds.
    pub async fn io_identify_with_snapshot_control(payload: &IoPayload, limits: SqliteDatabaseLimits, progress: &mut dyn FnMut(SqliteSnapshotProgress) -> bool) -> Result<Vec<(ArtifactDialect, Confidence)>, IoError> {
        if let IoPayload::Binary(bytes) = payload {
            if bytes.starts_with(b"SQLite format 3\0") {
                let mut database = import_sqlite_database(bytes, limits, progress).map_err(IoError::from_value_error)?;
                if take_sqlite_snapshot_metadata(&mut database).is_ok() && !database.tables.is_empty() {
                    return Ok(vec![(ArtifactDialect::from(SQLITE_SNAPSHOT), Confidence::High)]);
                }
            }
        }
        let registry = io_mechanism_registry().read().map_err(|_| refusal(ValueRefusalKind::InvariantViolated, "io mechanism registry unavailable"))?.clone();
        Ok(resolve_identify(&registry, payload).await)
    }

    /// 📇️ Every registered entry, erased to owned descriptors — the WIT `list-io-entries` guest
    /// export body and the TS `IoEntryDescriptor[]` mirror both use this shape. Builds the vec with
    /// a plain `for` loop rather than `.map(..).collect()` because `descriptor_of` is `async` and a
    /// `Iterator::map` closure (std's fixed `FnMut` signature) cannot itself await.
    pub fn io_entries() -> Vec<IoEntryDescriptor> {
        match io_mechanism_registry().read() {
            Ok(registry) => {
                let mut descriptors = Vec::with_capacity(registry.len());
                for entry in registry.values() {
                    descriptors.push(descriptor_of(entry));
                }
                if let Ok(snapshots) = native_snapshot_registry().read() {
                    let sqlite = ArtifactDialect::from(SQLITE_SNAPSHOT);
                    for dialect in snapshots.keys() {
                        descriptors.push(snapshot_descriptor(dialect.clone(), sqlite.clone()));
                        descriptors.push(snapshot_descriptor(sqlite.clone(), dialect.clone()));
                    }
                    descriptors.sort_by(|a, b| (&a.from, &a.into).cmp(&(&b.from, &b.into)));
                    descriptors.dedup();
                }
                descriptors
            }
            Err(_) => Vec::new(),
        }
    }

    /// 🛣️ Every registered entry from its native side ([`IoNativeRoute`], registry order) — what `describe` lists as a package's
    /// import/export rows. The synthesized native ↔ sqlite-snapshot hops of [`io_entries`] are generic, not registered entries.
    pub fn io_native_routes() -> Vec<IoNativeRoute> {
        let Ok(registry) = io_mechanism_registry().read() else { return Vec::new() };
        registry
            .values()
            .map(|entry| {
                let (from, into) = (ArtifactDialect::from(entry.from), ArtifactDialect::from(entry.into));
                match entry.direction {
                    IoEntryDirection::Export => IoNativeRoute { native: from, foreign: into, direction: entry.direction },
                    IoEntryDirection::Import => IoNativeRoute { native: into, foreign: from, direction: entry.direction },
                }
            })
            .collect()
    }
    //#endregion 🔖️Identify

    //#region 🔖️Constructors
    /// 🎹️ Builds an `IoEntry` from a `Serializer<S>` impl for a binary-pack-native `S`. Bounds on
    /// `store::ArtifactPack` directly rather than inventing a parallel "native decode" trait —
    /// this file already depends on `store::` throughout (e.g. `ArtifactAssemblyRegistryPlan`
    /// a few regions up), so this introduces no new dependency. The plugin's own decode function is
    /// `S::decode_pack`, resolved by monomorphization of the inner `run` item — never a runtime
    /// `fn` pointer parameter threaded through a closure, which `IoEntry.run`'s bare-`fn`-pointer
    /// field cannot capture anyway. This is the smallest constructor shape that keeps a plugin from
    /// ever hand-writing an `IoEntry` literal while never leaking pack/DSL encoding into its code. A composed head carrier
    /// ([`native_carrier`]) hands its owned children to the serializer.
    pub fn serializer_entry<S: store::ArtifactPack, T: Serializer<S>>(own: Dialect) -> IoEntry {
        // 🚫️async: E4 fn-pointer slot — `IoEntry.run` is a bare, non-capturing `fn` pointer;
        // `Serializer::serialize` stays `async fn` (a real trait method) and this thunk drives it
        // to completion synchronously via `resolve_ready` (io-async-signatures already documents
        // every codec body here as suspension-free).
        fn run<S: store::ArtifactPack, T: Serializer<S>>(payload: &IoPayload) -> IoResult<IoPayload> {
            let IoPayload::Binary(bytes) = payload else {
                return Err(refusal(ValueRefusalKind::InvalidValue, "serializer_entry: expected a binary native payload"));
            };
            let (parent, children) = native_carrier(bytes)?;
            let value = S::decode_pack(&parent).map_err(|error| match error { store::PackError::Refusal(pack::PackRefusal::TextRefusal(error)) => text_refusal(error), error => match error.into_value_error() { Ok(error) => IoError::from_value_error(error.under("native pack decode")), Err(error) => refusal(ValueRefusalKind::InvariantViolated, format!("native pack decode: in-memory decode reported a transport failure: {error}")) } })?;
            ::semio_framework_async::poll::resolve_ready(T::serialize(&value, &children))
        }
        IoEntry { from: own, into: T::INTO, fidelity: T::FIDELITY, direction: IoEntryDirection::Export, sniff: None, run: run::<S, T> }
    }

    /// 🎹️ `serializer_entry`'s twin for a DSL-text-native `S` (`store::ArtifactDsl` instead of
    /// `store::ArtifactPack`) — together the smallest constructor pair covering both halves of the
    /// payload law's native encoding (pack XOR DSL). A composed head carrier ([`native_carrier`]) whose parent bytes are the
    /// parent's DSL text hands its owned children to the serializer.
    pub fn serializer_entry_text<S: store::ArtifactDsl, T: Serializer<S>>(own: Dialect) -> IoEntry {
        // 🚫️async: E4 fn-pointer slot — see `serializer_entry`'s twin above.
        fn run<S: store::ArtifactDsl, T: Serializer<S>>(payload: &IoPayload) -> IoResult<IoPayload> {
            let (value, children) = match payload {
                IoPayload::Text(text) => (S::parse_dsl(text).map_err(text_refusal)?, ArchiveChildren::empty()),
                IoPayload::Binary(bytes) if bytes.first() == Some(&store::channel::DOCUMENT_ARCHIVE_VERSION) => {
                    let (parent, children) = native_carrier(bytes)?;
                    let text = std::str::from_utf8(&parent).map_err(|_| refusal(ValueRefusalKind::InvalidValue, "serializer_entry_text: the composed carrier's parent is not DSL text"))?;
                    (S::parse_dsl(text).map_err(text_refusal)?, children)
                }
                IoPayload::Binary(_) => return Err(refusal(ValueRefusalKind::InvalidValue, "serializer_entry_text: expected a text native payload or a composed carrier")),
            };
            ::semio_framework_async::poll::resolve_ready(T::serialize(&value, &children))
        }
        IoEntry { from: own, into: T::INTO, fidelity: T::FIDELITY, direction: IoEntryDirection::Export, sniff: None, run: run::<S, T> }
    }

    // 🚫️async: E4 fn-pointer slot — `IoEntry.sniff` is a bare `fn` pointer; `Deserializer::sniff`
    // stays `async fn` (it has a default body and may be overridden with real decode logic) and
    // this thunk resolves it synchronously via `resolve_ready`.
    fn deserializer_sniff<S, T: Deserializer<S>>(payload: &IoPayload) -> Confidence {
        ::semio_framework_async::poll::resolve_ready(T::sniff(payload))
    }

    /// 🎹️ Builds an `IoEntry` from a `Deserializer<S>` impl for a binary-pack-native `S`. Runs
    /// `T::CONFORMANCE` (D5's `SubsetValidator` replacement) after a successful deserialize,
    /// folding its diagnostics into the returned `IoOutcome` — `conformance_runs_after_deserialize`
    /// below proves the diagnostics reach the caller.
    pub fn deserializer_entry<S: store::ArtifactPack, T: Deserializer<S>>(own: Dialect) -> IoEntry {
        // 🚫️async: E4 fn-pointer slot — see `serializer_entry`'s twin above; `T::CONFORMANCE` is
        // already a plain `fn(&S) -> Vec<Diagnostic>` (never async, see `Deserializer`'s own doc
        // comment) so it needs no `resolve_ready` wrapping, only `T::deserialize`.
        fn run<S: store::ArtifactPack, T: Deserializer<S>>(payload: &IoPayload) -> IoResult<IoPayload> {
            let outcome = ::semio_framework_async::poll::resolve_ready(T::deserialize(payload))?;
            let mut diagnostics = outcome.diagnostics;
            if let Some(conformance) = T::CONFORMANCE {
                diagnostics.extend(conformance(&outcome.value));
            }
            Ok(IoOutcome { value: IoPayload::Binary(outcome.value.encode_pack()), diagnostics })
        }
        IoEntry { from: T::FROM, into: own, fidelity: T::FIDELITY, direction: IoEntryDirection::Import, sniff: Some(deserializer_sniff::<S, T>), run: run::<S, T> }
    }

    /// 🎹️ `deserializer_entry`'s twin for a DSL-text-native `S`.
    pub fn deserializer_entry_text<S: store::ArtifactDsl, T: Deserializer<S>>(own: Dialect) -> IoEntry {
        // 🚫️async: E4 fn-pointer slot — see `deserializer_entry` above.
        fn run<S: store::ArtifactDsl, T: Deserializer<S>>(payload: &IoPayload) -> IoResult<IoPayload> {
            let outcome = ::semio_framework_async::poll::resolve_ready(T::deserialize(payload))?;
            let mut diagnostics = outcome.diagnostics;
            if let Some(conformance) = T::CONFORMANCE {
                diagnostics.extend(conformance(&outcome.value));
            }
            Ok(IoOutcome { value: IoPayload::Text(outcome.value.print_dsl()), diagnostics })
        }
        IoEntry { from: T::FROM, into: own, fidelity: T::FIDELITY, direction: IoEntryDirection::Import, sniff: Some(deserializer_sniff::<S, T>), run: run::<S, T> }
    }
    //#endregion 🔖️Constructors

    //#region 🔖️Laws
    #[cfg(test)]
    include!("🧪️tests/🔬️io-mechanism-laws/🦀️.rs");
    //#endregion 🔖️Laws
}
//#endregion 🔖️IoMechanism
// #endregion io
