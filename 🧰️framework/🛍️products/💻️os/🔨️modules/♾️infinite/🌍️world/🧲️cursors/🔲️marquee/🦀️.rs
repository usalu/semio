//! 🔲️ World3d marquee gesture, pick cursor and publish jobs for objects and components.

use super::*;

pub struct WorldMarqueeGesture {
    pub revision: u64,
    pub start_generation: u64,
    pub points: Box<[Option<[f32; 2]>; WORLD_INTERACTION_MARQUEE_POINT_CAPACITY]>,
    pub len: u16,
    pub retiring: bool,
}

pub const WORLD_MARQUEE_RESULT_PAGE_CAPACITY: usize = 64;

pub const WORLD_MARQUEE_RESULT_PAGE_COUNT: usize = 16;

pub const WORLD_COMPONENT_MARQUEE_CAPACITY: usize = 64;

//#region 🧵️WorldInteractionTransaction
#[derive(Clone, Copy)]
pub enum WorldMarqueeResult {
    Object(WorldInteractionObjectToken),
    Component { object: WorldInteractionObjectToken, id: u32 },
}

pub struct WorldMarqueeResultPages {
    pub pages: Box<[[Option<WorldMarqueeResult>; WORLD_MARQUEE_RESULT_PAGE_CAPACITY]; WORLD_MARQUEE_RESULT_PAGE_COUNT]>,
    pub lens: [u8; WORLD_MARQUEE_RESULT_PAGE_COUNT],
    id_bytes: [u16; WORLD_MARQUEE_RESULT_PAGE_COUNT],
    pub page_len: u8,
}

impl Default for WorldMarqueeResultPages {
    fn default() -> Self {
        Self { pages: Box::new([[None; WORLD_MARQUEE_RESULT_PAGE_CAPACITY]; WORLD_MARQUEE_RESULT_PAGE_COUNT]), lens: [0; WORLD_MARQUEE_RESULT_PAGE_COUNT], id_bytes: [0; WORLD_MARQUEE_RESULT_PAGE_COUNT], page_len: 0 }
    }
}

impl WorldMarqueeResultPages {
    pub fn push(&mut self, result: WorldMarqueeResult, id_bytes: usize) -> bool {
        if let WorldMarqueeResult::Component { object, id } = result {
            if self.pages[0][..usize::from(self.lens[0])].iter().flatten().any(|entry| matches!(entry, WorldMarqueeResult::Component { object: existing_object, id: existing } if *existing_object == object && *existing == id)) {
                return true;
            }
        }
        if matches!(result, WorldMarqueeResult::Component { .. }) && (self.page_len > 1 || self.page_len == 1 && usize::from(self.lens[0]) == WORLD_COMPONENT_MARQUEE_CAPACITY) {
            return false;
        }
        let mut page = usize::from(self.page_len.saturating_sub(1));
        if self.page_len == 0 || usize::from(self.lens[page]) == WORLD_MARQUEE_RESULT_PAGE_CAPACITY {
            page = usize::from(self.page_len);
            if page == WORLD_MARQUEE_RESULT_PAGE_COUNT {
                return false;
            }
            self.page_len += 1;
        }
        let item = usize::from(self.lens[page]);
        let Some(bytes) = usize::from(self.id_bytes[page]).checked_add(id_bytes).filter(|bytes| *bytes <= WORLD_INTERACTION_BYTE_CAPACITY) else {
            return false;
        };
        self.pages[page][item] = Some(result);
        self.lens[page] += 1;
        self.id_bytes[page] = bytes as u16;
        true
    }

    pub fn close_step(&mut self) -> bool {
        if self.page_len == 0 {
            return true;
        }
        let page = usize::from(self.page_len - 1);
        if self.lens[page] > 0 {
            self.lens[page] -= 1;
            self.pages[page][usize::from(self.lens[page])] = None;
            return false;
        }
        self.id_bytes[page] = 0;
        self.page_len -= 1;
        false
    }
}

pub struct WorldMarqueePickCursor {
    pub generation: u64,
    pub gesture: WorldMarqueeGesture,
    pub viewport: Rect,
    pub view_projection: Mat4,
    pub projection_spec: Viewport3dProjectionSpec,
    pub rectangle: bool,
    pub component_kind: Option<WorldComponentKind>,
    pub crossing: Option<bool>,
    direction_index: u16,
    pub slot: u16,
    pub current: Option<WorldInteractionObjectToken>,
    pub topology: u32,
    polygon_edge: u16,
    pub candidate: Option<WorldMarqueeCandidate>,
    winding: [i16; 3],
    intersects: bool,
    any_visible: bool,
    pub results: WorldMarqueeResultPages,
    pub complete: bool,
    pub faulted: bool,
}

#[derive(Clone, Copy)]
pub enum WorldMarqueeCandidate {
    Point { point: [f32; 2], component: Option<u32> },
    Segment { points: [[f32; 2]; 2], component: u32 },
    Triangle { points: [[f32; 2]; 3], component: Option<u32> },
}

impl WorldMarqueePickCursor {
    pub fn new(session_state: &WorldSessionState, camera_state: &WorldCameraState, selection_state: &WorldSelectionState, generation: u64, gesture: WorldMarqueeGesture) -> Option<Self> {
        let viewport = render_pick_viewport(session_state);
        if viewport.w <= 0.0 || viewport.h <= 0.0 || gesture.len < 2 {
            return None;
        }
        let camera = camera_state.orbit.to_camera();
        let view_projection = ui_scene::math::projection_spec_view_proj(&camera, camera_state.projection_spec, viewport.w, viewport.h);
        let rectangle = selection_state.selection_method != "lasso";
        let component_kind = match selection_state.granularity.as_str() {
            "vertex" => Some(WorldComponentKind::Vertex),
            "edge" => Some(WorldComponentKind::Edge),
            "face" | "component" => Some(WorldComponentKind::Face),
            _ => None,
        };
        let crossing = rectangle.then(|| {
            let start = gesture.points[0].expect("marquee start");
            let end = gesture.points[usize::from(gesture.len - 1)].expect("marquee end");
            end[0] < start[0]
        });
        Some(Self {
            generation,
            gesture,
            viewport,
            view_projection,
            projection_spec: camera_state.projection_spec,
            rectangle,
            component_kind,
            crossing,
            direction_index: 1,
            slot: 0,
            current: None,
            topology: 0,
            polygon_edge: 0,
            candidate: None,
            winding: [0; 3],
            intersects: false,
            any_visible: false,
            results: WorldMarqueeResultPages::default(),
            complete: false,
            faulted: false,
        })
    }

    fn local_point(&self, index: usize) -> [f32; 2] {
        let point = self.gesture.points[index].expect("marquee point within retained length");
        [point[0] - self.viewport.x, point[1] - self.viewport.y]
    }

    fn admit_candidate(&mut self, interaction_state: &WorldInteractionState, result: WorldMarqueeResult) -> WorldInteractionStep {
        let token = match result {
            WorldMarqueeResult::Object(token) | WorldMarqueeResult::Component { object: token, .. } => token,
        };
        let Some(entry) = interaction_state.interaction_objects.resolve(token) else {
            return WorldInteractionStep::Stale;
        };
        let id_bytes = if matches!(result, WorldMarqueeResult::Object(_)) { entry.id.as_str().len() } else { 0 };
        if !self.results.push(result, id_bytes) {
            self.faulted = true;
            return WorldInteractionStep::Fault;
        }
        WorldInteractionStep::Pending
    }

    fn orient(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f32 {
        (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
    }

    fn on_segment(point: [f32; 2], a: [f32; 2], b: [f32; 2]) -> bool {
        point[0] >= a[0].min(b[0]) && point[0] <= a[0].max(b[0]) && point[1] >= a[1].min(b[1]) && point[1] <= a[1].max(b[1])
    }

    fn segments_intersect(a0: [f32; 2], a1: [f32; 2], b0: [f32; 2], b1: [f32; 2]) -> bool {
        let o1 = Self::orient(a0, a1, b0);
        let o2 = Self::orient(a0, a1, b1);
        let o3 = Self::orient(b0, b1, a0);
        let o4 = Self::orient(b0, b1, a1);
        (o1 == 0.0 && Self::on_segment(b0, a0, a1)) || (o2 == 0.0 && Self::on_segment(b1, a0, a1)) || (o3 == 0.0 && Self::on_segment(a0, b0, b1)) || (o4 == 0.0 && Self::on_segment(a1, b0, b1)) || ((o1 > 0.0) != (o2 > 0.0) && (o3 > 0.0) != (o4 > 0.0))
    }

    pub fn rect(&self) -> [f32; 4] {
        let start = self.local_point(0);
        let end = self.local_point(usize::from(self.gesture.len - 1));
        [start[0].min(end[0]), start[1].min(end[1]), start[0].max(end[0]), start[1].max(end[1])]
    }

    fn rect_contains(rect: [f32; 4], point: [f32; 2]) -> bool {
        point[0] >= rect[0] && point[0] <= rect[2] && point[1] >= rect[1] && point[1] <= rect[3]
    }

    fn triangle_intersects_rect(points: [[f32; 2]; 3], rect: [f32; 4]) -> bool {
        if points.iter().any(|point| Self::rect_contains(rect, *point)) {
            return true;
        }
        let corners = [[rect[0], rect[1]], [rect[2], rect[1]], [rect[2], rect[3]], [rect[0], rect[3]]];
        (0..3).any(|edge| (0..4).any(|side| Self::segments_intersect(points[edge], points[(edge + 1) % 3], corners[side], corners[(side + 1) % 4])))
    }

    fn begin_candidate(&mut self, candidate: WorldMarqueeCandidate) {
        self.candidate = Some(candidate);
        self.polygon_edge = 0;
        self.winding = [0; 3];
        self.intersects = false;
    }

    fn candidate_step(&mut self) -> Option<bool> {
        let candidate = self.candidate?;
        let edge = usize::from(self.polygon_edge);
        if edge == usize::from(self.gesture.len) {
            let selected = match candidate {
                WorldMarqueeCandidate::Point { .. } => self.winding[0] != 0,
                WorldMarqueeCandidate::Segment { .. } => {
                    if self.crossing.expect("marquee direction resolved") {
                        self.intersects || self.winding[0] != 0 || self.winding[1] != 0
                    } else {
                        self.winding[0] != 0 && self.winding[1] != 0
                    }
                }
                WorldMarqueeCandidate::Triangle { .. } => {
                    if self.crossing.expect("marquee direction resolved") {
                        self.intersects || self.winding.iter().any(|winding| *winding != 0)
                    } else {
                        self.winding.iter().all(|winding| *winding != 0)
                    }
                }
            };
            self.candidate = None;
            self.polygon_edge = 0;
            self.winding = [0; 3];
            self.intersects = false;
            return Some(selected);
        }
        let a = self.local_point(edge);
        let b = self.local_point((edge + 1) % usize::from(self.gesture.len));
        let points = match candidate {
            WorldMarqueeCandidate::Point { point, .. } => [point, point, point],
            WorldMarqueeCandidate::Segment { points, .. } => [points[0], points[1], points[1]],
            WorldMarqueeCandidate::Triangle { points, .. } => points,
        };
        let count = match candidate {
            WorldMarqueeCandidate::Point { .. } => 1,
            WorldMarqueeCandidate::Segment { .. } => 2,
            WorldMarqueeCandidate::Triangle { .. } => 3,
        };
        for (index, point) in points.iter().copied().take(count).enumerate() {
            if a[1] <= point[1] {
                if b[1] > point[1] && Self::orient(a, b, point) > 0.0 {
                    self.winding[index] += 1;
                }
            } else if b[1] <= point[1] && Self::orient(a, b, point) < 0.0 {
                self.winding[index] -= 1;
            }
        }
        match candidate {
            WorldMarqueeCandidate::Segment { points, .. } => {
                self.intersects |= Self::segments_intersect(points[0], points[1], a, b);
            }
            WorldMarqueeCandidate::Triangle { points, .. } => {
                self.intersects |= (0..3).any(|index| Self::segments_intersect(points[index], points[(index + 1) % 3], a, b));
            }
            WorldMarqueeCandidate::Point { .. } => {}
        }
        self.polygon_edge += 1;
        None
    }

    pub fn step(&mut self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.faulted {
            return WorldInteractionStep::Fault;
        }
        if self.complete {
            return WorldInteractionStep::Complete;
        }
        if self.gesture.revision != session_state.interaction_revision || self.generation != generation || !interaction_state.interaction_objects.terminal_for_revision(self.gesture.revision) {
            return WorldInteractionStep::Stale;
        }
        if self.crossing.is_none() {
            let index = usize::from(self.direction_index);
            if index < usize::from(self.gesture.len) {
                let start = self.local_point(0);
                let point = self.local_point(index);
                self.direction_index += 1;
                if (point[0] - start[0]).abs() >= 2.0 {
                    self.crossing = Some(point[0] < start[0]);
                }
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            let start = self.local_point(0);
            let end = self.local_point(usize::from(self.gesture.len - 1));
            self.crossing = Some(end[0] < start[0]);
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        if self.candidate.is_some() {
            let candidate = self.candidate.expect("retained marquee candidate");
            let Some(selected) = self.candidate_step() else {
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            };
            let token = self.current.expect("marquee candidate object");
            let component = match candidate {
                WorldMarqueeCandidate::Point { component, .. } | WorldMarqueeCandidate::Triangle { component, .. } => component,
                WorldMarqueeCandidate::Segment { component, .. } => Some(component),
            };
            if let Some(id) = component {
                let outcome = if selected { self.admit_candidate(interaction_state, WorldMarqueeResult::Component { object: token, id }) } else { WorldInteractionStep::Pending };
                context.consume_fuel(1);
                return outcome;
            }
            if !selected && !self.crossing.expect("crossing resolved") {
                self.current = None;
                self.topology = 0;
                self.any_visible = false;
            }
            let outcome = if selected && self.crossing.expect("crossing resolved") {
                self.current = None;
                self.topology = 0;
                self.any_visible = false;
                self.admit_candidate(interaction_state, WorldMarqueeResult::Object(token))
            } else {
                WorldInteractionStep::Pending
            };
            context.consume_fuel(1);
            return outcome;
        }
        if self.current.is_none() {
            let index = usize::from(self.slot);
            if index >= usize::from(interaction_state.interaction_objects.instance_len) {
                if self.results.page_len == 0 {
                    self.results.page_len = 1;
                }
                self.complete = true;
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            self.slot += 1;
            let Some(token) = interaction_state.interaction_objects.instance_order[index] else {
                return WorldInteractionStep::Fault;
            };
            let Some(entry) = interaction_state.interaction_objects.resolve(token).filter(|entry| entry.revision == self.gesture.revision && entry.kind == WorldInteractionObjectKind::Instance) else {
                return WorldInteractionStep::Stale;
            };
            if self.component_kind.is_some() && selection_state.active_object_id.as_deref().is_some_and(|active| active != entry.id.as_str()) {
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            self.current = Some(token);
            self.topology = 0;
            self.any_visible = false;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let token = self.current.expect("marquee current object");
        let Some(entry) = interaction_state.interaction_objects.resolve(token) else {
            return WorldInteractionStep::Stale;
        };
        let Some(mesh_token) = entry.mesh else {
            return WorldInteractionStep::Fault;
        };
        let Some(admitted) = interaction_state.interaction_meshes.resolve(mesh_token) else {
            return WorldInteractionStep::Stale;
        };
        let Some(&mesh) = scene_state.meshes.get(admitted.id.as_str()) else {
            return WorldInteractionStep::Fault;
        };
        let Ok(schema) = mesh.schema() else {
            return WorldInteractionStep::Stale;
        };
        if admitted.vertices != schema.vertices || admitted.edges != if schema.edge_ids == schema.edges { schema.edges } else { 0 } || admitted.triangles != schema.indices / 3 {
            return WorldInteractionStep::Stale;
        }
        let crossing = self.crossing.expect("crossing resolved");
        if let Some(kind) = self.component_kind {
            let count = match kind {
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
            let id = match kind {
                WorldComponentKind::Vertex => world_mesh_component_id(mesh, Mesh3dField::VertexIds, index),
                WorldComponentKind::Edge => world_mesh_component_id(mesh, Mesh3dField::EdgeIds, index),
                WorldComponentKind::Face => world_mesh_component_id(mesh, Mesh3dField::FaceIds, index),
            };
            if kind == WorldComponentKind::Vertex && id == u32::MAX {
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            }
            let mut selected = false;
            match kind {
                WorldComponentKind::Vertex => {
                    let Ok(point) = mesh.vec3(Mesh3dField::Positions, index) else {
                        return WorldInteractionStep::Stale;
                    };
                    let world = entry.model.transform_point(Vec3::new(point[0], point[1], point[2]));
                    let Some(screen) = ui_scene::math::projection_spec_project_point(self.view_projection, self.projection_spec, world, self.viewport.w, self.viewport.h) else {
                        context.consume_fuel(1);
                        return WorldInteractionStep::Pending;
                    };
                    if self.rectangle {
                        selected = Self::rect_contains(self.rect(), screen);
                    } else {
                        self.begin_candidate(WorldMarqueeCandidate::Point { point: screen, component: Some(id) });
                    }
                }
                WorldComponentKind::Edge => {
                    let Ok(edge) = mesh.edge(index) else {
                        return WorldInteractionStep::Stale;
                    };
                    let a = entry.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                    let b = entry.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                    let Some(segment) = ui_scene::math::projection_spec_project_segment(self.view_projection, self.projection_spec, a, b, self.viewport.w, self.viewport.h) else {
                        context.consume_fuel(1);
                        return WorldInteractionStep::Pending;
                    };
                    let [screen_a, screen_b] = segment.screen;
                    if self.rectangle {
                        selected = if crossing {
                            Self::rect_contains(self.rect(), screen_a) || Self::rect_contains(self.rect(), screen_b) || {
                                let rect = self.rect();
                                let corners = [[rect[0], rect[1]], [rect[2], rect[1]], [rect[2], rect[3]], [rect[0], rect[3]]];
                                (0..4).any(|side| Self::segments_intersect(screen_a, screen_b, corners[side], corners[(side + 1) % 4]))
                            }
                        } else {
                            Self::rect_contains(self.rect(), screen_a) && Self::rect_contains(self.rect(), screen_b)
                        };
                    } else {
                        self.begin_candidate(WorldMarqueeCandidate::Segment { points: [screen_a, screen_b], component: id });
                    }
                }
                WorldComponentKind::Face => {
                    let Some(indices) = world_mesh_triangle(mesh, index) else {
                        return WorldInteractionStep::Stale;
                    };
                    let mut points = [[0.0; 2]; 3];
                    for corner in 0..3 {
                        let Some(world) = world_mesh_vertex(mesh, indices[corner]).map(|vertex| entry.model.transform_point(vertex)) else {
                            return WorldInteractionStep::Stale;
                        };
                        let Some(screen) = ui_scene::math::projection_spec_project_point(self.view_projection, self.projection_spec, world, self.viewport.w, self.viewport.h) else {
                            context.consume_fuel(1);
                            return WorldInteractionStep::Pending;
                        };
                        points[corner] = screen;
                    }
                    if self.rectangle {
                        selected = if crossing { Self::triangle_intersects_rect(points, self.rect()) } else { points.iter().all(|point| Self::rect_contains(self.rect(), *point)) };
                    } else {
                        self.begin_candidate(WorldMarqueeCandidate::Triangle { points, component: Some(id) });
                    }
                }
            }
            let outcome = if selected { self.admit_candidate(interaction_state, WorldMarqueeResult::Component { object: token, id }) } else { WorldInteractionStep::Pending };
            context.consume_fuel(1);
            return outcome;
        }
        if !crossing {
            if self.topology >= admitted.vertices {
                self.current = None;
                self.topology = 0;
                let selected = self.any_visible;
                self.any_visible = false;
                let outcome = if selected { self.admit_candidate(interaction_state, WorldMarqueeResult::Object(token)) } else { WorldInteractionStep::Pending };
                context.consume_fuel(1);
                return outcome;
            }
            let index = self.topology;
            self.topology += 1;
            let Some(world) = world_mesh_vertex(mesh, index).map(|vertex| entry.model.transform_point(vertex)) else {
                return WorldInteractionStep::Stale;
            };
            let Some(point) = ui_scene::math::projection_spec_project_point(self.view_projection, self.projection_spec, world, self.viewport.w, self.viewport.h) else {
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            };
            self.any_visible = true;
            if self.rectangle {
                if !Self::rect_contains(self.rect(), point) {
                    self.current = None;
                    self.topology = 0;
                    self.any_visible = false;
                }
            } else {
                self.begin_candidate(WorldMarqueeCandidate::Point { point, component: None });
            }
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        if self.topology >= admitted.triangles {
            self.current = None;
            self.topology = 0;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        let triangle = self.topology;
        self.topology += 1;
        let Some(indices) = world_mesh_triangle(mesh, triangle) else {
            return WorldInteractionStep::Stale;
        };
        let mut points = [[0.0; 2]; 3];
        for index in 0..3 {
            let Some(world) = world_mesh_vertex(mesh, indices[index]).map(|vertex| entry.model.transform_point(vertex)) else {
                return WorldInteractionStep::Stale;
            };
            let Some(point) = ui_scene::math::projection_spec_project_point(self.view_projection, self.projection_spec, world, self.viewport.w, self.viewport.h) else {
                context.consume_fuel(1);
                return WorldInteractionStep::Pending;
            };
            points[index] = point;
        }
        let selected = if self.rectangle {
            Self::triangle_intersects_rect(points, self.rect())
        } else {
            self.begin_candidate(WorldMarqueeCandidate::Triangle { points, component: None });
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        };
        let outcome = if selected {
            self.current = None;
            self.topology = 0;
            self.admit_candidate(interaction_state, WorldMarqueeResult::Object(token))
        } else {
            WorldInteractionStep::Pending
        };
        context.consume_fuel(1);
        outcome
    }

    pub fn close_step(&mut self) -> bool {
        if self.candidate.take().is_some() {
            self.polygon_edge = 0;
            self.winding = [0; 3];
            self.intersects = false;
            return false;
        }
        if self.current.take().is_some() {
            self.topology = 0;
            self.any_visible = false;
            return false;
        }
        if !self.results.close_step() {
            return false;
        }
        self.gesture.close_step()
    }

    #[expect(clippy::result_large_err, reason = "Rejected transfer returns the exact admitted owner for bounded retirement without allocating on the failure path.")]
    pub fn finish(self, session_state: &WorldSessionState, generation: u64) -> Result<(WorldMarqueeGesture, WorldMarqueeResultPages), (Self, WorldInteractionStep)> {
        if !self.complete {
            return Err((self, WorldInteractionStep::Pending));
        }
        if self.gesture.revision != session_state.interaction_revision || self.generation != generation {
            return Err((self, WorldInteractionStep::Stale));
        }
        Ok((self.gesture, self.results))
    }
}

pub struct WorldMarqueePublishJob {
    pub generation: u64,
    pub gesture: WorldMarqueeGesture,
    pub results: WorldMarqueeResultPages,
    pub merge: &'static str,
    pub prepared: Option<semio_framework_ui_action::PreparedClaimedActionBatch>,
    pub draft: Option<semio_framework_ui_action::BoundedClaimedActionDraft>,
    /// 🎯️ The page's `targets` value BEING BUILT — one JSON-encoded string, assembled one target per
    /// bounded step. See [`INTERACTION_TARGETS_OPEN`] for why the arg is text rather than an array.
    pub targets: Box<[u8; semio_framework_ui_action::ACTION_STRING_BYTE_CAPACITY]>,
    pub targets_len: u16,
    pub page: u8,
    pub stage: u16,
    pub published: bool,
}

/// 🔎️ First index of `needle` in `haystack`, or `None` — a fixed-window scan, no allocation, for
/// [`WorldMarqueePublishJob::targets_contain`].
fn world_find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    (0..=haystack.len() - needle.len()).find(|start| &haystack[*start..*start + needle.len()] == needle)
}

impl WorldMarqueePublishJob {
    pub fn new(generation: u64, gesture: WorldMarqueeGesture, results: WorldMarqueeResultPages, shift: bool, ctrl: bool) -> Self {
        let merge = world_merge_mode(shift, ctrl).wire_label();
        Self { generation, gesture, results, merge, prepared: None, draft: None, targets: Box::new([0; semio_framework_ui_action::ACTION_STRING_BYTE_CAPACITY]), targets_len: 0, page: 0, stage: 0, published: false }
    }

    fn push_targets(&mut self, parts: &[&str]) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
        let mut len = usize::from(self.targets_len);
        for part in parts {
            let end = len.checked_add(part.len()).filter(|end| *end <= semio_framework_ui_action::ACTION_STRING_BYTE_CAPACITY).ok_or(semio_framework_ui_action::BoundedActionFault::ByteCredits)?;
            self.targets[len..end].copy_from_slice(part.as_bytes());
            len = end;
        }
        self.targets_len = len as u16;
        Ok(())
    }

    fn targets_text(&self) -> Result<&str, semio_framework_ui_action::BoundedActionFault> {
        core::str::from_utf8(&self.targets[..usize::from(self.targets_len)]).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)
    }

    /// 🎯️ Whether this page's `targets` already names the exact `(granularity, id)` pair.
    ///
    /// ⚖️ A selection is a SET of topology target pairs: several instances can share one target,
    /// while the same id under two declared granularities remains two distinct targets. React's
    /// `world3dInstanceInteractionTargets` dedups on the same rule, and
    /// `🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json`'s `marquee-release-replaces` case pins it.
    fn targets_contain(&self, granularity: &str, id: &[&str]) -> bool {
        let text = &self.targets[..usize::from(self.targets_len)];
        let mut cursor = 0usize;
        while let Some(found) = world_find_bytes(&text[cursor..], INTERACTION_TARGETS_OPEN.as_bytes()) {
            let start = cursor + found;
            cursor = start + INTERACTION_TARGETS_OPEN.len();
            let mut offset = start;
            let matches = [INTERACTION_TARGETS_OPEN, granularity, INTERACTION_TARGETS_MIDDLE]
                .into_iter()
                .chain(id.iter().copied())
                .chain([INTERACTION_TARGETS_TAIL])
                .all(|part| {
                    let Some(end) = offset.checked_add(part.len()) else { return false };
                    let matched = text.get(offset..end) == Some(part.as_bytes());
                    offset = end;
                    matched
                });
            if matches {
                return true;
            }
        }
        false
    }

    /// 🎯️ The `targets` VALUE is JSON text now, so its own punctuation is part of the reservation —
    /// including the two bytes of `[]` a page with NO targets still writes. Charging the array
    /// delimiters per target instead of once left an empty page two bytes short, and the shortfall
    /// surfaced on the LAST string of the action (`method`), faulting the whole publish
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-world3d-interaction-2026-09-13.md`).
    pub fn page_credit(&self, session_state: &WorldSessionState, interaction_state: &WorldInteractionState, page: usize) -> Result<usize, semio_framework_ui_action::BoundedActionFault> {
        let target_count = usize::from(self.results.lens[page]);
        let target_keys = target_count
            .checked_mul(INTERACTION_TARGETS_OPEN.len() + INTERACTION_TARGETS_MIDDLE.len() + INTERACTION_TARGETS_TAIL.len() + 1)
            .and_then(|bytes| bytes.checked_add(INTERACTION_TARGETS_ARRAY_OPEN.len() + INTERACTION_TARGETS_ARRAY_CLOSE.len()))
            .ok_or(semio_framework_ui_action::BoundedActionFault::ByteCredits)?;
        let granularities = target_count.checked_mul(maximum_instance_interaction_granularity_bytes(session_state, interaction_state)).ok_or(semio_framework_ui_action::BoundedActionFault::ByteCredits)?;
        let id_prefix = if session_state.bound_domain_id.is_some() { 0 } else { target_count.checked_mul(session_state.surface_id.len() + WORLD_ITEM_PATH_DELIMITER.len()).ok_or(semio_framework_ui_action::BoundedActionFault::ByteCredits)? };
        semio_framework_ui_action::checked_action_string_bytes(&[&session_state.controller_id, "interactionSelect", "domainId", resolved_domain_id(session_state), "targets", "merge", self.merge, "method", selection_method_wire_str(SelectionMethod::Rectangle)])?
            .checked_add(target_keys)
            .and_then(|bytes| bytes.checked_add(granularities))
            .and_then(|bytes| bytes.checked_add(id_prefix))
            .and_then(|bytes| bytes.checked_add(usize::from(self.results.id_bytes[page])))
            .filter(|bytes| *bytes <= semio_framework_ui_action::ACTION_ITEM_BYTE_CAPACITY)
            .ok_or(semio_framework_ui_action::BoundedActionFault::ByteCredits)
    }

    /// 💥️ `then`, never `then_some`: `then_some`'s argument is evaluated EAGERLY, so `self.stage - 3`
    /// ran on stages 0, 1 and 2 as well and underflowed the `u16` — an arithmetic panic that aborted
    /// the wasm instance. Measured on 6118: every marquee release on the World3d preview killed the
    /// renderer outright, and because the Worker installed no panic hook the only visible symptom was
    /// the next `#[wasm_bindgen] &mut self` call failing with `recursive use of an object`
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-world3d-interaction-2026-09-13.md`).
    ///
    /// 🎯️ One TARGET per bounded step, appended to this page's `targets` text — the whole value is
    /// handed to the builder once, at `target_stage_end`. The id is the TOPOLOGY target the rendered
    /// instance stands for, deduplicated, exactly as a pick's is.
    pub fn step(&mut self, session_state: &WorldSessionState, interaction_state: &WorldInteractionState, generation: u64, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> Result<WorldInteractionStep, semio_framework_ui_action::BoundedActionFault> {
        if context.should_yield() {
            return Ok(WorldInteractionStep::Pending);
        }
        if self.gesture.revision != session_state.interaction_revision || self.generation != generation {
            return Ok(WorldInteractionStep::Stale);
        }
        if self.published {
            if !self.results.close_step() {
                context.consume_fuel(1);
                return Ok(WorldInteractionStep::Pending);
            }
            if !self.gesture.close_step() {
                context.consume_fuel(1);
                return Ok(WorldInteractionStep::Pending);
            }
            return Ok(WorldInteractionStep::Complete);
        }
        if self.prepared.is_none() {
            let mut credits = [0usize; WORLD_MARQUEE_RESULT_PAGE_COUNT];
            for (page, credit) in credits.iter_mut().enumerate().take(usize::from(self.results.page_len)) {
                *credit = self.page_credit(session_state, interaction_state, page)?;
            }
            let claims = input.claim_actions(&credits[..usize::from(self.results.page_len)])?;
            self.prepared = Some(semio_framework_ui_action::PreparedClaimedActionBatch::new(claims));
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if usize::from(self.page) == usize::from(self.results.page_len) {
            input.publish_prepared_claimed_actions(self.prepared.take().expect("terminal marquee claimed batch"))?;
            self.published = true;
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        let page = usize::from(self.page);
        if self.draft.is_none() {
            let claim = self.prepared.as_ref().and_then(|batch| batch.claim(page)).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
            self.draft = Some(input.draft_claimed_action(claim, &session_state.controller_id, "interactionSelect")?);
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        let target_count = u16::from(self.results.lens[page]);
        let target_stage_end = 3 + target_count;
        let target_index = (self.stage >= 3 && self.stage < target_stage_end).then(|| usize::from(self.stage - 3));
        let token = target_index
            .and_then(|index| self.results.pages[page][index])
            .map(|result| match result {
                WorldMarqueeResult::Object(token) => Ok(token),
                WorldMarqueeResult::Component { .. } => Err(semio_framework_ui_action::BoundedActionFault::Structure),
            })
            .transpose()?;
        let entry = token.map(|token| interaction_state.interaction_objects.resolve(token).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)).transpose()?;
        if self.stage >= 3 && self.stage < target_stage_end {
            let entry = entry.expect("marquee token resolved");
            let id = instance_interaction_id(interaction_state, entry.id.as_str());
            let granularity = instance_interaction_granularity_id(session_state, interaction_state, entry.id.as_str());
            let separator: &str = if self.targets_len > 1 { "," } else { "" };
            let target: [&str; 8] = if session_state.bound_domain_id.is_some() {
                [separator, INTERACTION_TARGETS_OPEN, granularity, INTERACTION_TARGETS_MIDDLE, id, "", "", INTERACTION_TARGETS_TAIL]
            } else {
                [separator, INTERACTION_TARGETS_OPEN, granularity, INTERACTION_TARGETS_MIDDLE, &session_state.surface_id, WORLD_ITEM_PATH_DELIMITER, id, INTERACTION_TARGETS_TAIL]
            };
            if !self.targets_contain(granularity, &target[4..7]) {
                self.push_targets(&target)?;
            }
            self.stage += 1;
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if self.stage == 2 {
            self.targets_len = 0;
            self.push_targets(&["["])?;
            self.stage += 1;
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        if self.stage == target_stage_end {
            self.push_targets(&["]"])?;
            let targets = core::str::from_utf8(&self.targets[..usize::from(self.targets_len)]).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)?;
            self.draft.as_mut().expect("marquee page draft").builder().string(Some("targets"), targets)?;
            self.stage += 1;
            context.consume_fuel(1);
            return Ok(WorldInteractionStep::Pending);
        }
        let draft = self.draft.as_mut().expect("marquee page draft");
        match self.stage {
            0 => draft.builder().begin_object(None)?,
            1 => draft.builder().string(Some("domainId"), resolved_domain_id(session_state))?,
            stage if stage == target_stage_end + 1 => draft.builder().string(Some("merge"), self.merge)?,
            stage if stage == target_stage_end + 2 => draft.builder().string(Some("method"), selection_method_wire_str(SelectionMethod::Rectangle))?,
            stage if stage == target_stage_end + 3 => draft.builder().end_container()?,
            _ => {
                let action = self.draft.take().expect("terminal marquee page draft").finish()?;
                self.prepared.as_mut().expect("marquee claimed batch").push(action)?;
                self.page += 1;
                self.stage = 0;
                context.consume_fuel(1);
                return Ok(WorldInteractionStep::Pending);
            }
        }
        self.stage += 1;
        context.consume_fuel(1);
        Ok(WorldInteractionStep::Pending)
    }

    pub fn close_step(&mut self, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
        if self.draft.take().is_some() {
            return false;
        }
        if let Some(prepared) = self.prepared.as_mut() {
            if let Some(claim) = prepared.take_last_claim() {
                if input.release_action_claim(claim).is_err() {
                    prepared.restore_last_claim(claim).expect("marquee close restores retained claim slot");
                    return false;
                }
                return false;
            }
            self.prepared = None;
            return false;
        }
        if !self.results.close_step() {
            return false;
        }
        self.gesture.close_step()
    }
}

pub struct WorldComponentMarqueePublishJob {
    pub generation: u64,
    pub gesture: WorldMarqueeGesture,
    pub results: WorldMarqueeResultPages,
    pub kind: WorldComponentKind,
    pub merge: &'static str,
    pub targets: Box<[u8; semio_framework_ui_action::ACTION_STRING_BYTE_CAPACITY]>,
    pub targets_len: u16,
    result_cursor: u8,
    targets_ready: bool,
    pub claim: Option<semio_framework_ui_action::BoundedActionClaim>,
    pub draft: Option<semio_framework_ui_action::BoundedClaimedActionDraft>,
    pub prepared: Option<semio_framework_ui_action::PreparedClaimedAction>,
    pub stage: u16,
    pub published: bool,
}

impl WorldComponentMarqueePublishJob {
    pub fn new(generation: u64, gesture: WorldMarqueeGesture, results: WorldMarqueeResultPages, kind: WorldComponentKind, shift: bool, ctrl: bool) -> Self {
        Self { generation, gesture, results, kind, merge: world_merge_mode(shift, ctrl).wire_label(), targets: Box::new([0; semio_framework_ui_action::ACTION_STRING_BYTE_CAPACITY]), targets_len: 0, result_cursor: 0, targets_ready: false, claim: None, draft: None, prepared: None, stage: 0, published: false }
    }

    fn push_targets(&mut self, text: &str) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
        let end = usize::from(self.targets_len).checked_add(text.len()).filter(|end| *end <= self.targets.len()).ok_or(semio_framework_ui_action::BoundedActionFault::StringCredits)?;
        self.targets[usize::from(self.targets_len)..end].copy_from_slice(text.as_bytes()); self.targets_len = end as u16; Ok(())
    }

    fn targets_text(&self) -> &str { std::str::from_utf8(&self.targets[..usize::from(self.targets_len)]).expect("component targets retain valid UTF-8") }

    fn targets_step(&mut self, session_state: &WorldSessionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState) -> Result<bool, semio_framework_ui_action::BoundedActionFault> {
        if self.targets_len == 0 { self.push_targets("[")?; return Ok(false); }
        let index = usize::from(self.result_cursor);
        if index < usize::from(self.results.lens[0]) {
            let Some(WorldMarqueeResult::Component { object, id }) = self.results.pages[0][index] else { return Err(semio_framework_ui_action::BoundedActionFault::Structure); };
            let entry = interaction_state.interaction_objects.resolve(object).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
            let target = WorldComponentTarget::new(interaction_state, scene_state, entry, self.kind, id)?;
            let mut row = WorldInteractionText::default();
            let parts = if session_state.bound_domain_id.is_some() { ["", "", target.as_str()] } else { [&session_state.surface_id, WORLD_ITEM_PATH_DELIMITER, target.as_str()] };
            world_write_interaction_target(&mut row, self.kind.as_str(), &parts).map_err(|_| semio_framework_ui_action::BoundedActionFault::StringCredits)?;
            if index > 0 { self.push_targets(",")?; }
            self.push_targets(row.as_str())?;
            self.result_cursor += 1;
            return Ok(false);
        }
        self.push_targets("]")?;
        Ok(true)
    }

    pub fn step(&mut self, session_state: &WorldSessionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, generation: u64, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> Result<WorldInteractionStep, semio_framework_ui_action::BoundedActionFault> {
        if context.should_yield() { return Ok(WorldInteractionStep::Pending); }
        if self.gesture.revision != session_state.interaction_revision || self.generation != generation { return Ok(WorldInteractionStep::Stale); }
        if self.published {
            if self.targets_len > 0 { self.targets_len = 0; context.consume_fuel(1); return Ok(WorldInteractionStep::Pending); }
            if !self.results.close_step() || !self.gesture.close_step() { context.consume_fuel(1); return Ok(WorldInteractionStep::Pending); }
            return Ok(WorldInteractionStep::Complete);
        }
        if !self.targets_ready { self.targets_ready = self.targets_step(session_state, interaction_state, scene_state)?; context.consume_fuel(1); return Ok(WorldInteractionStep::Pending); }
        if self.claim.is_none() {
            let credits = semio_framework_ui_action::checked_action_string_bytes(&[&session_state.controller_id, "interactionSelect", "domainId", resolved_domain_id(session_state), "targets", self.targets_text(), "merge", self.merge, "method", selection_method_wire_str(SelectionMethod::Rectangle)])?;
            self.claim = Some(input.claim_action(credits)?); context.consume_fuel(1); return Ok(WorldInteractionStep::Pending);
        }
        if self.draft.is_none() && self.prepared.is_none() { self.draft = Some(input.draft_claimed_action(self.claim.expect("component marquee claim"), &session_state.controller_id, "interactionSelect")?); context.consume_fuel(1); return Ok(WorldInteractionStep::Pending); }
        if let Some(prepared) = self.prepared.take() { input.publish_prepared_claimed_action(prepared)?; self.claim = None; self.published = true; context.consume_fuel(1); return Ok(WorldInteractionStep::Pending); }
        let targets = std::str::from_utf8(&self.targets[..usize::from(self.targets_len)]).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)?;
        let draft = self.draft.as_mut().expect("component marquee draft");
        match self.stage {
            0 => draft.builder().begin_object(None)?,
            1 => draft.builder().string(Some("domainId"), resolved_domain_id(session_state))?,
            2 => draft.builder().string(Some("targets"), targets)?,
            3 => draft.builder().string(Some("merge"), self.merge)?,
            4 => draft.builder().string(Some("method"), selection_method_wire_str(SelectionMethod::Rectangle))?,
            5 => draft.builder().end_container()?,
            _ => { self.prepared = Some(self.draft.take().expect("terminal component marquee draft").finish()?); context.consume_fuel(1); return Ok(WorldInteractionStep::Pending); }
        }
        self.stage += 1; context.consume_fuel(1); Ok(WorldInteractionStep::Pending)
    }

    pub fn close_step(&mut self, input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> bool {
        if self.prepared.take().is_some() || self.draft.take().is_some() { return false; }
        if let Some(claim) = self.claim.take() { if input.release_action_claim(claim).is_err() { self.claim = Some(claim); } return false; }
        if self.targets_len > 0 { self.targets_len = 0; return false; }
        if !self.results.close_step() { return false; }
        self.gesture.close_step()
    }
}

impl WorldMarqueeGesture {
    pub fn new(revision: u64, generation: u64, point: [f32; 2]) -> Self {
        let mut points = Box::new([None; WORLD_INTERACTION_MARQUEE_POINT_CAPACITY]);
        points[0] = Some(point);
        Self { revision, start_generation: generation, points, len: 1, retiring: false }
    }

    pub fn push(&mut self, point: [f32; 2]) -> bool {
        let index = usize::from(self.len);
        if self.retiring || index == WORLD_INTERACTION_MARQUEE_POINT_CAPACITY {
            return false;
        }
        self.points[index] = Some(point);
        self.len += 1;
        true
    }

    pub fn is_click(&self, end: [f32; 2]) -> bool {
        let start = self.points[0].expect("marquee gesture owns start point");
        let dx = end[0] - start[0];
        let dy = end[1] - start[1];
        (dx * dx + dy * dy).sqrt() <= CLICK_DRAG_THRESHOLD_PX
    }

    pub fn close_step(&mut self) -> bool {
        self.retiring = true;
        if self.len == 0 {
            return true;
        }
        self.len -= 1;
        self.points[usize::from(self.len)] = None;
        false
    }
}
//#endregion 🧵️WorldInteractionTransaction
