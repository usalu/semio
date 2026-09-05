//! 🎥️ Edit-mode window option — the world projection picker (orthographic/one-two-three-point plus
//! the cardinal/free orientation and its parameters), delegated wholesale to the framework's shared
//! `world3d_projection_measures` builder. Shared by every window instance of the one puzzle3d window
//! kind; each instance keeps its own `Puzzle3dCamera::projection` (see `🦀️config.rs`).

use crate::editor::puzzle3d::config::Puzzle3dRuntime;
use crate::editor::puzzle3d::puzzle3d_action;
use semio_framework_plugin::{world3d_projection_measures, WindowMeasure};

/// 🧭️ The projection select + its parameter sliders for the materialized window's camera.
/// 🔗️ `world3d_projection_measures` is a framework helper still bound to `serde_json::Value`
/// (framework file, out of this crate's remit); `puzzle3d_action` takes the DSL-side `dsl::
/// os_pack::json::Value`, so this closure bridges the unavoidable seam via the framework's own
/// pre-existing `DslValue: From<&serde_json::Value>` impl (see the `☀️sun` sibling option's
/// identical note) rather than widening `puzzle3d_action`'s signature.
pub fn measure(runtime: &Puzzle3dRuntime) -> WindowMeasure {
    world3d_projection_measures("puzzle3d", &runtime.camera.projection, |action, args| puzzle3d_action(action, args.map(|value| dsl::os_pack::json::from_dsl_value(&dsl::DslValue::from(&value)))))
}
