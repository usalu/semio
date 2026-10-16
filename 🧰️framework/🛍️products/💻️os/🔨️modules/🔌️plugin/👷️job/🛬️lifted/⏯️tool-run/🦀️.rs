//! ⏯️ Lift file: 1 item(s), about 4 lines, of `⏯️tool-run/🦀️.rs` that belong to crate `job` (WP3).
//!
//! WP5 owns the source file and cuts the items here (is_tool_run_action_id); WP3 wires this file with one `mod` line in the crate root. Until `job` exists, the source module re-exports the items with `pub use`.
use super::*;

/// 🔖️ True for the seven framework-reserved tool run action ids (§2.5).
pub fn is_tool_run_action_id(action: &str) -> bool {
    TOOL_RUN_ACTION_IDS.contains(&action)
}
