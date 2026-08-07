//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("trinity")
        .label("Trinity")
        .version("0.1.0")
        .setup(crate::register_trinity_exports)
        .register_document_app::<crate::apps::jack::TrinityJackPlayApp>(crate::apps::jack::create_trinity_jack_app())
        .register_document_app::<crate::apps::rewrite::TrinityRewritePlayApp>(crate::apps::rewrite::create_rewrite_app())
        .build()
}
