//! 🧬️ Sole owner of the trusted-catalog publication command, receipt and current-pointer contract.
//!
//! The normative shape lives in the sibling [`🔣️.json`](🔣️.json) draft-07 module as
//! `TrustedCatalogPublicationCommandV1`, `TrustedCatalogPublicationReceiptV1` and
//! `TrustedCatalogCurrentPointerV1`; these types are its Rust projection.

use super::{catalog, catalog_error, decode_digest, AuthorityError, TRUSTED_IDENTITY_MAX_BYTES};
use semio_framework::PackageRole;
use serde::{Deserialize, Serialize};

/// 🧬️ The draft-07 module every implementation of this contract is projected from.
pub const TRUSTED_CATALOG_SCHEMA_JSON: &str = include_str!("🔣️.json");
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

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TrustedBundleOpenRole {
    Viewer,
    Editor,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedBundleGrantV1 {
    pub read: bool,
    pub write: bool,
    pub observe: bool,
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
    pub native_codecs: Vec<TrustedBundleCodecV1>,
    pub open_targets: Vec<TrustedBundleOpenTargetV1>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TrustedBundleProfileV1 {
    pub id: String,
    pub selected_closure: Vec<TrustedBundleIdentityV1>,
    pub selected_closure_sha256: String,
    pub open_target: TrustedBundleProfileOpenTargetV1,
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
