//! 📏️ World3d lines, grid and textured family: the lines, grid and textured shaders with their pipelines and the world3d family. See [`super`].

use super::*;

//#region 🔖️ShaderContract
//#region 🌐️World3dFamily
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

fn world3d_svg_attachment_output(linear_rgb: vec3<f32>) -> vec3<f32> {
return world3d_linear_to_srgb(clamp(linear_rgb, vec3<f32>(0.0), vec3<f32>(1.0)));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
let color = select(world3d_attachment_output(in.color.rgb), world3d_svg_attachment_output(in.color.rgb), globals.shadow.y > 0.5);
return vec4<f32>(color, in.color.a);
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

pub const WORLD3D_GRID_PIPELINE: PipelineSpec = PipelineSpec {
    label: "world3d_grid_pipeline",
    vertex_entry: "vs_main",
    fragment_entry: "fs_main",
    vertex_buffers: &[VertexBufferSpec { stride: 20, step_mode: VertexStepMode::Vertex, attributes: &[VertexAttributeSpec { shader_location: 0, format: VertexFormat::Float32x3, offset: 0 }] }],
    bind_groups: &[
        WORLD_GLOBALS_BIND_GROUP,
        BindGroupSpec { group_index: 1, entries: &[BindGroupEntrySpec { binding: 0, visibility: VERTEX_FRAGMENT_STAGE, kind: BindingKind::UniformBuffer { dynamic_offset: false, min_size: Some(256) } }] },
    ],
    blend: BlendMode::AlphaBlending,
    color_write: ColorWriteMask::All,
    target: ColorTarget::SurfaceFormat,
    topology: PrimitiveTopology::TriangleList,
    cull_mode: CullMode::None,
    depth_stencil: Some(DepthStencilSpec { format: DepthStencilFormat::Depth24PlusStencil8, depth_write_enabled: false, depth_compare: CompareFunction::LessEqual, stencil: WORLD_CONTENT_STENCIL, bias: NO_DEPTH_BIAS }),
};

/// 🖼️ Textured world quad (position + uv, per-instance model + Basic appearance) — the reference underlay every
/// `ScenePass3d::textured_draws` entry carries. Built as `world3d_textured_pipeline` in
/// `🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🦀️.rs` since ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W5c; the
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
        ShaderVariant { name: "world3d_grid", wgsl: WORLD3D_GRID_SHADER, pipelines: &[WORLD3D_GRID_PIPELINE] },
        ShaderVariant { name: "world3d_textured", wgsl: WORLD3D_TEXTURED_SHADER, pipelines: &[WORLD3D_TEXTURED_PIPELINE] },
    ],
};
//#endregion 🌐️World3dFamily
//#endregion 🔖️ShaderContract
