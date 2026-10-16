//! 🕹️ Input and timing for one frame: stable element identity, the dispatch tree with hit-testing,
//! focus, drag, overlay and edit routing, and the invalidation/deadline scheduler that decides when a
//! window renders again.
//!
//! Sits above the packet crate (an element's [`Bounds`] is its [`LayoutRect`](semio_framework_ui_render_packet::LayoutRect))
//! and below the element model, which registers into the dispatch tree during prepaint.

#[path = "../🖱️dispatch/🦀️.rs"]
mod dispatch;
#[path = "../🪪️identity/🦀️.rs"]
mod identity;
#[path = "../⏱️schedule/🦀️.rs"]
mod schedule;

pub use dispatch::{
    bubble, hit_test, hit_test_subtree, node_bounds, resolve_overlay_placement, resolve_overlay_placement_side, AccessibilityEvent, AccessibilityTarget, AnchoredPlacement, CaptureKind, CursorRequest, DismissPolicy, DispatchEvent, DispatchFlags,
    DispatchNode, DispatchOutcome, DispatchTree, Dispatcher, DragGhost, DragPayload, DragSession, EditState, EventModifiers, FocusId, FrameNodeId, HitboxId, ImeDirective, ImeEvent, ListenerSet, OpenOverlay, OverlayAlign, OverlayAnchor, OverlayKind,
    OverlayPlacement, OverlayRect, OverlaySide, PointerButton, PointerId, PointerInfo, PointerKind, ResolvedOverlayPlacement, ScrollAxis, TextEditTarget, TOOLTIP_DWELL_SECONDS, TOOLTIP_HOVER_OUT_SECONDS,
};
pub use identity::{Bounds, ElementId, Hitbox, ReconciliationKey};
pub use schedule::{Deadline, DeadlineKey, FrameScheduler, InvalidationReason};
