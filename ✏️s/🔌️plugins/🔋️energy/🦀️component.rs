//! 🔌️ Plugin root contract for the headless energy library.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the energy library plugin (no document apps). `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old bare
/// `crate::artifacts::model::engine::register()` call made before `Plugin::builder(...)` was even
/// constructed. `.setup()` survives here for exactly one call — `register_document_codec`, which
/// registers `EnergyModelSnapshot`/`EnergyModelMutation`'s pack↔dsl codec directly against `store`'s
/// registry. This is NOT app-scope config/presence schema (the one documented `.setup()` exception —
/// see `🗒️note`'s root): it is `register_document_codec_for_app`'s non-app sibling,
/// `store::register_document_codec`, needed because this plugin has zero `ArtifactApp`s to bind
/// `.document_codec::<A>()` to. `ArtifactDeclaration` has no field that can express a document codec
/// without an owning app — a real mechanism gap, reported prominently rather than silently ported.
pub fn plugin() -> Plugin {
    Plugin::builder("energy")
        .label("Energy")
        .version("0.1.0")
        .setup(crate::artifacts::model::standards::v1::engine::register_document_codec)
        .artifact(crate::artifacts::model::declaration())
        .library()
}
