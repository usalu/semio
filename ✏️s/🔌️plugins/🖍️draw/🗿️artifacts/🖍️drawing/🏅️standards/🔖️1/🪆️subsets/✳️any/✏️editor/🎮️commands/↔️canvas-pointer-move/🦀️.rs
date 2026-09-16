//! 🖱️ 🖱️ Drawing play app commands command — `canvas-pointer-move`.
//!
//! 🎯️ Batched (design L4 / §2 D): the host folds every DOM `pointermove` of one turn into ONE
//! command whose `samples` carry every canvas-pixel position oldest-first; `x`/`y` stay the LAST
//! sample. The Drawing gesture machine holds only a cursor (its marquee/lasso is a start→end
//! rectangle query, its shape/draft previews follow the cursor), so intermediate samples carry no
//! state here: the gesture steps once per batch on the last sample and the activation threshold
//! is judged against that last sample, never an intermediate one.

use crate::editor::drawing::commands::canvas_pointer_down::{canvas_point_to_world, drawing_gesture, DrawingSession, DRAWING_MARQUEE_THRESHOLD_PX};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-move")]
pub struct CanvasPointerMove {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    /// 🧵️ Every pointer sample of this batch as canvas pixels, oldest first. Empty on a legacy
    /// (unbatched) wire — [`CanvasPointerMove::last_sample`] then yields `(x, y)`.
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

    /// 🧵️ The newest sample of the batch (`[x, y]` on a legacy wire).
    pub fn last_sample(&self) -> [f64; 2] {
        self.samples.last().copied().unwrap_or([self.x, self.y])
    }
}

pub fn handle(payload: &CanvasPointerMove, doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    let [x, y] = payload.last_sample();
    let (world_x, world_y) = canvas_point_to_world(&session.window_config.viewport, x, y, payload.width, payload.height);
    let world = [world_x, world_y];
    if session.gesture.matches("idle") {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("drawing.gesture.retained-route"), "idle hover requires the retained Drawing tree-query owner"));
    }
    let marquee_threshold_world = DRAWING_MARQUEE_THRESHOLD_PX / session.window_config.viewport.zoom.max(1e-6);
    let emit = session.step_gesture(drawing_gesture::Event::PointerMove { world, marquee_threshold_world }, document, cfg.snapshot);
    Ok(emit)
}
