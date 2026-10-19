//! 🥣️ semio wgpu event router laws: hit test, capture, focus, overlays, drag and drop, keyboard and control commit.

#![cfg(test)]

use std::collections::{BTreeMap, HashMap};
use semio_framework_ui_arena::NodeId;
use semio_framework_ui_wgpu_layout::chrome::UiDriverDrag;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_declaration::ui::{SurfaceKind, UiNode, UiNumberStepperNode, UiSliderNode, UiState, UiTreeItemNode, UiTreeSectionNode};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_wgpu_layout::layout::{number_stepper_segments, ring_t_at, slider_control_presentation, tree_drag_handle_rect, tree_drag_handle_reservation, tree_drag_role, tree_section_header_band, tree_section_header_height, TreeRowMetrics};
use semio_framework_ui_wgpu_controls::select;
use semio_framework_ui_retained_tree::tree::{EditState, Node, NodeFlags, NodeKey, UiTree};
use semio_framework_ui_action::{intent_is_stale, UiIntentAddress, UiIntentCommand, UiIntentSequencer, INTENT_VALUE_FIELD};
use semio_framework_value::DslValue;
use ui_contract::{FlowInline, Trigger, UiFlow};
pub use ui_contract::{
    resolve_centered_placement, resolve_select_inline_left, AnchoredPlacement, DismissPolicy, OverlayAlign, OverlayKind, OverlayPlacement, OverlayRect, OverlaySide, ResolvedOverlayPlacement, TOOLTIP_DWELL_SECONDS, TOOLTIP_HOVER_OUT_SECONDS,
};
use semio_framework_ui_wgpu_widget_kit::constrain_number_input;
use semio_framework_ui_wgpu_events::*;
use semio_framework_ui_wgpu_events::FocusState;
use semio_framework_ui_wgpu_events::SCRUB_ABORT_ARG;
use semio_framework_ui_wgpu_events::SCRUB_COMMIT_ARG;
use semio_framework_ui_wgpu_events::SCRUB_GESTURE_ARG;
use semio_framework_ui_wgpu_events::bubble;
use semio_framework_ui_wgpu_events::constrain_number_field;
use semio_framework_ui_wgpu_events::number_field_key;
use semio_framework_ui_wgpu_events::pointer_commit_action;
use semio_framework_ui_wgpu_events::slider_key_value;
use semio_framework_ui_wgpu_events::text_field_key;

#[path = "../../../../../🧪️tests/🔬️targets-wgpu-events-unit/🦀️.rs"]
mod tests;

