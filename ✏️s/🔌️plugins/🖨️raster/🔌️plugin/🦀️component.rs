//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("raster")
        .label("Raster")
        .version("0.1.0")
        .setup(crate::artifacts::raster::engine::register)
        .register_document_app::<crate::apps::raster::RasterPlayApp>(crate::apps::raster::create_raster_app())
        .build()
}
