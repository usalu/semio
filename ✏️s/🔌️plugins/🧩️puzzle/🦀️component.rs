//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old umbrella
/// `.setup(puzzle2d::engine::register)` escape hatch — one declaration per artifact (puzzle2d,
/// puzzle3d, puzzle5d), each built by its own artifact engine. `.setup()` survives here for two
/// calls neither has an `ArtifactDeclaration` field for by design: `register_app_schemas()` — all
/// three play apps' own config/presence schema, an app-scope concern (see that struct's doc) — and
/// `register_media_io`/`register_mesh_io` — the OS media-host export/import bridges, a wholly
/// separate 14-function family (`register_2d_export_handlers`/`register_mesh_exporter`/…) from the
/// nine §6 registrars `ArtifactDeclaration` covers. Both are named loudly rather than silently kept;
/// see `📓️w1b-semio-s-plugin-puzzle-report.md`.
pub fn plugin() -> Plugin {
    Plugin::builder("puzzle")
        .label("Puzzle")
        .version("0.1.0")
        .setup(setup)
        .artifact(crate::artifacts::puzzle2d::declaration())
        .artifact(crate::artifacts::puzzle3d::declaration())
        .artifact(crate::artifacts::puzzle5d::declaration())
        .register_document_app::<crate::apps::puzzle2d::Puzzle2dPlayApp>(crate::apps::puzzle2d::create_puzzle2d_app())
        .register_document_app::<crate::apps::puzzle3d::Puzzle3dPlayApp>(crate::apps::puzzle3d::create_puzzle3d_app())
        .register_document_app::<crate::apps::puzzle5d::Puzzle5dPlayApp>(crate::apps::puzzle5d::create_puzzle5d_app())
        .build()
}

/// 🔧️ `PluginBuilder::setup` is a single `Option<fn()>`, not a repeatable slot, so the two
/// no-declaration-field escape hatches from the three artifact engines' own docs — app-scope
/// config/presence schema, and the OS media-host export/import bridges — are combined into this one
/// callback rather than four `.setup(...)` calls silently overwriting each other down to the last.
fn setup() {
    crate::artifacts::puzzle2d::standards::v1::engine::register_app_schemas();
    crate::artifacts::puzzle2d::standards::v1::engine::register_media_io();
    crate::artifacts::puzzle3d::standards::v1::engine::register_mesh_io();
    crate::artifacts::puzzle5d::standards::v1::engine::register_mesh_io();
}
