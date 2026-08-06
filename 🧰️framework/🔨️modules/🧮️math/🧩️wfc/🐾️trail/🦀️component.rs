//! ↩️ The search trail: an append-only log of every pattern removal, grouped into decision frames
//! so backtracking can undo exactly one decision's consequences (including everything propagation
//! did because of it) in one call.

use crate::wfc::bitset::PatternSet;
use crate::wfc::domain::DomainStore;
use crate::wfc::ids::{DecisionId, NodeId, PatternId};
use crate::wfc::weights::WeightTable;

// #region 🔖️Entry
#[derive(Clone, Copy, PartialEq, Debug)]
struct RemovedEntry {
    node: NodeId,
    pattern: PatternId,
}

/// ↩️ Everything needed to undo one decision and retry with a different pattern: which node was
/// decided, which pattern was tried, where the trail stood before the decision, and the RNG state
/// right before that pattern was sampled (so a replay reproduces the exact same draw).
#[derive(Clone, Copy, Debug)]
// `decision`/`rng_state` are read by trace replay and checkpointing (a later phase); the current
// search driver uses the "keep RNG, don't restore" policy, so they're written but not yet read.
#[allow(dead_code)]
pub(crate) struct DecisionFrame {
    pub decision: DecisionId,
    pub node: NodeId,
    pub candidate: PatternId,
    pub trail_mark: usize,
    pub rng_state: [u64; 4],
}
// #endregion 🔖️Entry

// #region 🔖️Trail
/// ↩️ Append-only removal log plus a decision-frame stack.
#[derive(Clone, Debug, Default)]
pub(crate) struct Trail {
    entries: Vec<RemovedEntry>,
    frames: Vec<DecisionFrame>,
}

impl Trail {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_removed(&mut self, node: NodeId, pattern: PatternId) {
        self.entries.push(RemovedEntry { node, pattern });
    }

    pub fn record_removed_set(&mut self, node: NodeId, removed: &PatternSet) {
        for p in removed.iter_ones() {
            self.record_removed(node, p);
        }
    }

    pub fn push_frame(&mut self, decision: DecisionId, node: NodeId, candidate: PatternId, rng_state: [u64; 4]) {
        self.frames.push(DecisionFrame { decision, node, candidate, trail_mark: self.entries.len(), rng_state });
    }

    pub fn pop_frame(&mut self) -> Option<DecisionFrame> {
        self.frames.pop()
    }

    /// ↩️ Replays removal entries in exact reverse order down to (but not including) `mark`,
    /// re-adding each pattern. Order matters: entries must be undone LIFO so a pattern removed
    /// twice by different propagation steps is restored to the state each undo expects.
    pub fn undo_to(&mut self, mark: usize, domains: &mut DomainStore, w: &WeightTable) {
        while self.entries.len() > mark {
            let entry = self.entries.pop().expect("checked entries.len() > mark > 0 above");
            domains.get_mut(entry.node).re_add(entry.pattern, w);
        }
    }

    #[inline]
    #[allow(dead_code)] // used by checkpointing (a later phase); exercised today only by trail tests
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[inline]
    #[allow(dead_code)] // backs the public decision_depth() query added with the solver step API (a later phase)
    pub fn depth(&self) -> usize {
        self.frames.len()
    }

    /// ↩️ Every still-active decision's `(node, chosen-pattern)`, in the order each was made —
    /// exactly the combination [`crate::wfc::nogood::NogoodIndex::record`] needs to learn from a
    /// contradiction: these decisions, together, are what led to it.
    pub fn active_decisions(&self) -> Vec<(NodeId, PatternId)> {
        self.frames.iter().map(|f| (f.node, f.candidate)).collect()
    }
}
// #endregion 🔖️Trail

// #region 🔖️Checkpoint
/// ↩️ A resumable snapshot of a solve's domain state. Deliberately lighter than a full trail/
/// decision-stack serialization: resuming re-seeds a fresh search from these domains (via the same
/// `init_domains` path a heterogeneous-domain build already uses), so it is sound — the domains
/// already reflect every permanent removal made before the checkpoint — but backtracking after
/// resume can only undo decisions made *after* resume, not the ones baked into the snapshot.
#[derive(Clone, Debug)]
pub struct Checkpoint {
    pub domains: Vec<PatternSet>,
    pub model_fingerprint: u64,
    pub seed: u64,
}

impl Checkpoint {
    pub fn new(domains: Vec<PatternSet>, model_fingerprint: u64, seed: u64) -> Self {
        Self { domains, model_fingerprint, seed }
    }
}
// #endregion 🔖️Checkpoint

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wfc::weights::WeightTable;

    #[test]
    fn undo_to_restores_removed_patterns() {
        let w = WeightTable::new(&[1.0, 2.0, 3.0]).unwrap();
        let mut domains = DomainStore::new_full(1, &w);
        let mut trail = Trail::new();
        domains.get_mut(NodeId(0)).remove(PatternId(0), &w);
        trail.record_removed(NodeId(0), PatternId(0));
        domains.get_mut(NodeId(0)).remove(PatternId(1), &w);
        trail.record_removed(NodeId(0), PatternId(1));
        assert_eq!(domains.get(NodeId(0)).cardinality(), 1);

        trail.undo_to(0, &mut domains, &w);
        assert_eq!(domains.get(NodeId(0)).cardinality(), 3);
        domains.get(NodeId(0)).debug_assert_consistent(&w);
    }

    #[test]
    fn undo_to_partial_mark_restores_only_later_entries() {
        let w = WeightTable::new(&[1.0, 1.0, 1.0]).unwrap();
        let mut domains = DomainStore::new_full(1, &w);
        let mut trail = Trail::new();
        domains.get_mut(NodeId(0)).remove(PatternId(0), &w);
        trail.record_removed(NodeId(0), PatternId(0));
        let mark = trail.len();
        domains.get_mut(NodeId(0)).remove(PatternId(1), &w);
        trail.record_removed(NodeId(0), PatternId(1));

        trail.undo_to(mark, &mut domains, &w);
        assert_eq!(domains.get(NodeId(0)).cardinality(), 2); // pattern 1 restored, pattern 0 still gone
        assert!(!domains.get(NodeId(0)).bits().get(PatternId(0)));
        assert!(domains.get(NodeId(0)).bits().get(PatternId(1)));
    }

    #[test]
    fn decision_frames_push_and_pop() {
        let mut trail = Trail::new();
        trail.push_frame(DecisionId(0), NodeId(1), PatternId(2), [1, 2, 3, 4]);
        assert_eq!(trail.depth(), 1);
        let frame = trail.pop_frame().unwrap();
        assert_eq!(frame.node, NodeId(1));
        assert_eq!(frame.candidate, PatternId(2));
        assert_eq!(trail.depth(), 0);
        assert!(trail.pop_frame().is_none());
    }

    #[test]
    fn active_decisions_lists_every_frame_in_order() {
        let mut trail = Trail::new();
        assert!(trail.active_decisions().is_empty());
        trail.push_frame(DecisionId(0), NodeId(1), PatternId(2), [0; 4]);
        trail.push_frame(DecisionId(1), NodeId(3), PatternId(0), [0; 4]);
        assert_eq!(trail.active_decisions(), vec![(NodeId(1), PatternId(2)), (NodeId(3), PatternId(0))]);
        trail.pop_frame();
        assert_eq!(trail.active_decisions(), vec![(NodeId(1), PatternId(2))]);
    }
}
// #endregion 🔖️Tests
