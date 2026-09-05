//! 📋️ Schema registry: describe schemas and validate owned JSON at kernel boundaries.

use pack::json::{parse as parse_json, to_string as json_to_string, JsonError, Value};
use pack::{content_hash, ContentHash};
use std::collections::HashMap;

pub use semio_framework_os_kernel::StateClass;
pub use semio_framework_schema_derive::ArtifactSchema;

//#region 🔖️Errors
#[derive(Debug, PartialEq, Eq)]
pub enum SchemaError {
    UnknownSchema(String),
    Validation(String),
    Cancelled,
    LimitExceeded(usize),
}

impl std::fmt::Display for SchemaError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownSchema(id) => write!(formatter, "unknown schema id: {id}"),
            Self::Validation(message) => write!(formatter, "validation failed: {message}"),
            Self::Cancelled => formatter.write_str("validation cancelled"),
            Self::LimitExceeded(limit) => write!(formatter, "validation node limit exceeded: {limit}"),
        }
    }
}

impl std::error::Error for SchemaError {}
//#endregion 🔖️Errors

//#region 🔖️EntityCatalog
include!("🤖️generated.rs");
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
    // 🚫️async: R9 pure constructor — no I/O (two `HashMap::new()`); every real consumer reaches
    // it through a synchronous `FnOnce` visit closure (this crate's `with_*_catalog` helpers below,
    // whose closures are handed to `semio-framework-replication`'s `with_kernel_*_catalog`, itself
    // a fixed sync-closure signature outside this packet's writable scope) and cannot itself be made async.
    pub fn new() -> Self {
        Self { schemas: HashMap::new(), validators: HashMap::new() }
    }

    // 🚫️async: R9 pure mutation — no I/O; same visit-closure consumers as `new()`.
    pub fn register_json(&mut self, id: &str, schema: Value) -> Result<(), SchemaError> {
        let validator = crate::OwnedJsonSchemaValidator::new(&schema)?;
        self.schemas.insert(id.to_string(), schema);
        self.validators.insert(id.to_string(), validator);
        Ok(())
    }

    /// 📥 Stores a handcrafted JSON Schema document without compiling a validator (catalog registration of normative leaves).
    // 🚫️async: R9 pure mutation — no I/O; called from inside the `with_kernel_*_catalog(|entries| ...)`
    // sync closures in `with_json_schema_catalog`/`with_inference_json_schema_catalog`/`with_app_json_schema_catalog` below.
    pub fn load_json(&mut self, id: &str, schema: Value) {
        self.schemas.insert(id.to_string(), schema);
    }

    // 🚫️async: R9 pure accessor — no I/O; same visit-closure consumers as `new()`.
    pub fn schema(&self, id: &str) -> Option<&Value> {
        self.schemas.get(id)
    }

    // 🚫️async: R9 pure accessor — no I/O (in-memory owned validator); same
    // visit-closure consumers as `new()`.
    pub fn validate(&self, id: &str, value: &Value) -> Result<(), SchemaError> {
        let validator = self.validators.get(id).ok_or_else(|| SchemaError::UnknownSchema(id.to_string()))?;
        validator.validate(value)
    }
}
//#endregion 🔖️SchemaCatalog

//#region 🔖️GraphQlStatePreamble
/// 🔗 Shared GraphQL `@state`/`@derived` SDL preamble — declared once, never repeated per artifact.
/// `@state` names one of the four state lanes; `@derived` is the ORTHOGONAL derivation marker, never
/// a fifth lane — a derived field is computed from a snapshot, so it is not state at all.
pub const GRAPHQL_STATE_PREAMBLE: &str = "\
enum StateClass { ARTIFACT CONFIG PRESENCE TRANSIENT }\n\
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
pub use semio_framework_os_kernel::os_schema_composition::{ArtifactCompositionFields, ChildFieldRefs, ChildRefFields, ChildRefVisitor, ChildSlotSpec, LinkSlotSpec};

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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FacetLeaves {
    pub rust: &'static str,
    pub typescript: &'static str,
    pub graphql: &'static str,
    pub json_schema: &'static str,
    pub proto: &'static str,
}

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

/// 🧬️ Registered descriptor for one artifact's four schema facets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactSchemaDescriptor {
    pub id: &'static str,
    pub artifact: FacetLeaves,
    pub snapshot: FacetLeaves,
    pub diff: FacetLeaves,
    pub mutations: FacetLeaves,
}

impl ArtifactSchemaDescriptor {
    pub fn artifact_schema_version(&self) -> Result<SchemaVersion, JsonError> {
        schema_version(self.artifact.json_schema)
    }

    pub fn snapshot_schema_version(&self) -> Result<SchemaVersion, JsonError> {
        schema_version(self.snapshot.json_schema)
    }

    pub fn diff_schema_version(&self) -> Result<SchemaVersion, JsonError> {
        schema_version(self.diff.json_schema)
    }

    pub fn mutations_schema_version(&self) -> Result<SchemaVersion, JsonError> {
        schema_version(self.mutations.json_schema)
    }
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
    // 🚫️async: R9 pure constructor — no I/O; every consumer is a synchronous `FnOnce` visit
    // closure (`with_artifact_schema_registry` below, whose own body loops via the wire crate's
    // fixed-signature `with_kernel_artifact_schema_catalog`, and the `semio-framework-plugin`
    // call site `with_artifact_schema_registry(|registry| registry.len())`, outside this packet's scope).
    pub fn new() -> Self {
        Self { by_id: HashMap::new() }
    }

    /// 📎 Insert or replace a descriptor by id.
    // 🚫️async: R9 pure mutation — no I/O; same visit-closure consumers as `new()`.
    pub fn register(&mut self, descriptor: ArtifactSchemaDescriptor) {
        self.by_id.insert(descriptor.id, descriptor);
    }

    /// 🔎 Lookup by artifact schema id.
    // 🚫️async: R9 pure accessor — no I/O; same visit-closure consumers as `new()`.
    pub fn get(&self, id: &str) -> Option<&ArtifactSchemaDescriptor> {
        self.by_id.get(id)
    }

    /// 🚶 Walk every registered descriptor.
    // 🚫️async: R9 pure accessor — no I/O; same visit-closure consumers as `new()`.
    pub fn iter(&self) -> impl Iterator<Item = &ArtifactSchemaDescriptor> {
        self.by_id.values()
    }

    /// 🔢 Count of registered artifact schema ids.
    // 🚫️async: R9 pure accessor — no I/O; the `semio-framework-plugin` call site
    // `with_artifact_schema_registry(|registry| registry.len())` reads the `usize` directly out of
    // a synchronous `FnOnce`, so this cannot become async without editing that crate (outside this packet's scope).
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    /// 📭 Whether no artifact schema ids are registered.
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }
}
//#endregion 🔖️ArtifactSchemaRegistry

//#region 🔖️GlobalArtifactSchemaCatalog
use semio_framework_os_kernel::{
    register_kernel_app_schema_descriptor, register_kernel_artifact_inference_descriptor, register_kernel_artifact_schema_descriptor, with_kernel_app_schema_catalog, with_kernel_artifact_inference_catalog, with_kernel_artifact_schema_catalog,
    KernelAppSchemaDescriptor, KernelArtifactInferenceDescriptor, KernelArtifactSchemaDescriptor, KernelFacetLeaves,
};

/// ⚠️ Schema descriptor registration rejects a conflicting established or batch descriptor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchemaDescriptorRegistryError {
    pub registry: &'static str,
    pub id: String,
}

impl std::fmt::Display for SchemaDescriptorRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} descriptor conflicts for {}", self.registry, self.id)
    }
}

impl std::error::Error for SchemaDescriptorRegistryError {}

fn facet_leaves_to_kernel(leaves: FacetLeaves) -> KernelFacetLeaves {
    KernelFacetLeaves { rust: leaves.rust, typescript: leaves.typescript, graphql: leaves.graphql, json_schema: leaves.json_schema, proto: leaves.proto }
}

// 🚫️async: R9 pure conversion — no I/O; only consumer is `descriptor_from_kernel` below, itself
// forced sync by the wire crate's fixed-signature `with_kernel_artifact_schema_catalog` closure.
fn facet_leaves_from_kernel(leaves: &KernelFacetLeaves) -> FacetLeaves {
    FacetLeaves { rust: leaves.rust, typescript: leaves.typescript, graphql: leaves.graphql, json_schema: leaves.json_schema, proto: leaves.proto }
}

fn descriptor_to_kernel(descriptor: &ArtifactSchemaDescriptor) -> KernelArtifactSchemaDescriptor {
    KernelArtifactSchemaDescriptor {
        id: descriptor.id,
        artifact: facet_leaves_to_kernel(descriptor.artifact),
        snapshot: facet_leaves_to_kernel(descriptor.snapshot),
        diff: facet_leaves_to_kernel(descriptor.diff),
        mutations: facet_leaves_to_kernel(descriptor.mutations),
    }
}

// 🚫️async: R9 pure conversion — no I/O; called from inside the synchronous `FnOnce` closure
// `with_artifact_schema_registry` hands to `semio-framework-replication`'s
// `with_kernel_artifact_schema_catalog` (fixed signature outside this packet's scope).
fn descriptor_from_kernel(kernel: &KernelArtifactSchemaDescriptor) -> ArtifactSchemaDescriptor {
    ArtifactSchemaDescriptor {
        id: kernel.id,
        artifact: facet_leaves_from_kernel(&kernel.artifact),
        snapshot: facet_leaves_from_kernel(&kernel.snapshot),
        diff: facet_leaves_from_kernel(&kernel.diff),
        mutations: facet_leaves_from_kernel(&kernel.mutations),
    }
}

// 🚫️async: R9 pure parse — no I/O (owned JSON parser over an already-loaded `&str`); called
// from inside the same sync `with_kernel_*_catalog` visit closures as `descriptor_from_kernel`.
fn parse_normative_json_leaf(descriptor_id: &str, facet: &str, body: &str) -> Value {
    parse_json(body).unwrap_or_else(|error| panic!("{descriptor_id}: {facet} json_schema parse: {error}"))
}

// 🚫️async: R9 pure formatting — no I/O; called from inside the sync `with_kernel_*_catalog` visit
// closures in `artifact_schema_graphql_sdl`/`artifact_inference_graphql_sdl`/`app_schema_graphql_sdl` below.
fn graphql_leaf_with_preamble(body: &str) -> String {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return GRAPHQL_STATE_PREAMBLE.to_string();
    }
    format!("{}\n\n{trimmed}", GRAPHQL_STATE_PREAMBLE)
}

/// 📎 Registers one artifact's handcrafted descriptor into the OS-wide catalog (kernel descriptors + normative JSON + GraphQL SDL).
pub fn register_artifact_schema_descriptor(descriptor: ArtifactSchemaDescriptor) {
    register_kernel_artifact_schema_descriptor(descriptor_to_kernel(&descriptor));
}

/// 🔬️ Verifies artifact schema descriptors against the established catalog without mutation.
pub fn preflight_artifact_schema_descriptors(descriptors: &[ArtifactSchemaDescriptor]) -> Result<(), SchemaDescriptorRegistryError> {
    let mut proposed = HashMap::new();
    for descriptor in descriptors {
        match proposed.insert(descriptor.id, descriptor) {
            Some(existing) if existing == descriptor => {}
            Some(_) => return Err(SchemaDescriptorRegistryError { registry: "artifact-schema", id: descriptor.id.to_string() }),
            None => {}
        }
    }
    with_artifact_schema_registry(|registry| {
        for descriptor in descriptors {
            if let Some(existing) = registry.get(descriptor.id) {
                if existing != descriptor {
                    return Err(SchemaDescriptorRegistryError { registry: "artifact-schema", id: descriptor.id.to_string() });
                }
            }
        }
        Ok(())
    })
}

/// 📌️ Registers an atomically prevalidated artifact schema batch.
pub fn register_artifact_schema_descriptors(descriptors: Vec<ArtifactSchemaDescriptor>) -> Result<(), SchemaDescriptorRegistryError> {
    preflight_artifact_schema_descriptors(&descriptors)?;
    for descriptor in descriptors {
        if !artifact_schema_descriptor_registered(descriptor.id) {
            register_artifact_schema_descriptor(descriptor);
        }
    }
    Ok(())
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
pub async fn with_json_schema_catalog<R>(visit: impl FnOnce(&SchemaCatalog) -> R) -> R {
    let mut catalog = SchemaCatalog::new();
    with_kernel_artifact_schema_catalog(|entries| {
        for entry in entries {
            catalog.load_json(entry.id, parse_normative_json_leaf(entry.id, "artifact", entry.artifact.json_schema));
        }
    });
    visit(&catalog)
}

/// 🔗 Returns composed GraphQL SDL (shared `@state` preamble + facet leaf) for a catalog key (`id`, `{id}.snapshot`, `{id}.diff`).
pub async fn artifact_schema_graphql_sdl(key: &str) -> Option<String> {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactInferenceDescriptor {
    pub id: &'static str,
    pub inference: FacetLeaves,
}

fn inference_descriptor_to_kernel(descriptor: &ArtifactInferenceDescriptor) -> KernelArtifactInferenceDescriptor {
    KernelArtifactInferenceDescriptor { id: descriptor.id, inference: facet_leaves_to_kernel(descriptor.inference) }
}

// 🚫️async: R9 pure conversion — no I/O; called from inside the synchronous `FnOnce` closure
// `with_artifact_inference_registry` hands to `with_kernel_artifact_inference_catalog` (fixed
// signature outside this packet's scope).
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
    // 🚫️async: R9 pure constructor — no I/O; consumed only through the synchronous `FnOnce`
    // visit closure `with_artifact_inference_registry` below.
    pub fn new() -> Self {
        Self { by_id: HashMap::new() }
    }

    // 🚫️async: R9 pure mutation — no I/O; same visit-closure consumer as `new()`.
    pub fn register(&mut self, descriptor: ArtifactInferenceDescriptor) {
        self.by_id.insert(descriptor.id, descriptor);
    }

    // 🚫️async: R9 pure accessor — no I/O; same visit-closure consumer as `new()`.
    pub fn get(&self, id: &str) -> Option<&ArtifactInferenceDescriptor> {
        self.by_id.get(id)
    }

    // 🚫️async: R9 pure accessor — no I/O; same visit-closure consumer as `new()`.
    pub fn iter(&self) -> impl Iterator<Item = &ArtifactInferenceDescriptor> {
        self.by_id.values()
    }

    // 🚫️async: R9 pure accessor — no I/O; same visit-closure consumer as `new()`.
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    // 🚫️async: R9 pure accessor — no I/O; same visit-closure consumer as `new()`.
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }
}

/// 📎 Registers one artifact's handcrafted inference descriptor into the OS-wide catalog. `id` on
/// the descriptor must be `"{artifact_id}.inference"`, matching its owning `ArtifactSchemaDescriptor`'s id.
pub fn register_artifact_inference_descriptor(descriptor: ArtifactInferenceDescriptor) {
    register_kernel_artifact_inference_descriptor(inference_descriptor_to_kernel(&descriptor));
}

/// 🔬️ Verifies inference schema descriptors against the established catalog without mutation.
pub fn preflight_artifact_inference_descriptors(descriptors: &[ArtifactInferenceDescriptor]) -> Result<(), SchemaDescriptorRegistryError> {
    let mut proposed = HashMap::new();
    for descriptor in descriptors {
        match proposed.insert(descriptor.id, descriptor) {
            Some(existing) if existing == descriptor => {}
            Some(_) => return Err(SchemaDescriptorRegistryError { registry: "artifact-inference", id: descriptor.id.to_string() }),
            None => {}
        }
    }
    with_artifact_inference_registry(|registry| {
        for descriptor in descriptors {
            if let Some(existing) = registry.get(descriptor.id) {
                if existing != descriptor {
                    return Err(SchemaDescriptorRegistryError { registry: "artifact-inference", id: descriptor.id.to_string() });
                }
            }
        }
        Ok(())
    })
}

/// 📌️ Registers an atomically prevalidated inference schema batch.
pub fn register_artifact_inference_descriptors(descriptors: Vec<ArtifactInferenceDescriptor>) -> Result<(), SchemaDescriptorRegistryError> {
    preflight_artifact_inference_descriptors(&descriptors)?;
    for descriptor in descriptors {
        if !artifact_inference_descriptor_registered(descriptor.id) {
            register_artifact_inference_descriptor(descriptor);
        }
    }
    Ok(())
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
pub async fn with_inference_json_schema_catalog<R>(visit: impl FnOnce(&SchemaCatalog) -> R) -> R {
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
pub async fn artifact_inference_graphql_sdl(key: &str) -> Option<String> {
    with_kernel_artifact_inference_catalog(|entries| entries.iter().find(|entry| entry.id == key).map(|entry| graphql_leaf_with_preamble(entry.inference.graphql)))
}
//#endregion 🔖️ArtifactInferenceDescriptor

//#region 🔖️AppSchemaDescriptor
/// 🧬️ Registered descriptor for one app owner's config + presence schema facets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppSchemaDescriptor {
    pub id: &'static str,
    pub config: FacetLeaves,
    pub presence: FacetLeaves,
}

impl AppSchemaDescriptor {
    pub fn config_schema_version(&self) -> Result<SchemaVersion, JsonError> {
        schema_version(self.config.json_schema)
    }

    pub fn presence_schema_version(&self) -> Result<SchemaVersion, JsonError> {
        schema_version(self.presence.json_schema)
    }
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
    // 🚫️async: R9 pure constructor — no I/O; consumed only through the synchronous `FnOnce`
    // visit closure `with_app_schema_registry` below.
    pub fn new() -> Self {
        Self { by_id: HashMap::new() }
    }

    /// 📎 Insert or replace a descriptor by owner id.
    // 🚫️async: R9 pure mutation — no I/O; same visit-closure consumer as `new()`.
    pub fn register(&mut self, descriptor: AppSchemaDescriptor) {
        self.by_id.insert(descriptor.id, descriptor);
    }

    /// 🔎 Lookup by app schema owner id.
    // 🚫️async: R9 pure accessor — no I/O; same visit-closure consumer as `new()`.
    pub fn get(&self, id: &str) -> Option<&AppSchemaDescriptor> {
        self.by_id.get(id)
    }

    /// 🚶 Walk every registered descriptor.
    // 🚫️async: R9 pure accessor — no I/O; same visit-closure consumer as `new()`.
    pub fn iter(&self) -> impl Iterator<Item = &AppSchemaDescriptor> {
        self.by_id.values()
    }

    /// 🔢 Count of registered app schema owner ids.
    // 🚫️async: R9 pure accessor — no I/O; same visit-closure consumer as `new()`.
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    /// 📭 Whether no owners are registered yet (A6 fills the catalog).
    // 🚫️async: R9 pure accessor — no I/O; same visit-closure consumer as `new()`.
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }
}
//#endregion 🔖️AppSchemaRegistry

//#region 🔖️GlobalAppSchemaCatalog
async fn app_descriptor_to_kernel(descriptor: &AppSchemaDescriptor) -> KernelAppSchemaDescriptor {
    KernelAppSchemaDescriptor { id: descriptor.id, config: facet_leaves_to_kernel(descriptor.config), presence: facet_leaves_to_kernel(descriptor.presence) }
}

// 🚫️async: R9 pure conversion — no I/O; called from inside the synchronous `FnOnce` closure
// `with_app_schema_registry` hands to `with_kernel_app_schema_catalog` (fixed signature outside
// this packet's scope).
fn app_descriptor_from_kernel(kernel: &KernelAppSchemaDescriptor) -> AppSchemaDescriptor {
    AppSchemaDescriptor { id: kernel.id, config: facet_leaves_from_kernel(&kernel.config), presence: facet_leaves_from_kernel(&kernel.presence) }
}

/// 🔌 Open app-schema registry API for plugin crates — call these from your own `🔧️setup`/init code to register your app's config + presence schema facets. Every app owner self-registers via [`register_app_schema_descriptor`]; there is no closed framework-side catalog.
///
/// - 📎 [`register_app_schema_descriptor`] registers one app owner's handcrafted descriptor into the OS-wide catalog.
/// - 🔎 [`app_schema_descriptor_registered`] checks whether an owner id is already registered.
/// - 📚 [`with_app_schema_registry`] snapshots the OS-wide [`AppSchemaRegistry`] for lookup/iteration.
/// - 🔣 [`with_app_json_schema_catalog`] snapshots normative config/presence JSON leaves as a [`SchemaCatalog`].
/// - 🔗 [`app_schema_graphql_sdl`] resolves composed GraphQL SDL for an owner or `{id}.presence` key.
/// - ✅ [`validate_registered_app_descriptor`] validates a descriptor's JSON Schema leaves and `x-semio-state` tagging before registering.
pub async fn register_app_schema_descriptor(descriptor: AppSchemaDescriptor) {
    register_kernel_app_schema_descriptor(app_descriptor_to_kernel(&descriptor).await);
}

/// 🔬️ Verifies app schema descriptors against the established catalog without mutation.
pub async fn preflight_app_schema_descriptors(descriptors: &[AppSchemaDescriptor]) -> Result<(), SchemaDescriptorRegistryError> {
    let mut proposed = HashMap::new();
    for descriptor in descriptors {
        match proposed.insert(descriptor.id, descriptor) {
            Some(existing) if existing == descriptor => {}
            Some(_) => return Err(SchemaDescriptorRegistryError { registry: "app-schema", id: descriptor.id.to_string() }),
            None => {}
        }
    }
    with_app_schema_registry(|registry| {
        for descriptor in descriptors {
            if let Some(existing) = registry.get(descriptor.id) {
                if existing != descriptor {
                    return Err(SchemaDescriptorRegistryError { registry: "app-schema", id: descriptor.id.to_string() });
                }
            }
        }
        Ok(())
    })
    .await
}

/// 📌️ Registers an atomically prevalidated app schema batch.
pub async fn register_app_schema_descriptors(descriptors: Vec<AppSchemaDescriptor>) -> Result<(), SchemaDescriptorRegistryError> {
    preflight_app_schema_descriptors(&descriptors).await?;
    for descriptor in descriptors {
        if !app_schema_descriptor_registered(descriptor.id).await {
            register_app_schema_descriptor(descriptor).await;
        }
    }
    Ok(())
}

/// 🔎 Whether `id` is present in the OS-wide app descriptor registry.
pub async fn app_schema_descriptor_registered(id: &str) -> bool {
    semio_framework_os_kernel::kernel_app_schema_descriptor_registered(id)
}

/// 📚 Invokes `visit` with the OS-wide [`AppSchemaRegistry`] snapshot.
pub async fn with_app_schema_registry<R>(visit: impl FnOnce(&AppSchemaRegistry) -> R) -> R {
    let mut registry = AppSchemaRegistry::new();
    with_kernel_app_schema_catalog(|entries| {
        for entry in entries {
            registry.register(app_descriptor_from_kernel(entry));
        }
    });
    visit(&registry)
}

/// 🔣 Invokes `visit` with a [`SchemaCatalog`] of normative app config JSON leaves.
pub async fn with_app_json_schema_catalog<R>(visit: impl FnOnce(&SchemaCatalog) -> R) -> R {
    let mut catalog = SchemaCatalog::new();
    with_kernel_app_schema_catalog(|entries| {
        for entry in entries {
            catalog.load_json(entry.id, parse_normative_json_leaf(entry.id, "config", entry.config.json_schema));
            catalog.load_json(&format!("{}.presence", entry.id), parse_normative_json_leaf(entry.id, "presence", entry.presence.json_schema));
        }
    });
    visit(&catalog)
}

/// 🔗 Returns composed GraphQL SDL (shared `@state` preamble + facet leaf) for an app catalog key (`id`, `{id}.presence`).
pub async fn app_schema_graphql_sdl(key: &str) -> Option<String> {
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

/// ✅ Validates a descriptor's JSON Schema leaves: each non-empty facet must be an object schema whose properties all carry a valid `x-semio-state` matching the facet's expected [`StateClass`] (`config` for config, `presence` for presence). Panics with a descriptor-id-prefixed message on the first violation — call this from a plugin's own tests before [`register_app_schema_descriptor`].
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
//#endregion 🔖️GlobalAppSchemaCatalog

//#region 🔖️StateClassKebab
/// 🏷️ Parse the canonical kebab `x-semio-state` string into [`StateClass`].
///
/// Lives here (not a second enum) so JSON Schema leaves can be checked against the kernel enum
/// without inventing a parallel source of truth. The kernel already owns [`StateClass`].
// 🚫️async: R9 pure parse — no I/O; `✏️s/🔌️plugins/💠️lowpoly` calls this synchronously inside an
// `Iterator::map` closure (`parse_state_class_kebab(raw).expect("parse")`), a language-barred
// consumer outside this packet's writable scope.
pub fn parse_state_class_kebab(value: &str) -> Option<StateClass> {
    match value {
        "artifact" => Some(StateClass::Artifact),
        "config" => Some(StateClass::Config),
        "presence" => Some(StateClass::Presence),
        "transient" => Some(StateClass::Transient),
        _ => None,
    }
}

/// 🏷️ Canonical kebab spelling of a [`StateClass`] for JSON Schema `x-semio-state`.
pub async fn state_class_kebab(class: StateClass) -> &'static str {
    match class {
        StateClass::Artifact => "artifact",
        StateClass::Config => "config",
        StateClass::Presence => "presence",
        StateClass::Transient => "transient",
    }
}
//#endregion 🔖️StateClassKebab

#[cfg(test)]
//#region 🔖️Tests
mod tests {
    use super::*;

    //#region 🔖️SyntheticArtifact
    #[derive(Clone, Debug, PartialEq, ArtifactSchema)]
    #[artifact_schema(id = "s.wave3.synthetic")]
    struct SyntheticArtifact {
        #[state(artifact)]
        schema: String,
        #[state(artifact)]
        label: String,
        #[state(presence)]
        active_id: Option<String>,
    }

    #[derive(Clone, Debug, Default, PartialEq, ArtifactSchema)]
    #[artifact_schema(id = "s.wave3.synthetic")]
    struct SyntheticSnapshot {
        #[state(artifact)]
        schema: String,
        #[state(artifact)]
        label: String,
    }

    const SYNTHETIC_SNAPSHOT_JSON_SCHEMA: &str = r#"{
  "$id": "https://semio.tech/schema/s/wave3/synthetic/snapshot.json",
  "title": "SyntheticSnapshot",
  "type": "object",
  "additionalProperties": false,
  "required": ["schema", "label"],
  "properties": {
    "schema": { "type": "string", "x-semio-state": "artifact" },
    "label": { "type": "string", "x-semio-state": "artifact" }
  }
}"#;

    async fn synthetic_descriptor() -> ArtifactSchemaDescriptor {
        let empty = FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" };
        ArtifactSchemaDescriptor { id: "s.wave3.synthetic", artifact: empty, snapshot: FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: SYNTHETIC_SNAPSHOT_JSON_SCHEMA, proto: "" }, diff: empty, mutations: empty }
    }

    async fn expected_snapshot_title(id: &str) -> String {
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

    impl<T> ChildFieldRefs for ArtifactChild<T> {
        const MANY: bool = false;
        fn visit_child_field<'a, V: ChildRefVisitor<'a>>(&'a self, slot: &'static str, visitor: &mut V) -> Result<(), V::Error> {
            visitor.step()?;
            visitor.child(slot, ChildRefFields { child_id: "child", artifact_id: "child", artifact_kind: "s.stdio.mesh", standard: "v1", subset: "*" })
        }
    }

    type AliasedChild = ArtifactChild<()>;

    #[derive(ArtifactSchema)]
    #[artifact_schema(id = "s.test.aliased-composition")]
    struct AliasedComposition {
        #[state(artifact)]
        #[child(kind = "s.stdio.mesh")]
        optional_child: Option<Option<AliasedChild>>,
        #[state(artifact)]
        #[child(kind = "s.stdio.mesh")]
        children: Vec<Option<AliasedChild>>,
    }

    #[expect(dead_code, reason = "the derive test inspects the field declarations without constructing this schema-only fixture")]
    #[derive(ArtifactSchema)]
    #[artifact_schema(id = "s.wave3.composite")]
    struct CompositeArtifact {
        #[state(artifact)]
        #[child(kind = "s.stdio.mesh")]
        primary_mesh: ArtifactChild<()>,
        #[state(artifact)]
        #[child(kind = "s.stdio.image")]
        textures: Vec<ArtifactChild<()>>,
        #[state(artifact)]
        #[link_slot(roles("base", "material"))]
        base_material: ArtifactLink,
        #[state(artifact)]
        label: String,
    }
    //#endregion 🔖️ArtifactCompositionFixture

    #[semio_framework_async_macros::async_test]
    async fn artifact_composition_fields_derive_emits_expected_slot_tables() {
        let children = CompositeArtifact::child_slots();
        assert_eq!(children.len(), 2, "single child + Vec child must both be captured, plain field must not");
        assert_eq!(children[0], ChildSlotSpec { name: "primaryMesh", kind: "s.stdio.mesh", many: false });
        assert_eq!(children[1], ChildSlotSpec { name: "textures", kind: "s.stdio.image", many: true });

        let links = CompositeArtifact::link_slots();
        assert_eq!(links.len(), 1, "only the ArtifactLink field must be captured");
        assert_eq!(links[0], LinkSlotSpec { name: "baseMaterial", roles: &["base", "material"], many: false });
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_composition_fields_default_to_empty_for_leaf_artifacts() {
        assert!(SyntheticSnapshot::child_slots().is_empty());
        assert!(SyntheticSnapshot::link_slots().is_empty());
        assert_eq!(SyntheticSnapshot::artifact_schema_id().await, "s.wave3.synthetic");
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_composition_projection_walks_aliases_nested_options_and_cancels() {
        struct Visitor { steps: usize, maximum_steps: usize, rows: Vec<(&'static str, &'static str)> }
        impl<'a> ChildRefVisitor<'a> for Visitor {
            type Error = ();
            fn step(&mut self) -> Result<(), ()> {
                if self.steps == self.maximum_steps { return Err(()); }
                self.steps += 1;
                Ok(())
            }
            fn child(&mut self, slot: &'static str, fields: ChildRefFields<'a>) -> Result<(), ()> {
                assert_eq!(fields.child_id, fields.artifact_id);
                self.rows.push((slot, "child"));
                Ok(())
            }
        }
        let slots = AliasedComposition::child_slots();
        assert_eq!(slots, &[ChildSlotSpec { name: "optionalChild", kind: "s.stdio.mesh", many: false }, ChildSlotSpec { name: "children", kind: "s.stdio.mesh", many: true }]);
        let snapshot = AliasedComposition { optional_child: Some(Some(ArtifactChild { _marker: std::marker::PhantomData })), children: vec![None, Some(ArtifactChild { _marker: std::marker::PhantomData })] };
        let mut visitor = Visitor { steps: 0, maximum_steps: 16, rows: Vec::new() };
        snapshot.visit_child_refs(&mut visitor).unwrap();
        assert_eq!(visitor.rows, [("optionalChild", "child"), ("children", "child")]);
        assert_eq!(visitor.steps, 7);
        let mut visitor = Visitor { steps: 0, maximum_steps: 4, rows: Vec::new() };
        assert!(snapshot.visit_child_refs(&mut visitor).is_err());
        assert_eq!(visitor.steps, 4);
        assert_eq!(visitor.rows, [("optionalChild", "child")]);
        eprintln!("[DEBUG] schema child visitor: alias, nested option, collection and bounded early-stop assertions");
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_composition_projection_real_child_alias_has_fixed_admission_bounds() {
        use semio_framework_os_kernel::{ArtifactChild, ChildRestoreProjection, ChildRestoreProjectionError};
        type ChildAlias = ArtifactChild<()>;
        #[derive(semio_framework_schema::ArtifactSchema)]
        #[artifact_schema(id = "s.test.parent")]
        struct DerivedParent {
            #[state(artifact)]
            #[child(kind = "s.test.member")]
            many: Vec<Option<ChildAlias>>,
        }
        let child = |id: String| Some(ArtifactChild::new(id.clone(), semio_framework_os_kernel::os_io::ArtifactRef { artifact_id: id, dialect: semio_framework_os_kernel::os_io::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "v1".into(), subset: "first".into() } }));
        let mut parent = DerivedParent { many: (0..64).map(|index| child(index.to_string())).collect() };
        assert_eq!(ChildRestoreProjection::from_snapshot(&parent).unwrap().len(), 64);
        parent.many.push(child("overflow".into()));
        assert!(matches!(ChildRestoreProjection::from_snapshot(&parent), Err(ChildRestoreProjectionError::ReferenceLimit)));
        parent.many = (0..257).map(|_| None).collect();
        assert!(matches!(ChildRestoreProjection::from_snapshot(&parent), Err(ChildRestoreProjectionError::TraversalLimit)));
        parent.many = vec![child("ä".repeat(128))];
        assert!(ChildRestoreProjection::from_snapshot(&parent).is_ok());
        parent.many = vec![child(format!("{}x", "ä".repeat(128)))];
        assert!(matches!(ChildRestoreProjection::from_snapshot(&parent), Err(ChildRestoreProjectionError::InvalidReference)));
        eprintln!("[DEBUG] real derived child projection: 64/65 references, sparse traversal and 256/257 UTF-8 byte boundaries");
    }

    #[semio_framework_async_macros::async_test]
    async fn registry_descriptors_carry_valid_snapshot_state_and_match_field_states() {
        let mut registry = ArtifactSchemaRegistry::new();
        registry.register(synthetic_descriptor().await);

        let mut walked = 0usize;
        for descriptor in registry.iter() {
            walked += 1;
            let schema = parse_json(descriptor.snapshot.json_schema).unwrap_or_else(|error| panic!("{}: snapshot json_schema parse: {error}", descriptor.id));
            let title = schema.get("title").and_then(Value::as_str).unwrap_or("");
            assert_eq!(title, expected_snapshot_title(descriptor.id).await, "{}: snapshot title must be XSnapshot for id", descriptor.id);

            let properties = schema.get("properties").and_then(Value::as_object).unwrap_or_else(|| panic!("{}: snapshot properties object required", descriptor.id));

            let mut json_states = Vec::new();
            for (name, prop) in properties {
                let raw = prop.get("x-semio-state").and_then(Value::as_str).unwrap_or_else(|| panic!("{}: property `{name}` missing x-semio-state", descriptor.id));
                let class = parse_state_class_kebab(raw).unwrap_or_else(|| panic!("{}: property `{name}` has invalid x-semio-state `{raw}`", descriptor.id));
                json_states.push((name.to_string(), class));
            }
            json_states.sort_by(|a, b| a.0.cmp(&b.0));

            let mut derived: Vec<(String, StateClass)> = SyntheticSnapshot::field_states().await.iter().map(|(name, class)| ((*name).to_string(), *class)).collect();
            derived.sort_by(|a, b| a.0.cmp(&b.0));
            assert_eq!(derived, json_states, "{}: field_states() must agree with snapshot JSON x-semio-state", descriptor.id);
            assert_eq!(SyntheticSnapshot::artifact_schema_id().await, descriptor.id);
            assert_eq!(SyntheticArtifact::artifact_schema_id().await, descriptor.id);
        }
        assert_eq!(walked, 1, "registry must be walked for the synthetic descriptor");
        assert!(registry.get("s.wave3.synthetic").is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn graphql_state_preamble_matches_normative_sdl() {
        assert!(GRAPHQL_STATE_PREAMBLE.contains("enum StateClass { ARTIFACT CONFIG PRESENCE TRANSIENT }"));
        assert!(GRAPHQL_STATE_PREAMBLE.contains("directive @state(class: StateClass!) on FIELD_DEFINITION"));
        assert!(GRAPHQL_STATE_PREAMBLE.contains("directive @derived on FIELD_DEFINITION"));
    }

    #[semio_framework_async_macros::async_test]
    async fn state_class_kebab_round_trips_exactly_the_four_lanes() {
        for class in [StateClass::Artifact, StateClass::Config, StateClass::Presence, StateClass::Transient] {
            let kebab = state_class_kebab(class).await;
            assert_eq!(parse_state_class_kebab(kebab), Some(class));
        }
        assert_eq!(state_class_kebab(StateClass::Artifact).await, "artifact");
        assert_eq!(state_class_kebab(StateClass::Config).await, "config");
        assert_eq!(state_class_kebab(StateClass::Presence).await, "presence");
        assert_eq!(state_class_kebab(StateClass::Transient).await, "transient");
    }

    #[semio_framework_async_macros::async_test]
    async fn retired_state_vocabulary_no_longer_parses() {
        for retired in ["persistent", "shared-ui", "local-ui", "preview", "effect", "inferred", "identity"] {
            assert_eq!(parse_state_class_kebab(retired), None, "`{retired}` must not resolve to a state lane");
        }
    }

    //#region 🔖️DerivedAxis
    /// 💡️ Derivation travels on its own axis: `#[derived]` fields carry no [`StateClass`] and are
    /// reported by `derived_fields()`, never by `field_states()`.
    #[derive(Clone, Debug, PartialEq, ArtifactSchema)]
    #[artifact_schema(id = "s.wave3.synthetic.inference")]
    struct SyntheticInference {
        #[derived]
        topology: String,
        #[derived]
        depth: u32,
    }

    #[semio_framework_async_macros::async_test]
    async fn derived_fields_leave_the_state_class_axis_entirely() {
        assert!(SyntheticInference::field_states().await.is_empty(), "a #[derived] field is not state");
        assert_eq!(SyntheticInference::derived_fields().await, &["topology", "depth"]);
        assert_eq!(SyntheticInference::artifact_schema_id().await, "s.wave3.synthetic.inference");
        assert!(SyntheticSnapshot::derived_fields().await.is_empty(), "state-only structs derive an empty derived table");
        assert_eq!(JSON_SCHEMA_DERIVED_KEY, "x-semio-derived");
    }
    //#endregion 🔖️DerivedAxis

    #[semio_framework_async_macros::async_test]
    async fn schema_catalog_still_registers_json() {
        let mut catalog = SchemaCatalog::new();
        catalog.register_json("probe", parse_json(r#"{"type":"object","properties":{"n":{"type":"integer"}}}"#).expect("schema json")).expect("register");
        catalog.validate("probe", &parse_json(r#"{"n":1}"#).expect("probe json")).expect("validate");
        assert!(catalog.validate("probe", &parse_json(r#"{"n":1.5}"#).expect("fractional probe json")).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn owned_validator_preserves_supported_keyword_corpus() {
        let schema_text = r#"{
            "type":"object",
            "additionalProperties":false,
            "required":["n"],
            "properties":{
                "n":{"type":"integer"},
                "mode":{"enum":["a","b"]},
                "rank":{"enum":[1,2]},
                "enabled":{"type":"boolean"},
                "nested":{"type":"object","additionalProperties":false,"properties":{"label":{"type":"string"}}}
            }
        }"#;
        let mut owned = SchemaCatalog::new();
        owned.register_json("probe", parse_json(schema_text).expect("owned schema")).expect("owned compile");
        let corpus = [
            (r#"{"n":1}"#, true),
            (r#"{"n":1.0}"#, true),
            (r#"{"n":-2,"mode":"a","enabled":true}"#, true),
            (r#"{"n":7,"nested":{"label":"ok"}}"#, true),
            (r#"{"n":7,"rank":1.0}"#, true),
            (r#"{}"#, false),
            (r#"{"n":1.5}"#, false),
            (r#"{"n":"1"}"#, false),
            (r#"{"n":1,"mode":"c"}"#, false),
            (r#"{"n":1,"rank":1.5}"#, false),
            (r#"{"n":1,"enabled":0}"#, false),
            (r#"{"n":1,"extra":true}"#, false),
            (r#"{"n":1,"nested":{"extra":true}}"#, false),
            (r#"null"#, false),
            (r#"[]"#, false),
        ];
        for (text, expected) in corpus {
            let owned_value = parse_json(text).expect("owned value");
            assert_eq!(owned.validate("probe", &owned_value).is_ok(), expected, "unexpected validator outcome for {text}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn owned_validator_preserves_every_exercised_keyword_family() {
        let schemas = [
            r##"{"$defs":{"name":{"type":"string","minLength":2,"maxLength":4}},"type":"object","properties":{"name":{"$ref":"#/$defs/name"}},"required":["name"],"additionalProperties":{"type":"integer"}}"##,
            r#"{"type":"array","items":{"type":["string","null"]},"minItems":1,"maxItems":3,"uniqueItems":true}"#,
            r#"{"allOf":[{"type":"number","minimum":0,"maximum":10},{"multipleOf":0.5}],"not":{"const":3.5}}"#,
            r#"{"anyOf":[{"const":"automatic"},{"type":"integer","exclusiveMinimum":0,"exclusiveMaximum":4}]}"#,
            r#"{"oneOf":[{"const":"left"},{"const":"right"}],"title":"side","description":"side choice","default":"left","examples":["right"],"readOnly":false,"writeOnly":false,"deprecated":false,"format":"semio-side","x-semio-kind":"choice"}"#,
        ];
        let corpora: [&[&str]; 5] = [
            &[r#"{"name":"ab"}"#, r#"{"name":"abcd","rank":2}"#, r#"{"name":"a"}"#, r#"{"name":"abcde"}"#, r#"{"name":"ok","rank":"2"}"#, r#"{}"#],
            &[r#"["a"]"#, r#"[null,"b"]"#, r#"[]"#, r#"["a","b","c","d"]"#, r#"["a","a"]"#, r#"[1]"#],
            &["0", "2.5", "10", "-0.5", "10.5", "2.25", "3.5", r#""2.5""#],
            &[r#""automatic""#, "1", "3", "0", "4", r#""manual""#],
            &[r#""left""#, r#""right""#, r#""center""#, "null"],
        ];
        let outcomes: [&[bool]; 5] =
            [&[true, true, false, false, false, false], &[true, true, false, false, false, false], &[true, true, true, false, false, false, false, false], &[true, true, true, false, false, false], &[true, true, false, false]];
        for ((schema_text, corpus), expected) in schemas.into_iter().zip(corpora).zip(outcomes) {
            let owned = crate::OwnedJsonSchemaValidator::compile(schema_text).expect("owned compile");
            for (text, expected) in corpus.iter().zip(expected) {
                assert_eq!(owned.is_valid_json(text), *expected, "unexpected validator outcome for schema {schema_text} and value {text}");
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn owned_validator_diagnostics_progress_and_cancellation_are_deterministic() {
        let validator = crate::OwnedJsonSchemaValidator::compile(r#"{"type":"object","properties":{"n":{"type":"integer"}},"required":["n"],"additionalProperties":false}"#).expect("compile");
        assert_eq!(validator.validate_json(r#"{"n":"wrong"}"#), Err(SchemaError::Validation("$.n: expected integer".to_string())));
        assert_eq!(validator.validate_json("{}"), Err(SchemaError::Validation("$: missing required property `n`".to_string())));
        assert_eq!(validator.validate_json(r#"{"n":1,"z":2}"#), Err(SchemaError::Validation("$: additional property `z` is not allowed".to_string())));

        let progress = validator.validate_json(r#"{"n":2}"#).expect("valid");
        assert_eq!(progress.visited_nodes, 2);
        let limited = crate::ValidationControl::new(1);
        assert_eq!(validator.validate_json_with_control(r#"{"n":2}"#, &limited), Err(SchemaError::LimitExceeded(1)));
        let cancelled = crate::ValidationControl::default();
        cancelled.cancel();
        assert_eq!(validator.validate_json_with_control(r#"{"n":2}"#, &cancelled), Err(SchemaError::Cancelled));
    }

    #[semio_framework_async_macros::async_test]
    async fn schema_versions_ignore_whitespace_and_detect_drift() {
        let compact = schema_version(r#"{"type":"object","properties":{"n":{"type":"integer"}}}"#).expect("compact");
        let spaced = schema_version(r#"{ "type": "object", "properties": { "n": { "type": "integer" } } }"#).expect("spaced");
        let reordered = schema_version(r#"{"properties":{"n":{"type":"integer"}},"type":"object"}"#).expect("reordered");
        let drifted = schema_version(r#"{"type":"object","required":["n"],"properties":{"n":{"type":"integer"}}}"#).expect("drifted");
        assert_eq!(compact, spaced);
        assert_eq!(compact, reordered);
        assert_ne!(compact, drifted);
    }

    //#region 🔖️ArtifactInferenceDescriptorParity
    #[semio_framework_async_macros::async_test]
    async fn artifact_inference_registry_registers_independently_of_the_snapshot_diff_mutations_descriptor() {
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

    #[semio_framework_async_macros::async_test]
    async fn artifact_inference_graphql_sdl_composes_shared_preamble_with_facet_leaf() {
        register_artifact_inference_descriptor(ArtifactInferenceDescriptor {
            id: "s.wave3.synthetic.sdl-probe.inference",
            inference: FacetLeaves { rust: "", typescript: "", graphql: "type SdlProbeInference { flag: Boolean }", json_schema: "", proto: "" },
        });
        assert!(artifact_inference_descriptor_registered("s.wave3.synthetic.sdl-probe.inference"));
        let sdl = artifact_inference_graphql_sdl("s.wave3.synthetic.sdl-probe.inference").await.expect("registered inference sdl");
        assert!(sdl.contains("TRANSIENT"), "composed SDL must carry the shared @state preamble");
        assert!(sdl.contains("type SdlProbeInference"));
        assert!(artifact_inference_graphql_sdl("s.wave3.synthetic.unregistered.inference").await.is_none());
    }
    //#endregion 🔖️ArtifactInferenceDescriptorParity

    //#region 🔖️AppSchemaRegistryParity

    async fn empty_app_facet_leaves() -> FacetLeaves {
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

    #[semio_framework_async_macros::async_test]
    async fn app_schema_registry_accepts_placeholder_owner_for_wave_structure() {
        let mut registry = AppSchemaRegistry::new();
        let empty = empty_app_facet_leaves().await;
        registry.register(AppSchemaDescriptor {
            id: "s.wave.a3.placeholder",
            config: empty,
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
        validate_registered_app_descriptor(registry.get("s.wave.a3.placeholder").expect("placeholder")).await;
        assert!(GRAPHQL_STATE_PREAMBLE.contains("directive @state"));
    }
    //#endregion 🔖️AppSchemaRegistryParity
}
//#endregion 🔖️Tests
