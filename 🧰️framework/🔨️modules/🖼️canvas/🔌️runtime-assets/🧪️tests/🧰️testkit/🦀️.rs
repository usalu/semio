//! 🧰️ Test support for every crate that paints or resolves icons: installs the repository's own runtime assets once per test
//! process, the way a host would at boot. Enabled by the `testkit` feature (and in this crate's own tests).

use super::{install_runtime_assets, RuntimeAssets, RuntimeAssetsInstallError};
use semio_framework_test_fixtures::fixture_path;
use std::path::PathBuf;
use std::sync::OnceLock;

/// 📂️ The repository root, found from this crate's manifest directory.
pub fn repository_root() -> PathBuf {
    fixture_path!("../../../../../..")
}

/// 🔌️ Reads the seven assets from the repository once per process and installs them; a process that already installed
/// them keeps them.
pub fn install_repository_assets() {
    static ASSETS: OnceLock<RuntimeAssets<'static>> = OnceLock::new();
    let assets = *ASSETS.get_or_init(|| RuntimeAssets::from_root(&repository_root()).expect("runtime assets must be readable from the repository"));
    match install_runtime_assets(assets) {
        Ok(()) | Err(RuntimeAssetsInstallError::AlreadyInstalled) => {}
        Err(failure) => panic!("{failure}"),
    }
}
