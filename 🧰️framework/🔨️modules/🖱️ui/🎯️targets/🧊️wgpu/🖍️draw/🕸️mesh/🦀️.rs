//! 🕸️ Actual mesh device resources retain the canonical original mesh loan and source return.
use semio_framework_ui_wgpu_device as gpu_device;
use semio_framework_ui_draw_contract::*;
use bytemuck::{Pod,Zeroable};
use std::mem::size_of;
use gpu_device::util::DeviceExt;
mod kernel_3d_scene{pub use semio_framework_ui_scene_algebra::*;pub use semio_framework_ui_scene_camera::*;pub use semio_framework_ui_scene_mesh::*;pub use semio_framework_ui_scene_pass::*;}
use kernel_3d_scene::{Mat4Math,ProceduralGrid3d,ScenePass3d,SceneRenderProfile3d,PROCEDURAL_GRID_CELL_THICKNESS,PROCEDURAL_GRID_FADE_STRENGTH};
use semio_framework_ui_prepared as prepared;
use semio_framework_ui_raster_owner as raster_ownership;
use semio_framework_ui_theme as theme;
use semio_framework_ui_theme::{geometry,Theme};
#[cfg(test)]
use semio_framework_ui_theme::Rgba;
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UiGlobals {
    pub screen_size: [f32; 2],
    pub _pad: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct World3dVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 4],
    pub uv: [f32; 2],
    pub surface_uvs:[[f32;2];4],
    pub tangent:[f32;4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct World3dGlobals {
    pub view_proj: [f32; 16],
    pub shadow_view_proj: [f32; 16],
    pub camera_position: [f32; 4],
    pub light_dir: [f32; 4],
    pub ambient: [f32; 4],
    pub sun: [f32; 4],
    pub material: [f32; 4],
    pub material_emissive: [f32; 4],
    pub shadow: [f32; 4],
    /// ✂️ The section plane (`n · p + constant < 0` is discarded) for instances flagged `SECTION_CLIP_FLAG`; all zeros clips nothing.
    pub clip_plane: [f32; 4],
}

impl World3dGlobals {
    pub fn from_pass(pass: &ScenePass3d) -> Self {
        Self {
            view_proj: pass.view_proj,
            shadow_view_proj: pass.shadow.view_proj,
            camera_position: [pass.camera_position[0], pass.camera_position[1], pass.camera_position[2], 0.0],
            light_dir: [pass.light_dir[0], pass.light_dir[1], pass.light_dir[2], 0.0],
            ambient: [pass.lighting.ambient_color[0], pass.lighting.ambient_color[1], pass.lighting.ambient_color[2], pass.lighting.ambient_intensity],
            sun: [pass.lighting.sun_color[0], pass.lighting.sun_color[1], pass.lighting.sun_color[2], pass.lighting.sun_intensity],
            material: [pass.neutral_material.metalness, pass.neutral_material.roughness, pass.neutral_material.emissive_intensity, if pass.lighting.sun_enabled { 1.0 } else { 0.0 }],
            material_emissive: [pass.neutral_material.emissive[0], pass.neutral_material.emissive[1], pass.neutral_material.emissive[2], 0.0],
            shadow: [
                if pass.shadow.enabled { 1.0 } else { 0.0 },
                if pass.render_profile == SceneRenderProfile3d::SvgFlatLit { 1.0 } else { 0.0 },
                0.0,
                1.0,
            ],
            clip_plane: pass.section_clip.unwrap_or([0.0; 4]),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct World3dGridUniforms {
    pub plane_cell: [f32; 4],
    pub camera_fade: [f32; 4],
    pub cell_color: [f32; 4],
    pub _pad: [[f32; 4]; 13],
}

impl World3dGridUniforms {
    pub fn from_grid(grid: &ProceduralGrid3d) -> Self {
        Self {
            plane_cell: [grid.plane_z, grid.cell_size, PROCEDURAL_GRID_CELL_THICKNESS, grid.fade_distance],
            camera_fade: [grid.camera_plane_projection[0], grid.camera_plane_projection[1], grid.camera_plane_projection[2], PROCEDURAL_GRID_FADE_STRENGTH],
            cell_color: [grid.cell_color[0], grid.cell_color[1], grid.cell_color[2], 0.0],
            _pad: [[0.0; 4]; 13],
        }
    }
}

/// 🔢️ The one stencil bit the section cap borrows (the high bit; the UI clip masks use `0` and `1`): a cut solid's parity toggles it, the cap plane draws where it is set, and a second toggle pass restores it to zero.
pub const SECTION_STENCIL_BIT: u32 = 0x80;

/// 🚩️ The policy bit of `World3dGpuInstance::flags.x` that makes the lit mesh shader test the fragment against `World3dGlobals::clip_plane`.
pub const SECTION_CLIP_FLAG: f32 = 32.0;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct World3dGpuInstance {
    pub model0: [f32; 4],
    pub model1: [f32; 4],
    pub model2: [f32; 4],
    pub model3: [f32; 4],
    pub color: [f32; 4],
    pub flags: [f32; 4],
    pub emissive_cutoff: [f32; 4],
    pub surface_parameters:[f32;4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct World3dCelebrationGpuInstance {
    pub model0: [f32; 4],
    pub model1: [f32; 4],
    pub model2: [f32; 4],
    pub model3: [f32; 4],
    pub color_a: [f32; 4],
    pub color_b: [f32; 4],
    pub color_c: [f32; 4],
    pub params: [f32; 4],
}

impl World3dCelebrationGpuInstance {
    pub fn from_instance(model: [f32; 16], stops: [[f32; 4]; 3], angle: f32, opacity: f32) -> Self {
        Self {
            model0: [model[0], model[1], model[2], model[3]],
            model1: [model[4], model[5], model[6], model[7]],
            model2: [model[8], model[9], model[10], model[11]],
            model3: [model[12], model[13], model[14], model[15]],
            color_a: stops[0],
            color_b: stops[1],
            color_c: stops[2],
            params: [angle, opacity, 0.0, 0.0],
        }
    }
}

impl World3dGpuInstance {
    pub fn from_instance(model: [f32; 16], color: [f32; 4], preserve_vertex_color: bool, emissive_intensity: f32, metalness: f32, roughness: f32, receives_shadow: bool) -> Self {
        let policy = (if preserve_vertex_color { 1 } else { 0 }) | (if receives_shadow { 2 } else { 0 });
        Self {
            model0: [model[0], model[1], model[2], model[3]],
            model1: [model[4], model[5], model[6], model[7]],
            model2: [model[8], model[9], model[10], model[11]],
            model3: [model[12], model[13], model[14], model[15]],
            color,
            flags: [policy as f32, emissive_intensity, metalness, roughness],
            emissive_cutoff: [0.0, 0.0, 0.0, -1.0],surface_parameters:[1.0,1.0,1.0,0.0],
        }
    }

    /// 📑️ Flags the instance for the pass's section plane when its section role clips (policy bit [`SECTION_CLIP_FLAG`]).
    pub fn with_section(mut self, role: crate::kernel_3d_scene::SceneSectionRole3d) -> Self {
        if role.clips() {
            self.flags[0] += SECTION_CLIP_FLAG;
        }
        self
    }

    pub fn from_authored(model: [f32; 16], material: &crate::kernel_3d_scene::SceneAuthoredMaterial3d, receives_shadow: bool) -> Self {
        let policy = (if material.preserve_vertex_color { 1 } else { 0 }) | (if receives_shadow { 2 } else { 0 }) | 4 | (if material.alpha == crate::kernel_3d_scene::SceneMaterialAlpha3d::Opaque { 8 } else { 0 }) | (if material.normal_texture.is_some() { 16 } else { 0 });
        Self {
            model0: [model[0], model[1], model[2], model[3]],
            model1: [model[4], model[5], model[6], model[7]],
            model2: [model[8], model[9], model[10], model[11]],
            model3: [model[12], model[13], model[14], model[15]],
            color: material.base_color,
            flags: [policy as f32, 0.0, material.metalness, material.roughness],
            emissive_cutoff: [material.emissive[0], material.emissive[1], material.emissive[2], if material.alpha == crate::kernel_3d_scene::SceneMaterialAlpha3d::Mask { material.alpha_cutoff } else { -1.0 }],
            surface_parameters:[material.normal_scale[0],material.normal_scale[1],material.occlusion_strength,0.0],
        }
    }
}

/// 🖼️ One world-space textured quad: a centred unit XY plane posed by `model` and painted, the wgpu
/// twin of React's `<mesh><planeGeometry args={[w, h]} /><meshBasicMaterial map=… transparent /></mesh>`
/// in `WorldReferencePlaneItem` (`🎨️r3f/🟦️.tsx`). 96 bytes, the stride
/// `semio_framework_ui_render::shader_contract::WORLD3D_TEXTURED_PIPELINE` declares.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct World3dTexturedGpuInstance {
    pub model0: [f32; 4],
    pub model1: [f32; 4],
    pub model2: [f32; 4],
    pub model3: [f32; 4],
    pub background: [f32; 4],
    pub appearance: [f32; 4],
}

impl World3dTexturedGpuInstance {
    pub fn from_instance(model: [f32; 16], background: [f32; 4], appearance: [f32; 4]) -> Self {
        Self { model0: [model[0], model[1], model[2], model[3]], model1: [model[4], model[5], model[6], model[7]], model2: [model[8], model[9], model[10], model[11]], model3: [model[12], model[13], model[14], model[15]], background, appearance }
    }
}

/// 📐️ The one geometry every textured world draw uses: a unit XY quad centred on the model origin,
/// with `v = 0` at `+y` so row 0 of the decoded image is its TOP — the orientation three.js gives a
/// `PlaneGeometry` with a `flipY` texture, which is what React's reference underlay shows.
pub const WORLD_PLANE_VERTICES: &[f32] = &[
    -0.5, -0.5, 0.0, 0.0, 1.0, //
    0.5, -0.5, 0.0, 1.0, 1.0, //
    0.5, 0.5, 0.0, 1.0, 0.0, //
    -0.5, -0.5, 0.0, 0.0, 1.0, //
    0.5, 0.5, 0.0, 1.0, 0.0, //
    -0.5, 0.5, 0.0, 0.0, 0.0,
];

pub struct GpuMeshBuffers {
    pub vertex_buffer: Option<gpu_device::Buffer>,
    pub index_buffer: Option<gpu_device::Buffer>,
    pub index_count: u32,
    source: crate::kernel_3d_scene::Mesh3dLease,
}

impl GpuMeshBuffers{pub fn original_source_lease(&self)->crate::kernel_3d_scene::Mesh3dLease{self.source}}

pub const MESH_GPU_TABLE_CAPACITY: usize = 256;
const MESH_GPU_KEY_BYTES: usize = 256;
pub const MESH_GPU_KEEP_VERSION_CAPACITY: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MeshGpuKey {
    bytes: [u8; MESH_GPU_KEY_BYTES],
    len: u16,
}

impl MeshGpuKey {
    fn new(key: &str) -> Result<Self, &'static str> {
        if key.is_empty() || key.len() > MESH_GPU_KEY_BYTES {
            return Err("mesh GPU key exceeded fixed credits");
        }
        let mut bytes = [0; MESH_GPU_KEY_BYTES];
        bytes[..key.len()].copy_from_slice(key.as_bytes());
        Ok(Self { bytes, len: key.len() as u16 })
    }

    fn matches(self, key: &str) -> bool {
        usize::from(self.len) == key.len() && &self.bytes[..usize::from(self.len)] == key.as_bytes()
    }
}

struct MeshGpuEntry<T> {
    key: MeshGpuKey,
    version: u64,
    value: T,
}

struct FixedMeshGpuRegistry<T> {
    slots: [Option<MeshGpuEntry<T>>; MESH_GPU_TABLE_CAPACITY],
    len: usize,
}

impl<T> Default for FixedMeshGpuRegistry<T> {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), len: 0 }
    }
}

impl<T> FixedMeshGpuRegistry<T> {
    fn get(&self, key: &str, version: u64) -> Option<&T> {
        self.slots.iter().flatten().find(|entry| entry.version == version && entry.key.matches(key)).map(|entry| &entry.value)
    }

    #[expect(clippy::result_large_err, reason = "GPU admission returns the exact resource and reservation owner without allocating during refusal.")]
    fn insert(&mut self, entry: MeshGpuEntry<T>) -> Result<(), MeshGpuEntry<T>> {
        let Some(slot) = self.slots.iter_mut().find(|slot| slot.is_none()) else { return Err(entry) };
        *slot = Some(entry);
        self.len += 1;
        Ok(())
    }

    fn take(&mut self, index: usize) -> Option<MeshGpuEntry<T>> {
        let value = self.slots.get_mut(index)?.take();
        if value.is_some() {
            self.len = self.len.saturating_sub(1);
        }
        value
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == MESH_GPU_TABLE_CAPACITY
    }
}

#[derive(Default)]
pub struct MeshGpuTable {
    meshes: FixedMeshGpuRegistry<GpuMeshBuffers>,
    upload: Option<MeshGpuUploadCursor>,
    retirement: Option<MeshGpuRetirementCursor>,
    closing: bool,
}

/// 🧾️ Only the actual upload/cache owner can issue the exact native source handback.
pub struct MeshGpuSourceReturn{key:MeshGpuKey,identity:crate::kernel_3d_scene::Mesh3dResourceIdentity,source:crate::kernel_3d_scene::Mesh3dLease}
impl MeshGpuSourceReturn{pub fn validates(&self,key:&str,source:&crate::kernel_3d_scene::Mesh3dResourceLoan)->Result<bool,&'static str>{Ok(self.key==MeshGpuKey::new(key)?&&self.identity==source.identity()&&source.lease()==Some(self.source))}}

struct MeshGpuUploadCursor {
    key: MeshGpuKey,
    version: u64,
    lease: crate::kernel_3d_scene::Mesh3dLease,
    schema: crate::kernel_3d_scene::Mesh3dSchema,
    vertex_buffer: Option<gpu_device::Buffer>,
    index_buffer: Option<gpu_device::Buffer>,
    vertex: u32,
    index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "The fixed version roster is copied into the existing retirement cursor without an uncredited heap allocation.")]
enum MeshGpuRetirementSelector {
    Exact { key: MeshGpuKey, version: u64 },
    KeyExcept { key: MeshGpuKey, versions: [u64; MESH_GPU_KEEP_VERSION_CAPACITY], len: u16 },
    All,
}

impl MeshGpuRetirementSelector {
    fn selects<T>(self, entry: &MeshGpuEntry<T>) -> bool {
        match self {
            Self::Exact { key, version } => entry.key == key && entry.version == version,
            Self::KeyExcept { key, versions, len } => entry.key == key && !versions[..usize::from(len)].contains(&entry.version),
            Self::All => true,
        }
    }
}

struct MeshGpuRetirementCursor {
    selector: MeshGpuRetirementSelector,
    scan: usize,
    owner: Option<MeshGpuRetirementOwner>,
}

struct MeshGpuRetirementOwner {
    vertex_buffer: Option<gpu_device::Buffer>,
    index_buffer: Option<gpu_device::Buffer>,
}

impl MeshGpuTable {
    pub fn get(&self, key: &str) -> Option<&GpuMeshBuffers> {
        let (key, version) = key.rsplit_once(':')?;
        self.get_versioned(key, version.parse().ok()?)
    }

    pub fn get_versioned(&self, mesh_key: &str, version: u64) -> Option<&GpuMeshBuffers> {
        self.meshes.get(mesh_key, version)
    }

    pub fn ensure_mesh_step(&mut self, device: &gpu_device::Device, queue: &gpu_device::Queue, key: &str, version: u64, lease: crate::kernel_3d_scene::Mesh3dLease) -> Result<bool, &'static str> {
        if self.closing {
            return Err("mesh GPU table is closing");
        }
        if let Some(cached)=self.meshes.get(key, version) {
            return if cached.source==lease{Ok(true)}else{Err("mesh GPU cache differs from its original source")};
        }
        if self.upload.is_none() {
            if self.meshes.is_full() {
                return Err("mesh GPU table capacity exhausted");
            }
            let key = MeshGpuKey::new(key)?;
            let schema = lease.schema().map_err(|_| "mesh upload lease was stale")?;
            let vertex_bytes = u64::from(schema.vertices).checked_mul(size_of::<World3dVertex>() as u64).ok_or("mesh upload vertex byte credits overflowed")?;
            let index_bytes = u64::from(schema.indices).checked_mul(size_of::<u32>() as u64).ok_or("mesh upload index byte credits overflowed")?;
            let vertex_buffer = (vertex_bytes != 0).then(|| device.create_buffer(&gpu_device::BufferDescriptor { label: Some("world3d_vertices"), size: vertex_bytes, usage: gpu_device::BufferUsages::VERTEX | gpu_device::BufferUsages::COPY_DST, mapped_at_creation: false }));
            let index_buffer = (index_bytes != 0).then(|| device.create_buffer(&gpu_device::BufferDescriptor { label: Some("world3d_indices"), size: index_bytes, usage: gpu_device::BufferUsages::INDEX | gpu_device::BufferUsages::COPY_DST, mapped_at_creation: false }));
            self.upload = Some(MeshGpuUploadCursor { key, version, lease, schema, vertex_buffer, index_buffer, vertex: 0, index: 0 });
        }
        let cursor = self.upload.as_mut().expect("mesh upload cursor initialized above");
        if !cursor.key.matches(key) || cursor.version != version || cursor.lease != lease {
            return Err("mesh upload authority is occupied by another generation");
        }
        if cursor.vertex < cursor.schema.vertices {
            let position = cursor.lease.vec3(crate::kernel_3d_scene::Mesh3dField::Positions, cursor.vertex).map_err(|_| "mesh upload position lease was stale")?;
            let normal = cursor.lease.vec3(crate::kernel_3d_scene::Mesh3dField::Normals, cursor.vertex).unwrap_or([0.0, 1.0, 0.0]);
            let color = cursor.lease.vec4(crate::kernel_3d_scene::Mesh3dField::Colors, cursor.vertex).unwrap_or([1.0, 1.0, 1.0, 1.0]);
            let uv = cursor.lease.vec2(crate::kernel_3d_scene::Mesh3dField::Uvs, cursor.vertex).unwrap_or([0.0; 2]);
            let surface_uvs=[crate::kernel_3d_scene::Mesh3dField::UvsMetallicRoughness,crate::kernel_3d_scene::Mesh3dField::UvsNormal,crate::kernel_3d_scene::Mesh3dField::UvsOcclusion,crate::kernel_3d_scene::Mesh3dField::UvsEmissive].map(|field|cursor.lease.vec2(field,cursor.vertex).unwrap_or(uv));
            let tangent=cursor.lease.vec4(crate::kernel_3d_scene::Mesh3dField::Tangents,cursor.vertex).unwrap_or([0.0;4]);
            let vertex = World3dVertex { position, normal, color, uv,surface_uvs,tangent };
            queue.write_buffer(cursor.vertex_buffer.as_ref().ok_or("mesh upload vertex buffer was retired")?, u64::from(cursor.vertex) * size_of::<World3dVertex>() as u64, bytemuck::bytes_of(&vertex));
            cursor.vertex += 1;
            return Ok(false);
        }
        if cursor.index < cursor.schema.indices {
            let value = cursor.lease.u32(crate::kernel_3d_scene::Mesh3dField::Indices, cursor.index).map_err(|_| "mesh upload index lease was stale")?;
            queue.write_buffer(cursor.index_buffer.as_ref().ok_or("mesh upload index buffer was retired")?, u64::from(cursor.index) * size_of::<u32>() as u64, &value.to_le_bytes());
            cursor.index += 1;
            return Ok(false);
        }
        let mut cursor = self.upload.take().expect("completed mesh upload cursor");
        let entry = MeshGpuEntry {
            key: cursor.key,
            version: cursor.version,
            value: GpuMeshBuffers { vertex_buffer: cursor.vertex_buffer.take(), index_buffer: cursor.index_buffer.take(), index_count: cursor.schema.indices, source:cursor.lease },
        };
        match self.meshes.insert(entry) {
            Ok(()) => Ok(true),
            Err(entry) => {
                cursor.vertex_buffer = entry.value.vertex_buffer;
                cursor.index_buffer = entry.value.index_buffer;
                self.upload = Some(cursor);
                Err("mesh GPU table capacity exhausted")
            }
        }
    }

    pub fn close_upload_step(&mut self) -> bool {
        let Some(cursor) = self.upload.as_mut() else { return true };
        if let Some(buffer) = cursor.vertex_buffer.take() {
            buffer.destroy();
            return false;
        }
        if let Some(buffer) = cursor.index_buffer.take() {
            buffer.destroy();
            return false;
        }
        self.upload = None;
        false
    }

    pub fn upload_terminal_is_empty(&self) -> bool {
        self.upload.is_none()
    }

    /// 🐕️ How far the in-flight mesh upload has walked its lease: `(vertex, index)`, `(0, 0)` when no
    /// cursor is held. The presentation watchdog's only window into a per-scalar upload.
    pub fn upload_progress(&self) -> (u32, u32) {
        self.upload.as_ref().map_or((0, 0), |cursor| (cursor.vertex, cursor.index))
    }

    fn begin_retirement(&mut self, selector: MeshGpuRetirementSelector) -> Result<(), &'static str> {
        if let Some(retirement) = self.retirement.as_ref() {
            return (retirement.selector == selector).then_some(()).ok_or("mesh GPU retirement authority is occupied");
        }
        self.retirement = Some(MeshGpuRetirementCursor { selector, scan: 0, owner: None });
        Ok(())
    }

    fn retirement_step(&mut self) -> bool {
        let Some(cursor) = self.retirement.as_mut() else { return true };
        if let Some(owner) = cursor.owner.as_mut() {
            if let Some(buffer) = owner.vertex_buffer.take() {
                buffer.destroy();
                return false;
            }
            if let Some(buffer) = owner.index_buffer.take() {
                buffer.destroy();
                return false;
            }
            cursor.owner = None;
            if matches!(cursor.selector, MeshGpuRetirementSelector::Exact { .. }) {
                cursor.scan = MESH_GPU_TABLE_CAPACITY;
            }
            return false;
        }
        if cursor.scan >= MESH_GPU_TABLE_CAPACITY {
            self.retirement = None;
            return true;
        }
        let index = cursor.scan;
        cursor.scan += 1;
        let selected = self.meshes.slots[index].as_ref().is_some_and(|entry| cursor.selector.selects(entry));
        if selected {
            let entry = self.meshes.take(index).expect("selected mesh GPU entry");
            cursor.owner = Some(MeshGpuRetirementOwner { vertex_buffer: entry.value.vertex_buffer, index_buffer: entry.value.index_buffer });
        }
        false
    }

    pub fn retire_exact_step(&mut self, key: &str, version: u64) -> Result<bool, &'static str> {
        self.begin_retirement(MeshGpuRetirementSelector::Exact { key: MeshGpuKey::new(key)?, version })?;
        Ok(self.retirement_step())
    }

    /// 🏁️ Original pages remain borrowed until their real upload and exact GPU buffers close.
    pub fn retire_original_source_step(&mut self,key:&str,source:&crate::kernel_3d_scene::Mesh3dResourceLoan)->Result<Option<MeshGpuSourceReturn>,&'static str>{
        let identity=source.identity();let lease=source.lease().ok_or("original native mesh source has returned")?;let original_key=MeshGpuKey::new(key)?;
        if let Some(upload)=self.upload.as_ref(){if upload.key==original_key&&upload.version==identity.cache_epoch{if upload.lease!=lease{return Err("native GPU upload differs from its original source")};self.close_upload_step();return Ok(None)}}
        if self.meshes.get(key,identity.cache_epoch).is_some_and(|cached|cached.source!=lease){return Err("native GPU cache differs from its original source")}
        if !self.retire_exact_step(key,identity.cache_epoch)?{return Ok(None)}
        Ok(Some(MeshGpuSourceReturn{key:original_key,identity,source:lease}))
    }

    pub fn evict_mesh_step(&mut self, key: &str) -> Result<bool, &'static str> {
        self.evict_mesh_except_step(key, &[])
    }

    pub fn evict_mesh_except_step(&mut self, key: &str, keep_versions: &[u64]) -> Result<bool, &'static str> {
        if keep_versions.len() > MESH_GPU_KEEP_VERSION_CAPACITY {
            return Err("mesh GPU eviction keep-version credits exhausted");
        }
        let mut versions = [0; MESH_GPU_KEEP_VERSION_CAPACITY];
        versions[..keep_versions.len()].copy_from_slice(keep_versions);
        self.begin_retirement(MeshGpuRetirementSelector::KeyExcept { key: MeshGpuKey::new(key)?, versions, len: keep_versions.len() as u16 })?;
        Ok(self.retirement_step())
    }

    pub fn close_step(&mut self) -> bool {
        self.closing = true;
        if !self.close_upload_step() {
            return false;
        }
        if self.retirement.is_none() {
            let _ = self.begin_retirement(MeshGpuRetirementSelector::All);
        }
        if !self.retirement_step() {
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.upload.is_none() && self.retirement.is_none() && self.meshes.is_empty()
    }
}


#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
