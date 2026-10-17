//! 🌌️ Companion test bin of the wgpu renderer: chrome and views (2).
//!
//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

use semio_framework::{AppDefinition, PanelGroup, PanelTabDefinition};
use semio_framework_os_renderer_wgpu_dock as dock;
use semio_framework_os_renderer_wgpu_dock::*;
use semio_framework_os_renderer_wgpu_dock::{
    DockDragKind, DockDragPayload, DockDropZone, DockNode, DockPath, DockRenderContext, DockSide, DockStackTab, DockState, MODE_DOCK_TAB_MAX_WIDTH_PX, SPLIT_MIN_FRACTION, WindowSilhouette, WindowSilhouetteEdge, WindowSilhouetteSpan,
    compute_dock_drop_zone, compute_tab_insert_index, dock_cap_depth, dock_node_to_layout_root, dock_tab_actions, dock_tab_content_width, resolve_split_side, split_drop_preview_in_body, truncate_label_to_width,
};
use semio_framework_os_renderer_wgpu_engine_board::tiled_map;
use semio_framework_os_renderer_wgpu_shell_commands::resolved_execute_args;
use semio_framework_os_renderer_wgpu_shell_dock::resolve_window_utilities;
use semio_framework_os_renderer_wgpu_shell_docs::{SpacePanelState, SpawnedAppEntry, key_event_matches_chord, take_spawned_entry};
use semio_framework_os_renderer_wgpu_shell_prefs::labels;
use semio_framework_os_renderer_wgpu_shell_session::SessionCtx;
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};
use semio_framework_value::ToValue;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use ui_wgpu::wgpu::{ActionDescriptor, DragAxis, DrawList, FontAtlas, HitKind, HitTarget, IconAtlas, InputState, KeyAction, PointerModifiers, Rect, Theme, WindowLayout, WindowMeasure, WindowStackCorner};

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🛰️Dock/🧪️tests/🔬️wgpu-shell-bound/🦀️.rs"]
mod shell_bound_tests;
