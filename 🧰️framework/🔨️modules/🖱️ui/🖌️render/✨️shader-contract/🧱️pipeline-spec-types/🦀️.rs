//! 🧱️ Pipeline spec types: the vertex formats and buffers, the shader stage visibility, the bind groups, the blend, colour write, depth and stencil specs, the primitive topology and the pipeline, shader variant and family records. See [`super`].

use super::*;

//#region 🔖️ShaderContract
//#region 🧱️PipelineTypes
/// 🎚️ The scalar layout of one vertex attribute, as declared by a WGSL `@location`. A backend-neutral
/// mirror of the handful of `wgpu::VertexFormat` variants every pipeline here actually uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VertexFormat {
    Float32x2,
    Float32x3,
    Float32x4,
}

impl VertexFormat {
    /// 📏️ Byte width of this format, used to cross-check `VertexAttributeSpec::offset` arithmetic.
    pub const fn byte_size(self) -> u64 {
        match self {
            VertexFormat::Float32x2 => 8,
            VertexFormat::Float32x3 => 12,
            VertexFormat::Float32x4 => 16,
        }
    }
}

/// 🔁️ Whether a vertex buffer advances per-vertex or per-instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VertexStepMode {
    Vertex,
    Instance,
}

/// 📌️ One `@location` slot inside a vertex buffer.
#[derive(Clone, Copy, Debug)]
pub struct VertexAttributeSpec {
    pub shader_location: u32,
    pub format: VertexFormat,
    pub offset: u64,
}

/// 📦️ One vertex buffer binding — its stride, step mode, and the attributes it carries.
#[derive(Clone, Copy, Debug)]
pub struct VertexBufferSpec {
    pub stride: u64,
    pub step_mode: VertexStepMode,
    pub attributes: &'static [VertexAttributeSpec],
}

/// 🚦️ Which shader stages a bind group entry is visible to. Every entry in this contract is
/// visible to exactly vertex, exactly fragment, or both — no compute stage exists in this family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShaderStageVisibility {
    pub vertex: bool,
    pub fragment: bool,
}

pub const FRAGMENT_STAGE: ShaderStageVisibility = ShaderStageVisibility { vertex: false, fragment: true };

pub const VERTEX_FRAGMENT_STAGE: ShaderStageVisibility = ShaderStageVisibility { vertex: true, fragment: true };

/// 🗄️ What a bind group entry resolves to. `min_size` on `UniformBuffer` is `None` where the source
/// pipeline left `min_binding_size` unspecified (wgpu infers it) rather than a guess at a number.
#[derive(Clone, Copy, Debug)]
pub enum BindingKind {
    UniformBuffer { dynamic_offset: bool, min_size: Option<u64> },
    Texture2D,
    DepthTexture2D,
    Sampler,
    ComparisonSampler,
}

/// 🔌️ One binding inside a bind group layout.
#[derive(Clone, Copy, Debug)]
pub struct BindGroupEntrySpec {
    pub binding: u32,
    pub visibility: ShaderStageVisibility,
    pub kind: BindingKind,
}

/// 🗂️ One `@group(N)` — its index and the bindings a backend must supply.
#[derive(Clone, Copy, Debug)]
pub struct BindGroupSpec {
    pub group_index: u32,
    pub entries: &'static [BindGroupEntrySpec],
}

/// 🎨️ Fixed-function color blending. `Replace` writes the fragment color unblended (used by the
/// opaque world mesh and both blur/blit fullscreen passes); `None` disables blending altogether
/// (the silhouette mask pass, which writes no color at all — see `color_write`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendMode {
    None,
    AlphaBlending,
    Replace,
}

/// ✍️ Color target write mask. Only two shapes occur in this family: write nothing (the stencil
/// mask pass) or write every channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorWriteMask {
    None,
    All,
}

/// 🖥️ The color attachment a pipeline renders into. Every pipeline in this contract targets
/// whatever color format the backend's active surface/render-target uses — none hard-codes a
/// format — so this is a marker, not a concrete pixel format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorTarget {
    SurfaceFormat,
}

/// 🆚️ Depth/stencil compare functions used anywhere in this family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompareFunction {
    Always,
    Equal,
    Less,
    LessEqual,
}

/// 🪄️ Stencil operations used anywhere in this family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StencilOperation {
    Keep,
    Replace,
}

/// 🎯️ One stencil face's test + write behavior. Every pipeline in this contract sets front and
/// back faces identically, so one spec covers both.
#[derive(Clone, Copy, Debug)]
pub struct StencilStateSpec {
    pub compare: CompareFunction,
    pub fail_op: StencilOperation,
    pub depth_fail_op: StencilOperation,
    pub pass_op: StencilOperation,
    pub read_mask: u32,
    pub write_mask: u32,
}

/// 📐️ Depth bias (polygon offset), applied to the translucent world pass to avoid z-fighting
/// against the opaque pass it draws over.
#[derive(Clone, Copy, Debug)]
pub struct DepthBiasSpec {
    pub constant: i32,
    pub slope_scale: f32,
    pub clamp: f32,
}

/// 🧊️ The one depth/stencil texture format used anywhere in this family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DepthStencilFormat {
    Depth24PlusStencil8,
}

/// 🕳️ Full depth/stencil pipeline state.
#[derive(Clone, Copy, Debug)]
pub struct DepthStencilSpec {
    pub format: DepthStencilFormat,
    pub depth_write_enabled: bool,
    pub depth_compare: CompareFunction,
    pub stencil: StencilStateSpec,
    pub bias: DepthBiasSpec,
}

pub const NO_DEPTH_BIAS: DepthBiasSpec = DepthBiasSpec { constant: 0, slope_scale: 0.0, clamp: 0.0 };

/// 🔺️ Primitive assembly topology.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimitiveTopology {
    TriangleList,
    LineList,
}

/// ✂️ Backface culling mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CullMode {
    None,
    Back,
}

/// 🏗️ Everything a `GraphicsBackend` needs to build one pipeline from a `ShaderVariant`'s WGSL,
/// without parsing that WGSL itself. `vertex_entry`/`fragment_entry` are cross-checked against the
/// variant's actual shader entry points by the `pipeline_entry_points_exist_in_shader` test below —
/// a spec that has drifted from its shader fails loudly there instead of silently at device time.
#[derive(Clone, Copy, Debug)]
pub struct PipelineSpec {
    pub label: &'static str,
    pub vertex_entry: &'static str,
    pub fragment_entry: &'static str,
    pub vertex_buffers: &'static [VertexBufferSpec],
    pub bind_groups: &'static [BindGroupSpec],
    pub blend: BlendMode,
    pub color_write: ColorWriteMask,
    pub target: ColorTarget,
    pub topology: PrimitiveTopology,
    pub cull_mode: CullMode,
    pub depth_stencil: Option<DepthStencilSpec>,
}

/// 🧬️ One WGSL source plus the one or more `PipelineSpec`s built from it (the UI shader alone
/// backs two pipelines — a stencil mask pass and the content pass — sharing one `vs_main`/`fs_main`
/// pair).
#[derive(Clone, Copy, Debug)]
pub struct ShaderVariant {
    pub name: &'static str,
    pub wgsl: &'static str,
    pub pipelines: &'static [PipelineSpec],
}

/// 👪️ A group of `ShaderVariant`s that render the same semantic content (UI quads, vector
/// triangles, 3D world geometry, blur mip chain, glass backdrop).
#[derive(Clone, Copy, Debug)]
pub struct ShaderFamily {
    pub name: &'static str,
    pub variants: &'static [ShaderVariant],
}
//#endregion 🧱️PipelineTypes
//#endregion 🔖️ShaderContract
