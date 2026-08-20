//! @emoji 🏗️ The one root signature every pipeline in this backend shares, the static sampler heap,
//! and every `ID3D12PipelineState` this backend needs — built from the hand-written HLSL in
//! `🦀️hlsl.rs` via `D3DCompile`.
//!
//! **Root signature mapping (ticket-specified): the wgpu reference's 5-entry UI bind group
//! (`@binding(0)` uniform, `@binding(1)` glyph texture, `@binding(2)` glyph sampler, `@binding(3)`
//! icon texture, `@binding(4)` icon sampler) maps to 1 root CBV + 2 descriptor tables:**
//!
//! | wgpu `@binding` | root signature landing |
//! |---|---|
//! | 0 — uniform | root param 0, `D3D12_ROOT_PARAMETER_TYPE_CBV` at `b0` — a *root descriptor*, no descriptor object needed; every family (ui/vector/glass globals, world3d view_proj+light_dir, the blur per-mip scalar) shares this one slot, bound to a different GPU virtual address per draw via `SetGraphicsRootConstantBufferView`, exactly mirroring how Metal's `buffer(2)` slot is reused across families. |
//! | 1, 3 — glyph/icon textures | root param 1, a `DESCRIPTOR_TABLE` of one SRV range, `NumDescriptors: 2` at `t0..t1`. D3D12 forbids mixing SRV and Sampler ranges in one table (they live in different heap *types*), which is exactly why this needs to be two tables rather than one two-entry table per texture. |
//! | 2, 4 — glyph/icon samplers | root param 2, a `DESCRIPTOR_TABLE` of one Sampler range, `NumDescriptors: 2` at `s0..s1`. |
//!
//! A pipeline that samples fewer than two textures (glass, blur, blit: one; vector, world3d: zero)
//! still shares this same root signature and simply never reads the unused registers — D3D12 only
//! requires a shader's *used* registers to be covered by the bound root signature, not the reverse.
//!
//! **The SRV table's contents are per-draw** (root param 1 is rebound to a different GPU descriptor
//! handle for nearly every batch — see `🦀️frame_buffers.rs::FrameDescriptors`), but **the Sampler
//! table's contents never change after construction**: this backend needs exactly two sampler
//! *kinds* (linear/no-mip/clamp for glyph+icon; linear/with-mip/clamp for the scene target), so
//! `SamplerHeap` below builds a small shader-visible heap once and hands out two fixed GPU handles —
//! no per-frame sampler writes anywhere in this crate.
//!
//! **Depth/stencil and rasterizer state are baked into each PSO here, not set at encode time.**
//! Unlike Metal (`MTLDepthStencilState` is a separate object bound per-draw, and cull mode/depth bias
//! are pure encoder state — see the Metal backend's `🦀️pipelines.rs`/`🦀️world3d.rs` headers), D3D12's
//! `D3D12_GRAPHICS_PIPELINE_STATE_DESC` carries `DepthStencilState` *and* `RasterizerState` (cull
//! mode, depth bias) as part of one immutable PSO. Concretely this means `world3d_translucent`'s
//! depth bias (`-2`/`-1.0`, matching `WORLD3D_TRANSLUCENT_PIPELINE`) and back-face culling are baked
//! into its own PSO permanently — `🦀️world3d.rs::encode_passes` never pushes/pops bias or cull mode
//! around specific draws the way the Metal backend's does, because `SetPipelineState` already carries
//! that state switch for free. This is the one place this backend's shape is *simpler* than Metal's,
//! not harder — flagged in `📓️terra-backend-d3d12-report.md`'s decisions.

use crate::hlsl::{BLUR_DOWNSAMPLE_SHADER_HLSL, GLASS_SHADER_HLSL, SCENE_BLIT_SHADER_HLSL, UI_SHADER_HLSL, VECTOR_SHADER_HLSL, WORLD3D_LINES_SHADER_HLSL, WORLD3D_MESH_SHADER_HLSL};
use crate::types::{World3dGpuInstance, World3dGpuVertex, WorldLineGpuVertex};
use std::ffi::CString;
use windows::core::{Interface, PCSTR};
use windows::Win32::Graphics::Direct3D::Fxc::D3DCompile;
use windows::Win32::Graphics::Direct3D::ID3DBlob;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT, DXGI_FORMAT_D24_UNORM_S8_UINT, DXGI_FORMAT_R32G32B32A32_FLOAT, DXGI_FORMAT_R32G32B32_FLOAT, DXGI_FORMAT_R32G32_FLOAT, DXGI_SAMPLE_DESC};
use ui_render::{GlassInstance, QuadInstance, VectorVertex};

//#region 🔖️Pipelines

/// 🧊️ The one combined depth+stencil pixel format every pipeline/pass in this backend that touches
/// depth or stencil agrees on — the closest guaranteed-available D3D12 format to the wgpu contract's
/// `Depth24PlusStencil8` (same choice reasoning as Metal's `Depth32Float_Stencil8`: pick a format
/// every conformant device supports rather than guess at hardware-dependent 24-bit depth availability
/// — `DXGI_FORMAT_D24_UNORM_S8_UINT` is guaranteed on every D3D12 feature-level-11_0+ device, checked
/// against the vendored `windows-0.62.2` `Direct3D12/mod.rs` resource-state/format tables).
pub const DEPTH_STENCIL_FORMAT: DXGI_FORMAT = DXGI_FORMAT_D24_UNORM_S8_UINT;

//#region ⚠️Error

#[derive(Debug)]
pub enum PipelineBuildError {
    ShaderCompilationFailed(String),
    DeviceCallFailed(String),
}

//#endregion ⚠️Error

//#region 🧩️ShaderCompile

const ATTRIB: &[u8] = b"ATTRIB\0";

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn attrib_name() -> PCSTR {
    PCSTR(ATTRIB.as_ptr())
}

/// 🛡️ Reads a `D3DCompile` error blob's bytes into a `String` for `PipelineBuildError` — the blob is
/// not null-terminated-guaranteed by contract, so this copies exactly `GetBufferSize()` bytes rather
/// than scanning for a nul.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn blob_to_string(blob: &ID3DBlob) -> String {
    // 🔓️ SAFETY: `GetBufferPointer`/`GetBufferSize` are plain accessors on a live `ID3DBlob` this
    // function borrows for its whole body; the returned pointer is valid for exactly `GetBufferSize()`
    // bytes for the blob's lifetime (documented `ID3DBlob` contract), and `blob` outlives this read.
    unsafe {
        let pointer = blob.GetBufferPointer().cast::<u8>();
        let length = blob.GetBufferSize();
        let bytes = std::slice::from_raw_parts(pointer, length);
        String::from_utf8_lossy(bytes).into_owned()
    }
}

/// 🏗️ Compiles `source`'s `entry` function to `target` (`"vs_5_0"`/`"ps_5_0"`) via `D3DCompile` — the
/// interim shader route this file's header describes. `D3DCompile` is part of this crate's already-
/// declared `Win32_Graphics_Direct3D_Fxc` feature, so this needs no `Cargo.toml` change.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn compile_shader(label: &str, source: &str, entry: &str, target: &str) -> Result<ID3DBlob, PipelineBuildError> {
    let entry_c = CString::new(entry).expect("hlsl entry point names are ASCII literals with no interior nul");
    let target_c = CString::new(target).expect("hlsl shader-model targets are ASCII literals with no interior nul");
    let mut code: Option<ID3DBlob> = None;
    let mut errors: Option<ID3DBlob> = None;
    // 🔓️ SAFETY: `source.as_ptr()`/`source.len()` describe a valid UTF-8 (hence valid ASCII-superset
    // byte) buffer alive for this call; `entry_c`/`target_c` are nul-terminated `CString`s alive for
    // this call; `pdefines`/`pinclude` are `None` (no macros, no `#include`, matching every shader
    // family here being self-contained); `code`/`errors` are valid `&mut Option<ID3DBlob>` out-params
    // D3DCompile fills in before returning, matching the generic `D3D12CreateDevice`-style pattern
    // this crate already relies on elsewhere.
    let result = unsafe { D3DCompile(source.as_ptr().cast(), source.len(), PCSTR::null(), None, None, PCSTR(entry_c.as_ptr().cast()), PCSTR(target_c.as_ptr().cast()), 0, 0, &mut code, Some(&mut errors)) };
    if result.is_err() {
        let message = errors.as_ref().map(blob_to_string).unwrap_or_else(|| "D3DCompile failed with no error blob".to_string());
        return Err(PipelineBuildError::ShaderCompilationFailed(format!("{label} ({entry}): {message}")));
    }
    code.ok_or_else(|| PipelineBuildError::ShaderCompilationFailed(format!("{label} ({entry}): D3DCompile reported success but returned no blob")))
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn shader_bytecode(blob: &ID3DBlob) -> D3D12_SHADER_BYTECODE {
    // 🔓️ SAFETY: same accessor contract as `blob_to_string` — `blob` outlives the
    // `D3D12_GRAPHICS_PIPELINE_STATE_DESC` this bytecode is embedded into, because every call site
    // keeps its `ID3DBlob`s alive on the stack for the whole `CreateGraphicsPipelineState` call.
    unsafe { D3D12_SHADER_BYTECODE { pShaderBytecode: blob.GetBufferPointer(), BytecodeLength: blob.GetBufferSize() } }
}

//#endregion 🧩️ShaderCompile

//#region 🗄️RootSignature

/// 🗄️ Builds the one root signature this backend's every graphics PSO shares — see this file's header
/// for the full 5-entry-wgpu-bind-group mapping.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn build_root_signature(device: &ID3D12Device) -> Result<ID3D12RootSignature, PipelineBuildError> {
    let srv_range = D3D12_DESCRIPTOR_RANGE { RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_SRV, NumDescriptors: 2, BaseShaderRegister: 0, RegisterSpace: 0, OffsetInDescriptorsFromTableStart: 0 };
    let sampler_range = D3D12_DESCRIPTOR_RANGE { RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER, NumDescriptors: 2, BaseShaderRegister: 0, RegisterSpace: 0, OffsetInDescriptorsFromTableStart: 0 };

    let mut cbv_param = D3D12_ROOT_PARAMETER::default();
    cbv_param.ParameterType = D3D12_ROOT_PARAMETER_TYPE_CBV;
    cbv_param.ShaderVisibility = D3D12_SHADER_VISIBILITY_ALL;
    cbv_param.Anonymous.Descriptor = D3D12_ROOT_DESCRIPTOR { ShaderRegister: 0, RegisterSpace: 0 };

    let mut srv_param = D3D12_ROOT_PARAMETER::default();
    srv_param.ParameterType = D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE;
    srv_param.ShaderVisibility = D3D12_SHADER_VISIBILITY_PIXEL;
    srv_param.Anonymous.DescriptorTable = D3D12_ROOT_DESCRIPTOR_TABLE { NumDescriptorRanges: 1, pDescriptorRanges: &srv_range };

    let mut sampler_param = D3D12_ROOT_PARAMETER::default();
    sampler_param.ParameterType = D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE;
    sampler_param.ShaderVisibility = D3D12_SHADER_VISIBILITY_PIXEL;
    sampler_param.Anonymous.DescriptorTable = D3D12_ROOT_DESCRIPTOR_TABLE { NumDescriptorRanges: 1, pDescriptorRanges: &sampler_range };

    let parameters = [cbv_param, srv_param, sampler_param];
    let desc = D3D12_ROOT_SIGNATURE_DESC { NumParameters: parameters.len() as u32, pParameters: parameters.as_ptr(), NumStaticSamplers: 0, pStaticSamplers: std::ptr::null(), Flags: D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT };

    let mut blob: Option<ID3DBlob> = None;
    let mut errors: Option<ID3DBlob> = None;
    // 🔓️ SAFETY: `desc` and the arrays it borrows (`parameters`, `srv_range`, `sampler_range`) are all
    // still on this function's stack for the duration of this single synchronous call — D3D12
    // serializes the root signature description into `blob` before returning, it does not retain
    // `desc`'s pointers past the call.
    let result = unsafe { D3D12SerializeRootSignature(&desc, D3D_ROOT_SIGNATURE_VERSION_1, &mut blob, Some(&mut errors)) };
    if result.is_err() {
        let message = errors.as_ref().map(blob_to_string).unwrap_or_else(|| "D3D12SerializeRootSignature failed with no error blob".to_string());
        return Err(PipelineBuildError::DeviceCallFailed(format!("root signature: {message}")));
    }
    let blob = blob.ok_or_else(|| PipelineBuildError::DeviceCallFailed("root signature: D3D12SerializeRootSignature reported success but returned no blob".to_string()))?;
    // 🔓️ SAFETY: `bytes` borrows `blob`'s buffer for exactly the `CreateRootSignature` call below,
    // which copies the serialized bytes into the device-owned root signature object synchronously.
    let bytes = unsafe { std::slice::from_raw_parts(blob.GetBufferPointer().cast::<u8>(), blob.GetBufferSize()) };
    // 🔓️ SAFETY: `device` is a live device this whole crate holds for its own lifetime; `bytes` is a
    // valid, exactly-sized serialized root signature blob per the check above.
    unsafe { device.CreateRootSignature(0, bytes) }.map_err(|error| PipelineBuildError::DeviceCallFailed(format!("CreateRootSignature: {error:?}")))
}

//#endregion 🗄️RootSignature

//#region 🧵️SamplerHeap

/// 🧵️ The two fixed sampler *kinds* this backend ever needs, built once into a small shader-visible
/// heap and never rewritten — see this file's header for why the Sampler table (unlike the SRV table)
/// needs no per-frame bump allocation. Slots `[0,1]` are `(glyph-style, icon-style)` — identical
/// linear/no-mip/clamp descriptors, mirroring Metal's `glyph_sampler`/`icon_sampler` pair — and slots
/// `[2,3]` are `(scene-style, scene-style)` duplicated so a 2-wide table at either base index is
/// always valid, mirroring Metal's `scene_sampler` (linear, *with* mip filtering, used by
/// glass/blur/blit).
pub struct SamplerHeap {
    heap: ID3D12DescriptorHeap,
}

impl SamplerHeap {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn new(device: &ID3D12Device) -> Result<Self, PipelineBuildError> {
        let desc = D3D12_DESCRIPTOR_HEAP_DESC { Type: D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER, NumDescriptors: 4, Flags: D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE, NodeMask: 0 };
        // 🔓️ SAFETY: plain descriptor-heap creation; `desc` is a stack value valid for the call.
        let heap: ID3D12DescriptorHeap = unsafe { device.CreateDescriptorHeap(&desc) }.map_err(|error| PipelineBuildError::DeviceCallFailed(format!("sampler heap: {error:?}")))?;
        let stride = unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER) } as usize;
        let start = unsafe { heap.GetCPUDescriptorHandleForHeapStart() };

        let no_mip = D3D12_SAMPLER_DESC { Filter: D3D12_FILTER_MIN_MAG_MIP_LINEAR, AddressU: D3D12_TEXTURE_ADDRESS_MODE_CLAMP, AddressV: D3D12_TEXTURE_ADDRESS_MODE_CLAMP, AddressW: D3D12_TEXTURE_ADDRESS_MODE_CLAMP, MipLODBias: 0.0, MaxAnisotropy: 1, ComparisonFunc: D3D12_COMPARISON_FUNC_NONE, BorderColor: [0.0; 4], MinLOD: 0.0, MaxLOD: 0.0 };
        let with_mip = D3D12_SAMPLER_DESC { MaxLOD: f32::MAX, ..no_mip };

        for (index, sampler) in [&no_mip, &no_mip, &with_mip, &with_mip].into_iter().enumerate() {
            let handle = D3D12_CPU_DESCRIPTOR_HANDLE { ptr: start.ptr + index * stride };
            // 🔓️ SAFETY: `handle` is within the just-created heap's `NumDescriptors: 4` bounds
            // (`index` ranges 0..4); `sampler` is a valid stack-borrowed descriptor for the call.
            unsafe { device.CreateSampler(sampler, handle) };
        }
        Ok(Self { heap })
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn heap(&self) -> &ID3D12DescriptorHeap {
        &self.heap
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn gpu_handle_at(&self, device: &ID3D12Device, index: usize) -> D3D12_GPU_DESCRIPTOR_HANDLE {
        let stride = unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER) } as u64;
        let start = unsafe { self.heap.GetGPUDescriptorHandleForHeapStart() };
        D3D12_GPU_DESCRIPTOR_HANDLE { ptr: start.ptr + (index as u64) * stride }
    }

    /// 🖋️ `(glyph_sampler, icon_sampler)` — the UI/vector 2D pipeline family's table base.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn ui_table(&self, device: &ID3D12Device) -> D3D12_GPU_DESCRIPTOR_HANDLE {
        self.gpu_handle_at(device, 0)
    }

    /// 🌫️ `(scene_sampler, scene_sampler)` — the glass/blur/blit pipeline family's table base.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn scene_table(&self, device: &ID3D12Device) -> D3D12_GPU_DESCRIPTOR_HANDLE {
        self.gpu_handle_at(device, 2)
    }
}

//#endregion 🧵️SamplerHeap

//#region 🎨️InputLayouts

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn element(index: u32, format: DXGI_FORMAT, slot: u32, offset: u32, per_instance: bool) -> D3D12_INPUT_ELEMENT_DESC {
    D3D12_INPUT_ELEMENT_DESC {
        SemanticName: attrib_name(),
        SemanticIndex: index,
        Format: format,
        InputSlot: slot,
        AlignedByteOffset: offset,
        InputSlotClass: if per_instance { D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA } else { D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA },
        InstanceDataStepRate: if per_instance { 1 } else { 0 },
    }
}

const F2: DXGI_FORMAT = DXGI_FORMAT_R32G32_FLOAT;
const F3: DXGI_FORMAT = DXGI_FORMAT_R32G32B32_FLOAT;
const F4: DXGI_FORMAT = DXGI_FORMAT_R32G32B32A32_FLOAT;

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn ui_layout() -> Vec<D3D12_INPUT_ELEMENT_DESC> {
    vec![element(0, F2, 0, 0, false), element(1, F4, 1, 0, true), element(2, F4, 1, 16, true), element(3, F4, 1, 32, true), element(4, F4, 1, 48, true)]
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn vector_layout() -> Vec<D3D12_INPUT_ELEMENT_DESC> {
    vec![element(0, F2, 0, 0, false), element(1, F4, 0, 8, false)]
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn glass_layout() -> Vec<D3D12_INPUT_ELEMENT_DESC> {
    vec![element(0, F2, 0, 0, false), element(1, F4, 1, 0, true), element(2, F4, 1, 16, true), element(3, F4, 1, 32, true)]
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn world3d_mesh_layout() -> Vec<D3D12_INPUT_ELEMENT_DESC> {
    vec![
        element(0, F3, 0, 0, false),
        element(1, F3, 0, 12, false),
        element(3, F4, 1, 0, true),
        element(4, F4, 1, 16, true),
        element(5, F4, 1, 32, true),
        element(6, F4, 1, 48, true),
        element(7, F4, 1, 64, true),
        element(8, F4, 1, 80, true),
    ]
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn world3d_line_layout() -> Vec<D3D12_INPUT_ELEMENT_DESC> {
    vec![element(0, F3, 0, 0, false), element(1, F4, 0, 12, false)]
}

//#endregion 🎨️InputLayouts

//#region 🖇️FixedFunctionState

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn no_blend(write_mask: u8) -> D3D12_RENDER_TARGET_BLEND_DESC {
    D3D12_RENDER_TARGET_BLEND_DESC { BlendEnable: false.into(), LogicOpEnable: false.into(), SrcBlend: D3D12_BLEND_ONE, DestBlend: D3D12_BLEND_ZERO, BlendOp: D3D12_BLEND_OP_ADD, SrcBlendAlpha: D3D12_BLEND_ONE, DestBlendAlpha: D3D12_BLEND_ZERO, BlendOpAlpha: D3D12_BLEND_OP_ADD, LogicOp: D3D12_LOGIC_OP_NOOP, RenderTargetWriteMask: write_mask }
}

/// 🎨️ Mirrors `wgpu::BlendState::ALPHA_BLENDING` (`SourceAlpha`/`OneMinusSourceAlpha`, op `Add`, both
/// channels) — the same factor pair Metal's `ALPHA_BLEND` constant uses.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn alpha_blend() -> D3D12_RENDER_TARGET_BLEND_DESC {
    D3D12_RENDER_TARGET_BLEND_DESC { BlendEnable: true.into(), LogicOpEnable: false.into(), SrcBlend: D3D12_BLEND_SRC_ALPHA, DestBlend: D3D12_BLEND_INV_SRC_ALPHA, BlendOp: D3D12_BLEND_OP_ADD, SrcBlendAlpha: D3D12_BLEND_SRC_ALPHA, DestBlendAlpha: D3D12_BLEND_INV_SRC_ALPHA, BlendOpAlpha: D3D12_BLEND_OP_ADD, LogicOp: D3D12_LOGIC_OP_NOOP, RenderTargetWriteMask: D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8 }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn blend_desc(target: D3D12_RENDER_TARGET_BLEND_DESC) -> D3D12_BLEND_DESC {
    let mut desc = D3D12_BLEND_DESC::default();
    desc.RenderTarget[0] = target;
    desc
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn rasterizer(cull: D3D12_CULL_MODE, depth_bias: i32, slope_scale: f32) -> D3D12_RASTERIZER_DESC {
    D3D12_RASTERIZER_DESC {
        FillMode: D3D12_FILL_MODE_SOLID,
        CullMode: cull,
        FrontCounterClockwise: false.into(),
        DepthBias: depth_bias,
        DepthBiasClamp: 0.0,
        SlopeScaledDepthBias: slope_scale,
        DepthClipEnable: true.into(),
        MultisampleEnable: false.into(),
        AntialiasedLineEnable: false.into(),
        ForcedSampleCount: 0,
        ConservativeRaster: D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn no_depth_stencil() -> D3D12_DEPTH_STENCIL_DESC {
    D3D12_DEPTH_STENCIL_DESC::default()
}

/// 🩹️ Mirrors `UI_MASK_PIPELINE`'s depth/stencil spec: depth `Always`/no-write, stencil
/// `Always`/`Replace`/`Replace`/`Replace`, both masks `0xff`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn mask_depth_stencil() -> D3D12_DEPTH_STENCIL_DESC {
    let face = D3D12_DEPTH_STENCILOP_DESC { StencilFailOp: D3D12_STENCIL_OP_REPLACE, StencilDepthFailOp: D3D12_STENCIL_OP_REPLACE, StencilPassOp: D3D12_STENCIL_OP_REPLACE, StencilFunc: D3D12_COMPARISON_FUNC_ALWAYS };
    D3D12_DEPTH_STENCIL_DESC { DepthEnable: true.into(), DepthWriteMask: D3D12_DEPTH_WRITE_MASK_ZERO, DepthFunc: D3D12_COMPARISON_FUNC_ALWAYS, StencilEnable: true.into(), StencilReadMask: D3D12_DEFAULT_STENCIL_READ_MASK as u8, StencilWriteMask: D3D12_DEFAULT_STENCIL_WRITE_MASK as u8, FrontFace: face, BackFace: face }
}

/// 🔒 Mirrors `UI_CONTENT_PIPELINE`/`VECTOR_PIPELINE`'s depth/stencil spec: depth `Always`/no-write,
/// stencil `Equal`/`Keep`, read mask `0xff`, write mask `0x00`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn content_depth_stencil() -> D3D12_DEPTH_STENCIL_DESC {
    let face = D3D12_DEPTH_STENCILOP_DESC { StencilFailOp: D3D12_STENCIL_OP_KEEP, StencilDepthFailOp: D3D12_STENCIL_OP_KEEP, StencilPassOp: D3D12_STENCIL_OP_KEEP, StencilFunc: D3D12_COMPARISON_FUNC_EQUAL };
    D3D12_DEPTH_STENCIL_DESC { DepthEnable: true.into(), DepthWriteMask: D3D12_DEPTH_WRITE_MASK_ZERO, DepthFunc: D3D12_COMPARISON_FUNC_ALWAYS, StencilEnable: true.into(), StencilReadMask: D3D12_DEFAULT_STENCIL_READ_MASK as u8, StencilWriteMask: 0, FrontFace: face, BackFace: face }
}

/// 🗻️ Mirrors `WORLD3D_OPAQUE_PIPELINE`'s depth/stencil spec: depth `Less`, write on; stencil `Equal`/
/// `Keep`, write mask `0x00` (reads the same silhouette mask the 2D content wrote, never writes it).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn world3d_opaque_depth_stencil() -> D3D12_DEPTH_STENCIL_DESC {
    let face = D3D12_DEPTH_STENCILOP_DESC { StencilFailOp: D3D12_STENCIL_OP_KEEP, StencilDepthFailOp: D3D12_STENCIL_OP_KEEP, StencilPassOp: D3D12_STENCIL_OP_KEEP, StencilFunc: D3D12_COMPARISON_FUNC_EQUAL };
    D3D12_DEPTH_STENCIL_DESC { DepthEnable: true.into(), DepthWriteMask: D3D12_DEPTH_WRITE_MASK_ALL, DepthFunc: D3D12_COMPARISON_FUNC_LESS, StencilEnable: true.into(), StencilReadMask: D3D12_DEFAULT_STENCIL_READ_MASK as u8, StencilWriteMask: 0, FrontFace: face, BackFace: face }
}

/// 🫧️ Mirrors `WORLD3D_TRANSLUCENT_PIPELINE`/`WORLD3D_LINE_PIPELINE`'s shared depth/stencil spec:
/// depth `LessEqual`, no write; same stencil face as the opaque pass.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn world3d_translucent_depth_stencil() -> D3D12_DEPTH_STENCIL_DESC {
    let face = D3D12_DEPTH_STENCILOP_DESC { StencilFailOp: D3D12_STENCIL_OP_KEEP, StencilDepthFailOp: D3D12_STENCIL_OP_KEEP, StencilPassOp: D3D12_STENCIL_OP_KEEP, StencilFunc: D3D12_COMPARISON_FUNC_EQUAL };
    D3D12_DEPTH_STENCIL_DESC { DepthEnable: true.into(), DepthWriteMask: D3D12_DEPTH_WRITE_MASK_ZERO, DepthFunc: D3D12_COMPARISON_FUNC_LESS_EQUAL, StencilEnable: true.into(), StencilReadMask: D3D12_DEFAULT_STENCIL_READ_MASK as u8, StencilWriteMask: 0, FrontFace: face, BackFace: face }
}

//#endregion 🖇️FixedFunctionState

//#region 📦️Pipelines

/// 📦️ Every compiled `ID3D12PipelineState` this backend replays batches through, plus the shared
/// root signature and sampler heap. Built once in `D3d12Backend::new` against the swapchain's pixel
/// format and the offscreen scene target's pixel format (the two color targets in play — see
/// `🦀️scene_target.rs`).
pub struct Pipelines {
    pub root_signature: ID3D12RootSignature,
    pub samplers: SamplerHeap,
    pub ui_mask: ID3D12PipelineState,
    pub ui_content: ID3D12PipelineState,
    pub vector: ID3D12PipelineState,
    pub glass: ID3D12PipelineState,
    pub blur_downsample: ID3D12PipelineState,
    pub scene_blit: ID3D12PipelineState,
    pub world3d_opaque: ID3D12PipelineState,
    pub world3d_translucent: ID3D12PipelineState,
    pub world3d_line: ID3D12PipelineState,
}

#[allow(clippy::too_many_arguments)]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn build_pso(
    device: &ID3D12Device,
    root_signature: &ID3D12RootSignature,
    label: &str,
    vs: &ID3DBlob,
    ps: &ID3DBlob,
    input_layout: &[D3D12_INPUT_ELEMENT_DESC],
    blend: D3D12_BLEND_DESC,
    rasterizer_state: D3D12_RASTERIZER_DESC,
    depth_stencil: D3D12_DEPTH_STENCIL_DESC,
    topology: D3D12_PRIMITIVE_TOPOLOGY_TYPE,
    render_target_format: DXGI_FORMAT,
    depth_format: DXGI_FORMAT,
) -> Result<ID3D12PipelineState, PipelineBuildError> {
    let mut desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC::default();
    // 🔓️ SAFETY: `ManuallyDrop::new(Some(...))` borrows `root_signature`'s underlying COM pointer via
    // a bitwise copy (`transmute_copy`, no `AddRef`) rather than cloning it — sound because `desc` is
    // consumed by `CreateGraphicsPipelineState` below within this same function, never stored or
    // returned, so the borrow never outlives `root_signature` itself.
    desc.pRootSignature = std::mem::ManuallyDrop::new(Some(unsafe { std::mem::transmute_copy(root_signature) }));
    desc.VS = shader_bytecode(vs);
    desc.PS = shader_bytecode(ps);
    desc.BlendState = blend;
    desc.SampleMask = u32::MAX;
    desc.RasterizerState = rasterizer_state;
    desc.DepthStencilState = depth_stencil;
    desc.InputLayout = D3D12_INPUT_LAYOUT_DESC { pInputElementDescs: if input_layout.is_empty() { std::ptr::null() } else { input_layout.as_ptr() }, NumElements: input_layout.len() as u32 };
    desc.PrimitiveTopologyType = topology;
    desc.NumRenderTargets = 1;
    desc.RTVFormats[0] = render_target_format;
    desc.DSVFormat = depth_format;
    desc.SampleDesc = DXGI_SAMPLE_DESC { Count: 1, Quality: 0 };

    // 🔓️ SAFETY: every pointer `desc` carries (`pRootSignature`, `VS`/`PS` bytecode, `InputLayout`)
    // borrows a value alive for the whole call — `root_signature`/`vs`/`ps` are caller-owned
    // references outliving this call, and `input_layout` is the caller's slice.
    let pso = unsafe { device.CreateGraphicsPipelineState(&desc) }.map_err(|error| PipelineBuildError::DeviceCallFailed(format!("{label}: {error:?}")));
    // 🔓️ SAFETY: the `ManuallyDrop`'d borrowed pointer above is never released automatically (that is
    // the point of `ManuallyDrop`) — dropping it here via `ManuallyDrop::into_inner` runs its `Option`
    // destructor, which would `Release` a COM refcount `root_signature` never actually gained (since
    // the copy above skipped `AddRef`); discarding the value without calling `Release` (`mem::forget`)
    // is therefore the correct match for a call that only ever borrowed the pointer.
    std::mem::forget(std::mem::ManuallyDrop::into_inner(desc.pRootSignature));
    pso
}

impl Pipelines {
    /// 🏗️ `surface_format` is the swapchain's RTV format (`scene_blit`, `glass` target it);
    /// `scene_format` is the offscreen scene color target's format (`blur_downsample` targets it, and
    /// every 2D/3D content pipeline targets it too — mirrors `GpuContext::render_frame`'s two-pass
    /// structure, same as the Metal backend's `Pipelines::new`).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(device: &ID3D12Device, surface_format: DXGI_FORMAT, scene_format: DXGI_FORMAT) -> Result<Self, PipelineBuildError> {
        let root_signature = build_root_signature(device)?;
        let samplers = SamplerHeap::new(device)?;

        let ui_vs = compile_shader("ui_shader", UI_SHADER_HLSL, "ui_vertex_main", "vs_5_0")?;
        let ui_ps = compile_shader("ui_shader", UI_SHADER_HLSL, "ui_fragment_main", "ps_5_0")?;
        let ui_input = ui_layout();

        let ui_mask = build_pso(device, &root_signature, "silhouette_mask_pipeline", &ui_vs, &ui_ps, &ui_input, blend_desc(no_blend(0)), rasterizer(D3D12_CULL_MODE_NONE, 0, 0.0), mask_depth_stencil(), D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE, scene_format, DEPTH_STENCIL_FORMAT)?;
        let ui_content = build_pso(device, &root_signature, "ui_pipeline", &ui_vs, &ui_ps, &ui_input, blend_desc(alpha_blend()), rasterizer(D3D12_CULL_MODE_NONE, 0, 0.0), content_depth_stencil(), D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE, scene_format, DEPTH_STENCIL_FORMAT)?;

        let vector_vs = compile_shader("vector_shader", VECTOR_SHADER_HLSL, "vector_vertex_main", "vs_5_0")?;
        let vector_ps = compile_shader("vector_shader", VECTOR_SHADER_HLSL, "vector_fragment_main", "ps_5_0")?;
        let vector = build_pso(device, &root_signature, "vector_pipeline", &vector_vs, &vector_ps, &vector_layout(), blend_desc(alpha_blend()), rasterizer(D3D12_CULL_MODE_NONE, 0, 0.0), content_depth_stencil(), D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE, scene_format, DEPTH_STENCIL_FORMAT)?;

        let glass_vs = compile_shader("glass_shader", GLASS_SHADER_HLSL, "glass_vertex_main", "vs_5_0")?;
        let glass_ps = compile_shader("glass_shader", GLASS_SHADER_HLSL, "glass_fragment_main", "ps_5_0")?;
        let glass = build_pso(device, &root_signature, "glass_pipeline", &glass_vs, &glass_ps, &glass_layout(), blend_desc(alpha_blend()), rasterizer(D3D12_CULL_MODE_NONE, 0, 0.0), no_depth_stencil(), D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE, surface_format, DXGI_FORMAT(0))?;

        let blur_vs = compile_shader("blur_downsample_shader", BLUR_DOWNSAMPLE_SHADER_HLSL, "blur_downsample_vertex_main", "vs_5_0")?;
        let blur_ps = compile_shader("blur_downsample_shader", BLUR_DOWNSAMPLE_SHADER_HLSL, "blur_downsample_fragment_main", "ps_5_0")?;
        let blur_downsample = build_pso(device, &root_signature, "blur_downsample_pipeline", &blur_vs, &blur_ps, &[], blend_desc(no_blend(D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8)), rasterizer(D3D12_CULL_MODE_NONE, 0, 0.0), no_depth_stencil(), D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE, scene_format, DXGI_FORMAT(0))?;

        let blit_vs = compile_shader("scene_blit_shader", SCENE_BLIT_SHADER_HLSL, "scene_blit_vertex_main", "vs_5_0")?;
        let blit_ps = compile_shader("scene_blit_shader", SCENE_BLIT_SHADER_HLSL, "scene_blit_fragment_main", "ps_5_0")?;
        let scene_blit = build_pso(device, &root_signature, "scene_blit_pipeline", &blit_vs, &blit_ps, &[], blend_desc(no_blend(D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8)), rasterizer(D3D12_CULL_MODE_NONE, 0, 0.0), no_depth_stencil(), D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE, surface_format, DXGI_FORMAT(0))?;

        let world_mesh_vs = compile_shader("world3d_mesh_shader", WORLD3D_MESH_SHADER_HLSL, "world3d_mesh_vertex_main", "vs_5_0")?;
        let world_mesh_ps = compile_shader("world3d_mesh_shader", WORLD3D_MESH_SHADER_HLSL, "world3d_mesh_fragment_main", "ps_5_0")?;
        let world_mesh_layout = world3d_mesh_layout();
        let world3d_opaque = build_pso(device, &root_signature, "world3d_pipeline", &world_mesh_vs, &world_mesh_ps, &world_mesh_layout, blend_desc(no_blend(D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8)), rasterizer(D3D12_CULL_MODE_NONE, 0, 0.0), world3d_opaque_depth_stencil(), D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE, scene_format, DEPTH_STENCIL_FORMAT)?;
        // 🫧️ Depth bias -2/-1.0 mirrors `WORLD3D_TRANSLUCENT_PIPELINE` and is baked into this PSO's
        // `RasterizerState` directly (see this file's header) — `🦀️world3d.rs` never sets it at
        // encode time the way the Metal backend's `encode_passes` does.
        let world3d_translucent = build_pso(device, &root_signature, "world3d_pipeline_translucent", &world_mesh_vs, &world_mesh_ps, &world_mesh_layout, blend_desc(alpha_blend()), rasterizer(D3D12_CULL_MODE_BACK, -2, -1.0), world3d_translucent_depth_stencil(), D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE, scene_format, DEPTH_STENCIL_FORMAT)?;

        let world_line_vs = compile_shader("world3d_lines_shader", WORLD3D_LINES_SHADER_HLSL, "world3d_line_vertex_main", "vs_5_0")?;
        let world_line_ps = compile_shader("world3d_lines_shader", WORLD3D_LINES_SHADER_HLSL, "world3d_line_fragment_main", "ps_5_0")?;
        let world3d_line = build_pso(device, &root_signature, "world3d_line_pipeline", &world_line_vs, &world_line_ps, &world3d_line_layout(), blend_desc(alpha_blend()), rasterizer(D3D12_CULL_MODE_NONE, 0, 0.0), world3d_translucent_depth_stencil(), D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE, scene_format, DEPTH_STENCIL_FORMAT)?;

        Ok(Self { root_signature, samplers, ui_mask, ui_content, vector, glass, blur_downsample, scene_blit, world3d_opaque, world3d_translucent, world3d_line })
    }
}

//#endregion 📦️Pipelines

//#endregion 🔖️Pipelines

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world3d_gpu_instance_layout_matches_the_hlsl_input_layout_strides() {
        assert_eq!(std::mem::size_of::<World3dGpuInstance>(), 96);
        assert_eq!(std::mem::size_of::<World3dGpuVertex>(), 24);
        assert_eq!(std::mem::size_of::<WorldLineGpuVertex>(), 28);
        assert_eq!(std::mem::size_of::<QuadInstance>(), 64);
        assert_eq!(std::mem::size_of::<VectorVertex>(), 24);
        assert_eq!(std::mem::size_of::<GlassInstance>(), 48);
    }

    #[test]
    fn ui_input_layout_has_five_attributes_across_two_slots() {
        let layout = ui_layout();
        assert_eq!(layout.len(), 5);
        assert_eq!(layout[0].InputSlot, 0);
        assert!(layout[1..].iter().all(|element| element.InputSlot == 1));
    }

    #[test]
    fn world3d_mesh_input_layout_has_eight_attributes_across_two_slots() {
        let layout = world3d_mesh_layout();
        assert_eq!(layout.len(), 8);
        assert_eq!(layout[0].InputSlot, 0);
        assert_eq!(layout[1].InputSlot, 0);
        assert!(layout[2..].iter().all(|element| element.InputSlot == 1 && element.InstanceDataStepRate == 1));
    }
}

//#endregion Tests
