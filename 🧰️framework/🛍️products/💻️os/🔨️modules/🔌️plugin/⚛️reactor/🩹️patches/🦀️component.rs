//! 🩹️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2, design-abi.md §4) / SEMANTIC-UI-CONTRACT-AND-
//! RENDERER-FAMILY (`sdk-flip`, 26/08/20): one `semio_framework_ui_runtime::SurfaceReconciler` per `(instance,
//! surface)` key — replaces the old full-body-`Replace`-only `PatchTracker`. `reconcile()` is the
//! real differ the old file's own header comment invited: keyed `(parent, key)` identity survives a
//! reorder, and a dirty surface now emits only the `UiPatchOp`s that actually changed (`SetStyle`,
//! `SetChildren`, …) instead of the whole tree every turn — see `ui_runtime`'s own `🦀️reconcile.rs`
//! module doc for the identity invariant this relies on.
//!
//! Deliberately `SurfaceReconciler`, not the full `semio_framework_ui_runtime::UiRuntime`: `UiRuntime` additionally
//! owns an `EntityStore`/`CommandGateway`/`PresenceHub`/`ProjectionInbox`, which need a per-plugin
//! `Present`/`HandleIntent` impl to mean anything — that is fleet-domain work this packet's `OWNS`
//! does not cover (see `📓️terra-sdk-flip-report.md`'s decisions section). `SurfaceReconciler` is the
//! exact piece that matches what this file always did: revisioned per-surface diffing off a tree the
//! plugin's own `render()` already produced.

use std::cell::RefCell;
use std::collections::HashMap;
use semio_framework_ui_contract as ui_contract;
use semio_framework_ui_runtime::{ComponentTree, SurfaceReconciler};

/// 🩹️ One per app instance — surfaces from different instances never share a reconciler, and
/// therefore never share revision counters or node-id allocators.
#[derive(Default)]
pub struct PatchTracker {
    reconcilers: RefCell<HashMap<String, SurfaceReconciler>>,
}

impl PatchTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🩹️ Reconciles `tree` against `surface`'s retained state and returns the minimal `UiPatch` to
    /// emit — `None` when `tree` is structurally and semantically identical to what was last
    /// presented. Lazily creates the surface's reconciler on first observation; that reconciler's own
    /// first `reconcile` call is what emits the initial full `SetRoot` plus one `Upsert` per node.
    pub fn diff(&self, surface: &str, tree: &ComponentTree) -> Option<ui_contract::UiPatch> {
        let mut reconcilers = self.reconcilers.borrow_mut();
        let reconciler = reconcilers.entry(surface.to_string()).or_insert_with(|| SurfaceReconciler::new(surface));
        reconciler.reconcile(tree)
    }

    /// 🩹️ `Event::PatchRejected` handling: drops the surface's retained state so the NEXT `diff()`
    /// call re-sends everything from a fresh `base_revision` of 0, matching the host's own reset
    /// expectation on rejection. A surface never yet observed simply has nothing to reset.
    pub fn mark_rejected(&self, surface: &str) {
        if let Some(reconciler) = self.reconcilers.borrow_mut().get_mut(surface) {
            reconciler.mark_rejected();
        }
    }

    /// 🩹️ `Event::PatchAck` handling — no-op today (the reconciler already advanced its revision
    /// optimistically in `diff()`); kept as a real entry point so a future ack-then-advance scheme
    /// doesn't need a new method name.
    pub fn mark_ack(&self, _surface: &str, _revision: u64) {}

    /// 🎯️ M1 (ticket 26/08/17 `design-unified.md`): the revision `ui_runtime::is_stale_intent`
    /// compares an incoming `UiIntent` against — reads through `SurfaceReconciler::snapshot`, the
    /// only accessor its landed API exposes for this (matching `ui_runtime`'s own
    /// `SurfaceSlot::current_revision` idiom, `🦀️transaction.rs:82-88`, which the same gap note
    /// leaves as a registrar-request for a cheaper accessor). A surface never yet observed reads as
    /// revision 0 — nothing has been sent to be stale against, so `is_stale_intent` never rejects it.
    pub fn revision(&self, surface: &str) -> ui_contract::UiRevision {
        self.reconcilers.borrow().get(surface).map(|reconciler| reconciler.snapshot().revision).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_ui_runtime::TreeNode;

    fn leaf(key: &str, text: &str) -> ComponentTree {
        ComponentTree::new(TreeNode::new(key, ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::from(text), emphasize: None, data_attributes: None })))
    }

    #[test]
    fn first_diff_for_a_surface_is_dirty_with_base_revision_zero() {
        let tracker = PatchTracker::new();
        let patch = tracker.diff("main", &leaf("root", "a")).expect("first observation of a surface must be dirty");
        assert_eq!(patch.base_revision, ui_contract::UiRevision(0));
        assert_eq!(patch.revision, ui_contract::UiRevision(1));
    }

    #[test]
    fn an_identical_tree_produces_no_patch() {
        let tracker = PatchTracker::new();
        tracker.diff("main", &leaf("root", "a"));
        assert!(tracker.diff("main", &leaf("root", "a")).is_none(), "an unchanged tree must not re-emit a patch");
    }

    #[test]
    fn a_changed_tree_advances_the_revision() {
        let tracker = PatchTracker::new();
        tracker.diff("main", &leaf("root", "a"));
        let patch = tracker.diff("main", &leaf("root", "b")).expect("a changed tree must be dirty");
        assert_eq!(patch.base_revision, ui_contract::UiRevision(1));
        assert_eq!(patch.revision, ui_contract::UiRevision(2));
    }

    /// 🎯️ M1: `revision()` is the accessor `⚛️reactor::poll`'s intent-batching loop guards every
    /// `UiIntent` against — a never-observed surface reads 0, and it tracks `diff()`'s own revision
    /// exactly, with no separate bookkeeping to drift.
    #[test]
    fn revision_reads_zero_for_a_never_observed_surface_and_tracks_diff_afterwards() {
        let tracker = PatchTracker::new();
        assert_eq!(tracker.revision("main"), ui_contract::UiRevision(0));
        tracker.diff("main", &leaf("root", "a"));
        assert_eq!(tracker.revision("main"), ui_contract::UiRevision(1));
        tracker.diff("main", &leaf("root", "b"));
        assert_eq!(tracker.revision("main"), ui_contract::UiRevision(2));
    }

    #[test]
    fn mark_rejected_resets_the_surface_to_base_revision_zero() {
        let tracker = PatchTracker::new();
        tracker.diff("main", &leaf("root", "a"));
        tracker.mark_rejected("main");
        let patch = tracker.diff("main", &leaf("root", "b")).expect("a surface must be dirty again right after rejection");
        assert_eq!(patch.base_revision, ui_contract::UiRevision(0));
    }

    /// ♻️ The property that makes this replacement worth shipping: a field-only change (style, not
    /// structure) emits a single targeted `SetStyle`, never a whole-tree `Upsert`/`SetRoot` pair.
    #[test]
    fn a_style_only_change_emits_a_targeted_set_style_not_a_full_replace() {
        let tracker = PatchTracker::new();
        tracker.diff("main", &leaf("root", "a"));
        let mut styled = leaf("root", "a");
        styled.root.style = ui_contract::StyleSpec { tone: ui_contract::Tone::Danger, ..Default::default() };
        let patch = tracker.diff("main", &styled).expect("a style change must be dirty");
        assert_eq!(patch.ops.len(), 1, "expected exactly one targeted op, got {:?}", patch.ops);
        assert!(matches!(patch.ops[0], ui_contract::UiPatchOp::SetStyle { .. }), "expected SetStyle, got {:?}", patch.ops[0]);
    }
}
