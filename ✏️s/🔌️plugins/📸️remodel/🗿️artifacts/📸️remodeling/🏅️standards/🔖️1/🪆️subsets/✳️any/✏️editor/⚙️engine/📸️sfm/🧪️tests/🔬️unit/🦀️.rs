use semio_s_artifact_remodel_remodeling_images::editor::remodeling::engine::step_ceiling;
use super::*;

/// 🧪️ Per-point `(camera-0 pixel, camera-1 pixel)` accumulator shared by a few two-view tests below.
type CorrByPoint = HashMap<usize, (Option<[f64; 2]>, Option<[f64; 2]>)>;

#[path = "🔀️geometry-tests/🦀️.rs"]
mod geometry_tests;

#[path = "🧭️incremental-tests/🦀️.rs"]
mod incremental_tests;

#[path = "🌐️global-and-prior-tests/🦀️.rs"]
mod global_and_prior_tests;

#[path = "🎯️five-point-and-diagnostics/🦀️.rs"]
mod five_point_and_diagnostics;
use five_point_and_diagnostics::*;

// #region 🔖️TwoViewFivePointTests
use std::collections::HashMap;

// #endregion 🔖️TwoViewFivePointTests
