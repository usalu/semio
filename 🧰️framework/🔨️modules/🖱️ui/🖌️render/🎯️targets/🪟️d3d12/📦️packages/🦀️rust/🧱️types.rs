//! @emoji 🧱️ GPU-layout mirrors this crate needs that `ui_render::scene` does not already export as
//! `#[repr(C)]`/`Pod` — `ui_render::{QuadInstance, VectorVertex, GlassInstance}` are byte-identical to
//! their D3D12 buffer layout already and are used directly via `bytemuck::cast_slice`; only the
//! world3d instance/vertex/uniform shapes and the blur mip scalar need a D3D12-side GPU form, built
//! from `ui_render::scene::{MeshInstance, LineVertex3}` (which are not `Pod` — `MeshInstance` carries
//! `bool` fields no GPU buffer can hold directly). Direct structural port of the Metal backend's
//! `🧱️types.rs` (packet `backend-metal`) — same fields, same byte layout, only the doc comments
//! differ where the D3D12 rationale differs from Metal's.

use bytemuck::{Pod, Zeroable};
use ui_render::{LineVertex3, MeshInstance};
use windows::Win32::Graphics::Direct3D12::{
    ID3D12Device, ID3D12Fence, ID3D12Resource, D3D12_CLEAR_VALUE, D3D12_CPU_PAGE_PROPERTY_UNKNOWN, D3D12_HEAP_FLAG_NONE, D3D12_HEAP_PROPERTIES, D3D12_HEAP_TYPE_DEFAULT, D3D12_HEAP_TYPE_READBACK, D3D12_HEAP_TYPE_UPLOAD, D3D12_MEMORY_POOL_UNKNOWN,
    D3D12_RESOURCE_BARRIER, D3D12_RESOURCE_BARRIER_0, D3D12_RESOURCE_BARRIER_FLAG_NONE, D3D12_RESOURCE_BARRIER_TYPE_TRANSITION, D3D12_RESOURCE_DESC, D3D12_RESOURCE_DIMENSION_BUFFER, D3D12_RESOURCE_DIMENSION_TEXTURE2D, D3D12_RESOURCE_FLAGS,
    D3D12_RESOURCE_STATES, D3D12_RESOURCE_TRANSITION_BARRIER, D3D12_TEXTURE_LAYOUT_ROW_MAJOR, D3D12_TEXTURE_LAYOUT_UNKNOWN,
};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT, DXGI_SAMPLE_DESC};

//#region 🔖️Types

//#region 🌐️World3d

/// 🌐️ Byte-identical to `WorldGlobals` in `✨️hlsl.rs`'s world3d HLSL: a 4x4 row-major `view_proj`
/// plus `light_dir` padded to a `float4`. 80 bytes; `WORLD_GLOBALS_SLOT_SIZE` below is the
/// GPU-virtual-address stride a caller must round up to when packing several of these into one ring
/// buffer bound through a root CBV (`ID3D12GraphicsCommandList::SetGraphicsRootConstantBufferView`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct WorldGlobalsGpu {
    pub view_proj: [f32; 16],
    pub light_dir: [f32; 4],
}

/// 📏️ Byte stride between consecutive `WorldGlobalsGpu` (and `BlurMipGpu`) slots inside their ring
/// buffers — exactly `D3D12_CONSTANT_BUFFER_DATA_PLACEMENT_ALIGNMENT` (confirmed against the vendored
/// `windows-0.62.2` source, `Direct3D12/mod.rs:1016`: `pub const
/// D3D12_CONSTANT_BUFFER_DATA_PLACEMENT_ALIGNMENT: u32 = 256`), unlike Metal's identically-valued 256
/// which was only a conservative universal bound — here it is the hardware's own mandatory alignment
/// for any GPU virtual address bound through a root CBV.
pub const WORLD_GLOBALS_SLOT_SIZE: u64 = 256;

/// 🧊️ Byte-identical to `WorldMeshVertexIn`'s per-instance attributes (input slot 1, locations
/// 3..8) in `✨️hlsl.rs`'s world3d mesh HLSL: four `float4` model-matrix rows, `color`, `flags`.
/// Mirrors the wgpu target's `World3dGpuInstance` and the Metal backend's identical struct.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct World3dGpuInstance {
    pub model0: [f32; 4],
    pub model1: [f32; 4],
    pub model2: [f32; 4],
    pub model3: [f32; 4],
    pub color: [f32; 4],
    pub flags: [f32; 4],
}

impl From<&MeshInstance> for World3dGpuInstance {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(instance: &MeshInstance) -> Self {
        let m = instance.model;
        Self {
            model0: [m[0], m[1], m[2], m[3]],
            model1: [m[4], m[5], m[6], m[7]],
            model2: [m[8], m[9], m[10], m[11]],
            model3: [m[12], m[13], m[14], m[15]],
            color: instance.color,
            flags: [if instance.selected { 1.0 } else { 0.0 }, if instance.hovered { 1.0 } else { 0.0 }, 0.0, 0.0],
        }
    }
}

/// ➖️ Byte-identical to `WorldLineVertexIn` in `✨️hlsl.rs`'s world3d lines HLSL.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct WorldLineGpuVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl From<&LineVertex3> for WorldLineGpuVertex {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(vertex: &LineVertex3) -> Self {
        Self { position: vertex.position, color: vertex.color }
    }
}

/// 🧊️ Byte-identical to `WorldMeshVertexIn`'s per-vertex attributes (input slot 0, locations 0..1).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct World3dGpuVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

//#endregion 🌐️World3d

//#region 📐️QuadVertex

/// 📐️ The shared unit-quad corner buffer (two CCW triangles) every 2D pipeline's input slot 0 reads —
/// byte-identical to the wgpu target's `quad_vertex_buffer` contents and the Metal backend's
/// `UNIT_QUAD_CORNERS`.
pub const UNIT_QUAD_CORNERS: [[f32; 2]; 6] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

//#endregion 📐️QuadVertex

//#region 🌫️BlurGlobals

/// 🌫️ The tiny per-draw uniform `✨️hlsl.rs`'s blur downsample pixel shader reads its source mip from
/// — bound through the same root CBV (`b0`) every other pipeline's globals use, at a
/// `WORLD_GLOBALS_SLOT_SIZE`-strided offset into a small per-frame ring (`🪟️backend.rs::run_blur_chain`),
/// so no extra root parameter is needed for it. Padded to 16 bytes to match the HLSL `cbuffer`'s
/// `float4`-aligned layout (mirrors `BlurGlobals` in `ui_render::shader_contract::BLUR_DOWNSAMPLE_SHADER`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct BlurMipGpu {
    pub src_mip: f32,
    pub _pad: [f32; 3],
}

//#endregion 🌫️BlurGlobals

//#region 🚧️Barriers

/// 🚧️ Builds one `D3D12_RESOURCE_BARRIER_TYPE_TRANSITION` barrier for `resource`'s `subresource`.
/// Shared by every file that touches resource state (`🗃️resources.rs`'s texture upload,
/// `🌫️scene_target.rs`'s per-mip blur transitions, `🪟️backend.rs`'s swapchain-buffer transitions) so
/// the one non-obvious `unsafe` in the whole crate's barrier plumbing is written, and its soundness
/// argued, exactly once.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn transition_barrier(resource: &ID3D12Resource, subresource: u32, before: D3D12_RESOURCE_STATES, after: D3D12_RESOURCE_STATES) -> D3D12_RESOURCE_BARRIER {
    // 🔓️ SAFETY: `D3D12_RESOURCE_TRANSITION_BARRIER::pResource` is a `ManuallyDrop<Option<ID3D12Resource>>`
    // — this crate never lets its `Drop` run (nothing here ever calls `ManuallyDrop::into_inner` on a
    // barrier's `pResource` and drops the result), so `transmute_copy` here creates a bitwise-copied
    // "borrowed" COM pointer with no matching `AddRef`, which is sound exactly because it is never
    // released either: the barrier is consumed synchronously by `ID3D12GraphicsCommandList::ResourceBarrier`
    // within the same function that built it and then dropped as a plain stack value (running
    // `ManuallyDrop`'s no-op destructor, never `Option<ID3D12Resource>`'s real one) — `resource` itself,
    // the actual owner, outlives that call in every call site in this crate.
    let transition = D3D12_RESOURCE_TRANSITION_BARRIER { pResource: std::mem::ManuallyDrop::new(Some(unsafe { std::mem::transmute_copy(resource) })), Subresource: subresource, StateBefore: before, StateAfter: after };
    D3D12_RESOURCE_BARRIER { Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION, Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE, Anonymous: D3D12_RESOURCE_BARRIER_0 { Transition: std::mem::ManuallyDrop::new(transition) } }
}

//#endregion 🚧️Barriers

//#region ⏱️Fence

/// ⏱️ Blocks the calling thread until `fence`'s completed value reaches `target`, by polling
/// `ID3D12Fence::GetCompletedValue` rather than an OS wait event. `CreateEventW`/`WaitForSingleObject`
/// would be the idiomatic D3D12 approach, but `CreateEventW` is `#[cfg(feature = "Win32_Security")]`
/// in the vendored `windows-0.62.2` source (confirmed: `Win32/System/Threading/mod.rs`) — a feature
/// this crate's `Cargo.toml` does not declare and is not permitted to add (U7; see
/// `registrar-requests` in `📓️terra-backend-d3d12-report.md`). Polling is a correct, if less
/// efficient, substitute: it busy-waits the CPU during a GPU wait instead of blocking on a kernel
/// object, acceptable for this backend's synchronous per-frame/per-upload wait points.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn wait_for_fence_value(fence: &ID3D12Fence, target: u64) {
    // 🔓️ SAFETY: `GetCompletedValue` is a plain, side-effect-free accessor on a live fence this
    // function borrows for its whole body.
    while unsafe { fence.GetCompletedValue() } < target {
        std::thread::sleep(std::time::Duration::from_micros(200));
    }
}

//#endregion ⏱️Fence

//#region 🏗️ResourceCreation

/// 🏗️ A `D3D12_HEAP_TYPE_DEFAULT` heap-properties value — GPU-local memory, no CPU access, used for
/// every texture this crate creates (glyph/icon/raster atlases, the offscreen scene target and its
/// blur scratch texture, the depth/stencil buffer).
pub const DEFAULT_HEAP: D3D12_HEAP_PROPERTIES = D3D12_HEAP_PROPERTIES { Type: D3D12_HEAP_TYPE_DEFAULT, CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN, MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN, CreationNodeMask: 1, VisibleNodeMask: 1 };

/// 🏗️ A `D3D12_HEAP_TYPE_UPLOAD` heap-properties value — CPU-writable (`Map`), GPU-readable directly,
/// used for every buffer this crate creates (vertex/index/constant data) and for texture-upload
/// staging buffers. Mirrors Metal's `StorageModeShared` buffers — no explicit copy/barrier step is
/// needed for buffers the GPU only ever reads, unlike textures (see `🗃️resources.rs::upload_texture`).
pub const UPLOAD_HEAP: D3D12_HEAP_PROPERTIES = D3D12_HEAP_PROPERTIES { Type: D3D12_HEAP_TYPE_UPLOAD, CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN, MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN, CreationNodeMask: 1, VisibleNodeMask: 1 };

/// 🏗️ A `D3D12_HEAP_TYPE_READBACK` heap-properties value — GPU-writes-then-CPU-reads memory, used only
/// by `🪟️backend.rs`'s `backend-testing` `capture_readback`/`read_back` path.
#[cfg(feature = "backend-testing")]
pub const READBACK_HEAP: D3D12_HEAP_PROPERTIES = D3D12_HEAP_PROPERTIES { Type: D3D12_HEAP_TYPE_READBACK, CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN, MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN, CreationNodeMask: 1, VisibleNodeMask: 1 };

/// 📐️ A `D3D12_RESOURCE_DESC` for a linear (row-major) buffer of `size` bytes — vertex/index/constant
/// buffers and texture-upload staging buffers all share this shape.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn buffer_desc(size: u64) -> D3D12_RESOURCE_DESC {
    D3D12_RESOURCE_DESC {
        Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
        Alignment: 0,
        Width: size.max(1),
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: DXGI_FORMAT(0),
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
        Flags: D3D12_RESOURCE_FLAGS(0),
    }
}

/// 📐️ A `D3D12_RESOURCE_DESC` for a `width`x`height` 2D texture with `mip_levels` levels and `flags`
/// (render-target/depth-stencil/shader-read combinations) — every texture in this crate (atlases,
/// raster images, the scene target + blur scratch, the depth/stencil buffer) shares this shape.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn texture2d_desc(format: DXGI_FORMAT, width: u32, height: u32, mip_levels: u16, flags: D3D12_RESOURCE_FLAGS) -> D3D12_RESOURCE_DESC {
    D3D12_RESOURCE_DESC {
        Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
        Alignment: 0,
        Width: width.max(1) as u64,
        Height: height.max(1),
        DepthOrArraySize: 1,
        MipLevels: mip_levels.max(1),
        Format: format,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,
        Flags: flags,
    }
}

/// 🏗️ Creates a `D3D12_HEAP_TYPE_UPLOAD` buffer of `size` bytes in the `GENERIC_READ` state (the only
/// legal initial/steady state for an upload-heap resource) and copies `bytes` into it via `Map`. A
/// zero-length `bytes` still allocates (minimum 1 byte) so callers always get a valid resource back.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn create_upload_buffer(device: &ID3D12Device, bytes: &[u8], label: &str) -> ID3D12Resource {
    use windows::Win32::Graphics::Direct3D12::D3D12_RESOURCE_STATE_GENERIC_READ;
    let desc = buffer_desc(bytes.len() as u64);
    let mut resource: Option<ID3D12Resource> = None;
    // 🔓️ SAFETY: `desc` is a stack value valid for the call; `poptimizedclearvalue` is `None` (never
    // meaningful for a buffer); `result__` is a valid `*mut Option<ID3D12Resource>` out-param.
    unsafe { device.CreateCommittedResource(&UPLOAD_HEAP, D3D12_HEAP_FLAG_NONE, &desc, D3D12_RESOURCE_STATE_GENERIC_READ, None, &mut resource) }.unwrap_or_else(|error| panic!("d3d12 backend: failed to allocate upload buffer {label}: {error:?}"));
    let resource = resource.expect("CreateCommittedResource succeeded but returned no resource");
    if !bytes.is_empty() {
        let mut mapped: *mut core::ffi::c_void = std::ptr::null_mut();
        // 🔓️ SAFETY: `Map(0, None, Some(&mut mapped))` on a freshly created `UPLOAD_HEAP` resource with
        // no prior GPU access is always legal; the read-range `None` means "the CPU will not read this
        // resource", which is true here (write-only staging).
        unsafe { resource.Map(0, None, Some(&mut mapped)) }.unwrap_or_else(|error| panic!("d3d12 backend: failed to map upload buffer {label}: {error:?}"));
        // 🔓️ SAFETY: `mapped` is non-null (checked implicitly — `Map` returning `Ok` guarantees a valid
        // pointer per its documented contract) and valid for `desc.Width` bytes, which is
        // `>= bytes.len()` by construction (`buffer_desc` sizes the resource to exactly `bytes.len()`).
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), mapped.cast::<u8>(), bytes.len()) };
        // 🔓️ SAFETY: `Unmap(0, None)` matches the `Map` above; `None` as the written-range means "the
        // whole resource may have been written", the conservative and always-correct choice.
        unsafe { resource.Unmap(0, None) };
    }
    resource
}

/// 🏗️ Creates a `D3D12_HEAP_TYPE_DEFAULT` 2D texture in `initial_state`, optionally with an optimized
/// clear value (render targets and the depth/stencil buffer pass one; sampled-only textures pass
/// `None`). Shared by `🗃️resources.rs` (atlases/raster images), `🌫️scene_target.rs` (the offscreen
/// scene target + blur scratch) and `🪟️backend.rs` (the depth/stencil buffer).
#[allow(clippy::too_many_arguments)]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn create_default_texture2d(
    device: &ID3D12Device,
    format: DXGI_FORMAT,
    width: u32,
    height: u32,
    mip_levels: u16,
    flags: D3D12_RESOURCE_FLAGS,
    initial_state: D3D12_RESOURCE_STATES,
    clear: Option<D3D12_CLEAR_VALUE>,
    label: &str,
) -> ID3D12Resource {
    let desc = texture2d_desc(format, width, height, mip_levels, flags);
    let mut resource: Option<ID3D12Resource> = None;
    let clear_ptr = clear.as_ref().map(|value| value as *const _);
    // 🔓️ SAFETY: `desc` and `clear` are stack values valid for the call; `result__` is a valid
    // `*mut Option<ID3D12Resource>` out-param, matching every other `CreateCommittedResource` call in
    // this crate.
    unsafe { device.CreateCommittedResource(&DEFAULT_HEAP, D3D12_HEAP_FLAG_NONE, &desc, initial_state, clear_ptr, &mut resource) }.unwrap_or_else(|error| panic!("d3d12 backend: failed to allocate texture {label} ({width}x{height}): {error:?}"));
    resource.expect("CreateCommittedResource succeeded but returned no resource")
}

//#endregion 🏗️ResourceCreation

//#endregion 🔖️Types
