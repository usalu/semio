//! 📋️ Schema registry: derive JSON Schema from Rust types and validate at kernel boundaries.

use jsonschema::Validator;
use schemars::{schema_for, JsonSchema};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

pub use semio_framework_os_kernel::StateClass;
pub use semio_framework_schema_derive::ArtifactSchema;

//#region 🔖️Errors
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SchemaError {
    #[error("unknown schema id: {0}")]
    UnknownSchema(String),
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("serialize error: {0}")]
    Serialize(String),
}
//#endregion 🔖️Errors

//#region 🔖️EntityCatalog
include!("🤖️generated.rs");
//#endregion 🔖️EntityCatalog

//#region 🔖️SchemaCatalog
pub struct SchemaCatalog {
    schemas: HashMap<String, Value>,
    validators: HashMap<String, Validator>,
}

impl Default for SchemaCatalog {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaCatalog {
    pub fn new() -> Self {
        Self { schemas: HashMap::new(), validators: HashMap::new() }
    }

    pub fn register<T: JsonSchema>(&mut self, id: &str) -> Result<(), SchemaError> {
        let schema = schema_for!(T);
        let value = serde_json::to_value(schema).map_err(|error| SchemaError::Serialize(error.to_string()))?;
        let validator = Validator::new(&value).map_err(|error| SchemaError::Validation(error.to_string()))?;
        self.schemas.insert(id.to_string(), value);
        self.validators.insert(id.to_string(), validator);
        Ok(())
    }

    pub fn register_json(&mut self, id: &str, schema: Value) -> Result<(), SchemaError> {
        let validator = Validator::new(&schema).map_err(|error| SchemaError::Validation(error.to_string()))?;
        self.schemas.insert(id.to_string(), schema);
        self.validators.insert(id.to_string(), validator);
        Ok(())
    }

    pub fn schema(&self, id: &str) -> Option<&Value> {
        self.schemas.get(id)
    }

    pub fn validate(&self, id: &str, value: &Value) -> Result<(), SchemaError> {
        let validator = self.validators.get(id).ok_or_else(|| SchemaError::UnknownSchema(id.to_string()))?;
        validator.validate(value).map_err(|error| SchemaError::Validation(error.to_string()))
    }
}
//#endregion 🔖️SchemaCatalog

//#region 🔖️GraphQlStatePreamble
/// 🔗 Shared GraphQL `@state` SDL preamble — declared once, never repeated per artifact.
pub const GRAPHQL_STATE_PREAMBLE: &str = "\
enum StateClass { PERSISTENT SHARED_UI LOCAL_UI PREVIEW EFFECT }\n\
directive @state(class: StateClass!) on FIELD_DEFINITION\
";
//#endregion 🔖️GraphQlStatePreamble

//#region 🔖️ArtifactSchemaFields
/// ✨️ Per-artifact field → [`StateClass`] table emitted by [`ArtifactSchema`].
pub trait ArtifactSchemaFields {
    fn artifact_schema_id() -> &'static str;
    fn field_states() -> &'static [(&'static str, StateClass)];
}
//#endregion 🔖️ArtifactSchemaFields

//#region 🔖️ArtifactSchemaDescriptor
/// 🍃 Five handcrafted leaf bodies for one facet (`include_str!` at each artifact's registration site).
#[derive(Clone, Debug)]
pub struct FacetLeaves {
    pub rust: &'static str,
    pub typescript: &'static str,
    pub graphql: &'static str,
    pub json_schema: &'static str,
    pub proto: &'static str,
}

/// 🧬️ Registered descriptor for one artifact's three schema facets.
#[derive(Clone, Debug)]
pub struct ArtifactSchemaDescriptor {
    pub id: &'static str,
    pub artifact: FacetLeaves,
    pub snapshot: FacetLeaves,
    pub diff: FacetLeaves,
}
//#endregion 🔖️ArtifactSchemaDescriptor

//#region 🔖️ArtifactSchemaRegistry
/// 📚 Runtime registry of [`ArtifactSchemaDescriptor`] values — same shape as [`SchemaCatalog`].
pub struct ArtifactSchemaRegistry {
    by_id: HashMap<&'static str, ArtifactSchemaDescriptor>,
}

impl Default for ArtifactSchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtifactSchemaRegistry {
    /// 🏗️ Empty registry.
    pub fn new() -> Self {
        Self { by_id: HashMap::new() }
    }

    /// 📎 Insert or replace a descriptor by id.
    pub fn register(&mut self, descriptor: ArtifactSchemaDescriptor) {
        self.by_id.insert(descriptor.id, descriptor);
    }

    /// 🔎 Lookup by artifact schema id.
    pub fn get(&self, id: &str) -> Option<&ArtifactSchemaDescriptor> {
        self.by_id.get(id)
    }

    /// 🚶 Walk every registered descriptor.
    pub fn iter(&self) -> impl Iterator<Item = &ArtifactSchemaDescriptor> {
        self.by_id.values()
    }
}
//#endregion 🔖️ArtifactSchemaRegistry

//#region 🔖️StateClassKebab
/// 🏷️ Parse the canonical kebab `x-semio-state` string into [`StateClass`].
///
/// Lives here (not a second enum) so JSON Schema leaves can be checked against the kernel enum
/// without inventing a parallel source of truth. The kernel already owns [`StateClass`].
pub fn parse_state_class_kebab(value: &str) -> Option<StateClass> {
    match value {
        "persistent" => Some(StateClass::Persistent),
        "shared-ui" => Some(StateClass::SharedUi),
        "local-ui" => Some(StateClass::LocalUi),
        "preview" => Some(StateClass::Preview),
        "effect" => Some(StateClass::Effect),
        _ => None,
    }
}

/// 🏷️ Canonical kebab spelling of a [`StateClass`] for JSON Schema `x-semio-state`.
pub fn state_class_kebab(class: StateClass) -> &'static str {
    match class {
        StateClass::Persistent => "persistent",
        StateClass::SharedUi => "shared-ui",
        StateClass::LocalUi => "local-ui",
        StateClass::Preview => "preview",
        StateClass::Effect => "effect",
    }
}
//#endregion 🔖️StateClassKebab

#[cfg(test)]
//#region 🔖️Tests
mod tests {
    use super::*;
    use serde_json::json;

    //#region 🔖️SyntheticArtifact
    #[derive(Clone, Debug, PartialEq, ArtifactSchema)]
    #[artifact_schema(id = "s.wave3.synthetic")]
    struct SyntheticArtifact {
        #[state(persistent)]
        schema: String,
        #[state(persistent)]
        label: String,
        #[state(shared_ui)]
        active_id: Option<String>,
    }

    #[derive(Clone, Debug, Default, PartialEq, ArtifactSchema)]
    #[artifact_schema(id = "s.wave3.synthetic")]
    struct SyntheticSnapshot {
        #[state(persistent)]
        schema: String,
        #[state(persistent)]
        label: String,
    }

    const SYNTHETIC_SNAPSHOT_JSON_SCHEMA: &str = r#"{
  "$id": "https://semio.tech/schema/s/wave3/synthetic/snapshot.json",
  "title": "SyntheticSnapshot",
  "type": "object",
  "additionalProperties": false,
  "required": ["schema", "label"],
  "properties": {
    "schema": { "type": "string", "x-semio-state": "persistent" },
    "label": { "type": "string", "x-semio-state": "persistent" }
  }
}"#;

    fn synthetic_descriptor() -> ArtifactSchemaDescriptor {
        let empty = FacetLeaves {
            rust: "",
            typescript: "",
            graphql: "",
            json_schema: "",
            proto: "",
        };
        ArtifactSchemaDescriptor {
            id: "s.wave3.synthetic",
            artifact: empty.clone(),
            snapshot: FacetLeaves {
                rust: "",
                typescript: "",
                graphql: "",
                json_schema: SYNTHETIC_SNAPSHOT_JSON_SCHEMA,
                proto: "",
            },
            diff: empty,
        }
    }

    fn expected_snapshot_title(id: &str) -> String {
        let key = id.rsplit('.').next().unwrap_or(id);
        let mut chars = key.chars();
        let titled = match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        };
        format!("{titled}Snapshot")
    }
    //#endregion 🔖️SyntheticArtifact

    #[test]
    fn registry_descriptors_carry_valid_snapshot_state_and_match_field_states() {
        let mut registry = ArtifactSchemaRegistry::new();
        registry.register(synthetic_descriptor());

        let mut walked = 0usize;
        for descriptor in registry.iter() {
            walked += 1;
            let schema: Value = serde_json::from_str(descriptor.snapshot.json_schema)
                .unwrap_or_else(|error| panic!("{}: snapshot json_schema parse: {error}", descriptor.id));
            let title = schema.get("title").and_then(Value::as_str).unwrap_or("");
            assert_eq!(
                title,
                expected_snapshot_title(descriptor.id),
                "{}: snapshot title must be XSnapshot for id",
                descriptor.id
            );

            let properties = schema
                .get("properties")
                .and_then(Value::as_object)
                .unwrap_or_else(|| panic!("{}: snapshot properties object required", descriptor.id));

            let mut json_states = Vec::new();
            for (name, prop) in properties {
                let raw = prop
                    .get("x-semio-state")
                    .and_then(Value::as_str)
                    .unwrap_or_else(|| panic!("{}: property `{name}` missing x-semio-state", descriptor.id));
                let class = parse_state_class_kebab(raw)
                    .unwrap_or_else(|| panic!("{}: property `{name}` has invalid x-semio-state `{raw}`", descriptor.id));
                json_states.push((name.clone(), class));
            }
            json_states.sort_by(|a, b| a.0.cmp(&b.0));

            let mut derived: Vec<(String, StateClass)> = SyntheticSnapshot::field_states()
                .iter()
                .map(|(name, class)| ((*name).to_string(), *class))
                .collect();
            derived.sort_by(|a, b| a.0.cmp(&b.0));
            assert_eq!(
                derived, json_states,
                "{}: field_states() must agree with snapshot JSON x-semio-state",
                descriptor.id
            );
            assert_eq!(SyntheticSnapshot::artifact_schema_id(), descriptor.id);
            assert_eq!(SyntheticArtifact::artifact_schema_id(), descriptor.id);
        }
        assert_eq!(walked, 1, "registry must be walked for the synthetic descriptor");
        assert!(registry.get("s.wave3.synthetic").is_some());
    }

    #[test]
    fn graphql_state_preamble_matches_normative_sdl() {
        assert!(GRAPHQL_STATE_PREAMBLE.contains("enum StateClass { PERSISTENT SHARED_UI LOCAL_UI PREVIEW EFFECT }"));
        assert!(GRAPHQL_STATE_PREAMBLE.contains("directive @state(class: StateClass!) on FIELD_DEFINITION"));
    }

    #[test]
    fn schema_catalog_still_registers_json() {
        let mut catalog = SchemaCatalog::new();
        catalog
            .register_json(
                "probe",
                json!({
                    "type": "object",
                    "properties": { "n": { "type": "integer" } }
                }),
            )
            .expect("register");
        catalog.validate("probe", &json!({ "n": 1 })).expect("validate");
    }
}
//#endregion 🔖️Tests
