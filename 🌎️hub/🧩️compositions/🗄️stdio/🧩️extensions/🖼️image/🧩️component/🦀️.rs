//! 🖼️ The `stdio-image` plugin component: the one cdylib of `semio-hub-stdio-image`, holding only its entry point.

semio_framework_plugin::plugin_exports!(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_stdio_image::plugin, semio_hub_stdio_image::StdioImageApps);
