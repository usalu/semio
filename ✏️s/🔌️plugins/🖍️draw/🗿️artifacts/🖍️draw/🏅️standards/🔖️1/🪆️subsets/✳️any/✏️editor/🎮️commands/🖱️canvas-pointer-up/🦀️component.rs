//! 🖱️ 🖱️ Draw play app commands command — `canvas-pointer-up`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use crate::editor::draw::commands::canvas_pointer_down::{canvas_point_to_world, draw_gesture, DrawSession};
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-up")]
pub struct CanvasPointerUp {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub shift: bool,
    pub ctrl: bool,
    pub meta: bool,
}

pub fn handle(payload: &CanvasPointerUp, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let (world_x, world_y) = canvas_point_to_world(&config.camera, payload.x, payload.y, payload.width, payload.height);
    let active_utility = config.active_utility_id.clone();
    let emit = session.step_gesture(draw_gesture::Event::PointerUp { utility: active_utility, world: [world_x, world_y], shift: payload.shift, ctrl: payload.ctrl, meta: payload.meta }, document, config);
    Ok(emit)
}
