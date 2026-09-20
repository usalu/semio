//! @emoji ✨️ Canonical WGSL for the five shader families plus their `PipelineSpec` metadata.
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

//#region 🧵️UiFamily

/// 🧊️ SDF quad megashader: rounded rects, glyphs, solid fills, textured/raster quads, and the
/// animated loading/waiting/finished/introducing border rings (`kind` 1..9), all driven by
/// `globals._pad.x` (elapsed seconds) for the animated variants. Repaired from the asyncify
/// corruption — every `async fn vs_main`/`async fn fs_main`/`async fn sdf_rounded_rect` had its
/// bogus `async` stripped; no other damage was found in this constant.
pub const UI_SHADER: &str = r#"
struct Globals {
screen_size: vec2<f32>,
_pad: vec2<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;
@group(0) @binding(1) var glyph_atlas: texture_2d<f32>;
@group(0) @binding(2) var glyph_sampler: sampler;
@group(0) @binding(3) var icon_atlas: texture_2d<f32>;
@group(0) @binding(4) var icon_sampler: sampler;

struct VertexInput {
@location(0) corner: vec2<f32>,
}

struct InstanceInput {
@location(1) rect: vec4<f32>,
@location(2) color: vec4<f32>,
@location(3) params: vec4<f32>,
@location(4) uv_rect: vec4<f32>,
}

struct VertexOutput {
@builtin(position) clip_position: vec4<f32>,
@location(0) local: vec2<f32>,
@location(1) size: vec2<f32>,
@location(2) color: vec4<f32>,
@location(3) params: vec4<f32>,
@location(4) uv: vec2<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
var out: VertexOutput;
let pos = instance.rect.xy + vertex.corner * instance.rect.zw;
let ndc = (pos / globals.screen_size) * 2.0 - vec2<f32>(1.0, 1.0);
out.clip_position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0);
out.local = vertex.corner * instance.rect.zw;
out.size = instance.rect.zw;
out.color = instance.color;
out.params = instance.params;
let uv_min = instance.uv_rect.xy;
let uv_max = instance.uv_rect.zw;
out.uv = mix(uv_min, uv_max, vertex.corner);
return out;
}

fn sdf_rounded_rect(p: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
let q = abs(p) - half_size + vec2<f32>(radius);
return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
let kind = i32(in.params.z + 0.5);
let glyph = textureSample(glyph_atlas, glyph_sampler, in.uv);
let icon = textureSample(icon_atlas, icon_sampler, in.uv);
if (kind == 1) {
    let half = in.size * 0.5;
    let p = in.local - half;
    let radius = in.params.x;
    let border = in.params.y;
    let dist = sdf_rounded_rect(p, half, radius);
    let fill_alpha = 1.0 - smoothstep(-1.0, 0.0, dist);
    let border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
    let alpha = max(fill_alpha * in.color.a, border_alpha * in.params.w);
    return vec4<f32>(in.color.rgb, alpha);
}
if (kind == 6) {
    let half = in.size * 0.5;
    let p = in.local - half;
    let radius = in.params.x;
    let border = in.params.y;
    let dist = sdf_rounded_rect(p, half, radius);
    let border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
    let two_pi = 6.28318530718;
    let duration = 1.6;
    let phase = globals._pad.x / duration;
    var theta = atan2(p.x, -p.y);
    theta = theta - floor(theta / two_pi) * two_pi;
    var spin = phase * two_pi;
    spin = spin - floor(spin / two_pi) * two_pi;
    var sweep = theta - spin;
    sweep = sweep - floor(sweep / two_pi) * two_pi;
    let comet_alpha = sweep / two_pi;
    let ring_alpha = max(comet_alpha, 0.2);
    let pulse = 0.775 - 0.225 * cos(two_pi * phase);
    let alpha = border_alpha * ring_alpha * pulse * in.color.a;
    return vec4<f32>(in.color.rgb, alpha);
}
if (kind == 7) {
    let half = in.size * 0.5;
    let p = in.local - half;
    let radius = in.params.x;
    let border = in.params.y;
    let dist = sdf_rounded_rect(p, half, radius);
    let border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
    let two_pi = 6.28318530718;
    let duration = 3.2;
    let phase = globals._pad.x / duration;
    var theta = atan2(p.x, -p.y);
    theta = theta - floor(theta / two_pi) * two_pi;
    var spin = phase * two_pi;
    spin = spin - floor(spin / two_pi) * two_pi;
    var sweep = theta - spin;
    sweep = sweep - floor(sweep / two_pi) * two_pi;
    let dash = step(fract(sweep / two_pi * 12.0), 0.6);
    let ring_alpha = max(dash, 0.2);
    let pulse = 0.85 - 0.15 * cos(two_pi * phase);
    let alpha = border_alpha * ring_alpha * pulse * in.color.a;
    return vec4<f32>(in.color.rgb, alpha);
}
if (kind == 8) {
    let half = in.size * 0.5;
    let p = in.local - half;
    let radius = in.params.x;
    let border = in.params.y;
    let dist = sdf_rounded_rect(p, half, radius);
    let border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
    let alpha = border_alpha * in.color.a;
    return vec4<f32>(in.color.rgb, alpha);
}
if (kind == 9) {
    let half = in.size * 0.5;
    let p = in.local - half;
    let radius = in.params.x;
    let border = in.params.y;
    let dist = sdf_rounded_rect(p, half, radius);
    let border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
    let two_pi = 6.28318530718;
    let duration = 1.6;
    let phase = globals._pad.x / duration;
    let pulse = 0.5 - 0.5 * cos(two_pi * phase);
    let alpha = border_alpha * pulse * in.color.a;
    return vec4<f32>(in.color.rgb, alpha);
}
if (kind == 2) {
    return vec4<f32>(in.color.rgb, glyph.r * in.color.a);
}
if (kind == 4 || kind == 5) {
    return vec4<f32>(icon.rgb * in.color.rgb, icon.a * in.color.a);
}
if (kind == 3) {
    return in.color;
}
return in.color;
}
"#;

const UI_GLOBALS_BIND_GROUP: BindGroupSpec = BindGroupSpec {
    group_index: 0,
    entries: &[
        BindGroupEntrySpec { binding: 0, visibility: VERTEX_FRAGMENT_STAGE, kind: BindingKind::UniformBuffer { dynamic_offset: false, min_size: None } },
        BindGroupEntrySpec { binding: 1, visibility: FRAGMENT_STAGE, kind: BindingKind::Texture2D },
        BindGroupEntrySpec { binding: 2, visibility: FRAGMENT_STAGE, kind: BindingKind::Sampler },
        BindGroupEntrySpec { binding: 3, visibility: FRAGMENT_STAGE, kind: BindingKind::Texture2D },
        BindGroupEntrySpec { binding: 4, visibility: FRAGMENT_STAGE, kind: BindingKind::Sampler },
    ],
};

const UI_VERTEX_BUFFERS: &[VertexBufferSpec] = &[
    VertexBufferSpec { stride: 8, step_mode: VertexStepMode::Vertex, attributes: &[VertexAttributeSpec { shader_location: 0, format: VertexFormat::Float32x2, offset: 0 }] },
    VertexBufferSpec {
        stride: 64,
        step_mode: VertexStepMode::Instance,
        attributes: &[
            VertexAttributeSpec { shader_location: 1, format: VertexFormat::Float32x4, offset: 0 },
            VertexAttributeSpec { shader_location: 2, format: VertexFormat::Float32x4, offset: 16 },
            VertexAttributeSpec { shader_location: 3, format: VertexFormat::Float32x4, offset: 32 },
            VertexAttributeSpec { shader_location: 4, format: VertexFormat::Float32x4, offset: 48 },
        ],
    },
];

/// 🩹️ The stencil-only silhouette mask pass: writes no color (`ColorWriteMask::None`), stamps the
/// clip region into the stencil buffer (`Always`/`Replace`) so `UI_CONTENT_PIPELINE` can clip
/// against it with an `Equal` test.
pub const UI_MASK_PIPELINE: PipelineSpec = PipelineSpec {
    label: "silhouette_mask_pipeline",
    vertex_entry: "vs_main",
    fragment_entry: "fs_main",
    vertex_buffers: UI_VERTEX_BUFFERS,
    bind_groups: &[UI_GLOBALS_BIND_GROUP],
    blend: BlendMode::None,
    color_write: ColorWriteMask::None,
    target: ColorTarget::SurfaceFormat,
    topology: PrimitiveTopology::TriangleList,
    cull_mode: CullMode::None,
    depth_stencil: Some(DepthStencilSpec {
        format: DepthStencilFormat::Depth24PlusStencil8,
        depth_write_enabled: false,
        depth_compare: CompareFunction::Always,
        stencil: StencilStateSpec { compare: CompareFunction::Always, fail_op: StencilOperation::Replace, depth_fail_op: StencilOperation::Replace, pass_op: StencilOperation::Replace, read_mask: 0xff, write_mask: 0xff },
        bias: NO_DEPTH_BIAS,
    }),
};

/// 🖌️ The content pass: alpha-blended, clipped against `UI_MASK_PIPELINE`'s stencil silhouette via
/// an `Equal` read (`write_mask: 0x00` — it reads the mask, never writes it).
pub const UI_CONTENT_PIPELINE: PipelineSpec = PipelineSpec {
    label: "ui_pipeline",
    vertex_entry: "vs_main",
    fragment_entry: "fs_main",
    vertex_buffers: UI_VERTEX_BUFFERS,
    bind_groups: &[UI_GLOBALS_BIND_GROUP],
    blend: BlendMode::AlphaBlending,
    color_write: ColorWriteMask::All,
    target: ColorTarget::SurfaceFormat,
    topology: PrimitiveTopology::TriangleList,
    cull_mode: CullMode::None,
    depth_stencil: Some(DepthStencilSpec {
        format: DepthStencilFormat::Depth24PlusStencil8,
        depth_write_enabled: false,
        depth_compare: CompareFunction::Always,
        stencil: StencilStateSpec { compare: CompareFunction::Equal, fail_op: StencilOperation::Keep, depth_fail_op: StencilOperation::Keep, pass_op: StencilOperation::Keep, read_mask: 0xff, write_mask: 0x00 },
        bias: NO_DEPTH_BIAS,
    }),
};

pub const UI_FAMILY: ShaderFamily = ShaderFamily { name: "ui", variants: &[ShaderVariant { name: "ui", wgsl: UI_SHADER, pipelines: &[UI_MASK_PIPELINE, UI_CONTENT_PIPELINE] }] };

//#endregion 🧵️UiFamily

//#region 🔺️VectorFamily

/// 📐️ Flat-colored triangles (selection boxes, guides, debug overlays). Repaired from the same
/// `async fn vs_main`/`async fn fs_main` corruption as `UI_SHADER`; no other damage found.
pub const VECTOR_SHADER: &str = r#"
struct Globals {
screen_size: vec2<f32>,
_pad: vec2<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;

struct VertexInput {
@location(0) position: vec2<f32>,
@location(1) color: vec4<f32>,
}

struct VertexOutput {
@builtin(position) clip_position: vec4<f32>,
@location(0) color: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
var out: VertexOutput;
let ndc = (vertex.position / globals.screen_size) * 2.0 - vec2<f32>(1.0, 1.0);
out.clip_position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0);
out.color = vertex.color;
return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
return in.color;
}
"#;

/// 🔺️ `wgpu-old`'s `vector_pipeline` layout reuses the UI globals bind group layout verbatim (it
/// carries the glyph/icon bindings this shader never samples) rather than a narrower dedicated one
/// — kept as-is here since backends must build the same layout the reference implementation does.
pub const VECTOR_PIPELINE: PipelineSpec = PipelineSpec {
    label: "vector_pipeline",
    vertex_entry: "vs_main",
    fragment_entry: "fs_main",
    vertex_buffers: &[VertexBufferSpec {
        stride: 24,
        step_mode: VertexStepMode::Vertex,
        attributes: &[VertexAttributeSpec { shader_location: 0, format: VertexFormat::Float32x2, offset: 0 }, VertexAttributeSpec { shader_location: 1, format: VertexFormat::Float32x4, offset: 8 }],
    }],
    bind_groups: &[UI_GLOBALS_BIND_GROUP],
    blend: BlendMode::AlphaBlending,
    color_write: ColorWriteMask::All,
    target: ColorTarget::SurfaceFormat,
    topology: PrimitiveTopology::TriangleList,
    cull_mode: CullMode::None,
    depth_stencil: Some(DepthStencilSpec {
        format: DepthStencilFormat::Depth24PlusStencil8,
        depth_write_enabled: false,
        depth_compare: CompareFunction::Always,
        stencil: StencilStateSpec { compare: CompareFunction::Equal, fail_op: StencilOperation::Keep, depth_fail_op: StencilOperation::Keep, pass_op: StencilOperation::Keep, read_mask: 0xff, write_mask: 0x00 },
        bias: NO_DEPTH_BIAS,
    }),
};

pub const VECTOR_FAMILY: ShaderFamily = ShaderFamily { name: "vector", variants: &[ShaderVariant { name: "vector", wgsl: VECTOR_SHADER, pipelines: &[VECTOR_PIPELINE] }] };

//#endregion 🔺️VectorFamily

//#region 🌐️World3dFamily

const WORLD_GLOBALS_BIND_GROUP: BindGroupSpec = BindGroupSpec { group_index: 0, entries: &[BindGroupEntrySpec { binding: 0, visibility: VERTEX_FRAGMENT_STAGE, kind: BindingKind::UniformBuffer { dynamic_offset: true, min_size: Some(240) } }] };
const WORLD_SHADOW_BIND_GROUP: BindGroupSpec =
    BindGroupSpec { group_index: 1, entries: &[BindGroupEntrySpec { binding: 0, visibility: FRAGMENT_STAGE, kind: BindingKind::DepthTexture2D }, BindGroupEntrySpec { binding: 1, visibility: FRAGMENT_STAGE, kind: BindingKind::ComparisonSampler }] };

const WORLD_CONTENT_STENCIL: StencilStateSpec = StencilStateSpec { compare: CompareFunction::Equal, fail_op: StencilOperation::Keep, depth_fail_op: StencilOperation::Keep, pass_op: StencilOperation::Keep, read_mask: 0xff, write_mask: 0x00 };

/// 🗻️ Lit mesh instances with the World3d ambient/sun and neutral standard-material contract.
pub const WORLD3D_SHADER: &str = r#"
struct Globals {
view_proj: mat4x4<f32>,
shadow_view_proj: mat4x4<f32>,
camera_position: vec4<f32>,
light_dir: vec4<f32>,
ambient: vec4<f32>,
sun: vec4<f32>,
material: vec4<f32>,
material_emissive: vec4<f32>,
shadow: vec4<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var shadow_map: texture_depth_2d;
@group(1) @binding(1) var shadow_sampler: sampler_comparison;

struct VertexInput {
@location(0) position: vec3<f32>,
@location(1) normal: vec3<f32>,
@location(2) color: vec4<f32>,
}

struct InstanceInput {
@location(3) model0: vec4<f32>,
@location(4) model1: vec4<f32>,
@location(5) model2: vec4<f32>,
@location(6) model3: vec4<f32>,
@location(7) color: vec4<f32>,
@location(8) flags: vec4<f32>,
}

struct VertexOutput {
@builtin(position) clip_position: vec4<f32>,
@location(0) color: vec4<f32>,
@location(1) normal: vec3<f32>,
@location(2) flags: vec4<f32>,
@location(3) world_position: vec3<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
var out: VertexOutput;
let model = mat4x4<f32>(instance.model0, instance.model1, instance.model2, instance.model3);
let world_pos = model * vec4<f32>(vertex.position, 1.0);
out.clip_position = globals.view_proj * world_pos;
let model_linear = mat3x3<f32>(model[0].xyz, model[1].xyz, model[2].xyz);
let cofactor = mat3x3<f32>(cross(model[1].xyz, model[2].xyz), cross(model[2].xyz, model[0].xyz), cross(model[0].xyz, model[1].xyz));
let determinant = dot(model[0].xyz, cofactor[0]);
var transformed_normal = model_linear * vertex.normal;
if (abs(determinant) > 0.0000001) {
    transformed_normal = (cofactor * vertex.normal) / determinant;
}
let transformed_length_squared = dot(transformed_normal, transformed_normal);
if (!(transformed_length_squared > 0.000000000001 && transformed_length_squared < 3.402823e38)) {
    transformed_normal = vertex.normal;
}
out.normal = normalize(transformed_normal);
let vertex_weight = f32(u32(instance.flags.x) & 1u);
out.color = vec4<f32>(instance.color.rgb * mix(vec3<f32>(1.0), vertex.color.rgb, vertex_weight), instance.color.a * mix(1.0, vertex.color.a, vertex_weight));
out.flags = instance.flags;
out.world_position = world_pos.xyz;
return out;
}

@vertex
fn vs_shadow(vertex: VertexInput, instance: InstanceInput) -> @builtin(position) vec4<f32> {
let model = mat4x4<f32>(instance.model0, instance.model1, instance.model2, instance.model3);
return globals.shadow_view_proj * model * vec4<f32>(vertex.position, 1.0);
}

const WORLD3D_RECIPROCAL_PI: f32 = 0.31830989;
const WORLD3D_HEMISPHERE_INTENSITY: f32 = 1.35;
const WORLD3D_HEMISPHERE_GROUND: vec3<f32> = vec3<f32>(0.32314321, 0.3515326, 0.40724021);
const WORLD3D_DFG_LUT: array<vec2<f32>, 256> = array<vec2<f32>, 256>(
    vec2<f32>(0.147094727, 0.852050781), vec2<f32>(0.165527344, 0.787597656), vec2<f32>(0.244384766, 0.638671875), vec2<f32>(0.370849609, 0.51953125),
    vec2<f32>(0.496826172, 0.415527344), vec2<f32>(0.602050781, 0.326416016), vec2<f32>(0.684082031, 0.25390625), vec2<f32>(0.74609375, 0.197509766),
    vec2<f32>(0.790527344, 0.154296875), vec2<f32>(0.822265625, 0.121643066), vec2<f32>(0.843261719, 0.0969848633), vec2<f32>(0.856445312, 0.0784301758),
    vec2<f32>(0.86328125, 0.0643920898), vec2<f32>(0.865722656, 0.0537109375), vec2<f32>(0.864257812, 0.0454406738), vec2<f32>(0.859863281, 0.0390319824),
    vec2<f32>(0.388671875, 0.611328125), vec2<f32>(0.393066406, 0.600585938), vec2<f32>(0.412353516, 0.545898438), vec2<f32>(0.456542969, 0.448242188),
    vec2<f32>(0.527832031, 0.352539062), vec2<f32>(0.607421875, 0.273925781), vec2<f32>(0.678710938, 0.211425781), vec2<f32>(0.733398438, 0.162597656),
    vec2<f32>(0.770996094, 0.125366211), vec2<f32>(0.793457031, 0.0972900391), vec2<f32>(0.803222656, 0.0762329102), vec2<f32>(0.803710938, 0.0603637695),
    vec2<f32>(0.796386719, 0.0484313965), vec2<f32>(0.785644531, 0.0393676758), vec2<f32>(0.771972656, 0.032409668), vec2<f32>(0.754882812, 0.0269775391),
    vec2<f32>(0.572265625, 0.427490234), vec2<f32>(0.573730469, 0.424072266), vec2<f32>(0.579589844, 0.403564453), vec2<f32>(0.591796875, 0.354492188),
    vec2<f32>(0.616210938, 0.288085938), vec2<f32>(0.655273438, 0.224853516), vec2<f32>(0.698730469, 0.172607422), vec2<f32>(0.735351562, 0.131835938),
    vec2<f32>(0.759277344, 0.100891113), vec2<f32>(0.770019531, 0.0774536133), vec2<f32>(0.771972656, 0.0599365234), vec2<f32>(0.766113281, 0.0468444824),
    vec2<f32>(0.751953125, 0.0369873047), vec2<f32>(0.732421875, 0.0295410156), vec2<f32>(0.709472656, 0.0238342285), vec2<f32>(0.68359375, 0.0194396973),
    vec2<f32>(0.708984375, 0.291015625), vec2<f32>(0.708984375, 0.289794922), vec2<f32>(0.709960938, 0.28125), vec2<f32>(0.709960938, 0.258544922),
    vec2<f32>(0.711425781, 0.220458984), vec2<f32>(0.719726562, 0.176879883), vec2<f32>(0.734375, 0.137084961), vec2<f32>(0.748046875, 0.104797363),
    vec2<f32>(0.755859375, 0.0798950195), vec2<f32>(0.759765625, 0.0610046387), vec2<f32>(0.753417969, 0.0468444824), vec2<f32>(0.738769531, 0.0362243652),
    vec2<f32>(0.717773438, 0.0282592773), vec2<f32>(0.691894531, 0.0222625732), vec2<f32>(0.661132812, 0.0177001953), vec2<f32>(0.628417969, 0.0141983032),
    vec2<f32>(0.808105469, 0.191772461), vec2<f32>(0.807617188, 0.19128418), vec2<f32>(0.806152344, 0.187988281), vec2<f32>(0.801757812, 0.178100586),
    vec2<f32>(0.79296875, 0.158691406), vec2<f32>(0.783691406, 0.132202148), vec2<f32>(0.775390625, 0.104858398), vec2<f32>(0.768554688, 0.0811157227),
    vec2<f32>(0.764648438, 0.0619812012), vec2<f32>(0.755371094, 0.0472717285), vec2<f32>(0.740234375, 0.0361633301), vec2<f32>(0.71875, 0.0277557373),
    vec2<f32>(0.690917969, 0.021484375), vec2<f32>(0.658203125, 0.0167388916), vec2<f32>(0.622070312, 0.0131607056), vec2<f32>(0.583984375, 0.0104141235),
    vec2<f32>(0.878417969, 0.121704102), vec2<f32>(0.877929688, 0.121704102), vec2<f32>(0.875, 0.120605469), vec2<f32>(0.869140625, 0.116943359),
    vec2<f32>(0.856933594, 0.108032227), vec2<f32>(0.837890625, 0.09375), vec2<f32>(0.814941406, 0.0769042969), vec2<f32>(0.795898438, 0.0606994629),
    vec2<f32>(0.776367188, 0.046875), vec2<f32>(0.756347656, 0.0359191895), vec2<f32>(0.732421875, 0.0274505615), vec2<f32>(0.703125, 0.0210266113),
    vec2<f32>(0.668945312, 0.0161743164), vec2<f32>(0.630371094, 0.012512207), vec2<f32>(0.589355469, 0.00974273682), vec2<f32>(0.546386719, 0.00763320923),
    vec2<f32>(0.926269531, 0.0737915039), vec2<f32>(0.92578125, 0.0739135742), vec2<f32>(0.922851562, 0.0739135742), vec2<f32>(0.916992188, 0.0731201172),
    vec2<f32>(0.903808594, 0.0698242188), vec2<f32>(0.881347656, 0.0631103516), vec2<f32>(0.851074219, 0.0538024902), vec2<f32>(0.821289062, 0.0437011719),
    vec2<f32>(0.791015625, 0.0343933105), vec2<f32>(0.761230469, 0.0266113281), vec2<f32>(0.728027344, 0.0204467773), vec2<f32>(0.691894531, 0.0156555176),
    vec2<f32>(0.650878906, 0.012008667), vec2<f32>(0.607421875, 0.00925445557), vec2<f32>(0.561035156, 0.00715637207), vec2<f32>(0.514160156, 0.00556564331),
    vec2<f32>(0.957519531, 0.0423278809), vec2<f32>(0.95703125, 0.0424499512), vec2<f32>(0.954589844, 0.0428161621), vec2<f32>(0.94921875, 0.043182373),
    vec2<f32>(0.937011719, 0.0426635742), vec2<f32>(0.913085938, 0.0402526855), vec2<f32>(0.881835938, 0.0357971191), vec2<f32>(0.844726562, 0.0301361084),
    vec2<f32>(0.806152344, 0.0243377686), vec2<f32>(0.767089844, 0.0191497803), vec2<f32>(0.7265625, 0.0148544312), vec2<f32>(0.682617188, 0.0114212036),
    vec2<f32>(0.636230469, 0.00877380371), vec2<f32>(0.586914062, 0.00674057007), vec2<f32>(0.536621094, 0.00519943237), vec2<f32>(0.486083984, 0.00402069092),
    vec2<f32>(0.977539062, 0.0226287842), vec2<f32>(0.977050781, 0.0227508545), vec2<f32>(0.975097656, 0.0231933594), vec2<f32>(0.969726562, 0.0239105225),
    vec2<f32>(0.959472656, 0.0244903564), vec2<f32>(0.936035156, 0.0241241455), vec2<f32>(0.905273438, 0.0225219727), vec2<f32>(0.865234375, 0.0197601318),
    vec2<f32>(0.821777344, 0.0165100098), vec2<f32>(0.774414062, 0.0132904053), vec2<f32>(0.7265625, 0.0104598999), vec2<f32>(0.676269531, 0.00813293457),
    vec2<f32>(0.624023438, 0.0062789917), vec2<f32>(0.569824219, 0.00482559204), vec2<f32>(0.515136719, 0.00371360779), vec2<f32>(0.461425781, 0.0028629303),
    vec2<f32>(0.988769531, 0.0110702515), vec2<f32>(0.988769531, 0.0111618042), vec2<f32>(0.986816406, 0.0115127563), vec2<f32>(0.982910156, 0.0122146606),
    vec2<f32>(0.973144531, 0.0129928589), vec2<f32>(0.953125, 0.0135192871), vec2<f32>(0.922851562, 0.0132827759), vec2<f32>(0.882324219, 0.012260437),
    vec2<f32>(0.834960938, 0.0106582642), vec2<f32>(0.783203125, 0.00885009766), vec2<f32>(0.728515625, 0.0071144104), vec2<f32>(0.671875, 0.00560760498),
    vec2<f32>(0.613769531, 0.00436019897), vec2<f32>(0.5546875, 0.00337219238), vec2<f32>(0.496337891, 0.00259971619), vec2<f32>(0.439453125, 0.00200462341),
    vec2<f32>(0.995117188, 0.00479888916), vec2<f32>(0.995117188, 0.00486373901), vec2<f32>(0.993652344, 0.00509643555), vec2<f32>(0.990234375, 0.00560379028),
    vec2<f32>(0.981445312, 0.00633239746), vec2<f32>(0.964355469, 0.0069770813), vec2<f32>(0.936035156, 0.00729751587), vec2<f32>(0.896484375, 0.00712585449),
    vec2<f32>(0.846679688, 0.00649261475), vec2<f32>(0.791503906, 0.00559616089), vec2<f32>(0.731445312, 0.00462722778), vec2<f32>(0.668945312, 0.00371742249),
    vec2<f32>(0.60546875, 0.0029296875), vec2<f32>(0.541503906, 0.00228118896), vec2<f32>(0.479248047, 0.00176620483), vec2<f32>(0.419677734, 0.00136566162),
    vec2<f32>(0.998046875, 0.00176048279), vec2<f32>(0.998046875, 0.00179386139), vec2<f32>(0.996582031, 0.00192928314), vec2<f32>(0.994140625, 0.00223922729),
    vec2<f32>(0.986328125, 0.00272941589), vec2<f32>(0.971679688, 0.00325012207), vec2<f32>(0.945800781, 0.00366973877), vec2<f32>(0.907714844, 0.00381851196),
    vec2<f32>(0.858398438, 0.00368118286), vec2<f32>(0.799316406, 0.00332069397), vec2<f32>(0.735351562, 0.00284385681), vec2<f32>(0.667480469, 0.00234413147),
    vec2<f32>(0.598632812, 0.00187969208), vec2<f32>(0.530273438, 0.00148296356), vec2<f32>(0.464111328, 0.00115871429), vec2<f32>(0.402099609, 0.00089931488),
    vec2<f32>(0.999511719, 0.000501155853), vec2<f32>(0.999511719, 0.000515460968), vec2<f32>(0.998046875, 0.000583648682), vec2<f32>(0.996582031, 0.000750541687),
    vec2<f32>(0.989257812, 0.00101470947), vec2<f32>(0.976074219, 0.00134658813), vec2<f32>(0.952636719, 0.00165271759), vec2<f32>(0.916015625, 0.00185585022),
    vec2<f32>(0.8671875, 0.00190544128), vec2<f32>(0.807617188, 0.00181674957), vec2<f32>(0.739257812, 0.00162124634), vec2<f32>(0.666992188, 0.00137996674),
    vec2<f32>(0.593261719, 0.00113582611), vec2<f32>(0.520019531, 0.000912189484), vec2<f32>(0.450439453, 0.000721931458), vec2<f32>(0.385986328, 0.000565052032),
    vec2<f32>(1.0, 9.31620598e-05), vec2<f32>(1.0, 9.78708267e-05), vec2<f32>(0.999023438, 0.000125408173), vec2<f32>(0.997070312, 0.000192165375),
    vec2<f32>(0.990722656, 0.00031042099), vec2<f32>(0.979003906, 0.000469923019), vec2<f32>(0.957519531, 0.000647068024), vec2<f32>(0.923339844, 0.000791549683),
    vec2<f32>(0.875488281, 0.000876903534), vec2<f32>(0.814941406, 0.000886917114), vec2<f32>(0.744140625, 0.000832557678), vec2<f32>(0.667480469, 0.000738620758),
    vec2<f32>(0.588378906, 0.000626564026), vec2<f32>(0.511230469, 0.000516891479), vec2<f32>(0.438232422, 0.000416517258), vec2<f32>(0.37109375, 0.000331163406),
    vec2<f32>(1.0, 7.27176666e-06), vec2<f32>(1.0, 8.16583633e-06), vec2<f32>(0.999023438, 1.69873238e-05), vec2<f32>(0.997558594, 3.79085541e-05),
    vec2<f32>(0.9921875, 7.59363174e-05), vec2<f32>(0.981445312, 0.000137448311), vec2<f32>(0.961425781, 0.000207543373), vec2<f32>(0.929199219, 0.00028014183),
    vec2<f32>(0.8828125, 0.000334501266), vec2<f32>(0.821777344, 0.000362634659), vec2<f32>(0.749023438, 0.000362157822), vec2<f32>(0.668457031, 0.000338077545),
    vec2<f32>(0.585449219, 0.000299692154), vec2<f32>(0.50390625, 0.000255823135), vec2<f32>(0.427001953, 0.000211715698), vec2<f32>(0.357666016, 0.000172019005),
    vec2<f32>(1.0, 0.0), vec2<f32>(1.0, 5.96046448e-08), vec2<f32>(0.999511719, 1.25169754e-06), vec2<f32>(0.997558594, 5.30481339e-06),
    vec2<f32>(0.993164062, 1.50799751e-05), vec2<f32>(0.982910156, 2.85506248e-05), vec2<f32>(0.964355469, 4.74452972e-05), vec2<f32>(0.934082031, 6.84261322e-05),
    vec2<f32>(0.889160156, 8.893013e-05), vec2<f32>(0.828125, 0.000104248524), vec2<f32>(0.75390625, 0.000112175941), vec2<f32>(0.670410156, 0.00011241436),
    vec2<f32>(0.583007812, 0.000106275082), vec2<f32>(0.497070312, 9.58442688e-05), vec2<f32>(0.416992188, 8.33272934e-05), vec2<f32>(0.345214844, 7.05122948e-05),
);

fn world3d_dfg(roughness: f32, dot_normal: f32) -> vec2<f32> {
    let coordinate = clamp(vec2<f32>(roughness, dot_normal), vec2<f32>(0.0), vec2<f32>(1.0)) * 16.0 - vec2<f32>(0.5);
    let lower = floor(coordinate);
    let fraction = coordinate - lower;
    let x0 = u32(clamp(lower.x, 0.0, 15.0));
    let y0 = u32(clamp(lower.y, 0.0, 15.0));
    let x1 = u32(clamp(lower.x + 1.0, 0.0, 15.0));
    let y1 = u32(clamp(lower.y + 1.0, 0.0, 15.0));
    let a = mix(WORLD3D_DFG_LUT[y0 * 16u + x0], WORLD3D_DFG_LUT[y0 * 16u + x1], fraction.x);
    let b = mix(WORLD3D_DFG_LUT[y1 * 16u + x0], WORLD3D_DFG_LUT[y1 * 16u + x1], fraction.x);
    return mix(a, b, fraction.y);
}

fn world3d_f_schlick(f0: vec3<f32>, dot_view_half: f32) -> vec3<f32> {
    let fresnel = exp2((-5.55473 * dot_view_half - 6.98316) * dot_view_half);
    return f0 * (1.0 - fresnel) + vec3<f32>(fresnel);
}

fn world3d_v_ggx_smith_correlated(alpha: f32, dot_normal_light: f32, dot_normal_view: f32) -> f32 {
    let alpha_squared = alpha * alpha;
    let gv = dot_normal_light * sqrt(alpha_squared + (1.0 - alpha_squared) * dot_normal_view * dot_normal_view);
    let gl = dot_normal_view * sqrt(alpha_squared + (1.0 - alpha_squared) * dot_normal_light * dot_normal_light);
    return 0.5 / max(gv + gl, 0.000001);
}

fn world3d_d_ggx(alpha: f32, dot_normal_half: f32) -> f32 {
    let alpha_squared = alpha * alpha;
    let denominator = dot_normal_half * dot_normal_half * (alpha_squared - 1.0) + 1.0;
    return WORLD3D_RECIPROCAL_PI * alpha_squared / (denominator * denominator);
}

fn world3d_brdf_ggx(light: vec3<f32>, view: vec3<f32>, normal: vec3<f32>, f0: vec3<f32>, roughness: f32) -> vec3<f32> {
    let half_direction = normalize(light + view);
    let dot_normal_light = clamp(dot(normal, light), 0.0, 1.0);
    let dot_normal_view = clamp(dot(normal, view), 0.0, 1.0);
    let dot_normal_half = clamp(dot(normal, half_direction), 0.0, 1.0);
    let dot_view_half = clamp(dot(view, half_direction), 0.0, 1.0);
    let alpha = roughness * roughness;
    return world3d_f_schlick(f0, dot_view_half) * (world3d_v_ggx_smith_correlated(alpha, dot_normal_light, dot_normal_view) * world3d_d_ggx(alpha, dot_normal_half));
}

fn world3d_brdf_ggx_multiscatter(light: vec3<f32>, view: vec3<f32>, normal: vec3<f32>, f0: vec3<f32>, roughness: f32) -> vec3<f32> {
    let single_scatter = world3d_brdf_ggx(light, view, normal, f0, roughness);
    let dot_normal_light = clamp(dot(normal, light), 0.0, 1.0);
    let dot_normal_view = clamp(dot(normal, view), 0.0, 1.0);
    let dfg_view = world3d_dfg(roughness, dot_normal_view);
    let dfg_light = world3d_dfg(roughness, dot_normal_light);
    let fss_ess_view = f0 * dfg_view.x + vec3<f32>(dfg_view.y);
    let fss_ess_light = f0 * dfg_light.x + vec3<f32>(dfg_light.y);
    let ems_view = 1.0 - dfg_view.x - dfg_view.y;
    let ems_light = 1.0 - dfg_light.x - dfg_light.y;
    let fresnel_average = f0 + (vec3<f32>(1.0) - f0) * 0.047619;
    let multiple_scatter = fss_ess_view * fss_ess_light * fresnel_average / (vec3<f32>(1.0) - ems_view * ems_light * fresnel_average + vec3<f32>(0.000001));
    return single_scatter + multiple_scatter * (ems_view * ems_light);
}

fn world3d_direct_light(n: vec3<f32>, v: vec3<f32>, l: vec3<f32>, radiance: vec3<f32>, base_color: vec3<f32>, metalness: f32, roughness: f32) -> vec3<f32> {
    let dot_normal_light = max(dot(n, l), 0.0);
    let irradiance = radiance * dot_normal_light;
    let f0 = mix(vec3<f32>(0.04), base_color, metalness);
    let diffuse = base_color * (1.0 - metalness) * WORLD3D_RECIPROCAL_PI;
    return irradiance * (diffuse + world3d_brdf_ggx_multiscatter(l, v, n, f0, roughness));
}

fn world3d_rrt_and_odt_fit(value: vec3<f32>) -> vec3<f32> {
    let a = value * (value + vec3<f32>(0.0245786)) - vec3<f32>(0.000090537);
    let b = value * (vec3<f32>(0.983729) * value + vec3<f32>(0.432951)) + vec3<f32>(0.238081);
    return a / b;
}

fn world3d_output_transform(linear_rgb: vec3<f32>) -> vec3<f32> {
    let input_matrix = mat3x3<f32>(vec3<f32>(0.59719, 0.07600, 0.02840), vec3<f32>(0.35458, 0.90834, 0.13383), vec3<f32>(0.04823, 0.01566, 0.83777));
    let output_matrix = mat3x3<f32>(vec3<f32>(1.60475, -0.10208, -0.00327), vec3<f32>(-0.53108, 1.10813, -0.07276), vec3<f32>(-0.07367, -0.00605, 1.07602));
    let fitted = world3d_rrt_and_odt_fit(input_matrix * (linear_rgb / 0.6));
    return clamp(output_matrix * fitted, vec3<f32>(0.0), vec3<f32>(1.0));
}

fn world3d_linear_to_srgb(linear_rgb: vec3<f32>) -> vec3<f32> {
    let high = pow(linear_rgb, vec3<f32>(0.41666)) * 1.055 - vec3<f32>(0.055);
    let low = linear_rgb * 12.92;
    return select(high, low, linear_rgb <= vec3<f32>(0.0031308));
}

fn world3d_attachment_output(linear_rgb: vec3<f32>) -> vec3<f32> {
    let display_linear = world3d_output_transform(linear_rgb);
    return mix(display_linear, world3d_linear_to_srgb(display_linear), clamp(globals.shadow.w, 0.0, 1.0));
}

fn world3d_interleaved_gradient_noise(position: vec2<f32>) -> f32 {
return fract(52.9829189 * fract(dot(position, vec2<f32>(0.06711056, 0.00583715))));
}

fn world3d_vogel_disk_sample(index: u32, count: u32, rotation: f32) -> vec2<f32> {
let radius = sqrt((f32(index) + 0.5) / f32(count));
let angle = f32(index) * 2.399963229728653 + rotation;
return vec2<f32>(cos(angle), sin(angle)) * radius;
}

fn world3d_shadow_visibility(world_position: vec3<f32>, fragment_position: vec2<f32>) -> f32 {
if (globals.shadow.x < 0.5) {
    return 1.0;
}
let clip = globals.shadow_view_proj * vec4<f32>(world_position, 1.0);
let ndc = clip.xyz / clip.w;
let uv = vec2<f32>(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0 || ndc.z > 1.0) {
    return 1.0;
}
let radius = 1.0 / f32(textureDimensions(shadow_map).x);
let rotation = world3d_interleaved_gradient_noise(fragment_position) * 6.28318530718;
var visibility = 0.0;
for (var index = 0u; index < 5u; index = index + 1u) {
    visibility = visibility + textureSampleCompareLevel(shadow_map, shadow_sampler, uv + world3d_vogel_disk_sample(index, 5u, rotation) * radius, ndc.z);
}
return visibility * 0.2;
}

fn world3d_lighting(n: vec3<f32>, v: vec3<f32>, base_color: vec3<f32>, metalness: f32, roughness: f32, shadow_visibility: f32) -> vec3<f32> {
var indirect = globals.ambient.rgb * globals.ambient.a;
var direct = vec3<f32>(0.0);
if (globals.material.w > 0.5) {
    direct = world3d_direct_light(n, v, normalize(globals.light_dir.xyz), globals.sun.rgb * globals.sun.a, base_color, metalness, roughness) * shadow_visibility;
} else {
    let hemisphere = 0.5 * dot(n, vec3<f32>(0.0, 0.0, 1.0)) + 0.5;
    indirect = indirect + mix(WORLD3D_HEMISPHERE_GROUND, vec3<f32>(1.0), hemisphere) * WORLD3D_HEMISPHERE_INTENSITY;
    direct = direct + world3d_direct_light(n, v, normalize(vec3<f32>(12.0, 18.0, 10.0)), vec3<f32>(2.4), base_color, metalness, roughness);
    direct = direct + world3d_direct_light(n, v, normalize(vec3<f32>(-14.0, -10.0, 6.0)), vec3<f32>(1.2), base_color, metalness, roughness);
    direct = direct + world3d_direct_light(n, v, normalize(vec3<f32>(0.0, 0.0, -16.0)), vec3<f32>(0.75), base_color, metalness, roughness);
}
return indirect * base_color * (1.0 - metalness) * WORLD3D_RECIPROCAL_PI + direct;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
let n = normalize(in.normal);
let v = normalize(globals.camera_position.xyz - in.world_position);
let metalness = clamp(in.flags.z, 0.0, 1.0);
let normal_derivative = max(abs(dpdx(in.normal)), abs(dpdy(in.normal)));
let geometry_roughness = max(max(normal_derivative.x, normal_derivative.y), normal_derivative.z);
let roughness = min(max(in.flags.w, 0.0525) + geometry_roughness, 1.0);
let emissive = globals.material_emissive.rgb * globals.material.z + in.color.rgb * max(in.flags.y, 0.0);
var shadow_visibility = 1.0;
if ((u32(in.flags.x) & 2u) != 0u) {
    shadow_visibility = world3d_shadow_visibility(in.world_position, in.clip_position.xy);
}
let color = world3d_lighting(n, v, in.color.rgb, metalness, roughness, shadow_visibility) + emissive;
return vec4<f32>(world3d_attachment_output(color), in.color.a);
}
"#;

const WORLD3D_MESH_VERTEX_BUFFERS: &[VertexBufferSpec] = &[
    VertexBufferSpec {
        stride: 40,
        step_mode: VertexStepMode::Vertex,
        attributes: &[
            VertexAttributeSpec { shader_location: 0, format: VertexFormat::Float32x3, offset: 0 },
            VertexAttributeSpec { shader_location: 1, format: VertexFormat::Float32x3, offset: 12 },
            VertexAttributeSpec { shader_location: 2, format: VertexFormat::Float32x4, offset: 24 },
        ],
    },
    VertexBufferSpec {
        stride: 96,
        step_mode: VertexStepMode::Instance,
        attributes: &[
            VertexAttributeSpec { shader_location: 3, format: VertexFormat::Float32x4, offset: 0 },
            VertexAttributeSpec { shader_location: 4, format: VertexFormat::Float32x4, offset: 16 },
            VertexAttributeSpec { shader_location: 5, format: VertexFormat::Float32x4, offset: 32 },
            VertexAttributeSpec { shader_location: 6, format: VertexFormat::Float32x4, offset: 48 },
            VertexAttributeSpec { shader_location: 7, format: VertexFormat::Float32x4, offset: 64 },
            VertexAttributeSpec { shader_location: 8, format: VertexFormat::Float32x4, offset: 80 },
        ],
    },
];

/// 🧱️ Opaque mesh pass: full depth write, `Less` test.
pub const WORLD3D_OPAQUE_PIPELINE: PipelineSpec = PipelineSpec {
    label: "world3d_pipeline",
    vertex_entry: "vs_main",
    fragment_entry: "fs_main",
    vertex_buffers: WORLD3D_MESH_VERTEX_BUFFERS,
    bind_groups: &[WORLD_GLOBALS_BIND_GROUP, WORLD_SHADOW_BIND_GROUP],
    blend: BlendMode::Replace,
    color_write: ColorWriteMask::All,
    target: ColorTarget::SurfaceFormat,
    topology: PrimitiveTopology::TriangleList,
    cull_mode: CullMode::None,
    depth_stencil: Some(DepthStencilSpec { format: DepthStencilFormat::Depth24PlusStencil8, depth_write_enabled: true, depth_compare: CompareFunction::Less, stencil: WORLD_CONTENT_STENCIL, bias: NO_DEPTH_BIAS }),
};

/// 🫧️ Translucent mesh pass: reads but never writes depth (`LessEqual`, `depth_write_enabled:
/// false`), back-face culled, biased `constant: -2, slope_scale: -1.0` to fight z-fighting against
/// the opaque pass underneath.
pub const WORLD3D_TRANSLUCENT_PIPELINE: PipelineSpec = PipelineSpec {
    label: "world3d_pipeline_translucent",
    vertex_entry: "vs_main",
    fragment_entry: "fs_main",
    vertex_buffers: WORLD3D_MESH_VERTEX_BUFFERS,
    bind_groups: &[WORLD_GLOBALS_BIND_GROUP, WORLD_SHADOW_BIND_GROUP],
    blend: BlendMode::AlphaBlending,
    color_write: ColorWriteMask::All,
    target: ColorTarget::SurfaceFormat,
    topology: PrimitiveTopology::TriangleList,
    cull_mode: CullMode::Back,
    depth_stencil: Some(DepthStencilSpec {
        format: DepthStencilFormat::Depth24PlusStencil8,
        depth_write_enabled: false,
        depth_compare: CompareFunction::LessEqual,
        stencil: WORLD_CONTENT_STENCIL,
        bias: DepthBiasSpec { constant: -2, slope_scale: -1.0, clamp: 0.0 },
    }),
};

/// ➰️ Unlit colored line segments (gizmos, wireframes) sharing the mesh pass's globals uniform.
/// Repaired from the `async` corruption; no other damage found.
pub const WORLD3D_LINES_SHADER: &str = r#"
struct Globals {
view_proj: mat4x4<f32>,
shadow_view_proj: mat4x4<f32>,
camera_position: vec4<f32>,
light_dir: vec4<f32>,
ambient: vec4<f32>,
sun: vec4<f32>,
material: vec4<f32>,
material_emissive: vec4<f32>,
shadow: vec4<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;

struct VertexInput {
@location(0) position: vec3<f32>,
@location(1) color: vec4<f32>,
}

struct VertexOutput {
@builtin(position) clip_position: vec4<f32>,
@location(0) color: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
var out: VertexOutput;
out.clip_position = globals.view_proj * vec4<f32>(vertex.position, 1.0);
out.color = vertex.color;
return out;
}

fn world3d_rrt_and_odt_fit(value: vec3<f32>) -> vec3<f32> {
let a = value * (value + vec3<f32>(0.0245786)) - vec3<f32>(0.000090537);
let b = value * (vec3<f32>(0.983729) * value + vec3<f32>(0.432951)) + vec3<f32>(0.238081);
return a / b;
}

fn world3d_output_transform(linear_rgb: vec3<f32>) -> vec3<f32> {
let input_matrix = mat3x3<f32>(vec3<f32>(0.59719, 0.07600, 0.02840), vec3<f32>(0.35458, 0.90834, 0.13383), vec3<f32>(0.04823, 0.01566, 0.83777));
let output_matrix = mat3x3<f32>(vec3<f32>(1.60475, -0.10208, -0.00327), vec3<f32>(-0.53108, 1.10813, -0.07276), vec3<f32>(-0.07367, -0.00605, 1.07602));
return clamp(output_matrix * world3d_rrt_and_odt_fit(input_matrix * (linear_rgb / 0.6)), vec3<f32>(0.0), vec3<f32>(1.0));
}

fn world3d_linear_to_srgb(linear_rgb: vec3<f32>) -> vec3<f32> {
let high = pow(linear_rgb, vec3<f32>(0.41666)) * 1.055 - vec3<f32>(0.055);
let low = linear_rgb * 12.92;
return select(high, low, linear_rgb <= vec3<f32>(0.0031308));
}

fn world3d_attachment_output(linear_rgb: vec3<f32>) -> vec3<f32> {
let display_linear = world3d_output_transform(linear_rgb);
return mix(display_linear, world3d_linear_to_srgb(display_linear), clamp(globals.shadow.w, 0.0, 1.0));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
return vec4<f32>(world3d_attachment_output(in.color.rgb), in.color.a);
}
"#;

/// ➖️ Line-list topology, alpha-blended, depth-tested but not depth-writing (drawn over the mesh
/// passes without occluding them).
pub const WORLD3D_LINE_PIPELINE: PipelineSpec = PipelineSpec {
    label: "world3d_line_pipeline",
    vertex_entry: "vs_main",
    fragment_entry: "fs_main",
    vertex_buffers: &[VertexBufferSpec {
        stride: 28,
        step_mode: VertexStepMode::Vertex,
        attributes: &[VertexAttributeSpec { shader_location: 0, format: VertexFormat::Float32x3, offset: 0 }, VertexAttributeSpec { shader_location: 1, format: VertexFormat::Float32x4, offset: 12 }],
    }],
    bind_groups: &[WORLD_GLOBALS_BIND_GROUP],
    blend: BlendMode::AlphaBlending,
    color_write: ColorWriteMask::All,
    target: ColorTarget::SurfaceFormat,
    topology: PrimitiveTopology::LineList,
    cull_mode: CullMode::None,
    depth_stencil: Some(DepthStencilSpec { format: DepthStencilFormat::Depth24PlusStencil8, depth_write_enabled: false, depth_compare: CompareFunction::LessEqual, stencil: WORLD_CONTENT_STENCIL, bias: NO_DEPTH_BIAS }),
};

/// 🖼️ Textured world quad (position + uv, per-instance model + Basic appearance) — the reference underlay every
/// `ScenePass3d::textured_draws` entry carries. Built as `world3d_textured_pipeline` in
/// `🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs` since ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W5c; the
/// spec below is read off that construction, not inferred.
pub const WORLD3D_TEXTURED_SHADER: &str = r#"struct Globals {
view_proj: mat4x4<f32>,
shadow_view_proj: mat4x4<f32>,
camera_position: vec4<f32>,
light_dir: vec4<f32>,
ambient: vec4<f32>,
sun: vec4<f32>,
material: vec4<f32>,
material_emissive: vec4<f32>,
shadow: vec4<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var tex: texture_2d<f32>;
@group(1) @binding(1) var tex_sampler: sampler;

struct VertexInput {
@location(0) position: vec3<f32>,
@location(1) uv: vec2<f32>,
}

struct InstanceInput {
@location(3) model0: vec4<f32>,
@location(4) model1: vec4<f32>,
@location(5) model2: vec4<f32>,
@location(6) model3: vec4<f32>,
@location(7) background: vec4<f32>,
@location(8) appearance: vec4<f32>,
}

struct VertexOutput {
@builtin(position) clip_position: vec4<f32>,
@location(0) uv: vec2<f32>,
@location(1) background: vec4<f32>,
@location(2) appearance: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
var out: VertexOutput;
let model = mat4x4<f32>(instance.model0, instance.model1, instance.model2, instance.model3);
let world_pos = model * vec4<f32>(vertex.position, 1.0);
out.clip_position = globals.view_proj * world_pos;
out.uv = vertex.uv;
out.background = instance.background;
out.appearance = instance.appearance;
return out;
}

fn world3d_linear_to_srgb(linear_rgb: vec3<f32>) -> vec3<f32> {
let high = pow(linear_rgb, vec3<f32>(0.41666)) * 1.055 - vec3<f32>(0.055);
let low = linear_rgb * 12.92;
return select(high, low, linear_rgb <= vec3<f32>(0.0031308));
}

fn world3d_basic_attachment_output(linear_rgb: vec3<f32>) -> vec3<f32> {
return mix(linear_rgb, world3d_linear_to_srgb(linear_rgb), clamp(globals.shadow.w, 0.0, 1.0));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
let sampled = textureSample(tex, tex_sampler, in.uv);
let source_linear = select(world3d_linear_to_srgb(sampled.rgb), sampled.rgb, in.appearance.y >= 0.5);
let content_rgb = world3d_basic_attachment_output(source_linear);
let background_rgb = world3d_basic_attachment_output(in.background.rgb);
let content_alpha = clamp(sampled.a * in.appearance.x, 0.0, 1.0);
let background_alpha = clamp(in.background.a, 0.0, 1.0);
let alpha = content_alpha + background_alpha * (1.0 - content_alpha);
let premultiplied = content_rgb * content_alpha + background_rgb * background_alpha * (1.0 - content_alpha);
return vec4<f32>(premultiplied / max(alpha, 0.000001), alpha);
}
"#;

/// 🖼️ A Basic-material plane, so it blends and never writes depth. Its instance carries the
/// reference background, content opacity, and source transfer policy; `cull_mode: None` is React's
/// `side={DoubleSide}` and `ALPHA_BLENDING` composites its encoded output over the scene.
pub const WORLD3D_TEXTURED_PIPELINE: PipelineSpec = PipelineSpec {
    label: "world3d_textured_pipeline",
    vertex_entry: "vs_main",
    fragment_entry: "fs_main",
    vertex_buffers: &[
        VertexBufferSpec {
            stride: 20,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttributeSpec { shader_location: 0, format: VertexFormat::Float32x3, offset: 0 }, VertexAttributeSpec { shader_location: 1, format: VertexFormat::Float32x2, offset: 12 }],
        },
        VertexBufferSpec {
            stride: 96,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                VertexAttributeSpec { shader_location: 3, format: VertexFormat::Float32x4, offset: 0 },
                VertexAttributeSpec { shader_location: 4, format: VertexFormat::Float32x4, offset: 16 },
                VertexAttributeSpec { shader_location: 5, format: VertexFormat::Float32x4, offset: 32 },
                VertexAttributeSpec { shader_location: 6, format: VertexFormat::Float32x4, offset: 48 },
                VertexAttributeSpec { shader_location: 7, format: VertexFormat::Float32x4, offset: 64 },
                VertexAttributeSpec { shader_location: 8, format: VertexFormat::Float32x4, offset: 80 },
            ],
        },
    ],
    bind_groups: &[
        WORLD_GLOBALS_BIND_GROUP,
        BindGroupSpec { group_index: 1, entries: &[BindGroupEntrySpec { binding: 0, visibility: FRAGMENT_STAGE, kind: BindingKind::Texture2D }, BindGroupEntrySpec { binding: 1, visibility: FRAGMENT_STAGE, kind: BindingKind::Sampler }] },
    ],
    blend: BlendMode::AlphaBlending,
    color_write: ColorWriteMask::All,
    target: ColorTarget::SurfaceFormat,
    topology: PrimitiveTopology::TriangleList,
    cull_mode: CullMode::None,
    depth_stencil: Some(DepthStencilSpec { format: DepthStencilFormat::Depth24PlusStencil8, depth_write_enabled: false, depth_compare: CompareFunction::LessEqual, stencil: WORLD_CONTENT_STENCIL, bias: NO_DEPTH_BIAS }),
};

pub const WORLD3D_FAMILY: ShaderFamily = ShaderFamily {
    name: "world3d",
    variants: &[
        ShaderVariant { name: "world3d_mesh", wgsl: WORLD3D_SHADER, pipelines: &[WORLD3D_OPAQUE_PIPELINE, WORLD3D_TRANSLUCENT_PIPELINE] },
        ShaderVariant { name: "world3d_lines", wgsl: WORLD3D_LINES_SHADER, pipelines: &[WORLD3D_LINE_PIPELINE] },
        ShaderVariant { name: "world3d_textured", wgsl: WORLD3D_TEXTURED_SHADER, pipelines: &[WORLD3D_TEXTURED_PIPELINE] },
    ],
};

//#endregion 🌐️World3dFamily

//#region 🌫️BlurFamily

/// 🌫️ One mip level of a 5-tap box downsample (`SCENE_MIP_LEVELS = 5` in `draw.rs`), sampled at
/// `blur_globals.src_mip` to build the glass backdrop's mip chain. Repaired from the `async`
/// corruption; no other damage found.
pub const BLUR_DOWNSAMPLE_SHADER: &str = r#"
struct BlurGlobals {
src_mip: f32,
_pad: vec3<f32>,
}

@group(0) @binding(0) var<uniform> blur_globals: BlurGlobals;
@group(0) @binding(1) var src_tex: texture_2d<f32>;
@group(0) @binding(2) var src_samp: sampler;

struct VertexOutput {
@builtin(position) clip_position: vec4<f32>,
@location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VertexOutput {
var positions = array<vec2<f32>, 6>(
    vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
    vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0)
);
var uvs = array<vec2<f32>, 6>(
    vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 1.0), vec2<f32>(0.0, 0.0),
    vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 1.0), vec2<f32>(1.0, 0.0)
);
var out: VertexOutput;
let pos = positions[vid];
out.clip_position = vec4<f32>(pos, 0.0, 1.0);
out.uv = uvs[vid];
return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
let mip = u32(blur_globals.src_mip);
let dim = vec2<f32>(textureDimensions(src_tex, mip));
let texel = vec2<f32>(1.0) / dim;
let uv = in.uv;
let src_mip = blur_globals.src_mip;
var c = textureSampleLevel(src_tex, src_samp, uv, src_mip) * 4.0;
c += textureSampleLevel(src_tex, src_samp, uv + vec2<f32>(-texel.x, 0.0), src_mip);
c += textureSampleLevel(src_tex, src_samp, uv + vec2<f32>(texel.x, 0.0), src_mip);
c += textureSampleLevel(src_tex, src_samp, uv + vec2<f32>(0.0, -texel.y), src_mip);
c += textureSampleLevel(src_tex, src_samp, uv + vec2<f32>(0.0, texel.y), src_mip);
return c / 8.0;
}
"#;

/// 🎞️ Fullscreen triangle-pair, no vertex buffers — geometry comes from `@builtin(vertex_index)`.
pub const BLUR_DOWNSAMPLE_PIPELINE: PipelineSpec = PipelineSpec {
    label: "blur_downsample_pipeline",
    vertex_entry: "vs_main",
    fragment_entry: "fs_main",
    vertex_buffers: &[],
    bind_groups: &[BindGroupSpec {
        group_index: 0,
        entries: &[
            BindGroupEntrySpec { binding: 0, visibility: FRAGMENT_STAGE, kind: BindingKind::UniformBuffer { dynamic_offset: false, min_size: Some(32) } },
            BindGroupEntrySpec { binding: 1, visibility: FRAGMENT_STAGE, kind: BindingKind::Texture2D },
            BindGroupEntrySpec { binding: 2, visibility: FRAGMENT_STAGE, kind: BindingKind::Sampler },
        ],
    }],
    blend: BlendMode::Replace,
    color_write: ColorWriteMask::All,
    target: ColorTarget::SurfaceFormat,
    topology: PrimitiveTopology::TriangleList,
    cull_mode: CullMode::None,
    depth_stencil: None,
};

/// 🪟️ Blits the fully-composited offscreen scene texture back onto the swapchain. Repaired from the
/// `async` corruption; no other damage found.
pub const SCENE_BLIT_SHADER: &str = r#"
@group(0) @binding(0) var scene_tex: texture_2d<f32>;
@group(0) @binding(1) var scene_samp: sampler;

struct VertexOutput {
@builtin(position) clip_position: vec4<f32>,
@location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VertexOutput {
var positions = array<vec2<f32>, 6>(
    vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
    vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0)
);
var uvs = array<vec2<f32>, 6>(
    vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 1.0), vec2<f32>(0.0, 0.0),
    vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 1.0), vec2<f32>(1.0, 0.0)
);
var out: VertexOutput;
let pos = positions[vid];
out.clip_position = vec4<f32>(pos, 0.0, 1.0);
out.uv = uvs[vid];
return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
return textureSampleLevel(scene_tex, scene_samp, in.uv, 0.0);
}
"#;

const SCENE_SAMPLE_BIND_GROUP: BindGroupSpec =
    BindGroupSpec { group_index: 0, entries: &[BindGroupEntrySpec { binding: 0, visibility: FRAGMENT_STAGE, kind: BindingKind::Texture2D }, BindGroupEntrySpec { binding: 1, visibility: FRAGMENT_STAGE, kind: BindingKind::Sampler }] };

/// 🎞️ Fullscreen triangle-pair, no vertex buffers, same shape as `BLUR_DOWNSAMPLE_PIPELINE`.
pub const SCENE_BLIT_PIPELINE: PipelineSpec = PipelineSpec {
    label: "scene_blit_pipeline",
    vertex_entry: "vs_main",
    fragment_entry: "fs_main",
    vertex_buffers: &[],
    bind_groups: &[SCENE_SAMPLE_BIND_GROUP],
    blend: BlendMode::Replace,
    color_write: ColorWriteMask::All,
    target: ColorTarget::SurfaceFormat,
    topology: PrimitiveTopology::TriangleList,
    cull_mode: CullMode::None,
    depth_stencil: None,
};

pub const BLUR_FAMILY: ShaderFamily = ShaderFamily {
    name: "blur",
    variants: &[ShaderVariant { name: "blur_downsample", wgsl: BLUR_DOWNSAMPLE_SHADER, pipelines: &[BLUR_DOWNSAMPLE_PIPELINE] }, ShaderVariant { name: "scene_blit", wgsl: SCENE_BLIT_SHADER, pipelines: &[SCENE_BLIT_PIPELINE] }],
};

//#endregion 🌫️BlurFamily

//#region 🥂️GlassFamily

/// 🥂️ Frosted-glass backdrop: samples the blurred scene at a per-instance mip, desaturates toward
/// luma, tints, and clips to a rounded-rect SDF (`discard`ed outside it). Repaired from the `async`
/// corruption; no other damage found.
pub const GLASS_SHADER: &str = r#"
struct Globals {
screen_size: vec2<f32>,
_pad: vec2<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var scene_tex: texture_2d<f32>;
@group(1) @binding(1) var scene_samp: sampler;

struct VertexInput {
@location(0) corner: vec2<f32>,
}

struct GlassInstanceInput {
@location(1) rect: vec4<f32>,
@location(2) tint: vec4<f32>,
@location(3) params: vec4<f32>,
}

struct VertexOutput {
@builtin(position) clip_position: vec4<f32>,
@location(0) local: vec2<f32>,
@location(1) size: vec2<f32>,
@location(2) tint: vec4<f32>,
@location(3) params: vec4<f32>,
@location(4) scene_uv: vec2<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: GlassInstanceInput) -> VertexOutput {
var out: VertexOutput;
let pos = instance.rect.xy + vertex.corner * instance.rect.zw;
let ndc = (pos / globals.screen_size) * 2.0 - vec2<f32>(1.0, 1.0);
out.clip_position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0);
out.local = vertex.corner * instance.rect.zw;
out.size = instance.rect.zw;
out.tint = instance.tint;
out.params = instance.params;
out.scene_uv = pos / globals.screen_size;
return out;
}

fn sdf_rounded_rect(p: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
let q = abs(p) - half_size + vec2<f32>(radius);
return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
let half = in.size * 0.5;
let p = in.local - half;
let radius = in.params.x;
let dist = sdf_rounded_rect(p, half, radius);
let fill_alpha = 1.0 - smoothstep(-1.0, 0.0, dist);
if (fill_alpha <= 0.001) {
    discard;
}
let mip = in.params.z;
let saturate = in.params.w;
let tint_alpha = in.params.y;
let blurred = textureSampleLevel(scene_tex, scene_samp, in.scene_uv, mip);
let luma = dot(blurred.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
let saturated = mix(vec3<f32>(luma), blurred.rgb, saturate);
let rgb = mix(saturated, in.tint.rgb, tint_alpha);
return vec4<f32>(rgb, fill_alpha);
}
"#;

/// 🥂️ Group 0 reuses `UI_GLOBALS_BIND_GROUP`'s layout (same as `VECTOR_PIPELINE`, unused glyph/icon
/// entries included) since that is the layout `draw.rs` actually builds; group 1 is the scene
/// sample layout shared with `SCENE_BLIT_PIPELINE`.
pub const GLASS_PIPELINE: PipelineSpec = PipelineSpec {
    label: "glass_pipeline",
    vertex_entry: "vs_main",
    fragment_entry: "fs_main",
    vertex_buffers: &[
        VertexBufferSpec { stride: 8, step_mode: VertexStepMode::Vertex, attributes: &[VertexAttributeSpec { shader_location: 0, format: VertexFormat::Float32x2, offset: 0 }] },
        VertexBufferSpec {
            stride: 48,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                VertexAttributeSpec { shader_location: 1, format: VertexFormat::Float32x4, offset: 0 },
                VertexAttributeSpec { shader_location: 2, format: VertexFormat::Float32x4, offset: 16 },
                VertexAttributeSpec { shader_location: 3, format: VertexFormat::Float32x4, offset: 32 },
            ],
        },
    ],
    bind_groups: &[UI_GLOBALS_BIND_GROUP, BindGroupSpec { group_index: 1, entries: SCENE_SAMPLE_BIND_GROUP.entries }],
    blend: BlendMode::AlphaBlending,
    color_write: ColorWriteMask::All,
    target: ColorTarget::SurfaceFormat,
    topology: PrimitiveTopology::TriangleList,
    cull_mode: CullMode::None,
    depth_stencil: None,
};

pub const GLASS_FAMILY: ShaderFamily = ShaderFamily { name: "glass", variants: &[ShaderVariant { name: "glass", wgsl: GLASS_SHADER, pipelines: &[GLASS_PIPELINE] }] };

//#endregion 🥂️GlassFamily

//#region 📚️Registry

/// 📚️ Every canonical shader family. Iterated by the structural regression test below and by any backend
/// that wants to build (or cross-compile) every pipeline up front.
pub const ALL_SHADERS: &[ShaderFamily] = &[UI_FAMILY, VECTOR_FAMILY, WORLD3D_FAMILY, BLUR_FAMILY, GLASS_FAMILY];

//#endregion 📚️Registry

//#endregion 🔖️ShaderContract

#[cfg(test)]
#[path = "../🧪️tests/🔬️shader-contract-unit/🦀️.rs"]
mod tests;
