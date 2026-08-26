//! @emoji 🗃️ GPU-side residency for `ui_render::resource::ResourceOp` — the Metal counterpart of the
//! wgpu target's `RasterTextureTable`/`MeshGpuTable`, keyed by the typed generational ids
//! (`TextureId`/`MeshId`/`AtlasId`) rather than interned strings, since `ui_render::ResourceRegistry`
//! already did the interning.
//!
//! **Atlas routing.** The contract has exactly one `AtlasId` type for both the glyph (alpha, 1
//! channel) and icon/color (RGBA, 4 channel) atlas pages — see `ui_render`'s `text.rs`
//! (`AtlasPage::new` calls with `channels: 1` for the alpha page, `channels: 4` for the color page).
//! There is no `AtlasKind` in the contract to dispatch on, so this crate infers the page from the
//! upload's own byte density (`pixels.len() / (width * height)`): 1 byte/pixel routes to the fixed
//! `glyph_atlas` texture slot the UI megashader samples at `texture(0)`, 4 bytes/pixel routes to the
//! fixed `icon_atlas` slot at `texture(1)` — mirroring `UI_SHADER`'s two hard-coded atlas bindings
//! exactly (never a per-draw atlas choice).

use crate::backend::MetalGraphicsError;
use crate::objective_c::{MTLBuffer as MetalBuffer, MTLDevice as Device, MTLTexture as MetalTexture, MTLTextureDescriptor, Owned};
use crate::types::World3dGpuVertex;
use objc2_metal::{MTLPixelFormat, MTLRegion, MTLResourceOptions, MTLSize, MTLTextureUsage};
use std::collections::{HashMap, HashSet};
use ui_render::{AtlasId, MeshId, ResourceOp, TextureId};

//#region 🔖️Resources

/// 🧊️ One resident world3d mesh: interleaved position+normal vertex buffer, u32 index buffer.
pub struct MeshBuffers {
    pub vertex_buffer: Owned<MetalBuffer>,
    pub index_buffer: Owned<MetalBuffer>,
    pub index_count: u32,
}

/// 🗃️ Owns every device-resident resource a `RenderPacket` can reference. `apply_resources` is the
/// only mutator; everything else is a lookup a render pass consults while replaying batches.
#[derive(Default)]
pub struct GpuResources {
    glyph_atlas: Option<Owned<MetalTexture>>,
    icon_atlas: Option<Owned<MetalTexture>>,
    raster_textures: HashMap<TextureId, Owned<MetalTexture>>,
    meshes: HashMap<MeshId, MeshBuffers>,
    known_textures: HashSet<TextureId>,
    known_meshes: HashSet<MeshId>,
    known_atlases: HashSet<AtlasId>,
}

impl GpuResources {
    /// 🕳️ Seeds the glyph/icon atlas slots with 1x1 dummy textures so the UI megashader always has
    /// something bound at `texture(0)`/`texture(1)` even on a frame painted before any glyph/icon has
    /// ever been requested (a real, common first-frame state — `NullBackend` has no such gap because
    /// it never touches a device at all, but a real backend must bind *something*). Overwritten by the
    /// first real `ResourceOp::UploadAtlas` of each byte density.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(device: &Device) -> Self {
        let glyph_atlas = Some(create_texture(device, MTLPixelFormat::R8Unorm, 1, 1, 1, "glyph_atlas_dummy"));
        replace_region(glyph_atlas.as_deref().expect("just created"), 1, 1, &[0u8], 1);
        let icon_atlas = Some(create_texture(device, MTLPixelFormat::RGBA8Unorm_sRGB, 1, 1, 1, "icon_atlas_dummy"));
        replace_region(icon_atlas.as_deref().expect("just created"), 1, 1, &[0u8, 0, 0, 0], 4);
        Self { glyph_atlas, icon_atlas, ..Self::default() }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn glyph_atlas(&self) -> Option<&MetalTexture> {
        self.glyph_atlas.as_deref()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn icon_atlas(&self) -> Option<&MetalTexture> {
        self.icon_atlas.as_deref()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn raster_texture(&self, id: TextureId) -> Option<&MetalTexture> {
        self.raster_textures.get(&id).map(|texture| texture.as_ref())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn mesh(&self, id: MeshId) -> Option<&MeshBuffers> {
        self.meshes.get(&id)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn knows_texture(&self, id: TextureId) -> bool {
        self.known_textures.contains(&id)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn knows_mesh(&self, id: MeshId) -> bool {
        self.known_meshes.contains(&id)
    }

    /// ♻️ Drains every id this table currently believes resident, for `GraphicsBackend::recover`
    /// reporting after `debug_force_device_loss` — mirrors `NullBackend::debug_force_device_loss`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn drain_known(&mut self) -> (Vec<TextureId>, Vec<MeshId>, Vec<AtlasId>) {
        self.glyph_atlas = None;
        self.icon_atlas = None;
        self.raster_textures.clear();
        self.meshes.clear();
        (self.known_textures.drain().collect(), self.known_meshes.drain().collect(), self.known_atlases.drain().collect())
    }

    /// 📤️ Applies one `ResourceOp` stream, always *before* the `render` call whose packet references
    /// the ids it uploads (the trait's own invariant — see `backend.rs`'s `GraphicsBackend` docstring).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn apply(&mut self, device: &Device, ops: &[ResourceOp]) -> Result<(), MetalGraphicsError> {
        for op in ops {
            match op {
                ResourceOp::UploadAtlas { id, width, height, pixels } => self.upload_atlas(device, *id, *width, *height, pixels)?,
                ResourceOp::UploadTexture { id, width, height, pixels } => self.upload_texture(device, *id, *width, *height, pixels)?,
                ResourceOp::CreateOrUpdateMesh { id, positions, normals, indices } => self.create_or_update_mesh(device, *id, positions, normals, indices)?,
                ResourceOp::EvictTexture(id) => {
                    self.raster_textures.remove(id);
                    self.known_textures.remove(id);
                }
                ResourceOp::EvictMesh(id) => {
                    self.meshes.remove(id);
                    self.known_meshes.remove(id);
                }
            }
        }
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn upload_atlas(&mut self, device: &Device, id: AtlasId, width: u32, height: u32, pixels: &[u8]) -> Result<(), MetalGraphicsError> {
        let pixel_count = (width as usize) * (height as usize);
        if pixel_count == 0 {
            self.known_atlases.insert(id);
            return Ok(());
        }
        let bytes_per_pixel = pixels.len() / pixel_count;
        match bytes_per_pixel {
            1 => {
                let texture = create_texture(device, MTLPixelFormat::R8Unorm, width, height, 1, "glyph_atlas");
                replace_region(&texture, width, height, pixels, width);
                self.glyph_atlas = Some(texture);
            }
            4 => {
                let texture = create_texture(device, MTLPixelFormat::RGBA8Unorm_sRGB, width, height, 1, "icon_atlas");
                replace_region(&texture, width, height, pixels, width * 4);
                self.icon_atlas = Some(texture);
            }
            other => return Err(MetalGraphicsError::UnsupportedAtlasChannels(other as u32)),
        }
        self.known_atlases.insert(id);
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn upload_texture(&mut self, device: &Device, id: TextureId, width: u32, height: u32, pixels: &[u8]) -> Result<(), MetalGraphicsError> {
        if width == 0 || height == 0 {
            self.known_textures.insert(id);
            return Ok(());
        }
        let texture = create_texture(device, MTLPixelFormat::RGBA8Unorm_sRGB, width, height, 1, "raster_texture");
        replace_region(&texture, width, height, pixels, width * 4);
        self.raster_textures.insert(id, texture);
        self.known_textures.insert(id);
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn create_or_update_mesh(&mut self, device: &Device, id: MeshId, positions: &[f32], normals: &[f32], indices: &[u32]) -> Result<(), MetalGraphicsError> {
        let vertex_count = positions.len() / 3;
        let mut vertices = Vec::with_capacity(vertex_count);
        for index in 0..vertex_count {
            let position = [positions[index * 3], positions[index * 3 + 1], positions[index * 3 + 2]];
            let normal = [normals.get(index * 3).copied().unwrap_or(0.0), normals.get(index * 3 + 1).copied().unwrap_or(1.0), normals.get(index * 3 + 2).copied().unwrap_or(0.0)];
            vertices.push(World3dGpuVertex { position, normal });
        }
        let vertex_bytes = bytemuck::cast_slice(&vertices);
        let vertex_buffer = new_buffer_with_bytes(device, vertex_bytes, "world3d_vertices")?;
        let index_bytes = bytemuck::cast_slice(indices);
        let index_buffer = new_buffer_with_bytes(device, index_bytes, "world3d_indices")?;
        self.meshes.insert(id, MeshBuffers { vertex_buffer, index_buffer, index_count: indices.len() as u32 });
        self.known_meshes.insert(id);
        Ok(())
    }
}

/// 🏗️ `usage` is always `ShaderRead` here (every texture this table owns is sampled, never a render
/// target) with `StorageModeManaged` on Intel/`Shared` behaviour unified by Metal's `storageMode`
/// default resolution — explicit `Shared` keeps `replaceRegion` valid on every Mac (unlike
/// `Private`, which requires a blit upload).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn create_texture(device: &Device, format: MTLPixelFormat, width: u32, height: u32, mip_levels: u32, label: &str) -> Owned<MetalTexture> {
    let descriptor = MTLTextureDescriptor::new();
    descriptor.setPixelFormat(format);
    // 🔓️ SAFETY: width/height/mipmapLevelCount are ordinary dimension setters; Metal validates and
    // clamps rather than reading out of bounds, and this crate always passes caller-checked u32s.
    unsafe {
        descriptor.setWidth(width.max(1) as _);
        descriptor.setHeight(height.max(1) as _);
        descriptor.setMipmapLevelCount(mip_levels.max(1) as _);
    }
    descriptor.setUsage(MTLTextureUsage::ShaderRead);
    descriptor.setResourceOptions(MTLResourceOptions::StorageModeShared);
    let texture = device.newTextureWithDescriptor(&descriptor).unwrap_or_else(|| panic!("metal backend: failed to allocate texture {label}"));
    let _ = label;
    texture
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn replace_region(texture: &MetalTexture, width: u32, height: u32, pixels: &[u8], bytes_per_row: u32) {
    if pixels.is_empty() {
        return;
    }
    let region = MTLRegion { origin: objc2_metal::MTLOrigin { x: 0, y: 0, z: 0 }, size: MTLSize { width: width as _, height: height as _, depth: 1 } };
    let Some(pointer) = std::ptr::NonNull::new(pixels.as_ptr() as *mut std::ffi::c_void) else { return };
    // 🔓️ SAFETY: `pointer` is valid for `pixels.len()` bytes for the duration of this call (borrowed
    // from the caller's slice, not retained past it); `bytes_per_row * height` never exceeds
    // `pixels.len()` because every call site derives `bytes_per_row` from the same `width` the pixel
    // buffer was packed at.
    unsafe {
        texture.replaceRegion_mipmapLevel_withBytes_bytesPerRow(region, 0, pointer, bytes_per_row as _);
    }
}

/// 🏗️ A `Shared`-storage buffer initialized by copy — mirrors `wgpu::util::DeviceExt::create_buffer_init`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn new_buffer_with_bytes(device: &Device, bytes: &[u8], label: &str) -> Result<Owned<MetalBuffer>, MetalGraphicsError> {
    if bytes.is_empty() {
        return device.newBufferWithLength_options(16, MTLResourceOptions::StorageModeShared).ok_or(MetalGraphicsError::AllocationFailed(label.to_string()));
    }
    let Some(pointer) = std::ptr::NonNull::new(bytes.as_ptr() as *mut std::ffi::c_void) else {
        return Err(MetalGraphicsError::AllocationFailed(label.to_string()));
    };
    // 🔓️ SAFETY: `pointer` is valid for `bytes.len()` bytes for the duration of this call — Metal
    // copies the contents into the new buffer synchronously and retains no reference to `pointer`
    // afterward.
    let buffer = unsafe { device.newBufferWithBytes_length_options(pointer, bytes.len() as _, MTLResourceOptions::StorageModeShared) };
    buffer.ok_or_else(|| MetalGraphicsError::AllocationFailed(label.to_string()))
}

//#endregion 🔖️Resources

// 🧪️ Device-dependent behaviour for this table (`apply`, atlas routing, eviction) is exercised from
// `🦀️backend.rs`'s test module via `MetalBackend::apply_resources`/`render`, which is the shape a real
// caller uses — this file has no pure-data logic worth testing in isolation from a device.
