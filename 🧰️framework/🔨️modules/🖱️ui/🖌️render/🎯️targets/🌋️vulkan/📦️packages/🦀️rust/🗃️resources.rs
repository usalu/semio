//! @emoji 🗃️ GPU-side residency for `ui_render::ResourceOp` — the Vulkan counterpart of the Metal
//! target's `🗃️resources.rs`, keyed by the same typed generational ids (`TextureId`/`MeshId`/
//! `AtlasId`). Every upload goes through a host-visible staging buffer into a `DEVICE_LOCAL` image or
//! buffer (dedicated `vkAllocateMemory` per resource, per the ticket brief — no VMA, no per-resource
//! sub-allocation pool) with the transfer submitted and waited on synchronously — simple and correct
//! first; a transfer queue / async upload ring is future work once a real frame loop needs the
//! throughput.
//!
//! **Atlas routing** mirrors the Metal target's `🗃️resources.rs` exactly (see that file's header):
//! the contract has one `AtlasId` type for both the glyph (1 byte/pixel) and icon (4 bytes/pixel)
//! pages, so this table infers which fixed slot an upload belongs to from its own byte density.
//!
//! **Sampling is not wired** — `raster_textures`/`glyph_atlas`/`icon_atlas` are created with
//! `SHADER_READ_ONLY_OPTIMAL` final layout and `SAMPLED` usage so they are *ready* to be sampled the
//! moment a pipeline exists, but no `vk::Sampler`/descriptor set binds them yet (blocked on the same
//! missing `vk::ShaderModule` as milestone 2's pipelines — see `📓️terra-backend-vulkan-report.md`).

use crate::memory::{as_bytes, find_memory_type};
use crate::vk_error::VulkanGraphicsError;
use ash::vk;
use std::collections::{HashMap, HashSet};
use ui_render::{AtlasId, MeshId, ResourceOp, TextureId};

//#region 🔖️Resources

//#region 🎨️AtlasRouting

/// 🎨️ Which fixed atlas slot an `UploadAtlas` op belongs to, inferred from its own byte density —
/// pure and tested without a device (mirrors the Metal target's routing exactly, see this file's
/// header). `pixel_count == 0` (a genuinely empty atlas request) routes to `Glyph` arbitrarily; no
/// upload actually happens for it either way (`upload_atlas` below returns early on empty pixels).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtlasFormat {
    Glyph,
    Icon,
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn classify_atlas_upload(width: u32, height: u32, byte_len: usize) -> Result<AtlasFormat, VulkanGraphicsError> {
    let pixel_count = (width as usize) * (height as usize);
    if pixel_count == 0 {
        return Ok(AtlasFormat::Glyph);
    }
    match byte_len / pixel_count {
        1 => Ok(AtlasFormat::Glyph),
        4 => Ok(AtlasFormat::Icon),
        other => Err(VulkanGraphicsError::UnsupportedAtlasChannels(other as u32)),
    }
}

impl AtlasFormat {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn vk_format(self) -> vk::Format {
        match self {
            AtlasFormat::Glyph => vk::Format::R8_UNORM,
            AtlasFormat::Icon => vk::Format::R8G8B8A8_SRGB,
        }
    }
}

//#endregion 🎨️AtlasRouting

//#region 🧊️Handles

/// 🧊️ One device-resident 2D image + its view — owns both and must be destroyed by the caller before
/// the memory backing it is freed (`destroy` frees all three together in the correct order).
pub struct VulkanImage {
    pub image: vk::Image,
    memory: vk::DeviceMemory,
    pub view: vk::ImageView,
    pub width: u32,
    pub height: u32,
}

impl VulkanImage {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn destroy(&self, device: &ash::Device) {
        // 🔓️ SAFETY: caller (`GpuResources::drain_known`/`Drop`) guarantees the device is idle before
        // any destroy path runs — no in-flight command buffer references this image/view.
        unsafe {
            device.destroy_image_view(self.view, None);
            device.destroy_image(self.image, None);
            device.free_memory(self.memory, None);
        }
    }
}

/// 🧊️ One resident world3d mesh: interleaved position+normal vertex buffer, u32 index buffer — both
/// `DEVICE_LOCAL`, uploaded once via staging and never rewritten in place (a changed mesh gets a new
/// `MeshId`, per `ui_render::ResourceRegistry::request_mesh_upload`'s content-hash versioning).
pub struct VulkanMesh {
    vertex_buffer: vk::Buffer,
    vertex_memory: vk::DeviceMemory,
    index_buffer: vk::Buffer,
    index_memory: vk::DeviceMemory,
    pub index_count: u32,
}

impl VulkanMesh {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn destroy(&self, device: &ash::Device) {
        // 🔓️ SAFETY: same idle-device precondition as `VulkanImage::destroy`.
        unsafe {
            device.destroy_buffer(self.vertex_buffer, None);
            device.free_memory(self.vertex_memory, None);
            device.destroy_buffer(self.index_buffer, None);
            device.free_memory(self.index_memory, None);
        }
    }
}

//#endregion 🧊️Handles

//#region 🗃️GpuResources

/// 🗃️ Owns every device-resident resource a `RenderPacket` can reference. `apply` is the only
/// mutator; everything else is a lookup a render pass would consult while replaying batches (not yet
/// wired — see this file's header).
#[derive(Default)]
pub struct GpuResources {
    glyph_atlas: Option<VulkanImage>,
    icon_atlas: Option<VulkanImage>,
    raster_textures: HashMap<TextureId, VulkanImage>,
    meshes: HashMap<MeshId, VulkanMesh>,
    known_textures: HashSet<TextureId>,
    known_meshes: HashSet<MeshId>,
    known_atlases: HashSet<AtlasId>,
}

impl GpuResources {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn knows_texture(&self, id: TextureId) -> bool {
        self.known_textures.contains(&id)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn knows_mesh(&self, id: MeshId) -> bool {
        self.known_meshes.contains(&id)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn raster_texture(&self, id: TextureId) -> Option<&VulkanImage> {
        self.raster_textures.get(&id)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn mesh(&self, id: MeshId) -> Option<&VulkanMesh> {
        self.meshes.get(&id)
    }

    /// ♻️ Destroys every device-resident resource and drains the known-id tables, for
    /// `GraphicsBackend::recover` reporting after `debug_force_device_loss` — mirrors the Metal
    /// target's `GpuResources::drain_known`. Caller (`VulkanBackend::recover`) must have already
    /// waited for the device to go idle (or, on a genuine device-lost, accepted that `device_wait_idle`
    /// itself may report `ERROR_DEVICE_LOST` and skipped straight to dropping handles — a lost device's
    /// resources are already invalid GPU-side either way).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn drain_known(&mut self, device: &ash::Device) -> (Vec<TextureId>, Vec<MeshId>, Vec<AtlasId>) {
        if let Some(atlas) = self.glyph_atlas.take() {
            atlas.destroy(device);
        }
        if let Some(atlas) = self.icon_atlas.take() {
            atlas.destroy(device);
        }
        for (_, texture) in self.raster_textures.drain() {
            texture.destroy(device);
        }
        for (_, mesh) in self.meshes.drain() {
            mesh.destroy(device);
        }
        (self.known_textures.drain().collect(), self.known_meshes.drain().collect(), self.known_atlases.drain().collect())
    }

    /// 📤️ Applies one `ResourceOp` stream, always *before* the `render` call whose packet references
    /// the ids it uploads (the `GraphicsBackend` contract's own invariant).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn apply(&mut self, device: &ash::Device, memory_properties: &vk::PhysicalDeviceMemoryProperties, command_pool: vk::CommandPool, queue: vk::Queue, ops: &[ResourceOp]) -> Result<(), VulkanGraphicsError> {
        for op in ops {
            match op {
                ResourceOp::UploadAtlas { id, width, height, pixels } => self.upload_atlas(device, memory_properties, command_pool, queue, *id, *width, *height, pixels)?,
                ResourceOp::UploadTexture { id, width, height, pixels } => self.upload_texture(device, memory_properties, command_pool, queue, *id, *width, *height, pixels)?,
                ResourceOp::CreateOrUpdateMesh { id, positions, normals, indices } => self.create_or_update_mesh(device, memory_properties, command_pool, queue, *id, positions, normals, indices)?,
                ResourceOp::EvictTexture(id) => {
                    if let Some(texture) = self.raster_textures.remove(id) {
                        texture.destroy(device);
                    }
                    self.known_textures.remove(id);
                }
                ResourceOp::EvictMesh(id) => {
                    if let Some(mesh) = self.meshes.remove(id) {
                        mesh.destroy(device);
                    }
                    self.known_meshes.remove(id);
                }
            }
        }
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[allow(clippy::too_many_arguments)]
    fn upload_atlas(&mut self, device: &ash::Device, memory_properties: &vk::PhysicalDeviceMemoryProperties, command_pool: vk::CommandPool, queue: vk::Queue, id: AtlasId, width: u32, height: u32, pixels: &[u8]) -> Result<(), VulkanGraphicsError> {
        self.known_atlases.insert(id);
        if pixels.is_empty() || width == 0 || height == 0 {
            return Ok(());
        }
        let format = classify_atlas_upload(width, height, pixels.len())?;
        let image = create_sampled_image(device, memory_properties, command_pool, queue, width, height, format.vk_format(), pixels)?;
        match format {
            AtlasFormat::Glyph => self.glyph_atlas = Some(image),
            AtlasFormat::Icon => self.icon_atlas = Some(image),
        }
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[allow(clippy::too_many_arguments)]
    fn upload_texture(
        &mut self,
        device: &ash::Device,
        memory_properties: &vk::PhysicalDeviceMemoryProperties,
        command_pool: vk::CommandPool,
        queue: vk::Queue,
        id: TextureId,
        width: u32,
        height: u32,
        pixels: &[u8],
    ) -> Result<(), VulkanGraphicsError> {
        self.known_textures.insert(id);
        if pixels.is_empty() || width == 0 || height == 0 {
            return Ok(());
        }
        let image = create_sampled_image(device, memory_properties, command_pool, queue, width, height, vk::Format::R8G8B8A8_SRGB, pixels)?;
        self.raster_textures.insert(id, image);
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[allow(clippy::too_many_arguments)]
    fn create_or_update_mesh(
        &mut self,
        device: &ash::Device,
        memory_properties: &vk::PhysicalDeviceMemoryProperties,
        command_pool: vk::CommandPool,
        queue: vk::Queue,
        id: MeshId,
        positions: &[f32],
        normals: &[f32],
        indices: &[u32],
    ) -> Result<(), VulkanGraphicsError> {
        self.known_meshes.insert(id);
        let vertex_count = positions.len() / 3;
        let mut vertices = Vec::with_capacity(vertex_count * 6);
        for index in 0..vertex_count {
            vertices.extend_from_slice(&positions[index * 3..index * 3 + 3]);
            vertices.push(normals.get(index * 3).copied().unwrap_or(0.0));
            vertices.push(normals.get(index * 3 + 1).copied().unwrap_or(1.0));
            vertices.push(normals.get(index * 3 + 2).copied().unwrap_or(0.0));
        }
        let (vertex_buffer, vertex_memory) = create_device_local_buffer(device, memory_properties, command_pool, queue, vk::BufferUsageFlags::VERTEX_BUFFER, as_bytes(&vertices))?;
        let (index_buffer, index_memory) = create_device_local_buffer(device, memory_properties, command_pool, queue, vk::BufferUsageFlags::INDEX_BUFFER, as_bytes(indices))?;
        self.meshes.insert(id, VulkanMesh { vertex_buffer, vertex_memory, index_buffer, index_memory, index_count: indices.len() as u32 });
        Ok(())
    }
}

//#endregion 🗃️GpuResources

//#region 🚚️Transfer

/// 🚚️ Allocates, begins, hands `record` the command buffer, ends, submits, and waits on `queue` —
/// the standard "one-shot upload" pattern (vulkan-tutorial's `beginSingleTimeCommands`/
/// `endSingleTimeCommands`, ported). Synchronous by construction (`queue_wait_idle` blocks until the
/// transfer completes), which is the right trade-off for `apply_resources`: it always runs before the
/// `render` call whose packet references the uploaded ids, so nothing here can race a draw.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn one_time_commands(device: &ash::Device, command_pool: vk::CommandPool, queue: vk::Queue, record: impl FnOnce(vk::CommandBuffer)) -> Result<(), vk::Result> {
    let allocate_info = vk::CommandBufferAllocateInfo::default().command_pool(command_pool).level(vk::CommandBufferLevel::PRIMARY).command_buffer_count(1);
    // 🔓️ SAFETY: `command_pool` is a live pool owned by `VulkanBackend` for this device's whole
    // lifetime; this call only reads `allocate_info`, which borrows stack locals.
    let command_buffers = unsafe { device.allocate_command_buffers(&allocate_info)? };
    let command_buffer = command_buffers[0];
    let begin_info = vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    // 🔓️ SAFETY: `command_buffer` was just allocated above and is in the initial state, the only
    // state `begin_command_buffer` accepts.
    unsafe { device.begin_command_buffer(command_buffer, &begin_info)? };
    record(command_buffer);
    // 🔓️ SAFETY: `command_buffer` is in the recording state entered by `begin_command_buffer` above;
    // `record` only ever calls `cmd_*` methods against it (its own contract, upheld by every call site
    // in this file).
    unsafe { device.end_command_buffer(command_buffer)? };
    let command_buffers_ref = [command_buffer];
    let submit_info = vk::SubmitInfo::default().command_buffers(&command_buffers_ref);
    // 🔓️ SAFETY: `queue` is the backend's single graphics queue, valid for the device's lifetime;
    // `vk::Fence::null()` is a documented valid "no fence" submission — this function waits via
    // `queue_wait_idle` instead, which is coarser but correct for a rare, non-hot-path upload.
    unsafe {
        device.queue_submit(queue, &[submit_info], vk::Fence::null())?;
        device.queue_wait_idle(queue)?;
        device.free_command_buffers(command_pool, &command_buffers);
    }
    Ok(())
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn create_staging_buffer(device: &ash::Device, memory_properties: &vk::PhysicalDeviceMemoryProperties, bytes: &[u8]) -> Result<(vk::Buffer, vk::DeviceMemory), VulkanGraphicsError> {
    let size = bytes.len().max(1) as vk::DeviceSize;
    let buffer_info = vk::BufferCreateInfo::default().size(size).usage(vk::BufferUsageFlags::TRANSFER_SRC).sharing_mode(vk::SharingMode::EXCLUSIVE);
    // 🔓️ SAFETY: `device` outlives this call; `buffer_info` borrows only stack locals.
    let buffer = unsafe { device.create_buffer(&buffer_info, None)? };
    // 🔓️ SAFETY: `buffer` was just created above, not yet bound — the documented call order.
    let requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
    let type_index = find_memory_type(memory_properties, &requirements, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT).ok_or(VulkanGraphicsError::NoSuitableMemoryType)?;
    let allocate_info = vk::MemoryAllocateInfo::default().allocation_size(requirements.size).memory_type_index(type_index);
    // 🔓️ SAFETY: sized/typed against `requirements`/`memory_properties` immediately above.
    let memory = unsafe { device.allocate_memory(&allocate_info, None)? };
    // 🔓️ SAFETY: both handles freshly created on `device`, offset `0`, sized to satisfy `requirements`.
    unsafe { device.bind_buffer_memory(buffer, memory, 0)? };
    if !bytes.is_empty() {
        // 🔓️ SAFETY: `memory` is host-visible and freshly allocated (not already mapped); the mapping
        // covers exactly the allocation and is unmapped before this function returns, so no dangling
        // mapping outlives the call.
        unsafe {
            let mapped = device.map_memory(memory, 0, size, vk::MemoryMapFlags::empty())?;
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), mapped.cast::<u8>(), bytes.len());
            device.unmap_memory(memory);
        }
    }
    Ok((buffer, memory))
}

/// 🖼️ Creates a `DEVICE_LOCAL`, `SAMPLED | TRANSFER_DST` 2D image at `format`, uploads `pixels` via a
/// staging buffer, and transitions it to `SHADER_READ_ONLY_OPTIMAL` — ready to be sampled the moment a
/// pipeline/descriptor set exists (not yet wired, see this file's header). Mip level 0 only; every
/// caller in this file passes already-decoded, already-packed pixel bytes (`bytes_per_row = width *
/// bytes_per_pixel`, no row padding) — the same assumption the Metal target's `replaceRegion` makes.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
#[allow(clippy::too_many_arguments)]
fn create_sampled_image(
    device: &ash::Device,
    memory_properties: &vk::PhysicalDeviceMemoryProperties,
    command_pool: vk::CommandPool,
    queue: vk::Queue,
    width: u32,
    height: u32,
    format: vk::Format,
    pixels: &[u8],
) -> Result<VulkanImage, VulkanGraphicsError> {
    let (staging_buffer, staging_memory) = create_staging_buffer(device, memory_properties, pixels)?;

    let image_info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::TYPE_2D)
        .format(format)
        .extent(vk::Extent3D { width, height, depth: 1 })
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    // 🔓️ SAFETY: `device` outlives this call; `image_info` borrows only stack locals.
    let image = unsafe { device.create_image(&image_info, None) };
    let image = match image {
        Ok(image) => image,
        Err(error) => {
            destroy_staging(device, staging_buffer, staging_memory);
            return Err(error.into());
        }
    };
    // 🔓️ SAFETY: `image` was just created above, not yet bound.
    let requirements = unsafe { device.get_image_memory_requirements(image) };
    let type_index = match find_memory_type(memory_properties, &requirements, vk::MemoryPropertyFlags::DEVICE_LOCAL) {
        Some(index) => index,
        None => {
            destroy_staging(device, staging_buffer, staging_memory);
            // 🔓️ SAFETY: `image` was created on `device` above and never bound to memory — safe to
            // destroy immediately on this early-return path.
            unsafe { device.destroy_image(image, None) };
            return Err(VulkanGraphicsError::NoSuitableMemoryType);
        }
    };
    let allocate_info = vk::MemoryAllocateInfo::default().allocation_size(requirements.size).memory_type_index(type_index);
    // 🔓️ SAFETY: sized/typed against `requirements`/`memory_properties` immediately above.
    let memory = unsafe { device.allocate_memory(&allocate_info, None) };
    let memory = match memory {
        Ok(memory) => memory,
        Err(error) => {
            destroy_staging(device, staging_buffer, staging_memory);
            unsafe { device.destroy_image(image, None) };
            return Err(error.into());
        }
    };
    // 🔓️ SAFETY: both handles freshly created on `device`, offset `0`, sized to satisfy `requirements`.
    if let Err(error) = unsafe { device.bind_image_memory(image, memory, 0) } {
        destroy_staging(device, staging_buffer, staging_memory);
        unsafe {
            device.destroy_image(image, None);
            device.free_memory(memory, None);
        }
        return Err(error.into());
    }

    let subresource = vk::ImageSubresourceRange { aspect_mask: vk::ImageAspectFlags::COLOR, base_mip_level: 0, level_count: 1, base_array_layer: 0, layer_count: 1 };
    let upload = one_time_commands(device, command_pool, queue, |command_buffer| {
        let to_transfer = vk::ImageMemoryBarrier::default()
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .image(image)
            .subresource_range(subresource)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE);
        // 🔓️ SAFETY: `command_buffer` is in the recording state (guaranteed by `one_time_commands`);
        // `image` was just created above and is not referenced by any other in-flight command buffer.
        unsafe { device.cmd_pipeline_barrier(command_buffer, vk::PipelineStageFlags::TOP_OF_PIPE, vk::PipelineStageFlags::TRANSFER, vk::DependencyFlags::empty(), &[], &[], &[to_transfer]) };

        let region = vk::BufferImageCopy::default()
            .buffer_offset(0)
            .buffer_row_length(0)
            .buffer_image_height(0)
            .image_subresource(vk::ImageSubresourceLayers { aspect_mask: vk::ImageAspectFlags::COLOR, mip_level: 0, base_array_layer: 0, layer_count: 1 })
            .image_offset(vk::Offset3D { x: 0, y: 0, z: 0 })
            .image_extent(vk::Extent3D { width, height, depth: 1 });
        // 🔓️ SAFETY: `staging_buffer` holds exactly `pixels.len()` tightly-packed bytes (no row
        // padding — `buffer_row_length`/`buffer_image_height` of `0` means "tightly packed", per the
        // spec), matching `region`'s `image_extent`; `image` is in `TRANSFER_DST_OPTIMAL` from the
        // barrier just recorded above.
        unsafe { device.cmd_copy_buffer_to_image(command_buffer, staging_buffer, image, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &[region]) };

        let to_shader_read = vk::ImageMemoryBarrier::default()
            .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .image(image)
            .subresource_range(subresource)
            .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .dst_access_mask(vk::AccessFlags::SHADER_READ);
        // 🔓️ SAFETY: same command buffer, still recording; the copy above is ordered before this
        // barrier by submission order within one command buffer (no reordering across `cmd_*` calls).
        unsafe { device.cmd_pipeline_barrier(command_buffer, vk::PipelineStageFlags::TRANSFER, vk::PipelineStageFlags::FRAGMENT_SHADER, vk::DependencyFlags::empty(), &[], &[], &[to_shader_read]) };
    });
    destroy_staging(device, staging_buffer, staging_memory);
    if let Err(error) = upload {
        unsafe {
            device.destroy_image(image, None);
            device.free_memory(memory, None);
        }
        return Err(error.into());
    }

    let view_info = vk::ImageViewCreateInfo::default().image(image).view_type(vk::ImageViewType::TYPE_2D).format(format).components(vk::ComponentMapping::default()).subresource_range(subresource);
    // 🔓️ SAFETY: `image` is now fully resident and in `SHADER_READ_ONLY_OPTIMAL`; `view_info`
    // borrows only stack locals.
    let view = match unsafe { device.create_image_view(&view_info, None) } {
        Ok(view) => view,
        Err(error) => {
            unsafe {
                device.destroy_image(image, None);
                device.free_memory(memory, None);
            }
            return Err(error.into());
        }
    };

    Ok(VulkanImage { image, memory, view, width, height })
}

/// 📤️ Staging-buffer-then-copy for a plain data buffer (vertex/index data) — same shape as
/// `create_sampled_image` without the layout transitions a buffer does not need.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn create_device_local_buffer(
    device: &ash::Device,
    memory_properties: &vk::PhysicalDeviceMemoryProperties,
    command_pool: vk::CommandPool,
    queue: vk::Queue,
    usage: vk::BufferUsageFlags,
    bytes: &[u8],
) -> Result<(vk::Buffer, vk::DeviceMemory), VulkanGraphicsError> {
    let (staging_buffer, staging_memory) = create_staging_buffer(device, memory_properties, bytes)?;
    let size = bytes.len().max(1) as vk::DeviceSize;
    let buffer_info = vk::BufferCreateInfo::default().size(size).usage(usage | vk::BufferUsageFlags::TRANSFER_DST).sharing_mode(vk::SharingMode::EXCLUSIVE);
    // 🔓️ SAFETY: `device` outlives this call; `buffer_info` borrows only stack locals.
    let buffer = unsafe { device.create_buffer(&buffer_info, None) };
    let buffer = match buffer {
        Ok(buffer) => buffer,
        Err(error) => {
            destroy_staging(device, staging_buffer, staging_memory);
            return Err(error.into());
        }
    };
    // 🔓️ SAFETY: `buffer` was just created above, not yet bound.
    let requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
    let type_index = match find_memory_type(memory_properties, &requirements, vk::MemoryPropertyFlags::DEVICE_LOCAL) {
        Some(index) => index,
        None => {
            destroy_staging(device, staging_buffer, staging_memory);
            unsafe { device.destroy_buffer(buffer, None) };
            return Err(VulkanGraphicsError::NoSuitableMemoryType);
        }
    };
    let allocate_info = vk::MemoryAllocateInfo::default().allocation_size(requirements.size).memory_type_index(type_index);
    // 🔓️ SAFETY: sized/typed against `requirements`/`memory_properties` immediately above.
    let memory = unsafe { device.allocate_memory(&allocate_info, None) };
    let memory = match memory {
        Ok(memory) => memory,
        Err(error) => {
            destroy_staging(device, staging_buffer, staging_memory);
            unsafe { device.destroy_buffer(buffer, None) };
            return Err(error.into());
        }
    };
    // 🔓️ SAFETY: both handles freshly created on `device`, offset `0`, sized to satisfy `requirements`.
    if let Err(error) = unsafe { device.bind_buffer_memory(buffer, memory, 0) } {
        destroy_staging(device, staging_buffer, staging_memory);
        unsafe {
            device.destroy_buffer(buffer, None);
            device.free_memory(memory, None);
        }
        return Err(error.into());
    }

    let copy = one_time_commands(device, command_pool, queue, |command_buffer| {
        let region = vk::BufferCopy { src_offset: 0, dst_offset: 0, size };
        // 🔓️ SAFETY: `staging_buffer` holds exactly `size` bytes (allocated to fit them above);
        // `buffer` was just bound to memory of at least `size` bytes.
        unsafe { device.cmd_copy_buffer(command_buffer, staging_buffer, buffer, &[region]) };
    });
    destroy_staging(device, staging_buffer, staging_memory);
    if let Err(error) = copy {
        unsafe {
            device.destroy_buffer(buffer, None);
            device.free_memory(memory, None);
        }
        return Err(error.into());
    }
    Ok((buffer, memory))
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn destroy_staging(device: &ash::Device, buffer: vk::Buffer, memory: vk::DeviceMemory) {
    // 🔓️ SAFETY: `buffer`/`memory` were allocated in this same file and `one_time_commands` has
    // already waited (`queue_wait_idle`) for every command referencing `buffer` to finish before this
    // is called — no in-flight GPU read of `buffer` remains.
    unsafe {
        device.destroy_buffer(buffer, None);
        device.free_memory(memory, None);
    }
}

//#endregion 🚚️Transfer

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_byte_per_pixel_routes_to_the_glyph_atlas() {
        assert_eq!(classify_atlas_upload(4, 4, 16).expect("classify"), AtlasFormat::Glyph);
    }

    #[test]
    fn four_bytes_per_pixel_routes_to_the_icon_atlas() {
        assert_eq!(classify_atlas_upload(4, 4, 64).expect("classify"), AtlasFormat::Icon);
    }

    #[test]
    fn an_unsupported_byte_density_is_rejected_cleanly() {
        let error = classify_atlas_upload(4, 4, 48).expect_err("3 bytes/pixel is not a supported atlas density");
        assert!(matches!(error, VulkanGraphicsError::UnsupportedAtlasChannels(3)));
    }

    #[test]
    fn a_zero_area_request_defaults_to_glyph_without_dividing_by_zero() {
        assert_eq!(classify_atlas_upload(0, 0, 0).expect("classify"), AtlasFormat::Glyph);
    }

    #[test]
    fn glyph_and_icon_formats_map_to_the_expected_vk_formats() {
        assert_eq!(AtlasFormat::Glyph.vk_format(), vk::Format::R8_UNORM);
        assert_eq!(AtlasFormat::Icon.vk_format(), vk::Format::R8G8B8A8_SRGB);
    }

    #[test]
    fn a_fresh_gpu_resources_table_knows_nothing_about_an_interned_but_unapplied_texture() {
        let mut registry = ui_render::ResourceRegistry::default();
        let id = registry.intern_texture("never_applied");
        let resources = GpuResources::default();
        assert!(!resources.knows_texture(id));
    }
}

//#endregion Tests

//#endregion 🔖️Resources
