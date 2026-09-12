//! @emoji ✨️ Hand-written HLSL (Shader Model 5.1, compiled via `D3DCompile`) for the five shader
//! families, ported line-for-line from the canonical WGSL in `ui_render::shader_contract` (packet
//! `shader-repair`'s repaired constants — never the asyncify-corrupted `🎯️targets/🧊️wgpu/🦀️shaders.rs`).
//!
//! **Interim path, not the planned one.** The plan (this ticket's D3D12 section) is build-time `naga`
//! cross-compilation of the canonical WGSL to HLSL via a `build.rs`; that needs a
//! `[build-dependencies] naga` line in this crate's `Cargo.toml`, which is registrar-only (U7) — see
//! `registrar-requests` in `📓️terra-backend-d3d12-report.md`. Until that lands, every family below is
//! hand-transcribed HLSL, each annotated with which WGSL constant it mirrors. Unlike Metal's interim
//! path (MSL compiled at construction time via `MTLDevice::newLibraryWithSource`), this crate's
//! interim compile step is `D3DCompile` — a real Win32 API this crate's declared `🪟️windows` features
//! (`Win32_Graphics_Direct3D_Fxc`) already cover, so no `Cargo.toml` change is needed for *this* step;
//! only the eventual `naga`/`build.rs` swap is registrar-gated.
//!
//! **Binding convention, shared by every family below and by `🏗️pipelines.rs`'s single root
//! signature** (see that file's header for the full 5-entry-wgpu-bind-group → 1 CBV + 2 descriptor
//! table mapping): a small globals `cbuffer` always lives at `register(b0)` (root param 0, a root
//! CBV — no descriptor needed); up to two `Texture2D`s live at `register(t0)`/`register(t1)` (root
//! param 1, an SRV descriptor table); up to two `SamplerState`s live at `register(s0)`/`register(s1)`
//! (root param 2, a sampler descriptor table). A family that needs fewer than two textures/samplers
//! simply never reads the unused registers — the root signature still declares them (unused root
//! parameters are legal; D3D12 only requires a shader's *used* registers to be covered).
//!
//! **Vertex input semantics.** HLSL has no WGSL-style `@location(N)`; every input layout below uses
//! the arbitrary semantic name `ATTRIB` with `SemanticIndex = N`, matched exactly between each
//! `D3D12_INPUT_ELEMENT_DESC` in `🏗️pipelines.rs` and the corresponding `: ATTRIBN` in the struct
//! below — the same location-number correspondence the WGSL contract itself uses, just spelled the
//! HLSL way (semantic name is arbitrary in HLSL; only the name+index *pairing* between layout and
//! shader has to agree, and it does here by construction since both are written by this packet).

//#region 🔖️Hlsl

//#region 🧵️UiFamily

/// 🧊️ Mirrors `ui_render::shader_contract::UI_SHADER`. `sdf_rounded_rect` and every `kind` branch
/// (1,6,7,8,9 rounded/animated rings; 2 glyph; 4/5 icon/raster; 3/default solid) are transcribed
/// verbatim — same constants (`two_pi`, ring durations 1.6/3.2s, dash period 12, pulse curves).
pub const UI_SHADER_HLSL: &str = r#"
cbuffer UiGlobals : register(b0) {
    float2 screen_size;
    float2 _pad;
};
Texture2D glyph_atlas : register(t0);
SamplerState glyph_sampler : register(s0);
Texture2D icon_atlas : register(t1);
SamplerState icon_sampler : register(s1);

struct VSInput {
    float2 corner : ATTRIB0;
    float4 rect : ATTRIB1;
    float4 color : ATTRIB2;
    float4 params : ATTRIB3;
    float4 uv_rect : ATTRIB4;
};

struct PSInput {
    float4 clip_position : SV_POSITION;
    float2 local : TEXCOORD0;
    float2 size : TEXCOORD1;
    float4 color : TEXCOORD2;
    float4 params : TEXCOORD3;
    float2 uv : TEXCOORD4;
};

PSInput ui_vertex_main(VSInput input) {
    PSInput out_v;
    float2 pos = input.rect.xy + input.corner * input.rect.zw;
    float2 ndc = (pos / screen_size) * 2.0 - float2(1.0, 1.0);
    out_v.clip_position = float4(ndc.x, -ndc.y, 0.0, 1.0);
    out_v.local = input.corner * input.rect.zw;
    out_v.size = input.rect.zw;
    out_v.color = input.color;
    out_v.params = input.params;
    float2 uv_min = input.uv_rect.xy;
    float2 uv_max = input.uv_rect.zw;
    out_v.uv = lerp(uv_min, uv_max, input.corner);
    return out_v;
}

float ui_sdf_rounded_rect(float2 p, float2 half_size, float radius) {
    float2 q = abs(p) - half_size + float2(radius, radius);
    return length(max(q, float2(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

float4 ui_fragment_main(PSInput input) : SV_TARGET {
    int kind = (int)(input.params.z + 0.5);
    float4 glyph = glyph_atlas.Sample(glyph_sampler, input.uv);
    float4 icon = icon_atlas.Sample(icon_sampler, input.uv);

    if (kind == 1) {
        float2 half_size = input.size * 0.5;
        float2 p = input.local - half_size;
        float radius = input.params.x;
        float border = input.params.y;
        float dist = ui_sdf_rounded_rect(p, half_size, radius);
        float fill_alpha = 1.0 - smoothstep(-1.0, 0.0, dist);
        float border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        float alpha = max(fill_alpha * input.color.a, border_alpha * input.params.w);
        return float4(input.color.rgb, alpha);
    }
    if (kind == 6) {
        float2 half_size = input.size * 0.5;
        float2 p = input.local - half_size;
        float radius = input.params.x;
        float border = input.params.y;
        float dist = ui_sdf_rounded_rect(p, half_size, radius);
        float border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        float two_pi = 6.28318530718;
        float duration = 1.6;
        float phase = _pad.x / duration;
        float theta = atan2(p.x, -p.y);
        theta = theta - floor(theta / two_pi) * two_pi;
        float spin = phase * two_pi;
        spin = spin - floor(spin / two_pi) * two_pi;
        float sweep = theta - spin;
        sweep = sweep - floor(sweep / two_pi) * two_pi;
        float comet_alpha = sweep / two_pi;
        float ring_alpha = max(comet_alpha, 0.2);
        float pulse = 0.775 - 0.225 * cos(two_pi * phase);
        float alpha = border_alpha * ring_alpha * pulse * input.color.a;
        return float4(input.color.rgb, alpha);
    }
    if (kind == 7) {
        float2 half_size = input.size * 0.5;
        float2 p = input.local - half_size;
        float radius = input.params.x;
        float border = input.params.y;
        float dist = ui_sdf_rounded_rect(p, half_size, radius);
        float border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        float two_pi = 6.28318530718;
        float duration = 3.2;
        float phase = _pad.x / duration;
        float theta = atan2(p.x, -p.y);
        theta = theta - floor(theta / two_pi) * two_pi;
        float spin = phase * two_pi;
        spin = spin - floor(spin / two_pi) * two_pi;
        float sweep = theta - spin;
        sweep = sweep - floor(sweep / two_pi) * two_pi;
        float dash = step(frac(sweep / two_pi * 12.0), 0.6);
        float ring_alpha = max(dash, 0.2);
        float pulse = 0.85 - 0.15 * cos(two_pi * phase);
        float alpha = border_alpha * ring_alpha * pulse * input.color.a;
        return float4(input.color.rgb, alpha);
    }
    if (kind == 8) {
        float2 half_size = input.size * 0.5;
        float2 p = input.local - half_size;
        float radius = input.params.x;
        float border = input.params.y;
        float dist = ui_sdf_rounded_rect(p, half_size, radius);
        float border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        float alpha = border_alpha * input.color.a;
        return float4(input.color.rgb, alpha);
    }
    if (kind == 9) {
        float2 half_size = input.size * 0.5;
        float2 p = input.local - half_size;
        float radius = input.params.x;
        float border = input.params.y;
        float dist = ui_sdf_rounded_rect(p, half_size, radius);
        float border_alpha = 1.0 - smoothstep(border - 1.0, border, abs(dist));
        float two_pi = 6.28318530718;
        float duration = 1.6;
        float phase = _pad.x / duration;
        float pulse = 0.5 - 0.5 * cos(two_pi * phase);
        float alpha = border_alpha * pulse * input.color.a;
        return float4(input.color.rgb, alpha);
    }
    if (kind == 2) {
        return float4(input.color.rgb, glyph.r * input.color.a);
    }
    if (kind == 4 || kind == 5) {
        return float4(icon.rgb * input.color.rgb, icon.a * input.color.a);
    }
    if (kind == 3) {
        return input.color;
    }
    return input.color;
}
"#;

//#endregion 🧵️UiFamily

//#region 🔺️VectorFamily

/// 📐️ Mirrors `ui_render::shader_contract::VECTOR_SHADER`. Reuses the UI globals `cbuffer` layout
/// verbatim (same as the wgpu reference's `vector_pipeline` reusing the UI bind group layout).
pub const VECTOR_SHADER_HLSL: &str = r#"
cbuffer VectorGlobals : register(b0) {
    float2 screen_size;
    float2 _pad;
};

struct VSInput {
    float2 position : ATTRIB0;
    float4 color : ATTRIB1;
};

struct PSInput {
    float4 clip_position : SV_POSITION;
    float4 color : TEXCOORD0;
};

PSInput vector_vertex_main(VSInput input) {
    PSInput out_v;
    float2 ndc = (input.position / screen_size) * 2.0 - float2(1.0, 1.0);
    out_v.clip_position = float4(ndc.x, -ndc.y, 0.0, 1.0);
    out_v.color = input.color;
    return out_v;
}

float4 vector_fragment_main(PSInput input) : SV_TARGET {
    return input.color;
}
"#;

//#endregion 🔺️VectorFamily

//#region 🌐️World3dFamily

/// 🗻️ Mirrors `ui_render::shader_contract::WORLD3D_SHADER` (opaque + translucent share this).
pub const WORLD3D_MESH_SHADER_HLSL: &str = r#"
cbuffer WorldGlobals : register(b0) {
    float4x4 view_proj;
    float4 light_dir;
};

struct VSInput {
    float3 position : ATTRIB0;
    float3 normal : ATTRIB1;
    float4 model0 : ATTRIB3;
    float4 model1 : ATTRIB4;
    float4 model2 : ATTRIB5;
    float4 model3 : ATTRIB6;
    float4 color : ATTRIB7;
    float4 flags : ATTRIB8;
};

struct PSInput {
    float4 clip_position : SV_POSITION;
    float4 color : TEXCOORD0;
    float3 normal : TEXCOORD1;
    float4 flags : TEXCOORD2;
};

PSInput world3d_mesh_vertex_main(VSInput input) {
    PSInput out_v;
    float4x4 model = float4x4(input.model0, input.model1, input.model2, input.model3);
    float4 world_pos = mul(model, float4(input.position, 1.0));
    out_v.clip_position = mul(view_proj, world_pos);
    float3x3 normal_matrix = float3x3(model[0].xyz, model[1].xyz, model[2].xyz);
    out_v.normal = normalize(mul(normal_matrix, input.normal));
    out_v.color = input.color;
    out_v.flags = input.flags;
    return out_v;
}

float4 world3d_mesh_fragment_main(PSInput input) : SV_TARGET {
    float3 n = normalize(input.normal);
    float diffuse = max(dot(n, normalize(light_dir.xyz)), 0.28);
    float3 color = input.color.rgb * diffuse;
    if (input.flags.x > 0.5) {
        color = lerp(color, float3(0.35, 0.75, 1.0), 0.65);
    }
    if (input.flags.y > 0.5) {
        color = lerp(color, float3(1.0, 0.85, 0.35), 0.55);
    }
    return float4(color, input.color.a);
}
"#;

/// ➰️ Mirrors `ui_render::shader_contract::WORLD3D_LINES_SHADER`.
pub const WORLD3D_LINES_SHADER_HLSL: &str = r#"
cbuffer WorldGlobals : register(b0) {
    float4x4 view_proj;
    float4 light_dir;
};

struct VSInput {
    float3 position : ATTRIB0;
    float4 color : ATTRIB1;
};

struct PSInput {
    float4 clip_position : SV_POSITION;
    float4 color : TEXCOORD0;
};

PSInput world3d_line_vertex_main(VSInput input) {
    PSInput out_v;
    out_v.clip_position = mul(view_proj, float4(input.position, 1.0));
    out_v.color = input.color;
    return out_v;
}

float4 world3d_line_fragment_main(PSInput input) : SV_TARGET {
    return input.color;
}
"#;

//#endregion 🌐️World3dFamily

//#region 🌫️BlurFamily

/// 🌫️ Mirrors `ui_render::shader_contract::BLUR_DOWNSAMPLE_SHADER`. `src_tex` is bound as the whole
/// mip chain (root param 1, `t0`) and `src_mip` selects the level explicitly via `SampleLevel`/
/// `GetDimensions(mip, ...)` — same "no per-mip view" simplification the Metal backend documents in
/// its `✨️msl.rs` header, ported here since D3D12's `Texture2D::SampleLevel`/`GetDimensions` support
/// an explicit mip argument exactly like Metal's `sample(..., level(lod))`/`get_width(lod)`.
pub const BLUR_DOWNSAMPLE_SHADER_HLSL: &str = r#"
cbuffer BlurGlobals : register(b0) {
    float src_mip;
    float3 _pad;
};
Texture2D src_tex : register(t0);
SamplerState src_samp : register(s0);

struct PSInput {
    float4 clip_position : SV_POSITION;
    float2 uv : TEXCOORD0;
};

static const float2 kBlurFullscreenPositions[6] = {
    float2(-1.0, -1.0), float2(1.0, -1.0), float2(-1.0, 1.0),
    float2(-1.0, 1.0), float2(1.0, -1.0), float2(1.0, 1.0)
};
static const float2 kBlurFullscreenUvs[6] = {
    float2(0.0, 1.0), float2(1.0, 1.0), float2(0.0, 0.0),
    float2(0.0, 0.0), float2(1.0, 1.0), float2(1.0, 0.0)
};

PSInput blur_downsample_vertex_main(uint vid : SV_VertexID) {
    PSInput out_v;
    out_v.clip_position = float4(kBlurFullscreenPositions[vid], 0.0, 1.0);
    out_v.uv = kBlurFullscreenUvs[vid];
    return out_v;
}

float4 blur_downsample_fragment_main(PSInput input) : SV_TARGET {
    uint mip = (uint)src_mip;
    uint width, height, levels;
    src_tex.GetDimensions(mip, width, height, levels);
    float2 dim = float2(width, height);
    float2 texel = float2(1.0, 1.0) / dim;
    float2 uv = input.uv;
    float4 c = src_tex.SampleLevel(src_samp, uv, src_mip) * 4.0;
    c += src_tex.SampleLevel(src_samp, uv + float2(-texel.x, 0.0), src_mip);
    c += src_tex.SampleLevel(src_samp, uv + float2(texel.x, 0.0), src_mip);
    c += src_tex.SampleLevel(src_samp, uv + float2(0.0, -texel.y), src_mip);
    c += src_tex.SampleLevel(src_samp, uv + float2(0.0, texel.y), src_mip);
    return c / 8.0;
}
"#;

/// 🪟️ Mirrors `ui_render::shader_contract::SCENE_BLIT_SHADER`. Samples mip 0 of the whole scene
/// texture directly (see this file's header / `BLUR_DOWNSAMPLE_SHADER_HLSL`'s doc comment).
pub const SCENE_BLIT_SHADER_HLSL: &str = r#"
Texture2D scene_tex : register(t0);
SamplerState scene_samp : register(s0);

struct PSInput {
    float4 clip_position : SV_POSITION;
    float2 uv : TEXCOORD0;
};

static const float2 kBlitFullscreenPositions[6] = {
    float2(-1.0, -1.0), float2(1.0, -1.0), float2(-1.0, 1.0),
    float2(-1.0, 1.0), float2(1.0, -1.0), float2(1.0, 1.0)
};
static const float2 kBlitFullscreenUvs[6] = {
    float2(0.0, 1.0), float2(1.0, 1.0), float2(0.0, 0.0),
    float2(0.0, 0.0), float2(1.0, 1.0), float2(1.0, 0.0)
};

PSInput scene_blit_vertex_main(uint vid : SV_VertexID) {
    PSInput out_v;
    out_v.clip_position = float4(kBlitFullscreenPositions[vid], 0.0, 1.0);
    out_v.uv = kBlitFullscreenUvs[vid];
    return out_v;
}

float4 scene_blit_fragment_main(PSInput input) : SV_TARGET {
    return scene_tex.SampleLevel(scene_samp, input.uv, 0.0);
}
"#;

//#endregion 🌫️BlurFamily

//#region 🥂️GlassFamily

/// 🥂️ Mirrors `ui_render::shader_contract::GLASS_SHADER`. Group 0 (globals) lands on the same `b0`
/// root CBV every other family uses; group 1 (scene sample) lands on `t0`/`s0` — the glass PSO never
/// declares a second texture/sampler, so `t1`/`s1` (present in the shared root signature, per this
/// file's header) simply go unread.
pub const GLASS_SHADER_HLSL: &str = r#"
cbuffer GlassGlobals : register(b0) {
    float2 screen_size;
    float2 _pad;
};
Texture2D scene_tex : register(t0);
SamplerState scene_samp : register(s0);

struct VSInput {
    float2 corner : ATTRIB0;
    float4 rect : ATTRIB1;
    float4 tint : ATTRIB2;
    float4 params : ATTRIB3;
};

struct PSInput {
    float4 clip_position : SV_POSITION;
    float2 local : TEXCOORD0;
    float2 size : TEXCOORD1;
    float4 tint : TEXCOORD2;
    float4 params : TEXCOORD3;
    float2 scene_uv : TEXCOORD4;
};

PSInput glass_vertex_main(VSInput input) {
    PSInput out_v;
    float2 pos = input.rect.xy + input.corner * input.rect.zw;
    float2 ndc = (pos / screen_size) * 2.0 - float2(1.0, 1.0);
    out_v.clip_position = float4(ndc.x, -ndc.y, 0.0, 1.0);
    out_v.local = input.corner * input.rect.zw;
    out_v.size = input.rect.zw;
    out_v.tint = input.tint;
    out_v.params = input.params;
    out_v.scene_uv = pos / screen_size;
    return out_v;
}

float glass_sdf_rounded_rect(float2 p, float2 half_size, float radius) {
    float2 q = abs(p) - half_size + float2(radius, radius);
    return length(max(q, float2(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

float4 glass_fragment_main(PSInput input) : SV_TARGET {
    float2 half_size = input.size * 0.5;
    float2 p = input.local - half_size;
    float radius = input.params.x;
    float dist = glass_sdf_rounded_rect(p, half_size, radius);
    float fill_alpha = 1.0 - smoothstep(-1.0, 0.0, dist);
    if (fill_alpha <= 0.001) {
        discard;
    }
    float mip = input.params.z;
    float saturate_amount = input.params.w;
    float tint_alpha = input.params.y;
    float4 blurred = scene_tex.SampleLevel(scene_samp, input.scene_uv, mip);
    float luma = dot(blurred.rgb, float3(0.2126, 0.7152, 0.0722));
    float3 saturated = lerp(float3(luma, luma, luma), blurred.rgb, saturate_amount);
    float3 rgb = lerp(saturated, input.tint.rgb, tint_alpha);
    return float4(rgb, fill_alpha);
}
"#;

//#endregion 🥂️GlassFamily

//#endregion 🔖️Hlsl
