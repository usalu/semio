//! 🖨️ The `raster` plugin component: the one cdylib of `semio-hub-raster`, holding only its entry point.

semio_framework_plugin::plugin_exports!(semio_hub_raster::plugin::raster_mounted_owner_policy(), semio_hub_raster::plugin::plugin, semio_hub_raster::plugin::RasterApps);
