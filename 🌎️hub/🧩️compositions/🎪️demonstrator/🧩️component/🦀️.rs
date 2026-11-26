//! 🎪️ The `demonstrator` plugin component: the one cdylib of `semio-hub-demonstrator`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_demonstrator_router::manifest::plugin, semio_hub_demonstrator_router::manifest::DemonstratorApps);
