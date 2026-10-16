//! 🎁️ Delivery of a package's declared file assets to a guest at open: the host half of `Event::InstanceOpen.assets`
//! (`📓️design-abi.md` §2, the `read-asset` replacement), shared by every native host that opens a guest. A package declares
//! an asset by name, exact size and SHA-256 (`PluginBuilder::asset`); when the declared name is the repository-relative path
//! of an existing file, the host reads it, checks it against the declaration and preloads it into the open event. A declared
//! name that is no file in the repository (an externalized example body, for instance) is not this module's to deliver and
//! is skipped. The browser kernel keeps the same contract (`🎠️kernel/📦️package-assets`).

use semio_framework::AssetDeclaration;
use std::path::{Component, Path, PathBuf};

/// 📥️ The declared assets that are files below `repo_root`, as `(name, bytes)` in declaration order. A file whose size or
/// SHA-256 differs from its declaration is refused by name, because delivering other bytes than the package declared
/// would make the guest run on content its descriptor never admitted.
pub fn package_file_assets(repo_root: &Path, declared: &[AssetDeclaration]) -> Result<Vec<(String, Vec<u8>)>, String> {
    let mut delivered = Vec::new();
    for declaration in declared {
        let relative = Path::new(&declaration.name);
        if relative.is_absolute() || !relative.components().all(|component| matches!(component, Component::Normal(_))) {
            continue;
        }
        let path = repo_root.join(relative);
        if !path.is_file() {
            continue;
        }
        let bytes = std::fs::read(&path).map_err(|error| format!("package asset {}: {error}", declaration.name))?;
        if bytes.len() as u64 != declaration.size_bytes {
            return Err(format!("package asset {} has {} bytes, its declaration says {}", declaration.name, bytes.len(), declaration.size_bytes));
        }
        let sha256 = semio_framework_hash::sha256_hex(&bytes);
        if sha256 != declaration.sha256 {
            return Err(format!("package asset {} hashes to {sha256}, its declaration says {}", declaration.name, declaration.sha256));
        }
        delivered.push((declaration.name.clone(), bytes));
    }
    Ok(delivered)
}

/// 🔍️ The nearest ancestor of `start` (or `start` itself) that is the repository root, marked by `nx.json`.
pub fn repository_root_above(start: &Path) -> Option<PathBuf> {
    start.ancestors().find(|candidate| candidate.join("nx.json").is_file()).map(Path::to_path_buf)
}

/// 🔍️ The `assets` a package descriptor JSON document declares (`🔣️.json` beside a served module); an absent list is empty.
pub fn declared_assets_in_descriptor_json(text: &str) -> Result<Vec<AssetDeclaration>, String> {
    let document: serde_json::Value = serde_json::from_str(text).map_err(|error| format!("package descriptor is not JSON: {error}"))?;
    match document.get("assets") {
        None => Ok(Vec::new()),
        Some(assets) => serde_json::from_value(assets.clone()).map_err(|error| format!("package descriptor assets are malformed: {error}")),
    }
}

/// 🎁️ What a host that loaded the module `<dir>/<file>.wasm` delivers on open: the assets declared by the descriptor
/// `<dir>/🔣️.json` beside it, resolved below the repository root above the module. A module without a descriptor, or one
/// outside any repository, delivers none.
pub fn package_assets_beside_module(wasm_path: &Path) -> Result<Vec<(String, Vec<u8>)>, String> {
    let Some(directory) = wasm_path.parent() else { return Ok(Vec::new()) };
    let Ok(text) = std::fs::read_to_string(directory.join("🔣️.json")) else { return Ok(Vec::new()) };
    let declared = declared_assets_in_descriptor_json(&text)?;
    if declared.is_empty() {
        return Ok(Vec::new());
    }
    match repository_root_above(directory) {
        Some(root) => package_file_assets(&root, &declared),
        None => Ok(Vec::new()),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
