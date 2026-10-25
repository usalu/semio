//! 🌫️ Blur and glass families and the registry: the blur downsample and scene blit shaders and pipelines, the glass shader and pipeline and the `ALL_SHADERS` registry. See [`super`].

use super::*;

//#region 🔖️ShaderContract
//#region 🌫️BlurFamily
/// 🔽️ One mip level of a 5-tap box downsample (`SCENE_MIP_LEVELS = 5` in `draw.rs`), sampled at
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

/// 🚂️ Fullscreen triangle-pair, no vertex buffers, same shape as `BLUR_DOWNSAMPLE_PIPELINE`.
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

/// 🔍️ Group 0 reuses `UI_GLOBALS_BIND_GROUP`'s layout (same as `VECTOR_PIPELINE`, unused glyph/icon
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
