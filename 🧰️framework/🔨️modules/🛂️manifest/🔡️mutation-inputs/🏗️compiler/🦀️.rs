//! 🏗️ Canonical mutation schemas compile into immutable native input JSON before any interactive source is created.
pub use semio_framework_manifest_mutation_inputs::{InputSchemaError, InputSchemaErrorCode};
use semio_framework_manifest_mutation_inputs::{mutation_input_defs, FRAMEWORK_INPUT_SCHEMA_DOCUMENTS};
use semio_framework_hash::Sha256;
use semio_framework_value::{DslValue, ToValue};
use std::collections::BTreeMap;
use std::sync::OnceLock;

/// 🧾️ Compiler output keeps declaration data and exact source fingerprints independent of the eventual native owner.
#[derive(Debug)]
pub struct InputDefinitionCompilation {
    pub schema_sha256: [u8; 32],
    pub document_sha256: Vec<[u8; 32]>,
    pub framework_document_sha256: Vec<[u8; 32]>,
    pub definitions_json: Result<String, InputSchemaError>,
    pub seed_definitions_json: Result<String, InputSchemaError>,
}
fn malformed(detail: impl Into<String>) -> InputSchemaError { InputSchemaError { code: InputSchemaErrorCode::Malformed, pointer: String::new(), detail: detail.into() } }
fn document(raw: &str) -> Result<(String, DslValue), InputSchemaError> {
    let value = semio_framework_pack_json::parse(raw, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| malformed(format!("input schema document is invalid JSON: {error}")))?;
    let value = semio_framework_pack_json::to_dsl_value(&value);
    let id = value.get("$id").and_then(DslValue::as_str).filter(|id| !id.is_empty()).ok_or_else(|| malformed("input schema document has no $id"))?.to_string();
    Ok((id, value))
}
fn framework_documents() -> Result<&'static BTreeMap<String, DslValue>, InputSchemaError> {
    static DOCUMENTS: OnceLock<Result<BTreeMap<String, DslValue>, InputSchemaError>> = OnceLock::new();
    DOCUMENTS.get_or_init(|| FRAMEWORK_INPUT_SCHEMA_DOCUMENTS.iter().map(|raw| document(raw)).collect()).as_ref().map_err(Clone::clone)
}
fn declarations(schema: &str, documents: &[&str]) -> Result<(String, String), InputSchemaError> {
    let mut catalog = BTreeMap::new();
    for raw in documents {
        let (id, value) = document(raw)?;
        match catalog.entry(id) {
            std::collections::btree_map::Entry::Vacant(entry) => { entry.insert(value); },
            std::collections::btree_map::Entry::Occupied(entry) if entry.get() != &value => return Err(malformed(format!("input schema document id is duplicated: {}", entry.key()))),
            _ => {},
        }
    }
    let framework = framework_documents()?;
    let inputs = mutation_input_defs(schema, &|id: &str| catalog.get(id).or_else(|| framework.get(id)).cloned())?;
    let seeds = semio_framework_manifest_arg_schema::action_arg_shape_seeds(&inputs);
    semio_framework_manifest_arg_schema::validate_action_arg_shape_seeds(&inputs, &seeds).map_err(malformed)?;
    Ok((semio_framework_pack_json::to_json_string(&inputs.to_value()), semio_framework_pack_json::to_json_string(&seeds.to_value())))
}

/// 🧬️ The build caller supplies the exact native payload schema and transitive document roster; runtime never calls this compiler.
pub fn compile_input_definitions(schema: &str, documents: &[&str]) -> InputDefinitionCompilation {
    static FRAMEWORK_DIGESTS: OnceLock<Vec<[u8; 32]>> = OnceLock::new();
    let (definitions_json, seed_definitions_json) = match declarations(schema, documents) { Ok((definitions, seeds)) => (Ok(definitions), Ok(seeds)), Err(error) => (Err(error.clone()), Err(error)) };
    InputDefinitionCompilation {
        schema_sha256: Sha256::digest(schema.as_bytes()),
        document_sha256: documents.iter().map(|document| Sha256::digest(document.as_bytes())).collect(),
        framework_document_sha256: FRAMEWORK_DIGESTS.get_or_init(|| FRAMEWORK_INPUT_SCHEMA_DOCUMENTS.iter().map(|document| Sha256::digest(document.as_bytes())).collect()).clone(),
        definitions_json,
        seed_definitions_json,
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
