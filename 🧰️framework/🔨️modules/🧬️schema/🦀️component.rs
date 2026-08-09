//! 📋️ Schema registry: derive JSON Schema from Rust types and validate at kernel boundaries.

use jsonschema::Validator;
use schemars::{schema_for, JsonSchema};
use serde_json::Value;
use std::collections::HashMap;
#[cfg(all(test, feature = "catalog-integration"))]
use std::collections::BTreeSet;
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

    /// 📥 Stores a handcrafted JSON Schema document without compiling a validator (catalog registration of normative leaves).
    pub fn load_json(&mut self, id: &str, schema: Value) {
        self.schemas.insert(id.to_string(), schema);
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

    /// 🔢 Count of registered artifact schema ids.
    pub fn len(&self) -> usize {
        self.by_id.len()
    }
}
//#endregion 🔖️ArtifactSchemaRegistry

//#region 🔖️GlobalArtifactSchemaCatalog
use semio_framework_os_kernel::{
    register_kernel_app_schema_descriptor, register_kernel_artifact_schema_descriptor, with_kernel_app_schema_catalog,
    with_kernel_artifact_schema_catalog, KernelAppSchemaDescriptor, KernelArtifactSchemaDescriptor, KernelFacetLeaves,
};

fn facet_leaves_to_kernel(leaves: FacetLeaves) -> KernelFacetLeaves {
    KernelFacetLeaves {
        rust: leaves.rust,
        typescript: leaves.typescript,
        graphql: leaves.graphql,
        json_schema: leaves.json_schema,
        proto: leaves.proto,
    }
}

fn facet_leaves_from_kernel(leaves: &KernelFacetLeaves) -> FacetLeaves {
    FacetLeaves {
        rust: leaves.rust,
        typescript: leaves.typescript,
        graphql: leaves.graphql,
        json_schema: leaves.json_schema,
        proto: leaves.proto,
    }
}

fn descriptor_to_kernel(descriptor: ArtifactSchemaDescriptor) -> KernelArtifactSchemaDescriptor {
    KernelArtifactSchemaDescriptor {
        id: descriptor.id,
        artifact: facet_leaves_to_kernel(descriptor.artifact),
        snapshot: facet_leaves_to_kernel(descriptor.snapshot),
        diff: facet_leaves_to_kernel(descriptor.diff),
    }
}

fn descriptor_from_kernel(kernel: &KernelArtifactSchemaDescriptor) -> ArtifactSchemaDescriptor {
    ArtifactSchemaDescriptor {
        id: kernel.id,
        artifact: facet_leaves_from_kernel(&kernel.artifact),
        snapshot: facet_leaves_from_kernel(&kernel.snapshot),
        diff: facet_leaves_from_kernel(&kernel.diff),
    }
}

fn parse_normative_json_leaf(descriptor_id: &str, facet: &str, body: &str) -> Value {
    serde_json::from_str(body)
        .unwrap_or_else(|error| panic!("{descriptor_id}: {facet} json_schema parse: {error}"))
}

fn graphql_leaf_with_preamble(body: &str) -> String {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return GRAPHQL_STATE_PREAMBLE.to_string();
    }
    format!("{}\n\n{trimmed}", GRAPHQL_STATE_PREAMBLE)
}

/// 📎 Registers one artifact's handcrafted descriptor into the OS-wide catalog (kernel descriptors + normative JSON + GraphQL SDL).
pub fn register_artifact_schema_descriptor(descriptor: ArtifactSchemaDescriptor) {
    register_kernel_artifact_schema_descriptor(descriptor_to_kernel(descriptor));
}

/// 🔎 Whether `id` is present in the OS-wide descriptor registry.
pub fn artifact_schema_descriptor_registered(id: &str) -> bool {
    semio_framework_os_kernel::kernel_artifact_schema_descriptor_registered(id)
}

/// 📚 Invokes `visit` with the OS-wide [`ArtifactSchemaRegistry`] snapshot.
pub fn with_artifact_schema_registry<R>(visit: impl FnOnce(&ArtifactSchemaRegistry) -> R) -> R {
    let mut registry = ArtifactSchemaRegistry::new();
    with_kernel_artifact_schema_catalog(|entries| {
        for entry in entries {
            registry.register(descriptor_from_kernel(entry));
        }
    });
    visit(&registry)
}

/// 🔣 Invokes `visit` with a [`SchemaCatalog`] of normative artifact JSON leaves.
pub fn with_json_schema_catalog<R>(visit: impl FnOnce(&SchemaCatalog) -> R) -> R {
    let mut catalog = SchemaCatalog::new();
    with_kernel_artifact_schema_catalog(|entries| {
        for entry in entries {
            catalog.load_json(
                entry.id,
                parse_normative_json_leaf(entry.id, "artifact", entry.artifact.json_schema),
            );
        }
    });
    visit(&catalog)
}

/// 🔗 Returns composed GraphQL SDL (shared `@state` preamble + facet leaf) for a catalog key (`id`, `{id}.snapshot`, `{id}.diff`).
pub fn artifact_schema_graphql_sdl(key: &str) -> Option<String> {
    with_kernel_artifact_schema_catalog(|entries| {
        for entry in entries {
            if key == entry.id {
                return Some(graphql_leaf_with_preamble(entry.artifact.graphql));
            }
            let snapshot_key = format!("{}.snapshot", entry.id);
            if key == snapshot_key {
                return Some(graphql_leaf_with_preamble(entry.snapshot.graphql));
            }
            let diff_key = format!("{}.diff", entry.id);
            if key == diff_key {
                return Some(graphql_leaf_with_preamble(entry.diff.graphql));
            }
        }
        None
    })
}
//#endregion 🔖️GlobalArtifactSchemaCatalog

//#region 🔖️AppSchemaDescriptor
/// 🧬️ Registered descriptor for one app owner's config + presence schema facets.
#[derive(Clone, Debug)]
pub struct AppSchemaDescriptor {
    pub id: &'static str,
    pub config: FacetLeaves,
    pub presence: FacetLeaves,
}
//#endregion 🔖️AppSchemaDescriptor

//#region 🔖️AppSchemaRegistry
/// 📚 Runtime registry of [`AppSchemaDescriptor`] values — app twin of [`ArtifactSchemaRegistry`].
pub struct AppSchemaRegistry {
    by_id: HashMap<&'static str, AppSchemaDescriptor>,
}

impl Default for AppSchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AppSchemaRegistry {
    /// 🏗️ Empty registry.
    pub fn new() -> Self {
        Self { by_id: HashMap::new() }
    }

    /// 📎 Insert or replace a descriptor by owner id.
    pub fn register(&mut self, descriptor: AppSchemaDescriptor) {
        self.by_id.insert(descriptor.id, descriptor);
    }

    /// 🔎 Lookup by app schema owner id.
    pub fn get(&self, id: &str) -> Option<&AppSchemaDescriptor> {
        self.by_id.get(id)
    }

    /// 🚶 Walk every registered descriptor.
    pub fn iter(&self) -> impl Iterator<Item = &AppSchemaDescriptor> {
        self.by_id.values()
    }

    /// 🔢 Count of registered app schema owner ids.
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    /// 📭 Whether no owners are registered yet (A6 fills the catalog).
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }
}
//#endregion 🔖️AppSchemaRegistry

//#region 🔖️GlobalAppSchemaCatalog
fn app_descriptor_to_kernel(descriptor: AppSchemaDescriptor) -> KernelAppSchemaDescriptor {
    KernelAppSchemaDescriptor {
        id: descriptor.id,
        config: facet_leaves_to_kernel(descriptor.config),
        presence: facet_leaves_to_kernel(descriptor.presence),
    }
}

fn app_descriptor_from_kernel(kernel: &KernelAppSchemaDescriptor) -> AppSchemaDescriptor {
    AppSchemaDescriptor {
        id: kernel.id,
        config: facet_leaves_from_kernel(&kernel.config),
        presence: facet_leaves_from_kernel(&kernel.presence),
    }
}

/// 📎 Registers one app owner's handcrafted descriptor into the OS-wide catalog (kernel descriptors + normative JSON + GraphQL SDL).
pub fn register_app_schema_descriptor(descriptor: AppSchemaDescriptor) {
    register_kernel_app_schema_descriptor(app_descriptor_to_kernel(descriptor));
}

/// 🔎 Whether `id` is present in the OS-wide app descriptor registry.
pub fn app_schema_descriptor_registered(id: &str) -> bool {
    semio_framework_os_kernel::kernel_app_schema_descriptor_registered(id)
}

/// 📚 Invokes `visit` with the OS-wide [`AppSchemaRegistry`] snapshot.
pub fn with_app_schema_registry<R>(visit: impl FnOnce(&AppSchemaRegistry) -> R) -> R {
    let mut registry = AppSchemaRegistry::new();
    with_kernel_app_schema_catalog(|entries| {
        for entry in entries {
            registry.register(app_descriptor_from_kernel(entry));
        }
    });
    visit(&registry)
}

/// 🔣 Invokes `visit` with a [`SchemaCatalog`] of normative app config JSON leaves.
pub fn with_app_json_schema_catalog<R>(visit: impl FnOnce(&SchemaCatalog) -> R) -> R {
    let mut catalog = SchemaCatalog::new();
    with_kernel_app_schema_catalog(|entries| {
        for entry in entries {
            catalog.load_json(
                entry.id,
                parse_normative_json_leaf(entry.id, "config", entry.config.json_schema),
            );
            catalog.load_json(
                &format!("{}.presence", entry.id),
                parse_normative_json_leaf(entry.id, "presence", entry.presence.json_schema),
            );
        }
    });
    visit(&catalog)
}

/// 🔗 Returns composed GraphQL SDL (shared `@state` preamble + facet leaf) for an app catalog key (`id`, `{id}.presence`).
pub fn app_schema_graphql_sdl(key: &str) -> Option<String> {
    with_kernel_app_schema_catalog(|entries| {
        for entry in entries {
            if key == entry.id {
                return Some(graphql_leaf_with_preamble(entry.config.graphql));
            }
            let presence_key = format!("{}.presence", entry.id);
            if key == presence_key {
                return Some(graphql_leaf_with_preamble(entry.presence.graphql));
            }
        }
        None
    })
}
//#endregion 🔖️GlobalAppSchemaCatalog

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

    //#region 🔖️CatalogIntegration
    #[cfg(feature = "catalog-integration")]
    fn register_all_plugin_artifact_schema_descriptors() {
        semio_s_plugin_animate::artifacts::present::engine::register_artifact_schema();
        semio_s_plugin_architect::artifacts::program::engine::register_artifact_schema();
        semio_s_plugin_block::artifacts::block2d::engine::register_artifact_schema();
        semio_s_plugin_block::artifacts::block3d::engine::register_artifact_schema();
        semio_s_plugin_block::artifacts::block5d::engine::register_artifact_schema();
        semio_s_plugin_cad::artifacts::cad::engine::register_artifact_schema();
        semio_s_plugin_dag::artifacts::dag::engine::register_artifact_schema();
        semio_s_plugin_demonstrator::artifacts::playground::engine::register_artifact_schema();
        semio_s_plugin_draw::artifacts::draw::engine::register_artifact_schema();
        semio_s_plugin_energy::artifacts::model::engine::register_artifact_schema();
        semio_s_plugin_fem::artifacts::fem2d::engine::register_artifact_schema();
        semio_s_plugin_fem::artifacts::fem3d::engine::register_artifact_schema();
        semio_s_plugin_flow::artifacts::flow::engine::register_artifact_schema();
        semio_s_plugin_forms::artifacts::forms::engine::register_artifact_schema();
        semio_s_plugin_gis::artifacts::gisterrain::engine::register_artifact_schema();
        semio_s_plugin_gis::artifacts::gismap::engine::register_artifact_schema();
        semio_s_plugin_imperative::artifacts::imperative::engine::register_artifact_schema();
        semio_s_plugin_layout::artifacts::layout::engine::register_artifact_schema();
        semio_s_plugin_lowpoly::artifacts::lowpoly::engine::register_artifact_schema();
        semio_s_plugin_mathematical::artifacts::mathematical::engine::register_artifact_schema();
        semio_s_plugin_note::artifacts::note::engine::register_artifact_schema();
        semio_s_plugin_playbook::artifacts::playbook::engine::register_artifact_schema();
        semio_s_plugin_procedural::artifacts::procedural2d::engine::register_artifact_schema();
        semio_s_plugin_procedural::artifacts::procedural3d::engine::register_artifact_schema();
        semio_s_plugin_process::artifacts::process3d::engine::register_artifact_schema();
        semio_s_plugin_puzzle::artifacts::puzzle2d::engine::register_artifact_schemas();
        semio_s_plugin_raster::artifacts::raster::engine::register_artifact_schema();
        semio_s_plugin_reasoning_mindmap::artifacts::wires::engine::register_artifact_schema();
        semio_s_plugin_remodel::artifacts::remodel::engine::register_artifact_schema();
        semio_s_plugin_sequence::artifacts::sequence::engine::register_artifact_schema();
        semio_s_plugin_shooting::artifacts::shooting::engine::register_artifact_schema();
        semio_s_plugin_sourcing::artifacts::curate::engine::register_artifact_schema();
        semio_s_plugin_space::artifacts::home::engine::register_artifact_schema();
        semio_s_plugin_trinity::artifacts::jack::engine::register_artifact_schema();
        semio_s_plugin_trinity::artifacts::rewrite::engine::register_artifact_schema();
        semio_s_plugin_vcs::artifacts::vcs::engine::register_artifact_schema();
        semio_s_plugin_writer::artifacts::writer::engine::register_artifact_schema();
        semio_s_plugin_norm::artifacts::din4108::engine::register_artifact_schema();
        semio_s_plugin_norm::artifacts::din16798::engine::register_artifact_schema();
        semio_s_plugin_norm::artifacts::din18599::engine::register_artifact_schema();
        semio_s_plugin_norm::artifacts::en1990::engine::register_artifact_schema();
        semio_s_plugin_norm::artifacts::en1991::engine::register_artifact_schema();
        semio_s_plugin_norm::artifacts::en1992::engine::register_artifact_schema();
        semio_s_plugin_norm::artifacts::en1993::engine::register_artifact_schema();
        semio_s_plugin_norm::artifacts::en1994::engine::register_artifact_schema();
        semio_s_plugin_norm::artifacts::en1995::engine::register_artifact_schema();
        semio_s_plugin_norm::artifacts::en1996::engine::register_artifact_schema();
        semio_s_plugin_norm::artifacts::en1997::engine::register_artifact_schema();
        semio_s_plugin_norm::artifacts::en1998::engine::register_artifact_schema();
        semio_s_plugin_norm::artifacts::en1999::engine::register_artifact_schema();
        semio_s_plugin_norm::artifacts::iso16757::engine::register_artifact_schema();
        semio_s_plugin_norm::artifacts::vdi3805::engine::register_artifact_schema();
    }

    #[cfg(feature = "catalog-integration")]
    fn facet_formats_resolved(leaves: &FacetLeaves) -> [bool; 5] {
        [
            !leaves.rust.is_empty(),
            !leaves.typescript.is_empty(),
            !leaves.graphql.is_empty(),
            !leaves.json_schema.is_empty(),
            !leaves.proto.is_empty(),
        ]
    }

    #[cfg(feature = "catalog-integration")]
    fn json_property_keys(schema: &Value) -> BTreeSet<String> {
        schema
            .get("properties")
            .and_then(Value::as_object)
            .map(|properties| properties.keys().cloned().collect())
            .unwrap_or_default()
    }

    #[cfg(feature = "catalog-integration")]
    fn persistent_property_keys_from_artifact_json(schema: &Value) -> BTreeSet<String> {
        let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
            return BTreeSet::new();
        };
        properties
            .iter()
            .filter_map(|(name, prop)| {
                let state = prop.get("x-semio-state")?.as_str()?;
                (state == "persistent").then(|| name.clone())
            })
            .collect()
    }

    #[cfg(feature = "catalog-integration")]
    fn assert_json_states_parse(descriptor_id: &str, facet: &str, schema: &Value) {
        let properties = schema
            .get("properties")
            .and_then(Value::as_object)
            .unwrap_or_else(|| panic!("{descriptor_id}: {facet} properties object required"));
        for (name, prop) in properties {
            let raw = prop
                .get("x-semio-state")
                .and_then(Value::as_str)
                .unwrap_or_else(|| panic!("{descriptor_id}: {facet} property `{name}` missing x-semio-state"));
            parse_state_class_kebab(raw)
                .unwrap_or_else(|| panic!("{descriptor_id}: {facet} property `{name}` has invalid x-semio-state `{raw}`"));
        }
    }

    #[cfg(feature = "catalog-integration")]
    fn validate_registered_artifact_descriptor(descriptor: &ArtifactSchemaDescriptor) {
        let artifact_json: Value = serde_json::from_str(descriptor.artifact.json_schema)
            .unwrap_or_else(|error| panic!("{}: artifact json_schema parse: {error}", descriptor.id));
        let snapshot_json: Value = serde_json::from_str(descriptor.snapshot.json_schema)
            .unwrap_or_else(|error| panic!("{}: snapshot json_schema parse: {error}", descriptor.id));
        let diff_json: Value = serde_json::from_str(descriptor.diff.json_schema)
            .unwrap_or_else(|error| panic!("{}: diff json_schema parse: {error}", descriptor.id));
        assert_json_states_parse(descriptor.id, "artifact", &artifact_json);
        assert_json_states_parse(descriptor.id, "snapshot", &snapshot_json);
        assert_json_states_parse(descriptor.id, "diff", &diff_json);
        let persistent = persistent_property_keys_from_artifact_json(&artifact_json);
        let snapshot_keys = json_property_keys(&snapshot_json);
        assert_eq!(
            snapshot_keys, persistent,
            "{}: snapshot properties must equal persistent artifact properties",
            descriptor.id
        );
        with_json_schema_catalog(|catalog| {
            catalog
                .schema(descriptor.id)
                .unwrap_or_else(|| panic!("{}: missing normative artifact json in SchemaCatalog", descriptor.id));
        });
        let gql = artifact_schema_graphql_sdl(descriptor.id)
            .unwrap_or_else(|| panic!("{}: missing artifact graphql SDL in catalog", descriptor.id));
        assert!(
            gql.contains("directive @state"),
            "{}: graphql must include shared preamble",
            descriptor.id
        );
    }

    #[cfg(feature = "catalog-integration")]
    #[test]
    fn artifact_schema_catalog_registers_and_validates_all_fifty_four_artifacts() {
        register_all_plugin_artifact_schema_descriptors();
        let mut ids = Vec::new();
        with_artifact_schema_registry(|registry| {
            assert_eq!(
                registry.len(),
                54,
                "global artifact schema catalog must register exactly 54 artifacts"
            );
            for descriptor in registry.iter() {
                ids.push(descriptor.id.to_string());
                let artifact_formats = facet_formats_resolved(&descriptor.artifact);
                let snapshot_formats = facet_formats_resolved(&descriptor.snapshot);
                let diff_formats = facet_formats_resolved(&descriptor.diff);
                println!(
                    "[DEBUG] {} facets artifact={:?} snapshot={:?} diff={:?}",
                    descriptor.id, artifact_formats, snapshot_formats, diff_formats
                );
                assert!(
                    artifact_formats.iter().all(|resolved| *resolved),
                    "{}: artifact facet missing a format leaf",
                    descriptor.id
                );
                assert!(
                    snapshot_formats.iter().all(|resolved| *resolved),
                    "{}: snapshot facet missing a format leaf",
                    descriptor.id
                );
                assert!(
                    diff_formats.iter().all(|resolved| *resolved),
                    "{}: diff facet missing a format leaf",
                    descriptor.id
                );
                validate_registered_artifact_descriptor(descriptor);
            }
        });
        ids.sort();
        for id in ids {
            println!("[DEBUG] catalog artifact id {id}");
        }
    }


        #[cfg(feature = "catalog-integration")]
    fn register_all_plugin_app_schema_descriptors() {
semio_s_plugin_writer::apps::writer::config::schema::register_app_schema();
        semio_s_plugin_mathematical::apps::mathematical::config::schema::register_app_schema();
        semio_s_plugin_procedural::apps::procedural2d::config::schema::register_app_schema();
        semio_s_plugin_procedural::apps::procedural3d::config::schema::register_app_schema();
        semio_s_plugin_flow::apps::flow::config::schema::register_app_schema();
        semio_s_plugin_gis::apps::gis2d::config::schema::register_app_schema();
        semio_s_plugin_gis::apps::gis3d::config::schema::register_app_schema();
        semio_s_plugin_vcs::apps::vcs::config::schema::register_app_schema();
        semio_s_plugin_animate::apps::present::config::schema::register_app_schema();
        semio_s_plugin_shooting::apps::shooting::config::schema::register_app_schema();
        semio_s_plugin_sequence::apps::sequence::config::schema::register_app_schema();
        semio_s_plugin_fem::apps::fem2d::config::schema::register_app_schema();
        semio_s_plugin_fem::apps::fem3d::config::schema::register_app_schema();
        semio_s_plugin_architect::apps::architect::config::schema::register_app_schema();
        semio_s_plugin_process::apps::process3d::config::schema::register_app_schema();
        semio_s_plugin_lowpoly::apps::lowpoly::config::schema::register_app_schema();
        semio_s_plugin_reasoning_mindmap::apps::wires::config::schema::register_app_schema();
        semio_s_plugin_forms::apps::forms::config::schema::register_app_schema();
        semio_s_plugin_layout::apps::layout::config::schema::register_app_schema();
        semio_s_plugin_cad::apps::cad::config::schema::register_app_schema();
        semio_s_plugin_norm::config::schema::register_app_schema();
        semio_s_plugin_playbook::apps::playbook::config::schema::register_app_schema();
        semio_s_plugin_imperative::apps::imperative::config::schema::register_app_schema();
        semio_s_plugin_remodel::apps::remodel::config::schema::register_app_schema();
        semio_s_plugin_trinity::apps::rewrite::config::schema::register_app_schema();
        semio_s_plugin_trinity::apps::jack::config::schema::register_app_schema();
        semio_s_plugin_dag::apps::dag::config::schema::register_app_schema();
        semio_s_plugin_draw::apps::draw::config::schema::register_app_schema();
        semio_s_plugin_raster::apps::raster::config::schema::register_app_schema();
        semio_s_plugin_note::apps::note::config::schema::register_app_schema();
        semio_s_plugin_puzzle::apps::puzzle2d::config::schema::register_app_schema();
        semio_s_plugin_puzzle::apps::puzzle5d::config::schema::register_app_schema();
        semio_s_plugin_puzzle::apps::puzzle3d::config::schema::register_app_schema();
        semio_s_plugin_block::apps::block2d::config::schema::register_app_schema();
        semio_s_plugin_block::apps::block5d::config::schema::register_app_schema();
        semio_s_plugin_block::apps::block3d::config::schema::register_app_schema();
        semio_s_plugin_space::apps::home::config::schema::register_app_schema();
        semio_s_plugin_space::apps::space::config::schema::register_app_schema();
        semio_s_plugin_sourcing::apps::curate::config::schema::register_app_schema();
    }

//#endregion 🔖️CatalogIntegration

    //#region 🔖️AppSchemaRegistryParity
    fn empty_app_facet_leaves() -> FacetLeaves {
        FacetLeaves {
            rust: "",
            typescript: "",
            graphql: "",
            json_schema: r#"{
  "$id": "https://semio.tech/schema/app/placeholder/empty/config.json",
  "title": "EmptyConfig",
  "type": "object",
  "additionalProperties": false,
  "properties": {}
}"#,
            proto: "",
        }
    }

    fn validate_registered_app_descriptor(descriptor: &AppSchemaDescriptor) {
        for (facet, leaves) in [("config", &descriptor.config), ("presence", &descriptor.presence)] {
            if leaves.json_schema.trim().is_empty() {
                continue;
            }
            let schema: Value = serde_json::from_str(leaves.json_schema)
                .unwrap_or_else(|error| panic!("{}: {facet} json_schema parse: {error}", descriptor.id));
            assert_eq!(
                schema.get("type").and_then(Value::as_str),
                Some("object"),
                "{}: {facet} must be an object schema",
                descriptor.id
            );
            let properties = schema
                .get("properties")
                .and_then(Value::as_object)
                .unwrap_or_else(|| panic!("{}: {facet} properties object required", descriptor.id));
            for (name, prop) in properties {
                let raw = prop
                    .get("x-semio-state")
                    .and_then(Value::as_str)
                    .unwrap_or_else(|| panic!("{}: {facet} property `{name}` missing x-semio-state", descriptor.id));
                let class = parse_state_class_kebab(raw)
                    .unwrap_or_else(|| panic!("{}: {facet} property `{name}` has invalid x-semio-state `{raw}`", descriptor.id));
                let expected = if facet == "config" { StateClass::LocalUi } else { StateClass::SharedUi };
                assert_eq!(
                    class, expected,
                    "{}: {facet} field `{name}` must be {:?}",
                    descriptor.id, expected
                );
            }
        }
    }

    #[cfg(feature = "catalog-integration")]
    #[test]
    fn app_schema_registry_registers_and_validates_all_thirty_nine_owners() {
        register_all_plugin_app_schema_descriptors();
        let mut registry = AppSchemaRegistry::new();
        with_kernel_app_schema_catalog(|entries| {
            for entry in entries {
                registry.register(app_descriptor_from_kernel(entry));
            }
        });
        assert_eq!(registry.len(), 39, "A6 registers all 39 app schema owners");
        let mut walked = 0usize;
        for descriptor in registry.iter() {
            walked += 1;
            validate_registered_app_descriptor(descriptor);
        }
        assert_eq!(walked, registry.len());
    }

    #[test]
    fn app_schema_registry_is_empty_without_catalog_integration_feature() {
        let mut registry = AppSchemaRegistry::new();
        with_kernel_app_schema_catalog(|entries| {
            for entry in entries {
                registry.register(app_descriptor_from_kernel(entry));
            }
        });
        #[cfg(not(feature = "catalog-integration"))]
        assert_eq!(registry.len(), 0);
        #[cfg(feature = "catalog-integration")]
        {
            register_all_plugin_app_schema_descriptors();
            let mut registry = AppSchemaRegistry::new();
            with_kernel_app_schema_catalog(|entries| {
                for entry in entries {
                    registry.register(app_descriptor_from_kernel(entry));
                }
            });
            assert_eq!(registry.len(), 39);
        }
    }

    #[test]
    fn app_schema_registry_accepts_placeholder_owner_for_wave_structure() {
        let mut registry = AppSchemaRegistry::new();
        let empty = empty_app_facet_leaves();
        registry.register(AppSchemaDescriptor {
            id: "s.wave.a3.placeholder",
            config: empty.clone(),
            presence: FacetLeaves {
                json_schema: r#"{
  "$id": "https://semio.tech/schema/app/placeholder/empty/presence.json",
  "title": "EmptyPresence",
  "type": "object",
  "additionalProperties": false,
  "properties": {}
}"#,
                ..empty
            },
        });
        assert_eq!(registry.len(), 1);
        validate_registered_app_descriptor(registry.get("s.wave.a3.placeholder").expect("placeholder"));
        assert!(GRAPHQL_STATE_PREAMBLE.contains("directive @state"));
    }
    //#endregion 🔖️AppSchemaRegistryParity
}
//#endregion 🔖️Tests
