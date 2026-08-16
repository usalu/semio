//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `RasterPlayApp::app_schema()` now answers the one
/// thing it used to survive for, registered automatically by `.editor(...)` below. Ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: `.document_app(...)` (single mutation-capable
/// surface) split into `.editor(...)` + `.viewer(...)` (contract §2.4) — the same dialect, two roles.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("raster")
        .label("Raster")
        .version("0.1.0")
        .artifact(crate::artifacts::raster::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::raster::RasterPlayApp>(crate::editor::raster::create_raster_app())
        .viewer::<crate::viewer::raster::RasterViewer>(crate::viewer::raster::create_raster_viewer())
        .try_build()
}

//#region 🧪️Tests
#[cfg(test)]
mod surface_tests {
    //! 🧪️ Contract §2.5 surface laws, now the real framework functions (`📓️w0-f-report.md` Gap 2) —
    //! no local stand-ins.
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn raster_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::raster::RasterViewer>();
    }

    #[test]
    fn raster_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::raster::RasterPlayApp, crate::viewer::raster::RasterViewer>();
    }
}
//#endregion 🧪️Tests
