//! 🖍️ The `draw` plugin component: the one cdylib of `semio-hub-draw`, holding only its entry point.

semio_plugin::plugin_exports!(semio_hub_draw::DRAW_MOUNTED_OWNER_POLICY, semio_hub_draw::plugin::plugin, semio_hub_draw::DrawApps);
