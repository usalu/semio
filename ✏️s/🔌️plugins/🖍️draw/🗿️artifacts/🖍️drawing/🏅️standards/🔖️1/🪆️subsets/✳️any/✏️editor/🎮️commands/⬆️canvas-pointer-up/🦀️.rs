//! 🖱️ 🖱️ Drawing play app commands command — `canvas-pointer-up`.

use crate::editor::drawing::commands::canvas_pointer_down::{canvas_point_to_world, DrawingPointer, DrawingSession};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::DrawingSnapshot;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "canvas-pointer-up")]
pub struct CanvasPointerUp {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub shift: bool,
    #[value(default)]
    pub alt: bool,
    pub ctrl: bool,
    pub meta: bool,
    /// 🚫️ `true` when the host closed the gesture without a release (pointer left the canvas,
    /// capture lost): a live marquee/shape drag is dropped and NOTHING is selected, picked or committed.
    #[value(default)]
    pub cancelled: bool,
}

/// ⬆️ A cancelled release aborts the live gesture with zero trace; a real release goes to the canvas tool, which commits a
/// drag or a shape as one transaction (a selection release only arms the bounded pick, published by the retained owner).
pub fn handle(payload: &CanvasPointerUp, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    if payload.cancelled {
        return Ok(session.cancel());
    }
    let (world_x, world_y) = canvas_point_to_world(&session.window_config.viewport, payload.x, payload.y, payload.width, payload.height);
    let pointer = DrawingPointer { utility: session.active_utility_id.clone(), world: [world_x, world_y], shift: payload.shift, alt: payload.alt, ctrl: payload.ctrl, meta: payload.meta };
    let base = session.tool_base(doc);
    Ok(session.release("canvasPointerUp", pointer, base)?.unwrap_or_default())
}
