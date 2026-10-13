//! 📐️ The `cad` plugin component: the one cdylib of `semio-hub-cad`, holding only its entry point.

semio_framework_plugin::plugin_exports!(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_cad::plugin::plugin, semio_hub_cad::plugin::CadApps);
