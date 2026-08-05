//! 🖌️ Overview-window utility — Brush: places compatible nodes into open handle slots. Its live
//! options (suggestion offset, per-kind distribution, candidate picker) are the mode-level
//! `🎚️options/🖌️brush` measure group, tagged with this utility's id.

use semio_framework_plugin::{LocalizedLabel, UtilityCategory, UtilityDefinition};

pub const UTILITY_ID: &str = "brush";

/// 🧱️ Stitched into the app manifest by `crate::apps::puzzle2d::create_puzzle2d_app`.
pub fn definition(label: LocalizedLabel) -> UtilityDefinition {
    UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new(UTILITY_ID, label, "paintbrush") }
}
