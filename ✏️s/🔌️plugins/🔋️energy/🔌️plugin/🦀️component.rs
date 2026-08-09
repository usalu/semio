//! 🔌️ Plugin root contract for the headless energy library.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the energy library plugin (no document apps).
pub fn plugin() -> Plugin {
    crate::artifacts::model::engine::register();
    Plugin::builder("energy")
        .label("Energy")
        .version("0.1.0")
        .library()
}
