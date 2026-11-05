//! 🪵️ The `sourcing` plugin component: the one cdylib of `semio-hub-sourcing`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_sourcing::plugin::plugin, semio_hub_sourcing::SourcingApps);
