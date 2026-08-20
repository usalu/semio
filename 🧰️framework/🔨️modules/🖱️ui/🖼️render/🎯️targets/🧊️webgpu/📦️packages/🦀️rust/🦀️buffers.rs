//! @emoji 📈️ Per-frame growable GPU buffers: [`GrowBuffer`] (upload-on-demand, doubles capacity
//! rather than reallocating every frame) and [`WorldGlobalsRing`] (a dynamic-offset uniform ring for
//! per-`SurfacePass` `view_proj`/`light_dir`). Ported from `🎯️targets/🧊️wgpu/🦀️draw.rs`.

use crate::gpu_uniforms::World3dGlobals;
use bytemuck::Pod;

//#region 🔖️Buffers

//#region 📈️GrowBuffer

/// 📏️ The byte stride of one dynamic-offset slot in [`WorldGlobalsRing`] — must be a multiple of the
/// device's `min_uniform_buffer_offset_alignment` (typically 256); 256 is always safe.
pub(crate) const WORLD_GLOBALS_SLOT_SIZE: u64 = 256;

/// 📈️ Grows its backing buffer to the next power of two whenever a larger upload arrives, instead of
/// reallocating every frame for a same-size (or shrinking) payload.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn grown_capacity(current: usize, required: usize) -> usize {
    if required <= current {
        current
    } else {
        required.next_power_of_two().max(256)
    }
}

#[derive(Default)]
pub(crate) struct GrowBuffer {
    buffer: Option<wgpu::Buffer>,
    capacity: usize,
}

impl GrowBuffer {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn slice(&self) -> Option<wgpu::BufferSlice<'_>> {
        self.buffer.as_ref().map(|buffer| buffer.slice(..))
    }

    /// 📤️ Uploads `data`, growing the backing buffer first if needed. Returns `None` for empty data
    /// (nothing to bind, matching `draw.rs`'s own early return) rather than an empty-but-valid slice.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn upload<T: Pod>(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, data: &[T], usage: wgpu::BufferUsages, label: &str) -> Option<wgpu::BufferSlice<'_>> {
        if data.is_empty() {
            return None;
        }
        let bytes = bytemuck::cast_slice(data);
        let grown = grown_capacity(self.capacity, bytes.len());
        if grown != self.capacity || self.buffer.is_none() {
            self.capacity = grown;
            self.buffer = Some(device.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: self.capacity as u64, usage, mapped_at_creation: false }));
        }
        let buffer = self.buffer.as_ref()?;
        queue.write_buffer(buffer, 0, bytes);
        Some(buffer.slice(..))
    }
}

//#endregion 📈️GrowBuffer

//#region 🌐️WorldGlobalsRing

/// 🌐️ A dynamic-offset uniform buffer holding one [`World3dGlobals`] per resident [`ui_render::
/// SurfacePass`] slot this frame, addressed via `set_bind_group`'s dynamic offset rather than a
/// separate bind group per pass.
pub(crate) struct WorldGlobalsRing {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    capacity_slots: u32,
}

impl WorldGlobalsRing {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, initial_slots: u32) -> Self {
        let capacity_slots = initial_slots.max(1);
        let buffer = Self::make_buffer(device, capacity_slots);
        let bind_group = Self::make_bind_group(device, layout, &buffer);
        Self { buffer, bind_group, capacity_slots }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn make_buffer(device: &wgpu::Device, slots: u32) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor { label: Some("world3d_globals_ring"), size: WORLD_GLOBALS_SLOT_SIZE * slots as u64, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false })
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn make_bind_group(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, buffer: &wgpu::Buffer) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("world3d_bind_group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding { buffer, offset: 0, size: std::num::NonZeroU64::new(std::mem::size_of::<World3dGlobals>() as u64) }),
            }],
        })
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn ensure_slots(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, slots: u32) {
        if slots <= self.capacity_slots {
            return;
        }
        self.capacity_slots = slots.next_power_of_two().max(4);
        self.buffer = Self::make_buffer(device, self.capacity_slots);
        self.bind_group = Self::make_bind_group(device, layout, &self.buffer);
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn write_passes(&self, queue: &wgpu::Queue, passes: &[World3dGlobals]) {
        for (index, globals) in passes.iter().enumerate() {
            queue.write_buffer(&self.buffer, index as u64 * WORLD_GLOBALS_SLOT_SIZE, bytemuck::bytes_of(globals));
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn offset_for_slot(&self, slot: u32) -> u32 {
        slot * WORLD_GLOBALS_SLOT_SIZE as u32
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

//#endregion 🌐️WorldGlobalsRing

//#region 🧺️FrameBuffers

/// 🧺️ Every per-frame [`GrowBuffer`] the render pass replay needs, bundled so `crate::frame::render`
/// can pass one `&mut` around instead of six.
#[derive(Default)]
pub(crate) struct FrameBuffers {
    pub quad_instances: GrowBuffer,
    pub vector_vertices: GrowBuffer,
    pub glass_instances: GrowBuffer,
    pub world_instances: GrowBuffer,
    pub world_lines: GrowBuffer,
    pub world_masks: GrowBuffer,
}

//#endregion 🧺️FrameBuffers

//#endregion 🔖️Buffers

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacity_stays_when_required_fits() {
        assert_eq!(grown_capacity(256, 100), 256);
        assert_eq!(grown_capacity(256, 256), 256);
    }

    #[test]
    fn capacity_grows_to_next_power_of_two() {
        assert_eq!(grown_capacity(256, 257), 512);
        assert_eq!(grown_capacity(0, 1), 256);
        assert_eq!(grown_capacity(0, 1000), 1024);
    }

    #[test]
    fn world_globals_slot_size_is_uniform_alignment_safe() {
        assert_eq!(WORLD_GLOBALS_SLOT_SIZE % 256, 0);
        assert!(WORLD_GLOBALS_SLOT_SIZE >= std::mem::size_of::<World3dGlobals>() as u64);
    }
}

//#endregion Tests
