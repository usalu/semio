//! 🎥️ 3D-window option — the world projection picker (orthographic / axonometric / oblique /
//! one-two-three-point / curvilinear plus their parameters), delegated wholesale to the framework's
//! shared `world3d_projection_measures` builder over this pane's own `Puzzle5dCamera3d::projection`.

use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::puzzle5d_action;
use semio_framework_plugin::{world3d_projection_measures, WindowMeasure};

/// 🧭️ The projection select plus its parameter sliders for this window instance's camera.
/// 🔗️ `world3d_projection_measures` is a framework helper still bound to `serde_json::Value`
/// (framework file, out of this crate's remit) while `puzzle5d_action` takes the DSL-side
/// `dsl::os_pack::json::Value`, so this closure bridges the seam through the framework's own
/// `DslValue: From<&serde_json::Value>` impl rather than widening `puzzle5d_action`'s signature —
/// the identical bridge `☑️options/☀️sun` already carries.
pub fn measure(runtime: &Puzzle5dRuntime) -> WindowMeasure {
    world3d_projection_measures("puzzle5d", &runtime.camera3d.projection, |action, args| puzzle5d_action(action, args.map(|value| dsl::os_pack::json::from_dsl_value(&dsl::DslValue::from(&value)))))
}
