//! 🧊️ shapes), linear buckling and nodal-averaged stress recovery for contour rendering — sparse-backed (RCM-ordered,
//! 📈️ Multi-case/combination linear-static analysis, self-weight load generation, modal analysis (frequencies and
//! one LDLT factorization shared across every load case and eigen-solve) — plus the FEM job graph that stages them.

use semio_s_fem_algebra::MatD;
use semio_s_fem_algebra::VecD;
use semio_s_fem_model::Dof;
use semio_s_fem_model::Element;
use semio_s_fem_model::ElementContext;
use semio_s_fem_model::ElementResult;
use semio_s_fem_static::Elements;
use semio_s_fem_model::FemError;
use semio_s_fem_model::MemberUdl;
use semio_s_fem_model::NodalLoad;
use semio_s_fem_model::Node;
use semio_s_fem_model::NodeDisplacement;
use semio_s_fem_model::NodeReaction;
use semio_s_fem_model::SolutionChecks;
use semio_s_fem_model::StaticResult;
use semio_s_fem_model::Support;
use semio_s_fem_sparse::numerical_close_gate;
use semio_s_fem_sparse::numerical_ladder_turn;
use semio_s_fem_sparse::numerical_rung_demand;
use semio_s_fem_sparse::NumericalBatchEnd;
use semio_s_fem_sparse::NumericalOutcomeDesk;
use semio_s_fem_sparse::NumericalRun;
use semio_s_fem_sparse_ldlt::ldlt_factor;
use semio_s_fem_sparse::rcm_order;
use semio_s_fem_sparse_subspace::subspace_iteration;
use semio_s_fem_sparse::Coo;
use semio_s_fem_sparse::Csr;
use semio_s_fem_sparse_subspace::EigenPairs;
use semio_s_fem_sparse_ldlt::LdltFactor;
use semio_framework_value_paged_list::list::PagedList;
use semio_framework_job::InteractiveJob;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_job::JobOutcomeDescriptor;
use semio_framework_job::JobOutcomeBorrow;
use semio_framework_job_payload::JobPayloadStream;
use semio_framework_job::Operation;
use semio_framework_job::RetainedCloneGrant;
use semio_framework_job::RetainedCloneProgress;
use semio_framework_job::StepContext;
use semio_framework_value_edit::sort::HeapSortCursor;
use semio_framework_value_edit::sort::HeapSortWork;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use std::collections::HashSet;
use std::sync::Arc;
use semio_framework_job::OriginalTurnFailure;
use std::collections::HashMap;
use semio_s_fem_analyses_model::AnalysisModel;
use semio_s_fem_analyses_model::BucklingResult;
use semio_s_fem_analyses_model::Combination;
use semio_s_fem_analyses_model::LoadCase;
use semio_s_fem_analyses_model::ModalResult;
use semio_s_fem_analyses_model::decode_value;
use semio_s_fem_analyses_model::element_global_indices;
use semio_s_fem_analyses_model::encode_value;
use semio_s_fem_analyses_model::node_index_of;
use semio_s_fem_analyses_model::positions_of;
use semio_s_fem_analyses_model::validate_case;
use semio_s_fem_analyses_assembly::AssembledSystem;
#[cfg(test)]
use semio_s_fem_analyses_assembly::{AnalysisModelOwner, AssemblyConstructionStage, PendingElementBuildStage, UnfactoredSystem};
use semio_s_fem_analyses_assembly::assemble_system;
use semio_s_fem_analyses_assembly::case_rhs_old;
use semio_s_fem_model::BeamStation;
use semio_s_fem_model::PlaneStress;
use semio_s_fem_model::PlateMoments;
use semio_s_fem_model::ShellState;
use semio_s_fem_model::SolidStress;
use semio_s_fem_sparse::numerical_close_original;
// #region 🧩️JobGraph
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[value(tag = "kind")]
pub enum FemJobStage {
    ValidateReferences,
    BuildDofMap,
    OrderEquations,
    Assemble,
    Factor,
    Solve,
    Recover,
    Finalize,
}

impl FemJobStage {
    fn label(self) -> &'static str {
        match self {
            Self::ValidateReferences => "fem.validate-references",
            Self::BuildDofMap => "fem.build-dof-map",
            Self::OrderEquations => "fem.order-equations",
            Self::Assemble => "fem.assemble",
            Self::Factor => "fem.factor",
            Self::Solve => "fem.solve",
            Self::Recover => "fem.recover",
            Self::Finalize => "fem.finalize",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_value::RetireOwned)]
pub struct FemStagePlan {
    pub stage: FemJobStage,
    pub units: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_value::RetireOwned)]
pub struct FemJobProgress {
    pub stage: Option<FemJobStage>,
    pub completed_stages: usize,
    pub total_stages: usize,
    pub completed_units: u64,
    pub total_units: u64,
}

#[derive(Clone, ToValue, FromValue, semio_framework_value::RetireOwned)]
struct FemGraphCheckpoint {
    plans: Vec<FemStagePlan>,
    stage_cursor: usize,
    unit_cursor: u64,
    completed_units: u64,
    units_per_step: u64,
    checkpoint_due: bool,
}

pub struct FemJobGraph {
    operation: Operation,
    state: FemGraphCheckpoint,
    desk: NumericalOutcomeDesk,
}

impl FemJobGraph {
    pub fn new(operation: Operation, plans: Vec<FemStagePlan>, units_per_step: u64) -> Self {
        assert!(units_per_step > 0, "fem graph batch must contain work");
        Self { operation, state: FemGraphCheckpoint { plans, stage_cursor: 0, unit_cursor: 0, completed_units: 0, units_per_step, checkpoint_due: false }, desk: NumericalOutcomeDesk::default() }
    }

    pub fn from_checkpoint(operation: Operation, bytes: &[u8]) -> Result<Self, String> {
        Ok(Self { operation, state: decode_value(bytes)?, desk: NumericalOutcomeDesk::default() })
    }

    pub fn checkpoint_bytes(&self) -> Vec<u8> {
        encode_value(&self.state)
    }

    pub fn progress(&self) -> FemJobProgress {
        FemJobProgress {
            stage: self.state.plans.get(self.state.stage_cursor).map(|plan| plan.stage),
            completed_stages: self.state.stage_cursor,
            total_stages: self.state.plans.len(),
            completed_units: self.state.completed_units,
            total_units: self.state.plans.iter().map(|plan| plan.units).sum(),
        }
    }

    /// 🧹️ Retires at most one retained plan owner. `true` is an exact empty witness.
    pub fn close_retained_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        let bytes = size_of::<FemStagePlan>();
        if maximum_bytes < bytes {
            return (false, 0, 0);
        }
        if self.state.plans.pop().is_some() {
            return (false, 1, 0);
        }
        let backing_bytes = self.state.plans.capacity() * bytes;
        if backing_bytes != 0 {
            if backing_bytes > maximum_bytes {
                return (false, 0, 0);
            }
            self.state.plans = Vec::new();
            return (false, 1, backing_bytes);
        }
        (true, 0, 0)
    }
}

impl FemJobGraph {
    fn run(&mut self, context: &mut StepContext<'_>) -> NumericalRun {
        if context.is_cancelled() {
            return NumericalRun::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return NumericalRun::Fault;
        }
        if self.state.checkpoint_due {
            self.state.checkpoint_due = false;
            let bytes = self.checkpoint_bytes();
            return match context.payload_from_bytes(JobPayloadStream::CheckpointState, &bytes) {
                Ok(state) => self.desk.lend_checkpoint(state, self.state.completed_units),
                Err(_) => NumericalRun::Fault,
            };
        }
        if self.state.stage_cursor == self.state.plans.len() {
            let progress = self.progress();
            let bytes = encode_value(&progress);
            return match context.payload_from_bytes(JobPayloadStream::CommitOutput, &bytes) {
                Ok(output) => self.desk.lend_complete(output),
                Err(_) => NumericalRun::Fault,
            };
        }
        let stage = self.state.plans[self.state.stage_cursor].stage;
        context.set_stage(stage.label());
        let mut stepped = 0;
        while stepped < self.state.units_per_step && !context.should_yield() && self.state.stage_cursor < self.state.plans.len() {
            let remaining = self.state.plans[self.state.stage_cursor].units.saturating_sub(self.state.unit_cursor);
            if remaining == 0 {
                self.state.stage_cursor += 1;
                self.state.unit_cursor = 0;
                self.state.checkpoint_due = true;
                break;
            }
            let take = remaining.min(self.state.units_per_step - stepped).min(context.fuel_remaining());
            if take == 0 {
                break;
            }
            self.state.unit_cursor += take;
            self.state.completed_units += take;
            stepped += take;
            context.consume_fuel(take);
            if context.is_cancelled() {
                return NumericalRun::Cancelled;
            }
        }
        if self.state.stage_cursor == self.state.plans.len() {
            let progress = self.progress();
            let bytes = encode_value(&progress);
            return match context.payload_from_bytes(JobPayloadStream::CommitOutput, &bytes) {
                Ok(output) => self.desk.lend_complete(output),
                Err(_) => NumericalRun::Fault,
            };
        }
        let bytes = encode_value(&self.progress());
        match context.payload_from_bytes(JobPayloadStream::Preview, &bytes) {
            Ok(preview) => self.desk.lend_preview(preview),
            Err(_) => NumericalRun::Fault,
        }
    }

    fn close_frontier(&self) -> Result<Option<RetirementDemand>, ValueError> {
        self.desk.retirement_demands().transpose()
    }

    fn close_demand(&self) -> Result<RetirementDemand, ValueError> {
        if let Some(demand) = self.close_frontier()? {
            return Ok(demand);
        }
        let backing_bytes = self.state.plans.capacity() * size_of::<FemStagePlan>();
        Ok(if backing_bytes == 0 && self.state.plans.is_empty() { RetirementDemand::default() } else { RetirementDemand { release_bytes: backing_bytes.max(size_of::<FemStagePlan>()), depth: 1, ..Default::default() } })
    }
}

impl InteractiveJob for FemJobGraph {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        if self.desk.retire_due(cx, self.operation) {
            self.desk.retire_delivered(cx)?;
            return Ok(None);
        }
        let run = match self.desk.take_pending() {
            Some(run) => run,
            None => self.run(cx),
        };
        Ok(self.desk.admit(cx, run)?)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
        self.desk.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {}

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if self.terminal_is_empty() {
            return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
        }
        let demand = self.close_demand()?;
        numerical_close_original(cx, self, demand, Self::close_granted, Self::terminal_is_empty)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand()?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> { Ok(self.close_demand()?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand()?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand()?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.desk.terminal_is_empty() && self.state.plans.is_empty() && self.state.plans.capacity() == 0
    }
}

impl FemJobGraph {
    /// 🎟️ One close turn under an explicit grant: the owners of a child job and the original-context `close_step`
    /// both admit it through the job's own quote.
    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        let frontier = self.close_frontier()?;
        if self.terminal_is_empty() {
            return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
        }
        if !numerical_close_gate(grant, frontier) {
            return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
        }
        let step = match self.desk.close_step(grant) {
            Some(step) => step?,
            None => numerical_ladder_turn(self.close_retained_step(grant.maximum_release_bytes)),
        };
        step.admit(grant, self.terminal_is_empty())
    }
}
// #endregion 🧩️JobGraph

// #region 🔖️Combine
/// 🌱️ A zero-valued `ElementResult` of the same variant/shape as `result` — the seed for superposition.
fn zero_like(result: &ElementResult) -> ElementResult {
    match result {
        ElementResult::Bar { .. } => ElementResult::Bar { n: 0.0 },
        ElementResult::Beam { stations } => ElementResult::Beam { stations: stations.iter().map(|s| BeamStation { x: s.x, n: 0.0, v: 0.0, m: 0.0 }).collect() },
        ElementResult::Plane { gauss } => ElementResult::Plane { gauss: gauss.iter().map(|_| PlaneStress { sxx: 0.0, syy: 0.0, sxy: 0.0, von_mises: 0.0 }).collect() },
        ElementResult::Plate { gauss } => ElementResult::Plate { gauss: gauss.iter().map(|_| PlateMoments { mx: 0.0, my: 0.0, mxy: 0.0 }).collect() },
        ElementResult::Solid { gauss } => ElementResult::Solid { gauss: gauss.iter().map(|_| SolidStress { sxx: 0.0, syy: 0.0, szz: 0.0, sxy: 0.0, syz: 0.0, sxz: 0.0, von_mises: 0.0 }).collect() },
        ElementResult::Shell { gauss } => ElementResult::Shell { gauss: gauss.iter().map(|_| ShellState { nxx: 0.0, nyy: 0.0, nxy: 0.0, mxx: 0.0, myy: 0.0, mxy: 0.0, von_mises_top: 0.0, von_mises_bottom: 0.0 }).collect() },
    }
}

/// ➕️ `acc + factor * term`, field-by-field, matched by `ElementResult` variant and Gauss-point index.
fn add_scaled_element_result(acc: &ElementResult, term: &ElementResult, factor: f64) -> ElementResult {
    match (acc, term) {
        (ElementResult::Bar { n: an }, ElementResult::Bar { n: tn }) => ElementResult::Bar { n: an + factor * tn },
        (ElementResult::Beam { stations: acc_s }, ElementResult::Beam { stations: term_s }) => {
            ElementResult::Beam { stations: acc_s.iter().zip(term_s.iter()).map(|(a, t)| BeamStation { x: a.x, n: a.n + factor * t.n, v: a.v + factor * t.v, m: a.m + factor * t.m }).collect() }
        }
        (ElementResult::Plane { gauss: acc_g }, ElementResult::Plane { gauss: term_g }) => {
            ElementResult::Plane { gauss: acc_g.iter().zip(term_g.iter()).map(|(a, t)| PlaneStress { sxx: a.sxx + factor * t.sxx, syy: a.syy + factor * t.syy, sxy: a.sxy + factor * t.sxy, von_mises: a.von_mises + factor * t.von_mises }).collect() }
        }
        (ElementResult::Plate { gauss: acc_g }, ElementResult::Plate { gauss: term_g }) => {
            ElementResult::Plate { gauss: acc_g.iter().zip(term_g.iter()).map(|(a, t)| PlateMoments { mx: a.mx + factor * t.mx, my: a.my + factor * t.my, mxy: a.mxy + factor * t.mxy }).collect() }
        }
        (ElementResult::Solid { gauss: acc_g }, ElementResult::Solid { gauss: term_g }) => ElementResult::Solid {
            gauss: acc_g
                .iter()
                .zip(term_g.iter())
                .map(|(a, t)| SolidStress {
                    sxx: a.sxx + factor * t.sxx,
                    syy: a.syy + factor * t.syy,
                    szz: a.szz + factor * t.szz,
                    sxy: a.sxy + factor * t.sxy,
                    syz: a.syz + factor * t.syz,
                    sxz: a.sxz + factor * t.sxz,
                    von_mises: a.von_mises + factor * t.von_mises,
                })
                .collect(),
        },
        (ElementResult::Shell { gauss: acc_g }, ElementResult::Shell { gauss: term_g }) => ElementResult::Shell {
            gauss: acc_g
                .iter()
                .zip(term_g.iter())
                .map(|(a, t)| ShellState {
                    nxx: a.nxx + factor * t.nxx,
                    nyy: a.nyy + factor * t.nyy,
                    nxy: a.nxy + factor * t.nxy,
                    mxx: a.mxx + factor * t.mxx,
                    myy: a.myy + factor * t.myy,
                    mxy: a.mxy + factor * t.mxy,
                    von_mises_top: a.von_mises_top + factor * t.von_mises_top,
                    von_mises_bottom: a.von_mises_bottom + factor * t.von_mises_bottom,
                })
                .collect(),
        },
        _ => acc.clone(),
    }
}

fn combine_results(model: &AnalysisModel, case_results: &[StaticResult], cases: &[LoadCase], combo: &Combination) -> Result<StaticResult, FemError> {
    let mut displacements: Vec<NodeDisplacement> = model.nodes.iter().map(|node| NodeDisplacement { node_id: node.id.clone(), values: [0.0; 6] }).collect();
    let mut reactions: Vec<NodeReaction> = Vec::new();
    let mut member_loads: HashMap<&str, MemberUdl> = HashMap::new();
    let mut reaction_sum = [0.0; 6];
    let mut residual_norm = 0.0;

    for (case_id, factor) in &combo.terms {
        let idx = cases.iter().position(|c| &c.id == case_id).ok_or_else(|| FemError::DanglingNodeRef(case_id.clone()))?;
        let cr = &case_results[idx];
        for (i, d) in cr.displacements.iter().enumerate() {
            for k in 0..6 {
                displacements[i].values[k] += factor * d.values[k];
            }
        }
        for r in &cr.reactions {
            if let Some(existing) = reactions.iter_mut().find(|e: &&mut NodeReaction| e.node_id == r.node_id && e.dof == r.dof) {
                existing.value += factor * r.value;
            } else {
                reactions.push(NodeReaction { node_id: r.node_id.clone(), dof: r.dof, value: factor * r.value });
            }
        }
        let mut seen = HashSet::new();
        for (id, load) in &cases[idx].member_loads {
            if !seen.insert(id.as_str()) { continue; }
            let combined = member_loads.entry(id.as_str()).or_default();
            combined.wx += factor * load.wx;
            combined.wy += factor * load.wy;
            combined.wz += factor * load.wz;
        }
        for (sum, reaction) in reaction_sum.iter_mut().zip(cr.checks.reaction_sum) {
            *sum += factor * reaction;
        }
        residual_norm += factor.abs() * cr.checks.residual_norm;
    }

    let elements = recover_elements(model, &displacements, |id| member_loads.get(id).copied());
    Ok(StaticResult { displacements, reactions, elements, checks: SolutionChecks { residual_norm, reaction_sum } })
}
// #endregion 🔖️Combine

// #region 🔖️SolveMultiCase
/// 🧮️ Assembles the model ONCE (sparse, RCM-ordered, free-free LDLT factored once), then solves every
/// load case as one shared multi-RHS `solve_many` call, superposes `combinations` from the already-
/// solved case results, and un-permutes everything back to original node identity.
pub fn solve_multi_case(model: &AnalysisModel, cases: &[LoadCase], combinations: &[Combination], gravity: [f64; 3]) -> Result<HashMap<ResultSource, StaticResult>, FemError> {
    for case in cases {
        validate_case(model, case)?;
    }
    let system = assemble_system(model)?;
    let dof_map = &system.dof_map;
    let ndof = system.ndof;
    let n_free = system.n_free();

    let rhs_full_old: Vec<VecD> = cases.iter().map(|case| case_rhs_old(model, dof_map, case, gravity)).collect();

    let mut rhs_compact = MatD::zeros(n_free, cases.len().max(1));
    for (c, f_old) in rhs_full_old.iter().enumerate() {
        for old_idx in 0..ndof {
            let new_idx = system.inv_perm[old_idx];
            if let Some(compact) = system.compact_of_new[new_idx] {
                rhs_compact.set(compact, c, f_old.get(old_idx));
            }
        }
    }
    let u_compact = system.k_factor.solve_many(&rhs_compact);

    let mut results: HashMap<ResultSource, StaticResult> = HashMap::new();
    let mut case_results: Vec<StaticResult> = Vec::with_capacity(cases.len());
    let displacement_slot: HashMap<&str, usize> = model.nodes.iter().enumerate().map(|(slot, node)| (node.id.as_str(), slot)).collect();

    for (c, case) in cases.iter().enumerate() {
        let mut u_new = VecD::zeros(ndof);
        for (k, &new_idx) in system.free_new.iter().enumerate() {
            u_new.set(new_idx, u_compact.get(k, c));
        }
        let f_old = &rhs_full_old[c];
        let mut f_new = VecD::zeros(ndof);
        for old_idx in 0..ndof {
            f_new.set(system.inv_perm[old_idx], f_old.get(old_idx));
        }
        let ku_new = system.k_full.mul_vec(&u_new);

        let mut reactions = Vec::new();
        for old_idx in 0..ndof {
            let new_idx = system.inv_perm[old_idx];
            if system.compact_of_new[new_idx].is_none() {
                let r = ku_new.get(new_idx) - f_new.get(new_idx);
                let (node_id, dof) = dof_map.order[old_idx].clone();
                reactions.push(NodeReaction { node_id, dof, value: r });
            }
        }

        let mut displacements: Vec<NodeDisplacement> = model.nodes.iter().map(|n| NodeDisplacement { node_id: n.id.clone(), values: [0.0; 6] }).collect();
        for (old_idx, (node_id, dof)) in dof_map.order.iter().enumerate() {
            let new_idx = system.inv_perm[old_idx];
            if let Some(&slot) = displacement_slot.get(node_id.as_str()) {
                displacements[slot].values[dof.index()] = u_new.get(new_idx);
            }
        }

        let elements_out = recover_elements(model, &displacements, |id| case.member_loads.iter().find(|(member, _)| member == id).map(|(_, load)| *load));

        let mut reaction_sum = [0.0; 6];
        for r in &reactions {
            reaction_sum[r.dof.index()] += r.value;
        }
        for old_idx in 0..ndof {
            let (_, dof) = &dof_map.order[old_idx];
            reaction_sum[dof.index()] += f_old.get(old_idx);
        }
        let free_ku = VecD::from_vec(system.free_new.iter().map(|&new_idx| ku_new.get(new_idx)).collect());
        let free_f = VecD::from_vec(system.free_new.iter().map(|&new_idx| f_new.get(new_idx)).collect());
        let residual_norm = free_ku.sub(&free_f).norm2() / free_f.norm2().max(1e-9);

        let result = StaticResult { displacements, reactions, elements: elements_out, checks: SolutionChecks { residual_norm, reaction_sum } };
        case_results.push(result.clone());
        results.insert(ResultSource::LoadCase(case.id.clone()), result);
    }

    for combo in combinations {
        let combined = combine_results(model, &case_results, cases, combo)?;
        results.insert(ResultSource::Combination(combo.id.clone()), combined);
    }

    Ok(results)
}
// #endregion 🔖️SolveMultiCase

// #region 🔖️Modal
/// 🎯️ Modal analysis: shares `solve_multi_case`'s sparse RCM-ordered free-free LDLT factor, assembles
/// the global mass matrix over the SAME free DOFs (elements with `mass() == None` contribute nothing),
/// and calls `subspace_iteration` for the lowest `count` frequencies/shapes.
pub fn modal(model: &AnalysisModel, count: usize) -> Result<ModalResult, FemError> {
    let system = assemble_system(model)?;
    let ndof = system.ndof;
    let n_free = system.n_free();

    let mut m_coo = Coo::new(n_free);
    let node_slot = node_index_of(&model.nodes);
    for element in &model.elements {
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let Some(indices_old) = element_global_indices(&system.dof_map, &node_ids, dofs) else { continue };
        let ctx = ElementContext { positions: positions_of(&node_slot, &model.nodes, &node_ids) };
        let Some(me) = element.mass(&ctx) else { continue };
        let indices_new: Vec<usize> = indices_old.iter().map(|&old| system.inv_perm[old]).collect();
        for (local_row, &new_row) in indices_new.iter().enumerate() {
            let Some(compact_row) = system.compact_of_new[new_row] else { continue };
            for (local_col, &new_col) in indices_new.iter().enumerate() {
                let Some(compact_col) = system.compact_of_new[new_col] else { continue };
                let v = me.get(local_row, local_col);
                if v != 0.0 {
                    m_coo.add(compact_row, compact_col, v);
                }
            }
        }
    }
    let m_csr = m_coo.to_csr();

    let pairs: EigenPairs = subspace_iteration(&system.k_factor, &m_csr, n_free, count, 30);
    let frequencies_hz: Vec<f64> = pairs.values.iter().map(|&lambda| lambda.max(0.0).sqrt() / (2.0 * std::f64::consts::PI)).collect();
    let shapes = unpermute_shapes(&system, ndof, &pairs.vectors);

    Ok(ModalResult { frequencies_hz, shapes })
}

/// 🔁️ Expands each compact free-DOF eigenvector back to full `ndof` (zero at constrained slots), then
/// un-permutes RCM (new) index space back to the ORIGINAL `dof_map` order (node-major, matching
/// `model.nodes`, DOF sub-order filtered to active DOFs).
fn unpermute_shapes(system: &AssembledSystem, ndof: usize, vectors: &[VecD]) -> Vec<VecD> {
    vectors
        .iter()
        .map(|vec_compact| {
            let mut u_new = VecD::zeros(ndof);
            for (k, &new_idx) in system.free_new.iter().enumerate() {
                u_new.set(new_idx, vec_compact.get(k));
            }
            let mut shape = VecD::zeros(ndof);
            for old_idx in 0..ndof {
                shape.set(old_idx, u_new.get(system.inv_perm[old_idx]));
            }
            shape
        })
        .collect()
}
// #endregion 🔖️Modal

// #region 🔖️Buckling
/// 🌀️ Linear buckling: solves `reference_case` (via `solve_multi_case`) for `u_ref`, assembles the
/// geometric stiffness `Kg` from every element's own axial state under `u_ref`, then solves
/// `K φ = λ (−Kg) φ` via `subspace_iteration` — `factors[i] * reference_case` is the i-th critical load.
pub fn buckling(model: &AnalysisModel, reference_case: &LoadCase, count: usize, gravity: [f64; 3]) -> Result<BucklingResult, FemError> {
    let ref_results = solve_multi_case(model, std::slice::from_ref(reference_case), &[], gravity)?;
    let ref_result = ref_results.get(&ResultSource::LoadCase(reference_case.id.clone())).expect("reference case was just solved");

    let system = assemble_system(model)?;
    let ndof = system.ndof;
    let n_free = system.n_free();

    let mut neg_kg_coo = Coo::new(n_free);
    let node_slot = node_index_of(&model.nodes);
    for element in &model.elements {
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let Some(indices_old) = element_global_indices(&system.dof_map, &node_ids, dofs) else { continue };
        let ctx = ElementContext { positions: positions_of(&node_slot, &model.nodes, &node_ids) };

        let mut u_element = VecD::zeros(indices_old.len());
        for (i, &old_idx) in indices_old.iter().enumerate() {
            let (node_id, dof) = &system.dof_map.order[old_idx];
            let d = ref_result.displacements.iter().find(|d| &d.node_id == node_id).expect("node exists in reference result");
            u_element.set(i, d.values[dof.index()]);
        }

        let Some(kg) = element.geometric_stiffness(&ctx, &u_element) else { continue };
        let indices_new: Vec<usize> = indices_old.iter().map(|&old| system.inv_perm[old]).collect();
        for (local_row, &new_row) in indices_new.iter().enumerate() {
            let Some(compact_row) = system.compact_of_new[new_row] else { continue };
            for (local_col, &new_col) in indices_new.iter().enumerate() {
                let Some(compact_col) = system.compact_of_new[new_col] else { continue };
                let v = kg.get(local_row, local_col);
                if v != 0.0 {
                    neg_kg_coo.add(compact_row, compact_col, -v);
                }
            }
        }
    }

    let neg_kg_csr = neg_kg_coo.to_csr();

    let pairs: EigenPairs = subspace_iteration(&system.k_factor, &neg_kg_csr, n_free, count, 30);
    let shapes = unpermute_shapes(&system, ndof, &pairs.vectors);

    Ok(BucklingResult { factors: pairs.values, shapes })
}
// #endregion 🔖️Buckling

// #region 🔖️NodalAveraging
/// 🎨️ A scalar quantity `nodal_averaged_scalar` can recover from an `ElementResult`, for contour
/// rendering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StressScalar {
    VonMises,
    Sxx,
    Syy,
    Sxy,
    Szz,
    Syz,
    Sxz,
    VonMisesTop,
    VonMisesBottom,
}

/// 📊️ An element's own Gauss-point-averaged value of `scalar`, or `None` if that element kind/scalar
/// combination isn't defined (e.g. `VonMisesTop` on a `Plane` result, or any scalar on a `Bar`/`Beam`
/// result — those carry no stress tensor to project).
fn element_scalar_average(result: &ElementResult, scalar: StressScalar) -> Option<f64> {
    fn avg(values: impl Iterator<Item = f64>) -> f64 {
        let mut sum = 0.0;
        let mut count = 0usize;
        for v in values {
            sum += v;
            count += 1;
        }
        sum / (count.max(1) as f64)
    }
    match result {
        ElementResult::Plane { gauss } => match scalar {
            StressScalar::VonMises => Some(avg(gauss.iter().map(|g| g.von_mises))),
            StressScalar::Sxx => Some(avg(gauss.iter().map(|g| g.sxx))),
            StressScalar::Syy => Some(avg(gauss.iter().map(|g| g.syy))),
            StressScalar::Sxy => Some(avg(gauss.iter().map(|g| g.sxy))),
            _ => None,
        },
        ElementResult::Solid { gauss } => match scalar {
            StressScalar::VonMises => Some(avg(gauss.iter().map(|g| g.von_mises))),
            StressScalar::Sxx => Some(avg(gauss.iter().map(|g| g.sxx))),
            StressScalar::Syy => Some(avg(gauss.iter().map(|g| g.syy))),
            StressScalar::Szz => Some(avg(gauss.iter().map(|g| g.szz))),
            StressScalar::Sxy => Some(avg(gauss.iter().map(|g| g.sxy))),
            StressScalar::Syz => Some(avg(gauss.iter().map(|g| g.syz))),
            StressScalar::Sxz => Some(avg(gauss.iter().map(|g| g.sxz))),
            _ => None,
        },
        ElementResult::Shell { gauss } => match scalar {
            StressScalar::VonMisesTop => Some(avg(gauss.iter().map(|g| g.von_mises_top))),
            StressScalar::VonMisesBottom => Some(avg(gauss.iter().map(|g| g.von_mises_bottom))),
            _ => None,
        },
        _ => None,
    }
}

/// 🎨️ Nodal-averaged contour values: each element's OWN Gauss-point average of `scalar` (constant
/// across Gauss points for a 1-point-integrated `Tri3Cst`, a genuine average for higher-order
/// elements — deliberately NOT a polynomial extrapolation-to-nodes, a simple scope choice) is
/// accumulated, UNWEIGHTED (by element count, not by tributary area/volume), into every node it
/// touches; the returned value per node is that accumulation's mean. A node touched only by elements
/// that report no value for `scalar` (e.g. a `Bar` in a mixed mesh) simply never appears in the map.
/// Element-to-model matching is by `element.id()` against `result.elements`' ids.
pub fn nodal_averaged_scalar(model: &AnalysisModel, result: &StaticResult, scalar: StressScalar) -> HashMap<String, f64> {
    let mut sums: HashMap<String, (f64, usize)> = HashMap::new();
    for (element_id, element_result) in &result.elements {
        let Some(value) = element_scalar_average(element_result, scalar) else { continue };
        let Some(element) = model.elements.iter().find(|e| e.id() == element_id) else { continue };
        for node_id in element.node_ids() {
            let entry = sums.entry(node_id).or_insert((0.0, 0));
            entry.0 += value;
            entry.1 += 1;
        }
    }
    sums.into_iter().map(|(node_id, (sum, count))| (node_id, sum / count as f64)).collect()
}
// #endregion 🔖️NodalAveraging

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️solid/🦀️.rs"]
mod solid_tests;
// #endregion 🔖️Tests

pub use semio_s_fem_analyses_model::{ResultSource, ResultSourceIdentity, ResultSourceIdentityError, encode_result_source_identity, encode_result_source_parts, validate_result_source_parts, borrow_result_source_identity, validate_result_source_identity, decode_result_source_identity};
