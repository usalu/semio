//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("puzzle")
        .label("Puzzle")
        .version("0.1.0")
        .setup(crate::artifacts::puzzle2d::engine::register)
        .register_document_app::<crate::apps::puzzle2d::Puzzle2dPlayApp>(crate::apps::puzzle2d::create_puzzle2d_app())
        .register_document_app::<crate::apps::puzzle3d::Puzzle3dPlayApp>(crate::apps::puzzle3d::create_puzzle3d_app())
        .register_document_app::<crate::apps::puzzle5d::Puzzle5dPlayApp>(crate::apps::puzzle5d::create_puzzle5d_app())
        .build()
}
