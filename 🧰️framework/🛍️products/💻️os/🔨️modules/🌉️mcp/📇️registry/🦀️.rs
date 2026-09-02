//! 📇️ `registry` — ticket 26/08/29/AI-MCP-END-TO-END packet W1: discovers the REAL installed plugin
//! set at runtime (reusing `🏠️workspace`'s own `load_plugin_registry`/`load_package_descriptor` — the
//! SAME generated `🔌️plugin/📇️registry/🤖️generated/🔣️plugins.json` + committed `descriptor.json` files
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
mod quick {
    use super::*;

    /// 📝️ Writes a synthetic `🔣️plugins.json` under `root`, matching `load_plugin_registry`'s real
    /// schema (`pluginId`/`cratePath`/`wasmOut`) — `cratePath` is `<owner_rel>/📦️packages/🦀️rust` so
    /// `load_plugin_registry`'s own owner-root derivation (two components back) resolves to
    /// `root.join(owner_rel)`, exactly like the real generated registry.
    fn write_registry(root: &Path, rows: &[(&str, &str)]) {
        let entries: Vec<serde_json::Value> =
            rows.iter().map(|(plugin_id, owner_rel)| serde_json::json!({ "pluginId": plugin_id, "cratePath": format!("{owner_rel}/📦️packages/🦀️rust"), "wasmOut": format!("{plugin_id}.wasm") })).collect();
        let registry_path = root.join("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔣️plugins.json");
        std::fs::create_dir_all(registry_path.parent().expect("registry path has a parent")).expect("create registry dir");
        std::fs::write(&registry_path, serde_json::to_vec(&entries).expect("registry rows serialize")).expect("write registry");
    }

    fn write_descriptor(root: &Path, owner_rel: &str, descriptor: &manifest::PackageDescriptor) {
        let owner_root = root.join(owner_rel);
        std::fs::create_dir_all(&owner_root).expect("create owner root");
        std::fs::write(owner_root.join("🔣️.json"), serde_json::to_vec(descriptor).expect("descriptor serializes")).expect("write descriptor");
    }

    #[test]
    fn discovers_exactly_the_descriptors_a_synthetic_registry_names() {
        let dir = store::test_support::tempdir().expect("tempdir");
        let root = dir.path().to_path_buf();
        write_registry(&root, &[("note", "plugins/note"), ("cad", "plugins/cad")]);
        write_descriptor(&root, "plugins/note", &crate::note_descriptor());
        write_descriptor(&root, "plugins/cad", &crate::cad_descriptor());

        let descriptors = discover_descriptors(&root).expect("discovery over a valid directory never hard-fails");
        let ids: Vec<&str> = descriptors.iter().map(|descriptor| descriptor.manifest.plugin_id.as_str()).collect();
        assert_eq!(ids, vec!["cad", "note"], "sorted by plugin id, exactly the two synthetic plugins, nothing else");
    }

    #[test]
    fn a_malformed_descriptor_is_skipped_with_a_diagnostic_not_a_panic_or_err() {
        let dir = store::test_support::tempdir().expect("tempdir");
        let root = dir.path().to_path_buf();
        write_registry(&root, &[("broken", "plugins/broken")]);
        let owner_root = root.join("plugins/broken");
        std::fs::create_dir_all(&owner_root).expect("create owner root");
        std::fs::write(owner_root.join("🔣️.json"), b"not valid json").expect("write malformed descriptor");

        let discovery = RegistryDiscovery::scan(root.clone());
        assert!(discovery.descriptors.is_empty(), "the malformed descriptor must not decode into a fabricated value");
        assert_eq!(discovery.diagnostics().len(), 1);
        assert!(discovery.diagnostics()[0].contains("broken"), "the diagnostic names the skipped plugin: {:?}", discovery.diagnostics());

        let descriptors = discover_descriptors(&root).expect("a malformed descriptor is a diagnostic, not a hard Err");
        assert!(descriptors.is_empty());
    }

    #[test]
    fn a_missing_registry_yields_an_empty_descriptor_list_plus_gateway_capabilities() {
        let dir = store::test_support::tempdir().expect("tempdir");
        let source = discover_catalog_source(Some(dir.path()));
        assert!(source.descriptors.is_empty(), "no registry was ever written under this tempdir");
        assert_eq!(source.gateway, crate::core_tool_capabilities(), "gateway capabilities are always present, fixture-independent");
        assert!(source.shell.is_empty());
        assert!(source.os_commands.is_empty());
    }

    #[test]
    fn the_same_input_compiles_to_a_byte_identical_catalog_hash_twice() {
        let dir = store::test_support::tempdir().expect("tempdir");
        let root = dir.path().to_path_buf();
        write_registry(&root, &[("note", "plugins/note"), ("cad", "plugins/cad")]);
        write_descriptor(&root, "plugins/note", &crate::note_descriptor());
        write_descriptor(&root, "plugins/cad", &crate::cad_descriptor());

        let source_a = discover_catalog_source(Some(&root));
        let source_b = discover_catalog_source(Some(&root));
        let catalog_a = crate::compile(&source_a, semio_framework::Locale::En, semio_framework::Terminology::Native).expect("compiles");
        let catalog_b = crate::compile(&source_b, semio_framework::Locale::En, semio_framework::Terminology::Native).expect("compiles");
        assert_eq!(catalog_a.hash, catalog_b.hash, "discovering + compiling the same install twice must be byte-identical");
    }
}
//#endregion 🧪️Tests
