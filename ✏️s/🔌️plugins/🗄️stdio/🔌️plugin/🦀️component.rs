//! 🔌️ Plugin root contract for the headless stdio library.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the stdio library plugin (no document apps).
pub fn plugin() -> Plugin {
    crate::artifacts::binary::engine::register();
    crate::artifacts::txt::engine::register();
    crate::artifacts::json::engine::register();
    Plugin::builder("stdio")
        .label("Stdio")
        .version("0.1.0")
        .library()
}
