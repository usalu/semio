//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("imperative")
        .label("Imperative")
        .version("0.1.0")
        .setup(crate::artifacts::imperative::engine::register)
        .register_document_app::<crate::apps::imperative::ImperativePlayApp>(crate::apps::imperative::create_imperative_app())
        .build()
}
