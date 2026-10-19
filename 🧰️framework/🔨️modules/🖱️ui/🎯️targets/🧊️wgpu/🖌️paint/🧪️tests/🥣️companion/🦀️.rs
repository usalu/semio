//! 🥣️ semio wgpu retained paint laws: fixtures, steppers, sliders, select popups, carets, grants, progress and skeletons.

#![cfg(test)]

use semio_framework_ui_arena::NodeId;
#[cfg(test)] use semio_framework_ui_wgpu_layout::chrome::chrome_item_bg;
use semio_framework_ui_wgpu_layout::chrome::{foreground_on_fill, item_bg, item_text, push_chrome_border, push_control_border, push_icon, UiDriverDrag, ICON_TINY, SIZE_TINY};
#[cfg(test)] use semio_framework_ui_declaration::ui::{UiButtonNode, UiComponentSceneNode, UiExternalSlotNode, UiGroupNode, UiIconSelectNode, UiImageNode, UiInputNode, UiKeyValueNode, UiNumberStepperNode, UiRingNode, UiSelectItem, UiSelectNode, UiSliderNode, UiTextNode, UiToggleNode};
use semio_framework_ui_declaration::ui::{UiControlNode, UiFieldNode, UiNode, UiPresence, UiProgressNode, UiSectionNode, UiStackNode, UiState, UiStatus, UiTreeItemNode, UiTreeNode, UI_INSPECTOR_MIXED_PLACEHOLDER};
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_wgpu_layout::layout::tree_section_header_height;
use semio_framework_ui_wgpu_layout::layout::{tree_drag_handle_rect, tree_drag_handle_reservation, tree_drag_role, TreeDragRole, TreeRowMetrics};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_theme::{Level, Rgba, Theme};
#[cfg(test)] use semio_framework_ui_retained_tree::tree::EditState;
use semio_framework_ui_retained_tree::tree::{Node, NodeFlags, NodeKey, UiTree, WidgetSpec};
use semio_framework_ui_wgpu_widget_kit::draw_text_on;
#[cfg(test)] use semio_framework_ui_wgpu_widget_kit::wrap_text;
#[cfg(test)] use semio_framework_ui_declaration::IconName;
use semio_framework_ui_locale::Label;
use semio_framework_ui_declaration::ui::UiTreeActionPlacement;
use semio_framework_ui_wgpu_controls::stepper::{push_stepper_chrome, push_stepper_icon};
use semio_framework_ui_wgpu_paint::*;
use semio_framework_ui_wgpu_paint::RETAINED_NODE_COLLECTION_ITEMS;
use semio_framework_ui_wgpu_paint::RETAINED_SYNC_COLLECTION_ITEMS;
use semio_framework_ui_wgpu_paint_reference::find_child_by_key;
use semio_framework_ui_wgpu_paint::paint_node_step;
use semio_framework_ui_wgpu_paint_reference::paint_slider;
use semio_framework_ui_wgpu_paint_reference::paint_tree;

#[path = "../../../../../🧪️tests/🔬️targets-wgpu-paint-unit/🦀️.rs"]
mod tests;

