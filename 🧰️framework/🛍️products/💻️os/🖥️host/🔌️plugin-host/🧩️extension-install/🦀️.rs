//! 🧩️ Extension install of the plugin host: installed extensions, the install error with its conversions and the extension install and query methods of the plugin host. See [`super`].

use super::*;

// #region host
//#region 🔖️ExtensionInstall
/// 🧩️ One verified, installed `.sxt` extension package — `manifest` is the byte-for-byte
/// unpacked `store::extension::ExtensionPackageManifest` (contract freeze §4 already re-checked
/// by `install_extension_package` below, not merely trusted from `verify`); `content_hash` is
/// the same blake3 digest `store::extension::content_hash` computes over the full `.sxt` bytes,
/// kept alongside so a re-install of byte-identical bytes is detectable without re-hashing.
///
/// Deliberately the `.sxt`-shaped twin of `kernel::ExtensionDescriptor`
/// (`🧰️framework/🔨️modules/🎠️kernel`, this ticket's other owned path) rather than a shared type:
/// this crate has no mount of that file, and `semio-framework-os-kernel` must never depend on
/// `semio-framework` (the same dependency-edge-law reason `PackagePluginDependency`'s own
/// docstring in `🧩️extension/🦀️.rs` gives for ITS wire-shape duplication) — so the two
/// shapes are duplicated on purpose, exactly like that established precedent.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[derive(Clone, Debug, PartialEq)]
pub struct InstalledExtension {
    pub manifest: store::extension::ExtensionPackageManifest,
    pub content_hash: String,
}

/// ⚠️ Failures installing an `.sxt` extension package. `Package` bubbles `store::extension`'s own
/// verify/unpack errors; `ExtendsMismatch` re-checks contract freeze §4
/// (`extends == dependencies[0].plugin_id`) at INSTALL time rather than only trusting the guest
/// SDK-side `assert!` (`ExtensionBundle::assert_extends_matches_primary_dependency`, which only
/// fires when the extension is BUILT) — a hand-crafted or corrupted `.sxt` must still be rejected
/// here, at the one place every runtime-installed extension actually passes through.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[derive(Debug)]
pub enum ExtensionInstallError {
    Package(store::extension::ExtensionPackageError),
    ExtendsMismatch { extension_id: String, extends: String, actual: String },
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl std::fmt::Display for ExtensionInstallError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Package(error) => write!(formatter, "extension package: {error}"),
            Self::ExtendsMismatch { extension_id, extends, actual } => {
                write!(formatter, "extension {extension_id:?} declares extends={extends:?} but its first dependency is {actual:?} — contract freeze §4 requires extends == dependencies[0].plugin_id")
            }
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl std::error::Error for ExtensionInstallError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Package(error) => Some(error),
            Self::ExtendsMismatch { .. } => None,
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl From<store::extension::ExtensionPackageError> for ExtensionInstallError {
    fn from(error: store::extension::ExtensionPackageError) -> Self {
        Self::Package(error)
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl PluginHost {
    /// 📥️ Verifies + unpacks an `.sxt` byte stream and registers its descriptor, keyed by
    /// `extension_id` — idempotent for a byte-identical reinstall. This is the "install" half of
    /// `important.md`'s extension-activation ruling: "verify/unpack an `.sxt` → register its
    /// descriptor". `async` (unlike this impl block's sibling query methods below): `verify` and
    /// `ExtensionPackageManifest::extends_matches_primary_dependency` are still genuinely `async
    /// fn`; `content_hash` is a pure blake3 digest and is called synchronously below.
    pub async fn install_extension_package(&mut self, bytes: &[u8]) -> Result<InstalledExtension, ExtensionInstallError> {
        let manifest = store::extension::verify(bytes).await?;
        if !manifest.extends_matches_primary_dependency().await {
            let actual = manifest.dependencies.first().map(|dependency| dependency.plugin_id.clone()).unwrap_or_default();
            return Err(ExtensionInstallError::ExtendsMismatch { extension_id: manifest.extension_id.clone(), extends: manifest.extends.clone(), actual });
        }
        let content_hash = store::extension::content_hash(bytes);
        let installed = InstalledExtension { manifest, content_hash };
        self.installed_extensions.insert(installed.manifest.extension_id.clone(), installed.clone());
        Ok(installed)
    }

    /// 🗑️ Removes an installed extension's descriptor. Once uninstalled it is no longer handed out
    /// by `extensions_extending_plugin`, whether or not an actor for it is still live — see that
    /// method's own docstring for the deactivation-cascade gap this leaves open (`Kernel` itself
    /// exposes no `deactivate(...)` yet; see the report).
    pub fn uninstall_extension_package(&mut self, extension_id: &str) -> Option<InstalledExtension> {
        self.installed_extensions.remove(extension_id)
    }

    pub fn installed_extension(&self, extension_id: &str) -> Option<&InstalledExtension> {
        self.installed_extensions.get(extension_id)
    }

    /// 🔍️ Every installed extension descriptor whose `extends` names `plugin_id` — the exact
    /// query `important.md`'s extension-activation ruling specifies: "on plugin activation, the
    /// kernel queries installed descriptors for `extends == plugin_id` and activates each as
    /// `ActorKind::Extension`". Data-driven over `self.installed_extensions`: correct for 0, 1, or
    /// the scale fixture's 2,500 synthetic extensions with zero special-casing by count (proven at
    /// that scale by `🧰️framework/🔨️modules/🎠️kernel`'s own `extensions_extending` test and this
    /// packet's standalone verification script — see the report). The caller feeds each returned
    /// descriptor's `extension_id`/`extends` into `ActorKind::Extension { plugin, extension_id }`
    /// and `🎠️activation/🦀️.rs`'s `NativeKernelRuntime::activate` — see this method's own docstring
    /// in the report for why that final wiring step is not done HERE.
    pub fn extensions_extending_plugin(&self, plugin_id: &str) -> Vec<&InstalledExtension> {
        self.installed_extensions.values().filter(|installed| installed.manifest.extends == plugin_id).collect()
    }

    /// 🔒️ "capabilities scoped to the parent" — intersects an extension's own capability asks
    /// (`manifest.capabilities`, dotted capability-id strings) with its parent plugin's own
    /// already-effective set, so an extension actor can never end up holding a capability its
    /// host plugin does not itself hold (`📓️design-abi.md` §5's admission formula).
    pub fn extension_capabilities_scoped_to_parent<'a>(&self, extension: &'a InstalledExtension, parent_effective_capabilities: &[String]) -> Vec<&'a str> {
        extension.manifest.capabilities.iter().map(String::as_str).filter(|capability| parent_effective_capabilities.iter().any(|granted| granted == capability)).collect()
    }
}
//#endregion 🔖️ExtensionInstall
// #endregion host
