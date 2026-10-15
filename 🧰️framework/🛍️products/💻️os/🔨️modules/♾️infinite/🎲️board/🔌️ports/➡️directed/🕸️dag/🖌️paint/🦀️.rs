//! 🖌️ DAG host pointer move and up, overlays, node and edge painting and scene paint.

use super::*;

// #region 🔖️DagHost
impl DagHost {
    pub fn pointer_move(&mut self, x: f64, y: f64) {
        self.pointer_move_screen(x, y, false, false, false);
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        if let Some((ox, oy)) = self.minimap_widget_drag {
            if let Some(layout) = self.minimap_widget_layout(self.width, self.height) {
                let (wx, wy) = self.minimap_widget_screen_to_world(&layout, sx, sy);
                let zoom = self.host_snapshot.camera.zoom;
                self.set_camera(wx + ox, wy + oy, zoom);
                self.last_screen_x = sx;
                self.last_screen_y = sy;
                return;
            }
        }
        if let Some((start_sx, start_sy, cam_x, cam_y)) = self.pan_anchor {
            let zoom = self.host_snapshot.camera.zoom.max(1e-9);
            let dx = (sx - start_sx) / zoom;
            let dy = (sy - start_sy) / zoom;
            self.set_camera(cam_x - dx, cam_y - dy, zoom);
            self.last_screen_x = sx;
            self.last_screen_y = sy;
            return;
        }
        let minimap_hovered = self.minimap_widget_pointer_hit(sx, sy).is_some();
        self.minimap_widget_hovered = minimap_hovered;
        if minimap_hovered {
            self.last_screen_x = sx;
            self.last_screen_y = sy;
            return;
        }
        self.sync_connection_hit_picking_for_lod();
        self.last_screen_x = sx;
        self.last_screen_y = sy;
        let world = self.screen_to_world_point(sx, sy);
        if let Some(idx) = self.widget_drag {
            set_slider_value_from_x(&mut self.host_snapshot.nodes[idx], world.x);
            return;
        }
        let (hit_x, hit_y) = self.connection_hit_world_while_wiring(world.x, world.y);
        self.engine.pointer_move_screen(sx, sy, hit_x, hit_y, shift, ctrl_or_meta, alt);
        self.sync_channel_row_pointer_hover(world.x, world.y);
        self.sync_minimap_pointer_hover(world.x, world.y);
        if matches!(self.engine.interaction, InteractionMode::DragNode { .. } | InteractionMode::DragNodes { .. }) {
            self.sync_node_positions_from_engine();
        }
        self.process_engine_events();
        self.sync_camera_from_engine();
    }

    pub fn pointer_up(&mut self, x: f64, y: f64) {
        self.pointer_up_screen(x, y, false, false, false);
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.pan_anchor = None;
        self.minimap_widget_drag = None;
        self.sync_connection_hit_picking_for_lod();
        self.last_screen_x = sx;
        self.last_screen_y = sy;
        if let Some(index) = self.widget_drag.take() {
            if let Some(DagNodeSpec { id, kind: DagNodeKind::Slider { value, .. }, .. }) = self.host_snapshot.nodes.get(index) {
                let edit = DagGraphEdit::SetSlider { node_id: id.clone(), value: *value };
                self.push_graph_edit(edit);
            }
            return;
        }
        let world = self.screen_to_world_point(sx, sy);
        let (hit_x, hit_y) = self.connection_hit_world_while_wiring(world.x, world.y);
        self.engine.pointer_up_screen(sx, sy, hit_x, hit_y, shift, ctrl_or_meta, alt);
        self.process_engine_events();
        self.sync_node_positions_from_engine();
        self.sync_camera_from_engine();
    }

    pub fn pointer_cancel_screen(&mut self) -> bool {
        self.pan_anchor = None;
        self.minimap_widget_drag = None;
        self.widget_drag = None;
        self.pending_port_insert = None;
        let cancelled = self.engine.pointer_cancel_screen();
        self.pending_graph_edits.clear();
        self.sync_node_positions_from_engine();
        self.sync_camera_from_engine();
        cancelled
    }

    pub fn set_canvas_palette(&mut self, overlay:&infinite::board::BoardPaletteOverlay) {
        self.canvas_theme.apply_overlay(overlay);
        self.icon_paint_cache.clear();
    }

    /// 🖼️ Screen-node overlay rects in CSS pixel space for DOM media layers.
    pub fn node_overlays_json(&self) -> Result<String, DagError> {
        use canvas::camera::{world_to_screen, Camera as CanvasCamera, Viewport};
        use canvas::Point;
        let cam = CanvasCamera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom };
        let viewport = Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) };
        let mut overlays = Vec::new();
        for node in &self.host_snapshot.nodes {
            let DagNodeKind::Screen { media: Some(media), .. } = &node.kind else {
                continue;
            };
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            let inset = 8.0 / cam.zoom.max(0.05);
            let top = node.y - hh + hh * 0.35;
            let bottom = node.y + hh - inset;
            let left = node.x - hw + inset;
            let right = node.x + hw - inset;
            let tl = world_to_screen(&cam, &viewport, Point::new(left, top));
            let br = world_to_screen(&cam, &viewport, Point::new(right, bottom));
            let media_kind = match media.kind {
                DagMediaKind::Image => "image",
                DagMediaKind::Svg => "svg",
                DagMediaKind::Pdf => "pdf",
                DagMediaKind::Video => "video",
            };
            overlays.push(semio_framework_pack_json::object([
                ("id".to_string(), Value::from(node.id.clone())),
                ("mediaKind".to_string(), Value::from(media_kind)),
                ("src".to_string(), Value::from(media.src.clone())),
                (
                    "rect".to_string(),
                    semio_framework_pack_json::object([("x".to_string(), Value::from(tl.x)), ("y".to_string(), Value::from(tl.y)), ("w".to_string(), Value::from((br.x - tl.x).max(1.0))), ("h".to_string(), Value::from((br.y - tl.y).max(1.0)))]),
                ),
            ]));
        }
        Ok(semio_framework_pack_json::to_string(&Value::Array(overlays)))
    }

    fn handle_cap_peak(&self, center: canvas::Point, outward: canvas::Vec2, radius: f64, shape: PortShape) -> canvas::Point {
        match shape {
            PortShape::Semicircle => handle_exterior_cap_peak(center, outward, radius),
            PortShape::Triangle => handle_exterior_cap_triangle_peak(center, outward, radius),
        }
    }

    fn handle_cap_fill_path(&self, center: canvas::Point, outward: canvas::Vec2, radius: f64, shape: PortShape) -> canvas::BezPath {
        match shape {
            PortShape::Semicircle => handle_exterior_cap_fill_path(center, outward, radius),
            PortShape::Triangle => handle_exterior_cap_triangle_fill_path(center, outward, radius),
        }
    }

    fn handle_cap_stroke_path(&self, center: canvas::Point, outward: canvas::Vec2, radius: f64, shape: PortShape) -> canvas::BezPath {
        match shape {
            PortShape::Semicircle => handle_exterior_cap_stroke_path(center, outward, radius),
            PortShape::Triangle => handle_exterior_cap_triangle_stroke_path(center, outward, radius),
        }
    }

    fn edge_sharp_path(&self, eid: EdgeId) -> Option<canvas::BezPath> {
        let edge = self.engine.edges.get(&eid)?;
        let source_handle = self.engine.handles.get(&edge.source)?;
        let target_handle = self.engine.handles.get(&edge.target)?;
        let source_node = self.engine.nodes.get(&source_handle.node_id)?;
        let target_node = self.engine.nodes.get(&target_handle.node_id)?;
        let source_position = handle_position(source_node, source_handle);
        let target_position = handle_position(target_node, target_handle);
        let source_out = handle_outward_at_node_rim(source_position, source_node.center, source_node.shape, source_node.radius, source_node.width, source_node.height)?;
        let target_out = handle_outward_at_node_rim(target_position, target_node.center, target_node.shape, target_node.radius, target_node.width, target_node.height)?;
        let source_shape = self.handle_port_shape.get(&edge.source).copied().unwrap_or_default();
        let target_shape = self.handle_port_shape.get(&edge.target).copied().unwrap_or_default();
        let source_wire = self.handle_cap_peak(source_position, source_out, source_handle.radius, source_shape);
        let target_wire = self.handle_cap_peak(target_position, target_out, target_handle.radius, target_shape);
        Some(compute_edge_sharp_sz_path(source_wire, target_wire, source_out, target_out))
    }

    fn paint_node_handles_for_spec(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, cam: &canvas::camera::Camera, node: &DagNodeSpec, chrome: &DagNodePaintChrome) {
        use canvas::FillRule;
        use canvas::Stroke;
        use canvas::{Circle, Point};
        use graph::{handle_outward_at_node_rim, handle_position_on_rectangle, NodeShape};

        let theme = &self.canvas_theme;
        let handle_stroke_px = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
        let tint = chrome.tint_highlighted();
        let fill = dag_handle_body_fill(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let stroke_c = dag_handle_body_stroke(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let handle_chrome = chrome.has_interaction_chrome();
        let center = Point::new(node.x, node.y);
        let paint_port = |scene: &mut canvas::Scene, port_idx: usize, inputs: bool, port: &IoPortSpec| {
            if !port.visible {
                return;
            }
            let angle = io_node_rect_port_angle_for_node(node, port_idx, inputs);
            let handle_center = handle_position_on_rectangle(center, node.width, node.height, angle);
            let outward = handle_outward_at_node_rim(handle_center, center, NodeShape::Rectangle, 0.0, node.width, node.height);
            if let Some(out) = outward {
                if handle_chrome {
                    scene.fill(FillRule::NonZero, *aff, fill, None, &self.handle_cap_fill_path(handle_center, out, DAG_HANDLE_WORLD_RADIUS, port.shape));
                }
                scene.stroke(&Stroke::new(handle_stroke_px), *aff, stroke_c, None, &self.handle_cap_stroke_path(handle_center, out, DAG_HANDLE_WORLD_RADIUS, port.shape));
            } else {
                let circle = Circle::new(handle_center, DAG_HANDLE_WORLD_RADIUS);
                if handle_chrome {
                    scene.fill(FillRule::NonZero, *aff, fill, None, &circle);
                }
                scene.stroke(&Stroke::new(handle_stroke_px), *aff, stroke_c, None, &circle);
            }
        };
        for (port_idx, port) in node.inputs().iter().enumerate() {
            paint_port(scene, port_idx, true, port);
        }
        for (port_idx, port) in node.outputs().iter().enumerate() {
            paint_port(scene, port_idx, false, port);
        }
    }

    pub fn label_overlay_rows_for_node_spec(&self, node: &DagNodeSpec, ghost: bool) -> Vec<Value> {
        let lod = self.draw_lod_for_frame();
        let zoom = self.host_snapshot.camera.zoom;
        let lod_index = dag_lod_index(zoom);
        let engine_nid = self.node_id_for_widget_id(&node.id);
        Self::label_overlay_rows_for_node(node, lod, zoom, lod_index, ghost, engine_nid, &self.unresolved_input_ports)
    }

    /// 🏷️ `above_body` is the computation/slider layout, where the caption is drawn ABOVE the
    /// node rather than inside it (`computation_name_world_center`) — nothing there is clipped
    /// by the body's own width, and nodes are laid out far enough apart to overhang it. Its
    /// budget is therefore `NODE_TITLE_WIDTH_FACTOR` node widths, while a caption INSIDE the
    /// body (the compact tier) and a rotated one keep the extent they actually sit in.
    fn label_overlay_rows_for_node(node: &DagNodeSpec, lod: DagDrawLod, zoom: f64, lod_index: usize, ghost: bool, engine_nid: Option<NodeId>, unresolved_input_ports: &NumericSet<(NodeId,usize)>) -> Vec<Value> {
        let paint_px = dag_label_paint_px(zoom, lod_index);
        let mut labels = Vec::new();
        if let Some(text) = Self::node_label_text(node, lod).map(str::to_string) {
            let above_body = !lod.node_label_is_horizontal()
                && ((uses_computation_layout(&node.kind) && lod.shows_computation_layout()) || (matches!(node.kind, DagNodeKind::Slider { .. }) && lod.shows_controls()));
            let (layout, x, y) = if lod.node_label_is_horizontal() {
                ("horizontal", node.x, node.y)
            } else if above_body {
                let (lx, ly) = computation_name_world_center(node, &text, paint_px, zoom);
                ("horizontal", lx, ly)
            } else {
                let (lx, ly) = io_widget_label_center(node);
                ("vertical", lx, ly)
            };
            let world_budget = if above_body {
                node.width * ui_styling::metrics::dag::NODE_TITLE_WIDTH_FACTOR
            } else if layout == "vertical" {
                node.height
            } else {
                node.width
            };
            labels.push(semio_framework_pack_json::object([
                ("id".to_string(), Value::from(node.id.clone())),
                ("text".to_string(), Value::from(text)),
                ("layout".to_string(), Value::from(layout)),
                ("x".to_string(), Value::from(x)),
                ("y".to_string(), Value::from(y)),
                ("nodeW".to_string(), Value::from(node.width)),
                ("nodeH".to_string(), Value::from(node.height)),
                ("maxScreenW".to_string(), Value::from(world_budget * zoom.max(0.05) * DAG_LABEL_SCREEN_INSET)),
                ("fontScreenPx".to_string(), Value::from(paint_px)),
                ("ghost".to_string(), Value::from(ghost)),
            ]));
        }
        if lod.shows_port_labels() && !matches!(node.kind, DagNodeKind::Preview { .. } | DagNodeKind::Note { .. }) {
            for mut row in Self::port_label_overlay_rows(node, lod, zoom, lod_index, engine_nid, unresolved_input_ports) {
                if let Some(obj) = row.as_object_mut() {
                    obj.insert("ghost", Value::Bool(ghost));
                }
                labels.push(row);
            }
        }
        labels
    }

    fn port_label_overlay_rows(node: &DagNodeSpec, lod: DagDrawLod, zoom: f64, lod_index: usize, engine_nid: Option<NodeId>, unresolved_input_ports: &NumericSet<(NodeId,usize)>) -> Vec<Value> {
        use canvas::text::label_extent;
        let hw = node.width * 0.5;
        let handle_inset = 8.0 / zoom.max(0.05);
        let inputs = node.inputs();
        let outputs = node.outputs();
        let computation = uses_computation_layout(&node.kind);
        let port_layout_px = if computation { dag_label_compact_paint_px(zoom, lod_index) } else { dag_label_paint_px(zoom, lod_index) };
        let mut rows = Vec::new();
        let input_column_w = if computation { io_port_column_width(inputs, port_layout_px) } else { (hw - handle_inset).max(8.0) };
        let output_column_w = if computation { io_port_column_width(outputs, port_layout_px) } else { (hw - handle_inset).max(8.0) };
        let input_port_label = |index:usize,port: &IoPortSpec| -> String {
            let mut port = port.clone();
            if let Some(nid) = engine_nid {
                if unresolved_input_ports.contains(&(nid,index)) {
                    port.resolved = Some(false);
                }
            }
            port.label_with_cardinality(lod)
        };
        for (i, port) in inputs.iter().enumerate() {
            if port.shape == PortShape::Triangle || !port.visible {
                continue;
            }
            let label = input_port_label(i,port);
            if label.trim().is_empty() {
                continue;
            }
            let world_y = port_center_y(node, i, inputs.len());
            let world_x = if computation { computation_input_label_x(node) } else { node.x - hw + handle_inset };
            rows.push(semio_framework_pack_json::object([
                ("id".to_string(), Value::from(node.id.clone())),
                ("kind".to_string(), Value::from("port")),
                ("text".to_string(), Value::from(label)),
                ("layout".to_string(), Value::from("horizontal")),
                ("align".to_string(), Value::from("left")),
                ("x".to_string(), Value::from(world_x)),
                ("y".to_string(), Value::from(world_y)),
                ("nodeW".to_string(), Value::from(input_column_w)),
                ("nodeH".to_string(), Value::from(DAG_CHANNEL_ROW_HEIGHT)),
                ("maxScreenW".to_string(), Value::from(input_column_w * zoom.max(0.05) * DAG_LABEL_SCREEN_INSET)),
                ("fontScreenPx".to_string(), Value::from(port_layout_px)),
                ("maxScreenH".to_string(), Value::from(port_layout_px * 1.3)),
            ]));
        }
        for (i, port) in outputs.iter().enumerate() {
            if port.shape == PortShape::Triangle || !port.visible {
                continue;
            }
            let label = port.label_with_cardinality(lod);
            if label.trim().is_empty() {
                continue;
            }
            let world_y = port_center_y(node, i, outputs.len());
            let (world_x, column_w) = if computation {
                let left = computation_output_label_x(node, &label, port_layout_px);
                (left + port_label_text_width(&label, port_layout_px), output_column_w)
            } else {
                let (label_w, _) = label_extent(&label, port_layout_px);
                (node.x + hw - handle_inset, label_w / zoom.max(0.05))
            };
            rows.push(semio_framework_pack_json::object([
                ("id".to_string(), Value::from(node.id.clone())),
                ("kind".to_string(), Value::from("port")),
                ("text".to_string(), Value::from(label)),
                ("layout".to_string(), Value::from("horizontal")),
                ("align".to_string(), Value::from("right")),
                ("x".to_string(), Value::from(world_x)),
                ("y".to_string(), Value::from(world_y)),
                ("nodeW".to_string(), Value::from(column_w)),
                ("nodeH".to_string(), Value::from(DAG_CHANNEL_ROW_HEIGHT)),
                ("maxScreenW".to_string(), Value::from(column_w * zoom.max(0.05) * DAG_LABEL_SCREEN_INSET)),
                ("fontScreenPx".to_string(), Value::from(port_layout_px)),
                ("maxScreenH".to_string(), Value::from(port_layout_px * 1.3)),
            ]));
        }
        rows
    }

    /// 🎚️ Slider track anchors for the HTML slider overlay — the ONE row that carries a knob's value.
    ///
    /// 🔢 `value` is published exactly once and the overlay renders it twice: as the control's own
    /// value (hence `aria-valuenow`) and as the readout beside the track. The readout used to be
    /// painted on the GPU instead, which made it a second producer that could only move when a frame
    /// was drawn — a released knob then read `10` to a screen reader and `0.0` on the canvas. The
    /// readout's typography rides along as `fontScreenPx`/`gapScreenPx` so the DOM lands where the
    /// paint did without re-deriving the board's own metrics
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️slider-reevaluation-correctness-2026-09-15.md`).
    pub fn slider_overlay_state_json(&self) -> Result<String, DagError> {
        let cam = &self.host_snapshot.camera;
        let mut sliders: Vec<Value> = Vec::new();
        for (idx, snapshot_node) in self.host_snapshot.nodes.iter().enumerate() {
            let node = self.node_spec_for_paint(idx, snapshot_node);
            let DagNodeKind::Slider { min, max, step, value, .. } = &node.kind else {
                continue;
            };
            let (x0, y0, x1, y1) = slider_track_bounds(&node);
            sliders.push(semio_framework_pack_json::object([
                ("widgetId".to_string(), Value::from(snapshot_node.id.clone())),
                ("label".to_string(), Value::from(node.name.clone())),
                ("value".to_string(), Value::from(*value)),
                ("min".to_string(), Value::from(*min)),
                ("max".to_string(), Value::from(*max)),
                ("step".to_string(), Value::from(*step)),
                ("x".to_string(), Value::from((x0 + x1) * 0.5)),
                ("y".to_string(), Value::from((y0 + y1) * 0.5)),
                ("w".to_string(), Value::from((x1 - x0).max(1.0))),
                ("h".to_string(), Value::from((y1 - y0).max(1.0))),
                ("fontScreenPx".to_string(), Value::from(DAG_LABEL_SCREEN_PX * DAG_SLIDER_VALUE_LABEL_RATIO)),
                ("gapScreenPx".to_string(), Value::from(DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_GAP_RATIO)),
            ]));
        }
        Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("camera".to_string(), semio_framework_pack_json::object([("x".to_string(), Value::from(cam.x)), ("y".to_string(), Value::from(cam.y)), ("zoom".to_string(), Value::from(cam.zoom))])),
            ("width".to_string(), Value::from(self.width)),
            ("height".to_string(), Value::from(self.height)),
            ("sliders".to_string(), Value::Array(sliders)),
        ])))
    }

    /// 🏷️ Camera, draw LOD, and node label anchors for the JS canvas text overlay (must match the last GPU frame).
    pub fn label_overlay_paint_state_json(&self) -> Result<String, DagError> {
        let lod = self.draw_lod_for_frame();
        let cam = &self.host_snapshot.camera;
        let lod_index = dag_lod_index(cam.zoom);
        let mut labels = Vec::new();
        for (idx, snapshot_node) in self.host_snapshot.nodes.iter().enumerate() {
            let node = self.node_spec_for_paint(idx, snapshot_node);
            let engine_nid = self.engine_node_id_for_index(idx);
            labels.extend(Self::label_overlay_rows_for_node(node.as_ref(), lod, cam.zoom, lod_index, false, engine_nid, &self.unresolved_input_ports));
        }
        if let Some(ghost) = self.ghost_node.as_ref() {
            labels.extend(Self::label_overlay_rows_for_node(ghost, lod, cam.zoom, lod_index, true, None, &self.unresolved_input_ports));
        }
        let minimap_widget = self.minimap_widget_json();
        Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("camera".to_string(), semio_framework_pack_json::object([("x".to_string(), Value::from(cam.x)), ("y".to_string(), Value::from(cam.y)), ("zoom".to_string(), Value::from(cam.zoom))])),
            ("lod".to_string(), Value::from(lod.label())),
            ("width".to_string(), Value::from(self.width)),
            ("height".to_string(), Value::from(self.height)),
            ("labels".to_string(), Value::Array(labels)),
            ("minimapWidget".to_string(), Value::from(minimap_widget)),
        ])))
    }

    fn paint_variadic_plus_controls(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, px: f64, fill: canvas::Color, halo: canvas::Color) {
        use canvas::camera::world_to_screen;
        use canvas::text::append_label;
        for (_, px_world, py_world) in variadic_input_insert_positions(node) {
            let screen = world_to_screen(cam, viewport, canvas::Point::new(px_world, py_world));
            append_label(scene, "+", screen, px * 0.95, fill, halo);
        }
        for (_, px_world, py_world) in variadic_output_insert_positions(node) {
            let screen = world_to_screen(cam, viewport, canvas::Point::new(px_world, py_world));
            append_label(scene, "+", screen, px * 0.95, fill, halo);
        }
    }

    fn paint_node_name_vertical(scene: &mut canvas::Scene, center_screen: canvas::Point, name: &str, px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
        use canvas::text::{append_label, label_extent};
        use canvas::{Affine, Point};
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return;
        }
        let (w, h) = label_extent(trimmed, px);
        let mut label_scene = canvas::Scene::new();
        append_label(&mut label_scene, trimmed, Point::new(0.0, 0.0), px, label_fill, label_halo);
        let rot = Affine::IDENTITY.translate((center_screen.x, center_screen.y)) * Affine::IDENTITY.rotate(-std::f64::consts::FRAC_PI_2) * Affine::IDENTITY.translate((-w * 0.5, -h * 0.5));
        scene.append(&label_scene, Some(rot));
    }

    fn paint_computation_column_divider(scene: &mut canvas::Scene, aff: canvas::Affine, node: &DagNodeSpec, chrome_stroke: f64, stroke: canvas::Color) {
        use canvas::{Line, Point, Stroke};
        let Some(divider_x) = computation_column_divider_x(node) else {
            return;
        };
        let hh = node.height * 0.5;
        let top = node.y - hh;
        let bottom = node.y + hh;
        let stroke_style = Stroke::new(chrome_stroke);
        scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(divider_x, top), Point::new(divider_x, bottom)));
    }

    fn paint_computation_channel_row_highlights(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, node: &DagNodeSpec, theme: &CanvasPalette, is_dimmed: bool) {
        use canvas::FillRule;
        use canvas::Rect;
        let mut paint_bounds = |(x0, y0, x1, y1): (f64, f64, f64, f64), selected: bool, highlighted: bool, hovered: bool| {
            if !selected && !highlighted && !hovered {
                return;
            }
            let fill = dag_handle_body_fill(theme, is_dimmed, selected, highlighted, hovered);
            scene.fill(FillRule::NonZero, *aff, fill, None, &Rect::new(x0, y0, x1, y1));
        };
        for (port_idx, port) in node.inputs().iter().enumerate() {
            let Some(bounds) = input_port_row_hit_bounds(node, port_idx) else {
                continue;
            };
            let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, true) else {
                continue;
            };
            let (selected, highlighted, hovered) = self.handle_interaction_chrome(hid);
            paint_bounds(bounds, selected, highlighted, hovered);
        }
        for (port_idx, port) in node.outputs().iter().enumerate() {
            let Some(bounds) = output_port_row_hit_bounds(node, port_idx) else {
                continue;
            };
            let Some(hid) = self.handle_id_for_port_side(&node.id, &port.id, false) else {
                continue;
            };
            let (selected, highlighted, hovered) = self.handle_interaction_chrome(hid);
            paint_bounds(bounds, selected, highlighted, hovered);
        }
    }

    fn computation_channel_row_divider_stroke(&self, node: &DagNodeSpec, port_id: &str, body_stroke: canvas::Color, label_fill: canvas::Color, default_stroke: canvas::Color) -> canvas::Color {
        let Some(hid) = self.handle_id_for_port(&node.id, port_id) else {
            return default_stroke;
        };
        let (selected, _, hovered) = self.handle_interaction_chrome(hid);
        if selected || hovered {
            dag_node_internal_chrome_stroke(body_stroke, label_fill, true)
        } else {
            default_stroke
        }
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_computation_channel_row_dividers(&self, scene: &mut canvas::Scene, aff: canvas::Affine, node: &DagNodeSpec, chrome_stroke: f64, stroke: canvas::Color, body_stroke: canvas::Color, label_fill: canvas::Color, channel_row_pick: bool) {
        use canvas::{Line, Point, Stroke};
        let grid_rows = computation_channel_row_count(node);
        if grid_rows <= 1 {
            return;
        }
        let (input_rows, output_rows) = computation_io_side_row_counts(node);
        let stroke_style = Stroke::new(chrome_stroke);
        let row_stroke = |port_id: &str| {
            if channel_row_pick {
                self.computation_channel_row_divider_stroke(node, port_id, body_stroke, label_fill, stroke)
            } else {
                stroke
            }
        };
        if computation_input_column_x_bounds(node).is_some() {
            let (left, right) = computation_channel_row_divider_x_span(node, ComputationChannelRowSide::Input);
            let inputs = node.inputs();
            for row in computation_io_side_row_divider_indices(input_rows, grid_rows) {
                let y = channel_row_divider_y(node.y, node.height, row);
                let port_id = inputs.get(row.saturating_sub(1)).map_or("", |port| port.id.as_str());
                scene.stroke(&stroke_style, aff, row_stroke(port_id), None, &Line::new(Point::new(left, y), Point::new(right, y)));
            }
        }
        if computation_output_column_x_bounds(node).is_some() {
            let (left, right) = computation_channel_row_divider_x_span(node, ComputationChannelRowSide::Output);
            let outputs = node.outputs();
            for row in computation_io_side_row_divider_indices(output_rows, grid_rows) {
                let y = channel_row_divider_y(node.y, node.height, row);
                let port_id = outputs.get(row.saturating_sub(1)).map_or("", |port| port.id.as_str());
                scene.stroke(&stroke_style, aff, row_stroke(port_id), None, &Line::new(Point::new(left, y), Point::new(right, y)));
            }
        }
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_preview_image_content(&self, scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, src: &str, label_fill: canvas::Color, bg: canvas::Color) {
        use canvas::camera::world_to_screen;
        use canvas::text::append_label;
        use canvas::Point;
        if src.is_empty() {
            let (x0, y0, x1, y1) = preview_content_bounds(node);
            let pos = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
            append_label(scene, "Image…", pos, DAG_LABEL_COMPACT_SCREEN_PX, label_fill, bg);
            return;
        }
        let (x0, y0, x1, y1) = preview_content_bounds(node);
        let center = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
        let w = (x1 - x0).max(1.0);
        let h = (y1 - y0).max(1.0);
        self.icon_paint_cache.append_icon_at_screen_rect(scene, src, graph::IconScreenRect { center, width: w, height: h }, label_fill, bg, true);
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_preview_content(
        &self,
        scene: &mut canvas::Scene,
        cam: &canvas::camera::Camera,
        viewport: &canvas::camera::Viewport,
        node: &DagNodeSpec,
        content: &DagPreviewContent,
        expanded: &DagExpandedPaths,
        paint_px: f64,
        label_fill: canvas::Color,
        label_halo: canvas::Color,
        bg: canvas::Color,
    ) {
        use canvas::camera::world_to_screen;
        use canvas::text::append_label;
        use canvas::Point;
        match content {
            DagPreviewContent::Empty => {
                let (x0, y0, x1, y1) = preview_content_bounds(node);
                let pos = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
                append_label(scene, "—", pos, paint_px, label_fill, label_halo);
            }
            DagPreviewContent::Scalar { text } => {
                let (x0, y0, x1, y1) = preview_content_bounds(node);
                let display = if text.is_empty() { "—" } else { text.as_str() };
                let pos = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
                append_label(scene, display, pos, paint_px * 1.05, label_fill, label_halo);
            }
            DagPreviewContent::Image { src } => {
                self.paint_preview_image_content(scene, cam, viewport, node, src, label_fill, bg);
            }
            DagPreviewContent::Tree { json } => {
                let (x0, y0, x1, _) = preview_content_bounds(node);
                let rows = preview_tree_rows(json, expanded, "", 0);
                let row_font_px = paint_px * 0.9;
                for (index, row) in rows.iter().enumerate() {
                    let row_y = y0 + index as f64 * DAG_PREVIEW_ROW_HEIGHT + DAG_PREVIEW_ROW_HEIGHT * 0.5;
                    let indent = row.depth as f64 * DAG_PREVIEW_TREE_INDENT;
                    if row.has_children {
                        let glyph = if row.expanded { "▾️" } else { "▸️" };
                        let toggle_pos = world_to_screen(cam, viewport, Point::new(x0 + indent + DAG_PREVIEW_TOGGLE_WIDTH * 0.5, row_y));
                        append_label(scene, glyph, toggle_pos, row_font_px, label_fill, label_halo);
                    }
                    let text_x = x0 + indent + if row.has_children { DAG_PREVIEW_TOGGLE_WIDTH } else { 0.0 } + 2.0;
                    let line = if row.has_children && row.expanded { row.label.clone() } else { format!("{}: {}", row.label, row.summary) };
                    let max_w = (x1 - text_x).max(1.0);
                    let shown = truncate_label_to_fit_width(&line, max_w, row_font_px);
                    let text_pos = world_to_screen(cam, viewport, Point::new(text_x, row_y));
                    append_label(scene, &shown, text_pos, row_font_px, label_fill, label_halo);
                }
            }
        }
    }

    fn paint_io_widget_channel_borders(scene: &mut canvas::Scene, aff: canvas::Affine, node: &DagNodeSpec, px: f64, chrome_stroke: f64, stroke: canvas::Color) {
        use canvas::{Line, Point, Stroke};
        let (name_left, top, name_right, bottom) = io_widget_name_column_bounds(node, px);
        let stroke_style = Stroke::new(chrome_stroke);
        scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(name_left, top), Point::new(name_left, bottom)));
        scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(name_right, top), Point::new(name_right, bottom)));
    }

    fn node_label_text(node: &DagNodeSpec, lod: DagDrawLod) -> Option<&str> {
        if matches!(node.kind, DagNodeKind::Preview { .. } | DagNodeKind::Note { .. }) {
            return None;
        }
        let text = match lod.node_label() {
            DagNodeLabel::None => return None,
            DagNodeLabel::Name => node.name.trim(),
        };
        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    }

    fn node_caption_delegated_to_js_overlay(node: &DagNodeSpec, lod: DagDrawLod) -> bool {
        Self::node_label_text(node, lod).is_some()
    }

    pub(super) fn should_paint_node_lod_icon(node: &DagNodeSpec, lod: DagDrawLod) -> bool {
        if !lod.node_icon_visible() {
            return false;
        }
        !uses_computation_layout(&node.kind) || !lod.shows_computation_layout()
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_node_lod_icon(&self, scene: &mut canvas::Scene, lod: DagDrawLod, center_screen: canvas::Point, node: &DagNodeSpec, zoom: f64, fg: canvas::Color, bg: canvas::Color) {
        if !Self::should_paint_node_lod_icon(node, lod) {
            return;
        }
        let icon = node.icon.trim();
        if icon.is_empty() {
            return;
        }
        let screen_w = node.width * zoom.max(0.05);
        let screen_h = node.height * zoom.max(0.05);
        self.icon_paint_cache.append_icon_at_screen_rect(scene, icon, graph::IconScreenRect { center: center_screen, width: screen_w, height: screen_h }, fg, bg, false);
    }

    fn paint_cluster_affordances(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, paint_px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
        use canvas::camera::world_to_screen;
        use canvas::text::append_label;
        use canvas::Point;
        let (name_x, name_y) = computation_name_world_center(node, &node.name, paint_px, cam.zoom);
        let glyph_pos = world_to_screen(cam, viewport, Point::new(name_x - paint_px * 0.55, name_y));
        append_label(scene, "🧩️", glyph_pos, paint_px * 0.85, label_fill, label_halo);
        if let Some((x0, y0, x1, y1)) = cluster_explode_hit_rect(node) {
            let cx = (x0 + x1) * 0.5;
            let cy = (y0 + y1) * 0.5;
            let explode_pos = world_to_screen(cam, viewport, Point::new(cx, cy));
            append_label(scene, "⤢", explode_pos, paint_px * 0.75, label_fill, label_halo);
        }
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_computation_node_name(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, label: &str, px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
        use canvas::camera::world_to_screen;
        use canvas::Point;
        let (label_x, label_y) = computation_name_world_center(node, label, px, cam.zoom);
        let anchor = world_to_screen(cam, viewport, Point::new(label_x, label_y));
        Self::paint_node_name_horizontal(scene, anchor, label, px, label_fill, label_halo);
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_slider_name(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, label: &str, px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
        Self::paint_computation_node_name(scene, cam, viewport, node, label, px, label_fill, label_halo);
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_io_widget_name(scene: &mut canvas::Scene, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, node: &DagNodeSpec, lod: DagDrawLod, label: &str, px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
        use canvas::camera::world_to_screen;
        use canvas::Point;
        if lod.node_label() == DagNodeLabel::None && !lod.shows_controls() {
            return;
        }
        let (label_x, label_y) = io_widget_label_center(node);
        let name_anchor = world_to_screen(cam, viewport, Point::new(label_x, label_y));
        Self::paint_node_name_vertical(scene, name_anchor, label, px, label_fill, label_halo);
    }

    fn paint_node_name_horizontal(scene: &mut canvas::Scene, center_screen: canvas::Point, name: &str, px: f64, label_fill: canvas::Color, label_halo: canvas::Color) {
        use canvas::text::{append_label, label_extent};
        use canvas::Point;
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return;
        }
        let (w, h) = label_extent(trimmed, px);
        append_label(scene, trimmed, Point::new(center_screen.x - w * 0.5, center_screen.y - h * 0.5), px, label_fill, label_halo);
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_computing_border_arc(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, rect: &canvas::Rect, cam_zoom: f64, color: canvas::Color, start_t: f64, dashed: bool) {
        use canvas::{BezPath, Stroke};
        const SEGMENTS: usize = 40;
        const ARC_FRACTION: f64 = 0.24;
        let mut path = BezPath::new();
        for i in 0..=SEGMENTS {
            let local = i as f64 / SEGMENTS as f64;
            let t = (start_t + local * ARC_FRACTION).fract();
            let p = Self::rect_perimeter_point(rect, t);
            if i == 0 {
                path.move_to(p);
            } else {
                path.line_to(p);
            }
        }
        let stroke_px = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX * 1.75, cam_zoom);
        let mut stroke = Stroke::new(stroke_px);
        if dashed {
            stroke.set_dash_pattern(vec![stroke_px * 2.5, stroke_px * 2.0]);
        }
        scene.stroke(&stroke, *aff, color, None, &path);
    }

    fn paint_computing_active_border(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, rect: &canvas::Rect, cam_zoom: f64, color: canvas::Color) {
        self.paint_computing_border_arc(scene, aff, rect, cam_zoom, color, self.computing_active_anim_phase.get(), false);
    }

    fn paint_computing_stale_border(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, rect: &canvas::Rect, cam_zoom: f64, color: canvas::Color) {
        self.paint_computing_border_arc(scene, aff, rect, cam_zoom, color, self.computing_stale_anim_phase.get(), true);
    }

    fn paint_eval_status_border(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, rect: &canvas::Rect, cam_zoom: f64, color: canvas::Color, dashed: bool) {
        self.paint_computing_border_arc(scene, aff, rect, cam_zoom, color, 0.0, dashed);
    }

    fn rect_perimeter_point(rect: &canvas::Rect, t: f64) -> canvas::Point {
        use canvas::Point;
        let t = t.fract();
        let w = rect.width();
        let h = rect.height();
        let perim = 2.0 * (w + h);
        let mut d = t * perim;
        if d <= w {
            return Point::new(rect.x0() + d, rect.y0());
        }
        d -= w;
        if d <= h {
            return Point::new(rect.x1(), rect.y0() + d);
        }
        d -= h;
        if d <= w {
            return Point::new(rect.x1() - d, rect.y1());
        }
        d -= w;
        Point::new(rect.x0(), rect.y1() - d)
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_note_caret_bar(scene: &mut canvas::Scene, aff: &canvas::Affine, node: &DagNodeSpec, caret_byte: usize, text: &str, font_px: f64, fill: canvas::Color, zoom: f64) {
        use canvas::text::label_byte_world_x;
        use canvas::FillRule;
        use canvas::Rect;
        let (x0, y0, _x1, y1) = preview_content_bounds(node);
        let origin_x = x0 + DAG_PREVIEW_PAD;
        let caret_x = label_byte_world_x(text, caret_byte.min(text.len()), origin_x, font_px);
        let caret_y = (y0 + y1) * 0.5;
        let lh = font_px * 1.2;
        let bar_w = 1.5 / zoom.max(0.05);
        let rect = Rect::new(caret_x, caret_y - lh * 0.4, caret_x + bar_w, caret_y + lh * 0.4);
        scene.fill(FillRule::NonZero, *aff, fill, None, &rect);
    }

    /// 📐️ Center divider and channel-row chrome for a one-sided or two-sided widget.
    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_sided_widget_columns(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, lod: DagDrawLod, node: &DagNodeSpec, theme: &CanvasPalette, chrome: &DagNodePaintChrome, chrome_stroke: f64, internal_stroke: canvas::Color, body_stroke: canvas::Color, label_fill: canvas::Color) {
        if !lod.shows_computation_layout() {
            return;
        }
        let channel_row_pick = lod.uses_channel_row_pick();
        if channel_row_pick {
            self.paint_computation_channel_row_highlights(scene, aff, node, theme, chrome.is_dimmed);
        }
        Self::paint_computation_column_divider(scene, *aff, node, chrome_stroke, internal_stroke);
        self.paint_computation_channel_row_dividers(scene, *aff, node, chrome_stroke, internal_stroke, body_stroke, label_fill, channel_row_pick);
    }

    /// 🎚️ Track, knob AND value readout live on the HTML `GraphSliderOverlays` control,
    /// which renders all three off ONE published row (`slider_overlay_state_json`). A
    /// readout painted here was a SECOND producer of the same number: it could only
    /// change when a GPU frame was drawn, while `aria-valuenow` moved with the DOM
    /// control, so a released knob read `10` to a screen reader and `0.0` on the canvas
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
    /// `📓️slider-reevaluation-correctness-2026-09-15.md`).
    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_node_visual(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, cam: &canvas::camera::Camera, viewport: &canvas::camera::Viewport, lod: DagDrawLod, lod_index: usize, node: &DagNodeSpec, chrome: DagNodePaintChrome) {
        use canvas::camera::world_to_screen;
        use canvas::text::append_label;
        use canvas::FillRule;
        use canvas::{Point, Rect, Stroke};

        let theme = &self.canvas_theme;
        let label_halo = theme.label_halo;
        let hw = node.width * 0.5;
        let hh = node.height * 0.5;
        let rect = Rect::new(node.x - hw, node.y - hh, node.x + hw, node.y + hh);
        let tint = chrome.tint_highlighted();
        let fill = dag_node_paint_fill(lod, theme, chrome.is_dimmed, chrome.is_selected, chrome.is_highlighted, chrome.is_hovered).map(|color| canvas_color_with_alpha(color, chrome.body_fill_alpha));
        let stroke = dag_node_body_stroke(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let label_fill = dag_node_label_fill(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let internal_chrome_stroke = dag_node_internal_chrome_stroke(stroke, label_fill, chrome.is_hovered || chrome.is_selected || chrome.is_highlighted);
        let stroke_screen_px = dag_node_stroke_screen_px(chrome.is_dimmed, chrome.is_selected, chrome.is_highlighted, chrome.is_hovered);
        if let Some(fill) = fill {
            scene.fill(FillRule::NonZero, *aff, fill, None, &rect);
        }
        if !chrome.is_selected {
            scene.stroke(&Stroke::new(dag_world_stroke(stroke_screen_px, cam.zoom)), *aff, stroke, None, &rect);
        }
        match chrome.eval_status {
            DagNodeEvalStatusKind::Computing => self.paint_computing_active_border(scene, aff, &rect, cam.zoom, theme.node_stroke_computing),
            DagNodeEvalStatusKind::Queued => self.paint_computing_stale_border(scene, aff, &rect, cam.zoom, theme.node_stroke_stale),
            DagNodeEvalStatusKind::Error => self.paint_eval_status_border(scene, aff, &rect, cam.zoom, theme.node_stroke_error, false),
            DagNodeEvalStatusKind::Blocked => self.paint_eval_status_border(scene, aff, &rect, cam.zoom, theme.node_stroke_blocked, true),
            DagNodeEvalStatusKind::Ok => {}
        }
        let layout_px = dag_label_layout_px();
        let paint_px = dag_label_paint_px(cam.zoom, lod_index);
        let chrome_stroke = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
        let center_screen = world_to_screen(cam, viewport, Point::new(node.x, node.y));
        let node_fill = fill.unwrap_or(theme.node_fill);
        if !matches!(node.kind, DagNodeKind::Preview { .. } | DagNodeKind::Note { .. }) {
            self.paint_node_lod_icon(scene, lod, center_screen, node, cam.zoom, label_fill, node_fill);
        }
        let label_text = Self::node_label_text(node, lod);
        let caption_on_overlay = Self::node_caption_delegated_to_js_overlay(node, lod);
        if lod.node_label_is_horizontal() {
            if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                Self::paint_node_name_horizontal(scene, center_screen, label, paint_px, label_fill, label_halo);
            }
        } else {
            match &node.kind {
                DagNodeKind::Computation { .. } | DagNodeKind::Cluster { .. } => {
                    if lod.shows_computation_layout() {
                        let channel_row_pick = lod.uses_channel_row_pick();
                        if channel_row_pick {
                            self.paint_computation_channel_row_highlights(scene, aff, node, theme, chrome.is_dimmed);
                        }
                        Self::paint_computation_column_divider(scene, *aff, node, chrome_stroke, internal_chrome_stroke);
                        self.paint_computation_channel_row_dividers(scene, *aff, node, chrome_stroke, internal_chrome_stroke, stroke, label_fill, channel_row_pick);
                        if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                            Self::paint_computation_node_name(scene, cam, viewport, node, label, paint_px, label_fill, label_halo);
                        }
                        if matches!(node.kind, DagNodeKind::Cluster { .. }) {
                            Self::paint_cluster_affordances(scene, cam, viewport, node, paint_px, label_fill, label_halo);
                        }
                        if let DagNodeKind::Computation { variadic_inputs, variadic_outputs, .. } = &node.kind {
                            if cam.zoom >= DAG_VARIADIC_PLUS_ZOOM_THRESHOLD && (*variadic_inputs || *variadic_outputs) {
                                Self::paint_variadic_plus_controls(scene, cam, viewport, node, paint_px, label_fill, label_halo);
                            }
                        }
                    }
                }
                DagNodeKind::Slider { .. } => {
                    self.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                    if let Some(label) = label_text.filter(|_| lod.shows_controls() && !caption_on_overlay) {
                        Self::paint_slider_name(scene, cam, viewport, node, label, paint_px, label_fill, label_halo);
                    }
                }
                DagNodeKind::Select { options, selected, .. } => {
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                    }
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                    }
                    if lod.shows_controls() {
                        let (cx0, cy0, cx1, cy1) = select_control_bounds(node);
                        let control = Rect::new(cx0, cy0, cx1, cy1);
                        scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &control);
                        if lod.shows_detail_text() {
                            let option = usize::try_from(*selected).ok().and_then(|index| options.get(index)).map_or("—", String::as_str);
                            let option_pos = world_to_screen(cam, viewport, Point::new((cx0 + cx1) * 0.5, (cy0 + cy1) * 0.5));
                            append_label(scene, option, option_pos, paint_px * 0.95, label_fill, label_halo);
                            let chevron = world_to_screen(cam, viewport, Point::new(cx1 - 6.0 / cam.zoom.max(0.05), (cy0 + cy1) * 0.5));
                            append_label(scene, "▾️", chevron, paint_px, label_fill, label_halo);
                        }
                    }
                }
                DagNodeKind::Screen { media, .. } => {
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                    }
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                    }
                    if lod.shows_controls() {
                        let inset = 8.0 / cam.zoom.max(0.05);
                        let frame = Rect::new(node.x - hw + inset, node.y - hh + hh * 0.35, node.x + hw - inset, node.y + hh - inset);
                        scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke_selection_exit, None, &frame);
                    }
                    if lod.shows_detail_text() {
                        if let Some(media) = media {
                            let kind_label = match media.kind {
                                DagMediaKind::Image => "image",
                                DagMediaKind::Svg => "svg",
                                DagMediaKind::Pdf => "pdf",
                                DagMediaKind::Video => "video",
                            };
                            let hint = world_to_screen(cam, viewport, Point::new(node.x, node.y + hh * 0.1));
                            append_label(scene, kind_label, hint, paint_px * 0.85, canvas_color_with_alpha(label_fill, ui_styling::opacities::KIND_HINT_ALPHA), label_halo);
                        }
                    }
                }
                DagNodeKind::Note { text, .. } => {
                    self.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                    if lod.shows_detail_text() || lod.shows_controls() {
                        let (x0, y0, x1, y1) = preview_content_bounds(node);
                        let font_px = paint_px * 1.05;
                        let text_x = x0 + DAG_PREVIEW_PAD;
                        let max_w = (x1 - text_x).max(1.0);
                        let display = if text.is_empty() { "…" } else { text.as_str() };
                        let shown = truncate_label_to_fit_width(display, max_w, font_px);
                        let pos = world_to_screen(cam, viewport, Point::new(text_x, (y0 + y1) * 0.5));
                        append_label(scene, &shown, pos, font_px, label_fill, label_halo);
                        if let Some(edit) = &self.editing_note {
                            if edit.node_id == node.id && self.caret_visible {
                                Self::paint_note_caret_bar(scene, aff, node, edit.caret, text, font_px, label_fill, cam.zoom);
                            }
                        }
                    }
                }
                DagNodeKind::Image { src, .. } => {
                    self.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                    }
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                    }
                    if lod.shows_controls() {
                        let (x0, y0, x1, y1) = preview_content_bounds(node);
                        let frame = Rect::new(x0, y0, x1, y1);
                        scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &frame);
                    }
                    if lod.shows_detail_text() || lod.shows_controls() {
                        self.paint_preview_image_content(scene, cam, viewport, node, src, label_fill, theme.raster_clear);
                    }
                }
                DagNodeKind::Preview { content, expanded, .. } => {
                    self.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                    if lod.shows_detail_text() || lod.shows_controls() {
                        self.paint_preview_content(scene, cam, viewport, node, content, expanded, paint_px, label_fill, label_halo, theme.raster_clear);
                    }
                }
                DagNodeKind::Action { label, .. } => {
                    self.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                    }
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                    }
                    if lod.shows_controls() {
                        let (x0, y0, x1, y1) = action_control_bounds(node);
                        let control = Rect::new(x0, y0, x1, y1);
                        scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &control);
                        if lod.shows_detail_text() {
                            let pos = world_to_screen(cam, viewport, Point::new(node.x, node.y));
                            append_label(scene, label, pos, paint_px * 0.95, label_fill, label_halo);
                        }
                    }
                }
                DagNodeKind::Export { format, .. } => {
                    self.paint_sided_widget_columns(scene, aff, lod, node, theme, &chrome, chrome_stroke, internal_chrome_stroke, stroke, label_fill);
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                    }
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                    }
                    if lod.shows_controls() {
                        let (x0, y0, x1, y1) = action_control_bounds(node);
                        let control = Rect::new(x0, y0, x1, y1);
                        scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &control);
                        if lod.shows_detail_text() {
                            let export_label = format.to_uppercase();
                            let pos = world_to_screen(cam, viewport, Point::new(node.x, node.y));
                            append_label(scene, &export_label, pos, paint_px * 0.95, label_fill, label_halo);
                        }
                    }
                }
                DagNodeKind::AppInstance { plugin_id, app_id, .. } => {
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_node_name_horizontal(scene, center_screen, label, paint_px, label_fill, label_halo);
                    }
                    if lod.shows_detail_text() {
                        let subtitle = format!("{plugin_id}/{app_id}");
                        let subtitle_pos = world_to_screen(cam, viewport, Point::new(node.x, node.y + hh * 0.22));
                        append_label(scene, &subtitle, subtitle_pos, paint_px * 0.85, label_fill, label_halo);
                    }
                }
            }
        }
    }

    pub fn paint_scene(&self, scene: &mut canvas::Scene, viewport_w: u32, viewport_h: u32, dpr: f64) {
        use canvas::camera::{camera_content_affine, Camera as CanvasCamera, Viewport};
        use canvas::FillRule;
        use canvas::{Circle, Rect, Stroke};

        let theme = &self.canvas_theme;
        self.tick_computing_animation();
        let cam = CanvasCamera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom };
        let viewport = Viewport { width: viewport_w.max(1), height: viewport_h.max(1), dpr: dpr.max(1.0) };
        let aff = camera_content_affine(&cam, &viewport);
        let lod = self.draw_lod_for_frame();
        let lod_index = dag_lod_index(cam.zoom);
        self.paint_lod_grid(scene, &cam, &viewport, lod);
        let snap = self.engine.render_snapshot();
        let edge_stroke = dag_world_stroke(lod.edge_stroke_screen_px(), cam.zoom);
        for &eid in self.engine.edges.keys() {
            let (is_selected, is_highlighted, is_hovered) = self.edge_interaction_chrome(eid);
            let stroke_c = dag_edge_body_stroke(theme, false, is_selected, is_highlighted, is_hovered);
            let style = self.edge_route_style.get(&eid).copied().unwrap_or_default();
            match style {
                EdgeRouteStyle::Bezier => {
                    if let Some(curve) = self.engine.edge_curve(eid) {
                        scene.stroke(&Stroke::new(edge_stroke), aff, stroke_c, None, &curve);
                    }
                }
                EdgeRouteStyle::SharpSz => {
                    if let Some(path) = self.edge_sharp_path(eid) {
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
            if self.handle_port_visible.get(hid) == Some(&false) {
                return;
            }
            let shape = self.handle_port_shape.get(hid).copied().unwrap_or_default();
            if let Some(filter) = shape_filter {
                if shape != filter {
                    return;
                }
            }
            if !lod.shows_handles() && shape != PortShape::Triangle {
                return;
            }
            let node_id = self.engine.handles.get(hid).map(|handle| handle.node_id);
            let is_dimmed = node_id.is_some_and(|nid| self.dimmed.contains(&nid));
            let (is_selected, is_highlighted, is_hovered) = self.handle_interaction_chrome(*hid);
            let fill = dag_handle_body_fill(theme, is_dimmed, is_selected, is_highlighted, is_hovered);
            let stroke_c = dag_handle_body_stroke(theme, is_dimmed, is_selected, is_highlighted, is_hovered);
            let chrome = is_dimmed || is_selected || is_highlighted || is_hovered;
            let outward = node_id.and_then(|nid| self.engine.nodes.get(&nid).and_then(|node| handle_outward_at_node_rim(*center, node.center, node.shape, node.radius, node.width, node.height)));
            if let Some(out) = outward {
                if chrome {
                    scene.fill(FillRule::NonZero, aff, fill, None, &self.handle_cap_fill_path(*center, out, DAG_HANDLE_WORLD_RADIUS, shape));
                }
                scene.stroke(&Stroke::new(handle_stroke_px), aff, stroke_c, None, &self.handle_cap_stroke_path(*center, out, DAG_HANDLE_WORLD_RADIUS, shape));
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
            let node = self.node_spec_for_paint(idx, snapshot_node);
            let node = node.as_ref();
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            let rect = Rect::new(node.x - hw, node.y - hh, node.x + hw, node.y + hh);
            let engine_nid = self.engine_node_id_for_index(idx);
            let is_dimmed = engine_nid.is_some_and(|nid| self.dimmed.contains(&nid));
            let (is_selected, is_highlighted, is_hovered) = engine_nid.map_or((false, false, false), |nid| self.node_interaction_chrome(nid));
            if let Some(fill) = dag_node_paint_fill(lod, theme, is_dimmed, is_selected, is_highlighted, is_hovered) {
                scene.fill(FillRule::NonZero, aff, fill, None, &rect);
            }
        };
        if lod == DagDrawLod::Minimap {
            for (idx, snapshot_node) in self.host_snapshot.nodes.iter().enumerate() {
                let engine_nid = self.engine_node_id_for_index(idx);
                let chrome = engine_nid.is_some_and(|nid| {
                    let (selected, highlighted, hovered) = self.node_interaction_chrome(nid);
                    selected || highlighted || hovered
                });
                if !chrome {
                    paint_minimap_node(scene, idx, snapshot_node);
                }
            }
            for (idx, snapshot_node) in self.host_snapshot.nodes.iter().enumerate() {
                let engine_nid = self.engine_node_id_for_index(idx);
                let chrome = engine_nid.is_some_and(|nid| {
                    let (selected, highlighted, hovered) = self.node_interaction_chrome(nid);
                    selected || highlighted || hovered
                });
                if chrome {
                    paint_minimap_node(scene, idx, snapshot_node);
                }
            }
        } else {
            for (idx, snapshot_node) in self.host_snapshot.nodes.iter().enumerate() {
                let node = self.node_spec_for_paint(idx, snapshot_node);
                let node = node.as_ref();
                let engine_nid = self.engine_node_id_for_index(idx);
                let is_dimmed = engine_nid.is_some_and(|nid| self.dimmed.contains(&nid));
                let (is_selected, is_highlighted, is_hovered) = engine_nid.map_or((false, false, false), |nid| self.node_interaction_chrome(nid));
                let eval_status = engine_nid.and_then(|nid| self.node_eval_status.get(&nid).copied()).unwrap_or(DagNodeEvalStatusKind::Ok);
                self.paint_node_visual(scene, &aff, &cam, &viewport, lod, lod_index, node, DagNodePaintChrome { is_dimmed, is_selected, is_highlighted, is_hovered, eval_status, body_fill_alpha: 255, ghost_tint: false });
            }
        }
        if let Some(ghost) = self.ghost_node.as_ref() {
            self.paint_node_visual(scene, &aff, &cam, &viewport, lod, lod_index, ghost, DagNodePaintChrome::ghost_preview());
        }
        for (hid, center, _radius) in &snap.handles {
            paint_snap_handle(scene, hid, center, Some(PortShape::Triangle));
        }
        if let Some(ghost) = self.ghost_node.as_ref() {
            let paint_ghost_handles = lod.shows_handles() || ghost.inputs().iter().any(|port| port.shape == PortShape::Triangle) || ghost.outputs().iter().any(|port| port.shape == PortShape::Triangle);
            if paint_ghost_handles {
                self.paint_node_handles_for_spec(scene, &aff, &cam, ghost, &DagNodePaintChrome::ghost_preview());
            }
        }
        self.paint_minimap_widget(scene, viewport_w, viewport_h);
    }
}
// #endregion 🔖️DagHost
