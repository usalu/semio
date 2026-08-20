//! @emoji 💾️ Manual GPU memory management — no VMA (ticket brief, `backend-vulkan`). Two shapes only:
//! [`find_memory_type`] (dedicated `vkAllocateMemory` per resource — used by
//! `crate::resources::GpuResources` for textures/atlases/meshes, which are long-lived and few per
//! frame) and [`GrowBuffer`] (one capacity-doubling host-visible arena per frame-in-flight — the
//! Vulkan counterpart of the Metal target's `GrowBuffer`/`FrameBuffers`, used for the per-frame
//! `quad_instances`/`vector_vertices`/`glass_instances` upload once pipelines land). Every allocation
//! this file makes is `HOST_VISIBLE | HOST_COHERENT` (never `flush_mapped_memory_ranges`-managed) —
//! simple and correct first; a `DEVICE_LOCAL` + staging-buffer path is what `crate::resources` uses
//! instead for resources that outlive a frame.

use ash::vk;

//#region 🔖️Memory

//#region 🔍️FindMemoryType

/// 🔍️ Picks the first memory type index Vulkan reports as both eligible for `requirements`
/// (`memory_type_bits` bit `index` set) and carrying every flag in `required_properties` — the
/// textbook `vkGetPhysicalDeviceMemoryProperties` scan (vulkan-tutorial's `findMemoryType`, ported
/// verbatim). Pure over caller-supplied structs, so it is exercised here without a device — the
/// values a real call site passes (`instance.get_physical_device_memory_properties(..)`,
/// `device.get_buffer_memory_requirements(..)`) are themselves unverified without a Vulkan loader.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn find_memory_type(memory_properties: &vk::PhysicalDeviceMemoryProperties, requirements: &vk::MemoryRequirements, required_properties: vk::MemoryPropertyFlags) -> Option<u32> {
    for index in 0..memory_properties.memory_type_count {
        let bit = 1u32 << index;
        let type_supported = requirements.memory_type_bits & bit != 0;
        let memory_type = memory_properties.memory_types[index as usize];
        let properties_supported = memory_type.property_flags.contains(required_properties);
        if type_supported && properties_supported {
            return Some(index);
        }
    }
    None
}

//#endregion 🔍️FindMemoryType

//#region 📤️Bytes

/// 📤️ `bytemuck::cast_slice`'s job without the dependency — this crate's `Cargo.toml` is
/// registrar-only (ticket U7) and does not list `bytemuck`, unlike the Metal target's. Every caller
/// passes a `#[repr(C)]` `Copy` type from `ui_render::scene` (`QuadInstance`/`VectorVertex`/
/// `GlassInstance`, all also `bytemuck::Pod` there — that bound is stricter than what this function
/// needs: `Copy` alone already guarantees no destructor/interior-mutability hazard for a byte-view).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn as_bytes<T: Copy>(items: &[T]) -> &[u8] {
    // 🔓️ SAFETY: `T: Copy` rules out `Drop`/interior mutability; the byte range read below spans
    // exactly `items.len() * size_of::<T>()` bytes starting at `items.as_ptr()`, which is precisely
    // the slice's own backing allocation, so this never reads out of bounds. Every call site's `T` is additionally
    // `#[repr(C)]` with no padding (`ui_render::scene`'s own byte-layout tests assert this), so the
    // resulting bytes are a faithful GPU upload image, not an artifact of Rust's default layout.
    unsafe { std::slice::from_raw_parts(items.as_ptr().cast::<u8>(), std::mem::size_of_val(items)) }
}

//#endregion 📤️Bytes

//#region 📬️GrowBuffer

/// 📏️ Matches the Metal target's growth policy exactly (`next_power_of_two`, floor 256 bytes) so the
/// two backends' upload-buffer churn is comparable.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn grow_capacity(current_capacity: u64, needed_bytes: u64) -> u64 {
    if current_capacity >= needed_bytes {
        current_capacity
    } else {
        needed_bytes.next_power_of_two().max(256)
    }
}

/// 📬️ One growable `HOST_VISIBLE | HOST_COHERENT` buffer, rewritten wholesale every upload — the
/// per-frame-in-flight arena `crate::backend::VulkanBackend` allocates one of per
/// `FRAMES_IN_FLIGHT` slot once pipelines replay `RenderPacket::quad_instances`/`vector_vertices`/
/// `glass_instances` (milestone 2; not yet wired — see `📓️terra-backend-vulkan-report.md`). Never a
/// dedicated allocation: this is exactly the "grow-buffer arena per frame in flight" the ticket brief
/// asks for, ported from the Metal target's `GrowBuffer` with `Retained<MTLBuffer>` replaced by a raw
/// `(vk::Buffer, vk::DeviceMemory)` pair this type owns and must free itself (no ARC here).
pub struct GrowBuffer {
    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
    capacity: u64,
    mapped: *mut u8,
}

impl Default for GrowBuffer {
    fn default() -> Self {
        Self { buffer: vk::Buffer::null(), memory: vk::DeviceMemory::null(), capacity: 0, mapped: std::ptr::null_mut() }
    }
}

impl GrowBuffer {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn buffer(&self) -> Option<vk::Buffer> {
        (self.capacity > 0).then_some(self.buffer)
    }

    /// 🧨️ Frees the backing allocation. Callers (`VulkanBackend::drop`) must ensure the device is idle
    /// (no in-flight command buffer references `self.buffer`) before calling this — the same rule
    /// every other `destroy_*`/`free_*` call in this crate follows.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn destroy(&mut self, device: &ash::Device) {
        if self.capacity == 0 {
            return;
        }
        // 🔓️ SAFETY: `self.buffer`/`self.memory` were allocated by a prior `ensure` call on this same
        // `device` and are not referenced by any pending GPU work — guaranteed by the caller per this
        // method's doc comment. `unmap_memory` on already-unmapped memory is not attempted twice
        // because `capacity` is reset to `0` immediately after.
        unsafe {
            device.unmap_memory(self.memory);
            device.destroy_buffer(self.buffer, None);
            device.free_memory(self.memory, None);
        }
        *self = Self::default();
    }

    /// 📤️ Grows (never shrinks) to fit `bytes.len()`, then copies `bytes` into the mapped pointer's
    /// start. `HOST_COHERENT` means no explicit `flush_mapped_memory_ranges` is needed for the GPU to
    /// observe the write (mirrors the Metal target's `Shared`-storage reasoning). Returns `None` for
    /// empty input, matching `crate::memory`'s Metal analog's "nothing to draw" no-op.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn upload(&mut self, device: &ash::Device, memory_properties: &vk::PhysicalDeviceMemoryProperties, bytes: &[u8]) -> Result<Option<vk::Buffer>, vk::Result> {
        if bytes.is_empty() {
            return Ok(None);
        }
        let needed = grow_capacity(self.capacity, bytes.len() as u64);
        if needed != self.capacity {
            self.destroy(device);
            let buffer_info = vk::BufferCreateInfo::default().size(needed).usage(vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::UNIFORM_BUFFER).sharing_mode(vk::SharingMode::EXCLUSIVE);
            // 🔓️ SAFETY: `device` is the live logical device this buffer's lifetime is tied to;
            // `buffer_info` borrows only stack locals for the duration of the call.
            let buffer = unsafe { device.create_buffer(&buffer_info, None)? };
            // 🔓️ SAFETY: `buffer` was just created on `device` above and is not yet bound to memory —
            // querying its requirements before binding is the documented call order.
            let requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
            let type_index = find_memory_type(memory_properties, &requirements, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT).ok_or(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY)?;
            let allocate_info = vk::MemoryAllocateInfo::default().allocation_size(requirements.size).memory_type_index(type_index);
            // 🔓️ SAFETY: `allocate_info` is caller-checked against `requirements`/`memory_properties`
            // above; no `p_next` chain is attached.
            let memory = unsafe { device.allocate_memory(&allocate_info, None)? };
            // 🔓️ SAFETY: `buffer`/`memory` were both just created on `device`, at offset `0`, sized to
            // satisfy `requirements` — the exact precondition `bind_buffer_memory` documents.
            unsafe { device.bind_buffer_memory(buffer, memory, 0)? };
            // 🔓️ SAFETY: `memory` is host-visible (selected above) and not already mapped (freshly
            // allocated); the mapping covers the whole allocation (`0..vk::WHOLE_SIZE`) and is kept
            // for this buffer's lifetime, matching the Metal target's permanently-mapped `Shared`
            // buffers.
            let mapped = unsafe { device.map_memory(memory, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty())? };
            self.buffer = buffer;
            self.memory = memory;
            self.capacity = needed;
            self.mapped = mapped.cast::<u8>();
        }
        // 🔓️ SAFETY: `self.mapped` is valid for `self.capacity` bytes for as long as `self.memory`
        // stays mapped (its whole lifetime, per the mapping above), and `bytes.len() <= self.capacity`
        // by construction of `needed` — this copy never writes past the allocation.
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), self.mapped, bytes.len());
        }
        Ok(Some(self.buffer))
    }
}

//#endregion 📬️GrowBuffer

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn memory_properties_with(types: &[(u32, vk::MemoryPropertyFlags)]) -> vk::PhysicalDeviceMemoryProperties {
        let mut properties = vk::PhysicalDeviceMemoryProperties::default();
        properties.memory_type_count = types.len() as u32;
        for (index, (heap_index, flags)) in types.iter().enumerate() {
            properties.memory_types[index] = vk::MemoryType { property_flags: *flags, heap_index: *heap_index };
        }
        properties
    }

    #[test]
    fn finds_the_first_type_matching_both_the_bitmask_and_the_required_flags() {
        let properties = memory_properties_with(&[(0, vk::MemoryPropertyFlags::DEVICE_LOCAL), (0, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT)]);
        let requirements = vk::MemoryRequirements { size: 256, alignment: 16, memory_type_bits: 0b11 };
        let found = find_memory_type(&properties, &requirements, vk::MemoryPropertyFlags::HOST_VISIBLE);
        assert_eq!(found, Some(1));
    }

    #[test]
    fn refuses_a_type_the_requirements_bitmask_excludes_even_if_flags_match() {
        let properties = memory_properties_with(&[(0, vk::MemoryPropertyFlags::HOST_VISIBLE)]);
        // 🎯️ bit 0 excluded from the mask — the only host-visible type is not actually usable for
        // this resource, and the scan must not fall back to it anyway.
        let requirements = vk::MemoryRequirements { size: 256, alignment: 16, memory_type_bits: 0b0 };
        assert_eq!(find_memory_type(&properties, &requirements, vk::MemoryPropertyFlags::HOST_VISIBLE), None);
    }

    #[test]
    fn returns_none_when_no_type_carries_every_required_flag() {
        let properties = memory_properties_with(&[(0, vk::MemoryPropertyFlags::DEVICE_LOCAL)]);
        let requirements = vk::MemoryRequirements { size: 256, alignment: 16, memory_type_bits: 0b1 };
        assert_eq!(find_memory_type(&properties, &requirements, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT), None);
    }

    #[test]
    fn grow_capacity_never_shrinks_and_rounds_up_to_a_power_of_two_floored_at_256() {
        assert_eq!(grow_capacity(0, 10), 256);
        assert_eq!(grow_capacity(256, 10), 256);
        assert_eq!(grow_capacity(256, 300), 512);
        assert_eq!(grow_capacity(1024, 300), 1024);
    }

    #[test]
    fn as_bytes_reports_the_exact_byte_length_of_the_slice() {
        let values: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(as_bytes(&values).len(), 16);
    }
}

//#endregion Tests

//#endregion 🔖️Memory
