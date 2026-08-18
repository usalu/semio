//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `DrawPlayApp::app_schema()` now answers the one
/// thing it used to survive for, registered automatically by `register_document_app` below.
/// ✏️👁️ `.document_app(…)` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1)
/// is replaced by two role-split registrations: `.editor::<E>(…)` (mutation-capable) and
/// `.viewer::<V>(…)` (read-only) for the same `s.draw.draw@1/*` dialect.
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
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("draw")
        .label("Draw")
        .version("0.1.0")
        .artifact(crate::artifacts::draw::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::draw::DrawPlayApp>(crate::editor::draw::create_draw_app())
        .editor_mutation_roster::<crate::editor::draw::DrawPlayApp>()
        .viewer::<crate::viewer::draw::DrawViewer>(crate::viewer::draw::create_draw_viewer())
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
    #[test]
    fn draw_viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<crate::viewer::draw::DrawViewer>();
    }

    #[test]
    fn draw_editor_and_viewer_share_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::draw::DrawPlayApp, crate::viewer::draw::DrawViewer>();
    }
}
//#endregion 🧪️SurfaceTests
