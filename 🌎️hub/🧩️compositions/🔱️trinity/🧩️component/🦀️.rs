//! 🔱️ The `trinity` plugin component: the one cdylib of `semio-hub-trinity`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_trinity::plugin, semio_hub_trinity::TrinityApps);
