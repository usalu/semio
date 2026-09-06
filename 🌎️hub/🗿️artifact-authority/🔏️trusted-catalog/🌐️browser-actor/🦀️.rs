//! 🌐️ Private catalog actor locations and source-bound generation framing.

use super::{append_document_open_catalog_field, catalog, decode_digest, AuthorityError, BundleFile, TRUSTED_RELATIVE_PATH_MAX_BYTES};
use directory::os_directory::schema::{DocumentBrowserActorByteLengthV1, DocumentBrowserActorSourceV1, DocumentOpenBrowserActorV1, DOCUMENT_BROWSER_ACTOR_MAX_BYTES};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub(super) enum BundleBrowserActor {
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

impl BundleBrowserActor {
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

    pub(super) fn file(&self) -> Option<BundleFile> {
        match self {
            Self::None => None,
            Self::ClosedBrowserActor { path, byte_length, sha256, .. } => Some(BundleFile { path: path.clone(), byte_length: *byte_length, sha256: sha256.clone() }),
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
mod tests {
    use super::*;
    use semio_framework_hash::Sha256;

    #[test]
    fn trusted_browser_actor_metadata_and_generation_match_neutral_corpus() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️fixtures/🌐️browser-actor/🔣️.json")).unwrap();
        let source = DocumentBrowserActorSourceV1 { component_sha256: fixture["closed"]["sourceComponentSha256"].as_str().unwrap(), descriptor_byte_sha256: fixture["closed"]["sourceDescriptorByteSha256"].as_str().unwrap() };
        for law in fixture["cases"].as_array().unwrap() {
            let mut value = if law["kind"] == "none" { serde_json::json!({"kind":"none"}) } else { fixture["closed"].clone() };
            for (key, field) in law["set"].as_object().unwrap() {
                value[key] = field.clone();
            }
            if let Some(remove) = law["remove"].as_str() {
                value.as_object_mut().unwrap().remove(remove);
            }
            let parsed = serde_json::from_value::<BundleBrowserActor>(value);
            let result = parsed.map_err(|_| catalog("actor shape")).and_then(|actor| {
                actor.validate(source, law["renderer"].as_str().unwrap())?;
                Ok(actor)
            });
            assert_eq!(result.is_ok(), law["accepted"].as_bool().unwrap(), "{}", law["id"]);
            if let Ok(actor) = result {
                let public = directory::os_pack::json::to_json_string(&actor.identity());
                assert!(!public.contains("path") && !public.contains("byteLength"));
            }
        }
        for (actor, expected) in [(serde_json::from_value::<BundleBrowserActor>(fixture["closed"].clone()).unwrap(), &fixture["encodingSha256"]), (BundleBrowserActor::None, &fixture["noneEncodingSha256"])] {
            let mut bytes = Vec::new();
            actor.append_generation(&mut bytes).unwrap();
            assert_eq!(directory::os_directory::hex_lower(&Sha256::digest(&bytes)), expected.as_str().unwrap());
        }
        let raw = serde_json::to_string(&fixture["closed"]).unwrap();
        assert_eq!(raw.matches("\"byteLength\":3").count(), 1);
        for law in fixture["rawLengths"].as_array().unwrap() {
            let candidate = raw.replace("\"byteLength\":3", &format!("\"byteLength\":{}", law["token"].as_str().unwrap()));
            let admitted = serde_json::from_str::<BundleBrowserActor>(&candidate).is_ok_and(|actor| actor.validate(source, "wasm").is_ok());
            assert_eq!(admitted, law["accepted"].as_bool().unwrap(), "raw actor length {}", law["token"]);
        }
    }
}
