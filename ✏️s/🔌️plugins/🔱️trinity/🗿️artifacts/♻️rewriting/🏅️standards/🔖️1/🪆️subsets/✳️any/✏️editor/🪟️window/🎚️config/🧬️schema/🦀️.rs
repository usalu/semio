//! 🪟️ Persisted-local configuration schema for one concrete Rewriting graph window.

use semio_s_artifact_trinity_jack::Camera;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "trinity.rewritingwindowcfg")]
#[dsl(layout = "lines")]
pub struct RewritingWindowConfig {
    #[dsl(block)]
    pub camera: Option<Camera>,
    pub lod_mode: String,
}

impl Default for RewritingWindowConfig {
    fn default() -> Self {
        Self { camera: None, lod_mode: "automatic".into() }
    }
}
