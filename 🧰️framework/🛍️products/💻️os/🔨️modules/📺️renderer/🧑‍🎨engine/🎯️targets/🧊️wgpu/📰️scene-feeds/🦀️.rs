//! 📰️ Event feed, graph timeline, node graph and tiled map scenes.
//!
//! The event feed and graph timeline render with their accessibility controls and host temporal presentation, and the node graph and tiled map scene surfaces.

extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;
extern crate semio_framework_os_renderer_wgpu_host_base as host_base;
use crate::element_contracts::{AdmittedSurfaceToken, SCENE_SURFACE_ID_BYTE_CAPACITY};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_native_platform::{native_host_temporal_formatter};
use crate::element_contracts::FrameworkWidgetContext;
use semio_framework::interaction::gesture::apply_pinch_to_camera;
use semio_framework::interaction::gesture::Camera2d;
use semio_framework::interaction::gesture::GesturePointer;
use semio_framework::interaction::gesture::GestureVerdict;
use semio_framework::interaction::gesture::ZoomBounds;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Rgba;
use semio_framework_ui_wgpu_widget_kit::draw_text;
use semio_framework_ui_wgpu_widget_kit::draw_text_weighted;
use semio_framework_ui_wgpu_layout::chrome::foreground_on_fill;
use semio_framework_ui_wgpu_input::HitKind;
use semio_framework_ui_wgpu_input::HitTarget;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_declaration::ui::UiComponentSceneNode;
use semio_framework_os_renderer_wgpu_scene_state::{BlockListAccessibilityControl, EventFeedAccessibilityControl, GraphTimelineAccessibilityControl, MAP_MARQUEE_THRESHOLD_PX, SceneDrag, SceneDragMode, SceneListHit, SceneSurfaceState, TableButtonAccessibilityCell, TableEditableTextAccessibilityCell, TableStepperAccessibilityCell, VfsAccessibilityControl, VfsAccessibilityFocusTarget, World3dAccessibilityControl, list_visible_row_capacity, mutate_scene_state, render_placeholder, reserve_list_rows, scene_action, scene_hovered_control_id, scene_state, scroll_key, scroll_offset, set_scroll_offset, write_scene_action};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_scene_state::{SCENE_STATE, now_ms};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_scene_state::{SCENE_STATE, now_ms};
use semio_framework_os_renderer_wgpu_engine_board::tiled_map::{map_local_pointer, map_marquee_crossing, map_marquee_mode, parse_map_feature_hit, retire_map_interaction_owner, stamp_map_interaction_owner, tiled_map_camera, tiled_map_publish_camera_into, tiled_map_set_camera_silent, with_map_host, with_map_interaction_into};

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/📰️event-feed-and-timeline/🦀️.rs"]
mod event_feed_and_timeline;
pub use event_feed_and_timeline::*;

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧭️node-graph-and-tiled-map/🦀️.rs"]
mod node_graph_and_tiled_map;
pub use node_graph_and_tiled_map::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
