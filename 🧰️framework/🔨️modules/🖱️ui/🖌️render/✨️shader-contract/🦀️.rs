//! ✨️ Canonical WGSL for the five shader families plus their `PipelineSpec` metadata.
//!
//! This is the single source of truth every `GraphicsBackend` builds its pipelines from — no
//! backend reads WGSL directly. webgpu consumes [`ShaderVariant::wgsl`] verbatim; vulkan/metal/
//! d3d12 cross-compile it at build time (SPIR-V/MSL/HLSL backends, never linked at runtime).
//! Copied here from `🎯️targets/🧊️wgpu/🦀️shaders.rs` after repairing the asyncify corruption
//! (`async fn vs_main`/`async fn fs_main` — WGSL has no `async` keyword) — see
//! `📓️terra-shader-repair-report.md` in ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`
//! for the full corruption inventory. The `#[cfg(test)]` module below is the permanent regression
//! guard: it runs the owned lexical and structural validator over every constant so this class of
//! defect fails in CI,
//! on any platform, without a GPU.

#[path = "🧱️pipeline-spec-types/🦀️.rs"]
mod pipeline_spec_types;
pub use pipeline_spec_types::{
    BindGroupEntrySpec, BindGroupSpec, BindingKind, BlendMode, ColorTarget, ColorWriteMask,
    CompareFunction, CullMode, DepthBiasSpec, DepthStencilFormat, DepthStencilSpec, PipelineSpec,
    PrimitiveTopology, ShaderFamily, ShaderStageVisibility, ShaderVariant, StencilOperation,
    StencilStateSpec, VertexAttributeSpec, VertexBufferSpec, VertexFormat, VertexStepMode,
    FRAGMENT_STAGE, NO_DEPTH_BIAS, VERTEX_FRAGMENT_STAGE,
};

#[path = "🧵️ui-and-vector-families/🦀️.rs"]
mod ui_and_vector_families;
pub use ui_and_vector_families::{
    UI_CONTENT_PIPELINE, UI_FAMILY, UI_MASK_PIPELINE, UI_SHADER, VECTOR_FAMILY, VECTOR_PIPELINE,
    VECTOR_SHADER,
};
use ui_and_vector_families::UI_GLOBALS_BIND_GROUP;

#[path = "🌐️world3d-mesh-family/🦀️.rs"]
mod world3d_mesh_family;
use world3d_mesh_family::{WORLD_CONTENT_STENCIL, WORLD_GLOBALS_BIND_GROUP};
pub use world3d_mesh_family::{
    WORLD3D_OPAQUE_PIPELINE, WORLD3D_SHADER, WORLD3D_TRANSLUCENT_PIPELINE,
};

#[path = "📏️world3d-lines-grid-and-textured-family/🦀️.rs"]
mod world3d_lines_grid_and_textured_family;
pub use world3d_lines_grid_and_textured_family::{
    WORLD3D_FAMILY, WORLD3D_GRID_PIPELINE, WORLD3D_GRID_SHADER, WORLD3D_LINES_SHADER,
    WORLD3D_LINE_PIPELINE, WORLD3D_TEXTURED_PIPELINE, WORLD3D_TEXTURED_SHADER,
};

#[path = "🌫️blur-glass-families-and-registry/🦀️.rs"]
mod blur_glass_families_and_registry;
pub use blur_glass_families_and_registry::{
    ALL_SHADERS, BLUR_DOWNSAMPLE_PIPELINE, BLUR_DOWNSAMPLE_SHADER, BLUR_FAMILY, GLASS_FAMILY,
    GLASS_PIPELINE, GLASS_SHADER, SCENE_BLIT_PIPELINE, SCENE_BLIT_SHADER,
};

#[cfg(test)]
#[path = "../🧪️tests/🔬️shader-contract-unit/🦀️.rs"]
mod tests;
