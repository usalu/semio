// #region io
//! 🚪️ Dialect vocabulary and typed artifact-to-artifact IO dispatch registry.
//! Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase 2 (standards/subsets). Lives beside
//! `🔺️mesh` (not `os`) so plugins and the OS product share one definition without an
//! inverted dependency — same reasoning as mesh's now-retired legacy format enum.

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::sync::RwLock;

//#region 🔖️Dialect
/// 🏅️ A standard slug — the text after `🔖️` in `🏅️standards/🔖️<standard>/` (e.g. "2.0", "ap214", "1").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StandardId(pub &'static str);

/// 🪆️ A subset id — the text materialized as `🪆️subsets/✳️<dir>/`. Real subset ids name an
/// industry-defined conformance profile/class/view of the standard (e.g. `"a"` = PDF/A, `"cc6"` =
/// STEP AP214 CC6 advanced B-Rep, `"rv"` = IFC4 Reference View) — never a version and never a
/// conformance LEVEL (PDF/A-2 vs -3, level "b"/"u"): level is data the subset's own analyzer
/// detects and reports, not part of the id (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES).
/// `ANY` is the unconstrained base subset every standard carries (dir `✳️any`) — there is by
/// definition nothing to validate against it. The vocabulary of real subsets a given standard
/// declares lives in that standard's `🪆️subsets/🔣️component.json` manifest (checked by
/// `policyStandardSubsetVocabularyBreaches` in script.ts and by each standard's own
/// `composer_roster_matches_declared_subset_vocabulary` test), not in this type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubsetId(pub &'static str);

impl SubsetId {
    pub const ANY: SubsetId = SubsetId("*");
}

/// 🎯️ Fully-qualified dialect coordinate: which artifact, which standard, which subset.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dialect {
    pub artifact_kind: &'static str,
    pub standard: StandardId,
    pub subset: SubsetId,
}

/// 🎯️ Owned serde twin of `Dialect` — `Dialect` is `&'static str`-based (fine for compile-time
/// composer registration) and so cannot be persisted, sent over the WIT wire, or built from
/// runtime UI/store input. `ArtifactDialect` is the persisted/wire form; every other dialect
/// consumer (document envelopes, the hub's multi-user pin, WIT `migrate-artifact`, the io leaf
/// generators) should read/write THIS type, converting to/from `Dialect` only at the point a
/// `'static` compose call actually needs one.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactDialect {
    pub artifact_kind: String,
    pub standard: String,
    pub subset: String,
}

impl From<Dialect> for ArtifactDialect {
    fn from(d: Dialect) -> Self {
        ArtifactDialect { artifact_kind: d.artifact_kind.to_string(), standard: d.standard.0.to_string(), subset: d.subset.0.to_string() }
    }
}

impl ArtifactDialect {
    /// 🧵️ Canonical single-string coordinate form: `"s.stdio.gif@87a/*"`. This is the one format
    /// that crosses every boundary in the system (WIT `migrate-artifact` from/to fields, the hub's
    /// `Hello` dialect pin, `ArtifactEnvelope.dialect` when serialized to a human-legible log) —
    /// picking ONE textual encoding here means none of those call sites need their own parser.
    pub fn to_coordinate(&self) -> String {
        format!("{}@{}/{}", self.artifact_kind, self.standard, self.subset)
    }

    /// 🧵️ Inverse of `to_coordinate`. `@` separates artifact_kind from standard/subset; the LAST
    /// `/` separates standard from subset (artifact_kind may itself contain `/`-free dots, e.g.
    /// `s.stdio.gif`, but never a literal `@` or trailing-`/`-ambiguous suffix by construction).
    pub fn parse_coordinate(s: &str) -> Result<Self, String> {
        let (kind, rest) = s.split_once('@').ok_or_else(|| format!("dialect coordinate {s:?} missing '@'"))?;
        let (standard, subset) = rest.rsplit_once('/').ok_or_else(|| format!("dialect coordinate {s:?} missing '/'"))?;
        if kind.is_empty() || standard.is_empty() || subset.is_empty() {
            return Err(format!("dialect coordinate {s:?} has an empty component"));
        }
        Ok(ArtifactDialect { artifact_kind: kind.to_string(), standard: standard.to_string(), subset: subset.to_string() })
    }
}
//#endregion 🔖️Dialect

//#region 🔖️ArtifactRef
/// 🪪️ Canonical artifact-kind id — the ONLY spelling `Dialect.artifact_kind`, schema ids, catalog
/// keys, and format rows are meant to derive from (ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM `📓️design-full-plan.md` section "1. Kernel
/// primitives"). Grammar: exactly three dot-separated ASCII segments, `s.<plugin>.<artifact>` —
/// the first segment is always the literal `s`, the remaining two are lowercase-ASCII kebab
/// (`[a-z0-9-]`, no leading/trailing/doubled hyphen). This wave lands the type and validator
/// only; renaming existing artifact ids to this grammar is a later wave — see
/// `is_canonical_artifact_kind`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtifactKindId(String);

impl ArtifactKindId {
    /// 🧵️ Parses and validates the canonical grammar, failing with a message that names which
    /// rule broke rather than a generic "invalid" — the same courtesy `ArtifactDialect::parse_coordinate`
    /// gives its callers.
    pub fn parse(s: &str) -> Result<Self, String> {
        if !is_canonical_artifact_kind(s) {
            return Err(format!("artifact kind {s:?} is not canonical grammar `s.<plugin>.<artifact>` (three dot-separated ASCII segments, first literally `s`, the rest lowercase-kebab)"));
        }
        Ok(ArtifactKindId(s.to_string()))
    }

    /// 🔍️ Borrowed access to the full `s.<plugin>.<artifact>` string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 🔌️ Second segment — the owning plugin slug.
    pub fn plugin(&self) -> &str {
        self.0.split('.').nth(1).expect("ArtifactKindId invariant: exactly 3 dot-separated segments")
    }

    /// 🗿️ Third segment — the artifact slug within the plugin.
    pub fn artifact(&self) -> &str {
        self.0.split('.').nth(2).expect("ArtifactKindId invariant: exactly 3 dot-separated segments")
    }
}

impl std::fmt::Display for ArtifactKindId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// ✅️ Standalone canonical-grammar predicate behind `ArtifactKindId::parse` — usable wherever a
/// `bool` fits better than a `Result` (e.g. `script.ts`-adjacent policy breach scans).
pub fn is_canonical_artifact_kind(kind: &str) -> bool {
    let mut segments = kind.split('.');
    let Some(first) = segments.next() else { return false };
    if first != "s" {
        return false;
    }
    let Some(plugin) = segments.next() else { return false };
    let Some(artifact) = segments.next() else { return false };
    if segments.next().is_some() {
        return false;
    }
    is_kebab_segment(plugin) && is_kebab_segment(artifact)
}

/// 🔡️ One canonical-grammar segment: non-empty lowercase-ASCII `[a-z0-9-]`, no leading/trailing
/// hyphen, no doubled hyphen.
fn is_kebab_segment(segment: &str) -> bool {
    if segment.is_empty() || segment.starts_with('-') || segment.ends_with('-') || segment.contains("--") {
        return false;
    }
    segment.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// 🔗️ A reference to one artifact: its id plus the dialect it is materialized in. Renders to/from
/// the wire URI `"<artifact_id>!<kind>@<standard>/<subset>"` — the `!` separates identity (which
/// artifact) from dialect (which coordinate it is read/written in). Reuses
/// `ArtifactDialect::to_coordinate`/`parse_coordinate` for the half after `!` so there remains
/// exactly one dialect-coordinate codec in the codebase.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactRef {
    pub artifact_id: String,
    pub dialect: ArtifactDialect,
}

impl ArtifactRef {
    /// 🧵️ Canonical wire form: `"<artifact_id>!<kind>@<standard>/<subset>"`.
    pub fn to_uri(&self) -> String {
        format!("{}!{}", self.artifact_id, self.dialect.to_coordinate())
    }

    /// 🧵️ Inverse of `to_uri`. Splits on the FIRST `!` — the dialect coordinate after it resolves
    /// its own `@`/`/` boundaries via `ArtifactDialect::parse_coordinate`, so an artifact id may
    /// itself contain dots or dashes and still round-trip exactly.
    pub fn parse_uri(s: &str) -> Result<Self, String> {
        let (artifact_id, coordinate) = s.split_once('!').ok_or_else(|| format!("artifact ref uri {s:?} missing '!'"))?;
        if artifact_id.is_empty() {
            return Err(format!("artifact ref uri {s:?} has an empty artifact id"));
        }
        let dialect = ArtifactDialect::parse_coordinate(coordinate)?;
        Ok(ArtifactRef { artifact_id: artifact_id.to_string(), dialect })
    }
}
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
    pub fn validate(&self) -> Result<(), SourceSpanError> {
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
    pub fn validate(&self) -> Result<(), CodecFailure> {
        if self.anchor.trim().is_empty() {
            return Err(CodecFailure::error("io.codec.empty-anchor", "source anchor is empty"));
        }
        self.span.validate().map_err(|error| CodecFailure::error("io.codec.invalid-source-span", format!("invalid anchor {:?}: {error:?}", self.anchor)))?;
        let width = self.span.byte_end - self.span.byte_start;
        if width != self.bytes.len() as u64 {
            return Err(CodecFailure::error("io.codec.anchor-width", format!("anchor {:?} has {} bytes for source width {width}", self.anchor, self.bytes.len())));
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
    pub fn validate_lossless(&self) -> Result<(), CodecFailure> {
        let mut anchors = std::collections::BTreeSet::new();
        for anchor in &self.anchors {
            if !anchors.insert(anchor.anchor.clone()) {
                return Err(CodecFailure::error("io.codec.duplicate-anchor", format!("duplicate anchor {:?}", anchor.anchor)));
            }
            anchor.validate()?;
        }
        let mut opaque_extensions = std::collections::BTreeSet::new();
        for extension in &self.opaque_extensions {
            if extension.kind.trim().is_empty() {
                return Err(CodecFailure::error("io.codec.empty-opaque-kind", "opaque extension kind is empty"));
            }
            extension.source.validate()?;
            let key = (extension.kind.as_str(), extension.source.anchor.as_str(), extension.source.span.byte_start, extension.source.span.byte_end);
            if !opaque_extensions.insert(key) {
                return Err(CodecFailure::error("io.codec.duplicate-opaque-extension", format!("duplicate opaque extension {:?} at {:?}", extension.kind, extension.source.anchor)));
            }
        }
        Ok(())
    }

    /// 📏️ Orders retained opaque records independently of insertion or registry order.
    pub fn canonical_opaque_extensions(&self) -> Vec<&OpaqueExtension> {
        let mut extensions = self.opaque_extensions.iter().collect::<Vec<_>>();
        extensions.sort_by(|a, b| (a.kind.as_str(), a.source.anchor.as_str(), a.source.span.byte_start, a.source.span.byte_end).cmp(&(b.kind.as_str(), b.source.anchor.as_str(), b.source.span.byte_start, b.source.span.byte_end)));
        extensions
    }

    /// 🧭️ Applies the deterministic source-record order required by canonical codec output.
    pub fn canonicalize(&mut self) {
        self.anchors.sort_by(|a, b| (a.anchor.as_str(), a.span.byte_start, a.span.byte_end).cmp(&(b.anchor.as_str(), b.span.byte_start, b.span.byte_end)));
        self.opaque_extensions.sort_by(|a, b| (a.kind.as_str(), a.source.anchor.as_str(), a.source.span.byte_start, a.source.span.byte_end).cmp(&(b.kind.as_str(), b.source.anchor.as_str(), b.source.span.byte_start, b.source.span.byte_end)));
    }

    /// 🧭️ Verifies the representation promise attached to a completed codec result.
    pub fn validate_representation(&self, representation: CodecRepresentation) -> Result<(), CodecFailure> {
        self.validate_lossless()?;
        if representation == CodecRepresentation::Canonical {
            let mut canonical = self.anchors.iter().collect::<Vec<_>>();
            canonical.sort_by_key(|anchor| (anchor.anchor.as_str(), anchor.span.byte_start, anchor.span.byte_end));
            if canonical != self.anchors.iter().collect::<Vec<_>>() {
                return Err(CodecFailure::error("io.codec.noncanonical-anchors", "canonical result anchors are not deterministically ordered"));
            }
            if self.canonical_opaque_extensions() != self.opaque_extensions.iter().collect::<Vec<_>>() {
                return Err(CodecFailure::error("io.codec.noncanonical-opaque-extensions", "canonical opaque extensions are not deterministically ordered"));
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
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
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
    /// 🌱️ Creates an active cancellation token.
    pub fn new() -> Self {
        Self::default()
    }

    /// 🛑️ Cancels every context sharing this token.
    pub fn cancel(&self) {
        self.0.store(true, std::sync::atomic::Ordering::Release);
    }

    /// 🔎️ Reads cancellation with acquire semantics.
    pub fn is_cancelled(&self) -> bool {
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
    pub fn new(limits: CodecLimits, cancellation: CancellationToken) -> Self {
        Self { limits, cancellation, consumption: CodecConsumption::default() }
    }

    /// 🔎️ Returns work already consumed by this invocation.
    pub fn consumption(&self) -> CodecConsumption {
        self.consumption
    }

    /// 🛑️ Fails immediately when the caller has cancelled this operation.
    pub fn ensure_active(&self) -> Result<(), CodecFailure> {
        if self.cancellation.is_cancelled() {
            return Err(CodecFailure::error("io.codec.cancelled", "codec operation cancelled"));
        }
        Ok(())
    }

    /// 📥️ Charges streaming input bytes before retaining or processing them.
    pub fn charge_read(&mut self, bytes: u64) -> Result<(), CodecFailure> {
        self.ensure_active()?;
        Self::charge(&mut self.consumption.read_bytes, bytes, self.limits.max_read_bytes, "read-bytes")
    }

    /// 📤️ Charges output bytes before a host sink accepts them.
    pub fn charge_write(&mut self, bytes: u64) -> Result<(), CodecFailure> {
        self.ensure_active()?;
        Self::charge(&mut self.consumption.written_bytes, bytes, self.limits.max_written_bytes, "written-bytes")
    }

    /// ⚙️ Charges deterministic implementation work units.
    pub fn charge_work(&mut self, units: u64) -> Result<(), CodecFailure> {
        self.ensure_active()?;
        Self::charge(&mut self.consumption.work_units, units, self.limits.max_work_units, "work-units")
    }

    /// 🧱️ Charges a logical allocation before allocating externally supplied data.
    pub fn charge_allocation(&mut self, allocations: u64) -> Result<(), CodecFailure> {
        self.ensure_active()?;
        Self::charge(&mut self.consumption.allocations, allocations, self.limits.max_allocations, "allocations")
    }

    /// 🪆️ Enters a bounded parser or encoder recursion frame.
    pub fn enter_recursion(&mut self) -> Result<(), CodecFailure> {
        self.ensure_active()?;
        if self.consumption.recursion_depth >= self.limits.max_recursion_depth {
            return Err(CodecFailure::error("io.codec.recursion-exhausted", format!("recursion budget {} exhausted", self.limits.max_recursion_depth)));
        }
        self.consumption.recursion_depth += 1;
        self.consumption.peak_recursion_depth = self.consumption.peak_recursion_depth.max(self.consumption.recursion_depth);
        Ok(())
    }

    /// 🪆️ Leaves one parser or encoder recursion frame.
    pub fn leave_recursion(&mut self) -> Result<(), CodecFailure> {
        self.ensure_active()?;
        self.consumption.recursion_depth = self.consumption.recursion_depth.checked_sub(1).ok_or_else(|| CodecFailure::error("io.codec.recursion-underflow", "codec left a recursion frame it did not enter"))?;
        Ok(())
    }

    fn charge(used: &mut u64, increment: u64, limit: u64, resource: &str) -> Result<(), CodecFailure> {
        let next = used.checked_add(increment).ok_or_else(|| CodecFailure::error("io.codec.budget-overflow", format!("{resource} counter overflow")))?;
        if next > limit {
            return Err(CodecFailure::error("io.codec.budget-exhausted", format!("{resource} budget {limit} exhausted by request for {increment}")));
        }
        *used = next;
        Ok(())
    }
}

/// 🔍️ Decode invocation state; codecs charge it instead of owning hidden global limits.
#[derive(Clone)]
pub struct DecodeContext {
    pub policy: DecodePolicy,
    pub budget: CodecBudget,
    resolver: Option<std::sync::Arc<dyn ResourceResolver>>,
}

impl DecodeContext {
    /// 🌱️ Starts one decode invocation.
    pub fn new(policy: DecodePolicy) -> Self {
        let budget = CodecBudget::new(policy.limits.clone(), policy.cancellation.clone());
        Self { policy, budget, resolver: None }
    }

    /// 🔗️ Starts one decode invocation with the host-owned external resource resolver.
    pub fn with_resolver(policy: DecodePolicy, resolver: std::sync::Arc<dyn ResourceResolver>) -> Self {
        let budget = CodecBudget::new(policy.limits.clone(), policy.cancellation.clone());
        Self { policy, budget, resolver: Some(resolver) }
    }

    /// 🌊️ Creates the only codec-facing bounded view over a payload source.
    pub fn source<'source>(&'source mut self, source: &'source mut dyn PayloadSource) -> CodecResult<BoundedPayloadSource<'source>> {
        source.span().validate().map_err(|error| CodecFailure::error("io.codec.invalid-source-span", format!("invalid payload source span: {error:?}")))?;
        self.budget.ensure_active()?;
        Ok(CodecOutput { value: BoundedPayloadSource { source, context: self }, diagnostics: Vec::new() })
    }

    /// 🔗️ Resolves one external source exclusively through the host-owned resolver.
    pub fn resolve<'context>(&'context mut self, request: &ResourceRequest) -> CodecResult<ResolvedPayloadSource<'context>> {
        self.budget.ensure_active()?;
        let resolver = self.resolver.clone().ok_or_else(|| CodecFailure::error("io.codec.resource-resolver-unavailable", "decode context has no resource resolver"))?;
        let resolved = resolver.resolve_decode(request)?;
        resolved.value.span().validate().map_err(|error| CodecFailure::error("io.codec.invalid-source-span", format!("invalid resolved payload source span: {error:?}")))?;
        Ok(CodecOutput { value: ResolvedPayloadSource { source: resolved.value, context: self }, diagnostics: resolved.diagnostics })
    }

    /// ✅️ Finalizes a decode result only when its requested representation is valid.
    pub fn finalize_result<T>(&mut self, mut result: ArtifactCodecResult<T>) -> CodecResult<ArtifactCodecResult<T>> {
        self.budget.charge_work(1)?;
        if self.policy.representation == CodecRepresentation::Canonical {
            result.canonicalize();
        }
        result.validate_representation(self.policy.representation)?;
        Ok(CodecOutput { value: result, diagnostics: Vec::new() })
    }
}

/// 🔍️ Encode invocation state; codecs charge it instead of owning hidden global limits.
#[derive(Clone)]
pub struct EncodeContext {
    pub policy: EncodePolicy,
    pub budget: CodecBudget,
    resolver: Option<std::sync::Arc<dyn ResourceResolver>>,
}

impl EncodeContext {
    /// 🌱️ Starts one encode invocation.
    pub fn new(policy: EncodePolicy) -> Self {
        let budget = CodecBudget::new(policy.limits.clone(), policy.cancellation.clone());
        Self { policy, budget, resolver: None }
    }

    /// 🔗️ Starts one encode invocation with the host-owned external resource resolver.
    pub fn with_resolver(policy: EncodePolicy, resolver: std::sync::Arc<dyn ResourceResolver>) -> Self {
        let budget = CodecBudget::new(policy.limits.clone(), policy.cancellation.clone());
        Self { policy, budget, resolver: Some(resolver) }
    }

    /// 🚰️ Creates the only codec-facing bounded view over a payload sink.
    pub fn sink<'sink>(&'sink mut self, sink: &'sink mut dyn PayloadSink) -> CodecResult<BoundedPayloadSink<'sink>> {
        self.budget.ensure_active()?;
        Ok(CodecOutput { value: BoundedPayloadSink { sink, context: self }, diagnostics: Vec::new() })
    }

    /// 🔗️ Resolves one host-owned encoded resource destination.
    pub fn resolve<'context>(&'context mut self, request: &ResourceRequest) -> CodecResult<ResolvedPayloadSink<'context>> {
        self.budget.ensure_active()?;
        let resolver = self.resolver.clone().ok_or_else(|| CodecFailure::error("io.codec.resource-resolver-unavailable", "encode context has no resource resolver"))?;
        let resolved = resolver.resolve_encode(request)?;
        Ok(CodecOutput { value: ResolvedPayloadSink { sink: resolved.value, context: self }, diagnostics: resolved.diagnostics })
    }

    /// ✅️ Finalizes an encode result only when host policy has made it canonical or lossless-valid.
    pub fn finalize_result<T>(&mut self, mut result: ArtifactCodecResult<T>) -> CodecResult<ArtifactCodecResult<T>> {
        self.budget.charge_work(1)?;
        if self.policy.representation == CodecRepresentation::Canonical {
            result.canonicalize();
        }
        result.validate_representation(self.policy.representation)?;
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
    fn len(&self) -> CodecResult<u64>;
    fn read_at(&self, offset: u64, output: &mut [u8]) -> CodecResult<usize>;
}

/// 🌊️ A forward-only source which may additionally expose random access.
pub trait PayloadSource: Send {
    fn span(&self) -> SourceSpan;
    fn read_chunk(&mut self, output: &mut [u8]) -> CodecResult<usize>;
    fn random_access(&self) -> Option<&dyn RandomAccessPayload> {
        None
    }
}

/// 🚰️ A streaming output sink.
pub trait PayloadSink: Send {
    fn write_chunk(&mut self, input: &[u8]) -> CodecResult<()>;
}

/// 🔗️ Resource request expressed without tying codecs to a filesystem, HTTP client, or host API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceRequest {
    pub locator: String,
    pub expected_media_type: Option<String>,
}

/// 🧭️ Resolves a resource into the common streaming/random-access source contract.
pub trait ResourceResolver: Send + Sync {
    fn resolve_decode(&self, request: &ResourceRequest) -> CodecResult<Box<dyn PayloadSource>>;
    fn resolve_encode(&self, request: &ResourceRequest) -> CodecResult<Box<dyn PayloadSink>>;
}

/// 🔒️ Host-owned bounded random-access view exposed to codecs.
pub struct BoundedRandomAccessPayload<'a> {
    source: &'a dyn RandomAccessPayload,
    context: &'a mut DecodeContext,
}

impl BoundedRandomAccessPayload<'_> {
    /// 📏️ Reads the declared resource length while checking cancellation.
    pub fn len(&mut self) -> CodecResult<u64> {
        self.context.budget.ensure_active()?;
        self.context.budget.charge_work(1)?;
        self.source.len()
    }

    /// 🎯️ Reads at one exact offset without letting codecs bypass read/work limits.
    pub fn read_at(&mut self, offset: u64, output: &mut [u8]) -> CodecResult<usize> {
        let allowed = self.permitted(output.len())?;
        let result = self.source.read_at(offset, &mut output[..allowed])?;
        self.charge_result(result.value, allowed)
    }

    fn permitted(&mut self, requested: usize) -> Result<usize, CodecFailure> {
        self.context.budget.ensure_active()?;
        self.context.budget.charge_work(1)?;
        let remaining = self.context.policy.limits.max_read_bytes.saturating_sub(self.context.budget.consumption().read_bytes);
        let permitted = if remaining > usize::MAX as u64 { requested } else { requested.min(remaining as usize) };
        if requested > 0 && permitted == 0 {
            return Err(CodecFailure::error("io.codec.budget-exhausted", "read-bytes budget exhausted"));
        }
        Ok(permitted)
    }

    fn charge_result(&mut self, read: usize, permitted: usize) -> CodecResult<usize> {
        if read > permitted {
            return Err(CodecFailure::error("io.codec.source-overread", format!("payload source returned {read} bytes after being limited to {permitted}")));
        }
        self.context.budget.charge_read(read as u64)?;
        Ok(CodecOutput { value: read, diagnostics: Vec::new() })
    }
}

/// 🔒️ Host-owned bounded streaming view exposed to codecs.
pub struct BoundedPayloadSource<'a> {
    source: &'a mut dyn PayloadSource,
    context: &'a mut DecodeContext,
}

impl BoundedPayloadSource<'_> {
    /// 📥️ Reads one bounded streaming chunk and charges its actual retained bytes.
    pub fn read_chunk(&mut self, output: &mut [u8]) -> CodecResult<usize> {
        let allowed = self.permitted(output.len())?;
        let result = self.source.read_chunk(&mut output[..allowed])?;
        self.charge_result(result.value, allowed)
    }

    /// 🎯️ Opens a bounded random-access view when the source supports it.
    pub fn random_access(&mut self) -> Option<BoundedRandomAccessPayload<'_>> {
        let source = self.source.random_access()?;
        Some(BoundedRandomAccessPayload { source, context: self.context })
    }

    fn permitted(&mut self, requested: usize) -> Result<usize, CodecFailure> {
        self.context.budget.ensure_active()?;
        self.context.budget.charge_work(1)?;
        let remaining = self.context.policy.limits.max_read_bytes.saturating_sub(self.context.budget.consumption().read_bytes);
        let permitted = if remaining > usize::MAX as u64 { requested } else { requested.min(remaining as usize) };
        if requested > 0 && permitted == 0 {
            return Err(CodecFailure::error("io.codec.budget-exhausted", "read-bytes budget exhausted"));
        }
        Ok(permitted)
    }

    fn charge_result(&mut self, read: usize, permitted: usize) -> CodecResult<usize> {
        if read > permitted {
            return Err(CodecFailure::error("io.codec.source-overread", format!("payload source returned {read} bytes after being limited to {permitted}")));
        }
        self.context.budget.charge_read(read as u64)?;
        Ok(CodecOutput { value: read, diagnostics: Vec::new() })
    }
}

/// 🔒️ A resolver-owned source whose only codec-facing operations share this context's budget.
pub struct ResolvedPayloadSource<'a> {
    source: Box<dyn PayloadSource>,
    context: &'a mut DecodeContext,
}

impl ResolvedPayloadSource<'_> {
    /// 📥️ Reads one bounded streaming chunk from a resolved source.
    pub fn read_chunk(&mut self, output: &mut [u8]) -> CodecResult<usize> {
        let allowed = self.permitted(output.len())?;
        let result = self.source.read_chunk(&mut output[..allowed])?;
        self.charge_result(result.value, allowed)
    }

    /// 🎯️ Opens bounded random access when this resolved source supports it.
    pub fn random_access(&mut self) -> Option<BoundedRandomAccessPayload<'_>> {
        Some(BoundedRandomAccessPayload { source: self.source.random_access()?, context: self.context })
    }

    fn permitted(&mut self, requested: usize) -> Result<usize, CodecFailure> {
        self.context.budget.ensure_active()?;
        self.context.budget.charge_work(1)?;
        let remaining = self.context.policy.limits.max_read_bytes.saturating_sub(self.context.budget.consumption().read_bytes);
        let permitted = if remaining > usize::MAX as u64 { requested } else { requested.min(remaining as usize) };
        if requested > 0 && permitted == 0 {
            return Err(CodecFailure::error("io.codec.budget-exhausted", "read-bytes budget exhausted"));
        }
        Ok(permitted)
    }

    fn charge_result(&mut self, read: usize, permitted: usize) -> CodecResult<usize> {
        if read > permitted {
            return Err(CodecFailure::error("io.codec.source-overread", format!("payload source returned {read} bytes after being limited to {permitted}")));
        }
        self.context.budget.charge_read(read as u64)?;
        Ok(CodecOutput { value: read, diagnostics: Vec::new() })
    }
}

/// 🔒️ Host-owned bounded streaming sink exposed to codecs.
pub struct BoundedPayloadSink<'a> {
    sink: &'a mut dyn PayloadSink,
    context: &'a mut EncodeContext,
}

impl BoundedPayloadSink<'_> {
    /// 📤️ Writes one bounded chunk while charging work, allocation, and output bytes first.
    pub fn write_chunk(&mut self, input: &[u8]) -> CodecResult<()> {
        self.context.budget.charge_work(1)?;
        self.context.budget.charge_allocation(input.len() as u64)?;
        self.context.budget.charge_write(input.len() as u64)?;
        self.sink.write_chunk(input)
    }
}

/// 🔒️ A resolver-owned sink whose only codec-facing operation shares this context's budget.
pub struct ResolvedPayloadSink<'a> {
    sink: Box<dyn PayloadSink>,
    context: &'a mut EncodeContext,
}

impl ResolvedPayloadSink<'_> {
    /// 📤️ Writes one budgeted chunk to a resolved destination.
    pub fn write_chunk(&mut self, input: &[u8]) -> CodecResult<()> {
        self.context.budget.charge_work(1)?;
        self.context.budget.charge_allocation(input.len() as u64)?;
        self.context.budget.charge_write(input.len() as u64)?;
        self.sink.write_chunk(input)
    }
}
//#endregion 🌊️Resources

//#region 🧬️Codecs
/// 📦️ Codec for a transport-level payload. It owns sniffing, bounded streaming, and output policy.
pub trait PayloadCodec: Send + Sync {
    type Payload;

    fn sniff(&self, source: &mut BoundedPayloadSource<'_>) -> CodecResult<PayloadSniff>;
    fn decode_payload(&self, source: &mut BoundedPayloadSource<'_>) -> CodecResult<Self::Payload>;
    fn encode_payload(&self, payload: &Self::Payload, sink: &mut BoundedPayloadSink<'_>) -> CodecResult<()>;
}

/// 🗿️ Semantic artifact codec layered over a transport `PayloadCodec` implementation.
pub trait ArtifactCodec: PayloadCodec {
    type Artifact;

    fn dialect(&self) -> &ArtifactDialect;
    fn decode_artifact(&self, payload: Self::Payload, context: &mut DecodeContext) -> CodecResult<Self::Artifact>;
    fn encode_artifact(&self, artifact: &Self::Artifact, context: &mut EncodeContext) -> CodecResult<Self::Payload>;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComposeError {
    pub message: String,
    pub diagnostics: Vec<Diagnostic>,
}
//#endregion 🔖️ComposeTypes

//#region 🔖️ErasedRegistry
/// 🧾️ Erased payload crossing composer/registry boundaries (dispatch, UI, wire).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IoPayload {
    Text(String),
    Binary(Vec<u8>),
}

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

/// 🎹️ Type-erased composer vtable row. Built by a plugin's composer facet from its typed
/// `ArtifactComposer` impl (SDK trait lives in the plugin crate, this struct only carries the
/// erased shape so the registry never needs the plugin's concrete snapshot types).
pub struct ComposerEntry {
    pub writes: Dialect,
    pub reads: &'static [Dialect],
    pub compose: fn(&[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum IoDirection {
    Import,
    Export,
}

/// 🗝️ Owned mirror of two dialects + direction — the registry key. Owned (not `&'static`) so it
/// can be built from runtime UI input (format kind strings) as well as static composer entries.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    Conflict(IoRegistryConflict),
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
            for key in [IoKey::from_owner_counterpart(entry.writes, source, IoDirection::Import), IoKey::from_owner_counterpart(source, entry.writes, IoDirection::Export)] {
                if let Some(existing) = proposed.get(&key) {
                    if !same_composer_entry(existing, entry) {
                        return Err(IoRegistryRegistrationError::Conflict(IoRegistryConflict { key }));
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
                return Err(IoRegistryRegistrationError::Conflict(IoRegistryConflict { key: key.clone() }));
            }
        }
    }
    Ok(())
}

/// 🔬️ Verifies a static composer table against all established keys without mutating the registry.
#[must_use]
pub fn preflight_composer_entries(entries: &'static [ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
    preflight_composer_entry_refs(&entries.iter().collect::<Vec<_>>())
}

/// 🔬️ Verifies independently declared static composers as one atomic candidate set.
#[must_use]
pub fn preflight_composer_entry_refs(entries: &[&'static ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
    let assembly = store::begin_artifact_assembly().map_err(|_| IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    preflight_composer_entry_refs_in_assembly(&assembly, entries)
}

/// 🔬️ Verifies composers while one artifact assembly owns the shared publication barrier.
#[must_use]
pub fn preflight_composer_entry_refs_in_assembly(_assembly: &store::ArtifactAssemblyTransaction, entries: &[&'static ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
    let proposed = composer_entries_by_key(entries.iter().copied())?;
    let registry = io_registry().read().map_err(|_| IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "io-composer" }))?;
    validate_composer_entries(&registry, &proposed)
}

/// 📌️ Registers one artifact's composer entries atomically. Re-registering the exact static entry
/// is idempotent; a different entry for any exact key fails and leaves the registry unchanged.
#[must_use]
pub fn register_composer_entries(entries: &'static [ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
    register_composer_entry_refs(&entries.iter().collect::<Vec<_>>())
}

/// 📌️ Registers independently declared static composers as one all-or-nothing candidate set.
#[must_use]
pub fn register_composer_entry_refs(entries: &[&'static ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
    let assembly = store::begin_artifact_assembly().map_err(|_| IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    register_composer_entry_refs_in_assembly(&assembly, entries)
}

/// 📌️ Publishes preflighted composers while one artifact assembly owns the shared barrier.
#[must_use]
pub fn register_composer_entry_refs_in_assembly(_assembly: &store::ArtifactAssemblyTransaction, entries: &[&'static ComposerEntry]) -> Result<(), IoRegistryRegistrationError> {
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
pub fn resolve(key: &IoKey) -> Result<&'static ComposerEntry, IoResolveError> {
    let reg = io_registry().read().map_err(|_| IoResolveError { message: "io composer registry unavailable".to_string(), candidates: Vec::new(), unavailable: Some(IoRegistryUnavailable { registry: "io-composer" }) })?;
    reg.get(key).copied().ok_or_else(|| IoResolveError {
        message: format!("no composer registered for {}/{}/{} {:?} {}/{}/{}", key.artifact_kind, key.standard, key.subset, key.direction, key.format_kind, key.format_standard, key.format_subset),
        candidates: reg.keys().filter(|k| k.artifact_kind == key.artifact_kind).cloned().collect(),
        unavailable: None,
    })
}

/// 📚️ Lists every dialect one artifact can move data through in a given direction.
#[must_use]
pub fn dialects_for(artifact_kind: &str, direction: IoDirection) -> Result<Vec<Dialect>, IoRegistryUnavailable> {
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
#[must_use]
pub fn io_keys_for(artifact_kind: &str, direction: IoDirection) -> Result<Vec<IoKey>, IoRegistryUnavailable> {
    let reg = io_registry().read().map_err(|_| IoRegistryUnavailable { registry: "io-composer" })?;
    Ok(reg.keys().filter(|key| key.artifact_kind == artifact_kind && key.direction == direction).cloned().collect())
}

/// 📇️ Every registered composer entry, erased to owned dialects -- the shape the WIT
/// `list-artifact-dialects` guest export mirrors verbatim (one row per distinct `writes` entry
/// registered locally, each carrying the full `reads` list).
#[must_use]
pub fn list_composer_entries() -> Result<Vec<(ArtifactDialect, Vec<ArtifactDialect>)>, IoRegistryUnavailable> {
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
/// 🔌️ A host-owned erased fallback executable for cross-plugin IO dispatch.
pub type IoFallback = dyn Fn(&IoKey, &[ErasedComposeSource]) -> Option<Result<ComposedArtifact, ComposeError>> + Send + Sync;

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
/// should reach it (host boot / guest `ensure_plugin_initialized`). Re-registration is idempotent
/// only for the same descriptor and executable identity; every other race is a typed conflict.
#[must_use]
pub fn set_io_fallback_dispatcher(dispatcher: IoFallbackDispatcher) -> Result<(), IoFallbackRegistrationError> {
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
pub fn io_dispatch(key: &IoKey, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    match resolve(key) {
        Ok(entry) => validate_composed_subset((entry.compose)(sources)?),
        Err(local_err) => match IO_FALLBACK.get().and_then(|dispatcher| (dispatcher.dispatch)(key, sources)) {
            Some(result) => validate_composed_subset(result?),
            None => Err(ComposeError { message: local_err.message, diagnostics: Vec::new() }),
        },
    }
}

fn validate_composed_subset(mut composed: ComposedArtifact) -> Result<ComposedArtifact, ComposeError> {
    run_subset_validation(composed.dialect, &composed.payload, &mut composed.diagnostics).map_err(|error| ComposeError { message: format!("subset validation failed: {error:?}"), diagnostics: Vec::new() })?;
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
pub fn io_compose_via(hub: &IoKey, target: &IoKey, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    let hub_composed = io_dispatch(hub, sources)?;
    let hop_source = ErasedComposeSource { dialect: hub_composed.dialect, payload: hub_composed.payload };
    io_dispatch(target, std::slice::from_ref(&hop_source))
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
    fn validate(payload: &IoPayload) -> Vec<Diagnostic>;
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
    SubsetValidatorEntry { dialect: V::DIALECT, validate: V::validate }
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
#[must_use]
pub fn register_subset_validator(entry: &'static SubsetValidatorEntry) -> Result<(), SubsetValidatorRegistryError> {
    register_subset_validators(&[entry])
}

fn validate_subset_validators(registry: &BTreeMap<ArtifactDialect, &'static SubsetValidatorEntry>, entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {
    let mut proposed: BTreeMap<ArtifactDialect, &'static SubsetValidatorEntry> = BTreeMap::new();
    for entry in entries {
        let dialect = ArtifactDialect::from(entry.dialect);
        match proposed.get(&dialect) {
            Some(existing) if same_subset_validator_entry(existing, entry) => {}
            Some(_) => return Err(SubsetValidatorRegistryError::Conflict(SubsetValidatorRegistryConflict { dialect })),
            None => {
                proposed.insert(dialect, entry);
            }
        }
    }
    for (dialect, entry) in proposed {
        match registry.get(&dialect) {
            Some(existing) if same_subset_validator_entry(existing, entry) => {}
            Some(_) => return Err(SubsetValidatorRegistryError::Conflict(SubsetValidatorRegistryConflict { dialect })),
            None => {}
        }
    }
    Ok(())
}

/// 🔬️ Verifies subset-validator entries without changing their established owners.
#[must_use]
pub fn preflight_subset_validators(entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {
    let assembly = store::begin_artifact_assembly().map_err(|_| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    preflight_subset_validators_in_assembly(&assembly, entries)
}

/// 🔬️ Verifies subset validators while one artifact assembly owns the shared publication barrier.
#[must_use]
pub fn preflight_subset_validators_in_assembly(_assembly: &store::ArtifactAssemblyTransaction, entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {
    let registry = subset_validator_registry().read().map_err(|_| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" }))?;
    validate_subset_validators(&registry, entries)
}

/// 📌️ Registers subset-validator entries only when the entire candidate set is conflict-free.
#[must_use]
pub fn register_subset_validators(entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {
    let assembly = store::begin_artifact_assembly().map_err(|_| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    register_subset_validators_in_assembly(&assembly, entries)
}

/// 📌️ Publishes preflighted subset validators while one artifact assembly owns the shared barrier.
#[must_use]
pub fn register_subset_validators_in_assembly(_assembly: &store::ArtifactAssemblyTransaction, entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {
    let mut reg = subset_validator_registry().write().map_err(|_| SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" }))?;
    validate_subset_validators(&reg, entries)?;
    for entry in entries {
        let dialect = ArtifactDialect::from(entry.dialect);
        if !reg.contains_key(&dialect) {
            reg.insert(dialect, entry);
        }
    }
    Ok(())
}

/// 📚️ Every dialect key currently registered in `SUBSET_VALIDATOR_REGISTRY`.
#[must_use]
pub fn list_registered_subset_validator_dialects() -> Result<Vec<Dialect>, SubsetValidatorRegistryError> {
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
fn run_subset_validation(dialect: Dialect, payload: &IoPayload, diagnostics: &mut Vec<Diagnostic>) -> Result<(), SubsetValidationError> {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IoFidelityClass {
    Exact,
    Canonical,
    Semantic,
    Lossy,
}

impl IoFidelityClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Canonical => "canonical",
            Self::Semantic => "semantic",
            Self::Lossy => "lossy",
        }
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "exact" => Ok(Self::Exact),
            "canonical" => Ok(Self::Canonical),
            "semantic" => Ok(Self::Semantic),
            "lossy" => Ok(Self::Lossy),
            other => Err(format!("unknown io fidelity class {other:?}")),
        }
    }

    /// Ordered strength: Exact > Canonical > Semantic > Lossy
    pub fn rank(self) -> u8 {
        match self {
            Self::Exact => 3,
            Self::Canonical => 2,
            Self::Semantic => 1,
            Self::Lossy => 0,
        }
    }
}

/// 📜 Manifest-facing IO fidelity declaration for a subset dialect.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IoFidelityDeclaration {
    pub class: IoFidelityClass,
    /// Field paths dropped under Lossy codecs; must be empty for stronger classes.
    pub drops: Vec<String>,
}

impl IoFidelityDeclaration {
    pub fn validate(&self) -> Result<(), String> {
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
mod io_fidelity_tests {
    use super::*;

    #[test]
    fn io_fidelity_class_parse_and_rank() {
        assert_eq!(IoFidelityClass::parse("exact").unwrap(), IoFidelityClass::Exact);
        assert_eq!(IoFidelityClass::parse("lossy").unwrap(), IoFidelityClass::Lossy);
        assert!(IoFidelityClass::parse("bogus").is_err());
        assert!(IoFidelityClass::Exact.rank() > IoFidelityClass::Canonical.rank());
        assert!(IoFidelityClass::Canonical.rank() > IoFidelityClass::Semantic.rank());
        assert!(IoFidelityClass::Semantic.rank() > IoFidelityClass::Lossy.rank());
        assert_eq!(IoFidelityClass::Exact.as_str(), "exact");
    }

    #[test]
    fn io_fidelity_declaration_validate() {
        IoFidelityDeclaration { class: IoFidelityClass::Exact, drops: vec![] }.validate().unwrap();
        assert!(IoFidelityDeclaration { class: IoFidelityClass::Exact, drops: vec!["x".into()] }.validate().is_err());
        IoFidelityDeclaration { class: IoFidelityClass::Lossy, drops: vec!["meta.author".into()] }.validate().unwrap();
        assert!(IoFidelityDeclaration { class: IoFidelityClass::Lossy, drops: vec![] }.validate().is_err());
    }
}
//#endregion 🔖️IoFidelity

//#region 🔖️Wire
/// 🎹️ Wire twin of `ErasedComposeSource` for crossing a wasm component boundary: `Dialect` is
/// `&'static str`-based and can't be safely deserialized from arbitrary runtime bytes (would need
/// to leak memory per call), so the wire form always carries the owned `ArtifactDialect` and gets
/// resolved back to the real `&'static Dialect` locally by matching coordinate strings against the
/// receiving side's own `ComposerEntry.reads` — exactly like the native W15 dispatch sites in
/// `🧰️framework/🛍️products/💻️os/🦀️component.rs` already do via `io_dialects_for(...).find(...)`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireComposeSource {
    pub dialect: ArtifactDialect,
    pub payload: IoPayload,
}

/// 🎹️ Wire twin of `ComposedArtifact`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

fn validate_wire_dialect(operation: &'static str, dialect: &ArtifactDialect) -> Result<(), IoWireError> {
    for (name, value) in [("artifact_kind", &dialect.artifact_kind), ("standard", &dialect.standard), ("subset", &dialect.subset)] {
        if value.is_empty() || value.len() > MAX_IO_WIRE_DIALECT_COMPONENT_BYTES || value.bytes().any(|byte| byte.is_ascii_whitespace() || byte.is_ascii_control() || matches!(byte, b'@' | b'/' | b'!')) {
            return Err(IoWireError::Limit { operation, detail: format!("invalid bounded dialect {name}") });
        }
    }
    Ok(())
}

fn validate_wire_payload(operation: &'static str, payload: &IoPayload) -> Result<(), IoWireError> {
    let bytes = match payload {
        IoPayload::Text(text) => text.len(),
        IoPayload::Binary(bytes) => bytes.len(),
    };
    if bytes > MAX_IO_WIRE_BYTES {
        return Err(IoWireError::Limit { operation, detail: format!("payload {bytes} bytes exceeds {MAX_IO_WIRE_BYTES}") });
    }
    Ok(())
}

fn validate_wire_key(key: &IoKey) -> Result<(), IoWireError> {
    for value in [&key.artifact_kind, &key.standard, &key.subset, &key.format_kind, &key.format_standard, &key.format_subset] {
        if value.is_empty() || value.len() > MAX_IO_WIRE_DIALECT_COMPONENT_BYTES || value.bytes().any(|byte| byte.is_ascii_whitespace() || byte.is_ascii_control()) {
            return Err(IoWireError::Limit { operation: "io-key", detail: "key component is empty or exceeds the bounded wire grammar".to_string() });
        }
    }
    Ok(())
}

fn encode_wire_json<T: Serialize>(operation: &'static str, value: &T) -> Result<Vec<u8>, IoWireError> {
    let bytes = serde_json::to_vec(value).map_err(|error| IoWireError::Encode { operation, message: error.to_string() })?;
    ensure_wire_bytes(operation, &bytes)?;
    Ok(bytes)
}

fn intern_dialect(dialect: &ArtifactDialect) -> Result<Dialect, IoWireError> {
    validate_wire_dialect("dialect", dialect)?;
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
#[must_use]
pub fn wire_decode_composed_artifact(bytes: &[u8]) -> Result<ComposedArtifact, IoWireError> {
    ensure_wire_bytes("composed-artifact", bytes)?;
    let wire: WireComposedArtifact = serde_json::from_slice(bytes).map_err(|error| IoWireError::Decode { operation: "composed-artifact", message: error.to_string() })?;
    validate_wire_dialect("composed-artifact", &wire.dialect)?;
    validate_wire_payload("composed-artifact", &wire.payload)?;
    if wire.diagnostics.len() > MAX_IO_WIRE_SOURCES {
        return Err(IoWireError::Limit { operation: "composed-artifact", detail: "too many diagnostics".to_string() });
    }
    Ok(ComposedArtifact { dialect: intern_dialect(&wire.dialect)?, payload: wire.payload, diagnostics: wire.diagnostics, confidence: wire.confidence })
}

/// 🌉️ Encodes this process's own composer roster (`list_composer_entries`) as JSON bytes — the
/// body of the WIT `list-artifact-dialects` guest export (see D3, ticket 26/08/10/
/// ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION). JSON (not `pack_rt::encode_wire_value`)
/// is a deliberate simplification for this first cut: the WIT signature is an opaque `list<u8>`
/// either way, so swapping the wire encoding later needs no ABI change, and this module has no
/// existing dependency on `store`/`dsl`'s pack machinery worth introducing just for this.
#[must_use]
pub fn wire_list_composer_entries() -> Result<Vec<u8>, IoWireError> {
    let entries = list_composer_entries().map_err(IoWireError::Registry)?;
    encode_wire_json("composer-entry-list", &entries)
}

/// 🌉️ Decodes a wire `(IoKey, Vec<WireComposeSource>)` request and composes it against THIS
/// process's own local registry only — never the fallback hook. A guest receiving an incoming
/// `artifact-compose` call is, by construction, the plugin the host router already decided owns
/// the key; falling through again here would be a pointless extra hop at best and a reentrancy
/// risk at worst (see the host router's own one-hop guard). The body of the WIT
/// `artifact-compose` guest export. Errors are flattened to a message string, matching how every
/// other fallible call on this ABI surfaces errors (a `Fault`, not structured data) — see
/// `migrate-artifact`'s `plugin-error` for the existing precedent.
#[must_use]
pub fn wire_artifact_compose(key_bytes: &[u8], sources_bytes: &[u8]) -> Result<Vec<u8>, IoWireError> {
    ensure_wire_bytes("io-key", key_bytes)?;
    ensure_wire_bytes("compose-source", sources_bytes)?;
    let key: IoKey = serde_json::from_slice(key_bytes).map_err(|error| IoWireError::Decode { operation: "io-key", message: error.to_string() })?;
    validate_wire_key(&key)?;
    let wire_sources: Vec<WireComposeSource> = serde_json::from_slice(sources_bytes).map_err(|error| IoWireError::Decode { operation: "compose-source", message: error.to_string() })?;
    if wire_sources.len() > MAX_IO_WIRE_SOURCES {
        return Err(IoWireError::Limit { operation: "compose-source", detail: format!("{} sources exceeds {MAX_IO_WIRE_SOURCES}", wire_sources.len()) });
    }
    let entry = resolve(&key).map_err(|error| error.unavailable.map(IoWireError::Registry).unwrap_or_else(|| IoWireError::Resolve(error.message)))?;
    let mut sources = Vec::with_capacity(wire_sources.len());
    for wire in wire_sources {
        validate_wire_dialect("compose-source", &wire.dialect)?;
        validate_wire_payload("compose-source", &wire.payload)?;
        let dialect = entry.reads.iter().copied().find(|&d| ArtifactDialect::from(d) == wire.dialect).ok_or_else(|| IoWireError::Resolve(format!("composer for {} does not read dialect {}", key.artifact_kind, wire.dialect.to_coordinate())))?;
        sources.push(ErasedComposeSource { dialect, payload: wire.payload });
    }
    match (entry.compose)(&sources) {
        Ok(mut composed) => {
            run_subset_validation(composed.dialect, &composed.payload, &mut composed.diagnostics).map_err(IoWireError::Subset)?;
            encode_wire_json("composed-artifact", &WireComposedArtifact::from(composed))
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

fn format_catalog() -> &'static RwLock<BTreeMap<String, FormatDescriptor>> {
    FORMAT_CATALOG.get_or_init(|| RwLock::new(BTreeMap::new()))
}

/// 📌️ Registers format rows atomically. Identity, extension, and non-empty MIME claims are each
/// globally singular; equal duplicate rows are idempotent and never replace an established owner.
#[must_use]
pub fn register_format_descriptors(descriptors: impl IntoIterator<Item = FormatDescriptor>) -> Result<(), FormatRegistryError> {
    let assembly = store::begin_artifact_assembly().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    register_format_descriptors_in_assembly(&assembly, descriptors)
}

/// 📌️ Publishes preflighted format rows while one artifact assembly owns the shared barrier.
#[must_use]
pub fn register_format_descriptors_in_assembly(_assembly: &store::ArtifactAssemblyTransaction, descriptors: impl IntoIterator<Item = FormatDescriptor>) -> Result<(), FormatRegistryError> {
    let (proposed, proposed_by_kind) = index_format_descriptors(descriptors).map_err(FormatRegistryError::Conflict)?;
    let mut registry = format_catalog().write().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;
    validate_format_descriptors(&registry, &proposed, &proposed_by_kind).map_err(FormatRegistryError::Conflict)?;
    for (key, descriptor) in proposed {
        registry.entry(key).or_insert(descriptor);
    }
    Ok(())
}

/// 🔬️ Verifies format rows against the catalog without mutating their global ownership.
#[must_use]
pub fn preflight_format_descriptors(rows: &[FormatDescriptor]) -> Result<(), FormatRegistryError> {
    let assembly = store::begin_artifact_assembly().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "artifact-assembly" }))?;
    preflight_format_descriptors_in_assembly(&assembly, rows)
}

/// 🔬️ Verifies format rows while one artifact assembly owns the shared publication barrier.
#[must_use]
pub fn preflight_format_descriptors_in_assembly(_assembly: &store::ArtifactAssemblyTransaction, rows: &[FormatDescriptor]) -> Result<(), FormatRegistryError> {
    let (proposed, proposed_by_kind) = index_format_descriptors(rows.iter().cloned()).map_err(FormatRegistryError::Conflict)?;
    let registry = format_catalog().read().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;
    validate_format_descriptors(&registry, &proposed, &proposed_by_kind).map_err(FormatRegistryError::Conflict)
}

fn index_format_descriptors(descriptors: impl IntoIterator<Item = FormatDescriptor>) -> Result<(BTreeMap<String, FormatDescriptor>, BTreeMap<String, FormatDescriptor>), FormatRegistryConflict> {
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
#[must_use]
pub fn format_descriptor(kind_or_short_or_alias: &str) -> Result<Option<FormatDescriptor>, FormatRegistryError> {
    let registry = format_catalog().read().map_err(|_| FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" }))?;
    Ok(registry.get(kind_or_short_or_alias).cloned())
}

/// 🏷️ Normalize any recognized form (kind id, short id, alias) to the canonical `kind_id`.
#[must_use]
pub fn normalize_format_kind(input: &str) -> Result<Option<String>, FormatRegistryError> {
    Ok(format_descriptor(input)?.map(|descriptor| descriptor.kind_id))
}

/// 🗂️ File-picker `accept` filter (comma-joined extensions) for a list of kind/short/alias
/// strings -- the generic successor to `mesh::stdio_accept_filter`.
#[must_use]
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
pub struct ArtifactAssemblyRegistryPlan {
    pub composer_entries: Vec<&'static ComposerEntry>,
    pub subset_validators: Vec<&'static SubsetValidatorEntry>,
    pub format_descriptors: Vec<FormatDescriptor>,
    pub document_codecs: Vec<store::ArtifactCodec>,
    pub dialect_migrations: Vec<store::DialectMigration>,
}

impl ArtifactAssemblyRegistryPlan {
    /// 🌱️ Starts an empty plan; callers append every owned registry row before committing once.
    pub fn new() -> Self {
        Self { composer_entries: Vec::new(), subset_validators: Vec::new(), format_descriptors: Vec::new(), document_codecs: Vec::new(), dialect_migrations: Vec::new() }
    }
}

impl Default for ArtifactAssemblyRegistryPlan {
    fn default() -> Self {
        Self::new()
    }
}

/// 🚫️ An all-registry assembly cannot acquire its locks or pass preflight.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactAssemblyRegistryError {
    Composer(IoRegistryRegistrationError),
    SubsetValidator(SubsetValidatorRegistryError),
    Format(FormatRegistryError),
    Store(store::ArtifactAssemblyStoreRegistryError),
}

impl std::fmt::Display for ArtifactAssemblyRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Composer(error) => error.fmt(formatter),
            Self::SubsetValidator(error) => error.fmt(formatter),
            Self::Format(error) => error.fmt(formatter),
            Self::Store(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ArtifactAssemblyRegistryError {}

/// 📌️ Acquires every affected write lock, preflights every candidate, then commits without any
/// fallible operation after the first registry mutation.
#[must_use]
pub fn commit_artifact_assembly_registry_plan(assembly: &store::ArtifactAssemblyTransaction, plan: ArtifactAssemblyRegistryPlan) -> Result<(), ArtifactAssemblyRegistryError> {
    let mut store_guards = store::acquire_artifact_assembly_store_registry_guards(assembly).map_err(ArtifactAssemblyRegistryError::Store)?;
    let mut composers = io_registry().write().map_err(|_| ArtifactAssemblyRegistryError::Composer(IoRegistryRegistrationError::Unavailable(IoRegistryUnavailable { registry: "io-composer" })))?;
    let mut subset_validators = subset_validator_registry().write().map_err(|_| ArtifactAssemblyRegistryError::SubsetValidator(SubsetValidatorRegistryError::Unavailable(IoRegistryUnavailable { registry: "subset-validator" })))?;
    let mut formats = format_catalog().write().map_err(|_| ArtifactAssemblyRegistryError::Format(FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" })))?;
    let proposed_composers = composer_entries_by_key(plan.composer_entries.iter().copied()).map_err(ArtifactAssemblyRegistryError::Composer)?;
    validate_composer_entries(&composers, &proposed_composers).map_err(ArtifactAssemblyRegistryError::Composer)?;
    validate_subset_validators(&subset_validators, &plan.subset_validators).map_err(ArtifactAssemblyRegistryError::SubsetValidator)?;
    let (proposed_formats, proposed_formats_by_kind) = index_format_descriptors(plan.format_descriptors.iter().cloned()).map_err(|error| ArtifactAssemblyRegistryError::Format(FormatRegistryError::Conflict(error)))?;
    validate_format_descriptors(&formats, &proposed_formats, &proposed_formats_by_kind).map_err(|error| ArtifactAssemblyRegistryError::Format(FormatRegistryError::Conflict(error)))?;
    store::preflight_artifact_assembly_store_registry_guards(&store_guards, &plan.document_codecs, &plan.dialect_migrations).map_err(ArtifactAssemblyRegistryError::Store)?;
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
    Ok(())
}

/// 📋️ Serialize every distinct registered format as a `mimes.csv`-shaped body (header + one row
/// per distinct `kind_id`, sorted for determinism) -- the generic successor to
/// `mesh::stdio_mimes_csv`.
#[must_use]
pub fn formats_csv() -> Result<String, FormatRegistryError> {
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
mod tests {
    //! 🧪️ `io_compose_via`'s own unit test (this file had no prior `#[cfg(test)]` region — this
    //! module has no stdio dependency to borrow a real chain from, so this registers a minimal
    //! synthetic 2-hop chain through the SAME `register_composer_entries`/`io_dispatch` machinery
    //! every real chain (e.g. stdio's png↔deflate↔binary) goes through, proving the mechanism
    //! against the real registry rather than a hand-simulated call graph.
    use super::*;

    const HOP1_FROM: Dialect = Dialect { artifact_kind: "test.io-compose-via.hop1.from", standard: StandardId("1"), subset: SubsetId("*") };
    const HOP1_INTO: Dialect = Dialect { artifact_kind: "test.io-compose-via.hop1.into", standard: StandardId("1"), subset: SubsetId("*") };
    const HOP2_INTO: Dialect = Dialect { artifact_kind: "test.io-compose-via.hop2.into", standard: StandardId("1"), subset: SubsetId("*") };

    fn hop_text(sources: &[ErasedComposeSource]) -> Result<String, ComposeError> {
        match sources {
            [one] => Ok(match &one.payload {
                IoPayload::Text(t) => t.clone(),
                IoPayload::Binary(b) => String::from_utf8_lossy(b).into_owned(),
            }),
            other => Err(ComposeError { message: format!("expected exactly 1 source, got {}", other.len()), diagnostics: Vec::new() }),
        }
    }

    fn compose_hop1(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let text = hop_text(sources)?;
        Ok(ComposedArtifact { dialect: HOP1_INTO, payload: IoPayload::Text(format!("hop1({text})")), diagnostics: Vec::new(), confidence: Confidence::High })
    }

    fn compose_hop2(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let text = hop_text(sources)?;
        Ok(ComposedArtifact { dialect: HOP2_INTO, payload: IoPayload::Text(format!("hop2({text})")), diagnostics: Vec::new(), confidence: Confidence::High })
    }

    static HOP1_READS: [Dialect; 1] = [HOP1_FROM];
    static HOP2_READS: [Dialect; 1] = [HOP1_INTO];

    static ENTRIES: [ComposerEntry; 2] = [ComposerEntry { writes: HOP1_INTO, reads: &HOP1_READS, compose: compose_hop1 }, ComposerEntry { writes: HOP2_INTO, reads: &HOP2_READS, compose: compose_hop2 }];

    const CONFLICT_FROM: Dialect = Dialect { artifact_kind: "test.io-registry-conflict.from", standard: StandardId("1"), subset: SubsetId("*") };
    const CONFLICT_INTO: Dialect = Dialect { artifact_kind: "test.io-registry-conflict.into", standard: StandardId("1"), subset: SubsetId("*") };
    static CONFLICT_READS: [Dialect; 1] = [CONFLICT_FROM];
    static CONFLICT_FIRST: ComposerEntry = ComposerEntry { writes: CONFLICT_INTO, reads: &CONFLICT_READS, compose: compose_hop1 };
    static CONFLICT_SECOND: ComposerEntry = ComposerEntry { writes: CONFLICT_INTO, reads: &CONFLICT_READS, compose: compose_hop2 };

    /// 🌉️🌉️ hub = HOP1_INTO (resolved directly from the seed source), target = HOP2_INTO
    /// (resolved from hub's own composed output alone) — the exact 2-hop shape `io_compose_via`'s
    /// doc comment describes, registered and resolved through the real `IO_REGISTRY`.
    #[test]
    fn io_compose_via_chains_two_registered_hops() {
        register_composer_entries(&ENTRIES).expect("register two-hop test entries");
        let hub_key = IoKey::from_owner_counterpart(HOP1_INTO, HOP1_FROM, IoDirection::Import);
        let target_key = IoKey::from_owner_counterpart(HOP2_INTO, HOP1_INTO, IoDirection::Import);
        let sources = [ErasedComposeSource { dialect: HOP1_FROM, payload: IoPayload::Text("seed".to_string()) }];

        let result = io_compose_via(&hub_key, &target_key, &sources).expect("2-hop compose over real registered entries should succeed");
        assert_eq!(result.dialect, HOP2_INTO);
        match result.payload {
            IoPayload::Text(t) => assert_eq!(t, "hop2(hop1(seed))"),
            IoPayload::Binary(_) => panic!("expected Text payload"),
        }
    }

    /// ⚠️ The hub hop itself failing (no registered entry) must surface as the hub's own
    /// `ComposeError`, never silently attempt the target hop with stale/absent data.
    #[test]
    fn io_compose_via_surfaces_hub_resolve_failure() {
        let unregistered_hub = IoKey::from_owner_counterpart(Dialect { artifact_kind: "test.io-compose-via.unregistered", standard: StandardId("1"), subset: SubsetId("*") }, HOP1_FROM, IoDirection::Import);
        let target_key = IoKey::from_owner_counterpart(HOP2_INTO, HOP1_INTO, IoDirection::Import);
        let sources = [ErasedComposeSource { dialect: HOP1_FROM, payload: IoPayload::Text("seed".to_string()) }];
        let err = match io_compose_via(&unregistered_hub, &target_key, &sources) {
            Err(err) => err,
            Ok(_) => panic!("unregistered hub key must fail hop 1"),
        };
        assert!(err.message.contains("no composer registered"), "{}", err.message);
    }

    #[test]
    fn io_registry_rejects_a_conflicting_key_without_replacing_the_first_entry() {
        register_composer_entries(std::slice::from_ref(&CONFLICT_FIRST)).expect("first owner registers");
        assert!(matches!(preflight_composer_entry_refs(&[&CONFLICT_SECOND]), Err(IoRegistryRegistrationError::Conflict(_))), "preflight must expose the same conflict before any later assembly mutation");
        let conflict = match register_composer_entries(std::slice::from_ref(&CONFLICT_SECOND)).expect_err("a second owner for the same IO key must fail") {
            IoRegistryRegistrationError::Conflict(conflict) => conflict,
            IoRegistryRegistrationError::Unavailable(error) => panic!("registry unavailable: {error:?}"),
        };
        assert_eq!(conflict.key, IoKey::from_owner_counterpart(CONFLICT_FROM, CONFLICT_INTO, IoDirection::Export));
        let resolved = resolve(&conflict.key).expect("first owner remains resolvable");
        assert!(std::ptr::eq(resolved, &CONFLICT_FIRST));
    }

    fn format_descriptor_fixture(kind_id: &str, short_id: &str, mimes: &[&str], extensions: &[&str]) -> FormatDescriptor {
        FormatDescriptor {
            kind_id: kind_id.to_string(),
            short_id: short_id.to_string(),
            aliases: Vec::new(),
            mimes: mimes.iter().map(|mime| (*mime).to_string()).collect(),
            extensions: extensions.iter().map(|extension| (*extension).to_string()).collect(),
            name: short_id.to_string(),
            full_name: kind_id.to_string(),
            neutral: true,
            dir_name: short_id.to_string(),
            is_binary: false,
        }
    }

    #[test]
    fn format_registry_allows_an_unregistered_mime_and_rejects_duplicate_claims() {
        let txt = format_descriptor_fixture("test.format.txt", "txt", &["text/plain"], &[".txt"]);
        let epw = format_descriptor_fixture("test.format.epw", "epw", &[], &[".epw"]);
        preflight_format_descriptors(&[txt.clone(), epw.clone()]).expect("preflight accepts unclaimed distinct format metadata without mutation");
        assert!(format_descriptor("test.format.epw").expect("catalog availability").is_none(), "preflight must not publish a descriptor");
        register_format_descriptors(vec![txt.clone(), epw.clone()]).expect("txt MIME and EPW's absent MIME are unambiguous");
        assert!(format_mimes(&epw).next().is_none());
        assert!(format_mimes(&format_descriptor("test.format.epw").expect("catalog availability").expect("EPW descriptor")).next().is_none());

        let duplicate_step = format_descriptor_fixture("test.format.step-duplicate", "step-duplicate", &["application/step", "APPLICATION/STEP"], &[".step", ".stp", ".STEP"]);
        assert!(
            matches!(register_format_descriptors(vec![duplicate_step]), Err(FormatRegistryError::Conflict(FormatRegistryConflict::Invalid { .. }))),
            "claims that normalize to the same MIME or extension must reject instead of being silently deduplicated"
        );
        let step = format_descriptor_fixture("test.format.step", "step", &["application/step"], &[".step", ".stp"]);
        register_format_descriptors(vec![step]).expect("plural representation claims register when each identity is distinct");
        let step = format_descriptor("test.format.step").expect("catalog availability").expect("STEP descriptor");
        assert_eq!(step.mimes, ["application/step"]);
        assert_eq!(step.extensions, [".step", ".stp"]);
        assert_eq!(format_accept_filter(&["test.format.step"]).expect("catalog availability"), ".step,.stp");
        assert!(matches!(format_accept_filter(&["test.format.unknown"]), Err(FormatRegistryError::Unknown { input }) if input == "test.format.unknown"));
        assert!(formats_csv().expect("catalog availability").contains("application/step,.step"));

        let first = format_descriptor_fixture("test.format.mime-first", "mime-first", &["application/x-wave0-conflict"], &[".first"]);
        let second = format_descriptor_fixture("test.format.mime-second", "mime-second", &["application/x-wave0-conflict"], &[".second"]);
        register_format_descriptors(vec![first]).expect("first MIME owner registers");
        assert!(matches!(register_format_descriptors(vec![second]), Err(FormatRegistryError::Conflict(FormatRegistryConflict::Mime { mime, .. })) if mime == "application/x-wave0-conflict"));
    }

    #[test]
    fn codec_budget_enforces_limits_and_shared_cancellation() {
        let cancellation = CancellationToken::new();
        let policy = DecodePolicy { representation: CodecRepresentation::Lossless, limits: CodecLimits { max_read_bytes: 4, max_written_bytes: 4, max_work_units: 2, max_allocations: 1, max_recursion_depth: 1 }, cancellation: cancellation.clone() };
        let mut context = DecodeContext::new(policy);
        assert_eq!(context.policy.representation, CodecRepresentation::Lossless);
        context.budget.charge_read(4).expect("limit edge is allowed");
        assert!(context.budget.charge_work(3).is_err(), "work overrun must fail");
        context.budget.charge_allocation(1).expect("allocation limit edge is allowed");
        cancellation.cancel();
        assert!(context.budget.charge_read(1).is_err(), "shared cancellation must stop later work");
    }

    struct TestPayload {
        bytes: Vec<u8>,
        cursor: usize,
        span: SourceSpan,
    }

    impl TestPayload {
        fn new(bytes: &[u8], resource: &str) -> Self {
            Self { bytes: bytes.to_vec(), cursor: 0, span: SourceSpan { resource: resource.to_string(), byte_start: 0, byte_end: bytes.len() as u64, line: Some(1), column: Some(1) } }
        }
    }

    impl RandomAccessPayload for TestPayload {
        fn len(&self) -> CodecResult<u64> {
            Ok(CodecOutput { value: self.bytes.len() as u64, diagnostics: Vec::new() })
        }

        fn read_at(&self, offset: u64, output: &mut [u8]) -> CodecResult<usize> {
            let start = usize::try_from(offset).map_err(|_| CodecFailure::error("test.offset", "offset does not fit usize"))?;
            let available = match self.bytes.get(start..) {
                Some(available) => available,
                None => &[],
            };
            let count = available.len().min(output.len());
            output[..count].copy_from_slice(&available[..count]);
            Ok(CodecOutput { value: count, diagnostics: Vec::new() })
        }
    }

    impl PayloadSource for TestPayload {
        fn span(&self) -> SourceSpan {
            self.span.clone()
        }

        fn read_chunk(&mut self, output: &mut [u8]) -> CodecResult<usize> {
            let available = &self.bytes[self.cursor..];
            let count = available.len().min(output.len());
            output[..count].copy_from_slice(&available[..count]);
            self.cursor += count;
            Ok(CodecOutput { value: count, diagnostics: Vec::new() })
        }

        fn random_access(&self) -> Option<&dyn RandomAccessPayload> {
            Some(self)
        }
    }

    struct TestSink;

    impl PayloadSink for TestSink {
        fn write_chunk(&mut self, _input: &[u8]) -> CodecResult<()> {
            Ok(CodecOutput { value: (), diagnostics: Vec::new() })
        }
    }

    struct TestResolver;

    impl ResourceResolver for TestResolver {
        fn resolve_decode(&self, _request: &ResourceRequest) -> CodecResult<Box<dyn PayloadSource>> {
            Ok(CodecOutput { value: Box::new(TestPayload::new(b"resolved", "resolver://decode")), diagnostics: Vec::new() })
        }

        fn resolve_encode(&self, _request: &ResourceRequest) -> CodecResult<Box<dyn PayloadSink>> {
            Ok(CodecOutput { value: Box::new(TestSink), diagnostics: Vec::new() })
        }
    }

    #[test]
    fn codec_context_bounds_streaming_random_access_recursion_and_resolved_resources() {
        let limits = CodecLimits { max_read_bytes: 4, max_written_bytes: 4, max_work_units: 16, max_allocations: 4, max_recursion_depth: 1 };
        let resolver = std::sync::Arc::new(TestResolver);
        let mut decode = DecodeContext::with_resolver(DecodePolicy { representation: CodecRepresentation::Lossless, limits: limits.clone(), cancellation: CancellationToken::new() }, resolver.clone());
        let request = ResourceRequest { locator: "resolver://document".to_string(), expected_media_type: None };
        let mut source = decode.resolve(&request).expect("decode resource resolves").value;
        let mut output = [0u8; 3];
        assert_eq!(source.read_chunk(&mut output).expect("bounded stream read").value, 3);
        let mut random = source.random_access().expect("random access is available");
        assert_eq!(random.read_at(0, &mut [0u8; 2]).expect("remaining bounded random read").value, 1);
        assert!(random.read_at(0, &mut [0u8; 1]).is_err(), "random access cannot bypass the shared read budget");
        drop(random);
        drop(source);
        decode.budget.enter_recursion().expect("first recursion frame");
        assert!(decode.budget.enter_recursion().is_err(), "recursion limit is finite");
        decode.budget.leave_recursion().expect("leave recursion frame");

        let mut encode = EncodeContext::with_resolver(EncodePolicy { representation: CodecRepresentation::Canonical, limits, cancellation: CancellationToken::new() }, resolver);
        let mut sink = encode.resolve(&request).expect("encode resource resolves").value;
        sink.write_chunk(b"four").expect("bounded write");
        assert!(sink.write_chunk(b"x").is_err(), "writes cannot bypass the output budget");
    }

    #[test]
    fn resolved_resources_cannot_outlive_their_cancellation_budget() {
        let cancellation = CancellationToken::new();
        let policy = DecodePolicy { representation: CodecRepresentation::Lossless, limits: CodecLimits::default(), cancellation: cancellation.clone() };
        let resolver = std::sync::Arc::new(TestResolver);
        let request = ResourceRequest { locator: "resolver://cancelled".to_string(), expected_media_type: None };
        let mut context = DecodeContext::with_resolver(policy, resolver);
        let mut source = context.resolve(&request).expect("resolve while active").value;
        cancellation.cancel();
        assert!(source.read_chunk(&mut [0u8; 1]).is_err(), "the resolved source must be cancellable after resolution");
    }

    #[test]
    fn wire_rejects_oversized_and_unbounded_dialect_inputs_before_interning() {
        assert!(matches!(wire_decode_composed_artifact(&vec![b' '; MAX_IO_WIRE_BYTES + 1]), Err(IoWireError::Limit { operation: "composed-artifact", .. })));
        let wire = WireComposedArtifact {
            dialect: ArtifactDialect { artifact_kind: "x".repeat(MAX_IO_WIRE_DIALECT_COMPONENT_BYTES + 1), standard: "1".into(), subset: "*".into() },
            payload: IoPayload::Text("payload".into()),
            diagnostics: Vec::new(),
            confidence: Confidence::High,
        };
        let bytes = serde_json::to_vec(&wire).expect("wire fixture");
        assert!(matches!(wire_decode_composed_artifact(&bytes), Err(IoWireError::Limit { operation: "composed-artifact", .. })));
    }

    #[test]
    fn codec_result_requires_valid_owned_spans_and_deterministic_opaque_order() {
        let invalid = SourceSpan { resource: String::new(), byte_start: 3, byte_end: 2, line: Some(1), column: None };
        assert!(invalid.validate().is_err());
        let result = ArtifactCodecResult {
            semantic: (),
            anchors: vec![AnchoredSyntax { anchor: "root".to_string(), span: SourceSpan { resource: "memory://source".to_string(), byte_start: 0, byte_end: 2, line: Some(1), column: Some(1) }, bytes: b"ok".to_vec() }],
            opaque_extensions: vec![
                OpaqueExtension {
                    kind: "z".to_string(),
                    source: AnchoredSyntax { anchor: "z".to_string(), span: SourceSpan { resource: "memory://source".to_string(), byte_start: 2, byte_end: 3, line: Some(1), column: Some(3) }, bytes: b"z".to_vec() },
                },
                OpaqueExtension {
                    kind: "a".to_string(),
                    source: AnchoredSyntax { anchor: "a".to_string(), span: SourceSpan { resource: "memory://source".to_string(), byte_start: 3, byte_end: 4, line: Some(1), column: Some(4) }, bytes: b"a".to_vec() },
                },
            ],
        };
        result.validate_lossless().expect("owned anchored result is lossless-valid");
        assert!(result.validate_representation(CodecRepresentation::Canonical).is_err(), "a canonical result cannot merely declare a canonical policy while retaining insertion order");
        let mut context = EncodeContext::new(EncodePolicy::default());
        let finalized = context.finalize_result(result).expect("host finalization canonicalizes an owned result").value;
        assert_eq!(finalized.canonical_opaque_extensions().iter().map(|extension| extension.kind.as_str()).collect::<Vec<_>>(), vec!["a", "z"]);
        finalized.validate_representation(CodecRepresentation::Canonical).expect("finalized canonical result is executable-policy-valid");
    }

    /// ✅️ Accept table for `is_canonical_artifact_kind`/`ArtifactKindId::parse`: exactly three
    /// dot-separated ASCII segments, first literally `s`, the rest lowercase-kebab.
    #[test]
    fn artifact_kind_id_accepts_canonical_grammar() {
        for kind in ["s.stdio.stl", "s.stdio.semio"] {
            assert!(is_canonical_artifact_kind(kind), "{kind:?} should be canonical");
            ArtifactKindId::parse(kind).unwrap_or_else(|e| panic!("{kind:?} should parse: {e}"));
        }
    }

    /// ⚠️ Reject table covering: missing `s.` prefix, non-canonical vocabulary, uppercase, emoji,
    /// too few/too many segments, empty segment, leading hyphen.
    #[test]
    fn artifact_kind_id_rejects_non_canonical_grammar() {
        for kind in ["stdio.stl", "3d.cad", "data.🏛️program", "s.Stdio.stl", "s.stdio", "s.stdio.stl.extra", "s..stl", "s.stdio.-stl"] {
            assert!(!is_canonical_artifact_kind(kind), "{kind:?} should be rejected");
            assert!(ArtifactKindId::parse(kind).is_err(), "{kind:?} should fail to parse");
        }
    }

    /// 🔁️ `ArtifactRef::to_uri`/`parse_uri` round-trip, including an artifact id containing dots
    /// and dashes (must not be mistaken for dialect-coordinate delimiters since only the FIRST
    /// `!` is significant).
    #[test]
    fn artifact_ref_uri_round_trips() {
        let cases = [
            ArtifactRef { artifact_id: "abc123".to_string(), dialect: ArtifactDialect { artifact_kind: "s.stdio.stl".to_string(), standard: "1".to_string(), subset: "*".to_string() } },
            ArtifactRef { artifact_id: "doc.v2-final.draft".to_string(), dialect: ArtifactDialect { artifact_kind: "s.norm.en-1994-1".to_string(), standard: "2024".to_string(), subset: "cc6".to_string() } },
        ];
        for artifact_ref in cases {
            let uri = artifact_ref.to_uri();
            let parsed = ArtifactRef::parse_uri(&uri).unwrap_or_else(|e| panic!("{uri:?} should round-trip: {e}"));
            assert_eq!(parsed, artifact_ref);
        }
    }

    /// 🔁️ Exact expected shape of `to_uri`, pinned so the format doesn't silently drift.
    #[test]
    fn artifact_ref_to_uri_matches_expected_shape() {
        let artifact_ref = ArtifactRef { artifact_id: "abc123".to_string(), dialect: ArtifactDialect { artifact_kind: "s.stdio.gif".to_string(), standard: "87a".to_string(), subset: "*".to_string() } };
        assert_eq!(artifact_ref.to_uri(), "abc123!s.stdio.gif@87a/*");
    }

    /// ⚠️ `parse_uri` rejects a missing `!` and an empty artifact id, mirroring
    /// `parse_coordinate`'s own empty-component rejection.
    #[test]
    fn artifact_ref_parse_uri_rejects_malformed_input() {
        assert!(ArtifactRef::parse_uri("s.stdio.gif@87a/*").is_err(), "missing '!' should fail");
        assert!(ArtifactRef::parse_uri("!s.stdio.gif@87a/*").is_err(), "empty artifact id should fail");
    }
}
//#endregion 🔖️Tests
// #endregion io
