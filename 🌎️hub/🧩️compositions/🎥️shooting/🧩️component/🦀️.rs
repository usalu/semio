//! 🎥️ The `shooting` plugin component: the one cdylib of `semio-hub-shooting`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_shooting::plugin::plugin, semio_hub_shooting::plugin::ShootingApps);
