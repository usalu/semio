//! @emoji 🗃️ GPU-side resource residency: [`ui_render::ResourceOp`] → textures/meshes/atlases, keyed
//! directly by [`ui_render::TextureId`]/[`ui_render::MeshId`] — no more `String` key, no per-frame
//! clone (`resource.rs`'s own docstring, ticket brief "what the typed-id change bought"). Replaces the
//! wgpu target's string-keyed `RasterTextureTable`/`MeshGpuTable`/fixed `glyph_texture`/`icon_texture`.

use crate::pipelines::Pipelines;
use std::collections::HashMap;
use ui_render::{AtlasId, BackendError, MeshId, ResourceKind, ResourceOp, TextureId};
use wgpu::util::DeviceExt;

//#region 🔖️Resources

//#region 🔤️Atlas

/// 🔤️ Which of the two fixed WGSL texture slots (`glyph_atlas` binding 1, `icon_atlas` binding 3) an
/// uploaded atlas belongs in. `ui_render::ResourceOp::UploadAtlas` carries no channel/format tag, so a
/// backend infers it from `pixels.len()` against `width * height` (1 byte/px) vs `width * height * 4`
/// (4 bytes/px) — the same distinction the reference shader bakes in (`glyph_atlas` is `R8Unorm`,
/// `icon_atlas` is `Rgba8UnormSrgb`). See this packet's report for the registrar-request to make this
/// explicit on `ResourceOp` instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AtlasSlotKind {
    Glyph,
    Icon,
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn classify_atlas_upload(width: u32, height: u32, pixel_len: usize) -> Option<AtlasSlotKind> {
    let pixels = width as usize * height as usize;
    if pixels == 0 {
        return None;
    }
    if pixel_len == pixels {
        Some(AtlasSlotKind::Glyph)
    } else if pixel_len == pixels * 4 {
        Some(AtlasSlotKind::Icon)
    } else {
        None
    }
}

struct AtlasSlot {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    width: u32,
    height: u32,
}

impl AtlasSlot {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn placeholder(device: &wgpu::Device, label: &str, format: wgpu::TextureFormat) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self { texture, view, width: 1, height: 1 }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn write(&self, queue: &wgpu::Queue, pixels: &[u8], bytes_per_pixel: u32) {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &self.texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            pixels,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(self.width * bytes_per_pixel), rows_per_image: Some(self.height) },
            wgpu::Extent3d { width: self.width, height: self.height, depth_or_array_layers: 1 },
        );
    }
}

//#endregion 🔤️Atlas

//#region 🖼️RasterTexture

struct RasterTexture {
    texture: wgpu::Texture,
    bind_group: wgpu::BindGroup,
}

//#endregion 🖼️RasterTexture

//#region 🧊️GpuMesh

pub(crate) struct GpuMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

//#endregion 🧊️GpuMesh

//#region 🗄️GpuResources

/// 🗄️ Owns every resident GPU texture/mesh/atlas plus the raster/glyph sampler shared by all of them.
/// [`Self::apply`] is this crate's whole implementation of [`ui_render::GraphicsBackend::apply_resources`].
pub(crate) struct GpuResources {
    textures: HashMap<TextureId, RasterTexture>,
    meshes: HashMap<MeshId, GpuMesh>,
    glyph: AtlasSlot,
    icon: AtlasSlot,
    glyph_atlas_id: Option<AtlasId>,
    icon_atlas_id: Option<AtlasId>,
    sampler: wgpu::Sampler,
    content_bind_group: wgpu::BindGroup,
}

impl GpuResources {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn new(device: &wgpu::Device, pipelines: &Pipelines) -> Self {
        let glyph = AtlasSlot::placeholder(device, "glyph_atlas", wgpu::TextureFormat::R8Unorm);
        let icon = AtlasSlot::placeholder(device, "icon_atlas", wgpu::TextureFormat::Rgba8UnormSrgb);
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor { label: Some("atlas_sampler"), mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, ..Default::default() });
        let content_bind_group = Self::build_content_bind_group(device, pipelines, &glyph, &icon, &sampler);
        Self { textures: HashMap::new(), meshes: HashMap::new(), glyph, icon, glyph_atlas_id: None, icon_atlas_id: None, sampler, content_bind_group }
    }

    /// 🧪️ `(textures, meshes, atlases)` — every id currently believed resident. Used by
    /// `ui_render::GraphicsBackend::recover` to report dead generations, and by `backend-testing`'s
    /// `debug_force_device_loss` to snapshot what a forced loss should report.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn resident_ids(&self) -> (Vec<TextureId>, Vec<MeshId>, Vec<AtlasId>) {
        let atlases = self.glyph_atlas_id.into_iter().chain(self.icon_atlas_id).collect();
        (self.textures.keys().copied().collect(), self.meshes.keys().copied().collect(), atlases)
    }

    /// 🧹️ Drops every tracked residency id (not the GPU objects themselves — the atlas placeholder
    /// textures stay put) so `recover`'s caller re-uploads from a clean slate.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn clear(&mut self) {
        self.textures.clear();
        self.meshes.clear();
        self.glyph_atlas_id = None;
        self.icon_atlas_id = None;
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn build_content_bind_group(device: &wgpu::Device, pipelines: &Pipelines, glyph: &AtlasSlot, icon: &AtlasSlot, sampler: &wgpu::Sampler) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ui_content_bind_group"),
            layout: &pipelines.ui_globals_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: pipelines.globals_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&glyph.view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(sampler) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&icon.view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(sampler) },
            ],
        })
    }

    /// 🖇️ Every raster texture's bind group also carries the glyph atlas at binding 1/2 (the canonical
    /// layout has no narrower shape), so a glyph-atlas resize invalidates every one of them along with
    /// the shared content bind group.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn rebuild_dependent_bind_groups(&mut self, device: &wgpu::Device, pipelines: &Pipelines) {
        self.content_bind_group = Self::build_content_bind_group(device, pipelines, &self.glyph, &self.icon, &self.sampler);
        let refreshed: Vec<(TextureId, RasterTexture)> = self
            .textures
            .drain()
            .map(|(id, existing)| {
                let bind_group = Self::build_raster_bind_group(device, pipelines, &self.glyph, &self.sampler, &existing.texture);
                (id, RasterTexture { texture: existing.texture, bind_group })
            })
            .collect();
        self.textures = refreshed.into_iter().collect();
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn build_raster_bind_group(device: &wgpu::Device, pipelines: &Pipelines, glyph: &AtlasSlot, sampler: &wgpu::Sampler, texture: &wgpu::Texture) -> wgpu::BindGroup {
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("raster_texture_bind_group"),
            layout: &pipelines.ui_globals_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: pipelines.globals_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&glyph.view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(sampler) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(sampler) },
            ],
        })
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn content_bind_group(&self) -> &wgpu::BindGroup {
        &self.content_bind_group
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn raster_bind_group(&self, id: TextureId) -> Option<&wgpu::BindGroup> {
        self.textures.get(&id).map(|entry| &entry.bind_group)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn mesh(&self, id: MeshId) -> Option<&GpuMesh> {
        self.meshes.get(&id)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn has_texture(&self, id: TextureId) -> bool {
        self.textures.contains_key(&id)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn has_mesh(&self, id: MeshId) -> bool {
        self.meshes.contains_key(&id)
    }

    /// 📤️ Applies one frame's worth of resource ops in order, mirroring
    /// `ui_render::GraphicsBackend::apply_resources`'s contract precisely.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn apply(&mut self, ops: &[ResourceOp], device: &wgpu::Device, queue: &wgpu::Queue, pipelines: &Pipelines) -> Result<(), BackendError> {
        for op in ops {
            match op {
                ResourceOp::UploadAtlas { id, width, height, pixels } => self.upload_atlas(device, queue, pipelines, *id, *width, *height, pixels)?,
                ResourceOp::UploadTexture { id, width, height, pixels } => self.upload_texture(device, queue, pipelines, *id, *width, *height, pixels),
                ResourceOp::CreateOrUpdateMesh { id, positions, normals, indices } => self.upload_mesh(device, *id, positions, normals, indices),
                ResourceOp::EvictTexture(id) => {
                    self.textures.remove(id);
                }
                ResourceOp::EvictMesh(id) => {
                    self.meshes.remove(id);
                }
            }
        }
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn upload_atlas(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, pipelines: &Pipelines, id: AtlasId, width: u32, height: u32, pixels: &[u8]) -> Result<(), BackendError> {
        let Some(kind) = classify_atlas_upload(width, height, pixels.len()) else {
            return Err(BackendError::UnsupportedFormat("atlas pixel length matches neither R8Unorm (glyph) nor Rgba8UnormSrgb (icon) for its width*height"));
        };
        match kind {
            AtlasSlotKind::Glyph => self.glyph_atlas_id = Some(id),
            AtlasSlotKind::Icon => self.icon_atlas_id = Some(id),
        }
        let (slot, format, bytes_per_pixel) = match kind {
            AtlasSlotKind::Glyph => (&mut self.glyph, wgpu::TextureFormat::R8Unorm, 1),
            AtlasSlotKind::Icon => (&mut self.icon, wgpu::TextureFormat::Rgba8UnormSrgb, 4),
        };
        let resized = slot.width != width || slot.height != height;
        if resized {
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some(match kind {
                    AtlasSlotKind::Glyph => "glyph_atlas",
                    AtlasSlotKind::Icon => "icon_atlas",
                }),
                size: wgpu::Extent3d { width: width.max(1), height: height.max(1), depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            *slot = AtlasSlot { texture, view, width: width.max(1), height: height.max(1) };
        }
        let target = match kind {
            AtlasSlotKind::Glyph => &self.glyph,
            AtlasSlotKind::Icon => &self.icon,
        };
        target.write(queue, pixels, bytes_per_pixel);
        if resized {
            self.rebuild_dependent_bind_groups(device, pipelines);
        }
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn upload_texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, pipelines: &Pipelines, id: TextureId, width: u32, height: u32, pixels: &[u8]) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("raster_texture"),
            size: wgpu::Extent3d { width: width.max(1), height: height.max(1), depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::TexelCopyTextureInfo { texture: &texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            pixels,
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(width * 4), rows_per_image: Some(height) },
            wgpu::Extent3d { width: width.max(1), height: height.max(1), depth_or_array_layers: 1 },
        );
        let bind_group = Self::build_raster_bind_group(device, pipelines, &self.glyph, &self.sampler, &texture);
        self.textures.insert(id, RasterTexture { texture, bind_group });
    }

    /// 🧊️ Interleaves `positions`/`normals` into `World3dVertex`; a short `normals` (or none) pads
    /// with `[0,1,0]`, matching `MeshGpuTable::ensure_mesh`'s own `unwrap_or` fallback.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn upload_mesh(&mut self, device: &wgpu::Device, id: MeshId, positions: &[f32], normals: &[f32], indices: &[u32]) {
        let vertex_count = positions.len() / 3;
        let mut vertices = Vec::with_capacity(vertex_count);
        for index in 0..vertex_count {
            let position = [positions[index * 3], positions[index * 3 + 1], positions[index * 3 + 2]];
            let normal = [normals.get(index * 3).copied().unwrap_or(0.0), normals.get(index * 3 + 1).copied().unwrap_or(1.0), normals.get(index * 3 + 2).copied().unwrap_or(0.0)];
            vertices.push(World3dVertex { position, normal });
        }
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("world3d_vertices"), contents: bytemuck::cast_slice(&vertices), usage: wgpu::BufferUsages::VERTEX });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("world3d_indices"), contents: bytemuck::cast_slice(indices), usage: wgpu::BufferUsages::INDEX });
        self.meshes.insert(id, GpuMesh { vertex_buffer, index_buffer, index_count: indices.len() as u32 });
    }
}

/// 🧊️ Mirrors `WORLD3D_SHADER`'s per-vertex `VertexInput { position, normal }` — the GPU-side vertex
/// layout `crate::pipelines`'s `WORLD3D_OPAQUE_PIPELINE`/`WORLD3D_TRANSLUCENT_PIPELINE` declare.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct World3dVertex {
    position: [f32; 3],
    normal: [f32; 3],
}

//#endregion 🗄️GpuResources

//#endregion 🔖️Resources

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_channel_pixels_classify_as_glyph() {
        assert_eq!(classify_atlas_upload(4, 4, 16), Some(AtlasSlotKind::Glyph));
    }

    #[test]
    fn four_channel_pixels_classify_as_icon() {
        assert_eq!(classify_atlas_upload(4, 4, 64), Some(AtlasSlotKind::Icon));
    }

    #[test]
    fn mismatched_length_classifies_as_none() {
        assert_eq!(classify_atlas_upload(4, 4, 10), None);
    }

    #[test]
    fn zero_area_classifies_as_none() {
        assert_eq!(classify_atlas_upload(0, 4, 0), None);
    }
}

//#endregion Tests
