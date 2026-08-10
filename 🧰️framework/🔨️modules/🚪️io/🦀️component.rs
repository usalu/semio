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
//#endregion 🔖️Dialect

//#region 🔖️ComposeTypes
/// 📥 One typed compose source: a foreign or native dialect plus its payload.
#[derive(Clone, Debug)]
pub enum AnalyzeSource<'a> {
    Text(&'a str),
    Binary(&'a [u8]),
}

/// 🎚 Soft confidence for partial analysis/composition success.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(Clone, Debug)]
pub struct ComposeError {
    pub message: String,
    pub diagnostics: Vec<Diagnostic>,
}
//#endregion 🔖️ComposeTypes

//#region 🔖️ErasedRegistry
/// 🧾️ Erased payload crossing composer/registry boundaries (dispatch, UI, wire).
#[derive(Clone, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IoDirection {
    Import,
    Export,
}

/// 🗝️ Owned mirror of two dialects + direction — the registry key. Owned (not `&'static`) so it
/// can be built from runtime UI input (format kind strings) as well as static composer entries.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
//#endregion 🔖️ErasedRegistry
// #endregion io
