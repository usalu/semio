//! 💬️ Render steps for the hub workspace, chrome dialogs, tour and tooltips.
//!
//! Third render half of `impl ShellState`: the hub workspace, space administration, agent approvals, chrome
//! dialog render/keys/accessibility, the chrome tour, tooltips and context menu levels.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;
