//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `RemodelPlayApp::app_schema()` now answers the
/// one thing it used to survive for, registered automatically below. Ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the former single `.document_app(...)` call
/// split into an independent `.editor()` + `.viewer()` pair, one surface per role.
pub async fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("remodel")
        .label("Remodel")
        .version("0.1.0")
        .artifact(crate::artifacts::remodel::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::remodel::RemodelPlayApp>(crate::editor::remodel::create_remodel_app())
        .editor_mutation_roster::<crate::editor::remodel::RemodelPlayApp>()
        .viewer::<crate::viewer::remodel::RemodelViewer>(crate::viewer::remodel::create_remodel_viewer())
        .viewer_mutation_roster::<crate::viewer::remodel::RemodelViewer>()
        // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M5 — `.activation(…)`/`.execution(…)`/
        // `.requests(…)` (`📓️design-abi.md` §3/§6). See `📓️terra-M5-report.md` for why
        // `run_whole_pipeline`'s synchronous structure-from-motion loop (`🎮️commands/🚀️run-reconstruction`,
        // `🚀️run-stage`, `🚀️retry-stage`) is this packet's genuine "SfM" long-running-compute finding,
        // and why its `Effect::SpawnJob` conversion is blocked upstream, not by anything in this crate.
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::remodel::artifact_kind().id })
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

    #[test]
    async fn remodel_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::remodel::RemodelViewer>();
    }

    #[test]
    async fn remodel_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::remodel::RemodelPlayApp, crate::viewer::remodel::RemodelViewer>();
    }
}
//#endregion 🧪️Tests
