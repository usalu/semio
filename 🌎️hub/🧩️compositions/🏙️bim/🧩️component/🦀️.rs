//! 🏙️ The `bim` plugin component: the one cdylib of `semio-hub-bim`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_bim::plugin::plugin, semio_hub_bim::plugin::BimApps);
