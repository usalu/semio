//! 🏔️ Two-level hierarchical generation: solve a coarse "macro" model first, then solve one
//! independent "micro" (child) model per macro node — the child model selected by a
//! caller-supplied mapping from the macro node's decided pattern. Each child is solved with a
//! seed deterministically derived from the macro solve's seed and the macro node's id, so
//! re-running with the same macro seed reproduces identical micro content everywhere.
//!
//! **Scope, stated explicitly**: this does not implement backtrack-to-macro (undoing and
//! retrying a *specific* macro decision when its child turns out unsatisfiable while keeping
//! every other macro decision fixed) or a boundary-contract mechanism (macro-adjacent micro
//! regions constraining each other's seam cells) — both need meaningfully deeper integration
//! with `crate::search`'s trail (essentially a `crate::repair`-style halo re-solve scoped to one
//! macro node's region, triggered from a different layer's failure) than a first hierarchical
//! pass warrants without a concrete consumer driving the requirements. What this module *does*
//! provide, and what every hierarchical generator needs regardless of that fallback machinery: a
//! sound way to solve the macro layer once, deterministically fan out to per-node child solves,
//! and report exactly which macro node's child failed if any did — `HierarchyOutcome::ChildFailed`
//! carries that node, so a caller can implement its own retry/fallback ladder on top (e.g. retry
//! the whole hierarchy with a new seed, or narrow `child_model_for` to avoid the failing pattern
//! at that node next time) without this module dictating the policy.

use crate::ids::{NodeId, PatternId};
use crate::model::CompiledModel;
use crate::outcome::{Solution, SolveOutcome};
use crate::search::{self, SearchConfig};
use crate::topology::Topology;

// #region 🔖️Outcome
/// 🏔️ The result of a two-level hierarchical solve.
#[allow(dead_code)] // no solver-level public wrapper yet (deferred — see this module's docs); exercised today only by this module's own tests
pub(crate) enum HierarchyOutcome {
    /// 🏔️ The macro layer solved and every macro node's child model also solved.
    Solved {
        macro_solution: Solution,
        /// 🏔️ Indexed by macro `NodeId`; `children[i]` is macro node `i`'s child solution.
        children: Vec<Solution>,
    },
    /// 🏔️ The macro layer itself had no solution — nothing below it was attempted.
    MacroUnsatisfiable,
    /// 🏔️ The macro layer solved, but the child model at `node` (given its macro-decided
    /// pattern) had no solution. Every macro node before `node` (in iteration order) already
    /// solved its child successfully; nothing after `node` was attempted.
    ChildFailed { node: NodeId },
}
// #endregion 🔖️Outcome

// #region 🔖️Solve
/// 🏔️ Deterministically derives macro node `node`'s child seed from the macro solve's own seed —
/// a single splitmix64-style mixing step (matching `crate::chunk::chunk_seed`'s approach), so the
/// same macro seed always regenerates identical children.
#[allow(dead_code)] // exercised today only by this module's own tests; see HierarchyOutcome's note
fn child_seed(macro_seed: u64, node: NodeId) -> u64 {
    let mut z = macro_seed ^ (node.get() as u64).wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// 🏔️ Solves `macro_model`/`macro_topo`, then for every macro node calls `child_model_for(node,
/// macro_pattern)` to get that node's child model/topology/config, solving each independently
/// with a seed derived from `macro_seed`. Stops at the first child failure (see
/// [`HierarchyOutcome::ChildFailed`]) rather than attempting the remaining nodes — a partially-
/// generated hierarchy usually isn't useful to a caller that hasn't decided its own fallback
/// policy yet.
#[allow(dead_code)] // exercised today only by this module's own tests; see HierarchyOutcome's note
pub(crate) fn solve_hierarchy<MT, CT>(macro_model: &CompiledModel, macro_topo: &MT, macro_config: &SearchConfig, macro_seed: u64, child_model_for: impl Fn(NodeId, PatternId) -> (CompiledModel, CT, SearchConfig)) -> HierarchyOutcome
where
    MT: Topology,
    CT: Topology,
{
    let macro_solution = match search::solve(macro_model, macro_topo, macro_config, macro_seed, None, &[]) {
        SolveOutcome::Solved(sol) => sol,
        _ => return HierarchyOutcome::MacroUnsatisfiable,
    };

    let mut children = Vec::with_capacity(macro_solution.assignment.len());
    for (i, &pattern) in macro_solution.assignment.iter().enumerate() {
        let node = NodeId::from_index(i);
        let (child_model, child_topo, child_config) = child_model_for(node, pattern);
        let seed = child_seed(macro_seed, node);
        match search::solve(&child_model, &child_topo, &child_config, seed, None, &[]) {
            SolveOutcome::Solved(sol) => children.push(sol),
            _ => return HierarchyOutcome::ChildFailed { node },
        }
    }
    HierarchyOutcome::Solved { macro_solution, children }
}
// #endregion 🔖️Solve

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ModelBuilder;
    use crate::topology::{GraphTopology, GraphTopologyBuilder};

    fn checkerboard(n: usize) -> (CompiledModel, GraphTopology) {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(n);
        for i in 0..n.saturating_sub(1) {
            tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
            tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
        }
        (model, tb.build().unwrap())
    }

    fn single_node_model(pattern_count: usize) -> (CompiledModel, GraphTopology) {
        let mut b = ModelBuilder::new();
        for _ in 0..pattern_count {
            b.add_pattern(1.0);
        }
        b.add_relation("r");
        (b.compile().unwrap(), GraphTopologyBuilder::new(1).build().unwrap())
    }

    fn always_satisfiable_child(_node: NodeId, _pattern: PatternId) -> (CompiledModel, GraphTopology, SearchConfig) {
        let (model, topo) = single_node_model(2);
        (model, topo, SearchConfig::default())
    }

    #[test]
    fn solves_macro_then_every_child() {
        let (model, topo) = checkerboard(4);
        let config = SearchConfig::default();
        match solve_hierarchy(&model, &topo, &config, 1, always_satisfiable_child) {
            HierarchyOutcome::Solved { macro_solution, children } => {
                assert_eq!(macro_solution.assignment.len(), 4);
                assert_eq!(children.len(), 4);
                for child in &children {
                    assert_eq!(child.assignment.len(), 1);
                }
            }
            HierarchyOutcome::MacroUnsatisfiable => panic!("expected Solved, got MacroUnsatisfiable"),
            HierarchyOutcome::ChildFailed { node } => panic!("expected Solved, got ChildFailed at {node}"),
        }
    }

    #[test]
    fn reports_macro_unsatisfiable_without_attempting_children() {
        // K5 with only 4 colors: unsatisfiable regardless of any child model.
        let mut b = ModelBuilder::new();
        let patterns: Vec<_> = (0..4).map(|_| b.add_pattern(1.0)).collect();
        let ne = b.add_relation("ne");
        for &a in &patterns {
            for &c in &patterns {
                if a != c {
                    b.allow(ne, a, c);
                }
            }
        }
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(5);
        for i in 0..5 {
            for j in (i + 1)..5 {
                tb.arc(NodeId::from_index(i), NodeId::from_index(j), ne);
                tb.arc(NodeId::from_index(j), NodeId::from_index(i), ne);
            }
        }
        let topo = tb.build().unwrap();
        let config = SearchConfig::default();

        let never_called = |_: NodeId, _: PatternId| -> (CompiledModel, GraphTopology, SearchConfig) { panic!("child should never be attempted") };
        let outcome = solve_hierarchy(&model, &topo, &config, 1, never_called);
        assert!(matches!(outcome, HierarchyOutcome::MacroUnsatisfiable));
    }

    /// 🧪️ A two-node model whose single relation allows nothing at all — any arc using it is
    /// unsatisfiable, regardless of pattern count.
    fn unsatisfiable_child(_node: NodeId, _pattern: PatternId) -> (CompiledModel, GraphTopology, SearchConfig) {
        let mut b = ModelBuilder::new();
        b.add_pattern(1.0);
        let never = b.add_relation("never");
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(2);
        tb.arc(NodeId(0), NodeId(1), never);
        tb.arc(NodeId(1), NodeId(0), never);
        (model, tb.build().unwrap(), SearchConfig::default())
    }

    #[test]
    fn reports_which_node_child_failed_at() {
        let (model, topo) = checkerboard(3);
        let config = SearchConfig::default();
        let child_model_for = |node: NodeId, pattern: PatternId| -> (CompiledModel, GraphTopology, SearchConfig) {
            if node == NodeId(1) {
                unsatisfiable_child(node, pattern)
            } else {
                always_satisfiable_child(node, pattern)
            }
        };

        let outcome = solve_hierarchy(&model, &topo, &config, 1, child_model_for);
        assert!(matches!(outcome, HierarchyOutcome::ChildFailed { node: NodeId(1) }));
    }

    #[test]
    fn child_seeds_differ_by_node_and_reproduce_deterministically() {
        assert_eq!(child_seed(1, NodeId(0)), child_seed(1, NodeId(0)));
        assert_ne!(child_seed(1, NodeId(0)), child_seed(1, NodeId(1)));
        assert_ne!(child_seed(1, NodeId(0)), child_seed(2, NodeId(0)));
    }
}
// #endregion 🔖️Tests
