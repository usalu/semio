//! 🖱️ 🖱️ Draw play app commands command — `canvas-pointer-move`.

use crate::apps::draw::config::{DrawConfig, DrawConfigMutation};
use crate::artifacts::draw::schema::{create_draw_path_layer, create_draw_trace_layer, draw_layer_world_bounds, draw_transform_to_matrix, find_draw_layer, flatten_draw_layers, layer_base, layer_id, layer_to_path_segments};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawCamera, DrawSnapshot, DrawLayerNode, PathSegment};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use crate::apps::draw::commands::canvas_pointer_down::{best_pick_layer_id, canvas_point_to_world, draw_gesture, finish_gesture_emit, resolve_pick_targets_at, DrawSession, DRAW_MARQUEE_THRESHOLD_PX, DRAW_PICK_TOLERANCE_PX};
use serde::{Deserialize, Serialize};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-move")]
pub struct CanvasPointerMove {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub fn handle(payload: &CanvasPointerMove, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let mut config = cfg.snapshot.clone();
    let (world_x, world_y) = canvas_point_to_world(&config.camera, payload.x, payload.y, payload.width, payload.height);
    let world = [world_x, world_y];
    if session.gesture.matches("idle") {
        let include_control_points = config.active_utility_id == "selectDirect";
        let tolerance = DRAW_PICK_TOLERANCE_PX / config.camera.zoom.max(1e-6);
        let hovered_id = best_pick_layer_id(&resolve_pick_targets_at(document, world, tolerance, include_control_points));
        if hovered_id == config.hovered_id {
            return Ok(Emit::default());
        }
        return Ok(Emit::config(vec![DrawConfigMutation::SetHovered { id: hovered_id }]));
    }
    let marquee_threshold_world = DRAW_MARQUEE_THRESHOLD_PX / config.camera.zoom.max(1e-6);
    let emit = session.step_gesture(draw_gesture::Event::PointerMove { world, marquee_threshold_world }, document, &mut config);
    Ok(finish_gesture_emit(emit, cfg.snapshot, &config))
}
