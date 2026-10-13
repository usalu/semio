//! 🌀️ The `playbook-module-procedural` extension component: the one cdylib of `semio-s-plugin-playbook-procedural`, holding only its entry point.

semio_framework_plugin::extension_exports!(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_s_plugin_playbook_procedural::module_extension_bundle, semio_s_plugin_playbook_procedural::module_plugin_bundle, semio_s_plugin_playbook_procedural::ProceduralModuleApps);
