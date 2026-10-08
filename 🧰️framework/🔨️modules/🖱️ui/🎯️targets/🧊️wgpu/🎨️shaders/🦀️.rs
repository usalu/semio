// #region shaders
//! 🧊️ WGSL shader sources for the raw wgpu UI renderer.

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
let pos = instance.rect.xy + vertex.corner * instance.rect.zw;
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
if (kind == 2) {
    return vec4<f32>(in.color.rgb, glyph.r * in.color.a);
}
if (kind == 4 || kind == 5) {
    var coverage = 1.0;
    if (in.params.x > 0.0) {
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
clip_plane: vec4<f32>,
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
@location(10) emissive_cutoff: vec4<f32>,
}

struct VertexOutput {
@builtin(position) clip_position: vec4<f32>,
@location(0) color: vec4<f32>,
@location(1) normal: vec3<f32>,
@location(2) flags: vec4<f32>,
@location(3) world_position: vec3<f32>,
@location(4) emissive_cutoff: vec4<f32>,
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
if ((u32(instance.flags.x) & 8u) != 0u) {out.color.a = 1.0;}
out.flags = instance.flags;
out.world_position = world_pos.xyz;
out.emissive_cutoff = instance.emissive_cutoff;
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

fn world3d_svg_attachment_output(linear_rgb: vec3<f32>) -> vec3<f32> {
    return world3d_linear_to_srgb(clamp(linear_rgb, vec3<f32>(0.0), vec3<f32>(1.0)));
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

fn world3d_section_clipped(flags: f32, world_position: vec3<f32>) -> bool {
return (u32(flags) & 32u) != 0u && dot(world_position, globals.clip_plane.xyz) + globals.clip_plane.w < 0.0;
}

@fragment
fn fs_main(in: VertexOutput, @builtin(front_facing) front_facing: bool) -> @location(0) vec4<f32> {
if (world3d_section_clipped(in.flags.x, in.world_position)) {
    discard;
}
if (in.emissive_cutoff.w >= 0.0 && in.color.a < in.emissive_cutoff.w) {
    discard;
}
let n = normalize(in.normal);
let v = normalize(globals.camera_position.xyz - in.world_position);
if (globals.shadow.y > 0.5) {
    if (!front_facing) {
        discard;
    }
    var face_normal = normalize(cross(dpdx(in.world_position), dpdy(in.world_position)));
    if (dot(face_normal, n) < 0.0) {
        face_normal = -face_normal;
    }
    var svg_light = globals.ambient.rgb;
    if (globals.material.w > 0.5) {
        svg_light = svg_light + globals.sun.rgb * globals.sun.a * max(dot(face_normal, normalize(globals.light_dir.xyz)), 0.0);
    }
    let svg_color = svg_light * in.color.rgb + globals.material_emissive.rgb;
    return vec4<f32>(world3d_svg_attachment_output(svg_color), in.color.a);
}
let metalness = clamp(in.flags.z, 0.0, 1.0);
let normal_derivative = max(abs(dpdx(in.normal)), abs(dpdy(in.normal)));
let geometry_roughness = max(max(normal_derivative.x, normal_derivative.y), normal_derivative.z);
let roughness = min(max(in.flags.w, 0.0525) + geometry_roughness, 1.0);
let emissive = globals.material_emissive.rgb * globals.material.z + in.color.rgb * max(in.flags.y, 0.0) + in.emissive_cutoff.rgb;
var shadow_visibility = 1.0;
if ((u32(in.flags.x) & 2u) != 0u) {
    shadow_visibility = world3d_shadow_visibility(in.world_position, in.clip_position.xy);
}
let color = world3d_lighting(n, v, in.color.rgb, metalness, roughness, shadow_visibility) + emissive;
return vec4<f32>(world3d_attachment_output(color), in.color.a);
}

@fragment
fn fs_section_stencil(in: VertexOutput) -> @location(0) vec4<f32> {
if (world3d_section_clipped(in.flags.x, in.world_position)) {
    discard;
}
return vec4<f32>(0.0);
}

@fragment
fn fs_section_cap(in: VertexOutput) -> @location(0) vec4<f32> {
return vec4<f32>(world3d_attachment_output(in.color.rgb), 1.0);
}
"#;

/// 🎨️ Adds React's `MeshStandardMaterial.map` lane to the canonical lit mesh shader without
/// duplicating its BRDF/normal/ACES implementation. Paint PNG bytes live in the shared sRGB raster
/// table, so applying the OETF once recovers their `NoColorSpace` byte values before lighting.
pub fn world3d_painted_shader() -> String {
    WORLD3D_PAINTED_SHADER_EDITS.iter().fold(WORLD3D_SHADER.to_string(), |shader, (anchor, edit)| shader.replacen(anchor, edit, 1))
}

/// 🎨️ The ordered edits that turn [`WORLD3D_SHADER`] into [`world3d_painted_shader`]: the paint map bindings, the UV
/// varyings, the sampled albedo with its mask test, and the sampled alpha. Every anchor occurs EXACTLY once in the lit
/// shader (law `every_painted_shader_edit_finds_its_anchor_in_the_lit_shader_once`) — an anchor that stopped matching once
/// dropped the `lit_color` declaration silently and every GPU context refused the painted shader module.
pub const WORLD3D_PAINTED_SHADER_EDITS: [(&str, &str); 6] = [
    (
        "@group(1) @binding(1) var shadow_sampler: sampler_comparison;",
        "@group(1) @binding(1) var shadow_sampler: sampler_comparison;\n@group(2) @binding(0) var paint_map: texture_2d<f32>;\n@group(2) @binding(1) var paint_sampler: sampler;",
    ),
    ("@location(2) color: vec4<f32>,\n}", "@location(2) color: vec4<f32>,\n@location(9) uv: vec2<f32>,\n}"),
    ("@location(4) emissive_cutoff: vec4<f32>,\n}", "@location(4) emissive_cutoff: vec4<f32>,\n@location(5) uv: vec2<f32>,\n}"),
    ("out.emissive_cutoff = instance.emissive_cutoff;\nreturn out;", "out.emissive_cutoff = instance.emissive_cutoff;\nout.uv = vertex.uv;\nreturn out;"),
    (
        "let emissive = globals.material_emissive.rgb * globals.material.z + in.color.rgb * max(in.flags.y, 0.0) + in.emissive_cutoff.rgb;",
        "let sampled = textureSample(paint_map, paint_sampler, in.uv);\nlet authored_texture = (u32(in.flags.x) & 4u) != 0u;\nlet paint_color = select(world3d_linear_to_srgb(sampled.rgb), sampled.rgb, authored_texture);\nlet lit_color = in.color.rgb * paint_color;\nif (in.emissive_cutoff.w >= 0.0 && sampled.a * in.color.a < in.emissive_cutoff.w) { discard; }\nlet emissive = globals.material_emissive.rgb * globals.material.z + in.color.rgb * max(in.flags.y, 0.0) + in.emissive_cutoff.rgb;",
    ),
    (
        "let color = world3d_lighting(n, v, in.color.rgb, metalness, roughness, shadow_visibility) + emissive;\nreturn vec4<f32>(world3d_attachment_output(color), in.color.a);",
        "let color = world3d_lighting(n, v, lit_color, metalness, roughness, shadow_visibility) + emissive;\nreturn vec4<f32>(world3d_attachment_output(color), sampled.a * in.color.a);",
    ),
];

/// 🎨️ Samples the five owned authored surface roles through the canonical lighting shader.
pub fn world3d_authored_shader() -> String {
    let mut shader = world3d_painted_shader();
    shader = shader.replace("@group(2) @binding(1) var paint_sampler: sampler;", "@group(2) @binding(1) var paint_sampler: sampler;\n@group(2) @binding(2) var metallic_roughness_map: texture_2d<f32>;\n@group(2) @binding(3) var metallic_roughness_sampler: sampler;\n@group(2) @binding(4) var normal_map: texture_2d<f32>;\n@group(2) @binding(5) var normal_sampler: sampler;\n@group(2) @binding(6) var occlusion_map: texture_2d<f32>;\n@group(2) @binding(7) var occlusion_sampler: sampler;\n@group(2) @binding(8) var emissive_map: texture_2d<f32>;\n@group(2) @binding(9) var emissive_sampler: sampler;");
    shader = shader.replace("if (in.emissive_cutoff.w >= 0.0 && in.color.a < in.emissive_cutoff.w) {\n    discard;\n}", "");
    shader = shader.replace("let n = normalize(in.normal);", r#"var n = normalize(in.normal) * select(-1.0, 1.0, front_facing);
    let q0 = dpdx(in.world_position);
    let q1 = -dpdy(in.world_position);
    let st0 = dpdx(in.uv);
    let st1 = -dpdy(in.uv);
    let q1perp = cross(q1, n);
    let q0perp = cross(n, q0);
    let tangent = q1perp * st0.x + q0perp * st1.x;
    let bitangent = q1perp * st0.y + q0perp * st1.y;
    let determinant = max(dot(tangent, tangent), dot(bitangent, bitangent));
    let scale = select(0.0, inverseSqrt(max(determinant, 0.00000001)), determinant > 0.0);
    if ((u32(in.flags.x) & 16u) != 0u) { n = normalize(mat3x3<f32>(tangent * scale * select(select(-1.0,1.0,front_facing),1.0,abs(in.tangent.w)>0.5), bitangent * scale * select(select(-1.0,1.0,front_facing),1.0,abs(in.tangent.w)>0.5), n) * sampled_normal); }"#);
    shader = shader.replace("let metalness = clamp(in.flags.z, 0.0, 1.0);", "let metallic_roughness = textureSample(metallic_roughness_map, metallic_roughness_sampler, in.uv);\nlet metalness = clamp(in.flags.z * metallic_roughness.b, 0.0, 1.0);");
    shader = shader.replace("max(in.flags.w, 0.0525)", "max(in.flags.w * metallic_roughness.g, 0.0525)");
    shader = shader.replace("sampled.a * in.color.a", "select(sampled.a * in.color.a, 1.0, (u32(in.flags.x) & 8u) != 0u)");
    shader = shader.replace("+ in.emissive_cutoff.rgb;", "+ in.emissive_cutoff.rgb * textureSample(emissive_map, emissive_sampler, in.uv).rgb;");
    shader = shader.replace("shadow_visibility: f32) -> vec3<f32>", "shadow_visibility: f32, occlusion: f32) -> vec3<f32>");
    shader = shader.replace("return indirect * base_color", "return indirect * occlusion * base_color");
    shader = shader.replace("roughness, shadow_visibility) + emissive", "roughness, shadow_visibility, textureSample(occlusion_map, occlusion_sampler, in.uv).r) + emissive");
    for (expression, name) in [
        ("textureSample(normal_map, normal_sampler, in.uv).xyz * 2.0 - vec3<f32>(1.0)", "sampled_normal"),
        ("textureSample(metallic_roughness_map, metallic_roughness_sampler, in.uv)", "sampled_metallic_roughness"),
        ("textureSample(paint_map, paint_sampler, in.uv)", "sampled_base"),
        ("textureSample(emissive_map, emissive_sampler, in.uv).rgb", "sampled_emissive"),
        ("textureSample(occlusion_map, occlusion_sampler, in.uv).r", "sampled_occlusion"),
        ("max(abs(dpdx(in.normal)), abs(dpdy(in.normal)))", "authored_normal_derivative"),
    ] {
        shader = shader.replace(expression, name);
        shader = shader.replace("fn fs_main(in: VertexOutput, @builtin(front_facing) front_facing: bool) -> @location(0) vec4<f32> {", &format!("fn fs_main(in: VertexOutput, @builtin(front_facing) front_facing: bool) -> @location(0) vec4<f32> {{\nlet {name} = {expression};"));
    }
    shader = shader.replace("let sampled_normal = sampled_normal;", "");
    shader = shader.replace("var face_normal = normalize(cross(dpdx(in.world_position), dpdy(in.world_position)));", "var face_normal = normalize(cross(q0, q1));");
    shader = shader.replace("@location(9) uv: vec2<f32>,", "@location(9) uv_base_metallic:vec4<f32>,\n@location(11) uv_normal_occlusion:vec4<f32>,\n@location(12) uv_emissive:vec2<f32>,\n@location(13) tangent:vec4<f32>,");
    shader = shader.replace("@location(10) emissive_cutoff: vec4<f32>,", "@location(10) emissive_cutoff: vec4<f32>,\n@location(15) surface_parameters:vec4<f32>,");
    shader = shader.replace("@location(5) uv: vec2<f32>,", "@location(5) uv: vec2<f32>,\n@location(6) surface_parameters:vec4<f32>,\n@location(7) uv_metallic_roughness:vec2<f32>,\n@location(8) uv_normal:vec2<f32>,\n@location(9) uv_occlusion:vec2<f32>,\n@location(10) uv_emissive:vec2<f32>,\n@location(11) tangent:vec4<f32>,");
    shader = shader.replace("out.uv = vertex.uv;", "out.uv = vertex.uv_base_metallic.xy;\nout.uv_metallic_roughness=vertex.uv_base_metallic.zw;\nout.uv_normal=vertex.uv_normal_occlusion.xy;\nout.uv_occlusion=vertex.uv_normal_occlusion.zw;\nout.uv_emissive=vertex.uv_emissive;\nout.surface_parameters=instance.surface_parameters;");
    for (role,coordinate) in [("metallic_roughness","uv_metallic_roughness"),("normal","uv_normal"),("occlusion","uv_occlusion"),("emissive","uv_emissive")] {shader=shader.replace(&format!("{role}_sampler, in.uv)"),&format!("{role}_sampler, in.{coordinate})"));}
    shader=shader.replace("out.world_position = world_pos.xyz;","out.world_position = world_pos.xyz;\nlet authored_tangent=(model * vec4<f32>(vertex.tangent.xyz,0.0)).xyz;\nlet tangent_length=max(length(authored_tangent),0.00000001);\nout.tangent=vec4<f32>(authored_tangent/tangent_length,vertex.tangent.w);");
    shader=shader.replace("let tangent = q1perp", "var tangent = q1perp").replace("let bitangent = q1perp", "var bitangent = q1perp");
    shader=shader.replace("let determinant = max(dot(tangent, tangent)","if (abs(in.tangent.w)>0.5) { tangent=normalize(in.tangent.xyz) * select(-1.0,1.0,front_facing); bitangent=normalize(cross(normalize(in.normal),normalize(in.tangent.xyz))) * in.tangent.w * select(-1.0,1.0,front_facing); }\n    let determinant = max(dot(tangent, tangent)");
    shader=shader.replace("dpdx(in.uv)","dpdx(in.uv_normal)").replace("dpdy(in.uv)","dpdy(in.uv_normal)");
    shader=shader.replace("* sampled_normal);", "* vec3<f32>(sampled_normal.xy * in.surface_parameters.xy, sampled_normal.z));");
    shader=shader.replace("roughness, shadow_visibility, sampled_occlusion)","roughness, shadow_visibility, mix(1.0,sampled_occlusion,in.surface_parameters.z))");
    shader
}

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
