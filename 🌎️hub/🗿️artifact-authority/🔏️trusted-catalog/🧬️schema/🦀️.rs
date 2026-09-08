//! 🧬️ Sole owner of the trusted-catalog publication command, receipt and current-pointer contract.
//!
//! The normative shape lives in the sibling [`🔣️.json`](🔣️.json) draft-07 module as
//! `TrustedCatalogPublicationCommandV1`, `TrustedCatalogPublicationReceiptV1` and
//! `TrustedCatalogCurrentPointerV1`; these types are its Rust projection.

use super::{catalog, catalog_error, decode_digest, AuthorityError, TRUSTED_IDENTITY_MAX_BYTES};
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
