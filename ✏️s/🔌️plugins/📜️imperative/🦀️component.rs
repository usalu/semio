//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

fn register_exports() {
    crate::artifacts::imperative::engine::register();
    crate::apps::imperative::config::schema::register_app_schema();
}

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("imperative")
        .label("Imperative")
        .version("0.1.0")
        .setup(register_exports)
        .register_document_app::<crate::apps::imperative::ImperativePlayApp>(crate::apps::imperative::create_imperative_app())
        .build()
}
