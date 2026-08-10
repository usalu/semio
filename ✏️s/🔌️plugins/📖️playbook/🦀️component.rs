//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("playbook-play")
        .label("Playbook")
        .version("0.1.0")
        .setup(crate::apps::playbook::setup)
        .register_document_app::<crate::apps::playbook::PlaybookPlayApp>(crate::apps::playbook::create_playbook_play_app())
        .build()
}
