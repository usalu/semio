//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(register_exports)`
/// escape hatch; `.setup()` itself is gone (W1c) — `ImperativePlayApp::app_schema()` now answers the
/// one thing it used to survive for, registered automatically by `register_document_app` below.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("imperative")
        .label("Imperative")
        .version("0.1.0")
        .artifact(crate::artifacts::imperative::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .document_app::<crate::apps::imperative::ImperativePlayApp>(crate::apps::imperative::create_imperative_app())
        .try_build()
}
