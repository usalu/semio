//! 🖱️ 2D-window utility — Select: the default pointer utility (rectangle/lasso marquee plus click
//! picking). Bound only by the 2D board window; the 3D world window leads with the transform gumball
//! instead, so this node stays under `◻2d`.

use semio_framework_plugin::{LocalizedLabel, UtilityCategory, UtilityDefinition};

pub const UTILITY_ID: &str = "select";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`.
pub fn definition(label: LocalizedLabel) -> UtilityDefinition {
    UtilityDefinition { category: Some(UtilityCategory::Selection), ..UtilityDefinition::new(UTILITY_ID, label, "mouse-pointer") }
}
