//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the remodel editor and viewer surfaces.
    pub enum RemodelApps: PluginApp {
        Editor(VcsArtifactApp<EditorApp<crate::editor::remodeling::RemodelingPlayApp>>),
        Viewer(VcsArtifactApp<ViewerApp<crate::viewer::remodeling::RemodelingViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.declare_artifact(…)` (ticket
/// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §2) replaces `.artifact(declaration())`
/// + `.editor::<>()` + `.viewer::<>()` outright — a second parallel registration channel is the
/// compatibility layer this ticket forbids, so the old calls are gone rather than kept alongside.
/// That cutover is also what finally reaches ShellHost's example picker: `manifest.apps[].examples`
/// is fed from `SubsetDeclaration.examples`, a field the bare `AppDefinition` the old `.editor::<>()`
/// took never had. `.editor_mutation_roster()`/`.viewer_mutation_roster()` stay — an orthogonal
/// `contributor.list-artifact-mutations` opt-in the declaration tree's `SurfaceDeclaration.mutation_roster`
/// does not yet wire live, not a second registration of the artifact/schema/io itself.
pub fn plugin() -> Result<Plugin<RemodelApps>, PluginAssemblyError> {
    Plugin::<RemodelApps>::builder("remodel")
        .label("Remodel")
        .version("0.1.0")
        .package_id("semio:remodel")
        .declare_artifact(crate::artifacts::remodeling::artifact())
        .editor_mutation_roster::<crate::editor::remodeling::RemodelingPlayApp>()
        .viewer_mutation_roster::<crate::viewer::remodeling::RemodelingViewer>()
        // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M5 — `.activation(…)`/`.execution(…)`/
        // `.requests(…)` (`📓️design-abi.md` §3/§6). See `📓️terra-M5-report.md` for why
        // `run_whole_pipeline`'s synchronous structure-from-motion loop (`🎮️commands/🏗️run-reconstruction`,
        // `▶️run-stage`, `🔁️retry-stage`) is this packet's genuine "SfM" long-running-compute finding,
        // and why its `Effect::SpawnJob` conversion is blocked upstream, not by anything in this crate.
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::remodeling::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("documents.write".into()),
            scope: "plugin".into(),
            reason: "persist reconstruction results (job status, sparse cloud, camera trajectory, mesh, qc, geo products) to the open document".into(),
            optional: false,
        })
        .requests(CapabilityRequest {
            id: CapabilityId("ui.dialog".into()),
            scope: "plugin".into(),
            reason: "the import-frames command opens a file-open dialog (Effect::RequestFileOpen) and import-video opens a media-frame-picker dialog (Effect::RequestMediaFrames)".into(),
            optional: false,
        })
        .try_build()
}

//#region 🧪️Tests
#[cfg(test)]
mod surface_tests {
    //! 🧪️ Contract §2.5's two cross-surface guarantees (`assert_viewer_never_mutates`,
    //! `assert_editor_and_viewer_share_dialect`), landed for real in `semio_framework_plugin::testkit`
    //! per `📓️w0-f-report.md` gap 2 — used directly, no local stand-ins.
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[semio_framework_async_macros::async_test]
    async fn remodeling_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::remodeling::RemodelingViewer>().await;
    }

    #[semio_framework_async_macros::async_test]
    async fn remodeling_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::remodeling::RemodelingPlayApp, crate::viewer::remodeling::RemodelingViewer>().await;
    }
}
//#endregion 🧪️Tests
