// #region io
//! 🚪️ Dialect vocabulary and typed artifact-to-artifact IO dispatch registry.
//! Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase 2 (standards/subsets). Lives beside
//! `🔺️mesh` (not `os`) so plugins and the OS product share one definition without an
//! inverted dependency — same reasoning as `mesh::MediaFormat`.

use dsl::Diagnostic;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

//#region 🔖️Dialect
/// 🏅️ A standard slug — the text after `🔖️` in `🏅️standards/🔖️<standard>/` (e.g. "2.0", "ap214", "1").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StandardId(pub &'static str);

/// 🪆️ A subset id — the text materialized as `🪆️subsets/✳️<dir>/`. `ANY` is the only subset today.
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
        Ok(entry) => (entry.compose)(sources),
        Err(local_err) => match IO_FALLBACK.get().and_then(|hook| hook(key, sources)) {
            Some(result) => result,
            None => Err(ComposeError { message: local_err.message, diagnostics: Vec::new() }),
        },
    }
}
//#endregion 🔖️Dispatch

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
        Ok(composed) => serde_json::to_vec(&WireComposedArtifact::from(composed)).map_err(|e| format!("compose result encode: {e}")),
        Err(error) => Err(error.message),
    }
}
//#endregion 🔖️Wire
//#endregion 🔖️ErasedRegistry
// #endregion io
