//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Plugin {
    Plugin::builder("note")
        .label("Note")
        .version("0.1.0")
        .artifact_kind(crate::artifacts::note::artifact_kind())
        .setup(crate::artifacts::note::engine::register)
        .register_document_app::<crate::apps::note::NotePlayApp>(crate::apps::note::create_note_app())
        .build()
}
