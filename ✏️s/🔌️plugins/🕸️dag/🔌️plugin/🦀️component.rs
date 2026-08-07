//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("dag")
        .label("DAG")
        .version("0.1.0")
        .setup(crate::artifacts::dag::engine::register)
        .register_document_app::<crate::apps::dag::DagPlayApp>(crate::apps::dag::create_dag_app())
        .build()
}
