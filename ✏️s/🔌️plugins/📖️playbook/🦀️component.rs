//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `PlaybookPlayApp::app_schema()` now answers the
/// one thing it used to survive for, registered automatically by `register_document_app` below.
/// `.editor()`/`.viewer()` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.4)
/// replace the retired `.document_app()` — one mutation-capable surface, one read-only surface, both
/// over the same `PLAYBOOK_DIALECT` coordinate. `.activation(…)`/`.execution(…)`/`.requests(…)`
/// (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M4, `📓️design-abi.md` §5/§6) are this
/// crate's proof-of-migration: the host activates one instance whenever a `"text.playbook"`
/// artifact (`crate::artifacts::playbook::artifact_kind().id`) is opened, this plugin's own actor
/// runs `Isolated` (its one `🧩️extensions/🌀️procedural` runs `Declarative` instead — see that
/// extension's own `bundle()`), and it asks the broker for document write access to persist edits.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("playbook-play")
        .label("Playbook")
        .version("0.1.0")
        .artifact(crate::artifacts::playbook::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::playbook::PlaybookPlayApp>(crate::editor::playbook::create_playbook_play_app())
        .editor_mutation_roster::<crate::editor::playbook::PlaybookPlayApp>()
        .viewer::<crate::viewer::playbook::PlaybookViewer>(crate::viewer::playbook::create_playbook_viewer())
        .viewer_mutation_roster::<crate::viewer::playbook::PlaybookViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::playbook::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist playbook step-list edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 — w0-f (SDK gap
    //! closure lane) landed the canonical `testkit::{assert_viewer_never_mutates,
    //! assert_editor_and_viewer_share_dialect, new_viewer}` in `semio_framework_plugin`; used directly
    //! here rather than a local stand-in (the pilot cad packet's `📓️w2-cad-report.md` had to write one
    //! before this landed — see that report's "SDK gaps found" §2 for the closed gap).
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn playbook_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::playbook::PlaybookViewer>();
    }

    #[test]
    fn playbook_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::playbook::PlaybookPlayApp, crate::viewer::playbook::PlaybookViewer>();
    }
}
//#endregion 🧪️SurfaceTests
