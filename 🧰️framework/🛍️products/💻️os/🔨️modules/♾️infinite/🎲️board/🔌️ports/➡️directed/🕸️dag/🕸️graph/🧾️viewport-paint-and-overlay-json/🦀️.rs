//! 🧾️ Viewport paint and overlay JSON of the DAG graph: the viewport state methods, the node overlay JSON and the slider overlay state JSON. See [`super`].

use super::*;

impl DagViewportState {
    pub fn paint_node_handles_for_spec(&self, scene: &mut canvas::Scene, aff: &canvas::Affine, cam: &canvas::camera::Camera, node: &DagNodeSpec, chrome: &DagNodePaintChrome) {
        use canvas::FillRule;
        use canvas::Stroke;
        use canvas::{Circle, Point};
        use graph::NodeShape;
        use semio_framework_graph_drawing::routing::{handle_outward_at_node_rim, handle_position_on_rectangle};

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
                    scene.fill(FillRule::NonZero, *aff, fill, None, &handle_cap_fill_path(handle_center, out, DAG_HANDLE_WORLD_RADIUS, port.shape));
                }
                scene.stroke(&Stroke::new(handle_stroke_px), *aff, stroke_c, None, &handle_cap_stroke_path(handle_center, out, DAG_HANDLE_WORLD_RADIUS, port.shape));
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
}

/// 🖼️ Screen-node overlay rects in CSS pixel space for DOM media layers.
pub fn dag_node_overlays_json(viewport_state: &DagViewportState, graph: &DagGraphState) -> Result<String, DagError> {
    use canvas::camera::{world_to_screen, Camera as CanvasCamera, Viewport};
    use canvas::Point;
    let cam = CanvasCamera { x: graph.host_snapshot.camera.x, y: graph.host_snapshot.camera.y, zoom: graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: viewport_state.width.max(1), height: viewport_state.height.max(1), dpr: viewport_state.dpr.max(1.0) };
    let mut overlays = Vec::new();
    for node in &graph.host_snapshot.nodes {
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

/// 🎚️ Slider track anchors for the HTML slider overlay — the ONE row that carries a knob's value.
///
/// 🔢 `value` is published exactly once and the overlay renders it twice: as the control's own
/// value (hence `aria-valuenow`) and as the readout beside the track. The readout used to be
/// painted on the GPU instead, which made it a second producer that could only move when a frame
/// was drawn — a released knob then read `10` to a screen reader and `0.0` on the canvas. The
/// readout's typography rides along as `fontScreenPx`/`gapScreenPx` so the DOM lands where the
/// paint did without re-deriving the board's own metrics
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️slider-reevaluation-correctness-2026-09-15.md`).
pub fn slider_overlay_state_json(viewport: &DagViewportState, graph: &DagGraphState) -> Result<String, DagError> {
    let cam = &graph.host_snapshot.camera;
    let mut sliders: Vec<Value> = Vec::new();
    for (idx, snapshot_node) in graph.host_snapshot.nodes.iter().enumerate() {
        let node = graph.node_spec_for_paint(idx, snapshot_node);
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
        ("width".to_string(), Value::from(viewport.width)),
        ("height".to_string(), Value::from(viewport.height)),
        ("sliders".to_string(), Value::Array(sliders)),
    ])))
}
