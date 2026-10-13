//! 💡️ The `reasoning` plugin component: the one cdylib of `semio-hub-reasoning`, holding only its entry point.

semio_framework_plugin::plugin_exports!(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_reasoning::plugin::plugin, semio_hub_reasoning::plugin::ReasoningApps);
