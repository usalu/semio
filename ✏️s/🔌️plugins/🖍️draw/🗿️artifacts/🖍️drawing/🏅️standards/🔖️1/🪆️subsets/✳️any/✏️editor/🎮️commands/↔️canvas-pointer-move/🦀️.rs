//! 🖱️ 🖱️ Drawing play app commands command — `canvas-pointer-move`.
//!
//! 🎯️ Pointer batches retain every sample. Lasso routes consume one sample per retained turn;
//! hover, shape and draft cursor projections consume the newest position.

use crate::editor::drawing::commands::canvas_pointer_down::{canvas_point_to_world, DrawingSession};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::DrawingSnapshot;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "canvas-pointer-move")]
pub struct CanvasPointerMove {
    #[value(default)]
    pub shift: bool,
    #[value(default)]
    pub alt: bool,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    /// 🧵️ Every pointer sample of this batch, oldest first; an empty batch uses the current x/y.
    #[value(default)]
    pub samples: Vec<[f64; 2]>,
}

impl CanvasPointerMove {
    /// 🧵️ The batch as canvas samples, oldest first — never empty: an absent/empty `samples`
    /// degrades to the single `[x, y]`.
    pub fn samples_or_last(&self) -> Vec<[f64; 2]> {
        if self.samples.is_empty() {
            vec![[self.x, self.y]]
        } else {
            self.samples.clone()
        }
    }

    /// 🧵️ The newest sample of the batch, or the current x/y.
    pub fn last_sample(&self) -> [f64; 2] {
        self.samples.last().copied().unwrap_or([self.x, self.y])
    }
}

/// ↔️ One pointer batch drives the canvas tool to its newest sample with the live transform modifiers.
pub fn handle(payload: &CanvasPointerMove, _doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let [x, y] = payload.last_sample();
    let (world_x, world_y) = canvas_point_to_world(&session.window_config.viewport, x, y, payload.width, payload.height);
    if session.tool.at_rest() {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("drawing.gesture.retained-route"), "idle hover requires the retained Drawing tree-query owner"));
    }
    session.sample([world_x, world_y], payload.shift, payload.alt)
}
