//! 🌊️🔦️ Beam search: an incomplete alternative to full backtracking. Maintains up to `width`
//! independent partial-domain states ("beams"); each round, every surviving beam's heuristically
//! chosen frontier node is expanded into one child beam per still-possible pattern (not just one
//! sampled value — the width of exploration comes from branching wide then pruning, not from
//! random sampling), all children are scored, and only the best `width` survive into the next
//! round. Trades completeness (a beam that was actually still solvable can be pruned away) for
//! bounded memory and no exponential blowup.
//!
//! Unlike [`crate::search`]'s kernel, a dropped beam is never revisited: this is **not** a
//! sound-or-complete search strategy. [`beam_search`] never returns `Unsatisfiable` — only
//! `Solved` (some beam reached an all-singleton, arc-consistent state) or `Contradiction` (every
//! beam died, or `max_steps` ran out, before any reached completion). A `Contradiction` from this
//! function is not proof the model is unsatisfiable, only that beam search didn't find a solution
//! at this `width`/`max_steps`.

use crate::bitset::PatternSet;
use crate::diag::Metrics;
use crate::domain::{DomainStore, RestrictResult};
use crate::heuristics::{self, ObserveHeuristic};
use crate::ids::{NodeId, PatternId};
use crate::model::CompiledModel;
use crate::outcome::{ContradictionReport, RunReport, Solution, SolveOutcome};
use crate::prop_ac3;
use crate::propagate::PropQueue;
use crate::topology::Topology;
use crate::trail::Trail;
use mathematical_random::Rng;

// #region 🔖️Config
#[derive(Clone, Copy, Debug)]
pub struct BeamConfig {
    /// 🌊️🔦️ Maximum number of beams kept after each round's prune.
    pub width: usize,
    /// 🌊️🔦️ Hard cap on rounds (roughly one decision per surviving beam per round).
    pub max_steps: usize,
    pub heuristic: ObserveHeuristic,
}

impl Default for BeamConfig {
    fn default() -> Self {
        Self { width: 4, max_steps: 100_000, heuristic: ObserveHeuristic::default() }
    }
}
// #endregion 🔖️Config

// #region 🔖️Search
#[derive(Clone)]
struct Beam {
    domains: DomainStore,
}

/// 🌊️🔦️ Higher is better: fewer total remaining candidates across every domain, i.e. closer to a
/// complete assignment. A cheap, purely domain-shape-based progress proxy — no soft-constraint
/// scoring hook yet (deferred: a caller wanting soft-guided beam search can layer
/// `crate::soft::SoftConstraint` scoring on top once there's a concrete consumer).
fn score(domains: &DomainStore) -> f64 {
    -(domains.iter().map(|(_, d)| (d.cardinality().max(1) - 1) as f64).sum::<f64>())
}

fn build_root<T: Topology>(model: &CompiledModel, topo: &T, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)], metrics: &mut Metrics) -> Option<DomainStore> {
    let node_count = topo.node_count();
    let w = model.weights();
    let mut domains = DomainStore::new_full(node_count, w);
    let mut trail = Trail::new();
    let mut wiped = false;

    if let Some(overrides) = init_domains {
        for (i, allowed) in overrides.iter().enumerate() {
            if matches!(domains.get_mut(NodeId::from_index(i)).restrict(allowed, w), RestrictResult::Wipeout) {
                wiped = true;
            }
        }
    }
    for &(n, p) in fixed {
        if matches!(domains.get_mut(n).assign(p, w), RestrictResult::Wipeout) {
            wiped = true;
        }
    }

    let mut queue = PropQueue::new(node_count);
    queue.push_all(node_count);
    if !wiped && prop_ac3::run_to_fixed_point(model, topo, &mut domains, &mut queue, &mut trail, metrics).is_err() {
        wiped = true;
    }
    if wiped { None } else { Some(domains) }
}

/// 🌊️🔦️ Runs beam search to either a solution or exhaustion of every beam / `max_steps`. See this
/// module's docs for the exact (incomplete) guarantees.
pub(crate) fn beam_search<T: Topology>(model: &CompiledModel, topo: &T, beam_config: BeamConfig, seed: u64, init_domains: Option<&[PatternSet]>, fixed: &[(NodeId, PatternId)]) -> SolveOutcome {
    let fingerprint = model.fingerprint();
    let mut metrics = Metrics::default();
    let contradiction = |metrics: Metrics| SolveOutcome::Contradiction(ContradictionReport { node: NodeId(0), report: RunReport { metrics, model_fingerprint: fingerprint, seed, events: Vec::new() } });

    let Some(root_domains) = build_root(model, topo, init_domains, fixed, &mut metrics) else {
        return contradiction(metrics);
    };
    let mut beams = vec![Beam { domains: root_domains }];
    let mut rng = Rng::from_seed(seed);
    let w = model.weights();

    for _ in 0..beam_config.max_steps {
        if let Some(i) = beams.iter().position(|b| b.domains.all_singleton()) {
            let assignment: Vec<PatternId> = beams[i].domains.iter().map(|(_, d)| d.singleton().expect("all_singleton guarantees a singleton")).collect();
            return SolveOutcome::Solved(Solution { assignment, report: RunReport { metrics, model_fingerprint: fingerprint, seed, events: Vec::new() } });
        }

        let mut candidates: Vec<Beam> = Vec::new();
        for beam in &beams {
            let Some(node) = heuristics::select_unresolved(beam_config.heuristic, &beam.domains) else { continue };
            let live_patterns: Vec<PatternId> = beam.domains.get(node).bits().iter_ones().collect();
            for pattern in live_patterns {
                let mut next_domains = beam.domains.clone();
                let mut next_trail = Trail::new();
                let mut next_queue = PropQueue::new(next_domains.len());
                metrics.observations += 1;
                next_domains.get_mut(node).assign(pattern, w);
                next_queue.push(node);
                if prop_ac3::run_to_fixed_point(model, topo, &mut next_domains, &mut next_queue, &mut next_trail, &mut metrics).is_ok() {
                    candidates.push(Beam { domains: next_domains });
                }
            }
        }
        if candidates.is_empty() {
            return contradiction(metrics);
        }

        // Shuffle before the stable sort so equally-scored candidates aren't always ordered by
        // ascending pattern id — `seed` gives every beam search a genuinely different tie-break
        // rather than a purely deterministic-by-pattern-index one.
        rng.shuffle(&mut candidates);
        candidates.sort_by(|a, b| score(&b.domains).partial_cmp(&score(&a.domains)).expect("score is never NaN"));
        candidates.truncate(beam_config.width.max(1));
        beams = candidates;
    }
    contradiction(metrics)
}
// #endregion 🔖️Search

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ModelBuilder;
    use crate::oracle;
    use crate::topology::GraphTopologyBuilder;

    fn checkerboard(n: usize) -> (CompiledModel, crate::topology::GraphTopology, Vec<oracle::ArcSpec>) {
        let mut b = ModelBuilder::new();
        let black = b.add_pattern(1.0);
        let white = b.add_pattern(1.0);
        let adj = b.add_relation("adjacent");
        b.allow_mirrored(adj, black, white);
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(n);
        let mut arcs = Vec::new();
        for i in 0..n.saturating_sub(1) {
            tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
            tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
            arcs.push(oracle::ArcSpec { from: NodeId::from_index(i), to: NodeId::from_index(i + 1), relation: adj });
            arcs.push(oracle::ArcSpec { from: NodeId::from_index(i + 1), to: NodeId::from_index(i), relation: adj });
        }
        (model, tb.build().unwrap(), arcs)
    }

    fn k_graph(n: usize, k: usize) -> (CompiledModel, crate::topology::GraphTopology, Vec<oracle::ArcSpec>) {
        let mut b = ModelBuilder::new();
        let patterns: Vec<_> = (0..k).map(|_| b.add_pattern(1.0)).collect();
        let ne = b.add_relation("ne");
        for &a in &patterns {
            for &c in &patterns {
                if a != c {
                    b.allow(ne, a, c);
                }
            }
        }
        let model = b.compile().unwrap();
        let mut tb = GraphTopologyBuilder::new(n);
        let mut arcs = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                let a = NodeId::from_index(i);
                let c = NodeId::from_index(j);
                tb.arc(a, c, ne);
                tb.arc(c, a, ne);
                arcs.push(oracle::ArcSpec { from: a, to: c, relation: ne });
                arcs.push(oracle::ArcSpec { from: c, to: a, relation: ne });
            }
        }
        (model, tb.build().unwrap(), arcs)
    }

    #[test]
    fn finds_a_valid_solution_on_a_satisfiable_instance() {
        let (model, topo, arcs) = checkerboard(8);
        let config = BeamConfig::default();
        for seed in 0..10 {
            match beam_search(&model, &topo, config, seed, None, &[]) {
                SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok(), "seed {seed}: invalid solution"),
                other => panic!("seed {seed}: expected Solved, got {other:?}"),
            }
        }
    }

    #[test]
    fn finds_a_valid_solution_on_a_harder_coloring_instance() {
        let (model, topo, arcs) = k_graph(4, 4);
        let config = BeamConfig { width: 6, ..Default::default() };
        match beam_search(&model, &topo, config, 7, None, &[]) {
            SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok()),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn reports_contradiction_not_a_panic_on_an_unsatisfiable_instance() {
        // K5 needs 5 colors, only 4 available: genuinely unsatisfiable. Beam search must still
        // terminate cleanly and report Contradiction, never claim Solved or panic.
        let (model, topo, _arcs) = k_graph(5, 4);
        let config = BeamConfig { width: 4, max_steps: 50, ..Default::default() };
        let outcome = beam_search(&model, &topo, config, 1, None, &[]);
        assert!(matches!(outcome, SolveOutcome::Contradiction(_)), "expected Contradiction, got {outcome:?}");
    }

    #[test]
    fn width_one_still_finds_a_solution_when_no_backtrack_is_needed() {
        // A checkerboard path never needs more than one live branch at a time (propagation alone
        // forces every other node), so width=1 should still succeed here.
        let (model, topo, arcs) = checkerboard(10);
        let config = BeamConfig { width: 1, ..Default::default() };
        match beam_search(&model, &topo, config, 3, None, &[]) {
            SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok()),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn respects_fixed_pins() {
        let (model, topo, _arcs) = checkerboard(4);
        let config = BeamConfig::default();
        match beam_search(&model, &topo, config, 1, None, &[(NodeId(0), PatternId(0))]) {
            SolveOutcome::Solved(sol) => assert_eq!(sol.assignment[0], PatternId(0)),
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn contradictory_fixed_pins_report_contradiction_immediately() {
        let (model, topo, _arcs) = checkerboard(2);
        let config = BeamConfig::default();
        // Both nodes pinned to the same pattern is impossible under the mirrored adjacency rule.
        let outcome = beam_search(&model, &topo, config, 1, None, &[(NodeId(0), PatternId(0)), (NodeId(1), PatternId(0))]);
        assert!(matches!(outcome, SolveOutcome::Contradiction(_)));
    }
}
// #endregion 🔖️Tests
