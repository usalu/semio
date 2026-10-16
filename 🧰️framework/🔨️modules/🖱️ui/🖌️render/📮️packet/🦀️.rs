//! 📮️ The producer side of a frame: the typed resource registry, the scene builder and its immutable
//! `RenderPacket`, the CPU tessellation that feeds it, and the `GraphicsBackend` contract every backend
//! replays that packet through.
//!
//! This crate describes pixels and never touches a device: `wgpu`, `winit` and every graphics binding
//! live in the backend and host crates. It sits at the bottom of the render crate graph — nothing here
//! knows about elements, layout, text or input dispatch.

#[path = "../🔌️backend/🦀️.rs"]
mod backend;
#[path = "../🗃️resource/🦀️.rs"]
mod resource;
#[path = "../🎬️scene/🦀️.rs"]
mod scene;
#[path = "../📐️tessellate/🦀️.rs"]
mod tessellate;

#[cfg(feature = "backend-testing")]
pub use backend::ReadbackImage;
pub use backend::{ActiveBackend, BackendError, DeviceCapabilities, DeviceStatus, FrameStats, GpuTier, GraphicsBackend, LossReason, MemoryClass, NullBackend, PhysicalSize, RecoveredResources, RenderReport, ResourceKind, SurfaceFormat};
pub use resource::{AtlasId, MeshId, ResourceError, ResourceOp, ResourceRegistry, ResourceState, TextureId};
pub use scene::{
    ClipRegion, DrawBatch, FinishParams, GlassInstance, GlassRegion, GlassStyle, LayerState, LayoutRect, LineVertex3, MeshInstance, PipelineKind, QuadInstance, RenderPacket, Scene, SceneBuilder, SceneError, SceneLayer, ScissorRect, StencilPolicy,
    SurfaceLighting, SurfaceLineDraw, SurfaceMaterial, SurfaceMeshDraw, SurfacePass, SurfaceShadow, SurfaceTexturedDraw, TexturedMeshInstance, VectorVertex, KIND_FINISHED_BORDER, KIND_GLYPH, KIND_INTRODUCING_BORDER, KIND_LOADING_BORDER, KIND_RASTER,
    KIND_ROUNDED, KIND_SOLID, KIND_TEXTURED, KIND_WAITING_BORDER,
};
pub use tessellate::{dashed_line_segments, ear_clip_polygon, layer_scissors, mask_instances, merge_scissor_bounds, snap_point, snap_rect, snap_to_device_pixels, thick_line_positions, triangle_fan_positions, union_scissors};
