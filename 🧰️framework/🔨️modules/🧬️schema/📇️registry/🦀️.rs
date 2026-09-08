//! 📇️ The `(scope id, export id, format id)` resolution contract, with **no** dependency at all —
//! not on `semio-framework-os-kernel`, not on `pack`, not on any third-party crate.
//!
//! Every scope that publishes named exports registers them here, including scopes whose crates are
//! dependency-restricted (`semio-framework-ui-contract` forbids an os-kernel edge and asserts it with
//! `cargo tree`). `semio-framework-schema` depends on this crate, re-exports its whole surface, and
//! adds the two pieces that genuinely need more: the artifact schema descriptor's kernel round-trip
//! and the draft-07 structural validator.
//!
//! See `📋️execution-contract.md` §A/§C of ticket `26/09/08/SCOPE-OWNED-SCHEMA-CONTRACTS` and the
//! `framework.schema` JSON Schema facet at `🧰️framework/🔨️modules/🧬️schema/🔣️.json`.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

//#region 🔖️SchemaFormat
/// 🗂️ One of the five schema formats a scope publishes, mirroring the taxonomy `schemaFormats` keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchemaFormat {
    Rust,
    Typescript,
    Graphql,
    JsonSchema,
    Protobuf,
}

impl SchemaFormat {
    /// 🧾 Every format, in taxonomy declaration order.
    pub const ALL: [Self; 5] = [Self::Rust, Self::Typescript, Self::Graphql, Self::JsonSchema, Self::Protobuf];

    /// 🏷️ Ascii format id used by the derived catalog and the `schema://` resolver.
    pub fn id(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Typescript => "typescript",
            Self::Graphql => "graphql",
            Self::JsonSchema => "jsonschema",
            Self::Protobuf => "protobuf",
        }
    }

    /// 🧩 Taxonomy `schemaFormats` key this format is the twin of.
    pub fn taxonomy_key(self) -> &'static str {
        match self {
            Self::Rust => "🦀️rust",
            Self::Typescript => "🟦️typescript",
            Self::Graphql => "🔗️graphql",
            Self::JsonSchema => "🔣️jsonschema",
            Self::Protobuf => "🛰️protobuf",
        }
    }

    /// 🔎 Parses either the ascii id or the taxonomy key.
    pub fn parse(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|format| format.id() == id || format.taxonomy_key() == id)
    }

    /// 🍃 The leaf body this format occupies inside a [`FacetLeaves`].
    pub fn leaf(self, leaves: &FacetLeaves) -> &'static str {
        match self {
            Self::Rust => leaves.rust,
            Self::Typescript => leaves.typescript,
            Self::Graphql => leaves.graphql,
            Self::JsonSchema => leaves.json_schema,
            Self::Protobuf => leaves.proto,
        }
    }
}

impl std::fmt::Display for SchemaFormat {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.id())
    }
}
//#endregion 🔖️SchemaFormat

//#region 🔖️ScopeSchemaExports
/// 🍃 Five handcrafted leaf bodies for one facet or one named export (`include_str!` at each
/// registration site). An empty body means the scope does not provide that format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FacetLeaves {
    pub rust: &'static str,
    pub typescript: &'static str,
    pub graphql: &'static str,
    pub json_schema: &'static str,
    pub proto: &'static str,
}

/// 🏷️ One named export of a scope — the `(export id, five format leaves)` pair the resolution key
/// `(scope id, export id, format id)` needs beyond the four fixed facets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchemaExport {
    pub id: &'static str,
    pub leaves: FacetLeaves,
}

/// 🧬️ A scope's named exports. Sibling to the four fixed facets of an artifact schema descriptor for
/// the same reason the inference descriptor is a sibling: the four-facet descriptor is handcrafted at
/// 235 call sites in 115 files, and a scope declares named exports independently of them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScopeSchemaExports {
    pub scope: &'static str,
    pub exports: &'static [SchemaExport],
}

/// 🔒️ Export ids reserved by the four fixed facets of an artifact schema descriptor, in the order
/// [`SchemaExportRegistry::register_facet_leaves`] expects them.
pub const RESERVED_FACET_EXPORT_IDS: [&str; 4] = ["artifact", "snapshot", "diff", "mutations"];

/// ⚠️ Named-export registration rejects a conflicting or internally duplicated declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SchemaExportRegistryError {
    ConflictingScope { scope: String },
    DuplicateExportId { scope: String, export: String },
}

impl std::fmt::Display for SchemaExportRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConflictingScope { scope } => write!(formatter, "schema-export declaration conflicts for scope {scope}"),
            Self::DuplicateExportId { scope, export } => write!(formatter, "scope {scope} declares export id {export} twice"),
        }
    }
}

impl std::error::Error for SchemaExportRegistryError {}

/// ⚠️ Why `(scope id, export id, format id)` did not resolve to a leaf body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SchemaResolveError {
    UnknownScope { scope: String },
    UnknownExport { scope: String, export: String },
    FormatAbsent { scope: String, export: String, format: SchemaFormat },
    AmbiguousScope { scope: String, export: String },
}

impl std::fmt::Display for SchemaResolveError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownScope { scope } => write!(formatter, "unknown schema scope {scope}"),
            Self::UnknownExport { scope, export } => write!(formatter, "scope {scope} declares no export {export}"),
            Self::FormatAbsent { scope, export, format } => write!(formatter, "scope {scope} export {export} has no {format} leaf"),
            Self::AmbiguousScope { scope, export } => write!(formatter, "scope {scope} resolves export {export} from both a fixed facet and a named export"),
        }
    }
}

impl std::error::Error for SchemaResolveError {}

/// 📇️ One resolvable `(scope id, export id, format id)` triple with a non-empty leaf body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchemaExportEntry {
    pub scope: &'static str,
    pub export: &'static str,
    pub format: SchemaFormat,
}
//#endregion 🔖️ScopeSchemaExports

//#region 🔖️SchemaExportRegistry
/// 📚 Resolves `(scope id, export id, format id)` over every scope's four fixed facets plus its
/// [`ScopeSchemaExports`]. Exact resolution only — no nearest-parent search, no glob, no
/// fixture-local fallback.
pub struct SchemaExportRegistry {
    facets: HashMap<&'static str, [FacetLeaves; 4]>,
    named: HashMap<&'static str, &'static [SchemaExport]>,
}

impl Default for SchemaExportRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaExportRegistry {
    /// 🏗️ Empty registry.
    pub fn new() -> Self {
        Self { facets: HashMap::new(), named: HashMap::new() }
    }

    /// 📎 Adds one scope's four fixed facet leaves, positionally keyed by
    /// [`RESERVED_FACET_EXPORT_IDS`]. Exact duplicates are accepted, conflicts are fatal.
    pub fn register_facet_leaves(&mut self, scope: &'static str, facets: [FacetLeaves; 4]) -> Result<(), SchemaExportRegistryError> {
        match self.facets.get(scope) {
            Some(established) if *established == facets => Ok(()),
            Some(_) => Err(SchemaExportRegistryError::ConflictingScope { scope: scope.to_string() }),
            None => {
                self.facets.insert(scope, facets);
                Ok(())
            }
        }
    }

    /// 📎 Adds one scope's named exports. Export ids must be unique inside the scope; exact duplicate
    /// declarations are accepted, conflicts are fatal.
    pub fn register_exports(&mut self, declaration: ScopeSchemaExports) -> Result<(), SchemaExportRegistryError> {
        for (index, export) in declaration.exports.iter().enumerate() {
            if declaration.exports[..index].iter().any(|previous| previous.id == export.id) {
                return Err(SchemaExportRegistryError::DuplicateExportId { scope: declaration.scope.to_string(), export: export.id.to_string() });
            }
        }
        match self.named.get(declaration.scope) {
            Some(established) if *established == declaration.exports => Ok(()),
            Some(_) => Err(SchemaExportRegistryError::ConflictingScope { scope: declaration.scope.to_string() }),
            None => {
                self.named.insert(declaration.scope, declaration.exports);
                Ok(())
            }
        }
    }

    /// 🚶 Every registered scope id, sorted.
    pub fn scopes(&self) -> Vec<&'static str> {
        let mut scopes: Vec<&'static str> = self.facets.keys().chain(self.named.keys()).copied().collect();
        scopes.sort_unstable();
        scopes.dedup();
        scopes
    }

    /// 🧾 Every export id a scope publishes — the four fixed facets first, then the named exports in
    /// declaration order.
    pub fn exports(&self, scope: &str) -> Result<Vec<&'static str>, SchemaResolveError> {
        let facets = self.facets.get(scope);
        let named = self.named.get(scope);
        if facets.is_none() && named.is_none() {
            return Err(SchemaResolveError::UnknownScope { scope: scope.to_string() });
        }
        let mut exports: Vec<&'static str> = facets.map(|_| RESERVED_FACET_EXPORT_IDS.to_vec()).unwrap_or_default();
        exports.extend(named.into_iter().flat_map(|exports| exports.iter().map(|export| export.id)));
        Ok(exports)
    }

    /// 🔎 Resolves one `(scope id, export id, format id)` triple to its handcrafted leaf body.
    pub fn resolve(&self, scope: &str, export: &str, format: SchemaFormat) -> Result<&'static str, SchemaResolveError> {
        let leaves = self.leaves(scope, export)?;
        let body = format.leaf(&leaves);
        if body.trim().is_empty() {
            return Err(SchemaResolveError::FormatAbsent { scope: scope.to_string(), export: export.to_string(), format });
        }
        Ok(body)
    }

    /// 🍃 The five format leaves behind one `(scope id, export id)` pair.
    pub fn leaves(&self, scope: &str, export: &str) -> Result<FacetLeaves, SchemaResolveError> {
        let facets = self.facets.get(scope);
        let named = self.named.get(scope);
        if facets.is_none() && named.is_none() {
            return Err(SchemaResolveError::UnknownScope { scope: scope.to_string() });
        }
        let facet = facets.and_then(|facets| RESERVED_FACET_EXPORT_IDS.iter().position(|reserved| *reserved == export).map(|index| facets[index]));
        let declared = named.and_then(|exports| exports.iter().find(|candidate| candidate.id == export)).map(|candidate| candidate.leaves);
        match (facet, declared) {
            (Some(_), Some(_)) => Err(SchemaResolveError::AmbiguousScope { scope: scope.to_string(), export: export.to_string() }),
            (Some(leaves), None) | (None, Some(leaves)) => Ok(leaves),
            (None, None) => Err(SchemaResolveError::UnknownExport { scope: scope.to_string(), export: export.to_string() }),
        }
    }

    /// 🚶 Every resolvable `(scope, export, format)` triple with a non-empty leaf, in deterministic
    /// order — the cross-check input for the derived schema catalog.
    pub fn entries(&self) -> impl Iterator<Item = SchemaExportEntry> + '_ {
        self.scopes().into_iter().flat_map(move |scope| {
            let exports = self.exports(scope).unwrap_or_default();
            exports.into_iter().flat_map(move |export| {
                SchemaFormat::ALL.into_iter().filter_map(move |format| match self.resolve(scope, export, format) {
                    Ok(_) => Some(SchemaExportEntry { scope, export, format }),
                    Err(_) => None,
                })
            })
        })
    }
}
//#endregion 🔖️SchemaExportRegistry

//#region 🔖️GlobalSchemaExportCatalog
static SCOPE_SCHEMA_EXPORTS: OnceLock<Mutex<HashMap<&'static str, &'static [SchemaExport]>>> = OnceLock::new();
static SCOPE_SCHEMA_FACETS: OnceLock<Mutex<HashMap<&'static str, [FacetLeaves; 4]>>> = OnceLock::new();

fn scope_schema_exports() -> &'static Mutex<HashMap<&'static str, &'static [SchemaExport]>> {
    SCOPE_SCHEMA_EXPORTS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn scope_schema_facets() -> &'static Mutex<HashMap<&'static str, [FacetLeaves; 4]>> {
    SCOPE_SCHEMA_FACETS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 🔌 Open named-export registry API for scope-owning crates — call [`register_scope_schema_exports`]
/// from your own init code, then resolve anywhere with [`resolve_schema_export`].
///
/// 📎 Registers one scope's named exports into the OS-wide catalog. Exact duplicates are accepted so a
/// scope registered twice by two consumers is not an error; a differing declaration is fatal.
pub fn register_scope_schema_exports(declaration: ScopeSchemaExports) -> Result<(), SchemaExportRegistryError> {
    let mut registry = SchemaExportRegistry::new();
    let established = scope_schema_exports().lock().expect("scope schema export catalog lock").clone();
    for (scope, exports) in established {
        registry.register_exports(ScopeSchemaExports { scope, exports })?;
    }
    registry.register_exports(declaration)?;
    scope_schema_exports().lock().expect("scope schema export catalog lock").insert(declaration.scope, declaration.exports);
    Ok(())
}

/// 📎 Registers one scope's four fixed facet leaves into the OS-wide catalog, positionally keyed by
/// [`RESERVED_FACET_EXPORT_IDS`]. `semio-framework-schema` calls this from
/// `register_artifact_schema_descriptor` so a descriptor's facets resolve through the same
/// `(scope, export, format)` key as its named exports.
pub fn register_scope_facet_leaves(scope: &'static str, facets: [FacetLeaves; 4]) -> Result<(), SchemaExportRegistryError> {
    let mut registry = SchemaExportRegistry::new();
    let established = scope_schema_facets().lock().expect("scope schema facet catalog lock").clone();
    for (established_scope, established_facets) in established {
        registry.register_facet_leaves(established_scope, established_facets)?;
    }
    registry.register_facet_leaves(scope, facets)?;
    scope_schema_facets().lock().expect("scope schema facet catalog lock").insert(scope, facets);
    Ok(())
}

/// 🔎 Whether `scope` has registered named exports in the OS-wide catalog.
pub fn scope_schema_exports_registered(scope: &str) -> bool {
    scope_schema_exports().lock().expect("scope schema export catalog lock").contains_key(scope)
}

/// 🔎 Whether `scope` has registered its four fixed facet leaves in the OS-wide catalog.
pub fn scope_schema_facets_registered(scope: &str) -> bool {
    scope_schema_facets().lock().expect("scope schema facet catalog lock").contains_key(scope)
}

/// 📚 Invokes `visit` with a [`SchemaExportRegistry`] snapshot over every registered fixed-facet set
/// and every registered named-export declaration.
pub fn with_schema_export_registry<R>(visit: impl FnOnce(&SchemaExportRegistry) -> R) -> R {
    let mut registry = SchemaExportRegistry::new();
    for (scope, facets) in scope_schema_facets().lock().expect("scope schema facet catalog lock").iter() {
        let _ = registry.register_facet_leaves(scope, *facets);
    }
    for (scope, exports) in scope_schema_exports().lock().expect("scope schema export catalog lock").iter() {
        let _ = registry.register_exports(ScopeSchemaExports { scope, exports });
    }
    visit(&registry)
}

/// 🔎 Resolves `(scope id, export id, format id)` against the OS-wide catalog.
pub fn resolve_schema_export(scope: &str, export: &str, format: SchemaFormat) -> Result<&'static str, SchemaResolveError> {
    with_schema_export_registry(|registry| registry.resolve(scope, export, format))
}

/// 🚶 Snapshots every resolvable `(scope, export, format)` triple in the OS-wide catalog.
pub fn schema_export_catalog_entries() -> Vec<SchemaExportEntry> {
    with_schema_export_registry(|registry| registry.entries().collect())
}
//#endregion 🔖️GlobalSchemaExportCatalog

//#region 🔖️SchemaExportEntries
/// 📤️ The runtime export registry rendered as the `schema-export-registry-entries-v1` dump that
/// `schema verify --rust-entries` reads. Entries are sorted and deduplicated by
/// `(scope, export, format)`, so the dump of one binary is byte-stable across runs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchemaExportEntries {
    pub contract_id: &'static str,
    pub generator: String,
    pub entries: Vec<SchemaExportEntry>,
}

impl SchemaExportEntries {
    /// 🪪️ Taxonomy `schemaExportResolution.rustEntriesContractId` — the consumer bails with
    /// `rust-entries-contract-unknown` on any other value, so the two are changed together.
    pub const CONTRACT_ID: &'static str = "schema-export-registry-entries-v1";

    /// 🚶 Snapshots the OS-wide catalog under the command that produced the snapshot, sorted and
    /// deduplicated by `(scope, export, format)` in the ascii spelling the dump renders.
    pub fn from_catalog(generator: impl Into<String>) -> Self {
        let mut entries = schema_export_catalog_entries();
        entries.sort_unstable_by_key(|entry| (entry.scope, entry.export, entry.format.id()));
        entries.dedup();
        Self { contract_id: Self::CONTRACT_ID, generator: generator.into(), entries }
    }

    /// 🧾 Renders the dump, one entry per line, exactly as the consumer parses it.
    pub fn to_json(&self) -> String {
        let entries: Vec<String> = self
            .entries
            .iter()
            .map(|entry| format!("\n    {{ \"scope\": \"{}\", \"export\": \"{}\", \"format\": \"{}\" }}", entry.scope, entry.export, entry.format.id()))
            .collect();
        format!("{{\n  \"contractId\": \"{}\",\n  \"generator\": \"{}\",\n  \"entries\": [{}\n  ]\n}}\n", self.contract_id, escape_json_string(&self.generator), entries.join(","))
    }
}

/// 🔡 Escapes a string for a JSON string body — the only serialization this crate performs, kept
/// in-house so the resolution contract stays dependency-free.
fn escape_json_string(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0c}' => escaped.push_str("\\f"),
            character if (character as u32) < 0x20 => escaped.push_str(&format!("\\u{:04x}", character as u32)),
            character => escaped.push(character),
        }
    }
    escaped
}
//#endregion 🔖️SchemaExportEntries

#[cfg(test)]
//#region 🔖️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
