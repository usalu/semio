//! 📇️ Canonical schema descriptor, named-export and facet publication with one std-only owner lock.

#[path = "🧷️assembly/🦀️.rs"]
pub mod assembly;

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

//#region 🔖️SchemaFormat
/// 🗂️ One of the five schema formats a scope publishes, mirroring the taxonomy `schemaFormats` keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchemaFormat {
    Rust,
    Typescript,
    Graphql,
    JsonSchema,
    Protobuf,
}

impl SchemaFormat {
    /// 🧾 Every format, in taxonomy declaration order.
    pub const ALL: [Self; 5] = [Self::Rust, Self::Typescript, Self::Graphql, Self::JsonSchema, Self::Protobuf];

    /// 🏷️ Ascii format id used by the derived catalog and the `schema://` resolver.
    pub fn id(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Typescript => "typescript",
            Self::Graphql => "graphql",
            Self::JsonSchema => "jsonschema",
            Self::Protobuf => "protobuf",
        }
    }

    /// 🧩 Taxonomy `schemaFormats` key this format is the twin of.
    pub fn taxonomy_key(self) -> &'static str {
        match self {
            Self::Rust => "🦀️rust",
            Self::Typescript => "🟦️typescript",
            Self::Graphql => "🔗️graphql",
            Self::JsonSchema => "🔣️jsonschema",
            Self::Protobuf => "🛰️protobuf",
        }
    }

    /// 🔎 Parses either the ascii id or the taxonomy key.
    pub fn parse(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|format| format.id() == id || format.taxonomy_key() == id)
    }

    /// 🍃 The leaf body this format occupies inside a [`FacetLeaves`].
    pub fn leaf(self, leaves: &FacetLeaves) -> &'static str {
        match self {
            Self::Rust => leaves.rust,
            Self::Typescript => leaves.typescript,
            Self::Graphql => leaves.graphql,
            Self::JsonSchema => leaves.json_schema,
            Self::Protobuf => leaves.proto,
        }
    }
}

impl std::fmt::Display for SchemaFormat {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.id())
    }
}
//#endregion 🔖️SchemaFormat

//#region 🔖️ScopeSchemaExports
/// 🍃 Five handcrafted leaf bodies for one facet or one named export (`include_str!` at each
/// registration site). An empty body means the scope does not provide that format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FacetLeaves {
    pub rust: &'static str,
    pub typescript: &'static str,
    pub graphql: &'static str,
    pub json_schema: &'static str,
    pub proto: &'static str,
}

/// 🏷️ One named export of a scope — the `(export id, five format leaves)` pair the resolution key
/// `(scope id, export id, format id)` needs beyond the four fixed facets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchemaExport {
    pub id: &'static str,
    pub leaves: FacetLeaves,
}

/// 🧬️ A scope's named exports. Sibling to the four fixed facets of an artifact schema descriptor for
/// the same reason the inference descriptor is a sibling: the four-facet descriptor is handcrafted at
/// 235 call sites in 115 files, and a scope declares named exports independently of them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScopeSchemaExports {
    pub scope: &'static str,
    pub exports: &'static [SchemaExport],
}

/// 🔒️ Export ids reserved by the four fixed facets of an artifact schema descriptor, in the order
/// [`SchemaExportRegistry::register_facet_leaves`] expects them.
pub const RESERVED_FACET_EXPORT_IDS: [&str; 4] = ["artifact", "snapshot", "diff", "mutations"];

/// ⚠️ Named-export registration rejects a conflicting or internally duplicated declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SchemaExportRegistryError {
    ConflictingScope { scope: String },
    DuplicateExportId { scope: String, export: String },
}

impl std::fmt::Display for SchemaExportRegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConflictingScope { scope } => write!(formatter, "schema-export declaration conflicts for scope {scope}"),
            Self::DuplicateExportId { scope, export } => write!(formatter, "scope {scope} declares export id {export} twice"),
        }
    }
}

impl std::error::Error for SchemaExportRegistryError {}

/// ⚠️ Why `(scope id, export id, format id)` did not resolve to a leaf body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SchemaResolveError {
    UnknownScope { scope: String },
    UnknownExport { scope: String, export: String },
    FormatAbsent { scope: String, export: String, format: SchemaFormat },
    AmbiguousScope { scope: String, export: String },
}

impl std::fmt::Display for SchemaResolveError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownScope { scope } => write!(formatter, "unknown schema scope {scope}"),
            Self::UnknownExport { scope, export } => write!(formatter, "scope {scope} declares no export {export}"),
            Self::FormatAbsent { scope, export, format } => write!(formatter, "scope {scope} export {export} has no {format} leaf"),
            Self::AmbiguousScope { scope, export } => write!(formatter, "scope {scope} resolves export {export} from both a fixed facet and a named export"),
        }
    }
}

impl std::error::Error for SchemaResolveError {}

/// 📇️ One resolvable `(scope id, export id, format id)` triple with a non-empty leaf body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchemaExportEntry {
    pub scope: &'static str,
    pub export: &'static str,
    pub format: SchemaFormat,
}
//#endregion 🔖️ScopeSchemaExports

//#region 🔖️SchemaExportRegistry
/// 📚 Resolves `(scope id, export id, format id)` over every scope's four fixed facets plus its
/// [`ScopeSchemaExports`]. Exact resolution only — no nearest-parent search, no glob, no
/// fixture-local fallback.
#[derive(Clone)]
pub struct SchemaExportRegistry {
    facets: HashMap<&'static str, [FacetLeaves; 4]>,
    named: HashMap<&'static str, &'static [SchemaExport]>,
}

impl Default for SchemaExportRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaExportRegistry {
    /// 🏗️ Empty registry.
    pub fn new() -> Self {
        Self { facets: HashMap::new(), named: HashMap::new() }
    }

    /// 📎 Adds one scope's four fixed facet leaves, positionally keyed by
    /// [`RESERVED_FACET_EXPORT_IDS`]. Exact duplicates are accepted, conflicts are fatal.
    pub fn register_facet_leaves(&mut self, scope: &'static str, facets: [FacetLeaves; 4]) -> Result<(), SchemaExportRegistryError> {
        match self.facets.get(scope) {
            Some(established) if *established == facets => Ok(()),
            Some(_) => Err(SchemaExportRegistryError::ConflictingScope { scope: scope.to_string() }),
            None => {
                self.facets.insert(scope, facets);
                Ok(())
            }
        }
    }

    /// 📎 Adds one scope's named exports. Export ids must be unique inside the scope; exact duplicate
    /// declarations are accepted, conflicts are fatal.
    pub fn register_exports(&mut self, declaration: ScopeSchemaExports) -> Result<(), SchemaExportRegistryError> {
        for (index, export) in declaration.exports.iter().enumerate() {
            if declaration.exports[..index].iter().any(|previous| previous.id == export.id) {
                return Err(SchemaExportRegistryError::DuplicateExportId { scope: declaration.scope.to_string(), export: export.id.to_string() });
            }
        }
        match self.named.get(declaration.scope) {
            Some(established) if *established == declaration.exports => Ok(()),
            Some(_) => Err(SchemaExportRegistryError::ConflictingScope { scope: declaration.scope.to_string() }),
            None => {
                self.named.insert(declaration.scope, declaration.exports);
                Ok(())
            }
        }
    }

    /// 🚶 Every registered scope id, sorted.
    pub fn scopes(&self) -> Vec<&'static str> {
        let mut scopes: Vec<&'static str> = self.facets.keys().chain(self.named.keys()).copied().collect();
        scopes.sort_unstable();
        scopes.dedup();
        scopes
    }

    /// 🧾 Every export id a scope publishes — the four fixed facets first, then the named exports in
    /// declaration order.
    pub fn exports(&self, scope: &str) -> Result<Vec<&'static str>, SchemaResolveError> {
        let facets = self.facets.get(scope);
        let named = self.named.get(scope);
        if facets.is_none() && named.is_none() {
            return Err(SchemaResolveError::UnknownScope { scope: scope.to_string() });
        }
        let mut exports: Vec<&'static str> = facets.map(|_| RESERVED_FACET_EXPORT_IDS.to_vec()).unwrap_or_default();
        exports.extend(named.into_iter().flat_map(|exports| exports.iter().map(|export| export.id)));
        Ok(exports)
    }

    /// 🔎 Resolves one `(scope id, export id, format id)` triple to its handcrafted leaf body.
    pub fn resolve(&self, scope: &str, export: &str, format: SchemaFormat) -> Result<&'static str, SchemaResolveError> {
        let leaves = self.leaves(scope, export)?;
        let body = format.leaf(&leaves);
        if body.trim().is_empty() {
            return Err(SchemaResolveError::FormatAbsent { scope: scope.to_string(), export: export.to_string(), format });
        }
        Ok(body)
    }

    /// 🍃 The five format leaves behind one `(scope id, export id)` pair.
    pub fn leaves(&self, scope: &str, export: &str) -> Result<FacetLeaves, SchemaResolveError> {
        let facets = self.facets.get(scope);
        let named = self.named.get(scope);
        if facets.is_none() && named.is_none() {
            return Err(SchemaResolveError::UnknownScope { scope: scope.to_string() });
        }
        let facet = facets.and_then(|facets| RESERVED_FACET_EXPORT_IDS.iter().position(|reserved| *reserved == export).map(|index| facets[index]));
        let declared = named.and_then(|exports| exports.iter().find(|candidate| candidate.id == export)).map(|candidate| candidate.leaves);
        match (facet, declared) {
            (Some(_), Some(_)) => Err(SchemaResolveError::AmbiguousScope { scope: scope.to_string(), export: export.to_string() }),
            (Some(leaves), None) | (None, Some(leaves)) => Ok(leaves),
            (None, None) => Err(SchemaResolveError::UnknownExport { scope: scope.to_string(), export: export.to_string() }),
        }
    }

    /// 🚶 Every resolvable `(scope, export, format)` triple with a non-empty leaf, in deterministic
    /// order — the cross-check input for the derived schema catalog.
    pub fn entries(&self) -> impl Iterator<Item = SchemaExportEntry> + '_ {
        self.scopes().into_iter().flat_map(move |scope| {
            let exports = self.exports(scope).unwrap_or_default();
            exports.into_iter().flat_map(move |export| {
                SchemaFormat::ALL.into_iter().filter_map(move |format| match self.resolve(scope, export, format) {
                    Ok(_) => Some(SchemaExportEntry { scope, export, format }),
                    Err(_) => None,
                })
            })
        })
    }
}
//#endregion 🔖️SchemaExportRegistry


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
    pub fn facet_leaves(&self) -> [FacetLeaves; 4] {
        [self.artifact, self.snapshot, self.diff, self.mutations]
    }
}

/// 💡️ Registered descriptor for one artifact's 💡️inference schema facet — a SIBLING to
/// [`ArtifactSchemaDescriptor`], not a field on it (see [`ArtifactInferenceDescriptor`]'s own
/// doc for why). `id` is the inference schema's own id, `"{artifact_id}.inference"`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactInferenceDescriptor {
    pub id: &'static str,
    pub inference: FacetLeaves,
}


/// 🧬️ Registered descriptor for one app owner's config + presence schema facets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppSchemaDescriptor {
    pub id: &'static str,
    pub config: FacetLeaves,
    pub presence: FacetLeaves,
}


//#region 🔖️ArtifactSchemaRegistry
/// 📚 Runtime registry of [`ArtifactSchemaDescriptor`] values — same shape as [`SchemaCatalog`].
#[derive(Clone)]
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

    /// 📭 Whether no artifact schema ids are registered.
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }
}
//#endregion 🔖️ArtifactSchemaRegistry

//#region 🔖️ArtifactInferenceRegistry
/// 📚 Runtime registry of [`ArtifactInferenceDescriptor`] values.
#[derive(Clone)]
pub struct ArtifactInferenceRegistry {
    by_id: HashMap<&'static str, ArtifactInferenceDescriptor>,
}

impl Default for ArtifactInferenceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtifactInferenceRegistry {
    /// 🏗️ Empty registry.
    pub fn new() -> Self {
        Self { by_id: HashMap::new() }
    }

    /// 📎 Insert or replace a descriptor by id.
    pub fn register(&mut self, descriptor: ArtifactInferenceDescriptor) {
        self.by_id.insert(descriptor.id, descriptor);
    }

    /// 🔎 Lookup by inference schema id.
    pub fn get(&self, id: &str) -> Option<&ArtifactInferenceDescriptor> {
        self.by_id.get(id)
    }

    /// 🚶 Walk every registered descriptor.
    pub fn iter(&self) -> impl Iterator<Item = &ArtifactInferenceDescriptor> {
        self.by_id.values()
    }

    /// 🔢 Count of registered inference schema ids.
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    /// 📭 Whether no inference schema ids are registered.
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }
}
//#endregion 🔖️ArtifactInferenceRegistry

//#region 🔖️AppSchemaRegistry
/// 📚 Runtime registry of [`AppSchemaDescriptor`] values — app twin of [`ArtifactSchemaRegistry`].
#[derive(Clone)]
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


#[derive(Clone, Copy, PartialEq, Eq)]
enum FacetPublication { Descriptor, External }

#[derive(Default)]
struct CatalogState {
    artifacts: ArtifactSchemaRegistry,
    inferences: ArtifactInferenceRegistry,
    apps: AppSchemaRegistry,
    exports: SchemaExportRegistry,
    facet_publications: HashMap<&'static str, FacetPublication>,
    documents: Vec<&'static str>,
}

static CATALOG: OnceLock<Mutex<CatalogState>> = OnceLock::new();

fn catalog() -> &'static Mutex<CatalogState> {
    CATALOG.get_or_init(|| Mutex::new(CatalogState::default()))
}

trait Descriptor: Copy + PartialEq {
    const FAMILY: &'static str;
    fn id(&self) -> &'static str;
    fn entries(state: &CatalogState) -> &HashMap<&'static str, Self>;
    fn entries_mut(state: &mut CatalogState) -> &mut HashMap<&'static str, Self>;
    fn mirror(&self, _state: &CatalogState) -> Result<(), SchemaDescriptorRegistryError> { Ok(()) }
    fn publish_mirror(&self, _state: &mut CatalogState) {}
}

impl Descriptor for ArtifactSchemaDescriptor {
    const FAMILY: &'static str = "artifact-schema";
    fn id(&self) -> &'static str { self.id }
    fn entries(state: &CatalogState) -> &HashMap<&'static str, Self> { &state.artifacts.by_id }
    fn entries_mut(state: &mut CatalogState) -> &mut HashMap<&'static str, Self> { &mut state.artifacts.by_id }
    fn mirror(&self, state: &CatalogState) -> Result<(), SchemaDescriptorRegistryError> {
        if state.exports.facets.get(self.id).is_some_and(|existing| *existing != self.facet_leaves())
            && state.facet_publications.get(self.id) != Some(&FacetPublication::Descriptor) {
            return Err(SchemaDescriptorRegistryError { registry: "schema-export", id: self.id.to_string() });
        }
        Ok(())
    }
    fn publish_mirror(&self, state: &mut CatalogState) {
        state.exports.facets.insert(self.id, self.facet_leaves());
        state.facet_publications.entry(self.id).or_insert(FacetPublication::Descriptor);
    }
}

impl Descriptor for ArtifactInferenceDescriptor {
    const FAMILY: &'static str = "artifact-inference";
    fn id(&self) -> &'static str { self.id }
    fn entries(state: &CatalogState) -> &HashMap<&'static str, Self> { &state.inferences.by_id }
    fn entries_mut(state: &mut CatalogState) -> &mut HashMap<&'static str, Self> { &mut state.inferences.by_id }
}

impl Descriptor for AppSchemaDescriptor {
    const FAMILY: &'static str = "app-schema";
    fn id(&self) -> &'static str { self.id }
    fn entries(state: &CatalogState) -> &HashMap<&'static str, Self> { &state.apps.by_id }
    fn entries_mut(state: &mut CatalogState) -> &mut HashMap<&'static str, Self> { &mut state.apps.by_id }
}

fn validate_descriptors<T: Descriptor>(state: &CatalogState, descriptors: &[T]) -> Result<(), SchemaDescriptorRegistryError> {
    let mut proposed = HashMap::new();
    for descriptor in descriptors {
        let existing = proposed.get(descriptor.id()).or_else(|| T::entries(state).get(descriptor.id()));
        if existing.is_some_and(|existing| existing != descriptor) {
            return Err(SchemaDescriptorRegistryError { registry: T::FAMILY, id: descriptor.id().to_string() });
        }
        descriptor.mirror(state)?;
        proposed.insert(descriptor.id(), *descriptor);
    }
    Ok(())
}

fn register_descriptor<T: Descriptor>(descriptor: T) -> Result<(), SchemaDescriptorRegistryError> {
    let mut state = catalog().lock().expect("schema catalog lock");
    descriptor.mirror(&state)?;
    T::entries_mut(&mut state).insert(descriptor.id(), descriptor);
    descriptor.publish_mirror(&mut state);
    Ok(())
}

fn register_descriptors<T: Descriptor>(descriptors: Vec<T>) -> Result<(), SchemaDescriptorRegistryError> {
    let mut state = catalog().lock().expect("schema catalog lock");
    validate_descriptors(&state, &descriptors)?;
    for descriptor in descriptors {
        T::entries_mut(&mut state).insert(descriptor.id(), descriptor);
        descriptor.publish_mirror(&mut state);
    }
    Ok(())
}

/// 📎️ Publishes an artifact descriptor and its facets together, refusing mirror conflicts.
pub fn register_artifact_schema_descriptor(descriptor: ArtifactSchemaDescriptor) -> Result<(), SchemaDescriptorRegistryError> { register_descriptor(descriptor) }
/// 📎️ Replaces one inference descriptor independently of other catalog families.
pub fn register_artifact_inference_descriptor(descriptor: ArtifactInferenceDescriptor) -> Result<(), SchemaDescriptorRegistryError> { register_descriptor(descriptor) }
/// 📎️ Replaces one app descriptor independently of other catalog families.
pub fn register_app_schema_descriptor(descriptor: AppSchemaDescriptor) -> Result<(), SchemaDescriptorRegistryError> { register_descriptor(descriptor) }
/// 🔬️ Observes artifact batch admission; publication independently validates under its lock.
pub fn preflight_artifact_schema_descriptors(descriptors: &[ArtifactSchemaDescriptor]) -> Result<(), SchemaDescriptorRegistryError> { validate_descriptors(&catalog().lock().expect("schema catalog lock"), descriptors) }
/// 🔬️ Observes inference batch admission without granting a publication permit.
pub fn preflight_artifact_inference_descriptors(descriptors: &[ArtifactInferenceDescriptor]) -> Result<(), SchemaDescriptorRegistryError> { validate_descriptors(&catalog().lock().expect("schema catalog lock"), descriptors) }
/// 🔬️ Observes app batch admission without granting a publication permit.
pub fn preflight_app_schema_descriptors(descriptors: &[AppSchemaDescriptor]) -> Result<(), SchemaDescriptorRegistryError> { validate_descriptors(&catalog().lock().expect("schema catalog lock"), descriptors) }
/// 📌️ Validates and publishes a complete artifact batch and mirrors under one lock.
pub fn register_artifact_schema_descriptors(descriptors: Vec<ArtifactSchemaDescriptor>) -> Result<(), SchemaDescriptorRegistryError> { register_descriptors(descriptors) }
/// 📌️ Validates and publishes a complete inference batch under one lock.
pub fn register_artifact_inference_descriptors(descriptors: Vec<ArtifactInferenceDescriptor>) -> Result<(), SchemaDescriptorRegistryError> { register_descriptors(descriptors) }
/// 📌️ Validates and publishes a complete app batch under one lock.
pub fn register_app_schema_descriptors(descriptors: Vec<AppSchemaDescriptor>) -> Result<(), SchemaDescriptorRegistryError> { register_descriptors(descriptors) }
/// 🔎️ Whether an artifact descriptor is published.
pub fn artifact_schema_descriptor_registered(id: &str) -> bool { catalog().lock().expect("schema catalog lock").artifacts.by_id.contains_key(id) }
/// 🔎️ Whether an inference descriptor is published.
pub fn artifact_inference_descriptor_registered(id: &str) -> bool { catalog().lock().expect("schema catalog lock").inferences.by_id.contains_key(id) }
/// 🔎️ Whether an app descriptor is published.
pub fn app_schema_descriptor_registered(id: &str) -> bool { catalog().lock().expect("schema catalog lock").apps.by_id.contains_key(id) }
/// 🔢️ Number of published artifact descriptors.
pub fn artifact_schema_catalog_len() -> usize { catalog().lock().expect("schema catalog lock").artifacts.len() }
/// 🔢️ Number of published inference descriptors.
pub fn artifact_inference_catalog_len() -> usize { catalog().lock().expect("schema catalog lock").inferences.len() }
/// 🔢️ Number of published app descriptors.
pub fn app_schema_catalog_len() -> usize { catalog().lock().expect("schema catalog lock").apps.len() }
/// 📚️ Visits an artifact snapshot after releasing the owner lock.
pub fn with_artifact_schema_registry<R>(visit: impl FnOnce(&ArtifactSchemaRegistry) -> R) -> R {
    let snapshot = catalog().lock().expect("schema catalog lock").artifacts.clone();
    visit(&snapshot)
}
/// 📚️ Visits an inference snapshot after releasing the owner lock.
pub fn with_artifact_inference_registry<R>(visit: impl FnOnce(&ArtifactInferenceRegistry) -> R) -> R {
    let snapshot = catalog().lock().expect("schema catalog lock").inferences.clone();
    visit(&snapshot)
}
/// 📚️ Visits an app snapshot after releasing the owner lock.
pub fn with_app_schema_registry<R>(visit: impl FnOnce(&AppSchemaRegistry) -> R) -> R {
    let snapshot = catalog().lock().expect("schema catalog lock").apps.clone();
    visit(&snapshot)
}
/// 📚️ Visits sorted artifact descriptors after releasing the owner lock.
pub fn with_artifact_schema_catalog<R>(visit: impl FnOnce(&[ArtifactSchemaDescriptor]) -> R) -> R {
    with_artifact_schema_registry(|snapshot| { let mut entries: Vec<_> = snapshot.iter().copied().collect(); entries.sort_by_key(|entry| entry.id); visit(&entries) })
}
/// 📚️ Visits sorted inference descriptors after releasing the owner lock.
pub fn with_artifact_inference_catalog<R>(visit: impl FnOnce(&[ArtifactInferenceDescriptor]) -> R) -> R {
    with_artifact_inference_registry(|snapshot| { let mut entries: Vec<_> = snapshot.iter().copied().collect(); entries.sort_by_key(|entry| entry.id); visit(&entries) })
}
/// 📚️ Visits sorted app descriptors after releasing the owner lock.
pub fn with_app_schema_catalog<R>(visit: impl FnOnce(&[AppSchemaDescriptor]) -> R) -> R {
    with_app_schema_registry(|snapshot| { let mut entries: Vec<_> = snapshot.iter().copied().collect(); entries.sort_by_key(|entry| entry.id); visit(&entries) })
}
/// 📎️ Registers a named export declaration under the canonical catalog lock.
pub fn register_scope_schema_exports(declaration: ScopeSchemaExports) -> Result<(), SchemaExportRegistryError> { catalog().lock().expect("schema catalog lock").exports.register_exports(declaration) }
/// 📎️ Registers fixed facets under the same lock as descriptor publication.
pub fn register_scope_facet_leaves(scope: &'static str, facets: [FacetLeaves; 4]) -> Result<(), SchemaExportRegistryError> {
    let mut state = catalog().lock().expect("schema catalog lock");
    state.exports.register_facet_leaves(scope, facets)?;
    state.facet_publications.insert(scope, FacetPublication::External);
    Ok(())
}
/// 🔎️ Whether a scope publishes named exports.
pub fn scope_schema_exports_registered(scope: &str) -> bool { catalog().lock().expect("schema catalog lock").exports.named.contains_key(scope) }
/// 🔎️ Whether a scope publishes fixed facets.
pub fn scope_schema_facets_registered(scope: &str) -> bool { catalog().lock().expect("schema catalog lock").exports.facets.contains_key(scope) }
/// 📚️ Visits an export snapshot after releasing the owner lock.
pub fn with_schema_export_registry<R>(visit: impl FnOnce(&SchemaExportRegistry) -> R) -> R {
    let snapshot = catalog().lock().expect("schema catalog lock").exports.clone();
    visit(&snapshot)
}
/// 🔎️ Resolves an exact published scope/export/format.
pub fn resolve_schema_export(scope: &str, export: &str, format: SchemaFormat) -> Result<&'static str, SchemaResolveError> { with_schema_export_registry(|registry| registry.resolve(scope, export, format)) }
/// 🚶️ Snapshots all resolvable scope/export/format triples.
pub fn schema_export_catalog_entries() -> Vec<SchemaExportEntry> { with_schema_export_registry(|registry| registry.entries().collect()) }
/// 📎️ Publishes referenced documents once, preserving publication order.
pub fn register_referenced_schema_documents(documents: &[&'static str]) {
    let mut state = catalog().lock().expect("schema catalog lock");
    for document in documents { if !state.documents.contains(document) { state.documents.push(document); } }
}
/// 🚶️ Every referenced schema document in publication order.
pub fn registered_referenced_schema_documents() -> Vec<&'static str> { catalog().lock().expect("schema catalog lock").documents.clone() }

//#region 🔖️SchemaExportEntries
/// 📤️ The runtime export registry rendered as the `schema-export-registry-entries-v1` dump that
/// `schema verify` reads. Entries are sorted and deduplicated by
/// `(scope, export, format)`, so the dump of one binary is byte-stable across runs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchemaExportEntries {
    pub contract_id: &'static str,
    pub generator: String,
    pub entries: Vec<SchemaExportEntry>,
}

impl SchemaExportEntries {
    /// 🪪️ Taxonomy `schemaExportResolution.rustEntriesContractId` — the consumer bails with
    /// `rust-entries-contract-unknown` on any other value, so the two are changed together.
    pub const CONTRACT_ID: &'static str = "schema-export-registry-entries-v1";

    /// 🚶 Snapshots the OS-wide catalog under the command that produced the snapshot, sorted and
    /// deduplicated by `(scope, export, format)` in the ascii spelling the dump renders.
    pub fn from_catalog(generator: impl Into<String>) -> Self {
        let mut entries = schema_export_catalog_entries();
        entries.sort_unstable_by_key(|entry| (entry.scope, entry.export, entry.format.id()));
        entries.dedup();
        Self { contract_id: Self::CONTRACT_ID, generator: generator.into(), entries }
    }

    /// 🧾 Renders the dump, one entry per line, exactly as the consumer parses it.
    pub fn to_json(&self) -> String {
        let entries: Vec<String> = self
            .entries
            .iter()
            .map(|entry| format!("\n    {{ \"scope\": \"{}\", \"export\": \"{}\", \"format\": \"{}\" }}", entry.scope, entry.export, entry.format.id()))
            .collect();
        format!("{{\n  \"contractId\": \"{}\",\n  \"generator\": \"{}\",\n  \"entries\": [{}\n  ]\n}}\n", self.contract_id, escape_json_string(&self.generator), entries.join(","))
    }
}

/// 🔡 Escapes a string for a JSON string body — the only serialization this crate performs, kept
/// in-house so the resolution contract stays dependency-free.
fn escape_json_string(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0c}' => escaped.push_str("\\f"),
            character if (character as u32) < 0x20 => escaped.push_str(&format!("\\u{:04x}", character as u32)),
            character => escaped.push(character),
        }
    }
    escaped
}
//#endregion 🔖️SchemaExportEntries

#[cfg(test)]
//#region 🔖️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
