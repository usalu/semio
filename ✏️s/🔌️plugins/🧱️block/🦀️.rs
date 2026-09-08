//! 🧱️ Block composition of independently owned 2D, 3D and 5D artifact packages.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the block editor and viewer surfaces.
    pub enum BlockApps: PluginApp {
        Block2dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_block_2d::editor::block2d::Block2dPlayApp>>),
        Block2dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_block_2d::viewer::block2d::Block2dViewer>>),
        Block3dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_block_3d::editor::block3d::Block3dPlayApp>>),
        Block3dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_block_3d::viewer::block3d::Block3dViewer>>),
        Block5dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_block_5d::editor::block5d::Block5dPlayApp>>),
        Block5dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_block_5d::viewer::block5d::Block5dViewer>>),
    }
}
//#endregion 🗃️Apps

//#region 🔌️Registration
/// 🔌️ Composes the three artifact declarations with their mutation rosters and activation events.
pub fn plugin() -> Result<Plugin<BlockApps>, PluginAssemblyError> {
    Plugin::<BlockApps>::builder("block")
        .label("Block")
        .version("0.1.0")
        .package_id("semio:block")
        .declare_artifact(semio_s_artifact_block_2d::artifact::<BlockApps>())
        .declare_artifact(semio_s_artifact_block_3d::artifact::<BlockApps>())
        .declare_artifact(semio_s_artifact_block_5d::artifact::<BlockApps>())
        .editor_mutation_roster::<semio_s_artifact_block_2d::editor::block2d::Block2dPlayApp>()
        .viewer_mutation_roster::<semio_s_artifact_block_2d::viewer::block2d::Block2dViewer>()
        .editor_mutation_roster::<semio_s_artifact_block_3d::editor::block3d::Block3dPlayApp>()
        .viewer_mutation_roster::<semio_s_artifact_block_3d::viewer::block3d::Block3dViewer>()
        .editor_mutation_roster::<semio_s_artifact_block_5d::editor::block5d::Block5dPlayApp>()
        .viewer_mutation_roster::<semio_s_artifact_block_5d::viewer::block5d::Block5dViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_block_2d::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_block_3d::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_block_5d::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist block2d/block3d/block5d edits to the open document".into(), optional: false })
        .try_build()
}
//#endregion 🔌️Registration

//#region 🧪️SurfaceTests
/// 🧪️ Verifies read-only viewers and matching editor/viewer dialects.
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests

#[cfg(feature = "plugin-entry")]
semio_framework_plugin::plugin_exports!(plugin, BlockApps);
