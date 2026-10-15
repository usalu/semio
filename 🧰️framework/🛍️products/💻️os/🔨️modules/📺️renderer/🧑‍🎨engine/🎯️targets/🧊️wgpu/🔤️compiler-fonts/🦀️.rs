//! 🔤️ The renderer's side of the compiler font port. `semio_framework_canvas` resolves math, emoji and
//! text icons through the compiler's font set, which no crate embeds; this module is where the two
//! renderer hosts hand those bytes in, once, before the first icon resolves.
//!
//! 🖥️ Native: [`install_native_compiler_fonts`] reads the five `FontRole::asset_file_name` files from
//! the host's font directory, `SEMIO_COMPILER_FONTS` when the runner names one, otherwise the repository
//! directory found above the executable or the plugin module root.
//!
//! 🌐️ Browser: `semioWgpuSetCompilerFonts` receives the five files the frame Worker fetched from the
//! served `/renderer-modules/compiler-fonts` route, before `semioWgpuWorkerBootstrap`.

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_canvas::{install_compiler_fonts, CompilerFontsInstallError, FontSet};
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};

//#region 🔖️Native
/// 🧭️ The variable a native runner sets to the font directory it serves.
#[cfg(not(target_arch = "wasm32"))]
pub const COMPILER_FONTS_ENV: &str = "SEMIO_COMPILER_FONTS";

/// 📂️ Where the compiler's font assets live, relative to the repository root.
#[cfg(not(target_arch = "wasm32"))]
pub const COMPILER_FONTS_REPOSITORY_DIRECTORY: &str = "🧰️framework/🔨️modules/📚️compiler/🌍️world/🔤️fonts";

/// 🔎️ An explicit directory must be a directory; without one, the first anchor with a repository font
/// directory among its ancestors wins.
#[cfg(not(target_arch = "wasm32"))]
pub fn compiler_font_directory(explicit: Option<&Path>, anchors: &[PathBuf]) -> Result<PathBuf, String> {
    if let Some(directory) = explicit {
        return if directory.is_dir() { Ok(directory.to_path_buf()) } else { Err(format!("{COMPILER_FONTS_ENV} names {}, which is not a directory", directory.display())) };
    }
    anchors
        .iter()
        .flat_map(|anchor| anchor.ancestors())
        .map(|ancestor| ancestor.join(COMPILER_FONTS_REPOSITORY_DIRECTORY))
        .find(|candidate| candidate.is_dir())
        .ok_or_else(|| format!("no {COMPILER_FONTS_REPOSITORY_DIRECTORY} above {} anchor(s) and {COMPILER_FONTS_ENV} is unset", anchors.len()))
}

/// 🔌️ Loads the font set from `directory` and installs it; an already installed set is the same process
/// booting twice and stays installed.
#[cfg(not(target_arch = "wasm32"))]
pub fn install_compiler_fonts_from_directory(directory: &Path) -> Result<(), String> {
    let set = FontSet::from_directory(directory).map_err(|error| error.to_string())?;
    match install_compiler_fonts(set) {
        Ok(()) | Err(CompilerFontsInstallError::AlreadyInstalled) => Ok(()),
        Err(failure) => Err(failure.to_string()),
    }
}

/// 🖥️ The native boot step: resolves the directory from the environment and the process location, then
/// installs. A host without the assets keeps booting and says so, because only math, emoji and text
/// icons depend on them.
#[cfg(not(target_arch = "wasm32"))]
pub fn install_native_compiler_fonts() {
    let explicit = std::env::var_os(COMPILER_FONTS_ENV).map(PathBuf::from);
    let anchors: Vec<PathBuf> = [std::env::current_exe().ok(), std::env::var_os("SEMIO_PLUGIN_MODULES").map(PathBuf::from), std::env::current_dir().ok()].into_iter().flatten().collect();
    if let Err(error) = compiler_font_directory(explicit.as_deref(), &anchors).and_then(|directory| install_compiler_fonts_from_directory(&directory)) {
        eprintln!("[TRACE] native compiler fonts unavailable: {error}; math, emoji and text icons will not render");
    }
}
//#endregion 🔖️Native

//#region 🔖️Browser
/// 🌐️ The browser door: the frame Worker passes the five fetched files in `FontRole::ALL` order.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = semioWgpuSetCompilerFonts)]
pub fn semio_wgpu_set_compiler_fonts(math: Vec<u8>, serif: Vec<u8>, serif_italic: Vec<u8>, mono: Vec<u8>, emoji: Vec<u8>) -> Result<(), wasm_bindgen::JsValue> {
    semio_framework_canvas::install_compiler_font_bytes(math, serif, serif_italic, mono, emoji).map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}
//#endregion 🔖️Browser

//#region 🧪️Tests
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
