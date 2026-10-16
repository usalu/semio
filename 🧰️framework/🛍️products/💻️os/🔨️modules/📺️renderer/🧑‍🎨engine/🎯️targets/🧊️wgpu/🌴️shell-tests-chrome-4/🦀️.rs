//! 🌴️ Companion test bin of the wgpu renderer: chrome and views (4).
//!
//! 🧭️ The Shell tests of this bin are mounted from their place in the Shell element tree; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

use semio_framework_os_renderer_wgpu_shell_actions::staged_arg_value;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_renderer_wgpu_shell_async::{INTRODUCED_PULSE_PERIOD_MS, engagement_completion_suffix, engagement_ghost_accept_on_click, introduced_pulse_thickness, utility_subtree_has_active_path};
use semio_framework_os_renderer_wgpu_shell_chrome_state::{CHROME_TOOLTIP_DELAY_MS, ChromeDialogRequest, ChromeDialogStop, ChromeTooltipHover, RetainedContentFocus, ShellChromeBuildState, chrome_tooltip_ready};
#[cfg(all(any(test, feature = "testkit"), not(target_arch = "wasm32")))]
use semio_framework_os_renderer_wgpu_shell_chrome_state::assert_shell_chrome_build_state_is_send;
use semio_framework_os_renderer_wgpu_shell_input::ChromeDialogPaintOp;
use semio_framework_os_renderer_wgpu_shell_pointer::PointerCtx;
use semio_framework_os_renderer_wgpu_shell_prefs::dsl_value_as_json;
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_renderer_wgpu_shell_view_chrome::render_utility_nodes;
use semio_framework_os_renderer_wgpu_shell_view_overlays::{INTRODUCTION_INFO_BOX_GAP, OverlaysViewCtx, chrome_dialog_paint_ops, introduction_veil_bands, punch_introduction_cutout, resolve_introduction_placement};
use semio_framework_os_renderer_wgpu_shell_view_panels::PanelsViewCtx;
use semio_framework_os_renderer_wgpu_shell_window_actions::staged_color_value;
use semio_framework_os_renderer_wgpu_contracts::{SHELL_FIND_ITEM_CAPACITY, ShellFindItem, ShellFindItems, try_push_find_item};
use semio_framework_os_renderer_wgpu_dock::WindowSilhouette;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_dock::push_window_silhouette_border;
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::RetainedNodeFocusKind;
use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};
use semio_framework_value::DslValue;
use serde_json::Value;
use std::collections::HashMap;
use ui_wgpu::wgpu::{ActionDescriptor, DrawList, FontAtlas, IconAtlas, InputState, KeyAction, PointerModifiers, Rect, Rgba, Theme};

#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-chrome-overlays-tour/🦀️.rs"]
mod chrome_overlays_tour_tests;
