//! 📒️ Norm Energy plugin — building energy performance assessment (DIN V 18599), composed with semio members.

#![allow(async_fn_in_trait)]

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};
use semio_plugin::PluginBuilder;

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet of the Norm Energy editor/viewer pairs.
    pub enum NormEnergyApps: PluginApp {
        Din18599Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_din18599_editor::Din18599PlayApp>, <semio_s_artifact_norm_din18599_editor::Din18599PlayApp as semio_plugin::ArtifactEditor>::Members>),
        Din18599Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_din18599_viewer::Din18599Viewer>, <semio_s_artifact_norm_din18599_viewer::Din18599Viewer as semio_plugin::ArtifactViewer>::Members>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the Norm Energy plugin: one artifact declaration, editor/viewer pair and activation per family,
/// split from `norm` so each component stays below the wasm function-count limit.
pub fn plugin() -> Result<Plugin<NormEnergyApps>, PluginAssemblyError> {
    let din18599 = semio_s_artifact_norm_din18599::declaration(semio_s_artifact_norm_din18599::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    PluginBuilder::<_, NormEnergyApps>::new("norm-energy")
        .label(semio_plugin::LocalizedLabel::native("Norm Energy", "Norm Energie"))
        .version("0.1.0")
        .package_id("semio:norm-energy")
        .shell_hooks(std::sync::Arc::new(semio_s_plugin_norm_app_surface::NormShellHooks))
        .artifact(din18599)
        .editor::<semio_s_artifact_norm_din18599_editor::Din18599PlayApp>(semio_s_artifact_norm_din18599_editor::create_din18599_app())
        .editor_mutation_roster::<semio_s_artifact_norm_din18599_editor::Din18599PlayApp>()
        .viewer::<semio_s_artifact_norm_din18599_viewer::Din18599Viewer>(semio_s_artifact_norm_din18599_viewer::create_din18599_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_din18599_viewer::Din18599Viewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_din18599_model::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist norm family edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
#[cfg(test)]
#[path = "🧪️tests/🎯️action-args/🦀️.rs"]
mod action_args_tests;
//#endregion 🧪️SurfaceTests
