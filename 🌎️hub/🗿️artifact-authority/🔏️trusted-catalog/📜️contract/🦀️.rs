//! 📜️ Trusted-catalog contract: the bundle schema, the retained-file roots, the browser-actor and plugin-module sources and the bounded helpers every trusted-catalog crate shares.

use semio_hub_artifact_authority::{AuthorityError, AuthorityProgress, AuthorityProgressStage, OperationContext};
use semio_framework_hash::{Hasher, Sha256};
use crate::schema::{TrustedBundleFileV1, TrustedBundleIdentityV1};
use semio_hub_artifact_authority::adapters::bounded_message;

#[path = "../🧬️schema/🦀️.rs"]
pub mod schema;

#[path = "../🌐️browser-actor/🦀️.rs"]
mod browser_actor;

#[path = "../🧩️plugin-module/🦀️.rs"]
pub mod plugin_module;

#[path = "../🛡️opened-root/🦀️.rs"]
mod opened_root;

/// 🧯️ Maximum accepted serialized bundle bytes.
pub const TRUSTED_BUNDLE_MAX_BYTES: u64 = 4 * 1024 * 1024;

/// 🧯️ Maximum accepted committed package-descriptor bytes: the schema-declared execution-target
/// descriptor bound the browser enforces on the same bytes.
pub const TRUSTED_DESCRIPTOR_MAX_BYTES: u64 = directory::os_directory::DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES;

/// 🧮️ Maximum logical owned storage admitted before descriptor schema/canonical projection.
pub const TRUSTED_DESCRIPTOR_MAX_MATERIALIZATION: u64 = 32 * 1024 * 1024;

/// 🧯️ Maximum accepted bytes for one component: the schema-declared execution-target component bound
/// the browser enforces on the same bytes. A closure has no byte total: components and actors are
/// retained by identity on disk ([`TrustedCatalogAsset`]), so the catalog is bounded by storage and the
/// hub's memory by the assets in use.
pub const TRUSTED_COMPONENT_MAX_BYTES: u64 = directory::os_directory::DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES;

/// 🧯️ Maximum UTF-8 bytes retained for one identity or version field.
pub const TRUSTED_IDENTITY_MAX_BYTES: usize = 256;

/// 🧯️ Maximum UTF-8 bytes accepted for one bundle-relative path.
pub const TRUSTED_RELATIVE_PATH_MAX_BYTES: usize = 1024;

/// 🧯️ Maximum direct dependencies accepted for one package.
pub const TRUSTED_PACKAGE_MAX_DEPENDENCIES: usize = 256;

/// 🧯️ Maximum selectable profiles in one bundle.
pub const TRUSTED_BUNDLE_MAX_PROFILES: usize = 256;

/// 🧯 Maximum immutable document-open selections retained by one catalog generation.
pub const TRUSTED_CATALOG_MAX_OPEN_TARGETS: usize = 1024;

pub fn catalog(message: &str) -> AuthorityError {
    AuthorityError::Catalog(bounded_message(message))
}

pub fn catalog_error(error: impl std::fmt::Display) -> AuthorityError {
    AuthorityError::Catalog(bounded_message(error))
}

pub fn valid_identity(value: &str) -> bool {
    !value.is_empty() && value.len() <= TRUSTED_IDENTITY_MAX_BYTES && value.trim() == value
}

pub fn valid_open_identity(value: &str) -> bool {
    valid_identity(value) && !value.chars().any(char::is_control)
}

pub fn append_document_open_catalog_field(output: &mut Vec<u8>, value: &[u8]) -> Result<(), AuthorityError> {
    let length = u64::try_from(value.len()).map_err(catalog_error)?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value);
    Ok(())
}

pub fn valid_package_id(value: &str) -> bool {
    let Some(name) = value.strip_prefix("semio:") else { return false };
    !name.is_empty() && value.len() <= TRUSTED_IDENTITY_MAX_BYTES && !name.starts_with('-') && !name.ends_with('-') && !name.contains("--") && name.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

pub fn ensure_count(actual: usize, maximum: usize, resource: &'static str) -> Result<(), AuthorityError> {
    if actual > maximum {
        return Err(AuthorityError::ResourceLimit(resource));
    }
    Ok(())
}

pub fn validate_identity(identity: &TrustedBundleIdentityV1) -> Result<(), AuthorityError> {
    if !valid_identity(&identity.plugin_id) || !valid_package_id(&identity.package_id) || !valid_identity(&identity.version) {
        return Err(catalog("trusted package identity is empty, padded, or oversized"));
    }
    Ok(())
}

pub fn validate_file(file: &TrustedBundleFileV1, maximum: u64) -> Result<(), AuthorityError> {
    if file.path.is_empty() || file.path.len() > TRUSTED_RELATIVE_PATH_MAX_BYTES || file.byte_length == 0 || file.byte_length > maximum {
        return Err(catalog("trusted file record is empty or exceeds its fixed boundary"));
    }
    decode_digest(&file.sha256, "sha256")?;
    Ok(())
}

pub async fn dual_hash(bytes: &[u8], context: &OperationContext<'_>) -> Result<([u8; 32], [u8; 32]), AuthorityError> {
    let mut sha256 = Sha256::new();
    let mut blake3 = Hasher::new();
    let total_units = u64::try_from(bytes.len().max(1)).map_err(catalog_error)?;
    let mut completed_units = 0u64;
    let mut last_report_ms = 0u64;
    for chunk in bytes.chunks(64 * 1024) {
        context.checkpoint()?;
        sha256.update(chunk);
        blake3.update(chunk);
        completed_units = completed_units.saturating_add(u64::try_from(chunk.len()).map_err(catalog_error)?);
        let now_ms = context.now_ms();
        if now_ms.saturating_sub(last_report_ms) >= 1_000 {
            last_report_ms = now_ms;
            context.report(AuthorityProgress {
                stage: AuthorityProgressStage::CatalogLoading,
                completed_units: completed_units.min(total_units),
                total_units,
            })?;
        }
        semio_framework_async::yield_once().await;
    }
    context.checkpoint()?;
    Ok((sha256.finalize(), *blake3.finalize().as_bytes()))
}

pub async fn sha256(bytes: &[u8], context: &OperationContext<'_>) -> Result<[u8; 32], AuthorityError> {
    let mut hash = Sha256::new();
    let total_units = u64::try_from(bytes.len().max(1)).map_err(catalog_error)?;
    let mut completed_units = 0u64;
    let mut last_report_ms = 0u64;
    for chunk in bytes.chunks(64 * 1024) {
        context.checkpoint()?;
        hash.update(chunk);
        completed_units = completed_units.saturating_add(u64::try_from(chunk.len()).map_err(catalog_error)?);
        let now_ms = context.now_ms();
        if now_ms.saturating_sub(last_report_ms) >= 1_000 {
            last_report_ms = now_ms;
            context.report(AuthorityProgress {
                stage: AuthorityProgressStage::CatalogLoading,
                completed_units: completed_units.min(total_units),
                total_units,
            })?;
        }
        semio_framework_async::yield_once().await;
    }
    context.checkpoint()?;
    Ok(hash.finalize())
}

pub fn verify_length(expected: u64, actual: usize) -> Result<(), AuthorityError> {
    if u64::try_from(actual).map_err(catalog_error)? != expected {
        return Err(catalog("trusted file byte length does not match its trust record"));
    }
    Ok(())
}

pub fn verify_digest(expected: &str, actual: [u8; 32], label: &str) -> Result<(), AuthorityError> {
    if decode_digest(expected, label)? != actual {
        return Err(catalog("trusted file digest does not match its trust record"));
    }
    Ok(())
}

pub fn decode_digest(value: &str, label: &str) -> Result<[u8; 32], AuthorityError> {
    if value.len() != 64 || value.bytes().any(|byte| !matches!(byte, b'0'..=b'9' | b'a'..=b'f')) {
        return Err(catalog(&format!("{label} is not canonical lowercase hexadecimal")));
    }
    let mut result = [0; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let nibble = |byte: u8| if byte <= b'9' { byte - b'0' } else { byte - b'a' + 10 };
        result[index] = (nibble(pair[0]) << 4) | nibble(pair[1]);
    }
    Ok(result)
}

pub use browser_actor::TrustedBundleBrowserActorV1;
pub use opened_root::{TRUSTED_READ_CHUNK_BYTES, TrustedCatalogDataRoot, TrustedCatalogGenerationRoot, TrustedCatalogRelativePathV1, TrustedPublicationSync};
#[cfg(all(any(test, feature = "testkit"),unix))]
pub use opened_root::create_fifo_fixture;
pub use opened_root::platform::{open_lock, open_server_owned};
