//! @emoji 🗃️ Typed generational resource ids, residency states and the `ResourceOp` upload/evict stream.
//!
//! Replaces the wgpu target's string-keyed tables (`raster_instances: Vec<(String, UiInstance)>`,
//! which cloned a `String` per instance per frame — a correctness hazard and a per-frame allocation)
//! with handles interned once. The registry never touches a device: it owns id allocation, string
//! interning and residency bookkeeping, and emits a plain [`ResourceOp`] stream that a backend applies
//! before replaying a [`crate::scene::RenderPacket`].

//#region 🔖️Resource

//#region Ids

/// 🎰️ One entry in a [`ResourceTable`]: a reusable slot plus the generation of its current occupant.
/// A stale id (wrong generation for its slot) is a dangling reference, never a panic — every lookup
/// checks the generation and returns `None`/`false` on mismatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Slot {
    index: u32,
    generation: u32,
}

/// 🏷️ A typed generational resource handle. Implemented by [`TextureId`], [`MeshId`] and [`AtlasId`]
/// via [`resource_id`] so [`ResourceTable`] can allocate, look up and evict any of the three through
/// one generic implementation instead of three hand-copied ones. `pub(crate)`, never `pub` — it
/// references the crate-private [`Slot`], and nothing outside this crate needs to implement it.
pub(crate) trait ResourceId: Copy + Eq + std::hash::Hash + std::fmt::Debug {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from_slot(slot: Slot) -> Self;
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn slot(self) -> Slot;
}

macro_rules! resource_id {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $name {
            slot: u32,
            generation: u32,
        }

        impl ResourceId for $name {
            fn from_slot(slot: Slot) -> Self {
                Self { slot: slot.index, generation: slot.generation }
            }
            fn slot(self) -> Slot {
                Slot { index: self.slot, generation: self.generation }
            }
        }
    };
}

resource_id!(TextureId, "🖼️ A resident (or pending) GPU texture — glyph/icon rasters, raster images.");
resource_id!(MeshId, "🧊️ A resident (or pending) GPU mesh — world3d vertex/index buffers, content-versioned.");
resource_id!(AtlasId, "🔤️ A resident (or pending) glyph/icon atlas texture.");

//#endregion Ids

//#region State

/// 🚦️ Where a resource is in its upload lifecycle. `Evicted` is terminal for the id (the slot is
/// recycled under a bumped generation); `Requested` is re-entered by [`ResourceRegistry::report_device_loss`]
/// so the *same* id resumes the lifecycle without a new allocation.
#[derive(Clone, Debug, PartialEq)]
pub enum ResourceState {
    Requested,
    Decoding,
    PendingUpload,
    Resident,
    Evicted,
    Failed(ResourceError),
}

/// ⚠️ Why a resource never reached `Resident`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceError {
    pub message: String,
}

//#endregion State

//#region Ops

/// 📤️ One instruction for a backend to apply before it replays a [`crate::scene::RenderPacket`]. The
/// registry decides *what* changed; a backend decides *how* to move bytes to a device — this crate
/// never does the latter.
#[derive(Clone, Debug, PartialEq)]
pub enum ResourceOp {
    UploadAtlas { id: AtlasId, width: u32, height: u32, pixels: Vec<u8> },
    UploadTexture { id: TextureId, width: u32, height: u32, pixels: Vec<u8> },
    CreateOrUpdateMesh { id: MeshId, positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32> },
    EvictTexture(TextureId),
    EvictMesh(MeshId),
}

//#endregion Ops

//#region Table

/// 🗄️ Generational slot storage shared by the texture/mesh/atlas tables: allocation, string
/// interning (so a caller's `"icon/foo"` becomes an id once, never per frame) and eviction with
/// generation bump. Kept private — [`ResourceRegistry`] is the crate's only public entry point so a
/// caller never juggles three near-identical tables directly.
struct TableEntry {
    generation: u32,
    state: ResourceState,
    key: Option<String>,
}

struct ResourceTable<Id: ResourceId> {
    entries: Vec<TableEntry>,
    free: Vec<u32>,
    interned: std::collections::HashMap<String, Id>,
}

impl<Id: ResourceId> Default for ResourceTable<Id> {
    fn default() -> Self {
        Self { entries: Vec::new(), free: Vec::new(), interned: std::collections::HashMap::new() }
    }
}

impl<Id: ResourceId> ResourceTable<Id> {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn alloc(&mut self, key: Option<&str>, state: ResourceState) -> Id {
        let index = match self.free.pop() {
            Some(index) => {
                let entry = &mut self.entries[index as usize];
                entry.state = state;
                entry.key = key.map(str::to_string);
                index
            }
            None => {
                let index = self.entries.len() as u32;
                self.entries.push(TableEntry { generation: 0, state, key: key.map(str::to_string) });
                index
            }
        };
        let generation = self.entries[index as usize].generation;
        let id = Id::from_slot(Slot { index, generation });
        if let Some(key) = key {
            self.interned.insert(key.to_string(), id);
        }
        id
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn interned(&self, key: &str) -> Option<Id> {
        self.interned.get(key).copied()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn entry(&self, id: Id) -> Option<&TableEntry> {
        let slot = id.slot();
        self.entries.get(slot.index as usize).filter(|entry| entry.generation == slot.generation)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn set_state(&mut self, id: Id, state: ResourceState) -> bool {
        let slot = id.slot();
        match self.entries.get_mut(slot.index as usize) {
            Some(entry) if entry.generation == slot.generation => {
                entry.state = state;
                true
            }
            _ => false,
        }
    }

    /// 🧨️ Retires `id`: drops the string interning so a future request re-allocates, and bumps the
    /// slot's generation so `id` itself becomes permanently stale (any stray copy fails every lookup).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn evict(&mut self, id: Id) -> bool {
        let slot = id.slot();
        let Some(entry) = self.entries.get_mut(slot.index as usize) else { return false };
        if entry.generation != slot.generation {
            return false;
        }
        if let Some(key) = entry.key.take() {
            self.interned.remove(&key);
        }
        entry.state = ResourceState::Evicted;
        entry.generation = entry.generation.wrapping_add(1);
        self.free.push(slot.index);
        true
    }
}

//#endregion Table

//#region Registry

/// 🧬️ Owns id allocation/interning for textures, meshes and atlases, and accumulates the
/// [`ResourceOp`] stream a backend must apply before replaying the next [`crate::scene::RenderPacket`].
/// One registry is long-lived across frames — interning makes a repeated `"icon/foo"` key resolve to
/// the same [`TextureId`] every frame instead of re-allocating.
#[derive(Default)]
pub struct ResourceRegistry {
    textures: ResourceTable<TextureId>,
    meshes: ResourceTable<MeshId>,
    atlases: ResourceTable<AtlasId>,
    ops: Vec<ResourceOp>,
}

impl ResourceRegistry {
    //#region Textures

    /// 🔗️ Resolves `key` to a stable [`TextureId`], allocating on first sight. Does not queue an
    /// upload — call [`Self::request_texture_upload`] when pixel data is ready.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn intern_texture(&mut self, key: &str) -> TextureId {
        self.textures.interned(key).unwrap_or_else(|| self.textures.alloc(Some(key), ResourceState::Requested))
    }

    /// 📤️ Interns `key` and queues an upload unless the texture is already `Resident`. Safe to call
    /// every frame with unchanged pixels — it becomes a no-op once resident.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn request_texture_upload(&mut self, key: &str, width: u32, height: u32, pixels: Vec<u8>) -> TextureId {
        let id = self.intern_texture(key);
        let resident = matches!(self.textures.entry(id).map(|entry| &entry.state), Some(ResourceState::Resident));
        if !resident {
            self.textures.set_state(id, ResourceState::PendingUpload);
            let queued = self.ops.iter_mut().find(|op| matches!(op, ResourceOp::UploadTexture { id: queued, .. } if *queued == id));
            match queued {
                Some(op) => *op = ResourceOp::UploadTexture { id, width, height, pixels },
                None => self.ops.push(ResourceOp::UploadTexture { id, width, height, pixels }),
            }
        }
        id
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn texture_state(&self, id: TextureId) -> Option<&ResourceState> {
        self.textures.entry(id).map(|entry| &entry.state)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    /// ✅️ Marks a texture resident and drops any still-queued upload for it. The invariant this keeps
    /// is `ops` never carries an upload for a resource the backend already holds — without it a
    /// resource that went resident mid-frame would be re-uploaded on the next drain.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn mark_texture_resident(&mut self, id: TextureId) -> bool {
        self.ops.retain(|op| !matches!(op, ResourceOp::UploadTexture { id: queued, .. } if *queued == id));
        self.textures.set_state(id, ResourceState::Resident)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn mark_texture_failed(&mut self, id: TextureId, message: impl Into<String>) -> bool {
        self.textures.set_state(id, ResourceState::Failed(ResourceError { message: message.into() }))
    }

    /// 🧨️ Retires `id` and queues [`ResourceOp::EvictTexture`]. A later request under the same key
    /// allocates a fresh id — `id` itself never resolves again.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn evict_texture(&mut self, id: TextureId) {
        if self.textures.evict(id) {
            self.ops.push(ResourceOp::EvictTexture(id));
        }
    }

    //#endregion Textures

    //#region Atlases

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn intern_atlas(&mut self, key: &str) -> AtlasId {
        self.atlases.interned(key).unwrap_or_else(|| self.atlases.alloc(Some(key), ResourceState::Requested))
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn request_atlas_upload(&mut self, key: &str, width: u32, height: u32, pixels: Vec<u8>) -> AtlasId {
        let id = self.intern_atlas(key);
        let resident = matches!(self.atlases.entry(id).map(|entry| &entry.state), Some(ResourceState::Resident));
        if !resident {
            self.atlases.set_state(id, ResourceState::PendingUpload);
            let queued = self.ops.iter_mut().find(|op| matches!(op, ResourceOp::UploadAtlas { id: queued, .. } if *queued == id));
            match queued {
                Some(op) => *op = ResourceOp::UploadAtlas { id, width, height, pixels },
                None => self.ops.push(ResourceOp::UploadAtlas { id, width, height, pixels }),
            }
        }
        id
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    /// ✅️ Marks an atlas resident and drops any still-queued upload for it — same invariant as
    /// [`Self::mark_texture_resident`].
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn mark_atlas_resident(&mut self, id: AtlasId) -> bool {
        self.ops.retain(|op| !matches!(op, ResourceOp::UploadAtlas { id: queued, .. } if *queued == id));
        self.atlases.set_state(id, ResourceState::Resident)
    }

    //#endregion Atlases

    //#region Meshes

    /// 🔢️ Content hash (FNV-1a) over positions, normals and indices — the mesh-versioning scheme
    /// ported from the wgpu target's `mesh_content_version`: identical geometry hashes identically, so
    /// re-requesting an unchanged mesh resolves to the same id instead of re-uploading.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn mesh_content_hash(positions: &[f32], normals: &[f32], indices: &[u32]) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for value in positions.iter().chain(normals.iter()) {
            hash ^= u64::from(value.to_bits());
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        for value in indices {
            hash ^= u64::from(*value);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    /// 📤️ Interns `key` content-versioned by [`Self::mesh_content_hash`] and queues
    /// [`ResourceOp::CreateOrUpdateMesh`] on first sight of that exact content; an unchanged mesh
    /// resolves to the same id with no new op, exactly like the ported `MeshGpuTable::ensure_mesh`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn request_mesh_upload(&mut self, key: &str, positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32>) -> MeshId {
        let version = Self::mesh_content_hash(&positions, &normals, &indices);
        let versioned_key = format!("{key}:{version:016x}");
        if let Some(id) = self.meshes.interned(&versioned_key) {
            return id;
        }
        let id = self.meshes.alloc(Some(&versioned_key), ResourceState::PendingUpload);
        self.ops.push(ResourceOp::CreateOrUpdateMesh { id, positions, normals, indices });
        id
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    /// ✅️ Marks a mesh resident and drops any still-queued upload for it — same invariant as
    /// [`Self::mark_texture_resident`].
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn mark_mesh_resident(&mut self, id: MeshId) -> bool {
        self.ops.retain(|op| !matches!(op, ResourceOp::CreateOrUpdateMesh { id: queued, .. } if *queued == id));
        self.meshes.set_state(id, ResourceState::Resident)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn evict_mesh(&mut self, id: MeshId) {
        if self.meshes.evict(id) {
            self.ops.push(ResourceOp::EvictMesh(id));
        }
    }

    //#endregion Meshes

    //#region Stream

    /// 📬️ Takes the accumulated `ResourceOp` stream, leaving the registry's queue empty. A caller
    /// hands the result to a backend before replaying the packet built in the same frame.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn drain_ops(&mut self) -> Vec<ResourceOp> {
        std::mem::take(&mut self.ops)
    }

    /// ♻️ A backend reports the ids that died in a device loss; each surviving id (generation still
    /// matches) is re-marked `Requested` *without* a generation bump, so it resumes the same identity
    /// and the next frame's upload request repopulates it. An id whose slot was already evicted for
    /// another reason is silently ignored.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn report_device_loss(&mut self, lost_textures: &[TextureId], lost_meshes: &[MeshId], lost_atlases: &[AtlasId]) {
        for &id in lost_textures {
            self.textures.set_state(id, ResourceState::Requested);
        }
        for &id in lost_meshes {
            self.meshes.set_state(id, ResourceState::Requested);
        }
        for &id in lost_atlases {
            self.atlases.set_state(id, ResourceState::Requested);
        }
    }

    //#endregion Stream
}

//#endregion Registry

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interning_the_same_key_twice_returns_the_same_id_and_queues_one_upload() {
        let mut registry = ResourceRegistry::default();
        let a = registry.request_texture_upload("icon/foo", 4, 4, vec![0; 64]);
        let b = registry.request_texture_upload("icon/foo", 4, 4, vec![0; 64]);
        assert_eq!(a, b);
        assert_eq!(registry.drain_ops().len(), 1);
    }

    #[test]
    fn resident_texture_is_not_re_queued() {
        let mut registry = ResourceRegistry::default();
        let id = registry.request_texture_upload("icon/foo", 4, 4, vec![0; 64]);
        registry.mark_texture_resident(id);
        registry.request_texture_upload("icon/foo", 4, 4, vec![0; 64]);
        assert!(registry.drain_ops().is_empty());
    }

    #[test]
    fn eviction_bumps_generation_so_the_old_id_never_resolves_again() {
        let mut registry = ResourceRegistry::default();
        let id = registry.request_texture_upload("icon/foo", 4, 4, vec![0; 64]);
        registry.drain_ops();
        registry.evict_texture(id);
        assert!(registry.texture_state(id).is_none());
        let ops = registry.drain_ops();
        assert_eq!(ops, vec![ResourceOp::EvictTexture(id)]);
        let reallocated = registry.intern_texture("icon/foo");
        assert_ne!(reallocated, id);
    }

    #[test]
    fn evicted_slot_is_reused_by_the_next_allocation() {
        let mut registry = ResourceRegistry::default();
        let first = registry.intern_texture("a");
        registry.evict_texture(first);
        let second = registry.intern_texture("b");
        assert_eq!(first.slot, second.slot);
        assert_ne!(first.generation, second.generation);
    }

    #[test]
    fn device_loss_re_marks_requested_without_changing_identity() {
        let mut registry = ResourceRegistry::default();
        let id = registry.request_texture_upload("icon/foo", 4, 4, vec![0; 64]);
        registry.mark_texture_resident(id);
        registry.report_device_loss(&[id], &[], &[]);
        assert_eq!(registry.texture_state(id), Some(&ResourceState::Requested));
        let reused = registry.intern_texture("icon/foo");
        assert_eq!(reused, id);
    }

    #[test]
    fn mesh_content_hash_changes_with_indices() {
        let a = ResourceRegistry::mesh_content_hash(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 1, 2]);
        let b = ResourceRegistry::mesh_content_hash(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0], &[0, 2, 1]);
        assert_ne!(a, b);
    }

    #[test]
    fn unchanged_mesh_content_resolves_to_the_same_id_with_no_new_op() {
        let mut registry = ResourceRegistry::default();
        let a = registry.request_mesh_upload("box", vec![0.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0, 1, 2]);
        registry.drain_ops();
        let b = registry.request_mesh_upload("box", vec![0.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0, 1, 2]);
        assert_eq!(a, b);
        assert!(registry.drain_ops().is_empty());
    }

    #[test]
    fn changed_mesh_content_allocates_a_new_id_and_queues_an_upload() {
        let mut registry = ResourceRegistry::default();
        let a = registry.request_mesh_upload("box", vec![0.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0, 1, 2]);
        let b = registry.request_mesh_upload("box", vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0, 1, 2]);
        assert_ne!(a, b);
    }
}

//#endregion Tests

//#endregion 🔖️Resource
