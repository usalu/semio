//! @emoji 🪟️ `D3d12Backend`: the concrete `ui_render::GraphicsBackend` implementation for Windows.
//!
//! Milestones reached (see `📓️terra-backend-d3d12-report.md` for the authoritative statement):
//! device + DXGI flip-model swapchain (3 buffers) + direct command queue + per-frame fence (resize
//! incl. zero-size park/restore, `Present`, clear-colour frame); UI quad + vector pipelines with
//! glyph/icon atlas + raster texture upload and per-batch scissor; stencil silhouette clip (mask/
//! content PSO pair, `mask_range` replayed verbatim); offscreen scene target → 5-level mip blur →
//! glass composite → blit; world3d mesh/lines with a real depth buffer and a 256-byte-aligned root-CBV
//! ring; `GetDeviceRemovedReason` → `DeviceStatus::Lost`, `recover()`, `backend-testing`'s
//! `debug_force_device_loss`/`read_back`.
//!
//! `render` replays `RenderPacket` in two device passes, mirroring `GpuContext::render_frame`
//! (`🎯️targets/🧊️wgpu/🦀️gpu.rs`) and the Metal backend's identical two-pass split: an offscreen pass
//! into the scene target (2D backdrop content, then every `SurfacePass`, then 2D backdrop overlay
//! content — see `🦀️world3d.rs` for why world3d is not interleaved layer-by-layer), then a composite
//! pass that blurs the scene's mip chain, blits it to the real swapchain back buffer, composites glass
//! regions on top, and finally paints foreground (glass-content) 2D batches directly onto the back
//! buffer.
//!
//! **Synchronization model, genuinely simpler than a deeply-pipelined engine and worth stating
//! plainly**: one command allocator, one command list, one fence. `render()` waits for the *previous*
//! frame's fence value before resetting and recording the next one — full CPU/GPU serialization per
//! frame, not double/triple buffering of command-list resources. This is a correctness-first decision
//! (see `📓️terra-backend-d3d12-report.md`'s decisions), not an accident: it is exactly what makes the
//! per-frame `FrameDescriptors`/`GrowBuffer` reuse in `🦀️frame_buffers.rs` sound without more
//! bookkeeping (see that file's header).

#[cfg(not(target_os = "windows"))]
compile_error!("semio-framework-ui-backend-d3d12 builds only on Windows.");

use crate::frame_buffers::{FrameBuffers, FrameDescriptors, GrowBuffer};
use crate::pipelines::{Pipelines, DEPTH_STENCIL_FORMAT};
use crate::resources::GpuResources;
use crate::scene_target::{SceneTarget, SCENE_MIP_LEVELS};
use crate::types::{create_default_texture2d, create_upload_buffer, transition_barrier, wait_for_fence_value, BlurMipGpu, UNIT_QUAD_CORNERS, WORLD_GLOBALS_SLOT_SIZE};
use raw_window_handle::{RawWindowHandle, WindowHandle};
use ui_render::{
    BackendError, DeviceCapabilities, DeviceStatus, DrawBatch, FrameStats, GpuTier, GraphicsBackend, LossReason, MemoryClass, PhysicalSize, PipelineKind, QuadInstance, RecoveredResources, RenderPacket, RenderReport, ResourceKind, ResourceOp,
    SurfaceFormat, TextureId, VectorVertex,
};
use windows::core::Interface;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_11_0;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Dxgi::*;

#[cfg(feature = "backend-testing")]
use ui_render::ReadbackImage;

//#region 🔖️Backend

type Device = ID3D12Device;

//#region 🎨️Formats

/// 🖼️ The swapchain's own creation format — DXGI flip-model swapchains reject sRGB formats directly
/// (documented D3D11/D3D12 swapchain restriction; not something the `windows` crate's bindings
/// encode, so unverifiable from vendored source — flagged plainly in
/// `📓️terra-backend-d3d12-report.md`'s "what is unverified without Windows"). `SURFACE_RTV_FORMAT`
/// below is the sRGB *view* this backend creates over the same UNORM buffers, the standard technique
/// for getting automatic sRGB write curves on a flip-model back buffer.
const SURFACE_FORMAT: DXGI_FORMAT = DXGI_FORMAT_B8G8R8A8_UNORM;
const SURFACE_RTV_FORMAT: DXGI_FORMAT = DXGI_FORMAT_B8G8R8A8_UNORM_SRGB;
/// 🌫️ The offscreen scene target's format — plain (non-sRGB), matching the Metal backend's identical
/// choice and identical reasoning: the scene target is read back through `Sample`/`CopyTextureRegion`,
/// not presented directly, so sRGB conversion happens once, at the final `scene_blit_pipeline` write.
const SCENE_FORMAT: DXGI_FORMAT = DXGI_FORMAT_B8G8R8A8_UNORM;
const BUFFER_COUNT: u32 = 3;

//#endregion 🎨️Formats

//#region 🪟️D3d12Backend

/// 🪟️ The concrete Windows `GraphicsBackend`. Every D3D12/DXGI/`windows`-crate type stays behind this
/// crate's own interface — nothing here appears in a public signature outside this crate.
pub struct D3d12Backend {
    device: Device,
    queue: ID3D12CommandQueue,
    swapchain: IDXGISwapChain3,
    allocator: ID3D12CommandAllocator,
    list: ID3D12GraphicsCommandList,
    fence: ID3D12Fence,
    fence_value: u64,
    rtv_heap: ID3D12DescriptorHeap,
    rtv_stride: u32,
    back_buffers: Vec<ID3D12Resource>,
    back_buffer_states: Vec<D3D12_RESOURCE_STATES>,
    size: PhysicalSize,
    dpr: f32,
    pipelines: Pipelines,
    resources: GpuResources,
    scene_target: SceneTarget,
    /// 🚧️ Per-mip tracked state of `scene_target.texture()` — D3D12 has no render-pass-descriptor
    /// load/store model the way Metal does, so this crate tracks every subresource's current state
    /// explicitly and issues exactly the barriers needed each frame. See `run_blur_chain`'s doc
    /// comment for the full per-mip state machine.
    scene_state: [D3D12_RESOURCE_STATES; 5],
    /// 🚧️ `scene_target.blur_scratch()`'s tracked state — one scalar, not per-mip, because every mip
    /// of `blur_scratch` this crate ever touches is processed identically and left in the same state
    /// every frame (see `run_blur_chain`'s doc comment for why that makes a single scalar sound).
    blur_scratch_state: D3D12_RESOURCE_STATES,
    depth_texture: ID3D12Resource,
    dsv_heap: ID3D12DescriptorHeap,
    frame_buffers: FrameBuffers,
    frame_descriptors: FrameDescriptors,
    /// 🌐️ The tiny `{screen_size, _pad}` uniform UI/vector/glass share at root CBV `b0` — rewritten
    /// once per frame by `update_globals`. A plain `GrowBuffer`, not a dedicated field, because a root
    /// CBV bind is just a GPU virtual address — no distinct buffer *type* is needed the way Metal
    /// needed `ui_globals_buffer: Retained<MetalBuffer>` bound at a fixed slot.
    ui_globals: GrowBuffer,
    /// 📐️ The static 6-corner unit-quad input-slot-0 buffer every UI/vector/glass pipeline's vertex
    /// function reads — created once, never rewritten.
    quad_vertex_buffer: ID3D12Resource,
    quad_vertex_view: D3D12_VERTEX_BUFFER_VIEW,
    status: DeviceStatus,
    #[cfg(feature = "backend-testing")]
    forced_loss: bool,
    #[cfg(feature = "backend-testing")]
    readback: Option<(ID3D12Resource, u32)>,
}

//#region Construction

impl D3d12Backend {
    /// 🏗️ Builds a device, a direct command queue, and a flip-model DXGI swapchain attached to
    /// `window_handle`'s `HWND`. Only construction is async per U1 — the body below performs no real
    /// `.await` because D3D12/DXGI device and swapchain creation is synchronous, unlike wgpu's
    /// adapter/device request (same reasoning as the Metal backend's `new`).
    // 🚫️async: U1 — the ONE permitted async fn per the `GraphicsBackend` docstring; construction only.
    pub async fn new(window_handle: WindowHandle<'_>, size: PhysicalSize, dpr: f32) -> Result<Self, BackendError> {
        let RawWindowHandle::Win32(handle) = window_handle.as_raw() else {
            return Err(BackendError::UnsupportedFormat("d3d12 backend requires a Win32 window handle"));
        };
        let hwnd = HWND(handle.hwnd.get() as *mut core::ffi::c_void);
        let (device, queue, factory) = create_device_and_queue()?;
        let swapchain = create_swapchain_for_hwnd(&factory, &queue, hwnd, size.width.max(1), size.height.max(1))?;
        Ok(Self::from_parts(device, queue, swapchain, size, dpr))
    }

    /// 🧪️ A composition-target swapchain (`IDXGIFactory2::CreateSwapChainForComposition`) with no
    /// `HWND` at all — DirectComposition/XAML's own window-free swapchain path, which needs no window
    /// handle and hence no `Win32_UI_WindowsAndMessaging` feature (not declared in this crate's
    /// `Cargo.toml`, and creating a real hidden window for testing would have required it). This
    /// exercises the full render pipeline for `backend-testing` without any window at all. Never used
    /// outside tests/the conformance harness.
    #[cfg(feature = "backend-testing")]
    // 🚫️async: U1 — the ONE permitted async fn per the `GraphicsBackend` docstring; construction only.
    pub async fn new_headless(size: PhysicalSize, dpr: f32) -> Result<Self, BackendError> {
        let (device, queue, factory) = create_device_and_queue()?;
        let swapchain = create_swapchain_for_composition(&factory, &queue, size.width.max(1), size.height.max(1))?;
        Ok(Self::from_parts(device, queue, swapchain, size, dpr))
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from_parts(device: Device, queue: ID3D12CommandQueue, swapchain: IDXGISwapChain3, size: PhysicalSize, dpr: f32) -> Self {
        let allocator: ID3D12CommandAllocator = unsafe { device.CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT) }.expect("d3d12 backend: failed to allocate the render command allocator");
        let list: ID3D12GraphicsCommandList = unsafe { device.CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, &allocator, None) }.expect("d3d12 backend: failed to allocate the render command list");
        unsafe { list.Close() }.expect("d3d12 backend: failed to close the freshly created render command list");
        let fence: ID3D12Fence = unsafe { device.CreateFence(0, D3D12_FENCE_FLAG_NONE) }.expect("d3d12 backend: failed to allocate the frame fence");

        let pipelines = Pipelines::new(&device, SURFACE_RTV_FORMAT, SCENE_FORMAT).expect("d3d12 backend: failed to build pipelines");
        let mut resources = GpuResources::new(&device);
        resources.flush_construction(&queue);
        let scene_target = SceneTarget::new(&device, size.width.max(1), size.height.max(1), SCENE_FORMAT);
        let depth_texture = allocate_depth_texture(&device, size.width.max(1), size.height.max(1));
        let dsv_heap = build_dsv_heap(&device, &depth_texture);
        let quad_vertex_buffer = create_upload_buffer(&device, bytemuck::cast_slice(&UNIT_QUAD_CORNERS), "unit_quad_corners");
        let quad_vertex_view = D3D12_VERTEX_BUFFER_VIEW { BufferLocation: unsafe { quad_vertex_buffer.GetGPUVirtualAddress() }, SizeInBytes: (std::mem::size_of_val(&UNIT_QUAD_CORNERS)) as u32, StrideInBytes: 8 };

        let (rtv_heap, rtv_stride, back_buffers) = build_swapchain_views(&device, &swapchain);
        let back_buffer_states = vec![D3D12_RESOURCE_STATE_PRESENT; back_buffers.len()];

        Self {
            device,
            queue,
            swapchain,
            allocator,
            list,
            fence,
            fence_value: 0,
            rtv_heap,
            rtv_stride,
            back_buffers,
            back_buffer_states,
            size,
            dpr,
            pipelines,
            resources,
            scene_target,
            scene_state: [D3D12_RESOURCE_STATE_RENDER_TARGET; 5],
            blur_scratch_state: D3D12_RESOURCE_STATE_RENDER_TARGET,
            depth_texture,
            dsv_heap,
            frame_buffers: FrameBuffers::default(),
            frame_descriptors: FrameDescriptors::default(),
            ui_globals: GrowBuffer::default(),
            quad_vertex_buffer,
            quad_vertex_view,
            status: DeviceStatus::Healthy,
            #[cfg(feature = "backend-testing")]
            forced_loss: false,
            #[cfg(feature = "backend-testing")]
            readback: None,
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn update_globals(&mut self, width: f32, height: f32, time_seconds: f32) {
        let values: [f32; 4] = [width, height, time_seconds, 0.0];
        self.ui_globals.upload(&self.device, bytemuck::cast_slice(&values));
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn validate_known_resources(&self, packet: &RenderPacket) -> Result<(), BackendError> {
        for batch in &packet.batches {
            if let Some(texture) = batch.texture {
                if !self.resources.knows_texture(texture) {
                    return Err(BackendError::UnknownResource(ResourceKind::Texture));
                }
            }
        }
        for pass in &packet.surface_passes {
            for draw in pass.draws.iter().chain(pass.translucent_draws.iter()) {
                if !self.resources.knows_mesh(draw.mesh) {
                    return Err(BackendError::UnknownResource(ResourceKind::Mesh));
                }
            }
        }
        Ok(())
    }
}

/// 🏗️ Shared by `new`/`new_headless`: device + direct command queue + the DXGI factory both swapchain
/// constructors need.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn create_device_and_queue() -> Result<(Device, ID3D12CommandQueue, IDXGIFactory2), BackendError> {
    // 🔓️ SAFETY: `CreateDXGIFactory2`/`D3D12CreateDevice` are the documented Win32 entry points for
    // exactly this purpose; `flags`/`padapter`(`None` — default adapter)/`result__` all match the
    // generic pattern this whole crate uses for every `windows`-crate out-param call.
    let factory: IDXGIFactory2 = unsafe { CreateDXGIFactory2(DXGI_CREATE_FACTORY_FLAGS(0)) }.map_err(|_| BackendError::DeviceLost(LossReason::Device))?;
    let mut device: Option<Device> = None;
    unsafe { D3D12CreateDevice(None, D3D_FEATURE_LEVEL_11_0, &mut device) }.map_err(|_| BackendError::DeviceLost(LossReason::Device))?;
    let device = device.ok_or(BackendError::DeviceLost(LossReason::Device))?;
    let queue_desc = D3D12_COMMAND_QUEUE_DESC { Type: D3D12_COMMAND_LIST_TYPE_DIRECT, Priority: 0, Flags: D3D12_COMMAND_QUEUE_FLAG_NONE, NodeMask: 0 };
    let queue: ID3D12CommandQueue = unsafe { device.CreateCommandQueue(&queue_desc) }.map_err(|_| BackendError::DeviceLost(LossReason::Device))?;
    Ok((device, queue, factory))
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn swap_chain_desc(width: u32, height: u32, swap_effect: DXGI_SWAP_EFFECT) -> DXGI_SWAP_CHAIN_DESC1 {
    DXGI_SWAP_CHAIN_DESC1 {
        Width: width,
        Height: height,
        Format: SURFACE_FORMAT,
        Stereo: false.into(),
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: BUFFER_COUNT,
        Scaling: DXGI_SCALING_STRETCH,
        SwapEffect: swap_effect,
        AlphaMode: DXGI_ALPHA_MODE_UNSPECIFIED,
        Flags: 0,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn create_swapchain_for_hwnd(factory: &IDXGIFactory2, queue: &ID3D12CommandQueue, hwnd: HWND, width: u32, height: u32) -> Result<IDXGISwapChain3, BackendError> {
    let desc = swap_chain_desc(width, height, DXGI_SWAP_EFFECT_FLIP_DISCARD);
    // 🔓️ SAFETY: `queue` is the D3D12 command queue this crate holds for its own lifetime — D3D12
    // requires the command queue (not the device) here, a real API constraint confirmed against the
    // vendored source's `pdevice: P0` parameter accepting any `IUnknown`-castable object; `hwnd` comes
    // from a live `raw_window_handle::Win32WindowHandle` whose contract guarantees a valid window for
    // the call's duration; `desc` is a stack value valid for the call.
    let swapchain1 = unsafe { factory.CreateSwapChainForHwnd(queue, hwnd, &desc, None, None) }.map_err(|_| BackendError::SurfaceLost)?;
    swapchain1.cast::<IDXGISwapChain3>().map_err(|_| BackendError::SurfaceLost)
}

/// 🧪️ Composition swapchains only support `DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL`, not `FLIP_DISCARD`
/// (confirmed by the vendored source's own `DXGI_MSG_IDXGIFactory_CreateSwapChainForComposition_
/// OnlyFlipSequentialSupported` diagnostic id — a documented DXGI validation message, not something
/// the binding enforces at compile time, so genuinely worth citing rather than assuming).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn create_swapchain_for_composition(factory: &IDXGIFactory2, queue: &ID3D12CommandQueue, width: u32, height: u32) -> Result<IDXGISwapChain3, BackendError> {
    let desc = swap_chain_desc(width, height, DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL);
    // 🔓️ SAFETY: same reasoning as `create_swapchain_for_hwnd`, minus the `HWND` (this entry point
    // takes none — the whole point of the composition path).
    let swapchain1 = unsafe { factory.CreateSwapChainForComposition(queue, &desc, None) }.map_err(|_| BackendError::SurfaceLost)?;
    swapchain1.cast::<IDXGISwapChain3>().map_err(|_| BackendError::SurfaceLost)
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn build_swapchain_views(device: &Device, swapchain: &IDXGISwapChain3) -> (ID3D12DescriptorHeap, u32, Vec<ID3D12Resource>) {
    let desc = D3D12_DESCRIPTOR_HEAP_DESC { Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV, NumDescriptors: BUFFER_COUNT, Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE, NodeMask: 0 };
    let heap: ID3D12DescriptorHeap = unsafe { device.CreateDescriptorHeap(&desc) }.expect("d3d12 backend: failed to allocate the swapchain RTV heap");
    let stride = unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV) };
    let start = unsafe { heap.GetCPUDescriptorHandleForHeapStart() };
    let mut buffers = Vec::with_capacity(BUFFER_COUNT as usize);
    for index in 0..BUFFER_COUNT {
        let buffer: ID3D12Resource = unsafe { swapchain.GetBuffer(index) }.expect("d3d12 backend: failed to fetch a swapchain back buffer");
        let mut view = D3D12_RENDER_TARGET_VIEW_DESC::default();
        view.Format = SURFACE_RTV_FORMAT;
        view.ViewDimension = D3D12_RTV_DIMENSION_TEXTURE2D;
        view.Anonymous.Texture2D = D3D12_TEX2D_RTV { MipSlice: 0, PlaneSlice: 0 };
        let handle = D3D12_CPU_DESCRIPTOR_HANDLE { ptr: start.ptr + (index as usize) * (stride as usize) };
        // 🔓️ SAFETY: `buffer` is alive for this call; `handle` is within this just-created
        // `NumDescriptors: BUFFER_COUNT` heap's bounds.
        unsafe { device.CreateRenderTargetView(&buffer, Some(&view), handle) };
        buffers.push(buffer);
    }
    (heap, stride, buffers)
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn allocate_depth_texture(device: &Device, width: u32, height: u32) -> ID3D12Resource {
    let clear = D3D12_CLEAR_VALUE { Format: DEPTH_STENCIL_FORMAT, Anonymous: D3D12_CLEAR_VALUE_0 { DepthStencil: D3D12_DEPTH_STENCIL_VALUE { Depth: 1.0, Stencil: 0 } } };
    create_default_texture2d(device, DEPTH_STENCIL_FORMAT, width, height, 1, D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL, D3D12_RESOURCE_STATE_DEPTH_WRITE, Some(clear), "depth_stencil")
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn build_dsv_heap(device: &Device, depth_texture: &ID3D12Resource) -> ID3D12DescriptorHeap {
    let desc = D3D12_DESCRIPTOR_HEAP_DESC { Type: D3D12_DESCRIPTOR_HEAP_TYPE_DSV, NumDescriptors: 1, Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE, NodeMask: 0 };
    let heap: ID3D12DescriptorHeap = unsafe { device.CreateDescriptorHeap(&desc) }.expect("d3d12 backend: failed to allocate the DSV heap");
    let mut view = D3D12_DEPTH_STENCIL_VIEW_DESC::default();
    view.Format = DEPTH_STENCIL_FORMAT;
    view.ViewDimension = D3D12_DSV_DIMENSION_TEXTURE2D;
    view.Flags = D3D12_DSV_FLAG_NONE;
    view.Anonymous.Texture2D = D3D12_TEX2D_DSV { MipSlice: 0 };
    let handle = unsafe { heap.GetCPUDescriptorHandleForHeapStart() };
    // 🔓️ SAFETY: `depth_texture` is alive for this call; `handle` is the only slot in this just-created
    // 1-descriptor heap.
    unsafe { device.CreateDepthStencilView(depth_texture, Some(&view), handle) };
    heap
}

//#endregion Construction

//#region Rendering

impl D3d12Backend {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn dsv_handle(&self) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        unsafe { self.dsv_heap.GetCPUDescriptorHandleForHeapStart() }
    }

    /// 🎬️ Pass 1 — mirrors `render_scene_content`/the Metal backend's `encode_scene_pass`: draws
    /// every backdrop 2D batch, then every `SurfacePass`, then every backdrop-overlay 2D batch, all
    /// into `scene_target` mip 0 + `depth_texture`, cleared once at pass start. Establishes the root
    /// signature, descriptor heaps and IA topology once — they persist for the rest of this frame's
    /// one command list, including `encode_composite_pass`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn encode_scene_pass(&mut self, packet: &RenderPacket, time_seconds: f32) {
        let width = self.size.width as f32;
        let height = self.size.height as f32;
        self.update_globals(width, height, time_seconds);
        self.frame_buffers.quad_instances.upload(&self.device, bytemuck::cast_slice(&packet.quad_instances));
        self.frame_buffers.vector_vertices.upload(&self.device, bytemuck::cast_slice(&packet.vector_vertices));
        crate::world3d::upload_world_passes(&self.device, &mut self.frame_buffers, &packet.surface_passes);

        unsafe {
            self.list.SetGraphicsRootSignature(&self.pipelines.root_signature);
            let heaps = [Some(self.frame_descriptors.heap().expect("begin_frame ran before encode_scene_pass").clone()), Some(self.pipelines.samplers.heap().clone())];
            self.list.SetDescriptorHeaps(&heaps);
            self.list.IASetPrimitiveTopology(windows::Win32::Graphics::Direct3D::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        }
        let ui_globals_address = self.ui_globals.gpu_address().expect("update_globals always uploads a non-empty buffer");
        let default_pair = self.frame_descriptors.allocate_pair(&self.device, self.resources.glyph_atlas_handle(), self.resources.icon_atlas_handle());
        unsafe {
            self.list.SetGraphicsRootConstantBufferView(0, ui_globals_address);
            self.list.SetGraphicsRootDescriptorTable(1, default_pair);
            self.list.SetGraphicsRootDescriptorTable(2, self.pipelines.samplers.ui_table(&self.device));
        }

        self.transition_scene_mip(0, D3D12_RESOURCE_STATE_RENDER_TARGET);
        let scene_rtv = self.scene_target.rtv(0);
        let dsv = self.dsv_handle();
        unsafe {
            self.list.OMSetRenderTargets(1, Some(&scene_rtv), false, Some(&dsv));
            self.list.RSSetViewports(&[D3D12_VIEWPORT { TopLeftX: 0.0, TopLeftY: 0.0, Width: width, Height: height, MinDepth: 0.0, MaxDepth: 1.0 }]);
            self.list.RSSetScissorRects(&[RECT { left: 0, top: 0, right: width as i32, bottom: height as i32 }]);
            self.list.ClearRenderTargetView(scene_rtv, &[0.05, 0.05, 0.06, 1.0], None);
            self.list.ClearDepthStencilView(dsv, D3D12_CLEAR_FLAG_DEPTH | D3D12_CLEAR_FLAG_STENCIL, 1.0, 0, None);
        }

        let quad_address = self.frame_buffers.quad_instances.gpu_address();
        let vector_address = self.frame_buffers.vector_vertices.gpu_address();

        let backdrop_normal = packet.batches.iter().filter(|batch| batch.pipeline != PipelineKind::Glass && batch.layer_state.foreground_of.is_none() && !batch.layer_state.overlay);
        self.encode_2d_batches(quad_address, vector_address, ui_globals_address, backdrop_normal, width, height);

        crate::world3d::encode_passes(&self.list, &self.pipelines, &self.resources, &self.frame_buffers, &packet.surface_passes, width, height);

        let backdrop_overlay = packet.batches.iter().filter(|batch| batch.pipeline != PipelineKind::Glass && batch.layer_state.foreground_of.is_none() && batch.layer_state.overlay);
        self.encode_2d_batches(quad_address, vector_address, ui_globals_address, backdrop_overlay, width, height);
    }

    /// 🎬️ Pass 2 — mirrors `composite_to_swapchain`/the Metal backend's `encode_composite_pass`: blur
    /// the scene's mip chain, blit it to the real swapchain back buffer, composite every glass region
    /// on top, then paint glass-foreground 2D content (normal then overlay) directly onto the back
    /// buffer, reusing `depth_texture` (no clear — "Load") so it still clips against the same stencil
    /// silhouettes the offscreen pass wrote.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn encode_composite_pass(&mut self, back_buffer_index: u32, packet: &RenderPacket) {
        let width = self.size.width as f32;
        let height = self.size.height as f32;

        self.run_blur_chain();
        self.blit_scene_to_back_buffer(back_buffer_index);
        self.composite_glass(back_buffer_index, packet);

        let has_foreground = packet.batches.iter().any(|batch| batch.layer_state.foreground_of.is_some());
        if !has_foreground {
            return;
        }
        let back_buffer_rtv = self.back_buffer_rtv(back_buffer_index);
        let dsv = self.dsv_handle();
        unsafe {
            self.list.OMSetRenderTargets(1, Some(&back_buffer_rtv), false, Some(&dsv));
            self.list.RSSetViewports(&[D3D12_VIEWPORT { TopLeftX: 0.0, TopLeftY: 0.0, Width: width, Height: height, MinDepth: 0.0, MaxDepth: 1.0 }]);
            self.list.RSSetScissorRects(&[RECT { left: 0, top: 0, right: width as i32, bottom: height as i32 }]);
        }
        let ui_globals_address = self.ui_globals.gpu_address().expect("update_globals always uploads a non-empty buffer");
        let quad_address = self.frame_buffers.quad_instances.gpu_address();
        let vector_address = self.frame_buffers.vector_vertices.gpu_address();
        let foreground_normal = packet.batches.iter().filter(|batch| batch.pipeline != PipelineKind::Glass && batch.layer_state.foreground_of.is_some() && !batch.layer_state.overlay);
        self.encode_2d_batches(quad_address, vector_address, ui_globals_address, foreground_normal, width, height);
        let foreground_overlay = packet.batches.iter().filter(|batch| batch.pipeline != PipelineKind::Glass && batch.layer_state.foreground_of.is_some() && batch.layer_state.overlay);
        self.encode_2d_batches(quad_address, vector_address, ui_globals_address, foreground_overlay, width, height);
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn back_buffer_rtv(&self, index: u32) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        let start = unsafe { self.rtv_heap.GetCPUDescriptorHandleForHeapStart() };
        D3D12_CPU_DESCRIPTOR_HANDLE { ptr: start.ptr + (index as usize) * (self.rtv_stride as usize) }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn transition_scene_mip(&mut self, mip: u32, after: D3D12_RESOURCE_STATES) {
        let before = self.scene_state[mip as usize];
        if before.0 != after.0 {
            unsafe { self.list.ResourceBarrier(&[transition_barrier(self.scene_target.texture(), mip, before, after)]) };
            self.scene_state[mip as usize] = after;
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn transition_blur_scratch(&mut self, mip: u32, after: D3D12_RESOURCE_STATES) {
        let before = self.blur_scratch_state;
        if before.0 != after.0 {
            unsafe { self.list.ResourceBarrier(&[transition_barrier(self.scene_target.blur_scratch(), mip, before, after)]) };
            self.blur_scratch_state = after;
        }
    }

    /// 🌫️ Ports `run_blur_chain`/the Metal backend's identical method: for each mip `1..SCENE_MIP_LEVELS`,
    /// copies the previous mip from `scene_target.texture()` into `scene_target.blur_scratch()`
    /// (D3D12, like Metal/wgpu, cannot read and write the same texture within one pass), then renders
    /// a fullscreen 5-tap box downsample from the scratch texture into this mip. After the loop, every
    /// mip of `scene_target.texture()` is transitioned to `PIXEL_SHADER_RESOURCE` in one batched
    /// barrier call — mip 0 was left in `RENDER_TARGET` by `encode_scene_pass`, mips `1..` were each
    /// left in `RENDER_TARGET` by their own downsample draw — so the whole chain is sampleable for
    /// `blit_scene_to_back_buffer`/`composite_glass`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn run_blur_chain(&mut self) {
        let mut blur_globals_bytes = vec![0u8; (SCENE_MIP_LEVELS as usize) * (WORLD_GLOBALS_SLOT_SIZE as usize)];
        for mip in 1..SCENE_MIP_LEVELS {
            let src_mip = mip - 1;
            let value = BlurMipGpu { src_mip: src_mip as f32, _pad: [0.0; 3] };
            let start = (src_mip as usize) * (WORLD_GLOBALS_SLOT_SIZE as usize);
            blur_globals_bytes[start..start + std::mem::size_of::<BlurMipGpu>()].copy_from_slice(bytemuck::bytes_of(&value));
        }
        self.frame_buffers.blur_globals.upload(&self.device, &blur_globals_bytes);
        let Some(blur_globals_address) = self.frame_buffers.blur_globals.gpu_address() else { return };

        for mip in 1..SCENE_MIP_LEVELS {
            let src_mip = mip - 1;
            self.transition_scene_mip(src_mip, D3D12_RESOURCE_STATE_COPY_SOURCE);
            self.transition_blur_scratch(src_mip, D3D12_RESOURCE_STATE_COPY_DEST);
            let src_location = D3D12_TEXTURE_COPY_LOCATION {
                pResource: std::mem::ManuallyDrop::new(Some(unsafe { std::mem::transmute_copy(self.scene_target.texture()) })),
                Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 { SubresourceIndex: src_mip },
            };
            let dst_location = D3D12_TEXTURE_COPY_LOCATION {
                pResource: std::mem::ManuallyDrop::new(Some(unsafe { std::mem::transmute_copy(self.scene_target.blur_scratch()) })),
                Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 { SubresourceIndex: src_mip },
            };
            // 🔓️ SAFETY: same borrowed-pointer-without-`AddRef` technique `crate::types::transition_barrier`
            // documents — both locations are consumed synchronously by `CopyTextureRegion` and then
            // dropped as plain stack values within this loop iteration, never `ManuallyDrop::into_inner`'d.
            unsafe { self.list.CopyTextureRegion(&dst_location, 0, 0, 0, &src_location, None) };
            self.transition_blur_scratch(src_mip, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE);
            self.transition_scene_mip(mip, D3D12_RESOURCE_STATE_RENDER_TARGET);

            let scene_rtv = self.scene_target.rtv(mip);
            let width = (self.scene_target.width() >> mip).max(1) as f32;
            let height = (self.scene_target.height() >> mip).max(1) as f32;
            let pair = self.frame_descriptors.allocate_pair(&self.device, self.scene_target.blur_scratch_srv_handle(&self.device), self.scene_target.blur_scratch_srv_handle(&self.device));
            unsafe {
                self.list.OMSetRenderTargets(1, Some(&scene_rtv), false, None);
                self.list.RSSetViewports(&[D3D12_VIEWPORT { TopLeftX: 0.0, TopLeftY: 0.0, Width: width, Height: height, MinDepth: 0.0, MaxDepth: 1.0 }]);
                self.list.RSSetScissorRects(&[RECT { left: 0, top: 0, right: width as i32, bottom: height as i32 }]);
                self.list.SetPipelineState(&self.pipelines.blur_downsample);
                self.list.SetGraphicsRootConstantBufferView(0, blur_globals_address + (src_mip as u64) * WORLD_GLOBALS_SLOT_SIZE);
                self.list.SetGraphicsRootDescriptorTable(1, pair);
                self.list.SetGraphicsRootDescriptorTable(2, self.pipelines.samplers.scene_table(&self.device));
                self.list.DrawInstanced(6, 1, 0, 0);
            }
        }

        let mut barriers = Vec::new();
        for mip in 0..5u32 {
            if self.scene_state[mip as usize].0 != D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE.0 {
                barriers.push(transition_barrier(self.scene_target.texture(), mip, self.scene_state[mip as usize], D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE));
                self.scene_state[mip as usize] = D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE;
            }
        }
        if !barriers.is_empty() {
            unsafe { self.list.ResourceBarrier(&barriers) };
        }
    }

    /// 🪟️ Ports `blit_scene_to_swapchain`: samples mip 0 of the fully-blurred scene chain into the
    /// back buffer, clearing it first (this is the frame's real "clear colour" as seen by the user —
    /// the offscreen scene clear earlier is invisible, fully overwritten here).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn blit_scene_to_back_buffer(&mut self, back_buffer_index: u32) {
        let width = self.size.width as f32;
        let height = self.size.height as f32;
        let before = self.back_buffer_states[back_buffer_index as usize];
        unsafe { self.list.ResourceBarrier(&[transition_barrier(&self.back_buffers[back_buffer_index as usize], 0, before, D3D12_RESOURCE_STATE_RENDER_TARGET)]) };
        self.back_buffer_states[back_buffer_index as usize] = D3D12_RESOURCE_STATE_RENDER_TARGET;

        let rtv = self.back_buffer_rtv(back_buffer_index);
        let pair = self.frame_descriptors.allocate_pair(&self.device, self.scene_target.texture_srv_handle(&self.device), self.scene_target.texture_srv_handle(&self.device));
        unsafe {
            self.list.OMSetRenderTargets(1, Some(&rtv), false, None);
            self.list.RSSetViewports(&[D3D12_VIEWPORT { TopLeftX: 0.0, TopLeftY: 0.0, Width: width, Height: height, MinDepth: 0.0, MaxDepth: 1.0 }]);
            self.list.RSSetScissorRects(&[RECT { left: 0, top: 0, right: width as i32, bottom: height as i32 }]);
            self.list.ClearRenderTargetView(rtv, &[0.05, 0.05, 0.06, 1.0], None);
            self.list.SetPipelineState(&self.pipelines.scene_blit);
            self.list.SetGraphicsRootDescriptorTable(1, pair);
            self.list.SetGraphicsRootDescriptorTable(2, self.pipelines.samplers.scene_table(&self.device));
            self.list.DrawInstanced(6, 1, 0, 0);
        }
    }

    /// 🥂️ Ports `composite_glass_regions`: one instanced draw over every glass region in the packet —
    /// same "one draw, `instanceCount = glass_instances.len()`" simplification the Metal backend's
    /// `composite_glass` documents (glass has no stencil mask and every instance shares one pipeline/
    /// state, so a single instanced draw is pixel-identical to a per-region loop).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn composite_glass(&mut self, back_buffer_index: u32, packet: &RenderPacket) {
        if packet.glass_instances.is_empty() {
            return;
        }
        self.frame_buffers.glass_instances.upload(&self.device, bytemuck::cast_slice(&packet.glass_instances));
        let Some(glass_address) = self.frame_buffers.glass_instances.gpu_address() else { return };
        let ui_globals_address = self.ui_globals.gpu_address().expect("update_globals always uploads a non-empty buffer");
        let rtv = self.back_buffer_rtv(back_buffer_index);
        let pair = self.frame_descriptors.allocate_pair(&self.device, self.scene_target.texture_srv_handle(&self.device), self.scene_target.texture_srv_handle(&self.device));
        let instance_view =
            D3D12_VERTEX_BUFFER_VIEW { BufferLocation: glass_address, SizeInBytes: (packet.glass_instances.len() * std::mem::size_of::<ui_render::GlassInstance>()) as u32, StrideInBytes: std::mem::size_of::<ui_render::GlassInstance>() as u32 };
        unsafe {
            self.list.OMSetRenderTargets(1, Some(&rtv), false, None);
            self.list.SetPipelineState(&self.pipelines.glass);
            self.list.SetGraphicsRootConstantBufferView(0, ui_globals_address);
            self.list.SetGraphicsRootDescriptorTable(1, pair);
            self.list.SetGraphicsRootDescriptorTable(2, self.pipelines.samplers.scene_table(&self.device));
            self.list.IASetVertexBuffers(0, Some(&[self.quad_vertex_view, instance_view]));
            self.list.DrawInstanced(6, packet.glass_instances.len() as u32, 0, 0);
        }
    }

    /// 🎞️ Replays one filtered group of `DrawBatch`es verbatim — the trait's own invariant. Makes no
    /// ordering/batching/clipping decision of its own.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn encode_2d_batches<'a>(&mut self, quad_address: Option<u64>, vector_address: Option<u64>, ui_globals_address: u64, batches: impl Iterator<Item = &'a DrawBatch>, width: f32, height: f32) {
        let quad_stride = std::mem::size_of::<QuadInstance>() as u64;
        let vector_stride = std::mem::size_of::<VectorVertex>() as u64;
        for batch in batches {
            match batch.mask_range {
                Some((start, count)) => self.draw_silhouette_mask(quad_address, ui_globals_address, start, count, quad_stride, width, height),
                None => unsafe { self.list.OMSetStencilRef(1) },
            }
            match batch.pipeline {
                PipelineKind::UiQuad | PipelineKind::UiRasterTextured => {
                    let Some(quad_address) = quad_address else { continue };
                    let pair = self.bind_ui_textures(batch.texture);
                    let instance_view = D3D12_VERTEX_BUFFER_VIEW { BufferLocation: quad_address + (batch.instance_range.0 as u64) * quad_stride, SizeInBytes: batch.instance_range.1 * (quad_stride as u32), StrideInBytes: quad_stride as u32 };
                    unsafe {
                        self.list.SetPipelineState(&self.pipelines.ui_content);
                        self.list.SetGraphicsRootConstantBufferView(0, ui_globals_address);
                        self.list.SetGraphicsRootDescriptorTable(1, pair);
                        self.list.SetGraphicsRootDescriptorTable(2, self.pipelines.samplers.ui_table(&self.device));
                        self.list.IASetVertexBuffers(0, Some(&[self.quad_vertex_view, instance_view]));
                        self.list.DrawInstanced(6, batch.instance_range.1, 0, 0);
                    }
                }
                PipelineKind::Vector => {
                    let Some(vector_address) = vector_address else { continue };
                    let view = D3D12_VERTEX_BUFFER_VIEW { BufferLocation: vector_address + (batch.instance_range.0 as u64) * vector_stride, SizeInBytes: batch.instance_range.1 * (vector_stride as u32), StrideInBytes: vector_stride as u32 };
                    unsafe {
                        self.list.SetPipelineState(&self.pipelines.vector);
                        self.list.SetGraphicsRootConstantBufferView(0, ui_globals_address);
                        self.list.IASetVertexBuffers(0, Some(&[view]));
                        self.list.DrawInstanced(batch.instance_range.1, 1, 0, 0);
                    }
                }
                PipelineKind::Glass | PipelineKind::BlurMipChain | PipelineKind::SceneBlit | PipelineKind::StencilMask | PipelineKind::World3dMesh | PipelineKind::World3dLines | PipelineKind::World3dTextured => {
                    // 🕳️ Never constructed by `Scene::finish` into `RenderPacket::batches` — see the
                    // Metal backend's identical match arm's doc comment for the full reasoning.
                }
            }
        }
        unsafe { self.list.RSSetScissorRects(&[RECT { left: 0, top: 0, right: width as i32, bottom: height as i32 }]) };
    }

    /// 🩹️ Ports `draw_silhouette_mask`: stamps `reset_bounds` (the first instance in the range) with
    /// stencil ref `0`, then every remaining "piece" instance with ref `1`, leaving ref `1` set for the
    /// content draw that follows.
    #[allow(clippy::too_many_arguments)]
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn draw_silhouette_mask(&mut self, quad_address: Option<u64>, ui_globals_address: u64, start: u32, count: u32, quad_stride: u64, width: f32, height: f32) {
        let Some(quad_address) = quad_address else {
            unsafe { self.list.OMSetStencilRef(1) };
            return;
        };
        unsafe {
            self.list.RSSetScissorRects(&[RECT { left: 0, top: 0, right: width as i32, bottom: height as i32 }]);
            self.list.SetPipelineState(&self.pipelines.ui_mask);
            self.list.SetGraphicsRootConstantBufferView(0, ui_globals_address);
            self.list.OMSetStencilRef(0);
        }
        let first_view = D3D12_VERTEX_BUFFER_VIEW { BufferLocation: quad_address + (start as u64) * quad_stride, SizeInBytes: quad_stride as u32, StrideInBytes: quad_stride as u32 };
        unsafe {
            self.list.IASetVertexBuffers(0, Some(&[self.quad_vertex_view, first_view]));
            self.list.DrawInstanced(6, 1, 0, 0);
        }
        if count > 1 {
            let rest_view = D3D12_VERTEX_BUFFER_VIEW { BufferLocation: quad_address + ((start + 1) as u64) * quad_stride, SizeInBytes: (count - 1) * (quad_stride as u32), StrideInBytes: quad_stride as u32 };
            unsafe {
                self.list.OMSetStencilRef(1);
                self.list.IASetVertexBuffers(0, Some(&[self.quad_vertex_view, rest_view]));
                self.list.DrawInstanced(6, count - 1, 0, 0);
            }
        } else {
            unsafe { self.list.OMSetStencilRef(1) };
        }
    }

    /// 🖼️ `t0`/`s0` are always the shared glyph atlas; `t1`/`s1` are the shared icon atlas for
    /// `UiQuad` batches, or the batch's specific raster texture for `UiRasterTextured` batches
    /// (`batch.texture`) — mirrors the Metal backend's `bind_ui_textures`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn bind_ui_textures(&mut self, texture: Option<TextureId>) -> D3D12_GPU_DESCRIPTOR_HANDLE {
        let icon_or_raster = match texture {
            Some(id) => self.resources.raster_texture_handle(id).unwrap_or_else(|| self.resources.icon_atlas_handle()),
            None => self.resources.icon_atlas_handle(),
        };
        self.frame_descriptors.allocate_pair(&self.device, self.resources.glyph_atlas_handle(), icon_or_raster)
    }
}

//#endregion Rendering

//#region 🔌️GraphicsBackendImpl

impl GraphicsBackend for D3d12Backend {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn name(&self) -> &'static str {
        "d3d12"
    }

    /// ⚠️ `memory_class`/`gpu_tier` are conservative fixed defaults, not queried from the real
    /// adapter — that needs `IDXGIAdapter1::GetDesc1`/`DXGI_QUERY_VIDEO_MEMORY_INFO`, which this crate
    /// does not implement (this backend never keeps an `IDXGIAdapter` reference, having passed `None`
    /// to `D3D12CreateDevice` for the default adapter). Flagged plainly rather than reported as a real
    /// capability query, unlike Metal's `capabilities()` which does query `hasUnifiedMemory()`/
    /// `isLowPower()` for real.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn capabilities(&self) -> DeviceCapabilities {
        DeviceCapabilities {
            max_texture_dimension: 16384,
            max_bind_groups: 4,
            supports_msaa: true,
            supports_timestamp_queries: false,
            supports_storage_buffers: true,
            preferred_surface_format: SurfaceFormat::Bgra8UnormSrgb,
            memory_class: MemoryClass::Standard,
            gpu_tier: GpuTier::Discrete,
        }
    }

    /// 🕳️ A zero-size request parks: `self.size` is still recorded, but the swapchain/scene target/
    /// depth texture are left untouched (never resized to zero — `IDXGISwapChain::ResizeBuffers`
    /// documents `0` width/height as "keep the current value", not "resize to zero", so this crate
    /// must skip the call entirely rather than pass zeros through). `render` itself refuses to draw
    /// while parked, so the staleness is never observed.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn resize(&mut self, size: PhysicalSize, dpr: f32) -> Result<(), BackendError> {
        self.size = size;
        self.dpr = dpr;
        if size.is_zero() {
            return Ok(());
        }
        // 🔓️ SAFETY / correctness note: `ResizeBuffers` requires every outstanding reference to a
        // swapchain buffer to be released first — waiting for the fence then clearing `back_buffers`
        // (dropping every `ID3D12Resource`, which `Release`s the underlying COM object) satisfies that
        // before the call below.
        wait_for_fence_value(&self.fence, self.fence_value);
        self.back_buffers.clear();
        unsafe { self.swapchain.ResizeBuffers(BUFFER_COUNT, size.width, size.height, SURFACE_FORMAT, DXGI_SWAP_CHAIN_FLAG(0)) }.map_err(|_| BackendError::SurfaceLost)?;
        let (rtv_heap, rtv_stride, back_buffers) = build_swapchain_views(&self.device, &self.swapchain);
        self.rtv_heap = rtv_heap;
        self.rtv_stride = rtv_stride;
        self.back_buffers = back_buffers;
        self.back_buffer_states = vec![D3D12_RESOURCE_STATE_PRESENT; self.back_buffers.len()];

        if self.scene_target.ensure(&self.device, size.width, size.height) {
            self.scene_state = [D3D12_RESOURCE_STATE_RENDER_TARGET; 5];
            self.blur_scratch_state = D3D12_RESOURCE_STATE_RENDER_TARGET;
        }
        self.depth_texture = allocate_depth_texture(&self.device, size.width, size.height);
        self.dsv_heap = build_dsv_heap(&self.device, &self.depth_texture);
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn apply_resources(&mut self, ops: &[ResourceOp]) -> Result<(), BackendError> {
        self.resources.apply(&self.device, &self.queue, ops).map_err(Into::into)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn render(&mut self, packet: &RenderPacket, time_seconds: f32) -> Result<RenderReport, BackendError> {
        if let DeviceStatus::Lost(reason) = self.device_status() {
            return Err(BackendError::DeviceLost(reason));
        }
        if self.size.is_zero() {
            return Ok(RenderReport::SkippedZeroSize);
        }
        self.validate_known_resources(packet)?;

        wait_for_fence_value(&self.fence, self.fence_value);
        unsafe { self.allocator.Reset() }.map_err(|_| BackendError::DeviceLost(LossReason::Device))?;
        unsafe { self.list.Reset(&self.allocator, None) }.map_err(|_| BackendError::DeviceLost(LossReason::Device))?;
        // 🧮️ Beyond one `allocate_pair` per batch (the per-batch upper bound `encode_2d_batches`/
        // `bind_ui_textures` actually draws from), this frame also always spends: 1 default pair
        // (`encode_scene_pass`'s baseline root-table bind), `SCENE_MIP_LEVELS - 1` blur pairs
        // (`run_blur_chain`'s per-mip downsample, unconditional every frame), 1 blit pair
        // (`blit_scene_to_back_buffer`, unconditional), and 1 glass pair (`composite_glass`, only when
        // `glass_instances` is non-empty) — `+ 8` covers all of that with headroom.
        self.frame_descriptors.begin_frame(&self.device, (packet.batches.len() as u32) + 8);

        let back_buffer_index = unsafe { self.swapchain.GetCurrentBackBufferIndex() };
        self.encode_scene_pass(packet, time_seconds);
        self.encode_composite_pass(back_buffer_index, packet);

        #[cfg(feature = "backend-testing")]
        self.capture_readback(back_buffer_index);

        let before = self.back_buffer_states[back_buffer_index as usize];
        unsafe { self.list.ResourceBarrier(&[transition_barrier(&self.back_buffers[back_buffer_index as usize], 0, before, D3D12_RESOURCE_STATE_PRESENT)]) };
        self.back_buffer_states[back_buffer_index as usize] = D3D12_RESOURCE_STATE_PRESENT;

        unsafe { self.list.Close() }.map_err(|_| BackendError::DeviceLost(LossReason::Device))?;
        let command_list: ID3D12CommandList = self.list.cast().expect("ID3D12GraphicsCommandList always casts to its ID3D12CommandList base");
        let lists = [Some(command_list)];
        unsafe { self.queue.ExecuteCommandLists(&lists) };

        let present_result = unsafe { self.swapchain.Present(1, DXGI_PRESENT(0)) };
        if present_result.is_err() {
            return Ok(RenderReport::SkippedOutOfDate);
        }

        self.fence_value += 1;
        unsafe { self.queue.Signal(&self.fence, self.fence_value) }.map_err(|_| BackendError::DeviceLost(LossReason::Device))?;

        let stats =
            FrameStats { encode_duration_seconds: 0.0, submit_duration_seconds: 0.0, present_duration_seconds: 0.0, draw_call_count: packet.batches.len() as u32, instance_count: (packet.quad_instances.len() + packet.vector_vertices.len()) as u32 };
        Ok(RenderReport::Presented { stats })
    }

    /// 🚦️ Unlike Metal (which has no programmatic device-loss detection and simulates it entirely),
    /// D3D12 exposes a *real* check — `ID3D12Device::GetDeviceRemovedReason`. `forced_loss` (test-only)
    /// layers the same simulated-loss surface `backend-testing` needs on top.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn device_status(&self) -> DeviceStatus {
        #[cfg(feature = "backend-testing")]
        if self.forced_loss {
            return DeviceStatus::Lost(LossReason::Device);
        }
        if self.status == DeviceStatus::Healthy {
            // 🔓️ SAFETY: `GetDeviceRemovedReason` is a plain, side-effect-free accessor on a live
            // device this call borrows.
            if unsafe { self.device.GetDeviceRemovedReason() }.is_err() {
                return DeviceStatus::Lost(LossReason::Device);
            }
        }
        self.status
    }

    /// ♻️ A **real** device-removed event cannot actually be healed by this method — the device,
    /// queue and swapchain themselves are dead too, and `GraphicsBackend::recover`'s signature
    /// (`&mut self` returning `RecoveredResources`, not a fresh backend instance) has no way to
    /// express "reconstruct everything from scratch." This method is fully effective only for the
    /// `backend-testing` simulated-loss path (`forced_loss`); against a genuine `DEVICE_REMOVED`, it
    /// clears the resource tables and resets `status`, but the underlying device object is still
    /// unusable and every subsequent D3D12 call will keep failing until the whole backend is dropped
    /// and reconstructed by the caller. Flagged plainly — this is a real gap, not silently assumed
    /// away, and the honest reason `GraphicsBackend`'s current `recover()` contract cannot fully cover
    /// a real device loss on any backend, D3D12 included.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn recover(&mut self) -> Result<RecoveredResources, BackendError> {
        let (lost_textures, lost_meshes, lost_atlases) = self.resources.drain_known();
        self.status = DeviceStatus::Healthy;
        #[cfg(feature = "backend-testing")]
        {
            self.forced_loss = false;
        }
        Ok(RecoveredResources { lost_textures, lost_meshes, lost_atlases })
    }

    #[cfg(feature = "backend-testing")]
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn debug_force_device_loss(&mut self) {
        self.forced_loss = true;
    }

    /// 🧪️ Reads back the readback buffer `render` populated via a same-frame `CopyTextureRegion` from
    /// the just-presented back buffer (see `capture_readback`) — row-pitch-unpadded, BGRA-swizzled to
    /// the tightly packed RGBA8 `ReadbackImage` the conformance suite expects, same sRGB-vs-linear
    /// byte-reinterpretation acceptable-for-tolerance caveat the Metal backend's `read_back` documents.
    #[cfg(feature = "backend-testing")]
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn read_back(&mut self) -> Result<ReadbackImage, BackendError> {
        if self.size.is_zero() {
            return Err(BackendError::ZeroSizeSurface);
        }
        let Some((buffer, row_pitch)) = self.readback.as_ref() else {
            return Err(BackendError::Timeout);
        };
        let width = self.size.width;
        let height = self.size.height;
        let mut mapped: *mut core::ffi::c_void = std::ptr::null_mut();
        unsafe { buffer.Map(0, None, Some(&mut mapped)) }.map_err(|_| BackendError::OutOfMemory)?;
        let mut pixels = vec![0u8; (width as usize) * (height as usize) * 4];
        // 🔓️ SAFETY: `mapped` is valid for `row_pitch * height` bytes (the buffer `capture_readback`
        // sized it to); `pixels` is exactly `width * height * 4` bytes, and every row copy stays within
        // both bounds (`row_pitch >= width * 4` by construction).
        unsafe {
            let source = mapped.cast::<u8>();
            for row in 0..height as usize {
                let row_bytes = std::slice::from_raw_parts(source.add(row * (*row_pitch as usize)), (width as usize) * 4);
                let dest_start = row * (width as usize) * 4;
                for pixel in 0..width as usize {
                    let source_pixel = &row_bytes[pixel * 4..pixel * 4 + 4];
                    let dest_pixel = &mut pixels[dest_start + pixel * 4..dest_start + pixel * 4 + 4];
                    dest_pixel[0] = source_pixel[2];
                    dest_pixel[1] = source_pixel[1];
                    dest_pixel[2] = source_pixel[0];
                    dest_pixel[3] = source_pixel[3];
                }
            }
        }
        unsafe { buffer.Unmap(0, None) };
        Ok(ReadbackImage { width, height, pixels })
    }
}

#[cfg(feature = "backend-testing")]
impl D3d12Backend {
    /// 🧪️ Copies the just-composited back buffer into a fresh `READBACK`-heap buffer — always
    /// reallocated at the current size rather than cached, since this path only ever runs inside the
    /// conformance harness (never a hot path worth optimizing) — mirrors the Metal backend's
    /// `capture_readback`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn capture_readback(&mut self, back_buffer_index: u32) {
        let width = self.size.width;
        let height = self.size.height;
        let row_pitch = align_up(width * 4, D3D12_TEXTURE_DATA_PITCH_ALIGNMENT);
        let buffer_size = (row_pitch as u64) * (height as u64);
        let desc = crate::types::buffer_desc(buffer_size);
        let mut readback_resource: Option<ID3D12Resource> = None;
        // 🔓️ SAFETY: `desc` is a stack value valid for the call.
        unsafe { self.device.CreateCommittedResource(&crate::types::READBACK_HEAP, D3D12_HEAP_FLAG_NONE, &desc, D3D12_RESOURCE_STATE_COPY_DEST, None, &mut readback_resource) }.expect("d3d12 backend: failed to allocate the readback buffer");
        let readback_resource = readback_resource.expect("CreateCommittedResource succeeded but returned no resource");

        let before = self.back_buffer_states[back_buffer_index as usize];
        unsafe { self.list.ResourceBarrier(&[transition_barrier(&self.back_buffers[back_buffer_index as usize], 0, before, D3D12_RESOURCE_STATE_COPY_SOURCE)]) };
        let src_location = D3D12_TEXTURE_COPY_LOCATION {
            pResource: std::mem::ManuallyDrop::new(Some(unsafe { std::mem::transmute_copy(&self.back_buffers[back_buffer_index as usize]) })),
            Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 { SubresourceIndex: 0 },
        };
        let dst_location = D3D12_TEXTURE_COPY_LOCATION {
            pResource: std::mem::ManuallyDrop::new(Some(unsafe { std::mem::transmute_copy(&readback_resource) })),
            Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                PlacedFootprint: D3D12_PLACED_SUBRESOURCE_FOOTPRINT { Offset: 0, Footprint: D3D12_SUBRESOURCE_FOOTPRINT { Format: SURFACE_FORMAT, Width: width, Height: height, Depth: 1, RowPitch: row_pitch } },
            },
        };
        // 🔓️ SAFETY: same borrowed-pointer-without-`AddRef` technique used throughout this crate for
        // `D3D12_TEXTURE_COPY_LOCATION`; both locations are consumed synchronously here.
        unsafe { self.list.CopyTextureRegion(&dst_location, 0, 0, 0, &src_location, None) };
        unsafe { self.list.ResourceBarrier(&[transition_barrier(&self.back_buffers[back_buffer_index as usize], 0, D3D12_RESOURCE_STATE_COPY_SOURCE, before)]) };
        self.back_buffer_states[back_buffer_index as usize] = before;

        self.readback = Some((readback_resource, row_pitch));
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn align_up(value: u32, alignment: u32) -> u32 {
    (value + alignment - 1) / alignment * alignment
}

//#endregion 🔌️GraphicsBackendImpl

//#endregion 🪟️D3d12Backend

//#endregion 🔖️Backend

//#region Tests

/// 🧪️ Every test here needs a live D3D12 device on Windows — `D3d12Backend::new_headless` reports
/// `BackendError::DeviceLost` when `D3D12CreateDevice` finds none (a genuinely possible state on a
/// runner with no GPU/software rasterizer registered), so failing construction *is* this module's
/// "skip cleanly" signal — mirrors the Metal backend's identical test-module shape. `UNRUN` per U4 on
/// this machine (macOS); see `📓️terra-backend-d3d12-report.md` for the exact commands `sol` runs.
#[cfg(all(test, feature = "backend-testing"))]
mod tests {
    use super::*;
    use ui_render::{FinishParams, ResourceRegistry, Scene, SceneBuilder};

    /// 🧵️ Drives an `async fn` that structurally never suspends (D3D12/DXGI device/swapchain creation
    /// is synchronous — see `D3d12Backend::new`'s docstring) to completion without pulling in an
    /// executor crate — identical helper to the Metal backend's `block_on`.
    fn block_on<F: std::future::Future>(future: F) -> F::Output {
        use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
        const VTABLE: RawWakerVTable = RawWakerVTable::new(|_| RAW_WAKER, |_| {}, |_| {}, |_| {});
        const RAW_WAKER: RawWaker = RawWaker::new(std::ptr::null(), &VTABLE);
        let waker = unsafe { Waker::from_raw(RAW_WAKER) };
        let mut context = Context::from_waker(&waker);
        let mut future = Box::pin(future);
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("d3d12 backend: a construction future that should never suspend returned Pending"),
        }
    }

    fn finish_params(viewport: [f32; 2]) -> FinishParams {
        FinishParams { viewport, dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }
    }

    #[test]
    fn constructing_a_headless_backend_succeeds_or_skips_cleanly() {
        let Ok(mut backend) = block_on(D3d12Backend::new_headless(PhysicalSize::new(64, 64), 1.0)) else {
            eprintln!("skipping: no D3D12 device available on this machine");
            return;
        };
        assert_eq!(GraphicsBackend::name(&backend), "d3d12");
        assert!(GraphicsBackend::capabilities(&backend).max_texture_dimension > 0);
        assert_eq!(GraphicsBackend::device_status(&backend), DeviceStatus::Healthy);
        let _ = GraphicsBackend::resize(&mut backend, PhysicalSize::new(64, 64), 1.0);
    }

    #[test]
    fn zero_size_resize_parks_and_restores() {
        let Ok(mut backend) = block_on(D3d12Backend::new_headless(PhysicalSize::new(64, 64), 1.0)) else {
            eprintln!("skipping: no D3D12 device available on this machine");
            return;
        };
        GraphicsBackend::resize(&mut backend, PhysicalSize::ZERO, 1.0).expect("resize to zero");
        let packet = Scene::finish(SceneBuilder::default(), finish_params([0.0, 0.0])).expect("finish");
        assert!(matches!(GraphicsBackend::render(&mut backend, &packet, 0.0), Ok(RenderReport::SkippedZeroSize)));

        GraphicsBackend::resize(&mut backend, PhysicalSize::new(64, 64), 1.0).expect("resize back");
        let mut builder = SceneBuilder::default();
        builder.push_solid([0.0, 0.0, 10.0, 10.0], [1.0, 0.0, 0.0, 1.0]);
        let packet = Scene::finish(builder, finish_params([64.0, 64.0])).expect("finish");
        let report = GraphicsBackend::render(&mut backend, &packet, 0.0).expect("render after restoring a nonzero size");
        assert!(matches!(report, RenderReport::Presented { .. } | RenderReport::SkippedOutOfDate));
    }

    #[test]
    fn apply_resources_before_render_succeeds_and_an_unapplied_id_errors_cleanly() {
        let Ok(mut backend) = block_on(D3d12Backend::new_headless(PhysicalSize::new(64, 64), 1.0)) else {
            eprintln!("skipping: no D3D12 device available on this machine");
            return;
        };
        let mut registry = ResourceRegistry::default();
        registry.request_texture_upload("known", 4, 4, vec![0; 64]);
        let mut applied_builder = SceneBuilder::default();
        applied_builder.push_raster_quad(&mut registry, "known", [0.0, 0.0, 10.0, 10.0], [0.0, 0.0, 1.0, 1.0], 1.0);
        GraphicsBackend::apply_resources(&mut backend, &registry.drain_ops()).expect("apply_resources");
        let applied_packet = Scene::finish(applied_builder, finish_params([64.0, 64.0])).expect("finish");
        let report = GraphicsBackend::render(&mut backend, &applied_packet, 0.0).expect("render with an applied texture");
        assert!(matches!(report, RenderReport::Presented { .. } | RenderReport::SkippedOutOfDate));

        let mut unapplied_builder = SceneBuilder::default();
        unapplied_builder.push_raster_quad(&mut registry, "unknown", [0.0, 0.0, 10.0, 10.0], [0.0, 0.0, 1.0, 1.0], 1.0);
        let unapplied_packet = Scene::finish(unapplied_builder, finish_params([64.0, 64.0])).expect("finish");
        let result = GraphicsBackend::render(&mut backend, &unapplied_packet, 0.0);
        assert!(matches!(result, Err(BackendError::UnknownResource(ResourceKind::Texture))));
    }

    #[test]
    fn forced_device_loss_reports_lost_and_recover_names_the_dead_generation() {
        let Ok(mut backend) = block_on(D3d12Backend::new_headless(PhysicalSize::new(64, 64), 1.0)) else {
            eprintln!("skipping: no D3D12 device available on this machine");
            return;
        };
        let mut registry = ResourceRegistry::default();
        let texture = registry.request_texture_upload("icon", 4, 4, vec![0; 64]);
        GraphicsBackend::apply_resources(&mut backend, &registry.drain_ops()).expect("apply_resources");

        GraphicsBackend::debug_force_device_loss(&mut backend);
        assert!(matches!(GraphicsBackend::device_status(&backend), DeviceStatus::Lost(_)));
        let packet = Scene::finish(SceneBuilder::default(), finish_params([64.0, 64.0])).expect("finish");
        assert!(matches!(GraphicsBackend::render(&mut backend, &packet, 0.0), Err(BackendError::DeviceLost(_))));

        let recovered = GraphicsBackend::recover(&mut backend).expect("recover");
        assert_eq!(recovered.lost_textures, vec![texture]);
        assert_eq!(GraphicsBackend::device_status(&backend), DeviceStatus::Healthy);
    }

    #[test]
    fn read_back_reports_zero_size_cleanly_before_any_frame_is_presented() {
        let Ok(mut backend) = block_on(D3d12Backend::new_headless(PhysicalSize::ZERO, 1.0)) else {
            eprintln!("skipping: no D3D12 device available on this machine");
            return;
        };
        assert!(matches!(GraphicsBackend::read_back(&mut backend), Err(BackendError::ZeroSizeSurface)));
    }
}

//#endregion Tests
