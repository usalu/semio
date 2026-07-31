//! 🧱️ `Grid2dSolver`: dense 2D grid solving on top of [`crate::grid2d`] and the shared kernel.
//! Masked-out cells and [`Boundary::FixedOutside`] edges are folded into ordinary domain overrides
//! and fixed pins before delegating to the same generic [`crate::search::solve`] every solver uses.

use crate::bitset::PatternSet;
use crate::chunk;
use crate::constraint::{AdjacencyView, Constraint, ConstraintSet, build_adjacency_view};
use crate::error::SolveError;
use crate::grid2d::Grid2dTopology;
use crate::ids::PatternId;
use crate::model::CompiledModel;
use crate::outcome::{Solution, SolveOutcome};
use crate::search::{self, CancelToken, SearchConfig};
use crate::topology::Topology;

// #region 🔖️Builder
/// 🏗️ Builds a [`Grid2dSolver`] over a dense `width × height` grid.
pub struct Grid2dSolverBuilder {
    model: CompiledModel,
    topology: Grid2dTopology,
    init_domains: Option<Vec<PatternSet>>,
    fixed: Vec<(crate::ids::NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Box<dyn Constraint>>,
}

impl Grid2dSolverBuilder {
    pub fn new(model: CompiledModel, topology: Grid2dTopology) -> Self {
        Self { model, topology, init_domains: None, fixed: Vec::new(), config: SearchConfig::default(), constraints: Vec::new() }
    }

    pub fn fix(mut self, x: usize, y: usize, p: PatternId) -> Result<Self, SolveError> {
        let n = self.topology.node_at(x, y).ok_or(SolveError::ModelTopologyMismatch { reason: "fix() coordinate out of range" })?;
        self.fixed.push((n, p));
        Ok(self)
    }

    pub fn domain(mut self, x: usize, y: usize, allowed: PatternSet) -> Result<Self, SolveError> {
        let n = self.topology.node_at(x, y).ok_or(SolveError::ModelTopologyMismatch { reason: "domain() coordinate out of range" })?;
        let node_count = self.topology.node_count();
        let domains = self.init_domains.get_or_insert_with(|| vec![self.model.full_domain(); node_count]);
        domains[n.index()] = allowed;
        Ok(self)
    }

    pub fn config(mut self, cfg: SearchConfig) -> Self {
        self.config = cfg;
        self
    }

    /// 🏗️ Adds a global constraint. See [`crate::constraint::Constraint`]'s docs for exactly when
    /// it runs (initial restriction + complete-assignment validation, not incremental mid-search).
    pub fn constraint(mut self, c: Box<dyn Constraint>) -> Self {
        self.constraints.push(c);
        self
    }

    pub fn build(self) -> Result<Grid2dSolver, SolveError> {
        let node_count = self.topology.node_count();
        let mut init_domains = self.init_domains.unwrap_or_else(|| vec![self.model.full_domain(); node_count]);
        let mut fixed = self.fixed;

        for (n, rel, outside_pattern) in self.topology.fixed_outside_restrictions() {
            init_domains[n.index()].and_with(self.model.allowed(rel, outside_pattern));
        }
        let placeholder = PatternId(0);
        for n in self.topology.inactive_cells() {
            fixed.push((n, placeholder));
        }

        let adjacency = build_adjacency_view(&self.topology);
        Ok(Grid2dSolver { model: self.model, topology: self.topology, init_domains, fixed, config: self.config, constraints: self.constraints, adjacency })
    }
}
// #endregion 🔖️Builder

// #region 🔖️Solver
/// 🧱️ A WFC solver over a dense 2D grid.
pub struct Grid2dSolver {
    model: CompiledModel,
    topology: Grid2dTopology,
    init_domains: Vec<PatternSet>,
    fixed: Vec<(crate::ids::NodeId, PatternId)>,
    config: SearchConfig,
    constraints: Vec<Box<dyn Constraint>>,
    adjacency: AdjacencyView,
}

impl Grid2dSolver {
    fn constraint_set(&self) -> Option<ConstraintSet<'_>> {
        if self.constraints.is_empty() { None } else { Some(ConstraintSet { constraints: &self.constraints, adjacency: &self.adjacency }) }
    }

    pub fn solve(&mut self, seed: u64) -> SolveOutcome {
        match self.constraint_set() {
            Some(cs) => search::solve_with_constraints(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, None, &cs),
            None => search::solve(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed),
        }
    }

    pub fn solve_cancellable(&mut self, seed: u64, cancel: &CancelToken) -> SolveOutcome {
        match self.constraint_set() {
            Some(cs) => search::solve_with_constraints(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, Some(cancel), &cs),
            None => search::solve_cancellable(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, cancel),
        }
    }

    pub fn solve_all(&mut self, seed: u64, limit: usize) -> (Vec<Solution>, bool) {
        match self.constraint_set() {
            Some(cs) => search::solve_all_with_constraints(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, limit, &cs),
            None => search::solve_all(&self.model, &self.topology, &self.config, seed, Some(&self.init_domains), &self.fixed, limit),
        }
    }

    pub fn model(&self) -> &CompiledModel {
        &self.model
    }

    pub fn topology(&self) -> &Grid2dTopology {
        &self.topology
    }

    /// 🧩️ Solves this grid as one chunk of a larger streamed/tiled world: `seam_fixed` pins every
    /// cell whose value a neighboring chunk already committed, and the seed is deterministically
    /// derived from `world_seed` plus `(chunk_x, chunk_y)` — see [`crate::chunk`]'s docs for the
    /// exact contract. Re-solving the same chunk coordinate later reproduces identical content.
    pub fn solve_chunk(&self, world_seed: u64, chunk_x: i64, chunk_y: i64, seam_fixed: &[(crate::ids::NodeId, PatternId)]) -> SolveOutcome {
        chunk::solve_chunk(&self.model, &self.topology, &self.config, world_seed, chunk_x, chunk_y, Some(&self.init_domains), seam_fixed)
    }

    /// 🧱️ The pattern assigned at `(x, y)` in `solution`.
    pub fn get(&self, solution: &Solution, x: usize, y: usize) -> Option<PatternId> {
        let n = self.topology.node_at(x, y)?;
        solution.assignment.get(n.index()).copied()
    }

    /// 🧱️ Row-major `width * height` tile decode via each pattern's authored tile provenance.
    /// Patterns with no tile provenance (e.g. built directly via [`crate::model::ModelBuilder`])
    /// decode to `None` at that cell.
    pub fn decode_tiles(&self, solution: &Solution) -> Vec<Option<crate::ids::TileId>> {
        solution.assignment.iter().map(|&p| self.model.pattern_info(p).tile).collect()
    }
}
// #endregion 🔖️Solver

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid2d::{Boundary, Stencil2d, declare_stencil_relations_tiled};
    use crate::tiled::TiledModelBuilder;

    fn checkerboard(width: usize, height: usize, boundary: Boundary) -> (CompiledModel, Grid2dTopology) {
        let mut b = TiledModelBuilder::new();
        let black = b.tile(1.0);
        let white = b.tile(1.0);
        let rels = declare_stencil_relations_tiled(&mut b, &Stencil2d::VonNeumann).unwrap();
        for &r in &rels {
            b.allow_mirrored(r, black, white);
        }
        let model = b.compile().unwrap();
        let topo = Grid2dTopology::new(width, height, &Stencil2d::VonNeumann, rels, boundary, boundary, None).unwrap();
        (model, topo)
    }

    #[test]
    fn solves_a_checkerboard_grid() {
        let (model, topo) = checkerboard(5, 5, Boundary::Open);
        let mut solver = Grid2dSolverBuilder::new(model, topo).build().unwrap();
        let outcome = solver.solve(1);
        assert!(matches!(outcome, SolveOutcome::Solved(_)));
    }

    #[test]
    fn fix_pins_a_cell_and_propagates() {
        let (model, topo) = checkerboard(4, 4, Boundary::Open);
        let black = PatternId(0);
        let white = PatternId(1);
        let mut solver = Grid2dSolverBuilder::new(model, topo).fix(0, 0, black).unwrap().build().unwrap();
        match solver.solve(1) {
            SolveOutcome::Solved(sol) => {
                assert_eq!(solver.get(&sol, 0, 0), Some(black));
                assert_eq!(solver.get(&sol, 1, 0), Some(white));
                assert_eq!(solver.get(&sol, 0, 1), Some(white));
            }
            other => panic!("expected Solved, got {other:?}"),
        }
    }

    #[test]
    fn solve_chunk_respects_seam_pins_and_reproduces_deterministically() {
        let (model, topo) = checkerboard(5, 5, Boundary::Open);
        let seam_node = topo.node_at(0, 0).unwrap();
        let solver = Grid2dSolverBuilder::new(model, topo).build().unwrap();

        let white = PatternId(1);
        let a = solver.solve_chunk(42, 3, -2, &[(seam_node, white)]);
        let b = solver.solve_chunk(42, 3, -2, &[(seam_node, white)]);
        match (a, b) {
            (SolveOutcome::Solved(sa), SolveOutcome::Solved(sb)) => {
                assert_eq!(sa.assignment, sb.assignment);
                assert_eq!(solver.get(&sa, 0, 0), Some(white));
            }
            other => panic!("expected both calls to solve identically, got {other:?}"),
        }
    }

    #[test]
    fn masked_cells_are_excluded_and_solve_completes() {
        let mut b = TiledModelBuilder::new();
        let black = b.tile(1.0);
        let white = b.tile(1.0);
        let rels = declare_stencil_relations_tiled(&mut b, &Stencil2d::VonNeumann).unwrap();
        for &r in &rels {
            b.allow_mirrored(r, black, white);
        }
        let model = b.compile().unwrap();
        let mut mask = vec![true; 9];
        mask[4] = false;
        let topo = Grid2dTopology::new(3, 3, &Stencil2d::VonNeumann, rels, Boundary::Open, Boundary::Open, Some(mask)).unwrap();
        let mut solver = Grid2dSolverBuilder::new(model, topo).build().unwrap();
        let outcome = solver.solve(1);
        assert!(matches!(outcome, SolveOutcome::Solved(_)));
    }

    #[test]
    fn wrap_boundary_solves_consistently() {
        let (model, topo) = checkerboard(4, 4, Boundary::Wrap);
        let mut solver = Grid2dSolverBuilder::new(model, topo).build().unwrap();
        let outcome = solver.solve(1);
        assert!(matches!(outcome, SolveOutcome::Solved(_)));
    }

    #[test]
    fn odd_size_wrap_is_unsatisfiable_for_two_color_checkerboard() {
        // A 3x3 wrapped grid forces an odd cycle along each axis; two colors can't 2-color it.
        let (model, topo) = checkerboard(3, 3, Boundary::Wrap);
        let mut solver = Grid2dSolverBuilder::new(model, topo).config(SearchConfig { mode: search::SearchMode::Backtrack, ..Default::default() }).build().unwrap();
        let outcome = solver.solve(1);
        assert!(matches!(outcome, SolveOutcome::Unsatisfiable(_)));
    }

    #[test]
    fn graph_vs_grid2d_strict_equivalence_von_neumann_open() {
        // Independently hand-enumerated arcs for a 3x4 VonNeumann/Open grid (not derived from
        // 🦀️grid2d.rs's own resolve_coord logic) fed into a GraphTopology, compared against the
        // same model solved through Grid2dTopology: both must produce byte-identical assignments
        // and identical observation counts under the same seed/config.
        let width = 3usize;
        let height = 4usize;
        let mut b = TiledModelBuilder::new();
        let tiles: Vec<_> = (0..3).map(|i| b.tile(1.0 + i as f64)).collect();
        let rels = declare_stencil_relations_tiled(&mut b, &Stencil2d::VonNeumann).unwrap();
        for &r in &rels {
            for &a in &tiles {
                for &c in &tiles {
                    if a != c {
                        b.allow(r, a, c);
                    }
                }
            }
        }
        let model = b.compile().unwrap();

        let mut hand_arcs = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let idx = |x: usize, y: usize| crate::ids::NodeId::from_index(y * width + x);
                if x + 1 < width {
                    hand_arcs.push((idx(x, y), idx(x + 1, y), rels[0])); // east: offset (1,0)
                    hand_arcs.push((idx(x + 1, y), idx(x, y), rels[1])); // west: offset (-1,0)
                }
                if y + 1 < height {
                    hand_arcs.push((idx(x, y), idx(x, y + 1), rels[2])); // south: offset (0,1)
                    hand_arcs.push((idx(x, y + 1), idx(x, y), rels[3])); // north: offset (0,-1)
                }
            }
        }
        let mut gb = crate::topology::GraphTopologyBuilder::new(width * height);
        for (from, to, r) in hand_arcs {
            gb.arc(from, to, r);
        }
        let graph_topo = gb.build().unwrap();

        let grid_topo = Grid2dTopology::new(width, height, &Stencil2d::VonNeumann, rels, Boundary::Open, Boundary::Open, None).unwrap();

        let config = SearchConfig::default();
        for seed in 0..10u64 {
            let mut graph_solver = crate::solver_graph::GraphSolverBuilder::new(model.clone(), graph_topo.clone()).config(config).build().unwrap();
            let mut grid_solver = Grid2dSolverBuilder::new(model.clone(), grid_topo.clone()).config(config).build().unwrap();
            let graph_outcome = graph_solver.solve(seed);
            let grid_outcome = grid_solver.solve(seed);
            match (graph_outcome, grid_outcome) {
                (SolveOutcome::Solved(g), SolveOutcome::Solved(r)) => {
                    assert_eq!(g.assignment, r.assignment, "seed {seed}: graph and grid2d solutions diverged");
                    assert_eq!(g.report.metrics.observations, r.report.metrics.observations, "seed {seed}: observation counts diverged");
                }
                (a, b) => panic!("seed {seed}: outcome mismatch, graph={a:?} grid={b:?}"),
            }
        }
    }

    #[test]
    fn decode_tiles_round_trips_tile_provenance() {
        let (model, topo) = checkerboard(2, 2, Boundary::Open);
        let mut solver = Grid2dSolverBuilder::new(model, topo).build().unwrap();
        match solver.solve(1) {
            SolveOutcome::Solved(sol) => {
                let tiles = solver.decode_tiles(&sol);
                assert_eq!(tiles.len(), 4);
                assert!(tiles.iter().all(|t| t.is_some()));
            }
            other => panic!("expected Solved, got {other:?}"),
        }
    }
}
// #endregion 🔖️Tests
