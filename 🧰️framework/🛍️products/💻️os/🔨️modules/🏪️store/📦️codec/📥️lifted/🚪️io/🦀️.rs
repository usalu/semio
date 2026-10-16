// 📥️ Lifted from `🚪️io/🦀️.rs` by S0: the composer registry, `IoKey`, subset validators, format catalog and the native-snapshot registration plan that belong to `os-store-codec-registry`.
// The `io` module mounts this file as a module and re-exports it, so every public item keeps its `io` path until the file is the codec-registry crate.
use crate::composer_registry::bindings::catalog::{ArtifactCatalogBinding, ArtifactCatalogBindingError, ArtifactCatalogCapability, ArtifactCatalogTarget, artifact_catalog_bindings, preflight_artifact_catalog_bindings_in_assembly};
use crate::composer_registry::bindings::{ArtifactCodecBinding, ArtifactCodecBindingChannel, ArtifactCodecBindingError, artifact_codec_bindings, preflight_artifact_codec_bindings, preflight_artifact_codec_bindings_in_assembly};
use semio_framework_artifact_reference::{ArtifactDialect, Dialect};
use std::collections::BTreeMap;
use semio_framework_diagnostic::Diagnostic;
use semio_framework_value::{FromValue, ToValue};
use std::sync::RwLock;

//#region 🔖️ComposeTypes
/// 🎚 Soft confidence for partial analysis/composition success.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub enum Confidence {
    High,
    Medium,
    Low,
}
//#endregion 🔖️ComposeTypes

//#region 🔖️ComposeTypes
/// ⚠️ Composition failed: no compatible source dialect, or every candidate errored.
#[derive(Clone, Debug, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub struct ComposeError {
    pub message: String,
    pub diagnostics: Vec<Diagnostic>,
}
//#endregion 🔖️ComposeTypes


/// 🧾️ Erased payload crossing composer/registry boundaries (dispatch, UI, wire).
pub use semio_framework_io_schema::IoPayload;

//#region 🔖️ErasedRegistry
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
//#endregion 🔖️ErasedRegistry

//#region 🔖️ErasedRegistry
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

pub static IO_REGISTRY: std::sync::OnceLock<RwLock<BTreeMap<IoKey, &'static ComposerEntry>>> = std::sync::OnceLock::new();

pub fn io_registry() -> &'static RwLock<BTreeMap<IoKey, &'static ComposerEntry>> {
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

pub fn same_composer_entry(left: &ComposerEntry, right: &ComposerEntry) -> bool {
    left.writes == right.writes && left.reads == right.reads && std::ptr::fn_addr_eq(left.compose, right.compose)
}

pub fn composer_entries_by_key<'entry>(entries: impl IntoIterator<Item = &'entry ComposerEntry>) -> Result<BTreeMap<IoKey, &'entry ComposerEntry>, IoRegistryRegistrationError> {
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

pub fn validate_composer_entries(registry: &BTreeMap<IoKey, &'static ComposerEntry>, proposed: &BTreeMap<IoKey, &'static ComposerEntry>) -> Result<(), IoRegistryRegistrationError> {
    for (key, entry) in proposed {
        if let Some(existing) = registry.get(key) {
            if !same_composer_entry(existing, entry) {
                return Err(IoRegistryRegistrationError::Conflict(Box::new(IoRegistryConflict { key: key.clone() })));
            }
        }
    }
    Ok(())
}
//#endregion 🔖️ErasedRegistry

//#region 🔖️ErasedRegistry

/// 🛡️ SDK trait (D5, ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION):
/// one subset's own conformance checker (e.g. PDF/A-2b's "no `/Encrypt`, no JS/Launch actions,
/// `OutputIntent` present, fonts embedded"). Unlike `ArtifactComposer`/`ArtifactAnalyzer` (which
/// live in the plugin crate because they need typed `Snapshot` visibility this generic/erased io
/// module doesn't have), `SubsetValidator` can live directly HERE: its signature is already
/// erased over `IoPayload` (mirroring `ComposerEntry.compose`'s own `fn(&[ErasedComposeSource])`
/// erasure) -- a concrete artifact implements it by decoding its own typed `Snapshot` out of the
/// payload internally (via `pack::snapshot::ArtifactPack`/`ArtifactDsl`, exactly like `ComposerEntry`'s
/// erasure already does one layer up), so this module never needs to know the concrete type.
pub trait SubsetValidator {
    const DIALECT: Dialect;
    fn validate(payload: &IoPayload) -> impl std::future::Future<Output = Vec<Diagnostic>> + Send;
}
//#endregion 🔖️ErasedRegistry

//#region 🔖️ErasedRegistry
//#region 🔖️SubsetValidator
/// 🧩️ Type-erased `semio_framework_ui_contract_validation::validate_snapshot`; it refuses a snapshot of any other type.
pub type SnapshotSubsetValidation = fn(&dyn std::any::Any, &semio_framework_io_sqlite_snapshot::SqliteDatabase, &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>) -> semio_framework_io_schema::IoResult<()>;

/// 🧾️ Type-erased subset-validator vtable row -- the registry stores this, mirroring how
/// `ComposerEntry` stores a plain `fn` pointer rather than a trait object.
pub struct SubsetValidatorEntry {
    pub dialect: Dialect,
    pub validate: fn(&IoPayload) -> Vec<Diagnostic>,
    pub validate_snapshot: Option<SnapshotSubsetValidation>,
}
//#endregion 🔖️SubsetValidator
//#endregion 🔖️ErasedRegistry

//#region 🔖️ErasedRegistry
//#region 🔖️SubsetValidator
pub static SUBSET_VALIDATOR_REGISTRY: std::sync::OnceLock<RwLock<BTreeMap<ArtifactDialect, &'static SubsetValidatorEntry>>> = std::sync::OnceLock::new();

pub fn subset_validator_registry() -> &'static RwLock<BTreeMap<ArtifactDialect, &'static SubsetValidatorEntry>> {
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
//#endregion 🔖️SubsetValidator
//#endregion 🔖️ErasedRegistry

//#region 🔖️ErasedRegistry
//#region 🔖️SubsetValidator
pub fn same_subset_validator_entry(left: &SubsetValidatorEntry, right: &SubsetValidatorEntry) -> bool {
    left.dialect == right.dialect && std::ptr::fn_addr_eq(left.validate, right.validate) && match (left.validate_snapshot, right.validate_snapshot) {
        (Some(left), Some(right)) => std::ptr::fn_addr_eq(left, right),
        (None, None) => true,
        _ => false,
    }
}
//#endregion 🔖️SubsetValidator
//#endregion 🔖️ErasedRegistry

//#region 🔖️ErasedRegistry
//#region 🔖️SubsetValidator
pub fn validate_subset_validators(registry: &BTreeMap<ArtifactDialect, &'static SubsetValidatorEntry>, entries: &[&'static SubsetValidatorEntry]) -> Result<(), SubsetValidatorRegistryError> {
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
//#endregion 🔖️SubsetValidator
//#endregion 🔖️ErasedRegistry

//#region 🔖️ErasedRegistry

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
//#endregion 🔖️ErasedRegistry

//#region 🔖️ErasedRegistry
//#region 🔖️FormatCatalog
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

pub static FORMAT_CATALOG: std::sync::OnceLock<RwLock<BTreeMap<String, FormatDescriptor>>> = std::sync::OnceLock::new();

/// 🪶️ Framework-owned semantic SQLite file endpoint for every declared artifact snapshot.
pub fn sqlite_snapshot_format_descriptor() -> FormatDescriptor {
    FormatDescriptor { kind_id: semio_framework_io_schema::SQLITE_SNAPSHOT.artifact_kind.to_string(), short_id: "sqlite".to_string(), aliases: vec![], mimes: vec!["application/vnd.sqlite3".to_string()], extensions: vec![".sqlite".to_string()], name: "SQLite".to_string(), full_name: "Semantic Artifact Snapshot SQLite".to_string(), neutral: true, dir_name: "sqlite".to_string(), is_binary: true }
}

pub fn format_catalog() -> &'static RwLock<BTreeMap<String, FormatDescriptor>> {
    FORMAT_CATALOG.get_or_init(|| {
        let descriptor = sqlite_snapshot_format_descriptor();
        let rows = format_descriptor_keys(&descriptor).map(|key| (key, descriptor.clone())).collect();
        RwLock::new(rows)
    })
}
//#endregion 🔖️FormatCatalog
//#endregion 🔖️ErasedRegistry

//#region 🔖️ErasedRegistry
//#region 🔖️FormatCatalog
pub type FormatDescriptorIndexes = (BTreeMap<String, FormatDescriptor>, BTreeMap<String, FormatDescriptor>);

pub fn index_format_descriptors(descriptors: impl IntoIterator<Item = FormatDescriptor>) -> Result<FormatDescriptorIndexes, FormatRegistryConflict> {
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

pub fn canonicalize_format_descriptor(mut row: FormatDescriptor) -> Result<FormatDescriptor, FormatRegistryConflict> {
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

pub fn format_mimes(row: &FormatDescriptor) -> impl Iterator<Item = &str> {
    row.mimes.iter().map(String::as_str)
}

pub fn validate_format_descriptors(registry: &BTreeMap<String, FormatDescriptor>, proposed: &BTreeMap<String, FormatDescriptor>, proposed_by_kind: &BTreeMap<String, FormatDescriptor>) -> Result<(), FormatRegistryConflict> {
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

pub fn format_descriptor_keys(row: &FormatDescriptor) -> impl Iterator<Item = String> + '_ {
    std::iter::once(row.kind_id.clone()).chain(std::iter::once(row.short_id.clone())).chain(row.aliases.iter().cloned())
}
//#endregion 🔖️FormatCatalog
//#endregion 🔖️ErasedRegistry

//#region 🔖️ErasedRegistry
//#region 🔖️FormatCatalog
/// 🧷️ All IO and store rows a plugin must publish as one irreducible assembly unit.
#[path="../../../../🚪️io/🪪️bindings/🦀️.rs"]
pub mod bindings;



















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

