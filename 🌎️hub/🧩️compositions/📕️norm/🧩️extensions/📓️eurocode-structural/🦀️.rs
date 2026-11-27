//! 📓️ Norm Eurocode Structural plugin — Eurocode concrete, steel and composite structures (EN 1992, EN 1993, EN 1994).

#![allow(async_fn_in_trait)]

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};
use semio_plugin::PluginBuilder;

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet of the Norm Eurocode Structural editor/viewer pairs; each app is instantiated once in its own crate.
    pub enum NormEurocodeStructuralApps: PluginApp {
        En1992Editor(semio_hub_norm_eurocode_structural_en1992_editor::En1992EditorApp),
        En1992Viewer(semio_hub_norm_eurocode_structural_en1992_viewer::En1992ViewerApp),
        En1993Editor(semio_hub_norm_eurocode_structural_en1993_editor::En1993EditorApp),
        En1993Viewer(semio_hub_norm_eurocode_structural_en1993_viewer::En1993ViewerApp),
        En1994Editor(semio_hub_norm_eurocode_structural_en1994_editor::En1994EditorApp),
        En1994Viewer(semio_hub_norm_eurocode_structural_en1994_viewer::En1994ViewerApp),
    }
}

/// 🪄️ Implements `From<instance>` for the fleet, so the builder takes each editor and viewer instance and routes it into the app crate that instantiates it.
macro_rules! apps_from_instance {
    ($($variant:ident($wrapper:ty): $instance:ty),+ $(,)?) => {
        $(
            impl From<$instance> for NormEurocodeStructuralApps {
                fn from(app: $instance) -> Self {
                    Self::$variant(<$wrapper>::from(app))
                }
            }
        )+
    };
}

apps_from_instance! {
    En1992Editor(semio_hub_norm_eurocode_structural_en1992_editor::En1992EditorApp): VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1992_editor::En1992PlayApp>>,
    En1992Viewer(semio_hub_norm_eurocode_structural_en1992_viewer::En1992ViewerApp): VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1992_viewer::En1992Viewer>>,
    En1993Editor(semio_hub_norm_eurocode_structural_en1993_editor::En1993EditorApp): VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1993_editor::En1993PlayApp>>,
    En1993Viewer(semio_hub_norm_eurocode_structural_en1993_viewer::En1993ViewerApp): VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1993_viewer::En1993Viewer>>,
    En1994Editor(semio_hub_norm_eurocode_structural_en1994_editor::En1994EditorApp): VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1994_editor::En1994PlayApp>>,
    En1994Viewer(semio_hub_norm_eurocode_structural_en1994_viewer::En1994ViewerApp): VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1994_viewer::En1994Viewer>>,
}
//#endregion 🗃️Apps

/// 🔌️ Builds the Norm Eurocode Structural plugin: one artifact declaration, editor/viewer pair and activation per family,
/// split from `norm` so each component stays below the wasm function-count limit.
pub fn plugin() -> Result<Plugin<NormEurocodeStructuralApps>, PluginAssemblyError> {
    let en1992 = semio_s_artifact_norm_en1992::declaration(semio_s_artifact_norm_en1992::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1993 = semio_s_artifact_norm_en1993::declaration(semio_s_artifact_norm_en1993::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1994 = semio_s_artifact_norm_en1994::declaration(semio_s_artifact_norm_en1994::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    PluginBuilder::<_, NormEurocodeStructuralApps>::new("norm-eurocode-structural")
        .label(semio_plugin::LocalizedLabel::native("Norm Eurocode Structural", "Norm Eurocode Tragwerk"))
        .version("0.1.0")
        .package_id("semio:norm-eurocode-structural")
        .shell_hooks(std::sync::Arc::new(semio_s_plugin_norm_app_surface::NormShellHooks))
        .artifact(en1992)
        .artifact(en1993)
        .artifact(en1994)
        .editor::<semio_s_artifact_norm_en1992_editor::En1992PlayApp>(semio_s_artifact_norm_en1992_editor::create_en1992_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1992_editor::En1992PlayApp>()
        .viewer::<semio_s_artifact_norm_en1992_viewer::En1992Viewer>(semio_s_artifact_norm_en1992_viewer::create_en1992_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1992_viewer::En1992Viewer>()
        .editor::<semio_s_artifact_norm_en1993_editor::En1993PlayApp>(semio_s_artifact_norm_en1993_editor::create_en1993_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1993_editor::En1993PlayApp>()
        .viewer::<semio_s_artifact_norm_en1993_viewer::En1993Viewer>(semio_s_artifact_norm_en1993_viewer::create_en1993_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1993_viewer::En1993Viewer>()
        .editor::<semio_s_artifact_norm_en1994_editor::En1994PlayApp>(semio_s_artifact_norm_en1994_editor::create_en1994_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1994_editor::En1994PlayApp>()
        .viewer::<semio_s_artifact_norm_en1994_viewer::En1994Viewer>(semio_s_artifact_norm_en1994_viewer::create_en1994_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1994_viewer::En1994Viewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1992_model::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1993_model::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1994_model::artifact_kind().id })
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
