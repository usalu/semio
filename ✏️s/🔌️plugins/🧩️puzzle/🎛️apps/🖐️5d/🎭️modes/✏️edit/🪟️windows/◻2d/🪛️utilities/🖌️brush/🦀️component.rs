//! 🖌️ Brush utility — places compatible parts into open grip slots, in both projections at once.
//!
//! 🔁️ SHARED BY BOTH WINDOWS: the 2D board window and the 3D world window bind the identical `brush`
//! utility id, so the definition is declared ONCE here (under the 2D window, the first binder) and
//! `🪟️windows/🧊️3d`'s own `definition()` references this same module rather than duplicating it.
//! Its live options are the mode-level `🎭️modes/✏️edit/🎚️options/🖌️brush` measure group, tagged with
//! this utility's id.

use semio_framework_plugin::{LocalizedLabel, UtilityDefinition};

pub const UTILITY_ID: &str = "brush";

/// 🧱️ Stitched into the app manifest by `crate::apps::puzzle5d::create_puzzle5d_app`.
pub fn definition(label: LocalizedLabel) -> UtilityDefinition {
    UtilityDefinition::new(UTILITY_ID, label, "paintbrush")
}
