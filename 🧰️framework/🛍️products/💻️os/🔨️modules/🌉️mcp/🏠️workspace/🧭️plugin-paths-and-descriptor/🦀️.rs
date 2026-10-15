//! 🧭️ Plugin registry rows, repo-root discovery, component paths and committed package descriptors.

use super::{GatewayError, GatewayErrorCode, Path, PathBuf, is_repository_root};

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
    pub wasm_out: String,
}

use semio_framework_os_kernel::native_catalog as component_artifacts;

/// 🧭️ Locates the repo root — identical algorithm to `🏃️run/🏗️bootstrap/🦀️.rs::find_repo_root` (walk up from
/// this crate's own `CARGO_MANIFEST_DIR` for paired build and package workspace markers, `SEMIO_REPO_ROOT` override first) so a direct
/// `cargo run -p semio-framework-os-mcp` from an arbitrary cwd still resolves correctly.
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
        entries.push(PluginRegistryEntry{plugin_id:row.plugin_id.clone(),owner_root,crate_path:PathBuf::from(&row.crate_path),wasm_out:row.wasm_out.clone()});
    }
    Ok(entries)
}

pub fn find_plugin_entry<'a>(entries: &'a [PluginRegistryEntry], plugin_id: &str) -> Result<&'a PluginRegistryEntry, GatewayError> {
    entries.iter().find(|entry| entry.plugin_id == plugin_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("plugin `{plugin_id}` is not in the plugin registry")))
}

/// 🗺️ Resolves one plugin's shipped component under its crate's `dist/components` — the same path as
/// `🏃️run/🏗️bootstrap/🦀️.rs::resolve_plugin_paths` and the TypeScript preflight in `🌉️mcp/🟦️.ts`.
pub fn resolve_plugin_wasm_path(repo_root: &Path, entry: &PluginRegistryEntry) -> Result<PathBuf, GatewayError> {
    let directory = component_artifacts::plugin_component_directory(env!("SEMIO_BUILD_MODE")).map_err(|message| GatewayError::new(GatewayErrorCode::Internal, message))?;
    let path = repo_root.join(&entry.crate_path).join(directory).join(&entry.wasm_out);
    if path.is_file() {
        return Ok(path);
    }
    Err(GatewayError::new(GatewayErrorCode::NotFound, format!("plugin `{}`'s shipped component is missing at {}; build it with `bun repo run <its crate project>:describe`", entry.plugin_id, path.display())))
}
//#endregion 🔖️PluginPaths

//#region 🔖️Descriptor
/// 📖️ Reads `<owner_root>/🔣️.json` and decodes it as a real `semio_framework::PackageDescriptor`
/// — the same JSON `semio-framework-plugin-describe` writes (packet E1/E2, `📓️status.md`'s
/// "E2-builder-descriptor" entry) and the same type `🏃️run/🦀️.rs`'s own `WasmtimeNodeHost`
/// doc comment names as the eventual manifest source, once wired. Reading the COMMITTED descriptor
/// JSON directly (rather than instantiating the wasm to call `describe()` at runtime, the way
/// `semio-framework-plugin-describe` does at BUILD time) is deliberate: `🏃️run`'s own
/// `load_runtime_recursive` is blocked precisely because no `describe()`-decoder is wired into that
/// crate — this sidesteps that gap entirely for any plugin that (like `🗒️note`) already has a real
/// committed descriptor, without needing a second wasmtime linkage inside the gateway just to read a
/// manifest.
pub fn load_package_descriptor(owner_root: &Path) -> Result<semio_framework::PackageDescriptor, GatewayError> {
    let descriptor_path = owner_root.join("🔣️.json");
    let text = std::fs::read_to_string(&descriptor_path).map_err(|error| GatewayError::new(GatewayErrorCode::NotFound, format!("no committed descriptor at {}: {error}", descriptor_path.display())))?;
    serde_json::from_str(&text).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("{} did not decode as a PackageDescriptor: {error}", descriptor_path.display())))
}
//#endregion 🔖️Descriptor

