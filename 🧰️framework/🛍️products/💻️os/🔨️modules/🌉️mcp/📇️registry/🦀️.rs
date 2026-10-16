//! 📇️ Discovers only packages supplied by the calling native catalog; configured publication failures refuse.
//! Descriptor failures remain visible diagnostics, and an absent publication supplies gateway-only capabilities.

use semio_framework::manifest;
use semio_framework_os_kernel::native_catalog::NativeCatalogV1;
use semio_framework_os_kernel::native_catalog::configured_native_catalog_v1;
use std::path::Path;
use std::path::PathBuf;
use crate::plugin_paths_and_descriptor::{load_package_descriptor, plugin_registry_entries};
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};

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
    pub diagnostics: Vec<String>,
}

impl RegistryDiscovery {
    /// 🔎️ Reads the caller's exact contributed owners and retains per-descriptor diagnostics.
    pub fn scan(root:PathBuf,catalog:&NativeCatalogV1)->Result<Self,GatewayError>{
        let mut diagnostics=Vec::new();let entries=plugin_registry_entries(&root,catalog)?;
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
        Ok(Self { root, descriptor_paths, descriptors, diagnostics })
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
    let catalog=configured_native_catalog_v1().map_err(|error|GatewayError::new(GatewayErrorCode::Internal,error))?;
    let discovery=RegistryDiscovery::scan(root.to_path_buf(),&catalog)?;
    for diagnostic in discovery.diagnostics() {
        eprintln!("[mcp registry] {diagnostic}");
    }
    Ok(discovery.descriptors)
}

//#endregion 🔖️DiscoverDescriptors

//#region 🧪️Tests
//#endregion 🧪️Tests
