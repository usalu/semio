//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` survives here for exactly one call — `register_app_schema()`, which
/// registers `NotePlayApp`'s own config/presence schema, an app-scope concern
/// `ArtifactDeclaration` has no field for by design (see that struct's doc).
pub fn plugin() -> Plugin {
    Plugin::builder("note")
        .label("Note")
        .version("0.1.0")
        .artifact_kind(crate::artifacts::note::artifact_kind())
        .setup(crate::apps::note::config::schema::register_app_schema)
        .artifact(crate::artifacts::note::engine::declaration())
        .register_document_app::<crate::apps::note::NotePlayApp>(crate::apps::note::create_note_app())
        .build()
}
