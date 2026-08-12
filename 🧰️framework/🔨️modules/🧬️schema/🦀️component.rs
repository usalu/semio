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
enum StateClass { PERSISTENT SHARED_UI LOCAL_UI PREVIEW EFFECT INFERRED }\n\
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

//#region 🔖️ArtifactCompositionSpec
/// 🧒️ One declared CHILD slot on an artifact snapshot — an owned sub-artifact with its own document
/// and lifecycle (`ArtifactChild<T>` / `Vec<ArtifactChild<T>>` at the field level).
///
/// `kind` is a plain `&'static str` holding a canonical artifact kind id, grammar `s.<plugin>.<artifact>`
/// (e.g. `"s.stdio.mesh"`) — deliberately NOT the `ArtifactKindId` newtype from `🚪️io`'s `semio-framework`
/// crate: this crate (`semio-framework-schema`) must not gain a dependency on `semio-framework` merely to
/// name a kind inside a slot table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChildSlotSpec {
    pub name: &'static str,
    pub kind: &'static str,
    pub many: bool,
}

/// 🔗 One declared LINK slot on an artifact snapshot — a reference to an independent artifact, never
/// owned (`ArtifactLink` / `Vec<ArtifactLink>` at the field level).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkSlotSpec {
    pub name: &'static str,
    pub roles: &'static [&'static str],
    pub many: bool,
}

/// ✨️ Per-artifact CHILD/LINK slot tables, sibling of [`ArtifactSchemaFields`] and emitted alongside
/// it by [`ArtifactSchema`] — lets consumers (UI, manifest, io) read a snapshot type's declared
/// composition slots from the schema registry instead of hardcoding "renders as a link" / "derived
/// composition" behaviour. Leaf artifacts (no children, no links) get both methods for free via the
/// default `&[]` — no boilerplate impl required.
pub trait ArtifactCompositionFields {
    fn child_slots() -> &'static [ChildSlotSpec] {
        &[]
    }
    fn link_slots() -> &'static [LinkSlotSpec] {
        &[]
    }
}

/// 🔗 Shared GraphQL SDL fragment for CHILD/LINK slots — declares the `ArtifactLink` type and the
/// `@child`/`@link` directives once, so per-artifact GraphQL facets reference it instead of
/// redeclaring it (mirrors [`GRAPHQL_STATE_PREAMBLE`]'s composition role for `@state`).
pub const GRAPHQL_COMPOSITION_PREAMBLE: &str = "\
type ArtifactLink { targetId: String! kind: String! }\n\
directive @child(kind: String!) on FIELD_DEFINITION\n\
directive @link(roles: [String!]) on FIELD_DEFINITION\
";
//#endregion 🔖️ArtifactCompositionSpec

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

/// 🧬️ Registered descriptor for one artifact's four schema facets.
#[derive(Clone, Debug)]
pub struct ArtifactSchemaDescriptor {
    pub id: &'static str,
    pub artifact: FacetLeaves,
    pub snapshot: FacetLeaves,
    pub diff: FacetLeaves,
    pub mutations: FacetLeaves,
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
    register_kernel_app_schema_descriptor, register_kernel_artifact_inference_descriptor, register_kernel_artifact_schema_descriptor,
    with_kernel_app_schema_catalog, with_kernel_artifact_inference_catalog, with_kernel_artifact_schema_catalog, KernelAppSchemaDescriptor,
    KernelArtifactInferenceDescriptor, KernelArtifactSchemaDescriptor, KernelFacetLeaves,
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
        mutations: facet_leaves_to_kernel(descriptor.mutations),
    }
}

fn descriptor_from_kernel(kernel: &KernelArtifactSchemaDescriptor) -> ArtifactSchemaDescriptor {
    ArtifactSchemaDescriptor {
        id: kernel.id,
        artifact: facet_leaves_from_kernel(&kernel.artifact),
        snapshot: facet_leaves_from_kernel(&kernel.snapshot),
        diff: facet_leaves_from_kernel(&kernel.diff),
        mutations: facet_leaves_from_kernel(&kernel.mutations),
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

//#region 🔖️ArtifactInferenceDescriptor
/// 💡️ Registered descriptor for one artifact's 💡️inference schema facet — a SIBLING to
/// [`ArtifactSchemaDescriptor`], not a field on it (see [`KernelArtifactInferenceDescriptor`]'s own
/// doc for why). `id` is the inference schema's own id, `"{artifact_id}.inference"`.
#[derive(Clone, Debug)]
pub struct ArtifactInferenceDescriptor {
    pub id: &'static str,
    pub inference: FacetLeaves,
}

fn inference_descriptor_to_kernel(descriptor: ArtifactInferenceDescriptor) -> KernelArtifactInferenceDescriptor {
    KernelArtifactInferenceDescriptor { id: descriptor.id, inference: facet_leaves_to_kernel(descriptor.inference) }
}

fn inference_descriptor_from_kernel(kernel: &KernelArtifactInferenceDescriptor) -> ArtifactInferenceDescriptor {
    ArtifactInferenceDescriptor { id: kernel.id, inference: facet_leaves_from_kernel(&kernel.inference) }
}

/// 📚 Runtime registry of [`ArtifactInferenceDescriptor`] values — inference twin of [`ArtifactSchemaRegistry`].
pub struct ArtifactInferenceRegistry {
    by_id: HashMap<&'static str, ArtifactInferenceDescriptor>,
}

impl Default for ArtifactInferenceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtifactInferenceRegistry {
    pub fn new() -> Self {
        Self { by_id: HashMap::new() }
    }

    pub fn register(&mut self, descriptor: ArtifactInferenceDescriptor) {
        self.by_id.insert(descriptor.id, descriptor);
    }

    pub fn get(&self, id: &str) -> Option<&ArtifactInferenceDescriptor> {
        self.by_id.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &ArtifactInferenceDescriptor> {
        self.by_id.values()
    }

    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }
}

/// 📎 Registers one artifact's handcrafted inference descriptor into the OS-wide catalog. `id` on
/// the descriptor must be `"{artifact_id}.inference"`, matching its owning `ArtifactSchemaDescriptor`'s id.
pub fn register_artifact_inference_descriptor(descriptor: ArtifactInferenceDescriptor) {
    register_kernel_artifact_inference_descriptor(inference_descriptor_to_kernel(descriptor));
}

/// 🔎 Whether `id` (the inference schema id) is present in the OS-wide inference descriptor registry.
pub fn artifact_inference_descriptor_registered(id: &str) -> bool {
    semio_framework_os_kernel::kernel_artifact_inference_descriptor_registered(id)
}

/// 📚 Invokes `visit` with the OS-wide [`ArtifactInferenceRegistry`] snapshot.
pub fn with_artifact_inference_registry<R>(visit: impl FnOnce(&ArtifactInferenceRegistry) -> R) -> R {
    let mut registry = ArtifactInferenceRegistry::new();
    with_kernel_artifact_inference_catalog(|entries| {
        for entry in entries {
            registry.register(inference_descriptor_from_kernel(entry));
        }
    });
    visit(&registry)
}

/// 🔣 Invokes `visit` with a [`SchemaCatalog`] of normative inference JSON leaves, keyed by the
/// inference schema id (`"{artifact_id}.inference"`).
pub fn with_inference_json_schema_catalog<R>(visit: impl FnOnce(&SchemaCatalog) -> R) -> R {
    let mut catalog = SchemaCatalog::new();
    with_kernel_artifact_inference_catalog(|entries| {
        for entry in entries {
            catalog.load_json(entry.id, parse_normative_json_leaf(entry.id, "inference", entry.inference.json_schema));
        }
    });
    visit(&catalog)
}

/// 🔗 Returns composed GraphQL SDL (shared `@state` preamble + facet leaf) for an inference schema
/// id (`"{artifact_id}.inference"`).
pub fn artifact_inference_graphql_sdl(key: &str) -> Option<String> {
    with_kernel_artifact_inference_catalog(|entries| {
        entries.iter().find(|entry| entry.id == key).map(|entry| graphql_leaf_with_preamble(entry.inference.graphql))
    })
}
//#endregion 🔖️ArtifactInferenceDescriptor

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

/// 🔌 Open app-schema registry API for plugin crates — call these from your own `🔧️setup`/init code to register your app's config + presence schema facets. Every app owner self-registers via [`register_app_schema_descriptor`]; there is no closed framework-side catalog.
///
/// - 📎 [`register_app_schema_descriptor`] registers one app owner's handcrafted descriptor into the OS-wide catalog.
/// - 🔎 [`app_schema_descriptor_registered`] checks whether an owner id is already registered.
/// - 📚 [`with_app_schema_registry`] snapshots the OS-wide [`AppSchemaRegistry`] for lookup/iteration.
/// - 🔣 [`with_app_json_schema_catalog`] snapshots normative config/presence JSON leaves as a [`SchemaCatalog`].
/// - 🔗 [`app_schema_graphql_sdl`] resolves composed GraphQL SDL for an owner or `{id}.presence` key.
/// - ✅ [`validate_registered_app_descriptor`] validates a descriptor's JSON Schema leaves and `x-semio-state` tagging before registering.
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

/// ✅ Validates a descriptor's JSON Schema leaves: each non-empty facet must be an object schema whose properties all carry a valid `x-semio-state` matching the facet's expected [`StateClass`] (`local-ui` for config, `shared-ui` for presence). Panics with a descriptor-id-prefixed message on the first violation — call this from a plugin's own tests before [`register_app_schema_descriptor`].
pub fn validate_registered_app_descriptor(descriptor: &AppSchemaDescriptor) {
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
        "inferred" => Some(StateClass::Inferred),
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
        StateClass::Inferred => "inferred",
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
            diff: empty.clone(),
            mutations: empty,
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

    //#region 🔖️ArtifactCompositionFixture
    /// 🧪️ Local stand-ins for `semio-framework-os-kernel`'s store `ArtifactChild<T>` / `ArtifactLink`
    /// — legitimate here since `#[derive(ArtifactSchema)]`'s composition support matches field types
    /// SYNTACTICALLY (last path segment), never resolving the real types.
    struct ArtifactChild<T> {
        _marker: std::marker::PhantomData<T>,
    }
    struct ArtifactLink;

    #[derive(ArtifactSchema)]
    #[artifact_schema(id = "s.wave3.composite")]
    struct CompositeArtifact {
        #[state(persistent)]
        #[child(kind = "s.stdio.mesh")]
        primary_mesh: ArtifactChild<()>,
        #[state(persistent)]
        #[child(kind = "s.stdio.image")]
        textures: Vec<ArtifactChild<()>>,
        #[state(persistent)]
        #[link_slot(roles("base", "material"))]
        base_material: ArtifactLink,
        #[state(persistent)]
        label: String,
    }
    //#endregion 🔖️ArtifactCompositionFixture

    #[test]
    fn artifact_composition_fields_derive_emits_expected_slot_tables() {
        let children = CompositeArtifact::child_slots();
        assert_eq!(children.len(), 2, "single child + Vec child must both be captured, plain field must not");
        assert_eq!(children[0], ChildSlotSpec { name: "primaryMesh", kind: "s.stdio.mesh", many: false });
        assert_eq!(children[1], ChildSlotSpec { name: "textures", kind: "s.stdio.image", many: true });

        let links = CompositeArtifact::link_slots();
        assert_eq!(links.len(), 1, "only the ArtifactLink field must be captured");
        assert_eq!(links[0], LinkSlotSpec { name: "baseMaterial", roles: &["base", "material"], many: false });
    }

    #[test]
    fn artifact_composition_fields_default_to_empty_for_leaf_artifacts() {
        assert!(SyntheticSnapshot::child_slots().is_empty());
        assert!(SyntheticSnapshot::link_slots().is_empty());
        assert_eq!(SyntheticSnapshot::artifact_schema_id(), "s.wave3.synthetic");
    }

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
        assert!(GRAPHQL_STATE_PREAMBLE.contains("enum StateClass { PERSISTENT SHARED_UI LOCAL_UI PREVIEW EFFECT INFERRED }"));
        assert!(GRAPHQL_STATE_PREAMBLE.contains("directive @state(class: StateClass!) on FIELD_DEFINITION"));
    }

    #[test]
    fn state_class_kebab_round_trips_every_variant_including_inferred() {
        for class in [StateClass::Persistent, StateClass::SharedUi, StateClass::LocalUi, StateClass::Preview, StateClass::Effect, StateClass::Inferred] {
            let kebab = state_class_kebab(class);
            assert_eq!(parse_state_class_kebab(kebab), Some(class));
        }
        assert_eq!(state_class_kebab(StateClass::Inferred), "inferred");
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

    //#region 🔖️ArtifactInferenceDescriptorParity
    #[test]
    fn artifact_inference_registry_registers_independently_of_the_snapshot_diff_mutations_descriptor() {
        let mut registry = ArtifactInferenceRegistry::new();
        let empty = FacetLeaves { rust: "", typescript: "", graphql: "component { id }", json_schema: "", proto: "" };
        registry.register(ArtifactInferenceDescriptor { id: "s.wave3.synthetic.inference", inference: empty });
        assert_eq!(registry.len(), 1);
        assert!(!registry.is_empty());
        assert!(registry.get("s.wave3.synthetic.inference").is_some());

        let mut walked = 0usize;
        for descriptor in registry.iter() {
            walked += 1;
            assert_eq!(descriptor.id, "s.wave3.synthetic.inference");
        }
        assert_eq!(walked, 1);
    }

    #[test]
    fn artifact_inference_graphql_sdl_composes_shared_preamble_with_facet_leaf() {
        register_artifact_inference_descriptor(ArtifactInferenceDescriptor {
            id: "s.wave3.synthetic.sdl-probe.inference",
            inference: FacetLeaves { rust: "", typescript: "", graphql: "type SdlProbeInference { flag: Boolean }", json_schema: "", proto: "" },
        });
        assert!(artifact_inference_descriptor_registered("s.wave3.synthetic.sdl-probe.inference"));
        let sdl = artifact_inference_graphql_sdl("s.wave3.synthetic.sdl-probe.inference").expect("registered inference sdl");
        assert!(sdl.contains("INFERRED"), "composed SDL must carry the shared @state preamble");
        assert!(sdl.contains("type SdlProbeInference"));
        assert!(artifact_inference_graphql_sdl("s.wave3.synthetic.unregistered.inference").is_none());
    }
    //#endregion 🔖️ArtifactInferenceDescriptorParity

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
