//! 📇️ `registry` — ticket 26/08/29/AI-MCP-END-TO-END packet W1: discovers the REAL installed plugin
//! set at runtime (reusing `🏠️workspace`'s own `load_plugin_registry`/`load_package_descriptor` — the
//! SAME generated `🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json` + committed `descriptor.json` files
//! `find_repo_root` already resolves, never a second copy of that logic) and compiles the live
//! gateway's `CatalogSource` from it, replacing `root::build_catalog`'s previous hardcoded
//! `🧫️note_and_cad_source()` fixture (`📓️status.md` B2: "the production catalog is a hardcoded
//! note+cad fixture").
//!
//! Never fails hard: a missing/unreadable registry, a malformed descriptor, or a plugin with no
//! committed descriptor is skipped and recorded as a diagnostic (printed to stderr, never silently
//! swallowed), not a panic and not a propagated `Err` — discovery returning zero descriptors is the
//! legitimate "bare" tier (no plugins installed), and [`discover_catalog_source`] NEVER falls back to
//! the note/cad fixture: a production server with nothing installed advertises
//! `crate::core_tool_capabilities()` only, honestly. Descriptors are sorted by plugin id before
//! `compile()` sees them so `catalog_hash` stays deterministic across runs on the same install.

use crate::{find_repo_root, load_package_descriptor, load_plugin_registry, CatalogSource, GatewayError, GatewayErrorCode};
use semio_framework::manifest;
use std::path::{Path, PathBuf};

//#region 🔖️RegistryDiscovery
/// 📇️ One completed discovery pass over the real plugin registry under `root` — every descriptor's
/// on-disk path, the descriptors that decoded successfully, and every diagnostic recorded along the
/// way (missing registry, missing owner root, malformed JSON, …). Carries no hard error itself: an
/// empty `descriptors` is a legitimate outcome, never a reason to panic or propagate.
#[derive(Clone, Debug, Default)]
pub struct RegistryDiscovery {
    pub root: PathBuf,
    pub descriptor_paths: Vec<PathBuf>,
    pub descriptors: Vec<manifest::PackageDescriptor>,
    diagnostics: Vec<String>,
}

impl RegistryDiscovery {
    /// 🔎️ Walks `root`'s generated plugin registry and decodes every entry's committed
    /// `descriptor.json`, skipping (never panicking on) anything that goes wrong — an unreadable
    /// registry, a plugin with no committed descriptor, or a descriptor that fails to parse each
    /// become one diagnostic string rather than an aborted scan. Results are sorted by plugin id
    /// (Requirement 3: deterministic input order for a stable `catalog_hash`).
    pub fn scan(root: PathBuf) -> Self {
        let mut diagnostics = Vec::new();
        let entries = match load_plugin_registry(&root) {
            Ok(entries) => entries,
            Err(error) => {
                diagnostics.push(format!("plugin registry unavailable under {}: {error}", root.display()));
                return Self { root, descriptor_paths: Vec::new(), descriptors: Vec::new(), diagnostics };
            }
        };
        let mut descriptor_paths = Vec::with_capacity(entries.len());
        let mut descriptors = Vec::with_capacity(entries.len());
        for entry in &entries {
            descriptor_paths.push(entry.owner_root.join("🔣️.json"));
            match load_package_descriptor(&entry.owner_root) {
                Ok(descriptor) => descriptors.push(descriptor),
                Err(error) => diagnostics.push(format!("skipping plugin `{}`: {error}", entry.plugin_id)),
            }
        }
        descriptors.sort_by(|a, b| a.manifest.plugin_id.cmp(&b.manifest.plugin_id));
        Self { root, descriptor_paths, descriptors, diagnostics }
    }

    /// 📢️ Every diagnostic recorded during the scan — the caller (here: this module's own free
    /// functions) MUST surface these somewhere visible (stderr) rather than swallow them, per the
    /// packet brief's "never silently swallow" requirement.
    pub fn diagnostics(&self) -> &[String] {
        &self.diagnostics
    }
}
//#endregion 🔖️RegistryDiscovery

//#region 🔖️DiscoverDescriptors
/// 📖️ Decodes every real, installed plugin's `PackageDescriptor` under `root`, sorted by plugin id.
/// `Err` is reserved for `root` itself being unusable (not a directory) — a missing registry or a
/// malformed descriptor is a per-plugin diagnostic (printed to stderr), never a hard failure of the
/// whole call (Requirement 1).
pub fn discover_descriptors(root: &Path) -> Result<Vec<manifest::PackageDescriptor>, GatewayError> {
    if !root.is_dir() {
        return Err(GatewayError::new(GatewayErrorCode::NotFound, format!("registry discovery root {} is not a directory", root.display())));
    }
    let discovery = RegistryDiscovery::scan(root.to_path_buf());
    for diagnostic in discovery.diagnostics() {
        eprintln!("[mcp registry] {diagnostic}");
    }
    Ok(discovery.descriptors)
}
//#endregion 🔖️DiscoverDescriptors

//#region 🔖️DiscoverCatalogSource
/// 🗂️ Builds the live gateway's `CatalogSource` — `gateway` is always `crate::core_tool_capabilities()`
/// (Requirement 2: a production server with zero plugins installed still advertises its own gateway
/// tools, honestly, never the note/cad fixture); `descriptors` is whatever real discovery found under
/// `root`, or under `find_repo_root()` when `root` is `None` (the `--folder`-bound caller passes its
/// own root; the bare/no-argument caller lets this resolve the repo/space root itself). Root
/// resolution failing, or the resolved root having no registry, both degrade to an empty descriptor
/// list — never a fallback to fixture data, never a panic.
pub fn discover_catalog_source(root: Option<&Path>) -> CatalogSource {
    let resolved_root = root.map(Path::to_path_buf).or_else(|| find_repo_root().ok());
    let descriptors = match resolved_root {
        Some(resolved) if resolved.is_dir() => {
            let discovery = RegistryDiscovery::scan(resolved);
            for diagnostic in discovery.diagnostics() {
                eprintln!("[mcp registry] {diagnostic}");
            }
            discovery.descriptors
        }
        Some(resolved) => {
            eprintln!("[mcp registry] discovery root {} is not a directory — serving gateway-only capabilities", resolved.display());
            Vec::new()
        }
        None => {
            eprintln!("[mcp registry] could not locate a repo/space root — serving gateway-only capabilities");
            Vec::new()
        }
    };
    CatalogSource { descriptors, os_commands: Vec::new(), shell: Vec::new(), gateway: crate::core_tool_capabilities() }
}
//#endregion 🔖️DiscoverCatalogSource

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️quick/🦀️.rs"]
mod quick;
//#endregion 🧪️Tests
