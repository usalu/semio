//#region 🧵️MountedLayoutText
//! 🧵️ Fixed-credit mounted layout and text worker with one cursor opportunity per grant.

use semio_framework_ui_arena::NodeId;
use semio_framework_ui_declaration::ui::{UiNode, UiTreeItemNode, UiTreeNode};
use semio_framework_ui_surface_identity::UiSurfaceToken;
use semio_framework_ui_wgpu_layout::flex::{field_chrome_metrics, section_chrome_metrics, FlexRect, FlexTree, LayoutJobStage, LayoutJobStep, LayoutNodeKind, MeasureConstraint, TreeContentFlow, FIELD_DETAIL_FONT_SIZE, SECTION_TITLE_FONT_SIZE};
use semio_framework_ui_wgpu_layout::layout::{gap_for_token, padding_for_token, tree_content_height, tree_section_header_height, tree_window_row_extent_px, tree_window_spacer_px, TreeRowMetrics, TREE_ROW_MAX_DEPTH};
use semio_framework_ui_text::{is_wrap_space, may_break_between};
use semio_framework_ui_theme::Theme;
use semio_framework_ui_retained_tree::tree::{AcceptedLayout, NodeFlags, NodeKey, UiTree};
use semio_framework_ui_wgpu_light::tree_window::{TreeWindowContainerMeasure, TreeWindowRowMeasure};

#[path = "🎯️hit/🦀️.rs"]
pub mod hit;

#[path = "🌲️tree-metrics-and-window-measures/🦀️.rs"]
mod tree_metrics_and_window_measures;
pub use tree_metrics_and_window_measures::{
    document_row_action_id, document_table, document_table_row, find_tree_item, host_content_height,
    live_tree_item_height, live_tree_section_height, owning_tree_spec,
    retained_section_title_height, retained_tree_height, retained_tree_row_metrics,
    table_actions_width_of, tree_window_measures, MountedLayoutFault, LAYOUT_ATLAS_PAGE_BYTES,
    LAYOUT_ATLAS_PAGE_CREDITS, LAYOUT_DEPTH_CREDITS, LAYOUT_GLYPH_CREDITS, LAYOUT_NODE_CREDITS,
    TREE_DETAIL_HEIGHT,
};
use tree_metrics_and_window_measures::{
    is_tree_item_detail, tree_row_kind, IntrinsicSize, DEFAULT_TEXT_SIZE_PX,
};

#[path = "🔤️layout-input-and-text-measurement/🦀️.rs"]
mod layout_input_and_text_measurement;
use layout_input_and_text_measurement::{
    composite_chrome, measure_text, ChromeTextRanges, DeterministicTextWorker, GlyphRange,
    LayoutInputNode, OwnedTextWorker, RetainedAtlasCandidate, RetainedGlyphInput, RetainedLine,
    RetainedTextRun,
};
pub use layout_input_and_text_measurement::RetainedGlyphPreview;

#[path = "🧱️job-state-and-admission/🦀️.rs"]
mod job_state_and_admission;
pub use job_state_and_admission::{MountedLayoutIdentity, MountedLayoutJob, MountedLayoutResult};
use job_state_and_admission::{AdmissionPhase, WalkFrame};

#[path = "📐️text-admission-and-measure-steps/🦀️.rs"]
mod text_admission_and_measure_steps;

#[path = "🏁️collect-publish-and-close/🦀️.rs"]
mod collect_publish_and_close;
#[cfg(any(test,feature="testkit"))]
pub use collect_publish_and_close::layout_tree_now;

#[cfg(test)]
#[path = "../../../🧪️tests/🪟️tree-window-streaming/🦀️.rs"]
mod tree_window_streaming_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🦀️.rs"]
mod tests;
//#endregion 🧵️MountedLayoutText
