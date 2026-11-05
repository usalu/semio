//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::{ExecutionMode, Plugin, PluginApp};
use semio_plugin::PluginBuilder;

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the Puzzle 2D, 3D, and 5D surfaces.
    pub enum PuzzleApps: PluginApp {
        Puzzle2dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_puzzle_2d::editor::puzzle2d::Puzzle2dPlayApp>>),
        Puzzle2dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_puzzle_2d::viewer::puzzle2d::Puzzle2dViewer>>),
        Puzzle3dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_puzzle_3d_editor::Puzzle3dPlayApp>>),
        Puzzle3dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_puzzle_3d_viewer::Puzzle3dViewer>>),
        Puzzle5dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_puzzle_5d::editor::puzzle5d::Puzzle5dPlayApp>>),
        Puzzle5dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_puzzle_5d::viewer::puzzle5d::Puzzle5dViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🧩️ Registers the three artifact trees and their six editor/viewer surfaces.
/// Foreign IO routes are published by each subset's canonical typed declaration.
pub fn plugin() -> Result<Plugin<PuzzleApps>, PluginAssemblyError> {
    PluginBuilder::<_, PuzzleApps>::new("puzzle")
        .label("Puzzle")
        .version("0.1.0")
        .package_id("semio:puzzle")
        .declare_artifact(semio_s_artifact_puzzle_2d::artifact::<PuzzleApps>())
        .declare_artifact(semio_s_artifact_puzzle_3d::artifact::<PuzzleApps>())
        .declare_artifact(semio_s_artifact_puzzle_5d::artifact::<PuzzleApps>())
        .editor_mutation_roster::<semio_s_artifact_puzzle_2d::editor::puzzle2d::Puzzle2dPlayApp>()
        .viewer_mutation_roster::<semio_s_artifact_puzzle_2d::viewer::puzzle2d::Puzzle2dViewer>()
        .editor_mutation_roster::<semio_s_artifact_puzzle_3d_editor::Puzzle3dPlayApp>()
        .viewer_mutation_roster::<semio_s_artifact_puzzle_3d_viewer::Puzzle3dViewer>()
        .editor_mutation_roster::<semio_s_artifact_puzzle_5d::editor::puzzle5d::Puzzle5dPlayApp>()
        .viewer_mutation_roster::<semio_s_artifact_puzzle_5d::viewer::puzzle5d::Puzzle5dViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_puzzle_2d::standards::v1::subsets::any::schema::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_puzzle_3d_model::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_puzzle_5d::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("artifacts.write".into()),
            scope: "plugin".into(),
            reason: "persist puzzle2d/puzzle3d/puzzle5d editor edits (brush placement, fill build, engagement commits) to the open document".into(),
            optional: false,
        })
        .requests(CapabilityRequest {
            id: CapabilityId("ui.dialog".into()),
            scope: "plugin".into(),
            reason: "puzzle3d's add-object flow opens the addObject dialog and puzzle5d's add-part flow the addPart dialog (Effect::OpenDialog)".into(),
            optional: false,
        })
        .requests(CapabilityRequest {
            id: CapabilityId("shell.clipboard".into()),
            scope: "plugin".into(),
            reason: "puzzle5d's copy/cut interception writes fragments to the system clipboard (Effect::ClipboardWrite)".into(),
            optional: false,
        })
        .asset(semio_s_artifact_puzzle_5d::examples::puzzle5d::capsule_dream::dsl_asset_declaration())
        .try_build()
}

//#region 🔖️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🔖️SurfaceTests


#[cfg(test)]
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
