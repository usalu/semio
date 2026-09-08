//! 🌐️ Private catalog actor locations and source-bound generation framing.

use super::{append_document_open_catalog_field, catalog, decode_digest, AuthorityError, TrustedBundleFileV1, TRUSTED_RELATIVE_PATH_MAX_BYTES};
use directory::os_directory::schema::{DocumentBrowserActorByteLengthV1, DocumentBrowserActorSourceV1, DocumentOpenBrowserActorV1, DOCUMENT_BROWSER_ACTOR_MAX_BYTES};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum TrustedBundleBrowserActorV1 {
    None,
    ClosedBrowserActor {
        schema: String,
        codegen_policy: String,
        path: String,
        #[serde(deserialize_with = "actor_byte_length")]
        byte_length: u64,
        sha256: String,
        source_component_sha256: String,
        source_descriptor_byte_sha256: String,
        policy_sha256: String,
        import_interfaces: Vec<String>,
    },
}

/// 📏️ Applies the domain's semantic whole-number bound to JSON integer, decimal and exponent tokens.
fn actor_byte_length<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    semio_framework::from_dsl_value::<DocumentBrowserActorByteLengthV1>(semio_framework::DslValue::float(f64::deserialize(deserializer)?)).map(|length| length.get()).map_err(serde::de::Error::custom)
}

impl TrustedBundleBrowserActorV1 {
    pub(super) fn identity(&self) -> DocumentOpenBrowserActorV1 {
        match self {
            Self::None => DocumentOpenBrowserActorV1::None,
            Self::ClosedBrowserActor { schema, codegen_policy, sha256, source_component_sha256, source_descriptor_byte_sha256, policy_sha256, import_interfaces, .. } => DocumentOpenBrowserActorV1::ClosedBrowserActor {
                schema: schema.clone(),
                codegen_policy: codegen_policy.clone(),
                sha256: sha256.clone(),
                source_component_sha256: source_component_sha256.clone(),
                source_descriptor_byte_sha256: source_descriptor_byte_sha256.clone(),
                policy_sha256: policy_sha256.clone(),
                import_interfaces: import_interfaces.clone(),
            },
        }
    }

    pub(super) fn file(&self) -> Option<TrustedBundleFileV1> {
        match self {
            Self::None => None,
            Self::ClosedBrowserActor { path, byte_length, sha256, .. } => Some(TrustedBundleFileV1 { path: path.clone(), byte_length: *byte_length, sha256: sha256.clone() }),
        }
    }

    pub(super) fn validate(&self, source: DocumentBrowserActorSourceV1<'_>, renderer: &str) -> Result<(), AuthorityError> {
        self.identity().validate(source, renderer).map_err(|_| catalog("trusted browser actor identity differs from its package or renderer"))?;
        if let Some(file) = self.file() {
            if file.path.len() > TRUSTED_RELATIVE_PATH_MAX_BYTES
                || file.path.chars().any(|value| value.is_control() || value == '\\' || value == ':')
                || file.path.split('/').any(|part| part.is_empty() || part == "." || part == "..")
                || !(1..=DOCUMENT_BROWSER_ACTOR_MAX_BYTES).contains(&file.byte_length)
            {
                return Err(catalog("trusted browser actor location or length is invalid"));
            }
        }
        Ok(())
    }

    pub(super) fn append_generation(&self, output: &mut Vec<u8>) -> Result<(), AuthorityError> {
        match self {
            Self::None => append_document_open_catalog_field(output, b"none"),
            Self::ClosedBrowserActor { schema, codegen_policy, path, byte_length, sha256, source_component_sha256, source_descriptor_byte_sha256, policy_sha256, import_interfaces } => {
                for value in ["closed-browser-actor", schema, codegen_policy, path] {
                    append_document_open_catalog_field(output, value.as_bytes())?;
                }
                output.extend_from_slice(&byte_length.to_be_bytes());
                for value in [sha256, source_component_sha256, source_descriptor_byte_sha256, policy_sha256] {
                    append_document_open_catalog_field(output, &decode_digest(value, "browser actor generation hash")?)?;
                }
                output.extend_from_slice(&u32::try_from(import_interfaces.len()).map_err(|_| catalog("browser actor interface count overflows"))?.to_be_bytes());
                for value in import_interfaces {
                    append_document_open_catalog_field(output, value.as_bytes())?;
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
