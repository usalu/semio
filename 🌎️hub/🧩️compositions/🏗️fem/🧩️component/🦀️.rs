//! 🏗️ The `fem` plugin component: the one cdylib of `semio-hub-fem`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_fem::plugin, semio_hub_fem::FemApps);
