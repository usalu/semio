//! 🎬️ Scene primitives, the stacked-scissor/silhouette-clip/glass-region/overlay-stream
//! builder model, `Scene::finish` (validate → snap → order → batch → hash) and `RenderPacket`.
//!
//! This is the backend-neutral half of the wgpu target's fused `draw.rs`: the display-list model and
//! its CPU algorithms, with every `wgpu::` type either dropped (device/pipeline/buffer plumbing,
//! which belongs to a backend) or replaced by a plain description (`StencilPolicy` in place of
//! `wgpu::StencilState`). A `Scene` never suspends — building one and calling `Scene::finish` is a
//! single sync run-to-completion step (ruling U1).

use crate::resource::{MeshId, ResourceOp, ResourceRegistry, TextureId};
use crate::tessellate;
use bytemuck::{Pod, Zeroable};
#[cfg(any(test, feature = "backend-testing"))]
use std::collections::{HashMap, HashSet};

//#region 🔖️Scene

#[path = "🔲️rect-quad-glass-and-stencil-primitives/🦀️.rs"]
mod rect_quad_glass_and_stencil_primitives;
pub use rect_quad_glass_and_stencil_primitives::{
    ClipRegion, GlassInstance, GlassRegion, GlassStyle, LayoutRect, QuadInstance, ScissorRect,
    StencilPolicy, VectorVertex, KIND_FINISHED_BORDER, KIND_GLYPH, KIND_INTRODUCING_BORDER,
    KIND_LOADING_BORDER, KIND_RASTER, KIND_ROUNDED, KIND_SOLID, KIND_TEXTURED, KIND_WAITING_BORDER,
};
#[cfg(any(test,feature="backend-testing"))]
use rect_quad_glass_and_stencil_primitives::is_animated_kind;

#[path = "🎭️surface-draws-and-pipeline-kinds/🦀️.rs"]
mod surface_draws_and_pipeline_kinds;
pub use surface_draws_and_pipeline_kinds::{
    LineVertex3, MeshInstance, PipelineKind, SurfaceLighting, SurfaceLineDraw, SurfaceMaterial,
    SurfaceMeshDraw, SurfacePass, SurfaceShadow, SurfaceTexturedDraw, TexturedMeshInstance,
};

#[path = "🏗️scene-builder/🦀️.rs"]
mod scene_builder;
pub use scene_builder::{SceneBuilder, SceneLayer};

#[path = "🧮️scene-finish-and-render-packet/🦀️.rs"]
mod scene_finish_and_render_packet;
pub use scene_finish_and_render_packet::{
    DrawBatch, FinishParams, LayerState, RenderPacket, Scene, SceneError,
};
#[cfg(any(test,feature="backend-testing"))]
#[cfg(test)]
use scene_finish_and_render_packet::content_hash;

//#region Tests

#[cfg(test)]
#[path = "../🧪️tests/🔬️scene-unit/🦀️.rs"]
mod tests;

//#endregion Tests

//#endregion 🔖️Scene
