//! 📥️ Items of `io` that belong to `os-store-codec-registry` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🚪️io/🦀️.rs`.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_diagnostic::Diagnostic;
use semio_framework_artifact_reference::{ArtifactDialect, Dialect};
use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
use crate::composer_registry::{AsyncComposeFn, ComposeError, ComposeFuture, ComposedArtifact, ComposerEntry, Confidence, ErasedComposeSource, FORMAT_CATALOG, FormatDescriptor, FormatDescriptorIndexes, FormatRegistryConflict, FormatRegistryError, IO_REGISTRY, IoDirection, IoKey, IoRegistryConflict, IoRegistryRegistrationError, IoRegistryUnavailable, SUBSET_VALIDATOR_REGISTRY, SnapshotSubsetValidation, SubsetValidator, SubsetValidatorEntry, SubsetValidatorRegistryConflict, SubsetValidatorRegistryError, bindings, canonicalize_format_descriptor, composer_entries_by_key, format_catalog, format_descriptor_keys, format_mimes, formats_csv, index_format_descriptors, io_registry, same_composer_entry, same_subset_validator_entry, sqlite_snapshot_format_descriptor, subset_validator_registry, validate_composer_entries, validate_format_descriptors, validate_subset_validators};
use semio_framework_io_schema::IoPayload;
//#region 🔖️ComposeTypes
/// 📦 Analysis result carrying the dialect the analyzer determined it read, with soft diagnostics.
#[derive(Clone, Debug)]
pub struct Analysis<T> {
    pub parts: T,
    pub dialect: Dialect,
    pub confidence: Confidence,
    pub diagnostics: Vec<Diagnostic>,
}
//#endregion 🔖️ComposeTypes

//#region 🔖️ErasedRegistry
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
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

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
//#endregion 🔖️ErasedRegistry

//#region 🔖️ErasedRegistry
//#region 🔖️SubsetValidator

/// ⚖️ Declared strongest IO fidelity a subset codec achieves.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub enum IoFidelityClass {
    Exact,
    Canonical,
    Semantic,
    Lossy,
}
//#endregion 🔖️SubsetValidator
//#endregion 🔖️ErasedRegistry

//#region 🔖️ErasedRegistry
//#region 🔖️IoFidelity

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
//#endregion 🔖️IoFidelity
//#endregion 🔖️ErasedRegistry
