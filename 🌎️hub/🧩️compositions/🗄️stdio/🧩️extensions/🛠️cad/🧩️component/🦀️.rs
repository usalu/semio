//! 🛠️ The `stdio-cad` plugin component: the one cdylib of `semio-hub-stdio-cad`, holding only its entry point.

semio_framework_plugin::plugin_exports!(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_stdio_cad::plugin, semio_hub_stdio_cad::StdioCadApps);
