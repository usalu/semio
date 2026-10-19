//! 🖼️ Ui, vector, glass and raster mip shaders: the WGSL sources of the ui instance, vector, glass and raster mip passes. See [`super`].

use super::*;

// #region shaders
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
@location(5) clip_ellipse: vec4<f32>,
}

struct VertexOutput {
@builtin(position) clip_position: vec4<f32>,
@location(0) local: vec2<f32>,
@location(1) size: vec2<f32>,
@location(2) color: vec4<f32>,
@location(3) params: vec4<f32>,
@location(4) uv: vec2<f32>,
@location(5) clip_ellipse: vec4<f32>,
@location(6) logical_position: vec2<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
var out: VertexOutput;
let kind = i32(instance.params.z + 0.5);
let affine = kind == 10 || kind == 11;
let pos = select(instance.rect.xy + vertex.corner * instance.rect.zw, instance.rect.xy + vertex.corner.x * instance.rect.zw + vertex.corner.y * instance.params.xy, affine);
let ndc = (pos / globals.screen_size) * 2.0 - vec2<f32>(1.0, 1.0);
out.clip_position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0);
out.local = vertex.corner * instance.rect.zw;
out.clip_ellipse = instance.clip_ellipse;
out.logical_position = pos;
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

fn paint_fragment(in: VertexOutput) -> vec4<f32> {
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
if (kind == 2 || kind == 10) {
    return vec4<f32>(in.color.rgb, glyph.r * in.color.a);
}
if (kind == 4 || kind == 5 || kind == 11) {
    var coverage = 1.0;
    if (kind != 11 && in.params.x > 0.0) {
        let half = in.size * 0.5;
        let dist = sdf_rounded_rect(in.local - half, half, in.params.x);
        coverage = 1.0 - smoothstep(-1.0, 0.0, dist);
    }
    return vec4<f32>(icon.rgb * in.color.rgb, icon.a * in.color.a * coverage);
}
if (kind == 3) {
    return in.color;
}
return in.color;
}
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
let color = paint_fragment(in);
let radius = max(in.clip_ellipse.zw * 0.5, vec2<f32>(0.0001));
let normalized = (in.logical_position - in.clip_ellipse.xy - radius) / radius;
let distance = length(normalized) - 1.0;
let antialias = max(fwidth(distance), 0.0001);
let coverage = 1.0 - smoothstep(-antialias * 0.5, antialias * 0.5, distance);
let enabled = in.clip_ellipse.z > 0.0 && in.clip_ellipse.w > 0.0;
return vec4<f32>(color.rgb, color.a * select(1.0, coverage, enabled));
}
"#;

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
// #endregion shaders

/// 🖼️ Generates one filtered authored raster mip through the existing upload owner.
pub const RASTER_MIP_SHADER:&str=r#"
@group(0) @binding(0) var source:texture_2d<f32>;
@group(0) @binding(1) var source_sampler:sampler;
@vertex fn vs_main(@builtin(vertex_index) index:u32)->@builtin(position) vec4<f32>{let positions=array<vec2<f32>,3>(vec2<f32>(-1.0,-1.0),vec2<f32>(3.0,-1.0),vec2<f32>(-1.0,3.0));return vec4<f32>(positions[index],0.0,1.0);}
@fragment fn fs_main(@builtin(position) position:vec4<f32>)->@location(0) vec4<f32>{let target_size=max(textureDimensions(source)/vec2<u32>(2u),vec2<u32>(1u));return textureSampleLevel(source,source_sampler,position.xy/vec2<f32>(target_size),0.0);}
"#;
