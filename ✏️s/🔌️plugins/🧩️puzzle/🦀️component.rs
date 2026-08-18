//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::{ExecutionMode, Plugin};

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old umbrella
/// `.setup(puzzle2d::engine::register)` escape hatch — one declaration per artifact (puzzle2d,
/// puzzle3d, puzzle5d), each built by its own artifact engine.
///
/// **W1d update.** The app-schema half of the old `.setup()` callback is GONE:
/// `register_app_schemas()` was never actually a distinct `ArtifactDeclaration` coverage gap — it
/// was category-1 app-scope schema wearing a different name. `Puzzle2dPlayApp`/`Puzzle3dPlayApp`/
/// `Puzzle5dPlayApp` now each override `ArtifactApp::app_schema()`, so `.document_app()`
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
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("puzzle")
        .label("Puzzle")
        .version("0.1.0")
        .artifact(crate::artifacts::puzzle2d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .artifact(crate::artifacts::puzzle3d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .artifact(crate::artifacts::puzzle5d::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::puzzle2d::Puzzle2dPlayApp>(crate::editor::puzzle2d::create_puzzle2d_app())
        .editor_mutation_roster::<crate::editor::puzzle2d::Puzzle2dPlayApp>()
        .viewer::<crate::viewer::puzzle2d::Puzzle2dViewer>(crate::viewer::puzzle2d::create_puzzle2d_viewer())
        .viewer_mutation_roster::<crate::viewer::puzzle2d::Puzzle2dViewer>()
        .editor::<crate::editor::puzzle3d::Puzzle3dPlayApp>(crate::editor::puzzle3d::create_puzzle3d_app())
        .editor_mutation_roster::<crate::editor::puzzle3d::Puzzle3dPlayApp>()
        .viewer::<crate::viewer::puzzle3d::Puzzle3dViewer>(crate::viewer::puzzle3d::create_puzzle3d_viewer())
        .viewer_mutation_roster::<crate::viewer::puzzle3d::Puzzle3dViewer>()
        .editor::<crate::editor::puzzle5d::Puzzle5dPlayApp>(crate::editor::puzzle5d::create_puzzle5d_app())
        .editor_mutation_roster::<crate::editor::puzzle5d::Puzzle5dPlayApp>()
        .viewer::<crate::viewer::puzzle5d::Puzzle5dViewer>(crate::viewer::puzzle5d::create_puzzle5d_viewer())
        .viewer_mutation_roster::<crate::viewer::puzzle5d::Puzzle5dViewer>()
        // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M5 — `.activation(…)`/`.execution(…)`/
        // `.requests(…)` (`📓️design-abi.md` §3/§6), same shape M0/M1 already landed for
        // stdio/draw/forms/mathematical/layout/raster. One activation per owned artifact kind, read
        // live from each kind's own `artifact_kind().id` (never hardcoded).
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::puzzle2d::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::puzzle3d::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::puzzle5d::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("documents.write".into()),
            scope: "plugin".into(),
            reason: "persist puzzle2d/puzzle3d/puzzle5d editor edits (brush placement, fill build, engagement commits) to the open document".into(),
            optional: false,
        })
        .requests(CapabilityRequest {
            id: CapabilityId("ui.dialog".into()),
            scope: "plugin".into(),
            reason: "puzzle3d's add-object flow opens the addObject dialog (Effect::OpenDialog)".into(),
            optional: false,
        })
        .requests(CapabilityRequest {
            id: CapabilityId("shell.clipboard".into()),
            scope: "plugin".into(),
            reason: "puzzle5d's copy/cut interception writes fragments to the system clipboard (Effect::ClipboardWrite)".into(),
            optional: false,
        })
        .try_build()
}

//#region 🔖️SurfaceTests
#[cfg(test)]
mod surface_tests {
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[test]
    fn puzzle2d_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::puzzle2d::Puzzle2dViewer>();
    }

    #[test]
    fn puzzle2d_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::puzzle2d::Puzzle2dPlayApp, crate::viewer::puzzle2d::Puzzle2dViewer>();
    }

    #[test]
    fn puzzle3d_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::puzzle3d::Puzzle3dViewer>();
    }

    #[test]
    fn puzzle3d_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::puzzle3d::Puzzle3dPlayApp, crate::viewer::puzzle3d::Puzzle3dViewer>();
    }

    #[test]
    fn puzzle5d_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::puzzle5d::Puzzle5dViewer>();
    }

    #[test]
    fn puzzle5d_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::puzzle5d::Puzzle5dPlayApp, crate::viewer::puzzle5d::Puzzle5dViewer>();
    }
}
//#endregion 🔖️SurfaceTests
