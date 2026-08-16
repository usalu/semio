//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `RasterPlayApp::app_schema()` now answers the one
/// thing it used to survive for, registered automatically by `register_document_app` below.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("raster")
        .label("Raster")
        .version("0.1.0")
        .artifact(crate::artifacts::raster::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .document_app::<crate::apps::raster::RasterPlayApp>(crate::apps::raster::create_raster_app())
        .try_build()
}
