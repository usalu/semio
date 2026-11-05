//! 📜️ The `imperative` plugin component: the one cdylib of `semio-hub-imperative`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_imperative::plugin::plugin, semio_hub_imperative::plugin::ImperativeApps);
