//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for the draw editor and viewer.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum DrawApps: PluginApp {
        Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::draw::DrawPlayApp>>),
        Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::draw::DrawViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. Atomic cutover (ticket
/// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM): `.declare_artifact(…)` (new declaration
/// tree) replaces `.artifact(declaration())`/`.editor::<>()`/`.viewer::<>()` outright — the old
/// channel is NOT kept alongside it (a second parallel registration channel is the compatibility
/// layer this ticket forbids). `.editor_mutation_roster()`/`.viewer_mutation_roster()` stay: they
/// are an orthogonal, still-supported opt-in (`contributor.list-artifact-mutations`) the new
/// declaration tree's `SurfaceDeclaration.mutation_roster` does not yet wire live (`📓️w1-c-report.md`
/// openQuestion 3) — not a second registration of the artifact/schema/io itself.
/// `.activation(…)`/`.execution(…)`/`.requests(…)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M1, `📓️design-abi.md` §3/§6): the host
/// activates one instance whenever a `"2d.drawing"` artifact
/// (`crate::artifacts::draw::artifact_kind().id`) is opened, this plugin's actor runs `Isolated`
/// (no cross-plugin extension attachment; the canvas gesture FSM's own `loop`s are microstep- and
/// mailbox-bounded within one turn, not a self-tick/`pending_effects` poll — the SDK default
/// holds), and it asks the broker for document write access because `DrawPlayApp` persists edits
/// back to the open document. No quota declared: draw's ~14 `Effect` call sites
/// (`LoadDocument`/`SetActiveUtility`/`ReplayShellCommand`) are per-turn UI/document effects with
/// no evidence of long-running computation, large held buffers, or high-frequency timers.
pub async fn plugin() -> Result<Plugin<DrawApps>, semio_framework_plugin::PluginAssemblyError> {
    Plugin::<DrawApps>::builder("draw")
        .label("Draw")
        .version("0.1.0")
        .declare_artifact(crate::artifacts::draw::artifact())
        .editor_mutation_roster::<crate::editor::draw::DrawPlayApp>()
        .viewer_mutation_roster::<crate::viewer::draw::DrawViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::draw::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist draw edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
/// 🧪️ Contract §2.5 surface guarantees: a viewer never mutates the document (type + runtime proof)
/// and both surfaces share one dialect coordinate.
#[cfg(test)]
mod surface_tests {
    #[semio_framework_async_macros::async_test]
    async fn draw_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<crate::viewer::draw::DrawViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn draw_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::draw::DrawPlayApp, crate::viewer::draw::DrawViewer>();
    }
}
//#endregion 🧪️SurfaceTests
