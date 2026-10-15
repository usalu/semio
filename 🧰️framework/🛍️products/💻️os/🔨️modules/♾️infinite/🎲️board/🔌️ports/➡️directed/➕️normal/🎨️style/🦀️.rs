//! 🎨️ Board host defaults, camera and LOD, CSS color parsing, style resolution, edge tips, link compatibility and brush helpers.

use super::*;

// #region board_host
impl Default for BoardHost {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            nodes: BTreeMap::new(),
            handles: BTreeMap::new(),
            edges: BTreeMap::new(),
            wires: BTreeMap::new(),
            regions: BTreeMap::new(),
            handle_kinds: BTreeMap::new(),
            wire_kinds: BTreeMap::new(),
            node_kinds: BTreeMap::new(),
            edge_kinds: BTreeMap::new(),
            edge_tips: builtin_edge_tips(),
            link_compat_rules: Vec::new(),
            selection: BTreeSet::new(),
            preselect: BTreeSet::new(),
            preselect_removed: BTreeSet::new(),
            selection_exit_highlight: BTreeSet::new(),
            selection_options: SelectionOptions { method: "rectangle".into(), mode: "replace".into(), select_nodes: true, select_edges: true, select_handles: true },
            hovered_id: None,
            hovered_kind: None,
            highlighted_ids: BTreeSet::new(),
            interaction: Interaction::None,
            width: 1,
            height: 1,
            dpr: 1.0,
            world_raster_tiling: "world-clip".into(),
            events: BoardEventQueue::default(),
            event_overflow: None,
            event_batch_overflow: None,
            event_schema_fault: false,
            selection_screen_preview: None,
            selection_preview_crossing: false,
            link_screen_preview: None,
            canvas_theme: CanvasPalette::default(),
            grid_visible: true,
            grid_factor: GRID_FACTOR_DEFAULT,
            grid_snap_enabled: false,
            preserve_original_element_style: false,
            automatic_lod: true,
            forced_draw_lod: None,
            icon_paint_cache: IconPaintCache::new(),
            link_compat_nodes_emit_key: None,
            link_target_ring_emit_key: None,
            last_select_emit_sig: None,
            last_preselect_emit_sig: None,
            content_scene_generation: 0,
            world_content_cache: RefCell::new(None),
            opaque_scene_retirement: Cell::new(None),
            opaque_scene_fault: Cell::new(false),
            wheel_zoom_active: false,
            wheel_zoom_render_lod: None,
            active_utility: ActiveUtility::Select,
            suggestion_offset: DEFAULT_SUGGESTION_OFFSET,
            brush_node_size: DEFAULT_BRUSH_NODE_SIZE,
            brush_slot_source_id: None,
            brush_candidates: BrushCandidatePage::default(),
            brush_candidate_index: 0,
            brush_preview: None,
            drop_preview: None,
            brush_candidates_emit_key: None,
            brush_preview_emit_key: None,
            brush_placement_serial: 0,
            brush_node_kind_weights: HashMap::new(),
            brush_handle_kind_weights: HashMap::new(),
            brush_alt_pressed: false,
            brush_slot_suggestions_active: false,
            transform_flags: TransformGumballFlags::default(),
            transform_drag: None,
            region_drag: None,
            region_paint: None,
            area_brush_extent: (REGION_DEFAULT_BRUSH_EXTENT_WORLD, REGION_DEFAULT_BRUSH_EXTENT_WORLD),
            port_mode: GraphPortMode::Ported,
            interaction_revision: 0,
            gesture_serial: 0,
            pending_delete_planning: None,
            pending_delete_operation: None,
            pending_pointer_commit: None,
            queued_pointer_commit: None,
            cancelled_pointer_plan_retirement: None,
            pointer_publication: None,
            close_phase: BoardHostClosePhase::Events,
            close_entity_retirement: None,
            close_strings: std::array::from_fn(|_| None),
            close_string_len: 0,
            close_node_handles: None,
        }
    }
}

impl BoardHost {
    /// 📶️ Draw LOD used while building the vector scene (pins during wheel zoom).
    pub(super) fn draw_lod_for_frame(&self) -> BoardDrawLod {
        if self.wheel_zoom_active {
            if let Some(pinned) = self.wheel_zoom_render_lod {
                return pinned;
            }
        }
        self.current_draw_lod()
    }

    fn board_draw_lod_label(lod: BoardDrawLod) -> &'static str {
        lod.label()
    }

    /// 🏷️ Camera, draw LOD, and visible node centers from the WASM host for the JS text overlay (must match the last GPU frame).
    pub fn overlay_paint_state_json(&self) -> String {
        let nodes: Vec<serde_json::Value> = self.nodes.values().filter(|n| n.visible).map(|n| serde_json::json!({ "id": n.id, "x": n.x, "y": n.y })).collect();
        serde_json::json!({
            "camera": { "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom },
            "lod": Self::board_draw_lod_label(self.draw_lod_for_frame()),
            "nodes": nodes,
        })
        .to_string()
    }

    pub(super) fn bump_content_scene_generation(&mut self) {
        self.content_scene_generation = self.content_scene_generation.wrapping_add(1);
    }

    pub(super) fn advance_opaque_scene_retirement_step(&self) -> bool {
        let Some(token) = self.opaque_scene_retirement.get() else {
            return true;
        };
        match semio_framework_canvas::advance_opaque_scene_retirement(token, 1, 4096) {
            OpaqueSceneRetirementStep::Blocked | OpaqueSceneRetirementStep::Pending { .. } => false,
            OpaqueSceneRetirementStep::Complete { .. } => {
                self.opaque_scene_retirement.set(None);
                true
            }
            OpaqueSceneRetirementStep::Fault => {
                self.opaque_scene_fault.set(true);
                false
            }
        }
    }

    pub(super) fn publish_opaque_scene_retirement(&self, token: OpaqueSceneRetirementToken, scene: Scene) {
        assert!(self.opaque_scene_retirement.get().is_none(), "board retains at most one exact opaque scene retirement");
        semio_framework_canvas::publish_opaque_scene_retirement(token, scene);
        self.opaque_scene_retirement.set(Some(token));
    }

    pub fn quarantine_world_content_step(&mut self) -> bool {
        if self.opaque_scene_retirement.get().is_some() {
            self.advance_opaque_scene_retirement_step();
            return false;
        }
        let mut cache = self.world_content_cache.borrow_mut();
        if cache.is_none() {
            return true;
        }
        let Some(token) = semio_framework_canvas::reserve_opaque_scene_retirement() else {
            self.opaque_scene_fault.set(true);
            return false;
        };
        let WorldContentCache { scene, .. } = cache.take().expect("world content cache was witnessed occupied");
        self.publish_opaque_scene_retirement(token, scene);
        false
    }

    pub fn opaque_scene_faulted(&self) -> bool {
        self.opaque_scene_fault.get()
    }

    fn push_close_string(&mut self, value: String) {
        let index = usize::from(self.close_string_len);
        assert!(index < self.close_strings.len(), "board close string scratch is schema bounded");
        self.close_strings[index] = Some(value);
        self.close_string_len += 1;
    }

    fn push_close_optional_string(&mut self, value: Option<String>) {
        if let Some(value) = value {
            self.push_close_string(value);
        }
    }

    fn close_scratch_step(&mut self) -> bool {
        if self.close_node_handles.is_some() {
            let handle = self.close_node_handles.as_mut().and_then(Vec::pop);
            if let Some(handle) = handle {
                self.push_close_string(handle.handle_kind);
                return true;
            }
            self.close_node_handles = None;
            return true;
        }
        if self.close_string_len > 0 {
            self.close_string_len -= 1;
            drop(self.close_strings[usize::from(self.close_string_len)].take());
            return true;
        }
        false
    }

    fn close_interaction_step(&mut self) -> bool {
        match &mut self.interaction {
            Interaction::None | Interaction::Pan { .. } => {
                self.interaction = Interaction::None;
                true
            }
            Interaction::DragNodes { primary_id, start_positions, proximity_pair, gesture, .. } => {
                if let Some((id, _)) = start_positions.pop_first() {
                    drop(id);
                    return false;
                }
                if let Some(id) = gesture.select.as_mut().and_then(|(ids, _)| ids.pop()).or_else(|| gesture.restore.as_mut().and_then(BTreeSet::pop_first)) {
                    drop(id);
                    return false;
                }
                drop(std::mem::take(gesture));
                if let Some((left, right)) = proximity_pair.take() {
                    if right.is_empty() {
                        drop(left);
                    } else {
                        drop(right);
                        *proximity_pair = Some((left, String::new()));
                    }
                    return false;
                }
                drop(std::mem::take(primary_id));
                self.interaction = Interaction::None;
                true
            }
            Interaction::SelectionPending { initial_ids, .. } => {
                if let Some(id) = initial_ids.pop_first() {
                    drop(id);
                    return false;
                }
                self.interaction = Interaction::None;
                true
            }
            Interaction::Selection { initial_ids, points, screen_points, .. } => {
                if initial_ids.pop_first().is_some() || points.pop().is_some() || screen_points.pop().is_some() {
                    return false;
                }
                self.interaction = Interaction::None;
                true
            }
            Interaction::LinkAtSourceHandle { source_id, .. } => {
                drop(std::mem::take(source_id));
                self.interaction = Interaction::None;
                true
            }
            Interaction::LinkDragSnap { source_id, target_id, .. } => {
                if let Some(target) = target_id.take() {
                    drop(target);
                    return false;
                }
                drop(std::mem::take(source_id));
                self.interaction = Interaction::None;
                true
            }
            Interaction::LinkTargetNode { source_id, target_node_id } => {
                if !target_node_id.is_empty() {
                    drop(std::mem::take(target_node_id));
                    return false;
                }
                drop(std::mem::take(source_id));
                self.interaction = Interaction::None;
                true
            }
            Interaction::ExternalLinkPreview { source_id, compatible_node_ids, ring_node_id, ring_handle_ids, .. } => {
                if let Some(id) = compatible_node_ids.pop().or_else(|| ring_handle_ids.pop()).or_else(|| ring_node_id.take()) {
                    drop(id);
                    return false;
                }
                drop(std::mem::take(source_id));
                self.interaction = Interaction::None;
                true
            }
        }
    }

    pub fn close_nonopaque_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if context.should_yield() {
            return false;
        }
        if self.close_scratch_step() {
            context.consume_fuel(1);
            return false;
        }
        if let Some(retirement) = self.close_entity_retirement.as_mut() {
            match retirement.step() {
                Ok(true) => self.close_entity_retirement = None,
                Ok(false) => {}
                Err(()) => {
                    self.event_schema_fault = true;
                    return false;
                }
            }
            context.consume_fuel(1);
            return false;
        }
        match self.close_phase {
            BoardHostClosePhase::Events => {
                if self.close_event_authority_step(context) {
                    self.close_phase = BoardHostClosePhase::Pointer;
                }
                return false;
            }
            BoardHostClosePhase::Pointer => {
                if self.close_pointer_authority_step(context) {
                    self.close_phase = BoardHostClosePhase::WorldScene;
                }
                return false;
            }
            BoardHostClosePhase::WorldScene => {
                if self.quarantine_world_content_step() {
                    self.close_phase = BoardHostClosePhase::Icons;
                }
            }
            BoardHostClosePhase::Icons => {
                if self.icon_paint_cache.close_step() {
                    self.close_phase = BoardHostClosePhase::Nodes;
                }
            }
            BoardHostClosePhase::Nodes => {
                if let Some((key, value)) = self.nodes.pop_first() {
                    drop(key);
                    self.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Node(value)));
                } else {
                    self.close_phase = BoardHostClosePhase::Handles;
                }
            }
            BoardHostClosePhase::Handles => {
                if let Some((key, value)) = self.handles.pop_first() {
                    drop(key);
                    self.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Handle(value)));
                } else {
                    self.close_phase = BoardHostClosePhase::Edges;
                }
            }
            BoardHostClosePhase::Edges => {
                if let Some((key, value)) = self.edges.pop_first() {
                    drop(key);
                    self.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Edge(value)));
                } else {
                    self.close_phase = BoardHostClosePhase::Wires;
                }
            }
            BoardHostClosePhase::Wires => {
                if let Some((key, value)) = self.wires.pop_first() {
                    drop(key);
                    self.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Wire(value)));
                } else {
                    self.close_phase = BoardHostClosePhase::Regions;
                }
            }
            BoardHostClosePhase::Regions => {
                if let Some((key, value)) = self.regions.pop_first() {
                    self.push_close_string(key);
                    self.push_close_string(value.id);
                    self.push_close_optional_string(value.label);
                } else if let Some(id) = self.region_drag.as_mut().and_then(|drag| drag.gesture.restore.as_mut()).and_then(BTreeSet::pop_first) {
                    self.push_close_string(id);
                } else if let Some(drag) = self.region_drag.take() {
                    self.push_close_string(drag.id);
                    self.push_close_string(drag.gesture.id);
                } else {
                    self.region_paint = None;
                    self.close_phase = BoardHostClosePhase::Selection;
                }
            }
            BoardHostClosePhase::Selection => {
                if self.selection.pop_first().is_none() {
                    self.close_phase = BoardHostClosePhase::Preselect;
                }
            }
            BoardHostClosePhase::Preselect => {
                if self.preselect.pop_first().is_none() {
                    self.close_phase = BoardHostClosePhase::PreselectRemoved;
                }
            }
            BoardHostClosePhase::PreselectRemoved => {
                if self.preselect_removed.pop_first().is_none() {
                    self.close_phase = BoardHostClosePhase::SelectionExit;
                }
            }
            BoardHostClosePhase::SelectionExit => {
                if self.selection_exit_highlight.pop_first().is_none() {
                    self.close_phase = BoardHostClosePhase::Highlighted;
                }
            }
            BoardHostClosePhase::Highlighted => {
                if self.highlighted_ids.pop_first().is_none() {
                    self.close_phase = BoardHostClosePhase::Interaction;
                }
            }
            BoardHostClosePhase::Interaction => {
                if self.close_interaction_step() {
                    self.close_phase = BoardHostClosePhase::HandleKinds;
                }
            }
            BoardHostClosePhase::HandleKinds => {
                if let Some((key, value)) = self.handle_kinds.pop_first() {
                    self.push_close_string(key);
                    self.push_close_string(value.name);
                    self.push_close_optional_string(value.default_wire_kind);
                } else {
                    self.close_phase = BoardHostClosePhase::WireKinds;
                }
            }
            BoardHostClosePhase::WireKinds => {
                if let Some((key, value)) = self.wire_kinds.pop_first() {
                    self.push_close_string(key);
                    self.push_close_string(value.name);
                    self.push_close_optional_string(value.default_edge_kind);
                } else {
                    self.close_phase = BoardHostClosePhase::NodeKinds;
                }
            }
            BoardHostClosePhase::NodeKinds => {
                if let Some((key, value)) = self.node_kinds.pop_first() {
                    self.push_close_string(key);
                    self.push_close_string(value.name);
                    self.push_close_optional_string(value.icon);
                    self.close_node_handles = Some(value.handles);
                } else {
                    self.close_phase = BoardHostClosePhase::EdgeKinds;
                }
            }
            BoardHostClosePhase::EdgeKinds => {
                if let Some((key, value)) = self.edge_kinds.pop_first() {
                    self.push_close_string(key);
                    self.push_close_string(value.name);
                    self.push_close_optional_string(value.source_tip);
                    self.push_close_optional_string(value.target_tip);
                } else {
                    self.close_phase = BoardHostClosePhase::EdgeTips;
                }
            }
            BoardHostClosePhase::EdgeTips => {
                if self.edge_tips.pop_first().is_none() {
                    self.close_phase = BoardHostClosePhase::LinkRules;
                }
            }
            BoardHostClosePhase::LinkRules => {
                if let Some(rule) = self.link_compat_rules.pop() {
                    self.push_close_string(rule.source);
                    self.push_close_string(rule.target);
                } else {
                    self.close_phase = BoardHostClosePhase::Previews;
                }
            }
            BoardHostClosePhase::Previews => {
                if self.selection_screen_preview.as_mut().is_some_and(|values| values.pop().is_some()) || self.link_screen_preview.as_mut().is_some_and(|values| values.pop().is_some()) {
                } else if let Some(preview) = self.brush_preview.take() {
                    self.push_close_string(preview.source_handle_id);
                    self.push_close_string(preview.node_kind_id);
                    self.push_close_optional_string(preview.icon_kind);
                    self.close_node_handles = Some(preview.handles);
                } else if let Some(preview) = self.drop_preview.take() {
                    self.push_close_string(preview.node_kind_id);
                    self.push_close_optional_string(preview.icon_kind);
                } else if self
                    .transform_drag
                    .as_mut()
                    .is_some_and(|drag| drag.start_positions.pop_first().is_some() || drag.start_handle_angles.pop_first().is_some())
                {
                } else if self.transform_drag.take().is_some() {
                } else {
                    self.selection_screen_preview = None;
                    self.link_screen_preview = None;
                    self.close_phase = BoardHostClosePhase::Strings;
                }
            }
            BoardHostClosePhase::Strings => {
                if let Some(value) = self
                    .hovered_id
                    .take()
                    .or_else(|| self.link_compat_nodes_emit_key.take())
                    .or_else(|| self.link_target_ring_emit_key.take())
                    .or_else(|| self.brush_slot_source_id.take())
                    .or_else(|| self.brush_candidates_emit_key.take())
                    .or_else(|| self.brush_preview_emit_key.take())
                {
                    drop(value);
                } else if let Some((left, right)) = self.hovered_kind.take() {
                    self.push_close_string(left);
                    self.push_close_string(right);
                } else if let Some((values, mode)) = self.last_select_emit_sig.as_mut() {
                    if let Some(value) = values.pop().or_else(|| mode.take()) {
                        drop(value);
                    } else {
                        self.last_select_emit_sig = None;
                    }
                } else if let Some((left, right, mode)) = self.last_preselect_emit_sig.as_mut() {
                    if let Some(value) = left.pop().or_else(|| right.pop()).or_else(|| mode.take()) {
                        drop(value);
                    } else {
                        self.last_preselect_emit_sig = None;
                    }
                } else {
                    drop(std::mem::take(&mut self.world_raster_tiling));
                    let method = std::mem::take(&mut self.selection_options.method);
                    let mode = std::mem::take(&mut self.selection_options.mode);
                    self.push_close_string(method);
                    self.push_close_string(mode);
                    self.close_phase = BoardHostClosePhase::Weights;
                }
            }
            BoardHostClosePhase::Weights => {
                let released = self.brush_node_kind_weights.keys().next().cloned().and_then(|key| self.brush_node_kind_weights.remove_entry(&key)).is_some()
                    || self.brush_handle_kind_weights.keys().next().cloned().and_then(|key| self.brush_handle_kind_weights.remove_entry(&key)).is_some();
                if !released {
                    self.close_phase = BoardHostClosePhase::Done;
                }
            }
            BoardHostClosePhase::Done => return self.nonopaque_terminal_is_empty(),
        }
        context.consume_fuel(1);
        false
    }

    pub fn nonopaque_terminal_is_empty(&self) -> bool {
        self.close_phase == BoardHostClosePhase::Done
            && self.event_authority_terminal_is_empty()
            && self.pointer_authority_terminal_is_empty()
            && self.close_entity_retirement.is_none()
            && self.close_string_len == 0
            && self.close_node_handles.is_none()
            && self.nodes.is_empty()
            && self.handles.is_empty()
            && self.edges.is_empty()
            && self.wires.is_empty()
            && self.regions.is_empty()
            && self.region_drag.is_none()
            && self.region_paint.is_none()
            && self.handle_kinds.is_empty()
            && self.wire_kinds.is_empty()
            && self.node_kinds.is_empty()
            && self.edge_kinds.is_empty()
            && self.edge_tips.is_empty()
            && self.link_compat_rules.is_empty()
            && self.selection.is_empty()
            && self.preselect.is_empty()
            && self.preselect_removed.is_empty()
            && self.selection_exit_highlight.is_empty()
            && self.highlighted_ids.is_empty()
            && matches!(self.interaction, Interaction::None)
            && self.transform_drag.is_none()
            && self.icon_paint_cache.terminal_is_empty()
            && self.world_content_cache.borrow().is_none()
            && self.opaque_scene_retirement.get().is_none()
    }

    #[doc(hidden)]
    pub fn test_content_scene_generation(&self) -> u64 {
        self.content_scene_generation
    }

    pub(super) fn viewport(&self) -> semio_framework_canvas::camera::Viewport {
        semio_framework_canvas::camera::Viewport { width: self.width, height: self.height, dpr: self.dpr }
    }

    pub(super) fn camera_content_affine(&self) -> Affine {
        semio_framework_canvas::camera::camera_content_affine(&self.camera, &self.viewport())
    }
}

impl BoardHost {
    pub(super) fn grid_step_large_world(&self) -> f64 {
        GRID_WORLD_LARGE * self.grid_factor
    }
    pub(super) fn grid_step_medium_world(&self) -> f64 {
        GRID_WORLD_MEDIUM * self.grid_factor
    }
    pub(super) fn grid_step_small_world(&self) -> f64 {
        GRID_WORLD_SMALL * self.grid_factor
    }
    pub(super) fn grid_step_micro_world(&self) -> f64 {
        GRID_WORLD_MICRO * self.grid_factor
    }

    pub fn new() -> Self {
        Self::default()
    }

    /// 🧠️ Normal directed graph host: no handles, edges reference node ids.
    pub fn new_normal() -> Self {
        let mut host = Self { port_mode: GraphPortMode::Normal, ..Self::default() };
        host.selection_options.select_handles = false;
        host
    }

    pub(super) fn has_ports(&self) -> bool {
        self.port_mode.has_ports()
    }

    pub(super) fn node_rim_point_toward(&self, node: &NodeData, toward: Point) -> Point {
        let center = Point::new(node.x, node.y);
        match node.shape {
            NodeShape::Circle => {
                let radius = self.scaled_node_radius(node);
                let angle = circle_handle_angle_toward(center, toward);
                handle_position_on_circle(center, radius, angle)
            }
            NodeShape::Rectangle => {
                let width = self.scaled_node_width(node);
                let height = self.scaled_node_height(node);
                let angle = rectangle_handle_angle_toward(center, width, height, toward);
                handle_position_on_rectangle(center, width, height, angle)
            }
        }
    }

    pub(super) fn current_draw_lod(&self) -> BoardDrawLod {
        if !self.automatic_lod {
            if let Some(lod) = self.forced_draw_lod {
                return lod;
            }
        }
        BoardDrawLod::from_scale_index(PUZZLE_2D_LOD_SCALE.resolve_index(self.camera.zoom))
    }

    pub(super) fn lod_visible_grid_snap_step_world(&self) -> Option<f64> {
        match self.current_draw_lod() {
            BoardDrawLod::Minimap => None,
            BoardDrawLod::Overview | BoardDrawLod::Compact => Some(self.grid_step_large_world()),
            BoardDrawLod::Normal => Some(self.grid_step_medium_world()),
            BoardDrawLod::Detail => Some(self.grid_step_small_world()),
            BoardDrawLod::Micro => Some(self.grid_step_micro_world()),
        }
    }

    fn snap_world_scalar(&self, v: f64) -> f64 {
        if !self.grid_snap_enabled {
            return v;
        }
        let Some(step) = self.lod_visible_grid_snap_step_world() else {
            return v;
        };
        (v / step).round() * step
    }

    pub(super) fn snap_world_pair(&self, x: f64, y: f64) -> (f64, f64) {
        (self.snap_world_scalar(x), self.snap_world_scalar(y))
    }

    /// 👁️ Shows or hides the world grid. Hiding it never changes snapping, which reads `grid_snap_enabled`.
    pub fn set_grid_visible(&mut self, visible: bool) {
        if self.grid_visible != visible {
            self.grid_visible = visible;
            self.content_scene_generation = self.content_scene_generation.wrapping_add(1);
        }
    }

    pub fn set_grid_snap_enabled(&mut self, enabled: bool) {
        self.grid_snap_enabled = enabled;
    }

    pub fn set_automatic_lod(&mut self, enabled: bool) {
        self.automatic_lod = enabled;
        if enabled {
            self.forced_draw_lod = None;
        }
    }

    pub fn set_forced_draw_lod_label(&mut self, label: &str) {
        let t = label.trim();
        if t.is_empty() {
            self.forced_draw_lod = None;
            return;
        }
        self.forced_draw_lod = match BoardDrawLod::from_id(t) {
            Some(lod) => Some(lod),
            None => {
                self.forced_draw_lod = None;
                return;
            }
        };
    }

    pub fn set_grid_factor(&mut self, v: f64) -> Result<(), NormalPortError> {
        if !v.is_finite() || v <= 0.0 || v > 1_000_000.0 {
            return Err(NormalPortError::GridFactorOutOfRange);
        }
        self.grid_factor = v;
        Ok(())
    }

    /// 🔗️ Applies or clears a host-driven link preview session (cross-surface mirror).
    pub fn set_external_link_preview_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        let v: serde_json::Value = serde_json::from_str(json).map_err(NormalPortError::ExternalLinkPreviewJson)?;
        let source = v.get("source").and_then(|s| s.as_str()).unwrap_or("").trim().to_string();
        if source.is_empty() {
            if matches!(self.interaction, Interaction::ExternalLinkPreview { .. }) {
                self.interaction = Interaction::None;
                self.clear_link_gesture_events();
            }
            return Ok(());
        }
        let end_x = v.get("endX").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let end_y = v.get("endY").and_then(|y| y.as_f64()).unwrap_or(0.0);
        let compatible_node_ids: Vec<String> = v.get("compatiblePartIds").and_then(|a| a.as_array()).map(|arr| arr.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
        let ring_node_id = v.get("ringPartId").and_then(|n| n.as_str()).map(str::to_string);
        let ring_handle_ids: Vec<String> = v.get("ringAnchorIds").and_then(|a| a.as_array()).map(|arr| arr.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
        self.interaction = Interaction::ExternalLinkPreview { source_id: source, end_world: Point::new(end_x, end_y), compatible_node_ids, ring_node_id, ring_handle_ids };
        self.sync_link_gesture_events();
        Ok(())
    }

    /// 🔗️ Clears host-driven link preview without touching local link drags.
    pub fn clear_external_link_preview(&mut self) {
        if matches!(self.interaction, Interaction::ExternalLinkPreview { .. }) {
            self.interaction = Interaction::None;
            self.clear_link_gesture_events();
        }
    }

    pub(super) fn get_or_build_icon_paint(&self, encoded: &str, fg: Color, bg: Color, preserve_original_style: bool) -> Option<CachedIconPaintLease<'_>> {
        self.icon_paint_cache.get_or_build(encoded, fg, bg, preserve_original_style)
    }

    pub fn clear_icon_vector_cache(&mut self) {
        self.icon_paint_cache.clear();
    }

    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        self.width = width.max(1);
        self.height = height.max(1);
        self.dpr = dpr.max(1.0);
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
    }

    /// 📷️ Reads the accepted viewport pose used by touch and pointer gesture planning.
    pub fn camera(&self) -> [f64; 3] {
        [self.camera.x, self.camera.y, self.camera.zoom]
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.set_camera_internal(x, y, zoom, true);
    }

    /// 🔇️ Updates viewport camera without enqueueing a `camera` drain row (wheel / imperative sync).
    pub fn set_camera_silent(&mut self, x: f64, y: f64, zoom: f64) {
        self.set_camera_internal(x, y, zoom, false);
    }

    fn set_camera_internal(&mut self, x: f64, y: f64, zoom: f64, emit_event: bool) {
        let zoom = semio_framework_canvas::camera::clamp_zoom(zoom);
        if (self.camera.x - x).abs() < 1e-9 && (self.camera.y - y).abs() < 1e-9 && (self.camera.zoom - zoom).abs() < 1e-9 {
            return;
        }
        let reservation = if emit_event {
            let Some(reservation) = self.reserve_owned_event(BoardOwnedEvent::camera(x, y, zoom)) else {
                return;
            };
            Some(reservation)
        } else {
            None
        };
        self.camera.x = x;
        self.camera.y = y;
        self.camera.zoom = zoom;
        self.interaction_revision = self.interaction_revision.wrapping_add(1);
        if let Some(reservation) = reservation {
            self.publish_event_reservation(reservation);
        }
    }

    pub fn set_selection_options(&mut self, method: &str, mode: &str, select_nodes: bool, select_edges: bool, select_handles: bool) {
        self.selection_options.method = method.into();
        self.selection_options.mode = normalize_selection_mode(mode);
        self.selection_options.select_nodes = select_nodes;
        self.selection_options.select_edges = select_edges;
        self.selection_options.select_handles = select_handles;
    }

    /// 🔗️ JSON `[{ "source","target","bidirectional"?,"important"?,"specificity"? },…]` gates link gestures; empty clears restrictions.
    pub fn set_handle_link_compat_from_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        let v: serde_json::Value = serde_json::from_str(json)?;
        let arr = v.as_array().ok_or(NormalPortError::CompatNotArray)?;
        let mut next = Vec::new();
        for row in arr {
            let o = row.as_object().ok_or(NormalPortError::RowNotObject("compat"))?;
            let source = o.get("source").and_then(|x| x.as_str()).ok_or(NormalPortError::CompatSourceMissing)?.trim().to_string();
            let target = o.get("target").and_then(|x| x.as_str()).ok_or(NormalPortError::CompatTargetMissing)?.trim().to_string();
            let bidirectional = o.get("bidirectional").and_then(|x| x.as_bool()).unwrap_or(false);
            let important = o.get("important").and_then(|x| x.as_bool()).unwrap_or(false);
            let spec_s = o.get("specificity").and_then(|x| x.as_str()).unwrap_or("handle");
            let specificity = Self::parse_compat_specificity(spec_s)?;
            next.push(LinkCompatRule { source, target, bidirectional, important, specificity });
        }
        self.link_compat_rules = next;
        Ok(())
    }

    fn parse_compat_specificity(raw: &str) -> Result<CompatSpecificity, NormalPortError> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "general" => Ok(CompatSpecificity::General),
            "node" => Ok(CompatSpecificity::Node),
            "edge" => Ok(CompatSpecificity::Edge),
            "handle" | "vortex" => Ok(CompatSpecificity::Handle),
            "wire" => Ok(CompatSpecificity::Wire),
            _ => Err(NormalPortError::InvalidCompatSpecificity(raw.to_string())),
        }
    }

    fn reject_kind_catalog_row_legacy_label(row: &serde_json::Map<String, serde_json::Value>, slice: &'static str) -> Result<(), NormalPortError> {
        if row.contains_key("label") {
            return Err(NormalPortError::LegacyLabelField(slice));
        }
        Ok(())
    }

    /// 🧩️ JSON object `{ handleKinds?, wireKinds?, nodeKinds?, edgeKinds? }` replacing prior catalogs (omit arrays to clear that slice).
    pub fn set_board_kind_catalogs_from_json(&mut self, json: &str) -> Result<(), NormalPortError> {
        if json.len() > BOARD_EVENT_BYTE_CAPACITY {
            return Err(NormalPortError::EventCredits);
        }
        let v: serde_json::Value = serde_json::from_str(json)?;
        let o = v.as_object().ok_or(NormalPortError::KindCatalogsRootNotObject)?;
        for key in ["handleKinds", "wireKinds", "nodeKinds", "edgeTips", "edgeKinds"] {
            if o.get(key).and_then(serde_json::Value::as_array).is_some_and(|rows| rows.len() > BOARD_POINTER_ITEM_CAPACITY) {
                return Err(NormalPortError::EventCredits);
            }
        }
        let template_count = o
            .get("nodeKinds")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(serde_json::Value::as_object)
            .filter_map(|row| row.get("handles"))
            .filter_map(serde_json::Value::as_array)
            .try_fold(0usize, |count, handles| count.checked_add(handles.len()))
            .ok_or(NormalPortError::EventCredits)?;
        if template_count > BOARD_POINTER_ITEM_CAPACITY {
            return Err(NormalPortError::EventCredits);
        }
        if let Some(arr) = o.get("handleKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let ho = row.as_object().ok_or(NormalPortError::RowNotObject("handle kind"))?;
                Self::reject_kind_catalog_row_legacy_label(ho, "handle")?;
                let id = ho.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("handle kind"))?;
                let name = ho.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let color_s = ho.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::HandleKindColorMissing)?;
                let color = Self::parse_css_color(color_s).ok_or_else(|| NormalPortError::InvalidHandleKindColor(color_s.to_string()))?;
                let default_wire_kind = ho.get("defaultWireKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let scale = ho.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
                next.insert(id.to_string(), HandleKindDef { name, color, default_wire_kind, scale });
            }
            self.handle_kinds = next;
        }
        if let Some(arr) = o.get("wireKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let wo = row.as_object().ok_or(NormalPortError::RowNotObject("wire kind"))?;
                Self::reject_kind_catalog_row_legacy_label(wo, "wire")?;
                let id = wo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("wire kind"))?;
                let name = wo.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let default_edge_kind = wo.get("defaultEdgeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                next.insert(id.to_string(), WireKindDef { name, default_edge_kind });
            }
            self.wire_kinds = next;
        }
        if let Some(arr) = o.get("nodeKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let no = row.as_object().ok_or(NormalPortError::RowNotObject("node kind"))?;
                Self::reject_kind_catalog_row_legacy_label(no, "node")?;
                let id = no.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("node kind"))?;
                let name = no.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let scale = no.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
                let shape = match no.get("shape").and_then(|x| x.as_str()).map(str::trim) {
                    Some("rectangle") => NodeShape::Rectangle,
                    _ => NodeShape::Circle,
                };
                let mut handles: Vec<NodeKindHandleTemplate> = Vec::new();
                if let Some(arr) = no.get("handles").and_then(|x| x.as_array()) {
                    for row in arr {
                        let ho = row.as_object().ok_or(NormalPortError::RowNotObject("node kind handle"))?;
                        let handle_kind = ho.get("handleKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::NodeKindHandleKindMissing)?;
                        let angle = ho.get("angle").and_then(|x| x.as_f64()).filter(|x| x.is_finite()).ok_or(NormalPortError::NodeKindHandleAngleMissing)?;
                        let radius = ho.get("radius").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0);
                        handles.push(NodeKindHandleTemplate { handle_kind: handle_kind.to_string(), angle, radius });
                    }
                }
                let icon = no.get("icon").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let color_fill = no.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).and_then(Self::parse_css_hex_color);
                next.insert(id.to_string(), NodeKindDef { name, scale, shape, handles, icon, color_fill });
            }
            self.node_kinds = next;
        }
        if let Some(arr) = o.get("edgeTips").and_then(|x| x.as_array()) {
            let mut tips = builtin_edge_tips();
            for row in arr {
                let eo = row.as_object().ok_or(NormalPortError::RowNotObject("edge tip"))?;
                let id = eo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("edge tip"))?;
                let geometry=match eo.get("geometry") {
                    None=>None,
                    Some(value)=>Some(match value.as_str().map(str::trim){Some("arrow")=>EdgeTipGeometry::Arrow,Some("fine-arrow")=>EdgeTipGeometry::FineArrow,Some("diamond")=>EdgeTipGeometry::Diamond,Some("circle")=>EdgeTipGeometry::Circle,Some("bar")=>EdgeTipGeometry::Bar,_=>return Err(NormalPortError::EdgeTipRowInvalid(id.to_string()))})
                };
                let entry=infinite::board::ports::directed::EdgeTipCatalogEntry{id:id.to_string(),geometry,filled:eo.get("filled").and_then(|v|v.as_bool()),scale:eo.get("scale").and_then(|v|v.as_f64())};
                let def = EdgeTipDef::from_catalog_entry(&entry).ok_or_else(|| NormalPortError::EdgeTipRowInvalid(id.to_string()))?;
                tips.insert(id.to_string(), def);
            }
            self.edge_tips = tips;
        }
        if let Some(arr) = o.get("edgeKinds").and_then(|x| x.as_array()) {
            let mut next = BTreeMap::new();
            for row in arr {
                let eo = row.as_object().ok_or(NormalPortError::RowNotObject("edge kind"))?;
                Self::reject_kind_catalog_row_legacy_label(eo, "edge")?;
                let id = eo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("edge kind"))?;
                let name = eo.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                let color = eo.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).and_then(Self::parse_css_hex_color);
                let stroke_width = eo
                    .get("stroke")
                    .and_then(|x| x.as_f64())
                    .filter(|v| v.is_finite() && *v > 0.0)
                    .or_else(|| eo.get("stroke").and_then(|x| x.as_str()).and_then(|s| s.trim().parse::<f64>().ok()).filter(|v| v.is_finite() && *v > 0.0))
                    .unwrap_or(2.0);
                let pattern = match eo.get("pattern").and_then(|x| x.as_str()).map(str::trim) {
                    Some("dashed") => EdgeStrokePattern::Dashed,
                    Some("dotted") => EdgeStrokePattern::Dotted,
                    _ => EdgeStrokePattern::Solid,
                };
                let source_tip = Self::parse_catalog_tip_slot(eo.get("sourceTip").or_else(|| eo.get("source_tip")).and_then(|x| x.as_str()));
                let target_tip = Self::parse_catalog_tip_slot(eo.get("targetTip").or_else(|| eo.get("target_tip")).and_then(|x| x.as_str()).or_else(|| eo.get("marker").and_then(|x| x.as_str())));
                let directed = eo.get("directed").and_then(|x| x.as_bool()).unwrap_or(true);
                next.insert(id.to_string(), EdgeKindDef { name, color, stroke_width, pattern, source_tip, target_tip, directed });
            }
            self.edge_kinds = next;
        }
        Ok(())
    }

    /// 🛡️ Ensures runtime catalogs declare every kind from a compile-time manifest.
    pub fn validate_against_manifest(&self, gm: &GraphManifest) -> Result<(), NormalPortError> {
        for row in &gm.port_kinds {
            let visual = row.presentation.as_ref().is_some_and(|p| p.get("color").is_some());
            if visual && !self.handle_kinds.contains_key(&row.id) {
                return Err(NormalPortError::CatalogMissingKind("handle", row.id.clone()));
            }
        }
        for row in &gm.wire_kinds {
            if !self.wire_kinds.contains_key(&row.id) {
                return Err(NormalPortError::CatalogMissingKind("wire", row.id.clone()));
            }
        }
        for row in &gm.edge_kinds {
            if row.presentation.is_some() && !self.edge_kinds.contains_key(&row.id) {
                return Err(NormalPortError::CatalogMissingKind("edge", row.id.clone()));
            }
        }
        for row in &gm.node_kinds {
            if row.presentation.is_some() && !self.node_kinds.contains_key(&row.id) {
                return Err(NormalPortError::CatalogMissingKind("node", row.id.clone()));
            }
        }
        Ok(())
    }

    pub(super) fn parse_catalog_tip_slot(value: Option<&str>) -> Option<String> {
        let s = value?.trim();
        if s.is_empty() || s.eq_ignore_ascii_case("none") {
            Some(String::new())
        } else {
            Some(s.to_string())
        }
    }

    fn lookup_edge_tip<'a>(&'a self, id: &str) -> Option<&'a EdgeTipDef> {
        if id.is_empty() {
            return None;
        }
        self.edge_tips.get(id)
    }

    fn resolve_tip_slot<'a>(&'a self, slot: Option<&str>) -> Option<&'a EdgeTipDef> {
        match slot {
            Some("") => None,
            Some(id) => self.lookup_edge_tip(id),
            None => None,
        }
    }

    fn parse_css_hex_color(s: &str) -> Option<Color> {
        let s = s.trim();
        let hex = s.strip_prefix('#')?;
        match hex.len() {
            3 => {
                let mut full = String::new();
                for ch in hex.chars() {
                    full.push(ch);
                    full.push(ch);
                }
                let v = u32::from_str_radix(&full, 16).ok()?;
                let r = ((v >> 16) & 0xff) as u8;
                let g = ((v >> 8) & 0xff) as u8;
                let b = (v & 0xff) as u8;
                Some(Color::from_rgba8(r, g, b, 255))
            }
            6 => {
                let v = u32::from_str_radix(hex, 16).ok()?;
                let r = ((v >> 16) & 0xff) as u8;
                let g = ((v >> 8) & 0xff) as u8;
                let b = (v & 0xff) as u8;
                Some(Color::from_rgba8(r, g, b, 255))
            }
            8 => {
                let v = u32::from_str_radix(hex, 16).ok()?;
                let r = ((v >> 24) & 0xff) as u8;
                let g = ((v >> 16) & 0xff) as u8;
                let b = ((v >> 8) & 0xff) as u8;
                let a = (v & 0xff) as u8;
                Some(Color::from_rgba8(r, g, b, a))
            }
            _ => None,
        }
    }

    /// 🎨️ Accepts `#rgb`/`#rrggbb`/`#rrggbbaa` or CSS `hsl()` / `hsla()` (comma or space syntax, optional `/` alpha).
    pub(super) fn parse_css_color(s: &str) -> Option<Color> {
        if let Some(c) = Self::parse_css_hex_color(s) {
            return Some(c);
        }
        Self::parse_css_hsl_color(s)
    }

    fn parse_css_hsl_color(s: &str) -> Option<Color> {
        let low = s.trim().to_ascii_lowercase();
        let (legacy_alpha_form, inner) = if let Some(inner) = low.strip_prefix("hsla(").and_then(|x| x.strip_suffix(')')) {
            (true, inner)
        } else {
            let inner = low.strip_prefix("hsl(").and_then(|x| x.strip_suffix(')'))?;
            (false, inner)
        };
        let inner = inner.trim();
        let (main, alpha_slash) = inner.split_once('/').map_or((inner, None), |(a, b)| (a.trim(), Some(b.trim())));
        let normalized = main.replace(',', " ");
        let parts: Vec<&str> = normalized.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }
        let h = Self::parse_css_hsl_hue(parts[0])?;
        let sat = Self::parse_css_hsl_sl(parts[1])?;
        let light = Self::parse_css_hsl_sl(parts[2])?;
        let alpha = if let Some(a) = alpha_slash {
            Self::parse_css_alpha_channel(a)?
        } else if legacy_alpha_form && parts.len() >= 4 {
            Self::parse_css_alpha_channel(parts[3])?
        } else {
            1.0
        };
        let (r, g, b) = Self::hsl_to_rgb_bytes(h, sat, light);
        let a = (alpha * 255.0).round().clamp(0.0, 255.0) as u8;
        Some(Color::from_rgba8(r, g, b, a))
    }

    fn parse_css_hsl_hue(tok: &str) -> Option<f64> {
        let t = tok.trim();
        let n = t.strip_suffix("deg").map_or(t, str::trim);
        let v: f64 = n.parse().ok()?;
        v.is_finite().then_some(v)
    }

    fn parse_css_hsl_sl(tok: &str) -> Option<f64> {
        let t = tok.trim();
        if let Some(p) = t.strip_suffix('%') {
            let v: f64 = p.trim().parse().ok()?;
            Some((v / 100.0).clamp(0.0, 1.0))
        } else {
            let v: f64 = t.parse().ok()?;
            Some(v.clamp(0.0, 1.0))
        }
    }

    fn parse_css_alpha_channel(tok: &str) -> Option<f64> {
        Self::parse_css_hsl_sl(tok)
    }

    fn board_hsl_hue_to_rgb_component(p: f64, q: f64, mut t: f64) -> f64 {
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 0.5 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        }
    }

    fn hsl_to_rgb_bytes(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
        let h_norm = ((h % 360.0 + 360.0) % 360.0) / 360.0;
        let s = s.clamp(0.0, 1.0);
        let l = l.clamp(0.0, 1.0);
        if s <= f64::EPSILON {
            let v = (l * 255.0).round().clamp(0.0, 255.0) as u8;
            return (v, v, v);
        }
        let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
        let p = 2.0 * l - q;
        let r = Self::board_hsl_hue_to_rgb_component(p, q, h_norm + 1.0 / 3.0);
        let g = Self::board_hsl_hue_to_rgb_component(p, q, h_norm);
        let b = Self::board_hsl_hue_to_rgb_component(p, q, h_norm - 1.0 / 3.0);
        ((r * 255.0).round().clamp(0.0, 255.0) as u8, (g * 255.0).round().clamp(0.0, 255.0) as u8, (b * 255.0).round().clamp(0.0, 255.0) as u8)
    }

    fn explicit_style_kind(style: Option<&str>) -> Option<BoardElementStyleKind> {
        match style.map(str::trim).filter(|s| !s.is_empty()) {
            Some("original") => Some(BoardElementStyleKind::Original),
            Some("neutral") => Some(BoardElementStyleKind::Neutral),
            Some("hovered") => Some(BoardElementStyleKind::Hovered),
            Some("selected") => Some(BoardElementStyleKind::Selected),
            Some("highlighted") => Some(BoardElementStyleKind::Highlighted),
            Some("disabled") => Some(BoardElementStyleKind::Disabled),
            _ => None,
        }
    }

    fn hovered_style_kind(&self, id: &str, domain: &str, element_kind: &str) -> Option<BoardElementStyleKind> {
        if self.is_preselect_active() {
            return None;
        }
        if self.selection.contains(id) {
            return None;
        }
        if self.hovered_id.as_deref() == Some(id) {
            return Some(BoardElementStyleKind::Hovered);
        }
        if self.hovered_id.is_none() {
            if let Some((hover_domain, hover_kind)) = self.hovered_kind.as_ref() {
                if hover_domain == domain && hover_kind == element_kind {
                    return Some(BoardElementStyleKind::Hovered);
                }
            }
        }
        None
    }

    fn highlighted_style_kind(&self, id: &str) -> Option<BoardElementStyleKind> {
        if self.is_preselect_active() || self.selection.contains(id) {
            return None;
        }
        if self.hovered_id.as_deref() == Some(id) {
            return None;
        }
        if self.highlighted_ids.contains(id) {
            return Some(BoardElementStyleKind::Highlighted);
        }
        None
    }

    pub(super) fn resolve_element_kind_hover(&self, id: &str) -> Option<(String, String)> {
        if let Some(node) = self.nodes.get(id) {
            return Some(("node".to_string(), node.node_kind.clone()));
        }
        if let Some(handle) = self.handles.get(id) {
            return Some(("handle".to_string(), handle.handle_kind.clone()));
        }
        if let Some(edge) = self.edges.get(id) {
            return Some(("edge".to_string(), edge.edge_kind.clone()));
        }
        if let Some(wire) = self.wires.get(id) {
            return Some(("wire".to_string(), wire.wire_kind.clone()));
        }
        None
    }

    fn ids_matching_kind_hover(&self) -> Vec<String> {
        let Some((domain, kind_id)) = self.hovered_kind.as_ref() else {
            return Vec::new();
        };
        let mut ids = Vec::new();
        match domain.as_str() {
            "node" => {
                for node in self.nodes.values() {
                    if &node.node_kind == kind_id && !self.selection.contains(&node.id) {
                        ids.push(node.id.clone());
                    }
                }
            }
            "handle" => {
                for handle in self.handles.values() {
                    if &handle.handle_kind == kind_id && !self.selection.contains(&handle.id) {
                        ids.push(handle.id.clone());
                    }
                }
            }
            "edge" => {
                for edge in self.edges.values() {
                    if &edge.edge_kind == kind_id && !self.selection.contains(&edge.id) {
                        ids.push(edge.id.clone());
                    }
                }
            }
            "wire" => {
                for wire in self.wires.values() {
                    if &wire.wire_kind == kind_id && !self.selection.contains(&wire.id) {
                        ids.push(wire.id.clone());
                    }
                }
            }
            _ => {}
        }
        ids
    }

    pub(super) fn is_preselect_active(&self) -> bool {
        self.is_preselecting() || !self.preselect.is_empty()
    }

    /// 🎨️ During area-select: preselect → Selected; anchor∖preselect → Highlighted; idle selection → Selected.
    fn resolve_interaction_style_kind(&self, id: &str) -> BoardElementStyleKind {
        if self.is_preselect_active() {
            if self.preselect.contains(id) {
                return BoardElementStyleKind::Selected;
            }
            if self.selection.contains(id) {
                return BoardElementStyleKind::Highlighted;
            }
            return BoardElementStyleKind::Neutral;
        }
        if self.selection.contains(id) {
            return BoardElementStyleKind::Selected;
        }
        if self.highlighted_ids.contains(id) {
            return BoardElementStyleKind::Highlighted;
        }
        BoardElementStyleKind::Neutral
    }

    fn locked_style_dim(kind: BoardElementStyleKind, locked: bool) -> BoardElementStyleKind {
        if locked && !matches!(kind, BoardElementStyleKind::Selected | BoardElementStyleKind::Highlighted | BoardElementStyleKind::Hovered) {
            BoardElementStyleKind::Disabled
        } else {
            kind
        }
    }

    pub(super) fn resolve_node_style_kind(&self, n: &NodeData, pass: StyleChromePass) -> BoardElementStyleKind {
        if let Some(kind) = Self::explicit_style_kind(n.style.as_deref()) {
            return Self::locked_style_dim(kind, n.locked);
        }
        let kind = match pass {
            StyleChromePass::CachedBase => {
                if self.preserve_original_element_style {
                    BoardElementStyleKind::Original
                } else {
                    BoardElementStyleKind::Neutral
                }
            }
            StyleChromePass::InteractionOverlay => {
                if let Some(kind) = self.hovered_style_kind(n.id.as_str(), "node", n.node_kind.as_str()) {
                    return Self::locked_style_dim(kind, n.locked);
                }
                if let Some(kind) = self.highlighted_style_kind(n.id.as_str()) {
                    return Self::locked_style_dim(kind, n.locked);
                }
                self.resolve_interaction_style_kind(n.id.as_str())
            }
        };
        Self::locked_style_dim(kind, n.locked)
    }

    pub(super) fn resolve_handle_style_kind(&self, h: &HandleData, pass: StyleChromePass) -> BoardElementStyleKind {
        if let Some(kind) = Self::explicit_style_kind(h.style.as_deref()) {
            return Self::locked_style_dim(kind, h.locked);
        }
        let kind = match pass {
            StyleChromePass::CachedBase => {
                if self.preserve_original_element_style {
                    BoardElementStyleKind::Original
                } else {
                    BoardElementStyleKind::Neutral
                }
            }
            StyleChromePass::InteractionOverlay => {
                if let Some(kind) = self.hovered_style_kind(h.id.as_str(), "handle", h.handle_kind.as_str()) {
                    return Self::locked_style_dim(kind, h.locked);
                }
                if let Some(kind) = self.highlighted_style_kind(h.id.as_str()) {
                    return Self::locked_style_dim(kind, h.locked);
                }
                self.resolve_interaction_style_kind(h.id.as_str())
            }
        };
        Self::locked_style_dim(kind, h.locked)
    }

    fn resolve_edge_style_kind(&self, e: &EdgeData, pass: StyleChromePass) -> BoardElementStyleKind {
        if let Some(kind) = Self::explicit_style_kind(e.style.as_deref()) {
            return Self::locked_style_dim(kind, e.locked);
        }
        let kind = match pass {
            StyleChromePass::CachedBase => BoardElementStyleKind::Neutral,
            StyleChromePass::InteractionOverlay => {
                if let Some(kind) = self.hovered_style_kind(e.id.as_str(), "edge", e.edge_kind.as_str()) {
                    return Self::locked_style_dim(kind, e.locked);
                }
                if let Some(kind) = self.highlighted_style_kind(e.id.as_str()) {
                    return Self::locked_style_dim(kind, e.locked);
                }
                self.resolve_interaction_style_kind(e.id.as_str())
            }
        };
        Self::locked_style_dim(kind, e.locked)
    }

    pub(super) fn resolve_wire_style_kind(&self, w: &WireData, pass: StyleChromePass) -> BoardElementStyleKind {
        if let Some(kind) = Self::explicit_style_kind(w.style.as_deref()) {
            return Self::locked_style_dim(kind, w.locked);
        }
        let kind = match pass {
            StyleChromePass::CachedBase => BoardElementStyleKind::Neutral,
            StyleChromePass::InteractionOverlay => {
                if let Some(kind) = self.hovered_style_kind(w.id.as_str(), "wire", w.wire_kind.as_str()) {
                    return Self::locked_style_dim(kind, w.locked);
                }
                if let Some(kind) = self.highlighted_style_kind(w.id.as_str()) {
                    return Self::locked_style_dim(kind, w.locked);
                }
                self.resolve_interaction_style_kind(w.id.as_str())
            }
        };
        Self::locked_style_dim(kind, w.locked)
    }

    /// 💠️ Entity ids whose selection/preselect/hover chrome tints fills and strokes without rebuilding {@link BoardHost.world_content_cache}.
    pub(super) fn interaction_overlay_entity_ids(&self) -> BTreeSet<String> {
        let mut ids = BTreeSet::new();
        if self.is_preselect_active() {
            ids.extend(self.preselect.iter().cloned());
            ids.extend(self.selection.iter().cloned());
            ids.extend(self.preselect_removed.iter().cloned());
        } else {
            ids.extend(self.selection.iter().cloned());
            ids.extend(self.selection_exit_highlight.iter().cloned());
        }
        if let Some(ref hover_id) = self.hovered_id {
            if !self.is_preselect_active() && !self.selection.contains(hover_id) {
                ids.insert(hover_id.clone());
            }
        }
        if self.hovered_id.is_none() && !self.is_preselect_active() {
            for id in self.ids_matching_kind_hover() {
                ids.insert(id);
            }
        }
        for id in self.highlighted_ids.iter() {
            if !self.selection.contains(id) && self.hovered_id.as_deref() != Some(id.as_str()) {
                ids.insert(id.clone());
            }
        }
        ids
    }

    pub(super) fn chrome_pass_for_entity(&self, entity_id: &str, overlay_ids: &BTreeSet<String>) -> StyleChromePass {
        if overlay_ids.contains(entity_id) {
            StyleChromePass::InteractionOverlay
        } else {
            StyleChromePass::CachedBase
        }
    }

    pub(super) fn node_fill_for_style(theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
        match kind {
            BoardElementStyleKind::Hovered => theme.node_fill_hovered,
            BoardElementStyleKind::Selected => theme.node_fill_selected,
            BoardElementStyleKind::Highlighted => theme.node_fill_selection_exit,
            BoardElementStyleKind::Disabled => theme.node_fill_disabled,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.node_fill,
        }
    }

    pub(super) fn node_stroke_for_style(theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
        match kind {
            BoardElementStyleKind::Hovered => theme.node_stroke_hovered,
            BoardElementStyleKind::Selected => theme.node_stroke_selected,
            BoardElementStyleKind::Highlighted => theme.node_stroke_selection_exit,
            BoardElementStyleKind::Disabled => theme.node_stroke_disabled,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.node_stroke,
        }
    }

    pub(super) fn resolve_handle_fill_color(&self, h: &HandleData, theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
        match kind {
            BoardElementStyleKind::Hovered => theme.handle_fill_hovered,
            BoardElementStyleKind::Selected => theme.handle_fill_selected,
            BoardElementStyleKind::Highlighted => theme.handle_fill_selection_exit,
            BoardElementStyleKind::Disabled => theme.handle_fill_disabled,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => {
                if let Some(c) = h.color_fill {
                    return c;
                }
                if let Some(def) = self.handle_kinds.get(&h.handle_kind) {
                    return def.color;
                }
                theme.handle_fill
            }
        }
    }

    pub(super) fn resolve_handle_stroke_color(&self, _h: &HandleData, theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
        match kind {
            BoardElementStyleKind::Hovered => theme.handle_stroke_hovered,
            BoardElementStyleKind::Selected => theme.handle_stroke_selected,
            BoardElementStyleKind::Highlighted => theme.handle_stroke_selection_exit,
            BoardElementStyleKind::Disabled => theme.handle_stroke_disabled,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.handle_stroke,
        }
    }

    fn edge_stroke_for_style(theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
        match kind {
            BoardElementStyleKind::Hovered => theme.edge_stroke_hovered,
            BoardElementStyleKind::Selected => theme.edge_stroke_selected,
            BoardElementStyleKind::Highlighted => theme.edge_stroke_selection_exit,
            BoardElementStyleKind::Disabled => theme.edge_stroke_disabled,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.edge_stroke,
        }
    }

    fn lerp_color(a: Color, b: Color, t: f64) -> Color {
        let t = t.clamp(0.0, 1.0);
        let ac = a.to_rgba8();
        let bc = b.to_rgba8();
        Color::from_rgba8(
            (f64::from(ac.r) * (1.0 - t) + f64::from(bc.r) * t).round() as u8,
            (f64::from(ac.g) * (1.0 - t) + f64::from(bc.g) * t).round() as u8,
            (f64::from(ac.b) * (1.0 - t) + f64::from(bc.b) * t).round() as u8,
            (f64::from(ac.a) * (1.0 - t) + f64::from(bc.a) * t).round() as u8,
        )
    }

    pub(super) fn resolve_node_fill_color(&self, n: &NodeData, theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
        let theme_fill = Self::node_fill_for_style(theme, kind);
        match kind {
            BoardElementStyleKind::Hovered | BoardElementStyleKind::Selected | BoardElementStyleKind::Highlighted | BoardElementStyleKind::Disabled => theme_fill,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => {
                let kind_id = n.node_kind.trim();
                if kind_id.is_empty() {
                    return theme_fill;
                }
                self.node_kinds.get(kind_id).and_then(|def| def.color_fill).unwrap_or(theme_fill)
            }
        }
    }

    fn edge_stroke_for_kind_pattern(pattern: EdgeStrokePattern, width: f64) -> Stroke {
        use semio_framework_canvas::Cap;
        let mut stroke = Stroke::new(width);
        match pattern {
            EdgeStrokePattern::Solid => {}
            EdgeStrokePattern::Dashed => {
                stroke.set_dash_pattern(vec![width * 3.0, width * 2.0]);
            }
            EdgeStrokePattern::Dotted => {
                stroke.set_dash_pattern(vec![width * 0.35, width * 1.65]);
                stroke.set_start_cap(Cap::Round);
                stroke.set_end_cap(Cap::Round);
            }
        }
        stroke
    }

    pub(super) fn resolve_edge_stroke_paint(&self, e: &EdgeData, chrome_pass: StyleChromePass, lod: BoardDrawLod, lod_scale_width: f64) -> (Color, Stroke, f64) {
        let style_kind = self.resolve_edge_style_kind(e, chrome_pass);
        let chrome = Self::edge_stroke_for_style(&self.canvas_theme, style_kind);
        let kind_def = self.edge_kinds.get(e.edge_kind.as_str());
        let base_color = kind_def.and_then(|d| d.color).unwrap_or(self.canvas_theme.edge_stroke);
        let stroke_color = match style_kind {
            BoardElementStyleKind::Neutral | BoardElementStyleKind::Original => base_color,
            _ if lod == BoardDrawLod::Minimap => chrome,
            _ => Self::lerp_color(base_color, chrome, 0.55),
        };
        let catalog_w = kind_def.map_or(2.0, |d| d.stroke_width);
        let width_mult = match style_kind {
            BoardElementStyleKind::Selected => ui_styling::strokes::EDGE_SELECTED_MULT,
            BoardElementStyleKind::Hovered => ui_styling::strokes::EDGE_HOVERED_MULT,
            _ => 1.0,
        };
        let width = lod_scale_width * (catalog_w / 2.0) * width_mult;
        let pattern = kind_def.map_or(EdgeStrokePattern::Solid, |d| d.pattern);
        (stroke_color, Self::edge_stroke_for_kind_pattern(pattern, width), width)
    }

    pub(super) fn resolve_edge_tips<'a>(&'a self, e: &EdgeData) -> (Option<&'a EdgeTipDef>, Option<&'a EdgeTipDef>) {
        let kind_def = self.edge_kinds.get(e.edge_kind.as_str());
        let source_slot = e.source_tip.as_deref().or_else(|| kind_def.and_then(|d| d.source_tip.as_deref()));
        let target_slot = e.target_tip.as_deref().or_else(|| kind_def.and_then(|d| d.target_tip.as_deref()));
        let source = self.resolve_tip_slot(source_slot);
        let mut target = self.resolve_tip_slot(target_slot);
        if target.is_none() && target_slot.is_none() {
            let directed = kind_def.is_none_or(|d| d.directed);
            if directed {
                target = self.lookup_edge_tip("arrow");
            }
        }
        (source, target)
    }

    fn append_edge_tip(scene: &mut Scene, tip: Point, dir: Vec2, color: Color, stroke_width: f64, tip_def: &EdgeTipDef) {
        use semio_framework_canvas::BezPath;
        let len = dir.hypot();
        if len < 1e-9 {
            return;
        }
        let d = dir / len;
        let n = Vec2::new(-d.y, d.x);
        let sw = stroke_width.max(1.0) * tip_def.scale.max(0.25);
        match tip_def.geometry {
            EdgeTipGeometry::Arrow => {
                let length = sw * 4.2;
                let half_w = sw * 1.15;
                let base = tip - d * length;
                let mut path = BezPath::new();
                path.move_to(tip);
                path.line_to(base + n * half_w);
                path.line_to(base - n * half_w);
                path.close_path();
                if tip_def.filled {
                    scene.fill(FillRule::NonZero, Affine::IDENTITY, color, None, &path);
                } else {
                    scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &path);
                }
            }
            EdgeTipGeometry::FineArrow => {
                let length = sw * 3.2;
                let half_w = sw * 0.75;
                let base = tip - d * length;
                let mut path = BezPath::new();
                path.move_to(tip);
                path.line_to(base + n * half_w);
                path.move_to(tip);
                path.line_to(base - n * half_w);
                let outline = Stroke::new((sw * 0.9).max(1.0));
                scene.stroke(&outline, Affine::IDENTITY, color, None, &path);
            }
            EdgeTipGeometry::Diamond => {
                let length = sw * 3.6;
                let half_w = sw * 1.05;
                let back = tip - d * length;
                let mid = tip - d * (length * 0.5);
                let mut path = BezPath::new();
                path.move_to(tip);
                path.line_to(mid + n * half_w);
                path.line_to(back);
                path.line_to(mid - n * half_w);
                path.close_path();
                if tip_def.filled {
                    scene.fill(FillRule::NonZero, Affine::IDENTITY, color, None, &path);
                } else {
                    scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &path);
                }
            }
            EdgeTipGeometry::Circle => {
                let r = sw * 1.4;
                let center = tip - d * r;
                let circle = Circle::new(center, r);
                if tip_def.filled {
                    scene.fill(FillRule::NonZero, Affine::IDENTITY, color, None, &circle);
                } else {
                    scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &circle);
                }
            }
            EdgeTipGeometry::Bar => {
                let half = sw * ui_styling::strokes::EDGE_TIP_MIN;
                let center = tip - d * (sw * 0.5);
                let mut path = BezPath::new();
                path.move_to(center + n * half);
                path.line_to(center - n * half);
                scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &path);
            }
        }
    }

    pub(super) fn append_edge_tips_on_curve(scene: &mut Scene, curve: &CubicBez, color: Color, stroke_w: f64, source: Option<&EdgeTipDef>, target: Option<&EdgeTipDef>) {
        let inset = stroke_w * 0.35;
        if let Some(tip_def) = target {
            let mut tangent = curve.p3() - curve.p2();
            if tangent.hypot() < 1e-9 {
                tangent = curve.p3() - curve.p1();
            }
            if tangent.hypot() >= 1e-9 {
                let dir = tangent / tangent.hypot();
                let tip = curve.p3() - dir * inset;
                Self::append_edge_tip(scene, tip, tangent, color, stroke_w, tip_def);
            }
        }
        if let Some(tip_def) = source {
            let mut tangent = curve.p0() - curve.p1();
            if tangent.hypot() < 1e-9 {
                tangent = curve.p0() - curve.p2();
            }
            if tangent.hypot() >= 1e-9 {
                let dir = tangent / tangent.hypot();
                let tip = curve.p0() - dir * inset;
                Self::append_edge_tip(scene, tip, tangent, color, stroke_w, tip_def);
            }
        }
    }

    pub(super) fn wire_stroke_for_style(theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
        match kind {
            BoardElementStyleKind::Hovered => theme.wire_stroke_hovered,
            BoardElementStyleKind::Selected => theme.wire_stroke_selected,
            BoardElementStyleKind::Highlighted => theme.wire_stroke_highlighted,
            BoardElementStyleKind::Disabled => theme.wire_stroke_disabled,
            BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.wire_stroke,
        }
    }

    pub(super) fn handles_link_compatible_for_drag(&self, source: &HandleData, target: &HandleData) -> bool {
        if !Self::handle_port_shapes_compatible(source.handle_kind.as_str(), target.handle_kind.as_str()) {
            return false;
        }
        if self.link_compat_rules.is_empty() {
            return true;
        }
        let mut matched: Vec<&LinkCompatRule> = self.link_compat_rules.iter().filter(|rule| self.link_gesture_rule_applies(rule, source, target)).collect();
        if matched.is_empty() {
            return false;
        }
        if matched.iter().any(|r| r.important) {
            matched.retain(|r| r.important);
        } else {
            let max_rank = matched.iter().map(|r| r.specificity as i32).max().unwrap_or(0);
            matched.retain(|r| (r.specificity as i32) == max_rank);
        }
        !matched.is_empty()
    }

    fn compat_pair_matches(rule: &LinkCompatRule, a: &str, b: &str) -> bool {
        if rule.source == a && rule.target == b {
            return true;
        }
        if rule.bidirectional && rule.source == b && rule.target == a {
            return true;
        }
        false
    }

    fn handle_port_shape(handle_kind: &str) -> Option<&'static str> {
        if handle_kind.contains(" circular ") {
            Some("circular")
        } else if handle_kind.contains(" rectangular ") {
            Some("rectangular")
        } else {
            None
        }
    }

    pub(super) fn handle_port_shapes_compatible(source_handle_kind: &str, target_handle_kind: &str) -> bool {
        match (Self::handle_port_shape(source_handle_kind), Self::handle_port_shape(target_handle_kind)) {
            (Some(a), Some(b)) => a == b,
            _ => true,
        }
    }

    fn single_letter_port_family(handle_kind: &str) -> Option<char> {
        let head = handle_kind.split('-').next()?;
        if head.len() == 1 {
            head.chars().next().filter(|c| c.is_ascii_lowercase())
        } else {
            None
        }
    }

    pub(super) fn single_letter_port_families_compatible(source_handle_kind: &str, target_handle_kind: &str) -> bool {
        match (Self::single_letter_port_family(source_handle_kind), Self::single_letter_port_family(target_handle_kind)) {
            (Some(a), Some(b)) => a == b,
            _ => true,
        }
    }

    pub(super) fn resolve_default_wire_kind_for_handle(&self, h: &HandleData) -> String {
        self.handle_kinds.get(&h.handle_kind).and_then(|d| d.default_wire_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string())
    }

    fn resolve_default_edge_kind_for_wire_kind(&self, wire_kind: &str) -> String {
        self.wire_kinds.get(wire_kind).and_then(|d| d.default_edge_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default()
    }

    fn link_gesture_rule_applies(&self, rule: &LinkCompatRule, source: &HandleData, target: &HandleData) -> bool {
        let w_src = self.resolve_default_wire_kind_for_handle(source);
        let w_tgt = self.resolve_default_wire_kind_for_handle(target);
        let e_src = self.resolve_default_edge_kind_for_wire_kind(&w_src);
        let e_tgt = self.resolve_default_edge_kind_for_wire_kind(&w_tgt);
        let sn = self.nodes.get(&source.node_id).map_or("", |n| n.node_kind.as_str());
        let tn = self.nodes.get(&target.node_id).map_or("", |n| n.node_kind.as_str());
        let sh = source.handle_kind.as_str();
        let th = target.handle_kind.as_str();
        match rule.specificity {
            CompatSpecificity::General => Self::compat_pair_matches(rule, sh, th),
            CompatSpecificity::Node => Self::compat_pair_matches(rule, sn, tn),
            CompatSpecificity::Edge => Self::compat_pair_matches(rule, e_src.as_str(), e_tgt.as_str()),
            CompatSpecificity::Handle => Self::compat_pair_matches(rule, sh, th),
            CompatSpecificity::Wire => Self::compat_pair_matches(rule, w_src.as_str(), th),
        }
    }

    pub(super) fn default_edge_kind_for_created_link(&self, source: &HandleData, _target: &HandleData) -> String {
        let wk = self.resolve_default_wire_kind_for_handle(source);
        self.resolve_default_edge_kind_for_wire_kind(&wk)
    }

    fn resolve_default_wire_kind_for_handle_kind(&self, handle_kind: &str) -> String {
        self.handle_kinds.get(handle_kind).and_then(|d| d.default_wire_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string())
    }

    pub(super) fn board_fill_default_wire_kind<'a>(&'a self, handle_kind: &str) -> Result<&'a str, BoardFillCaptureFault> {
        let Some(value) = self.handle_kinds.get(handle_kind).and_then(|kind| kind.default_wire_kind.as_deref()) else {
            return Ok(DEFAULT_WIRE_KIND_ID);
        };
        if value.len() > BOARD_FILL_TEXT_BYTES {
            return Err(BoardFillCaptureFault::TextCapacity);
        }
        let value = value.trim();
        Ok(if value.is_empty() { DEFAULT_WIRE_KIND_ID } else { value })
    }

    pub(super) fn board_fill_default_edge_kind<'a>(&'a self, wire_kind: &str) -> Result<&'a str, BoardFillCaptureFault> {
        let Some(value) = self.wire_kinds.get(wire_kind).and_then(|kind| kind.default_edge_kind.as_deref()) else {
            return Ok("");
        };
        if value.len() > BOARD_FILL_TEXT_BYTES {
            return Err(BoardFillCaptureFault::TextCapacity);
        }
        Ok(value.trim())
    }

    fn link_gesture_rule_applies_kind_strings(&self, rule: &LinkCompatRule, sn: &str, sh: &str, w_src: &str, e_src: &str, tn: &str, th: &str, _w_tgt: &str, e_tgt: &str) -> bool {
        match rule.specificity {
            CompatSpecificity::General => Self::compat_pair_matches(rule, sh, th),
            CompatSpecificity::Node => Self::compat_pair_matches(rule, sn, tn),
            CompatSpecificity::Edge => Self::compat_pair_matches(rule, e_src, e_tgt),
            CompatSpecificity::Handle => Self::compat_pair_matches(rule, sh, th),
            CompatSpecificity::Wire => Self::compat_pair_matches(rule, w_src, th),
        }
    }

    fn link_kinds_compatible_for_brush(&self, sn: &str, sh: &str, tn: &str, th: &str) -> bool {
        if !Self::handle_port_shapes_compatible(sh, th) {
            return false;
        }
        if !Self::single_letter_port_families_compatible(sh, th) {
            return false;
        }
        if self.link_compat_rules.is_empty() {
            return true;
        }
        let w_src = self.resolve_default_wire_kind_for_handle_kind(sh);
        let w_tgt = self.resolve_default_wire_kind_for_handle_kind(th);
        let e_src = self.resolve_default_edge_kind_for_wire_kind(&w_src);
        let e_tgt = self.resolve_default_edge_kind_for_wire_kind(&w_tgt);
        let mut matched: Vec<&LinkCompatRule> = self.link_compat_rules.iter().filter(|rule| self.link_gesture_rule_applies_kind_strings(rule, sn, sh, &w_src, &e_src, tn, th, &w_tgt, &e_tgt)).collect();
        if matched.is_empty() {
            return false;
        }
        if matched.iter().any(|r| r.important) {
            matched.retain(|r| r.important);
        } else {
            let max_rank = matched.iter().map(|r| r.specificity as i32).max().unwrap_or(0);
            matched.retain(|r| (r.specificity as i32) == max_rank);
        }
        !matched.is_empty()
    }

    fn brush_slot_hit_radius_world(&self) -> f64 {
        (self.brush_node_size * 0.5).max(1.0)
    }

    /// 🖌️ Brush slot anchor follows indirect-handle layout at overview/normal LOD so hit targets match painted rings.
    pub(super) fn brush_handle_anchor_world(&self, h: &HandleData) -> Option<Point> {
        if matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
            self.indirect_handle_world_pos(h).or_else(|| self.handle_world_pos(h))
        } else {
            self.handle_world_pos(h)
        }
    }

    fn brush_effective_suggestion_offset(&self) -> f64 {
        if self.brush_alt_pressed || self.brush_slot_suggestions_active {
            self.suggestion_offset
        } else {
            0.0
        }
    }

    pub(super) fn handle_slot_center_world(&self, node_id: &str, hw: Point, offset: f64) -> Option<Point> {
        let n = self.nodes.get(node_id)?;
        let nc = Point::new(n.x, n.y);
        let normal = normalize_or_zero(hw - nc);
        Some(hw + normal * offset)
    }

    fn brush_slot_center_world(&self, h: &HandleData) -> Option<Point> {
        let hw = self.brush_handle_anchor_world(h)?;
        self.handle_slot_center_world(h.node_id.as_str(), hw, self.brush_effective_suggestion_offset())
    }

    /// 🖌️ World distance from pointer to brush slot when the pointer is on the slot, anchor, or sole-free node body.
    fn brush_slot_pointer_hit_distance(&self, world: Point, handle_id: &str, h: &HandleData) -> Option<f64> {
        let slot_center = self.brush_slot_center_world(h)?;
        let zoom = self.camera.zoom.max(1e-9);
        let slot_hit_r = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.brush_slot_hit_radius_world();
        let d_slot = distance_between(world, slot_center);
        if d_slot <= slot_hit_r {
            return Some(d_slot);
        }
        let anchor = self.brush_handle_anchor_world(h)?;
        let anchor_hit_r = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h).max(self.effective_handle_radius(h));
        if distance_between(world, anchor) <= anchor_hit_r {
            return Some(d_slot);
        }
        if self.sole_eligible_indirect_handle_on_node(&h.node_id).as_deref() == Some(handle_id) {
            let n = self.nodes.get(&h.node_id)?;
            if self.point_in_node_world(n, world) {
                return Some(d_slot);
            }
        }
        None
    }

    pub(super) fn brush_nearest_slot_source(&self, world: Point) -> Option<String> {
        let mut best: Option<(f64, String)> = None;
        for (hid, h) in &self.handles {
            if !self.handle_effectively_visible(hid.as_str()) || self.handle_has_incident_edge(hid.as_str()) {
                continue;
            }
            let Some(d) = self.brush_slot_pointer_hit_distance(world, hid.as_str(), h) else {
                continue;
            };
            if best.as_ref().is_none_or(|(bd, _)| d < *bd) {
                best = Some((d, hid.clone()));
            }
        }
        best.map(|(_, id)| id)
    }

    pub(super) fn brush_compatible_candidates(&self, source: &HandleData) -> Option<BrushCandidatePage> {
        let sn = self.nodes.get(&source.node_id).map_or("", |n| n.node_kind.as_str());
        let sh = source.handle_kind.as_str();
        let mut out = BrushCandidatePage::default();
        for (kind_id, kind) in &self.node_kinds {
            if kind.handles.is_empty() {
                continue;
            }
            let tn = kind_id.as_str();
            for (i, tmpl) in kind.handles.iter().enumerate() {
                if self.link_kinds_compatible_for_brush(sn, sh, tn, tmpl.handle_kind.as_str()) {
                    let delta = Self::brush_handle_alignment_delta(source.angle, tmpl.angle);
                    out.push(kind_id, i, delta).ok()?;
                }
            }
        }
        out.sort();
        Some(out)
    }

    fn brush_handle_alignment_delta(source_handle_angle: f64, target_template_angle: f64) -> f64 {
        let desired = source_handle_angle + std::f64::consts::PI;
        let mut d = (target_template_angle - desired).abs();
        if d > std::f64::consts::PI {
            d = std::f64::consts::TAU - d;
        }
        d
    }

    pub(super) fn brush_template_world_pos(&self, center: Point, shape: NodeShape, radius: f64, width: f64, height: f64, angle: f64) -> Point {
        match shape {
            NodeShape::Circle => handle_position_on_circle(center, radius, angle),
            NodeShape::Rectangle => handle_position_on_rectangle(center, width, height, angle),
        }
    }

    pub(super) fn brush_build_preview(&self, source_handle_id: &str, candidate: BrushCandidateRef<'_>) -> Option<BrushPreviewSnapshot> {
        self.brush_build_preview_with_offset(source_handle_id, candidate, self.brush_effective_suggestion_offset())
    }

    pub(super) fn brush_build_preview_with_offset(&self, source_handle_id: &str, candidate: BrushCandidateRef<'_>, offset: f64) -> Option<BrushPreviewSnapshot> {
        let source = self.handles.get(source_handle_id)?;
        let kind = self.node_kinds.get(candidate.node_kind_id)?;
        let center = self.handle_slot_center_world(&source.node_id, self.brush_handle_anchor_world(source)?, offset)?;
        let target_handle_index = candidate.target_handle_index;
        kind.handles.get(target_handle_index)?;
        let node_kind_id = candidate.node_kind_id;
        let radius = self.brush_node_size * 0.5 * kind.scale;
        let (width, height) = if kind.shape == NodeShape::Rectangle { (self.brush_node_size * kind.scale, self.brush_node_size * kind.scale) } else { (radius * 2.0, radius * 2.0) };
        Some(BrushPreviewSnapshot {
            source_handle_id: source_handle_id.to_string(),
            node_kind_id: node_kind_id.to_string(),
            x: center.x,
            y: center.y,
            shape: kind.shape,
            radius,
            width,
            height,
            handles: kind.handles.clone(),
            target_handle_index,
            icon_kind: kind.icon.clone(),
        })
    }

    pub(super) fn brush_sync_preview_events(&mut self) {
        if self.brush_candidates.len() > BOARD_POINTER_ITEM_CAPACITY || self.brush_preview.as_ref().is_some_and(|preview| preview.handles.len() > BOARD_POINTER_ITEM_CAPACITY) {
            self.event_schema_fault = true;
            return;
        }
        let key = self.brush_preview.as_ref().map(|p| format!("{}|{}|{}|{}|{}", p.source_handle_id, p.node_kind_id, p.target_handle_index, p.x, p.y)).unwrap_or_default();
        let mut candidates_key = format!("{}|", self.brush_slot_source_id.as_deref().unwrap_or(""));
        for (index, candidate) in self.brush_candidates.iter().enumerate() {
            if index > 0 {
                candidates_key.push(',');
            }
            candidates_key.push_str(candidate.node_kind_id);
            candidates_key.push('#');
            candidates_key.push_str(&candidate.target_handle_index.to_string());
        }
        candidates_key.push('|');
        candidates_key.push_str(&self.brush_candidate_index.to_string());
        let preview_changed = self.brush_preview_emit_key.as_deref() != Some(key.as_str());
        let candidates_changed = self.brush_candidates_emit_key.as_deref() != Some(candidates_key.as_str());
        let preview_event = preview_changed.then(|| BoardOwnedEvent::brush_preview(self.brush_preview.as_ref()));
        let candidate_event = candidates_changed.then(|| BoardOwnedEvent::brush_candidates(self.brush_slot_source_id.as_deref().unwrap_or(""), &self.brush_candidates, self.brush_candidate_index, self.brush_slot_suggestions_active));
        let (first, second) = match (preview_event, candidate_event) {
            (Some(first), second) => (first, second),
            (None, Some(first)) => (first, None),
            (None, None) => return,
        };
        let Some(reservation) = self.reserve_owned_batch(first, second) else {
            return;
        };
        if preview_changed {
            self.brush_preview_emit_key = Some(key);
        }
        if candidates_changed {
            self.brush_candidates_emit_key = Some(candidates_key);
        }
        self.publish_event_batch(reservation);
    }

    pub(super) fn brush_clear_slot(&mut self) {
        let had_preview = self.brush_preview.is_some();
        let clear_hover = self.brush_slot_suggestions_active;
        self.brush_slot_suggestions_active = false;
        self.brush_slot_source_id = None;
        self.brush_candidates.clear();
        self.brush_candidate_index = 0;
        self.brush_preview = None;
        if clear_hover {
            self.set_hovered_id(None);
        }
        if had_preview {
            self.bump_content_scene_generation();
            self.brush_preview_emit_key = None;
            self.brush_candidates_emit_key = None;
            self.brush_sync_preview_events();
        }
    }

    pub(super) fn brush_commit_preview(&mut self) {
        let Some(preview) = self.brush_preview.as_ref() else {
            return;
        };
        let serial = self.brush_placement_serial.wrapping_add(1);
        let node_id = format!("puzzle2d.brush.{serial}");
        let edge_id = format!("puzzle2d.brush.edge.{serial}");
        let Some(reservation) = self.reserve_owned_event(BoardOwnedEvent::brush_place(preview, &node_id, &edge_id)) else {
            return;
        };
        self.brush_placement_serial = serial;
        let _ = self.brush_preview.take();
        self.publish_event_reservation(reservation);
        self.bump_content_scene_generation();
        self.brush_preview_emit_key = None;
    }

    pub(super) fn brush_finish_slot(&mut self) {
        if self.brush_alt_pressed {
            self.brush_commit_preview();
        }
        self.brush_clear_slot();
    }

    pub(super) fn brush_update_alt(&mut self, alt: bool) {
        if self.brush_alt_pressed == alt {
            return;
        }
        self.brush_alt_pressed = alt;
        if self.brush_slot_source_id.is_some() {
            self.brush_preview_emit_key = None;
            self.brush_rebuild_preview();
        }
    }
}
// #endregion board_host
