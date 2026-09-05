//! @emoji 📬️ Per-frame growable upload buffers — the D3D12 counterpart of the wgpu target's
//! `GrowBuffer`/`FrameBuffers` and the Metal backend's identical `📬️frame_buffers.rs` — plus
//! `FrameDescriptors`, the per-frame shader-visible SRV bump allocator this backend's root signature
//! needs that Metal's per-draw argument-table binding never did.
//!
//! **Why a bump allocator, not an in-place-overwritten slot.** A D3D12 command list only *records*
//! GPU work; nothing executes until `ID3D12CommandQueue::ExecuteCommandLists`. If two draws in the
//! same list needed different textures and this backend rewrote *the same* shader-visible descriptor
//! slot for each one (mirroring how Metal's `setFragmentTexture_atIndex` rebinds per draw), both draws
//! would see whichever texture was written *last*, because the GPU reads a heap slot's content at the
//! moment it actually executes that draw — long after every CPU-side descriptor write for the whole
//! frame already happened. The fix (a well-known D3D12 pattern, sometimes called a "linear"/"ring"
//! descriptor allocator) is for every draw that needs its own texture pair to get its own *permanent-
//! for-the-frame* pair of heap slots — `FrameDescriptors::allocate_pair` bump-allocates a fresh pair
//! and `CopyDescriptorsSimple`s the resident CPU descriptors into it, so it is safe to reuse only after
//! the whole frame's GPU work is known complete (`D3d12Backend::render` fences the *previous* frame
//! before calling `FrameDescriptors::begin_frame`, never mid-frame).
//!
//! **No mid-frame growth.** `begin_frame` is sized once, up front, to `packet.batches.len() * 2` —
//! the exact worst case (every batch needing its own fresh pair) — so the heap this backend binds via
//! `SetDescriptorHeaps` is fixed for the whole `render()` call. Growing a *shader-visible* heap mid-
//! recording is legal in D3D12 (re-issuing `SetDescriptorHeaps` with a bigger heap partway through a
//! command list works, as long as the old heap object outlives every draw already recorded against
//! it) but adds real bookkeeping this backend does not need, since the exact per-frame descriptor
//! demand is knowable before recording starts.

use windows::Win32::Graphics::Direct3D12::*;

//#region 🔖️FrameBuffers

type Device = ID3D12Device;

//#region 📬️GrowBuffer

/// 📬️ One growable `D3D12_HEAP_TYPE_UPLOAD` buffer, rewritten wholesale every upload. Ported from the
/// wgpu target's/Metal backend's `GrowBuffer` 1:1 — same growth policy (next power of two, floor 256
/// bytes, never shrinks), same "empty data uploads nothing and returns `None`" contract.
#[derive(Default)]
pub struct GrowBuffer {
    buffer: Option<ID3D12Resource>,
    capacity: usize,
}

impl GrowBuffer {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn buffer(&self) -> Option<&ID3D12Resource> {
        self.buffer.as_ref()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn gpu_address(&self) -> Option<u64> {
        // 🔓️ SAFETY: `GetGPUVirtualAddress` is a plain accessor on a live buffer this call borrows.
        self.buffer.as_ref().map(|buffer| unsafe { buffer.GetGPUVirtualAddress() })
    }

    /// 📤️ Grows (never shrinks) to fit `bytes`, then copies `bytes` into the buffer's start via
    /// `Map`/`Unmap`. Returns `None` (and leaves any existing buffer untouched) for empty input,
    /// matching the wgpu target's/Metal's "nothing to draw" no-op.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn upload(&mut self, device: &Device, bytes: &[u8]) -> Option<&ID3D12Resource> {
        if bytes.is_empty() {
            return None;
        }
        if self.capacity < bytes.len() {
            self.capacity = bytes.len().next_power_of_two().max(256);
            self.buffer = Some(crate::types::create_upload_buffer(device, &vec![0u8; self.capacity], "frame_grow_buffer"));
        }
        let buffer = self.buffer.as_ref()?;
        let mut mapped: *mut core::ffi::c_void = std::ptr::null_mut();
        // 🔓️ SAFETY: `buffer` is a `D3D12_HEAP_TYPE_UPLOAD` resource this struct exclusively owns and
        // never hands to the GPU as anything but a read-only source (vertex/index/root-CBV) — mapping
        // it for a CPU write here, then unmapping, is always legal and the whole-resource
        // read-range/written-range of `None` is the documented conservative choice.
        unsafe { buffer.Map(0, None, Some(&mut mapped)) }.expect("d3d12 backend: failed to map a frame grow buffer");
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), mapped.cast::<u8>(), bytes.len()) };
        unsafe { buffer.Unmap(0, None) };
        self.buffer.as_ref()
    }
}

/// 📬️ Every per-frame growable buffer a render pass writes into before encoding draws.
///
/// **Same simplification as Metal's `FrameBuffers` vs. the wgpu target's**: `RenderPacket::
/// quad_instances`/`vector_vertices` are already flat arrays covering every batch (backdrop,
/// foreground, overlay, and the silhouette-mask quads `Scene::finish` appended), and every
/// `DrawBatch::instance_range`/`mask_range` is already an offset into that same shared array. So this
/// backend uploads each array **once per frame** and binds each batch's slice by
/// `D3D12_VERTEX_BUFFER_VIEW::BufferLocation` offset — no per-batch-group re-collection.
#[derive(Default)]
pub struct FrameBuffers {
    pub quad_instances: GrowBuffer,
    pub vector_vertices: GrowBuffer,
    pub glass_instances: GrowBuffer,
    pub world_instances: GrowBuffer,
    pub world_lines: GrowBuffer,
    /// 🌐️ The world-globals ring and the blur per-mip scalar ring both bind through the shared root
    /// CBV (`b0`) — see `🏗️pipelines.rs`'s header — so this backend keeps them as two more
    /// `GrowBuffer`s here rather than inventing a distinct buffer type per uniform kind.
    pub world_globals: GrowBuffer,
    pub blur_globals: GrowBuffer,
}

//#endregion 📬️GrowBuffer

//#region 🗄️FrameDescriptors

/// 🗄️ The per-frame shader-visible SRV bump allocator this file's header describes.
#[derive(Default)]
pub struct FrameDescriptors {
    heap: Option<ID3D12DescriptorHeap>,
    capacity: u32,
    cursor: u32,
    stride: u32,
}

impl FrameDescriptors {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn heap(&self) -> Option<&ID3D12DescriptorHeap> {
        self.heap.as_ref()
    }

    /// 🏁️ Called once at the start of `render()`, after the previous frame's GPU work is fenced
    /// complete — see this file's header for why growth never happens mid-frame instead. Recreates
    /// the heap only if `min_pairs * 2` exceeds the current capacity (never shrinks); always resets
    /// the bump cursor to 0.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn begin_frame(&mut self, device: &Device, min_pairs: u32) {
        let needed = min_pairs.max(1) * 2;
        if self.heap.is_none() || needed > self.capacity {
            let capacity = needed.max(self.capacity.max(32)).next_power_of_two();
            let desc = D3D12_DESCRIPTOR_HEAP_DESC { Type: D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, NumDescriptors: capacity, Flags: D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE, NodeMask: 0 };
            // 🔓️ SAFETY: plain descriptor-heap creation from a stack-local desc.
            let heap: ID3D12DescriptorHeap = unsafe { device.CreateDescriptorHeap(&desc) }.expect("d3d12 backend: failed to allocate the per-frame SRV heap");
            self.stride = unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV) };
            self.heap = Some(heap);
            self.capacity = capacity;
        }
        self.cursor = 0;
    }

    /// ✏️ Copies two resident CPU descriptors (`cpu_a` at the returned handle's `t0`, `cpu_b` at
    /// `t1`) into a fresh, never-reused-this-frame pair of slots, and returns the GPU handle of the
    /// first — ready to bind directly via `SetGraphicsRootDescriptorTable(1, handle)`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn allocate_pair(&mut self, device: &Device, cpu_a: D3D12_CPU_DESCRIPTOR_HANDLE, cpu_b: D3D12_CPU_DESCRIPTOR_HANDLE) -> D3D12_GPU_DESCRIPTOR_HANDLE {
        let heap = self.heap.as_ref().expect("begin_frame always allocates a heap before any allocate_pair call");
        assert!(self.cursor + 2 <= self.capacity, "d3d12 backend: FrameDescriptors exceeded its begin_frame-sized capacity — a batch count changed between sizing and recording");
        let cpu_start = unsafe { heap.GetCPUDescriptorHandleForHeapStart() };
        let dest_a = D3D12_CPU_DESCRIPTOR_HANDLE { ptr: cpu_start.ptr + (self.cursor as usize) * (self.stride as usize) };
        let dest_b = D3D12_CPU_DESCRIPTOR_HANDLE { ptr: cpu_start.ptr + ((self.cursor + 1) as usize) * (self.stride as usize) };
        // 🔓️ SAFETY: `dest_a`/`dest_b` are within `heap`'s `capacity` bound (checked by the assertion
        // above); `cpu_a`/`cpu_b` are resident descriptors the caller owns for the crate's lifetime
        // (`GpuResources`/`SceneTarget`'s own heaps), valid for this synchronous copy.
        unsafe {
            device.CopyDescriptorsSimple(1, dest_a, cpu_a, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV);
            device.CopyDescriptorsSimple(1, dest_b, cpu_b, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV);
        }
        let gpu_start = unsafe { heap.GetGPUDescriptorHandleForHeapStart() };
        let handle = D3D12_GPU_DESCRIPTOR_HANDLE { ptr: gpu_start.ptr + (self.cursor as u64) * (self.stride as u64) };
        self.cursor += 2;
        handle
    }
}

//#endregion 🗄️FrameDescriptors

//#endregion 🔖️FrameBuffers

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ The device-touching branches (`upload`/`begin_frame`/`allocate_pair`) need a live D3D12
    /// device — exercised from `🪟️backend.rs`'s gated test module instead of here, since neither a
    /// `&Device` nor a `D3D12_CPU_DESCRIPTOR_HANDLE` can be conjured without one. This only checks
    /// construction.
    #[test]
    fn a_fresh_grow_buffer_starts_with_no_backing_buffer() {
        let buffer = GrowBuffer::default();
        assert!(buffer.buffer().is_none());
        assert!(buffer.gpu_address().is_none());
        let frame = FrameBuffers::default();
        assert!(frame.quad_instances.buffer().is_none());
        assert!(frame.world_instances.buffer().is_none());
    }

    #[test]
    fn a_fresh_frame_descriptor_allocator_starts_with_no_heap() {
        let descriptors = FrameDescriptors::default();
        assert!(descriptors.heap().is_none());
    }
}

//#endregion Tests
