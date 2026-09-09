//! 🪟️ Persisted-local configuration schema for one concrete Jack graph window.

use crate::Camera;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "trinity.jackgraphwindowcfg")]
#[dsl(layout = "lines")]
pub struct JackGraphWindowConfig {
    #[dsl(block)]
    pub camera: Option<Camera>,
    pub lod_mode: String,
}

impl Default for JackGraphWindowConfig {
    fn default() -> Self {
        Self { camera: None, lod_mode: "automatic".into() }
    }
}
