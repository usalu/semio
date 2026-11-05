//! 💼️ The `stdio-office` plugin component: the one cdylib of `semio-hub-stdio-office`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_stdio_office::plugin, semio_hub_stdio_office::StdioOfficeApps);
