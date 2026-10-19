//! ✨️ World3d effect shaders: the WGSL sources of the world3d celebration, lines, grid, textured, blur downsample, scene blit and postprocess passes. See [`super`].

use super::*;

// #region shaders
pub const WORLD3D_CELEBRATION_SHADER: &str = r#"
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
}

struct InstanceInput {
@location(3) model0: vec4<f32>,
@location(4) model1: vec4<f32>,
@location(5) model2: vec4<f32>,
@location(6) model3: vec4<f32>,
@location(7) color_a: vec4<f32>,
@location(8) color_b: vec4<f32>,
@location(10) color_c: vec4<f32>,
@location(11) params: vec4<f32>,
}

struct VertexOutput {
@builtin(position) clip_position: vec4<f32>,
@location(0) object_position: vec3<f32>,
@location(1) color_a: vec3<f32>,
@location(2) color_b: vec3<f32>,
@location(3) color_c: vec3<f32>,
@location(4) params: vec2<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
var out: VertexOutput;
let model = mat4x4<f32>(instance.model0, instance.model1, instance.model2, instance.model3);
out.clip_position = globals.view_proj * model * vec4<f32>(vertex.position, 1.0);
out.object_position = vertex.position;
out.color_a = instance.color_a.rgb;
out.color_b = instance.color_b.rgb;
out.color_c = instance.color_c.rgb;
out.params = instance.params.xy;
return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
let a = atan2(in.object_position.y, in.object_position.x) + in.params.x;
let t = fract(a / 6.28318530718);
var color: vec3<f32>;
if (t < 0.333333) {
    color = mix(in.color_a, in.color_b, t / 0.333333);
} else if (t < 0.666667) {
    color = mix(in.color_b, in.color_c, (t - 0.333333) / 0.333333);
} else {
    color = mix(in.color_c, in.color_a, (t - 0.666667) / 0.333333);
}
return vec4<f32>(color, in.params.y);
}
"#;

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

fn world3d_svg_attachment_output(linear_rgb: vec3<f32>) -> vec3<f32> {
return world3d_linear_to_srgb(clamp(linear_rgb, vec3<f32>(0.0), vec3<f32>(1.0)));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
let color = select(world3d_attachment_output(in.color.rgb), world3d_svg_attachment_output(in.color.rgb), globals.shadow.y > 0.5);
return vec4<f32>(color, in.color.a);
}
"#;

pub const WORLD3D_GRID_SHADER: &str = r#"
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

struct GridUniforms {
plane_cell: vec4<f32>,
camera_fade: vec4<f32>,
cell_color: vec4<f32>,
}

@group(0) @binding(0) var<uniform> globals: Globals;
@group(1) @binding(0) var<uniform> grid: GridUniforms;

struct VertexInput {
@location(0) position: vec3<f32>,
}

struct VertexOutput {
@builtin(position) clip_position: vec4<f32>,
@location(0) local_xy: vec2<f32>,
@location(1) world_position: vec3<f32>,
}

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
var out: VertexOutput;
let extent = 2.0 * (1.0 + grid.plane_cell.w);
let world_position = vec3<f32>(vertex.position.xy * extent + grid.camera_fade.xy, grid.plane_cell.x);
out.clip_position = globals.view_proj * vec4<f32>(world_position, 1.0);
out.local_xy = world_position.xy;
out.world_position = world_position;
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
let cell_size = max(grid.plane_cell.y, 0.000001);
let r = in.local_xy / cell_size;
let grid_width = abs(fract(r - vec2<f32>(0.5)) - vec2<f32>(0.5)) / max(fwidth(r), vec2<f32>(0.000001));
let g1 = 1.0 - min(min(grid_width.x, grid_width.y) + 1.0 - grid.plane_cell.z, 1.0);
let fade_distance = max(grid.plane_cell.w, 0.000001);
let d = 1.0 - min(distance(grid.camera_fade.xyz, in.world_position) / fade_distance, 1.0);
let alpha = 0.75 * g1 * pow(d, grid.camera_fade.w);
if (alpha <= 0.0) {
discard;
}
return vec4<f32>(world3d_attachment_output(grid.cell_color.rgb), alpha);
}
"#;

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

pub const WORLD3D_POSTPROCESS_SHADER: &str = r#"
struct WorldPostprocessGlobals {
viewport: vec4<f32>,
surface_size: vec2<f32>,
fov_strength: vec2<f32>,
modes: vec4<u32>,
clear_color: vec4<f32>,
}

@group(0) @binding(0) var<uniform> globals: WorldPostprocessGlobals;
@group(0) @binding(1) var capture_tex: texture_2d<f32>;
@group(0) @binding(2) var capture_samp: sampler;
@group(0) @binding(3) var background_tex: texture_2d<f32>;

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
out.clip_position = vec4<f32>(positions[vid], 0.0, 1.0);
out.uv = uvs[vid];
return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
if (globals.modes.z == 1u) {
    return globals.clear_color;
}
let ndc = in.uv * 2.0 - 1.0;
let aspect = globals.viewport.z / max(1.0, globals.viewport.w);
let scaled = vec2<f32>(ndc.x * aspect, ndc.y);
let radius = length(scaled);
let destination_pixel = globals.viewport.xy + in.uv * globals.viewport.zw;
if (globals.modes.y == 1u && length(ndc) > 1.0) {
    return textureSampleLevel(background_tex, capture_samp, destination_pixel / globals.surface_size, 0.0);
}
var source_ndc = ndc;
if (globals.modes.x == 1u && radius > 0.00001) {
    let rectilinear_radius = tan(radius * globals.fov_strength.x * 0.5) / tan(globals.fov_strength.x * 0.5);
    let source_radius = mix(radius, rectilinear_radius, globals.fov_strength.y);
    let scale = source_radius / radius;
    source_ndc = vec2<f32>(scaled.x * scale / aspect, scaled.y * scale);
}
let source_local = source_ndc * 0.5 + 0.5;
if (source_local.x < 0.0 || source_local.x > 1.0 || source_local.y < 0.0 || source_local.y > 1.0) {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
let source_pixel = globals.viewport.xy + source_local * globals.viewport.zw;
return textureSampleLevel(capture_tex, capture_samp, source_pixel / globals.surface_size, 0.0);
}
"#;
// #endregion shaders
