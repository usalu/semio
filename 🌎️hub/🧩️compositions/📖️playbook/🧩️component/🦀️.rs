//! 📖️ The `playbook` plugin component: the one cdylib of `semio-hub-playbook`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_playbook::plugin::plugin, semio_hub_playbook::PlaybookApps);
