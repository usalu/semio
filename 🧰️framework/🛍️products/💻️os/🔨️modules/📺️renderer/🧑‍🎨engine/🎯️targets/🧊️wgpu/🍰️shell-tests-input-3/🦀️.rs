//! 🍰️ Companion test bin of the wgpu renderer: input and pointer (3).
//!
//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

extern crate infinite_canvas as infinite_world;
use infinite_world::world::World3dState;
use semio_framework::ViewModel;
use semio_framework_os_renderer_wgpu_contracts::{AdmittedSurfaceMap, SCENE_SURFACE_CAPACITY};
use semio_framework_os_renderer_wgpu_dock::{DockDragKind, DockSide, DockStackTab};
use semio_framework_os_renderer_wgpu_fixtures::shell::{host_test_shell, retained_world_sequence_probe, split_pane_shell, test_app, tour_introduction, tour_shell};
use semio_framework_os_renderer_wgpu_host_base::{keybinding_platform_uses_meta, set_host_platform_uses_meta};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_native_platform::native_accessibility;
use semio_framework_os_renderer_wgpu_program_bridge::ProgramBridgeEntry;
use semio_framework_os_renderer_wgpu_shell_actions::ActionsCtx;
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ChromeDialogRequest, format_keybinding_shortcut};
use semio_framework_os_renderer_wgpu_shell_dock::ShellKeyboardStop;
use semio_framework_os_renderer_wgpu_shell_docs::{ShellExampleRow, window_template_drag_data};
use semio_framework_os_renderer_wgpu_shell_input::{ContextMenuItem, ContextMenuState, OverlayState, PointerCapture, PointerHitOwner, context_menu_path_for_ordinal, panel_tab_introduction_element_id};
use semio_framework_os_renderer_wgpu_shell_pointer::{
    PointerCtx, ShellEditVerb, UI_INTRODUCTION_BACK_CONTROL_ID, UI_INTRODUCTION_NEXT_CONTROL_ID, UI_INTRODUCTION_SKIP_CONTROL_ID, UI_INTRODUCTION_VEIL_CONTROL_ID, is_reserved_shell_chord, pointer_hit_owner, pointer_press_belongs_to_shell_chrome,
    shell_edit_verb_for, shell_shortcut_for, shell_shortcut_for_control_id,
};
use semio_framework_os_renderer_wgpu_shell_prefs::{ActiveSession, CHROME_ICON_TINY, DockTabNode};
use semio_framework_os_renderer_wgpu_shell_session::{SessionCtx, ShellShortcut};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_surfaces::ShellSettleWatch;
use semio_framework_os_renderer_wgpu_shell_view_chrome::{ChromeViewCtx, ShellChromeChildCursor, shell_example_control};
use semio_framework_os_renderer_wgpu_shell_view_overlays::{
    ChromeTourLayout, OverlaysViewCtx, chrome_tour_chip_hotkey_box, chrome_tour_chip_icon_x, chrome_tour_chip_label_box, chrome_tour_layout, context_menu_level_width, context_menu_submenu_open, render_context_menu_level,
};
use semio_framework_os_renderer_wgpu_shell_view_paint::SHELL_WINDOW_PAINT_OPPORTUNITIES;
use semio_framework_os_renderer_wgpu_shell_view_panels::{PanelsViewCtx, empty_dock_notice_lines};
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_os_renderer_wgpu_shell_window_actions::{KEYBINDING_UNOWNED_CODE, WindowActionsCtx, window_activation_is_user_v1};
use semio_framework_os_renderer_wgpu_shell_windows::{PanelAnchor, world_projection_initial_seed, world_projection_template};
use semio_framework_os_renderer_wgpu_test_support::shell_source;
use semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn;
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_layout::layout::ActionDescriptor;
use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_value::DslValue;
use serde::Deserialize;
use serde_json::Value;
use ui_wgpu::wgpu::{HitKind, HitTarget, InputState, KeyAction, Level, PointerModifiers};
use winit::window::Window;

#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-context-menu-keyboard/🦀️.rs"]
mod context_menu_keyboard_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/⌨️native-keyboard-ring/🦀️.rs"]
mod native_keyboard_ring_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs"]
mod tour_overlay_and_chord_glyph_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/⌨️wgpu-chord-example-role-parity/🦀️.rs"]
mod chord_example_role_parity_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🎡️wgpu-wheel-and-escape-routing/🦀️.rs"]
mod wheel_and_escape_routing_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🪟️window-lifecycle-template-drag/🦀️.rs"]
mod window_lifecycle_template_drag_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🎯️wgpu-pointer-hit-ownership/🦀️.rs"]
mod pointer_hit_ownership_tests;
