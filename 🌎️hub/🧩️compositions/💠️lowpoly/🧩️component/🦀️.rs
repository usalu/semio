//! 💠️ The `lowpoly` plugin component: the one cdylib of `semio-hub-lowpoly`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_lowpoly::plugin::plugin, semio_hub_lowpoly::plugin::LowpolyApps);
