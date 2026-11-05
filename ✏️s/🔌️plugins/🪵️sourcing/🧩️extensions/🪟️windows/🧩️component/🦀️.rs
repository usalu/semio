//! 🪟️ The `sourcing-module-windows` extension component: the one cdylib of `semio-s-plugin-sourcing-windows`, holding only its entry point.

semio_plugin::extension_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_s_plugin_sourcing_windows::bundle);
