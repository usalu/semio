//! 🌅️ Companion test bin of the wgpu renderer: chrome and views (3).
//!
//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

use semio_framework::{AppDefinition, ViewModel};
use semio_framework_os_renderer_wgpu_dock::{DockNode, DockStackTab, dock_node_to_layout_root};
use semio_framework_os_renderer_wgpu_fixtures::shell::{host_test_shell, retained_world_sequence_probe};
use semio_framework_os_renderer_wgpu_host_base::resolve_theme;
use semio_framework_os_renderer_wgpu_shell_actions::{ActionsCtx, keybinding_unowned_text_v1, resolve_keybinding_target_window_v1, role_switch_target};
use semio_framework_os_renderer_wgpu_shell_dock::dock_tab_icon_id_v1;
use semio_framework_os_renderer_wgpu_shell_docs::{chord_carries_accelerator_v1, reserved_shell_chords_v1, shell_example_rows};
use semio_framework_os_renderer_wgpu_shell_pointer::{is_reserved_shell_chord, pointer_press_belongs_to_shell_chrome, shell_shortcut_for, world3d_cancel_affordance};
use semio_framework_os_renderer_wgpu_shell_prefs::ActiveSession;
use semio_framework_os_renderer_wgpu_shell_session::{SessionCtx, ShellShortcut, WindowScopeStackV1, dock_seed_active_window_id_v1, mode_layout_stacks_v1, resolve_boot_example_id, session_window_instances};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_surfaces::world3d_compute_status;
use semio_framework_os_renderer_wgpu_shell_view_chrome::{chrome_panel_safe_area, shell_example_control, shell_mode_controls, shell_role_controls, surface_overlay_controls_for, surface_status_pills_for, world3d_status_pill_for};
use semio_framework_os_renderer_wgpu_shell_view_paint::{SafeAreaYield, world3d_status_is_visible};
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_os_renderer_wgpu_shell_window_actions::{
    KEYBINDING_UNOWNED_CODE, KEYBINDING_UNOWNED_LABEL_DE, KEYBINDING_UNOWNED_LABEL_EN, WindowScopeInstanceV1, WindowScopeTargetKindV1, step_mode_id, surface_role_apps, window_action_definition,
};
use semio_framework_os_renderer_wgpu_shell_windows::PanelAnchor;
use semio_framework_os_renderer_wgpu_test_support::shell_source;
use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};
use serde_json::Value;
use std::collections::HashMap;
use ui_wgpu::wgpu::{ActionDescriptor, HitKind, HitTarget, KeyAction, PointerModifiers, Rect, WindowLayout, WindowStackCorner};

#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs"]
mod shell_chrome_parity_tests;
