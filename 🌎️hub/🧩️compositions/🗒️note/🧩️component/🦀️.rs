//! 🗒️ The `note` plugin component: the one cdylib of `semio-hub-note`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_note::plugin::plugin, semio_hub_note::NoteApps);
