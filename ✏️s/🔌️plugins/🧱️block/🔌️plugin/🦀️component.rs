//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("block")
        .label("Block")
        .version("0.1.0")
        .setup(crate::register_block_exports)
        .register_document_app::<crate::apps::block2d::Block2dPlayApp>(crate::apps::block2d::create_block2d_app())
        .register_document_app::<crate::apps::block3d::Block3dPlayApp>(crate::apps::block3d::create_block3d_app())
        .register_document_app::<crate::apps::block5d::Block5dPlayApp>(crate::apps::block5d::create_block5d_app())
        .build()
}
