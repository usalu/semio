//! 🖱️ 🖱️ Draw play app commands command — `canvas-pointer-move`.

use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use crate::editor::draw::commands::canvas_pointer_down::{canvas_point_to_world, draw_gesture, interaction_hover_effect, resolve_point_pick, DrawSession, DRAW_MARQUEE_THRESHOLD_PX};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-move")]
pub struct CanvasPointerMove {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub async fn handle(payload: &CanvasPointerMove, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let (world_x, world_y) = canvas_point_to_world(&config.camera, payload.x, payload.y, payload.width, payload.height);
    let world = [world_x, world_y];
    if session.gesture.matches("idle") {
        let include_control_points = config.active_utility_id == "selectDirect";
        let hovered_id = resolve_point_pick(document, &config.camera, world, include_control_points);
        let mut emit = Emit::default();
        emit.effects.push(interaction_hover_effect(&hovered_id.into_iter().collect::<Vec<_>>()));
        return Ok(emit);
    }
    let marquee_threshold_world = DRAW_MARQUEE_THRESHOLD_PX / config.camera.zoom.max(1e-6);
    let emit = session.step_gesture(draw_gesture::Event::PointerMove { world, marquee_threshold_world }, document, config);
    Ok(emit)
}
