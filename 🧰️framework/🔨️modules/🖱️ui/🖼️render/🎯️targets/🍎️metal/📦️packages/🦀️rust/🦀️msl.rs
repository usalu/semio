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
//!   assigned in `🦀️pipelines.rs`'s doc comments alongside each pipeline.

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
    float4 light_dir;
};

struct WorldMeshVertexIn {
    float3 position [[attribute(0)]];
    float3 normal [[attribute(1)]];
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
};

vertex WorldMeshVertexOut world3d_mesh_vertex_main(WorldMeshVertexIn in [[stage_in]],
                                                     constant WorldGlobals& globals [[buffer(2)]]) {
    WorldMeshVertexOut out;
    float4x4 model = float4x4(in.model0, in.model1, in.model2, in.model3);
    float4 world_pos = model * float4(in.position, 1.0);
    out.clip_position = globals.view_proj * world_pos;
    float3x3 normal_matrix = float3x3(model[0].xyz, model[1].xyz, model[2].xyz);
    out.normal = normalize(normal_matrix * in.normal);
    out.color = in.color;
    out.flags = in.flags;
    return out;
}

fragment float4 world3d_mesh_fragment_main(WorldMeshVertexOut in [[stage_in]],
                                            constant WorldGlobals& globals [[buffer(2)]]) {
    float3 n = normalize(in.normal);
    float diffuse = max(dot(n, normalize(globals.light_dir.xyz)), 0.28);
    float3 color = in.color.rgb * diffuse;
    if (in.flags.x > 0.5) {
        color = mix(color, float3(0.35, 0.75, 1.0), 0.65);
    }
    if (in.flags.y > 0.5) {
        color = mix(color, float3(1.0, 0.85, 0.35), 0.55);
    }
    return float4(color, in.color.a);
}
"#;

/// ➰️ Mirrors `ui_render::shader_contract::WORLD3D_LINES_SHADER`.
pub const WORLD3D_LINES_SHADER_MSL: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct WorldGlobals {
    float4x4 view_proj;
    float4 light_dir;
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

fragment float4 world3d_line_fragment_main(WorldLineVertexOut in [[stage_in]]) {
    return in.color;
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
