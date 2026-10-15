//! 🛠️ DAG host mutators: selection and hover setters, LOD and grid, journal, note editing and pointer-down routing.

use super::*;

// #region 🔖️DagHost
impl DagHost {
    /// 📍️ Sets a snapshot widget position in both the snapshot and engine snapshots.
    pub fn set_widget_position(&mut self, widget_id: &str, x: f64, y: f64) -> Result<(), DagError> {
        let idx = self.host_snapshot.nodes.iter().position(|node| node.id == widget_id).ok_or_else(|| DagError::UnknownWidget(widget_id.to_string()))?;
        self.host_snapshot.nodes[idx].x = x;
        self.host_snapshot.nodes[idx].y = y;
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return Ok(());
        };
        if let Some(node) = self.engine.nodes.get_mut(&nid) {
            node.center = canvas::Point::new(x, y);
        }
        Ok(())
    }

    /// 🗑️ Deletes the current selection from the snapshot.
    pub fn delete_selected(&mut self) {
        let widget_ids = self.selected_node_ids();
        self.engine.delete_selection();
        self.host_snapshot.nodes.retain(|node| !widget_ids.contains(&node.id));
        self.sync_edges_from_engine();
        self.rebuild_engine_with_layout(false);
    }

    /// ⌨️ Selects every snapshot node id.
    pub fn select_all_node_ids(&self) -> Vec<String> {
        self.host_snapshot.nodes.iter().map(|node| node.id.clone()).collect()
    }

    pub fn select_all(&mut self) {
        self.engine.select_all();
    }

    /// 🖱️ Sets hover to a snapshot widget id, or clears hover.
    pub fn set_hover(&mut self, widget_id: Option<&str>) {
        let next = widget_id.and_then(|id| self.node_id_for_widget_id(id));
        if self.engine.hover != next {
            self.engine.hover = next;
        }
    }

    /// 🔌️ Sets hover to a snapshot channel handle, falling back to node hover below channel LOD.
    pub fn set_hover_channel(&mut self, widget_id: Option<&str>, port_id: Option<&str>) {
        let Some(widget_id) = widget_id else {
            self.set_hover(None);
            return;
        };
        if let Some(port_id) = port_id {
            if self.draw_lod_for_frame().uses_channel_row_pick() {
                if let Some(hid) = self.handle_id_for_port(widget_id, port_id) {
                    if self.engine.hover != Some(hid) {
                        self.engine.hover = Some(hid);
                    }
                    return;
                }
            }
        }
        self.set_hover(Some(widget_id));
    }

    /// 🔌️ Replaces typed channel selection, using node selection below channel LOD.
    pub fn set_selected_channels(&mut self, channels: &[DagChannelRef]) {
        if self.draw_lod_for_frame().uses_channel_row_pick() {
            let mut selection = Selection::default();
            for channel in channels {
                if let Some(hid) = self.handle_id_for_port(&channel.widget_id, &channel.port) {
                    selection.handle_ids.insert(hid);
                }
            }
            self.engine.selection = selection;
            self.engine.preselect = Selection::default();
            self.engine.preselect_removed = Selection::default();
            return;
        }
        let widget_ids: Vec<String> = channels.iter().map(|channel| channel.widget_id.clone()).collect();
        self.set_selection(&widget_ids);
    }

    /// 🌫️ Marks preview-off nodes as dimmed on the canvas.
    pub fn set_dimmed(&mut self, widget_ids: &[String]) {
        self.dimmed.clear();
        for widget_id in widget_ids {
            if let Some(nid) = self.node_id_for_widget_id(widget_id) {
                self.dimmed.insert(nid);
            }
        }
    }

    /// ⚙️ Marks one actively computing widget and downstream widgets as stale.
    pub fn set_computing_progress(&mut self, active_widget_id: Option<&str>, stale_widget_ids: &[String]) {
        self.computing_active = active_widget_id.and_then(|widget_id| self.node_id_for_widget_id(widget_id));
        self.computing_stale.clear();
        for widget_id in stale_widget_ids {
            if let Some(nid) = self.node_id_for_widget_id(widget_id) {
                if self.computing_active != Some(nid) {
                    self.computing_stale.insert(nid);
                }
            }
        }
    }

    /// ✅️ Clears evaluating chrome from all nodes.
    pub fn clear_computing(&mut self) {
        self.computing_active = None;
        self.computing_stale.clear();
        self.node_eval_status.reset();
        self.unresolved_input_ports.clear();
    }

    /// 🚦️ Replaces chrome from complete admitted per-widget evaluation facts.
    pub fn set_node_statuses(&mut self,statuses:&DagNodeStatuses) {
        self.computing_active=None;
        self.computing_stale.clear();
        self.node_eval_status.reset();
        self.unresolved_input_ports.clear();
        for (widget_id,status)in statuses {
            let Some(nid)=self.node_id_for_widget_id(widget_id)else{continue};
            let kind=match status {
                DagNodeEvaluationStatus::Computing{}=>{self.computing_active=Some(nid);DagNodeEvalStatusKind::Computing},
                DagNodeEvaluationStatus::Queued{}=>{self.computing_stale.insert(nid);DagNodeEvalStatusKind::Queued},
                DagNodeEvaluationStatus::Error{..}=>DagNodeEvalStatusKind::Error,
                DagNodeEvaluationStatus::Blocked{ports}=>{for port in ports{if let Some(node)=self.host_snapshot.nodes.get((nid-1)as usize){for(index,input)in node.inputs().iter().enumerate(){if input.id==*port{self.unresolved_input_ports.insert((nid,index));}}}}DagNodeEvalStatusKind::Blocked},
                DagNodeEvaluationStatus::Ok{}=>DagNodeEvalStatusKind::Ok,
            };
            self.node_eval_status.insert(nid,kind);
        }
    }

    pub(super) fn tick_computing_animation(&self) {
        if self.computing_active.is_some() {
            let next = (self.computing_active_anim_phase.get() + 0.02) % 1.0;
            self.computing_active_anim_phase.set(next);
        }
        if !self.computing_stale.is_empty() {
            let next = (self.computing_stale_anim_phase.get() + 0.008) % 1.0;
            self.computing_stale_anim_phase.set(next);
        }
    }

    /// 📋️ Preview-off snapshot node ids currently dimmed on the canvas.
    pub fn dimmed_node_ids(&self) -> Vec<String> {
        self.dimmed.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect()
    }

    /// ➕️ Returns and clears a pending variadic insert request from the last pointer down.
    pub fn take_pending_port_insert(&mut self) -> Option<(DagPortSide, String, usize)> {
        self.pending_port_insert.take()
    }

    /// 🎯️ Hit-tests variadic `+` controls; returns side, node id, and insert index.
    pub fn port_insert_hit(&self, world_x: f64, world_y: f64, zoom: f64) -> Option<(DagPortSide, String, usize)> {
        if zoom < DAG_VARIADIC_PLUS_ZOOM_THRESHOLD {
            return None;
        }
        for node in self.host_snapshot.nodes.iter().rev() {
            if node.variadic_inputs() {
                let inputs = node.inputs();
                let hw = node.width * 0.5;
                let row = inputs.len() + DAG_COMPUTATION_HEADER_ROWS;
                let (x0, y0, hit_x1, y1) = {
                    let (x0, y0, _x1, y1) = channel_row_bounds(node, row);
                    (x0, y0, node.x - hw * 0.5, y1)
                };
                if point_in_rect(world_x, world_y, x0, y0, hit_x1, y1) {
                    return Some((DagPortSide::Input, node.id.clone(), inputs.len()));
                }
            }
            if node.variadic_outputs() {
                let outputs = node.outputs();
                let row = outputs.len() + DAG_COMPUTATION_HEADER_ROWS;
                let (x0, y0, x1, y1) = channel_row_bounds(node, row);
                let hit_x0 = computation_output_column_x_bounds(node).map_or(x0, |(left, _)| left);
                if point_in_rect(world_x, world_y, hit_x0, y0, x1, y1) {
                    return Some((DagPortSide::Output, node.id.clone(), outputs.len()));
                }
            }
        }
        None
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.width = width.max(1);
        self.height = height.max(1);
        self.dpr = dpr.max(1.0);
    }

    /// 👻️ Sets or clears the placement preview node painted with the normal LOD path.
    pub fn set_ghost_node(&mut self, node: Option<DagNodeSpec>) {
        self.ghost_node = node;
    }

    pub fn ghost_node(&self) -> Option<&DagNodeSpec> {
        self.ghost_node.as_ref()
    }

    pub fn automatic_lod(&self) -> bool {
        self.automatic_lod
    }

    pub fn forced_draw_lod_label(&self) -> Option<&'static str> {
        self.forced_draw_lod.map(|lod| lod.label())
    }

    /// 🔍️ Pins draw LOD while the wheel gesture is active so chrome does not flicker across bands.
    pub fn set_wheel_zoom_active(&mut self, active: bool) {
        if active && !self.wheel_zoom_active {
            self.wheel_zoom_render_lod = Some(self.resolved_draw_lod());
        } else if !active {
            self.wheel_zoom_render_lod = None;
        }
        self.wheel_zoom_active = active;
    }

    fn resolved_draw_lod(&self) -> DagDrawLod {
        if !self.automatic_lod {
            if let Some(lod) = self.forced_draw_lod {
                return lod;
            }
        }
        dag_draw_lod(self.host_snapshot.camera.zoom)
    }

    pub(super) fn draw_lod_for_frame(&self) -> DagDrawLod {
        if !self.automatic_lod {
            if let Some(lod) = self.forced_draw_lod {
                return lod;
            }
        }
        if self.wheel_zoom_active {
            if let Some(pinned) = self.wheel_zoom_render_lod {
                return pinned;
            }
        }
        dag_draw_lod(self.host_snapshot.camera.zoom)
    }

    /// 📶️ Active draw LOD tier label (`minimap`, `overview`, …).
    pub fn draw_lod_label(&self) -> &'static str {
        self.draw_lod_for_frame().label()
    }

    /// 📶️ When true (default), camera zoom selects draw LOD; when false, optional `forced_draw_lod` pins the tier.
    pub fn set_automatic_lod(&mut self, enabled: bool) {
        self.automatic_lod = enabled;
        if enabled {
            self.forced_draw_lod = None;
        }
    }

    /// 🔗️ World-space proximity radius for channel auto-connect; `0` disables snapping.
    pub fn set_proximity_distance(&mut self, world: f64) {
        self.engine.proximity_distance_world = world.max(0.0);
    }

    /// 🔲️ Toggles LOD-tiered world grid painting.
    pub fn set_grid_visible(&mut self, visible: bool) {
        self.grid_visible = visible;
    }

    /// 🧲️ Toggles node-drag snap to the finest visible LOD grid step.
    pub fn set_grid_snap_enabled(&mut self, enabled: bool) {
        self.grid_snap_enabled = enabled;
    }

    /// 📐️ Sets the positive multiplier for LOD world grid steps.
    pub fn set_grid_factor(&mut self, factor: f64) -> Result<(), DagError> {
        if !factor.is_finite() || factor <= 0.0 || factor > 1e6 {
            return Err(DagError::GridFactorOutOfRange);
        }
        self.grid_factor = factor;
        Ok(())
    }

    /// 🔍️ Frames the current node selection in the viewport camera.
    pub fn focus_selection_camera(&self, pad: f64) -> Option<DagCamera> {
        let selected = self.selected_snapshot_nodes();
        if selected.is_empty() {
            return None;
        }
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for (_, node) in &selected {
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            min_x = min_x.min(node.x - hw);
            min_y = min_y.min(node.y - hh);
            max_x = max_x.max(node.x + hw);
            max_y = max_y.max(node.y + hh);
        }
        if !min_x.is_finite() {
            return None;
        }
        let pad = pad.max(1.05);
        let cx = (min_x + max_x) * 0.5;
        let cy = (min_y + max_y) * 0.5;
        let span_w = (max_x - min_x).max(1.0);
        let span_h = (max_y - min_y).max(1.0);
        let vw = self.width.max(1) as f64;
        let vh = self.height.max(1) as f64;
        let zoom = (vw / (span_w * pad)).min(vh / (span_h * pad)).clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX);
        Some(DagCamera { x: cx, y: cy, zoom })
    }

    fn grid_step_large_world(&self) -> f64 {
        GRID_WORLD_LARGE * self.grid_factor
    }

    fn grid_step_medium_world(&self) -> f64 {
        GRID_WORLD_MEDIUM * self.grid_factor
    }

    fn grid_step_small_world(&self) -> f64 {
        GRID_WORLD_SMALL * self.grid_factor
    }

    fn grid_step_micro_world(&self) -> f64 {
        GRID_WORLD_MICRO * self.grid_factor
    }

    fn lod_visible_grid_snap_step_world(&self) -> Option<f64> {
        match self.draw_lod_for_frame() {
            DagDrawLod::Minimap => None,
            DagDrawLod::Overview | DagDrawLod::Compact => Some(self.grid_step_large_world()),
            DagDrawLod::Normal => Some(self.grid_step_medium_world()),
            DagDrawLod::Detail => Some(self.grid_step_small_world()),
            DagDrawLod::Micro => Some(self.grid_step_micro_world()),
        }
    }

    fn snap_world_scalar(&self, value: f64) -> f64 {
        if !self.grid_snap_enabled {
            return value;
        }
        let Some(step) = self.lod_visible_grid_snap_step_world() else {
            return value;
        };
        (value / step).round() * step
    }

    fn snap_world_pair(&self, x: f64, y: f64) -> (f64, f64) {
        (self.snap_world_scalar(x), self.snap_world_scalar(y))
    }

    fn stroke_world_step_grid(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, color: canvas::Color, stroke_px: f64, world_step: f64, min_step_screen: f64) {
        use canvas::camera::world_to_screen;
        use canvas::{Affine, Point, Stroke};
        let step = world_step * cam.zoom;
        if step < min_step_screen {
            return;
        }
        let stroke = Stroke::new(stroke_px);
        let w = viewport.width as f64;
        let h = viewport.height as f64;
        let origin = world_to_screen(cam, viewport, Point::new(0.0, 0.0));
        let x_off = ((origin.x % step) + step) % step;
        let y_off = ((origin.y % step) + step) % step;
        let mut path = canvas::BezPath::new();
        let mut x = x_off;
        while x <= w {
            path.move_to(Point::new(x, 0.0));
            path.line_to(Point::new(x, h));
            x += step;
        }
        let mut y = y_off;
        while y <= h {
            path.move_to(Point::new(0.0, y));
            path.line_to(Point::new(w, y));
            y += step;
        }
        scene.stroke(&stroke, Affine::IDENTITY, color, None, &path);
    }

    pub(super) fn paint_lod_grid(&self, scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, lod: DagDrawLod) {
        if !self.grid_visible || self.wheel_zoom_active || lod == DagDrawLod::Minimap {
            return;
        }
        let grid_color = self.canvas_theme.grid_minor_stroke;
        Self::stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_LARGE, self.grid_step_large_world(), 0.0);
        match lod {
            DagDrawLod::Normal | DagDrawLod::Detail | DagDrawLod::Micro => {
                Self::stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_MEDIUM, self.grid_step_medium_world(), 0.0);
            }
            DagDrawLod::Minimap | DagDrawLod::Overview | DagDrawLod::Compact => {}
        }
        if matches!(lod, DagDrawLod::Detail | DagDrawLod::Micro) {
            Self::stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_SMALL, self.grid_step_small_world(), 0.0);
        }
        if lod == DagDrawLod::Micro {
            Self::stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_MICRO, self.grid_step_micro_world(), 0.0);
        }
    }

    /// 📶️ Pins WASM draw LOD when {@link DagHost::set_automatic_lod} is false; pass an empty label to follow zoom bands.
    pub fn set_forced_draw_lod_label(&mut self, label: &str) {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            self.forced_draw_lod = None;
            return;
        }
        self.forced_draw_lod = DagDrawLod::from_id(trimmed);
    }

    pub fn load_host_snapshot_json(json: &str) -> Result<Self, DagError> {
        let host_snapshot: DagHostSnapshot = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject)?;
        if host_snapshot.schema != "dag.hostDocument" {
            return Err(DagError::SchemaMismatch);
        }
        Ok(Self::from_host_snapshot(host_snapshot))
    }

    pub fn host_snapshot_json(&self) -> Result<String, DagError> {
        Ok(semio_framework_pack_json::to_json_string(&self.host_snapshot))
    }

    /// 🌳️ Recomputes node positions from the current graph using layered tree layout.
    pub fn reorganize(&mut self, opts: &DagLayoutOptions,control:&mut LayoutControl<'_>) -> Result<(), DagError> {
        apply_dag_layout(&mut self.host_snapshot,opts,control)?;
        self.rebuild_engine_with_layout(false);
        Ok(())
    }

    pub(super) fn rebuild_engine_with_layout(&mut self, apply_layout: bool) {
        self.engine = DagBoardEngine::new();
        self.engine.enforce_acyclic = true;
        self.node_id_map.clear();
        self.handle_key_map.clear();
        self.handle_port_shape.clear();
        self.handle_port_visible.clear();
        self.edge_id_map.clear();
        self.edge_engine_ids.clear();
        self.edge_engine_ids.resize(self.host_snapshot.edges.len(), None);
        self.edge_route_style.clear();
        for node in &mut self.host_snapshot.nodes {
            fit_node_size(node);
        }
        let (cx, cy, zoom) = (self.host_snapshot.camera.x, self.host_snapshot.camera.y, self.host_snapshot.camera.zoom);
        self.engine.set_camera(cx, cy, zoom);
        if apply_layout {
            let mut progress=|_|true;let mut control=LayoutControl::new(100_000_000,&mut progress);
            let _=apply_dag_layout(&mut self.host_snapshot,&DagLayoutOptions::default(),&mut control);
        }
        let mut next_handle: u64 = 10;
        let mut handle_map: HashMap<String, u64> = HashMap::new();
        for (idx, node) in self.host_snapshot.nodes.iter().enumerate() {
            let nid = idx as u64 + 1;
            self.node_id_map.insert(nid, idx);
            self.engine.create_rect_node(nid, node.x, node.y, node.width, node.height, true);
            let inputs = node.inputs();
            let outputs = node.outputs();
            for (port_idx, port) in inputs.iter().enumerate() {
                let in_a = io_node_rect_port_angle_for_node(node, port_idx, true);
                let hid = next_handle;
                next_handle += 1;
                let public_key = format!("{}@{}", node.id, port.id);
                handle_map.insert(Self::dag_port_handle_key(&node.id, &port.id, true), hid);
                self.handle_key_map.insert(hid, public_key);
                self.handle_port_shape.insert(hid, port.shape);
                self.handle_port_visible.insert(hid, port.visible);
                self.engine.create_handle(hid, nid, in_a);
                if let Some(handle) = self.engine.handles.get_mut(&hid) {
                    handle.radius = DAG_HANDLE_WORLD_RADIUS;
                }
                self.engine.set_handle_role(hid, HandleRole::Target);
                self.engine.set_handle_value_types(hid, port_value_types(port));
            }
            for (port_idx, port) in outputs.iter().enumerate() {
                let out_a = io_node_rect_port_angle_for_node(node, port_idx, false);
                let hid = next_handle;
                next_handle += 1;
                let public_key = format!("{}@{}", node.id, port.id);
                handle_map.insert(Self::dag_port_handle_key(&node.id, &port.id, false), hid);
                self.handle_key_map.insert(hid, public_key);
                self.handle_port_shape.insert(hid, port.shape);
                self.handle_port_visible.insert(hid, port.visible);
                self.engine.create_handle(hid, nid, out_a);
                if let Some(handle) = self.engine.handles.get_mut(&hid) {
                    handle.radius = DAG_HANDLE_WORLD_RADIUS;
                }
                self.engine.set_handle_role(hid, HandleRole::Source);
                self.engine.set_handle_value_types(hid, port_value_types(port));
            }
        }
        let existing: Vec<(String, String)> = self
            .host_snapshot
            .edges
            .iter()
            .filter_map(|e| {
                let src = e.source.split('@').next()?.to_string();
                let tgt = e.target.split('@').next()?.to_string();
                Some((src, tgt))
            })
            .collect();
        let mut eid: u64 = 100;
        for (edge_index, edge) in self.host_snapshot.edges.iter().enumerate() {
            if would_create_cycle(&existing, edge.source.split('@').next().unwrap_or(""), edge.target.split('@').next().unwrap_or("")) {
                continue;
            }
            let (source_node, source_port) = Self::dag_port_endpoint_parts(&edge.source);
            let (target_node, target_port) = Self::dag_port_endpoint_parts(&edge.target);
            let src = handle_map.get(&Self::dag_port_handle_key(&source_node, &source_port, false)).copied();
            let tgt = handle_map.get(&Self::dag_port_handle_key(&target_node, &target_port, true)).copied();
            if let (Some(s), Some(t)) = (src, tgt) {
                let id = Self::parse_snapshot_edge_numeric_id(&edge.id).unwrap_or(eid);
                eid = eid.max(id).saturating_add(1);
                self.engine.create_edge(id, s, t);
                self.edge_id_map.insert(id, edge.id.clone());
                self.edge_engine_ids[edge_index] = Some(id);
                self.edge_route_style.insert(id, edge.route_style);
            }
        }
        self.engine.set_next_edge_id(eid);
    }

    fn parse_snapshot_edge_numeric_id(id: &str) -> Option<u64> {
        id.strip_prefix('e').and_then(|s| s.parse().ok())
    }

    fn dag_port_endpoint_parts(endpoint: &str) -> (String, String) {
        endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), String::new()), |(node, port)| (node.to_string(), port.to_string()))
    }

    fn dag_port_handle_key(node_id: &str, port_id: &str, input: bool) -> String {
        format!("{}:{}:{}", node_id, if input { "in" } else { "out" }, port_id)
    }

    pub(super) fn screen_to_world_point(&self, sx: f64, sy: f64) -> canvas::Point {
        use canvas::camera::{screen_to_world, Camera as CanvasCamera, Viewport};
        use canvas::Point;
        let cam = CanvasCamera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom };
        let viewport = Viewport { width: self.width, height: self.height, dpr: self.dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    pub(super) fn sync_node_positions_from_engine(&mut self) {
        for (&nid, &idx) in &self.node_id_map {
            let Some(engine_node) = self.engine.nodes.get(&nid) else {
                continue;
            };
            let (mut x, mut y) = (engine_node.center.x, engine_node.center.y);
            if self.grid_snap_enabled {
                (x, y) = self.snap_world_pair(x, y);
            }
            self.host_snapshot.nodes[idx].x = x;
            self.host_snapshot.nodes[idx].y = y;
            if self.grid_snap_enabled {
                if let Some(engine_node) = self.engine.nodes.get_mut(&nid) {
                    engine_node.center = canvas::Point::new(x, y);
                }
            }
        }
    }

    pub(super) fn node_spec_for_paint<'a>(&'a self, idx: usize, node: &'a DagNodeSpec) -> std::borrow::Cow<'a, DagNodeSpec> {
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return std::borrow::Cow::Borrowed(node);
        };
        let Some(engine_node) = self.engine.nodes.get(&nid) else {
            return std::borrow::Cow::Borrowed(node);
        };
        if (engine_node.center.x - node.x).abs() < 1e-9 && (engine_node.center.y - node.y).abs() < 1e-9 {
            return std::borrow::Cow::Borrowed(node);
        }
        let mut synced = node.clone();
        synced.x = engine_node.center.x;
        synced.y = engine_node.center.y;
        std::borrow::Cow::Owned(synced)
    }

    pub(super) fn sync_edges_from_engine(&mut self) {
        let mut edges = Vec::with_capacity(self.engine.edges.len());
        let mut edge_engine_ids = Vec::with_capacity(self.engine.edges.len());
        for (eid, edge) in &self.engine.edges {
            let Some(source) = self.handle_key_map.get(&edge.source).cloned() else {
                continue;
            };
            let Some(target) = self.handle_key_map.get(&edge.target).cloned() else {
                continue;
            };
            let id = self.edge_id_map.get(eid).cloned().unwrap_or_else(|| format!("e{eid}"));
            self.edge_id_map.insert(*eid, id.clone());
            edges.push(DagHostSnapshotEdge { id, source, target, ..Default::default() });
            edge_engine_ids.push(Some(*eid));
        }
        self.host_snapshot.edges = edges;
        self.edge_engine_ids = edge_engine_ids;
    }

    pub(super) fn process_engine_events(&mut self) {
        let events = self.engine.drain_events();
        let mut moved = false;
        let mut wired = false;
        for event in events {
            match event {
                BoardEvent::NodeMoved { .. } => moved = true,
                BoardEvent::EdgeConnected { source, target, .. } => {
                    wired = true;
                    self.journal_connect(source, target);
                }
                BoardEvent::EdgeRemoved { id } => {
                    wired = true;
                    self.journal_disconnect(id);
                }
                BoardEvent::SelectionChanged { .. } | BoardEvent::PreselectChanged { .. } | BoardEvent::HoverChanged { .. } => {}
            }
        }
        if moved {
            self.sync_node_positions_from_engine();
        }
        if wired {
            self.sync_edges_from_engine();
        }
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.host_snapshot.camera = DagCamera { x, y, zoom };
        self.engine.set_camera(x, y, zoom);
    }

    /// 🔗️ Records one completed connection in the guest's own vocabulary. `handle_key_map` already
    /// keys every handle as `"{nodeId}@{portId}"` — the same grammar the pick-target/hover channel
    /// uses — so this is a lookup, never a second derivation of what a port is.
    fn journal_connect(&mut self, source: HandleId, target: HandleId) {
        let Some((source_node_id, source_port_id)) = self.handle_key_map.get(&source).and_then(|key| key.split_once('@')) else { return };
        let Some((target_node_id, target_port_id)) = self.handle_key_map.get(&target).and_then(|key| key.split_once('@')) else { return };
        let edit = DagGraphEdit::Connect { source_node_id: source_node_id.to_string(), source_port_id: source_port_id.to_string(), target_node_id: target_node_id.to_string(), target_port_id: target_port_id.to_string() };
        self.push_graph_edit(edit);
    }

    /// 🔗️ Records one removed wire by the SYNAPSE id the guest knows it as. An edge the engine
    /// created in this same gesture has no synapse id yet and is simply not journalled — the guest
    /// never learned about it, so it has nothing to remove.
    fn journal_disconnect(&mut self, edge: EdgeId) {
        let Some(synapse_id) = self.edge_id_map.get(&edge).cloned() else { return };
        self.push_graph_edit(DagGraphEdit::Disconnect { synapse_id });
    }

    /// 🔗️ Journals one edit a press performed (a wire, a slider value, a port); an edit already pending is the same edit
    /// and is not journalled twice. A press journals a bounded handful, so these never compete with
    /// [`DAG_GRAPH_EDIT_CAPACITY`]; only moves grow with the graph and are reserved whole ([`Self::journal_moves`]).
    pub(super) fn push_graph_edit(&mut self, edit: DagGraphEdit) {
        if !self.pending_graph_edits.contains(&edit) {
            self.pending_graph_edits.push(edit);
        }
    }

    /// 🔗️ Drains the wire edits this host performed since the last call — the renderer's one read
    /// point after a completed pointer gesture.
    pub fn take_graph_edits(&mut self) -> Vec<DagGraphEdit> {
        std::mem::take(&mut self.pending_graph_edits)
    }

    /// 👁️ The journalled graph edits not yet drained, in gesture order — read without draining so the embedding host can record
    /// the gesture's own concrete edits as history leaves before the renderer drains them ([`Self::take_graph_edits`]).
    pub fn graph_edits(&self) -> &[DagGraphEdit] {
        &self.pending_graph_edits
    }

    /// 🚫️ Drains why the last gesture was refused whole, if it was.
    pub fn take_journal_refusal(&mut self) -> Option<DagJournalRefusal> {
        self.journal_refusal.take()
    }

    /// 🚫️ Keeps a refusal the embedding host carried across rebuilding this host from the gesture's baseline.
    pub fn carry_journal_refusal(&mut self, refusal: DagJournalRefusal) {
        self.journal_refusal = Some(refusal);
    }

    /// ✋️ Journals what a gesture moved — `displacements` of `(node id, dx, dy)` in node order — as node-graph gesture records
    /// of `gesture_id` (design §13.3): one record per distinct offset, the nodes of each in order; a zero offset moved
    /// nothing. All or nothing: when the records do not fit the journal ([`DAG_GRAPH_EDIT_CAPACITY`]) the whole gesture is
    /// refused — the journal is emptied, the refusal kept for the renderer — and the embedding host restores the gesture's
    /// baseline, so neither the document nor the host keeps any of it.
    pub fn journal_moves(&mut self, gesture_id: &str, displacements: impl IntoIterator<Item = (String, f64, f64)>) -> Result<(), DagJournalRefusal> {
        let mut drags: Vec<(f64, f64, Vec<String>)> = Vec::new();
        for (id, dx, dy) in displacements {
            if (dx, dy) == (0.0, 0.0) {
                continue;
            }
            match drags.iter_mut().find(|(known_x, known_y, _)| (*known_x, *known_y) == (dx, dy)) {
                Some((_, _, ids)) => ids.push(id),
                None => drags.push((dx, dy, vec![id])),
            }
        }
        let rows = self.pending_graph_edits.len() + drags.len();
        if rows > DAG_GRAPH_EDIT_CAPACITY {
            let refusal = DagJournalRefusal { rows, limit: DAG_GRAPH_EDIT_CAPACITY };
            self.pending_graph_edits.clear();
            self.journal_refusal = Some(refusal);
            return Err(refusal);
        }
        self.pending_graph_edits.extend(drags.into_iter().map(|(dx, dy, node_ids)| DagGraphEdit::Move { gesture_id: gesture_id.to_string(), node_ids, dx, dy }));
        Ok(())
    }

    /// 📍️ Every node's id and centre, in node order — the baseline an embedding host records at a press so
    /// [`Self::journal_moves_since`] can narrate what the gesture moved.
    pub fn node_positions(&self) -> Vec<(String, f64, f64)> {
        self.host_snapshot.nodes.iter().map(|node| (node.id.clone(), node.x, node.y)).collect()
    }

    /// ✋️ Journals every node that moved since `baseline` ([`Self::node_positions`] at the press) — a drag's release or an
    /// align — through [`Self::journal_moves`]; a node the baseline does not know is not narrated.
    pub fn journal_moves_since(&mut self, gesture_id: &str, baseline: &[(String, f64, f64)]) -> Result<(), DagJournalRefusal> {
        let displacements: Vec<(String, f64, f64)> = self.host_snapshot.nodes.iter().filter_map(|node| baseline.iter().find(|(id, _, _)| *id == node.id).map(|(_, x, y)| (node.id.clone(), node.x - x, node.y - y))).collect();
        self.journal_moves(gesture_id, displacements)
    }

    /// ↩️ Puts every node `baseline` names back where it stood — the zero trace of a gesture refused whole.
    pub fn restore_node_positions(&mut self, baseline: &[(String, f64, f64)]) {
        for (id, x, y) in baseline {
            let Some(index) = self.host_snapshot.nodes.iter().position(|node| node.id == *id) else { continue };
            (self.host_snapshot.nodes[index].x, self.host_snapshot.nodes[index].y) = (*x, *y);
            self.sync_snapshot_node_center_to_engine(index);
        }
    }

    /// 🔌️ Journals a variadic port the embedding host inserted on node `node_id` at `index` of `side`.
    pub fn journal_port_insert(&mut self, node_id: String, side: DagPortSide, index: usize) {
        self.push_graph_edit(DagGraphEdit::InsertPort { node_id, side, index });
    }

    /// 🖱️ Whether a screen-space interaction only `pointer_*_screen` implements is already in flight
    /// — drawing/reconnecting a wire, dragging the minimap viewport, panning, or dragging an inline
    /// widget. While one is, the bounded `derive_pointer_plan` path (whose gesture vocabulary is
    /// `Idle | Pan | Drag | Select`) must not claim the follow-up move/up and cancel it.
    pub fn screen_pointer_gesture_active(&self) -> bool {
        !matches!(self.engine.interaction, InteractionMode::Idle) || self.minimap_widget_drag.is_some() || self.pan_anchor.is_some() || self.widget_drag.is_some() || self.pending_port_insert.is_some()
    }

    /// 🔌️ Whether React Flow's centered connection cursor applies at this point: either the
    /// live hit is a port handle or a wire draw/reconnect gesture owns the pointer beyond its source.
    pub fn connection_cursor_active_at_screen(&self, sx: f64, sy: f64) -> bool {
        matches!(self.engine.interaction, InteractionMode::DrawEdge { .. }) || self.screen_hit(sx, sy).handle
    }

    /// 🖱️ Whether this SCREEN point belongs to the screen pointer path at all: the minimap widget, a
    /// wire handle, a port insertion target, or an inline widget. Exactly the four hits
    /// `bounded_node_hit_index` answers `Unsupported` for, asked as a question instead of as a fault.
    ///
    /// 🖐️ It answers for every PHASE, not only a press: a hover or a release over one of these hits
    /// is just as undescribable to the bounded plan path, and routing only presses left a plain move
    /// across a port faulting the whole dispatch (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn screen_pointer_gesture_begins_at(&self, sx: f64, sy: f64) -> bool {
        self.screen_hit(sx, sy).is_screen_path()
    }

    /// 🩺️ The ONE classification of a screen point both the path discriminator and a renderer
    /// diagnostic read, so a gesture can never be explained by a different hit test than the one that
    /// routed it. `node_id` is the draggable snapshot node under the point; the four booleans are the
    /// hits `bounded_node_hit_index` answers `Unsupported` for.
    pub fn screen_hit(&self, sx: f64, sy: f64) -> DagScreenHit {
        let minimap = self.minimap_widget_pointer_hit(sx, sy);
        let world = self.screen_to_world_point(sx, sy);
        let handle = self.port_pointer_handle_hit(world.x, world.y);
        DagScreenHit {
            node_id: self.snapshot_draggable_node_hit(world.x, world.y).and_then(|node_id| self.widget_id_for_node_id(node_id)),
            channel: handle.and_then(|hid| self.decode_channel_ref(hid)),
            minimap: minimap.is_some(),
            minimap_viewport: minimap.is_some_and(|(_, on_viewport)| on_viewport),
            port_insert: self.port_insert_hit(world.x, world.y, self.host_snapshot.camera.zoom).is_some(),
            handle: handle.is_some(),
            widget: self.widget_hit_at(world.x, world.y).is_some(),
        }
    }

    /// 🩺️ [`Self::screen_hit`] as JSON, for a renderer that has to report why a press went where it
    /// went across a wasm boundary that carries no Rust types.
    pub fn screen_hit_json(&self, sx: f64, sy: f64) -> String {
        let hit = self.screen_hit(sx, sy);
        semio_framework_pack_json::to_json_string(&DagScreenHitJson {
            node: hit.node_id.clone(),
            draggable: hit.is_draggable_body(),
            handle: hit.channel.as_ref().map(|channel| format!("{}@{}", channel.widget_id, channel.port)),
            direction: hit.channel.as_ref().map(|channel| channel.direction.clone()),
            widget: hit.widget,
            minimap: hit.minimap,
            minimap_viewport: hit.minimap_viewport,
            port_insert: hit.port_insert,
        })
    }

    pub(super) fn handle_id_for_port(&self, node_id: &str, port_id: &str) -> Option<HandleId> {
        let key = format!("{node_id}@{port_id}");
        self.handle_key_map.iter().filter(|(_, k)| k.as_str() == key).map(|(&hid, _)| hid).min()
    }

    /// 🔌️ The handle on ONE side of a port. `handle_key_map` keys every handle as `"{nodeId}@{portId}"`,
    /// and a node may carry an input AND an output under the same port id (`extrusion-axis` carries
    /// `vector`, `x`, `y`, `z` on both sides): a name-only lookup then answers whichever handle came
    /// first in the map, so a press on an OUTPUT could resolve to the input handle 40 world units away
    /// on the other side of the node, snap there, and start a wire from the wrong end. The engine
    /// already records the side as [`HandleRole`]; this reads it rather than keeping a second map.
    pub(super) fn handle_id_for_port_side(&self, node_id: &str, port_id: &str, input: bool) -> Option<HandleId> {
        let key = format!("{node_id}@{port_id}");
        let wanted = if input { HandleRole::Target } else { HandleRole::Source };
        self.handle_key_map
            .iter()
            .filter(|(_, candidate)| candidate.as_str() == key)
            .map(|(&hid, _)| hid)
            .find(|hid| self.engine.handles.get(hid).is_some_and(|handle| handle.role == wanted))
            .or_else(|| self.handle_id_for_port(node_id, port_id))
    }

    fn port_row_handle_hit(&self, world_x: f64, world_y: f64, inputs: bool, outputs: bool) -> Option<HandleId> {
        for node in self.host_snapshot.nodes.iter().rev() {
            if inputs {
                for (port_idx, port) in node.inputs().iter().enumerate() {
                    let Some((x0, y0, x1, y1)) = input_port_row_hit_bounds(node, port_idx) else {
                        continue;
                    };
                    if !point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        continue;
                    }
                    if let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, true) {
                        return Some(hid);
                    }
                }
            }
            if outputs {
                for (port_idx, port) in node.outputs().iter().enumerate() {
                    let Some((x0, y0, x1, y1)) = output_port_row_hit_bounds(node, port_idx) else {
                        continue;
                    };
                    if !point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        continue;
                    }
                    if let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, false) {
                        return Some(hid);
                    }
                }
            }
        }
        None
    }

    /// 🔌️ The port whose CONNECTOR rect — the published one — contains this world point.
    fn port_connector_handle_hit(&self, world_x: f64, world_y: f64) -> Option<HandleId> {
        for node in self.host_snapshot.nodes.iter().rev() {
            for (port_idx, port) in node.inputs().iter().enumerate() {
                let Some((x0, y0, x1, y1)) = input_port_connector_bounds(node, port_idx) else { continue };
                if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                    if let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, true) {
                        return Some(hid);
                    }
                }
            }
            for (port_idx, port) in node.outputs().iter().enumerate() {
                let Some((x0, y0, x1, y1)) = output_port_connector_bounds(node, port_idx) else { continue };
                if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                    if let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, false) {
                        return Some(hid);
                    }
                }
            }
        }
        None
    }

    fn channel_row_handle_hit(&self, world_x: f64, world_y: f64) -> Option<HandleId> {
        if !self.draw_lod_for_frame().uses_channel_row_pick() {
            return None;
        }
        self.port_row_handle_hit(world_x, world_y, true, true)
    }

    fn handle_anchor_hit(&self, world_x: f64, world_y: f64) -> Option<HandleId> {
        if !self.draw_lod_for_frame().allows_connection_hit_picking() {
            return None;
        }
        use canvas::Point;
        let p = Point::new(world_x, world_y);
        for (&hid, handle) in self.engine.handles.iter().rev() {
            let Some(node) = self.engine.nodes.get(&handle.node_id) else {
                continue;
            };
            let pos = handle_position(node, handle);
            let dx = p.x - pos.x;
            let dy = p.y - pos.y;
            let tol = handle.radius + 6.0;
            if dx * dx + dy * dy <= tol * tol {
                return Some(hid);
            }
        }
        None
    }

    /// 🔌️ The ONE port hit — the CONNECTOR rect, which is also the one rect
    /// [`Self::entity_screen_json`] publishes for `"handle"`. Grabbing and publishing used to be two
    /// different shapes (a 6.5-world-unit rim disc against the whole 20-unit row), so the published
    /// centre never wired at any zoom, and a radius-11 anchor disc reached past the row centre and
    /// stole the node drag (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️generate-mode-interactions-2026-09-13.md` §4).
    fn port_pointer_handle_hit(&self, world_x: f64, world_y: f64) -> Option<HandleId> {
        if !self.draw_lod_for_frame().allows_connection_hit_picking() {
            return None;
        }
        self.port_connector_handle_hit(world_x, world_y)
    }

    pub(super) fn snapshot_draggable_node_hit(&self, world_x: f64, world_y: f64) -> Option<NodeId> {
        for idx in (0..self.host_snapshot.nodes.len()).rev() {
            let node = &self.host_snapshot.nodes[idx];
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            if world_x < node.x - hw || world_x > node.x + hw || world_y < node.y - hh || world_y > node.y + hh {
                continue;
            }
            let Some(nid) = self.engine_node_id_for_index(idx) else {
                continue;
            };
            if self.engine.nodes.get(&nid).is_some_and(|n| n.draggable) {
                return Some(nid);
            }
        }
        None
    }

    /// 🔗️ Wire snapping belongs to a wire: while a node is being DRAGGED, snapping the pointer onto a
    /// port anchor teleports the node to that anchor. Only a gesture that can still land on a port —
    /// a press, or a wire in flight — is snapped.
    pub(super) fn connection_hit_world_while_wiring(&self, world_x: f64, world_y: f64) -> (f64, f64) {
        if matches!(self.engine.interaction, InteractionMode::DrawEdge { .. }) {
            return self.connection_hit_world(world_x, world_y);
        }
        (world_x, world_y)
    }

    fn connection_hit_world(&self, world_x: f64, world_y: f64) -> (f64, f64) {
        let Some(hid) = self.port_pointer_handle_hit(world_x, world_y) else {
            return (world_x, world_y);
        };
        let Some(handle) = self.engine.handles.get(&hid) else {
            return (world_x, world_y);
        };
        let Some(node) = self.engine.nodes.get(&handle.node_id) else {
            return (world_x, world_y);
        };
        let pos = handle_position(node, handle);
        (pos.x, pos.y)
    }

    pub(super) fn world_hits_handle(&self, world_x: f64, world_y: f64) -> bool {
        self.port_pointer_handle_hit(world_x, world_y).is_some()
    }

    pub(super) fn sync_channel_row_pointer_hover(&mut self, world_x: f64, world_y: f64) {
        if !self.draw_lod_for_frame().uses_channel_row_pick() {
            return;
        }
        if !matches!(self.engine.interaction, InteractionMode::Idle) {
            return;
        }
        if self.handle_anchor_hit(world_x, world_y).is_some() {
            return;
        }
        if let Some(hid) = self.channel_row_handle_hit(world_x, world_y) {
            self.engine.hover = Some(hid);
        }
    }

    /// 🔌️ A press on a port ROW never arrives here any more — `port_pointer_handle_hit` claims it
    /// first, and the engine's own `HitObject::Endpoint` arm already selects that channel before it
    /// begins the wire. The channel-row pick this branch used to duplicate is that selection.
    fn try_node_rectangle_pointer_down(&mut self, world_x: f64, world_y: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool) -> bool {
        if button != 0 || alt {
            return false;
        }
        let Some(node_id) = self.snapshot_draggable_node_hit(world_x, world_y) else {
            return false;
        };
        use canvas::Point;

        let point = Point::new(world_x, world_y);
        self.engine.pointer_down_on_draggable_node_at(node_id, point, shift, ctrl_or_meta);
        true
    }

    pub(super) fn widget_hit_at(&self, world_x: f64, world_y: f64) -> Option<(usize, WidgetPointerKind)> {
        for idx in (0..self.host_snapshot.nodes.len()).rev() {
            let node = &self.host_snapshot.nodes[idx];
            match &node.kind {
                DagNodeKind::Slider { .. } => {
                    let (x0, y0, x1, y1) = slider_track_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, WidgetPointerKind::SliderDrag));
                    }
                }
                DagNodeKind::Select { .. } => {
                    let (x0, y0, x1, y1) = select_control_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, WidgetPointerKind::SelectClick));
                    }
                }
                DagNodeKind::Preview { content: DagPreviewContent::Tree { json }, expanded, .. } => {
                    for row in preview_tree_row_layouts(node, json, expanded) {
                        let (x0, y0, x1, y1) = row.row_rect;
                        if x1 > x0 && point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                            return Some((idx, WidgetPointerKind::PreviewToggle(row.path)));
                        }
                    }
                }
                DagNodeKind::Cluster { .. } => {
                    if cluster_explode_hit(node, world_x, world_y) {
                        return Some((idx, WidgetPointerKind::ClusterExplode));
                    }
                }
                DagNodeKind::Export { .. } => {
                    let (x0, y0, x1, y1) = action_control_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, WidgetPointerKind::ExportClick));
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn sync_snapshot_node_size_to_engine(&mut self, idx: usize) {
        let node = &self.host_snapshot.nodes[idx];
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return;
        };
        let Some(engine_node) = self.engine.nodes.get_mut(&nid) else {
            return;
        };
        engine_node.width = node.width;
        engine_node.height = node.height;
    }

    fn toggle_preview_tree_path(node: &mut DagNodeSpec, path: &str) {
        let DagNodeKind::Preview { content, expanded, .. } = &mut node.kind else {
            return;
        };
        if !matches!(content, DagPreviewContent::Tree { .. }) {
            return;
        }
        if expanded.contains(path) {
            expanded.remove(path);
        } else {
            expanded.insert(path.to_string());
        }
        fit_node_size(node);
    }

    /// ✏️ Begins inline note text editing at a world-space click position.
    pub fn begin_note_edit(&mut self, node_id: &str, world_x: f64, _world_y: f64) -> bool {
        let Some(node) = self.host_snapshot.nodes.iter().find(|n| n.id == node_id) else {
            return false;
        };
        let DagNodeKind::Note { text, .. } = &node.kind else {
            return false;
        };
        let lod_index = dag_lod_index(self.host_snapshot.camera.zoom);
        let font_px = dag_label_paint_px(self.host_snapshot.camera.zoom, lod_index) * 1.05;
        let origin_x = note_text_origin_x(node);
        let offset = hit_byte_in_note_line(text, world_x, origin_x, font_px);
        self.editing_note = Some(NoteEditState { node_id: node_id.to_string(), caret: offset, anchor: offset });
        self.caret_visible = true;
        true
    }

    /// ✏️ Inserts text at the active note caret, replacing any selection.
    pub fn note_insert_text(&mut self, chunk: &str) -> bool {
        let Some(node_id) = self.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
            return false;
        };
        let Some(idx) = self.host_snapshot.nodes.iter().position(|n| n.id == node_id) else {
            return false;
        };
        let DagNodeKind::Note { text, .. } = &mut self.host_snapshot.nodes[idx].kind else {
            return false;
        };
        let Some(edit) = self.editing_note.as_mut() else {
            return false;
        };
        let start = edit.caret.min(edit.anchor);
        let end = edit.caret.max(edit.anchor);
        text.replace_range(start..end, chunk);
        let pos = start + chunk.len();
        edit.caret = pos;
        edit.anchor = pos;
        true
    }

    /// ✏️ Deletes the selection or the character before the note caret.
    pub fn note_backspace(&mut self) -> bool {
        let Some(node_id) = self.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
            return false;
        };
        let Some(idx) = self.host_snapshot.nodes.iter().position(|n| n.id == node_id) else {
            return false;
        };
        let DagNodeKind::Note { text, .. } = &mut self.host_snapshot.nodes[idx].kind else {
            return false;
        };
        let Some(edit) = self.editing_note.as_mut() else {
            return false;
        };
        let start = edit.caret.min(edit.anchor);
        let end = edit.caret.max(edit.anchor);
        if start < end {
            text.replace_range(start..end, "");
            edit.caret = start;
            edit.anchor = start;
            return true;
        }
        if start == 0 {
            return false;
        }
        let prev = text[..start].char_indices().last().map_or(0, |(i, _)| i);
        text.replace_range(prev..start, "");
        edit.caret = prev;
        edit.anchor = prev;
        true
    }

    /// ✏️ Deletes the selection or the character after the note caret.
    pub fn note_delete_forward(&mut self) -> bool {
        let Some(node_id) = self.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
            return false;
        };
        let Some(idx) = self.host_snapshot.nodes.iter().position(|n| n.id == node_id) else {
            return false;
        };
        let DagNodeKind::Note { text, .. } = &mut self.host_snapshot.nodes[idx].kind else {
            return false;
        };
        let Some(edit) = self.editing_note.as_mut() else {
            return false;
        };
        let start = edit.caret.min(edit.anchor);
        let end = edit.caret.max(edit.anchor);
        if start < end {
            text.replace_range(start..end, "");
            edit.caret = start;
            edit.anchor = start;
            return true;
        }
        if start >= text.len() {
            return false;
        }
        let next = text[start..].char_indices().nth(1).map_or(text.len(), |(i, _)| start + i);
        text.replace_range(start..next, "");
        edit.caret = start;
        edit.anchor = start;
        true
    }

    /// ✏️ Moves the note caret (`left` | `right` | `home` | `end`).
    pub fn note_move_caret(&mut self, direction: &str, extend: bool) -> bool {
        let Some(node_id) = self.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
            return false;
        };
        let Some(idx) = self.host_snapshot.nodes.iter().position(|n| n.id == node_id) else {
            return false;
        };
        let DagNodeKind::Note { text, .. } = &self.host_snapshot.nodes[idx].kind else {
            return false;
        };
        let Some(edit) = self.editing_note.as_mut() else {
            return false;
        };
        let pos = match direction {
            "left" => {
                if edit.caret == 0 {
                    0
                } else {
                    text[..edit.caret].char_indices().last().map_or(0, |(i, _)| i)
                }
            }
            "right" => text[edit.caret..].char_indices().nth(1).map_or(text.len(), |(i, _)| edit.caret + i),
            "home" => 0,
            "end" => text.len(),
            _ => return false,
        };
        if extend {
            edit.caret = pos;
        } else {
            edit.caret = pos;
            edit.anchor = pos;
        }
        true
    }

    /// ✏️ Ends inline note editing.
    pub fn note_commit_edit(&mut self) {
        self.editing_note = None;
    }

    /// ✏️ Toggles native caret visibility for the active note editor.
    pub fn set_note_caret_visible(&mut self, visible: bool) {
        self.caret_visible = visible;
    }

    /// ✏️ Returns the widget id currently being edited inline, if any.
    pub fn editing_note_id(&self) -> Option<&str> {
        self.editing_note.as_ref().map(|edit| edit.node_id.as_str())
    }

    /// 📐️ Recomputes preview and image node sizes after content changes.
    pub fn fit_preview_sizes(&mut self) {
        for idx in 0..self.host_snapshot.nodes.len() {
            let kind = &self.host_snapshot.nodes[idx].kind;
            if matches!(kind, DagNodeKind::Preview { .. } | DagNodeKind::Image { .. }) {
                fit_node_size(&mut self.host_snapshot.nodes[idx]);
                self.sync_snapshot_node_size_to_engine(idx);
            }
        }
    }

    /// 📐️ Recomputes note node sizes after text changes.
    pub fn fit_note_sizes(&mut self) {
        for idx in 0..self.host_snapshot.nodes.len() {
            if matches!(self.host_snapshot.nodes[idx].kind, DagNodeKind::Note { .. }) {
                fit_node_size(&mut self.host_snapshot.nodes[idx]);
                self.sync_snapshot_node_size_to_engine(idx);
            }
        }
    }

    fn try_widget_pointer_down(&mut self, world_x: f64, world_y: f64) -> bool {
        if !self.draw_lod_for_frame().shows_controls() {
            return false;
        }
        let Some((idx, kind)) = self.widget_hit_at(world_x, world_y) else {
            return false;
        };
        match kind {
            WidgetPointerKind::SliderDrag => {
                self.widget_drag = Some(idx);
                set_slider_value_from_x(&mut self.host_snapshot.nodes[idx], world_x);
            }
            WidgetPointerKind::SelectClick => {
                advance_select_option(&mut self.host_snapshot.nodes[idx]);
            }
            WidgetPointerKind::PreviewToggle(path) => {
                Self::toggle_preview_tree_path(&mut self.host_snapshot.nodes[idx], &path);
                self.sync_snapshot_node_size_to_engine(idx);
            }
            WidgetPointerKind::ClusterExplode => {
                self.pending_cluster_explode = Some(self.host_snapshot.nodes[idx].id.clone());
            }
            WidgetPointerKind::ExportClick => {
                self.pending_export_click = Some(self.host_snapshot.nodes[idx].id.clone());
            }
        }
        true
    }

    /// 💥️ Takes a pending cluster explode request from the last widget hit.
    pub fn take_pending_cluster_explode(&mut self) -> Option<String> {
        self.pending_cluster_explode.take()
    }

    /// 📤️ Takes a pending export control click from the last widget hit.
    pub fn take_pending_export_click(&mut self) -> Option<String> {
        self.pending_export_click.take()
    }

    pub(super) fn sync_connection_hit_picking_for_lod(&mut self) {
        let allow = self.draw_lod_for_frame().allows_connection_hit_picking();
        self.engine.handle_pointer_picking = allow;
        if !allow && matches!(self.engine.interaction, InteractionMode::DrawEdge { .. }) {
            self.engine.interaction = InteractionMode::Idle;
        }
    }

    /// 🧭️ Minimap LOD: pointer-down inside the selection AABB moves the group without a discrete hit.
    fn lod_uses_bounded_drag(&self) -> bool {
        matches!(self.draw_lod_for_frame(), DagDrawLod::Minimap)
    }

    pub fn pointer_down(&mut self, x: f64, y: f64, extend: bool) {
        self.pointer_down_screen(x, y, 0, extend, false, false, false);
    }

    #[allow(clippy::too_many_arguments, reason = "flat args mirror the shared WASM host-bridge pointer-event contract used across all `pointer_*_screen` methods in this repo (see infinite/board/rs, framework/editor/rs, layout/rs, etc.)")]
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        if button == 0 && !shift && !ctrl_or_meta && !alt && !pan {
            if let Some((layout, on_viewport)) = self.minimap_widget_pointer_hit(sx, sy) {
                let (wx, wy) = self.minimap_widget_screen_to_world(&layout, sx, sy);
                let zoom = self.host_snapshot.camera.zoom;
                if on_viewport {
                    let cam = &self.host_snapshot.camera;
                    self.minimap_widget_drag = Some((cam.x - wx, cam.y - wy));
                } else {
                    self.set_camera(wx, wy, zoom);
                    self.minimap_widget_drag = Some((0.0, 0.0));
                }
                return;
            }
        }
        if pan {
            self.pan_anchor = Some((sx, sy, self.host_snapshot.camera.x, self.host_snapshot.camera.y));
            return;
        }
        self.sync_connection_hit_picking_for_lod();
        self.last_screen_x = sx;
        self.last_screen_y = sy;
        let world = self.screen_to_world_point(sx, sy);
        if button == 0 && !shift && !ctrl_or_meta && !alt {
            if let Some(node_id) = self.snapshot_draggable_node_hit(world.x, world.y) {
                if !self.world_hits_handle(world.x, world.y) {
                    if let Some(widget_id) = self.widget_id_for_node_id(node_id) {
                        if let Some(node) = self.host_snapshot.nodes.iter().find(|entry| entry.id == widget_id) {
                            if let DagNodeKind::AppInstance { instance_id, .. } = &node.kind {
                                let now = pointer_event_now_ms();
                                let dist = ((world.x - self.last_pointer_down_world.0).powi(2) + (world.y - self.last_pointer_down_world.1).powi(2)).sqrt();
                                let zoom = self.host_snapshot.camera.zoom.max(1e-9);
                                if self.last_pointer_down_node_id.as_deref() == Some(widget_id.as_str()) && now - self.last_pointer_down_at_ms < 350.0 && dist < 8.0 / zoom {
                                    self.pending_open_instance_id = Some(instance_id.clone());
                                    return;
                                }
                                self.last_pointer_down_at_ms = now;
                                self.last_pointer_down_world = (world.x, world.y);
                                self.last_pointer_down_node_id = Some(widget_id);
                            }
                        }
                    }
                }
            }
        }
        if let Some(hit) = self.port_insert_hit(world.x, world.y, self.host_snapshot.camera.zoom) {
            self.pending_port_insert = Some(hit);
            return;
        }
        let (hit_x, hit_y) = self.connection_hit_world(world.x, world.y);
        if self.world_hits_handle(hit_x, hit_y) {
            self.engine.pointer_down_screen(sx, sy, hit_x, hit_y, button, shift, ctrl_or_meta, alt);
            self.process_engine_events();
            self.sync_camera_from_engine();
            return;
        }
        if self.try_widget_pointer_down(world.x, world.y) {
            return;
        }
        if self.try_node_rectangle_pointer_down(world.x, world.y, button, shift, ctrl_or_meta, alt) {
            self.process_engine_events();
            self.sync_camera_from_engine();
            return;
        }
        let merge_from_modifiers = ctrl_or_meta || shift;
        if button == 0 && !merge_from_modifiers && self.lod_uses_bounded_drag() {
            let pad = DAG_BOUNDED_DRAG_HIT_PAD_PX / self.host_snapshot.camera.zoom.max(1e-9);
            if self.engine.try_begin_selection_union_drag_at(world, pad) {
                self.process_engine_events();
                self.sync_camera_from_engine();
                return;
            }
        }
        self.engine.pointer_down_screen(sx, sy, hit_x, hit_y, button, shift, ctrl_or_meta, alt);
        self.process_engine_events();
        self.sync_camera_from_engine();
    }

    pub fn widget_drag_active(&self) -> bool {
        self.widget_drag.is_some()
    }
}
// #endregion 🔖️DagHost
