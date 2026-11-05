//! 🔺️ The `stdio-mesh` plugin component: the one cdylib of `semio-hub-stdio-mesh`, holding only its entry point.

semio_plugin::plugin_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_stdio_mesh::plugin, semio_hub_stdio_mesh::StdioMeshApps);
