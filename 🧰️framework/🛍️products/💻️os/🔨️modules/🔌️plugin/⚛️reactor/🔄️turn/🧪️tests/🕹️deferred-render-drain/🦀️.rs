//! 🕹️ The deferred dirty-render drain: what one turn owes the surfaces whose re-render it had to refuse.
//!
//! A dirty render that arrives while that surface's own previous reconcile is still in flight is
//! REFUSED by `PatchTracker::reserve_mounted` and only deferred. For a document-scale world body that
//! is the ordinary case, so the drain behind it is the only thing that ever makes the re-render happen —
//! and it used to take exactly ONE deferred surface per turn, i.e. one host round trip per refused
//! body. Wave B48, ticket 26/09/02/PUZZLE-3D-END-TO-END.
use super::*;

fn surface_id(text: &str) -> ui_contract::SurfaceId {
    ui_contract::SurfaceId::try_from(text.to_string()).expect("fixture surface id fits the fixed text capacity")
}

/// 🕹️ Wave B48 LAW: one turn re-dirties EVERY deferred surface the host has acknowledged.
///
/// 🧾️ The observable is the count [`redirty_acknowledged_deferred_surfaces`] answers against the number
/// deferred. One `refreshUi` for `puzzle3d_selection_scope` dirties three world bodies
/// (`puzzle3d-main-top`, `puzzle3d-main-perspective` and the leftover `window` alias) plus three panel
/// bodies, so the pre-B48 pacing needed five further turns before the last of them re-rendered — with
/// the settle that asked for them (`🔌️PluginRuntime/🟦️.tsx` `settlePluginTurn`) naming no required
/// surface for a body that already has a root, and therefore free to return first and answer the host's
/// cached hash off the still-unrendered retained tree (wave B46 §8.1: `data-guest-selection-json` stays
/// `selectedIds:[]` for 150 s after a pick that settled).
#[test]
fn one_turn_redirties_every_acknowledged_deferred_surface() {
    let patches = patches::PatchTracker::new();
    let deferred = ["7:puzzle3d.play.composite", "7:puzzle3d-main-top", "7:puzzle3d-main-perspective", "7:window", "7:framework.panel.inspection", "7:puzzle.3d.play.document"];
    for surface in deferred {
        patches.defer(surface_id(surface)).expect("a fresh tracker's deferred ring admits every surface of one instance");
    }
    let mut dirty = DirtyPollOwners::new();
    let taken = redirty_acknowledged_deferred_surfaces(&patches, &mut dirty).expect("the drain fits the fixed dirty authority");
    assert_eq!(taken, deferred.len(), "a turn that refused {} renders must re-dirty all {} of them, not one", deferred.len(), deferred.len());
    assert_eq!(dirty.surfaces.iter().count(), deferred.len(), "every re-dirtied surface reaches this turn's render pass");
    for surface in deferred {
        assert!(dirty.surfaces.iter().any(|queued| queued.0 == 7 && queued.1.as_ref() == surface), "{surface} must be queued for THIS turn's render");
    }
    assert!(redirty_acknowledged_deferred_surfaces(&patches, &mut dirty).expect("a drained ring drains again") == 0, "the drain terminates: an entry it took can never be handed back");
}

/// 🧯️ Wave B48 LAW: a surface nobody deferred is never invented, and a drained ring leaves the dirty
/// authority alone — the drain is the only path from "render refused" to "render retried", so it may
/// neither lose an entry nor conjure one.
#[test]
fn a_drain_over_an_empty_deferred_ring_dirties_nothing() {
    let patches = patches::PatchTracker::new();
    let mut dirty = DirtyPollOwners::new();
    assert_eq!(redirty_acknowledged_deferred_surfaces(&patches, &mut dirty).expect("an empty drain"), 0);
    assert_eq!(dirty.surfaces.iter().count(), 0);
    patches.defer(surface_id("9:window")).expect("one deferred surface");
    assert_eq!(redirty_acknowledged_deferred_surfaces(&patches, &mut dirty).expect("one deferred surface drains"), 1);
    assert_eq!(dirty.surfaces.iter().count(), 1, "and it is queued exactly once");
}
