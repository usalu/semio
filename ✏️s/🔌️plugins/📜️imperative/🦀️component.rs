//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(register_exports)`
/// escape hatch; `.setup()` itself is gone (W1c) — `ImperativePlayApp::app_schema()` now answers the
/// one thing it used to survive for, registered automatically by `.editor(…)` below.
/// `.editor(…)`/`.viewer(…)` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) replace the
/// old single `.document_app(…)` call — one surface per role, both bound to the same
/// `crate::artifacts::imperative::IMPERATIVE_DIALECT`.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("imperative")
        .label("Imperative")
        .version("0.1.0")
        .artifact(crate::artifacts::imperative::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::imperative::ImperativePlayApp>(crate::editor::imperative::create_imperative_app())
        .viewer::<crate::viewer::imperative::ImperativeViewer>(crate::viewer::imperative::create_imperative_viewer())
        .try_build()
}

//#region 🧪️Tests
/// 🧪️ Contract §2.5 surface-testkit assertions, canonical versions (framework SDK, w0-f gap 2
/// closure) — no local stand-ins needed, unlike the `📐️cad` pilot which predated their landing.
#[cfg(test)]
mod surface_tests {
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn imperative_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::imperative::ImperativeViewer>();
    }

    #[test]
    fn imperative_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::imperative::ImperativePlayApp, crate::viewer::imperative::ImperativeViewer>();
    }
}
//#endregion 🧪️Tests
