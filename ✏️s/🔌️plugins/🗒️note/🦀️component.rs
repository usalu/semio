//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `NotePlayApp::app_schema()` now answers the one
/// thing it used to survive for, registered automatically by `register_document_app` below.
/// `.editor(…)`/`.viewer(…)` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract
/// §2.1/§2.4) replace the retired `.document_app(…)` call — the `s.note.note@1/*` dialect now
/// registers one mutation-capable editor and one read-only viewer surface.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("note")
        .label("Note")
        .version("0.1.0")
        .artifact_kind(crate::artifacts::note::artifact_kind())
        .artifact(crate::artifacts::note::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::note::NotePlayApp>(crate::editor::note::create_note_app())
        .editor_mutation_roster::<crate::editor::note::NotePlayApp>()
        .viewer::<crate::viewer::note::NoteViewer>(crate::viewer::note::create_note_viewer())
        .viewer_mutation_roster::<crate::viewer::note::NoteViewer>()
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn note_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::note::NoteViewer>();
    }

    #[test]
    fn note_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::note::NotePlayApp, crate::viewer::note::NoteViewer>();
    }
}
//#endregion 🧪️SurfaceTests
