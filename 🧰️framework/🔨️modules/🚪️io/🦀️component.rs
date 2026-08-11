// #region io
//! 🚪️ Dialect vocabulary and typed artifact-to-artifact IO dispatch registry.
//! Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase 2 (standards/subsets). Lives beside
//! `🔺️mesh` (not `os`) so plugins and the OS product share one definition without an
//! inverted dependency — same reasoning as `mesh::MediaFormat`.

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum IoDirection {
    Import,
    Export,
}

/// 🗝️ Owned mirror of two dialects + direction — the registry key. Owned (not `&'static`) so it
/// can be built from runtime UI input (format kind strings) as well as static composer entries.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
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

static IO_REGISTRY: std::sync::OnceLock<RwLock<HashMap<IoKey, &'static ComposerEntry>>> = std::sync::OnceLock::new();

fn io_registry() -> &'static RwLock<HashMap<IoKey, &'static ComposerEntry>> {
    IO_REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

/// 📌️ Register one artifact's composer entries. Called once per artifact-level composer's
/// `register()`, itself called from the owning plugin's `🔧️setup` at init. `entries` must be
/// `'static` (composer entries are plain fn pointers + const slices, always leaked/static in
/// practice since they come from `ComposerEntry::of::<C>()` over a unit-struct composer type).
pub fn register_composer_entries(entries: &'static [ComposerEntry]) {
    let mut reg = io_registry().write().expect("io registry poisoned");
    for entry in entries {
        // One composer that writes W by reading R is, symmetrically, both "W can IMPORT from R"
        // and "R can EXPORT into W" -- both interpretations resolve to this same entry, since
        // `compose` already knows how to turn an R-shaped source into a W-shaped snapshot.
        for &source in entry.reads {
            reg.insert(IoKey::from_owner_counterpart(entry.writes, source, IoDirection::Import), entry);
            reg.insert(IoKey::from_owner_counterpart(source, entry.writes, IoDirection::Export), entry);
        }
    }
}

#[derive(Clone, Debug)]
pub struct IoResolveError {
    pub message: String,
    pub candidates: Vec<IoKey>,
}

/// 🔎️ Look up the composer entry for one exact (artifact/standard/subset, direction,
/// format/standard/subset) coordinate. No silent defaulting — callers with a partially-specified
/// query (unknown standard/subset) must enumerate `dialects_for` first and choose explicitly.
pub fn resolve(key: &IoKey) -> Result<&'static ComposerEntry, IoResolveError> {
    let reg = io_registry().read().expect("io registry poisoned");
    reg.get(key).copied().ok_or_else(|| IoResolveError {
        message: format!(
            "no composer registered for {}/{}/{} {:?} {}/{}/{}",
            key.artifact_kind, key.standard, key.subset, key.direction, key.format_kind, key.format_standard, key.format_subset
        ),
        candidates: reg.keys().filter(|k| k.artifact_kind == key.artifact_kind).cloned().collect(),
    })
}

/// 📚️ Every dialect one artifact can move data through in a given direction.
pub fn dialects_for(artifact_kind: &str, direction: IoDirection) -> Vec<Dialect> {
    let reg = io_registry().read().expect("io registry poisoned");
    reg.iter()
        .filter(|(k, _)| k.artifact_kind == artifact_kind && k.direction == direction)
        .map(|(_, entry)| entry.writes)
        .collect()
}

/// 🗝️ Every registered `IoKey` for one artifact_kind + direction, WITH the owner's real
/// standard/subset (not a hardcoded default) -- callers that used to build a key by hand and
/// guess `standard: "1", subset: "*"` should enumerate this instead and pick explicitly, the same
/// "no silent defaulting" policy `resolve` already documents.
pub fn io_keys_for(artifact_kind: &str, direction: IoDirection) -> Vec<IoKey> {
    let reg = io_registry().read().expect("io registry poisoned");
    reg.keys().filter(|k| k.artifact_kind == artifact_kind && k.direction == direction).cloned().collect()
}

/// 📇️ Every registered composer entry, erased to owned dialects -- the shape the WIT
/// `list-artifact-dialects` guest export mirrors verbatim (one row per distinct `writes` entry
/// registered locally, each carrying the full `reads` list).
pub fn list_composer_entries() -> Vec<(ArtifactDialect, Vec<ArtifactDialect>)> {
    let reg = io_registry().read().expect("io registry poisoned");
    let mut seen: HashMap<Dialect, &'static ComposerEntry> = HashMap::new();
    for entry in reg.values() {
        seen.insert(entry.writes, entry);
    }
    seen.into_values().map(|entry| (ArtifactDialect::from(entry.writes), entry.reads.iter().map(|&d| ArtifactDialect::from(d)).collect())).collect()
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
type IoFallback = dyn Fn(&IoKey, &[ErasedComposeSource]) -> Option<Result<ComposedArtifact, ComposeError>> + Send + Sync;

static IO_FALLBACK: std::sync::OnceLock<Box<IoFallback>> = std::sync::OnceLock::new();

/// 🔌️ Install the fallback dispatcher. Call exactly once, before any `io_dispatch` call that
/// should reach it (host boot / guest `ensure_plugin_initialized`). A second call is a no-op
/// (logged, not panicking) -- boot ordering across concurrent plugin loads shouldn't be able to
/// crash the process over a registration race.
pub fn set_io_fallback_dispatcher<F>(hook: F)
where
    F: Fn(&IoKey, &[ErasedComposeSource]) -> Option<Result<ComposedArtifact, ComposeError>> + Send + Sync + 'static,
{
    if IO_FALLBACK.set(Box::new(hook)).is_err() {
        eprintln!("[DEBUG] io::set_io_fallback_dispatcher called more than once; keeping the first installed hook");
    }
}

/// 🎹️ Resolve `key` locally; on a local miss, ask the installed fallback (if any). Returns the
/// SAME `IoResolveError`-shaped message as a local-only `resolve` when nothing (local or
/// fallback) has the key, so existing error-message-matching callers don't need to change.
pub fn io_dispatch(key: &IoKey, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    match resolve(key) {
        Ok(entry) => match (entry.compose)(sources) {
            Ok(mut composed) => {
                run_subset_validation(composed.dialect, &composed.payload, &mut composed.diagnostics);
                Ok(composed)
            }
            Err(e) => Err(e),
        },
        Err(local_err) => match IO_FALLBACK.get().and_then(|hook| hook(key, sources)) {
            Some(result) => result,
            None => Err(ComposeError { message: local_err.message, diagnostics: Vec::new() }),
        },
    }
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

static SUBSET_VALIDATOR_REGISTRY: std::sync::OnceLock<RwLock<HashMap<Dialect, &'static SubsetValidatorEntry>>> = std::sync::OnceLock::new();

fn subset_validator_registry() -> &'static RwLock<HashMap<Dialect, &'static SubsetValidatorEntry>> {
    SUBSET_VALIDATOR_REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

/// 📌️ Register one subset's validator. Called once from the owning artifact's subset-level facet
/// init (mirrors `register_composer_entries`'s own call convention). A second registration for
/// the same `dialect` overwrites the first (last-registered-wins, logged) rather than panicking --
/// boot ordering across concurrent plugin loads shouldn't be able to crash the process.
pub fn register_subset_validator(entry: &'static SubsetValidatorEntry) {
    let mut reg = subset_validator_registry().write().expect("subset validator registry poisoned");
    if reg.insert(entry.dialect, entry).is_some() {
        eprintln!("[DEBUG] io::register_subset_validator called twice for {:?}; keeping the latest registration", entry.dialect);
    }
}

/// 🛡️ The generic validate-on-build hook (D5): if `dialect.subset` is anything other than
/// `SubsetId::ANY` and a validator is registered for that EXACT dialect, run it and fold its
/// `Diagnostic`s onto `diagnostics`. Advisory only -- a validator that itself returns diagnostics
/// never fails composition; diagnostics are soft signals here exactly like `Composition<T>`/
/// `Analysis<T>` already carry elsewhere in this file (a subset composer wanting a HARD gate
/// enforces that itself, inside its own `compose`, before ever returning `Ok` -- see the PDF/A
/// pilot). Called from every generic compose-dispatch path in this module (`io_dispatch`,
/// `wire_artifact_compose`) so every future subset gets this for free the moment it registers a
/// validator -- no dispatch call site needs to change again. Never panics: a poisoned registry
/// lock degrades to a no-op.
///
/// `ANY` always short-circuits (nothing to validate against the unconstrained base subset). A
/// real (non-`ANY`) dialect with NO registered validator is, since ticket
/// 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES, treated as a defect rather than silence
/// -- every real subset is expected to register one (`policyStandardSubsetVocabularyBreaches`
/// checks this statically) -- and emits one `io.subset.validator-missing` Warning naming the
/// coordinate. Still never hard-fails here: the receiving side of a cross-plugin wire compose may
/// legitimately not host the owning plugin's validator locally, and a missing validator is exactly
/// the kind of thing a diagnostic (not a dispatch error) exists to surface.
fn run_subset_validation(dialect: Dialect, payload: &IoPayload, diagnostics: &mut Vec<Diagnostic>) {
    if dialect.subset == SubsetId::ANY {
        return;
    }
    let Ok(reg) = subset_validator_registry().read() else { return };
    match reg.get(&dialect) {
        Some(entry) => diagnostics.extend((entry.validate)(payload)),
        None => diagnostics.push(Diagnostic {
            code: FaultCode::new("io.subset.validator-missing"),
            severity: Severity::Warning,
            span: TextSpan::at(1, 1),
            message: format!("no SubsetValidator registered for {}@{}/{}", dialect.artifact_kind, dialect.standard.0, dialect.subset.0),
            expected: None,
            scope: FaultScope::default(),
        }),
    }
}
//#endregion 🔖️SubsetValidator

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
fn intern_dialect(dialect: &ArtifactDialect) -> Dialect {
    static INTERNED: std::sync::OnceLock<RwLock<HashMap<ArtifactDialect, Dialect>>> = std::sync::OnceLock::new();
    let table = INTERNED.get_or_init(|| RwLock::new(HashMap::new()));
    if let Some(found) = table.read().expect("dialect intern table poisoned").get(dialect) {
        return *found;
    }
    let mut write = table.write().expect("dialect intern table poisoned");
    if let Some(found) = write.get(dialect) {
        return *found;
    }
    let leaked = Dialect {
        artifact_kind: Box::leak(dialect.artifact_kind.clone().into_boxed_str()),
        standard: StandardId(Box::leak(dialect.standard.clone().into_boxed_str())),
        subset: SubsetId(Box::leak(dialect.subset.clone().into_boxed_str())),
    };
    write.insert(dialect.clone(), leaked);
    leaked
}

/// 🌉️ Decodes a wire `WireComposedArtifact` (JSON bytes) into a native `ComposedArtifact`,
/// interning its dialect via `intern_dialect`. The receiving-side half of `wire_artifact_compose`
/// — used by a guest's `io_dispatch` fallback hook once `host.io-compose` returns.
pub fn wire_decode_composed_artifact(bytes: &[u8]) -> Result<ComposedArtifact, String> {
    let wire: WireComposedArtifact = serde_json::from_slice(bytes).map_err(|e| format!("bad composed-artifact wire bytes: {e}"))?;
    Ok(ComposedArtifact { dialect: intern_dialect(&wire.dialect), payload: wire.payload, diagnostics: wire.diagnostics, confidence: wire.confidence })
}

/// 🌉️ Encodes this process's own composer roster (`list_composer_entries`) as JSON bytes — the
/// body of the WIT `list-artifact-dialects` guest export (see D3, ticket 26/08/10/
/// ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION). JSON (not `pack_rt::encode_wire_value`)
/// is a deliberate simplification for this first cut: the WIT signature is an opaque `list<u8>`
/// either way, so swapping the wire encoding later needs no ABI change, and this module has no
/// existing dependency on `store`/`dsl`'s pack machinery worth introducing just for this.
pub fn wire_list_composer_entries() -> Vec<u8> {
    serde_json::to_vec(&list_composer_entries()).unwrap_or_default()
}

/// 🌉️ Decodes a wire `(IoKey, Vec<WireComposeSource>)` request and composes it against THIS
/// process's own local registry only — never the fallback hook. A guest receiving an incoming
/// `artifact-compose` call is, by construction, the plugin the host router already decided owns
/// the key; falling through again here would be a pointless extra hop at best and a reentrancy
/// risk at worst (see the host router's own one-hop guard). The body of the WIT
/// `artifact-compose` guest export. Errors are flattened to a message string, matching how every
/// other fallible call on this ABI surfaces errors (a `Fault`, not structured data) — see
/// `migrate-artifact`'s `plugin-error` for the existing precedent.
pub fn wire_artifact_compose(key_bytes: &[u8], sources_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let key: IoKey = serde_json::from_slice(key_bytes).map_err(|e| format!("bad io key wire bytes: {e}"))?;
    let wire_sources: Vec<WireComposeSource> = serde_json::from_slice(sources_bytes).map_err(|e| format!("bad compose source wire bytes: {e}"))?;
    let entry = resolve(&key).map_err(|e| e.message)?;
    let mut sources = Vec::with_capacity(wire_sources.len());
    for wire in wire_sources {
        let dialect = entry
            .reads
            .iter()
            .copied()
            .find(|&d| ArtifactDialect::from(d) == wire.dialect)
            .ok_or_else(|| format!("composer for {} does not read dialect {}", key.artifact_kind, wire.dialect.to_coordinate()))?;
        sources.push(ErasedComposeSource { dialect, payload: wire.payload });
    }
    match (entry.compose)(&sources) {
        Ok(mut composed) => {
            run_subset_validation(composed.dialect, &composed.payload, &mut composed.diagnostics);
            serde_json::to_vec(&WireComposedArtifact::from(composed)).map_err(|e| format!("compose result encode: {e}"))
        }
        Err(error) => Err(error.message),
    }
}
//#endregion 🔖️Wire

//#region 🔖️FormatCatalog
/// 🗄️ One string-keyed format's metadata — kind id, mime, extension, folder slug. Generic
/// successor to the closed, `🔺️mesh`-local `StdioFormatEntry`/`STDIO_FORMAT_CATALOG` (ticket
/// 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT wave 2): where `mesh`'s catalog is a single
/// hardcoded `const` slice only `stdio` can ever contribute to, this registry is additive and
/// string-keyed like `IO_REGISTRY` above it, so ANY plugin that owns formats (not just `stdio`)
/// can call `register_format_descriptors` from its own init. `mesh`'s catalog itself is untouched
/// here -- evicting it onto this registry is a LATER wave's job, once every producer/consumer of
/// `StdioFormatEntry` has migrated to `FormatDescriptor`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatDescriptor {
    pub kind_id: String,
    pub short_id: String,
    pub aliases: Vec<String>,
    pub mime: String,
    pub extension: String,
    pub name: String,
    pub full_name: String,
    pub neutral: bool,
    pub dir_name: String,
    pub is_binary: bool,
}

static FORMAT_CATALOG: std::sync::OnceLock<RwLock<HashMap<String, FormatDescriptor>>> = std::sync::OnceLock::new();

fn format_catalog() -> &'static RwLock<HashMap<String, FormatDescriptor>> {
    FORMAT_CATALOG.get_or_init(|| RwLock::new(HashMap::new()))
}

/// 📌️ Register format descriptor rows, keyed by `kind_id`, `short_id`, and every alias so
/// `format_descriptor` resolves any of the three forms in O(1). Callable multiple times -- once
/// per plugin that owns formats -- mirroring `register_composer_entries`'s own additive
/// convention (never assume a single caller). A later registration overwriting a key already
/// present is logged, not panicking, the same "boot ordering across concurrent plugin loads
/// shouldn't crash the process" policy `register_subset_validator` documents above.
pub fn register_format_descriptors(rows: Vec<FormatDescriptor>) {
    let mut reg = format_catalog().write().expect("format catalog poisoned");
    for row in rows {
        for key in std::iter::once(row.kind_id.clone()).chain(std::iter::once(row.short_id.clone())).chain(row.aliases.iter().cloned()) {
            if reg.insert(key.clone(), row.clone()).is_some() {
                eprintln!("[DEBUG] io::register_format_descriptors overwrote an existing entry for key {key:?}");
            }
        }
    }
}

/// 🔎️ Resolve a format by its `kind_id`, `short_id`, or any registered alias.
pub fn format_descriptor(kind_or_short_or_alias: &str) -> Option<FormatDescriptor> {
    format_catalog().read().expect("format catalog poisoned").get(kind_or_short_or_alias).cloned()
}

/// 🏷️ Normalize any recognized form (kind id, short id, alias) to the canonical `kind_id`.
pub fn normalize_format_kind(input: &str) -> Option<String> {
    format_descriptor(input).map(|d| d.kind_id)
}

/// 🗂️ File-picker `accept` filter (comma-joined extensions) for a list of kind/short/alias
/// strings -- the generic successor to `mesh::stdio_accept_filter`.
pub fn format_accept_filter(kind_ids: &[&str]) -> String {
    kind_ids.iter().filter_map(|k| format_descriptor(k)).map(|d| d.extension).collect::<Vec<_>>().join(",")
}

/// 📋️ Serialize every distinct registered format as a `mimes.csv`-shaped body (header + one row
/// per distinct `kind_id`, sorted for determinism) -- the generic successor to
/// `mesh::stdio_mimes_csv`.
pub fn formats_csv() -> String {
    let reg = format_catalog().read().expect("format catalog poisoned");
    let mut seen: HashMap<&str, &FormatDescriptor> = HashMap::new();
    for row in reg.values() {
        seen.insert(row.kind_id.as_str(), row);
    }
    let mut rows: Vec<&FormatDescriptor> = seen.into_values().collect();
    rows.sort_by(|a, b| a.kind_id.cmp(&b.kind_id));
    let mut out = String::from("MIME,Extension,Name,FullName,Neutral,Dir,Kind\n");
    for row in rows {
        out.push_str(&row.mime);
        out.push(',');
        out.push_str(&row.extension);
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
    out
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

    static ENTRIES: [ComposerEntry; 2] = [
        ComposerEntry { writes: HOP1_INTO, reads: &HOP1_READS, compose: compose_hop1 },
        ComposerEntry { writes: HOP2_INTO, reads: &HOP2_READS, compose: compose_hop2 },
    ];

    /// 🌉️🌉️ hub = HOP1_INTO (resolved directly from the seed source), target = HOP2_INTO
    /// (resolved from hub's own composed output alone) — the exact 2-hop shape `io_compose_via`'s
    /// doc comment describes, registered and resolved through the real `IO_REGISTRY`.
    #[test]
    fn io_compose_via_chains_two_registered_hops() {
        register_composer_entries(&ENTRIES);
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
        let unregistered_hub = IoKey::from_owner_counterpart(
            Dialect { artifact_kind: "test.io-compose-via.unregistered", standard: StandardId("1"), subset: SubsetId("*") },
            HOP1_FROM,
            IoDirection::Import,
        );
        let target_key = IoKey::from_owner_counterpart(HOP2_INTO, HOP1_INTO, IoDirection::Import);
        let sources = [ErasedComposeSource { dialect: HOP1_FROM, payload: IoPayload::Text("seed".to_string()) }];
        let err = match io_compose_via(&unregistered_hub, &target_key, &sources) {
            Err(err) => err,
            Ok(_) => panic!("unregistered hub key must fail hop 1"),
        };
        assert!(err.message.contains("no composer registered"), "{}", err.message);
    }
}
//#endregion 🔖️Tests
// #endregion io
