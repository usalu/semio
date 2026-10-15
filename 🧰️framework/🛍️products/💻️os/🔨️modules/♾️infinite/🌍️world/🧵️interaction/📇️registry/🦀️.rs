//! 📇️ World interaction ids, mesh and object registries, build cursor, intent queue, plan and authority state.

use super::*;

//#region 🧵️WorldInteractionTransaction
pub const WORLD_INTERACTION_ITEM_CAPACITY: usize = 256;
pub const WORLD_INTERACTION_BYTE_CAPACITY: usize = 16 * 1024;
pub const WORLD_INTERACTION_INTENT_CAPACITY: usize = 64;
pub const WORLD_INTERACTION_MESH_CAPACITY: usize = 256;
pub const WORLD_INTERACTION_OBJECT_CAPACITY: usize = 1024;
pub const WORLD_INTERACTION_MARQUEE_POINT_CAPACITY: usize = 256;
pub const WORLD_INTERACTION_ID_BYTE_CAPACITY: usize = 256;
pub const WORLD_INTERACTION_TOPOLOGY_BYTE_CAPACITY: usize = 64 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::world) struct WorldInteractionId {
    pub(in crate::world) bytes: [u8; WORLD_INTERACTION_ID_BYTE_CAPACITY],
    pub(in crate::world) len: u16,
}

impl WorldInteractionId {
    pub(in crate::world) fn new(value: &str) -> Option<Self> {
        if value.len() > WORLD_INTERACTION_ID_BYTE_CAPACITY {
            return None;
        }
        let mut bytes = [0; WORLD_INTERACTION_ID_BYTE_CAPACITY];
        bytes[..value.len()].copy_from_slice(value.as_bytes());
        Some(Self { bytes, len: value.len() as u16 })
    }

    pub(in crate::world) fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("world registry id originates from UTF-8")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::world) struct WorldInteractionMeshToken {
    pub(in crate::world) slot: u16,
    pub(in crate::world) generation: u64,
}

#[derive(Clone, Copy)]
struct WorldInteractionMeshSlot {
    pub(in crate::world) id: WorldInteractionId,
    pub(in crate::world) generation: u64,
    pub(in crate::world) version: u64,
    pub(in crate::world) vertices: u32,
    pub(in crate::world) triangles: u32,
    pub(in crate::world) edges: u32,
    topology_bytes: u32,
}

pub(in crate::world) struct WorldInteractionMeshRegistry {
    pub(in crate::world) slots: Box<[Option<WorldInteractionMeshSlot>; WORLD_INTERACTION_MESH_CAPACITY]>,
    pub(in crate::world) epochs: Box<[u64; WORLD_INTERACTION_MESH_CAPACITY]>,
    pub(in crate::world) faulted: bool,
}

#[derive(Clone, Copy)]
struct WorldInteractionMeshPlan {
    pub(in crate::world) slot: u16,
    pub(in crate::world) epoch: u64,
    existing_generation: Option<u64>,
    pub(in crate::world) value: WorldInteractionMeshSlot,
}

fn mesh3d_schema_bytes(schema: Mesh3dSchema) -> Option<usize> {
    usize::try_from(schema.vertices)
        .ok()?
        .checked_mul(24)?
        .checked_add(usize::try_from(schema.indices).ok()?.checked_mul(4)?)?
        .checked_add(usize::try_from(schema.face_ids).ok()?.checked_mul(4)?)?
        .checked_add(usize::try_from(schema.vertex_ids).ok()?.checked_mul(4)?)?
        .checked_add(usize::try_from(schema.edges).ok()?.checked_mul(24)?)?
        .checked_add(usize::try_from(schema.edge_ids).ok()?.checked_mul(4)?)?
        .checked_add(usize::try_from(schema.uvs).ok()?.checked_mul(8)?)?
        .checked_add(usize::try_from(schema.colors).ok()?.checked_mul(16)?)
}

impl Default for WorldInteractionMeshRegistry {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: semio_framework_async::boxed_fixed_slots(|| 0), faulted: false }
    }
}

impl WorldInteractionMeshRegistry {
    pub(in crate::world) fn hash(id: &str) -> usize {
        (id.as_bytes().iter().fold(2_166_136_261u64, |hash, byte| hash.wrapping_mul(16_777_619) ^ u64::from(*byte)) % WORLD_INTERACTION_MESH_CAPACITY as u64) as usize
    }

    pub(in crate::world) fn plan_admit(&self, id: &str, version: u64, mesh: Mesh3dLease) -> Result<WorldInteractionMeshPlan, WorldDynamicFault> {
        let Some(id) = WorldInteractionId::new(id) else {
            return Err(WorldDynamicFault::IdCapacity);
        };
        self.plan_admit_id(id,version,mesh)
    }

    pub(in crate::world) fn plan_admit_id(&self, id: WorldInteractionId, version: u64, mesh: Mesh3dLease) -> Result<WorldInteractionMeshPlan, WorldDynamicFault> {
        let Ok(schema) = mesh.schema() else {
            return Err(WorldDynamicFault::StaleToken);
        };
        let vertices = schema.vertices;
        let triangles = schema.indices / 3;
        let edges = if schema.edge_ids == schema.edges { schema.edges } else { 0 };
        let topology_bytes = mesh3d_schema_bytes(schema);
        let Some(topology_bytes) = topology_bytes else {
            return Err(WorldDynamicFault::ByteCapacity);
        };
        if topology_bytes > WORLD_INTERACTION_TOPOLOGY_BYTE_CAPACITY {
            return Err(WorldDynamicFault::ByteCapacity);
        }
        let start = Self::hash(id.as_str());
        let mut empty = None;
        for offset in 0..WORLD_INTERACTION_MESH_CAPACITY {
            let slot = (start + offset) % WORLD_INTERACTION_MESH_CAPACITY;
            match self.slots[slot] {
                Some(existing) if existing.id == id => {
                    if existing.version == version && existing.vertices == vertices && existing.triangles == triangles && existing.edges == edges && existing.topology_bytes == topology_bytes as u32 {
                        return Ok(WorldInteractionMeshPlan { slot: slot as u16, epoch: self.epochs[slot], existing_generation: Some(existing.generation), value: existing });
                    }
                    let generation = self.epochs[slot].checked_add(1).ok_or(WorldDynamicFault::StaleToken)?;
                    return Ok(WorldInteractionMeshPlan {
                        slot: slot as u16,
                        epoch: self.epochs[slot],
                        existing_generation: Some(existing.generation),
                        value: WorldInteractionMeshSlot { id, generation, version, vertices, triangles, edges, topology_bytes: topology_bytes as u32 },
                    });
                }
                None => {
                    empty = Some(slot);
                    break;
                }
                _ => {}
            }
        }
        let slot = empty.ok_or(WorldDynamicFault::RegistryCapacity)?;
        let generation = self.epochs[slot].checked_add(1).ok_or(WorldDynamicFault::StaleToken)?;
        Ok(WorldInteractionMeshPlan { slot: slot as u16, epoch: self.epochs[slot], existing_generation: None, value: WorldInteractionMeshSlot { id, generation, version, vertices, triangles, edges, topology_bytes: topology_bytes as u32 } })
    }

    pub(in crate::world) fn commit_admit(&mut self, plan: WorldInteractionMeshPlan) -> Result<WorldInteractionMeshToken, WorldDynamicFault> {
        let slot = usize::from(plan.slot);
        let current = self.slots.get(slot).copied().ok_or(WorldDynamicFault::StaleToken)?;
        let matches = match (plan.existing_generation, current) {
            (Some(generation), Some(current)) => current.generation == generation && self.epochs[slot] == plan.epoch,
            (None, None) => self.epochs[slot] == plan.epoch,
            _ => false,
        };
        if !matches {
            return Err(WorldDynamicFault::StaleToken);
        }
        self.epochs[slot] = plan.value.generation;
        self.slots[slot] = Some(plan.value);
        Ok(WorldInteractionMeshToken { slot: plan.slot, generation: plan.value.generation })
    }

    #[cfg(test)]
    pub(in crate::world) fn admit(&mut self, id: &str, version: u64, mesh: Mesh3dLease) -> Option<WorldInteractionMeshToken> {
        let plan = match self.plan_admit(id, version, mesh) {
            Ok(plan) => plan,
            Err(_) => {
                self.faulted = true;
                return None;
            }
        };
        match self.commit_admit(plan) {
            Ok(token) => Some(token),
            Err(_) => {
                self.faulted = true;
                None
            }
        }
    }

    pub(in crate::world) fn resolve(&self, token: WorldInteractionMeshToken) -> Option<&WorldInteractionMeshSlot> {
        let slot = self.slots.get(usize::from(token.slot))?.as_ref()?;
        (slot.generation == token.generation).then_some(slot)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::world) enum WorldInteractionObjectKind {
    Instance,
    Vortex,
    Reference,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::world) struct WorldInteractionObjectToken {
    pub(in crate::world) slot: u16,
    pub(in crate::world) generation: u64,
    pub(in crate::world) revision: u64,
}

#[derive(Clone, Copy)]
pub(in crate::world) struct WorldInteractionObjectSlot {
    pub(in crate::world) id: WorldInteractionId,
    pub(in crate::world) kind: WorldInteractionObjectKind,
    pub(in crate::world) generation: u64,
    pub(in crate::world) revision: u64,
    pub(in crate::world) mesh: Option<WorldInteractionMeshToken>,
    pub(in crate::world) component_source: Option<ComponentSource3d>,
    pub(in crate::world) model: Mat4,
    pub(in crate::world) values: [f32; 8],
}

pub(in crate::world) struct WorldInteractionObjectRegistry {
    pub(in crate::world) slots: Box<[Option<WorldInteractionObjectSlot>; WORLD_INTERACTION_OBJECT_CAPACITY]>,
    pub(in crate::world) occupied: [u64; WORLD_INTERACTION_OBJECT_CAPACITY.div_ceil(64)],
    pub(in crate::world) epochs: Box<[u64; WORLD_INTERACTION_OBJECT_CAPACITY]>,
    pub(in crate::world) instance_order: Box<[Option<WorldInteractionObjectToken>; WORLD_INTERACTION_OBJECT_CAPACITY]>,
    pub(in crate::world) instance_len: u16,
    pub(in crate::world) revision: u64,
    pub(in crate::world) faulted: bool,
}

impl Default for WorldInteractionObjectRegistry {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), occupied: [0; WORLD_INTERACTION_OBJECT_CAPACITY.div_ceil(64)], epochs: semio_framework_async::boxed_fixed_slots(|| 0), instance_order: semio_framework_async::boxed_fixed_slots(|| None), instance_len: 0, revision: u64::MAX, faulted: false }
    }
}

impl WorldInteractionObjectRegistry {
    pub(in crate::world) fn hash(kind: WorldInteractionObjectKind, id: &str) -> usize {
        let seed = match kind {
            WorldInteractionObjectKind::Instance => 0x11u64,
            WorldInteractionObjectKind::Vortex => 0x22,
            WorldInteractionObjectKind::Reference => 0x33,
        };
        (id.as_bytes().iter().fold(2_166_136_261u64 ^ seed, |hash, byte| hash.wrapping_mul(16_777_619) ^ u64::from(*byte)) % WORLD_INTERACTION_OBJECT_CAPACITY as u64) as usize
    }

    /// 🪪️ Admits one object for one interaction revision, reusing the slot its own `(kind, id)`
    /// already holds — even from an EARLIER revision — before it takes a free one.
    ///
    /// 🩸️ A stale-revision slot of the same id only seeded `reusable` and the probe walked on to the
    /// next EMPTY slot, so re-admitting an id at a new revision left the previous revision's entry
    /// intact AND resolvable: `resolve` matches on generation and revision, both of which the old
    /// token still carried. Every revision therefore leaked a slot until the fixed table faulted, and
    /// a marquee cursor holding the previous token kept picking the object it had already replaced.
    pub(in crate::world) fn admit(&mut self, revision: u64, kind: WorldInteractionObjectKind, id: &str, mesh: Option<WorldInteractionMeshToken>, model: Mat4, values: [f32; 8], component_source: Option<ComponentSource3d>) -> Option<WorldInteractionObjectToken> {
        let Some(id) = WorldInteractionId::new(id) else {
            self.faulted = true;
            return None;
        };
        let start = Self::hash(kind, id.as_str());
        let mut reusable = None;
        for offset in 0..WORLD_INTERACTION_OBJECT_CAPACITY {
            let slot = (start + offset) % WORLD_INTERACTION_OBJECT_CAPACITY;
            match self.slots[slot] {
                Some(existing) if existing.revision == revision && existing.kind == kind && existing.id == id => {
                    if existing.mesh == mesh && existing.component_source == component_source && existing.model.cols == model.cols && existing.values == values {
                        return Some(WorldInteractionObjectToken { slot: slot as u16, generation: existing.generation, revision });
                    }
                    reusable = Some(slot);
                    break;
                }
                Some(existing) if existing.revision != revision => {
                    if existing.kind == kind && existing.id == id {
                        reusable = Some(slot);
                        break;
                    }
                    reusable.get_or_insert(slot);
                }
                None => {
                    reusable = Some(slot);
                    break;
                }
                _ => {}
            }
        }
        let Some(slot) = reusable else {
            self.faulted = true;
            return None;
        };
        let Some(generation) = self.epochs[slot].checked_add(1) else {
            self.faulted = true;
            return None;
        };
        self.epochs[slot] = generation;
        self.slots[slot] = Some(WorldInteractionObjectSlot { id, kind, generation, revision, mesh, component_source, model, values });
        self.occupied[slot / 64] |= 1u64 << (slot % 64);
        Some(WorldInteractionObjectToken { slot: slot as u16, generation, revision })
    }

    /// 🧭️ Finds the next occupied slot in at most sixteen bitmap words, preserving stable slot order.
    pub(super) fn next_occupied(&self, start: u16) -> Option<(usize, &WorldInteractionObjectSlot)> {
        let mut word = usize::from(start) / 64;
        let mut bits = *self.occupied.get(word)? & (u64::MAX << (usize::from(start) % 64));
        loop {
            if bits != 0 {
                let slot = word * 64 + bits.trailing_zeros() as usize;
                return self.slots.get(slot)?.as_ref().map(|entry| (slot, entry));
            }
            word += 1;
            bits = *self.occupied.get(word)?;
        }
    }

    pub(in crate::world) fn resolve(&self, token: WorldInteractionObjectToken) -> Option<&WorldInteractionObjectSlot> {
        let slot = self.slots.get(usize::from(token.slot))?.as_ref()?;
        (slot.generation == token.generation && slot.revision == token.revision).then_some(slot)
    }

    fn begin_instance_order(&mut self) {
        self.instance_len = 0;
    }

    fn push_instance_order(&mut self, token: WorldInteractionObjectToken) -> bool {
        let index = usize::from(self.instance_len);
        if index == WORLD_INTERACTION_OBJECT_CAPACITY {
            self.faulted = true;
            return false;
        }
        self.instance_order[index] = Some(token);
        self.instance_len += 1;
        true
    }

    pub(in crate::world) fn terminal_for_revision(&self, revision: u64) -> bool {
        self.revision == revision && !self.faulted
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::world) enum WorldInteractionRegistryBuildPhase {
    Instances,
    Vortices,
    References,
    Complete,
}

pub(in crate::world) struct WorldInteractionRegistryBuildCursor {
    pub(in crate::world) revision: u64,
    initialized: bool,
    pub(in crate::world) phase: WorldInteractionRegistryBuildPhase,
    pub(in crate::world) draw: usize,
    pub(in crate::world) instance: usize,
    pub(super) mesh_probe: u16,
    vortex: usize,
    reference: usize,
    pub(in crate::world) faulted: bool,
}

impl WorldInteractionRegistryBuildCursor {
    pub(in crate::world) fn new(revision: u64) -> Self {
        Self { revision, initialized: false, phase: WorldInteractionRegistryBuildPhase::Instances, draw: 0, instance: 0, mesh_probe: 0, vortex: 0, reference: 0, faulted: false }
    }

    /// 🫥️ An EMPTY probe slot is "this mesh was never published", not a defect: the
    /// guest publishes a draw list and its mesh leases in separate deliveries, so a
    /// draw naming a glb whose lease has not landed is normal for the frames in
    /// between. Faulting here killed the whole page with `world3d retained
    /// interaction authority faulted` on the first pointer move after a mesh-bearing
    /// document mounted (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w12c`).
    /// Skipping the draw is React's own behaviour — a `GlbInstanceMesh` whose loader
    /// has not resolved renders nothing and raycasts to nothing — and is the
    /// interaction twin of W11a's "a miss is a skipped draw, not a quarantine".
    pub(in crate::world) fn step(&mut self, state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.faulted || state.interaction_objects.faulted || state.interaction_meshes.faulted {
            return WorldInteractionStep::Fault;
        }
        if self.revision != state.interaction_revision {
            return WorldInteractionStep::Stale;
        }
        if !self.initialized {
            state.interaction_objects.begin_instance_order();
            self.initialized = true;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        match self.phase {
            WorldInteractionRegistryBuildPhase::Instances => {
                let Some(draw) = state.draws.get(self.draw) else {
                    self.phase = WorldInteractionRegistryBuildPhase::Vortices;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                if usize::from(self.mesh_probe) == WORLD_INTERACTION_MESH_CAPACITY || draw.mesh_key.len() > WORLD_INTERACTION_ID_BYTE_CAPACITY {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
                let mesh_slot = (WorldInteractionMeshRegistry::hash(&draw.mesh_key) + usize::from(self.mesh_probe)) % WORLD_INTERACTION_MESH_CAPACITY;
                let mesh = match state.interaction_meshes.slots[mesh_slot] {
                    Some(entry) if entry.id.as_str() == draw.mesh_key && entry.version == live_world3d_mesh_version(state, &draw.mesh_key) => Some(WorldInteractionMeshToken { slot: mesh_slot as u16, generation: entry.generation }),
                    Some(_) => {
                        self.mesh_probe += 1;
                        context.consume_fuel(1);
                        return WorldInteractionStep::Pending;
                    }
                    None => {
                        self.draw += 1;
                        self.instance = 0;
                        self.mesh_probe = 0;
                        context.consume_fuel(1);
                        return WorldInteractionStep::Pending;
                    }
                };
                let Some(instance) = draw.instances.get(self.instance) else {
                    self.draw += 1;
                    self.instance = 0;
                    self.mesh_probe = 0;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                let translation = instance.model.cols[3];
                let Some(token) = state.interaction_objects.admit(
                    self.revision,
                    WorldInteractionObjectKind::Instance,
                    &instance.id,
                    mesh,
                    instance.model,
                    [self.draw as f32, self.instance as f32, if instance.selected { 1.0 } else { 0.0 }, translation[0], translation[1], translation[2], 0.0, 0.0], instance.component_source,
                ) else {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                };
                if !state.interaction_objects.push_instance_order(token) {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
                self.instance += 1;
            }
            WorldInteractionRegistryBuildPhase::Vortices => {
                let Some(vortex) = state.vortices.get(self.vortex) else {
                    self.phase = WorldInteractionRegistryBuildPhase::References;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
                let direction = vortex.direction.unwrap_or([0.0, 0.0, -1.0]);
                let values = [position[0] as f32, position[1] as f32, position[2] as f32, direction[0] as f32, direction[1] as f32, direction[2] as f32, vortex.radius.unwrap_or(0.36) as f32, 0.0];
                if state.interaction_objects.admit(self.revision, WorldInteractionObjectKind::Vortex, &vortex.full_id, None, Mat4::identity(), values, None).is_none() {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
                self.vortex += 1;
            }
            WorldInteractionRegistryBuildPhase::References => {
                let Some(reference) = state.references.get(self.reference) else {
                    self.phase = WorldInteractionRegistryBuildPhase::Complete;
                    state.interaction_objects.revision = self.revision;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                self.reference += 1;
                if reference.hidden.unwrap_or(false) || reference.locked.unwrap_or(false) {
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                }
                let Some(url) = reference.url.as_deref() else {
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                let origin = reference.origin.unwrap_or([0.0, 0.0, 0.0]);
                let width = world_reference_width(reference);
                let aspect = reference_image_aspect(state, url);
                let values = [origin[0] as f32, origin[1] as f32, origin[2] as f32, width, width / aspect, 0.0, 0.0, 0.0];
                let id = reference.id.as_deref().unwrap_or(url);
                if state.interaction_objects.admit(self.revision, WorldInteractionObjectKind::Reference, id, None, Mat4::identity(), values, None).is_none() {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
            }
            WorldInteractionRegistryBuildPhase::Complete => return WorldInteractionStep::Complete,
        }
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    pub(in crate::world) fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if context.should_yield() {
            return false;
        }
        self.phase = WorldInteractionRegistryBuildPhase::Complete;
        context.consume_fuel(1);
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldInteractionPhase {
    PointerMove,
    PointerButton,
    PointerDrag,
    PointerLeave,
    Wheel,
    /// 🧯️ A host cancel of the gesture in flight (window blur, capture lost); its [`WorldCancelReason`] rides in `button`.
    Cancel,
    Close,
}

/// 🧯️ Why a host cancels a World3d gesture — the tool protocol's abort `reason`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldCancelReason {
    Blur,
    CaptureLost,
}

impl WorldCancelReason {
    /// 🏷️ The abort `reason` the guest's tool machine parses (`ToolAbortReason::parse`).
    pub fn code(self) -> &'static str {
        match self {
            Self::Blur => "blur",
            Self::CaptureLost => "captureLost",
        }
    }

    pub(super) fn from_button(button: i16) -> Self {
        if button == Self::CaptureLost as i16 {
            Self::CaptureLost
        } else {
            Self::Blur
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WorldInteractionIntent {
    pub phase: WorldInteractionPhase,
    pub generation: u64,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub delta: f32,
    pub button: i16,
    pub down: bool,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl WorldInteractionIntent {
    pub fn pointer_move(x: f32, y: f32, dx: f32, dy: f32, down: bool, button: i16, modifiers: &PointerModifiers) -> Self {
        Self { phase: WorldInteractionPhase::PointerMove, generation: 0, x, y, dx, dy, delta: 0.0, button, down, shift: modifiers.shift, ctrl: modifiers.ctrl, alt: modifiers.alt, meta: modifiers.meta }
    }

    pub fn pointer_button(x: f32, y: f32, down: bool, button: i16, modifiers: &PointerModifiers) -> Self {
        Self { phase: WorldInteractionPhase::PointerButton, generation: 0, x, y, dx: 0.0, dy: 0.0, delta: 0.0, button, down, shift: modifiers.shift, ctrl: modifiers.ctrl, alt: modifiers.alt, meta: modifiers.meta }
    }

    pub fn wheel(x: f32, y: f32, delta: f32, modifiers: &PointerModifiers) -> Self {
        Self { phase: WorldInteractionPhase::Wheel, generation: 0, x, y, dx: 0.0, dy: 0.0, delta, button: 0, down: false, shift: modifiers.shift, ctrl: modifiers.ctrl, alt: modifiers.alt, meta: modifiers.meta }
    }

    /// 🚪️ The DOM's `pointerout`: the pointer is no longer over THIS surface — it left its rect, or a
    /// chrome layer painted above it owns the point now. Carries no gesture, only the point it left
    /// at, and answers exactly one published hover with its clear. See
    /// [`plan_world3d_pointer_leave`].
    pub fn pointer_leave(x: f32, y: f32) -> Self {
        Self { phase: WorldInteractionPhase::PointerLeave, generation: 0, x, y, dx: 0.0, dy: 0.0, delta: 0.0, button: 0, down: false, shift: false, ctrl: false, alt: false, meta: false }
    }

    /// 🧯️ A host cancel (window blur, capture lost before the surface closes): a live gumball gesture that streamed aborts
    /// with zero trace and swallows the rest of its drag, an open paint stroke aborts; anything else is untouched.
    pub fn cancel(reason: WorldCancelReason) -> Self {
        Self { phase: WorldInteractionPhase::Cancel, generation: 0, x: 0.0, y: 0.0, dx: 0.0, dy: 0.0, delta: 0.0, button: reason as i16, down: false, shift: false, ctrl: false, alt: false, meta: false }
    }
}

pub struct WorldInteractionIntentQueue {
    pub(in crate::world) slots: Box<[Option<WorldInteractionIntent>; WORLD_INTERACTION_INTENT_CAPACITY]>,
    head: u8,
    pub(in crate::world) len: u8,
    pub(in crate::world) closing: bool,
}

impl Default for WorldInteractionIntentQueue {
    fn default() -> Self {
        Self { slots: Box::new([None; WORLD_INTERACTION_INTENT_CAPACITY]), head: 0, len: 0, closing: false }
    }
}

impl WorldInteractionIntentQueue {
    pub fn push(&mut self, intent: WorldInteractionIntent) -> Result<(), WorldInteractionIntent> {
        if self.closing || usize::from(self.len) == WORLD_INTERACTION_INTENT_CAPACITY {
            return Err(intent);
        }
        let index = (usize::from(self.head) + usize::from(self.len)) % WORLD_INTERACTION_INTENT_CAPACITY;
        self.slots[index] = Some(intent);
        self.len += 1;
        Ok(())
    }

    pub fn front(&self) -> Option<&WorldInteractionIntent> {
        self.slots[usize::from(self.head)].as_ref()
    }

    /// 🚪️ Whether a [`WorldInteractionPhase::PointerLeave`] is already waiting to be answered.
    pub(super) fn holds_pointer_leave(&self) -> bool {
        self.slots.iter().flatten().any(|intent| intent.phase == WorldInteractionPhase::PointerLeave)
    }

    /// 🫧 Folds a fresh pointer MOVE onto the one already waiting at the tail, answering the
    /// generation it keeps — the retained twin of React's own rAF batching.
    ///
    /// React never sees a move per DOM event either: `🚚️browser-frame-transport/🟦️.ts`'s
    /// `enqueueReplaceable` keeps ONE sample per pointer identity per animation frame, and
    /// `🌐️World3dHost/🟦️.tsx`'s hover dispatcher "keeps at most one outstanding and coalesces the
    /// rest onto the latest target". This queue had no such rung: every move the host drained became
    /// its own intent, so the probe's eight-move drag left `queued=10` in front of the authority and
    /// every one of them was answered with a full pick/plan turn
    /// (`🗑️generated/w12c-parity-run-19/wgpu/console.txt`, ticket 26/09/17 packet W13c §3).
    ///
    /// 🧭️ The fold is EXACT for gesture classification, which is the one thing it may not move:
    /// `dx`/`dy` accumulate (a drag plan reads the delta, so eight folded moves steer the camera by
    /// the same total as eight separate ones), `x`/`y` take the newest sample (a pick reads the
    /// point, and the newest point is the one React would have raycast), and the `down`/button/
    /// modifier state must MATCH — a change in any of them is a different gesture and starts a new
    /// intent. The FRONT is never folded into: it may already be half-stepped by a live cursor.
    pub(super) fn coalesce_tail_pointer_move(&mut self, intent: &WorldInteractionIntent) -> Option<u64> {
        if self.closing || intent.phase != WorldInteractionPhase::PointerMove || self.len < 2 {
            return None;
        }
        let index = (usize::from(self.head) + usize::from(self.len) - 1) % WORLD_INTERACTION_INTENT_CAPACITY;
        let tail = self.slots[index].as_mut()?;
        if tail.phase != WorldInteractionPhase::PointerMove || tail.down != intent.down || tail.button != intent.button || tail.shift != intent.shift || tail.ctrl != intent.ctrl || tail.alt != intent.alt || tail.meta != intent.meta {
            return None;
        }
        tail.x = intent.x;
        tail.y = intent.y;
        tail.dx += intent.dx;
        tail.dy += intent.dy;
        Some(tail.generation)
    }

    pub fn retire_front(&mut self, generation: u64) -> bool {
        let index = usize::from(self.head);
        let Some(intent) = self.slots[index] else {
            return false;
        };
        if intent.generation != generation {
            return false;
        }
        self.slots[index] = None;
        self.head = ((index + 1) % WORLD_INTERACTION_INTENT_CAPACITY) as u8;
        self.len -= 1;
        true
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn close_step(&mut self) -> bool {
        if self.len == 0 {
            return true;
        }
        let index = usize::from(self.head);
        self.slots[index] = None;
        self.head = ((index + 1) % WORLD_INTERACTION_INTENT_CAPACITY) as u8;
        self.len -= 1;
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.len == 0 && self.slots.iter().all(Option::is_none)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::world) enum WorldFlatActionKind {
    PaintAt,
    PaintCommit,
    /// 🧯️ `paintAt{phase:"abort", reason}` — a host cancel drops the open paint stroke with zero trace; `numbers[0]` is
    /// the [`WorldCancelReason`].
    PaintAbort,
    VortexSelect,
    VortexHover,
    SurfacePlace,
    Select,
    Hover,
    Camera,
    /// 🧭️ A navigation step that moves this pane's orbit and publishes NOTHING — see
    /// [`WorldCameraSync`]. The wire report is one debounced [`WorldFlatActionKind::Camera`] at the
    /// settle, never one per pointer move.
    CameraLocal,
    /// 🧭️ `noteWorldNavigation` — the classified gesture set of a completed navigation, published
    /// ahead of the settle's `setCamera` exactly as React's un-debounced `onNavigationGestures` lands
    /// ahead of its debounced `reportCamera`.
    Navigation,
    /// 🚚️ `worldRelocate` — the ONE absolute `{objectId, position}` a finished relocate drag commits.
    Relocate,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct WorldInteractionSpan {
    pub(in crate::world) start: u16,
    pub(in crate::world) len: u16,
}

#[derive(Clone, Copy, Debug)]
pub(in crate::world) struct WorldFlatAction {
    pub(in crate::world) kind: WorldFlatActionKind,
    pub(in crate::world) strings: [Option<WorldInteractionSpan>; 8],
    pub(in crate::world) numbers: [f64; 10],
}

pub struct WorldInteractionPlan {
    pub(in crate::world) revision: u64,
    pub(in crate::world) generation: u64,
    pub(in crate::world) bytes: Box<[u8; WORLD_INTERACTION_BYTE_CAPACITY]>,
    pub(in crate::world) byte_len: u16,
    pub(super) actions: Box<[Option<WorldFlatAction>; WORLD_INTERACTION_ITEM_CAPACITY]>,
    pub(super) action_len: u16,
    pub(in crate::world) cursor: u16,
    pub(in crate::world) faulted: bool,
}

#[expect(clippy::large_enum_variant, reason = "Interaction transitions retain fixed action claims and cursor storage without a fresh allocation between admitted steps.")]
pub(in crate::world) enum WorldInteractionActive {
    Plan { plan: WorldInteractionPlan, retirement: Option<WorldInteractionAuthorityStep> },
    Pick { cursor: WorldRayPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    ObjectPick { cursor: WorldObjectPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    ComponentPick { cursor: WorldComponentPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    MarqueePick { cursor: WorldMarqueePickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    MarqueePublish { job: WorldMarqueePublishJob, retirement: Option<WorldInteractionAuthorityStep> },
    ComponentMarqueePublish { job: WorldComponentMarqueePublishJob, retirement: Option<WorldInteractionAuthorityStep> },
    GumballPick { cursor: WorldGumballPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    GumballCommit { job: WorldGumballCommitJob, retirement: Option<WorldInteractionAuthorityStep> },
    BrushCommit { job: WorldBrushCommitJob, retirement: Option<WorldInteractionAuthorityStep> },
}

/// 🔎️ The active transition named together with the one fact that decides whether it will produce a
/// guest action — a pick's purpose and whether it found anything, a plan's pending action kind.
/// Used by {@link World3dState::interaction_census}.
pub(in crate::world) fn world_interaction_active_census(active: &WorldInteractionActive) -> String {
    match active {
        WorldInteractionActive::Plan { plan, .. } => format!("Plan[{}/{} {:?}]", plan.cursor, plan.action_len, plan.actions.get(usize::from(plan.cursor)).copied().flatten().map(|action| action.kind)),
        WorldInteractionActive::Pick { cursor, .. } => format!("Pick[{:?} hit={} done={}]", cursor.purpose, cursor.best.is_some(), cursor.complete),
        WorldInteractionActive::ObjectPick { cursor, .. } => format!("ObjectPick[{:?} hit={} done={}]", cursor.purpose, cursor.best.is_some(), cursor.complete),
        WorldInteractionActive::ComponentPick { cursor, .. } => format!("ComponentPick[{:?} hit={} done={}]", cursor.purpose, cursor.best.is_some(), cursor.complete),
        WorldInteractionActive::MarqueePick { .. } => "MarqueePick".to_string(),
        WorldInteractionActive::MarqueePublish { job, .. } => format!("MarqueePublish[page={} stage={} targets={}b]", job.page, job.stage, job.targets_len),
        WorldInteractionActive::ComponentMarqueePublish { .. } => "ComponentMarqueePublish".to_string(),
        WorldInteractionActive::GumballPick { .. } => "GumballPick".to_string(),
        WorldInteractionActive::GumballCommit { .. } => "GumballCommit".to_string(),
        WorldInteractionActive::BrushCommit { .. } => "BrushCommit".to_string(),
    }
}

pub(in crate::world) struct WorldInteractionAuthority {
    pub(in crate::world) queue: WorldInteractionIntentQueue,
    pub(in crate::world) blocked: Option<WorldInteractionIntent>,
    pub(in crate::world) registry: Option<WorldInteractionRegistryBuildCursor>,
    pub(in crate::world) active: Option<WorldInteractionActive>,
    pub(in crate::world) right_press: Option<[f32; 2]>,
    pub(super) right_dragged: bool,
    pub(in crate::world) marquee: Option<WorldMarqueeGesture>,
    pub(in crate::world) gumball: Option<WorldGumballGesture>,
    /// 🧯️ A cancelled live gumball drag: its remaining moves and its release are swallowed.
    pub(super) gumball_swallow: bool,
    pub(in crate::world) next_generation: u64,
    pub(in crate::world) faulted: bool,
    pub(in crate::world) closing: bool,
}

impl Default for WorldInteractionAuthority {
    fn default() -> Self {
        Self { queue: WorldInteractionIntentQueue::default(), blocked: None, registry: None, active: None, right_press: None, right_dragged: false, marquee: None, gumball: None, gumball_swallow: false, next_generation: 1, faulted: false, closing: false }
    }
}
//#endregion 🧵️WorldInteractionTransaction
