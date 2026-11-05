//! 🌿️ The `vcs` plugin component: the one cdylib of `semio-hub-vcs`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_vcs::plugin, semio_hub_vcs::VcsApps);
