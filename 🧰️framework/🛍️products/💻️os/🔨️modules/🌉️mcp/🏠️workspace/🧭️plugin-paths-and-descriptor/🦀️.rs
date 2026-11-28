//! 🧭️ Plugin registry rows, repo-root discovery, component paths and committed package descriptors.

use semio_os_kernel::native_catalog as component_artifacts;
use semio_framework_process_routing_workspace as workspace_identity;
use serde::Deserialize;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;
use semio_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};

/// 🏠️ Admits only a declared package workspace with regular build authority.
pub fn is_repository_root(path: &Path) -> Result<bool, String> {
    workspace_identity::is_workspace_root(path, |source| semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| matches!(value.get("workspaces"), Some(semio_framework_pack_json::Value::Array(_) | semio_framework_pack_json::Value::Object(_)))).map_err(|error| error.to_string()))
}

//#region 🔖️PluginPaths

// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P7-headless-workspace: this
#[derive(Clone, Debug)]
pub struct PluginRegistryEntry {
    pub plugin_id: String,
    /// 📁️ Owner root supplied by the plugin catalog, where
    /// `🔣️.json` lives) — NOT the `cratePath` the registry JSON stores (that names the
    /// `📦️packages/🦀️rust` crate root two levels below the owner root; see the taxonomy's own
    /// `_languageNeutralityComment`, restated in `📓️status.md`'s E1-describe entry: "descriptors
    /// live at the plugin/extension owner root ... siblings of `📦️packages`").
    pub owner_root: PathBuf,
    /// 📦️ The component crate path relative to the repo root, resolved with the exact build mode.
    pub crate_path: PathBuf,
}

/// 🧭️ Locates the repo root — identical algorithm to `🏃️run/🏗️bootstrap/🦀️.rs::find_repo_root` (walk up from
/// this crate's own `CARGO_MANIFEST_DIR` for paired build and package workspace markers, `SEMIO_REPO_ROOT` override first) so a direct
/// `cargo run -p semio-os-mcp` from an arbitrary cwd still resolves correctly.
pub fn find_repo_root() -> Result<PathBuf, GatewayError> {
    if let Ok(value) = std::env::var("SEMIO_REPO_ROOT") {
        let candidate = PathBuf::from(value);
        if is_repository_root(&candidate).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error))? {
            return Ok(candidate);
        }
    }
    let mut candidate = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if is_repository_root(&candidate).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error))? {
            return Ok(candidate);
        }
        if !candidate.pop() {
            return Err(GatewayError::new(GatewayErrorCode::Internal, "could not locate the repo root (no declared package workspace with build authority above SEMIO_REPO_ROOT or CARGO_MANIFEST_DIR)"));
        }
    }
}

/// 📇️ Reads the caller's admitted native publication and projects its exact package owners.
pub fn load_plugin_registry(repo_root:&Path)->Result<Vec<PluginRegistryEntry>,GatewayError>{
    let catalog=component_artifacts::configured_native_catalog_v1().map_err(|error|GatewayError::new(GatewayErrorCode::Internal,error))?;
    plugin_registry_entries(repo_root,&catalog)
}

pub(crate) fn plugin_registry_entries(repo_root:&Path,catalog:&component_artifacts::NativeCatalogV1)->Result<Vec<PluginRegistryEntry>,GatewayError>{
    let mut entries=Vec::with_capacity(catalog.plugins.len());
    for row in &catalog.plugins{
        let owner_root=repo_root.join(&row.crate_path).parent().and_then(Path::parent).ok_or_else(||GatewayError::new(GatewayErrorCode::Internal,format!("plugin registry row `{}` has an implausibly shallow cratePath `{}`",row.plugin_id,row.crate_path)))?.to_path_buf();
        entries.push(PluginRegistryEntry{plugin_id:row.plugin_id.clone(),owner_root,crate_path:PathBuf::from(&row.crate_path)});
    }
    Ok(entries)
}

pub fn find_plugin_entry<'a>(entries: &'a [PluginRegistryEntry], plugin_id: &str) -> Result<&'a PluginRegistryEntry, GatewayError> {
    entries.iter().find(|entry| entry.plugin_id == plugin_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("plugin `{plugin_id}` is not in the plugin registry")))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ComponentModelLocation {
    plugin_id: String,
    crate_path: String,
    root: PathBuf,
    #[serde(default, deserialize_with = "deserialize_preparation")]
    prepared_actor: Option<PreparedActorLocation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PreparedActorLocation {
    image_sha256: String,
    image_bytes: usize,
}

fn deserialize_preparation<'de, D: serde::Deserializer<'de>>(decoder: D) -> Result<Option<PreparedActorLocation>, D::Error> {
    PreparedActorLocation::deserialize(decoder).map(Some)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ComponentModelManifest {
    version: u32,
    mode: String,
    models: Vec<ComponentModelLocation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ComponentModelPolicy {
    environment: String,
    maximum_bytes: usize,
    prepared_actor_image: String,
    maximum_prepared_actor_bytes: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ComponentModelLayout {
    directory: String,
    generation_length: usize,
}

fn component_model_layout() -> &'static ComponentModelLayout {
    static LAYOUT: std::sync::OnceLock<ComponentModelLayout> = std::sync::OnceLock::new();
    LAYOUT.get_or_init(|| {
        let schema: serde_json::Value = serde_json::from_str(include_str!("../../../🔌️plugin/📦️model/🧬️schema/🔣️.json")).expect("valid component model schema");
        serde_json::from_value(schema["definitions"]["Layout"]["const"].clone()).expect("valid component model layout")
    })
}

fn component_model_policy() -> &'static ComponentModelPolicy {
    static POLICY: std::sync::OnceLock<ComponentModelPolicy> = std::sync::OnceLock::new();
    POLICY.get_or_init(|| {
        let schema: serde_json::Value = serde_json::from_str(include_str!("../../📡️stdio/📦️component-models/🔣️.json")).expect("valid MCP model schema");
        serde_json::from_value(schema["definitions"]["Protocol"]["const"].clone()).expect("valid MCP model policy")
    })
}

fn read_component_model_manifest(source: &str, mode: &str) -> Result<ComponentModelManifest, GatewayError> {
    let refused = || GatewayError::new(GatewayErrorCode::Internal, "closed MCP component model manifest required");
    if source.len() > component_model_policy().maximum_bytes { return Err(refused()); }
    let manifest: ComponentModelManifest = serde_json::from_str(source).map_err(|_| refused())?;
    if manifest.version != 1 || manifest.mode != mode || !matches!(mode, "prototype" | "release") || manifest.models.len() > 128 { return Err(refused()); }
    let mut seen = BTreeSet::new();
    for row in &manifest.models {
        let valid_id = !row.plugin_id.is_empty() && row.plugin_id.len() <= 64 && row.plugin_id.split('-').all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()));
        let valid_crate = row.crate_path.chars().count() <= 256 && !row.crate_path.contains(['\\', ':']) && !row.crate_path.chars().any(char::is_control) && row.crate_path.split('/').all(|part| !matches!(part, "" | "." | ".."));
        let digest = framework_hash::sha256_hex(row.crate_path.as_bytes());
        let layout = component_model_layout();
        let key = Path::new("browser").join(mode).join(&layout.directory).join(&digest[..24]);
        let generation = row.root.file_name().and_then(|name| name.to_str()).is_some_and(|name| name.len() == layout.generation_length && name.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
        let valid_root = row.root.is_absolute() && row.root.to_string_lossy().chars().count() <= 224 && !row.root.components().any(|part| matches!(part, std::path::Component::ParentDir | std::path::Component::CurDir)) && generation && row.root.parent().is_some_and(|parent| parent.ends_with(&key));
        let valid_preparation = row.prepared_actor.as_ref().is_none_or(|image| {
            (1..=component_model_policy().maximum_prepared_actor_bytes).contains(&image.image_bytes)
                && parse_sha256(&image.image_sha256).is_some()
        });
        if !valid_id || !valid_crate || !valid_root || !valid_preparation || !seen.insert(&row.plugin_id) { return Err(refused()); }
    }
    Ok(manifest)
}

fn parse_sha256(hex: &str) -> Option<[u8; 32]> {
    if hex.len() != 64 { return None; }
    let mut digest = [0; 32];
    let digit = |byte| match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    };
    for (result, pair) in digest.iter_mut().zip(hex.as_bytes().chunks_exact(2)) {
        *result = (digit(pair[0])? << 4) | digit(pair[1])?;
    }
    Some(digest)
}

/// 🌱️ The native child's exact retained model image with its launcher-verified digest and bounded extent.
pub struct StagedActorPreparation {
    pub image: PathBuf,
    pub image_sha256: [u8; 32],
    pub image_bytes: usize,
    pub maximum_bytes: usize,
}

/// 📦️ Resolves preparation only from the selected retained component's verified publication.
pub fn retained_actor_preparation(path: &Path, plugin_id: &str) -> Result<Option<StagedActorPreparation>, GatewayError> {
    let policy = component_model_policy();
    let Ok(source) = std::env::var(&policy.environment) else { return Ok(None) };
    let manifest = read_component_model_manifest(&source, env!("SEMIO_BUILD_MODE"))?;
    let Some(model) = manifest.models.into_iter().find(|model| model.plugin_id == plugin_id && model.root.join("📦️component.wasm") == path) else { return Ok(None) };
    Ok(model.prepared_actor.map(|actor| StagedActorPreparation {
        image: model.root.join(&policy.prepared_actor_image),
        image_sha256: parse_sha256(&actor.image_sha256).expect("validated staged digest"),
        image_bytes: actor.image_bytes,
        maximum_bytes: policy.maximum_prepared_actor_bytes,
    }))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MissingModelPolicy {
    details_key: String,
    cause: String,
    summary: MissingModelSummary,
}

#[derive(Deserialize)]
struct MissingModelSummary {
    en: String,
    de: String,
}

fn missing_model_policy() -> &'static MissingModelPolicy {
    static POLICY: std::sync::OnceLock<MissingModelPolicy> = std::sync::OnceLock::new();
    POLICY.get_or_init(|| {
        let schema: serde_json::Value = serde_json::from_str(include_str!("../../📡️stdio/📦️component-models/🔣️.json")).expect("valid MCP model schema");
        serde_json::from_value(schema["definitions"]["MissingModel"]["const"].clone()).expect("valid MCP missing model policy")
    })
}

/// 🕳️ The retryable refusal naming the plugin whose model the launcher has not published; the launcher builds exactly that model on sight.
pub fn missing_plugin_model_error(entry: &PluginRegistryEntry) -> GatewayError {
    let policy = missing_model_policy();
    let summary = |text: &str| text.replace("{pluginId}", &entry.plugin_id);
    let mut details = serde_json::Map::new();
    details.insert("cause".into(), policy.cause.clone().into());
    details.insert(policy.details_key.clone(), serde_json::json!({ "pluginId": entry.plugin_id, "cratePath": entry.crate_path.to_string_lossy() }));
    details.insert("summary".into(), serde_json::json!({ "en": summary(&policy.summary.en), "de": summary(&policy.summary.de) }));
    GatewayError::new(GatewayErrorCode::PluginUnavailable, summary(&policy.summary.en)).with_details(details.into()).retryable()
}

/// 🔎️ The typed missing-model details of a refusal [`missing_plugin_model_error`] raised, so a layer that cannot carry `details` natively can carry exactly these.
pub fn missing_model_details(error: &GatewayError) -> Option<&serde_json::Value> {
    (error.code == GatewayErrorCode::PluginUnavailable && error.details.get(missing_model_policy().details_key.as_str()).is_some()).then_some(&error.details)
}

fn resolve_plugin_model_root(repo_root: &Path, entry: &PluginRegistryEntry) -> Result<PathBuf, GatewayError> {
    let unavailable = || missing_plugin_model_error(entry);
    let source = std::env::var(&component_model_policy().environment).map_err(|_| unavailable())?;
    let manifest = read_component_model_manifest(&source, env!("SEMIO_BUILD_MODE"))?;
    if !entry.owner_root.starts_with(repo_root) { return Err(unavailable()); }
    manifest.models.into_iter().find(|row| row.plugin_id == entry.plugin_id && Path::new(&row.crate_path) == entry.crate_path.as_path()).map(|row| row.root).ok_or_else(unavailable)
}

/// 🗺️ Resolves a component in the model publication retained by the gateway launcher.
pub fn resolve_plugin_wasm_path(repo_root: &Path, entry: &PluginRegistryEntry) -> Result<PathBuf, GatewayError> {
    let path = resolve_plugin_model_root(repo_root, entry)?.join("📦️component.wasm");
    if !std::fs::symlink_metadata(&path).is_ok_and(|facts| facts.is_file()) {
        return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "retained MCP component was removed or replaced").retryable());
    }
    Ok(path)
}

/// 🧾️ Loads the validated semantic descriptor from the same publication as the guest.
pub fn load_plugin_model_descriptor(repo_root: &Path, entry: &PluginRegistryEntry) -> Result<semio_framework::PackageDescriptor, GatewayError> {
    let root = resolve_plugin_model_root(repo_root, entry)?;
    let descriptor = load_package_descriptor(&root)?;
    if descriptor.manifest.plugin_id != entry.plugin_id {
        return Err(GatewayError::new(GatewayErrorCode::Internal, "retained MCP model descriptor identity differs from its catalog owner"));
    }
    Ok(descriptor)
}

/// 📘️ Uses the installed model's descriptor for discovery and the authored declaration for unbuilt plugins.
pub fn load_registry_plugin_descriptor(repo_root: &Path, entry: &PluginRegistryEntry) -> Result<semio_framework::PackageDescriptor, GatewayError> {
    match load_plugin_model_descriptor(repo_root, entry) {
        Err(error) if error.code == GatewayErrorCode::PluginUnavailable => load_package_descriptor(&entry.owner_root),
        result => result,
    }
}

//#endregion 🔖️PluginPaths

//#region 🔖️Descriptor

/// 📖️ Reads `<owner_root>/🔣️.json` and decodes it as a real `semio_framework::PackageDescriptor` [Design notes](README.md#source-notes-86).
pub fn load_package_descriptor(owner_root: &Path) -> Result<semio_framework::PackageDescriptor, GatewayError> {
    let descriptor_path = owner_root.join("🔣️.json");
    let text = std::fs::read_to_string(&descriptor_path).map_err(|error| GatewayError::new(GatewayErrorCode::NotFound, format!("no committed descriptor at {}: {error}", descriptor_path.display())))?;
    serde_json::from_str(&text).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("{} did not decode as a PackageDescriptor: {error}", descriptor_path.display())))
}

//#endregion 🔖️Descriptor

#[cfg(test)]
mod tests {
    use super::{missing_plugin_model_error, read_component_model_manifest, PluginRegistryEntry};

    #[test]
    fn model_manifest_binds_the_exact_mode_crate_and_plugin() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🤝️client-journey/🔣️.json")).unwrap();
        let root = std::env::current_dir().unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            let mode = row["mode"].as_str().unwrap();
            let models: Vec<_> = row["models"].as_array().unwrap().iter().map(|model| {
                let mut fields = model.as_object().unwrap().clone();
                let location = fields.remove("location").unwrap();
                fields.insert("root".into(), serde_json::json!(root.join(location.as_str().unwrap())));
                serde_json::Value::Object(fields)
            }).collect();
            let manifest = serde_json::json!({"version": fixture["version"], "mode": mode, "models": models});
            assert_eq!(read_component_model_manifest(&manifest.to_string(), mode).is_ok(), row["expected"].as_bool().unwrap(), "{}", row["name"]);
        }
    }

    #[test]
    fn missing_model_refusal_names_the_plugin_as_the_launcher_schema_declares() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📡️stdio/📦️component-models/🧪️tests/🔣️.json")).unwrap();
        for row in fixture["missing"].as_array().unwrap() {
            let entry = PluginRegistryEntry { plugin_id: row["pluginId"].as_str().unwrap().into(), owner_root: "/owner".into(), crate_path: row["cratePath"].as_str().unwrap().into() };
            assert_eq!(missing_plugin_model_error(&entry).to_tool_error_payload(), row["payload"], "{}", row["name"]);
        }
    }
}
