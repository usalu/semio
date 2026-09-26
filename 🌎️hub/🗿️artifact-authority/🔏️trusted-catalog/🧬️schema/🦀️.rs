//! 🧬️ Sole owner of the trusted-catalog publication command, receipt and current-pointer contract.
//!
//! The normative shape lives in the sibling [`🔣️.json`](🔣️.json) draft-07 module as
//! `TrustedCatalogPublicationCommandV1`, `TrustedCatalogPublicationReceiptV1` and
//! `TrustedCatalogCurrentPointerV1`; these types are its Rust projection.

use super::{catalog, catalog_error, decode_digest, AuthorityError, TRUSTED_IDENTITY_MAX_BYTES};
use semio_framework::PackageRole;
use serde::{Deserialize, Serialize};

//#region 🔖️ScopeSchemaExports
use semio_framework_schema_registry::{register_scope_schema_exports, FacetLeaves, SchemaExport, ScopeSchemaExports};

/// 🧬️ The scope id every export of this module resolves under.
pub const SCHEMA_SCOPE: &str = "hub.artifact-authority.trusted-catalog";

/// 🍃 The leaves this module carries. An empty body is the registry's spelling of "not provided".
const ALL_LEAVES: FacetLeaves = FacetLeaves { rust: include_str!("🦀️.rs"), typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: "" };

/// 📚️ The module document the annotation is read from, so the law never restates it.
#[cfg(test)]
const MODULE_JSON: &str = include_str!("🔣️.json");

/// 🏷️ `$defs` of `🔣️.json`, in declaration order.
const EXPORTS: [SchemaExport; 31] = [
    SchemaExport { id: "TrustedCatalogRelativePathV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedBundleIdentityV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedBundleCodecV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedBundleParentDialectV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedBundleGrantV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedBundleOpenTargetV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedDescriptorOpenTargetV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedCatalogDescriptorOpenTargetsV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedBundleFileV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedBundleComponentV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedBundleExecutionProtocolV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedBundleBrowserActorV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedPluginModulePathV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedPluginModuleFileV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedPluginModuleBundleV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedBundlePluginModuleV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedBundlePackageV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedBundleProfileOpenTargetV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedBundleProfileV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedBundleV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedCatalogCurrentPointerV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedCatalogPublicationCommandV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedCatalogPublicationReceiptV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedPluginModuleIndexEntryV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedPluginModuleIndexV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedCatalogGuestResidencyV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedCatalogGuestResidencyStateV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedCatalogPackagePhaseV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedCatalogPackageProgressV1", leaves: ALL_LEAVES },
    SchemaExport { id: "TrustedCatalogLoadProgressV1", leaves: ALL_LEAVES },
    SchemaExport { id: "GuestCodecVerificationV1", leaves: ALL_LEAVES },
];

/// 📌️ Registers `hub.artifact-authority.trusted-catalog`'s named exports into the process-wide export catalog.
/// See `📋️execution-contract.md` §C and `semio_framework_schema_registry::resolve_schema_export`.
// 🚫️async: pure registration helper (no I/O)
pub fn register_scope_exports() {
    register_scope_schema_exports(ScopeSchemaExports { scope: SCHEMA_SCOPE, exports: &EXPORTS }).expect("hub.artifact-authority.trusted-catalog scope schema exports");
}
/// 🔬 Proves the registration at runtime rather than by inspection: it registers, resolves every
/// export in exactly the formats its `"x-semio-formats"` annotation names and in no other, and
/// asserts the scope is visible in the process-wide catalog.
#[cfg(test)]
#[path = "🧪️tests/🔬️scope-schema-export-law-standalone/🦀️.rs"]
mod scope_schema_export_law;
//#endregion 🔖️ScopeSchemaExports

/// 🧬️ The draft-07 module every implementation of this contract is projected from.
pub const TRUSTED_CATALOG_SCHEMA_JSON: &str = include_str!("🔣️.json");

/// 🧊️ `TrustedCatalogGuestResidencyV1`: how many component bytes of compiled guests a hub keeps
/// resident, how admission compares access counts, and how many guests a catalog load compiles and
/// interprets at once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrustedCatalogGuestResidencyV1 {
    pub resident_component_bytes: u64,
    pub concurrent_verifications: usize,
    pub access_count_ceiling: u32,
    pub access_count_aging_per_guest: u64,
}

/// 🌐️ The environment variable an operator sets `residentComponentBytes` with (decimal bytes).
pub const GUEST_RESIDENCY_BYTES_ENV: &str = "OS_HUB_GUEST_RESIDENCY_BYTES";

/// 📏️ The inclusive `minimum`/`maximum` of `residentComponentBytes`.
pub const TRUSTED_CATALOG_GUEST_RESIDENCY_BYTES_BOUNDS: std::ops::RangeInclusive<u64> = 0..=17_179_869_184;

impl TrustedCatalogGuestResidencyV1 {
    /// 🎚️ The residency with the operator's `residentComponentBytes` (`OS_HUB_GUEST_RESIDENCY_BYTES`): the
    /// schema default when absent or empty, refused when it is not a decimal byte count within the bounds.
    pub fn configured(value: Option<&str>) -> Result<Self, String> {
        let Some(text) = value.map(str::trim).filter(|text| !text.is_empty()) else { return Ok(TRUSTED_CATALOG_GUEST_RESIDENCY) };
        let bytes = text.parse::<u64>().ok().filter(|bytes| TRUSTED_CATALOG_GUEST_RESIDENCY_BYTES_BOUNDS.contains(bytes)).ok_or_else(|| {
            format!("{GUEST_RESIDENCY_BYTES_ENV} must be a decimal byte count from {} to {}", TRUSTED_CATALOG_GUEST_RESIDENCY_BYTES_BOUNDS.start(), TRUSTED_CATALOG_GUEST_RESIDENCY_BYTES_BOUNDS.end())
        })?;
        Ok(Self { resident_component_bytes: bytes, ..TRUSTED_CATALOG_GUEST_RESIDENCY })
    }
}

/// 📏️ `TrustedCatalogGuestResidencyStateV1`: what a hub's compiled-guest residency holds now and has
/// done since the hub started.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrustedCatalogGuestResidencyStateV1 {
    pub budget_bytes: u64,
    pub registered_guests: u64,
    pub resident_guests: u64,
    pub resident_bytes: u64,
    pub hits: u64,
    pub compiles: u64,
    pub admitted: u64,
    pub bypassed: u64,
    pub released: u64,
    pub compile_micros: u64,
}

/// 🚦️ `TrustedCatalogPackagePhaseV1`: where one selected package stands in its catalog's load and verification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrustedCatalogPackagePhaseV1 {
    Pending,
    Reading,
    Staged,
    Verifying,
    Ready,
    Refused,
}

/// 📦️ `TrustedCatalogPackageProgressV1`: one selected package's place in its catalog's load and verification.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrustedCatalogPackageProgressV1 {
    pub plugin_id: String,
    pub component_bytes: u64,
    pub phase: TrustedCatalogPackagePhaseV1,
    pub rows: u64,
    pub rows_pinned: u64,
    pub rows_verified: u64,
}

/// 📈️ `TrustedCatalogLoadProgressV1`: how far a hub's trusted catalog has come, counts only.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrustedCatalogLoadProgressV1 {
    pub packages: Vec<TrustedCatalogPackageProgressV1>,
    pub packages_total: u64,
    pub packages_ready: u64,
    pub packages_refused: u64,
    pub component_bytes_total: u64,
    pub component_bytes_read: u64,
    pub rows_total: u64,
    pub rows_pinned: u64,
    pub rows_verified: u64,
}

/// 🗃️ `GuestCodecVerificationV1`: one remembered guest codec verification, content-addressed by its key.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GuestCodecVerificationV1 {
    pub schema: String,
    pub component_sha256: String,
    pub artifact_schema: String,
    pub engine: String,
    pub pack_schema_hash: String,
}

/// 🏷️ The `schema` of every [`GuestCodecVerificationV1`].
pub const GUEST_CODEC_VERIFICATION_SCHEMA: &str = "semio.hub.guest-codec-verification/v1";

/// 🧊️ The residency a hub applies unless its operator configures `residentComponentBytes`: the schema's
/// `default` and `const` values of `TrustedCatalogGuestResidencyV1`.
pub const TRUSTED_CATALOG_GUEST_RESIDENCY: TrustedCatalogGuestResidencyV1 = TrustedCatalogGuestResidencyV1 { resident_component_bytes: 268_435_456, concurrent_verifications: 4, access_count_ceiling: 15, access_count_aging_per_guest: 16 };
/// 🏷️ Closed publication command schema identity.
pub const TRUSTED_CATALOG_PUBLICATION_SCHEMA: &str = "semio.hub.trusted-catalog-publication/v1";
/// 🏷️ Closed publication receipt schema identity.
pub const TRUSTED_CATALOG_PUBLICATION_RECEIPT_SCHEMA: &str = "semio.hub.trusted-catalog-publication-receipt/v1";
/// 🧯️ Maximum accepted serialized publication command or receipt bytes.
pub const TRUSTED_CATALOG_PUBLICATION_MAX_BYTES: usize = 4096;
/// ✅️ Durable publication outcome token.
pub const TRUSTED_CATALOG_PUBLICATION_OUTCOME_DURABLE: &str = "durable";
/// ⚠️ Visible-but-unconfirmed publication outcome token.
pub const TRUSTED_CATALOG_PUBLICATION_OUTCOME_UNCONFIRMED: &str = "replaced-unconfirmed";
/// 🏷️ Closed `os-hub trusted-catalog open-targets` answer schema identity.
pub const TRUSTED_CATALOG_DESCRIPTOR_OPEN_TARGETS_SCHEMA: &str = "semio.hub.trusted-catalog-descriptor-open-targets/v1";
/// 🏷️ Closed trusted plugin module bundle manifest schema identity.
pub const TRUSTED_PLUGIN_MODULE_SCHEMA: &str = "semio.hub.trusted-plugin-module/v1";
/// 🏷️ Closed `GET /trusted-catalog/plugin-modules` answer schema identity.
pub const TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA: &str = "semio.hub.trusted-plugin-module-index/v1";

/// 🔢️ Requires the canonical nonzero unsigned 64-bit spelling every revision token is compared by.
pub fn publication_revision(value: &str) -> Result<u64, AuthorityError> {
    let revision = value.parse::<u64>().map_err(catalog_error)?;
    if revision == 0 || revision.to_string() != value {
        return Err(catalog("trusted publication revision is not canonical nonzero u64"));
    }
    Ok(revision)
}

/// 📛️ One bundle-relative path; the validating newtype lives in the sibling opened-root leaf.
pub type TrustedCatalogRelativePathV1 = super::opened_root::TrustedCatalogRelativePathV1;

/// 🌐️ The catalog actor location of one package; its decoder is the sibling browser-actor leaf.
pub type TrustedBundleBrowserActorV1 = super::browser_actor::TrustedBundleBrowserActorV1;

/// 🎭️ The parent dialect a bundle open target is closed over; the framework owns the shape.
pub type TrustedBundleParentDialectV1 = semio_framework::ArtifactDialect;

/// ⚙️ The app-channel protocol version a bundle package declares; the framework owns the shape.
pub type TrustedBundleExecutionProtocolV1 = semio_framework::ExecutionProtocol;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TrustedBundlePackageRole {
    Plugin,
    Extension,
}

impl TrustedBundlePackageRole {
    pub fn matches(self, role: PackageRole) -> bool {
        matches!((self, role), (Self::Plugin, PackageRole::Plugin) | (Self::Extension, PackageRole::Extension))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedBundleIdentityV1 {
    pub plugin_id: String,
    pub package_id: String,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedBundleCodecV1 {
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub pack_schema_hash: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TrustedBundleOpenRole {
    Viewer,
    Editor,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TrustedBundleRendererTarget {
    React,
    Wgpu,
    Wasm,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedBundleOpenTargetV1 {
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub pack_schema_hash: String,
    pub surface_id: String,
    pub app_id: String,
    pub window_kind_id: String,
    pub role: TrustedBundleOpenRole,
    pub renderer_target: TrustedBundleRendererTarget,
    pub parent_dialect: TrustedBundleParentDialectV1,
    pub grant: TrustedBundleGrantV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedBundleGrantV1 {
    pub read: bool,
    pub write: bool,
    pub observe: bool,
}

/// 🎯️ One document-open surface a verified descriptor declares for one of its artifact kinds, before a codec
/// binds its pack fingerprint: [`TrustedBundleOpenTargetV1`] without `pack_schema_hash`. Produced only by the
/// hub's own pairing rule (`descriptor_open_targets`), never transcribed by a publisher.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedDescriptorOpenTargetV1 {
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub surface_id: String,
    pub app_id: String,
    pub window_kind_id: String,
    pub role: TrustedBundleOpenRole,
    pub renderer_target: TrustedBundleRendererTarget,
    pub parent_dialect: TrustedBundleParentDialectV1,
    pub grant: TrustedBundleGrantV1,
}

/// 📤️ What `os-hub trusted-catalog open-targets` answers for one package descriptor.
#[derive(Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedCatalogDescriptorOpenTargetsV1 {
    pub schema: &'static str,
    pub targets: Vec<TrustedDescriptorOpenTargetV1>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedBundleFileV1 {
    pub path: String,
    pub byte_length: u64,
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedBundleComponentV1 {
    pub path: String,
    pub byte_length: u64,
    pub sha256: String,
    pub blake3: String,
}

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

/// 🧩️ Where a package record finds its plugin module manifest inside the generation.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedBundlePluginModuleV1 {
    pub path: String,
    pub byte_length: u64,
    pub sha256: String,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedBundlePackageV1 {
    pub plugin_id: String,
    pub package_id: String,
    pub version: String,
    pub role: TrustedBundlePackageRole,
    pub execution_protocol: TrustedBundleExecutionProtocolV1,
    pub dependencies: Vec<TrustedBundleIdentityV1>,
    pub component: TrustedBundleComponentV1,
    pub descriptor: TrustedBundleFileV1,
    pub browser_actor: TrustedBundleBrowserActorV1,
    pub plugin_module: TrustedBundlePluginModuleV1,
    pub native_codecs: Vec<TrustedBundleCodecV1>,
    pub open_targets: Vec<TrustedBundleOpenTargetV1>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedBundleProfileV1 {
    pub id: String,
    pub selected_closure: Vec<TrustedBundleIdentityV1>,
    pub selected_closure_sha256: String,
    /// 🎯️ Every document-open target this generation admits, in the bundle's own order. It is a SET
    /// rather than the single row it was until ticket 26/09/18 slice TC3b: a generation carries one
    /// creatable kind per entry, so a `stdio + gis + note` bundle exposes a GIS map AND a note to
    /// `POST /spaces/{space}/artifact-creations` instead of whichever one the profile named.
    /// `trusted_profile_generation` frames the sorted set, so adding or removing a target rotates
    /// the generation id exactly as swapping the single one used to.
    pub open_targets: Vec<TrustedBundleProfileOpenTargetV1>,
    pub generation_id: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedBundleProfileOpenTargetV1 {
    pub package: TrustedBundleIdentityV1,
    pub target: TrustedBundleOpenTargetV1,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedBundleV1 {
    pub schema_version: u32,
    pub profiles: Vec<TrustedBundleProfileV1>,
    pub packages: Vec<TrustedBundlePackageV1>,
}

/// 📌️ The single durable selection token a hub start-up and every publisher agree on.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedCatalogCurrentPointerV1 {
    pub profile_id: String,
    pub generation_id: String,
    pub bundle_sha256: String,
    pub publication_revision: String,
}

impl TrustedCatalogCurrentPointerV1 {
    pub fn decode(bytes: &[u8]) -> Result<Self, AuthorityError> {
        let pointer: Self = serde_json::from_slice(bytes).map_err(catalog_error)?;
        if pointer.encode()? != bytes || pointer.profile_id.is_empty() || pointer.profile_id.len() > TRUSTED_IDENTITY_MAX_BYTES || pointer.profile_id.chars().any(char::is_control) {
            return Err(catalog("trusted catalog current pointer is not exact canonical metadata"));
        }
        decode_digest(&pointer.generation_id, "trusted generation id")?;
        decode_digest(&pointer.bundle_sha256, "trusted bundle sha256")?;
        publication_revision(&pointer.publication_revision)?;
        Ok(pointer)
    }

    pub fn encode(&self) -> Result<Vec<u8>, AuthorityError> {
        let mut bytes = serde_json::to_vec(self).map_err(catalog_error)?;
        bytes.push(b'\n');
        Ok(bytes)
    }
}

/// 📬️ The closed command a caller submits to replace the private server-owned current selection.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedCatalogPublicationCommandV1 {
    pub schema: String,
    pub request_id: String,
    pub profile_id: String,
    pub generation_id: String,
    pub bundle_sha256: String,
    pub expected_current_sha256: Option<String>,
}

/// 🧾️ The closed receipt a publisher emits, binding the request to its exact resulting pointer.
#[derive(Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedCatalogPublicationReceiptV1 {
    pub schema: &'static str,
    pub request_id: String,
    pub profile_id: String,
    pub generation_id: String,
    pub bundle_sha256: String,
    pub publication_revision: String,
    pub current_sha256: String,
    pub outcome: &'static str,
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
