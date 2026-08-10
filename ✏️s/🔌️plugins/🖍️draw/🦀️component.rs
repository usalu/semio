//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("draw")
        .label("Draw")
        .version("0.1.0")
        .setup(crate::artifacts::draw::engine::register)
        .register_document_app::<crate::apps::draw::DrawPlayApp>(crate::apps::draw::create_draw_app())
        .build()
}
