//! 🧾️ Plugin manifest, dependency graph, artifact and surface app choices and package descriptors.

use replication::SchemaId;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use semio_framework_kernel_identity::{ActivationEvent, CapabilityId, CapabilityRequest, CapabilityRequirement, QuotaSchema};
use semio_framework_manifest_actions::CommandDefinition;
use semio_framework_manifest_app_surface::{AppDefinition, AppRef, ExampleDefinition, PanelTabDefinition};
use semio_framework_manifest_arg_schema::AppRole;
use semio_framework_manifest_contribution::{ArtifactContributionDescriptor, ContributedInferenceMetadata, ContributedMutationMetadata, PluginDependency, TopicContribution};
use semio_framework_manifest_media::{ArtifactKindSpec, HostedArtifactKind, MediaType};

// #region 🛂️Manifest
//#region 🔖️Manifest

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PluginManifest {
    pub plugin_id: String,
    pub label: String,
    pub version: String,
    pub apps: Vec<AppDefinition>,
    pub examples: Vec<ExampleDefinition>,
    #[serde(default)]
    #[value(default)]
    pub capabilities: Vec<CapabilityRequirement>,
    /// 🗂️ Open plugin contributions — see `TopicContribution`.
    #[serde(default)]
    #[value(default)]
    pub topic_contributions: Vec<TopicContribution>,
    /// 🎛️ Plugin-scope commands this program exposes — apply whenever any of its apps is focused.
    #[serde(default)]
    #[value(default)]
    pub commands: Vec<CommandDefinition>,
    /// 🗂️ Plugin-level artifact kinds (library plugins with zero apps declare kinds here).
    #[serde(default)]
    #[value(default)]
    pub artifact_kinds: Vec<ArtifactKindSpec>,
    /// 🏠️ Artifact kinds another package owns whose documents this plugin's apps open — one row per kind and document
    /// schema its owner's codecs decode (`PluginBuilder::host_artifact`, or a surface of another plugin's artifact the plugin
    /// registers). Distinct from `artifact_kinds`: the codec rows stay
    /// the owner's, and a trusted catalog binds these kinds to the owner's codecs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub hosted_artifact_kinds: Vec<HostedArtifactKind>,
    /// 🔗️ Direct plugin dependencies this plugin requires to load — see `PluginDependency`/
    /// `resolve_load_order`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<PluginDependency>,
    /// 🗂️ Artifact-kind contributions (mutations/inferences) this plugin contributes onto artifact
    /// kinds it depends on — see `ArtifactContributionDescriptor`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub contributions: Vec<ArtifactContributionDescriptor>,
}

//#region 🔖️HostResolvedArgs
/// 🗂️ One artifact-kind choice offered by an `ActionArgControl::ArtifactKind` dialog field —
/// resolved by the host from its live plugin catalogue (`artifact_kind_choices`) into a plain
/// `Select { options }` right before the dialog renders. Round-trips through `ActionArgOption.value`
/// as JSON via `encode_artifact_kind_choice`/`decode_artifact_kind_choice` — the frozen wire shape
/// (contract §C8.1): `{"kindId":"s.draw.draw","schema":"draw.document","dialect":{"artifactKind":
/// "s.draw.draw","standard":"1","subset":"*"},"label":{"en":"Draw","de":"Zeichnung"}}`. TS twin:
/// `ArtifactKindChoice` (`🟦️.ts`) — both codecs must agree byte-for-byte over the pinned
/// fixtures.
#[derive(Clone, Debug, PartialEq)]
pub struct ArtifactKindChoice {
    pub kind_id: String,
    pub schema: String,
    pub dialect: ArtifactDialect,
    pub label: LocalizedLabel,
}

/// 🎭️ One `(pluginId, appId, role)` choice offered by an `ActionArgControl::SurfaceApp` dialog
/// field — resolved by the host against the dialect coordinate found in the dialog's seed argument
/// named `dialect_arg`. Round-trips through `ActionArgOption.value` as JSON via
/// `encode_surface_app_choice`/`decode_surface_app_choice`: `{"pluginId":"draw","appId":"s.draw.draw
/// @1/*#editor","role":"editor"}`. TS twin: `SurfaceAppChoice` (`🟦️.ts`).
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceAppChoice {
    pub app: AppRef,
    pub role: AppRole,
}

/// 🧵️ Encodes an `ArtifactKindChoice` into the frozen `ActionArgOption.value` JSON shape — `label`
/// resolves under `Terminology::Native`, the only terminology this wire shape carries (a dialog
/// re-resolves display strings client-side under the active terminology from `kind_id`/`schema`
/// alone if it ever needs to, but the frozen shape itself is native-only, matching `IconSelect`'s own
/// `classifier_kind`-not-label precedent for host-resolved controls).
pub async fn encode_artifact_kind_choice(choice: &ArtifactKindChoice) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
        ("kindId".to_string(), choice.kind_id.as_str().into()),
        ("schema".to_string(), choice.schema.as_str().into()),
        ("dialect".to_string(), semio_framework_pack_json::from_dsl_value(&choice.dialect.to_value())),
        (
            "label".to_string(),
            semio_framework_pack_json::object([
                ("en".to_string(), choice.label.resolve(Terminology::Native, Locale::En).to_string().into()),
                ("de".to_string(), choice.label.resolve(Terminology::Native, Locale::De).to_string().into()),
            ]),
        ),
    ]))
}

/// 🧵️ Inverse of `encode_artifact_kind_choice`.
pub async fn decode_artifact_kind_choice(value: &str) -> Result<ArtifactKindChoice, String> {
    let json = semio_framework_pack_json::parse(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("malformed artifact kind choice JSON: {error}"))?;
    let kind_id = json.get("kindId").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(|| "artifact kind choice missing string field kindId".to_string())?.to_string();
    let schema = json.get("schema").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(|| "artifact kind choice missing string field schema".to_string())?.to_string();
    let dialect: ArtifactDialect = json
        .get("dialect")
        .cloned()
        .ok_or_else(|| "artifact kind choice missing field dialect".to_string())
        .and_then(|value| ArtifactDialect::from_value(semio_framework_pack_json::to_dsl_value(&value)).map_err(|error| format!("artifact kind choice has a malformed dialect: {error}")))?;
    let en = json.pointer("/label/en").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(|| "artifact kind choice missing string field label.en".to_string())?;
    let de = json.pointer("/label/de").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(|| "artifact kind choice missing string field label.de".to_string())?;
    Ok(ArtifactKindChoice { kind_id, schema, dialect, label: LocalizedLabel::native(en, de) })
}

/// 🧵️ Encodes a `SurfaceAppChoice` into its frozen `ActionArgOption.value` JSON shape.
pub async fn encode_surface_app_choice(choice: &SurfaceAppChoice) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
        ("pluginId".to_string(), choice.app.plugin_id.as_str().into()),
        ("appId".to_string(), choice.app.app_id.as_str().into()),
        ("role".to_string(), choice.role.as_str().into()),
    ]))
}

/// 🧵️ Inverse of `encode_surface_app_choice`.
pub async fn decode_surface_app_choice(value: &str) -> Result<SurfaceAppChoice, String> {
    let json = semio_framework_pack_json::parse(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("malformed surface app choice JSON: {error}"))?;
    let plugin_id = json.get("pluginId").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(|| "surface app choice missing string field pluginId".to_string())?.to_string();
    let app_id = json.get("appId").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(|| "surface app choice missing string field appId".to_string())?.to_string();
    let role_str = json.get("role").and_then(semio_framework_pack_json::Value::as_str).ok_or_else(|| "surface app choice missing string field role".to_string())?;
    let role: AppRole = role_str.parse()?;
    Ok(SurfaceAppChoice { app: AppRef { plugin_id, app_id }, role })
}

/// 🗂️ Every artifact-kind choice for the given `roles` — Rust twin of TS `artifactKindChoices` (law in
/// `🧪️tests/🧪️hostresolvedargs/🟦️.ts`): every app across `manifests` whose `role` is in `roles`, whose
/// `io.artifact_schema` is non-empty and whose package declares a kind of that schema (on any of its apps
/// — a viewer shares its editor's kind — or on the manifest) contributes one choice per dialect coordinate,
/// labelled with that kind's own label, never its app's; an undeclared schema is not a creatable kind.
/// Deduped by dialect coordinate (first manifest/app wins — callers pass owner manifests first so the
/// owner's label wins over a later contributor's), sorted by coordinate for determinism.
pub async fn artifact_kind_choices(manifests: &[PluginManifest], roles: &[AppRole]) -> Vec<ArtifactKindChoice> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let mut by_coordinate: BTreeMap<String, ArtifactKindChoice> = BTreeMap::new();
    for manifest in manifests {
        for app in &manifest.apps {
            if !roles.contains(&app.role) || app.io.artifact_schema.is_empty() {
                continue;
            }
            let Some(kind) = manifest.apps.iter().flat_map(|other| other.artifact_kinds.iter()).chain(manifest.artifact_kinds.iter()).find(|kind| kind.schema == app.io.artifact_schema) else {
                continue;
            };
            by_coordinate.entry(app.dialect.to_coordinate()).or_insert_with(|| ArtifactKindChoice { kind_id: app.dialect.artifact_kind.clone(), schema: app.io.artifact_schema.clone(), dialect: app.dialect.clone(), label: kind.label.clone() });
        }
    }
    by_coordinate.into_values().collect()
}

//#endregion 🔖️HostResolvedArgs
//#region 🔖️DependencyGraph

/// 🚧️ Typed dependency-graph validation failures — contract freeze §4/§5: missing dependency,
/// version mismatch, or a cycle (naming every plugin id on the cycle, in traversal order).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DependencyGraphError {
    MissingDependency { plugin_id: String, depends_on: String },
    VersionMismatch { plugin_id: String, depends_on: String, required: String, actual: String },
    Cycle { members: Vec<String> },
}

impl std::fmt::Display for DependencyGraphError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDependency { plugin_id, depends_on } => write!(formatter, "plugin `{plugin_id}` depends on unknown plugin `{depends_on}`"),
            Self::VersionMismatch { plugin_id, depends_on, required, actual } => {
                write!(formatter, "plugin `{plugin_id}` requires `{depends_on}` `{required}` but the loaded version is `{actual}`")
            }
            Self::Cycle { members } => write!(formatter, "dependency cycle among plugins: {}", members.join(" -> ")),
        }
    }
}

impl std::error::Error for DependencyGraphError {}

/// ✅️ Checks every declared dependency resolves to a loaded plugin at a satisfying version —
/// deterministic: manifests are checked in input order, each manifest's dependencies in declaration
/// order, so the first violation found is always the same for the same input.
fn validate_dependency_graph(manifests: &[PluginManifest]) -> Result<(), DependencyGraphError> {
    let by_id: BTreeMap<&str, &PluginManifest> = manifests.iter().map(|manifest| (manifest.plugin_id.as_str(), manifest)).collect();
    for manifest in manifests {
        for dependency in &manifest.dependencies {
            let Some(target) = by_id.get(dependency.plugin_id.as_str()) else {
                return Err(DependencyGraphError::MissingDependency { plugin_id: manifest.plugin_id.clone(), depends_on: dependency.plugin_id.clone() });
            };
            if !dependency.version.matches_raw(&target.version) {
                return Err(DependencyGraphError::VersionMismatch { plugin_id: manifest.plugin_id.clone(), depends_on: dependency.plugin_id.clone(), required: dependency.version.to_string(), actual: target.version.clone() });
            }
        }
    }
    Ok(())
}

/// 🧭️ Kahn toposort of the plugin dependency graph: a dependency always precedes its dependents,
/// and among several simultaneously-ready plugins the lexicographically smallest id is always
/// picked next, so the returned order is a pure, deterministic function of the input set. Runs
/// `validate_dependency_graph` first, so a missing dependency or version mismatch is reported
/// before any cycle would be detected.
pub fn resolve_load_order(manifests: &[PluginManifest]) -> Result<Vec<String>, DependencyGraphError> {
    validate_dependency_graph(manifests)?;

    let mut in_degree: BTreeMap<&str, usize> = manifests.iter().map(|manifest| (manifest.plugin_id.as_str(), 0)).collect();
    let mut dependents_of: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for manifest in manifests {
        for dependency in &manifest.dependencies {
            *in_degree.get_mut(manifest.plugin_id.as_str()).expect("validated above") += 1;
            dependents_of.entry(dependency.plugin_id.as_str()).or_default().push(manifest.plugin_id.as_str());
        }
    }

    let mut ready: std::collections::BTreeSet<&str> = in_degree.iter().filter(|(_, degree)| **degree == 0).map(|(id, _)| *id).collect();
    let mut order: Vec<String> = Vec::with_capacity(manifests.len());
    while let Some(next) = ready.iter().next().copied() {
        ready.remove(next);
        order.push(next.to_string());
        if let Some(dependents) = dependents_of.get(next) {
            let mut sorted_dependents = dependents.clone();
            sorted_dependents.sort_unstable();
            for dependent in sorted_dependents {
                let degree = in_degree.get_mut(dependent).expect("validated above");
                *degree -= 1;
                if *degree == 0 {
                    ready.insert(dependent);
                }
            }
        }
    }

    if order.len() != manifests.len() {
        let resolved: std::collections::BTreeSet<&str> = order.iter().map(String::as_str).collect();
        let leftover: std::collections::BTreeSet<String> = manifests.iter().map(|manifest| manifest.plugin_id.as_str()).filter(|id| !resolved.contains(id)).map(str::to_string).collect();
        return Err(DependencyGraphError::Cycle { members: find_cycle_members(manifests, &leftover) });
    }
    Ok(order)
}

/// 🔁️ Walks the leftover (never-ready) subgraph depth-first from its lexicographically smallest
/// node, following each plugin's first declared dependency that is also leftover, until a node
/// repeats — the repeated slice of the walked path is the named cycle.
fn find_cycle_members(manifests: &[PluginManifest], leftover: &std::collections::BTreeSet<String>) -> Vec<String> {
    let by_id: BTreeMap<&str, &PluginManifest> = manifests.iter().map(|manifest| (manifest.plugin_id.as_str(), manifest)).collect();
    let mut visited: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for start in leftover {
        if visited.contains(start) {
            continue;
        }
        let mut path: Vec<String> = Vec::new();
        let mut on_path: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut node = start.clone();
        loop {
            if on_path.contains(&node) {
                let cycle_start = path.iter().position(|id| id == &node).expect("on_path implies present in path");
                return path[cycle_start..].to_vec();
            }
            if visited.contains(&node) {
                break;
            }
            visited.insert(node.clone());
            on_path.insert(node.clone());
            path.push(node.clone());
            let next = by_id.get(node.as_str()).and_then(|manifest| manifest.dependencies.iter().map(|dependency| dependency.plugin_id.clone()).find(|id| leftover.contains(id)));
            match next {
                Some(next_node) => node = next_node,
                None => break,
            }
        }
    }
    leftover.iter().cloned().collect()
}

/// 🔎️ Every plugin (direct dependents only, not transitive) that declares `plugin_id` as a
/// dependency, sorted for determinism — used to refuse unload/hot-reload while dependents are
/// loaded (contract freeze §4).
pub fn dependents(manifests: &[PluginManifest], plugin_id: &str) -> Vec<String> {
    let mut result: Vec<String> = manifests.iter().filter(|manifest| manifest.dependencies.iter().any(|dependency| dependency.plugin_id == plugin_id)).map(|manifest| manifest.plugin_id.clone()).collect();
    result.sort_unstable();
    result
}

//#endregion 🔖️DependencyGraph
//#region 🔖️PackageDescriptor

/// 🎭️ Which actor-world role a package fills — `📓️design-abi.md` §3.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum PackageRole {
    Plugin,
    Extension,
}

/// 🚦 How an extension actor runs relative to its host plugin — `📓️design-abi.md` §5. Default
/// `Isolated`: a same-process sandboxed actor, no publisher trust assumed. `Linked` additionally
/// requires the same publisher as the host plugin (enforced at link time, feature-gated to avoid
/// the `semio-framework-os-flow` ↔ extension-crate cycle); `Exclusive` gets a dedicated actor
/// (e.g. flow/brep tessellation); `Cold` runs as a bounded job, not a resident actor.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ExecutionMode {
    Declarative,
    Linked,
    #[default]
    Isolated,
    Exclusive,
    Cold,
}

/// 🧩️ One extension point a host plugin publishes — replaces the Cargo `consumes` tag
/// (`📓️design-abi.md` §5). `allowed_modes` gates `Linked` (same publisher required);
/// `capability_allowance`/`quota_ceiling` bound what any extension attaching here can ever hold,
/// regardless of what it requests — "a host can never delegate more than it holds".
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExtensionPointDeclaration {
    pub id: String,
    pub publisher_scope: String,
    pub allowed_modes: Vec<ExecutionMode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub capability_allowance: Vec<CapabilityId>,
    #[serde(default)]
    #[value(default)]
    pub quota_ceiling: QuotaSchema,
    pub payload_schema: SchemaId,
    pub activation: ActivationEvent,
}

/// 📦️ One asset bundled with a package and preloaded into `Event::InstanceOpen.assets` —
/// `📓️design-abi.md` §2's `read-asset` replacement.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct AssetDeclaration {
    pub name: String,
    pub media_type: MediaType,
    pub size_bytes: u64,
    pub sha256: String,
}

/// 🗂️ One free-form descriptor-only contribution row, keyed by `id` with an opaque JSON
/// `payload` — the residual placeholder shape for the two `ContributionSet` categories (`menus`,
/// `themes`) that still have no real declared-contribution precedent anywhere in the codebase
/// (E1-describe surveyed every `[package.metadata.semio]` `contributes`/`consumes` tag and every
/// manifest-adjacent type: no plugin declares a menu or theme as its own manifest concept today —
/// context menus are derived at runtime from `ActionSemantics`/category metadata, and there is no
/// declared theme/palette contribution anywhere under `🖱️ui/🎨️styling`). Additive: nothing
/// constructs one yet, and a future typed model can replace either category without a wire break.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct DescriptorEntry {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<DslValue>,
}

/// 🗂️ One file-format kind an app declares it can import and/or export — the typed shape for
/// `ContributionSet.file_types` (`📓️design-abi.md` §3), grounded in `AppIo.export_formats`/
/// `import_formats` (currently flat `Vec<String>` scaffolding on that type) paired with the app's
/// own `artifact_media_type`, flattened to one row per format kind across every app the package
/// declares.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct FileTypeContribution {
    pub format_kind: String,
    pub media_type: MediaType,
    pub imports: bool,
    pub exports: bool,
}

/// 🚪️ Which side of an `IoEntryDescriptor` route this row is — owned mirror of `io::IoDirection`
/// (`🚪️io/🦀️.rs`), the same "owned wire twin of a native type living in a sibling
/// framework module" idiom `ContributedMutationMetadata`/`ContributedInferenceMetadata` already
/// use for the os-kernel protocol crate.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum IoEntryDirection {
    Import,
    Export,
}

/// 🚪️ One registered IO dialect route — the typed shape for `ContributionSet.io_entries`
/// (`📓️design-abi.md` §2/§3's absorbed `io-dialects` routing table), an owned mirror of
/// `io::IoKey`'s `(owner, counterpart, direction)` identity built from the already-in-scope
/// `ArtifactDialect` (`🚪️io/🧬️schema/🦀️component.rs`) instead of `IoKey`'s seven flat fields —
/// `IoKey` itself isn't `owned schema exporter`-derived and this crate must not add that derive to a module it
/// doesn't own.
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct IoEntryDescriptor {
    pub owner: ArtifactDialect,
    pub counterpart: ArtifactDialect,
    pub direction: IoEntryDirection,
}

/// 🎹️ One registered composer/serializer/deserializer route — the typed shape for
/// `ContributionSet.composer_entries`, an owned mirror of `io::ComposerEntry`'s `(writes, reads)`
/// identity (its third field, the `compose` fn pointer, is runtime-only and has no wire form —
/// a descriptor is build-time, non-executable data).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ComposerEntryDescriptor {
    pub writes: ArtifactDialect,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub reads: Vec<ArtifactDialect>,
}

/// 🗂️ Everything a package contributes, gathered for static (`describe()`-time) emission —
/// `📓️design-abi.md` §3. `commands`/`topic_contributions`/`artifact_contributions` reuse this
/// crate's existing typed models; `panels` reuses `PanelTabDefinition` (already the typed shape
/// `AppDefinition.panel_tabs` declares, flattened across every app); `inference_services`/
/// `mutation_services` reuse `ContributedInferenceMetadata`/`ContributedMutationMetadata` (a
/// package's OWN registered services on artifact kinds it owns, as opposed to
/// `artifact_contributions`' services contributed onto a DEPENDENCY's kind — same wire shape
/// either way, `contributor == owner` and `depends_on` empty for a self-owned row);
/// `file_types`/`io_entries`/`composer_entries` are new types grounded in `AppIo`/`io::IoKey`/
/// `io::ComposerEntry` — see each type's own doc. `menus`/`themes` stay `DescriptorEntry` — see
/// its doc for why.
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ContributionSet {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<CommandDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub menus: Vec<DescriptorEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub file_types: Vec<FileTypeContribution>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub panels: Vec<PanelTabDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub themes: Vec<DescriptorEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub topic_contributions: Vec<TopicContribution>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub artifact_contributions: Vec<ArtifactContributionDescriptor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub inference_services: Vec<ContributedInferenceMetadata>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub mutation_services: Vec<ContributedMutationMetadata>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub io_entries: Vec<IoEntryDescriptor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub composer_entries: Vec<ComposerEntryDescriptor>,
}

/// 🚨️ The `plugin_id` a failed assembly mints instead of a real one. `plugin_manifest()`
/// (`🔌️plugin/🦀️.rs`) returns this stub whenever `PLUGIN_ASSEMBLY_ERROR` is set, carrying
/// the real error text in `label`. It looks like a descriptor, parses as JSON, and would feed the
/// generated registry catalog with fabricated contributions — so the emitter refuses to write one
/// (`🖨️describe/📦️packages/🦀️rust/🦀️.rs`). Lives here, beside [`PackageDescriptor`], because it
/// is the one crate BOTH the guest SDK that mints it and the host emitter that rejects it depend
/// on; a duplicated string literal in either would drift silently.
pub const ASSEMBLY_FAILED_PLUGIN_ID: &str = "assembly-failed";

/// 📡️ Exact guest-described application-channel ABI required by the compiled component.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExecutionProtocol {
    pub app_channel_version: u32,
}

/// 📦️ The static, build-time-emitted description of a plugin or extension package —
/// `📓️design-abi.md` §3's `describe()` output (`🛂️.descriptor.semio`/`🔣️.json`).
/// Nothing constructs or reads one yet in this packet: additive contract only (packet
/// A2-abi-sdk's builder wiring and E1-describe's emitter/registry `check` gate consume it next).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PackageDescriptor {
    pub descriptor_version: u32,
    pub package_id: String,
    pub role: PackageRole,
    pub manifest: PluginManifest,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub activation_events: Vec<ActivationEvent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub capability_requests: Vec<CapabilityRequest>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub extension_points: Vec<ExtensionPointDeclaration>,
    pub execution: ExecutionMode,
    pub execution_protocol: ExecutionProtocol,
    #[serde(default)]
    #[value(default)]
    pub quotas: QuotaSchema,
    #[serde(default)]
    #[value(default)]
    pub contributions: ContributionSet,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<AssetDeclaration>,
}

/// 🔤️ Descriptor value with every object ordered by UTF-8 key bytes at every depth.
/// Shared by guest descriptions, emitted packs and exact freshness projections.
#[derive(Clone, Debug, PartialEq)]
pub struct CanonicalDescriptorValue(DslValue);

impl CanonicalDescriptorValue {
    /// 🔃️ Orders the members of every object by key bytes; members of equal keys keep their authored order.
    pub fn new(value: DslValue) -> Self {
        Self(canonical_members(value))
    }

    /// 👁️ The canonical value itself.
    pub fn value(&self) -> &DslValue {
        &self.0
    }
}

/// 🪜️ [`CanonicalDescriptorValue::new`] through every array and object of `value`.
fn canonical_members(value: DslValue) -> DslValue {
    use semio_framework_value::DslValue;
    match value {
        DslValue::Array(items) => DslValue::Array(items.into_iter().map(canonical_members).collect()),
        DslValue::Object(entries) => {
            let mut entries: Vec<(String, DslValue)> = entries.into_iter().map(|(key, entry)| (key, canonical_members(entry))).collect();
            entries.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
            DslValue::Object(entries)
        }
        scalar => scalar,
    }
}

//#endregion 🔖️PackageDescriptor
//#region 🔖️AgentContributions

// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P8-agent-spi, `📋️master.md`
// §3.1: what a package OFFERS to agents — distinct from `capability_requests` on
// `PackageDescriptor` above (what a package NEEDS host permission for). Overloading one for the
// other is exactly the mistake this design forbids: `capability_requests: Vec<kernel::
// CapabilityRequest>` gates HOST PRIVILEGE (`artifacts.write`, `fs:*`, …) this package asks the
// broker for; `AgentContributions` below is a curated ADVERTISEMENT of which of this package's own
// already-declared capabilities (actions/commands, already fully described via `ActionSemantics`/
// `CapabilityPolicy` — P3, `🔖️ActionSemantics` above) an agent may discover and invoke at all, and
// which of those are further promoted to a first-class MCP tool (`📋️master.md` §3.1's `tools/list`
// "promoted" set).
//
// Attachment point, and why it is a LEASE rather than a field added directly here: the obvious
// home is a new `PackageDescriptor.agent: Option<AgentContributions>` field. `PackageDescriptor`'s
// only known construction sites (`describe_plugin()`/`describe_extension()`,
// `🔌️plugin/🛂️describe/🦀️.rs`) live in `semio-framework-plugin` — a crate this packet
// does not own and the peer ticket's W3 is about to freeze — and both build the value as a full
// explicit struct literal (verified by reading the file: no `..Default::default()` anywhere, and
// `PackageDescriptor` has no `Default` impl — `role: PackageRole` has no default variant — so a
// `Default` impl could not rescue an untouched call site either way). Adding the field HERE alone
// would therefore break `cargo check -p semio-framework-plugin --lib` the moment this region
// lands, in a live shared tree, before any reviewer applies a companion lease — exactly the
// destabilisation this packet must not cause. So the field itself, and its counterpart on the
// `PluginDescriptorExtras` side-channel (`🔌️plugin/🦀️.rs`, E2's own established pattern
// for precisely this "avoid cascading through construction sites I don't own" problem) and on
// `ExtensionManifest`, ship together as ONE atomic lease bundle — see `📓️terra-P8-report.md` §2
// for the full reasoning and `📓️lease-P8-agent-descriptor.md` for the exact diffs.
/// 🤖️ What a package OFFERS to agents — see the region header above for the critical
/// `capability_requests` vs `AgentContributions` distinction. `capabilities` are fully-qualified
/// capability ids (the same grammar `🌉️mcp/🗂️catalog` compiles — `<plugin_id>.<app_id>.
/// <action_id>` / `….cmd.<id>` / `….mode.<mode_id>.<id>`, `📋️master.md` §3.1); `promoted` is the
/// subset exposed as a first-class MCP tool (`tools/list`) rather than only reachable via
/// `capabilities.search`/`capabilities.describe`. Both empty by default — an absent
/// `AgentContributions` (the `Option` on `PackageDescriptor` stays `None`) means "not yet
/// agent-enabled", never "agent-enabled with zero capabilities" (an empty-but-`Some` value).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AgentContributions {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub promoted: Vec<String>,
}

impl AgentContributions {
    /// 🧪️ `promoted ⊆ capabilities` — the one structural invariant every producer
    /// (`describe_plugin()`/`describe_extension()`) and consumer (`📇️registry:check`) must hold.
    /// Pure and dependency-free so both the Rust builder side and the registry's own TypeScript
    /// check (which has no way to call back into this crate) can each verify it independently.
    pub async fn promoted_is_subset_of_capabilities(&self) -> bool {
        self.promoted.iter().all(|id| self.capabilities.contains(id))
    }
}

//#endregion 🔖️AgentContributions
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest

#[cfg(test)]
use semio_framework_manifest_contribution::{ContributedMutationSemantics, Version, VersionParseError, VersionPin, VersionPinParseError, tree_pin};

// #region 🛂️Manifest
//#region 🔖️Manifest
#[cfg(test)]
#[path = "../🧪️tests/🔬️plugin-dependency/🦀️.rs"]
mod plugin_dependency_tests;

//#region 🔖️AgentContributions
#[cfg(test)]
#[path = "../🧪️tests/🔬️agent-contributions/🦀️.rs"]
mod agent_contributions_tests;

//#endregion 🔖️AgentContributions
//#region 🔖️PackageDescriptor
#[cfg(test)]
#[path = "../🧪️tests/🔬️package-descriptor-value-codec/🦀️.rs"]
mod package_descriptor_value_codec_tests;
//#endregion 🔖️PackageDescriptor
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
