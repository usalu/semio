//! 📗️ Norm Eurocode plugin — Eurocode basis of design, actions, geotechnical and seismic design (EN 1990, EN 1991, EN 1997, EN 1998).

#![allow(async_fn_in_trait)]

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};
use semio_plugin::PluginBuilder;

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet of the Norm Eurocode editor/viewer pairs.
    pub enum NormEurocodeApps: PluginApp {
        En1990Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1990_editor::En1990PlayApp>>),
        En1990Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1990_viewer::En1990Viewer>>),
        En1991Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1991_editor::En1991PlayApp>>),
        En1991Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1991_viewer::En1991Viewer>>),
        En1997Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1997_editor::En1997PlayApp>>),
        En1997Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1997_viewer::En1997Viewer>>),
        En1998Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1998_editor::En1998PlayApp>>),
        En1998Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1998_viewer::En1998Viewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the Norm Eurocode plugin: one artifact declaration, editor/viewer pair and activation per family,
/// split from `norm` so each component stays below the wasm function-count limit.
pub fn plugin() -> Result<Plugin<NormEurocodeApps>, PluginAssemblyError> {
    let en1990 = semio_s_artifact_norm_en1990::declaration(semio_s_artifact_norm_en1990::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1991 = semio_s_artifact_norm_en1991::declaration(semio_s_artifact_norm_en1991::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1997 = semio_s_artifact_norm_en1997::declaration(semio_s_artifact_norm_en1997::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1998 = semio_s_artifact_norm_en1998::declaration(semio_s_artifact_norm_en1998::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    PluginBuilder::<_, NormEurocodeApps>::new("norm-eurocode")
        .label(semio_plugin::LocalizedLabel::native("Norm Eurocode", "Norm Eurocode"))
        .version("0.1.0")
        .package_id("semio:norm-eurocode")
        .shell_hooks(std::sync::Arc::new(semio_s_plugin_norm_app_surface::NormShellHooks))
        .artifact(en1990)
        .artifact(en1991)
        .artifact(en1997)
        .artifact(en1998)
        .editor::<semio_s_artifact_norm_en1990_editor::En1990PlayApp>(semio_s_artifact_norm_en1990_editor::create_en1990_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1990_editor::En1990PlayApp>()
        .viewer::<semio_s_artifact_norm_en1990_viewer::En1990Viewer>(semio_s_artifact_norm_en1990_viewer::create_en1990_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1990_viewer::En1990Viewer>()
        .editor::<semio_s_artifact_norm_en1991_editor::En1991PlayApp>(semio_s_artifact_norm_en1991_editor::create_en1991_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1991_editor::En1991PlayApp>()
        .viewer::<semio_s_artifact_norm_en1991_viewer::En1991Viewer>(semio_s_artifact_norm_en1991_viewer::create_en1991_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1991_viewer::En1991Viewer>()
        .editor::<semio_s_artifact_norm_en1997_editor::En1997PlayApp>(semio_s_artifact_norm_en1997_editor::create_en1997_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1997_editor::En1997PlayApp>()
        .viewer::<semio_s_artifact_norm_en1997_viewer::En1997Viewer>(semio_s_artifact_norm_en1997_viewer::create_en1997_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1997_viewer::En1997Viewer>()
        .editor::<semio_s_artifact_norm_en1998_editor::En1998PlayApp>(semio_s_artifact_norm_en1998_editor::create_en1998_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1998_editor::En1998PlayApp>()
        .viewer::<semio_s_artifact_norm_en1998_viewer::En1998Viewer>(semio_s_artifact_norm_en1998_viewer::create_en1998_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1998_viewer::En1998Viewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1990_model::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1991_model::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1997_model::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1998_model::artifact_kind().id })
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
