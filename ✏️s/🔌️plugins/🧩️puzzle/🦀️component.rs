//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old umbrella
/// `.setup(puzzle2d::engine::register)` escape hatch — one declaration per artifact (puzzle2d,
/// puzzle3d, puzzle5d), each built by its own artifact engine.
///
/// **W1d update.** The app-schema half of the old `.setup()` callback is GONE:
/// `register_app_schemas()` was never actually a distinct `ArtifactDeclaration` coverage gap — it
/// was category-1 app-scope schema wearing a different name. `Puzzle2dPlayApp`/`Puzzle3dPlayApp`/
/// `Puzzle5dPlayApp` now each override `ArtifactApp::app_schema()`, so `.register_document_app()`
/// below auto-registers all three, exactly like `🗒️note`'s exemplar — see each app's own
/// `app_schema` override doc.
///
/// **`.setup()` still survives for the OS media-host bridges** (`register_media_io`/
/// `register_mesh_io` — `register_2d_export_handlers`/`register_dwg_import_handler`/
/// `register_mesh_exporter`/`register_mesh_importer`/`register_mesh_dwg_{export,import}_handler`),
/// judged NOT to get a new `ArtifactDeclaration` field this pass (see `📓️w1d-declaration-gaps-report.md`
/// for the full reasoning): they write into `semio_framework_os`'s own process-global media-handler
/// registry — a SEPARATE registry from `io_registry`/`ComposerEntry` (which `.composers(...)` below
/// already covers, and which independently duplicates part of this same format coverage for 2d
/// SVG/DWG and 3d DWG/OBJ/STL export), keyed by a legacy "OS media kind" string (`"2d.puzzle"` /
/// `"3d.puzzle"` / `"5d.puzzle"`) that is NOT `ArtifactDeclaration.kind` (`"s.puzzle2d"` etc.) — so a
/// declaration field could not even validate ownership the way `.composers()`/`.migrations()` do.
/// This registry family is the SAME one `📓️status.md` finding #3 documents as non-deterministic
/// under concurrent registrants elsewhere in this ticket (demonstrator racing an owner for
/// `3d.process`/`3d.procedural`) — adding a declaration field here would legitimize exactly that
/// mechanism rather than close it. Deleting outright (the lowpoly precedent: check the composer tree,
/// delete pure duplicates) is NOT done here either, because the two registries' format coverage only
/// PARTIALLY overlaps (composer also serves PDF/JSON/DXF/LAS/PLY/GLTF that the OS bridge does not, and
/// the OS bridge's own live consumer — the OS-level export/import dispatch this file does not own —
/// was not traced this pass) — deleting on inference alone risks silently breaking real export/import
/// UI functionality, which this ticket's "get everything working" rule forbids doing speculatively.
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

/// 🔧️ The OS media-host export/import bridges — see `plugin()`'s own doc for why these, and only
/// these, still need a plugin-root callback.
fn setup() {
    crate::apps::puzzle2d::register_media_io();
    crate::apps::puzzle3d::register_mesh_io();
    crate::apps::puzzle5d::register_mesh_io();
}
