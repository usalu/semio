//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(register_trinity_exports)`
/// escape hatch for both artifacts; `.setup()` itself is gone (W1c) — `TrinityJackPlayApp::app_schema()`/
/// `TrinityRewritePlayApp::app_schema()` now answer the one thing it used to survive for, registered
/// automatically by each `register_document_app` call below.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("trinity")
        .label("Trinity")
        .version("0.1.0")
        .artifact(crate::artifacts::jack::declaration())
        .artifact(crate::artifacts::rewrite::declaration())
        .document_app::<crate::apps::jack::TrinityJackPlayApp>(crate::apps::jack::create_trinity_jack_app())
        .document_app::<crate::apps::rewrite::TrinityRewritePlayApp>(crate::apps::rewrite::create_rewrite_app())
        .try_build()
}
