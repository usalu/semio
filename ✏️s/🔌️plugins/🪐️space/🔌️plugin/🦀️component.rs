//! 🔌️ Plugin root contract for space (multi-app host).

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the S Studio plugin (home + space apps).
pub fn plugin() -> Plugin {
    crate::register_s_exports();
    Plugin::builder("s")
        .label("S Studio")
        .version("0.1.0")
        .local_backbone_storage()
        .register_document_app::<crate::apps::home::HomeApp>(crate::apps::home::create_home_app())
        .register_document_app::<crate::apps::space::SpaceApp>(crate::apps::space::create_space_app())
        .build()
}
