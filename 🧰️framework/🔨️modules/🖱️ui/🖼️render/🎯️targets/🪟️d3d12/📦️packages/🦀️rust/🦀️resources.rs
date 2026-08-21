//! @emoji 🗃️ GPU-side residency for `ui_render::resource::ResourceOp` — the D3D12 counterpart of the
//! wgpu target's `RasterTextureTable`/`MeshGpuTable` and the Metal backend's `🦀️resources.rs`, keyed
//! by the typed generational ids (`TextureId`/`MeshId`/`AtlasId`) rather than interned strings, since
//! `ui_render::ResourceRegistry` already did the interning.
//!
//! **Atlas routing** is identical to Metal's: the contract has exactly one `AtlasId` type for both
//! the glyph (alpha, 1 channel) and icon/color (RGBA, 4 channel) atlas pages, so this crate infers the
//! page from the upload's own byte density (`pixels.len() / (width * height)`) — 1 byte/pixel routes
//! to the fixed glyph-atlas descriptor slot (heap index 0), 4 bytes/pixel routes to the fixed icon-
//! atlas slot (heap index 1), mirroring `UI_SHADER`'s two hard-coded atlas bindings exactly.
//!
//! **The resident SRV table.** Every texture this crate owns (glyph atlas, icon atlas, each raster
//! image) gets one *permanent* CPU-visible (non-shader-visible) SRV descriptor in `heap` — index 0 is
//! always the glyph atlas, index 1 is always the icon atlas, and raster textures take indices `2..`
//! from a free list. This heap never binds to a draw call directly (D3D12 draws read from a *shader-
//! visible* heap); instead `🦀️frame_buffers.rs::FrameDescriptors` `CopyDescriptorsSimple`s the pair a
//! given batch needs into a fresh per-frame slot before that batch's draw — see that file's header for
//! why a fresh slot per draw (not an in-place overwrite of a shared slot) is the correctness-critical
//! part of this design. `heap` doubles in `NumDescriptors` (never shrinks) exactly like Metal's/wgpu's
//! `GrowBuffer` growth policy, just applied to a descriptor count instead of a byte count; growing
//! means allocating a new heap and re-issuing `CreateShaderResourceView` for every still-resident slot
//! (cheap CPU-only calls, no GPU synchronization needed).
//!
//! **Texture upload is synchronous.** `apply_resources` runs before `render` in the trait's own
//! ordering invariant, so this crate opens its own tiny command allocator/list, records the staging-
//! buffer→`CopyTextureRegion`→barrier sequence, executes it on the caller's queue, and blocks
//! (`crate::types::wait_for_fence_value`) until that upload is visibly complete before returning —
//! simple and correct, at the cost of the caller's thread blocking once per `apply_resources` call
//! that carries a texture upload. A production-grade backend would pipeline this (a persistent upload
//! ring, uploads submitted alongside — not blocking ahead of — the next `render`), which this crate
//! does not attempt; called out plainly in `📓️terra-backend-d3d12-report.md`'s decisions, the same way
//! the Metal backend calls out its own "panics instead of `Result` propagation on construction
//! failure" simplification.

use crate::types::{create_default_texture2d, create_upload_buffer, transition_barrier, wait_for_fence_value, World3dGpuVertex};
use std::collections::{HashMap, HashSet};
use ui_render::{AtlasId, BackendError, MeshId, ResourceOp, TextureId};
use windows::core::Interface;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_R8_UNORM};

//#region 🔖️Resources

type Device = ID3D12Device;

//#region ⚠️Error

#[derive(Debug)]
pub enum D3d12GraphicsError {
    AllocationFailed(String),
    UnsupportedAtlasChannels(u32),
    ShaderCompilationFailed(String),
}

impl From<D3d12GraphicsError> for BackendError {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(error: D3d12GraphicsError) -> Self {
        match error {
            D3d12GraphicsError::AllocationFailed(_) => BackendError::OutOfMemory,
            D3d12GraphicsError::UnsupportedAtlasChannels(_) => BackendError::UnsupportedFormat("atlas upload byte density must be 1 (R8) or 4 (RGBA8) bytes/pixel"),
            D3d12GraphicsError::ShaderCompilationFailed(message) => BackendError::ShaderCompilationFailed(message),
        }
    }
}

impl From<crate::pipelines::PipelineBuildError> for D3d12GraphicsError {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(error: crate::pipelines::PipelineBuildError) -> Self {
        match error {
            crate::pipelines::PipelineBuildError::ShaderCompilationFailed(message) => Self::ShaderCompilationFailed(message),
            crate::pipelines::PipelineBuildError::DeviceCallFailed(message) => Self::AllocationFailed(message),
        }
    }
}

//#endregion ⚠️Error

//#region 🧊️MeshBuffers

/// 🧊️ One resident world3d mesh: interleaved position+normal vertex buffer, u32 index buffer, both
/// `D3D12_HEAP_TYPE_UPLOAD` resources bound directly via `IASetVertexBuffers`/`IASetIndexBuffer` — no
/// device-local copy step, mirroring Metal's `Shared`-storage `MeshBuffers`.
pub struct MeshBuffers {
    pub vertex_buffer: ID3D12Resource,
    pub vertex_buffer_view: D3D12_VERTEX_BUFFER_VIEW,
    pub index_buffer: ID3D12Resource,
    pub index_buffer_view: D3D12_INDEX_BUFFER_VIEW,
    pub index_count: u32,
}

//#endregion 🧊️MeshBuffers

//#region 🗄️ResidentSrvTable

const INITIAL_CAPACITY: u32 = 8;
/// 🕳️ Fixed heap slots — 0 is always the glyph atlas, 1 is always the icon atlas (seeded with 1x1
/// dummy textures at construction so the UI megashader always has something bound — see
/// `GpuResources::new`'s doc comment). Raster textures start at slot 2.
const GLYPH_SLOT: u32 = 0;
const ICON_SLOT: u32 = 1;
const FIRST_RASTER_SLOT: u32 = 2;

/// 🗄️ The growable CPU-visible SRV table this file's header describes.
struct ResidentSrvTable {
    heap: ID3D12DescriptorHeap,
    capacity: u32,
    stride: u32,
    /// 📋️ Parallel to heap slots: `(resource, format)` for every occupied slot, `None` for a free one
    /// — kept so a heap grow can re-issue every `CreateShaderResourceView` against the new heap.
    slots: Vec<Option<(ID3D12Resource, DXGI_FORMAT)>>,
    free_raster: Vec<u32>,
}

impl ResidentSrvTable {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn new(device: &Device) -> Self {
        let heap = create_heap(device, INITIAL_CAPACITY);
        let stride = unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV) };
        Self { heap, capacity: INITIAL_CAPACITY, stride, slots: (0..INITIAL_CAPACITY).map(|_| None).collect(), free_raster: (FIRST_RASTER_SLOT..INITIAL_CAPACITY).rev().collect() }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn cpu_handle(&self, index: u32) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        let start = unsafe { self.heap.GetCPUDescriptorHandleForHeapStart() };
        D3D12_CPU_DESCRIPTOR_HANDLE { ptr: start.ptr + (index as usize) * (self.stride as usize) }
    }

    /// ✏️ Writes `resource` (a resident 2D texture, `Format` inferred from `format`) into `index`,
    /// growing the heap first if `index` is beyond its current capacity.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn write(&mut self, device: &Device, index: u32, resource: ID3D12Resource, format: DXGI_FORMAT) {
        self.ensure_capacity(device, index + 1);
        let handle = self.cpu_handle(index);
        create_srv(device, &resource, format, handle);
        self.slots[index as usize] = Some((resource, format));
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn clear(&mut self, index: u32) {
        if let Some(slot) = self.slots.get_mut(index as usize) {
            *slot = None;
        }
    }

    /// 🔢️ Allocates a fresh raster slot from the free list, growing first if the list is empty.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn allocate_raster_slot(&mut self, device: &Device) -> u32 {
        if let Some(index) = self.free_raster.pop() {
            return index;
        }
        let old_capacity = self.capacity;
        self.ensure_capacity(device, old_capacity + 1);
        self.free_raster.extend((old_capacity + 1..self.capacity).rev());
        old_capacity
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn free_raster_slot(&mut self, index: u32) {
        self.clear(index);
        self.free_raster.push(index);
    }

    /// 📈️ Doubles `capacity` (never shrinks) until it covers `needed`, rebuilding the heap and
    /// re-describing every still-occupied slot against it — the descriptor-count analog of
    /// `GrowBuffer::upload`'s byte-capacity doubling.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn ensure_capacity(&mut self, device: &Device, needed: u32) {
        if needed <= self.capacity {
            return;
        }
        let mut new_capacity = self.capacity;
        while new_capacity < needed {
            new_capacity *= 2;
        }
        let new_heap = create_heap(device, new_capacity);
        for (index, slot) in self.slots.iter().enumerate() {
            if let Some((resource, format)) = slot {
                let handle_ptr = unsafe { new_heap.GetCPUDescriptorHandleForHeapStart() }.ptr + index * (self.stride as usize);
                create_srv(device, resource, *format, D3D12_CPU_DESCRIPTOR_HANDLE { ptr: handle_ptr });
            }
        }
        self.slots.resize_with(new_capacity as usize, || None);
        self.heap = new_heap;
        self.capacity = new_capacity;
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn create_heap(device: &Device, capacity: u32) -> ID3D12DescriptorHeap {
    let desc = D3D12_DESCRIPTOR_HEAP_DESC { Type: D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV, NumDescriptors: capacity, Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE, NodeMask: 0 };
    // 🔓️ SAFETY: plain descriptor-heap creation from a stack-local desc.
    unsafe { device.CreateDescriptorHeap(&desc) }.unwrap_or_else(|error| panic!("d3d12 backend: failed to allocate resident SRV heap (capacity {capacity}): {error:?}"))
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn create_srv(device: &Device, resource: &ID3D12Resource, format: DXGI_FORMAT, handle: D3D12_CPU_DESCRIPTOR_HANDLE) {
    let mut desc = D3D12_SHADER_RESOURCE_VIEW_DESC::default();
    desc.Format = format;
    desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE2D;
    desc.Shader4ComponentMapping = D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING;
    desc.Anonymous.Texture2D = D3D12_TEX2D_SRV { MostDetailedMip: 0, MipLevels: u32::MAX, PlaneSlice: 0, ResourceMinLODClamp: 0.0 };
    // 🔓️ SAFETY: `resource` is a live texture this table already owns; `handle` is a CPU descriptor
    // handle this table's own heap allocated (checked by every call site via `cpu_handle`/growth).
    unsafe { device.CreateShaderResourceView(resource, Some(&desc), handle) };
}

//#endregion 🗄️ResidentSrvTable

//#region 🗃️GpuResources

/// 🗃️ Owns every device-resident resource a `RenderPacket` can reference. `apply` is the only
/// mutator; everything else is a lookup a render pass consults while replaying batches.
pub struct GpuResources {
    table: ResidentSrvTable,
    raster_slot: HashMap<TextureId, u32>,
    meshes: HashMap<MeshId, MeshBuffers>,
    known_textures: HashSet<TextureId>,
    known_meshes: HashSet<MeshId>,
    known_atlases: HashSet<AtlasId>,
    upload_allocator: ID3D12CommandAllocator,
    upload_list: ID3D12GraphicsCommandList,
    upload_fence: ID3D12Fence,
    upload_fence_value: u64,
    /// ⚠️ Staging buffers `record_texture_upload` creates, kept alive until `execute_and_wait`
    /// confirms the GPU has finished the copy that reads them. **Load-bearing, not bookkeeping**: a
    /// D3D12 command list only *records* work — a resource referenced by a not-yet-executed
    /// `CopyTextureRegion` must stay alive until the GPU actually runs it (D3D12 does not do this for
    /// you the way some higher-level APIs do), so a staging buffer dropped immediately after
    /// `record_texture_upload` returns (before `apply`'s `Close`/`ExecuteCommandLists`/fence-wait ever
    /// run) would `Release` the underlying COM object — and free the memory — while a GPU command
    /// still names it. Cleared only after `execute_and_wait` confirms completion.
    pending_staging: Vec<ID3D12Resource>,
}

impl GpuResources {
    /// 🕳️ Seeds the glyph/icon atlas slots with 1x1 dummy textures so the UI megashader always has
    /// something bound at `t0`/`t1` even on a frame painted before any glyph/icon has ever been
    /// requested — mirrors the Metal backend's identical seeding and its identical reasoning
    /// (`NullBackend` has no such gap because it never touches a device at all; a real backend must
    /// bind *something*). Overwritten by the first real `ResourceOp::UploadAtlas` of each byte
    /// density.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(device: &Device) -> Self {
        let mut table = ResidentSrvTable::new(device);
        let glyph_dummy = create_default_texture2d(device, DXGI_FORMAT_R8_UNORM, 1, 1, 1, D3D12_RESOURCE_FLAGS(0), D3D12_RESOURCE_STATE_COPY_DEST, None, "glyph_atlas_dummy");
        let icon_dummy = create_default_texture2d(device, DXGI_FORMAT_R8G8B8A8_UNORM, 1, 1, 1, D3D12_RESOURCE_FLAGS(0), D3D12_RESOURCE_STATE_COPY_DEST, None, "icon_atlas_dummy");

        let upload_allocator: ID3D12CommandAllocator = unsafe { device.CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT) }.expect("d3d12 backend: failed to allocate the upload command allocator");
        let upload_list: ID3D12GraphicsCommandList = unsafe { device.CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, &upload_allocator, None) }.expect("d3d12 backend: failed to allocate the upload command list");
        let upload_fence: ID3D12Fence = unsafe { device.CreateFence(0, D3D12_FENCE_FLAG_NONE) }.expect("d3d12 backend: failed to allocate the upload fence");

        // 🔓️ SAFETY: `glyph_dummy`/`icon_dummy` were just created in `COPY_DEST` above; transitioning
        // them straight to `PIXEL_SHADER_RESOURCE` with no actual copy is legal (their contents are
        // whatever the driver zero-initializes a fresh committed resource to — acceptable for a 1x1
        // placeholder never meant to be visually meaningful) and matches this table's steady-state
        // invariant that every occupied slot's resource is always sampleable.
        unsafe {
            upload_list.ResourceBarrier(&[
                transition_barrier(&glyph_dummy, 0, D3D12_RESOURCE_STATE_COPY_DEST, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE),
                transition_barrier(&icon_dummy, 0, D3D12_RESOURCE_STATE_COPY_DEST, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE),
            ]);
        }
        table.write(device, GLYPH_SLOT, glyph_dummy, DXGI_FORMAT_R8_UNORM);
        table.write(device, ICON_SLOT, icon_dummy, DXGI_FORMAT_R8G8B8A8_UNORM);

        Self {
            table,
            raster_slot: HashMap::new(),
            meshes: HashMap::new(),
            known_textures: HashSet::new(),
            known_meshes: HashSet::new(),
            known_atlases: HashSet::new(),
            upload_allocator,
            upload_list,
            upload_fence,
            upload_fence_value: 0,
            pending_staging: Vec::new(),
        }
    }

    /// 🔚️ Closes and (via `apply`'s own queue-execute-wait cycle) never-executed construction-time
    /// barriers must still be flushed once before the list is reused — called once, immediately after
    /// `new`, by `D3d12Backend::from_parts` so every later `apply` call can assume "list is open and
    /// ready to record" on entry.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn flush_construction(&mut self, queue: &ID3D12CommandQueue) {
        unsafe { self.upload_list.Close() }.expect("d3d12 backend: failed to close the upload command list");
        self.execute_and_wait(queue);
        self.reopen_upload_list();
    }

    /// ⏱️ Executes and blocks until the GPU confirms completion, then drops every staging buffer
    /// `record_texture_upload` accumulated into `pending_staging` — only now is it sound to release
    /// them (see that field's doc comment).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn execute_and_wait(&mut self, queue: &ID3D12CommandQueue) {
        let list: ID3D12CommandList = self.upload_list.cast().expect("ID3D12GraphicsCommandList always casts to its ID3D12CommandList base");
        let lists = [Some(list)];
        unsafe { queue.ExecuteCommandLists(&lists) };
        self.upload_fence_value += 1;
        unsafe { queue.Signal(&self.upload_fence, self.upload_fence_value) }.expect("d3d12 backend: failed to signal the upload fence");
        wait_for_fence_value(&self.upload_fence, self.upload_fence_value);
        self.pending_staging.clear();
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn reopen_upload_list(&mut self) {
        unsafe { self.upload_allocator.Reset() }.expect("d3d12 backend: failed to reset the upload command allocator");
        unsafe { self.upload_list.Reset(&self.upload_allocator, None) }.expect("d3d12 backend: failed to reset the upload command list");
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn glyph_atlas_handle(&self) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        self.table.cpu_handle(GLYPH_SLOT)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn icon_atlas_handle(&self) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        self.table.cpu_handle(ICON_SLOT)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn raster_texture_handle(&self, id: TextureId) -> Option<D3D12_CPU_DESCRIPTOR_HANDLE> {
        self.raster_slot.get(&id).map(|&index| self.table.cpu_handle(index))
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
    /// reporting after a device-removed transition — mirrors `NullBackend`/the Metal backend's
    /// identical method.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn drain_known(&mut self) -> (Vec<TextureId>, Vec<MeshId>, Vec<AtlasId>) {
        self.meshes.clear();
        self.raster_slot.clear();
        (self.known_textures.drain().collect(), self.known_meshes.drain().collect(), self.known_atlases.drain().collect())
    }

    /// 📤️ Applies one `ResourceOp` stream, always *before* the `render` call whose packet references
    /// the ids it uploads (the trait's own invariant). `queue` is the caller's command queue — this
    /// table owns no queue of its own, only the allocator/list/fence it schedules work through.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn apply(&mut self, device: &Device, queue: &ID3D12CommandQueue, ops: &[ResourceOp]) -> Result<(), D3d12GraphicsError> {
        let mut recorded_any = false;
        for op in ops {
            match op {
                ResourceOp::UploadAtlas { id, width, height, pixels } => {
                    self.upload_atlas(device, *id, *width, *height, pixels)?;
                    recorded_any = recorded_any || !pixels.is_empty();
                }
                ResourceOp::UploadTexture { id, width, height, pixels } => {
                    self.upload_texture(device, *id, *width, *height, pixels)?;
                    recorded_any = recorded_any || !pixels.is_empty();
                }
                ResourceOp::CreateOrUpdateMesh { id, positions, normals, indices } => {
                    self.create_or_update_mesh(device, *id, positions, normals, indices)?;
                }
                ResourceOp::EvictTexture(id) => {
                    if let Some(index) = self.raster_slot.remove(id) {
                        self.table.free_raster_slot(index);
                    }
                    self.known_textures.remove(id);
                }
                ResourceOp::EvictMesh(id) => {
                    self.meshes.remove(id);
                    self.known_meshes.remove(id);
                }
            }
        }
        if recorded_any {
            unsafe { self.upload_list.Close() }.map_err(|error| D3d12GraphicsError::AllocationFailed(format!("close upload list: {error:?}")))?;
            self.execute_and_wait(queue);
            self.reopen_upload_list();
        }
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn upload_atlas(&mut self, device: &Device, id: AtlasId, width: u32, height: u32, pixels: &[u8]) -> Result<(), D3d12GraphicsError> {
        let pixel_count = (width as usize) * (height as usize);
        if pixel_count == 0 {
            self.known_atlases.insert(id);
            return Ok(());
        }
        let bytes_per_pixel = pixels.len() / pixel_count;
        match bytes_per_pixel {
            1 => {
                let texture = self.record_texture_upload(device, DXGI_FORMAT_R8_UNORM, width, height, pixels, 1, "glyph_atlas");
                self.table.write(device, GLYPH_SLOT, texture, DXGI_FORMAT_R8_UNORM);
            }
            4 => {
                let texture = self.record_texture_upload(device, DXGI_FORMAT_R8G8B8A8_UNORM, width, height, pixels, 4, "icon_atlas");
                self.table.write(device, ICON_SLOT, texture, DXGI_FORMAT_R8G8B8A8_UNORM);
            }
            other => return Err(D3d12GraphicsError::UnsupportedAtlasChannels(other as u32)),
        }
        self.known_atlases.insert(id);
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn upload_texture(&mut self, device: &Device, id: TextureId, width: u32, height: u32, pixels: &[u8]) -> Result<(), D3d12GraphicsError> {
        if width == 0 || height == 0 {
            self.known_textures.insert(id);
            return Ok(());
        }
        let texture = self.record_texture_upload(device, DXGI_FORMAT_R8G8B8A8_UNORM, width, height, pixels, 4, "raster_texture");
        let slot = self.table.allocate_raster_slot(device);
        self.table.write(device, slot, texture, DXGI_FORMAT_R8G8B8A8_UNORM);
        self.raster_slot.insert(id, slot);
        self.known_textures.insert(id);
        Ok(())
    }

    /// 🏗️ Creates a `DEFAULT`-heap texture in `COPY_DEST`, records (into `self.upload_list`, which the
    /// caller flushes once per `apply` call) a staging-buffer→`CopyTextureRegion`→barrier sequence
    /// transitioning it to `PIXEL_SHADER_RESOURCE`, and returns it. `pixels` empty means "allocate but
    /// leave undefined" (mirrors the zero-size texture short-circuit in `upload_texture`/`upload_atlas`
    /// — never reached with non-empty `width`/`height` and empty `pixels` from this crate's own call
    /// sites, but handled defensively).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn record_texture_upload(&mut self, device: &Device, format: DXGI_FORMAT, width: u32, height: u32, pixels: &[u8], bytes_per_pixel: u32, label: &str) -> ID3D12Resource {
        let texture = create_default_texture2d(device, format, width, height, 1, D3D12_RESOURCE_FLAGS(0), D3D12_RESOURCE_STATE_COPY_DEST, None, label);
        if !pixels.is_empty() {
            let row_pitch = align_up(width * bytes_per_pixel, D3D12_TEXTURE_DATA_PITCH_ALIGNMENT);
            let mut staging_bytes = vec![0u8; (row_pitch as usize) * (height as usize)];
            let src_row_bytes = (width * bytes_per_pixel) as usize;
            for row in 0..height as usize {
                let src_start = row * src_row_bytes;
                let dst_start = row * row_pitch as usize;
                staging_bytes[dst_start..dst_start + src_row_bytes].copy_from_slice(&pixels[src_start..src_start + src_row_bytes]);
            }
            let staging = create_upload_buffer(device, &staging_bytes, label);
            let src_location = D3D12_TEXTURE_COPY_LOCATION {
                pResource: std::mem::ManuallyDrop::new(Some(unsafe { std::mem::transmute_copy(&staging) })),
                Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 { PlacedFootprint: D3D12_PLACED_SUBRESOURCE_FOOTPRINT { Offset: 0, Footprint: D3D12_SUBRESOURCE_FOOTPRINT { Format: format, Width: width, Height: height, Depth: 1, RowPitch: row_pitch } } },
            };
            let dst_location = D3D12_TEXTURE_COPY_LOCATION {
                pResource: std::mem::ManuallyDrop::new(Some(unsafe { std::mem::transmute_copy(&texture) })),
                Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 { SubresourceIndex: 0 },
            };
            // 🔓️ SAFETY: `src_location`/`dst_location` borrow `staging`/`texture` via the same
            // transmute-copy-without-`AddRef` technique `crate::types::transition_barrier` documents —
            // sound because both *locations* are consumed synchronously by `CopyTextureRegion` and then
            // dropped as plain stack values (never `ManuallyDrop::into_inner`'d), so their `Drop` never
            // runs and never mismatches the missing `AddRef`. This says nothing about `staging`/
            // `texture` *themselves* staying alive long enough for the GPU to actually execute the
            // command that references them — that is a separate, load-bearing requirement `self.
            // pending_staging.push(staging)` below satisfies (see that field's doc comment); `texture`
            // is kept alive by its caller (`upload_atlas`/`upload_texture` hand it to `ResidentSrvTable::
            // write`, which stores it). `staging`'s row-padded bytes match `Footprint` exactly by
            // construction above (`row_pitch`/`width`/`height` all derive from the same values).
            unsafe {
                self.upload_list.CopyTextureRegion(&dst_location, 0, 0, 0, &src_location, None);
                self.upload_list.ResourceBarrier(&[transition_barrier(&texture, 0, D3D12_RESOURCE_STATE_COPY_DEST, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE)]);
            }
            self.pending_staging.push(staging);
        } else {
            unsafe { self.upload_list.ResourceBarrier(&[transition_barrier(&texture, 0, D3D12_RESOURCE_STATE_COPY_DEST, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE)]) };
        }
        texture
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn create_or_update_mesh(&mut self, device: &Device, id: MeshId, positions: &[f32], normals: &[f32], indices: &[u32]) -> Result<(), D3d12GraphicsError> {
        let vertex_count = positions.len() / 3;
        let mut vertices = Vec::with_capacity(vertex_count);
        for index in 0..vertex_count {
            let position = [positions[index * 3], positions[index * 3 + 1], positions[index * 3 + 2]];
            let normal = [normals.get(index * 3).copied().unwrap_or(0.0), normals.get(index * 3 + 1).copied().unwrap_or(1.0), normals.get(index * 3 + 2).copied().unwrap_or(0.0)];
            vertices.push(World3dGpuVertex { position, normal });
        }
        let vertex_bytes: &[u8] = bytemuck::cast_slice(&vertices);
        let vertex_buffer = create_upload_buffer(device, vertex_bytes, "world3d_vertices");
        let vertex_buffer_view = D3D12_VERTEX_BUFFER_VIEW { BufferLocation: unsafe { vertex_buffer.GetGPUVirtualAddress() }, SizeInBytes: vertex_bytes.len() as u32, StrideInBytes: std::mem::size_of::<World3dGpuVertex>() as u32 };
        let index_bytes: &[u8] = bytemuck::cast_slice(indices);
        let index_buffer = create_upload_buffer(device, index_bytes, "world3d_indices");
        let index_buffer_view = D3D12_INDEX_BUFFER_VIEW { BufferLocation: unsafe { index_buffer.GetGPUVirtualAddress() }, SizeInBytes: index_bytes.len() as u32, Format: windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32_UINT };
        self.meshes.insert(id, MeshBuffers { vertex_buffer, vertex_buffer_view, index_buffer, index_buffer_view, index_count: indices.len() as u32 });
        self.known_meshes.insert(id);
        Ok(())
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn align_up(value: u32, alignment: u32) -> u32 {
    (value + alignment - 1) / alignment * alignment
}

//#endregion 🗃️GpuResources

//#endregion 🔖️Resources

// 🧪️ Device-dependent behaviour for this table (`apply`, atlas routing, eviction, texture upload) is
// exercised from `🦀️backend.rs`'s test module via `D3d12Backend::apply_resources`/`render`, which is
// the shape a real caller uses — this file has no pure-data logic worth testing in isolation from a
// device, mirroring the Metal backend's identical `🦀️resources.rs` footer note.
