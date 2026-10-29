//! 🔌️ Plugin root contract for the energy plugin.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{EditorApp, ExecutionMode, NeedsLabel, Plugin, PluginApp, PluginAssemblyError, PluginBuilder, ViewerApp};
use semio_framework_plugin_instance_construction::OriginalVcsApp;

//#region 🗃️Apps
// 🗃️ Closed runtime app fleet for the energy plugin's model surfaces. Both surfaces run over stdio's
// `SemioMembers` roster because the model's composed `structure`/`zones` children are
// `s.stdio.semio@v1/{value,table}` members (see `energy_genesis_child_pack`).
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum EnergyApps: PluginApp {
        ModelEditor(OriginalVcsApp<EditorApp<semio_s_artifact_energy_model_editor::EnergyModelEditor>, semio_s_artifact_stdio_semio::SemioMembers>),
        ModelViewer(OriginalVcsApp<ViewerApp<semio_s_artifact_energy_model_viewer::EnergyModelViewer>, semio_s_artifact_stdio_semio::SemioMembers>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the energy plugin. `.artifact(…)` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// M1) replaces the old bare `crate::engine::register()` call made before
/// `Plugin::builder(...)` was even constructed. `.setup()` is GONE (W1d): it survived here for exactly
/// one call — `register_document_codec`, registering `EnergyModelSnapshot`/`EnergyModelMutation`'s
/// pack↔dsl codec directly against `store`'s registry, because this plugin used to have zero
/// `ArtifactApp`s for `.document_codec::<A>()` to bind to. `ArtifactDeclaration::document_codec_bare::
/// <Snapshot, Mutation>(schema)` still expresses that — see `semio_s_artifact_energy_model::declaration()`.
///
/// 📚️ Ticket 26/09/06/ENERGY-PLUGIN-END-TO-END: `.editor` stamps `EnergyModelEditor::examples()`
/// onto `PluginManifest.examples`. The navbar dropdown is `examples_for_app` over that manifest, so
/// the viewer of the same dialect offers the same rows.
///
/// 🎭️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: `.editor::<EnergyModelEditor>(…)`/
/// `.viewer::<EnergyModelViewer>(…)` register this plugin's first two document surfaces for
/// `s.energy.model@1/*` — energy is no longer a zero-app headless library, so the terminal builder
/// call is `.try_build()` (checked against `PluginBuilder::try_library`/`::try_build` in
/// `🔌️plugin/🏗️builder/🦀️.rs`: `try_library` is a documentation-only alias that literally
/// calls `self.try_build()`, so both are functionally identical, but `try_build` is the semantically
/// honest choice now that this plugin carries real document apps).
pub fn plugin() -> Result<Plugin<EnergyApps>, PluginAssemblyError> {
    PluginBuilder::<NeedsLabel, EnergyApps>::new("energy")
        .label("Energy")
        .version("0.1.0")
        .package_id("semio:energy")
        .artifact(semio_s_artifact_energy_model::declaration().map_err(PluginAssemblyError::definition)?)
        .native_editor::<semio_s_artifact_energy_model_editor::EnergyModelEditor,semio_s_artifact_energy_model_editor::EnergyModelNativeRegistration>(semio_s_artifact_energy_model_editor::create_energy_model_editor())?
        .editor_mutation_roster::<semio_s_artifact_energy_model_editor::EnergyModelEditor>()
        .native_viewer::<semio_s_artifact_energy_model_viewer::EnergyModelViewer,semio_s_artifact_energy_model_viewer::EnergyModelViewerNativeRegistration>(semio_s_artifact_energy_model_viewer::create_energy_model_viewer())?
        .viewer_mutation_roster::<semio_s_artifact_energy_model_viewer::EnergyModelViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_energy_model_snapshot::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist energy model edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️Tests
