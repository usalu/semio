//! 🪟️ Persisted-local configuration schema for one concrete Rewriting graph window.

use semio_s_artifact_trinity_jack::Camera;

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "trinity.rewritingwindowcfg")]
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
