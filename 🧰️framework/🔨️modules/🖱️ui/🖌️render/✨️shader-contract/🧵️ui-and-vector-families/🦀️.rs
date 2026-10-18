//! 🧵️ Ui and vector families: the ui shader with its globals bind group, vertex buffers and mask and content pipelines, and the vector shader with its pipeline and family. See [`super`].

use super::*;

//#region 🔖️ShaderContract
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

pub(super) const UI_GLOBALS_BIND_GROUP: BindGroupSpec = BindGroupSpec {
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
//#endregion 🔖️ShaderContract
