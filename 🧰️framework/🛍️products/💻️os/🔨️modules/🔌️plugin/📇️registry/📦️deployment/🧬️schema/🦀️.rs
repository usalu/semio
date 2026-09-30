//! 🧩️ Shared plugin module manifests and generation indexes.
use serde::{Deserialize, Serialize};
use semio_framework_schema_registry::{register_scope_schema_exports, FacetLeaves, SchemaExport, ScopeSchemaExports};

/// 🧬️ The schema owner shared by publishers and clients.
pub const SCHEMA_SCOPE: &str = "os.plugin.registry.deployment";
const ALL_LEAVES: FacetLeaves = FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: "", json_schema: include_str!("🔣️.json"), proto: "" };
const JSON_LEAVES: FacetLeaves = FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: "" };
const EXPORTS: [SchemaExport; 9] = [
    SchemaExport { id: "PluginModuleIdentityV1", leaves: JSON_LEAVES },
    SchemaExport { id: "PluginModuleDigestV1", leaves: JSON_LEAVES },
    SchemaExport { id: "DeploymentCatalogV1", leaves: JSON_LEAVES },
    SchemaExport { id: "DeploymentModuleRoutesV1", leaves: JSON_LEAVES },
    SchemaExport { id: "TrustedPluginModulePathV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedPluginModuleFileV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedPluginModuleBundleV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedPluginModuleIndexEntryV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedPluginModuleIndexV1", leaves: ALL_LEAVES },
];

/// 📇️ Registers the shared plugin publication schema exports.
pub fn register_scope_exports() {
    register_scope_schema_exports(ScopeSchemaExports { scope: SCHEMA_SCOPE, exports: &EXPORTS }).expect("shared plugin module schema exports");
}

/// 🏷️ Closed trusted plugin module bundle manifest schema identity.
pub const TRUSTED_PLUGIN_MODULE_SCHEMA: &str = "semio.os.plugin-module/v1";
/// 🏷️ Closed `GET /trusted-catalog/plugin-modules` answer schema identity.
pub const TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA: &str = "semio.os.plugin-module-index/v1";
/// 📛️ One plugin-module-relative path: 1–16 segments, none empty, `.`, `..` or carrying `\`, a control
/// character, `?`, `#` or `%`, so it names the same file as a filesystem path and as a URL path.
pub type TrustedPluginModulePathV1 = String;

/// 🧩️ One file of a plugin module bundle, content-addressed by its digests.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedPluginModuleFileV1 {
    pub path: TrustedPluginModulePathV1,
    pub byte_length: u64,
    pub sha256: String,
    pub blake3: String,
}

/// 🧩️ The manifest of one package's browser plugin module (host shim, component module, core Wasm,
/// descriptor and vendored imports), derived from the package's own trusted component and descriptor.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedPluginModuleBundleV1 {
    pub schema: String,
    pub plugin_id: String,
    pub package_id: String,
    pub version: String,
    pub source_component_sha256: String,
    pub source_descriptor_byte_sha256: String,
    pub module_directory: String,
    pub entry: TrustedPluginModulePathV1,
    pub files: Vec<TrustedPluginModuleFileV1>,
}

/// 📇️ One installable plugin module of the current generation, addressed by its manifest digest.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedPluginModuleIndexEntryV1 {
    pub plugin_id: String,
    pub package_id: String,
    pub version: String,
    pub component_sha256: String,
    pub descriptor_byte_sha256: String,
    pub dependencies: Vec<String>,
    /// 🎭️ Every app dialect artifact kind the package's surfaces open, in ascending byte order: how a shell that never
    /// built the plugin finds the package that opens a kind.
    pub dialect_artifact_kinds: Vec<String>,
    /// 🧩️ The plugin an extension package extends (its first declared dependency), `None` for a plugin: how a shell
    /// activates a hub-resolved plugin's extensions from the same generation. Required on the wire (`null` for a plugin).
    #[serde(deserialize_with = "Option::deserialize")]
    pub extends_plugin_id: Option<String>,
    pub bundle_sha256: String,
    pub bundle_byte_length: u64,
    pub entry: TrustedPluginModulePathV1,
}

/// 📇️ What `GET /trusted-catalog/plugin-modules` answers: every plugin module of one generation.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedPluginModuleIndexV1 {
    pub schema: String,
    pub generation_id: String,
    pub modules: Vec<TrustedPluginModuleIndexEntryV1>,
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️scope-schema-exports/🦀️.rs"]
mod scope_schema_exports;
