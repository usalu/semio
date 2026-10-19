//! 🎬️ Actual draw encoding and progressive resource uploads consume prepared pipeline resources.
use semio_framework_ui_wgpu_device as gpu_device;
use semio_framework_ui_draw_contract::*;
use bytemuck::{Pod,Zeroable};
use std::mem::size_of;
use gpu_device::util::DeviceExt;
mod kernel_3d_scene{pub use semio_framework_ui_scene_algebra::*;pub use semio_framework_ui_scene_camera::*;pub use semio_framework_ui_scene_mesh::*;pub use semio_framework_ui_scene_pass::*;}
use kernel_3d_scene::{Mat4Math,ProceduralGrid3d,ScenePass3d,SceneRenderProfile3d,PROCEDURAL_GRID_CELL_THICKNESS,PROCEDURAL_GRID_FADE_STRENGTH};
use semio_framework_ui_prepared as prepared;
use semio_framework_ui_raster_owner as raster_ownership;
use semio_framework_ui_theme as theme;
use semio_framework_ui_theme::{geometry,Theme};
#[cfg(test)]
use semio_framework_ui_theme::Rgba;
pub use semio_framework_ui_wgpu_mesh::*;
pub use semio_framework_ui_wgpu_raster::*;
pub use semio_framework_ui_wgpu_pipelines::*;
use prepared::{PreparedRasterPages,RasterContentIdentity};

#[path = "🧱️pipeline-state-and-world-passes/🦀️.rs"]
mod pipeline_state_and_world_passes;
pub use pipeline_state_and_world_passes::{
    physical_scissor_rect, physical_viewport_rect, FrameBuffers, GrowBuffer, UiPipelines,
};
#[cfg(test)]
use pipeline_state_and_world_passes::{
    build_batch_masks, build_layer_batches, build_overlay_layer_batches, layer_matches_filter,
    mask_instances, LayerBatch, LayerBatchFilter, PreparedWorldPass, WorldDrawRange,
};
use pipeline_state_and_world_passes::{set_pass_scissor, stencil_attachment};

#[path = "🌍️world-pass-drawing/🦀️.rs"]
mod world_pass_drawing;

#[path = "🖼️atlas-uploads-and-instance-encoding/🦀️.rs"]
mod atlas_uploads_and_instance_encoding;

#[path = "🎨️world-material-and-effect-encoding/🦀️.rs"]
mod world_material_and_effect_encoding;

#[path = "🪟️scene-content-glass-and-composite/🦀️.rs"]
mod scene_content_glass_and_composite;

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
