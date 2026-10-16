//! 🧰️ Catalog source discovery of the capabilities crate: Gateway capability declarations and the core catalog.

use semio_framework_os_kernel::native_catalog::configured_native_catalog_v1;
use std::path::Path;
use crate::core_capabilities::core_tool_capabilities;
use semio_framework_os_mcp_catalog::catalog::CatalogSource;
use semio_framework_os_mcp_catalog::plugin_paths_and_descriptor::find_repo_root;
use semio_framework_os_mcp_catalog::registry::RegistryDiscovery;

//#region 🔖️DiscoverCatalogSource

/// 🗂️ Builds the live gateway's `CatalogSource` — `gateway` is always `crate::core_tool_capabilities()`
/// (Requirement 2: a production server with zero plugins installed still advertises its own gateway
/// tools, honestly, never the note/cad fixture); `descriptors` is whatever real discovery found under
/// `root`, or under `find_repo_root()` when `root` is `None` (the `--folder`-bound caller passes its
/// own root; the bare/no-argument caller lets this resolve the repo/space root itself). Root
/// resolution failing, or the resolved root having no registry, both degrade to an empty descriptor
/// list — never a fallback to fixture data, never a panic.
pub fn discover_catalog_source(root: Option<&Path>) -> CatalogSource {
    let catalog=configured_native_catalog_v1().expect("Configured MCP native catalog refused");
    let resolved_root = root.map(Path::to_path_buf).or_else(|| find_repo_root().ok());
    let descriptors = match resolved_root {
        Some(resolved) if resolved.is_dir() => {
            let discovery=RegistryDiscovery::scan(resolved,&catalog).expect("Configured MCP package ownership refused");
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
    CatalogSource { descriptors, os_commands: Vec::new(), shell: Vec::new(), gateway: core_tool_capabilities() }
}

//#endregion 🔖️DiscoverCatalogSource
