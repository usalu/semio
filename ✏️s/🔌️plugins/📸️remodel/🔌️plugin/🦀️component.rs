//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("remodel")
        .label("Remodel")
        .version("0.1.0")
        .setup(crate::artifacts::remodel::engine::register)
        .register_document_app::<crate::apps::remodel::RemodelPlayApp>(crate::apps::remodel::create_remodel_app())
        .build()
}
