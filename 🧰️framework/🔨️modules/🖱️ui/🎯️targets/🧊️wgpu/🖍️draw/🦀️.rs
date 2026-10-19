//! 🧊️ Actual private WGPU execution, resource custody and prepared presentation.
#[path="🎮️execution/🦀️.rs"]
mod gpu;
pub mod kernel_3d_scene {
    pub use semio_framework_ui_scene_algebra::*;
    pub use semio_framework_ui_scene_camera::*;
    pub use semio_framework_ui_scene_mesh::*;
    pub use semio_framework_ui_scene_pass::*;
}
pub use semio_framework_ui_theme::geometry;
pub use semio_framework_ui_wgpu_draw_execution::{MeshGpuTable,MeshGpuSourceReturn,RasterKeepSetV1,RasterResidencyLedger,RasterTextureCleanupStep,RasterTextureWitness,MESH_GPU_KEEP_VERSION_CAPACITY};
pub use gpu::{GpuContext,PreparedGpuPresentCursor,PreparedGpuReadback};

#[cfg(any(test,feature="testkit"))]
/// 🧪️ Exposes immutable production shader text without backend objects.
pub mod testing {pub fn ui_shader_source()->&'static str{semio_framework_ui_render_prepared_shaders::UI_SHADER}}

#[path="📥️platform/🦀️.rs"]
pub mod platform;

#[cfg(all(feature="canvas-raster",not(all(target_arch="wasm32",target_env="p2"))))]
pub use gpu::{EngineRasterBuild,EngineRasterBuildPhase,EngineRasterStageFault};

/// 🕸️ Observes actual retained mesh resources without exposing backend handles.
pub struct MeshGpuObservation{pub source:crate::kernel_3d_scene::Mesh3dLease,pub index_count:u32,pub index_bytes:u64,pub vertex_bytes:u64}
