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
use ui_wgpu::wgpu::input::KeyAction;
use ui_wgpu::wgpu::Rect;
use ui_wgpu::wgpu::Rgba;
use ui_wgpu::wgpu::draw_text;
use ui_wgpu::wgpu::draw_text_face;
use ui_wgpu::wgpu::foreground_on_fill;
use ui_wgpu::wgpu::push_icon;
use ui_wgpu::wgpu::render_widget;
use ui_wgpu::wgpu::HitKind;
use ui_wgpu::wgpu::HitTarget;
use ui_wgpu::wgpu::TextFace;
use ui_wgpu::wgpu::Theme;
use ui_wgpu::wgpu::UiDriverDrag;
use ui_wgpu::wgpu::WidgetNode;
use ui_wgpu::wgpu::ActionDescriptor;
use ui_wgpu::wgpu::SurfaceKind;
use ui_wgpu::wgpu::UiComponentSceneNode;
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
