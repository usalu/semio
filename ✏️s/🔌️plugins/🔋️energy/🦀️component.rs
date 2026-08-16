//! 🔌️ Plugin root contract for the headless energy library.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the energy library plugin (no document apps). `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old bare
/// `crate::artifacts::model::engine::register()` call made before `Plugin::builder(...)` was even
/// constructed. `.setup()` is GONE (W1d): it survived here for exactly one call —
/// `register_document_codec`, registering `EnergyModelSnapshot`/`EnergyModelMutation`'s pack↔dsl codec
/// directly against `store`'s registry, because this plugin has zero `ArtifactApp`s for
/// `.document_codec::<A>()` to bind to. `ArtifactDeclaration::document_codec_bare::<Snapshot,
/// Mutation>(schema)` now expresses exactly that — see `crate::artifacts::model::declaration()`.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("energy").label("Energy").version("0.1.0").artifact(crate::artifacts::model::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?).try_library()
}
