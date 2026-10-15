//! 🧱️ DAG host content framing, minimap widget and selection alignment.

use super::*;

impl DagHost {
    // #region 🔖️DagHost
    // #region 🔖️ContentFraming
    /// 🖼️ World bounds of every node this board holds — what a fit frames and what the minimap maps.
    /// `None` for an empty graph, which has nothing to frame.
    pub fn content_world_bounds(&self) -> Option<canvas::camera::ContentBounds> {
        let first = self.host_snapshot.nodes.first()?;
        let mut union = Self::dag_node_world_bounds(first);
        for node in self.host_snapshot.nodes.iter().skip(1) {
            let b = Self::dag_node_world_bounds(node);
            union.min_x = union.min_x.min(b.min_x);
            union.min_y = union.min_y.min(b.min_y);
            union.max_x = union.max_x.max(b.max_x);
            union.max_y = union.max_y.max(b.max_y);
        }
        Some(canvas::camera::ContentBounds { min_x: union.min_x, min_y: union.min_y, max_x: union.max_x, max_y: union.max_y })
    }

    fn camera_viewport(&self) -> canvas::camera::Viewport {
        canvas::camera::Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) }
    }

    fn camera_state(&self) -> canvas::camera::Camera {
        canvas::camera::Camera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom }
    }

    /// 🖼️ Fraction of the graph's own area the live camera currently shows — `1.0` when the whole
    /// graph is framed, `0.0` when none of it is. An empty graph is framed by definition.
    pub fn camera_content_coverage(&self) -> f64 {
        self.content_world_bounds().map_or(1.0, |content| canvas::camera::content_coverage(&content, &self.camera_state(), &self.camera_viewport()))
    }

    /// 🖼️ Moves the camera so the whole graph is framed with the shared screen padding. `false` when
    /// there is no graph to frame.
    pub fn fit_camera_to_content(&mut self) -> bool {
        let Some(content) = self.content_world_bounds() else { return false };
        let camera = canvas::camera::fit_camera(&content, &self.camera_viewport(), canvas::camera::CONTENT_FIT_PADDING_PX);
        self.set_camera(camera.x, camera.y, camera.zoom);
        true
    }

    /// 🖼️ The opening camera law: a STORED camera is adopted only when it already frames the graph
    /// it was stored for; otherwise the graph is fitted. Returns `true` when the fit won — the
    /// caller persists that exactly the way it persists a pan or a zoom gesture.
    pub fn adopt_camera_or_fit(&mut self, x: f64, y: f64, zoom: f64) -> bool {
        let stored = canvas::camera::Camera { x, y, zoom };
        let content = self.content_world_bounds();
        let (camera, fitted) =
            canvas::camera::startup_camera(Some(&stored), content.as_ref(), &self.camera_viewport(), canvas::camera::CONTENT_FIT_PADDING_PX, canvas::camera::CONTENT_FRAMED_MIN_COVERAGE);
        self.set_camera(camera.x, camera.y, camera.zoom);
        fitted
    }

    /// 🖼️ Re-fits a graph that CHANGED under a live camera (an example switch) only when the change
    /// left essentially nothing on screen. A camera the viewer set themselves is never yanked back
    /// for an ordinary edit — see `canvas::camera::CONTENT_REFIT_MAX_COVERAGE`.
    pub fn refit_camera_if_content_left_view(&mut self) -> bool {
        if self.content_world_bounds().is_none() || self.camera_content_coverage() > canvas::camera::CONTENT_REFIT_MAX_COVERAGE {
            return false;
        }
        self.fit_camera_to_content()
    }
    // #endregion 🔖️ContentFraming

    // #region 🔖️MinimapWidget
    /// 🗺️ Toggles the bottom-right flow minimap navigator.
    pub fn set_minimap_widget_visible(&mut self, visible: bool) {
        self.minimap_widget_visible = visible;
    }

    pub(super) fn minimap_widget_content_bounds(&self) -> Option<WorldBox> {
        let pad = ui_styling::metrics::dag::MINIMAP_WIDGET_CONTENT_PAD;
        let content = self.content_world_bounds()?;
        Some(WorldBox { min_x: content.min_x - pad, min_y: content.min_y - pad, max_x: content.max_x + pad, max_y: content.max_y + pad })
    }

    /// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::content_fully_visible` — pure layout math relocated there
    /// (see `.🧬semio/🦑️repo/🎫️tickets/26/08/05/FRAMEWORK-BUILDER-PASSTHROUGHS-APP-COMMANDS-MACRO-WIDGET-EXTRACTION`).
    pub(super) fn minimap_camera_fully_shows_content(&self, content: &WorldBox, viewport_w: u32, viewport_h: u32) -> bool {
        let cam = &self.host_snapshot.camera;
        ui_wgpu::wgpu::minimap::content_fully_visible(&ui_wgpu::wgpu::minimap::MinimapContentBounds { min_x: content.min_x, min_y: content.min_y, max_x: content.max_x, max_y: content.max_y }, viewport_w, viewport_h, cam.x, cam.y, cam.zoom, 12.0)
    }

    /// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::layout` — see `minimap_camera_fully_shows_content` above.
    pub(super) fn minimap_widget_layout(&self, viewport_w: u32, viewport_h: u32) -> Option<MinimapWidgetLayout> {
        if !self.minimap_widget_visible {
            return None;
        }
        let content = self.minimap_widget_content_bounds()?;
        if self.minimap_camera_fully_shows_content(&content, viewport_w, viewport_h) {
            return None;
        }
        let w = ui_styling::metrics::dag::MINIMAP_WIDGET_WIDTH;
        let h = ui_styling::metrics::dag::MINIMAP_WIDGET_HEIGHT;
        let margin = ui_styling::metrics::dag::MINIMAP_WIDGET_MARGIN;
        let ratio = ui_styling::metrics::dag::MINIMAP_WIDGET_MAX_CONTENT_RATIO;
        let cam = &self.host_snapshot.camera;
        let layout =
            ui_wgpu::wgpu::minimap::layout(&ui_wgpu::wgpu::minimap::MinimapContentBounds { min_x: content.min_x, min_y: content.min_y, max_x: content.max_x, max_y: content.max_y }, viewport_w, viewport_h, cam.x, cam.y, cam.zoom, w, h, margin, ratio);
        Some(MinimapWidgetLayout { panel: layout.panel, world_min_x: layout.world_min_x, world_min_y: layout.world_min_y, scale: layout.scale, map_origin_x: layout.map_origin_x, map_origin_y: layout.map_origin_y, viewport: layout.viewport })
    }

    /// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::screen_to_world` — see `minimap_camera_fully_shows_content` above.
    pub(super) fn minimap_widget_screen_to_world(&self, layout: &MinimapWidgetLayout, sx: f64, sy: f64) -> (f64, f64) {
        ui_wgpu::wgpu::minimap::screen_to_world(layout.map_origin_x, layout.map_origin_y, layout.world_min_x, layout.world_min_y, layout.scale, sx, sy)
    }

    /// 🗺️ Thin wrapper over `ui_wgpu::wgpu::minimap::point_in_rect` — see `minimap_camera_fully_shows_content` above.
    pub(super) fn minimap_widget_point_in_rect(rect: (f64, f64, f64, f64), sx: f64, sy: f64) -> bool {
        ui_wgpu::wgpu::minimap::point_in_rect(rect, sx, sy)
    }

    pub(super) fn minimap_widget_pointer_hit(&self, sx: f64, sy: f64) -> Option<(MinimapWidgetLayout, bool)> {
        let layout = self.minimap_widget_layout(self.width, self.height)?;
        if !Self::minimap_widget_point_in_rect(layout.panel, sx, sy) {
            return None;
        }
        let on_viewport = Self::minimap_widget_point_in_rect(layout.viewport, sx, sy);
        Some((layout, on_viewport))
    }

    fn minimap_widget_cursor_hint(&self) -> Option<&'static str> {
        if self.minimap_widget_drag.is_some() {
            return Some("grabbing");
        }
        if let Some((layout, on_viewport)) = self.minimap_widget_pointer_hit(self.last_screen_x, self.last_screen_y) {
            let _ = layout;
            return Some(if on_viewport { "grab" } else { "pointer" });
        }
        None
    }

    pub(super) fn minimap_widget_json(&self) -> Option<Value> {
        let layout = self.minimap_widget_layout(self.width, self.height)?;
        let (x0, y0, x1, y1) = layout.panel;
        Some(semio_framework_pack_json::object([
            ("x".to_string(), Value::from(x0)),
            ("y".to_string(), Value::from(y0)),
            ("width".to_string(), Value::from(x1 - x0)),
            ("height".to_string(), Value::from(y1 - y0)),
            ("cursor".to_string(), Value::from(self.minimap_widget_cursor_hint())),
        ]))
    }

    pub(super) fn paint_minimap_widget(&self, scene: &mut canvas::Scene, viewport_w: u32, viewport_h: u32) {
        let Some(layout) = self.minimap_widget_layout(viewport_w, viewport_h) else {
            return;
        };
        use canvas::{Affine, FillRule, Rect, Stroke};
        use ui_styling::strokes;
        let theme = &self.canvas_theme;
        let aff = Affine::IDENTITY;
        let (px0, py0, px1, py1) = layout.panel;
        let panel = Rect::new(px0, py0, px1, py1);
        scene.fill(FillRule::NonZero, aff, theme.minimap_widget_panel_fill, None, &panel);
        scene.stroke(&Stroke::new(strokes::DAG_MINIMAP_WIDGET_PANEL), aff, theme.minimap_widget_panel_stroke, None, &panel);
        let node_min = ui_styling::metrics::dag::MINIMAP_WIDGET_NODE_MIN_SIZE;
        let lod = DagDrawLod::Minimap;
        for (idx, snapshot_node) in self.host_snapshot.nodes.iter().enumerate() {
            let node = self.node_spec_for_paint(idx, snapshot_node);
            let node = node.as_ref();
            let engine_nid = self.engine_node_id_for_index(idx);
            let is_dimmed = engine_nid.is_some_and(|nid| self.dimmed.contains(&nid));
            let (is_selected, is_highlighted, is_hovered) = engine_nid.map_or((false, false, false), |nid| self.node_interaction_chrome(nid));
            let Some(fill) = dag_node_paint_fill(lod, theme, is_dimmed, is_selected, is_highlighted, is_hovered) else {
                continue;
            };
            let hw = (node.width * layout.scale * 0.5).max(node_min * 0.5);
            let hh = (node.height * layout.scale * 0.5).max(node_min * 0.5);
            let cx = layout.map_origin_x + (node.x - layout.world_min_x) * layout.scale;
            let cy = layout.map_origin_y + (node.y - layout.world_min_y) * layout.scale;
            let rect = Rect::new(cx - hw, cy - hh, cx + hw, cy + hh);
            scene.fill(FillRule::NonZero, aff, fill, None, &rect);
        }
        let (vx0, vy0, vx1, vy1) = layout.viewport;
        let view_rect = Rect::new(vx0, vy0, vx1, vy1);
        let active = self.minimap_widget_hovered || self.minimap_widget_drag.is_some();
        scene.fill(FillRule::NonZero, aff, theme.minimap_widget_viewport_fill, None, &view_rect);
        let stroke_c = if active { theme.minimap_widget_viewport_stroke_hovered } else { theme.minimap_widget_viewport_stroke };
        scene.stroke(&Stroke::new(strokes::DAG_MINIMAP_WIDGET_VIEWPORT), aff, stroke_c, None, &view_rect);
    }
    // #endregion 🔖️MinimapWidget

    // #region 🔖️SelectionAlign
    fn dag_node_world_bounds(node: &DagNodeSpec) -> WorldBox {
        let hw = node.width * 0.5;
        let hh = node.height * 0.5;
        WorldBox { min_x: node.x - hw, min_y: node.y - hh, max_x: node.x + hw, max_y: node.y + hh }
    }

    pub(super) fn selected_snapshot_nodes(&self) -> Vec<(usize, DagNodeSpec)> {
        let ids = self.selected_node_ids();
        ids.into_iter().filter_map(|id| self.host_snapshot.nodes.iter().enumerate().find(|(_, node)| node.id == id).map(|(idx, node)| (idx, node.clone()))).collect()
    }

    pub(super) fn sync_snapshot_node_center_to_engine(&mut self, idx: usize) {
        let node = &self.host_snapshot.nodes[idx];
        let Some(nid) = self.node_id_for_widget_id(&node.id) else {
            return;
        };
        if let Some(engine_node) = self.engine.nodes.get_mut(&nid) {
            engine_node.center = canvas::Point::new(node.x, node.y);
        }
    }

    /// 📦️ Screen-space union bounds of the current node selection for DOM chrome overlays.
    pub fn selection_union_bounds_screen_json(&self) -> String {
        let selected = self.selected_snapshot_nodes();
        if selected.is_empty() {
            return "null".into();
        }
        use canvas::camera::{world_to_screen, Camera as CanvasCamera, Viewport};
        use canvas::Point;
        let pad_world = 4.0 / self.host_snapshot.camera.zoom.max(0.05);
        let mut corners = Vec::new();
        for (_, node) in &selected {
            let hw = node.width * 0.5 + pad_world;
            let hh = node.height * 0.5 + pad_world;
            corners.push(Point::new(node.x - hw, node.y - hh));
            corners.push(Point::new(node.x + hw, node.y + hh));
        }
        let Some(bounds) = world_box_from_points(&corners) else {
            return "null".into();
        };
        let cam = CanvasCamera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom };
        let viewport = Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) };
        let tl = world_to_screen(&cam, &viewport, Point::new(bounds.min_x, bounds.min_y));
        let br = world_to_screen(&cam, &viewport, Point::new(bounds.max_x, bounds.max_y));
        semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("x".to_string(), Value::from(tl.x)),
            ("y".to_string(), Value::from(tl.y)),
            ("width".to_string(), Value::from((br.x - tl.x).max(1.0))),
            ("height".to_string(), Value::from((br.y - tl.y).max(1.0))),
        ]))
    }

    /// 🎯️ Screen-space geometry (canvas-local px) for a live entity in the shell's pick-target
    /// grammar (`domain`: `"node"` | `"handle"` | `"edge"`; `id`: a node's widget id, `"widgetId@port"`
    /// for a handle, or a mirrored edge id — `"*"` picks whichever matching entity's screen anchor is
    /// nearest the viewport center) — powers introduction-demonstration semantic targeting
    /// (`IntroductionPoint::Entity`/`Curve`). Never errors: an unresolved domain/id returns
    /// `{"visible":false}`.
    ///
    /// 🔌️ A port is published as its CONNECTOR rect — the very rect `port_connector_handle_hit`
    /// grabs a wire in, so anything that aims at what the host reports (a demonstration, an
    /// assistive caller, a scripted drag) presses a point that wires.
    pub fn entity_screen_json(&self, domain: &str, id: &str) -> String {
        #[derive(ToValue, FromValue)]
        struct EntityGeometry {
            visible: bool,
            #[value(default, skip_serializing_if = "Option::is_none")]
            x: Option<f64>,
            #[value(default, skip_serializing_if = "Option::is_none")]
            y: Option<f64>,
            #[value(default, skip_serializing_if = "Option::is_none")]
            rect: Option<[f64; 4]>,
            #[value(default, skip_serializing_if = "Option::is_none")]
            polyline: Option<Vec<[f64; 2]>>,
        }
        use canvas::camera::{world_to_screen, Camera as CanvasCamera, Viewport};
        use canvas::Point;
        let unresolved = EntityGeometry { visible: false, x: None, y: None, rect: None, polyline: None };
        let cam = CanvasCamera { x: self.host_snapshot.camera.x, y: self.host_snapshot.camera.y, zoom: self.host_snapshot.camera.zoom };
        let viewport = Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) };
        let viewport_center = (self.width as f64 * 0.5, self.height as f64 * 0.5);

        let world_rect_to_screen = |min_x: f64, min_y: f64, max_x: f64, max_y: f64| -> ([f64; 4], (f64, f64)) {
            let tl = world_to_screen(&cam, &viewport, Point::new(min_x, min_y));
            let br = world_to_screen(&cam, &viewport, Point::new(max_x, max_y));
            ([tl.x, tl.y, (br.x - tl.x).max(1.0), (br.y - tl.y).max(1.0)], ((tl.x + br.x) * 0.5, (tl.y + br.y) * 0.5))
        };
        let handle_world_bounds = |widget_id: &str, port: &str| -> Option<(f64, f64, f64, f64)> {
            let node = self.host_snapshot.nodes.iter().find(|node| node.id == widget_id)?;
            if let Some(index) = node.inputs().iter().position(|candidate| candidate.id == port) {
                return input_port_connector_bounds(node, index);
            }
            let index = node.outputs().iter().position(|candidate| candidate.id == port)?;
            output_port_connector_bounds(node, index)
        };
        let nearest_center_screen = |candidates: &[(f64, f64, f64, f64)]| -> Option<(f64, f64, f64, f64)> {
            let mut best: Option<((f64, f64, f64, f64), f64)> = None;
            for &bounds in candidates {
                let (_, screen_center) = world_rect_to_screen(bounds.0, bounds.1, bounds.2, bounds.3);
                let distance = (screen_center.0 - viewport_center.0).hypot(screen_center.1 - viewport_center.1);
                if best.is_none_or(|(_, best_distance)| distance < best_distance) {
                    best = Some((bounds, distance));
                }
            }
            best.map(|(bounds, _)| bounds)
        };

        let bounds_result: Option<(f64, f64, f64, f64)> = match domain {
            "node" => {
                if id == "*" {
                    let all: Vec<(f64, f64, f64, f64)> = self
                        .host_snapshot
                        .nodes
                        .iter()
                        .map(|node| {
                            let b = Self::dag_node_world_bounds(node);
                            (b.min_x, b.min_y, b.max_x, b.max_y)
                        })
                        .collect();
                    nearest_center_screen(&all)
                } else {
                    self.host_snapshot.nodes.iter().find(|node| node.id == id).map(|node| {
                        let b = Self::dag_node_world_bounds(node);
                        (b.min_x, b.min_y, b.max_x, b.max_y)
                    })
                }
            }
            "handle" => {
                if id == "*" {
                    let mut all: Vec<(f64, f64, f64, f64)> = Vec::new();
                    for node in &self.host_snapshot.nodes {
                        for port in node.inputs().iter().chain(node.outputs().iter()) {
                            if let Some(bounds) = handle_world_bounds(&node.id, &port.id) {
                                all.push(bounds);
                            }
                        }
                    }
                    nearest_center_screen(&all)
                } else {
                    id.split_once('@').and_then(|(widget_id, port)| handle_world_bounds(widget_id, port))
                }
            }
            "edge" => {
                let edge = if id == "*" { self.host_snapshot.edges.first() } else { self.host_snapshot.edges.iter().find(|edge| edge.id == id) };
                let Some(edge) = edge else { return semio_framework_pack_json::to_json_string(&unresolved) };
                let Some((source_widget, source_port)) = edge.source.split_once('@') else { return semio_framework_pack_json::to_json_string(&unresolved) };
                let Some((target_widget, target_port)) = edge.target.split_once('@') else { return semio_framework_pack_json::to_json_string(&unresolved) };
                let Some(source_bounds) = handle_world_bounds(source_widget, source_port) else { return semio_framework_pack_json::to_json_string(&unresolved) };
                let Some(target_bounds) = handle_world_bounds(target_widget, target_port) else { return semio_framework_pack_json::to_json_string(&unresolved) };
                let (_, source_center) = world_rect_to_screen(source_bounds.0, source_bounds.1, source_bounds.2, source_bounds.3);
                let (_, target_center) = world_rect_to_screen(target_bounds.0, target_bounds.1, target_bounds.2, target_bounds.3);
                let midpoint = ((source_center.0 + target_center.0) * 0.5, (source_center.1 + target_center.1) * 0.5);
                if !self.screen_point_is_on_surface(midpoint) {
                    return semio_framework_pack_json::to_json_string(&unresolved);
                }
                return semio_framework_pack_json::to_json_string(&EntityGeometry { visible: true, x: Some(midpoint.0), y: Some(midpoint.1), rect: None, polyline: Some(vec![[source_center.0, source_center.1], [target_center.0, target_center.1]]) });
            }
            _ => None,
        };

        let Some(bounds) = bounds_result else { return semio_framework_pack_json::to_json_string(&unresolved) };
        let (rect, center) = world_rect_to_screen(bounds.0, bounds.1, bounds.2, bounds.3);
        if !self.screen_point_is_on_surface(center) {
            return semio_framework_pack_json::to_json_string(&unresolved);
        }
        semio_framework_pack_json::to_json_string(&EntityGeometry { visible: true, x: Some(center.0), y: Some(center.1), rect: Some(rect), polyline: None })
    }

    /// 👁️ Whether the point a caller would AIM at is actually on the surface this geometry is
    /// published in. The centre is that point — `entity_screen_json` publishes a rect whose centre is
    /// where anything trusting the host presses — so the centre is what `visible` has to be true of.
    ///
    /// 🩸️ `visible` used to mean "the entity EXISTS", never "you can aim at it": a port the camera had
    /// scrolled past was published as `visible: true` with its centre at `x = -4.9` in surface space,
    /// so a caller that trusts the published geometry — a demonstration, an assistive caller, a
    /// scripted drag — pressed OUTSIDE the canvas and the press reached nothing at all. Measured on
    /// 6026 as the wire row's redraw press, twice, with the two aims byte-identical
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️generate-add-flow-wire-quiet-tick-2026-09-14.md`).
    fn screen_point_is_on_surface(&self, point: (f64, f64)) -> bool {
        point.0 >= 0.0 && point.1 >= 0.0 && point.0 <= f64::from(self.width.max(1)) && point.1 <= f64::from(self.height.max(1))
    }

    /// 📐️ Aligns or distributes the current multi-node selection.
    pub fn align_selection(&mut self, mode: &str) -> Result<(), DagError> {
        use canvas::Point;
        let mut selected = self.selected_snapshot_nodes();
        if selected.is_empty() {
            return Ok(());
        }
        match mode {
            "alignLeft" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let min_left = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).min_x).fold(f64::INFINITY, f64::min);
                for (_, node) in &mut selected {
                    node.x = min_left + node.width * 0.5;
                }
            }
            "alignRight" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let max_right = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).max_x).fold(f64::NEG_INFINITY, f64::max);
                for (_, node) in &mut selected {
                    node.x = max_right - node.width * 0.5;
                }
            }
            "alignTop" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let min_top = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).min_y).fold(f64::INFINITY, f64::min);
                for (_, node) in &mut selected {
                    node.y = min_top + node.height * 0.5;
                }
            }
            "alignBottom" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let max_bottom = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).max_y).fold(f64::NEG_INFINITY, f64::max);
                for (_, node) in &mut selected {
                    node.y = max_bottom - node.height * 0.5;
                }
            }
            "alignHorizontal" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let mut corners = Vec::new();
                for (_, node) in &selected {
                    let b = Self::dag_node_world_bounds(node);
                    corners.push(Point::new(b.min_x, b.min_y));
                    corners.push(Point::new(b.max_x, b.max_y));
                }
                let Some(union) = world_box_from_points(&corners) else {
                    return Ok(());
                };
                let center_x = (union.min_x + union.max_x) * 0.5;
                for (_, node) in &mut selected {
                    node.x = center_x;
                }
            }
            "alignVertical" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let mut corners = Vec::new();
                for (_, node) in &selected {
                    let b = Self::dag_node_world_bounds(node);
                    corners.push(Point::new(b.min_x, b.min_y));
                    corners.push(Point::new(b.max_x, b.max_y));
                }
                let Some(union) = world_box_from_points(&corners) else {
                    return Ok(());
                };
                let center_y = (union.min_y + union.max_y) * 0.5;
                for (_, node) in &mut selected {
                    node.y = center_y;
                }
            }
            "distributeHorizontal" => {
                if selected.len() < 3 {
                    return Ok(());
                }
                selected.sort_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap_or(std::cmp::Ordering::Equal));
                let left = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).min_x).fold(f64::INFINITY, f64::min);
                let right = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).max_x).fold(f64::NEG_INFINITY, f64::max);
                let total_width: f64 = selected.iter().map(|(_, node)| node.width).sum();
                let raw_gap = (right - left - total_width) / (selected.len() as f64 - 1.0);
                let gap = if raw_gap.is_finite() { raw_gap.max(DAG_DISTRIBUTE_MIN_GAP) } else { DAG_DISTRIBUTE_MIN_GAP };
                let mut cursor = left;
                for (_, node) in &mut selected {
                    node.x = cursor + node.width * 0.5;
                    cursor += node.width + gap;
                }
            }
            "distributeVertical" => {
                if selected.len() < 3 {
                    return Ok(());
                }
                selected.sort_by(|a, b| a.1.y.partial_cmp(&b.1.y).unwrap_or(std::cmp::Ordering::Equal));
                let top = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).min_y).fold(f64::INFINITY, f64::min);
                let bottom = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).max_y).fold(f64::NEG_INFINITY, f64::max);
                let total_height: f64 = selected.iter().map(|(_, node)| node.height).sum();
                let raw_gap = (bottom - top - total_height) / (selected.len() as f64 - 1.0);
                let gap = if raw_gap.is_finite() { raw_gap.max(DAG_DISTRIBUTE_MIN_GAP) } else { DAG_DISTRIBUTE_MIN_GAP };
                let mut cursor = top;
                for (_, node) in &mut selected {
                    node.y = cursor + node.height * 0.5;
                    cursor += node.height + gap;
                }
            }
            other => return Err(DagError::UnknownAlignMode(other.to_string())),
        }
        for (idx, node) in selected {
            self.host_snapshot.nodes[idx].x = node.x;
            self.host_snapshot.nodes[idx].y = node.y;
            self.sync_snapshot_node_center_to_engine(idx);
        }
        Ok(())
    }
    // #endregion 🔖️SelectionAlign
    // #endregion 🔖️DagHost
}
