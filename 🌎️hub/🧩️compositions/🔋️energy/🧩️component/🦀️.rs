//! 🔋️ The `energy` plugin component: the one cdylib of `semio-hub-energy`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_energy::plugin::plugin, semio_hub_energy::plugin::EnergyApps);
