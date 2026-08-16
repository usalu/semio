//! 🔄️ 3D-window utility — the transform gumball, exposed as the three grouped handles
//! `move`/`rotate`/`scale`. One node rather than three, because they are one concept with one
//! `group: "transform"` collapse: the utility bar renders them as a single dropdown, and
//! `puzzle5d_transform_handle` maps whichever is active onto the gumball mode the world engine draws.
//! Bound only by the 3D world window — the 2D board window leads with `select` instead.

use semio_framework_plugin::{LocalizedLabel, UtilityDefinition};

pub const MOVE_UTILITY_ID: &str = "move";
pub const ROTATE_UTILITY_ID: &str = "rotate";
pub const SCALE_UTILITY_ID: &str = "scale";
/// 🔗️ The utility-bar group all three handles collapse into.
const TRANSFORM_GROUP: &str = "transform";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`.
pub fn move_definition() -> UtilityDefinition {
    UtilityDefinition { group: Some(TRANSFORM_GROUP.into()), ..UtilityDefinition::new(MOVE_UTILITY_ID, LocalizedLabel::native("Move", "Verschieben"), "move") }
}

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`.
pub fn rotate_definition() -> UtilityDefinition {
    UtilityDefinition { group: Some(TRANSFORM_GROUP.into()), ..UtilityDefinition::new(ROTATE_UTILITY_ID, LocalizedLabel::native("Rotate", "Drehen"), "rotate-cw") }
}

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle5d::create_puzzle5d_app`.
pub fn scale_definition() -> UtilityDefinition {
    UtilityDefinition { group: Some(TRANSFORM_GROUP.into()), ..UtilityDefinition::new(SCALE_UTILITY_ID, LocalizedLabel::native("Scale", "Skalieren"), "maximize-2") }
}
