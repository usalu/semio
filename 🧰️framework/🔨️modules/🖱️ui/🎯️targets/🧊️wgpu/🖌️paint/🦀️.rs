// #region paint
//! 🖌️ Retained paint pass. Per-`UiNode`-variant drawing logic mechanically ported from the
//! immediate-mode `widgets::render_*` functions (see that region's doc comment for why it still
//! exists), reading resolved geometry from `tree::LayoutBucket` (accumulating parent-relative
//! offsets while walking, since taffy's `Layout::location` is parent-relative — see that struct's
//! doc comment) instead of the old `bounds: Rect` argument an immediate-mode caller threaded down.
//! Interaction-derived visuals (hover/focus/active/selected) read live `NodeFlags`/`WidgetState`,
//! written each frame by `events::EventRouter` (M5, landed) — no longer default/empty by the time
//! `paint_tree` runs, as an earlier revision of this comment used to caveat. `WidgetState`-backed
//! composites have since gained real paint support too: an open `Select`'s popup expands live
//! (`paint_select`'s `open`/`retained` params, wired by the W2 pass — see
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY/report-w2-ui-wgpu-integration.md`), and a focused
//! `Input`'s caret/selection-highlight render straight from its live `EditState` (`paint_input`,
//! W2 widget-visuals pass). `Tree`'s live scroll offset (`WidgetState::scroll_offset`) remains the
//! one rest-state-only exception — no scrollable-viewport paint exists yet, out of every pass to
//! date's scope.

use semio_framework_ui_arena::NodeId;
use semio_framework_ui_wgpu_layout::chrome::{foreground_on_fill, item_bg, item_text, push_chrome_border, push_control_border, push_icon, UiDriverDrag, ICON_TINY, SIZE_TINY};
use semio_framework_ui_declaration::ui::{UiControlNode, UiFieldNode, UiNode, UiPresence, UiProgressNode, UiSectionNode, UiStackNode, UiState, UiStatus, UiTreeItemNode, UiTreeNode, UI_INSPECTOR_MIXED_PLACEHOLDER};
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_wgpu_layout::layout::tree_section_header_height;
use semio_framework_ui_wgpu_layout::layout::{tree_drag_handle_rect, tree_drag_handle_reservation, tree_drag_role, TreeDragRole, TreeRowMetrics};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_theme::{Level, Rgba, Theme};
use semio_framework_ui_retained_tree::tree::{Node, NodeFlags, NodeKey, UiTree, WidgetSpec};
use semio_framework_ui_wgpu_widget_kit::draw_text_on;
use semio_framework_ui_locale::Label;
use semio_framework_ui_declaration::ui::UiTreeActionPlacement;
use semio_framework_ui_wgpu_controls::stepper::{push_stepper_chrome, push_stepper_icon};

#[path = "🔤️retained-glyph-and-node-cursors/🦀️.rs"]
mod retained_glyph_and_node_cursors;
pub use retained_glyph_and_node_cursors::{
    paint_retained_glyph_step, paint_retained_glyph_step_flowed, paint_retained_glyph_step_weighted,
    RetainedGlyphCursor, RetainedGlyphStep, RetainedNodePaintCursor, RetainedNodePaintStep,
    RetainedTextFlow, RETAINED_NODE_COLLECTION_ITEMS, RETAINED_NODE_TEXT_MAX_BYTES,
    RETAINED_NUMBER_STEPPER_CHROME_OUTPUT_ITEMS, TREE_ICON_SIZE, TREE_INDENT_PER_LEVEL,
    TREE_TOGGLE_WIDTH,
};
use retained_glyph_and_node_cursors::{
    RETAINED_NODE_FIXED_OUTPUT_BYTES, RETAINED_NODE_FIXED_OUTPUT_ITEMS, RETAINED_TREE_DEPTH,
};

#[path = "📝️retained-text-steps/🦀️.rs"]
mod retained_text_steps;
use retained_text_steps::{
    retained_caret_step, retained_control_is_bounded, retained_fixed_output,
    retained_fixed_output_budgeted, retained_presence_step, retained_presence_step_with_selection,
    retained_select_close_route, retained_select_popup_at_bounds, retained_stepper_text_step,
    retained_table_cells_step, retained_table_header_step, retained_text_end_step,
    retained_text_node_step, retained_text_node_step_weighted, retained_tree_content_height,
    retained_tree_item_at, retained_tree_item_row, retained_tree_row_advance,
    retained_tree_section_at, retained_tree_sibling_count, retained_tree_text_step,
    RetainedCaretAlignment,
};
pub use retained_text_steps::{
    chrome_band_rect, field_paint_metrics, section_paint_metrics, text_line_view,
};

#[path = "🌲️retained-tree-node-painting/🦀️.rs"]
mod retained_tree_node_painting;
use retained_tree_node_painting::retained_tree_node_step;
pub use retained_tree_node_painting::paint_node_step;

#[path = "🎨️node-painting-with-driver/🦀️.rs"]
mod node_painting_with_driver;
pub use node_painting_with_driver::paint_node_step_with_driver;

#[path = "🔄️interactive-state-sync/🦀️.rs"]
mod interactive_state_sync;
pub use interactive_state_sync::{
    select_menu_top_for, select_popup_row_rect, sync_interactive_state_node_step,
    RetainedInteractiveSyncCursor, RetainedInteractiveSyncStep, RETAINED_SYNC_COLLECTION_ITEMS,
};

#[path = "📶️progress-celebrate-and-widget-painters/🦀️.rs"]
mod progress_celebrate_and_widget_painters;
pub use progress_celebrate_and_widget_painters::{
    celebrate_color, celebrate_stroke, celebrate_turns, color_input_swatch, focus_ring_visible,
    node_selection_outline, paint_checkbox, paint_loading_border, paint_progress, paint_separator,
    paint_slider_travel, paint_stack_frame, paint_waiting_border, presence_overlay,
    progress_bar_rects, progress_bar_rects_of, PROGRESS_INDETERMINATE_SHARE,
};

#[path = "🖼️ui-image-sources-skeleton-and-overlays/🦀️.rs"]
mod ui_image_sources_skeleton_and_overlays;
pub use ui_image_sources_skeleton_and_overlays::{
    admit_ui_image, close_ui_image_ledger_step, paint_overlay_backdrop, paint_overlay_surface,
    paint_tooltip, retained_overlay_chrome_step, skeleton_blocks, skeleton_kind,
    skeleton_replaces_content, take_ui_image_upload, tooltip_surface_size, ui_image_content_rect,
    ui_image_natural_size, SkeletonKind, UiImageAdmission, UiImageUpload, OVERLAY_BACKDROP_ALPHA,
    RETAINED_OVERLAY_CHROME_ITEMS, SKELETON_MAX_BLOCKS, UI_IMAGE_LEDGER_ENTRIES,
    UI_IMAGE_MAX_BOX_HEIGHT, UI_IMAGE_MAX_DIMENSION, UI_IMAGE_SOURCE_MAX_BYTES,
};
use ui_image_sources_skeleton_and_overlays::retained_skeleton_step;

// #endregion paint
