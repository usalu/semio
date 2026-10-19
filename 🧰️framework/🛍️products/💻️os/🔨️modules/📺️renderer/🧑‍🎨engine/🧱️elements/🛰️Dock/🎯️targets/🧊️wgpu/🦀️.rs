//! 🪟️ framework/products/os/modules/renderer/engine/elements/🛰️Dock/component.rs — wgpu layout and
//! render implementation for the Dock element, extracted from lib.rs's inline
//! `pub mod dock { ... }` body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired via
//! `#[path = "../../../../🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod dock;` in lib.rs in place of the
//! former inline block; the module name `dock` is unchanged, so every existing `crate::dock::...`
//! call site elsewhere in the crate keeps resolving with zero other changes.
//! 🪟️ Mode dock — multi-window layout tree with stack chrome and split resize.

use semio_framework::AppDefinition;
use std::collections::HashMap;
use semio_framework_ui_wgpu_layout::chrome::chrome_item_text;
use semio_framework_ui_wgpu_widget_kit::draw_text;
use semio_framework_ui_declaration::layout::{even_window_layout, ActionDescriptor, WindowLayout, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, WindowStackCorner};
use semio_framework_ui_wgpu_input::{DragAxis, HitKind, HitTarget, InputState};
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_theme::{Level, Rgba, Theme};
use semio_framework_ui_theme::geometry::Rect;

#[path = "🧱️dock-types-and-window-state/🦀️.rs"]
mod dock_types_and_window_state;
pub use dock_types_and_window_state::{
    dock_tab_ids, DockControlName, DockDragKind, DockDragPayload, DockDragState, DockDropZone,
    DockNode, DockPath, DockRenderContext, DockSide, DockStackTab, DockState, SPLIT_MIN_FRACTION,
};
use dock_types_and_window_state::{empty_path, flat_index_for_corner_insert, tabs_by_corner};

#[path = "🎯️dock-state-render-and-drop/🦀️.rs"]
mod dock_state_render_and_drop;

#[path = "📐️layout-conversion-and-drop-zones/🦀️.rs"]
mod layout_conversion_and_drop_zones;
pub use layout_conversion_and_drop_zones::{
    compute_dock_drop_zone, compute_tab_insert_index, dock_cap_depth, dock_from_window_layout,
    dock_node_to_layout_root, drop_zone_indicator_rect, resolve_split_side,
    split_drop_preview_in_body, stack_tab_bar_rect, window_layout_instance_titles,
};
use layout_conversion_and_drop_zones::{
    collect_stack_tab_bars, dock_out_layout, empty_stack, extract_stack_from_layout,
    insert_tabs_at_corner_in_node, mobile_flat_stack, set_window_template, split_node_at_path,
    split_root_node, stack_of,
};

#[path = "🏷️tab-chrome-and-layout-edits/🦀️.rs"]
mod tab_chrome_and_layout_edits;
pub use tab_chrome_and_layout_edits::{
    diff_dock_node, dock_tab_actions, dock_tab_chip, even_layout, find_stack_path,
    mode_dock_device_for_width, node_at, remove_window_from_layout, truncate_label_to_width,
    MODE_DOCK_MOBILE_MAX_WIDTH_PX, MODE_DOCK_TABLET_MAX_WIDTH_PX, MODE_DOCK_TAB_MAX_WIDTH_PX,
};
use tab_chrome_and_layout_edits::{
    collapse_layout_node, collect_corner_tab_bars_for_stack, collect_stack_corner_tab_bars,
    dock_tab_action_width, dock_tab_chrome_width, dock_tab_label, first_window_id, node_at_mut,
    normalize_sizes, rebuild_axis, DockAxisSlot,
};

#[path = "🧮️axis-solving-and-stack-rendering/🦀️.rs"]
mod axis_solving_and_stack_rendering;
use axis_solving_and_stack_rendering::{
    axis_distributable_extent, dock_axis_slots, render_node, render_stack, solve_node_bounds,
    walk_resize_hits,
};
pub use axis_solving_and_stack_rendering::dock_tab_content_width;

#[path = "🔲️window-silhouette-and-stack-geometry/🦀️.rs"]
mod window_silhouette_and_stack_geometry;
pub use window_silhouette_and_stack_geometry::{
    layout_stack_cap, parse_path, path_str, push_window_silhouette_border, stack_window_silhouette,
    WindowSilhouette, WindowSilhouetteEdge, WindowSilhouetteSpan,
};
use window_silhouette_and_stack_geometry::{
    collect_stack_bodies, collect_stack_frames, dock_stack_body_rect, dock_text,
};
