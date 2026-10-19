//! 🧮️ Table, block list and diff view scenes.
//!
//! The table render with cell payloads, sort, stepper and editable cells and their accessibility staging, and the block list and diff view render.

extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;
use crate::element_contracts::FrameworkWidgetContext;
use semio_framework::IconName;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use semio_framework_ui_wgpu_input::KeyAction;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Rgba;
use semio_framework_ui_wgpu_widget_kit::draw_text;
use semio_framework_ui_wgpu_widget_kit::draw_text_face;
use semio_framework_ui_wgpu_layout::chrome::foreground_on_fill;
use semio_framework_ui_wgpu_layout::chrome::push_icon;
use semio_framework_ui_wgpu_widgets::render_widget;
use semio_framework_ui_wgpu_input::HitKind;
use semio_framework_ui_wgpu_input::HitTarget;
use semio_framework_ui_text::TextFace;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_wgpu_layout::chrome::UiDriverDrag;
use semio_framework_ui_wgpu_widget_kit::WidgetNode;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_declaration::ui::SurfaceKind;
use semio_framework_ui_declaration::ui::UiComponentSceneNode;
use semio_framework_os_renderer_wgpu_scene_state::{BLOCK_LIST_PALETTE_DRAG_MIME, BlockListAccessibilityControl, BlockListChromeLabels, SceneListHit, SceneListTransferSession, SceneListTransferSource, SceneListTransferStart, SceneModifiers, TableButtonAccessibilityCell, TableEditableTextAccessibilityCell, TableEditableTextFocusTarget, TableStepperAccessibilityCell, TableStepperFocusTarget, draw_ink_rect_outline, list_visible_row_capacity, render_placeholder, reserve_list_rows, scene_action, scene_hovered_control_id, scene_input_theme, scroll_key, scroll_offset, write_scene_action};
use semio_framework_os_renderer_wgpu_scene_feeds::{accepted_block_list_accessibility_controls, accepted_table_button_accessibility_cells, stage_block_list_accessibility_controls, stage_table_button_accessibility_cells, stage_table_editable_text_accessibility_cells, stage_table_stepper_accessibility_cells};

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧮️table-and-paint2d/🦀️.rs"]
mod table_and_paint2d;
pub use table_and_paint2d::*;

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧱️block-list-and-diff/🦀️.rs"]
mod block_list_and_diff;
pub use block_list_and_diff::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
