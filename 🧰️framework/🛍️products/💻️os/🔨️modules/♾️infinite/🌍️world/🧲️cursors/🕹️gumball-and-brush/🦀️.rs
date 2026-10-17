//! 🕹️ World3d component and gumball targets, the gumball pick cursor, gestures and commit jobs, and the brush commit job.

use super::*;

//#region 🧵️WorldInteractionTransaction
pub struct WorldComponentTarget {
    pub bytes: [u8; WORLD_COMPONENT_TARGET_BYTE_CAPACITY],
    pub len: usize,
}

impl std::fmt::Write for WorldComponentTarget {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        let end = self.len.checked_add(text.len()).filter(|end| *end <= self.bytes.len()).ok_or(std::fmt::Error)?;
        self.bytes[self.len..end].copy_from_slice(text.as_bytes()); self.len = end; Ok(())
    }
}

impl WorldComponentTarget {
    pub fn new(interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, entry: &WorldInteractionObjectSlot, kind: WorldComponentKind, group: u32) -> Result<Self, ui_wgpu::wgpu::BoundedActionFault> {
        use std::fmt::Write;
        let mut target = Self { bytes: [0; WORLD_COMPONENT_TARGET_BYTE_CAPACITY], len: 0 };
        write!(target, "{}.{}.{}", entry.id.as_str(), kind.as_str(), group).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::StringCredits)?;
        if let Some(source) = entry.component_source {
            let admitted = entry.mesh.and_then(|mesh| interaction_state.interaction_meshes.resolve(mesh)).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            let mesh = scene_state.meshes.get(admitted.id.as_str()).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            let field = match kind { WorldComponentKind::Vertex => Mesh3dField::VertexIds, WorldComponentKind::Edge => Mesh3dField::EdgeIds, WorldComponentKind::Face => Mesh3dField::FaceIds };
            let label = mesh.component_label(field, group).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?.ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
            write!(target, "~{}~{}~{}", source.handle(), label, source.revision()).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::StringCredits)?;
        }
        Ok(target)
    }

    pub fn as_str(&self) -> &str { std::str::from_utf8(&self.bytes[..self.len]).expect("component target retains valid UTF-8") }
}

impl WorldComponentKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vertex => "vertex",
            Self::Edge => "edge",
            Self::Face => "face",
        }
    }
}

pub const WORLD_GUMBALL_SELECTED_CAPACITY: usize = ui_wgpu::wgpu::ACTION_NODE_CAPACITY - 11;

#[derive(Clone, Copy)]
pub struct WorldGumballTarget {
    pub object: WorldInteractionObjectToken,
    source_index: Option<u32>,
    captured: [u8; WORLD_COMPONENT_TARGET_BYTE_CAPACITY],
    captured_len: u16,
}

impl WorldGumballTarget {
    pub fn new(object: WorldInteractionObjectToken, source_index: Option<u32>, text: &str) -> Option<Self> {
        if text.len() > WORLD_COMPONENT_TARGET_BYTE_CAPACITY { return None; }
        let mut target = Self { object, source_index, captured: [0; WORLD_COMPONENT_TARGET_BYTE_CAPACITY], captured_len: u16::try_from(text.len()).ok()? };
        target.captured[..text.len()].copy_from_slice(text.as_bytes());
        Some(target)
    }

    fn captured_text(&self) -> &str { std::str::from_utf8(&self.captured[..usize::from(self.captured_len)]).expect("captured gumball target is valid UTF-8") }

    pub fn text<'a>(&self, gumball_state: &'a WorldGumballState, interaction_state: &'a WorldInteractionState, scene_state: &'a WorldSceneState, mode: Option<WorldComponentKind>) -> Option<&'a str> {
        let entry = interaction_state.interaction_objects.resolve(self.object)?;
        match (self.source_index, mode) {
            (None, None) => Some(entry.id.as_str()),
            (Some(index), Some(kind)) => {
                let id = gumball_state.gumball_selection_ids.get(index as usize)?.as_str();
                let address = WorldComponentAddress::parse(id, kind.as_str())?;
                (address.object == entry.id.as_str() && address.matches_registry(interaction_state, scene_state, entry)).then_some(id)
            }
            _ => None,
        }
    }
}

pub struct WorldGumballPickCursor {
    pub revision: u64,
    pub generation: u64,
    pub x: f32,
    pub y: f32,
    pub slot: u16,
    source_cursor: usize,
    source_probe: u16,
    pub mode: Option<WorldComponentKind>,
    pub selected: Box<[Option<WorldGumballTarget>; WORLD_GUMBALL_SELECTED_CAPACITY]>,
    pub selected_len: u16,
    pub selected_bytes: u16,
    pub sum: Vec3,
    pub pivot: Option<Vec3>,
    pub origin: Vec3,
    pub direction: Vec3,
    pub eye: Vec3,
    pub extent: f32,
    pub handle: u8,
    validate: u16,
    pub best: Option<(f32, GumballHandle)>,
    pub complete: bool,
    pub faulted: bool,
}

pub struct WorldGumballGesture {
    pub revision: u64,
    pub start_generation: u64,
    pub handle: GumballHandle,
    pub pivot: Vec3,
    pub anchor: f32,
    pub start: Vec3,
    pub translate: Vec3,
    pub angle: f32,
    pub scale: Vec3,
    pub mode: Option<WorldComponentKind>,
    pub selected: Box<[Option<WorldGumballTarget>; WORLD_GUMBALL_SELECTED_CAPACITY]>,
    pub selected_len: u16,
    pub selected_bytes: u16,
    pub validation: u16,
    pub pending: Option<WorldGumballUpdate>,
    pub live: bool,
    pub streamed: bool,
    pub sent_translate: Vec3,
    pub sent_angle: f32,
    pub sent_scale: Vec3,
}

#[derive(Clone, Copy)]
struct WorldGumballUpdate {
    pub generation: u64,
    pub x: f32,
    pub y: f32,
}

/// 🎚️ The verb one gumball handle commits as — the single handle→action mapping, read by the pick
/// (which never offers a handle whose verb the window kind does not declare) and by the commit job
/// that mints it.
pub fn gumball_handle_action_id(handle: GumballHandle) -> &'static str {
    if handle.is_translate() {
        TRANSLATE_SELECTION_ACTION_ID
    } else if handle.is_rotate() {
        ROTATE_SELECTION_ACTION_ID
    } else {
        SCALE_SELECTION_ACTION_ID
    }
}

impl WorldGumballPickCursor {
    pub fn new(session_state: &WorldSessionState, selection_state: &WorldSelectionState, generation: u64, x: f32, y: f32) -> Self {
        Self {
            revision: session_state.interaction_revision,
            generation,
            x,
            y,
            slot: 0,
            source_cursor: 0,
            source_probe: 0,
            mode: match selection_state.granularity.as_str() { "vertex" => Some(WorldComponentKind::Vertex), "edge" => Some(WorldComponentKind::Edge), "face" | "component" => Some(WorldComponentKind::Face), _ => None },
            selected: Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY]),
            selected_len: 0,
            selected_bytes: 0,
            sum: Vec3::ZERO,
            pivot: None,
            origin: Vec3::ZERO,
            direction: Vec3::ZERO,
            eye: Vec3::ZERO,
            extent: 0.0,
            handle: 0,
            validate: 0,
            best: None,
            complete: false,
            faulted: false,
        }
    }

    pub fn retain(&mut self, target: WorldGumballTarget, bytes: usize, position: Vec3) -> bool {
        let index = usize::from(self.selected_len);
        let Some(bytes) = usize::from(self.selected_bytes).checked_add(bytes).filter(|bytes| *bytes <= WORLD_INTERACTION_BYTE_CAPACITY) else { return false; };
        let Some(slot) = self.selected.get_mut(index) else { return false; };
        *slot = Some(target);
        self.selected_bytes = bytes as u16;
        self.selected_len += 1;
        self.sum = self.sum.add(position);
        true
    }

    /// 🎛️ ONE gate for every handle: the resolved `gumballConfig` (authored, else the mode's
    /// fallback) ∩ the drafting plane — React's `gumballHandleEnabled`. A handle the gumball does
    /// not SHOW must not be pickable either.
    pub fn step(&mut self, session_state: &WorldSessionState, camera_state: &WorldCameraState, gumball_state: &WorldGumballState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.faulted {
            return WorldInteractionStep::Fault;
        }
        if self.revision != session_state.interaction_revision || self.generation != generation || !interaction_state.interaction_objects.terminal_for_revision(self.revision) {
            return WorldInteractionStep::Stale;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        if self.pivot.is_none() {
            if let Some(kind) = self.mode {
                if let Some(id) = gumball_state.gumball_selection_ids.get(self.source_cursor) {
                    let Some(address) = WorldComponentAddress::parse(id, kind.as_str()) else {
                        self.faulted = true;
                        return WorldInteractionStep::Fault;
                    };
                    if usize::from(self.source_probe) == WORLD_INTERACTION_OBJECT_CAPACITY {
                        self.faulted = true;
                        return WorldInteractionStep::Fault;
                    }
                    let index = (WorldInteractionObjectRegistry::hash(WorldInteractionObjectKind::Instance, address.object) + usize::from(self.source_probe)) % WORLD_INTERACTION_OBJECT_CAPACITY;
                    self.source_probe += 1;
                    if let Some(entry) = interaction_state.interaction_objects.slots[index].as_ref().filter(|entry| entry.revision == self.revision && entry.kind == WorldInteractionObjectKind::Instance && entry.id.as_str() == address.object) {
                        let Some(source_index) = u32::try_from(self.source_cursor).ok() else { self.faulted = true; return WorldInteractionStep::Fault; };
                        let Some(target) = WorldGumballTarget::new(WorldInteractionObjectToken { slot: index as u16, generation: entry.generation, revision: entry.revision }, Some(source_index), id) else { self.faulted = true; return WorldInteractionStep::Fault; };
                        if !address.matches_registry(interaction_state, scene_state, entry) || !self.retain(target, id.len(), Vec3::new(entry.values[3], entry.values[4], entry.values[5])) {
                            self.faulted = true;
                            return WorldInteractionStep::Fault;
                        }
                        self.source_cursor += 1;
                        self.source_probe = 0;
                    }
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                }
            } else {
                let index = usize::from(self.slot);
                if let Some(entry) = interaction_state.interaction_objects.slots.get(index) {
                    self.slot += 1;
                    if let Some(entry) = entry.as_ref().filter(|entry| entry.revision == self.revision && entry.kind == WorldInteractionObjectKind::Instance && entry.values[2] != 0.0) {
                        let Some(target) = WorldGumballTarget::new(WorldInteractionObjectToken { slot: index as u16, generation: entry.generation, revision: entry.revision }, None, entry.id.as_str()) else { self.faulted = true; return WorldInteractionStep::Fault; };
                        if !self.retain(target, entry.id.as_str().len(), Vec3::new(entry.values[3], entry.values[4], entry.values[5])) {
                            self.faulted = true;
                            return WorldInteractionStep::Fault;
                        }
                    }
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                }
            }
            if self.selected_len == 0 {
                self.complete = true;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            let pivot = gumball_state.gumball_target.map_or_else(|| self.sum.scale(1.0 / f32::from(self.selected_len)), |target| Vec3::new(target[0], target[1], target[2]));
            let Some((local_x, local_y, viewport)) = pointer_in_pick_rect(session_state, self.x, self.y) else {
                self.complete = true;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            };
            let camera = camera_state.orbit.to_camera();
            let (origin, direction) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
            self.pivot = Some(pivot);
            self.origin = origin;
            self.direction = direction;
            self.eye = gumball_eye(&camera, pivot);
            self.extent = gumball_extent(camera.position.sub(pivot).length());
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let handles = [
            GumballHandle::MoveX,
            GumballHandle::MoveY,
            GumballHandle::MoveZ,
            GumballHandle::MoveXY,
            GumballHandle::MoveYZ,
            GumballHandle::MoveXZ,
            GumballHandle::RotateX,
            GumballHandle::RotateY,
            GumballHandle::RotateZ,
            GumballHandle::ScaleX,
            GumballHandle::ScaleY,
            GumballHandle::ScaleZ,
        ];
        let Some(handle) = handles.get(usize::from(self.handle)).copied() else {
            if self.validate < self.selected_len {
                let target = self.selected[usize::from(self.validate)].expect("gumball selected target");
                self.validate += 1;
                if target.text(gumball_state, interaction_state, scene_state, self.mode).is_none() {
                    return WorldInteractionStep::Stale;
                }
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            self.complete = true;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        self.handle += 1;
        if !world3d_declares_action(session_state, gumball_handle_action_id(handle)) {
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        if !world3d_gumball_config(gumball_state).admits(handle) {
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let pivot = self.pivot.expect("gumball pivot resolved");
        let pick_radius = self.extent * 0.08;
        let candidate = if handle.is_translate() && handle.axis_dir().is_some() {
            let axis = handle.axis_dir().expect("translation axis");
            ray_segment_distance(self.origin, self.direction, pivot, pivot.add(axis.scale(self.extent))).filter(|distance| *distance <= pick_radius)
        } else if matches!(handle, GumballHandle::MoveXY | GumballHandle::MoveYZ | GumballHandle::MoveXZ) {
            let normal = handle.plane_normal().expect("translation plane");
            ray_plane_point(self.origin, self.direction, pivot, normal).and_then(|hit| {
                let offset = hit.sub(pivot);
                let u = if normal.z.abs() > 0.9 {
                    offset.x.abs()
                } else if normal.x.abs() > 0.9 {
                    offset.y.abs()
                } else {
                    offset.x.abs()
                };
                let v = if normal.z.abs() > 0.9 { offset.y.abs() } else { offset.z.abs() };
                (u <= self.extent * 0.35 && v <= self.extent * 0.35).then_some(self.origin.sub(hit).length())
            })
        } else if handle.is_rotate() {
            let normal = handle.plane_normal().expect("rotation plane");
            ray_plane_point(self.origin, self.direction, pivot, normal).and_then(|hit| {
                let distance = (hit.sub(pivot).length() - self.extent * 0.85).abs();
                (distance <= pick_radius * 2.0).then_some(distance)
            })
        } else if handle.is_scale() {
            let axis = handle.axis_dir().expect("scale axis");
            ray_segment_distance(self.origin, self.direction, pivot, pivot.add(axis.scale(self.extent * 1.1))).filter(|distance| *distance <= pick_radius)
        } else {
            None
        };
        if let Some(distance) = candidate {
            if self.best.is_none_or(|(best, _)| distance < best) {
                self.best = Some((distance, handle));
            }
        }
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected transfer returns the exact admitted owner for bounded retirement without allocating on the failure path."))]
    pub fn finish(mut self, session_state: &WorldSessionState, gumball_state: &WorldGumballState, generation: u64) -> Result<Option<WorldGumballGesture>, (Self, WorldInteractionStep)> {
        if !self.complete {
            return Err((self, WorldInteractionStep::Pending));
        }
        if self.revision != session_state.interaction_revision || self.generation != generation {
            return Err((self, WorldInteractionStep::Stale));
        }
        let Some((_, handle)) = self.best else {
            return Ok(None);
        };
        let pivot = self.pivot.expect("gumball terminal pivot");
        let anchor = handle.axis_dir().and_then(|axis| gumball_project_ray_onto_axis(self.origin, self.direction, pivot, axis, self.eye)).unwrap_or(0.0);
        let start = handle.plane_normal().and_then(|normal| ray_plane_point(self.origin, self.direction, pivot, normal)).map_or(pivot, |point| if handle.is_rotate() { point.sub(pivot) } else { point });
        Ok(Some(WorldGumballGesture {
            revision: self.revision,
            start_generation: self.generation,
            handle,
            pivot,
            anchor,
            start,
            translate: Vec3::ZERO,
            angle: 0.0,
            scale: Vec3::new(1.0, 1.0, 1.0),
            mode: self.mode,
            selected: std::mem::replace(&mut self.selected, Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY])),
            selected_len: self.selected_len,
            selected_bytes: self.selected_bytes,
            validation: 0,
            pending: None,
            live: gumball_state.gumball_live_dispatch,
            streamed: false,
            sent_translate: Vec3::ZERO,
            sent_angle: 0.0,
            sent_scale: Vec3::new(1.0, 1.0, 1.0),
        }))
    }

    pub fn close_step(&mut self) -> bool {
        if self.best.take().is_some() {
            return false;
        }
        if self.selected_len > 0 {
            self.selected_len -= 1;
            self.selected[usize::from(self.selected_len)] = None;
            return false;
        }
        self.selected_bytes = 0;
        self.complete = true;
        true
    }
}

/// 🛠️ Where one gumball publication sits in the tool protocol — the wgpu twin of `🌐️World3dHost` `worldGumballStep`
/// (`🧫️fixtures/🛠️gumball-live-protocol.json`): `Skip` publishes nothing (a local drag that never moved along its handle),
/// `Once` is a local gesture's ONE net delta, `Stream`/`Commit` are a live gesture's ticks and tail, `Abort` drops a live
/// gesture that streamed with zero trace.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldGumballPhase {
    Skip,
    Once,
    Stream,
    Commit,
    Abort(WorldCancelReason),
}

impl WorldGumballPhase {
    /// 🏷️ The `phase` (and an abort's `reason`) a publication carries after its motion.
    pub fn tail(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Skip | Self::Once => &[],
            Self::Stream => &[("phase", "stream")],
            Self::Commit => &[("phase", "commit")],
            Self::Abort(WorldCancelReason::Blur) => &[("phase", "abort"), ("reason", "blur")],
            Self::Abort(WorldCancelReason::CaptureLost) => &[("phase", "abort"), ("reason", "captureLost")],
        }
    }
}

/// 🎚️ The protocol's motion tolerance: a pose delta below it along the handle moves nothing.
const GUMBALL_MOTION_EPSILON: f32 = 1e-6;

/// 🧮️ One gumball motion — offset, angle about the handle's axis, factors.
type WorldGumballMotion = (Vec3, f32, Vec3);

/// 🫥️ The motion that moves nothing: what a live tail that moved nothing commits and what an abort names.
pub fn world_gumball_identity_motion() -> WorldGumballMotion {
    (Vec3::ZERO, 0.0, Vec3::new(1.0, 1.0, 1.0))
}

impl WorldGumballGesture {
    /// 🧮️ The motion still owed to the guest — from the pose the last streamed tick reached to the current one: offsets
    /// subtract, angles subtract, factors divide (a local gesture never streams, so it owes its whole motion).
    pub fn owed(&self) -> WorldGumballMotion {
        let ratio = |current: f32, sent: f32| current / if sent.abs() > GUMBALL_MOTION_EPSILON { sent } else { GUMBALL_MOTION_EPSILON };
        (self.translate.sub(self.sent_translate), self.angle - self.sent_angle, Vec3::new(ratio(self.scale.x, self.sent_scale.x), ratio(self.scale.y, self.sent_scale.y), ratio(self.scale.z, self.sent_scale.z)))
    }

    /// ⚖️ Whether `motion` moves along this gesture's handle, beyond [`GUMBALL_MOTION_EPSILON`].
    fn moves(&self, (translate, angle, scale): WorldGumballMotion) -> bool {
        if self.handle.is_translate() {
            [translate.x, translate.y, translate.z].iter().any(|value| value.abs() >= GUMBALL_MOTION_EPSILON)
        } else if self.handle.is_rotate() {
            angle.abs() >= GUMBALL_MOTION_EPSILON
        } else {
            [scale.x, scale.y, scale.z].iter().any(|value| (value - 1.0).abs() >= GUMBALL_MOTION_EPSILON)
        }
    }

    /// 🌊️ After a completed pose update: a live gesture owes a `Stream` tick of its owed motion, which it marks sent;
    /// a local gesture or a pose that moved nothing owes nothing.
    pub fn stream(&mut self) -> Option<(WorldGumballPhase, WorldGumballMotion)> {
        let owed = self.owed();
        if !self.live || self.selected_len == 0 || !self.moves(owed) {
            return None;
        }
        self.sent_translate = self.translate;
        self.sent_angle = self.angle;
        self.sent_scale = self.scale;
        self.streamed = true;
        Some((WorldGumballPhase::Stream, owed))
    }

    /// 🏁️ The release: a live gesture commits its owed tail (the identity when the tail moved nothing), a local gesture
    /// publishes its ONE net delta, and a gesture that grabbed nothing or never moved along its handle publishes nothing.
    pub fn release(&self) -> (WorldGumballPhase, WorldGumballMotion) {
        let owed = self.owed();
        match (self.selected_len > 0, self.live, self.moves(owed)) {
            (true, true, true) => (WorldGumballPhase::Commit, owed),
            (true, true, false) => (WorldGumballPhase::Commit, world_gumball_identity_motion()),
            (true, false, true) => (WorldGumballPhase::Once, owed),
            _ => (WorldGumballPhase::Skip, owed),
        }
    }

    /// 🧯️ A host cancel: a live gesture that streamed aborts with the identity motion; anything else is untouched.
    pub fn cancel(&self, reason: WorldCancelReason) -> Option<(WorldGumballPhase, WorldGumballMotion)> {
        (self.live && self.streamed).then(|| (WorldGumballPhase::Abort(reason), world_gumball_identity_motion()))
    }

    /// 📨️ This gesture's handle and targets carrying `motion` — what a publication job publishes.
    pub fn publication(&self, (translate, angle, scale): WorldGumballMotion) -> Self {
        Self {
            revision: self.revision,
            start_generation: self.start_generation,
            handle: self.handle,
            pivot: self.pivot,
            anchor: self.anchor,
            start: self.start,
            translate,
            angle,
            scale,
            mode: self.mode,
            selected: self.selected.clone(),
            selected_len: self.selected_len,
            selected_bytes: self.selected_bytes,
            validation: 0,
            pending: None,
            live: self.live,
            streamed: self.streamed,
            sent_translate: self.sent_translate,
            sent_angle: self.sent_angle,
            sent_scale: self.sent_scale,
        }
    }

    pub fn begin_update(&mut self, generation: u64, x: f32, y: f32) -> WorldInteractionStep {
        if generation <= self.start_generation {
            return WorldInteractionStep::Stale;
        }
        match self.pending {
            None => {
                self.pending = Some(WorldGumballUpdate { generation, x, y });
                self.validation = 0;
                WorldInteractionStep::Pending
            }
            Some(pending) if pending.generation == generation && pending.x == x && pending.y == y => WorldInteractionStep::Pending,
            Some(_) => WorldInteractionStep::Fault,
        }
    }

    pub fn update_step(&mut self, session_state: &WorldSessionState, camera_state: &WorldCameraState, gumball_state: &WorldGumballState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState) -> WorldInteractionStep {
        if self.revision != session_state.interaction_revision {
            return WorldInteractionStep::Stale;
        }
        let Some(pending) = self.pending else {
            return WorldInteractionStep::Fault;
        };
        if self.validation < self.selected_len {
            let target = self.selected[usize::from(self.validation)].expect("gumball selected target");
            self.validation += 1;
            return if self.live && self.streamed || target.text(gumball_state, interaction_state, scene_state, self.mode).is_some() { WorldInteractionStep::Pending } else { WorldInteractionStep::Stale };
        }
        let Some((local_x, local_y, viewport)) = pointer_in_pick_rect(session_state, pending.x, pending.y) else {
            self.pending = None;
            self.validation = 0;
            return WorldInteractionStep::Pending;
        };
        let camera = camera_state.orbit.to_camera();
        let (origin, direction) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
        let eye = gumball_eye(&camera, self.pivot);
        self.translate = Vec3::ZERO;
        self.angle = 0.0;
        self.scale = Vec3::new(1.0, 1.0, 1.0);
        if self.handle.is_translate() {
            if let Some(axis) = self.handle.axis_dir() {
                if let Some(current) = gumball_project_ray_onto_axis(origin, direction, self.pivot, axis, eye) {
                    self.translate = axis.normalize().scale(current - self.anchor);
                }
            } else if let Some(normal) = self.handle.plane_normal() {
                if let Some(current) = ray_plane_point(origin, direction, self.pivot, normal) {
                    self.translate = current.sub(self.start);
                }
            }
        } else if self.handle.is_rotate() {
            if let Some(normal) = self.handle.plane_normal() {
                if let Some(current) = ray_plane_point(origin, direction, self.pivot, normal) {
                    self.angle = axis_rotate_angle(self.start, current.sub(self.pivot), normal);
                }
            }
        } else if let Some(axis) = self.handle.axis_dir() {
            if let Some(current) = gumball_project_ray_onto_axis(origin, direction, self.pivot, axis, eye) {
                let factor = if self.anchor.abs() > 1e-4 { (current / self.anchor).clamp(0.05, 20.0) } else { 1.0 };
                match self.handle {
                    GumballHandle::ScaleX => self.scale.x = factor,
                    GumballHandle::ScaleY => self.scale.y = factor,
                    GumballHandle::ScaleZ => self.scale.z = factor,
                    _ => {}
                }
            }
        }
        self.pending = None;
        self.validation = 0;
        WorldInteractionStep::Complete
    }

    pub fn close_step(&mut self) -> bool {
        if self.pending.take().is_some() {
            self.validation = 0;
            return false;
        }
        if self.selected_len > 0 {
            self.selected_len -= 1;
            self.selected[usize::from(self.selected_len)] = None;
            return false;
        }
        self.selected_bytes = 0;
        true
    }
}

pub struct WorldGumballCommitJob {
    pub generation: u64,
    pub gesture: WorldGumballGesture,
    pub phase: WorldGumballPhase,
    pub claim: Option<ui_wgpu::wgpu::BoundedActionClaim>,
    pub draft: Option<ui_wgpu::wgpu::BoundedClaimedActionDraft>,
    pub stage: u16,
    pub complete: bool,
}

pub const WORLD_BRUSH_COPY_CHUNK_BYTES: usize = 256;

#[derive(Clone, Copy)]
enum WorldBrushScale {
    Null,
    Number(f64),
    Array { values: [f64; 3], len: u8 },
}

pub struct WorldBrushCommitJob {
    pub generation: u64,
    pub revision: u64,
    pub bytes: Box<[u8; WORLD_INTERACTION_BYTE_CAPACITY]>,
    pub target: WorldInteractionSpan,
    pub kind: WorldInteractionSpan,
    source_index: usize,
    pub origin: [f64; 3],
    pub orientation: [f64; 4],
    pub scale: WorldBrushScale,
    copy_field: u8,
    copy_offset: u16,
    pub validating: bool,
    pub claim: Option<ui_wgpu::wgpu::BoundedActionClaim>,
    pub draft: Option<ui_wgpu::wgpu::BoundedClaimedActionDraft>,
    pub stage: u8,
    scale_stage: u8,
    pub complete: bool,
}

impl WorldBrushCommitJob {
    pub fn new(session_state: &WorldSessionState, gumball_state: &WorldGumballState, generation: u64) -> Result<Option<Self>, ui_wgpu::wgpu::BoundedActionFault> {
        let Some(preview) = gumball_state.brush_preview.as_ref() else {
            return Ok(None);
        };
        let (Some(target), Some(kind), Some(source_index)) = (preview.target_vortex_full_id.as_deref(), preview.object_kind_id.as_deref(), preview.source_vortex_index) else {
            return Ok(None);
        };
        if source_index > u32::MAX as usize {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        if target.len() > ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY || kind.len() > ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::StringCredits);
        }
        let kind_start = target.len();
        kind_start.checked_add(kind.len()).filter(|bytes| *bytes <= WORLD_INTERACTION_BYTE_CAPACITY).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        let scale = match preview.scale.as_ref() {
            None | Some(serde_json::Value::Null) => WorldBrushScale::Null,
            Some(serde_json::Value::Number(value)) => WorldBrushScale::Number(value.as_f64().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?),
            Some(serde_json::Value::Array(values)) if values.len() <= 3 => {
                let mut scalars = [0.0; 3];
                for index in 0..values.len() {
                    scalars[index] = values[index].as_f64().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
                }
                WorldBrushScale::Array { values: scalars, len: values.len() as u8 }
            }
            _ => return Err(ui_wgpu::wgpu::BoundedActionFault::Structure),
        };
        Ok(Some(Self {
            generation,
            revision: session_state.interaction_revision,
            bytes: Box::new([0; WORLD_INTERACTION_BYTE_CAPACITY]),
            target: WorldInteractionSpan { start: 0, len: target.len() as u16 },
            kind: WorldInteractionSpan { start: kind_start as u16, len: kind.len() as u16 },
            source_index,
            origin: preview.origin.unwrap_or([0.0, 0.0, 0.0]),
            orientation: preview.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
            scale,
            copy_field: 0,
            copy_offset: 0,
            validating: false,
            claim: None,
            draft: None,
            stage: 0,
            scale_stage: 0,
            complete: false,
        }))
    }

    pub fn text(&self, span: WorldInteractionSpan) -> Result<&str, ui_wgpu::wgpu::BoundedActionFault> {
        let start = usize::from(span.start);
        let end = start.checked_add(usize::from(span.len)).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        std::str::from_utf8(self.bytes.get(start..end).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)
    }

    pub fn source<'a>(&self, gumball_state: &'a WorldGumballState) -> Result<(&'a str, &'a str), ui_wgpu::wgpu::BoundedActionFault> {
        let preview = gumball_state.brush_preview.as_ref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let target = preview.target_vortex_full_id.as_deref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let kind = preview.object_kind_id.as_deref().ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        if target.len() != usize::from(self.target.len) || kind.len() != usize::from(self.kind.len) || preview.source_vortex_index != Some(self.source_index) {
            return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
        }
        Ok((target, kind))
    }

    fn copy_step(&mut self, gumball_state: &WorldGumballState) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
        let (target, kind) = self.source(gumball_state)?;
        let (source, span) = if self.copy_field == 0 { (target.as_bytes(), self.target) } else { (kind.as_bytes(), self.kind) };
        let offset = usize::from(self.copy_offset);
        if offset == source.len() {
            self.copy_field += 1;
            self.copy_offset = 0;
            if self.copy_field == 2 {
                if self.validating {
                    return Ok(true);
                }
                self.validating = true;
                self.copy_field = 0;
            }
            return Ok(false);
        }
        let take = (source.len() - offset).min(WORLD_BRUSH_COPY_CHUNK_BYTES);
        let destination = usize::from(span.start) + offset;
        let existing = &mut self.bytes[destination..destination + take];
        if self.validating {
            if existing != &source[offset..offset + take] {
                return Err(ui_wgpu::wgpu::BoundedActionFault::Structure);
            }
        } else {
            existing.copy_from_slice(&source[offset..offset + take]);
        }
        self.copy_offset += take as u16;
        Ok(false)
    }

    pub fn credit(&self, session_state: &WorldSessionState) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
        ui_wgpu::wgpu::checked_action_string_bytes(&[&session_state.controller_id, "addBrushObject", "targetVortexFullId", self.text(self.target)?, "objectKindId", self.text(self.kind)?, "sourceVortexIndex", "origin", "orientation", "scale"])
    }

    pub fn step(&mut self, session_state: &WorldSessionState, gumball_state: &WorldGumballState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> Result<WorldInteractionStep, ui_wgpu::wgpu::BoundedActionFault> {
        if context.should_yield() {
            return Ok(WorldInteractionStep::Pending);
        }
        if self.revision != session_state.interaction_revision || self.generation != generation {
            return Ok(WorldInteractionStep::Stale);
        }
        if !self.complete {
            self.complete = self.copy_step(gumball_state)?;
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if self.claim.is_none() {
            self.claim = Some(input.claim_action(self.credit(session_state)?)?);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if self.draft.is_none() {
            self.draft = Some(input.draft_claimed_action(self.claim.expect("brush claim"), &session_state.controller_id, "addBrushObject")?);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        let target_start = usize::from(self.target.start);
        let target_end = target_start + usize::from(self.target.len);
        let kind_start = usize::from(self.kind.start);
        let kind_end = kind_start + usize::from(self.kind.len);
        let target = std::str::from_utf8(&self.bytes[target_start..target_end]).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let kind = std::str::from_utf8(&self.bytes[kind_start..kind_end]).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::Structure)?;
        let draft = self.draft.as_mut().expect("brush draft");
        match self.stage {
            0 => draft.builder().begin_object(None)?,
            1 => draft.builder().string(Some("targetVortexFullId"), target)?,
            2 => draft.builder().string(Some("objectKindId"), kind)?,
            3 => draft.builder().number(Some("sourceVortexIndex"), self.source_index as f64)?,
            4 => draft.builder().begin_array(Some("origin"))?,
            5..=7 => draft.builder().number(None, self.origin[usize::from(self.stage - 5)])?,
            8 => draft.builder().end_container()?,
            9 => draft.builder().begin_array(Some("orientation"))?,
            10..=13 => draft.builder().number(None, self.orientation[usize::from(self.stage - 10)])?,
            14 => draft.builder().end_container()?,
            15 => match self.scale {
                WorldBrushScale::Null => draft.builder().null(Some("scale"))?,
                WorldBrushScale::Number(value) => draft.builder().number(Some("scale"), value)?,
                WorldBrushScale::Array { .. } => draft.builder().begin_array(Some("scale"))?,
            },
            16 if matches!(self.scale, WorldBrushScale::Array { .. }) => {
                let WorldBrushScale::Array { values, len } = self.scale else { unreachable!("array scale") };
                if self.scale_stage < len {
                    draft.builder().number(None, values[usize::from(self.scale_stage)])?;
                    self.scale_stage += 1;
                    context.consume_fuel(1);
                    return Ok(WorldInteractionStep::Pending);
                }
                draft.builder().end_container()?;
            }
            16 => draft.builder().end_container()?,
            17 if matches!(self.scale, WorldBrushScale::Array { .. }) => draft.builder().end_container()?,
            _ => {
                let prepared = self.draft.take().expect("terminal brush draft").finish()?;
                input.publish_prepared_claimed_action(prepared)?;
                self.claim = None;
                return Ok(WorldInteractionStep::Complete);
            }
        }
        self.stage += 1;
        context.consume_fuel(1);
        Ok(WorldInteractionStep::Pending)
    }

    pub fn close_step(&mut self, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
        if self.draft.take().is_some() {
            return false;
        }
        if let Some(claim) = self.claim.take() {
            if input.release_action_claim(claim).is_err() {
                self.claim = Some(claim);
            }
            return false;
        }
        true
    }
}

impl WorldGumballCommitJob {
    pub fn new(generation: u64, gesture: WorldGumballGesture, phase: WorldGumballPhase) -> Self {
        Self { generation, gesture, phase, claim: None, draft: None, stage: 0, complete: false }
    }

    /// 🔢️ How many motion numbers this gesture's verb carries: `dx dy dz`, `ax ay az angle` or `sx sy sz`.
    fn motion_len(&self) -> usize {
        if self.gesture.handle.is_rotate() {
            4
        } else {
            3
        }
    }

    pub fn action_id(&self) -> &'static str {
        gumball_handle_action_id(self.gesture.handle)
    }

    fn string_bytes(&self, session_state: &WorldSessionState) -> Result<usize, ui_wgpu::wgpu::BoundedActionFault> {
        let keys: &[&str] = if self.gesture.handle.is_translate() {
            &["surfaceId", "windowId", "mode", "ids", "dx", "dy", "dz"]
        } else if self.gesture.handle.is_rotate() {
            &["surfaceId", "windowId", "mode", "ids", "ax", "ay", "az", "angle"]
        } else {
            &["surfaceId", "windowId", "mode", "ids", "sx", "sy", "sz"]
        };
        let mut bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&session_state.controller_id, self.action_id(), &session_state.surface_id, &session_state.surface_id, self.gesture.mode.map_or("mesh", WorldComponentKind::as_str)])?;
        for key in keys {
            bytes = bytes.checked_add(key.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        }
        bytes = bytes.checked_add(usize::from(self.gesture.selected_bytes)).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        for (key, value) in self.phase.tail() {
            bytes = bytes.checked_add(key.len() + value.len()).ok_or(ui_wgpu::wgpu::BoundedActionFault::ByteCredits)?;
        }
        if bytes > WORLD_INTERACTION_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::BoundedActionFault::ByteCredits);
        }
        Ok(bytes)
    }

    pub fn numeric(&self, index: usize) -> Option<(&'static str, f64)> {
        if self.gesture.handle.is_translate() {
            [("dx", self.gesture.translate.x as f64), ("dy", self.gesture.translate.y as f64), ("dz", self.gesture.translate.z as f64)].get(index).copied()
        } else if self.gesture.handle.is_rotate() {
            let axis = self.gesture.handle.axis_dir().unwrap_or(Vec3::ZERO);
            [("ax", axis.x as f64), ("ay", axis.y as f64), ("az", axis.z as f64), ("angle", self.gesture.angle as f64)].get(index).copied()
        } else {
            [("sx", self.gesture.scale.x as f64), ("sy", self.gesture.scale.y as f64), ("sz", self.gesture.scale.z as f64)].get(index).copied()
        }
    }

    /// 🪟️ A transform verb is WINDOW-OWNED, exactly like `setCamera`: the app declares it on
    /// its preview window kinds only, and the shell resolves `ActionAddress::window_instance_id`
    /// from this argument before it falls back to the focused window
    /// (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s `dispatch_action`). A World3d surface is keyed BY its
    /// window instance, so its own id is that address. Without it a gumball drag in a
    /// multi-window mode was addressed to whatever had focus, measured verbatim on 6118 as
    /// `handle_action promise failed: window kind generation3d-generate-form does not own
    /// action translateSelection` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn step(&mut self, session_state: &WorldSessionState, gumball_state: &WorldGumballState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> Result<WorldInteractionStep, ui_wgpu::wgpu::BoundedActionFault> {
        if context.should_yield() {
            return Ok(WorldInteractionStep::Pending);
        }
        if self.phase == WorldGumballPhase::Skip {
            self.complete = true;
        }
        if self.complete {
            if self.gesture.selected_len > 0 {
                self.gesture.selected_len -= 1;
                self.gesture.selected[usize::from(self.gesture.selected_len)] = None;
                context.consume_fuel(1);
                return Ok(WorldInteractionStep::Pending);
            }
            self.gesture.selected_bytes = 0;
            return Ok(WorldInteractionStep::Complete);
        }
        if self.gesture.revision != session_state.interaction_revision || self.generation != generation {
            return Ok(WorldInteractionStep::Stale);
        }
        if self.claim.is_none() {
            let claim = input.claim_action(self.string_bytes(session_state)?)?;
            self.claim = Some(claim);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if self.draft.is_none() {
            self.draft = Some(input.draft_claimed_action(self.claim.expect("gumball claim"), &session_state.controller_id, self.action_id())?);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        let selected_end = 5 + self.gesture.selected_len;
        let selected = if self.stage >= 5 && self.stage < selected_end {
            let target = self.gesture.selected[usize::from(self.stage - 5)].as_ref().expect("gumball selected target");
            Some(if self.gesture.live { target.captured_text() } else { target.text(gumball_state, interaction_state, scene_state, self.gesture.mode).ok_or(ui_wgpu::wgpu::BoundedActionFault::Structure)? })
        } else {
            None
        };
        let index = (self.stage > selected_end).then(|| usize::from(self.stage - selected_end - 1));
        let numeric = index.and_then(|index| self.numeric(index));
        let tail = index.and_then(|index| index.checked_sub(self.motion_len())).and_then(|index| self.phase.tail().get(index).copied());
        let draft = self.draft.as_mut().expect("gumball draft");
        match self.stage {
            0 => draft.builder().begin_object(None)?,
            1 => draft.builder().string(Some("surfaceId"), &session_state.surface_id)?,
            2 => draft.builder().string(Some("windowId"), &session_state.surface_id)?,
            3 => draft.builder().string(Some("mode"), self.gesture.mode.map_or("mesh", WorldComponentKind::as_str))?,
            4 => draft.builder().begin_array(Some("ids"))?,
            stage if usize::from(stage - 5) < usize::from(self.gesture.selected_len) => {
                draft.builder().string(None, selected.expect("selected id resolved"))?;
            }
            stage if stage == 5 + self.gesture.selected_len => draft.builder().end_container()?,
            _ => {
                if let Some((key, value)) = numeric {
                    draft.builder().number(Some(key), value)?;
                } else if let Some((key, value)) = tail {
                    draft.builder().string(Some(key), value)?;
                } else {
                    draft.builder().end_container()?;
                    let prepared = self.draft.take().expect("complete gumball draft").finish()?;
                    input.publish_prepared_claimed_action(prepared)?;
                    self.claim = None;
                    self.complete = true;
                    context.consume_fuel(1);
                    return Ok(WorldInteractionStep::Pending);
                }
            }
        }
        self.stage += 1;
        context.consume_fuel(1);
        Ok(WorldInteractionStep::Pending)
    }

    pub fn close_step(&mut self, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> bool {
        if self.draft.take().is_some() {
            return false;
        }
        if let Some(claim) = self.claim.take() {
            if input.release_action_claim(claim).is_err() {
                self.claim = Some(claim);
            }
            return false;
        }
        self.gesture.close_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.complete && self.claim.is_none() && self.draft.is_none() && self.gesture.selected_len == 0 && self.gesture.selected_bytes == 0 && self.gesture.pending.is_none()
    }
}
//#endregion 🧵️WorldInteractionTransaction
