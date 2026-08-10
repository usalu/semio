//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("sourcing")
        .label("Sourcing")
        .version("0.1.0")
        .setup(crate::artifacts::curate::engine::register)
        .register_document_app::<crate::apps::curate::SourcingCurateApp>(crate::apps::curate::create_sourcing_curate_app())
        .build()
}
