//! 🔌️ Plugin root contract for the energy plugin.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the energy plugin. `.artifact(…)` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// M1) replaces the old bare `crate::artifacts::model::engine::register()` call made before
/// `Plugin::builder(...)` was even constructed. `.setup()` is GONE (W1d): it survived here for exactly
/// one call — `register_document_codec`, registering `EnergyModelSnapshot`/`EnergyModelMutation`'s
/// pack↔dsl codec directly against `store`'s registry, because this plugin used to have zero
/// `ArtifactApp`s for `.document_codec::<A>()` to bind to. `ArtifactDeclaration::document_codec_bare::
/// <Snapshot, Mutation>(schema)` still expresses that — see `crate::artifacts::model::declaration()`.
///
/// 🎭️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: `.editor::<EnergyModelEditor>(…)`/
/// `.viewer::<EnergyModelViewer>(…)` register this plugin's first two document surfaces for
/// `s.energy.model@1/*` — energy is no longer a zero-app headless library, so the terminal builder
/// call is `.try_build()` (checked against `PluginBuilder::try_library`/`::try_build` in
/// `🔌️plugin/🏗️builder/🦀️component.rs`: `try_library` is a documentation-only alias that literally
/// calls `self.try_build()`, so both are functionally identical, but `try_build` is the semantically
/// honest choice now that this plugin carries real document apps).
pub async fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("energy")
        .label("Energy")
        .version("0.1.0")
        .artifact(crate::artifacts::model::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::model::EnergyModelEditor>(crate::editor::model::create_energy_model_editor())
        .editor_mutation_roster::<crate::editor::model::EnergyModelEditor>()
        .viewer::<crate::viewer::model::EnergyModelViewer>(crate::viewer::model::create_energy_model_viewer())
        .viewer_mutation_roster::<crate::viewer::model::EnergyModelViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::model::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist energy model edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️Tests
#[cfg(test)]
mod surface_tests {
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates, new_viewer};
    use semio_framework_plugin::ViewerApp;

    /// 🧪️ Contract §2.5 — real teeth: dispatches `EnergyModelViewCommand::default()` through the full
    /// `VcsArtifactApp<ViewerApp<EnergyModelViewer>>` runtime path and asserts the document/draft
    /// stores are byte-for-byte unchanged before/after (`semio_framework_plugin::testkit`, landed by
    /// W0-F — see `📓️w0-f-report.md` Gap 2; the pilot's own local stand-in is no longer needed here).
    #[test]
    async fn energy_model_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::model::EnergyModelViewer>();
    }

    #[test]
    async fn energy_model_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::model::EnergyModelEditor, crate::viewer::model::EnergyModelViewer>();
    }

    #[test]
    async fn new_viewer_builds_a_runnable_energy_model_viewer_app() {
        let _app: semio_framework_plugin::app::VcsArtifactApp<ViewerApp<crate::viewer::model::EnergyModelViewer>> = new_viewer::<crate::viewer::model::EnergyModelViewer>();
    }
}
//#endregion 🧪️Tests
