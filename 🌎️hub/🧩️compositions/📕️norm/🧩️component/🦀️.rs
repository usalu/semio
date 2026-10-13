//! 📕️ The `norm` plugin component: the one cdylib of `semio-hub-norm`, holding only its entry point.

semio_framework_plugin::plugin_exports!(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_norm::plugin, semio_hub_norm::NormApps);
