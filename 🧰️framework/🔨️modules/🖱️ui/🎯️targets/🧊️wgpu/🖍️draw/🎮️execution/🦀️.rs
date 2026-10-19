// #region gpu
//! 🖥️ WebGPU device, surface, and frame loop.
use semio_framework_ui_wgpu_device as gpu_device;

use semio_framework_ui_wgpu_draw_execution::{
    FrameBuffers, MeshGpuTable, PreparedCompositeTarget, RasterTextureAdmission, RasterTextureCancelFault, RasterTextureCleanupStep, RasterTextureStageFault, RasterTextureTable, RasterTextureWitness, RasterUploadPixels, SceneColorTarget, UiPipelines, SCENE_MIP_LEVELS,
};
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use semio_framework_ui_prepared::OffscreenPresentToken;
use semio_framework_ui_prepared::{DrawMeasureCursor, PreparedRenderEviction, PreparedRenderGate, PreparedRenderPacket, PreparedRenderUpload, RasterContentIdentity, UiPresentToken, PREPARED_RENDER_COMMAND_PAGES, PREPARED_RENDER_COMMAND_PAGE_ITEMS};
use semio_framework_ui_text::FontAtlas;
use std::sync::atomic::{AtomicPtr, AtomicU8, Ordering};
use std::sync::Arc;
use gpu_device::platform::Surface;

#[path = "🎚️present-cursor-and-helpers/🦀️.rs"]
mod present_cursor_and_helpers;
use present_cursor_and_helpers::{
    measure_prepared_gpu_opportunity, prepared_command_clip_piece,
    prepared_draw_scalar_uses_world_encoded_attachment, prepared_glass_command,
    PreparedAtlasUploadCursor, PreparedCommandClipPiece, PreparedGpuPresentPhase,
};
#[cfg(test)]
use present_cursor_and_helpers::{
    address_prepared_glass_region, admit_prepared_gpu_opportunity, PREPARED_GPU_ABANDONMENT_SLOTS,
    PREPARED_GPU_ABANDONMENT_STATE, PREPARED_GPU_OPPORTUNITY_CEILING_US,
};
pub use present_cursor_and_helpers::PreparedGpuPresentCursor;

#[path = "🧠️gpu-context-and-meshes/🦀️.rs"]
mod gpu_context_and_meshes;
use gpu_context_and_meshes::GpuSessionRoots;
pub use gpu_context_and_meshes::GpuContext;

#[path = "🧷️prepared-upload-and-present-steps/🦀️.rs"]
mod prepared_upload_and_present_steps;

#[path = "🖍️draw-scalar-encoding-and-raster-delegates/🦀️.rs"]
mod draw_scalar_encoding_and_raster_delegates;

#[path = "📤️readback/🦀️.rs"]
mod readback;
pub use readback::PreparedGpuReadback;
use readback::PreparedReadbackLayout;

#[cfg(test)]
#[path = "../🧪️tests/🔬️targets-wgpu-gpu-prepared-present/🦀️.rs"]
mod prepared_present_tests;

// #endregion gpu

#[cfg(all(feature="canvas-raster",not(all(target_arch="wasm32",target_env="p2"))))]
#[path="🎬️canvas/🦀️.rs"]
mod canvas;
#[cfg(all(feature="canvas-raster",not(all(target_arch="wasm32",target_env="p2"))))]
pub use canvas::{EngineRasterBuild,EngineRasterBuildPhase,EngineRasterStageFault};
