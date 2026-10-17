//! 📚️ Registry record: the registry with its entries, index and axes, the playground constants and the playground target. See [`super`].

use super::*;

// #region 🔖️Registry
/// 📚️ Every command of one workspace, searchable and resolvable.
#[derive(Debug, Default)]
pub struct Registry {
    pub(super) root: PathBuf,
    pub(super) entries: Vec<Entry>,
    pub(super) index: HashMap<String, usize>,
    pub(super) axes: Vec<Parameter>,
    pub(super) axes_source: String,
    pub(super) problems: Vec<Problem>,
    pub(super) definitions: Vec<String>,
    pub(super) basis: std::sync::Mutex<Option<GraphBasis>>,
}

/// 🎠️ The generated catalog the playground commands come from.
pub const PLAYGROUND_SOURCE: &str = "catalog:playgrounds";

pub(super) const RENDERERS: &[&str] = &["react", "wgpu-wasm", "wgpu-native"];

pub(super) const PLAYGROUND_PARAMETERS: &[&str] = &["renderer", "example", "user-slot", "app-role", "hub", "data", "local-only", "language", "terminology", "appearance"];

/// 🌱️ The environment variables of the shell contract the playground facts of the catalog are exported as.
pub(super) const HUB_ENV: &str = "S_HUB_URL";

pub(super) const DATA_ENV: &str = "S_DATA_DIR";

pub(super) const LOCAL_ONLY_ENV: &str = "S_LOCAL_ONLY";

pub(super) const CONFIGURATION: &str = "configuration";

/// 🏗️ The build project whose targets run a playground variant.
pub(super) const PLAYGROUND_OWNER: &str = "@semio-tech/framework-os-dev";

pub(super) const LABEL_CELLS: u16 = 72;

/// 🎡️ The build target that runs a playground variant with a renderer.
pub fn playground_target(variant: &str, renderer: &str) -> Option<String> {
    match renderer { "react" => Some(format!("dev-{variant}-react-dev")), "wgpu" | "wgpu-wasm" => Some(format!("dev-{variant}-wgpu-dev")), "wgpu-native" => Some(format!("run-{variant}-native-dev")), _ => None }
}
// #endregion 🔖️Registry
