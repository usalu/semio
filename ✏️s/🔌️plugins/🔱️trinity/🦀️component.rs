//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(register_trinity_exports)`
/// escape hatch for both artifacts; `.setup()` itself is gone (W1c) — `TrinityJackPlayApp::app_schema()`/
/// `TrinityRewritePlayApp::app_schema()` now answer the one thing it used to survive for, registered
/// automatically by each `register_document_app` call below. `.editor(…)`/`.viewer(…)` (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1/§2.4) replace the retired
/// `.document_app(…)` call for both artifacts — each dialect now registers one mutation-capable
/// editor and one read-only viewer surface.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("trinity")
        .label("Trinity")
        .version("0.1.0")
        .artifact(crate::artifacts::jack::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .artifact(crate::artifacts::rewrite::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::jack::TrinityJackPlayApp>(crate::editor::jack::create_trinity_jack_app())
        .editor_mutation_roster::<crate::editor::jack::TrinityJackPlayApp>()
        .viewer::<crate::viewer::jack::TrinityJackViewer>(crate::viewer::jack::create_trinity_jack_viewer())
        .viewer_mutation_roster::<crate::viewer::jack::TrinityJackViewer>()
        .editor::<crate::editor::rewrite::TrinityRewritePlayApp>(crate::editor::rewrite::create_rewrite_app())
        .editor_mutation_roster::<crate::editor::rewrite::TrinityRewritePlayApp>()
        .viewer::<crate::viewer::rewrite::TrinityRewriteViewer>(crate::viewer::rewrite::create_trinity_rewrite_viewer())
        .viewer_mutation_roster::<crate::viewer::rewrite::TrinityRewriteViewer>()
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn trinity_jack_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::jack::TrinityJackViewer>();
    }

    #[test]
    fn trinity_jack_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::jack::TrinityJackPlayApp, crate::viewer::jack::TrinityJackViewer>();
    }

    #[test]
    fn trinity_rewrite_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::rewrite::TrinityRewriteViewer>();
    }

    #[test]
    fn trinity_rewrite_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::rewrite::TrinityRewritePlayApp, crate::viewer::rewrite::TrinityRewriteViewer>();
    }
}
//#endregion 🧪️SurfaceTests
