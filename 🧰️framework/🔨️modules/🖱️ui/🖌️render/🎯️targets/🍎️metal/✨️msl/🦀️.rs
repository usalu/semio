//! @emoji ✨️ Hand-written MSL for the five shader families, ported line-for-line from the canonical
//! WGSL in `ui_render::shader_contract` (packet `shader-repair`'s repaired constants — never the
//! asyncify-corrupted `🎯️targets/🧊️wgpu/🦀️shaders.rs`).
//!
//! **Interim path, not the planned one.** The plan (master.md §4 "Shader strategy") is build-time
//! `naga` cross-compilation of the canonical WGSL to MSL via a `build.rs`; that needs a
//! `[build-dependencies] naga` line in this crate's `Cargo.toml`, which is registrar-only (U7) — see
//! `registrar-requests` in `📓️terra-backend-metal-report.md`. Until that lands, every family below is
//! hand-transcribed MSL, each annotated with which WGSL constant it mirrors and any deliberate
//! Metal-side simplification (never a semantic change — see each comment for the reasoning).
//!
//! Two simplifications recur enough to note once here rather than per-shader:
//! - **No per-mip `TextureView`s.** WGSL's `textureSampleLevel`/render-target-per-mip needs a
//!   `wgpu::TextureView` pinned to one mip level; Metal's `sample(..., level(lod))` takes an explicit
//!   LOD directly against the whole texture, and `MTLRenderPassColorAttachmentDescriptor.level` picks
//!   a render-target mip directly on the original texture. So this backend never allocates the
//!   per-mip view arrays `SceneColorTarget` (wgpu target) needed — same pixels, fewer objects.
//! - **Two-stage buffers, not a `@group`.** WGSL's `@group(0) @binding(N)` seam has no Metal
//!   equivalent (Metal binds buffers/textures/samplers directly by index per stage); every `@group`
//!   below is realized as `[[buffer(N)]]`/`[[texture(N)]]`/`[[sampler(N)]]` argument-table slots,
//!   assigned in `🏗️pipelines.rs`'s doc comments alongside each pipeline.

//#region 🔖️Msl

//#region 🧵️UiFamily

/// 🧊️ Mirrors `ui_render::shader_contract::UI_SHADER`. `sdf_rounded_rect` and every `kind` branch
/// (1,6,7,8,9 rounded/animated rings; 2 glyph; 4/5 icon/raster; 3/default solid) are transcribed
/// verbatim — same constants (`two_pi`, ring durations 1.6/3.2s, dash period 12, pulse curves).
pub const UI_SHADER_MSL: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct UiGlobals {
    float2 screen_size;
    float2 _pad;
};

struct UiVertexIn {
    float2 corner [[attribute(0)]];
    float4 rect [[attribute(1)]];
    float4 color [[attribute(2)]];
    float4 params [[attribute(3)]];
    float4 uv_rect [[attribute(4)]];
};

struct UiVertexOut {
    float4 clip_position [[position]];
    float2 local;
    float2 size;
    float4 color;
    float4 params;
    float2 uv;
};

vertex UiVertexOut ui_vertex_main(UiVertexIn in [[stage_in]],
                                   constant UiGlobals& globals [[buffer(2)]]) {
    UiVertexOut out;
    float2 pos = in.rect.xy + in.corner * in.rect.zw;
    float2 ndc = (pos / globals.screen_size) * 2.0 - float2(1.0, 1.0);
    out.clip_position = float4(ndc.x, -ndc.y, 0.0, 1.0);
    out.local = in.corner * in.rect.zw;
    out.size = in.rect.zw;
    out.color = in.color;
    out.params = in.params;
    float2 uv_min = in.uv_rect.xy;
    float2 uv_max = in.uv_rect.zw;
    out.uv = mix(uv_min, uv_max, in.corner);
    return out;
}

inline float ui_sdf_rounded_rect(float2 p, float2 half_size, float radius) {
    float2 q = abs(p) - half_size + float2(radius, radius);
    return length(max(q, float2(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

fragment float4 ui_fragment_main(UiVertexOut in [[stage_in]],
                                  constant UiGlobals& globals [[buffer(2)]],
                                  texture2d<float> glyph_atlas [[texture(0)]],
                                  sampler glyph_sampler [[sampler(0)]],
                                  texture2d<float> icon_atlas [[texture(1)]],
                                  sampler icon_sampler [[sampler(1)]]) {
    int kind = int(in.params.z + 0.5);
    float4 glyph = glyph_atlas.sample(glyph_sampler, in.uv);
    float4 icon = icon_atlas.sample(icon_sampler, in.uv);

    if (kind == 1) {
        float2 half_size = in.size * 0.5;
        float2 p = in.local - half_size;
        float radius = in.params.x;
        float border = in.params.y;
        float dist = ui_sdf_rounded_rect(p, half_size, radius);
        float fill_alpha = 1.0 - smoothstep(-1.0, 0.0, dist);
        float border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        float alpha = max(fill_alpha * in.color.a, border_alpha * in.params.w);
        return float4(in.color.rgb, alpha);
    }
    if (kind == 6) {
        float2 half_size = in.size * 0.5;
        float2 p = in.local - half_size;
        float radius = in.params.x;
        float border = in.params.y;
        float dist = ui_sdf_rounded_rect(p, half_size, radius);
        float border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        float two_pi = 6.28318530718;
        float duration = 1.6;
        float phase = globals._pad.x / duration;
        float theta = atan2(p.x, -p.y);
        theta = theta - floor(theta / two_pi) * two_pi;
        float spin = phase * two_pi;
        spin = spin - floor(spin / two_pi) * two_pi;
        float sweep = theta - spin;
        sweep = sweep - floor(sweep / two_pi) * two_pi;
        float comet_alpha = sweep / two_pi;
        float ring_alpha = max(comet_alpha, 0.2);
        float pulse = 0.775 - 0.225 * cos(two_pi * phase);
        float alpha = border_alpha * ring_alpha * pulse * in.color.a;
        return float4(in.color.rgb, alpha);
    }
    if (kind == 7) {
        float2 half_size = in.size * 0.5;
        float2 p = in.local - half_size;
        float radius = in.params.x;
        float border = in.params.y;
        float dist = ui_sdf_rounded_rect(p, half_size, radius);
        float border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        float two_pi = 6.28318530718;
        float duration = 3.2;
        float phase = globals._pad.x / duration;
        float theta = atan2(p.x, -p.y);
        theta = theta - floor(theta / two_pi) * two_pi;
        float spin = phase * two_pi;
        spin = spin - floor(spin / two_pi) * two_pi;
        float sweep = theta - spin;
        sweep = sweep - floor(sweep / two_pi) * two_pi;
        float dash = step(fract(sweep / two_pi * 12.0), 0.6);
        float ring_alpha = max(dash, 0.2);
        float pulse = 0.85 - 0.15 * cos(two_pi * phase);
        float alpha = border_alpha * ring_alpha * pulse * in.color.a;
        return float4(in.color.rgb, alpha);
    }
    if (kind == 8) {
        float2 half_size = in.size * 0.5;
        float2 p = in.local - half_size;
        float radius = in.params.x;
        float border = in.params.y;
        float dist = ui_sdf_rounded_rect(p, half_size, radius);
        float border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        float alpha = border_alpha * in.color.a;
        return float4(in.color.rgb, alpha);
    }
    if (kind == 9) {
        float2 half_size = in.size * 0.5;
        float2 p = in.local - half_size;
        float radius = in.params.x;
        float border = in.params.y;
        float dist = ui_sdf_rounded_rect(p, half_size, radius);
        float border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        float two_pi = 6.28318530718;
        float duration = 1.6;
        float phase = globals._pad.x / duration;
        float pulse = 0.5 - 0.5 * cos(two_pi * phase);
        float alpha = border_alpha * pulse * in.color.a;
        return float4(in.color.rgb, alpha);
    }
    if (kind == 2) {
        return float4(in.color.rgb, glyph.r * in.color.a);
    }
    if (kind == 4 || kind == 5) {
        return float4(icon.rgb * in.color.rgb, icon.a * in.color.a);
    }
    if (kind == 3) {
        return in.color;
    }
    return in.color;
}
"#;

//#endregion 🧵️UiFamily

//#region 🔺️VectorFamily

/// 📐️ Mirrors `ui_render::shader_contract::VECTOR_SHADER`.
pub const VECTOR_SHADER_MSL: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct VectorGlobals {
    float2 screen_size;
    float2 _pad;
};

struct VectorVertexIn {
    float2 position [[attribute(0)]];
    float4 color [[attribute(1)]];
};

struct VectorVertexOut {
    float4 clip_position [[position]];
    float4 color;
};

vertex VectorVertexOut vector_vertex_main(VectorVertexIn in [[stage_in]],
                                           constant VectorGlobals& globals [[buffer(2)]]) {
    VectorVertexOut out;
    float2 ndc = (in.position / globals.screen_size) * 2.0 - float2(1.0, 1.0);
    out.clip_position = float4(ndc.x, -ndc.y, 0.0, 1.0);
    out.color = in.color;
    return out;
}

fragment float4 vector_fragment_main(VectorVertexOut in [[stage_in]]) {
    return in.color;
}
"#;

//#endregion 🔺️VectorFamily

//#region 🌐️World3dFamily

/// 🗻️ Mirrors `ui_render::shader_contract::WORLD3D_SHADER` (opaque + translucent share this).
pub const WORLD3D_MESH_SHADER_MSL: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct WorldGlobals {
    float4x4 view_proj;
    float4x4 shadow_view_proj;
    float4 camera_position;
    float4 light_dir;
    float4 ambient;
    float4 sun;
    float4 material;
    float4 material_emissive;
    float4 shadow;
};

struct WorldMeshVertexIn {
    float3 position [[attribute(0)]];
    float3 normal [[attribute(1)]];
    float4 vertex_color [[attribute(2)]];
    float4 model0 [[attribute(3)]];
    float4 model1 [[attribute(4)]];
    float4 model2 [[attribute(5)]];
    float4 model3 [[attribute(6)]];
    float4 color [[attribute(7)]];
    float4 flags [[attribute(8)]];
};

struct WorldMeshVertexOut {
    float4 clip_position [[position]];
    float4 color;
    float3 normal;
    float4 flags;
    float3 world_position;
};

vertex WorldMeshVertexOut world3d_mesh_vertex_main(WorldMeshVertexIn in [[stage_in]],
                                                     constant WorldGlobals& globals [[buffer(2)]]) {
    WorldMeshVertexOut out;
    float4x4 model = float4x4(in.model0, in.model1, in.model2, in.model3);
    float4 world_pos = model * float4(in.position, 1.0);
    out.clip_position = globals.view_proj * world_pos;
    float3x3 model_linear = float3x3(model[0].xyz, model[1].xyz, model[2].xyz);
    float3x3 cofactor = float3x3(cross(model[1].xyz, model[2].xyz), cross(model[2].xyz, model[0].xyz), cross(model[0].xyz, model[1].xyz));
    float determinant = dot(model[0].xyz, cofactor[0]);
    float3 transformed_normal = model_linear * in.normal;
    if (abs(determinant) > 0.0000001) transformed_normal = (cofactor * in.normal) / determinant;
    float transformed_length_squared = dot(transformed_normal, transformed_normal);
    if (!(transformed_length_squared > 0.000000000001 && transformed_length_squared < 3.402823e38)) transformed_normal = in.normal;
    out.normal = normalize(transformed_normal);
    float vertex_weight = (uint(in.flags.x) & 1u) != 0u ? 1.0 : 0.0;
    out.color = float4(in.color.rgb * mix(float3(1.0), in.vertex_color.rgb, vertex_weight), in.color.a * mix(1.0, in.vertex_color.a, vertex_weight));
    out.flags = in.flags;
    out.world_position = world_pos.xyz;
    return out;
}

constant float WORLD3D_RECIPROCAL_PI = 0.31830989;
constant float WORLD3D_HEMISPHERE_INTENSITY = 1.35;
constant float3 WORLD3D_HEMISPHERE_GROUND = float3(0.32314321, 0.3515326, 0.40724021);
constant float2 WORLD3D_DFG_LUT[256] = {
    float2(0.147094727, 0.852050781), float2(0.165527344, 0.787597656), float2(0.244384766, 0.638671875), float2(0.370849609, 0.51953125),
    float2(0.496826172, 0.415527344), float2(0.602050781, 0.326416016), float2(0.684082031, 0.25390625), float2(0.74609375, 0.197509766),
    float2(0.790527344, 0.154296875), float2(0.822265625, 0.121643066), float2(0.843261719, 0.0969848633), float2(0.856445312, 0.0784301758),
    float2(0.86328125, 0.0643920898), float2(0.865722656, 0.0537109375), float2(0.864257812, 0.0454406738), float2(0.859863281, 0.0390319824),
    float2(0.388671875, 0.611328125), float2(0.393066406, 0.600585938), float2(0.412353516, 0.545898438), float2(0.456542969, 0.448242188),
    float2(0.527832031, 0.352539062), float2(0.607421875, 0.273925781), float2(0.678710938, 0.211425781), float2(0.733398438, 0.162597656),
    float2(0.770996094, 0.125366211), float2(0.793457031, 0.0972900391), float2(0.803222656, 0.0762329102), float2(0.803710938, 0.0603637695),
    float2(0.796386719, 0.0484313965), float2(0.785644531, 0.0393676758), float2(0.771972656, 0.032409668), float2(0.754882812, 0.0269775391),
    float2(0.572265625, 0.427490234), float2(0.573730469, 0.424072266), float2(0.579589844, 0.403564453), float2(0.591796875, 0.354492188),
    float2(0.616210938, 0.288085938), float2(0.655273438, 0.224853516), float2(0.698730469, 0.172607422), float2(0.735351562, 0.131835938),
    float2(0.759277344, 0.100891113), float2(0.770019531, 0.0774536133), float2(0.771972656, 0.0599365234), float2(0.766113281, 0.0468444824),
    float2(0.751953125, 0.0369873047), float2(0.732421875, 0.0295410156), float2(0.709472656, 0.0238342285), float2(0.68359375, 0.0194396973),
    float2(0.708984375, 0.291015625), float2(0.708984375, 0.289794922), float2(0.709960938, 0.28125), float2(0.709960938, 0.258544922),
    float2(0.711425781, 0.220458984), float2(0.719726562, 0.176879883), float2(0.734375, 0.137084961), float2(0.748046875, 0.104797363),
    float2(0.755859375, 0.0798950195), float2(0.759765625, 0.0610046387), float2(0.753417969, 0.0468444824), float2(0.738769531, 0.0362243652),
    float2(0.717773438, 0.0282592773), float2(0.691894531, 0.0222625732), float2(0.661132812, 0.0177001953), float2(0.628417969, 0.0141983032),
    float2(0.808105469, 0.191772461), float2(0.807617188, 0.19128418), float2(0.806152344, 0.187988281), float2(0.801757812, 0.178100586),
    float2(0.79296875, 0.158691406), float2(0.783691406, 0.132202148), float2(0.775390625, 0.104858398), float2(0.768554688, 0.0811157227),
    float2(0.764648438, 0.0619812012), float2(0.755371094, 0.0472717285), float2(0.740234375, 0.0361633301), float2(0.71875, 0.0277557373),
    float2(0.690917969, 0.021484375), float2(0.658203125, 0.0167388916), float2(0.622070312, 0.0131607056), float2(0.583984375, 0.0104141235),
    float2(0.878417969, 0.121704102), float2(0.877929688, 0.121704102), float2(0.875, 0.120605469), float2(0.869140625, 0.116943359),
    float2(0.856933594, 0.108032227), float2(0.837890625, 0.09375), float2(0.814941406, 0.0769042969), float2(0.795898438, 0.0606994629),
    float2(0.776367188, 0.046875), float2(0.756347656, 0.0359191895), float2(0.732421875, 0.0274505615), float2(0.703125, 0.0210266113),
    float2(0.668945312, 0.0161743164), float2(0.630371094, 0.012512207), float2(0.589355469, 0.00974273682), float2(0.546386719, 0.00763320923),
    float2(0.926269531, 0.0737915039), float2(0.92578125, 0.0739135742), float2(0.922851562, 0.0739135742), float2(0.916992188, 0.0731201172),
    float2(0.903808594, 0.0698242188), float2(0.881347656, 0.0631103516), float2(0.851074219, 0.0538024902), float2(0.821289062, 0.0437011719),
    float2(0.791015625, 0.0343933105), float2(0.761230469, 0.0266113281), float2(0.728027344, 0.0204467773), float2(0.691894531, 0.0156555176),
    float2(0.650878906, 0.012008667), float2(0.607421875, 0.00925445557), float2(0.561035156, 0.00715637207), float2(0.514160156, 0.00556564331),
    float2(0.957519531, 0.0423278809), float2(0.95703125, 0.0424499512), float2(0.954589844, 0.0428161621), float2(0.94921875, 0.043182373),
    float2(0.937011719, 0.0426635742), float2(0.913085938, 0.0402526855), float2(0.881835938, 0.0357971191), float2(0.844726562, 0.0301361084),
    float2(0.806152344, 0.0243377686), float2(0.767089844, 0.0191497803), float2(0.7265625, 0.0148544312), float2(0.682617188, 0.0114212036),
    float2(0.636230469, 0.00877380371), float2(0.586914062, 0.00674057007), float2(0.536621094, 0.00519943237), float2(0.486083984, 0.00402069092),
    float2(0.977539062, 0.0226287842), float2(0.977050781, 0.0227508545), float2(0.975097656, 0.0231933594), float2(0.969726562, 0.0239105225),
    float2(0.959472656, 0.0244903564), float2(0.936035156, 0.0241241455), float2(0.905273438, 0.0225219727), float2(0.865234375, 0.0197601318),
    float2(0.821777344, 0.0165100098), float2(0.774414062, 0.0132904053), float2(0.7265625, 0.0104598999), float2(0.676269531, 0.00813293457),
    float2(0.624023438, 0.0062789917), float2(0.569824219, 0.00482559204), float2(0.515136719, 0.00371360779), float2(0.461425781, 0.0028629303),
    float2(0.988769531, 0.0110702515), float2(0.988769531, 0.0111618042), float2(0.986816406, 0.0115127563), float2(0.982910156, 0.0122146606),
    float2(0.973144531, 0.0129928589), float2(0.953125, 0.0135192871), float2(0.922851562, 0.0132827759), float2(0.882324219, 0.012260437),
    float2(0.834960938, 0.0106582642), float2(0.783203125, 0.00885009766), float2(0.728515625, 0.0071144104), float2(0.671875, 0.00560760498),
    float2(0.613769531, 0.00436019897), float2(0.5546875, 0.00337219238), float2(0.496337891, 0.00259971619), float2(0.439453125, 0.00200462341),
    float2(0.995117188, 0.00479888916), float2(0.995117188, 0.00486373901), float2(0.993652344, 0.00509643555), float2(0.990234375, 0.00560379028),
    float2(0.981445312, 0.00633239746), float2(0.964355469, 0.0069770813), float2(0.936035156, 0.00729751587), float2(0.896484375, 0.00712585449),
    float2(0.846679688, 0.00649261475), float2(0.791503906, 0.00559616089), float2(0.731445312, 0.00462722778), float2(0.668945312, 0.00371742249),
    float2(0.60546875, 0.0029296875), float2(0.541503906, 0.00228118896), float2(0.479248047, 0.00176620483), float2(0.419677734, 0.00136566162),
    float2(0.998046875, 0.00176048279), float2(0.998046875, 0.00179386139), float2(0.996582031, 0.00192928314), float2(0.994140625, 0.00223922729),
    float2(0.986328125, 0.00272941589), float2(0.971679688, 0.00325012207), float2(0.945800781, 0.00366973877), float2(0.907714844, 0.00381851196),
    float2(0.858398438, 0.00368118286), float2(0.799316406, 0.00332069397), float2(0.735351562, 0.00284385681), float2(0.667480469, 0.00234413147),
    float2(0.598632812, 0.00187969208), float2(0.530273438, 0.00148296356), float2(0.464111328, 0.00115871429), float2(0.402099609, 0.00089931488),
    float2(0.999511719, 0.000501155853), float2(0.999511719, 0.000515460968), float2(0.998046875, 0.000583648682), float2(0.996582031, 0.000750541687),
    float2(0.989257812, 0.00101470947), float2(0.976074219, 0.00134658813), float2(0.952636719, 0.00165271759), float2(0.916015625, 0.00185585022),
    float2(0.8671875, 0.00190544128), float2(0.807617188, 0.00181674957), float2(0.739257812, 0.00162124634), float2(0.666992188, 0.00137996674),
    float2(0.593261719, 0.00113582611), float2(0.520019531, 0.000912189484), float2(0.450439453, 0.000721931458), float2(0.385986328, 0.000565052032),
    float2(1.0, 9.31620598e-05), float2(1.0, 9.78708267e-05), float2(0.999023438, 0.000125408173), float2(0.997070312, 0.000192165375),
    float2(0.990722656, 0.00031042099), float2(0.979003906, 0.000469923019), float2(0.957519531, 0.000647068024), float2(0.923339844, 0.000791549683),
    float2(0.875488281, 0.000876903534), float2(0.814941406, 0.000886917114), float2(0.744140625, 0.000832557678), float2(0.667480469, 0.000738620758),
    float2(0.588378906, 0.000626564026), float2(0.511230469, 0.000516891479), float2(0.438232422, 0.000416517258), float2(0.37109375, 0.000331163406),
    float2(1.0, 7.27176666e-06), float2(1.0, 8.16583633e-06), float2(0.999023438, 1.69873238e-05), float2(0.997558594, 3.79085541e-05),
    float2(0.9921875, 7.59363174e-05), float2(0.981445312, 0.000137448311), float2(0.961425781, 0.000207543373), float2(0.929199219, 0.00028014183),
    float2(0.8828125, 0.000334501266), float2(0.821777344, 0.000362634659), float2(0.749023438, 0.000362157822), float2(0.668457031, 0.000338077545),
    float2(0.585449219, 0.000299692154), float2(0.50390625, 0.000255823135), float2(0.427001953, 0.000211715698), float2(0.357666016, 0.000172019005),
    float2(1.0, 0.0), float2(1.0, 5.96046448e-08), float2(0.999511719, 1.25169754e-06), float2(0.997558594, 5.30481339e-06),
    float2(0.993164062, 1.50799751e-05), float2(0.982910156, 2.85506248e-05), float2(0.964355469, 4.74452972e-05), float2(0.934082031, 6.84261322e-05),
    float2(0.889160156, 8.893013e-05), float2(0.828125, 0.000104248524), float2(0.75390625, 0.000112175941), float2(0.670410156, 0.00011241436),
    float2(0.583007812, 0.000106275082), float2(0.497070312, 9.58442688e-05), float2(0.416992188, 8.33272934e-05), float2(0.345214844, 7.05122948e-05),
};

static inline float2 world3d_dfg(float roughness, float dot_normal) {
    float2 coordinate = clamp(float2(roughness, dot_normal), 0.0, 1.0) * 16.0 - 0.5;
    float2 lower = floor(coordinate);
    float2 fraction = coordinate - lower;
    uint x0 = uint(clamp(lower.x, 0.0, 15.0));
    uint y0 = uint(clamp(lower.y, 0.0, 15.0));
    uint x1 = uint(clamp(lower.x + 1.0, 0.0, 15.0));
    uint y1 = uint(clamp(lower.y + 1.0, 0.0, 15.0));
    float2 a = mix(WORLD3D_DFG_LUT[y0 * 16 + x0], WORLD3D_DFG_LUT[y0 * 16 + x1], fraction.x);
    float2 b = mix(WORLD3D_DFG_LUT[y1 * 16 + x0], WORLD3D_DFG_LUT[y1 * 16 + x1], fraction.x);
    return mix(a, b, fraction.y);
}

static inline float3 world3d_f_schlick(float3 f0, float dot_view_half) {
    float fresnel = exp2((-5.55473 * dot_view_half - 6.98316) * dot_view_half);
    return f0 * (1.0 - fresnel) + fresnel;
}

static inline float world3d_v_ggx_smith_correlated(float alpha, float dot_normal_light, float dot_normal_view) {
    float alpha_squared = alpha * alpha;
    float gv = dot_normal_light * sqrt(alpha_squared + (1.0 - alpha_squared) * dot_normal_view * dot_normal_view);
    float gl = dot_normal_view * sqrt(alpha_squared + (1.0 - alpha_squared) * dot_normal_light * dot_normal_light);
    return 0.5 / max(gv + gl, 0.000001);
}

static inline float world3d_d_ggx(float alpha, float dot_normal_half) {
    float alpha_squared = alpha * alpha;
    float denominator = dot_normal_half * dot_normal_half * (alpha_squared - 1.0) + 1.0;
    return WORLD3D_RECIPROCAL_PI * alpha_squared / (denominator * denominator);
}

static inline float3 world3d_brdf_ggx(float3 light, float3 view, float3 normal, float3 f0, float roughness) {
    float3 half_direction = normalize(light + view);
    float dot_normal_light = clamp(dot(normal, light), 0.0, 1.0);
    float dot_normal_view = clamp(dot(normal, view), 0.0, 1.0);
    float dot_normal_half = clamp(dot(normal, half_direction), 0.0, 1.0);
    float dot_view_half = clamp(dot(view, half_direction), 0.0, 1.0);
    float alpha = roughness * roughness;
    return world3d_f_schlick(f0, dot_view_half) * (world3d_v_ggx_smith_correlated(alpha, dot_normal_light, dot_normal_view) * world3d_d_ggx(alpha, dot_normal_half));
}

static inline float3 world3d_brdf_ggx_multiscatter(float3 light, float3 view, float3 normal, float3 f0, float roughness) {
    float3 single_scatter = world3d_brdf_ggx(light, view, normal, f0, roughness);
    float dot_normal_light = clamp(dot(normal, light), 0.0, 1.0);
    float dot_normal_view = clamp(dot(normal, view), 0.0, 1.0);
    float2 dfg_view = world3d_dfg(roughness, dot_normal_view);
    float2 dfg_light = world3d_dfg(roughness, dot_normal_light);
    float3 fss_ess_view = f0 * dfg_view.x + dfg_view.y;
    float3 fss_ess_light = f0 * dfg_light.x + dfg_light.y;
    float ems_view = 1.0 - dfg_view.x - dfg_view.y;
    float ems_light = 1.0 - dfg_light.x - dfg_light.y;
    float3 fresnel_average = f0 + (float3(1.0) - f0) * 0.047619;
    float3 multiple_scatter = fss_ess_view * fss_ess_light * fresnel_average / (float3(1.0) - ems_view * ems_light * fresnel_average + 0.000001);
    return single_scatter + multiple_scatter * (ems_view * ems_light);
}

static inline float3 world3d_direct_light(float3 n, float3 v, float3 l, float3 radiance, float3 base_color, float metalness, float roughness) {
    float3 irradiance = radiance * max(dot(n, l), 0.0);
    float3 f0 = mix(float3(0.04), base_color, metalness);
    float3 diffuse = base_color * (1.0 - metalness) * WORLD3D_RECIPROCAL_PI;
    return irradiance * (diffuse + world3d_brdf_ggx_multiscatter(l, v, n, f0, roughness));
}

static inline float3 world3d_rrt_and_odt_fit(float3 value) {
    float3 a = value * (value + 0.0245786) - 0.000090537;
    float3 b = value * (0.983729 * value + 0.4329510) + 0.238081;
    return a / b;
}
static inline float3 world3d_output_transform(float3 linear_rgb) {
    float3x3 input_matrix = float3x3(float3(0.59719, 0.07600, 0.02840), float3(0.35458, 0.90834, 0.13383), float3(0.04823, 0.01566, 0.83777));
    float3x3 output_matrix = float3x3(float3(1.60475, -0.10208, -0.00327), float3(-0.53108, 1.10813, -0.07276), float3(-0.07367, -0.00605, 1.07602));
    return clamp(output_matrix * world3d_rrt_and_odt_fit(input_matrix * (linear_rgb / 0.6)), float3(0.0), float3(1.0));
}

static inline float3 world3d_linear_to_srgb(float3 linear_rgb) {
    float3 high = pow(linear_rgb, float3(0.41666)) * 1.055 - float3(0.055);
    float3 low = linear_rgb * 12.92;
    return select(high, low, linear_rgb <= float3(0.0031308));
}

static inline float3 world3d_attachment_output(float3 linear_rgb, constant WorldGlobals& globals) {
    float3 display_linear = world3d_output_transform(linear_rgb);
    return mix(display_linear, world3d_linear_to_srgb(display_linear), clamp(globals.shadow.w, 0.0, 1.0));
}

static inline float3 world3d_lighting(float3 n, float3 v, float3 base_color, float metalness, float roughness, constant WorldGlobals& globals) {
    float3 indirect = globals.ambient.rgb * globals.ambient.a;
    float3 direct = float3(0.0);
    if (globals.material.w > 0.5) {
        direct = world3d_direct_light(n, v, normalize(globals.light_dir.xyz), globals.sun.rgb * globals.sun.a, base_color, metalness, roughness);
    } else {
        float hemisphere = 0.5 * dot(n, float3(0.0, 0.0, 1.0)) + 0.5;
        indirect += mix(WORLD3D_HEMISPHERE_GROUND, float3(1.0), hemisphere) * WORLD3D_HEMISPHERE_INTENSITY;
        direct += world3d_direct_light(n, v, normalize(float3(12.0, 18.0, 10.0)), float3(2.4), base_color, metalness, roughness);
        direct += world3d_direct_light(n, v, normalize(float3(-14.0, -10.0, 6.0)), float3(1.2), base_color, metalness, roughness);
        direct += world3d_direct_light(n, v, normalize(float3(0.0, 0.0, -16.0)), float3(0.75), base_color, metalness, roughness);
    }
    return indirect * base_color * (1.0 - metalness) * WORLD3D_RECIPROCAL_PI + direct;
}

fragment float4 world3d_mesh_fragment_main(WorldMeshVertexOut in [[stage_in]],
                                            constant WorldGlobals& globals [[buffer(2)]]) {
    float3 n = normalize(in.normal);
    float3 v = normalize(globals.camera_position.xyz - in.world_position);
    float metalness = clamp(in.flags.z, 0.0, 1.0);
    float3 normal_derivative = max(abs(dfdx(in.normal)), abs(dfdy(in.normal)));
    float geometry_roughness = max(max(normal_derivative.x, normal_derivative.y), normal_derivative.z);
    float roughness = min(max(in.flags.w, 0.0525) + geometry_roughness, 1.0);
    float3 emissive = globals.material_emissive.rgb * globals.material.z + in.color.rgb * max(in.flags.y, 0.0);
    float3 color = world3d_lighting(n, v, in.color.rgb, metalness, roughness, globals) + emissive;
    return float4(world3d_attachment_output(color, globals), in.color.a);
}
"#;

/// ➰️ Mirrors `ui_render::shader_contract::WORLD3D_LINES_SHADER`.
pub const WORLD3D_LINES_SHADER_MSL: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct WorldGlobals {
    float4x4 view_proj;
    float4x4 shadow_view_proj;
    float4 camera_position;
    float4 light_dir;
    float4 ambient;
    float4 sun;
    float4 material;
    float4 material_emissive;
    float4 shadow;
};

struct WorldLineVertexIn {
    float3 position [[attribute(0)]];
    float4 color [[attribute(1)]];
};

struct WorldLineVertexOut {
    float4 clip_position [[position]];
    float4 color;
};

vertex WorldLineVertexOut world3d_line_vertex_main(WorldLineVertexIn in [[stage_in]],
                                                     constant WorldGlobals& globals [[buffer(2)]]) {
    WorldLineVertexOut out;
    out.clip_position = globals.view_proj * float4(in.position, 1.0);
    out.color = in.color;
    return out;
}

static inline float3 world3d_rrt_and_odt_fit(float3 value) { float3 a = value * (value + 0.0245786) - 0.000090537; float3 b = value * (0.983729 * value + 0.4329510) + 0.238081; return a / b; }
static inline float3 world3d_output_transform(float3 linear_rgb) {
    float3x3 input_matrix = float3x3(float3(0.59719, 0.07600, 0.02840), float3(0.35458, 0.90834, 0.13383), float3(0.04823, 0.01566, 0.83777));
    float3x3 output_matrix = float3x3(float3(1.60475, -0.10208, -0.00327), float3(-0.53108, 1.10813, -0.07276), float3(-0.07367, -0.00605, 1.07602));
    return clamp(output_matrix * world3d_rrt_and_odt_fit(input_matrix * (linear_rgb / 0.6)), float3(0.0), float3(1.0));
}

static inline float3 world3d_linear_to_srgb(float3 linear_rgb) {
    float3 high = pow(linear_rgb, float3(0.41666)) * 1.055 - float3(0.055);
    float3 low = linear_rgb * 12.92;
    return select(high, low, linear_rgb <= float3(0.0031308));
}

static inline float3 world3d_attachment_output(float3 linear_rgb, constant WorldGlobals& globals) {
    float3 display_linear = world3d_output_transform(linear_rgb);
    return mix(display_linear, world3d_linear_to_srgb(display_linear), clamp(globals.shadow.w, 0.0, 1.0));
}

fragment float4 world3d_line_fragment_main(WorldLineVertexOut in [[stage_in]],
                                            constant WorldGlobals& globals [[buffer(2)]]) {
    return float4(world3d_attachment_output(in.color.rgb, globals), in.color.a);
}
"#;

//#endregion 🌐️World3dFamily

//#region 🌫️BlurFamily

/// 🌫️ Mirrors `ui_render::shader_contract::BLUR_DOWNSAMPLE_SHADER`. Deliberately drops the WGSL
/// constant's `src_mip` uniform: this backend binds `src_tex` as the *whole* mip chain (see this
/// file's header) and passes the source mip directly as an explicit `level(lod)` argument baked into
/// a tiny per-draw uniform instead of a bound texture *view* — same effective sample, one fewer
/// indirection. `get_width(lod)`/`get_height(lod)` give the correct per-level dimensions directly.
pub const BLUR_DOWNSAMPLE_SHADER_MSL: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct BlurVertexOut {
    float4 clip_position [[position]];
    float2 uv;
};

constant float2 kBlurFullscreenPositions[6] = {
    float2(-1.0, -1.0), float2(1.0, -1.0), float2(-1.0, 1.0),
    float2(-1.0, 1.0), float2(1.0, -1.0), float2(1.0, 1.0)
};
constant float2 kBlurFullscreenUvs[6] = {
    float2(0.0, 1.0), float2(1.0, 1.0), float2(0.0, 0.0),
    float2(0.0, 0.0), float2(1.0, 1.0), float2(1.0, 0.0)
};

vertex BlurVertexOut blur_downsample_vertex_main(uint vid [[vertex_id]]) {
    BlurVertexOut out;
    out.clip_position = float4(kBlurFullscreenPositions[vid], 0.0, 1.0);
    out.uv = kBlurFullscreenUvs[vid];
    return out;
}

fragment float4 blur_downsample_fragment_main(BlurVertexOut in [[stage_in]],
                                               texture2d<float> src_tex [[texture(0)]],
                                               sampler src_samp [[sampler(0)]],
                                               constant uint& src_mip [[buffer(0)]]) {
    float2 dim = float2(src_tex.get_width(src_mip), src_tex.get_height(src_mip));
    float2 texel = float2(1.0, 1.0) / dim;
    float2 uv = in.uv;
    float4 c = src_tex.sample(src_samp, uv, level(float(src_mip))) * 4.0;
    c += src_tex.sample(src_samp, uv + float2(-texel.x, 0.0), level(float(src_mip)));
    c += src_tex.sample(src_samp, uv + float2(texel.x, 0.0), level(float(src_mip)));
    c += src_tex.sample(src_samp, uv + float2(0.0, -texel.y), level(float(src_mip)));
    c += src_tex.sample(src_samp, uv + float2(0.0, texel.y), level(float(src_mip)));
    return c / 8.0;
}
"#;

/// 🪟️ Mirrors `ui_render::shader_contract::SCENE_BLIT_SHADER`. Samples mip 0 of the whole scene
/// texture directly (see this file's header note on per-mip views).
pub const SCENE_BLIT_SHADER_MSL: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct SceneBlitVertexOut {
    float4 clip_position [[position]];
    float2 uv;
};

constant float2 kBlitFullscreenPositions[6] = {
    float2(-1.0, -1.0), float2(1.0, -1.0), float2(-1.0, 1.0),
    float2(-1.0, 1.0), float2(1.0, -1.0), float2(1.0, 1.0)
};
constant float2 kBlitFullscreenUvs[6] = {
    float2(0.0, 1.0), float2(1.0, 1.0), float2(0.0, 0.0),
    float2(0.0, 0.0), float2(1.0, 1.0), float2(1.0, 0.0)
};

vertex SceneBlitVertexOut scene_blit_vertex_main(uint vid [[vertex_id]]) {
    SceneBlitVertexOut out;
    out.clip_position = float4(kBlitFullscreenPositions[vid], 0.0, 1.0);
    out.uv = kBlitFullscreenUvs[vid];
    return out;
}

fragment float4 scene_blit_fragment_main(SceneBlitVertexOut in [[stage_in]],
                                          texture2d<float> scene_tex [[texture(0)]],
                                          sampler scene_samp [[sampler(0)]]) {
    return scene_tex.sample(scene_samp, in.uv, level(0.0));
}
"#;

//#endregion 🌫️BlurFamily

//#region 🥂️GlassFamily

/// 🥂️ Mirrors `ui_render::shader_contract::GLASS_SHADER`.
pub const GLASS_SHADER_MSL: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct GlassGlobals {
    float2 screen_size;
    float2 _pad;
};

struct GlassVertexIn {
    float2 corner [[attribute(0)]];
    float4 rect [[attribute(1)]];
    float4 tint [[attribute(2)]];
    float4 params [[attribute(3)]];
};

struct GlassVertexOut {
    float4 clip_position [[position]];
    float2 local;
    float2 size;
    float4 tint;
    float4 params;
    float2 scene_uv;
};

vertex GlassVertexOut glass_vertex_main(GlassVertexIn in [[stage_in]],
                                         constant GlassGlobals& globals [[buffer(2)]]) {
    GlassVertexOut out;
    float2 pos = in.rect.xy + in.corner * in.rect.zw;
    float2 ndc = (pos / globals.screen_size) * 2.0 - float2(1.0, 1.0);
    out.clip_position = float4(ndc.x, -ndc.y, 0.0, 1.0);
    out.local = in.corner * in.rect.zw;
    out.size = in.rect.zw;
    out.tint = in.tint;
    out.params = in.params;
    out.scene_uv = pos / globals.screen_size;
    return out;
}

inline float glass_sdf_rounded_rect(float2 p, float2 half_size, float radius) {
    float2 q = abs(p) - half_size + float2(radius, radius);
    return length(max(q, float2(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

fragment float4 glass_fragment_main(GlassVertexOut in [[stage_in]],
                                     texture2d<float> scene_tex [[texture(1)]],
                                     sampler scene_samp [[sampler(1)]]) {
    float2 half_size = in.size * 0.5;
    float2 p = in.local - half_size;
    float radius = in.params.x;
    float dist = glass_sdf_rounded_rect(p, half_size, radius);
    float fill_alpha = 1.0 - smoothstep(-1.0, 0.0, dist);
    if (fill_alpha <= 0.001) {
        discard_fragment();
    }
    float mip = in.params.z;
    float saturate_amount = in.params.w;
    float tint_alpha = in.params.y;
    float4 blurred = scene_tex.sample(scene_samp, in.scene_uv, level(mip));
    float luma = dot(blurred.rgb, float3(0.2126, 0.7152, 0.0722));
    float3 saturated = mix(float3(luma, luma, luma), blurred.rgb, saturate_amount);
    float3 rgb = mix(saturated, in.tint.rgb, tint_alpha);
    return float4(rgb, fill_alpha);
}
"#;

//#endregion 🥂️GlassFamily

//#endregion 🔖️Msl
