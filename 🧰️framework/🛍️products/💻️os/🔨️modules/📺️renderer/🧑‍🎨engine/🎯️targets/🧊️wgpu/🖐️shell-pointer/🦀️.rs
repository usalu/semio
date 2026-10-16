//! 🖐️ Shell use-cases: pointer, keyboard, accessibility events and the settle driver.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store_sync;
use semio_framework_os_renderer_wgpu_shell_actions::{ActionsCtx, command_rejection_notice, keybinding_unowned_text_v1, resolve_keybinding_target_window_v1, settle_watch_verdict};
use semio_framework_os_renderer_wgpu_shell_async::{chrome_now_ms, debug_log};
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ChromeState, format_keybinding_shortcut, shell_shortcut_table, window_pane_owner};
use semio_framework_os_renderer_wgpu_shell_commands::CommandState;
use semio_framework_os_renderer_wgpu_shell_directory::DirectoryState;
use semio_framework_os_renderer_wgpu_shell_doc_open::{IconExportBatch, OpeningState};
use semio_framework_os_renderer_wgpu_shell_dock::{ShellDockState, ShellKeyboardStop, dock_window_order, retained_event_modifiers, retained_pointer_button};
use semio_framework_os_renderer_wgpu_shell_docs::{DocumentState, FRAMEWORK_CHAT_PANEL_ID, FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX, FRAMEWORK_SYNC_PANEL_TAB_ID, FRAMEWORK_TOOL_PANEL_TAB_PREFIX, command_host_platform, key_event_matches_chord, note_shell_command_action};
use semio_framework_os_renderer_wgpu_shell_history::HistoryState;
use semio_framework_os_renderer_wgpu_shell_input::{ContextMenuItem, ContextMenuKeyOutcome, ContextMenuState, ContextMenuSurfaceResolution, OverlayState, PointerHitOwner, RightClickState, ShellInputState, ShellPaletteKind, context_menu_item_at_path, context_menu_path_for_item_id, tree_drag_handle_item_id, tree_hit_item_id};
use semio_framework_os_renderer_wgpu_shell_plugins::{PluginState, flatten_panel_tab_leaves};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_shell_plugins::PendingExtensionInvocation;
use semio_framework_os_renderer_wgpu_shell_prefs::{PrefsState, TimeTravelVerb, command_address_stable_key, history_refusal_is_silent, history_refusal_notice};
use semio_framework_os_renderer_wgpu_shell_session::{FRAMEWORK_CATEGORY_COMMAND_ID, SessionCtx, ShellShortcut, claim_app_catalogue_fetch, import_transfer_notice, mobile_panel_reconcile_path, panel_state_from_view, resident_census};
use semio_framework_os_renderer_wgpu_shell_state::{SHELL_DISPATCH_ORIGIN_USER, ShellState};
use semio_framework_os_renderer_wgpu_shell_surfaces::{SettleState, ShellDetached, ShellRenderedRefresh, ShellSettleStep, SurfaceState, TutorialPendingDocOp, TutorialState, World3dComputeStatus, world3d_compute_status};
use semio_framework_os_renderer_wgpu_shell_sync::{ShellSyncReseed, SyncState, shell_hub_document_key, shell_sync_channel_owner};
use semio_framework_os_renderer_wgpu_shell_window_actions::{HOST_WINDOW_BLUR_PENDING, IMPORT_TRANSFER_TURN_MS, KEYBINDING_UNOWNED_CODE, SHELL_DEFERRED_CHAIN_ROUNDS, SHELL_DISPATCH_ORIGIN_GESTURE, SHELL_SETTLE_STALL_STEPS, SYNC_COMMAND_TRANSFORMED_NOTICE_CODE, ShellSettleVerdict, WindowActionsCtx, WindowScopeInstanceV1, WindowScopeTargetKindV1, merge_committed_args, parse_dock_tab_action_id, retained_router_owns_pointer, route_document_backbone_effects, scope_action_to_window, shell_hub_remote, shell_sync_link_after_status, shell_sync_link_terminal, shell_sync_owner_matches, window_action_definition};
use semio_framework_os_renderer_wgpu_shell_windows::{PanelAnchor, WINDOW_PANE_CHIP_PARENT, WORLD_PROJECTION_PANE_PARENT, WindowState, window_host_event_action};
use semio_framework::ViewModel;
use semio_framework::kernel::{UiDirtyScope, UiDirtySection};
use semio_framework_os_renderer_wgpu_dock::{DockDragKind, DockDragState, compute_dock_drop_zone};
use semio_framework_os_renderer_wgpu_program_bridge::ProgramBridgeEntry;
use semio_framework_os_renderer_wgpu_scene_state::toggle_vfs_row_expanded;
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_value::{DslValue, ToValue};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use store_sync::sync::ArtifactEvent;
use ui_wgpu::wgpu::{ActionDescriptor, DragAxis, HitKind, HitTarget, InputState, PointerModifiers, Rect, Theme, TreeDragState, TreeDropPosition};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_pointer__keyboard_and_palette;
pub use staged_pointer__keyboard_and_palette::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🎓️tutorial-runtime/🦀️.rs"]
mod staged_pointer__tutorial_runtime;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_pointer__theme_file_import_and_accessibility;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🎬️shell-actions/🦀️.rs"]
mod staged_pointer__shell_actions;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_pointer__space_admin_and_settle;
pub use staged_pointer__space_admin_and_settle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/📨️command-dispatch/🦀️.rs"]
mod staged_pointer__command_dispatch;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_pointer__storage_prefs_and_i18n;
pub use staged_pointer__storage_prefs_and_i18n::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🔖️identity-and-types/🦀️.rs"]
mod staged_pointer__identity_and_types;
pub use staged_pointer__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🖱️pointer-input/🦀️.rs"]
mod staged_pointer__pointer_input;
pub use staged_pointer__pointer_input::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🧩️session-and-topology/🦀️.rs"]
mod staged_pointer__session_and_topology;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_pointer__window_actions_and_commands;
pub use staged_pointer__window_actions_and_commands::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_pointer__chrome_paint_and_navbar;
pub use staged_pointer__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🧵️shell-async-runners/🦀️.rs"]
mod staged_pointer__shell_async_runners;
pub use staged_pointer__shell_async_runners::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/🧺️bundle/🦀️.rs"]
mod staged_pointer__bundle;
pub use staged_pointer__bundle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/📤️icon-export/🦀️.rs"]
mod staged_pointer__icon_export;
pub use staged_pointer__icon_export::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
