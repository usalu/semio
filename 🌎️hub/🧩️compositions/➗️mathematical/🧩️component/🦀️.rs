//! ➗️ The `mathematical` plugin component: the one cdylib of `semio-hub-mathematical`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_mathematical::plugin::plugin, semio_hub_mathematical::MathematicalApps);
