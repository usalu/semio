//! ⌨️ Shell input, part two: keyboard, palette, context menus and dock ring.
//!
//! Tree hover and selection, world3d catalogue drops, context menus, search/find items, the command palette,
//! keyboard handling, the dock and keyboard ring and shortcut capture.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellInput
#[cfg(test)]
#[path = "../../../🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs"]
mod shell_shortcuts_palette_tests;
#[cfg(test)]
#[path = "../../../🧪️tests/⌨️wgpu-chord-example-role-parity/🦀️.rs"]
mod chord_example_role_parity_tests;



#[cfg(test)]
#[path = "../../../🧪️tests/🧲️engine-surface-retention/🦀️.rs"]
mod engine_surface_retention_tests;
#[cfg(test)]
#[path = "../../../🧪️tests/🎯️wgpu-pointer-hit-ownership/🦀️.rs"]
mod pointer_hit_ownership_tests;


#[cfg(test)]
#[path = "../../../🧪️tests/🎡️wgpu-wheel-and-escape-routing/🦀️.rs"]
mod wheel_and_escape_routing_tests;
#[cfg(test)]
#[path = "../../../🧪️tests/🛟️panel-window-reservation/🦀️.rs"]
mod panel_window_reservation_tests;
#[cfg(test)]
#[path = "../../../🧪️tests/🛰️wgpu-dock-close-reopen-journals/🦀️.rs"]
mod dock_close_reopen_journal_tests;



#[cfg(test)]
#[path = "../../../🧪️tests/👕️board-presence/🦀️.rs"]
mod board_presence_tests;
#[cfg(test)]
#[path = "../../../🧪️tests/🔗️hub-projection-workspace/🦀️.rs"]
mod hub_projection_workspace_tests;
#[cfg(test)]
#[path = "../../../🧪️tests/🪟️window-lifecycle-template-drag/🦀️.rs"]
mod window_lifecycle_template_drag_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/⚙️settings-general-layout/🦀️.rs"]
mod settings_general_layout_tests;
#[cfg(test)]
#[path = "../../../🧪️tests/🔄️detached-sync-task-manager/🦀️.rs"]
mod detached_sync_task_manager_tests;


#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🔄️sync-owner-lifetime/🦀️.rs"]
mod sync_owner_lifetime_tests;
//#endregion ShellInput
