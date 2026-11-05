//! 🀄️ The `wfc` plugin component: the one cdylib of `semio-hub-wfc`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_wfc::plugin, semio_hub_wfc::WfcApps);
