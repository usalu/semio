//! 📁️ Virtual file system, board2d and text editor scenes.
//!
//! The virtual file system tree render, hit and accessibility, the board2d scene surface and the text editor scene overlays, completions and rename.

extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;
use crate::element_contracts::FrameworkWidgetContext;
use semio_framework::interaction::gesture::GestureVerdict;
use semio_framework::interaction::gesture::ZoomBounds;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use semio_framework_ui_wgpu_input::KeyAction;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_wgpu_widget_kit::draw_text;
use semio_framework_ui_wgpu_layout::chrome::foreground_on_fill;
use semio_framework_ui_wgpu_input::HitKind;
use semio_framework_ui_wgpu_input::HitTarget;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_declaration::ui::SurfaceKind;
use semio_framework_ui_declaration::ui::UiComponentSceneNode;
use semio_framework_os_renderer_wgpu_scene_state::{SceneListHit, SceneModifiers, SceneSurfaceState, TextEditorRenameDraft, TextEditorUiState, VfsAccessibilityControl, VfsAccessibilityControlKind, VfsAccessibilityFocusTarget, VirtualFileSystemChromeLabels, draw_ink_rect_outline, list_visible_row_capacity, render_placeholder, reserve_list_rows, scene_action, scene_hovered_control_id, scene_input_theme, scene_state, scroll_key, scroll_offset, seed_vfs_expanded, text_editor_ui, toggle_vfs_row_expanded, vfs_selection_for_click, write_scene_action};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_scene_state::{SCENE_STATE, now_ms};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_scene_state::{SCENE_STATE, now_ms};
use semio_framework_os_renderer_wgpu_scene_feeds::{accepted_vfs_accessibility_controls, host_temporal_now_ms, host_temporal_reply, pinch_camera, stage_vfs_accessibility_controls, touch_pointer_down, touch_pointer_move, touch_pointer_up};
use semio_framework_os_renderer_wgpu_engine_text::text_editor::{text_editor_apply_completion_into, text_editor_apply_explicit_draft_into, text_editor_cancel_explicit_publication, text_editor_caret, text_editor_caret_screen, text_editor_discard_explicit_draft, text_editor_explicit_draft_has_conflict, text_editor_explicit_draft_is_dirty, text_editor_explicit_draft_settings, text_editor_explicit_draft_status, text_editor_move_explicit_draft_history, text_editor_preview_rename, text_editor_select_span_into, text_editor_set_selection_into};
use semio_framework_os_renderer_wgpu_engine_board::board2d::{board2d_camera, board2d_publish_camera_into, board2d_set_camera_silent, board2d_yield_to_pinch_into, board_drag_active};
use semio_framework_os_renderer_wgpu_engine_board::tiled_map::{map_local_pointer};

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧩️board2d-and-vfs/🦀️.rs"]
mod board2d_and_vfs;
pub use board2d_and_vfs::*;

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✍️text-editor/🦀️.rs"]
mod text_editor;
pub use text_editor::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
