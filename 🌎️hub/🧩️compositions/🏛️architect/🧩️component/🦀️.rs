//! 🏛️ The `architect` plugin component: the one cdylib of `semio-hub-architect`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_architect::plugin::plugin, semio_hub_architect::plugin::ArchitectApps);
