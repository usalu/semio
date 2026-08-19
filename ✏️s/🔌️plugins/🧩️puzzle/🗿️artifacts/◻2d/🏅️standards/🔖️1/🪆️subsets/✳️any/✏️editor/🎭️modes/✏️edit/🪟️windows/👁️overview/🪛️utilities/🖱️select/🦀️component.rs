//! 🖱️ Overview-window utility — Select: the default pointer utility (rectangle/lasso marquee plus
//! click picking). Together with `🖌️brush` it forms this window's entire exclusive utility set, so it
//! carries `group: None` and renders as its own flat utility-bar icon, never a collapsed dropdown.

use semio_framework_plugin::{LocalizedLabel, UtilityCategory, UtilityDefinition};

pub const UTILITY_ID: &str = "select";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle2d::create_puzzle2d_app`.
pub async fn definition(label: LocalizedLabel) -> UtilityDefinition {
    UtilityDefinition { category: Some(UtilityCategory::Selection), ..UtilityDefinition::new(UTILITY_ID, label, "mouse-pointer") }
}
