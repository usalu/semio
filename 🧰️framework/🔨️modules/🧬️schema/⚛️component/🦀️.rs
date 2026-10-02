//! 📋️ Canonical schema versions, document projection and owned JSON validation.

use crate::SchemaError;
use pack::json::{parse as parse_json, to_string as json_to_string, JsonError, Value};
use pack::{content_hash, ContentHash};
use std::collections::HashMap;

pub use semio_framework_schema_state::StateClass;
use semio_framework_schema_state::parse_state_class_kebab;
pub use semio_framework_schema_composition::{ArtifactCompositionFields, ChildFieldRefs, ChildRefVisitor, ChildSlotSpec, LinkSlotSpec};
pub use semio_framework_schema_derive::ArtifactSchema;
use semio_framework_schema_registry::{
    ArtifactSchemaDescriptor, ArtifactInferenceDescriptor, AppSchemaDescriptor, with_artifact_schema_catalog, with_artifact_inference_catalog, with_app_schema_catalog, register_referenced_schema_documents, register_scope_facet_leaves, register_scope_schema_exports, registered_referenced_schema_documents, resolve_schema_export, schema_export_catalog_entries, scope_schema_exports_registered, scope_schema_facets_registered, with_schema_export_registry, FacetLeaves, SchemaExport, SchemaExportEntries, SchemaExportEntry, SchemaExportRegistry,
    SchemaExportRegistryError, SchemaFormat, SchemaResolveError, ScopeSchemaExports, RESERVED_FACET_EXPORT_IDS,
};

//#region 🔖️Errors
#[derive(Debug, PartialEq, Eq)]
pub enum SchemaCatalogError {
    UnknownSchema(String),
    Validation(SchemaError),
}

impl std::fmt::Display for SchemaCatalogError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownSchema(id) => write!(formatter, "unknown schema id: {id}"),
            Self::Validation(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for SchemaCatalogError {}
//#endregion 🔖️Errors

//#region 🔖️EntityCatalog
include!("../🤖️generated/🏷️entity-kinds/🦀️.rs");
//#endregion 🔖️EntityCatalog

//#region 🔖️SchemaCatalog
pub struct SchemaCatalog {
    schemas: HashMap<String, Value>,
    validators: HashMap<String, crate::OwnedJsonSchemaValidator>,
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

    pub fn register_json(&mut self, id: &str, schema: Value) -> Result<(), SchemaError> {
        let validator = crate::OwnedJsonSchemaValidator::compile_value(&schema)?;
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

    pub fn validate(&self, id: &str, value: &Value) -> Result<(), SchemaCatalogError> {
        let validator = self.validators.get(id).ok_or_else(|| SchemaCatalogError::UnknownSchema(id.to_string()))?;
        validator.validate_value(value).map_err(SchemaCatalogError::Validation)
    }
}
//#endregion 🔖️SchemaCatalog

//#region 🔖️GraphQlStatePreamble
/// 🔗 Shared GraphQL `@state`/`@derived` SDL preamble — declared once, never repeated per artifact.
/// `@state` names one of the four state lanes; `@derived` is the ORTHOGONAL derivation marker, never
/// a fifth lane — a derived field is computed from a snapshot, so it is not state at all.
/// `Long` is the 64-bit integer lane: GraphQL's built-in `Int` is 32-bit by specification, so a Rust
/// `i64`/`u64` carrier has no built-in spelling and is declared here once, exactly as `@state` is.
pub const GRAPHQL_STATE_PREAMBLE: &str = "\
enum StateClass { ARTIFACT CONFIG PRESENCE TRANSIENT }\n\
scalar Long\n\
directive @state(class: StateClass!) on FIELD_DEFINITION\n\
directive @derived on FIELD_DEFINITION\
";
//#endregion 🔖️GraphQlStatePreamble

//#region 🔖️ArtifactSchemaFields
/// ✨️ Per-artifact field → [`StateClass`] table emitted by [`ArtifactSchema`].
///
/// `field_states()` lists only STATE fields. Fields annotated `#[derived]` are computed from a
/// snapshot rather than stored in any lane, so they carry no [`StateClass`] at all and are reported
/// separately by [`ArtifactSchemaFields::derived_fields`] — the Rust twin of JSON Schema's
/// `x-semio-derived: true` and GraphQL's `@derived`.
pub trait ArtifactSchemaFields {
    fn artifact_schema_id() -> impl std::future::Future<Output = &'static str> + Send;
    fn field_states() -> impl std::future::Future<Output = &'static [(&'static str, StateClass)]> + Send;
    fn derived_fields() -> impl std::future::Future<Output = &'static [&'static str]> + Send {
        async { &[] as &'static [&'static str] }
    }
}

/// 🏷️ Canonical JSON Schema key carrying the derivation marker, sibling of `x-semio-state` on the
/// orthogonal axis. Its only legal value is `true`; an absent key means "not derived".
pub const JSON_SCHEMA_DERIVED_KEY: &str = "x-semio-derived";
//#endregion 🔖️ArtifactSchemaFields

//#region 🔖️ArtifactCompositionSpec

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
/// 🧬️ Stable identity of a canonical JSON Schema leaf.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SchemaVersion(pub ContentHash);

/// 🧬️ Computes a whitespace-independent version from an owned-parser canonical JSON leaf.
pub fn schema_version(body: &str) -> Result<SchemaVersion, JsonError> {
    let canonical = if body.trim().is_empty() { String::new() } else { json_to_string(&canonical_schema_value(parse_json(body)?)) };
    Ok(SchemaVersion(content_hash(canonical.as_bytes())))
}

fn canonical_schema_value(value: Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.into_iter().map(canonical_schema_value).collect()),
        Value::Object(object) => {
            let mut entries: Vec<_> = object.iter().map(|(key, value)| (key.to_string(), canonical_schema_value(value.clone()))).collect();
            entries.sort_by(|left, right| left.0.cmp(&right.0));
            pack::json::object(entries)
        }
        value => value,
    }
}


/// 🧬️ Computes the canonical artifact schema version.
pub fn artifact_schema_version(descriptor: &ArtifactSchemaDescriptor) -> Result<SchemaVersion, JsonError> { schema_version(descriptor.artifact.json_schema) }

/// 🧬️ Computes the canonical snapshot schema version.
pub fn snapshot_schema_version(descriptor: &ArtifactSchemaDescriptor) -> Result<SchemaVersion, JsonError> { schema_version(descriptor.snapshot.json_schema) }

/// 🧬️ Computes the canonical diff schema version.
pub fn diff_schema_version(descriptor: &ArtifactSchemaDescriptor) -> Result<SchemaVersion, JsonError> { schema_version(descriptor.diff.json_schema) }

/// 🧬️ Computes the canonical mutations schema version.
pub fn mutations_schema_version(descriptor: &ArtifactSchemaDescriptor) -> Result<SchemaVersion, JsonError> { schema_version(descriptor.mutations.json_schema) }

/// 🧬️ Computes the canonical config schema version.
pub fn config_schema_version(descriptor: &AppSchemaDescriptor) -> Result<SchemaVersion, JsonError> { schema_version(descriptor.config.json_schema) }

/// 🧬️ Computes the canonical presence schema version.
pub fn presence_schema_version(descriptor: &AppSchemaDescriptor) -> Result<SchemaVersion, JsonError> { schema_version(descriptor.presence.json_schema) }

fn parse_normative_json_leaf(descriptor_id: &str, facet: &str, body: &str) -> Value {
    parse_json(body).unwrap_or_else(|error| panic!("{descriptor_id}: {facet} json_schema parse: {error}"))
}

fn graphql_leaf_with_preamble(body: &str) -> String {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return GRAPHQL_STATE_PREAMBLE.to_string();
    }
    format!("{}\n\n{trimmed}", GRAPHQL_STATE_PREAMBLE)
}

pub async fn with_json_schema_catalog<R>(visit: impl FnOnce(&SchemaCatalog) -> R) -> R {
    let mut catalog = SchemaCatalog::new();
    with_artifact_schema_catalog(|entries| {
        for entry in entries {
            catalog.load_json(entry.id, parse_normative_json_leaf(entry.id, "artifact", entry.artifact.json_schema));
        }
    });
    visit(&catalog)
}

pub async fn artifact_schema_graphql_sdl(key: &str) -> Option<String> {
    with_artifact_schema_catalog(|entries| {
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

pub async fn with_inference_json_schema_catalog<R>(visit: impl FnOnce(&SchemaCatalog) -> R) -> R {
    let mut catalog = SchemaCatalog::new();
    with_artifact_inference_catalog(|entries| {
        for entry in entries {
            catalog.load_json(entry.id, parse_normative_json_leaf(entry.id, "inference", entry.inference.json_schema));
        }
    });
    visit(&catalog)
}

pub async fn artifact_inference_graphql_sdl(key: &str) -> Option<String> {
    with_artifact_inference_catalog(|entries| entries.iter().find(|entry| entry.id == key).map(|entry| graphql_leaf_with_preamble(entry.inference.graphql)))
}

pub async fn with_app_json_schema_catalog<R>(visit: impl FnOnce(&SchemaCatalog) -> R) -> R {
    let mut catalog = SchemaCatalog::new();
    with_app_schema_catalog(|entries| {
        for entry in entries {
            catalog.load_json(entry.id, parse_normative_json_leaf(entry.id, "config", entry.config.json_schema));
            catalog.load_json(&format!("{}.presence", entry.id), parse_normative_json_leaf(entry.id, "presence", entry.presence.json_schema));
        }
    });
    visit(&catalog)
}

pub async fn app_schema_graphql_sdl(key: &str) -> Option<String> {
    with_app_schema_catalog(|entries| {
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

pub async fn validate_registered_app_descriptor(descriptor: &AppSchemaDescriptor) {
    for (facet, leaves) in [("config", &descriptor.config), ("presence", &descriptor.presence)] {
        if leaves.json_schema.trim().is_empty() {
            continue;
        }
        let schema = parse_json(leaves.json_schema).unwrap_or_else(|error| panic!("{}: {facet} json_schema parse: {error}", descriptor.id));
        assert_eq!(schema.get("type").and_then(Value::as_str), Some("object"), "{}: {facet} must be an object schema", descriptor.id);
        let properties = schema.get("properties").and_then(Value::as_object).unwrap_or_else(|| panic!("{}: {facet} properties object required", descriptor.id));
        for (name, prop) in properties {
            let raw = prop.get("x-semio-state").and_then(Value::as_str).unwrap_or_else(|| panic!("{}: {facet} property `{name}` missing x-semio-state", descriptor.id));
            let class = parse_state_class_kebab(raw).unwrap_or_else(|| panic!("{}: {facet} property `{name}` has invalid x-semio-state `{raw}`", descriptor.id));
            let expected = if facet == "config" { StateClass::Config } else { StateClass::Presence };
            assert_eq!(class, expected, "{}: {facet} field `{name}` must be {:?}", descriptor.id, expected);
        }
    }
}

//#region 🔖️SchemaExportResolution
/// ⚠️ Boundary validation could not be prepared for a `(scope id, export id)` pair.
#[derive(Debug, PartialEq, Eq)]
pub enum SchemaBoundaryError {
    Resolve(SchemaResolveError),
    Schema(SchemaError),
}

impl std::fmt::Display for SchemaBoundaryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Resolve(error) => error.fmt(formatter),
            Self::Schema(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for SchemaBoundaryError {}

/// ✅ Compiles the structural validator for one export's JSON Schema leaf out of a
/// [`SchemaExportRegistry`] snapshot. Every other JSON Schema leaf in that snapshot is offered as a
/// sibling document, so a cross-scope `$ref` naming its target's `$id` resolves without a filesystem
/// read. Lives here rather than on the registry because structural validators consume the registry
/// through the neutral draft-07 validator package.
pub fn structural_validator_in(registry: &SchemaExportRegistry, scope: &str, export: &str) -> Result<crate::OwnedJsonSchemaValidator, SchemaBoundaryError> {
    let body = registry.resolve(scope, export, SchemaFormat::JsonSchema).map_err(SchemaBoundaryError::Resolve)?;
    let mut documents: std::collections::BTreeMap<String, &'static str> = std::collections::BTreeMap::new();
    for entry in registry.entries().filter(|entry| entry.format == SchemaFormat::JsonSchema) {
        let Ok(sibling) = registry.resolve(entry.scope, entry.export, SchemaFormat::JsonSchema) else { continue };
        let Some(id) = parse_json(sibling).ok().and_then(|document| document.get("$id").and_then(Value::as_str).map(str::to_string)) else { continue };
        match documents.insert(id.clone(), sibling) {
            Some(established) if established != sibling => return Err(SchemaBoundaryError::Schema(SchemaError::Validation(format!("two schema leaves declare `$id` {id}")))),
            _ => {}
        }
    }
    let bodies: Vec<&str> = documents.values().copied().collect();
    crate::OwnedJsonSchemaValidator::compile_with_documents(body, &bodies).map_err(SchemaBoundaryError::Schema)
}

/// ✅ Compiles the OS-wide structural validator for one export — the application boundary entry
/// point for validating serialized input before domain rules.
pub fn structural_validator_for(scope: &str, export: &str) -> Result<crate::OwnedJsonSchemaValidator, SchemaBoundaryError> {
    with_schema_export_registry(|registry| structural_validator_in(registry, scope, export))
}

/// 🪪️ Scope id of this module — the `framework.schema` resolution contract every scope speaks.
pub const FRAMEWORK_SCHEMA_SCOPE: &str = "framework.schema";

/// 🍃 Leaves of the seven exports whose Rust definitions live in the dependency-free registry crate.
const FRAMEWORK_SCHEMA_REGISTRY_LEAVES: FacetLeaves = FacetLeaves {
    rust: include_str!("../📇️registry/🦀️.rs"),
    typescript: include_str!("../🟦️.ts"),
    graphql: "",
    json_schema: include_str!("../🔣️.json"),
    proto: "",
};

/// 🍃 Leaves of the one export whose Rust definition stays beside the draft-07 validator.
const FRAMEWORK_SCHEMA_VALIDATION_LEAVES: FacetLeaves = FacetLeaves {
    rust: include_str!("../✅️validator/⚠️error/🦀️.rs"),
    typescript: include_str!("../🟦️.ts"),
    graphql: "",
    json_schema: include_str!("../🔣️.json"),
    proto: "",
};

/// 🍃 Leaves of the two exports whose Rust definition is the generated entity-catalog projection.
const FRAMEWORK_SCHEMA_ENTITY_CATALOG_LEAVES: FacetLeaves = FacetLeaves {
    rust: include_str!("../🤖️generated/🏷️entity-kinds/🦀️.rs"),
    typescript: include_str!("../🟦️.ts"),
    graphql: "",
    json_schema: include_str!("../🔣️.json"),
    proto: "",
};

/// 📚️ The single instance document of [`FRAMEWORK_SCHEMA_ENTITY_CATALOG_LEAVES`]'s `EntityKindCatalog`
/// export — the source every projection in `🤖️generated/🏷️entity-kinds/🦀️.rs`, `🤖️generated/🏷️entity-kinds/🟦️.ts` is emitted from by the `schema-entity-catalog` generator.
pub const ENTITY_KIND_CATALOG_JSON: &str = include_str!("../🏷️entity-kinds/🔣️.json");

/// 🏷️ The named exports of `framework.schema`, one per `$defs` key of the module's `🔣️.json`.
pub const FRAMEWORK_SCHEMA_EXPORTS: [SchemaExport; 10] = [
    SchemaExport { id: "SchemaFormat", leaves: FRAMEWORK_SCHEMA_REGISTRY_LEAVES },
    SchemaExport { id: "FacetLeaves", leaves: FRAMEWORK_SCHEMA_REGISTRY_LEAVES },
    SchemaExport { id: "SchemaExport", leaves: FRAMEWORK_SCHEMA_REGISTRY_LEAVES },
    SchemaExport { id: "ScopeSchemaExports", leaves: FRAMEWORK_SCHEMA_REGISTRY_LEAVES },
    SchemaExport { id: "SchemaExportEntry", leaves: FRAMEWORK_SCHEMA_REGISTRY_LEAVES },
    SchemaExport { id: "SchemaExportEntries", leaves: FRAMEWORK_SCHEMA_REGISTRY_LEAVES },
    SchemaExport { id: "SchemaResolveError", leaves: FRAMEWORK_SCHEMA_REGISTRY_LEAVES },
    SchemaExport { id: "ValidationDiagnostic", leaves: FRAMEWORK_SCHEMA_VALIDATION_LEAVES },
    SchemaExport { id: "EntityKind", leaves: FRAMEWORK_SCHEMA_ENTITY_CATALOG_LEAVES },
    SchemaExport { id: "EntityKindCatalog", leaves: FRAMEWORK_SCHEMA_ENTITY_CATALOG_LEAVES },
];

/// 📥 Registers this module's own scope into the OS-wide catalog, the way every other scope owner
/// registers its exports beside [`register_artifact_schema_descriptor`].
pub fn register_framework_schema_exports() -> Result<(), SchemaExportRegistryError> {
    register_scope_schema_exports(ScopeSchemaExports { scope: FRAMEWORK_SCHEMA_SCOPE, exports: &FRAMEWORK_SCHEMA_EXPORTS })
}
//#endregion 🔖️SchemaExportResolution

#[cfg(test)]
//#region 🔖️Tests
#[path = "../🧪️tests/🔬️component-unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
