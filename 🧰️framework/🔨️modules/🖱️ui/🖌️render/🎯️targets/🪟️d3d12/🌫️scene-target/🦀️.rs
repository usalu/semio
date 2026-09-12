//! @emoji 🌫️ The offscreen scene-color target + its mip-chain blur scratch texture — the D3D12
//! counterpart of the wgpu target's `SceneColorTarget` and the Metal backend's `🌫️scene_target.rs`,
//! mirroring `GpuContext::render_frame`'s two-pass structure (`🎯️targets/🧊️wgpu/🦀️gpu.rs`): render
//! 2D/3D content into this target, then blur its mip chain and composite glass regions on top before
//! blitting to the real swapchain view.
//!
//! **Fewer objects than the wgpu target, same reasoning as Metal.** `Texture2D::SampleLevel`/
//! `GetDimensions(mip, ...)` in `✨️hlsl.rs` take an explicit mip argument directly against the whole
//! mip chain, and a render-target-view's `Texture2D.MipSlice` picks a render-target mip directly on
//! the original texture — so, like Metal, this struct never allocates a `Vec` of per-mip *sampling*
//! views. It does allocate one RTV per mip (D3D12 RTVs, unlike Metal's `level` property on a shared
//! descriptor, are separate small CPU-only descriptor objects — cheap, five of them, built once).

use crate::types::create_default_texture2d;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT;

//#region 🔖️SceneTarget

/// 🌫️ `SCENE_MIP_LEVELS` in the wgpu target's `draw.rs` — five mip levels of box-downsample give the
/// glass backdrop's blur radius range. Identical to the Metal backend's constant of the same name.
pub const SCENE_MIP_LEVELS: u32 = 5;

type Device = ID3D12Device;

/// 🌫️ Owns the two textures the blur/glass composite pass needs: the scene's own full-mip-chain color
/// target, and a same-shaped scratch texture the blur downsample copies into before reading from it
/// (D3D12, like Metal/wgpu, cannot bind a texture as both a render-target attachment and a shader-read
/// source within the same draw — hence the scratch copy, ported from `SceneColorTarget::
/// copy_mip_to_blur_scratch`/the Metal backend's `run_blur_chain`).
pub struct SceneTarget {
    texture: ID3D12Resource,
    blur_scratch: ID3D12Resource,
    /// 🖼️ One RTV per mip level of `texture` — index `N` targets mip `N`.
    rtv_heap: ID3D12DescriptorHeap,
    rtv_stride: u32,
    /// 🔎️ Two CPU-visible (non-shader-visible) SRVs: index 0 is `texture`'s whole mip chain, index 1
    /// is `blur_scratch`'s. `🪟️backend.rs` copies whichever one a draw needs into the per-frame
    /// shader-visible SRV heap (`📬️frame_buffers.rs::FrameDescriptors`) before that draw, the same
    /// pattern `🗃️resources.rs::ResidentSrvTable` uses for atlas/raster textures.
    srv_heap: ID3D12DescriptorHeap,
    width: u32,
    height: u32,
    format: DXGI_FORMAT,
}

impl SceneTarget {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(device: &Device, width: u32, height: u32, format: DXGI_FORMAT) -> Self {
        let width = width.max(1);
        let height = height.max(1);
        let texture = allocate(device, format, width, height, "scene_color");
        let blur_scratch = allocate(device, format, width, height, "scene_blur_scratch");
        let rtv_heap = build_rtv_heap(device, &texture, format);
        let rtv_stride = unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV) };
        let srv_heap = build_srv_heap(device, &texture, &blur_scratch, format);
        Self { texture, blur_scratch, rtv_heap, rtv_stride, srv_heap, width, height, format }
    }

    /// 🔁️ Recreates both textures (and their RTV/SRV descriptors) only when the requested size
    /// actually changed — mirrors `SceneColorTarget::ensure`'s/Metal's `SceneTarget::ensure`'s early
    /// return. Returns whether a recreation happened, so `D3d12Backend` knows whether to reset its own
    /// per-mip resource-state tracking (a fresh texture is always born in `RENDER_TARGET`; an unchanged
    /// one keeps whatever state the last frame left it in).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn ensure(&mut self, device: &Device, width: u32, height: u32) -> bool {
        let width = width.max(1);
        let height = height.max(1);
        if self.width == width && self.height == height {
            return false;
        }
        self.texture = allocate(device, self.format, width, height, "scene_color");
        self.blur_scratch = allocate(device, self.format, width, height, "scene_blur_scratch");
        self.rtv_heap = build_rtv_heap(device, &self.texture, self.format);
        self.srv_heap = build_srv_heap(device, &self.texture, &self.blur_scratch, self.format);
        self.width = width;
        self.height = height;
        true
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn texture(&self) -> &ID3D12Resource {
        &self.texture
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn blur_scratch(&self) -> &ID3D12Resource {
        &self.blur_scratch
    }

    /// 🖼️ The RTV for `texture`'s mip `level` (`0..SCENE_MIP_LEVELS`).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn rtv(&self, level: u32) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        let start = unsafe { self.rtv_heap.GetCPUDescriptorHandleForHeapStart() };
        D3D12_CPU_DESCRIPTOR_HANDLE { ptr: start.ptr + (level as usize) * (self.rtv_stride as usize) }
    }

    /// 🔎️ The resident (non-shader-visible) SRV for `texture`'s whole mip chain.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn texture_srv_handle(&self, device: &Device) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        srv_handle_at(device, &self.srv_heap, 0)
    }

    /// 🔎️ The resident (non-shader-visible) SRV for `blur_scratch`'s whole mip chain.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn blur_scratch_srv_handle(&self, device: &Device) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        srv_handle_at(device, &self.srv_heap, 1)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn width(&self) -> u32 {
        self.width
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn height(&self) -> u32 {
        self.height
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn allocate(device: &Device, format: DXGI_FORMAT, width: u32, height: u32, label: &str) -> ID3D12Resource {
    let clear = D3D12_CLEAR_VALUE { Format: format, Anonymous: D3D12_CLEAR_VALUE_0 { Color: [0.05, 0.05, 0.06, 1.0] } };
    create_default_texture2d(device, format, width, height, SCENE_MIP_LEVELS as u16, D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET, D3D12_RESOURCE_STATE_RENDER_TARGET, Some(clear), label)
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn build_rtv_heap(device: &Device, texture: &ID3D12Resource, format: DXGI_FORMAT) -> ID3D12DescriptorHeap {
    let desc = D3D12_DESCRIPTOR_HEAP_DESC { Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV, NumDescriptors: SCENE_MIP_LEVELS, Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE, NodeMask: 0 };
    // 🔓️ SAFETY: plain descriptor-heap creation from a stack-local desc.
    let heap: ID3D12DescriptorHeap = unsafe { device.CreateDescriptorHeap(&desc) }.expect("d3d12 backend: failed to allocate the scene target's RTV heap");
    let stride = unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV) };
    let start = unsafe { heap.GetCPUDescriptorHandleForHeapStart() };
    for level in 0..SCENE_MIP_LEVELS {
        let mut view = D3D12_RENDER_TARGET_VIEW_DESC::default();
        view.Format = format;
        view.ViewDimension = D3D12_RTV_DIMENSION_TEXTURE2D;
        view.Anonymous.Texture2D = D3D12_TEX2D_RTV { MipSlice: level, PlaneSlice: 0 };
        let handle = D3D12_CPU_DESCRIPTOR_HANDLE { ptr: start.ptr + (level as usize) * (stride as usize) };
        // 🔓️ SAFETY: `texture` is alive for this call (borrowed for its whole body); `handle` is within
        // this just-created heap's `NumDescriptors: SCENE_MIP_LEVELS` bounds.
        unsafe { device.CreateRenderTargetView(texture, Some(&view), handle) };
    }
    heap
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn build_srv_heap(device: &Device, texture: &ID3D12Resource, blur_scratch: &ID3D12Resource, format: DXGI_FORMAT) -> ID3D12DescriptorHeap {
    let desc = D3D12_DESCRIPTOR_HEAP_DESC { Type: D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, NumDescriptors: 2, Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE, NodeMask: 0 };
    let heap: ID3D12DescriptorHeap = unsafe { device.CreateDescriptorHeap(&desc) }.expect("d3d12 backend: failed to allocate the scene target's SRV heap");
    let stride = unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV) } as usize;
    let start = unsafe { heap.GetCPUDescriptorHandleForHeapStart() };
    for (index, resource) in [texture, blur_scratch].into_iter().enumerate() {
        let mut view = D3D12_SHADER_RESOURCE_VIEW_DESC::default();
        view.Format = format;
        view.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE2D;
        view.Shader4ComponentMapping = D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING;
        view.Anonymous.Texture2D = D3D12_TEX2D_SRV { MostDetailedMip: 0, MipLevels: SCENE_MIP_LEVELS, PlaneSlice: 0, ResourceMinLODClamp: 0.0 };
        let handle = D3D12_CPU_DESCRIPTOR_HANDLE { ptr: start.ptr + index * stride };
        // 🔓️ SAFETY: `resource` (`texture`/`blur_scratch`) is alive for this call; `handle` is within
        // this just-created 2-slot heap's bounds.
        unsafe { device.CreateShaderResourceView(resource, Some(&view), handle) };
    }
    heap
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn srv_handle_at(device: &Device, heap: &ID3D12DescriptorHeap, index: usize) -> D3D12_CPU_DESCRIPTOR_HANDLE {
    let stride = unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV) } as usize;
    let start = unsafe { heap.GetCPUDescriptorHandleForHeapStart() };
    D3D12_CPU_DESCRIPTOR_HANDLE { ptr: start.ptr + index * stride }
}

//#endregion 🔖️SceneTarget
