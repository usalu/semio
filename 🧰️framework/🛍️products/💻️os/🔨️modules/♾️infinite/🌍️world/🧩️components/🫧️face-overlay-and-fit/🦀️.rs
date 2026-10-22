//! 🫧️ World3d camera fit, gumball config, brush runs, face overlay cursors, prepared status and snapshot apply cursor.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct World3dCameraFitRequest {
    pub revision: u32,
    pub padding: f32,
    pub bounds: Option<([f32; 3], [f32; 3])>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct World3dCameraFitOwner {
    pub revision: u32,
    pub camera_digest: Option<u64>,
}

#[derive(Clone, Copy, Debug)]
pub struct World3dCameraBoundsCursor {
    pub key: u64,
    pub draw_generation: u64,
    pub geometry_generation: u64,
    pub draw: u16,
    pub instance: u32,
    pub corner: u8,
    pub minimum: [f32; 3],
    pub maximum: [f32; 3],
    pub any: bool,
}

impl World3dCameraBoundsCursor {
    pub fn new(key: u64, draw_generation: u64, geometry_generation: u64) -> Self {
        Self { key, draw_generation, geometry_generation, draw: 0, instance: 0, corner: 0, minimum: [f32::INFINITY; 3], maximum: [f32::NEG_INFINITY; 3], any: false }
    }

    /// 💤️ A completed empty generation parks on the draw index no bounded registry can publish.
    /// Reusing this existing scalar preserves the cursor's `Option` niche and the measured surface
    /// table receipt; another boolean enlarged every admitted World state by eight bytes.
    pub fn empty_generation(self) -> bool {
        self.draw == u16::MAX
    }

    pub fn expand(&mut self, point: Vec3) {
        for (axis, value) in [point.x, point.y, point.z].into_iter().enumerate() {
            self.minimum[axis] = self.minimum[axis].min(value);
            self.maximum[axis] = self.maximum[axis].max(value);
        }
        self.any = true;
    }
}

/// 🎛️ `selection_json.gumballConfig` — React's `GumballConfig`
/// (`🌐️World3dHost/🟦️.tsx:236`, `gumballConfigForTransformMode`).
///
/// ⚖️ Every flag is `Option<bool>`, not `bool`: React's `resolveGumballConfig` reads each group as
/// `config?.group !== false`, so an ABSENT key means ENABLED and only an explicit `false` hides a
/// group. A plain `#[serde(default)] bool` would read every authored config as "all groups off".
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct World3dSceneGumballConfigRecord {
    #[serde(default)]
    move_axes: Option<bool>,
    #[serde(default)]
    move_planes: Option<bool>,
    #[serde(default)]
    rotate: Option<bool>,
    #[serde(default)]
    scale_axes: Option<bool>,
    #[serde(default)]
    scale_planes: Option<bool>,
    #[serde(default)]
    scale_uniform: Option<bool>,
    /// 📐️ The planar subset a window projection implies (`xy`/`xz`/`yz`), from React's
    /// `worldGumballConfigForProjection`. `None` offers all three axes.
    #[serde(default)]
    pub plane: Option<String>,
}

impl World3dSceneGumballConfigRecord {
    /// 🎛️ React's `gumballConfigForTransformMode` (`🌐️World3dHost/🟦️.tsx:2683`) — the single-mode
    /// fallback when a producer authors no `gumballConfig`.
    ///
    /// ⚖️ React switches on three literals (`transform`/`rotate`/`scale`) and falls through to the
    /// move config; this host's own `transform_mode` also carries the verb spellings
    /// (`rotateSelection`/`scaleSelection`, and the default `translate`), so the arms match on the
    /// PREFIX. Same three groups, same fall-through.
    pub fn for_transform_mode(mode: &str) -> Self {
        let flags = |move_axes, move_planes, rotate, scale_axes, scale_planes, scale_uniform| Self {
            move_axes: Some(move_axes),
            move_planes: Some(move_planes),
            rotate: Some(rotate),
            scale_axes: Some(scale_axes),
            scale_planes: Some(scale_planes),
            scale_uniform: Some(scale_uniform),
            plane: None,
        };
        if mode.starts_with("transform") {
            return flags(true, true, true, false, false, false);
        }
        if mode.starts_with("rotate") {
            return flags(false, false, true, false, false, false);
        }
        if mode.starts_with("scale") {
            return flags(false, false, false, true, true, true);
        }
        flags(true, true, false, false, false, false)
    }

    /// 🎛️ React's `gumballHandleEnabled` — the handle's own group flag ∩ the drafting-plane subset
    /// (`🧱️elements/🎬️Scene/🟦️.tsx:489-506`, `GUMBALL_PLANE_HANDLES`).
    pub fn admits(&self, handle: GumballHandle) -> bool {
        let group = match handle {
            GumballHandle::MoveX | GumballHandle::MoveY | GumballHandle::MoveZ => self.move_axes,
            GumballHandle::MoveXY | GumballHandle::MoveYZ | GumballHandle::MoveXZ => self.move_planes,
            GumballHandle::RotateX | GumballHandle::RotateY | GumballHandle::RotateZ => self.rotate,
            GumballHandle::ScaleX | GumballHandle::ScaleY | GumballHandle::ScaleZ => self.scale_axes,
        };
        if group == Some(false) {
            return false;
        }
        match self.plane.as_deref() {
            None => true,
            Some("xy") => matches!(handle, GumballHandle::MoveX | GumballHandle::MoveY | GumballHandle::MoveXY | GumballHandle::RotateZ | GumballHandle::ScaleX | GumballHandle::ScaleY),
            Some("yz") => matches!(handle, GumballHandle::MoveY | GumballHandle::MoveZ | GumballHandle::MoveYZ | GumballHandle::RotateX | GumballHandle::ScaleY | GumballHandle::ScaleZ),
            Some("xz") => matches!(handle, GumballHandle::MoveX | GumballHandle::MoveZ | GumballHandle::MoveXZ | GumballHandle::RotateY | GumballHandle::ScaleX | GumballHandle::ScaleZ),
            Some(_) => true,
        }
    }
}

/// 🥽️ One page of a `registerBrushMesh` upload run this surface owns. `bytes` is the mesh's whole
/// wire payload — little-endian `f32` positions followed by little-endian `u32` indices, the exact
/// layout the guest's `decode_brush_mesh_page_values` reassembles and its `brush_mesh_digest`
/// hashes — read back once out of the resident [`Mesh3dLease`] when the run opened.
pub struct WorldBrushMeshRun {
    pub url: String,
    pub digest: String,
    pub bytes: Vec<u8>,
    pub position_values: usize,
    pub capacity: usize,
    pub page: u32,
    pub page_count: u32,
    pub revision: u64,
}

pub const WORLD_REFERENCE_SOURCE_PIXELS: u64 = 64 * 1024 * 1024 / 4;

/// 🎨️ The existing owned image decoder preserves admitted mesh texture pixels without reference-image resizing.
pub fn decode_mesh_surface_image_bytes(bytes:&[u8])->Option<semio_framework_pixels::RasterImage> {
    use image::ImageDecoder as _;
    let reader=image::ImageReader::new(std::io::Cursor::new(bytes)).with_guessed_format().ok()?;let mut decoder=reader.into_decoder().ok()?;
    let (width,height)=decoder.dimensions();if u64::from(width).checked_mul(u64::from(height))?>WORLD_REFERENCE_SOURCE_PIXELS{return None}
    let orientation=decoder.orientation().unwrap_or(image::metadata::Orientation::NoTransforms);let mut image=image::DynamicImage::from_decoder(decoder).ok()?;image.apply_orientation(orientation);let rgba=image.to_rgba8();
    Some(semio_framework_pixels::RasterImage{width:rgba.width(),height:rgba.height(),pixels:rgba.into_raw()})
}

pub const FACE_OVERLAY_OFFSET: f32 = 0.003;

pub fn pick_targets_instance(selection_state: &WorldSelectionState, instance_id: &str) -> bool {
    selection_state.active_object_id.as_deref().is_none_or(|active_id| active_id == instance_id)
}

pub fn mesh_vertex(mesh: Mesh3dLease, index: u32) -> Option<Vec3> {
    world_mesh_vertex(mesh, index)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldFaceOverlayPhase {
    Count,
    Prepare,
    Allocate,
    Geometry,
    Indices,
    Seal,
    Publish,
    NextBucket,
    Complete,
}

#[derive(Clone, Copy)]
pub struct WorldFaceOverlayMatch {
    pub draw: u16,
    pub instance: u16,
    pub triangle: u32,
    pub category: u8,
    pub hovered: bool,
}

#[derive(Clone, Copy, Default)]
pub struct WorldFaceOverlayScan {
    pub draw: u16,
    pub instance: u16,
    pub triangle: u32,
    pub candidate: Option<u32>,
    pub hovered: bool,
    pub preview: usize,
    pub selected: usize,
}

pub enum WorldFaceOverlayScanStep {
    Pending,
    Match(WorldFaceOverlayMatch),
    Complete,
}

impl WorldFaceOverlayScan {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn finish_candidate(&mut self, category: Option<u8>) -> WorldFaceOverlayScanStep {
        let matched = category.map(|category| WorldFaceOverlayMatch { draw: self.draw, instance: self.instance, triangle: self.triangle, category, hovered: self.hovered });
        self.triangle = self.triangle.saturating_add(1);
        self.candidate = None;
        self.hovered = false;
        self.preview = 0;
        self.selected = 0;
        matched.map_or(WorldFaceOverlayScanStep::Pending, WorldFaceOverlayScanStep::Match)
    }

    pub fn step(&mut self, selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState) -> WorldFaceOverlayScanStep {
        if let Some(face_id) = self.candidate {
            let instance = &scene_state.draws.get(usize::from(self.draw)).expect("retained overlay draw").instances[usize::from(self.instance)];
            if self.preview < selection_state.marquee_preview_ids.len() {
                let matches = selection_state.active_object_id.as_deref()==Some(&instance.id) && decimal_component_id_matches(&selection_state.marquee_preview_ids[self.preview], face_id);
                self.preview += 1;
                return if matches { self.finish_candidate(Some(0)) } else { WorldFaceOverlayScanStep::Pending };
            }
            if self.hovered {
                return self.finish_candidate(Some(1));
            }
            if self.selected < gumball_state.gumball_selection_ids.len() {
                let matches = world_component_preview_target(scene_state,&gumball_state.gumball_selection_ids[self.selected],"face").is_some_and(|(object,group)|object==instance.id && group==face_id);
                self.selected += 1;
                return if matches { self.finish_candidate(Some(2)) } else { WorldFaceOverlayScanStep::Pending };
            }
            let numeric = self.selected - gumball_state.gumball_selection_ids.len();
            if instance.component_source.is_none() && selection_state.active_object_id.as_deref()==Some(&instance.id) && numeric < selection_state.component_ids.len() {
                let matches = decimal_component_id_matches(&selection_state.component_ids[numeric],face_id);
                self.selected += 1;
                return if matches { self.finish_candidate(Some(2)) } else { WorldFaceOverlayScanStep::Pending };
            }
            return self.finish_candidate(None);
        }
        if usize::from(self.draw) >= usize::from(scene_state.draws.len) {
            return WorldFaceOverlayScanStep::Complete;
        }
        let Some(draw) = scene_state.draws.get(usize::from(self.draw)) else {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        };
        if usize::from(self.instance) >= draw.instances.len() {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        }
        let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        };
        let Ok(schema) = mesh.schema() else {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        };
        if self.triangle >= schema.indices / 3 {
            self.instance += 1;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        }
        let face_id = world_mesh_component_id(mesh, Mesh3dField::FaceIds, self.triangle);
        self.candidate = Some(face_id);
        self.hovered = instance_hovered_component_matches(selection_state, &draw.instances[usize::from(self.instance)].id, face_id);
        WorldFaceOverlayScanStep::Pending
    }
}

#[derive(Clone, Copy)]
pub struct WorldFaceOverlayGeometry {
    pub vertices: [Vec3; 3],
    pub normal: Vec3,
}

pub enum WorldFaceOverlayMeshStep {
    Pending,
    Ready { index: usize, color: [f32; 4], key: String, lease: Mesh3dLease },
    Complete { generation: u64, revision: u64, draw_generation: u64, colors: [Option<[f32; 4]>; 3] },
    Stale,
    Fault,
}

pub struct WorldFaceOverlayMeshCursor {
    pub surface_id: String,
    pub generation: u64,
    pub revision: u64,
    pub draw_generation: u64,
    pub phase: WorldFaceOverlayPhase,
    pub scan: WorldFaceOverlayScan,
    pub counts: [u32; 3],
    order: [u8; 3],
    order_len: u8,
    pub published_colors: [Option<[f32; 4]>; 3],
    bucket: u8,
    pub geometry: Option<WorldFaceOverlayGeometry>,
    geometry_item: u8,
    index_item: u32,
    pub owner: WorldPlaceholderOwner,
    pub retry_key: String,
    pub close_started: bool,
    pub faulted: bool,
    pub stale: bool,
}

impl WorldFaceOverlayMeshCursor {
    pub fn new(surface_id: &str, generation: u64, revision: u64, draw_generation: u64) -> Result<Self, ui_scene::math::Mesh3dFault> {
        if surface_id.len().checked_add("component-face-overlay::2".len()).is_none_or(|bytes| bytes > WORLD_DYNAMIC_ID_BYTE_CAPACITY) {
            return Err(ui_scene::math::Mesh3dFault::ByteCapacity);
        }
        Ok(Self {
            surface_id: surface_id.to_owned(),
            generation,
            revision,
            draw_generation,
            phase: WorldFaceOverlayPhase::Count,
            scan: WorldFaceOverlayScan::default(),
            counts: [0; 3],
            order: [0; 3],
            order_len: 0,
            published_colors: [None; 3],
            bucket: 0,
            geometry: None,
            geometry_item: 0,
            index_item: 0,
            owner: WorldPlaceholderOwner::Empty,
            retry_key: String::new(),
            close_started: false,
            faulted: false,
            stale: false,
        })
    }

    pub fn key(&self, index: usize) -> String {
        format!("component-face-overlay:{}:{}:{index}", self.surface_id, self.generation)
    }

    pub fn token(&self) -> Result<Mesh3dWriteToken, ui_scene::math::Mesh3dFault> {
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => Ok(token),
            _ => Err(ui_scene::math::Mesh3dFault::Stale),
        }
    }

    pub fn step(&mut self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState, overlay_state: &WorldOverlayState) -> WorldFaceOverlayMeshStep {
        if session_state.interaction_revision != self.revision || scene_state.draw_generation != self.draw_generation {
            self.faulted = true;
            self.stale = true;
        }
        if self.faulted {
            return if self.close_step() && self.terminal_is_empty() {
                if self.stale {
                    WorldFaceOverlayMeshStep::Stale
                } else {
                    WorldFaceOverlayMeshStep::Fault
                }
            } else {
                WorldFaceOverlayMeshStep::Pending
            };
        }
        match self.step_live(selection_state, gumball_state, scene_state, overlay_state) {
            Ok(step) => step,
            Err(_) => {
                self.faulted = true;
                let _ = self.close_step();
                WorldFaceOverlayMeshStep::Pending
            }
        }
    }

    pub fn step_live(&mut self, selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState, overlay_state: &WorldOverlayState) -> Result<WorldFaceOverlayMeshStep, ui_scene::math::Mesh3dFault> {
        match self.phase {
            WorldFaceOverlayPhase::Count => match self.scan.step(selection_state, gumball_state, scene_state) {
                WorldFaceOverlayScanStep::Pending => {}
                WorldFaceOverlayScanStep::Match(found) => {
                    let category = usize::from(found.category);
                    if self.counts[category] == 0 {
                        self.order[usize::from(self.order_len)] = found.category;
                        self.order_len += 1;
                    }
                    self.counts[category] = self.counts[category].checked_add(1).ok_or(ui_scene::math::Mesh3dFault::ItemCapacity)?;
                }
                WorldFaceOverlayScanStep::Complete => {
                    self.scan.reset();
                    self.phase = WorldFaceOverlayPhase::Prepare;
                }
            },
            WorldFaceOverlayPhase::Prepare => {
                if self.bucket == self.order_len {
                    self.phase = WorldFaceOverlayPhase::Complete;
                } else {
                    let count = self.counts[usize::from(self.order[usize::from(self.bucket)])];
                    let vertices = count.checked_mul(3).ok_or(ui_scene::math::Mesh3dFault::ItemCapacity)?;
                    let indices = count.checked_mul(6).ok_or(ui_scene::math::Mesh3dFault::ItemCapacity)?;
                    let grant = world_mesh_ownership_grant(0, ui_scene::math::mesh3d_begin_capacity_byte_demand(), 0, 1);
                    let (token, progress) = mesh3d_begin(self.generation + u64::from(self.bucket), self.revision, Mesh3dSchema::triangle_mesh(vertices, indices), grant)?;
                    self.owner = WorldPlaceholderOwner::Writing(token);
                    if !progress.fits(grant) { return Err(ui_scene::math::Mesh3dFault::ByteCapacity); }
                    self.phase = WorldFaceOverlayPhase::Allocate;
                }
            }
            WorldFaceOverlayPhase::Allocate => {
                let token = self.token()?;
                let grant = world_mesh_ownership_grant(0, ui_scene::math::mesh3d_allocate_capacity_byte_demand(token)?, 0, 1);
                if world_mesh_ownership_complete(grant, mesh3d_allocate_step(token, grant)?)? {
                    self.scan.reset();
                    self.phase = WorldFaceOverlayPhase::Geometry;
                }
            }
            WorldFaceOverlayPhase::Geometry => {
                if let Some(geometry) = self.geometry {
                    let vertex = usize::from(self.geometry_item / 2);
                    let field = self.geometry_item % 2;
                    if field == 0 {
                        mesh3d_write_vec3(self.token()?, Mesh3dField::Positions, geometry.vertices[vertex].to_array())?;
                    } else {
                        mesh3d_write_vec3(self.token()?, Mesh3dField::Normals, geometry.normal.to_array())?;
                    }
                    self.geometry_item += 1;
                    if self.geometry_item == 6 {
                        self.geometry_item = 0;
                        self.geometry = None;
                    }
                } else {
                    match self.scan.step(selection_state, gumball_state, scene_state) {
                        WorldFaceOverlayScanStep::Pending => {}
                        WorldFaceOverlayScanStep::Match(found) if found.category == self.order[usize::from(self.bucket)] => {
                            self.geometry = Some(face_overlay_geometry(scene_state, found).ok_or(ui_scene::math::Mesh3dFault::Stale)?);
                        }
                        WorldFaceOverlayScanStep::Match(_) => {}
                        WorldFaceOverlayScanStep::Complete => {
                            self.index_item = 0;
                            self.phase = WorldFaceOverlayPhase::Indices;
                        }
                    }
                }
            }
            WorldFaceOverlayPhase::Indices => {
                let category = usize::from(self.order[usize::from(self.bucket)]);
                let total = self.counts[category] * 6;
                let pattern = [0, 1, 2, 0, 2, 1];
                let value = (self.index_item / 6) * 3 + pattern[(self.index_item % 6) as usize];
                mesh3d_write_u32(self.token()?, Mesh3dField::Indices, value)?;
                self.index_item += 1;
                if self.index_item == total {
                    self.phase = WorldFaceOverlayPhase::Seal;
                }
            }
            WorldFaceOverlayPhase::Seal => {
                self.owner = WorldPlaceholderOwner::Ready(mesh3d_seal(self.token()?)?);
                self.phase = WorldFaceOverlayPhase::Publish;
            }
            WorldFaceOverlayPhase::Publish => {
                let WorldPlaceholderOwner::Ready(lease) = self.owner else { return Err(ui_scene::math::Mesh3dFault::Stale) };
                self.owner = WorldPlaceholderOwner::Empty;
                self.phase = WorldFaceOverlayPhase::NextBucket;
                let index = usize::from(self.bucket);
                let key = if self.retry_key.is_empty() { self.key(index) } else { std::mem::take(&mut self.retry_key) };
                return Ok(WorldFaceOverlayMeshStep::Ready { index, color: face_overlay_color(&overlay_state.sub_element_paint, self.order[index]), key, lease });
            }
            WorldFaceOverlayPhase::NextBucket => {
                self.bucket += 1;
                self.scan.reset();
                self.phase = WorldFaceOverlayPhase::Prepare;
            }
            WorldFaceOverlayPhase::Complete => {
                self.surface_id.clear();
                return Ok(WorldFaceOverlayMeshStep::Complete { generation: self.generation, revision: self.revision, draw_generation: self.draw_generation, colors: self.published_colors });
            }
        }
        Ok(WorldFaceOverlayMeshStep::Pending)
    }

    pub fn close_step(&mut self) -> bool {
        self.geometry = None;
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => {
                if !self.close_started {
                    match mesh3d_abort(token) {
                        Ok(()) | Err(ui_scene::math::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_scene::math::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match ui_scene::math::mesh3d_abort_demands(token, ui_scene::math::MESH3D_PAGE_BYTES).and_then(|demand| { let grant = world_mesh_ownership_grant(ui_scene::math::MESH3D_PAGE_BYTES, demand.capacity_bytes, demand.release_bytes, demand.depth); world_mesh_ownership_complete(grant, mesh3d_abort_step(token, grant)?) }) {
                    Ok(true) | Err(ui_scene::math::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Ready(lease) => {
                if !self.close_started {
                    match mesh3d_begin_close(lease) {
                        Ok(()) | Err(ui_scene::math::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_scene::math::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match ui_scene::math::mesh3d_close_demands(lease, ui_scene::math::MESH3D_PAGE_BYTES).and_then(|demand| { let grant = world_mesh_ownership_grant(ui_scene::math::MESH3D_PAGE_BYTES, demand.capacity_bytes, demand.release_bytes, demand.depth); world_mesh_ownership_complete(grant, mesh3d_close_step(lease, grant)?) }) {
                    Ok(true) | Err(ui_scene::math::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Empty => {
                if self.retry_key.pop().is_some() {
                    return false;
                }
                if self.surface_id.pop().is_some() {
                    return false;
                }
                true
            }
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        matches!(self.owner, WorldPlaceholderOwner::Empty) && self.surface_id.is_empty() && self.retry_key.is_empty() && self.geometry.is_none()
    }
}

#[cfg(not(any(test, feature = "testkit")))]
impl Drop for WorldFaceOverlayMeshCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "face overlay mesh cursor reached Drop before its exact authority witness");
    }
}

fn decimal_component_id_matches(value: &str, mut expected: u32) -> bool {
    if expected == 0 {
        return value == "0";
    }
    let digits = (expected.ilog10() + 1) as usize;
    if value.len() != digits || !value.is_ascii() {
        return false;
    }
    for digit in value.as_bytes().iter().rev() {
        if *digit != b'0' + (expected % 10) as u8 {
            return false;
        }
        expected /= 10;
    }
    true
}

fn instance_hovered_component_matches(selection_state: &WorldSelectionState, instance_id: &str, face_id: u32) -> bool {
    pick_targets_instance(selection_state, instance_id)
        && selection_state.hovered_component_mode.as_deref() == Some(selection_state.granularity.as_str())
        && selection_state.hovered_component_object_id.as_deref().is_none_or(|object_id| object_id == instance_id)
        && selection_state.hovered_component_id.as_deref().is_some_and(|id| decimal_component_id_matches(id, face_id))
}

pub fn face_overlay_color(paint: &WorldSubElementPaint, category: u8) -> [f32; 4] {
    let with_alpha = |color: [f32; 4], alpha: f32| [color[0], color[1], color[2], alpha];
    match category {
        0 => with_alpha(paint.face_hover.unwrap_or([1.0, 0.85, 0.35, 1.0]), 0.36),
        1 => with_alpha(paint.face_hover.unwrap_or([0.35, 0.75, 1.0, 1.0]), 0.48),
        _ => with_alpha(paint.face_select.unwrap_or([0.35, 0.75, 1.0, 1.0]), 0.62),
    }
}

fn face_overlay_geometry(scene_state: &WorldSceneState, found: WorldFaceOverlayMatch) -> Option<WorldFaceOverlayGeometry> {
    let draw = scene_state.draws.get(usize::from(found.draw))?;
    let instance = draw.instances.get(usize::from(found.instance))?;
    let mesh = *scene_state.meshes.get(&draw.mesh_key)?;
    let triangle = world_mesh_triangle(mesh, found.triangle)?;
    let vertices = [instance.model.transform_point(mesh_vertex(mesh, triangle[0])?), instance.model.transform_point(mesh_vertex(mesh, triangle[1])?), instance.model.transform_point(mesh_vertex(mesh, triangle[2])?)];
    let normal = vertices[1].sub(vertices[0]).cross(vertices[2].sub(vertices[0])).normalize();
    let offset = if found.hovered { FACE_OVERLAY_OFFSET } else { FACE_OVERLAY_OFFSET * 0.5 };
    Some(WorldFaceOverlayGeometry { vertices: vertices.map(|vertex| vertex.add(normal.scale(offset))), normal })
}

const WORLD3D_PREPARED_STATUS_BYTES: usize = 192;

#[derive(Clone, Copy)]
pub struct World3dPreparedStatus {
    pub bytes: [u8; WORLD3D_PREPARED_STATUS_BYTES],
    pub len: u8,
    pub progress: [f64; 3],
    pub total: u32,
    pub state: u32,
}

impl World3dPreparedStatus {
    pub fn from_parts(text: &str, progress: [f64; 3], total: u32, state: u32) -> Option<Self> {
        if text.len() > WORLD3D_PREPARED_STATUS_BYTES {
            return None;
        }
        let mut bytes = [0; WORLD3D_PREPARED_STATUS_BYTES];
        bytes[..text.len()].copy_from_slice(text.as_bytes());
        Some(Self { bytes, len: text.len() as u8, progress, total, state })
    }

    pub fn text(&self) -> Option<&str> {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).ok()
    }
}

pub struct World3dSnapshotApplyCursor {
    pub lease: World3dSnapshotLease,
    pub page: u16,
    pub item: u8,
    pub staged_orbit: Option<OrbitController>,
    pub draw_started: bool,
    /// 🧱️ Which draw the `Instance` items being applied right now belong to. A single-draw producer
    /// (`Mesh` item flags `30`) leaves it at 0; a multi-draw one (flags `31`) re-points it at every
    /// `Mesh` page, which is why those pages interleave with their own instance pages.
    pub draw_index: u16,
    pub draw_permit: Option<World3dSnapshotDrawPermit>,
    pub faulted: bool,
    pub status: [Option<World3dPreparedStatus>; 2],
}

impl World3dSnapshotApplyCursor {
    pub fn new(lease: World3dSnapshotLease) -> Self {
        Self { lease, page: 0, item: 0, staged_orbit: None, draw_started: false, draw_index: 0, draw_permit: None, faulted: false, status: [None; 2] }
    }

    pub fn close_step(&mut self) -> bool {
        if self.staged_orbit.take().is_some() {
            return false;
        }
        self.page = self.lease.page_count;
        self.item = 0;
        true
    }
}

/// 🗺️ Applies hover/selection action payloads to renderer-local world state before the plugin round-trip.
///
/// 🕹️ An undecodable merge leaves this OPTIMISTIC LOCAL PREVIEW untouched — the word is
/// outside `🕹️interaction`'s schema enum, so the authoritative path faults on it and the
/// preview must not invent a meaning for it (see `MergeMode::from_wire_label`).
///
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: replaces the deleted ad-hoc
/// `worldSelect`/`worldHover` command strings with the framework interaction verbs
/// (`domainId`/`targets`/`merge`/`method`, `domainId`/`channel`/`targets`) — see
/// `resolved_domain_id` (the domain this surface actually targets: its bound app domain, or the
/// OS `world` fallback). This is still the OPTIMISTIC LOCAL PREVIEW only; the framework's
/// `next_selection`/`next_hover` machine (not this file) is the source of truth once the
/// round-trip settles.
///
/// 🎯️ Validates a component target against the original retained instance and mesh label.
pub fn world_component_preview_target<'a>(scene_state: &WorldSceneState, id: &'a str, granularity: &str) -> Option<(&'a str, u32)> {
    let address = WorldComponentAddress::parse(id, granularity)?;
    let (draw, instance) = scene_state.draws.iter().find_map(|draw| draw.instances.iter().find(|instance| instance.id == address.object).map(|instance| (draw, instance)))?;
    address.matches_source(scene_state, instance.component_source, &draw.mesh_key).then_some((address.object, address.group))
}
