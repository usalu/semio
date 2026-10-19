// #region events
//! 🎯️ Retained-mode input routing (`UiEvent` in, `UiCommand` out): reverse-paint-order hit testing
//! with clip pruning and overlay priority, pointer capture, Tab-order focus, and parent-chain
//! bubbling. Conceptually replaces the old immediate-mode `input` region's per-frame
//! `hit_targets`/`DragState` bookkeeping, but that region stays fully in place — `widgets`/`chrome`
//! and, transitively, `framework/renderer/wgpu` and `infinite_world` still consume it directly, and
//! the cutover to this module is later-phase renderer-thinning work (see the plan). `events` is
//! purely additive: it depends on `tree`/`component`/`geometry` only, never on `input`.

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

#[path = "🧭️event-model-and-focus/🦀️.rs"]
mod event_model_and_focus;
pub use event_model_and_focus::{
    hit_test, hit_test_subtree, node_abs_rect, AccessibilityUiEvent, CaptureKind, EventModifiers,
    FocusState, ImeEvent, PointerButton, ScrollAxis, UiEvent,
};
use event_model_and_focus::{find_tree_item_spec, hit_test_node, node_is_focusable, CaptureState};

#[path = "🧮️control-commit-rules/🦀️.rs"]
mod control_commit_rules;
use control_commit_rules::{
    absolute_rect, bare_action, cancelled_press, commits_on_blur, commits_while_dragging,
    constrain_slider_value, constrain_stepper_value, current_stored, descriptor_action_id,
    edit_commit_action, fired_action, is_continuous_control, is_descendant, number_field_text,
    number_stepper_can_step, number_stepper_live_value, number_stepper_sign_at, press_cancel,
    press_release, pressed, search_line_action, select_is_segmented, slider_key_value_from,
    slider_live_value, FiredAction, MirroredValue,
};
pub use control_commit_rules::{
    bubble, constrain_number_field, edit_refusal, editable_value, number_field_key,
    pointer_commit_action, slider_key_value, text_field_key, typed_number, SCRUB_ABORT_ARG,
    SCRUB_COMMIT_ARG, SCRUB_GESTURE_ARG,
};

#[path = "🧲️overlays-drag-and-commands/🦀️.rs"]
mod overlays_drag_and_commands;
pub use overlays_drag_and_commands::{
    disabled_row_action_reason, overlay_rect, resolve_anchored_placement, resolve_overlay_placement,
    resolve_overlay_placement_side, CaretSource, CaretStep, DragGhost, DragPayload, DragSession,
    EventRouter, OpenOverlay, OverlayAnchor, OverlayStack, RouterClockStep, RowReasonHint,
    TooltipStep, UiCommand, CARET_BLINK_SECONDS,
};
use overlays_drag_and_commands::{
    insert_at_caret, nearest_scrollable_ancestor, next_char_boundary, prev_char_boundary,
    selection_bounds, CaretClock, RetiringDragPayload, StepperRepeat, DRAG_PROMOTE_THRESHOLD_SQ,
    SLIDER_DOUBLE_CLICK_SECONDS, STEPPER_REPEAT_CLOCK_EPSILON_SECONDS, STEPPER_REPEAT_DELAY_SECONDS,
    STEPPER_REPEAT_INTERVAL_SECONDS,
};

#[path = "🏗️router-construction-and-intents/🦀️.rs"]
mod router_construction_and_intents;

#[path = "⏱️tooltip-clock-and-overlay-api/🦀️.rs"]
mod tooltip_clock_and_overlay_api;

#[path = "✍️drag-scroll-and-edit-api/🦀️.rs"]
mod drag_scroll_and_edit_api;

#[path = "🖱️accessibility-and-pointer-dispatch/🦀️.rs"]
mod accessibility_and_pointer_dispatch;

#[path = "⌨️widget-keyboard-and-commit/🦀️.rs"]
mod widget_keyboard_and_commit;

//#region 🔖️Overlay

/// 🪟️ The anchored-overlay placement vocabulary and its positioner are the CONTRACT's, not this
/// target's: `ui_contract::🪟️overlay` owns the one port of React's `resolvePopoverPlacement` so this
/// module and `🖌️render/🖱️dispatch` cannot drift apart again (ticket 26/09/17 packet W2k).
pub use ui_contract::{
    resolve_centered_placement, resolve_select_inline_left, AnchoredPlacement, DismissPolicy, OverlayAlign, OverlayKind, OverlayPlacement, OverlayRect, OverlaySide, ResolvedOverlayPlacement, TOOLTIP_DWELL_SECONDS, TOOLTIP_HOVER_OUT_SECONDS,
};
use semio_framework_ui_wgpu_widget_kit::constrain_number_input;

//#endregion 🔖️Overlay

// #endregion events
