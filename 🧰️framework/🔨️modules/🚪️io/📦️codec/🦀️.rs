//! 🌊️ Bounded domain-neutral payload, resource and lossless codec contracts.
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_io_schema::Confidence;

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
    /// ⚓️ Confirms an anchor retains exactly the source range it claims.
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

    /// 📹️ Orders retained opaque records independently of insertion or registry order.
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

    /// 📎️ Verifies the representation promise attached to a completed codec result.
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
    /// ❌️ Builds one structured error without coupling this contract to a parser implementation.
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
/// 📬️ The deterministic representation promise requested from a codec.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodecRepresentation {
    Canonical,
    Lossless,
}

/// 🧯️ Cross-thread cancellation owned by the caller, not a runtime or codec dependency.
#[derive(Clone, Debug, Default)]
pub struct CancellationToken(std::sync::Arc<std::sync::atomic::AtomicBool>);

impl CancellationToken {
    /// 🌱️ Creates an active cancellation token.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🛑️ Cancels every context sharing this token.
    pub async fn cancel(&self) {
        self.0.store(true, std::sync::atomic::Ordering::Release);
    }

    /// 📖️ Reads cancellation with acquire semantics.
    pub async fn is_cancelled(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Acquire)
    }
}

/// 🚧️ Finite caller-owned resource ceilings for one codec invocation.
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

/// 🔏️ Decode policy, including the representation promise, bounded work, and cancellation.
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

/// 💼️ Encode policy, including the representation promise, bounded work, and cancellation.
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
    /// 🆕️ Creates a fresh counter from caller-owned limits and cancellation.
    pub async fn new(limits: CodecLimits, cancellation: CancellationToken) -> Self {
        Self { limits, cancellation, consumption: CodecConsumption::default() }
    }

    /// ↩️ Returns work already consumed by this invocation.
    pub async fn consumption(&self) -> CodecConsumption {
        self.consumption
    }

    /// 🚨️ Fails immediately when the caller has cancelled this operation.
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

    /// 🔲️ Leaves one parser or encoder recursion frame.
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
    /// ▶️ Starts one decode invocation.
    pub async fn new(policy: DecodePolicy) -> Self {
        let budget = CodecBudget::new(policy.limits.clone(), policy.cancellation.clone());
        Self { policy, budget: budget.await, resolver: None }
    }

    /// 🔗️ Starts one decode invocation with the host-owned external resource resolver.
    pub async fn with_resolver(policy: DecodePolicy, resolver: std::sync::Arc<R>) -> Self {
        let budget = CodecBudget::new(policy.limits.clone(), policy.cancellation.clone());
        Self { policy, budget: budget.await, resolver: Some(resolver) }
    }

    /// 🪵️ Creates the only codec-facing bounded view over a payload source. `S` is trivially
    /// generic (R11 part a) — no runtime-chosen implementor lives at this call site.
    pub async fn source<'source, S: PayloadSource>(&'source mut self, source: &'source mut S) -> CodecResult<BoundedPayloadSource<'source, S>> {
        if let Err(error) = source.span().await.validate().await {
            return Err(CodecFailure::error("io.codec.invalid-source-span", format!("invalid payload source span: {error:?}")).await);
        }
        self.budget.ensure_active().await?;
        Ok(CodecOutput { value: BoundedPayloadSource { source, budget: &mut self.budget, policy: &self.policy }, diagnostics: Vec::new() })
    }

    /// 🖥️ Resolves one external source exclusively through the host-owned resolver. The resolved
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

/// 🗂️ Encode invocation state; codecs charge it instead of owning hidden global limits.
/// 🧬️ Generic over the host-owned resolver `R` — see `DecodeContext`'s doc comment and
/// `📌️important.md` R11.
#[derive(Clone)]
pub struct EncodeContext<R: ResourceResolver> {
    pub policy: EncodePolicy,
    pub budget: CodecBudget,
    resolver: Option<std::sync::Arc<R>>,
}

impl<R: ResourceResolver> EncodeContext<R> {
    /// 🔰️ Starts one encode invocation.
    pub async fn new(policy: EncodePolicy) -> Self {
        let budget = CodecBudget::new(policy.limits.clone(), policy.cancellation.clone());
        Self { policy, budget: budget.await, resolver: None }
    }

    /// 🌐️ Starts one encode invocation with the host-owned external resource resolver.
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

    /// ☁️ Resolves one host-owned encoded resource destination. The resolved sink's concrete
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

    /// 📗️ Finalizes an encode result only when host policy has made it canonical or lossless-valid.
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
/// 🎒️ Bounded type-identification result produced before full decode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PayloadSniff {
    pub media_type: Option<String>,
    pub confidence: Confidence,
    pub diagnostics: Vec<CodecDiagnostic>,
}

/// 🔑️ Optional random-access view over a streaming source.
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

/// ⏩️ A forward-only source which may additionally expose random access. `RandomAccess` is an
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

/// 🖨️ A streaming output sink.
pub trait PayloadSink: Send {
    fn write_chunk(&mut self, input: &[u8]) -> impl std::future::Future<Output = CodecResult<()>> + Send;
}

/// 🙋️ Resource request expressed without tying codecs to a filesystem, HTTP client, or host API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceRequest {
    pub locator: String,
    pub expected_media_type: Option<String>,
}

/// 🎞️ Resolves a resource into the common streaming/random-access source contract. `Source`/`Sink`
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

/// 👁️ Host-owned bounded streaming view exposed to codecs. Generic over `S: PayloadSource`
/// (R11 part a) and, like `BoundedRandomAccessPayload`, non-generic over the resolver `R`.
pub struct BoundedPayloadSource<'a, S: PayloadSource> {
    source: &'a mut S,
    budget: &'a mut CodecBudget,
    policy: &'a DecodePolicy,
}

impl<S: PayloadSource> BoundedPayloadSource<'_, S> {
    /// 📡️ Reads one bounded streaming chunk and charges its actual retained bytes.
    pub async fn read_chunk(&mut self, output: &mut [u8]) -> CodecResult<usize> {
        let allowed = self.permitted(output.len()).await?;
        let result = self.source.read_chunk(&mut output[..allowed]).await?;
        self.charge_result(result.value, allowed).await
    }

    /// 🔓️ Opens a bounded random-access view when the source supports it.
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

/// 💰️ A resolver-owned source whose only codec-facing operations share this context's budget.
/// `S` is the resolver's own `ResourceResolver::Source` associated type — the runtime choice made
/// by the implementor, never a box (`📌️important.md` R11 part b).
pub struct ResolvedPayloadSource<'a, S: PayloadSource> {
    source: S,
    budget: &'a mut CodecBudget,
    policy: &'a DecodePolicy,
}

impl<S: PayloadSource> ResolvedPayloadSource<'_, S> {
    /// 📃️ Reads one bounded streaming chunk from a resolved source.
    pub async fn read_chunk(&mut self, output: &mut [u8]) -> CodecResult<usize> {
        let allowed = self.permitted(output.len()).await?;
        let result = self.source.read_chunk(&mut output[..allowed]).await?;
        self.charge_result(result.value, allowed).await
    }

    /// 🔀️ Opens bounded random access when this resolved source supports it.
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

/// 🕳️ Host-owned bounded streaming sink exposed to codecs. Generic over `S: PayloadSink`
/// (R11 part a); write_chunk never reads `policy`, so unlike the source side this only borrows
/// the budget.
pub struct BoundedPayloadSink<'a, S: PayloadSink> {
    sink: &'a mut S,
    budget: &'a mut CodecBudget,
}

impl<S: PayloadSink> BoundedPayloadSink<'_, S> {
    /// 📝️ Writes one bounded chunk while charging work, allocation, and output bytes first.
    pub async fn write_chunk(&mut self, input: &[u8]) -> CodecResult<()> {
        self.budget.charge_work(1).await?;
        self.budget.charge_allocation(input.len() as u64).await?;
        self.budget.charge_write(input.len() as u64).await?;
        self.sink.write_chunk(input).await
    }
}

/// 🟢️ A resolver-owned sink whose only codec-facing operation shares this context's budget.
/// `S` is the resolver's own `ResourceResolver::Sink` associated type (R11 part b).
pub struct ResolvedPayloadSink<'a, S: PayloadSink> {
    sink: S,
    budget: &'a mut CodecBudget,
}

impl<S: PayloadSink> ResolvedPayloadSink<'_, S> {
    /// 💾️ Writes one budgeted chunk to a resolved destination.
    pub async fn write_chunk(&mut self, input: &[u8]) -> CodecResult<()> {
        self.budget.charge_work(1).await?;
        self.budget.charge_allocation(input.len() as u64).await?;
        self.budget.charge_write(input.len() as u64).await?;
        self.sink.write_chunk(input).await
    }
}
//#endregion 🌊️Resources

//#region 🧬️Codecs
/// 🎚️ Codec for a transport-level payload. It owns sniffing, bounded streaming, and output policy.
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

/// 🏺️ Semantic artifact codec layered over a transport `PayloadCodec` implementation.
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


#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
