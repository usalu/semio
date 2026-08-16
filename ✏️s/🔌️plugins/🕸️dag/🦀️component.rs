//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `DagPlayApp::app_schema()` now answers the one
/// thing it used to survive for, registered automatically by `register_document_app` below.
///
/// ✏️👁️ `.document_app::<X>(App)` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract
/// §2.1) is deleted, not deprecated — `s.dag.dag@1/*` now registers its two role surfaces
/// independently: `.editor::<E>(AppDefinition)` for the mutating authoring surface, `.viewer::<V>
/// (AppDefinition)` for the read-only one. Both derive their canonical id from `DAG_DIALECT` +
/// `AppRole`; neither takes a hand-written id.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("dag")
        .label("DAG")
        .version("0.1.0")
        .artifact(crate::artifacts::dag::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::dag::DagPlayApp>(crate::editor::dag::create_dag_app())
        .viewer::<crate::viewer::dag::DagViewer>(crate::viewer::dag::create_dag_viewer())
        .try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
mod surface_tests {
    //! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5's
    //! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}`
    //! (W0-F gap 2) are used directly here — no local stand-ins, unlike the pilot packet which
    //! predated their landing.
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn dag_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::dag::DagViewer>();
    }

    #[test]
    fn dag_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::dag::DagPlayApp, crate::viewer::dag::DagViewer>();
    }
}
//#endregion 🧪️SurfaceTests
