//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the Puzzle 2D, 3D, and 5D surfaces.
    pub enum PuzzleApps: PluginApp {
        Puzzle2dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_puzzle_2d::editor::puzzle2d::Puzzle2dPlayApp>>),
        Puzzle2dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_puzzle_2d::viewer::puzzle2d::Puzzle2dViewer>>),
        Puzzle3dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_puzzle_3d::editor::puzzle3d::Puzzle3dPlayApp>>),
        Puzzle3dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_puzzle_3d::viewer::puzzle3d::Puzzle3dViewer>>),
        Puzzle5dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_puzzle_5d::editor::puzzle5d::Puzzle5dPlayApp>>),
        Puzzle5dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_puzzle_5d::viewer::puzzle5d::Puzzle5dViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.declare_artifact(…)` (ticket
/// `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, `terra-descriptors` packet, following the
/// `terra-fleet-trinity-recipe` recipe) replaces the old `.artifact(declaration())`/`.editor()`/
/// `.viewer()` triad — one registration channel per artifact (puzzle2d, puzzle3d, puzzle5d), each
/// built by its own artifact engine.
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
pub fn plugin() -> Result<Plugin<PuzzleApps>, PluginAssemblyError> {
    semio_s_artifact_puzzle_3d::editor::puzzle3d::precompute::initialize();
    Plugin::<PuzzleApps>::builder("puzzle")
        .label("Puzzle")
        .version("0.1.0")
        .package_id("semio:puzzle")
        .declare_artifact(semio_s_artifact_puzzle_2d::artifact::<PuzzleApps>())
        .declare_artifact(semio_s_artifact_puzzle_3d::artifact::<PuzzleApps>())
        .declare_artifact(semio_s_artifact_puzzle_5d::artifact::<PuzzleApps>())
        .editor_mutation_roster::<semio_s_artifact_puzzle_2d::editor::puzzle2d::Puzzle2dPlayApp>()
        .viewer_mutation_roster::<semio_s_artifact_puzzle_2d::viewer::puzzle2d::Puzzle2dViewer>()
        .editor_mutation_roster::<semio_s_artifact_puzzle_3d::editor::puzzle3d::Puzzle3dPlayApp>()
        .viewer_mutation_roster::<semio_s_artifact_puzzle_3d::viewer::puzzle3d::Puzzle3dViewer>()
        .editor_mutation_roster::<semio_s_artifact_puzzle_5d::editor::puzzle5d::Puzzle5dPlayApp>()
        .viewer_mutation_roster::<semio_s_artifact_puzzle_5d::viewer::puzzle5d::Puzzle5dViewer>()
        // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M5 — `.activation(…)`/`.execution(…)`/
        // `.requests(…)` (`📓️design-abi.md` §3/§6), same shape M0/M1 already landed for
        // stdio/draw/forms/mathematical/layout/raster. One activation per owned artifact kind, read
        // live from each kind's own `artifact_kind().id` (never hardcoded).
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_puzzle_2d::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_puzzle_3d::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_puzzle_5d::artifact_kind().id })
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
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🔖️SurfaceTests

#[cfg(feature = "plugin-entry")]
semio_framework_plugin::plugin_exports!(plugin, PuzzleApps);
