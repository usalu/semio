//! @emoji 📬️ Per-frame growable upload buffers — the Metal counterpart of the wgpu target's
//! `GrowBuffer`/`FrameBuffers` (`🎯️targets/🧊️wgpu/🦀️draw.rs`). Every buffer here is `Shared` storage
//! (CPU-visible, no explicit `didModifyRange` sync needed — that flag only matters for `Managed`
//! storage on Intel Macs) and is reallocated only when a frame's data outgrows the current capacity,
//! never shrunk, mirroring the wgpu target's "next power of two, floor 256 bytes" growth policy.

use crate::objective_c::{MTLBuffer as MetalBuffer, MTLDevice as Device, Owned};
use objc2_metal::MTLResourceOptions;

//#region 🔖️FrameBuffers

/// 📬️ One growable `Shared`-storage buffer, rewritten wholesale every upload. Ported from
/// `GrowBuffer` in the wgpu target 1:1 — same growth policy, same "empty data uploads nothing and
/// returns `None`" contract.
#[derive(Default)]
pub struct GrowBuffer {
    buffer: Option<Owned<MetalBuffer>>,
    capacity: usize,
}

impl GrowBuffer {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn buffer(&self) -> Option<&MetalBuffer> {
        self.buffer.as_deref()
    }

    /// 📤️ Grows (never shrinks) to fit `bytes`, then copies `bytes` into the buffer's start via its
    /// CPU-visible `contents()` pointer — `Shared` storage keeps this synchronized with the GPU
    /// without an explicit flush. Returns `None` (and leaves any existing buffer untouched) for empty
    /// input, matching the wgpu target's "nothing to draw" no-op.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn upload(&mut self, device: &Device, bytes: &[u8]) -> Option<&MetalBuffer> {
        if bytes.is_empty() {
            return None;
        }
        if self.capacity < bytes.len() {
            self.capacity = bytes.len().next_power_of_two().max(256);
            self.buffer = device.newBufferWithLength_options(self.capacity as _, MTLResourceOptions::StorageModeShared);
        }
        let buffer = self.buffer.as_deref()?;
        // 🔓️ SAFETY: `Shared`-storage `contents()` is a CPU-writable pointer valid for `length()`
        // bytes for the buffer's whole lifetime; `bytes.len() <= self.capacity == buffer.length()` by
        // construction above, so this copy never writes past the allocation.
        unsafe {
            let destination = buffer.contents();
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), destination.as_ptr().cast::<u8>(), bytes.len());
        }
        Some(buffer)
    }
}

/// 📬️ Every per-frame growable buffer a render pass writes into before encoding draws.
///
/// **Simpler than the wgpu target's `FrameBuffers` — one buffer per array, not one per batch group.**
/// The wgpu target re-collects each filtered batch group (backdrop/foreground × normal/overlay) into
/// its own fresh small buffer (`build_layer_batches`/`build_overlay_layer_batches`) because its
/// `DrawLayer`s are the unit of storage. This crate's `RenderPacket` already did that flattening in
/// `Scene::finish`: `quad_instances`/`vector_vertices` are each **one** array covering every batch
/// (backdrop, foreground, overlay, and the silhouette-mask quads `build_batch_masks` appends), and
/// every `DrawBatch::instance_range`/`mask_range` is already an offset into that same shared array. So
/// this backend uploads each array **once per frame** and reads every batch's slice out of it by byte
/// offset — no separate mask/overlay buffers, and no re-collection pass.
#[derive(Default)]
pub struct FrameBuffers {
    pub quad_instances: GrowBuffer,
    pub vector_vertices: GrowBuffer,
    pub glass_instances: GrowBuffer,
    pub world_instances: GrowBuffer,
    pub world_lines: GrowBuffer,
}

//#endregion 🔖️FrameBuffers

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ The device-touching branch (`upload` with non-empty bytes, growth policy, the `contents()`
    /// copy) needs a live Metal device — exercised from `🦀️backend.rs`'s gated test module instead of
    /// here, since a `&Device` cannot be conjured without one. This only checks construction.
    #[test]
    fn a_fresh_grow_buffer_starts_with_no_backing_buffer() {
        let buffer = GrowBuffer::default();
        assert!(buffer.buffer().is_none());
        let frame = FrameBuffers::default();
        assert!(frame.quad_instances.buffer().is_none());
        assert!(frame.world_instances.buffer().is_none());
    }
}

//#endregion Tests
