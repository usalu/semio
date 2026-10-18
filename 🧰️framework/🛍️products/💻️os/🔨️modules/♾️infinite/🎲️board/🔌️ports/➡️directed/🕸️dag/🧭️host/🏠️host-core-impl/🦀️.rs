//! 🏠️ Host core impl of the DAG host: the large host method block with construction, loading and graph access. See [`super`].

use super::*;

// #region 🔖️DagHost
impl DagHost {
    pub fn pointer_move(&mut self, x: f64, y: f64) {
        self.pointer_move_screen(x, y, false, false, false);
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        if let Some((ox, oy)) = self.minimap.minimap_widget_drag {
            if let Some(layout) = minimap_widget_layout(&self.graph, &self.minimap, self.viewport.width, self.viewport.height) {
                let (wx, wy) = minimap_widget_screen_to_world(&layout, sx, sy);
                let zoom = self.graph.host_snapshot.camera.zoom;
                self.graph.set_camera(wx + ox, wy + oy, zoom);
                self.viewport.last_screen_x = sx;
                self.viewport.last_screen_y = sy;
                return;
            }
        }
        if let Some((start_sx, start_sy, cam_x, cam_y)) = self.viewport.pan_anchor {
            let zoom = self.graph.host_snapshot.camera.zoom.max(1e-9);
            let dx = (sx - start_sx) / zoom;
            let dy = (sy - start_sy) / zoom;
            self.graph.set_camera(cam_x - dx, cam_y - dy, zoom);
            self.viewport.last_screen_x = sx;
            self.viewport.last_screen_y = sy;
            return;
        }
        let minimap_hovered = minimap_widget_pointer_hit(&self.viewport, &self.graph, &self.minimap, sx, sy).is_some();
        self.minimap.minimap_widget_hovered = minimap_hovered;
        if minimap_hovered {
            self.viewport.last_screen_x = sx;
            self.viewport.last_screen_y = sy;
            return;
        }
        sync_connection_hit_picking_for_lod(&mut self.viewport, &mut self.graph);
        self.viewport.last_screen_x = sx;
        self.viewport.last_screen_y = sy;
        let world = screen_to_world_point(&self.viewport, &self.graph, sx, sy);
        if let Some(idx) = self.pointer.widget_drag {
            set_slider_value_from_x(&mut self.graph.host_snapshot.nodes[idx], world.x);
            return;
        }
        let (hit_x, hit_y) = connection_hit_world_while_wiring(&self.viewport, &self.graph, world.x, world.y);
        self.graph.engine.pointer_move_screen(sx, sy, hit_x, hit_y, shift, ctrl_or_meta, alt);
        sync_channel_row_pointer_hover(&mut self.viewport, &mut self.graph, world.x, world.y);
        sync_minimap_pointer_hover(&mut self.viewport, &mut self.graph, world.x, world.y);
        if matches!(self.graph.engine.interaction, InteractionMode::DragNode { .. } | InteractionMode::DragNodes { .. }) {
            sync_node_positions_from_engine(&mut self.viewport, &mut self.graph);
        }
        process_engine_events(&mut self.viewport, &mut self.graph);
        self.graph.sync_camera_from_engine();
    }

    pub fn pointer_up(&mut self, x: f64, y: f64) {
        self.pointer_up_screen(x, y, false, false, false);
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.viewport.pan_anchor = None;
        self.minimap.minimap_widget_drag = None;
        sync_connection_hit_picking_for_lod(&mut self.viewport, &mut self.graph);
        self.viewport.last_screen_x = sx;
        self.viewport.last_screen_y = sy;
        if let Some(index) = self.pointer.widget_drag.take() {
            if let Some(DagNodeSpec { id, kind: DagNodeKind::Slider { value, .. }, .. }) = self.graph.host_snapshot.nodes.get(index) {
                let edit = DagGraphEdit::SetSlider { node_id: id.clone(), value: *value };
                self.graph.push_graph_edit(edit);
            }
            return;
        }
        let world = screen_to_world_point(&self.viewport, &self.graph, sx, sy);
        let (hit_x, hit_y) = connection_hit_world_while_wiring(&self.viewport, &self.graph, world.x, world.y);
        self.graph.engine.pointer_up_screen(sx, sy, hit_x, hit_y, shift, ctrl_or_meta, alt);
        process_engine_events(&mut self.viewport, &mut self.graph);
        sync_node_positions_from_engine(&mut self.viewport, &mut self.graph);
        self.graph.sync_camera_from_engine();
    }

    pub fn pointer_cancel_screen(&mut self) -> bool {
        self.viewport.pan_anchor = None;
        self.minimap.minimap_widget_drag = None;
        self.pointer.widget_drag = None;
        self.pointer.pending_port_insert = None;
        let cancelled = self.graph.engine.pointer_cancel_screen();
        self.graph.pending_graph_edits.clear();
        sync_node_positions_from_engine(&mut self.viewport, &mut self.graph);
        self.graph.sync_camera_from_engine();
        cancelled
    }

    /// 🏷️ Camera, draw LOD, and node label anchors for the JS canvas text overlay (must match the last GPU frame).
    pub fn label_overlay_paint_state_json(&self) -> Result<String, DagError> {
        let lod = draw_lod_for_frame(&self.viewport, &self.graph);
        let cam = &self.graph.host_snapshot.camera;
        let lod_index = dag_lod_index(cam.zoom);
        let mut labels = Vec::new();
        for (idx, snapshot_node) in self.graph.host_snapshot.nodes.iter().enumerate() {
            let node = self.graph.node_spec_for_paint(idx, snapshot_node);
            let engine_nid = self.graph.engine_node_id_for_index(idx);
            labels.extend(label_overlay_rows_for_node(node.as_ref(), lod, cam.zoom, lod_index, false, engine_nid, &self.paint.unresolved_input_ports));
        }
        if let Some(ghost) = self.pointer.ghost_node.as_ref() {
            labels.extend(label_overlay_rows_for_node(ghost, lod, cam.zoom, lod_index, true, None, &self.paint.unresolved_input_ports));
        }
        let minimap_widget = minimap_widget_json(&self.viewport, &self.graph, &self.minimap);
        Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("camera".to_string(), semio_framework_pack_json::object([("x".to_string(), Value::from(cam.x)), ("y".to_string(), Value::from(cam.y)), ("zoom".to_string(), Value::from(cam.zoom))])),
            ("lod".to_string(), Value::from(lod.label())),
            ("width".to_string(), Value::from(self.viewport.width)),
            ("height".to_string(), Value::from(self.viewport.height)),
            ("labels".to_string(), Value::Array(labels)),
            ("minimapWidget".to_string(), Value::from(minimap_widget)),
        ])))
    }

    pub fn paint_scene(&self, scene: &mut canvas::Scene, viewport_w: u32, viewport_h: u32, dpr: f64) {
        use canvas::camera::{camera_content_affine, Camera as CanvasCamera, Viewport};
        use canvas::FillRule;
        use canvas::{Circle, Rect, Stroke};

        let theme = &self.viewport.canvas_theme;
        self.paint.tick_computing_animation();
        let cam = CanvasCamera { x: self.graph.host_snapshot.camera.x, y: self.graph.host_snapshot.camera.y, zoom: self.graph.host_snapshot.camera.zoom };
        let viewport = Viewport { width: viewport_w.max(1), height: viewport_h.max(1), dpr: dpr.max(1.0) };
        let aff = camera_content_affine(&cam, &viewport);
        let lod = draw_lod_for_frame(&self.viewport, &self.graph);
        let lod_index = dag_lod_index(cam.zoom);
        self.viewport.paint_lod_grid(scene, &cam, &viewport, lod);
        let snap = self.graph.engine.render_snapshot();
        let edge_stroke = dag_world_stroke(lod.edge_stroke_screen_px(), cam.zoom);
        for &eid in self.graph.engine.edges.keys() {
            let (is_selected, is_highlighted, is_hovered) = self.graph.edge_interaction_chrome(eid);
            let stroke_c = dag_edge_body_stroke(theme, false, is_selected, is_highlighted, is_hovered);
            let style = self.graph.edge_route_style.get(&eid).copied().unwrap_or_default();
            match style {
                EdgeRouteStyle::Bezier => {
                    if let Some(curve) = self.graph.engine.edge_curve(eid) {
                        scene.stroke(&Stroke::new(edge_stroke), aff, stroke_c, None, &curve);
                    }
                }
                EdgeRouteStyle::SharpSz => {
                    if let Some(path) = self.graph.edge_sharp_path(eid) {
                        scene.stroke(&Stroke::new(edge_stroke), aff, stroke_c, None, &path);
                    }
                }
            }
        }
        if let Some(preview) = snap.pending_edge {
            scene.stroke(&Stroke::new(edge_stroke), aff, dag_edge_body_stroke(theme, false, true, false, false), None, &preview);
        }
        let handle_stroke_px = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
        let paint_snap_handle = |scene: &mut canvas::Scene, hid: &HandleId, center: &canvas::Point, shape_filter: Option<PortShape>| {
            if self.graph.handle_port_visible.get(hid) == Some(&false) {
                return;
            }
            let shape = self.graph.handle_port_shape.get(hid).copied().unwrap_or_default();
            if let Some(filter) = shape_filter {
                if shape != filter {
                    return;
                }
            }
            if !lod.shows_handles() && shape != PortShape::Triangle {
                return;
            }
            let node_id = self.graph.engine.handles.get(hid).map(|handle| handle.node_id);
            let is_dimmed = node_id.is_some_and(|nid| self.paint.dimmed.contains(&nid));
            let (is_selected, is_highlighted, is_hovered) = self.graph.handle_interaction_chrome(*hid);
            let fill = dag_handle_body_fill(theme, is_dimmed, is_selected, is_highlighted, is_hovered);
            let stroke_c = dag_handle_body_stroke(theme, is_dimmed, is_selected, is_highlighted, is_hovered);
            let chrome = is_dimmed || is_selected || is_highlighted || is_hovered;
            let outward = node_id.and_then(|nid| self.graph.engine.nodes.get(&nid).and_then(|node| handle_outward_at_node_rim(*center, node.center, node.shape, node.radius, node.width, node.height)));
            if let Some(out) = outward {
                if chrome {
                    scene.fill(FillRule::NonZero, aff, fill, None, &handle_cap_fill_path(*center, out, DAG_HANDLE_WORLD_RADIUS, shape));
                }
                scene.stroke(&Stroke::new(handle_stroke_px), aff, stroke_c, None, &handle_cap_stroke_path(*center, out, DAG_HANDLE_WORLD_RADIUS, shape));
            } else {
                let circle = Circle::new(*center, DAG_HANDLE_WORLD_RADIUS);
                if chrome {
                    scene.fill(FillRule::NonZero, aff, fill, None, &circle);
                }
                scene.stroke(&Stroke::new(handle_stroke_px), aff, stroke_c, None, &circle);
            }
        };
        for (hid, center, _radius) in &snap.handles {
            paint_snap_handle(scene, hid, center, Some(PortShape::Semicircle));
        }
        let paint_minimap_node = |scene: &mut canvas::Scene, idx: usize, snapshot_node: &DagNodeSpec| {
            let node = self.graph.node_spec_for_paint(idx, snapshot_node);
            let node = node.as_ref();
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            let rect = Rect::new(node.x - hw, node.y - hh, node.x + hw, node.y + hh);
            let engine_nid = self.graph.engine_node_id_for_index(idx);
            let is_dimmed = engine_nid.is_some_and(|nid| self.paint.dimmed.contains(&nid));
            let (is_selected, is_highlighted, is_hovered) = engine_nid.map_or((false, false, false), |nid| self.graph.node_interaction_chrome(nid));
            if let Some(fill) = dag_node_paint_fill(lod, theme, is_dimmed, is_selected, is_highlighted, is_hovered) {
                scene.fill(FillRule::NonZero, aff, fill, None, &rect);
            }
        };
        if lod == DagDrawLod::Minimap {
            for (idx, snapshot_node) in self.graph.host_snapshot.nodes.iter().enumerate() {
                let engine_nid = self.graph.engine_node_id_for_index(idx);
                let chrome = engine_nid.is_some_and(|nid| {
                    let (selected, highlighted, hovered) = self.graph.node_interaction_chrome(nid);
                    selected || highlighted || hovered
                });
                if !chrome {
                    paint_minimap_node(scene, idx, snapshot_node);
                }
            }
            for (idx, snapshot_node) in self.graph.host_snapshot.nodes.iter().enumerate() {
                let engine_nid = self.graph.engine_node_id_for_index(idx);
                let chrome = engine_nid.is_some_and(|nid| {
                    let (selected, highlighted, hovered) = self.graph.node_interaction_chrome(nid);
                    selected || highlighted || hovered
                });
                if chrome {
                    paint_minimap_node(scene, idx, snapshot_node);
                }
            }
        } else {
            for (idx, snapshot_node) in self.graph.host_snapshot.nodes.iter().enumerate() {
                let node = self.graph.node_spec_for_paint(idx, snapshot_node);
                let node = node.as_ref();
                let engine_nid = self.graph.engine_node_id_for_index(idx);
                let is_dimmed = engine_nid.is_some_and(|nid| self.paint.dimmed.contains(&nid));
                let (is_selected, is_highlighted, is_hovered) = engine_nid.map_or((false, false, false), |nid| self.graph.node_interaction_chrome(nid));
                let eval_status = engine_nid.and_then(|nid| self.paint.node_eval_status.get(&nid).copied()).unwrap_or(DagNodeEvalStatusKind::Ok);
                paint_node_visual(&self.viewport, &self.graph, &self.paint, &self.notes, scene, &aff, &cam, &viewport, lod, lod_index, node, DagNodePaintChrome { is_dimmed, is_selected, is_highlighted, is_hovered, eval_status, body_fill_alpha: 255, ghost_tint: false });
            }
        }
        if let Some(ghost) = self.pointer.ghost_node.as_ref() {
            paint_node_visual(&self.viewport, &self.graph, &self.paint, &self.notes, scene, &aff, &cam, &viewport, lod, lod_index, ghost, DagNodePaintChrome::ghost_preview());
        }
        for (hid, center, _radius) in &snap.handles {
            paint_snap_handle(scene, hid, center, Some(PortShape::Triangle));
        }
        if let Some(ghost) = self.pointer.ghost_node.as_ref() {
            let paint_ghost_handles = lod.shows_handles() || ghost.inputs().iter().any(|port| port.shape == PortShape::Triangle) || ghost.outputs().iter().any(|port| port.shape == PortShape::Triangle);
            if paint_ghost_handles {
                self.viewport.paint_node_handles_for_spec(scene, &aff, &cam, ghost, &DagNodePaintChrome::ghost_preview());
            }
        }
        paint_minimap_widget(&self.viewport, &self.graph, &self.paint, &self.minimap, scene, viewport_w, viewport_h);
    }
}
