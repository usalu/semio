//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

#![allow(async_fn_in_trait)]

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};
use semio_plugin::PluginBuilder;

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the DIN 4108, DIN 16798, ISO 16757 and VDI 3805 editor/viewer pairs.
    pub enum NormApps: PluginApp {
        Din4108Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_din4108_editor::Din4108PlayApp>>),
        Din4108Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_din4108_viewer::Din4108Viewer>>),
        Din16798Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_din16798_editor::Din16798PlayApp>>),
        Din16798Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_din16798_viewer::Din16798Viewer>>),
        Iso16757Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_iso16757_editor::Iso16757PlayApp>>),
        Iso16757Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_iso16757_viewer::Iso16757Viewer>>),
        Vdi3805Editor(VcsArtifactApp<EditorApp<semio_s_artifact_norm_vdi3805_editor::Vdi3805PlayApp>>),
        Vdi3805Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_norm_vdi3805_viewer::Vdi3805Viewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b) replaces the deleted `register_norm_exports`
/// `.setup()` fan-out with four data declarations, one per norm family. `.editor(…)`/`.viewer(…)`
/// (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) replace the retired `.document_app(…)`
/// call per family with the role-split pair. Every editor publishes empty app config and presence
/// facets, then registers its family-specific Results-window config owner explicitly.
/// `.activation(…)`/`.execution(…)`/`.requests(…)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining, `📓️design-abi.md` §3/§6) are this
/// crate's migration proof: one `OnArtifactKind` event per owned norm family, read live from each
/// family's own `artifact_kind().id` (never hardcoded, same standard `🗄️stdio`'s 36-kind migration
/// set), `Isolated` execution, one `artifacts.write` ask covering all four editors.
pub fn plugin() -> Result<Plugin<NormApps>, PluginAssemblyError> {
    let din4108 = semio_s_artifact_norm_din4108::declaration(semio_s_artifact_norm_din4108::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let din16798 = semio_s_artifact_norm_din16798::declaration(semio_s_artifact_norm_din16798::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let iso16757 = semio_s_artifact_norm_iso16757::declaration(semio_s_artifact_norm_iso16757_model::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    let vdi3805 = semio_s_artifact_norm_vdi3805::declaration(semio_s_artifact_norm_vdi3805::definition().map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    PluginBuilder::<_, NormApps>::new("norm")
        .label("Norm")
        .version("0.1.0")
        .package_id("semio:norm")
        .shell_hooks(std::sync::Arc::new(semio_s_plugin_norm_app_surface::NormShellHooks))
        .artifact(din4108)
        .artifact(din16798)
        .artifact(iso16757)
        .artifact(vdi3805)
        .editor::<semio_s_artifact_norm_din4108_editor::Din4108PlayApp>(semio_s_artifact_norm_din4108_editor::create_din4108_app())
        .editor_mutation_roster::<semio_s_artifact_norm_din4108_editor::Din4108PlayApp>()
        .viewer::<semio_s_artifact_norm_din4108_viewer::Din4108Viewer>(semio_s_artifact_norm_din4108_viewer::create_din4108_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_din4108_viewer::Din4108Viewer>()
        .editor::<semio_s_artifact_norm_din16798_editor::Din16798PlayApp>(semio_s_artifact_norm_din16798_editor::create_din16798_app())
        .editor_mutation_roster::<semio_s_artifact_norm_din16798_editor::Din16798PlayApp>()
        .viewer::<semio_s_artifact_norm_din16798_viewer::Din16798Viewer>(semio_s_artifact_norm_din16798_viewer::create_din16798_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_din16798_viewer::Din16798Viewer>()
        .editor::<semio_s_artifact_norm_iso16757_editor::Iso16757PlayApp>(semio_s_artifact_norm_iso16757_editor::create_iso16757_app())
        .editor_mutation_roster::<semio_s_artifact_norm_iso16757_editor::Iso16757PlayApp>()
        .viewer::<semio_s_artifact_norm_iso16757_viewer::Iso16757Viewer>(semio_s_artifact_norm_iso16757_viewer::create_iso16757_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_iso16757_viewer::Iso16757Viewer>()
        .editor::<semio_s_artifact_norm_vdi3805_editor::Vdi3805PlayApp>(semio_s_artifact_norm_vdi3805_editor::create_vdi3805_app())
        .editor_mutation_roster::<semio_s_artifact_norm_vdi3805_editor::Vdi3805PlayApp>()
        .viewer::<semio_s_artifact_norm_vdi3805_viewer::Vdi3805Viewer>(semio_s_artifact_norm_vdi3805_viewer::create_vdi3805_viewer())
        .viewer_mutation_roster::<semio_s_artifact_norm_vdi3805_viewer::Vdi3805Viewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_din4108_model::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_din16798_model::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_iso16757_model::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_norm_vdi3805_model::artifact_kind().id })
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

