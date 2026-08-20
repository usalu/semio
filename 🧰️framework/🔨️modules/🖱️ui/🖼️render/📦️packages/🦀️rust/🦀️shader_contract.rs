//! @emoji ✨️ Canonical WGSL for the five shader families plus their `PipelineSpec` metadata.
//!
//! This is the single source of truth every `GraphicsBackend` builds its pipelines from — no
//! backend reads WGSL directly. webgpu consumes [`ShaderVariant::wgsl`] verbatim; vulkan/metal/
//! d3d12 cross-compile it at build time (naga SPIR-V/MSL/HLSL backends, never linked at runtime).
//! Copied here from `🎯️targets/🧊️wgpu/🦀️shaders.rs` after repairing the asyncify corruption
//! (`async fn vs_main`/`async fn fs_main` — WGSL has no `async` keyword) — see
//! `📓️terra-shader-repair-report.md` in ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`
//! for the full corruption inventory. The `#[cfg(test)]` module below is the permanent regression
//! guard: it runs naga's WGSL front end over every constant so this class of defect fails in CI,
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
    Sampler,
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
/// variant's actual naga entry points by the `pipeline_entry_points_exist_in_shader` test below —
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

const WORLD_GLOBALS_BIND_GROUP: BindGroupSpec = BindGroupSpec { group_index: 0, entries: &[BindGroupEntrySpec { binding: 0, visibility: VERTEX_FRAGMENT_STAGE, kind: BindingKind::UniformBuffer { dynamic_offset: true, min_size: Some(80) } }] };

const WORLD_CONTENT_STENCIL: StencilStateSpec = StencilStateSpec { compare: CompareFunction::Equal, fail_op: StencilOperation::Keep, depth_fail_op: StencilOperation::Keep, pass_op: StencilOperation::Keep, read_mask: 0xff, write_mask: 0x00 };

/// 🗻️ Lit mesh instances (Lambertian + selected/hovered tint), per-pass `view_proj`/`light_dir`
/// uniform via a dynamic-offset ring buffer. Repaired from the `async` corruption; no other damage
/// found.
pub const WORLD3D_SHADER: &str = r#"
struct Globals {
view_proj: mat4x4<f32>,
light_dir: vec4<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;

struct VertexInput {
@location(0) position: vec3<f32>,
@location(1) normal: vec3<f32>,
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
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
var out: VertexOutput;
let model = mat4x4<f32>(instance.model0, instance.model1, instance.model2, instance.model3);
let world_pos = model * vec4<f32>(vertex.position, 1.0);
out.clip_position = globals.view_proj * world_pos;
let normal_matrix = mat3x3<f32>(
    model[0].xyz,
    model[1].xyz,
    model[2].xyz
);
out.normal = normalize(normal_matrix * vertex.normal);
out.color = instance.color;
out.flags = instance.flags;
return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
let n = normalize(in.normal);
let diffuse = max(dot(n, normalize(globals.light_dir.xyz)), 0.28);
var color = in.color.rgb * diffuse;
if (in.flags.x > 0.5) {
    color = mix(color, vec3<f32>(0.35, 0.75, 1.0), 0.65);
}
if (in.flags.y > 0.5) {
    color = mix(color, vec3<f32>(1.0, 0.85, 0.35), 0.55);
}
return vec4<f32>(color, in.color.a);
}
"#;

const WORLD3D_MESH_VERTEX_BUFFERS: &[VertexBufferSpec] = &[
    VertexBufferSpec { stride: 24, step_mode: VertexStepMode::Vertex, attributes: &[VertexAttributeSpec { shader_location: 0, format: VertexFormat::Float32x3, offset: 0 }, VertexAttributeSpec { shader_location: 1, format: VertexFormat::Float32x3, offset: 12 }] },
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
    bind_groups: &[WORLD_GLOBALS_BIND_GROUP],
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
    bind_groups: &[WORLD_GLOBALS_BIND_GROUP],
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
light_dir: vec4<f32>,
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

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
return in.color;
}
"#;

/// ➖️ Line-list topology, alpha-blended, depth-tested but not depth-writing (drawn over the mesh
/// passes without occluding them).
pub const WORLD3D_LINE_PIPELINE: PipelineSpec = PipelineSpec {
    label: "world3d_line_pipeline",
    vertex_entry: "vs_main",
    fragment_entry: "fs_main",
    vertex_buffers: &[VertexBufferSpec { stride: 28, step_mode: VertexStepMode::Vertex, attributes: &[VertexAttributeSpec { shader_location: 0, format: VertexFormat::Float32x3, offset: 0 }, VertexAttributeSpec { shader_location: 1, format: VertexFormat::Float32x4, offset: 12 }] }],
    bind_groups: &[WORLD_GLOBALS_BIND_GROUP],
    blend: BlendMode::AlphaBlending,
    color_write: ColorWriteMask::All,
    target: ColorTarget::SurfaceFormat,
    topology: PrimitiveTopology::LineList,
    cull_mode: CullMode::None,
    depth_stencil: Some(DepthStencilSpec { format: DepthStencilFormat::Depth24PlusStencil8, depth_write_enabled: false, depth_compare: CompareFunction::LessEqual, stencil: WORLD_CONTENT_STENCIL, bias: NO_DEPTH_BIAS }),
};

/// 🖼️ Textured mesh variant (position + uv, per-instance model + tint). Declared in the original
/// `🎯️targets/🧊️wgpu/🦀️shaders.rs` but — confirmed by grepping the whole `🧰️framework` tree —
/// **never imported or built into a pipeline anywhere**: `draw.rs:4` imports only 7 of the 8
/// constants, omitting this one, and no other file references it. It is dead code in the committed
/// wgpu-old path, not a wired-but-broken feature. Kept here because the canonical contract is
/// forward-looking (the shader itself is valid WGSL and a real backend may want a textured mesh
/// pass), but its `PipelineSpec` below is **not** derived from any real pipeline construction —
/// see the doc comment on `WORLD3D_TEXTURED_PIPELINE` for exactly which fields are inferred.
pub const WORLD3D_TEXTURED_SHADER: &str = r#"
struct Globals {
view_proj: mat4x4<f32>,
light_dir: vec4<f32>,
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
@location(7) tint: vec4<f32>,
}

struct VertexOutput {
@builtin(position) clip_position: vec4<f32>,
@location(0) uv: vec2<f32>,
@location(1) tint: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
var out: VertexOutput;
let model = mat4x4<f32>(instance.model0, instance.model1, instance.model2, instance.model3);
let world_pos = model * vec4<f32>(vertex.position, 1.0);
out.clip_position = globals.view_proj * world_pos;
out.uv = vertex.uv;
out.tint = instance.tint;
return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
let sampled = textureSample(tex, tex_sampler, in.uv);
return vec4<f32>(sampled.rgb * in.tint.rgb, sampled.a * in.tint.a);
}
"#;

/// ⚠️ INFERRED, not source-derived — no pipeline for this shader exists in `draw.rs` to read state
/// from. `vertex_entry`/`fragment_entry`, `vertex_buffers` (stride/offsets from the WGSL struct
/// field order above) and `bind_groups` (group 0 mirrors `WORLD_GLOBALS_BIND_GROUP`'s uniform
/// shape; group 1 is read directly off this shader's own `@group(1)` declarations) are solid.
/// `blend`/`cull_mode`/`depth_stencil`/`WORLD_GLOBALS_BIND_GROUP`'s `dynamic_offset` reuse on group
/// 0 are a documented best guess mirroring `WORLD3D_OPAQUE_PIPELINE` (nearest analog: opaque,
/// depth-writing mesh geometry) — reported, not silently assumed; see
/// `📓️terra-shader-repair-report.md`.
pub const WORLD3D_TEXTURED_PIPELINE: PipelineSpec = PipelineSpec {
    label: "world3d_textured_pipeline (inferred — unwired in draw.rs)",
    vertex_entry: "vs_main",
    fragment_entry: "fs_main",
    vertex_buffers: &[
        VertexBufferSpec { stride: 20, step_mode: VertexStepMode::Vertex, attributes: &[VertexAttributeSpec { shader_location: 0, format: VertexFormat::Float32x3, offset: 0 }, VertexAttributeSpec { shader_location: 1, format: VertexFormat::Float32x2, offset: 12 }] },
        VertexBufferSpec {
            stride: 80,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                VertexAttributeSpec { shader_location: 3, format: VertexFormat::Float32x4, offset: 0 },
                VertexAttributeSpec { shader_location: 4, format: VertexFormat::Float32x4, offset: 16 },
                VertexAttributeSpec { shader_location: 5, format: VertexFormat::Float32x4, offset: 32 },
                VertexAttributeSpec { shader_location: 6, format: VertexFormat::Float32x4, offset: 48 },
                VertexAttributeSpec { shader_location: 7, format: VertexFormat::Float32x4, offset: 64 },
            ],
        },
    ],
    bind_groups: &[
        WORLD_GLOBALS_BIND_GROUP,
        BindGroupSpec { group_index: 1, entries: &[BindGroupEntrySpec { binding: 0, visibility: FRAGMENT_STAGE, kind: BindingKind::Texture2D }, BindGroupEntrySpec { binding: 1, visibility: FRAGMENT_STAGE, kind: BindingKind::Sampler }] },
    ],
    blend: BlendMode::Replace,
    color_write: ColorWriteMask::All,
    target: ColorTarget::SurfaceFormat,
    topology: PrimitiveTopology::TriangleList,
    cull_mode: CullMode::None,
    depth_stencil: Some(DepthStencilSpec { format: DepthStencilFormat::Depth24PlusStencil8, depth_write_enabled: true, depth_compare: CompareFunction::Less, stencil: WORLD_CONTENT_STENCIL, bias: NO_DEPTH_BIAS }),
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

const SCENE_SAMPLE_BIND_GROUP: BindGroupSpec = BindGroupSpec { group_index: 0, entries: &[BindGroupEntrySpec { binding: 0, visibility: FRAGMENT_STAGE, kind: BindingKind::Texture2D }, BindGroupEntrySpec { binding: 1, visibility: FRAGMENT_STAGE, kind: BindingKind::Sampler }] };

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
    variants: &[
        ShaderVariant { name: "blur_downsample", wgsl: BLUR_DOWNSAMPLE_SHADER, pipelines: &[BLUR_DOWNSAMPLE_PIPELINE] },
        ShaderVariant { name: "scene_blit", wgsl: SCENE_BLIT_SHADER, pipelines: &[SCENE_BLIT_PIPELINE] },
    ],
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

/// 📚️ Every canonical shader family. Iterated by the naga regression test below and by any backend
/// that wants to build (or cross-compile) every pipeline up front.
pub const ALL_SHADERS: &[ShaderFamily] = &[UI_FAMILY, VECTOR_FAMILY, WORLD3D_FAMILY, BLUR_FAMILY, GLASS_FAMILY];

//#endregion 📚️Registry

//#endregion 🔖️ShaderContract

#[cfg(test)]
mod tests {
    use super::*;

    /// 🛡️ Parses and validates `source` with naga's WGSL front end, panicking with `label` and
    /// naga's own diagnostic on failure — this is the check that would have caught `async fn
    /// vs_main` the moment the asyncify codemod introduced it.
    fn assert_wgsl_valid(label: &str, source: &str) -> naga::Module {
        let module = naga::front::wgsl::parse_str(source).unwrap_or_else(|error| panic!("{label}: naga failed to parse WGSL — {}", error.emit_to_string(source)));
        let mut validator = naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all());
        validator.validate(&module).unwrap_or_else(|error| panic!("{label}: naga validation failed — {}", error.emit_to_string(source)));
        module
    }

    #[test]
    fn all_canonical_shaders_parse_and_validate() {
        for family in ALL_SHADERS {
            for variant in family.variants {
                assert_wgsl_valid(variant.name, variant.wgsl);
            }
        }
    }

    #[test]
    fn pipeline_entry_points_exist_in_shader() {
        for family in ALL_SHADERS {
            for variant in family.variants {
                let module = assert_wgsl_valid(variant.name, variant.wgsl);
                let entry_names: Vec<&str> = module.entry_points.iter().map(|entry| entry.name.as_str()).collect();
                for pipeline in variant.pipelines {
                    assert!(entry_names.contains(&pipeline.vertex_entry), "{}/{}: PipelineSpec '{}' names vertex entry {:?} but the WGSL only declares {:?}", family.name, variant.name, pipeline.label, pipeline.vertex_entry, entry_names);
                    assert!(entry_names.contains(&pipeline.fragment_entry), "{}/{}: PipelineSpec '{}' names fragment entry {:?} but the WGSL only declares {:?}", family.name, variant.name, pipeline.label, pipeline.fragment_entry, entry_names);
                }
            }
        }
    }

    #[test]
    fn vertex_attribute_offsets_fit_declared_stride() {
        for family in ALL_SHADERS {
            for variant in family.variants {
                for pipeline in variant.pipelines {
                    for buffer in pipeline.vertex_buffers {
                        for attribute in buffer.attributes {
                            let end = attribute.offset + attribute.format.byte_size();
                            assert!(end <= buffer.stride, "{}/{}: PipelineSpec '{}' attribute at location {} ends at byte {} but the buffer stride is only {}", family.name, variant.name, pipeline.label, attribute.shader_location, end, buffer.stride);
                        }
                    }
                }
            }
        }
    }
}
