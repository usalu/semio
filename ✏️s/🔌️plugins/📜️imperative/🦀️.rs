//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for the imperative plugin's editor and viewer.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum ImperativeApps: PluginApp {
        Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::procedure::ImperativePlayApp>>),
        Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::procedure::ImperativeViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(register_exports)`
/// escape hatch; `.setup()` itself is gone (W1c) — `ImperativePlayApp::app_schema()` now answers the
/// one thing it used to survive for, registered automatically by `.editor(…)` below.
/// `.editor(…)`/`.viewer(…)` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) replace the
/// old single `.document_app(…)` call — one surface per role, both bound to the same
/// `crate::artifacts::procedure::PROCEDURE_DIALECT`. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M4, `📓️design-abi.md`
/// §5/§6) are this crate's proof-of-migration: the host activates one instance whenever a
/// `"computation.procedure"` artifact (`crate::artifacts::procedure::artifact_kind().id`) is
/// opened, this plugin's own actor runs `Isolated` (its 5 `🧩️extensions/` run `Linked` instead —
/// see each extension's own `bundle()`), and it asks the broker for document write access to
/// persist edits.
pub fn plugin() -> Result<Plugin<ImperativeApps>, semio_framework_plugin::PluginAssemblyError> {
    Plugin::<ImperativeApps>::builder("imperative")
        .label("Imperative")
        .version("0.1.0")
        .artifact(crate::artifacts::procedure::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::procedure::ImperativePlayApp>(crate::editor::procedure::create_imperative_app())
        .editor_mutation_roster::<crate::editor::procedure::ImperativePlayApp>()
        .viewer::<crate::viewer::procedure::ImperativeViewer>(crate::viewer::procedure::create_imperative_viewer())
        .viewer_mutation_roster::<crate::viewer::procedure::ImperativeViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::procedure::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist imperative graph edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️Tests
/// 🧪️ Contract §2.5 surface-testkit assertions, canonical versions (framework SDK, w0-f gap 2
/// closure) — no local stand-ins needed, unlike the `📐️cad` pilot which predated their landing.
#[cfg(test)]
mod surface_tests {
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[semio_framework_async_macros::async_test]
    async fn imperative_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::procedure::ImperativeViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn imperative_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::procedure::ImperativePlayApp, crate::viewer::procedure::ImperativeViewer>();
    }
}
//#endregion 🧪️Tests
