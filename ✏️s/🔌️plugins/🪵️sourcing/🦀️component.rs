//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.setup()` is gone (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) — `SourcingCurateApp::app_schema()` now answers
/// the app-scope config/presence schema call it used to carry, registered automatically by
/// `register_document_app` below. `.document_app::<X>(create_x_app())` split into `.editor()` +
/// `.viewer()` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, contract §2.1).
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("sourcing")
        .label("Sourcing")
        .version("0.1.0")
        .artifact(crate::artifacts::curate::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::sourcing::SourcingCurateApp>(crate::editor::sourcing::create_sourcing_curate_app())
        .editor_mutation_roster::<crate::editor::sourcing::SourcingCurateApp>()
        .viewer::<crate::viewer::sourcing::SourcingViewer>(crate::viewer::sourcing::create_sourcing_viewer())
        .viewer_mutation_roster::<crate::viewer::sourcing::SourcingViewer>()
        .try_build()
}

//#region 🧪️Tests
#[cfg(test)]
mod surface_tests {
    /// 👁️✏️ Editor and viewer must share the exact same `Dialect` — both surfaces address the same
    /// artifact coordinate, only the role differs (contract §2.5).
    #[test]
    fn editor_and_viewer_share_the_same_dialect() {
        semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::sourcing::SourcingCurateApp, crate::viewer::sourcing::SourcingViewer>();
    }

    /// 👁️ Structural + runtime proof the viewer can never mutate the document or draft store
    /// (contract §2.2/§2.5) — dispatches `SourcingViewCommand::default()` through the full
    /// `VcsArtifactApp<ViewerApp<SourcingViewer>>` runtime path.
    #[test]
    fn viewer_never_mutates() {
        semio_framework_plugin::testkit::assert_viewer_never_mutates::<crate::viewer::sourcing::SourcingViewer>();
    }
}
//#endregion 🧪️Tests
