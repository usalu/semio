//! 🏭️ The `process` plugin component: the one cdylib of `semio-hub-process`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_process::plugin::plugin, semio_hub_process::plugin::ProcessApps);
