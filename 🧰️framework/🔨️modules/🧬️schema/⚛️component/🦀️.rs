//! 📋️ Schema registry: describe schemas and validate owned JSON at kernel boundaries.

use pack::json::{parse as parse_json, to_string as json_to_string, JsonError, Value};
use pack::{content_hash, ContentHash};
use std::collections::HashMap;

pub use semio_framework_os_kernel::StateClass;
pub use semio_framework_schema_derive::ArtifactSchema;
pub use semio_framework_schema_registry::{
    register_scope_facet_leaves, register_scope_schema_exports, resolve_schema_export, schema_export_catalog_entries, scope_schema_exports_registered, scope_schema_facets_registered, with_schema_export_registry, FacetLeaves, SchemaExport, SchemaExportEntries, SchemaExportEntry, SchemaExportRegistry,
    SchemaExportRegistryError, SchemaFormat, SchemaResolveError, ScopeSchemaExports, RESERVED_FACET_EXPORT_IDS,
};

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

/// 🩺 One structural validation failure as the owned draft-07 validator renders it: the instance path
/// the failure is attributed to (`$`, `$.name`, `$.items[0]`) and the reason, joined by `": "` inside
/// [`SchemaError::Validation`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationDiagnostic {
    pub instance_path: String,
    pub reason: String,
}

impl ValidationDiagnostic {
    /// 🧾 Renders the diagnostic exactly as [`SchemaError::Validation`] carries it.
    pub fn message(&self) -> String {
        format!("{}: {}", self.instance_path, self.reason)
    }

    /// 🧭 The instance path as an RFC 6901 JSON pointer, the spelling third-party validators report.
    pub fn json_pointer(&self) -> String {
        self.instance_path.trim_start_matches('$').replace(['.', '['], "/").replace(']', "")
    }

    /// 🔎 Reads the diagnostic back out of a validation failure; any other error kind carries no path.
    pub fn from_error(error: &SchemaError) -> Option<Self> {
        let SchemaError::Validation(message) = error else { return None };
        let (instance_path, reason) = message.split_once(": ")?;
        if !instance_path.starts_with('$') {
            return None;
        }
        Some(Self { instance_path: instance_path.to_string(), reason: reason.to_string() })
    }
}
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

    /// 📇️ The four fixed facets in [`RESERVED_FACET_EXPORT_IDS`] order — the projection the
    /// dependency-free export registry resolves `(scope id, export id, format id)` over.
    pub fn facet_leaves(&self) -> [FacetLeaves; 4] {
        [self.artifact, self.snapshot, self.diff, self.mutations]
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
    let _ = register_scope_facet_leaves(descriptor.id, descriptor.facet_leaves());
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
/// read. Lives here rather than on the registry because only this crate owns the draft-07 compiler.
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
    rust: include_str!("🦀️.rs"),
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
/// export — the source every projection in `🤖️generated/🏷️entity-kinds/🦀️.rs`, `🤖️generated/🏷️entity-kinds/🟦️.ts` and
/// `⌨️cli/🏷️entity-kinds/🐹️.go` is emitted from by the `schema-entity-catalog` generator.
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
