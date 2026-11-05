//! 🧩️ The `puzzle` plugin component: the one cdylib of `semio-hub-puzzle`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_puzzle::plugin, semio_hub_puzzle::PuzzleApps);
