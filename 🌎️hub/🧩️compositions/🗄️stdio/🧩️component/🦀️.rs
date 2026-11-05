//! 🗄️ The `stdio` plugin component: the one cdylib of `semio-hub-stdio`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_stdio::plugin, semio_hub_stdio::plugin::StdioApps);
