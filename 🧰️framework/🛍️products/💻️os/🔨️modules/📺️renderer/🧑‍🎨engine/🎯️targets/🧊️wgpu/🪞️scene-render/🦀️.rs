//! 🪞️ Scene render: the canvas pointer wire and cancellation, the catalogue drag, the surface context menus and the component scene render entry.
//!
//! The top of Scenes: everything it shares with the scene leaves was lifted into the scene state; the render entry dispatches to every scene kind.

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;
extern crate semio_framework_os_renderer_wgpu_host_base as host_base;
use crate::element_contracts::{SCENE_SURFACE_ID_BYTE_CAPACITY};
use crate::element_contracts::FrameworkWidgetContext;
use crate::element_contracts::try_push_find_item;
use crate::element_contracts::ShellFindItem;
use serde_json::json;
use serde_json::Value;
use ui_wgpu::wgpu::Rect;
use ui_wgpu::wgpu::Rgba;
use ui_wgpu::wgpu::Theme;
use ui_wgpu::wgpu::UiDriverDrag;
use ui_wgpu::wgpu::ActionDescriptor;
use ui_wgpu::wgpu::SurfaceKind;
use ui_wgpu::wgpu::UiComponentSceneNode;
use semio_framework_os_renderer_wgpu_scene_state::{CanvasCatalogueHover, CanvasClick, CanvasGesture, CanvasGestureKind, SceneChromeLabels, SceneDrag, SceneDragMode, SceneEngineHosts, SceneListHit, SceneListTransferRelease, SceneListTransferSession, SceneListTransferSource, SceneListTransferStart, SceneModifiers, Viewport, World3dAccessibilityControl, bind_scene_engine_token, canvas_active_utility, list_visible_row_capacity, mutate_scene_state, note_world3d_live_camera, remember_scene_theme, request_cancel, reserve_list_rows, scene_action, scene_host_retiring, scene_input_theme, scene_state, schedule_scene_camera_dispatch, scroll_offset, set_scroll_offset, toggle_vfs_row_expanded, write_scene_action, write_scene_action_batch};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_scene_state::{CANVAS_CATALOGUE_HOVER, CANVAS_GESTURE, SCENE_LIST_TRANSFER, SCENE_STATE, now_ms};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_scene_state::{CANVAS_CATALOGUE_HOVER, CANVAS_GESTURE, SCENE_LIST_TRANSFER, SCENE_STATE, now_ms};
use semio_framework_os_renderer_wgpu_canvas2d_host::{cancel_due, cancel_into, cancel_stale_into, drag, paint, pointer_button_into, pointer_move_into, release, retire_window};
use semio_framework_os_renderer_wgpu_scene_canvas2d::{render_canvas_2d};
use semio_framework_os_renderer_wgpu_scene_ink::{INK_INTERACTION_ITEM_CAPACITY, InkCameraF, InkCameraJson, ink_hits_point, ink_screen_to_world, render_ink_canvas};
use semio_framework_os_renderer_wgpu_scene_feeds::{EventFeedEntryJson, event_feed_hit, event_feed_layout, event_feed_row_height, graph_timeline_hit, render_event_feed, render_graph_timeline, stage_world3d_accessibility_controls, tiled_map_context_menu_target};
use semio_framework_os_renderer_wgpu_scene_tables::{TableColumn, block_list_hit, block_list_transfer_drop_action, block_list_transfer_start, merge_action_args, render_block_list, render_diff_view, render_table, table_hit, table_metrics, table_row_id, table_transfer_start};
use semio_framework_os_renderer_wgpu_scene_text_vfs::{TEXT_EDITOR_OVERLAY_ITEMS, build_vfs_visible_rows, double_click_action, hit_double_click_target, render_text_editor_overlays, render_vfs, vfs_hit, vfs_metrics, vfs_row_id};
use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::{engine_surface_token};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_engine_board::paint2d::{board2d_context_menu_target, node_graph_context_menu_target, paint2d_context_menu_target, text_editor_context_menu_target};
use semio_framework_os_renderer_wgpu_engine_graph::node_graph::{paint_node_graph_labels, paint_node_graph_overlays, paint_paint2d_overlays};
use semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::{EngineSurfaceKindDetail, advance_raster_composite, engine_raster_key, register_engine_surface, stage_engine_scene_paint, sync_engine_scene};
use semio_framework_os_renderer_wgpu_icon_host::{render_icon_render};

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/⌨️scene-input/🦀️.rs"]
mod scene_input;
pub use scene_input::*;

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🎨️scene-render-entry/🦀️.rs"]
mod scene_render_entry;
pub use scene_render_entry::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
