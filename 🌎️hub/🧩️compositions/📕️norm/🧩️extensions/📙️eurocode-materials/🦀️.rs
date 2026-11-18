//! 📙️ Norm Eurocode Materials plugin — Eurocode material design: concrete, steel, composite, timber, masonry and aluminium (EN 1992–1996, EN 1999).

#![allow(async_fn_in_trait)]

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};
use semio_plugin::PluginBuilder;

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet of the Norm Eurocode Materials editor/viewer pairs.
    pub enum NormEurocodeMaterialsApps: PluginApp {
        En1992Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1992_editor::En1992PlayApp>>),
        En1992Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1992_viewer::En1992Viewer>>),
        En1993Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1993_editor::En1993PlayApp>>),
        En1993Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1993_viewer::En1993Viewer>>),
        En1994Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1994_editor::En1994PlayApp>>),
        En1994Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1994_viewer::En1994Viewer>>),
        En1995Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1995_editor::En1995PlayApp>>),
        En1995Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1995_viewer::En1995Viewer>>),
        En1996Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1996_editor::En1996PlayApp>>),
        En1996Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1996_viewer::En1996Viewer>>),
        En1999Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_en1999_editor::En1999PlayApp>>),
        En1999Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_en1999_viewer::En1999Viewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the Norm Eurocode Materials plugin: one artifact declaration, editor/viewer pair and activation per family,
/// split from `norm` so each component stays below the wasm function-count limit.
pub fn plugin() -> Result<Plugin<NormEurocodeMaterialsApps>, PluginAssemblyError> {
    let en1992 = semio_s_artifact_norm_en1992::declaration(semio_s_artifact_norm_en1992::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1993 = semio_s_artifact_norm_en1993::declaration(semio_s_artifact_norm_en1993::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1994 = semio_s_artifact_norm_en1994::declaration(semio_s_artifact_norm_en1994::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1995 = semio_s_artifact_norm_en1995::declaration(semio_s_artifact_norm_en1995::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1996 = semio_s_artifact_norm_en1996::declaration(semio_s_artifact_norm_en1996::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let en1999 = semio_s_artifact_norm_en1999::declaration(semio_s_artifact_norm_en1999::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    PluginBuilder::<_, NormEurocodeMaterialsApps>::new("norm-eurocode-materials")
        .label("Norm Eurocode Materials")
        .version("0.1.0")
        .package_id("semio:norm-eurocode-materials")
        .shell_hooks(std::sync::Arc::new(semio_s_plugin_norm_app_surface::NormShellHooks))
        .artifact(en1992)
        .artifact(en1993)
        .artifact(en1994)
        .artifact(en1995)
        .artifact(en1996)
        .artifact(en1999)
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
        .editor::<semio_s_artifact_norm_en1995_editor::En1995PlayApp>(semio_s_artifact_norm_en1995_editor::create_en1995_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1995_editor::En1995PlayApp>()
        .viewer::<semio_s_artifact_norm_en1995_viewer::En1995Viewer>(semio_s_artifact_norm_en1995_viewer::create_en1995_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1995_viewer::En1995Viewer>()
        .editor::<semio_s_artifact_norm_en1996_editor::En1996PlayApp>(semio_s_artifact_norm_en1996_editor::create_en1996_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1996_editor::En1996PlayApp>()
        .viewer::<semio_s_artifact_norm_en1996_viewer::En1996Viewer>(semio_s_artifact_norm_en1996_viewer::create_en1996_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1996_viewer::En1996Viewer>()
        .editor::<semio_s_artifact_norm_en1999_editor::En1999PlayApp>(semio_s_artifact_norm_en1999_editor::create_en1999_app())
        .editor_mutation_roster::<semio_s_artifact_norm_en1999_editor::En1999PlayApp>()
        .viewer::<semio_s_artifact_norm_en1999_viewer::En1999Viewer>(semio_s_artifact_norm_en1999_viewer::create_en1999_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_en1999_viewer::En1999Viewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1992_model::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1993_model::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1994_model::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1995_model::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1996_model::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_en1999_model::artifact_kind().id })
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
