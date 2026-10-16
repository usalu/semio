//! 🔤️ The renderer's side of Canvas's runtime asset port. `semio_framework_canvas` resolves icons and labels through the
//! fonts and the icon pack no crate embeds; this module is where the native renderer host hands those bytes in, once, before
//! the first icon resolves.
//!
//! 🖥️ Native: [`install_native_runtime_assets`] reads the seven `RuntimeAsset` files from the first root that exists: the
//! repository-shaped root `SEMIO_RUNTIME_ASSETS` names (an override), the staged `runtime-assets` folder next to the executable
//! (what a packaged install ships, the `stage-runtime-assets` output laid out as `<channel>/<file>`), or the repository found above
//! the executable, the plugin module root or the working directory.
//!
//! 🌐️ Browser: the frame Worker fetches the seven files from the served `/renderer-modules/*` routes and calls the
//! `semioSetRuntimeAssets` door Canvas exports from every wasm module, before `semioWgpuWorkerBootstrap`.

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_canvas::{install_runtime_assets, FontRole, RuntimeAsset, RuntimeAssets, RuntimeAssetsInstallError};
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};

//#region 🔖️Native
/// 🧭️ The variable a native runner sets to the repository-shaped root it serves the runtime assets from; it overrides the defaults.
#[cfg(not(target_arch = "wasm32"))]
pub const RUNTIME_ASSETS_ENV: &str = "SEMIO_RUNTIME_ASSETS";

/// 📦️ The folder next to the executable a packaged install stages the assets in, laid out as `<channel>/<file>`.
#[cfg(not(target_arch = "wasm32"))]
pub const STAGED_DIRECTORY: &str = "runtime-assets";

/// 🗺️ Where the seven assets are read from.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeAssetRoot {
    /// 📂️ A repository-shaped root: `<root>/<repository directory>/<file>`.
    Repository(PathBuf),
    /// 📦️ A staged folder: `<root>/<channel>/<file>`.
    Staged(PathBuf),
}

/// 🔎️ An explicit root must be a directory and wins; without one, the `runtime-assets` folder beside `executable` wins when it holds
/// the compiler font channel, then the first anchor with a repository above it, recognised by the compiler font directory every
/// checkout holds.
#[cfg(not(target_arch = "wasm32"))]
pub fn runtime_asset_root(explicit: Option<&Path>, executable: Option<&Path>, anchors: &[PathBuf]) -> Result<RuntimeAssetRoot, String> {
    if let Some(root) = explicit {
        return if root.is_dir() { Ok(RuntimeAssetRoot::Repository(root.to_path_buf())) } else { Err(format!("{RUNTIME_ASSETS_ENV} names {}, which is not a directory", root.display())) };
    }
    let staged = executable.and_then(Path::parent).map(|directory| directory.join(STAGED_DIRECTORY)).filter(|directory| directory.join(RuntimeAsset::CompilerFont(FontRole::Math).channel()).is_dir());
    if let Some(directory) = staged {
        return Ok(RuntimeAssetRoot::Staged(directory));
    }
    let marker = RuntimeAsset::CompilerFont(FontRole::Math).repository_directory();
    anchors
        .iter()
        .flat_map(|anchor| anchor.ancestors())
        .find(|ancestor| ancestor.join(marker).is_dir())
        .map(|ancestor| RuntimeAssetRoot::Repository(ancestor.to_path_buf()))
        .ok_or_else(|| format!("no {STAGED_DIRECTORY} folder beside the executable, no repository holding {marker} above {} anchor(s) and {RUNTIME_ASSETS_ENV} is unset", anchors.len()))
}

/// 🔌️ Loads the seven assets from `root` and installs them; an already installed set is the same process booting twice and
/// stays installed.
#[cfg(not(target_arch = "wasm32"))]
pub fn install_runtime_assets_from_root(root: &RuntimeAssetRoot) -> Result<(), String> {
    let assets = match root {
        RuntimeAssetRoot::Repository(path) => RuntimeAssets::from_root(path),
        RuntimeAssetRoot::Staged(path) => RuntimeAssets::from_staged_root(path),
    }
    .map_err(|error| error.to_string())?;
    match install_runtime_assets(assets) {
        Ok(()) | Err(RuntimeAssetsInstallError::AlreadyInstalled) => Ok(()),
        Err(failure) => Err(failure.to_string()),
    }
}

/// 🖥️ The native boot step: resolves the root from the environment and the process location, then installs. A host without
/// the assets keeps booting and says so, because only icons and labels depend on them.
#[cfg(not(target_arch = "wasm32"))]
pub fn install_native_runtime_assets() {
    let explicit = std::env::var_os(RUNTIME_ASSETS_ENV).map(PathBuf::from);
    let executable = std::env::current_exe().ok();
    let anchors: Vec<PathBuf> = [executable.clone(), std::env::var_os("SEMIO_PLUGIN_MODULES").map(PathBuf::from), std::env::current_dir().ok()].into_iter().flatten().collect();
    if let Err(error) = runtime_asset_root(explicit.as_deref(), executable.as_deref(), &anchors).and_then(|root| install_runtime_assets_from_root(&root)) {
        eprintln!("[TRACE] native runtime assets unavailable: {error}; icons and labels will not render");
    }
}
//#endregion 🔖️Native

//#region 🧪️Tests
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
