// #region flex
//! 📏️ The production flex/grid layout engine for the retained tree (`tree`/`reconcile`), driven one
//! bounded step at a time by `mounted_layout::MountedLayoutJob`. Taffy's own types (`taffy::TaffyTree`,
//! `taffy::NodeId`, `taffy::Style`, …) are fully wrapped by [`FlexTree`] and never appear in any item
//! visible outside this file — [`FlexRect`]/[`MeasureConstraint`] are this crate's own vocabulary.
//!
//! **Two layout dialects meet here, and the difference is deliberate.** A node whose `LayoutSpec`
//! was AUTHORED (a `UiNodeRecord` carrying `record.layout`, the same value React's `layoutSpecStyle`
//! reads — `os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`) is laid out by
//! plain CSS rules: direction/align/justify/wrap/gap/padding/grow/basis, real grid tracks, in-flow
//! overlay positioning contexts, and out-of-flow absolute positioning. A node that came off the LEGACY declarative `UiNode`
//! path (in-crate chrome: `shell`, the wgpu `Interpreter`/`Shell` elements — content with no React
//! counterpart at all) keeps the pre-parity rule that every child of a `Stack` grows into the
//! leftover main-axis space, because that chrome carries no `grow` information to replace it with.
//! [`FlexTree::push`] picks the dialect from whether the arena node carries `Node::layout_spec`.
//!
//! Composite wgpu widgets (`Field`/`Section`/`Tree`/`TreeSection`/`TreeRow`/`Control`) keep their own
//! hand-rolled geometry, expressed as flex styles here rather than as a second arrange pass: the
//! painter and `events::hit_test` derive their rows from the same `layout`-region constants, so an
//! authored spec on such a node must NOT move them (see [`style_for`]'s own arms).

use ui_contract::{Align, Axis, EdgePx, GridTrack, Justify, LayoutSpec, Sizing};

use crate::layout::TreeRowMetrics;

#[path = "🎚️layout-node-kind-geometry-and-job-stages/🦀️.rs"]
mod layout_node_kind_geometry_and_job_stages;
pub use layout_node_kind_geometry_and_job_stages::{
    FlexRect, LayoutJobStage, LayoutJobStep, LayoutNodeKind, MeasureConstraint,
};

#[path = "🌊️flow-style/🦀️.rs"]
mod flow_style;
pub use flow_style::{Dim, FlowKind, FlowStyle, TreeContentFlow};
use flow_style::{flow_for, tree_content_flow};

#[path = "🎨️style-mapping-and-chrome-metrics/🦀️.rs"]
mod style_mapping_and_chrome_metrics;
#[cfg(test)]
use style_mapping_and_chrome_metrics::align_items;
use style_mapping_and_chrome_metrics::{grows_children, taffy_style};
pub use style_mapping_and_chrome_metrics::{
    field_chrome_metrics, section_chrome_metrics, ChromeBand, SectionFieldChromeMetrics,
    FIELD_DETAIL_FONT_SIZE, FIELD_LABEL_FONT_SIZE, FIELD_LABEL_LINE_HEIGHT, SECTION_TITLE_BODY_GAP,
    SECTION_TITLE_FONT_SIZE, SECTION_TITLE_LINE_HEIGHT, SECTION_TRAILING_MARGIN,
};

#[path = "🧮️flex-tree/🦀️.rs"]
mod flex_tree;
pub use flex_tree::FlexTree;

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-flex-unit/🦀️.rs"]
mod tests;
// #endregion flex
