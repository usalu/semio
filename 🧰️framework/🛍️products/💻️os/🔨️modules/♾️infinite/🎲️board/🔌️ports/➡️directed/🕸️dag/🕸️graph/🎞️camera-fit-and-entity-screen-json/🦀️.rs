//! 🎞️ Camera fit and entity screen JSON of the DAG graph: content coverage, fitting and refitting the camera, the selection union bounds, the entity screen JSON and the retirement of both states. See [`super`].

use super::*;

impl DagViewportState {
    fn camera_viewport(&self) -> canvas::camera::Viewport {
        canvas::camera::Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) }
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
}

/// 🖼️ Fraction of the graph's own area the live camera currently shows — `1.0` when the whole
/// graph is framed, `0.0` when none of it is. An empty graph is framed by definition.
pub fn camera_content_coverage(viewport: &DagViewportState, graph: &DagGraphState) -> f64 {
    graph.content_world_bounds().map_or(1.0, |content| canvas::camera::content_coverage(&content, &graph.camera_state(), &viewport.camera_viewport()))
}

/// 🖼️ Moves the camera so the whole graph is framed with the shared screen padding. `false` when
/// there is no graph to frame.
pub fn fit_camera_to_content(viewport: &mut DagViewportState, graph: &mut DagGraphState) -> bool {
    let Some(content) = graph.content_world_bounds() else { return false };
    let camera = canvas::camera::fit_camera(&content, &viewport.camera_viewport(), canvas::camera::CONTENT_FIT_PADDING_PX);
    graph.set_camera(camera.x, camera.y, camera.zoom);
    true
}

/// 🖼️ The opening camera law: a STORED camera is adopted only when it already frames the graph
/// it was stored for; otherwise the graph is fitted. Returns `true` when the fit won — the
/// caller persists that exactly the way it persists a pan or a zoom gesture.
pub fn adopt_camera_or_fit(viewport: &mut DagViewportState, graph: &mut DagGraphState, x: f64, y: f64, zoom: f64) -> bool {
    let stored = canvas::camera::Camera { x, y, zoom };
    let content = graph.content_world_bounds();
    let (camera, fitted) =
        canvas::camera::startup_camera(Some(&stored), content.as_ref(), &viewport.camera_viewport(), canvas::camera::CONTENT_FIT_PADDING_PX, canvas::camera::CONTENT_FRAMED_MIN_COVERAGE);
    graph.set_camera(camera.x, camera.y, camera.zoom);
    fitted
}

/// 🖼️ Re-fits a graph that CHANGED under a live camera (an example switch) only when the change
/// left essentially nothing on screen. A camera the viewer set themselves is never yanked back
/// for an ordinary edit — see `canvas::camera::CONTENT_REFIT_MAX_COVERAGE`.
pub fn refit_camera_if_content_left_view(viewport: &mut DagViewportState, graph: &mut DagGraphState) -> bool {
    if graph.content_world_bounds().is_none() || camera_content_coverage(viewport, graph) > canvas::camera::CONTENT_REFIT_MAX_COVERAGE {
        return false;
    }
    fit_camera_to_content(viewport, graph)
}

// #endregion 🔖️ContentFraming

/// 📦️ Screen-space union bounds of the current node selection for DOM chrome overlays.
pub fn selection_union_bounds_screen_json(viewport_state: &DagViewportState, graph: &DagGraphState) -> String {
    let selected = graph.selected_snapshot_nodes();
    if selected.is_empty() {
        return "null".into();
    }
    use canvas::camera::{world_to_screen, Camera as CanvasCamera, Viewport};
    use canvas::Point;
    let pad_world = 4.0 / graph.host_snapshot.camera.zoom.max(0.05);
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
    let cam = CanvasCamera { x: graph.host_snapshot.camera.x, y: graph.host_snapshot.camera.y, zoom: graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: viewport_state.width.max(1), height: viewport_state.height.max(1), dpr: viewport_state.dpr.max(1.0) };
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
pub fn entity_screen_json(viewport_state: &DagViewportState, graph: &DagGraphState, domain: &str, id: &str) -> String {
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
    let cam = CanvasCamera { x: graph.host_snapshot.camera.x, y: graph.host_snapshot.camera.y, zoom: graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: viewport_state.width.max(1), height: viewport_state.height.max(1), dpr: viewport_state.dpr.max(1.0) };
    let viewport_center = (viewport_state.width as f64 * 0.5, viewport_state.height as f64 * 0.5);

    let world_rect_to_screen = |min_x: f64, min_y: f64, max_x: f64, max_y: f64| -> ([f64; 4], (f64, f64)) {
        let tl = world_to_screen(&cam, &viewport, Point::new(min_x, min_y));
        let br = world_to_screen(&cam, &viewport, Point::new(max_x, max_y));
        ([tl.x, tl.y, (br.x - tl.x).max(1.0), (br.y - tl.y).max(1.0)], ((tl.x + br.x) * 0.5, (tl.y + br.y) * 0.5))
    };
    let handle_world_bounds = |widget_id: &str, port: &str| -> Option<(f64, f64, f64, f64)> {
        let node = graph.host_snapshot.nodes.iter().find(|node| node.id == widget_id)?;
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
                let all: Vec<(f64, f64, f64, f64)> = graph
                    .host_snapshot
                    .nodes
                    .iter()
                    .map(|node| {
                        let b = dag_node_world_bounds(node);
                        (b.min_x, b.min_y, b.max_x, b.max_y)
                    })
                    .collect();
                nearest_center_screen(&all)
            } else {
                graph.host_snapshot.nodes.iter().find(|node| node.id == id).map(|node| {
                    let b = dag_node_world_bounds(node);
                    (b.min_x, b.min_y, b.max_x, b.max_y)
                })
            }
        }
        "handle" => {
            if id == "*" {
                let mut all: Vec<(f64, f64, f64, f64)> = Vec::new();
                for node in &graph.host_snapshot.nodes {
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
            let edge = if id == "*" { graph.host_snapshot.edges.first() } else { graph.host_snapshot.edges.iter().find(|edge| edge.id == id) };
            let Some(edge) = edge else { return semio_framework_pack_json::to_json_string(&unresolved) };
            let Some((source_widget, source_port)) = edge.source.split_once('@') else { return semio_framework_pack_json::to_json_string(&unresolved) };
            let Some((target_widget, target_port)) = edge.target.split_once('@') else { return semio_framework_pack_json::to_json_string(&unresolved) };
            let Some(source_bounds) = handle_world_bounds(source_widget, source_port) else { return semio_framework_pack_json::to_json_string(&unresolved) };
            let Some(target_bounds) = handle_world_bounds(target_widget, target_port) else { return semio_framework_pack_json::to_json_string(&unresolved) };
            let (_, source_center) = world_rect_to_screen(source_bounds.0, source_bounds.1, source_bounds.2, source_bounds.3);
            let (_, target_center) = world_rect_to_screen(target_bounds.0, target_bounds.1, target_bounds.2, target_bounds.3);
            let midpoint = ((source_center.0 + target_center.0) * 0.5, (source_center.1 + target_center.1) * 0.5);
            if !viewport_state.screen_point_is_on_surface(midpoint) {
                return semio_framework_pack_json::to_json_string(&unresolved);
            }
            return semio_framework_pack_json::to_json_string(&EntityGeometry { visible: true, x: Some(midpoint.0), y: Some(midpoint.1), rect: None, polyline: Some(vec![[source_center.0, source_center.1], [target_center.0, target_center.1]]) });
        }
        _ => None,
    };

    let Some(bounds) = bounds_result else { return semio_framework_pack_json::to_json_string(&unresolved) };
    let (rect, center) = world_rect_to_screen(bounds.0, bounds.1, bounds.2, bounds.3);
    if !viewport_state.screen_point_is_on_surface(center) {
        return semio_framework_pack_json::to_json_string(&unresolved);
    }
    semio_framework_pack_json::to_json_string(&EntityGeometry { visible: true, x: Some(center.0), y: Some(center.1), rect: Some(rect), polyline: None })
}

impl RetireOwned for DagViewportState {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{canvas_theme,width,height,dpr,last_screen_x,last_screen_y,wheel_zoom_active,wheel_zoom_render_lod,automatic_lod,forced_draw_lod,grid_visible,grid_snap_enabled,grid_factor,pan_anchor}=self;sequence(vec![deferred(canvas_theme),deferred(width),deferred(height),deferred(dpr),deferred(last_screen_x),deferred(last_screen_y),deferred(wheel_zoom_active),deferred(wheel_zoom_render_lod),deferred(automatic_lod),deferred(forced_draw_lod),deferred(grid_visible),deferred(grid_snap_enabled),deferred(grid_factor),deferred(pan_anchor)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.canvas_theme),deferred_birth_bytes_for(&self.width),deferred_birth_bytes_for(&self.height),deferred_birth_bytes_for(&self.dpr),deferred_birth_bytes_for(&self.last_screen_x),deferred_birth_bytes_for(&self.last_screen_y),deferred_birth_bytes_for(&self.wheel_zoom_active),deferred_birth_bytes_for(&self.wheel_zoom_render_lod),deferred_birth_bytes_for(&self.automatic_lod),deferred_birth_bytes_for(&self.forced_draw_lod),deferred_birth_bytes_for(&self.grid_visible),deferred_birth_bytes_for(&self.grid_snap_enabled),deferred_birth_bytes_for(&self.grid_factor),deferred_birth_bytes_for(&self.pan_anchor)])}
 fn controlled_retirement_supported()->bool{true}
}

impl RetireOwned for DagGraphState {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{host_snapshot,engine,node_id_map,handle_key_map,handle_port_shape,handle_port_visible,edge_id_map,edge_engine_ids,edge_route_style,pending_graph_edits,journal_refusal}=self;sequence(vec![deferred(host_snapshot),deferred(engine),deferred(node_id_map),deferred(handle_key_map),deferred(handle_port_shape),deferred(handle_port_visible),deferred(edge_id_map),deferred(edge_engine_ids),deferred(edge_route_style),deferred(pending_graph_edits),deferred(journal_refusal)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.host_snapshot),deferred_birth_bytes_for(&self.engine),deferred_birth_bytes_for(&self.node_id_map),deferred_birth_bytes_for(&self.handle_key_map),deferred_birth_bytes_for(&self.handle_port_shape),deferred_birth_bytes_for(&self.handle_port_visible),deferred_birth_bytes_for(&self.edge_id_map),deferred_birth_bytes_for(&self.edge_engine_ids),deferred_birth_bytes_for(&self.edge_route_style),deferred_birth_bytes_for(&self.pending_graph_edits),deferred_birth_bytes_for(&self.journal_refusal)])}
 fn controlled_retirement_supported()->bool{true}
}
