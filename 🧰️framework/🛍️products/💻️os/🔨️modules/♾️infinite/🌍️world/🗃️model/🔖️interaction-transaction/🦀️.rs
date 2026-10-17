//! 🔖️ World3d interaction transaction: capacities, ids, mesh and object registries, intents, plans and steps.

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
pub struct WorldInteractionId {
    pub bytes: [u8; WORLD_INTERACTION_ID_BYTE_CAPACITY],
    pub len: u16,
}

impl WorldInteractionId {
    pub fn new(value: &str) -> Option<Self> {
        if value.len() > WORLD_INTERACTION_ID_BYTE_CAPACITY {
            return None;
        }
        let mut bytes = [0; WORLD_INTERACTION_ID_BYTE_CAPACITY];
        bytes[..value.len()].copy_from_slice(value.as_bytes());
        Some(Self { bytes, len: value.len() as u16 })
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("world registry id originates from UTF-8")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldInteractionMeshToken {
    pub slot: u16,
    pub generation: u64,
}

#[derive(Clone, Copy)]
struct WorldInteractionMeshSlot {
    pub id: WorldInteractionId,
    pub generation: u64,
    pub version: u64,
    pub vertices: u32,
    pub triangles: u32,
    pub edges: u32,
    topology_bytes: u32,
}

pub struct WorldInteractionMeshRegistry {
    pub slots: Box<[Option<WorldInteractionMeshSlot>; WORLD_INTERACTION_MESH_CAPACITY]>,
    pub epochs: Box<[u64; WORLD_INTERACTION_MESH_CAPACITY]>,
    pub faulted: bool,
}

#[derive(Clone, Copy)]
struct WorldInteractionMeshPlan {
    pub slot: u16,
    pub epoch: u64,
    existing_generation: Option<u64>,
    pub value: WorldInteractionMeshSlot,
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
    pub fn hash(id: &str) -> usize {
        (id.as_bytes().iter().fold(2_166_136_261u64, |hash, byte| hash.wrapping_mul(16_777_619) ^ u64::from(*byte)) % WORLD_INTERACTION_MESH_CAPACITY as u64) as usize
    }

    pub fn plan_admit(&self, id: &str, version: u64, mesh: Mesh3dLease) -> Result<WorldInteractionMeshPlan, WorldDynamicFault> {
        let Some(id) = WorldInteractionId::new(id) else {
            return Err(WorldDynamicFault::IdCapacity);
        };
        self.plan_admit_id(id,version,mesh)
    }

    pub fn plan_admit_id(&self, id: WorldInteractionId, version: u64, mesh: Mesh3dLease) -> Result<WorldInteractionMeshPlan, WorldDynamicFault> {
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

    pub fn commit_admit(&mut self, plan: WorldInteractionMeshPlan) -> Result<WorldInteractionMeshToken, WorldDynamicFault> {
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

    #[cfg(any(test, feature = "testkit"))]
    pub fn admit(&mut self, id: &str, version: u64, mesh: Mesh3dLease) -> Option<WorldInteractionMeshToken> {
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

    pub fn resolve(&self, token: WorldInteractionMeshToken) -> Option<&WorldInteractionMeshSlot> {
        let slot = self.slots.get(usize::from(token.slot))?.as_ref()?;
        (slot.generation == token.generation).then_some(slot)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldInteractionObjectKind {
    Instance,
    Vortex,
    Reference,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldInteractionObjectToken {
    pub slot: u16,
    pub generation: u64,
    pub revision: u64,
}

#[derive(Clone, Copy)]
pub struct WorldInteractionObjectSlot {
    pub id: WorldInteractionId,
    pub kind: WorldInteractionObjectKind,
    pub generation: u64,
    pub revision: u64,
    pub mesh: Option<WorldInteractionMeshToken>,
    pub component_source: Option<ComponentSource3d>,
    pub model: Mat4,
    pub values: [f32; 8],
}

pub struct WorldInteractionObjectRegistry {
    pub slots: Box<[Option<WorldInteractionObjectSlot>; WORLD_INTERACTION_OBJECT_CAPACITY]>,
    pub occupied: [u64; WORLD_INTERACTION_OBJECT_CAPACITY.div_ceil(64)],
    pub epochs: Box<[u64; WORLD_INTERACTION_OBJECT_CAPACITY]>,
    pub instance_order: Box<[Option<WorldInteractionObjectToken>; WORLD_INTERACTION_OBJECT_CAPACITY]>,
    pub instance_len: u16,
    pub revision: u64,
    pub faulted: bool,
}

impl Default for WorldInteractionObjectRegistry {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), occupied: [0; WORLD_INTERACTION_OBJECT_CAPACITY.div_ceil(64)], epochs: semio_framework_async::boxed_fixed_slots(|| 0), instance_order: semio_framework_async::boxed_fixed_slots(|| None), instance_len: 0, revision: u64::MAX, faulted: false }
    }
}

impl WorldInteractionObjectRegistry {
    pub fn hash(kind: WorldInteractionObjectKind, id: &str) -> usize {
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
    pub fn admit(&mut self, revision: u64, kind: WorldInteractionObjectKind, id: &str, mesh: Option<WorldInteractionMeshToken>, model: Mat4, values: [f32; 8], component_source: Option<ComponentSource3d>) -> Option<WorldInteractionObjectToken> {
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
    pub fn next_occupied(&self, start: u16) -> Option<(usize, &WorldInteractionObjectSlot)> {
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

    pub fn resolve(&self, token: WorldInteractionObjectToken) -> Option<&WorldInteractionObjectSlot> {
        let slot = self.slots.get(usize::from(token.slot))?.as_ref()?;
        (slot.generation == token.generation && slot.revision == token.revision).then_some(slot)
    }

    pub fn begin_instance_order(&mut self) {
        self.instance_len = 0;
    }

    pub fn push_instance_order(&mut self, token: WorldInteractionObjectToken) -> bool {
        let index = usize::from(self.instance_len);
        if index == WORLD_INTERACTION_OBJECT_CAPACITY {
            self.faulted = true;
            return false;
        }
        self.instance_order[index] = Some(token);
        self.instance_len += 1;
        true
    }

    pub fn terminal_for_revision(&self, revision: u64) -> bool {
        self.revision == revision && !self.faulted
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldInteractionRegistryBuildPhase {
    Instances,
    Vortices,
    References,
    Complete,
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

    pub fn from_button(button: i16) -> Self {
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
    pub slots: Box<[Option<WorldInteractionIntent>; WORLD_INTERACTION_INTENT_CAPACITY]>,
    head: u8,
    pub len: u8,
    pub closing: bool,
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
    pub fn holds_pointer_leave(&self) -> bool {
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
    pub fn coalesce_tail_pointer_move(&mut self, intent: &WorldInteractionIntent) -> Option<u64> {
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
pub enum WorldFlatActionKind {
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
pub struct WorldInteractionSpan {
    pub start: u16,
    pub len: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct WorldFlatAction {
    pub kind: WorldFlatActionKind,
    pub strings: [Option<WorldInteractionSpan>; 8],
    pub numbers: [f64; 10],
}

pub struct WorldInteractionPlan {
    pub revision: u64,
    pub generation: u64,
    pub bytes: Box<[u8; WORLD_INTERACTION_BYTE_CAPACITY]>,
    pub byte_len: u16,
    pub actions: Box<[Option<WorldFlatAction>; WORLD_INTERACTION_ITEM_CAPACITY]>,
    pub action_len: u16,
    pub cursor: u16,
    pub faulted: bool,
}

pub const TRANSLATE_SELECTION_ACTION_ID: &str = "translateSelection";

pub const ROTATE_SELECTION_ACTION_ID: &str = "rotateSelection";

pub const SCALE_SELECTION_ACTION_ID: &str = "scaleSelection";

impl WorldInteractionPlan {
    pub fn new(revision: u64, generation: u64) -> Self {
        Self { revision, generation, bytes: Box::new([0; WORLD_INTERACTION_BYTE_CAPACITY]), byte_len: 0, actions: Box::new([None; WORLD_INTERACTION_ITEM_CAPACITY]), action_len: 0, cursor: 0, faulted: false }
    }

    pub fn push_string(&mut self, value: &str) -> Option<WorldInteractionSpan> {
        if value.len() > u16::MAX as usize {
            self.faulted = true;
            return None;
        }
        let start = usize::from(self.byte_len);
        let Some(end) = start.checked_add(value.len()).filter(|end| *end <= WORLD_INTERACTION_BYTE_CAPACITY) else {
            self.faulted = true;
            return None;
        };
        self.bytes[start..end].copy_from_slice(value.as_bytes());
        self.byte_len = end as u16;
        Some(WorldInteractionSpan { start: start as u16, len: value.len() as u16 })
    }

    fn push_joined(&mut self, parts: &[&str]) -> Option<WorldInteractionSpan> {
        let len = parts.iter().try_fold(0usize, |total, part| total.checked_add(part.len()))?;
        if len > u16::MAX as usize {
            self.faulted = true;
            return None;
        }
        let start = usize::from(self.byte_len);
        let Some(end) = start.checked_add(len).filter(|end| *end <= WORLD_INTERACTION_BYTE_CAPACITY) else {
            self.faulted = true;
            return None;
        };
        let mut cursor = start;
        for part in parts {
            let next = cursor + part.len();
            self.bytes[cursor..next].copy_from_slice(part.as_bytes());
            cursor = next;
        }
        self.byte_len = end as u16;
        Some(WorldInteractionSpan { start: start as u16, len: len as u16 })
    }

    pub fn push_action(&mut self, action: WorldFlatAction) -> bool {
        let index = usize::from(self.action_len);
        if index == WORLD_INTERACTION_ITEM_CAPACITY {
            self.faulted = true;
            return false;
        }
        self.actions[index] = Some(action);
        self.action_len += 1;
        true
    }

    pub fn string(&self, span: WorldInteractionSpan) -> &str {
        let start = usize::from(span.start);
        let end = start + usize::from(span.len);
        std::str::from_utf8(&self.bytes[start..end]).expect("world interaction strings originate from UTF-8")
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.cursor == self.action_len && self.byte_len == 0
    }

    pub fn close_step(&mut self) -> bool {
        if self.cursor < self.action_len {
            self.actions[usize::from(self.cursor)] = None;
            self.cursor += 1;
            return false;
        }
        self.byte_len = 0;
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldInteractionStep {
    Pending,
    Complete,
    Stale,
    Fault,
}
//#endregion 🧵️WorldInteractionTransaction
