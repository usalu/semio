//! 🏭️ Actual pipeline, atlas, target and bind-group construction owns every prepared render resource.
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
use semio_framework_ui_wgpu_mesh::*;
use semio_framework_ui_render_prepared_shaders::*;

#[path = "🎯️gpu-targets-and-pipeline-resources/🦀️.rs"]
mod gpu_targets_and_pipeline_resources;
pub use gpu_targets_and_pipeline_resources::{
    content_stencil_state, BlurGlobals, GlassInstance, PipelineResources, PreparedCompositeTarget,
    SceneColorTarget, WorldGlobalsRing, WorldLineGpuVertex, WorldPostprocessGlobals,
    WorldShadowTarget, SCENE_MIP_LEVELS, WORLD_GLOBALS_SLOT_SIZE,
};
#[cfg(any(test,feature="testkit"))]
pub use gpu_targets_and_pipeline_resources::mask_stencil_state;

#[path = "🏗️pipeline-resources-construction/🦀️.rs"]
mod pipeline_resources_construction;

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
