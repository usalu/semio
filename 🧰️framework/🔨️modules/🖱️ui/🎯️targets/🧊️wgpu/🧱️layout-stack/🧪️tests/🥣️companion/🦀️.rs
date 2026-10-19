//! 🥣️ semio wgpu layout laws: tree row rectangles and the table row grid against the engine, events and paint.

#![cfg(test)]

use semio_framework_ui_declaration::ui::{UiControlNode, UiTreeActionPlacement, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, UiTreeWindow, UiTreeWindowRowExtent};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Theme;
use ui_contract::SpaceToken;
pub use semio_framework_ui_layout::space_token_name;
pub use semio_framework_ui_retained_tree::tree::TREE_ROW_MAX_DEPTH;
pub use semio_framework_ui_retained_tree::tree::tree_section_default_open;
use semio_framework_ui_wgpu_layout::layout::*;

#[path = "../../../../../🧪️tests/🌳️tree-row-rects/🦀️.rs"]
mod tree_row_rect_tests;

#[path = "../../../../../🧪️tests/📊️table-row-grid/🦀️.rs"]
mod table_row_grid_tests;

