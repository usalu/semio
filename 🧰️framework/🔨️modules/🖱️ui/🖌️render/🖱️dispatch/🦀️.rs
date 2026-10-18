//! 🖱️ Hit testing, capture/target/bubble propagation, focus, pointer capture, drag/drop and
//! `DispatchOutcome` — the frame-local dispatch tree that replaces `wgpu-old`'s retained-mode
//! `events.rs`/`EventRouter`. Semantics are ported verbatim from that file; only the structure changes:
//! a frame-local [`DispatchTree`] of generic [`DispatchNode`]s with typed [`ListenerSet`]s stands in
//! for the retained `UiTree` of product-specific `UiNode` variants, and the persistent [`Dispatcher`]
//! stands in for `EventRouter`.
//!
//! **Read-only per frame, mutable across frames.** [`DispatchTree`] lives inside a `Rc<FrameSnapshot>`
//! (ruling U1's "dispatch always runs against the presented generation") and is never mutated by this
//! module — every old `EventRouter` behaviour that flipped a `NodeFlags` bit on the retained tree
//! (`ACTIVE`/`FOCUSED`/`HOVERED`/`OVERLAY`) instead becomes a read accessor on the persistent
//! [`Dispatcher`], keyed by [`ElementId`] (the one identity stable across a frame rebuild — see
//! `element.rs`'s own docstring). A widget `Element`'s own `prepaint`/`paint` consults those accessors
//! next frame to decide its own visual state; this module never reaches back into the tree it was
//! handed.
//!
//! **Absolute bounds simplify the port.** `events.rs`'s `layout.x`/`layout.y` were parent-relative,
//! which is why that file needed `node_abs_origin`'s parent-chain walk and `hit_test_subtree`'s
//! re-basing math. [`crate::identity::Hitbox::bounds`] is already window-absolute (the `bounds` param
//! `Element::prepaint` receives), so neither is needed here — `hit_test_subtree` degenerates to
//! `hit_test` with a different root.
//!
//! **Generic nodes, typed listeners.** The old `is_plain_stack_container` check pattern-matched
//! `UiNode::Stack`; this module has no product enum to match on, so the equivalent opt-in is the
//! [`DispatchFlags::LAYOUT_CONTAINER`] flag — an element that is *purely* layout (the old `Stack`) sets
//! it and becomes a hit-test pass-through unless it carries a binding of its own or is a drag source,
//! exactly `is_plain_stack_container`'s rule. Every other element (the old `Text`/`Button`/`Input`/…)
//! never sets it and stays a real hit-test target regardless of whether it has listeners — this is load
//! -bearing: ported test `hit_test_finds_the_topmost_of_two_non_overlapping_siblings` hits two bare
//! `Text`-equivalent leaves with zero bindings and still expects them to match.
//!
//! `ListenerSet` carries the contract's own [`ui_contract::ActionBinding`]s (already a closed,
//! product-agnostic `Trigger` enum — `Activate`/`Change`/`Commit`/`Delta`/`Drop`/`Submit`/`Abort`/
//! `RepeatLast`/`HoverPreview`) plus the protocol addressing (`surface`/`node`/`node_key`/`revision`)
//! needed to build a real [`ui_contract::UiIntent`] when one fires — the "typed listeners" the ticket
//! asks for, replacing the old `HitKind` enum without growing a new variant per product feature.
//!
//! 🚫️async: every `fn` below is plain sync per ruling U1 — a frame transaction (which input dispatch is
//! part of) is run-to-completion, never suspended. See ticket 26/08/20 📌️important.md.

use std::collections::HashMap;

use crate::identity::{Bounds, ElementId, Hitbox};
use crate::schedule::InvalidationReason;
use ui_contract::{ActionBinding, ActionId, FlowInline, SurfaceId, Trigger, UiIntent, UiNodeId, UiRevision, UiText, UiValue};

#[path = "🎯️hit-testing/🦀️.rs"]
mod hit_testing;
pub use hit_testing::{
    hit_test, hit_test_subtree, node_bounds, DispatchFlags, FocusId, FrameNodeId, HitboxId,
};
use hit_testing::rect_contains;

#[path = "🌿️listeners-and-dispatch-tree/🦀️.rs"]
mod listeners_and_dispatch_tree;
pub use listeners_and_dispatch_tree::{DispatchNode, DispatchTree, ListenerSet};

#[path = "🫧️events-bubble-and-capture/🦀️.rs"]
mod events_bubble_and_capture;
pub use events_bubble_and_capture::{
    bubble, AccessibilityEvent, AccessibilityTarget, CaptureKind, DispatchEvent, EventModifiers,
    ImeEvent, PointerButton, PointerId, PointerInfo, PointerKind, ScrollAxis, TextEditTarget,
};
use events_bubble_and_capture::{event_pointer, is_descendant, is_stale, CaptureEntry};

#[path = "🪟️overlays-scroll-and-edit-state/🦀️.rs"]
mod overlays_scroll_and_edit_state;
pub use overlays_scroll_and_edit_state::{
    resolve_overlay_placement, resolve_overlay_placement_side, EditState, OpenOverlay,
    OverlayAnchor,
};
use overlays_scroll_and_edit_state::{
    insert_at_caret, nearest_scrollable_ancestor, next_char_boundary, prev_char_boundary,
    selection_bounds, OverlayStack,
};

#[path = "🫳️drag-outcome-and-dispatcher-core/🦀️.rs"]
mod drag_outcome_and_dispatcher_core;
use drag_outcome_and_dispatcher_core::{
    collect_focusable, drag_payload_to_value, DRAG_PROMOTE_THRESHOLD_SQ,
};
pub use drag_outcome_and_dispatcher_core::{
    CursorRequest, DispatchOutcome, Dispatcher, DragGhost, DragPayload, DragSession, ImeDirective,
};

#[path = "🔦️focus-and-overlay-api/🦀️.rs"]
mod focus_and_overlay_api;

#[path = "✍️drag-scroll-and-edit-routing/🦀️.rs"]
mod drag_scroll_and_edit_routing;

#[path = "🎬️fire-cursor-and-dispatch/🦀️.rs"]
mod fire_cursor_and_dispatch;

//#region 🔖️Propagation

//#region 🪟️Overlay

/// 🪟️ The overlay vocabulary, its dismissal policy table and React's positioner all come from
/// `ui_contract::🪟️overlay` — the ONE implementation. This module used to carry a private, and by
/// 2026-09-17 diverged, copy (a three-variant `BelowAnchorWithFlip`/`AtPointer`/`Centered` model
/// that no longer matched React's `resolvePopoverPlacement`); ticket 26/09/17 packet W2k collapsed
/// both copies into the contract. Only [`OverlayAnchor`] stays local, because it names this module's
/// own [`ElementId`].
pub use ui_contract::{AnchoredPlacement, DismissPolicy, OverlayAlign, OverlayKind, OverlayPlacement, OverlayRect, OverlaySide, ResolvedOverlayPlacement, TOOLTIP_DWELL_SECONDS, TOOLTIP_HOVER_OUT_SECONDS};

//#endregion 🪟️Overlay

//#endregion 🔖️Propagation

/// 🧪️ Ports `events.rs`'s seven test regions onto [`DispatchTree`]/[`Dispatcher`] — they are this
/// port's specification (see the ticket). Built directly against hand-assembled [`DispatchTree`]s via
/// [`DispatchTree::insert`] — the same primitive `PrepaintCx::register` itself calls — exactly
/// as `events.rs`'s own tests built a `UiTree` by hand via a `leaf()` helper. `frame.rs`'s own test
/// module covers the *other* half: that a real `build_frame` call, going through prepaint registration
/// rather than a hand-built tree, produces the same kind of structure these tests assume.
#[cfg(test)]
#[path = "../🧪️tests/🔬️dispatch-unit/🦀️.rs"]
mod tests;
