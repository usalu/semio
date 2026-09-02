//! 🖱️ 🖱️ Drawing play app commands command — `canvas-pointer-move`.

use crate::artifacts::drawing::op::DrawingMutation;
use crate::artifacts::drawing::DrawingSnapshot;
use crate::editor::drawing::commands::canvas_pointer_down::{canvas_point_to_world, drawing_gesture, DrawingSession, DRAWING_MARQUEE_THRESHOLD_PX};
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-move")]
pub struct CanvasPointerMove {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub fn handle(payload: &CanvasPointerMove, doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, DrawingConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let (world_x, world_y) = canvas_point_to_world(&config.camera, payload.x, payload.y, payload.width, payload.height);
    let world = [world_x, world_y];
    if session.gesture.matches("idle") {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("drawing.gesture.retained-route"), "idle hover requires the retained Drawing tree-query owner"));
    }
    let marquee_threshold_world = DRAWING_MARQUEE_THRESHOLD_PX / config.camera.zoom.max(1e-6);
    let emit = session.step_gesture(drawing_gesture::Event::PointerMove { world, marquee_threshold_world }, document, config);
    Ok(emit)
}
