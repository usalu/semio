//! 🚚️ Main-window utility — Relocate: drag an object to a new world position and auto-attract it
//! onto whatever compatible vortex ends up within the proximity radius (see `🎮️commands/🔄️translate-selection`'s
//! `world_relocate`). It carries no Utility Options of its own — the proximity radius it honours is a
//! whole-app setting on the ⚙️settings panel.

use semio_framework_plugin::{LocalizedLabel, UtilityDefinition};

pub const UTILITY_ID: &str = "worldRelocate";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle3d::create_puzzle3d_app`.
pub async fn definition() -> UtilityDefinition {
    UtilityDefinition::new(UTILITY_ID, LocalizedLabel::native("Relocate", "Verlagern"), "relocate-3d")
}
