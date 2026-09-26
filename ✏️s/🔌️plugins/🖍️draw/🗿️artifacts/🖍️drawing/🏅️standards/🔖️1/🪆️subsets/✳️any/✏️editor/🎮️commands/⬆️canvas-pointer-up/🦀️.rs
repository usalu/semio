//! 🖱️ 🖱️ Drawing play app commands command — `canvas-pointer-up`.

use crate::editor::drawing::commands::canvas_pointer_down::{canvas_point_to_world, drawing_gesture, DrawingSession};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-up")]
pub struct CanvasPointerUp {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub shift: bool,
    pub ctrl: bool,
    pub meta: bool,
    /// 🚫️ `true` when the host closed the gesture without a release (pointer left the canvas,
    /// capture lost): a live marquee/shape drag is dropped and NOTHING is selected, picked or committed.
    #[value(default)]
    pub cancelled: bool,
}

/// 🚫️ A cancelled release: a live marquee/shape drag takes the statechart's `Escape` edge back to
/// `idle` (no effect is sinked), an idle canvas or a click-sequenced draft is left untouched — a
/// real release in `drafting` is a no-op too, and a cancel must never fall back to a pick.
pub fn cancel_gesture(session: &mut DrawingSession, document: &DrawingSnapshot, config: &NoConfig) -> Emit<DrawingMutation, NoConfigMutation> {
    if session.gesture.matches("marqueeing") || session.gesture.matches("shape_dragging") || session.gesture.matches("moving_layer") {
        return session.step_gesture(drawing_gesture::Event::Escape, document, config);
    }
    Emit::default()
}

pub fn handle(payload: &CanvasPointerUp, doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    if payload.cancelled {
        return Ok(cancel_gesture(session, document, cfg.snapshot));
    }
    let (world_x, world_y) = canvas_point_to_world(&session.window_config.viewport, payload.x, payload.y, payload.width, payload.height);
    let active_utility = session.active_utility_id.clone();
    let emit = session.step_gesture(drawing_gesture::Event::PointerUp { utility: active_utility, world: [world_x, world_y], shift: payload.shift, ctrl: payload.ctrl, meta: payload.meta }, document, cfg.snapshot);
    Ok(emit)
}
