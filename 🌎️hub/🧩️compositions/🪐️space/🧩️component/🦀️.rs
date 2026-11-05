//! 🪐️ The `space` plugin component: the one cdylib of `semio-hub-space`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_space::plugin, semio_hub_space::SpaceApps);
