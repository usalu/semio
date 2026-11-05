//! 🌍️ The `gis` plugin component: the one cdylib of `semio-hub-gis`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_gis::plugin, semio_hub_gis::GisApps);
