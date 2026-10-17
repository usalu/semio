//! 🎯️ World3d authority step and the ray, object and component pick cursors.

use super::*;

//#region 🧵️WorldInteractionTransaction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldInteractionAuthorityStep {
    Idle,
    Pending,
    OutputBlocked,
    Complete,
    Stale,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldRayPickPurpose {
    Instance,
    Hover,
    Paint,
    Surface,
}

#[derive(Clone, Copy, Debug)]
struct WorldRayHitRef {
    pub mesh: WorldInteractionMeshToken,
    pub draw: u16,
    pub instance: u16,
    pub triangle: Option<u32>,
    pub distance: f32,
    pub bary_u: f32,
    pub bary_v: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub struct WorldRayPickCursor {
    pub revision: u64,
    pub generation: u64,
    pub purpose: WorldRayPickPurpose,
    pub origin: Vec3,
    pub direction: Vec3,
    pub local_x: f32,
    pub local_y: f32,
    pub viewport: Rect,
    pub view_projection: Mat4,
    pub projection_spec: Viewport3dProjectionSpec,
    pub draw: usize,
    pub instance: usize,
    pub primitive: usize,
    pub mesh: Option<WorldInteractionMeshToken>,
    pub mesh_probe: u16,
    pub merge: u8,
    pub best: Option<WorldRayHitRef>,
    /// 🧲️ How many of `state.pick_targets` this cursor has scored — the sub-object PRE-PASS, one
    /// row per step, run before the first draw so the two compete in one ordering.
    pub pick_target: usize,
    /// 🧲️ `(score, index)` of the winning pick target, React's `targetRayScore` ordering.
    pub best_target: Option<(f32, u16)>,
    pub complete: bool,
    pub faulted: bool,
}

impl WorldRayPickCursor {
    pub fn new(session_state: &WorldSessionState, camera_state: &WorldCameraState, generation: u64, purpose: WorldRayPickPurpose, x: f32, y: f32) -> Option<Self> {
        let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
        let camera = camera_state.orbit.to_camera();
        let (origin, direction) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
        Some(Self {
            revision: session_state.interaction_revision,
            generation,
            purpose,
            origin,
            direction,
            local_x,
            local_y,
            viewport,
            view_projection: ui_wgpu::wgpu::projection_spec_view_proj(&camera, camera_state.projection_spec, viewport.w, viewport.h),
            projection_spec: camera_state.projection_spec,
            draw: 0,
            instance: 0,
            primitive: 0,
            mesh: None,
            mesh_probe: 0,
            merge: 0,
            best: None,
            pick_target: 0,
            best_target: None,
            complete: false,
            faulted: false,
        })
    }

    /// ⏭️ Advances past a draw the ray cannot test — one whose mesh the guest has not published yet.
    fn skip_draw(&mut self) {
        self.draw += 1;
        self.instance = 0;
        self.primitive = 0;
        self.mesh = None;
        self.mesh_probe = 0;
    }

    /// 🧲️ Sub-object PRE-PASS. One retained pick target per step (the set is capped at
    /// `WORLD_PICK_TARGET_CAPACITY` when the lane is parsed, so this loop is bounded by
    /// construction), scored with React's own `targetRayScore`. A `Paint`/`Surface` gesture aims
    /// at the model itself, never at a pick proxy, so it skips the pass entirely.
    ///
    /// 🫥️ Unpublished mesh ⇒ an uninteractable draw, never a fault — see the same arm of
    /// `WorldInteractionRegistryBuildCursor::step`.
    pub fn step(&mut self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.faulted {
            return WorldInteractionStep::Fault;
        }
        if interaction_state.interaction_meshes.faulted {
            return WorldInteractionStep::Fault;
        }
        if self.revision != session_state.interaction_revision || self.generation != generation {
            return WorldInteractionStep::Stale;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        if matches!(self.purpose, WorldRayPickPurpose::Instance | WorldRayPickPurpose::Hover) && self.pick_target < selection_state.pick_targets.len() {
            let index = self.pick_target;
            self.pick_target += 1;
            let target = &selection_state.pick_targets[index];
            if target.is_selectable() {
                if let (Some(score), Ok(slot)) = (world_pick_target_ray_score(target, self.origin, self.direction), u16::try_from(index)) {
                    if self.best_target.is_none_or(|(best, _)| score < best) {
                        self.best_target = Some((score, slot));
                    }
                }
            }
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let Some(draw) = scene_state.draws.get(self.draw) else {
            self.complete = true;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        if self.mesh.is_none() {
            if draw.mesh_key.len() > WORLD_INTERACTION_ID_BYTE_CAPACITY || usize::from(self.mesh_probe) == WORLD_INTERACTION_MESH_CAPACITY {
                self.faulted = true;
                return WorldInteractionStep::Fault;
            }
            let slot = (WorldInteractionMeshRegistry::hash(&draw.mesh_key) + usize::from(self.mesh_probe)) % WORLD_INTERACTION_MESH_CAPACITY;
            match interaction_state.interaction_meshes.slots[slot] {
                Some(entry) if entry.id.as_str() == draw.mesh_key => {
                    self.mesh = Some(WorldInteractionMeshToken { slot: slot as u16, generation: entry.generation });
                    self.mesh_probe = 0;
                }
                Some(_) => self.mesh_probe += 1,
                None => {
                    self.skip_draw();
                }
            }
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let token = self.mesh.expect("world pick mesh token resolved above");
        let Some(admitted) = interaction_state.interaction_meshes.resolve(token) else {
            return WorldInteractionStep::Stale;
        };
        if admitted.id.as_str() != draw.mesh_key || admitted.version != live_world3d_mesh_version(scene_state, &draw.mesh_key) {
            return WorldInteractionStep::Stale;
        }
        let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
            self.skip_draw();
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        let Ok(schema) = mesh.schema() else {
            return WorldInteractionStep::Stale;
        };
        if admitted.vertices != schema.vertices || admitted.triangles != schema.indices / 3 {
            return WorldInteractionStep::Stale;
        }
        let Some(instance) = draw.instances.get(self.instance) else {
            self.draw += 1;
            self.instance = 0;
            self.primitive = 0;
            self.mesh = None;
            self.mesh_probe = 0;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        if schema.indices == 0 && matches!(self.purpose, WorldRayPickPurpose::Instance | WorldRayPickPurpose::Hover) {
            let count = if schema.edges > 0 { schema.edges } else if schema.vertex_ids == schema.vertices { schema.vertices } else { 0 };
            if self.primitive >= count as usize {
                self.instance += 1;
                self.primitive = 0;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            let index = self.primitive as u32;
            let project = |point| ui_wgpu::wgpu::projection_spec_project_point(self.view_projection, self.projection_spec, point, self.viewport.w, self.viewport.h);
            let point = if schema.edges > 0 {
                mesh.edge(index).ok().and_then(|edge| {
                    let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                    let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                    let segment = ui_wgpu::wgpu::projection_spec_project_segment(self.view_projection, self.projection_spec, a, b, self.viewport.w, self.viewport.h)?;
                    let [screen_a, screen_b] = segment.screen;
                    (ui_wgpu::wgpu::screen_segment_distance(self.local_x, self.local_y, screen_a[0], screen_a[1], screen_b[0], screen_b[1]) <= PICK_EDGE_SCREEN_PX).then(|| ui_wgpu::wgpu::ray_segment_closest(self.origin, self.direction, segment.points[0], segment.points[1]).map(|closest| closest.segment_point)).flatten()
                })
            } else if world_mesh_component_id(mesh, Mesh3dField::VertexIds, index) != u32::MAX {
                world_mesh_vertex(mesh, index).and_then(|point| {
                    let point = instance.model.transform_point(point);
                    let screen = project(point)?;
                    ((screen[0] - self.local_x).hypot(screen[1] - self.local_y) <= PICK_VERTEX_SCREEN_PX).then_some(point)
                })
            } else { None };
            if let Some(point) = point {
                let distance = point.sub(self.origin).dot(self.direction);
                if distance.is_finite() && distance >= 0.0 && self.best.is_none_or(|best| distance < best.distance) {
                    let (Ok(draw), Ok(instance)) = (u16::try_from(self.draw), u16::try_from(self.instance)) else { self.faulted = true; return WorldInteractionStep::Fault; };
                    self.best = Some(WorldRayHitRef { mesh: token, draw, instance, triangle: None, distance, bary_u: 0.0, bary_v: 0.0, point, normal: Vec3::ZERO });
                }
            }
            self.primitive += 1;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let Some(indices) = u32::try_from(self.primitive).ok().and_then(|triangle| world_mesh_triangle(mesh, triangle)) else {
            self.instance += 1;
            self.primitive = 0;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        let Some(a) = world_mesh_vertex(mesh, indices[0]) else {
            self.faulted = true;
            return WorldInteractionStep::Fault;
        };
        let Some(b) = world_mesh_vertex(mesh, indices[1]) else {
            self.faulted = true;
            return WorldInteractionStep::Fault;
        };
        let Some(c) = world_mesh_vertex(mesh, indices[2]) else {
            self.faulted = true;
            return WorldInteractionStep::Fault;
        };
        let a = instance.model.transform_point(a);
        let b = instance.model.transform_point(b);
        let c = instance.model.transform_point(c);
        if let Some((distance, bary_u, bary_v)) = world_ray_triangle_barycentric(self.origin, self.direction, a, b, c) {
            if self.best.is_none_or(|best| distance < best.distance) {
                let point = self.origin.add(self.direction.scale(distance));
                let mut normal = b.sub(a).cross(c.sub(a));
                if normal.length() > 1e-6 {
                    normal = normal.normalize();
                }
                let Ok(draw_index) = u16::try_from(self.draw) else {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                };
                let Ok(instance_index) = u16::try_from(self.instance) else {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                };
                let Ok(triangle_index) = u32::try_from(self.primitive) else {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                };
                self.best = Some(WorldRayHitRef { mesh: token, draw: draw_index, instance: instance_index, triangle: Some(triangle_index), distance, bary_u, bary_v, point, normal });
            }
        }
        self.primitive += 1;
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    /// 🧲️ ONE ordering across the sub-object proxies and the instance meshes, exactly as r3f
    /// raycasts `SpatialPickHitTarget` and the model in the same scene: the lower score wins, and
    /// a tie goes to the pick target (React's `targetRayScore` already folded the kind's
    /// tie-break into the score, so `<=` here is that ladder's last rung).
    pub fn finish_plan(&self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64) -> Result<Option<WorldInteractionPlan>, WorldInteractionStep> {
        if self.faulted {
            return Err(WorldInteractionStep::Fault);
        }
        if self.revision != session_state.interaction_revision || self.generation != generation {
            return Err(WorldInteractionStep::Stale);
        }
        if !self.complete {
            return Err(WorldInteractionStep::Pending);
        }
        if let Some((score, slot)) = self.best_target {
            if score <= self.best.map_or(f32::INFINITY, |hit| hit.distance) {
                return self.pick_target_plan(session_state, selection_state, generation, slot);
            }
        }
        let Some(hit) = self.best else {
            return match self.purpose {
                WorldRayPickPurpose::Hover => {
                    if selection_state.local_hover_id.is_none() && selection_state.pick_hover_key.is_none() {
                        return Ok(None);
                    }
                    let mut plan = WorldInteractionPlan::new(self.revision, generation);
                    let controller = plan.push_string(&session_state.controller_id).ok_or(WorldInteractionStep::Fault)?;
                    let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
                    let action = WorldFlatAction { kind: WorldFlatActionKind::Hover, strings: [Some(controller), None, None, Some(domain), None, None, None, None], numbers: [0.0; 10] };
                    plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
                }
                WorldRayPickPurpose::Instance => {
                    let mut plan = WorldInteractionPlan::new(self.revision, generation);
                    let controller = plan.push_string(&session_state.controller_id).ok_or(WorldInteractionStep::Fault)?;
                    let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
                    let merge = plan.push_string(world_merge_wire_label(self.merge)).ok_or(WorldInteractionStep::Fault)?;
                    let method = plan.push_string(selection_method_wire_str(SelectionMethod::Pick)).ok_or(WorldInteractionStep::Fault)?;
                    let action = WorldFlatAction { kind: WorldFlatActionKind::Select, strings: [Some(controller), None, None, Some(domain), None, Some(merge), Some(method), None], numbers: [0.0; 10] };
                    plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
                }
                _ => Ok(None),
            };
        };
        let draw = scene_state.draws.get(usize::from(hit.draw)).ok_or(WorldInteractionStep::Fault)?;
        let admitted = interaction_state.interaction_meshes.resolve(hit.mesh).ok_or(WorldInteractionStep::Stale)?;
        if admitted.id.as_str() != draw.mesh_key || admitted.version != live_world3d_mesh_version(scene_state, &draw.mesh_key) {
            return Err(WorldInteractionStep::Stale);
        }
        let mesh = *scene_state.meshes.get(&draw.mesh_key).ok_or(WorldInteractionStep::Fault)?;
        let instance = draw.instances.get(usize::from(hit.instance)).ok_or(WorldInteractionStep::Fault)?;
        let target_id = instance_interaction_id(interaction_state, instance.id.as_str());
        let target_granularity_id = instance_interaction_granularity_id(session_state, interaction_state, instance.id.as_str());
        if self.purpose == WorldRayPickPurpose::Hover && selection_state.local_hover_id.as_deref() == Some(target_id) && selection_state.local_hover_granularity_id.as_deref() == Some(target_granularity_id) {
            return Ok(None);
        }
        let mut plan = WorldInteractionPlan::new(self.revision, generation);
        let controller = plan.push_string(&session_state.controller_id).ok_or(WorldInteractionStep::Fault)?;
        let surface = plan.push_string(&session_state.surface_id).ok_or(WorldInteractionStep::Fault)?;
        let object = plan.push_string(target_id).ok_or(WorldInteractionStep::Fault)?;
        let action = match self.purpose {
            WorldRayPickPurpose::Paint => {
                let (u, v) = interpolate_mesh_uv(mesh, hit.triangle.ok_or(WorldInteractionStep::Fault)? as usize, hit.bary_u, hit.bary_v).map_err(|fault|if fault==ui_wgpu::wgpu::Mesh3dFault::Busy{WorldInteractionStep::Pending}else{WorldInteractionStep::Fault})?.ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction { kind: WorldFlatActionKind::PaintAt, strings: [Some(controller), Some(surface), Some(object), None, None, None, None, None], numbers: [u as f64, v as f64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] }
            }
            WorldRayPickPurpose::Surface => WorldFlatAction {
                kind: WorldFlatActionKind::SurfacePlace,
                strings: [Some(controller), Some(surface), Some(object), None, None, None, None, None],
                numbers: [hit.point.x as f64, hit.point.y as f64, hit.point.z as f64, hit.normal.x as f64, hit.normal.y as f64, hit.normal.z as f64, 0.0, 0.0, 0.0, 0.0],
            },
            WorldRayPickPurpose::Instance => {
                let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
                let granularity = plan.push_string(target_granularity_id).ok_or(WorldInteractionStep::Fault)?;
                let merge = plan.push_string(world_merge_wire_label(self.merge)).ok_or(WorldInteractionStep::Fault)?;
                let method = plan.push_string(selection_method_wire_str(SelectionMethod::Pick)).ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction {
                    kind: WorldFlatActionKind::Select,
                    strings: [Some(controller), Some(surface), Some(object), Some(domain), Some(granularity), Some(merge), Some(method), None],
                    numbers: [if session_state.bound_domain_id.is_some() { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                }
            }
            WorldRayPickPurpose::Hover => {
                let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
                let granularity = plan.push_string(target_granularity_id).ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction {
                    kind: WorldFlatActionKind::Hover,
                    strings: [Some(controller), Some(surface), Some(object), Some(domain), Some(granularity), None, None, None],
                    numbers: [if session_state.bound_domain_id.is_some() { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                }
            }
        };
        plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
    }

    /// 🧲️ The `interactionSelect`/`interactionHover` one WINNING sub-object pick target dispatches —
    /// React's own payload for a `SpatialPickHitTarget` pick, with the target's KIND as the
    /// granularity (`vertex`/`edge`/`face`/`object`) and its kernel entity id as the target id.
    ///
    /// 🪪️ Alias-aware, so a guest that echoes `solid:foo` for a host that dispatched `object:foo`
    /// does not make every move re-publish the same hover.
    fn pick_target_plan(&self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, generation: u64, slot: u16) -> Result<Option<WorldInteractionPlan>, WorldInteractionStep> {
        let target = selection_state.pick_targets.get(usize::from(slot)).ok_or(WorldInteractionStep::Stale)?;
        let key = target.key();
        if self.purpose == WorldRayPickPurpose::Hover && world_pick_keys_match(selection_state.pick_hover_key.as_deref(), Some(key.as_str())) {
            return Ok(None);
        }
        let mut plan = WorldInteractionPlan::new(self.revision, generation);
        let controller = plan.push_string(&session_state.controller_id).ok_or(WorldInteractionStep::Fault)?;
        let surface = plan.push_string(&session_state.surface_id).ok_or(WorldInteractionStep::Fault)?;
        let object = plan.push_string(&target.id).ok_or(WorldInteractionStep::Fault)?;
        let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
        let granularity = plan.push_string(&target.kind).ok_or(WorldInteractionStep::Fault)?;
        let bare = if session_state.bound_domain_id.is_some() { 1.0 } else { 0.0 };
        let action = match self.purpose {
            WorldRayPickPurpose::Instance => {
                let merge = plan.push_string(world_merge_wire_label(self.merge)).ok_or(WorldInteractionStep::Fault)?;
                let method = plan.push_string(selection_method_wire_str(SelectionMethod::Pick)).ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction {
                    kind: WorldFlatActionKind::Select,
                    strings: [Some(controller), Some(surface), Some(object), Some(domain), Some(granularity), Some(merge), Some(method), None],
                    numbers: [bare, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                }
            }
            _ => WorldFlatAction { kind: WorldFlatActionKind::Hover, strings: [Some(controller), Some(surface), Some(object), Some(domain), Some(granularity), None, None, None], numbers: [bare, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] },
        };
        plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.complete && self.best.is_none() && self.best_target.is_none()
    }

    pub fn close_step(&mut self) -> bool {
        if self.best.take().is_some() {
            return false;
        }
        if self.best_target.take().is_some() {
            return false;
        }
        self.complete = true;
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldObjectPickPurpose {
    VortexHover,
    VortexSelect,
    ReferenceHover,
    ReferenceSelect,
}

pub struct WorldObjectPickCursor {
    pub revision: u64,
    pub generation: u64,
    pub purpose: WorldObjectPickPurpose,
    pub origin: Vec3,
    pub direction: Vec3,
    pub slot: u16,
    pub merge: u8,
    pub best: Option<(WorldInteractionObjectToken, f32)>,
    pub complete: bool,
}

impl WorldObjectPickCursor {
    pub fn new(session_state: &WorldSessionState, camera_state: &WorldCameraState, generation: u64, purpose: WorldObjectPickPurpose, x: f32, y: f32) -> Option<Self> {
        let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
        let camera = camera_state.orbit.to_camera();
        let (origin, direction) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
        Some(Self { revision: session_state.interaction_revision, generation, purpose, origin, direction, slot: 0, merge: 0, best: None, complete: false })
    }

    pub fn from_ray(revision: u64, generation: u64, purpose: WorldObjectPickPurpose, origin: Vec3, direction: Vec3) -> Self {
        Self { revision, generation, purpose, origin, direction, slot: 0, merge: 0, best: None, complete: false }
    }

    pub fn step(&mut self, session_state: &WorldSessionState, interaction_state: &WorldInteractionState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.revision != session_state.interaction_revision || self.generation != generation || !interaction_state.interaction_objects.terminal_for_revision(self.revision) {
            return WorldInteractionStep::Stale;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        let Some((index, entry)) = interaction_state.interaction_objects.next_occupied(self.slot) else {
            self.complete = true;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        self.slot = index as u16 + 1;
        if entry.revision != self.revision {
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let distance = match (self.purpose, entry.kind) {
            (WorldObjectPickPurpose::VortexHover | WorldObjectPickPurpose::VortexSelect, WorldInteractionObjectKind::Vortex) => {
                let center = Vec3::new(entry.values[0], entry.values[1], entry.values[2]);
                let radius = entry.values[6].max(0.0);
                ray_aabb_slab(self.origin, self.direction, [center.x - radius, center.y - radius, center.z - radius], [center.x + radius, center.y + radius, center.z + radius])
            }
            (WorldObjectPickPurpose::ReferenceHover | WorldObjectPickPurpose::ReferenceSelect, WorldInteractionObjectKind::Reference) => {
                let center = Vec3::new(entry.values[0], entry.values[1], entry.values[2]);
                ray_plane_point(self.origin, self.direction, center, Vec3::new(0.0, 0.0, 1.0)).and_then(|hit| {
                    let offset = hit.sub(center);
                    (offset.x.abs() <= entry.values[3] * 0.5 && offset.y.abs() <= entry.values[4] * 0.5).then_some(self.origin.sub(hit).length())
                })
            }
            _ => None,
        };
        if let Some(distance) = distance {
            if self.best.is_none_or(|(_, best)| distance < best) {
                self.best = Some((WorldInteractionObjectToken { slot: index as u16, generation: entry.generation, revision: entry.revision }, distance));
            }
        }
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    pub fn finish_plan(&self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, interaction_state: &WorldInteractionState, generation: u64) -> Result<Option<WorldInteractionPlan>, WorldInteractionStep> {
        if !self.complete {
            return Err(WorldInteractionStep::Pending);
        }
        if self.revision != session_state.interaction_revision || self.generation != generation {
            return Err(WorldInteractionStep::Stale);
        }
        let entry = match self.best {
            Some((token, _)) => Some(interaction_state.interaction_objects.resolve(token).ok_or(WorldInteractionStep::Stale)?),
            None => None,
        };
        let mut plan = WorldInteractionPlan::new(self.revision, generation);
        let controller = plan.push_string(&session_state.controller_id).ok_or(WorldInteractionStep::Fault)?;
        let surface = plan.push_string(&session_state.surface_id).ok_or(WorldInteractionStep::Fault)?;
        let action = match self.purpose {
            WorldObjectPickPurpose::VortexHover => {
                let hit = entry.map(|entry| plan.push_string(entry.id.as_str()).ok_or(WorldInteractionStep::Fault)).transpose()?;
                WorldFlatAction { kind: WorldFlatActionKind::VortexHover, strings: [Some(controller), Some(surface), hit, None, None, None, None, None], numbers: [0.0; 10] }
            }
            WorldObjectPickPurpose::VortexSelect => {
                let Some(entry) = entry else {
                    return Ok(None);
                };
                let hit = plan.push_string(entry.id.as_str()).ok_or(WorldInteractionStep::Fault)?;
                let merge = plan.push_string(world_merge_wire_label(self.merge)).ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction { kind: WorldFlatActionKind::VortexSelect, strings: [Some(controller), Some(surface), Some(hit), Some(merge), None, None, None, None], numbers: [0.0; 10] }
            }
            WorldObjectPickPurpose::ReferenceHover => {
                let hit = entry.map(|entry| plan.push_string(entry.id.as_str()).ok_or(WorldInteractionStep::Fault)).transpose()?;
                if hit.is_none() && selection_state.local_hover_id.is_none() {
                    return Ok(None);
                }
                let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
                let granularity = match hit {
                    Some(_) => Some(plan.push_string("reference").ok_or(WorldInteractionStep::Fault)?),
                    None => None,
                };
                WorldFlatAction {
                    kind: WorldFlatActionKind::Hover,
                    strings: [Some(controller), Some(surface), hit, Some(domain), granularity, None, None, None],
                    numbers: [if session_state.bound_domain_id.is_some() { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                }
            }
            WorldObjectPickPurpose::ReferenceSelect => {
                let hit = entry.map(|entry| plan.push_string(entry.id.as_str()).ok_or(WorldInteractionStep::Fault)).transpose()?;
                let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
                let granularity = match hit {
                    Some(_) => Some(plan.push_string("reference").ok_or(WorldInteractionStep::Fault)?),
                    None => None,
                };
                let merge = plan.push_string(world_merge_wire_label(self.merge)).ok_or(WorldInteractionStep::Fault)?;
                let method = plan.push_string(selection_method_wire_str(SelectionMethod::Pick)).ok_or(WorldInteractionStep::Fault)?;
                WorldFlatAction {
                    kind: WorldFlatActionKind::Select,
                    strings: [Some(controller), Some(surface), hit, Some(domain), granularity, Some(merge), Some(method), None],
                    numbers: [if session_state.bound_domain_id.is_some() { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                }
            }
        };
        plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
    }

    pub fn close_step(&mut self) -> bool {
        if self.best.take().is_some() {
            return false;
        }
        self.complete = true;
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldComponentKind {
    Vertex,
    Edge,
    Face,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldComponentPickPurpose {
    Hover,
    Select,
}

#[derive(Clone, Copy)]
pub struct WorldComponentHit {
    pub object: WorldInteractionObjectToken,
    pub id: u32,
    pub primary: f32,
    pub secondary: f32,
}

pub struct WorldComponentPickCursor {
    pub revision: u64,
    pub generation: u64,
    pub purpose: WorldComponentPickPurpose,
    pub kind: WorldComponentKind,
    pub local_x: f32,
    pub local_y: f32,
    pub viewport: Rect,
    pub view_projection: Mat4,
    pub projection_spec: Viewport3dProjectionSpec,
    pub origin: Vec3,
    pub direction: Vec3,
    pub slot: u16,
    pub current: Option<WorldInteractionObjectToken>,
    pub topology: u32,
    pub merge: u8,
    pub best: Option<WorldComponentHit>,
    pub complete: bool,
}

impl WorldComponentPickCursor {
    pub fn new(session_state: &WorldSessionState, camera_state: &WorldCameraState, selection_state: &WorldSelectionState, generation: u64, purpose: WorldComponentPickPurpose, x: f32, y: f32) -> Option<Self> {
        let kind = match selection_state.granularity.as_str() {
            "vertex" => WorldComponentKind::Vertex,
            "edge" => WorldComponentKind::Edge,
            "face" | "component" => WorldComponentKind::Face,
            _ => return None,
        };
        if selection_state.active_object_id.as_ref().is_some_and(|id| id.len() > WORLD_INTERACTION_ID_BYTE_CAPACITY) {
            return None;
        }
        let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
        let camera = camera_state.orbit.to_camera();
        let view_projection = ui_wgpu::wgpu::projection_spec_view_proj(&camera, camera_state.projection_spec, viewport.w, viewport.h);
        let (origin, direction) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
        Some(Self { revision: session_state.interaction_revision, generation, purpose, kind, local_x, local_y, viewport, view_projection, projection_spec: camera_state.projection_spec, origin, direction, slot: 0, current: None, topology: 0, merge: 0, best: None, complete: false })
    }

    pub fn step(&mut self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.revision != session_state.interaction_revision || self.generation != generation || !interaction_state.interaction_objects.terminal_for_revision(self.revision) {
            return WorldInteractionStep::Stale;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        if self.current.is_none() {
            let index = usize::from(self.slot);
            let Some(entry) = interaction_state.interaction_objects.slots.get(index) else {
                self.complete = true;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            };
            self.slot += 1;
            if let Some(entry) = entry.as_ref().filter(|entry| entry.revision == self.revision && entry.kind == WorldInteractionObjectKind::Instance) {
                if selection_state.active_object_id.as_deref().is_none_or(|active| active == entry.id.as_str()) {
                    self.current = Some(WorldInteractionObjectToken { slot: index as u16, generation: entry.generation, revision: entry.revision });
                    self.topology = 0;
                }
            }
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let token = self.current.expect("component cursor current token");
        let Some(instance) = interaction_state.interaction_objects.resolve(token) else {
            return WorldInteractionStep::Stale;
        };
        let Some(mesh_token) = instance.mesh else {
            return WorldInteractionStep::Fault;
        };
        let Some(admitted) = interaction_state.interaction_meshes.resolve(mesh_token) else {
            return WorldInteractionStep::Stale;
        };
        let Some(&mesh) = scene_state.meshes.get(admitted.id.as_str()) else {
            return WorldInteractionStep::Fault;
        };
        let Ok(schema) = mesh.schema() else { return WorldInteractionStep::Stale; };
        let count = match self.kind {
            WorldComponentKind::Vertex => if schema.vertex_ids == schema.vertices { admitted.vertices } else { 0 },
            WorldComponentKind::Edge => admitted.edges,
            WorldComponentKind::Face => admitted.triangles,
        };
        if self.topology >= count {
            self.current = None;
            self.topology = 0;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let index = self.topology;
        self.topology += 1;
        if self.kind == WorldComponentKind::Vertex && world_mesh_component_id(mesh, Mesh3dField::VertexIds, index) == u32::MAX {
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let candidate = match self.kind {
            WorldComponentKind::Vertex => mesh.vec3(Mesh3dField::Positions, index).ok().and_then(|point| {
                let world = instance.model.transform_point(Vec3::new(point[0], point[1], point[2]));
                ui_wgpu::wgpu::projection_spec_project_point(self.view_projection, self.projection_spec, world, self.viewport.w, self.viewport.h).and_then(|screen| {
                    let dx = screen[0] - self.local_x;
                    let dy = screen[1] - self.local_y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    (distance <= PICK_VERTEX_SCREEN_PX).then_some((distance, 0.0))
                })
            }),
            WorldComponentKind::Edge => mesh.edge(index).ok().and_then(|edge| {
                let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                let segment = ui_wgpu::wgpu::projection_spec_project_segment(self.view_projection, self.projection_spec, a, b, self.viewport.w, self.viewport.h)?;
                let [screen_a, screen_b] = segment.screen;
                let screen_distance = ui_wgpu::wgpu::screen_segment_distance(self.local_x, self.local_y, screen_a[0], screen_a[1], screen_b[0], screen_b[1]);
                if screen_distance > PICK_EDGE_SCREEN_PX { return None; }
                let closest = ui_wgpu::wgpu::ray_segment_closest(self.origin, self.direction, segment.points[0], segment.points[1])?;
                let depth = closest.segment_point.sub(self.origin).dot(self.direction);
                (depth.is_finite() && depth >= 0.0).then_some((depth, closest.distance))
            }),
            WorldComponentKind::Face => world_mesh_triangle(mesh, index).and_then(|indices| {
                let a = instance.model.transform_point(world_mesh_vertex(mesh, indices[0])?);
                let b = instance.model.transform_point(world_mesh_vertex(mesh, indices[1])?);
                let c = instance.model.transform_point(world_mesh_vertex(mesh, indices[2])?);
                world_ray_triangle_barycentric(self.origin, self.direction, a, b, c).map(|(distance, _, _)| (distance, 0.0))
            }),
        };
        if let Some((primary, secondary)) = candidate {
            let better = self.best.is_none_or(|best| primary < best.primary - 1e-4 || ((primary - best.primary).abs() <= 1e-4 && secondary < best.secondary));
            if better {
                let id = match self.kind {
                    WorldComponentKind::Vertex => world_mesh_component_id(mesh, Mesh3dField::VertexIds, index),
                    WorldComponentKind::Edge => world_mesh_component_id(mesh, Mesh3dField::EdgeIds, index),
                    WorldComponentKind::Face => world_mesh_component_id(mesh, Mesh3dField::FaceIds, index),
                };
                self.best = Some(WorldComponentHit { object: token, id, primary, secondary });
            }
        }
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    pub fn finish_plan(&self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64) -> Result<Option<WorldInteractionPlan>, WorldInteractionStep> {
        if !self.complete {
            return Err(WorldInteractionStep::Pending);
        }
        if self.revision != session_state.interaction_revision || self.generation != generation {
            return Err(WorldInteractionStep::Stale);
        }
        let object = match self.best {
            Some(hit) => Some((interaction_state.interaction_objects.resolve(hit.object).ok_or(WorldInteractionStep::Stale)?, hit.id)),
            None => None,
        };
        if self.purpose == WorldComponentPickPurpose::Hover {
            let unchanged = match object {
                Some((entry, id)) => {
                    selection_state.hovered_component_id.as_deref().and_then(|value| value.parse::<u32>().ok()) == Some(id)
                        && selection_state.hovered_component_object_id.as_deref() == Some(entry.id.as_str())
                        && selection_state.hovered_component_mode.as_deref() == Some(self.kind.as_str())
                }
                None => selection_state.hovered_component_id.is_none() && selection_state.hovered_component_object_id.is_none() && selection_state.hovered_component_mode.is_none(),
            };
            if unchanged {
                return Ok(None);
            }
        }
        let mut plan = WorldInteractionPlan::new(self.revision, generation);
        let controller = plan.push_string(&session_state.controller_id).ok_or(WorldInteractionStep::Fault)?;
        let surface = plan.push_string(&session_state.surface_id).ok_or(WorldInteractionStep::Fault)?;
        let object_span = object.map(|(entry, _)| plan.push_string(entry.id.as_str()).ok_or(WorldInteractionStep::Fault)).transpose()?;
        let mode = plan.push_string(self.kind.as_str()).ok_or(WorldInteractionStep::Fault)?;
        let merge = if self.purpose == WorldComponentPickPurpose::Select { Some(plan.push_string(world_merge_wire_label(self.merge)).ok_or(WorldInteractionStep::Fault)?) } else { None };
        let target = object.map(|(entry, group)| WorldComponentTarget::new(interaction_state, scene_state, entry, self.kind, group).map_err(|_| WorldInteractionStep::Fault)).transpose()?;
        let target_span = target.as_ref().map(|target| plan.push_string(target.as_str()).ok_or(WorldInteractionStep::Fault)).transpose()?;
        let domain = plan.push_string(resolved_domain_id(session_state)).ok_or(WorldInteractionStep::Fault)?;
        let method = plan.push_string(selection_method_wire_str(SelectionMethod::Pick)).ok_or(WorldInteractionStep::Fault)?;
        let action = WorldFlatAction {
            kind: if self.purpose == WorldComponentPickPurpose::Hover { WorldFlatActionKind::Hover } else { WorldFlatActionKind::Select },
            strings: [Some(controller), Some(surface), target_span, Some(domain), Some(mode), merge, Some(method), object_span],
            numbers: [if session_state.bound_domain_id.is_some() { 1.0 } else { 0.0 }, 1.0, 1.0, object.map_or(0.0, |(_, id)| id as f64), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        };
        plan.push_action(action).then_some(plan).ok_or(WorldInteractionStep::Fault).map(Some)
    }

    pub fn close_step(&mut self) -> bool {
        if self.best.take().is_some() {
            return false;
        }
        if self.current.take().is_some() {
            return false;
        }
        self.complete = true;
        true
    }
}
//#endregion 🧵️WorldInteractionTransaction
