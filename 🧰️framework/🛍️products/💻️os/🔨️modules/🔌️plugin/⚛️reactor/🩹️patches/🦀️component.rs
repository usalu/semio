//! 🩹️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §4). Revisioned per-surface UI
//! patches: `last` tracks `(revision, UiNode)` per `(instance, surface)` so a dirty surface can be
//! diffed against what the host last acknowledged instead of resent whole every turn.
//!
//! ⚠️ Scope note (reported honestly, not silently simplified): this wave ships the
//! `revision`/`base_revision` bookkeeping and the dirty-surface tracking real and working, but the
//! diff itself is **full-body only** — every dirty surface emits one `PatchOp::Replace` at the
//! root path with the whole new `UiNode`, never a node-identity-path partial diff. The ~60%-of-
//! body fallback threshold design-abi.md §4 describes has nothing to fall back FROM yet because
//! there is no partial diff implemented. Node-identity-path diffing is real, nontrivial tree-walk
//! work or genuine test coverage against `PatchRejected`'s `base_revision` mismatch path — out of
//! this wave's remaining budget; the `PatchTracker` shape below is written so that swapping
//! `full_replace` for a real differ is a one-function change, not a redesign.

use semio_framework::kernel::{PatchOp, UiPatch};
use std::cell::RefCell;
use std::collections::HashMap;
use ui_wgpu::wgpu::UiNode;

#[derive(Clone)]
struct SurfaceState {
    revision: u64,
    body: UiNode,
}

/// 🩹️ One per app instance — surfaces from different instances never share revision counters.
#[derive(Default)]
pub struct PatchTracker {
    last: RefCell<HashMap<String, SurfaceState>>,
}

impl PatchTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🩹️ Records `body` as `surface`'s new state and returns the `UiPatch` to emit — `None` when
    /// `body` is identical to what was last recorded (nothing dirty).
    pub fn diff(&self, surface: &str, body: UiNode) -> Option<UiPatch> {
        let mut last = self.last.borrow_mut();
        let base_revision = last.get(surface).map(|state| state.revision).unwrap_or(0);
        let unchanged = last.get(surface).map(|state| state.body == body).unwrap_or(false);
        if unchanged {
            return None;
        }
        let revision = base_revision + 1;
        last.insert(surface.to_string(), SurfaceState { revision, body: body.clone() });
        Some(UiPatch { surface: surface.to_string(), kind: "root".to_string(), revision, base_revision, ops: vec![PatchOp::Replace { path: String::new(), node: body }] })
    }

    /// 🩹️ `Event::PatchRejected` handling: the guest resends a full body unconditionally (never a
    /// diff) — dropping the tracked revision back to 0 forces the NEXT `diff()` call for this
    /// surface to treat everything as dirty again with a fresh `base_revision` of 0, matching the
    /// host's own reset expectation on rejection.
    pub fn mark_rejected(&self, surface: &str) {
        self.last.borrow_mut().remove(surface);
    }

    /// 🩹️ `Event::PatchAck` handling — no-op today (the tracker already advanced its revision
    /// optimistically in `diff()`); kept as a real entry point so a future ack-then-advance
    /// scheme doesn't need a new method name.
    pub fn mark_ack(&self, _surface: &str, _revision: u64) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_wgpu::wgpu::{Label, UiPresence, UiTextNode};

    /// 🧪️ `UiNode` is a big variant enum (`Text`/`Button`/`Stack`/...), not a plain struct — this
    /// mirrors the exact `UiNode::Text(UiTextNode { .. })` construction already used elsewhere in
    /// the codebase (e.g. `🖱️ui/🧱️elements/👥️PresenceBar/🧊️component.rs`).
    async fn node(text: &str) -> UiNode {
        UiNode::Text(UiTextNode { value: Label::data(text).await, emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
    }

    #[semio_framework_async_macros::async_test]
    async fn first_diff_for_a_surface_is_dirty_with_base_revision_zero() {
        let tracker = PatchTracker::new();
        let patch = tracker.diff("main", node("a").await).expect("first observation of a surface must be dirty");
        assert_eq!(patch.base_revision, 0);
        assert_eq!(patch.revision, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn an_identical_body_produces_no_patch() {
        let tracker = PatchTracker::new();
        tracker.diff("main", node("a").await);
        assert!(tracker.diff("main", node("a").await).is_none(), "an unchanged body must not re-emit a patch");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_changed_body_advances_the_revision() {
        let tracker = PatchTracker::new();
        tracker.diff("main", node("a").await);
        let patch = tracker.diff("main", node("b").await).expect("a changed body must be dirty");
        assert_eq!(patch.base_revision, 1);
        assert_eq!(patch.revision, 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn mark_rejected_resets_the_surface_to_base_revision_zero() {
        let tracker = PatchTracker::new();
        tracker.diff("main", node("a").await);
        tracker.mark_rejected("main");
        let patch = tracker.diff("main", node("b").await).expect("a surface must be dirty again right after rejection");
        assert_eq!(patch.base_revision, 0);
    }
}
