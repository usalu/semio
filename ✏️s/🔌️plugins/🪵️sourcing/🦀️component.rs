//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.setup()` is gone (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) — `SourcingCurateApp::app_schema()` now answers
/// the app-scope config/presence schema call it used to carry, registered automatically by
/// `register_document_app` below.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("sourcing")
        .label("Sourcing")
        .version("0.1.0")
        .artifact(crate::artifacts::curate::declaration())
        .document_app::<crate::apps::curate::SourcingCurateApp>(crate::apps::curate::create_sourcing_curate_app())
        .try_build()
}
