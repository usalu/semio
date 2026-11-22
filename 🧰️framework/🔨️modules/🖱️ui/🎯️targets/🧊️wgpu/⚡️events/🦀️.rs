// #region events
//! 🎯️ Retained-mode input routing (`UiEvent` in, `UiCommand` out): reverse-paint-order hit testing
//! with clip pruning and overlay priority, pointer capture, Tab-order focus, and parent-chain
//! bubbling. Conceptually replaces the old immediate-mode `input` region's per-frame
//! `hit_targets`/`DragState` bookkeeping, but that region stays fully in place — `widgets`/`chrome`
//! and, transitively, `framework/renderer/wgpu` and `infinite_world` still consume it directly, and
//! the cutover to this module is later-phase renderer-thinning work (see the plan). `events` is
//! purely additive: it depends on `tree`/`component`/`geometry` only, never on `input`.

use std::collections::{BTreeMap, HashMap};

use semio_ui_arena::NodeId;
use semio_ui_wgpu_layout::chrome::UiDriverDrag;
use semio_ui_wgpu_light::tree_roving::TreeMerge;
use semio_ui_wgpu_light::tree_drop::{tree_palette_drag_begins, tree_palette_payload, TREE_CATALOGUE_DRAG_MIME};
use semio_ui_declaration::ui::{SurfaceKind, UiNode, UiState, UiTreeItemNode};
use semio_ui_theme::geometry::Rect;
use semio_ui_wgpu_layout::layout::{ring_t_at, slider_control_presentation, tree_drag_handle_rect, tree_drag_handle_reservation, tree_drag_role, tree_section_header_band, TreeRowMetrics};
use semio_ui_wgpu_controls::select;
use semio_ui_retained_tree::tree::{EditState, NodeFlags, UiTree};
use semio_ui_action::{intent_is_stale, UiIntentAddress, UiIntentCommand, UiIntentSequencer};
use semio_framework_value::DslValue;
use ui_contract::{FlowInline, Trigger, UiFlow};

pub use semio_ui_wgpu_events_focus::{
    text_field_key, hit_test, hit_test_subtree, node_abs_rect, AccessibilityUiEvent, CaptureKind, EventModifiers,
    FocusState, ImeEvent, PointerButton, ScrollAxis, UiEvent,
};
use semio_ui_wgpu_events_focus::{find_tree_item_spec, hit_test_node, node_is_focusable, CaptureState, tree_row_owner};

use semio_ui_wgpu_events_commit::{
    absolute_rect, bare_action, cancelled_press, commits_on_blur, commits_while_dragging,
    constrain_slider_value, constrain_stepper_value, current_stored, descriptor_action_id,
    edit_commit_action, fired_action, is_continuous_control, is_descendant, number_field_text,
    number_stepper_can_step, number_stepper_live_value, number_stepper_sign_at,
    pressed, search_line_action, select_is_segmented, slider_key_value_from,
    slider_live_value, FiredAction, MirroredValue,
};
pub use semio_ui_wgpu_events_commit::{
    bubble, constrain_number_field, edit_refusal, editable_value, number_field_key,
    pointer_commit_action, slider_key_value, typed_number, SCRUB_ABORT_ARG,
    SCRUB_COMMIT_ARG, SCRUB_GESTURE_ARG,
};

#[path = "🧲️overlays-drag-and-commands/🦀️.rs"]
mod overlays_drag_and_commands;
pub use overlays_drag_and_commands::{
    disabled_row_action_reason, overlay_rect, scrollbar_for, resolve_anchored_placement, resolve_overlay_placement,
    resolve_overlay_placement_side, CaretSource, CaretStep, DragGhost, DragPayload, DragSession,
    EventRouter, OpenOverlay, OverlayAnchor, OverlayStack, OverlayVeto, RouterClockStep, RowReasonHint,
    TooltipStep, UiCommand, CARET_BLINK_SECONDS,
};
use overlays_drag_and_commands::{
    insert_at_caret, line_end, line_start, word_range_at, nearest_scrollable_ancestor, next_char_boundary, prev_char_boundary,
    selection_bounds, vertical_target, word_end_after, word_start_after, word_start_before, CaretClock, ClickChain, PendingTextPress, RetiringDragPayload, StepperRepeat, TextDrag, DOUBLE_CLICK_SECONDS, DOUBLE_CLICK_SLOP_PX, DRAG_PROMOTE_THRESHOLD_SQ,
    SLIDER_DOUBLE_CLICK_SECONDS, STEPPER_REPEAT_CLOCK_EPSILON_SECONDS, STEPPER_REPEAT_DELAY_SECONDS,
    STEPPER_REPEAT_INTERVAL_SECONDS,
};

#[path = "🏗️router-construction-and-intents/🦀️.rs"]
mod router_construction_and_intents;

#[path = "⏱️tooltip-clock-and-overlay-api/🦀️.rs"]
mod tooltip_clock_and_overlay_api;

#[path = "✍️drag-scroll-and-edit-api/🦀️.rs"]
mod drag_scroll_and_edit_api;

#[path = "🫳️drag-drop/🦀️.rs"]
mod drag_drop;

#[path = "🖱️accessibility-and-pointer-dispatch/🦀️.rs"]
mod accessibility_and_pointer_dispatch;

#[path = "⌨️widget-keyboard-and-commit/🦀️.rs"]
mod widget_keyboard_and_commit;

#[path = "🌳️tree-roving-and-keys/🦀️.rs"]
mod tree_roving_and_keys;

//#region 🔖️Overlay

/// 🪟️ The anchored-overlay placement vocabulary and its positioner are the CONTRACT's, not this
/// target's: `ui_contract::🪟️overlay` owns the one port of React's `resolvePopoverPlacement` so this
/// module and `🖌️render/🖱️dispatch` cannot drift apart again (ticket 26/09/17 packet W2k).
pub use ui_contract::{
    resolve_centered_placement, resolve_select_inline_left, AnchoredPlacement, DismissPolicy, OverlayAlign, OverlayKind, OverlayPlacement, OverlayRect, OverlaySide, ResolvedOverlayPlacement, TOOLTIP_DWELL_SECONDS, TOOLTIP_HOVER_OUT_SECONDS,
};

//#endregion 🔖️Overlay

// #endregion events
