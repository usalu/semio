//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `PlaybookPlayApp::app_schema()` now answers the
/// one thing it used to survive for, registered automatically by `register_document_app` below.
/// `.editor()`/`.viewer()` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.4)
/// replace the retired `.document_app()` — one mutation-capable surface, one read-only surface, both
/// over the same `PLAYBOOK_DIALECT` coordinate.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("playbook-play")
        .label("Playbook")
        .version("0.1.0")
        .artifact(crate::artifacts::playbook::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::playbook::PlaybookPlayApp>(crate::editor::playbook::create_playbook_play_app())
        .viewer::<crate::viewer::playbook::PlaybookViewer>(crate::viewer::playbook::create_playbook_viewer())
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
