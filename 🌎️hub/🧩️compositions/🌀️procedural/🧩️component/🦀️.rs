//! 🌀️ The `procedural` plugin component: the one cdylib of `semio-hub-procedural`, holding only its entry point.

semio_framework_plugin::plugin_exports!(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_procedural::plugin, semio_hub_procedural::ProceduralApps);
