//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `DrawPlayApp::app_schema()` now answers the one
/// thing it used to survive for, registered automatically by `register_document_app` below.
/// ✏️👁️ `.document_app(…)` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1)
/// is replaced by two role-split registrations: `.editor::<E>(…)` (mutation-capable) and
/// `.viewer::<V>(…)` (read-only) for the same `s.draw.draw@1/*` dialect.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("draw")
        .label("Draw")
        .version("0.1.0")
        .artifact(crate::artifacts::draw::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::draw::DrawPlayApp>(crate::editor::draw::create_draw_app())
        .viewer::<crate::viewer::draw::DrawViewer>(crate::viewer::draw::create_draw_viewer())
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
