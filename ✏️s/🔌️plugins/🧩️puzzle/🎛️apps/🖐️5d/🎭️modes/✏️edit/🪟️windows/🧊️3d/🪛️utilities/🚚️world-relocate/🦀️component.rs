//! 🚚️ 3D-window utility — Relocate: drags one part to a new world origin and auto-fastens it to every
//! grip that lands within the proximity radius. Bound only by the 3D world window (the flat board has
//! no world origin to drag).

use semio_framework_plugin::{LocalizedLabel, UtilityDefinition};

pub const UTILITY_ID: &str = "worldRelocate";

/// 🧱️ Stitched into the app manifest by `crate::apps::puzzle5d::create_puzzle5d_app`.
pub fn definition() -> UtilityDefinition {
    UtilityDefinition::new(UTILITY_ID, LocalizedLabel::native("Relocate", "Verlagern"), "relocate-3d")
}
