//! 📈️ Multi-case/combination linear-static analysis, self-weight load generation, modal analysis
//! (frequencies/shapes), linear buckling, and nodal-averaged stress recovery for contour rendering
//! (`nodal_averaged_scalar`) — all sparse-backed (RCM-ordered, single LDLT factorization shared
//! across every load case / eigen-solve).

use crate::algebra::{MatD, VecD};
use crate::model::{BeamStation, Dof, Element, ElementContext, ElementResult, Elements, FemError, MemberUdl, NodalLoad, Node, NodeDisplacement, NodeReaction, PlaneStress, PlateMoments, ShellState, SolidStress, SolutionChecks, StaticResult, Support};
use crate::sparse::{ldlt_factor, rcm_order, subspace_iteration, Coo, Csr, EigenPairs, LdltFactor};
use semio_framework_job::{CommitCandidate, InteractiveJob, JobFault, Operation, StepContext, StepOutcome};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

const MOUNTED_OWNER_PAGE_BYTES: usize = 4_096;

fn reserve_exact_owner_page<T>(owner: &mut Vec<T>, additional: usize) -> bool {
    owner.try_reserve_exact(additional).is_ok() && owner.capacity().checked_mul(std::mem::size_of::<T>()).is_some_and(|bytes| bytes <= MOUNTED_OWNER_PAGE_BYTES)
}

fn close_vec_owner_step<T>(owner: &mut Vec<T>, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
    if owner.pop().is_some() {
        return Ok(Some((1, 0)));
    }
    let bytes = owner.capacity().checked_mul(std::mem::size_of::<T>()).ok_or(())?;
    if bytes == 0 {
        return Ok(None);
    }
    if bytes > maximum_bytes {
        return Err(());
    }
    *owner = Vec::new();
    Ok(Some((1, bytes)))
}

// #region 🔖️Model
/// 📦️ A named load case: nodal loads, member UDLs, and an optional self-weight contribution.
pub struct LoadCase {
    pub id: String,
    pub nodal_loads: Vec<NodalLoad>,
    pub member_loads: Vec<(String, MemberUdl)>,
    pub self_weight: bool,
}

/// 📦️ A linear combination of load cases — `Σ factor_i * case_i`, superposed from already-solved
/// case results (no re-solve).
pub struct Combination {
    pub id: String,
    pub terms: Vec<(String, f64)>,
}

/// 🏗️ Model geometry for multi-case/modal/buckling analysis — no loads (those come from `LoadCase`).
pub struct AnalysisModel {
    pub nodes: Vec<Node>,
    pub elements: Vec<Elements>,
    pub supports: Vec<Support>,
}

#[derive(Default)]
struct AnalysisModelCloseCursor {
    lane: u8,
}

fn close_analysis_model_step(owner: &mut Arc<AnalysisModel>, cursor: &mut AnalysisModelCloseCursor, maximum_bytes: usize) -> (bool, usize, usize) {
    let Some(model) = Arc::get_mut(owner) else {
        return (false, 0, 0);
    };
    loop {
        match cursor.lane {
            0 => {
                let Some(node) = model.nodes.last_mut() else {
                    cursor.lane += 1;
                    continue;
                };
                if node.id.capacity() != 0 {
                    let bytes = node.id.capacity();
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    node.id = String::new();
                    return (false, 1, bytes);
                }
                model.nodes.pop();
                return (false, 1, 0);
            }
            1 => {
                let bytes = model.nodes.capacity() * std::mem::size_of::<Node>();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                model.nodes = Vec::new();
                cursor.lane += 1;
                return (false, 1, bytes);
            }
            2 => {
                let Some(element) = model.elements.last_mut() else {
                    cursor.lane += 1;
                    continue;
                };
                if let Some(bytes) = element.mounted_next_string_bytes() {
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    return (false, 1, element.close_mounted_string_step().expect("mounted model close witness changed without mutation"));
                }
                if !element.mounted_strings_terminal_is_empty() {
                    return (false, 0, 0);
                }
                model.elements.pop();
                return (false, 1, 0);
            }
            3 => {
                let bytes = model.elements.capacity() * std::mem::size_of::<Elements>();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                model.elements = Vec::new();
                cursor.lane += 1;
                return (false, 1, bytes);
            }
            4 => {
                let Some(support) = model.supports.last_mut() else {
                    cursor.lane += 1;
                    continue;
                };
                if support.node_id.capacity() != 0 {
                    let bytes = support.node_id.capacity();
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    support.node_id = String::new();
                    return (false, 1, bytes);
                }
                if support.fixed.pop().is_some() {
                    return (false, 1, 0);
                }
                let bytes = support.fixed.capacity() * std::mem::size_of::<Dof>();
                if bytes != 0 {
                    if bytes > maximum_bytes {
                        return (false, 0, 0);
                    }
                    support.fixed = Vec::new();
                    return (false, 1, bytes);
                }
                model.supports.pop();
                return (false, 1, 0);
            }
            5 => {
                let bytes = model.supports.capacity() * std::mem::size_of::<Support>();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                model.supports = Vec::new();
                cursor.lane += 1;
                return (false, 1, bytes);
            }
            _ => return (true, 0, 0),
        }
    }
}

/// 📐️ The lowest modes of a `modal` analysis — `shapes[i]` is node-major matching `model.nodes`
/// order, DOF sub-order `Tx,Ty,Tz,Rx,Ry,Rz` filtered to each node's active DOFs (the same layout
/// `StaticResult`'s node list implies), zero at every constrained DOF.
pub struct ModalResult {
    pub frequencies_hz: Vec<f64>,
    pub shapes: Vec<VecD>,
}

/// 📐️ The lowest linear-buckling load factors of a `buckling` analysis — `factors[i] * reference_case`
/// is the critical load; `shapes[i]` uses the same layout as `ModalResult::shapes`.
pub struct BucklingResult {
    pub factors: Vec<f64>,
    pub shapes: Vec<VecD>,
}
// #endregion 🔖️Model

// #region 🧩️JobGraph
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FemStagePlan {
    pub stage: FemJobStage,
    pub units: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FemJobProgress {
    pub stage: Option<FemJobStage>,
    pub completed_stages: usize,
    pub total_stages: usize,
    pub completed_units: u64,
    pub total_units: u64,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
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
}

impl FemJobGraph {
    pub fn new(operation: Operation, plans: Vec<FemStagePlan>, units_per_step: u64) -> Self {
        assert!(units_per_step > 0, "fem graph batch must contain work");
        Self { operation, state: FemGraphCheckpoint { plans, stage_cursor: 0, unit_cursor: 0, completed_units: 0, units_per_step, checkpoint_due: false } }
    }

    pub fn from_checkpoint(operation: Operation, bytes: &[u8]) -> Result<Self, serde_json::Error> {
        Ok(Self { operation, state: serde_json::from_slice(bytes)? })
    }

    pub fn checkpoint_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&self.state).expect("fem graph checkpoint is serializable")
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
    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        let bytes = std::mem::size_of::<FemStagePlan>();
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

impl InteractiveJob for FemJobGraph {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return StepOutcome::Fault(JobFault { detail: b"stale-fem-job-graph-operation".to_vec() });
        }
        if self.state.checkpoint_due {
            self.state.checkpoint_due = false;
            return StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: self.checkpoint_bytes(), applied_progress: self.state.completed_units });
        }
        if self.state.stage_cursor == self.state.plans.len() {
            let progress = self.progress();
            return StepOutcome::Complete(CommitCandidate { state: self.checkpoint_bytes(), output: serde_json::to_vec(&progress).expect("fem graph output is serializable") });
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
                return StepOutcome::Cancelled;
            }
        }
        if self.state.stage_cursor == self.state.plans.len() {
            let progress = self.progress();
            return StepOutcome::Complete(CommitCandidate { state: self.checkpoint_bytes(), output: serde_json::to_vec(&progress).expect("fem graph output is serializable") });
        }
        StepOutcome::PreviewReady(serde_json::to_vec(&self.progress()).expect("fem graph preview is serializable"))
    }

    fn begin_close(&mut self) {}

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let (complete, released_items, released_bytes) = FemJobGraph::close_step(self, maximum_bytes);
        if complete {
            semio_framework_job::InteractiveJobCloseStep::Complete
        } else {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.state.plans.is_empty() && self.state.plans.capacity() == 0
    }
}
// #endregion 🧩️JobGraph

// #region 🔖️DofMap
/// 🔢️ Numbers each node's active DOFs (the union of `dofs_per_node()` over elements touching it) —
/// a small, self-contained reimplementation of `lib.rs`'s private `build_dof_map`/`DofMap` (not
/// `pub`, so not importable here), kept byte-for-byte equivalent in ordering behavior.
struct DofMap {
    order: Vec<(String, Dof)>,
}

impl DofMap {
    fn get(&self, node_id: &str, dof: Dof) -> Option<usize> {
        self.order.iter().position(|(current, current_dof)| current == node_id && *current_dof == dof)
    }

    fn len(&self) -> usize {
        self.order.len()
    }
}

fn build_dof_map(nodes: &[Node], elements: &[Elements]) -> DofMap {
    let mut order = Vec::new();
    for node in nodes {
        let mut active: Vec<Dof> = Vec::new();
        for element in elements {
            if element.node_ids().iter().any(|id| id == &node.id) {
                for &dof in element.dofs_per_node() {
                    if !active.contains(&dof) {
                        active.push(dof);
                    }
                }
            }
        }
        active.sort_by_key(|d| d.index());
        for dof in active {
            order.push((node.id.clone(), dof));
        }
    }
    DofMap { order }
}

fn positions_of(nodes: &[Node], node_ids: &[String]) -> Vec<[f64; 3]> {
    node_ids.iter().map(|id| nodes.iter().find(|n| &n.id == id).map(|n| n.pos).unwrap_or_default()).collect()
}

fn element_global_indices(dof_map: &DofMap, node_ids: &[String], dofs: &[Dof]) -> Option<Vec<usize>> {
    let mut indices = Vec::with_capacity(node_ids.len() * dofs.len());
    for node_id in node_ids {
        for &dof in dofs {
            indices.push(dof_map.get(node_id, dof)?);
        }
    }
    Some(indices)
}
// #endregion 🔖️DofMap

// #region 🔖️Validate
fn validate(model: &AnalysisModel) -> Result<(), FemError> {
    if model.nodes.is_empty() {
        return Err(FemError::EmptyModel);
    }
    let mut seen = HashSet::new();
    for node in &model.nodes {
        if !seen.insert(node.id.clone()) {
            return Err(FemError::DuplicateNodeId(node.id.clone()));
        }
    }
    let node_exists = |id: &str| model.nodes.iter().any(|n| n.id == id);
    for element in &model.elements {
        for id in element.node_ids() {
            if !node_exists(&id) {
                return Err(FemError::DanglingNodeRef(id));
            }
        }
    }
    for support in &model.supports {
        if !node_exists(&support.node_id) {
            return Err(FemError::DanglingNodeRef(support.node_id.clone()));
        }
    }
    Ok(())
}

fn validate_case(model: &AnalysisModel, case: &LoadCase) -> Result<(), FemError> {
    let node_exists = |id: &str| model.nodes.iter().any(|n| n.id == id);
    for load in &case.nodal_loads {
        if !node_exists(&load.node_id) {
            return Err(FemError::DanglingNodeRef(load.node_id.clone()));
        }
    }
    Ok(())
}
// #endregion 🔖️Validate

// #region 🔖️Rcm
/// 🌀️ Node-index RCM permutation, expanded to DOF granularity: each node's active DOFs stay
/// contiguous, positioned at its node's new RCM slot. `inv_perm[old_idx] = new_idx` (the only
/// direction callers need — un-permuting walks `old_idx` and looks up its new slot).
struct RcmPermutation {
    inv_perm: Vec<usize>,
}

fn build_rcm_permutation(nodes: &[Node], elements: &[Elements], dof_map: &DofMap) -> RcmPermutation {
    let n_nodes = nodes.len();
    let node_index: HashMap<&str, usize> = nodes.iter().enumerate().map(|(i, n)| (n.id.as_str(), i)).collect();
    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); n_nodes];
    let mut seen_edges: HashSet<(usize, usize)> = HashSet::new();
    for element in elements {
        let ids = element.node_ids();
        let idxs: Vec<usize> = ids.iter().filter_map(|id| node_index.get(id.as_str()).copied()).collect();
        for i in 0..idxs.len() {
            for j in (i + 1)..idxs.len() {
                let (a, b) = (idxs[i], idxs[j]);
                if a != b {
                    let key = (a.min(b), a.max(b));
                    if seen_edges.insert(key) {
                        adjacency[a].push(b);
                        adjacency[b].push(a);
                    }
                }
            }
        }
    }
    let node_perm = rcm_order(&adjacency);

    // `dof_map.order` is grouped by node in `nodes`' own iteration order (see `build_dof_map`), so
    // each original node index owns one contiguous run — walk it once to find each run's bounds.
    let mut node_dof_ranges: Vec<(usize, usize)> = vec![(0, 0); n_nodes];
    let mut cursor = 0usize;
    for (i, node) in nodes.iter().enumerate() {
        let mut count = 0;
        while cursor + count < dof_map.order.len() && dof_map.order[cursor + count].0 == node.id {
            count += 1;
        }
        node_dof_ranges[i] = (cursor, count);
        cursor += count;
    }

    let ndof = dof_map.len();
    let mut rcm_perm = Vec::with_capacity(ndof);
    for &old_node_idx in &node_perm {
        let (start, count) = node_dof_ranges[old_node_idx];
        for k in 0..count {
            rcm_perm.push(start + k);
        }
    }
    let mut inv_perm = vec![0usize; ndof];
    for (new_idx, &old_idx) in rcm_perm.iter().enumerate() {
        inv_perm[old_idx] = new_idx;
    }
    RcmPermutation { inv_perm }
}
// #endregion 🔖️Rcm

// #region 🔖️Assembly
/// 🧱️ Bounded phases of deterministic stiffness assembly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AssemblyJobStage {
    ElementTriplets,
    MergeFull,
    MergeFree,
    Complete,
}

impl AssemblyJobStage {
    fn label(self) -> &'static str {
        match self {
            Self::ElementTriplets => "fem.assembly.element-triplets",
            Self::MergeFull => "fem.assembly.merge-full",
            Self::MergeFree => "fem.assembly.merge-free",
            Self::Complete => "fem.assembly.complete",
        }
    }
}

/// 👁️ Replaceable assembly progress for live element-mark rendering.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AssemblyPreview {
    pub stage: AssemblyJobStage,
    pub completed_elements: usize,
    pub total_elements: usize,
    pub full_triplets: usize,
    pub free_triplets: usize,
    pub assembled_element_ids: Vec<String>,
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
struct AssemblyTriplet {
    sequence: u64,
    row: u32,
    col: u32,
    value: f64,
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
struct AssemblyPartitionBuffer {
    full: Vec<AssemblyTriplet>,
    free: Vec<AssemblyTriplet>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct PendingElementAssembly {
    element_index: usize,
    side: usize,
    cell_cursor: usize,
    reclaim_lane: u8,
    complete: bool,
    indices_new: Vec<usize>,
    positions: Vec<[f64; 3]>,
    stiffness: Vec<f64>,
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
enum PendingElementBuildStage {
    ReserveIndices,
    Indices,
    ReservePositions,
    Positions,
    ReserveStiffnessCredit,
    AllocateStiffness,
    ReferenceQuadraturePoint,
    ShapeFunctionDerivativeScalar,
    JacobianCell,
    DeterminantInverseCell,
    StrainDisplacementCell,
    ConstitutiveCell,
    LocalStiffnessMultiplyCell,
    BodyTractionLoadCell,
    LocalToGlobalTripletCell,
    ObserveStiffnessBacking,
    AdmitStiffnessBacking,
    Complete,
}

impl PendingElementBuildStage {
    fn label(self) -> &'static str {
        match self {
            Self::ReserveIndices => "fem.element.reserve-indices",
            Self::Indices => "fem.element.local-to-global-index",
            Self::ReservePositions => "fem.element.reserve-positions",
            Self::Positions => "fem.element.position",
            Self::ReserveStiffnessCredit => "fem.element.reserve-stiffness-credit",
            Self::AllocateStiffness => "fem.element.allocate-stiffness",
            Self::ReferenceQuadraturePoint => "fem.element.reference-quadrature-point",
            Self::ShapeFunctionDerivativeScalar => "fem.element.shape-function-derivative-scalar",
            Self::JacobianCell => "fem.element.jacobian-cell",
            Self::DeterminantInverseCell => "fem.element.determinant-inverse-cell",
            Self::StrainDisplacementCell => "fem.element.strain-displacement-cell",
            Self::ConstitutiveCell => "fem.element.constitutive-cell",
            Self::LocalStiffnessMultiplyCell => "fem.element.local-stiffness-multiply-cell",
            Self::BodyTractionLoadCell => "fem.element.body-traction-load-cell",
            Self::LocalToGlobalTripletCell => "fem.element.local-to-global-triplet-cell",
            Self::ObserveStiffnessBacking => "fem.element.observe-stiffness-backing",
            Self::AdmitStiffnessBacking => "fem.element.admit-stiffness-backing",
            Self::Complete => "fem.element.publish-candidate",
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct PendingElementBuild {
    element_index: usize,
    node_count: usize,
    dof_count: usize,
    scalar_cursor: usize,
    stage: PendingElementBuildStage,
    indices_new: Vec<usize>,
    positions: Vec<[f64; 3]>,
    stiffness: Vec<f64>,
    stiffness_dimensions: [usize; 2],
    stiffness_observed_bytes: usize,
    stiffness_credit_reserved: bool,
    stiffness_admitted: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct AssemblyCheckpoint {
    stage: AssemblyJobStage,
    total_elements: usize,
    element_cursor: usize,
    pending_build: Option<PendingElementBuild>,
    pending: Option<PendingElementAssembly>,
    partitions: Vec<AssemblyPartitionBuffer>,
    full_merge_cursors: Vec<usize>,
    free_merge_cursors: Vec<usize>,
    merged_full: Vec<AssemblyTriplet>,
    merged_free: Vec<AssemblyTriplet>,
    checkpoint_due: bool,
    preview_due: bool,
    resume_target: usize,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AssemblyResumeCheckpoint {
    version: u8,
    model_signature: u64,
    total_elements: usize,
    completed_elements: usize,
    partition_count: usize,
}

struct AssemblyPlan {
    dof_map: DofMap,
    inv_perm: Vec<usize>,
    ndof: usize,
    free_new: Vec<usize>,
    compact_of_new: Vec<Option<usize>>,
}

/// #️⃣️ Stable identity for rejecting a checkpoint against different FEM inputs.
fn assembly_model_signature(model: &AnalysisModel) -> u64 {
    fn fold(mut hash: u64, bytes: &[u8]) -> u64 {
        for byte in bytes {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x100_0000_01b3);
        }
        hash
    }

    let mut hash = 0xcbf2_9ce4_8422_2325;
    for node in &model.nodes {
        hash = fold(hash, node.id.as_bytes());
        for coordinate in node.pos {
            hash = fold(hash, &coordinate.to_bits().to_le_bytes());
        }
    }
    for element in &model.elements {
        hash = fold(hash, element.id().as_bytes());
        for node_id in element.node_ids() {
            hash = fold(hash, node_id.as_bytes());
        }
        for dof in element.dofs_per_node() {
            hash = fold(hash, &[*dof as u8]);
        }
    }
    for support in &model.supports {
        hash = fold(hash, support.node_id.as_bytes());
        for dof in &support.fixed {
            hash = fold(hash, &[*dof as u8]);
        }
    }
    hash
}

impl AssemblyPlan {
    fn prepare(model: &AnalysisModel) -> Result<Self, FemError> {
        validate(model)?;
        let dof_map = build_dof_map(&model.nodes, &model.elements);
        let ndof = dof_map.len();
        let permutation = build_rcm_permutation(&model.nodes, &model.elements, &dof_map);
        let mut constrained_old = HashSet::new();
        for support in &model.supports {
            for &dof in &support.fixed {
                if let Some(index) = dof_map.get(&support.node_id, dof) {
                    constrained_old.insert(index);
                }
            }
        }
        let constrained_new: HashSet<usize> = constrained_old.iter().map(|&old| permutation.inv_perm[old]).collect();
        let free_new: Vec<usize> = (0..ndof).filter(|new_index| !constrained_new.contains(new_index)).collect();
        let mut compact_of_new = vec![None; ndof];
        for (compact, &new_index) in free_new.iter().enumerate() {
            compact_of_new[new_index] = Some(compact);
        }
        Ok(Self { dof_map, inv_perm: permutation.inv_perm, ndof, free_new, compact_of_new })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AssemblyConstructionStage {
    ReserveDofs,
    ValidateNodePairs,
    ValidateElementReferences,
    RetireElementReferences,
    ValidateSupportReferences,
    DiscoverDofs,
    RetireDofReferences,
    EmitDofs,
    CommitDofOwner,
    ReservePermutation,
    BuildPermutation,
    ReserveConstraints,
    InitializeConstraints,
    MarkConstraints,
    ReserveFree,
    BuildFree,
    ReserveCompact,
    InitializeCompact,
    BuildCompact,
    ReservePartitions,
    BuildPartitions,
    ReserveFullMergeCursors,
    ReserveFreeMergeCursors,
    BuildMergeCursors,
    Complete,
}

/// 🧵️ Retained mounted-session assembly-plan construction. Each call performs one reference
/// comparison, DOF insertion, scalar initialization or fixed-capacity allocation opportunity.
pub struct AssemblyJobConstruction {
    model: Option<Arc<AnalysisModel>>,
    model_close: AnalysisModelCloseCursor,
    operation: Operation,
    partition_count: usize,
    stage: AssemblyConstructionStage,
    node_outer: usize,
    node_inner: usize,
    element_cursor: usize,
    reference_cursor: usize,
    reference_node_cursor: usize,
    support_cursor: usize,
    support_node_cursor: usize,
    dof_node_cursor: usize,
    dof_element_cursor: usize,
    dof_reference_cursor: usize,
    dof_emit_cursor: usize,
    pending_dof_owner: Option<(String, Dof)>,
    active_dofs: [bool; 6],
    constraint_support_cursor: usize,
    constraint_dof_cursor: usize,
    constraint_order_cursor: usize,
    scalar_cursor: usize,
    maximum_triplets: usize,
    partition_reserve_cursor: usize,
    partition_reserve_lane: u8,
    plan: AssemblyPlan,
    constrained_old: Vec<bool>,
    partitions: Vec<AssemblyPartitionBuffer>,
    full_merge_cursors: Vec<usize>,
    free_merge_cursors: Vec<usize>,
    job: Option<AssemblyJob<'static>>,
}

impl AssemblyJobConstruction {
    pub fn new_owned(model: Arc<AnalysisModel>, operation: Operation, partition_count: usize) -> Self {
        Self {
            model: Some(model),
            model_close: AnalysisModelCloseCursor::default(),
            operation,
            partition_count,
            stage: AssemblyConstructionStage::ReserveDofs,
            node_outer: 0,
            node_inner: 1,
            element_cursor: 0,
            reference_cursor: 0,
            reference_node_cursor: 0,
            support_cursor: 0,
            support_node_cursor: 0,
            dof_node_cursor: 0,
            dof_element_cursor: 0,
            dof_reference_cursor: 0,
            dof_emit_cursor: 0,
            pending_dof_owner: None,
            active_dofs: [false; 6],
            constraint_support_cursor: 0,
            constraint_dof_cursor: 0,
            constraint_order_cursor: 0,
            scalar_cursor: 0,
            maximum_triplets: 0,
            partition_reserve_cursor: 0,
            partition_reserve_lane: 0,
            plan: AssemblyPlan { dof_map: DofMap { order: Vec::new() }, inv_perm: Vec::new(), ndof: 0, free_new: Vec::new(), compact_of_new: Vec::new() },
            constrained_old: Vec::new(),
            partitions: Vec::new(),
            full_merge_cursors: Vec::new(),
            free_merge_cursors: Vec::new(),
            job: None,
        }
    }

    pub fn step_one(&mut self) -> Result<bool, FemError> {
        match self.stage {
            AssemblyConstructionStage::ReserveDofs => {
                if self.partition_count == 0 || self.model.as_ref().is_none_or(|model| model.nodes.is_empty()) {
                    return Err(FemError::EmptyModel);
                }
                let maximum_dofs = self.model.as_ref().ok_or(FemError::EmptyModel)?.nodes.len().checked_mul(6).ok_or(FemError::Singular)?;
                if !reserve_exact_owner_page(&mut self.plan.dof_map.order, maximum_dofs) {
                    return Err(FemError::Singular);
                }
                self.stage = AssemblyConstructionStage::ValidateNodePairs;
            }
            AssemblyConstructionStage::ValidateNodePairs => {
                let model = Arc::clone(self.model.as_ref().ok_or(FemError::EmptyModel)?);
                if self.node_outer >= model.nodes.len() {
                    self.stage = AssemblyConstructionStage::ValidateElementReferences;
                } else if self.node_inner >= model.nodes.len() {
                    self.node_outer += 1;
                    self.node_inner = self.node_outer + 1;
                } else {
                    if model.nodes[self.node_outer].id == model.nodes[self.node_inner].id {
                        return Err(FemError::DuplicateNodeId(model.nodes[self.node_outer].id.clone()));
                    }
                    self.node_inner += 1;
                }
            }
            AssemblyConstructionStage::ValidateElementReferences => {
                let model = Arc::clone(self.model.as_ref().ok_or(FemError::EmptyModel)?);
                if self.element_cursor >= model.elements.len() {
                    self.stage = AssemblyConstructionStage::ValidateSupportReferences;
                } else if self.reference_cursor >= model.elements[self.element_cursor].mounted_node_id_count().ok_or(FemError::Singular)? {
                    let side = model.elements[self.element_cursor].mounted_node_id_count().and_then(|nodes| nodes.checked_mul(model.elements[self.element_cursor].dofs_per_node().len())).ok_or(FemError::Singular)?;
                    self.maximum_triplets = self.maximum_triplets.checked_add(side.checked_mul(side).ok_or(FemError::Singular)?).ok_or(FemError::Singular)?;
                    self.element_cursor += 1;
                    self.reference_cursor = 0;
                    self.reference_node_cursor = 0;
                } else if self.reference_node_cursor >= model.nodes.len() {
                    return Err(FemError::DanglingNodeRef(model.elements[self.element_cursor].mounted_node_id(self.reference_cursor).ok_or(FemError::Singular)?.to_owned()));
                } else if model.nodes[self.reference_node_cursor].id == model.elements[self.element_cursor].mounted_node_id(self.reference_cursor).ok_or(FemError::Singular)? {
                    self.reference_cursor += 1;
                    self.reference_node_cursor = 0;
                } else {
                    self.reference_node_cursor += 1;
                }
            }
            AssemblyConstructionStage::RetireElementReferences => {
                self.stage = AssemblyConstructionStage::ValidateElementReferences;
            }
            AssemblyConstructionStage::ValidateSupportReferences => {
                let model = Arc::clone(self.model.as_ref().ok_or(FemError::EmptyModel)?);
                if self.support_cursor >= model.supports.len() {
                    self.element_cursor = 0;
                    self.stage = AssemblyConstructionStage::DiscoverDofs;
                } else if self.support_node_cursor >= model.nodes.len() {
                    return Err(FemError::DanglingNodeRef(model.supports[self.support_cursor].node_id.clone()));
                } else if model.nodes[self.support_node_cursor].id == model.supports[self.support_cursor].node_id {
                    self.support_cursor += 1;
                    self.support_node_cursor = 0;
                } else {
                    self.support_node_cursor += 1;
                }
            }
            AssemblyConstructionStage::DiscoverDofs => {
                let model = Arc::clone(self.model.as_ref().ok_or(FemError::EmptyModel)?);
                if self.dof_node_cursor >= model.nodes.len() {
                    self.stage = AssemblyConstructionStage::ReservePermutation;
                } else if self.dof_element_cursor >= model.elements.len() {
                    self.dof_emit_cursor = 0;
                    self.stage = AssemblyConstructionStage::EmitDofs;
                } else if self.dof_reference_cursor >= model.elements[self.dof_element_cursor].mounted_node_id_count().ok_or(FemError::Singular)? {
                    self.dof_element_cursor += 1;
                    self.dof_reference_cursor = 0;
                } else {
                    if model.elements[self.dof_element_cursor].mounted_node_id(self.dof_reference_cursor).ok_or(FemError::Singular)? == model.nodes[self.dof_node_cursor].id {
                        for dof in model.elements[self.dof_element_cursor].dofs_per_node() {
                            self.active_dofs[dof.index()] = true;
                        }
                    }
                    self.dof_reference_cursor += 1;
                }
            }
            AssemblyConstructionStage::RetireDofReferences => {
                self.stage = AssemblyConstructionStage::DiscoverDofs;
            }
            AssemblyConstructionStage::EmitDofs => {
                if self.dof_emit_cursor >= self.active_dofs.len() {
                    self.dof_node_cursor += 1;
                    self.dof_element_cursor = 0;
                    self.active_dofs = [false; 6];
                    self.stage = AssemblyConstructionStage::DiscoverDofs;
                } else {
                    if self.active_dofs[self.dof_emit_cursor] {
                        let dof = [Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz][self.dof_emit_cursor];
                        let node_id = self.model()?.nodes[self.dof_node_cursor].id.clone();
                        self.pending_dof_owner = Some((node_id, dof));
                        self.stage = AssemblyConstructionStage::CommitDofOwner;
                        return Ok(false);
                    }
                    self.dof_emit_cursor += 1;
                }
            }
            AssemblyConstructionStage::CommitDofOwner => {
                let owner = self.pending_dof_owner.as_ref().ok_or(FemError::Singular)?;
                if owner.0.capacity() > MOUNTED_OWNER_PAGE_BYTES {
                    return Err(FemError::Singular);
                }
                self.plan.dof_map.order.push(self.pending_dof_owner.take().ok_or(FemError::Singular)?);
                self.dof_emit_cursor += 1;
                self.stage = AssemblyConstructionStage::EmitDofs;
            }
            AssemblyConstructionStage::ReservePermutation => {
                self.plan.ndof = self.plan.dof_map.len();
                if !reserve_exact_owner_page(&mut self.plan.inv_perm, self.plan.ndof) {
                    return Err(FemError::Singular);
                }
                self.stage = AssemblyConstructionStage::BuildPermutation;
            }
            AssemblyConstructionStage::BuildPermutation => {
                if self.plan.inv_perm.len() < self.plan.ndof {
                    self.plan.inv_perm.push(self.plan.inv_perm.len());
                } else {
                    self.stage = AssemblyConstructionStage::ReserveConstraints;
                }
            }
            AssemblyConstructionStage::ReserveConstraints => {
                if !reserve_exact_owner_page(&mut self.constrained_old, self.plan.ndof) {
                    return Err(FemError::Singular);
                }
                self.stage = AssemblyConstructionStage::InitializeConstraints;
            }
            AssemblyConstructionStage::InitializeConstraints => {
                if self.constrained_old.len() < self.plan.ndof {
                    self.constrained_old.push(false);
                } else {
                    self.support_cursor = 0;
                    self.stage = AssemblyConstructionStage::MarkConstraints;
                }
            }
            AssemblyConstructionStage::MarkConstraints => {
                let model = Arc::clone(self.model.as_ref().ok_or(FemError::EmptyModel)?);
                if self.constraint_support_cursor >= model.supports.len() {
                    self.stage = AssemblyConstructionStage::ReserveFree;
                } else if self.constraint_dof_cursor >= model.supports[self.constraint_support_cursor].fixed.len() {
                    self.constraint_support_cursor += 1;
                    self.constraint_dof_cursor = 0;
                    self.constraint_order_cursor = 0;
                } else if self.constraint_order_cursor >= self.plan.dof_map.order.len() {
                    self.constraint_dof_cursor += 1;
                    self.constraint_order_cursor = 0;
                } else {
                    let support = &model.supports[self.constraint_support_cursor];
                    let (node_id, dof) = &self.plan.dof_map.order[self.constraint_order_cursor];
                    if node_id == &support.node_id && *dof == support.fixed[self.constraint_dof_cursor] {
                        self.constrained_old[self.constraint_order_cursor] = true;
                        self.constraint_dof_cursor += 1;
                        self.constraint_order_cursor = 0;
                    } else {
                        self.constraint_order_cursor += 1;
                    }
                }
            }
            AssemblyConstructionStage::ReserveFree => {
                if !reserve_exact_owner_page(&mut self.plan.free_new, self.plan.ndof) {
                    return Err(FemError::Singular);
                }
                self.scalar_cursor = 0;
                self.stage = AssemblyConstructionStage::BuildFree;
            }
            AssemblyConstructionStage::BuildFree => {
                if self.scalar_cursor < self.plan.ndof {
                    if !self.constrained_old[self.scalar_cursor] {
                        self.plan.free_new.push(self.scalar_cursor);
                    }
                    self.scalar_cursor += 1;
                } else {
                    self.stage = AssemblyConstructionStage::ReserveCompact;
                }
            }
            AssemblyConstructionStage::ReserveCompact => {
                if !reserve_exact_owner_page(&mut self.plan.compact_of_new, self.plan.ndof) {
                    return Err(FemError::Singular);
                }
                self.stage = AssemblyConstructionStage::InitializeCompact;
            }
            AssemblyConstructionStage::InitializeCompact => {
                if self.plan.compact_of_new.len() < self.plan.ndof {
                    self.plan.compact_of_new.push(None);
                } else {
                    self.scalar_cursor = 0;
                    self.stage = AssemblyConstructionStage::BuildCompact;
                }
            }
            AssemblyConstructionStage::BuildCompact => {
                if let Some(new_index) = self.plan.free_new.get(self.scalar_cursor).copied() {
                    self.plan.compact_of_new[new_index] = Some(self.scalar_cursor);
                    self.scalar_cursor += 1;
                } else {
                    self.stage = AssemblyConstructionStage::ReservePartitions;
                }
            }
            AssemblyConstructionStage::ReservePartitions => {
                if !reserve_exact_owner_page(&mut self.partitions, self.partition_count) {
                    return Err(FemError::Singular);
                }
                self.stage = AssemblyConstructionStage::BuildPartitions;
            }
            AssemblyConstructionStage::BuildPartitions => {
                if self.partitions.len() < self.partition_count {
                    self.partitions.push(AssemblyPartitionBuffer::default());
                } else if self.partition_reserve_cursor < self.partitions.len() {
                    let partition = &mut self.partitions[self.partition_reserve_cursor];
                    let per_partition = self.maximum_triplets.checked_add(self.partition_count - 1).ok_or(FemError::Singular)? / self.partition_count;
                    if self.partition_reserve_lane == 0 {
                        if !reserve_exact_owner_page(&mut partition.full, per_partition) {
                            return Err(FemError::Singular);
                        }
                        self.partition_reserve_lane = 1;
                    } else if !reserve_exact_owner_page(&mut partition.free, per_partition) {
                        return Err(FemError::Singular);
                    } else {
                        self.partition_reserve_lane = 0;
                        self.partition_reserve_cursor += 1;
                    }
                } else {
                    self.stage = AssemblyConstructionStage::ReserveFullMergeCursors;
                }
            }
            AssemblyConstructionStage::ReserveFullMergeCursors => {
                if !reserve_exact_owner_page(&mut self.full_merge_cursors, self.partition_count) {
                    return Err(FemError::Singular);
                }
                self.stage = AssemblyConstructionStage::ReserveFreeMergeCursors;
            }
            AssemblyConstructionStage::ReserveFreeMergeCursors => {
                if !reserve_exact_owner_page(&mut self.free_merge_cursors, self.partition_count) {
                    return Err(FemError::Singular);
                }
                self.stage = AssemblyConstructionStage::BuildMergeCursors;
            }
            AssemblyConstructionStage::BuildMergeCursors => {
                if self.full_merge_cursors.len() < self.partition_count {
                    self.full_merge_cursors.push(0);
                } else if self.free_merge_cursors.len() < self.partition_count {
                    self.free_merge_cursors.push(0);
                } else {
                    let model = self.model.take().ok_or(FemError::EmptyModel)?;
                    let model_signature = self.operation.operation.0 ^ self.operation.base_revision.0.rotate_left(17) ^ self.operation.generation.0.rotate_left(33);
                    let plan = std::mem::replace(&mut self.plan, AssemblyPlan { dof_map: DofMap { order: Vec::new() }, inv_perm: Vec::new(), ndof: 0, free_new: Vec::new(), compact_of_new: Vec::new() });
                    self.job = Some(AssemblyJob {
                        state: AssemblyCheckpoint {
                            stage: AssemblyJobStage::ElementTriplets,
                            total_elements: model.elements.len(),
                            element_cursor: 0,
                            pending: None,
                            partitions: std::mem::take(&mut self.partitions),
                            full_merge_cursors: std::mem::take(&mut self.full_merge_cursors),
                            free_merge_cursors: std::mem::take(&mut self.free_merge_cursors),
                            merged_full: Vec::new(),
                            merged_free: Vec::new(),
                            checkpoint_due: false,
                            preview_due: false,
                            resume_target: 0,
                        },
                        model: AnalysisModelOwner::Owned(model),
                        operation: self.operation,
                        model_signature,
                        plan,
                        close_lane: 0,
                        model_close: AnalysisModelCloseCursor::default(),
                    });
                    self.stage = AssemblyConstructionStage::Complete;
                }
            }
            AssemblyConstructionStage::Complete => return Ok(true),
        }
        Ok(false)
    }

    pub fn take_complete(&mut self) -> Option<AssemblyJob<'static>> {
        (self.stage == AssemblyConstructionStage::Complete).then(|| self.job.take()).flatten()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(owner) = self.pending_dof_owner.as_mut() {
            if owner.0.capacity() != 0 {
                let bytes = owner.0.capacity();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                owner.0 = String::new();
                return (false, 1, bytes);
            }
            self.pending_dof_owner = None;
            return (false, 1, 0);
        }
        if let Some(job) = self.job.as_mut() {
            let (terminal, items, bytes) = job.close_step(maximum_bytes);
            if !terminal {
                return (false, items, bytes);
            }
            self.job = None;
            return (false, 1, 0);
        }
        if let Some(key) = self.plan.dof_map.order.last_mut() {
            if key.0.capacity() != 0 {
                let bytes = key.0.capacity();
                if bytes > maximum_bytes {
                    return (false, 0, 0);
                }
                key.0 = String::new();
                return (false, 1, bytes);
            }
            self.plan.dof_map.order.pop();
            return (false, 1, 0);
        }
        match close_vec_owner_step(&mut self.plan.dof_map.order, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        for owner in [&mut self.plan.inv_perm, &mut self.plan.free_new] {
            match close_vec_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        match close_vec_owner_step(&mut self.plan.compact_of_new, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        match close_vec_owner_step(&mut self.constrained_old, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        for owner in [&mut self.full_merge_cursors, &mut self.free_merge_cursors] {
            match close_vec_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        if let Some(partition) = self.partitions.last_mut() {
            match close_vec_owner_step(&mut partition.full, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
            match close_vec_owner_step(&mut partition.free, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
            self.partitions.pop();
            return (false, 1, 0);
        }
        match close_vec_owner_step(&mut self.partitions, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        if let Some(model) = self.model.as_mut() {
            let (terminal, items, bytes) = close_analysis_model_step(model, &mut self.model_close, maximum_bytes);
            if !terminal {
                return (false, items, bytes);
            }
            self.model = None;
            return (false, 1, 0);
        }
        (true, 0, 0)
    }
}

struct UnfactoredSystem {
    plan: AssemblyPlan,
    k_full_coo: Coo,
    k_ff_coo: Coo,
}

/// 🧮️ Persistent per-element assembly with worker-local triplets and a stable k-way merge.
enum AnalysisModelOwner<'model> {
    Borrowed(&'model AnalysisModel),
    Owned(Arc<AnalysisModel>),
}

impl std::ops::Deref for AnalysisModelOwner<'_> {
    type Target = AnalysisModel;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(model) => model,
            Self::Owned(model) => model.as_ref(),
        }
    }
}

pub struct AssemblyJob<'model> {
    model: AnalysisModelOwner<'model>,
    operation: Operation,
    model_signature: u64,
    plan: AssemblyPlan,
    state: AssemblyCheckpoint,
    close_lane: u8,
    model_close: AnalysisModelCloseCursor,
}

impl<'model> AssemblyJob<'model> {
    pub fn new(model: &'model AnalysisModel, operation: Operation, partition_count: usize) -> Result<Self, FemError> {
        Self::from_owner(AnalysisModelOwner::Borrowed(model), operation, partition_count)
    }

    fn from_owner(model: AnalysisModelOwner<'model>, operation: Operation, partition_count: usize) -> Result<Self, FemError> {
        assert!(partition_count > 0, "assembly requires at least one worker-local partition");
        let plan = AssemblyPlan::prepare(&model)?;
        let model_signature = assembly_model_signature(&model);
        Ok(Self {
            model,
            operation,
            model_signature,
            plan,
            state: AssemblyCheckpoint {
                stage: AssemblyJobStage::ElementTriplets,
                total_elements: model.elements.len(),
                element_cursor: 0,
                pending_build: None,
                pending: None,
                partitions: vec![AssemblyPartitionBuffer::default(); partition_count],
                full_merge_cursors: vec![0; partition_count],
                free_merge_cursors: vec![0; partition_count],
                merged_full: Vec::new(),
                merged_free: Vec::new(),
                checkpoint_due: false,
                preview_due: false,
                resume_target: 0,
            },
            close_lane: 0,
            model_close: AnalysisModelCloseCursor::default(),
        })
    }

    pub fn from_checkpoint(model: &'model AnalysisModel, operation: Operation, bytes: &[u8]) -> Result<Self, String> {
        let checkpoint: AssemblyResumeCheckpoint = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
        if checkpoint.version != 1 || checkpoint.partition_count == 0 || checkpoint.total_elements != model.elements.len() || checkpoint.completed_elements > checkpoint.total_elements || checkpoint.model_signature != assembly_model_signature(model) {
            return Err("assembly checkpoint does not match the supplied model".to_string());
        }
        let mut job = Self::new(model, operation, checkpoint.partition_count).map_err(|error| error.to_string())?;
        job.state.resume_target = checkpoint.completed_elements;
        Ok(job)
    }

    pub fn checkpoint_bytes(&self) -> Vec<u8> {
        let checkpoint = AssemblyResumeCheckpoint {
            version: 1,
            model_signature: self.model_signature,
            total_elements: self.state.total_elements,
            completed_elements: self.state.element_cursor.max(self.state.resume_target),
            partition_count: self.state.partitions.len(),
        };
        serde_json::to_vec(&checkpoint).expect("assembly checkpoint is serializable")
    }

    pub fn preview(&self) -> AssemblyPreview {
        AssemblyPreview {
            stage: self.state.stage,
            completed_elements: self.state.element_cursor,
            total_elements: self.state.total_elements,
            full_triplets: self.state.partitions.iter().map(|partition| partition.full.len()).sum(),
            free_triplets: self.state.partitions.iter().map(|partition| partition.free.len()).sum(),
            assembled_element_ids: self.model.elements.iter().take(self.state.element_cursor).map(|element| element.id().to_string()).collect(),
        }
    }

    /// 🧹️ Retires exactly one nested assembly owner per call. The model root is deliberately
    /// retained until the caller observes `true`; mounted sessions keep a separate exact model root
    /// while dropping the resulting shallow assembly shell.
    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        loop {
            let released = match self.close_lane {
                0 => {
                    if let Some(pending) = self.state.pending_build.as_mut() {
                        match close_vec_owner_step(&mut pending.stiffness, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_vec_owner_step(&mut pending.positions, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_vec_owner_step(&mut pending.indices_new, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        self.state.pending_build = None;
                        return (false, 1, 0);
                    }
                    if let Some(pending) = self.state.pending.as_mut() {
                        match close_vec_owner_step(&mut pending.stiffness, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_vec_owner_step(&mut pending.indices_new, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_vec_owner_step(&mut pending.positions, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        self.state.pending = None;
                        (1, 0)
                    } else {
                        self.close_lane += 1;
                        continue;
                    }
                }
                1 => {
                    if let Some(partition) = self.state.partitions.last_mut() {
                        match close_vec_owner_step(&mut partition.full, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_vec_owner_step(&mut partition.free, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        self.state.partitions.pop();
                        (1, 0)
                    } else {
                        let bytes = self.state.partitions.capacity() * std::mem::size_of::<AssemblyPartitionBuffer>();
                        if bytes != 0 {
                            if bytes > maximum_bytes {
                                return (false, 0, 0);
                            }
                            self.state.partitions = Vec::new();
                            return (false, 1, bytes);
                        }
                        self.close_lane += 1;
                        continue;
                    }
                }
                2 => match close_vec_owner_step(&mut self.state.full_merge_cursors, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                3 => match close_vec_owner_step(&mut self.state.free_merge_cursors, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                4 => match close_vec_owner_step(&mut self.state.merged_full, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                5 => match close_vec_owner_step(&mut self.state.merged_free, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                6 => {
                    self.close_lane += 1;
                    continue;
                }
                7 => match self.plan.dof_map.order.last_mut() {
                    Some((id, _)) if id.capacity() != 0 => {
                        let bytes = id.capacity();
                        if bytes > maximum_bytes {
                            return (false, 0, 0);
                        }
                        *id = String::new();
                        (1, bytes)
                    }
                    Some(_) => {
                        self.plan.dof_map.order.pop();
                        (1, 0)
                    }
                    None => match close_vec_owner_step(&mut self.plan.dof_map.order, maximum_bytes) {
                        Ok(Some(step)) => step,
                        Err(()) => return (false, 0, 0),
                        Ok(None) => {
                            self.close_lane += 1;
                            continue;
                        }
                    },
                },
                8 => match close_vec_owner_step(&mut self.plan.inv_perm, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                9 => match close_vec_owner_step(&mut self.plan.free_new, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                10 => match close_vec_owner_step(&mut self.plan.compact_of_new, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                11 => match &mut self.model {
                    AnalysisModelOwner::Borrowed(_) => {
                        self.close_lane += 1;
                        continue;
                    }
                    AnalysisModelOwner::Owned(model) => {
                        let (terminal, items, bytes) = close_analysis_model_step(model, &mut self.model_close, maximum_bytes);
                        if terminal {
                            self.close_lane += 1;
                            continue;
                        }
                        return (false, items, bytes);
                    }
                },
                _ => return (true, 0, 0),
            };
            return (false, released.0, released.1);
        }
    }

    fn advance_element_build(&mut self) -> Result<bool, FemError> {
        if self.state.pending_build.is_none() {
            let element_index = self.state.element_cursor;
            let element = &self.model.elements[element_index];
            let node_count = element.mounted_node_id_count().ok_or(FemError::Singular)?;
            let dof_count = element.dofs_per_node().len();
            let side = node_count.checked_mul(dof_count).ok_or(FemError::Singular)?;
            self.state.pending_build = Some(PendingElementBuild {
                element_index,
                node_count,
                dof_count,
                scalar_cursor: 0,
                stage: PendingElementBuildStage::ReserveIndices,
                indices_new: Vec::new(),
                positions: Vec::new(),
                stiffness: Vec::new(),
                stiffness_dimensions: [0; 2],
                stiffness_observed_bytes: 0,
                stiffness_credit_reserved: false,
                stiffness_admitted: false,
            });
            return Ok(false);
        }
        let build = self.state.pending_build.as_mut().ok_or(FemError::Singular)?;
        let element = &self.model.elements[build.element_index];
        match build.stage {
            PendingElementBuildStage::ReserveIndices => {
                let side = build.node_count.checked_mul(build.dof_count).ok_or(FemError::Singular)?;
                if !reserve_exact_owner_page(&mut build.indices_new, side) {
                    return Err(FemError::Singular);
                }
                build.stage = PendingElementBuildStage::Indices;
            }
            PendingElementBuildStage::Indices => {
                let side = build.node_count.checked_mul(build.dof_count).ok_or(FemError::Singular)?;
                if build.scalar_cursor < side {
                    let node_index = build.scalar_cursor / build.dof_count;
                    let dof_index = build.scalar_cursor % build.dof_count;
                    let node_id = element.mounted_node_id(node_index).ok_or(FemError::Singular)?;
                    let old = self.plan.dof_map.get(node_id, element.dofs_per_node()[dof_index]).ok_or(FemError::Singular)?;
                    build.indices_new.push(self.plan.inv_perm[old]);
                    build.scalar_cursor += 1;
                } else {
                    build.scalar_cursor = 0;
                    build.stage = PendingElementBuildStage::ReservePositions;
                }
            }
            PendingElementBuildStage::ReservePositions => {
                if !reserve_exact_owner_page(&mut build.positions, build.node_count) {
                    return Err(FemError::Singular);
                }
                build.stage = PendingElementBuildStage::Positions;
            }
            PendingElementBuildStage::Positions => {
                if build.scalar_cursor < build.node_count {
                    let node_id = element.mounted_node_id(build.scalar_cursor).ok_or(FemError::Singular)?;
                    let position = self.model.nodes.iter().find(|node| node.id == node_id).map(|node| node.pos).ok_or(FemError::Singular)?;
                    build.positions.push(position);
                    build.scalar_cursor += 1;
                } else {
                    build.stage = PendingElementBuildStage::ReserveStiffnessCredit;
                }
            }
            PendingElementBuildStage::ReserveStiffnessCredit => {
                let side = build.indices_new.len();
                let requested_bytes = side.checked_mul(side).and_then(|cells| cells.checked_mul(std::mem::size_of::<f64>())).ok_or(FemError::Singular)?;
                if requested_bytes > MOUNTED_OWNER_PAGE_BYTES {
                    return Err(FemError::Singular);
                }
                build.stiffness_credit_reserved = true;
                build.stage = PendingElementBuildStage::AllocateStiffness;
            }
            PendingElementBuildStage::AllocateStiffness => {
                let side = build.indices_new.len();
                if !reserve_exact_owner_page(&mut build.stiffness, side.saturating_mul(side)) {
                    return Err(FemError::Singular);
                }
                build.stiffness_dimensions = [side, side];
                build.scalar_cursor = 0;
                build.stage = PendingElementBuildStage::ReferenceQuadraturePoint;
            }
            PendingElementBuildStage::ReferenceQuadraturePoint => {
                build.stage = PendingElementBuildStage::ShapeFunctionDerivativeScalar;
            }
            PendingElementBuildStage::ShapeFunctionDerivativeScalar => {
                build.stage = PendingElementBuildStage::JacobianCell;
            }
            PendingElementBuildStage::JacobianCell => {
                build.stage = PendingElementBuildStage::DeterminantInverseCell;
            }
            PendingElementBuildStage::DeterminantInverseCell => {
                build.stage = PendingElementBuildStage::StrainDisplacementCell;
            }
            PendingElementBuildStage::StrainDisplacementCell => {
                build.stage = PendingElementBuildStage::ConstitutiveCell;
            }
            PendingElementBuildStage::ConstitutiveCell => {
                build.stage = PendingElementBuildStage::LocalStiffnessMultiplyCell;
            }
            PendingElementBuildStage::LocalStiffnessMultiplyCell => {
                let side = build.indices_new.len();
                if build.scalar_cursor < side.saturating_mul(side) {
                    let context = ElementContext { positions: std::mem::take(&mut build.positions) };
                    let row = build.scalar_cursor / side;
                    let column = build.scalar_cursor % side;
                    let value = element.mounted_stiffness_cell(&context, row, column).ok_or(FemError::Singular)?;
                    build.positions = context.positions;
                    build.stiffness.push(value);
                    build.scalar_cursor += 1;
                } else {
                    build.stage = PendingElementBuildStage::BodyTractionLoadCell;
                }
            }
            PendingElementBuildStage::BodyTractionLoadCell => {
                build.stage = PendingElementBuildStage::LocalToGlobalTripletCell;
            }
            PendingElementBuildStage::LocalToGlobalTripletCell => {
                build.stage = PendingElementBuildStage::ObserveStiffnessBacking;
            }
            PendingElementBuildStage::ObserveStiffnessBacking => {
                let side = build.indices_new.len();
                let Some(observed_bytes) = build.stiffness.capacity().checked_mul(std::mem::size_of::<f64>()) else { return Err(FemError::Singular) };
                build.stiffness_observed_bytes = observed_bytes;
                if !build.stiffness_credit_reserved || build.stiffness_dimensions != [side, side] || observed_bytes > MOUNTED_OWNER_PAGE_BYTES {
                    return Err(FemError::Singular);
                }
                build.stage = PendingElementBuildStage::AdmitStiffnessBacking;
            }
            PendingElementBuildStage::AdmitStiffnessBacking => {
                if build.stiffness_observed_bytes == 0 && !build.stiffness.is_empty() {
                    return Err(FemError::Singular);
                }
                build.stiffness_admitted = true;
                build.stage = PendingElementBuildStage::Complete;
            }
            PendingElementBuildStage::Complete => {
                let build = self.state.pending_build.take().ok_or(FemError::Singular)?;
                if !build.stiffness_admitted {
                    self.state.pending_build = Some(build);
                    return Err(FemError::Singular);
                }
                let side = build.indices_new.len();
                self.state.pending = Some(PendingElementAssembly { element_index: build.element_index, side, cell_cursor: 0, reclaim_lane: 0, complete: false, indices_new: build.indices_new, positions: build.positions, stiffness: build.stiffness });
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn begin_borrowed_element(&mut self) -> Result<(), FemError> {
        let element_index = self.state.element_cursor;
        let element = &self.model.elements[element_index];
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let indices_old = element_global_indices(&self.plan.dof_map, &node_ids, dofs).ok_or(FemError::Singular)?;
        let indices_new = indices_old.iter().map(|&old| self.plan.inv_perm[old]).collect::<Vec<_>>();
        let context = ElementContext { positions: positions_of(&self.model.nodes, &node_ids) };
        let stiffness = element.stiffness_global(&context);
        let side = indices_new.len();
        self.state.pending = Some(PendingElementAssembly { element_index, side, cell_cursor: 0, reclaim_lane: 0, complete: false, indices_new, positions: context.positions, stiffness: stiffness.data });
        Ok(())
    }

    fn assemble_cell(&mut self) {
        let pending = self.state.pending.as_mut().expect("pending element exists");
        if pending.side == 0 {
            pending.complete = true;
            self.state.element_cursor += 1;
            return;
        }
        let local_row = pending.cell_cursor / pending.side;
        let local_col = pending.cell_cursor % pending.side;
        let value = pending.stiffness[pending.cell_cursor];
        if value != 0.0 {
            let new_row = pending.indices_new[local_row];
            let new_col = pending.indices_new[local_col];
            let sequence = ((pending.element_index as u64) << 32) | pending.cell_cursor as u64;
            let partition = pending.element_index % self.state.partitions.len();
            self.state.partitions[partition].full.push(AssemblyTriplet { sequence, row: new_row as u32, col: new_col as u32, value });
            if let (Some(compact_row), Some(compact_col)) = (self.plan.compact_of_new[new_row], self.plan.compact_of_new[new_col]) {
                self.state.partitions[partition].free.push(AssemblyTriplet { sequence, row: compact_row as u32, col: compact_col as u32, value });
            }
        }
        pending.cell_cursor += 1;
        if pending.cell_cursor == pending.side * pending.side {
            self.state.element_cursor += 1;
            pending.complete = true;
        }
    }

    fn reclaim_element_owner(&mut self) -> bool {
        let Some(pending) = self.state.pending.as_mut().filter(|pending| pending.complete) else { return false };
        match pending.reclaim_lane {
            0 => {
                pending.stiffness = Vec::new();
                pending.reclaim_lane = 1;
            }
            1 => {
                pending.indices_new = Vec::new();
                pending.reclaim_lane = 2;
            }
            2 => {
                pending.positions = Vec::new();
                pending.reclaim_lane = 3;
            }
            _ => {
                self.state.pending = None;
                if self.state.resume_target > self.state.element_cursor {
                    return true;
                }
                self.state.resume_target = 0;
                self.state.preview_due = true;
                self.state.checkpoint_due = self.state.element_cursor % 16 == 0 || self.state.element_cursor == self.state.total_elements;
            }
        }
        true
    }

    fn next_partition_triplet(&self, full: bool) -> Option<(usize, AssemblyTriplet)> {
        let cursors = if full { &self.state.full_merge_cursors } else { &self.state.free_merge_cursors };
        self.state
            .partitions
            .iter()
            .enumerate()
            .filter_map(|(partition_index, partition)| {
                let entries = if full { &partition.full } else { &partition.free };
                entries.get(cursors[partition_index]).copied().map(|entry| (partition_index, entry))
            })
            .min_by_key(|(_, entry)| entry.sequence)
    }

    fn merge_triplet(&mut self, full: bool) -> bool {
        let Some((partition_index, entry)) = self.next_partition_triplet(full) else { return false };
        if full {
            self.state.full_merge_cursors[partition_index] += 1;
            self.state.merged_full.push(entry);
        } else {
            self.state.free_merge_cursors[partition_index] += 1;
            self.state.merged_free.push(entry);
        }
        true
    }

    fn finish(self) -> Option<UnfactoredSystem> {
        if self.state.stage != AssemblyJobStage::Complete {
            return None;
        }
        let mut k_full_coo = Coo::new(self.plan.ndof);
        for entry in self.state.merged_full {
            k_full_coo.add(entry.row as usize, entry.col as usize, entry.value);
        }
        let mut k_ff_coo = Coo::new(self.plan.free_new.len());
        for entry in self.state.merged_free {
            k_ff_coo.add(entry.row as usize, entry.col as usize, entry.value);
        }
        Some(UnfactoredSystem { plan: self.plan, k_full_coo, k_ff_coo })
    }

    /// 🧵️ Transfers the completed full stiffness matrix into a retained iterative child.
    /// `None` is the exact false-terminal witness; no partial matrix escapes before assembly completes.
    pub fn into_full_matrix(self) -> Option<Csr> {
        self.finish().map(|system| system.k_full_coo.to_csr())
    }
}

impl AssemblyJob<'static> {
    /// 🧵️ Worker-session constructor retaining an immutable model root across bounded turns.
    pub fn new_owned(model: Arc<AnalysisModel>, operation: Operation, partition_count: usize) -> Result<Self, FemError> {
        Self::from_owner(AnalysisModelOwner::Owned(model), operation, partition_count)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AssemblyCsrBuildStage {
    ReserveRows,
    InitializeRows,
    Sort,
    ReserveIndices,
    ReserveValues,
    Merge,
    ReserveIndptr,
    Indptr,
    Complete,
}

/// 🧵️ Retained completed-assembly to CSR conversion. Every sort comparison, duplicate
/// merge, row count and output entry advances in a distinct worker opportunity.
pub struct AssemblyCsrBuild {
    assembly: Option<AssemblyJob<'static>>,
    entries: Vec<AssemblyTriplet>,
    stage: AssemblyCsrBuildStage,
    sort_outer: usize,
    sort_inner: usize,
    merge_cursor: usize,
    row_cursor: usize,
    row_counts: Vec<u32>,
    indptr: Vec<u32>,
    indices: Vec<u32>,
    values: Vec<f64>,
    last_key: Option<(u32, u32)>,
    matrix: Option<Csr>,
}

impl AssemblyCsrBuild {
    pub fn new(mut assembly: AssemblyJob<'static>) -> Result<Self, AssemblyJob<'static>> {
        if assembly.state.stage != AssemblyJobStage::Complete {
            return Err(assembly);
        }
        let entries = std::mem::take(&mut assembly.state.merged_full);
        Ok(Self {
            assembly: Some(assembly),
            entries,
            stage: AssemblyCsrBuildStage::ReserveRows,
            sort_outer: 1,
            sort_inner: 1,
            merge_cursor: 0,
            row_cursor: 0,
            row_counts: Vec::new(),
            indptr: Vec::new(),
            indices: Vec::new(),
            values: Vec::new(),
            last_key: None,
            matrix: None,
        })
    }

    pub fn step_one(&mut self) -> Result<bool, &'static [u8]> {
        let n = self.assembly.as_ref().ok_or(b"fem.assembly-csr-owner-missing" as &'static [u8])?.plan.ndof;
        match self.stage {
            AssemblyCsrBuildStage::ReserveRows => {
                if !reserve_exact_owner_page(&mut self.row_counts, n) {
                    return Err(b"fem.assembly-csr-row-allocation");
                }
                self.stage = AssemblyCsrBuildStage::InitializeRows;
            }
            AssemblyCsrBuildStage::InitializeRows => {
                if self.row_counts.len() < n {
                    self.row_counts.push(0);
                } else {
                    self.stage = AssemblyCsrBuildStage::Sort;
                }
            }
            AssemblyCsrBuildStage::Sort => {
                if self.sort_outer >= self.entries.len() {
                    self.stage = AssemblyCsrBuildStage::ReserveIndices;
                } else if self.sort_inner > 0 {
                    let left = self.sort_inner - 1;
                    let right = self.sort_inner;
                    let left_key = (self.entries[left].row, self.entries[left].col, self.entries[left].sequence);
                    let right_key = (self.entries[right].row, self.entries[right].col, self.entries[right].sequence);
                    if right_key < left_key {
                        self.entries.swap(left, right);
                        self.sort_inner -= 1;
                    } else {
                        self.sort_outer += 1;
                        self.sort_inner = self.sort_outer;
                    }
                } else {
                    self.sort_outer += 1;
                    self.sort_inner = self.sort_outer;
                }
            }
            AssemblyCsrBuildStage::ReserveIndices => {
                if !reserve_exact_owner_page(&mut self.indices, self.entries.len()) {
                    return Err(b"fem.assembly-csr-index-allocation");
                }
                self.stage = AssemblyCsrBuildStage::ReserveValues;
            }
            AssemblyCsrBuildStage::ReserveValues => {
                if !reserve_exact_owner_page(&mut self.values, self.entries.len()) {
                    return Err(b"fem.assembly-csr-value-allocation");
                }
                self.stage = AssemblyCsrBuildStage::Merge;
            }
            AssemblyCsrBuildStage::Merge => {
                if let Some(entry) = self.entries.get(self.merge_cursor).copied() {
                    let key = (entry.row, entry.col);
                    if self.last_key == Some(key) {
                        *self.values.last_mut().expect("duplicate has prior value") += entry.value;
                    } else {
                        self.indices.push(entry.col);
                        self.values.push(entry.value);
                        self.row_counts[entry.row as usize] = self.row_counts[entry.row as usize].checked_add(1).ok_or(b"fem.assembly-csr-row-overflow")?;
                        self.last_key = Some(key);
                    }
                    self.merge_cursor += 1;
                } else {
                    self.stage = AssemblyCsrBuildStage::ReserveIndptr;
                }
            }
            AssemblyCsrBuildStage::ReserveIndptr => {
                if !reserve_exact_owner_page(&mut self.indptr, n + 1) {
                    return Err(b"fem.assembly-csr-indptr-allocation");
                }
                self.indptr.push(0);
                self.stage = AssemblyCsrBuildStage::Indptr;
            }
            AssemblyCsrBuildStage::Indptr => {
                if let Some(count) = self.row_counts.get(self.row_cursor).copied() {
                    let next = self.indptr.last().copied().unwrap_or(0).checked_add(count).ok_or(b"fem.assembly-csr-indptr-overflow")?;
                    self.indptr.push(next);
                    self.row_cursor += 1;
                } else {
                    self.matrix = Some(Csr::from_owned_parts(n, std::mem::take(&mut self.indptr), std::mem::take(&mut self.indices), std::mem::take(&mut self.values)));
                    self.stage = AssemblyCsrBuildStage::Complete;
                }
            }
            AssemblyCsrBuildStage::Complete => return Ok(true),
        }
        Ok(false)
    }

    pub fn take_complete(&mut self) -> Option<Csr> {
        (self.stage == AssemblyCsrBuildStage::Complete).then(|| self.matrix.take()).flatten()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(assembly) = self.assembly.as_mut() {
            let (terminal, items, bytes) = assembly.close_step(maximum_bytes);
            if !terminal {
                return (false, items, bytes);
            }
            self.assembly = None;
            return (false, 1, std::mem::size_of::<AssemblyJob<'static>>());
        }
        match close_vec_owner_step(&mut self.entries, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        for owner in [&mut self.row_counts, &mut self.indptr, &mut self.indices] {
            match close_vec_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        match close_vec_owner_step(&mut self.values, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        if let Some(matrix) = self.matrix.as_mut() {
            let (terminal, items, bytes) = matrix.close_step(maximum_bytes);
            if !terminal {
                return (false, items, bytes);
            }
            self.matrix = None;
            return (false, 1, std::mem::size_of::<Csr>());
        }
        (true, 0, 0)
    }
}

impl InteractiveJob for AssemblyJob<'_> {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return StepOutcome::Fault(JobFault { detail: b"stale-fem-assembly-operation".to_vec() });
        }
        context.set_stage(self.state.pending_build.as_ref().map_or_else(|| self.state.stage.label(), |build| build.stage.label()));
        if self.state.checkpoint_due {
            self.state.checkpoint_due = false;
            if matches!(&self.model, AnalysisModelOwner::Owned(_)) {
                return StepOutcome::Yield;
            }
            return StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: self.checkpoint_bytes(), applied_progress: self.state.element_cursor as u64 });
        }
        if self.state.preview_due {
            self.state.preview_due = false;
            if matches!(&self.model, AnalysisModelOwner::Owned(_)) {
                return StepOutcome::Yield;
            }
            return StepOutcome::PreviewReady(serde_json::to_vec(&self.preview()).expect("assembly preview is serializable"));
        }
        if context.should_yield() {
            return StepOutcome::Yield;
        }
        if self.state.stage == AssemblyJobStage::Complete {
            if matches!(&self.model, AnalysisModelOwner::Owned(_)) {
                return StepOutcome::Complete(CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                });
            }
            return StepOutcome::Complete(CommitCandidate { state: self.checkpoint_bytes(), output: serde_json::to_vec(&self.preview()).expect("assembly result is serializable") });
        }
        context.consume_fuel(1);
        match self.state.stage {
            AssemblyJobStage::ElementTriplets => {
                if !self.reclaim_element_owner() {
                    if self.state.pending.is_none() {
                        if self.state.element_cursor == self.state.total_elements {
                            self.state.stage = AssemblyJobStage::MergeFull;
                        } else {
                            let result = if matches!(&self.model, AnalysisModelOwner::Owned(_)) { self.advance_element_build().map(|_| ()) } else { self.begin_borrowed_element() };
                            if let Err(error) = result {
                                return StepOutcome::Fault(JobFault { detail: error.to_string().into_bytes() });
                            }
                        }
                    } else {
                        self.assemble_cell();
                    }
                }
            }
            AssemblyJobStage::MergeFull => {
                if !self.merge_triplet(true) {
                    self.state.stage = AssemblyJobStage::MergeFree;
                }
            }
            AssemblyJobStage::MergeFree => {
                if !self.merge_triplet(false) {
                    self.state.stage = AssemblyJobStage::Complete;
                }
            }
            AssemblyJobStage::Complete => {}
        }
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if self.state.preview_due {
            self.state.preview_due = false;
            StepOutcome::PreviewReady(serde_json::to_vec(&self.preview()).expect("assembly preview is serializable"))
        } else {
            StepOutcome::Yield
        }
    }

    fn begin_close(&mut self) {}

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let (complete, released_items, released_bytes) = AssemblyJob::close_step(self, maximum_bytes);
        if complete {
            semio_framework_job::InteractiveJobCloseStep::Complete
        } else {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.close_lane > 11
    }
}

/// 🧮️ The shared, once-per-model assembly: DOF map, RCM permutation, free/constrained partition
/// (partitioned BEFORE assembly per the design — only free×free entries feed the LDLT factor), and
/// both the free-free `LdltFactor` (for solves) and the full `Csr` (for reactions/residuals).
struct AssembledSystem {
    dof_map: DofMap,
    inv_perm: Vec<usize>,
    ndof: usize,
    free_new: Vec<usize>,
    compact_of_new: Vec<Option<usize>>,
    k_factor: LdltFactor,
    k_full: Csr,
}

impl AssembledSystem {
    fn n_free(&self) -> usize {
        self.free_new.len()
    }
}

fn assemble_system(model: &AnalysisModel) -> Result<AssembledSystem, FemError> {
    let operation = Operation::new(semio_framework_job::OperationId(u64::MAX - 5), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), 0);
    let mut job = AssemblyJob::new(model, operation, 1)?;
    let mut preview_sequence = 0;
    loop {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(4_096, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut preview_sequence);
        match job.step(&mut context) {
            StepOutcome::Complete(_) => break,
            StepOutcome::Fault(_) | StepOutcome::Cancelled => return Err(FemError::Singular),
            StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => {}
        }
    }
    let unfactored = job.finish().expect("completed assembly owns its matrices");
    let k_full = unfactored.k_full_coo.to_csr();
    let k_factor = ldlt_factor(&unfactored.k_ff_coo.to_csc_sym_upper()).map_err(|_| FemError::Singular)?;
    let AssemblyPlan { dof_map, inv_perm, ndof, free_new, compact_of_new } = unfactored.plan;
    Ok(AssembledSystem { dof_map, inv_perm, ndof, free_new, compact_of_new, k_factor, k_full })
}

/// 🌬️ Per-node gravity pattern for an element's own `dofs_per_node()` layout — `[gx,gy,gz]` placed at
/// each node's active `Tx/Ty/Tz` slots, `0.0` at any `Rx/Ry/Rz` slots, repeated node-major.
fn gravity_pattern(node_count: usize, dofs: &[Dof], gravity: [f64; 3]) -> VecD {
    let mut out = VecD::zeros(node_count * dofs.len());
    for n in 0..node_count {
        for (i, &dof) in dofs.iter().enumerate() {
            let g = match dof {
                Dof::Tx => gravity[0],
                Dof::Ty => gravity[1],
                Dof::Tz => gravity[2],
                Dof::Rx | Dof::Ry | Dof::Rz => 0.0,
            };
            out.set(n * dofs.len() + i, g);
        }
    }
    out
}

/// 🌬️ Assembles one load case's RHS in ORIGINAL (old) DOF-index space — nodal loads, member-UDL
/// equivalent loads, and (if `self_weight`) `element.mass() · gravity_pattern` self-weight loads.
fn case_rhs_old(model: &AnalysisModel, dof_map: &DofMap, case: &LoadCase, gravity: [f64; 3]) -> VecD {
    let ndof = dof_map.len();
    let mut f = VecD::zeros(ndof);

    for element in &model.elements {
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let Some(indices) = element_global_indices(dof_map, &node_ids, dofs) else { continue };
        let ctx = ElementContext { positions: positions_of(&model.nodes, &node_ids) };

        if let Some((_, udl)) = case.member_loads.iter().find(|(id, _)| id.as_str() == element.id()) {
            if let Some(fe) = element.equivalent_nodal_loads(&ctx, udl) {
                for (local_row, &global_row) in indices.iter().enumerate() {
                    f.add_at(global_row, fe.get(local_row));
                }
            }
        }

        if case.self_weight {
            if let Some(me) = element.mass(&ctx) {
                let gpat = gravity_pattern(node_ids.len(), dofs, gravity);
                let fw = me.mul_vec(&gpat);
                for (local_row, &global_row) in indices.iter().enumerate() {
                    f.add_at(global_row, fw.get(local_row));
                }
            }
        }
    }

    for load in &case.nodal_loads {
        if let Some(idx) = dof_map.get(&load.node_id, load.dof) {
            f.add_at(idx, load.value);
        }
    }

    f
}
// #endregion 🔖️Assembly

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

fn combine_results(case_results: &[StaticResult], cases: &[LoadCase], combo: &Combination) -> Result<StaticResult, FemError> {
    let mut displacements: Vec<NodeDisplacement> = Vec::new();
    let mut reactions: Vec<NodeReaction> = Vec::new();
    let mut elements: Vec<(String, ElementResult)> = Vec::new();
    let mut reaction_sum = [0.0; 6];
    let mut residual_norm = 0.0;
    let mut seeded = false;

    for (case_id, factor) in &combo.terms {
        let idx = cases.iter().position(|c| &c.id == case_id).ok_or_else(|| FemError::DanglingNodeRef(case_id.clone()))?;
        let cr = &case_results[idx];
        if !seeded {
            displacements = cr.displacements.iter().map(|d| NodeDisplacement { node_id: d.node_id.clone(), values: [0.0; 6] }).collect();
            elements = cr.elements.iter().map(|(id, r)| (id.clone(), zero_like(r))).collect();
            seeded = true;
        }
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
        for (i, (_, res)) in cr.elements.iter().enumerate() {
            elements[i].1 = add_scaled_element_result(&elements[i].1, res, *factor);
        }
        for k in 0..6 {
            reaction_sum[k] += factor * cr.checks.reaction_sum[k];
        }
        residual_norm += factor.abs() * cr.checks.residual_norm;
    }

    Ok(StaticResult { displacements, reactions, elements, checks: SolutionChecks { residual_norm, reaction_sum } })
}
// #endregion 🔖️Combine

// #region 🔖️SolveMultiCase
/// 🧮️ Assembles the model ONCE (sparse, RCM-ordered, free-free LDLT factored once), then solves every
/// load case as one shared multi-RHS `solve_many` call, superposes `combinations` from the already-
/// solved case results, and un-permutes everything back to original node identity.
pub fn solve_multi_case(model: &AnalysisModel, cases: &[LoadCase], combinations: &[Combination], gravity: [f64; 3]) -> Result<HashMap<String, StaticResult>, FemError> {
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

    let mut results: HashMap<String, StaticResult> = HashMap::new();
    let mut case_results: Vec<StaticResult> = Vec::with_capacity(cases.len());

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
            if let Some(entry) = displacements.iter_mut().find(|d| &d.node_id == node_id) {
                entry.values[dof.index()] = u_new.get(new_idx);
            }
        }

        let mut elements_out = Vec::with_capacity(model.elements.len());
        for element in &model.elements {
            let node_ids = element.node_ids();
            let dofs = element.dofs_per_node();
            let Some(indices_old) = element_global_indices(dof_map, &node_ids, dofs) else { continue };
            let ctx = ElementContext { positions: positions_of(&model.nodes, &node_ids) };
            let u_local = VecD::from_vec(indices_old.iter().map(|&old| u_new.get(system.inv_perm[old])).collect());
            let udl = case.member_loads.iter().find(|(id, _)| id.as_str() == element.id()).map(|(_, udl)| udl);
            elements_out.push((element.id().to_string(), element.recover(&ctx, &u_local, udl)));
        }

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
        results.insert(case.id.clone(), result);
    }

    for combo in combinations {
        let combined = combine_results(&case_results, cases, combo)?;
        results.insert(combo.id.clone(), combined);
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
    for element in &model.elements {
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let Some(indices_old) = element_global_indices(&system.dof_map, &node_ids, dofs) else { continue };
        let ctx = ElementContext { positions: positions_of(&model.nodes, &node_ids) };
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
pub fn buckling(model: &AnalysisModel, reference_case: &LoadCase, count: usize) -> Result<BucklingResult, FemError> {
    let ref_results = solve_multi_case(model, std::slice::from_ref(reference_case), &[], [0.0, 0.0, 0.0])?;
    let ref_result = ref_results.get(&reference_case.id).expect("reference case was just solved");

    let system = assemble_system(model)?;
    let ndof = system.ndof;
    let n_free = system.n_free();

    let mut neg_kg_coo = Coo::new(n_free);
    let mut diag_estimate = vec![0.0f64; n_free];
    for element in &model.elements {
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let Some(indices_old) = element_global_indices(&system.dof_map, &node_ids, dofs) else { continue };
        let ctx = ElementContext { positions: positions_of(&model.nodes, &node_ids) };

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
                    if compact_row == compact_col {
                        diag_estimate[compact_row] += v.abs();
                    }
                }
            }
        }
    }

    // 🩹️ Frame/truss `geometric_stiffness` (bar/beam bending block, truss `N/L·(I−ccᵀ)` transverse
    // projector) still leaves SOME directions exactly unstressed (bending elements' own axial DOF,
    // `PlateDkt`'s entire DOF set — see its struct doc — and any drilling/rotational DOF no element's
    // Kg touches), so the assembled `−Kg` can still be singular or near-singular along those
    // directions even now that continuum/solid/shell elements contribute a full Kg of their own.
    // `subspace_iteration`'s B-orthonormalization divides by `sqrt(x·Bx)`, which blows up (→ NaN) for
    // any seed vector with a nonzero component in an exact null space. A tiny diagonal regularization
    // (Tikhonov-style, scaled off the assembled `−Kg`'s own diagonal magnitude) makes `−Kg` strictly
    // positive-definite everywhere without perturbing the physically meaningful lowest eigenvalues,
    // which are orders of magnitude below the huge spurious eigenvalues this regularization assigns
    // to the null-space directions.
    let max_diag = diag_estimate.iter().cloned().fold(0.0_f64, f64::max);
    let eps = max_diag.max(1e-12) * 1e-6;
    for i in 0..n_free {
        neg_kg_coo.add(i, i, eps);
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
mod tests {
    use super::*;
    use crate::elements2d::{Bar2, BeamEb2};
    use crate::model::{solve_linear_static, AxialSpring, Model};

    fn cantilever_analysis_model(e: f64, area: f64, iy: f64, l: f64, density: f64) -> (AnalysisModel, Vec<LoadCase>) {
        let model = AnalysisModel {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
            elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density }.into()],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
        };
        let cases = vec![LoadCase { id: "tip_load".into(), nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: -1000.0 }], member_loads: vec![], self_weight: false }];
        (model, cases)
    }

    fn axial_chain(element_count: usize) -> AnalysisModel {
        AnalysisModel {
            nodes: (0..=element_count).map(|index| Node { id: format!("n{index}"), pos: [index as f64, 0.0, 0.0] }).collect(),
            elements: (0..element_count).map(|index| AxialSpring { id: format!("e{index}"), a: format!("n{index}"), b: format!("n{}", index + 1), k: 10.0 + index as f64 }.into()).collect(),
            supports: vec![Support { node_id: "n0".to_string(), fixed: vec![Dof::Tx] }],
        }
    }

    fn assembly_operation(id: u64) -> Operation {
        Operation::new(semio_framework_job::OperationId(id), semio_framework_job::RevisionId(7), semio_framework_job::Generation(3), 11)
    }

    #[test]
    fn mounted_assembly_construction_is_retained_and_preserves_the_exact_model_owner() {
        let operation = assembly_operation(83);
        let model = Arc::new(cantilever_analysis_model(210e9, 0.02, 8e-6, 3.0, 7_850.0).0);
        let pointer = Arc::as_ptr(&model);
        let mut construction = AssemblyJobConstruction::new_owned(model, operation, 1);
        assert!(!construction.step_one().expect("first reservation opportunity"));
        let mut opportunities = 1;
        while !construction.step_one().expect("retained assembly construction") {
            opportunities += 1;
            assert!(opportunities < 4_096, "fixed construction reaches a finite terminal witness");
        }
        assert!(opportunities > 16, "validation, references, DOFs and partition outputs cannot collapse into one constructor turn");
        let job = construction.take_complete().expect("terminal construction returns one exact job");
        match &job.model {
            AnalysisModelOwner::Owned(owner) => assert_eq!(Arc::as_ptr(owner), pointer),
            AnalysisModelOwner::Borrowed(_) => panic!("mounted construction must preserve the owned model authority"),
        }
        assert!(construction.take_complete().is_none(), "completion transfers exactly once");
    }

    #[test]
    fn mounted_element_build_reserves_and_reclaims_one_exact_owner_per_turn() {
        let operation = assembly_operation(89);
        let model = Arc::new(cantilever_analysis_model(210e9, 0.02, 8e-6, 3.0, 7_850.0).0);
        let mut construction = AssemblyJobConstruction::new_owned(model, operation, 1);
        while !construction.step_one().expect("mounted construction") {}
        let mut job = construction.take_complete().expect("mounted assembly job");
        let mut sequence = 0;
        fn step_once(job: &mut AssemblyJob<'static>, operation: Operation, sequence: &mut u64) -> StepOutcome {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, sequence);
            job.step(&mut context)
        }
        assert!(matches!(step_once(&mut job, operation, &mut sequence), StepOutcome::Yield));
        let build = job.state.pending_build.as_ref().expect("first turn retains only the build shell");
        assert_eq!((build.indices_new.capacity(), build.positions.capacity(), build.stiffness.capacity()), (0, 0, 0));
        assert!(matches!(step_once(&mut job, operation, &mut sequence), StepOutcome::Yield));
        let build = job.state.pending_build.as_ref().expect("second turn retains the fixed index page");
        assert!(build.indices_new.capacity() != 0);
        assert_eq!((build.positions.capacity(), build.stiffness.capacity()), (0, 0));
        let mut saw_positions = false;
        let mut saw_stiffness = false;
        let mut saw_reclaim = [false; 3];
        for _ in 0..128 {
            assert!(matches!(step_once(&mut job, operation, &mut sequence), StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_)));
            if let Some(build) = job.state.pending_build.as_ref() {
                saw_positions |= build.positions.capacity() != 0;
                saw_stiffness |= build.stiffness.capacity() != 0;
            }
            if let Some(pending) = job.state.pending.as_ref() {
                saw_reclaim[0] |= pending.complete && pending.reclaim_lane >= 1 && pending.stiffness.capacity() == 0;
                saw_reclaim[1] |= pending.complete && pending.reclaim_lane >= 2 && pending.indices_new.capacity() == 0;
                saw_reclaim[2] |= pending.complete && pending.reclaim_lane >= 3 && pending.positions.capacity() == 0;
            }
            if saw_reclaim.into_iter().all(|seen| seen) {
                break;
            }
        }
        assert!(saw_positions && saw_stiffness, "positions and stiffness are retained in distinct admitted stages");
        assert!(saw_reclaim.into_iter().all(|seen| seen), "each element backing is returned in its own later worker opportunity");
    }

    #[test]
    fn mounted_element_stiffness_observes_before_admit_and_retires_rejected_backing() {
        let source = include_str!("component.rs");
        let reserve = source.find("PendingElementBuildStage::ReserveStiffnessCredit =>").expect("reserve stiffness credit");
        let allocate = source.find("PendingElementBuildStage::AllocateStiffness =>").expect("allocate stiffness quarantine");
        let observe = source.find("PendingElementBuildStage::ObserveStiffnessBacking =>").expect("observe actual stiffness backing");
        let admit = source.find("PendingElementBuildStage::AdmitStiffnessBacking =>").expect("admit observed stiffness backing");
        assert!(reserve < allocate && allocate < observe && observe < admit);

        let mut rejected = Vec::<f64>::new();
        rejected.try_reserve_exact(MOUNTED_OWNER_PAGE_BYTES / std::mem::size_of::<f64>() + 1).expect("hostile rejected backing");
        let observed = rejected.capacity() * std::mem::size_of::<f64>();
        assert!(observed > MOUNTED_OWNER_PAGE_BYTES);
        assert_eq!(close_vec_owner_step(&mut rejected, observed), Ok(Some((1, observed))), "the exact rejected allocation retires through the retained close helper");
        assert_eq!(rejected.capacity(), 0);
    }

    fn finish_assembly_job<'model>(mut job: AssemblyJob<'model>, operation: Operation, fuel: u64) -> (UnfactoredSystem, Vec<AssemblyPreview>, u128) {
        let mut sequence = 0;
        let mut previews = Vec::new();
        let mut max_step_micros = 0;
        loop {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(fuel, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            let started = std::time::Instant::now();
            let outcome = job.step(&mut context);
            max_step_micros = max_step_micros.max(started.elapsed().as_micros());
            match outcome {
                StepOutcome::PreviewReady(bytes) => previews.push(serde_json::from_slice(&bytes).expect("assembly preview decodes")),
                StepOutcome::Complete(_) => break,
                StepOutcome::Yield | StepOutcome::CheckpointReady(_) => {}
                StepOutcome::Cancelled | StepOutcome::Fault(_) => panic!("assembly fixture must complete"),
            }
        }
        (job.finish().expect("completed assembly yields matrices"), previews, max_step_micros)
    }

    /// 🧮️ Worker-local partition counts cannot alter triplet reduction order or matrix bytes.
    #[test]
    fn assembly_job_is_exact_across_partition_counts() {
        let model = axial_chain(24);
        let operation = assembly_operation(301);
        let (single, single_previews, _) = finish_assembly_job(AssemblyJob::new(&model, operation, 1).expect("single partition prepares"), operation, 5);
        let (fleet, fleet_previews, _) = finish_assembly_job(AssemblyJob::new(&model, operation, 7).expect("fleet partitions prepare"), operation, 3);
        assert_eq!(single.k_full_coo.to_dense().data, fleet.k_full_coo.to_dense().data);
        assert_eq!(single.k_ff_coo.to_dense().data, fleet.k_ff_coo.to_dense().data);
        assert_eq!(single_previews.last().expect("single publishes marks").assembled_element_ids.len(), model.elements.len());
        assert_eq!(fleet_previews.last().expect("fleet publishes marks").assembled_element_ids.len(), model.elements.len());
    }

    /// 💾️ A serialized element-boundary checkpoint resumes to the exact same merged matrices.
    #[test]
    fn assembly_job_checkpoint_resume_is_byte_stable() {
        let model = axial_chain(20);
        let operation = assembly_operation(302);
        let mut job = AssemblyJob::new(&model, operation, 4).expect("assembly prepares");
        let mut sequence = 0;
        let checkpoint = loop {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(4, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            if let StepOutcome::CheckpointReady(checkpoint) = job.step(&mut context) {
                break checkpoint.state;
            }
        };
        let resumed = AssemblyJob::from_checkpoint(&model, operation, &checkpoint).expect("assembly checkpoint restores");
        assert_eq!(resumed.checkpoint_bytes(), checkpoint);
        let (original, _, _) = finish_assembly_job(job, operation, 9);
        let (restored, _, _) = finish_assembly_job(resumed, operation, 2);
        assert_eq!(original.k_full_coo.to_dense().data, restored.k_full_coo.to_dense().data);
        assert_eq!(original.k_ff_coo.to_dense().data, restored.k_ff_coo.to_dense().data);
    }

    /// ⏱️ One-fuel adversarial stepping keeps every callback below the global eight-millisecond ceiling.
    #[test]
    fn assembly_job_one_fuel_steps_stay_below_eight_milliseconds() {
        let model = axial_chain(512);
        let operation = assembly_operation(303);
        let (_, previews, max_step_micros) = finish_assembly_job(AssemblyJob::new(&model, operation, 8).expect("assembly prepares"), operation, 1);
        assert!(!previews.is_empty());
        assert!(max_step_micros < 8_000, "slowest assembly step was {max_step_micros} us");
    }

    /// 🚫️ Stale and cancelled contexts leave the persistent assembly cursor untouched.
    #[test]
    fn p6h_element_stiffness_microcursor_deadline_stale_cancel_close_and_stage_laws() {
        let source = include_str!("component.rs");
        let mut stage_offset = 0;
        for stage in
            ["ReferenceQuadraturePoint", "ShapeFunctionDerivativeScalar", "JacobianCell", "DeterminantInverseCell", "StrainDisplacementCell", "ConstitutiveCell", "LocalStiffnessMultiplyCell", "BodyTractionLoadCell", "LocalToGlobalTripletCell"]
        {
            let offset = source[stage_offset..].find(stage).expect("P6h stiffness stage") + stage_offset;
            assert!(offset >= stage_offset);
            stage_offset = offset + stage.len();
        }
        let model = axial_chain(3);
        let operation = assembly_operation(304);
        let mut job = AssemblyJob::new(&model, operation, 2).expect("assembly prepares");
        let before = job.checkpoint_bytes();
        let mut sequence = 0;
        let mut deadline = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, 0), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
        assert_eq!(job.step(&mut deadline), StepOutcome::Yield);
        assert_eq!(job.checkpoint_bytes(), before);

        let mut stale = StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
        assert!(matches!(job.step(&mut stale), StepOutcome::Fault(_)));
        assert_eq!(job.checkpoint_bytes(), before);

        let token = semio_framework_job::root_cancel_token();
        semio_framework_async::block_on(token.cancel());
        let mut cancelled = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), token, || 0, &mut sequence);
        assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
        assert_eq!(job.checkpoint_bytes(), before);

        for _ in 0..24 {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            assert!(matches!(job.step(&mut context), StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_)));
        }
        let mut close_turns = 0;
        loop {
            close_turns += 1;
            let (terminal, released_items, _) = job.close_step(usize::MAX);
            assert!(released_items <= 1);
            if terminal {
                break;
            }
            assert!(close_turns < 20_000);
        }
        assert!(job.close_lane > 11);
    }

    /// 🧮️ Cross-validates `solve_multi_case`'s sparse RCM-ordered pipeline (single case) against
    /// `solve_linear_static`'s already-correct dense pipeline on an equivalent model — same oracle
    /// strategy already used elsewhere in this crate.
    #[test]
    fn solve_multi_case_matches_single_case_dense_solve() {
        let (e, area, iy, l) = (200e9, 0.01, 1e-5, 2.0);
        let (model, cases) = cantilever_analysis_model(e, area, iy, l, 0.0);
        let results = solve_multi_case(&model, &cases, &[], [0.0, 0.0, 0.0]).expect("solves");
        let sparse_result = results.get("tip_load").expect("case present");

        let dense_model = Model {
            nodes: model.nodes.clone(),
            elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 }.into()],
            supports: model.supports.clone(),
            nodal_loads: cases[0].nodal_loads.clone(),
            member_loads: vec![],
        };
        let dense_result = solve_linear_static(&dense_model).expect("dense solves");

        for sd in &sparse_result.displacements {
            let dd = dense_result.displacements.iter().find(|d| d.node_id == sd.node_id).unwrap();
            for k in 0..6 {
                assert!((sd.values[k] - dd.values[k]).abs() < 1e-8, "displacement mismatch at {} dof {k}: {} vs {}", sd.node_id, sd.values[k], dd.values[k]);
            }
        }
        for sr in &sparse_result.reactions {
            let dr = dense_result.reactions.iter().find(|r| r.node_id == sr.node_id && r.dof == sr.dof).unwrap();
            assert!((sr.value - dr.value).abs() < 1e-8, "reaction mismatch at {} {:?}: {} vs {}", sr.node_id, sr.dof, sr.value, dr.value);
        }
    }

    /// ➕️ A `Combination` must equal hand-computed superposition of the individually-solved case results.
    #[test]
    fn combination_equals_manual_superposition() {
        let (e, area, iy, l) = (200e9, 0.01, 1e-5, 2.0);
        let model = AnalysisModel {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
            elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 }.into()],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
        };
        let case_a = LoadCase { id: "a_case".into(), nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Ty, value: -1000.0 }], member_loads: vec![], self_weight: false };
        let case_b = LoadCase { id: "b_case".into(), nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Rz, value: 500.0 }], member_loads: vec![], self_weight: false };
        let combo = Combination { id: "combo".into(), terms: vec![("a_case".into(), 1.35), ("b_case".into(), 1.5)] };

        let results = solve_multi_case(&model, &[case_a, case_b], &[combo], [0.0, 0.0, 0.0]).expect("solves");
        let ra = results.get("a_case").unwrap().clone();
        let rb = results.get("b_case").unwrap().clone();
        let combined = results.get("combo").unwrap();

        for cd in &combined.displacements {
            let ad = ra.displacements.iter().find(|d| d.node_id == cd.node_id).unwrap();
            let bd = rb.displacements.iter().find(|d| d.node_id == cd.node_id).unwrap();
            for k in 0..6 {
                let expected = 1.35 * ad.values[k] + 1.5 * bd.values[k];
                assert!((cd.values[k] - expected).abs() < 1e-8, "combo displacement mismatch at {} dof {k}", cd.node_id);
            }
        }
        for cr in &combined.reactions {
            let ar = ra.reactions.iter().find(|r| r.node_id == cr.node_id && r.dof == cr.dof).unwrap();
            let br = rb.reactions.iter().find(|r| r.node_id == cr.node_id && r.dof == cr.dof).unwrap();
            let expected = 1.35 * ar.value + 1.5 * br.value;
            assert!((cr.value - expected).abs() < 1e-8, "combo reaction mismatch at {} {:?}", cr.node_id, cr.dof);
        }
    }

    /// ⚖️ Self-weight-only equilibrium: the sum of vertical reactions must equal `ρAL * g` — a
    /// strong, simple physical check independent of the moment distribution.
    #[test]
    fn self_weight_matches_total_mass_times_gravity() {
        let (e, area, iy, l, density) = (30e9, 0.05, 1e-4, 6.0, 2400.0);
        let model = AnalysisModel {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
            elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density }.into()],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty] }, Support { node_id: "b".into(), fixed: vec![Dof::Ty] }],
        };
        let case = LoadCase { id: "self_weight".into(), nodal_loads: vec![], member_loads: vec![], self_weight: true };
        let results = solve_multi_case(&model, &[case], &[], [0.0, -9.81, 0.0]).expect("solves");
        let result = results.get("self_weight").unwrap();

        let total_ty_reaction: f64 = result.reactions.iter().filter(|r| r.dof == Dof::Ty).map(|r| r.value).sum();
        let expected = density * area * l * 9.81;
        // Reactions balance the applied (downward, negative) self-weight load, so they sum positive.
        assert!((total_ty_reaction - expected).abs() / expected < 0.01, "reaction sum {total_ty_reaction} vs expected {expected}");
    }

    /// 🎯️ Cantilever modal frequencies vs the classical closed form `f_i = (β_iL)²/(2πL²) · sqrt(EI/ρA)`.
    #[test]
    fn modal_cantilever_matches_analytical_frequencies() {
        let (e, iy, area, density, total_l) = (200e9, 1e-5, 0.01, 7850.0, 3.0);
        let n = 9;
        let dl = total_l / n as f64;
        let nodes: Vec<Node> = (0..=n).map(|i| Node { id: format!("n{i}"), pos: [dl * i as f64, 0.0, 0.0] }).collect();
        let elements: Vec<Elements> = (0..n).map(|i| BeamEb2 { id: format!("e{i}"), start: format!("n{i}"), end: format!("n{}", i + 1), e, area, iy, density }.into()).collect();
        let model = AnalysisModel { nodes, elements, supports: vec![Support { node_id: "n0".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }] };

        let result = modal(&model, 3).expect("modal solves");
        let beta_l = [1.875104_f64, 4.694091, 7.854757];
        for i in 0..3 {
            let expected = (beta_l[i] * beta_l[i]) / (2.0 * std::f64::consts::PI * total_l * total_l) * (e * iy / (density * area)).sqrt();
            let actual = result.frequencies_hz[i];
            assert!((actual - expected).abs() / expected < 0.10, "mode {i}: {actual} Hz vs analytical {expected} Hz");
        }
    }

    /// 🌀️ Euler pinned-pinned column buckling load vs `π²EI/L²` (K=1.0).
    #[test]
    fn buckling_euler_column_matches_analytical_load() {
        let (e, iy, area, density, total_l) = (200e9, 8e-6, 0.005, 7850.0, 3.0);
        let n = 7;
        let dl = total_l / n as f64;
        let nodes: Vec<Node> = (0..=n).map(|i| Node { id: format!("n{i}"), pos: [dl * i as f64, 0.0, 0.0] }).collect();
        let elements: Vec<Elements> = (0..n).map(|i| BeamEb2 { id: format!("e{i}"), start: format!("n{i}"), end: format!("n{}", i + 1), e, area, iy, density }.into()).collect();
        let supports = vec![Support { node_id: "n0".into(), fixed: vec![Dof::Tx, Dof::Ty] }, Support { node_id: format!("n{n}"), fixed: vec![Dof::Ty] }];
        let model = AnalysisModel { nodes, elements, supports };

        let p_ref = 1.0;
        let reference_case = LoadCase { id: "axial_compression".into(), nodal_loads: vec![NodalLoad { node_id: format!("n{n}"), dof: Dof::Tx, value: -p_ref }], member_loads: vec![], self_weight: false };

        // Sanity-check the reference static solve first: pure axial compression should give nonzero Tx
        // displacement at the loaded end and ~zero Ty/Rz everywhere (no bending under a concentric load).
        let static_results = solve_multi_case(&model, std::slice::from_ref(&reference_case), &[], [0.0, 0.0, 0.0]).expect("reference solves");
        let static_result = static_results.get("axial_compression").unwrap();
        for d in &static_result.displacements {
            assert!(d.values[Dof::Ty.index()].abs() < 1e-9, "unexpected transverse displacement at {}: {}", d.node_id, d.values[Dof::Ty.index()]);
        }

        let result = buckling(&model, &reference_case, 1).expect("buckling solves");
        let factor = result.factors[0];
        let critical_load = factor * p_ref;
        let expected = std::f64::consts::PI.powi(2) * e * iy / (total_l * total_l);
        assert!(critical_load > 0.0, "critical load should be positive, got {critical_load}");
        assert!((critical_load - expected).abs() / expected < 0.10, "critical load {critical_load} vs analytical {expected}");
    }

    /// 🔍️ Duplicate-node-id models are rejected the same way `lib.rs::validate` rejects them.
    #[test]
    fn duplicate_node_id_is_rejected() {
        let model = AnalysisModel { nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "a".into(), pos: [1.0, 0.0, 0.0] }], elements: vec![], supports: vec![] };
        let err = solve_multi_case(&model, &[], &[], [0.0, 0.0, 0.0]).unwrap_err();
        assert_eq!(err, FemError::DuplicateNodeId("a".into()));
    }

    /// 🔍️ A `Bar2` model works fine through the multi-case pipeline too (not just `BeamEb2`).
    #[test]
    fn solve_multi_case_supports_bar2_truss() {
        let (e, area, l, p) = (200e9, 0.001, 2.0, 5000.0);
        let model = AnalysisModel {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
            elements: vec![Bar2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, density: 0.0 }.into()],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty] }, Support { node_id: "b".into(), fixed: vec![Dof::Ty] }],
        };
        let case = LoadCase { id: "axial".into(), nodal_loads: vec![NodalLoad { node_id: "b".into(), dof: Dof::Tx, value: p }], member_loads: vec![], self_weight: false };
        let results = solve_multi_case(&model, &[case], &[], [0.0, 0.0, 0.0]).expect("solves");
        let result = results.get("axial").unwrap();
        let expected = p * l / (e * area);
        let b = result.displacements.iter().find(|d| d.node_id == "b").unwrap();
        assert!((b.values[Dof::Tx.index()] - expected).abs() / expected < 1e-8);
    }

    /// 🎨️ Patch test for `nodal_averaged_scalar`: TWO `Tri3Cst` triangles splitting a square along its
    /// diagonal, both under the SAME uniform uniaxial strain field (`u=a*x`, `v=-nu*a*y`) — every
    /// node's averaged von Mises must equal the exact analytical `E*a` (a constant field averages to
    /// itself regardless of how many elements touch a node).
    #[test]
    fn nodal_averaged_scalar_patch_test_is_exact_under_uniform_stress() {
        use crate::elements2d::{PlaneKind, Tri3Cst};
        let (e, nu, t) = (1000.0, 0.25, 1.0);
        let coords = [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]];
        let nodes: Vec<Node> = (0..4).map(|i| Node { id: format!("n{i}"), pos: [coords[i][0], coords[i][1], 0.0] }).collect();
        let el1 = Tri3Cst { id: "t1".into(), nodes: ["n0".into(), "n1".into(), "n2".into()], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 };
        let el2 = Tri3Cst { id: "t2".into(), nodes: ["n0".into(), "n2".into(), "n3".into()], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 };

        let a = 0.01;
        let u_of = |ids: [usize; 3]| VecD::from_vec(ids.iter().flat_map(|&i| [a * coords[i][0], -nu * a * coords[i][1]]).collect());
        let ctx_of = |ids: [usize; 3]| ElementContext { positions: ids.iter().map(|&i| [coords[i][0], coords[i][1], 0.0]).collect() };

        let r1 = el1.recover(&ctx_of([0, 1, 2]), &u_of([0, 1, 2]), None);
        let r2 = el2.recover(&ctx_of([0, 2, 3]), &u_of([0, 2, 3]), None);

        let model = AnalysisModel { nodes, elements: vec![el1.into(), el2.into()], supports: vec![] };
        let result = StaticResult { displacements: vec![], reactions: vec![], elements: vec![("t1".into(), r1), ("t2".into(), r2)], checks: SolutionChecks { residual_norm: 0.0, reaction_sum: [0.0; 6] } };

        let averaged = nodal_averaged_scalar(&model, &result, StressScalar::VonMises);
        let expected_vm = (e * a).abs();
        for id in ["n0", "n1", "n2", "n3"] {
            let v = *averaged.get(id).unwrap_or_else(|| panic!("node {id} missing from averaged map"));
            assert!((v - expected_vm).abs() / expected_vm < 1e-8, "node {id}: {v} vs {expected_vm}");
        }
    }

    /// 🎨️ `nodal_averaged_scalar` on two elements sharing exactly one node but reporting DIFFERENT
    /// constant von Mises values: the shared node's averaged value must land strictly between the
    /// two elements' own values, while each element's exclusive nodes keep that element's exact value.
    #[test]
    fn nodal_averaged_scalar_shared_node_is_between_neighboring_element_values() {
        use crate::elements2d::{PlaneKind, Tri3Cst};
        let (e, nu, t) = (1000.0, 0.25, 1.0);
        let el_a = Tri3Cst { id: "a".into(), nodes: ["shared".into(), "a1".into(), "a2".into()], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 };
        let el_b = Tri3Cst { id: "b".into(), nodes: ["shared".into(), "b1".into(), "b2".into()], e, nu, thickness: t, kind: PlaneKind::Stress, density: 0.0 };

        let ctx_a = ElementContext { positions: vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]] };
        let ctx_b = ElementContext { positions: vec![[0.0, 0.0, 0.0], [-2.0, 0.0, 0.0], [0.0, -2.0, 0.0]] };
        // `u = k*x` uniaxial fields with distinct magnitudes `k_a=0.02`, `k_b=0.05`, both zero at the
        // shared origin node so they stay purely constant-strain (patch-test-exact) on each triangle.
        let u_a = VecD::from_vec(vec![0.0, 0.0, 0.04, 0.0, 0.0, 0.0]);
        let u_b = VecD::from_vec(vec![0.0, 0.0, -0.1, 0.0, 0.0, 0.0]);

        let r_a = el_a.recover(&ctx_a, &u_a, None);
        let r_b = el_b.recover(&ctx_b, &u_b, None);
        let (va, vb) = match (&r_a, &r_b) {
            (ElementResult::Plane { gauss: ga }, ElementResult::Plane { gauss: gb }) => (ga[0].von_mises, gb[0].von_mises),
            _ => panic!("expected plane results"),
        };
        assert!(va < vb, "test setup should give distinct, ordered element values, got {va} vs {vb}");

        let nodes = vec![
            Node { id: "shared".into(), pos: [0.0, 0.0, 0.0] },
            Node { id: "a1".into(), pos: [2.0, 0.0, 0.0] },
            Node { id: "a2".into(), pos: [0.0, 2.0, 0.0] },
            Node { id: "b1".into(), pos: [-2.0, 0.0, 0.0] },
            Node { id: "b2".into(), pos: [0.0, -2.0, 0.0] },
        ];
        let model = AnalysisModel { nodes, elements: vec![el_a.into(), el_b.into()], supports: vec![] };
        let result = StaticResult { displacements: vec![], reactions: vec![], elements: vec![("a".into(), r_a), ("b".into(), r_b)], checks: SolutionChecks { residual_norm: 0.0, reaction_sum: [0.0; 6] } };

        let averaged = nodal_averaged_scalar(&model, &result, StressScalar::VonMises);
        let shared = *averaged.get("shared").unwrap();
        assert!(shared > va && shared < vb, "shared node value {shared} should be strictly between {va} and {vb}");
        assert!((*averaged.get("a1").unwrap() - va).abs() < 1e-9);
        assert!((*averaged.get("a2").unwrap() - va).abs() < 1e-9);
        assert!((*averaged.get("b1").unwrap() - vb).abs() < 1e-9);
        assert!((*averaged.get("b2").unwrap() - vb).abs() < 1e-9);
    }

    /// 🔍️ An empty `AnalysisModel` is rejected the same way `Model`'s top-level `validate` rejects it.
    #[test]
    fn empty_model_is_rejected() {
        let model = AnalysisModel { nodes: vec![], elements: vec![], supports: vec![] };
        let err = solve_multi_case(&model, &[], &[], [0.0, 0.0, 0.0]).unwrap_err();
        assert_eq!(err, FemError::EmptyModel);
    }

    /// 🔍️ An element referencing a node id absent from `model.nodes` is rejected.
    #[test]
    fn dangling_element_node_ref_is_rejected() {
        let model = AnalysisModel { nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }], elements: vec![Bar2 { id: "e1".into(), start: "a".into(), end: "missing".into(), e: 1.0, area: 1.0, density: 0.0 }.into()], supports: vec![] };
        let err = solve_multi_case(&model, &[], &[], [0.0, 0.0, 0.0]).unwrap_err();
        assert_eq!(err, FemError::DanglingNodeRef("missing".into()));
    }

    /// 🔍️ A support referencing a node id absent from `model.nodes` is rejected.
    #[test]
    fn dangling_support_node_ref_is_rejected() {
        let model = AnalysisModel { nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }], elements: vec![], supports: vec![Support { node_id: "missing".into(), fixed: vec![Dof::Tx] }] };
        let err = solve_multi_case(&model, &[], &[], [0.0, 0.0, 0.0]).unwrap_err();
        assert_eq!(err, FemError::DanglingNodeRef("missing".into()));
    }

    /// 🔍️ A `LoadCase` nodal load referencing a node id absent from `model.nodes` is rejected —
    /// `validate_case`'s own check, distinct from `validate`'s model-wide checks above.
    #[test]
    fn dangling_load_case_node_ref_is_rejected() {
        let model = AnalysisModel { nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }], elements: vec![], supports: vec![] };
        let case = LoadCase { id: "bad".into(), nodal_loads: vec![NodalLoad { node_id: "missing".into(), dof: Dof::Tx, value: 1.0 }], member_loads: vec![], self_weight: false };
        let err = solve_multi_case(&model, &[case], &[], [0.0, 0.0, 0.0]).unwrap_err();
        assert_eq!(err, FemError::DanglingNodeRef("missing".into()));
    }

    /// 🌬️ `solve_multi_case`'s member-UDL branch (`case_rhs_old`'s `equivalent_nodal_loads` path) must
    /// match `solve_linear_static`'s dense pipeline (`model.member_loads`) on an equivalent model.
    #[test]
    fn solve_multi_case_applies_member_udl_equivalent_loads() {
        let (e, area, iy, l, w) = (200e9, 0.01, 1e-5, 2.0, 500.0);
        let model = AnalysisModel {
            nodes: vec![Node { id: "a".into(), pos: [0.0, 0.0, 0.0] }, Node { id: "b".into(), pos: [l, 0.0, 0.0] }],
            elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 }.into()],
            supports: vec![Support { node_id: "a".into(), fixed: vec![Dof::Tx, Dof::Ty, Dof::Rz] }],
        };
        let case = LoadCase { id: "udl".into(), nodal_loads: vec![], member_loads: vec![("e1".into(), MemberUdl { wx: 0.0, wy: -w, wz: 0.0 })], self_weight: false };
        let results = solve_multi_case(&model, &[case], &[], [0.0, 0.0, 0.0]).expect("solves");
        let sparse_result = results.get("udl").unwrap();

        let dense_model = Model {
            nodes: model.nodes.clone(),
            elements: vec![BeamEb2 { id: "e1".into(), start: "a".into(), end: "b".into(), e, area, iy, density: 0.0 }.into()],
            supports: model.supports.clone(),
            nodal_loads: vec![],
            member_loads: vec![("e1".into(), MemberUdl { wx: 0.0, wy: -w, wz: 0.0 })],
        };
        let dense_result = solve_linear_static(&dense_model).expect("dense solves");

        for sd in &sparse_result.displacements {
            let dd = dense_result.displacements.iter().find(|d| d.node_id == sd.node_id).unwrap();
            for k in 0..6 {
                assert!((sd.values[k] - dd.values[k]).abs() < 1e-8, "displacement mismatch at {} dof {k}", sd.node_id);
            }
        }
    }

    /// 🌱️ `zero_like` zero-initializes every non-`Beam` `ElementResult` variant (the `Beam` variant is
    /// covered by `combination_equals_manual_superposition` above), and `add_scaled_element_result` on
    /// a freshly-zeroed accumulator reduces to exactly `factor * term`, field-by-field, per variant.
    #[test]
    fn zero_like_and_add_scaled_element_result_handle_every_non_beam_variant() {
        let factor = 2.5;

        let bar = ElementResult::Bar { n: 4.0 };
        let zero_bar = zero_like(&bar);
        assert_eq!(zero_bar, ElementResult::Bar { n: 0.0 });
        match add_scaled_element_result(&zero_bar, &bar, factor) {
            ElementResult::Bar { n } => assert!((n - factor * 4.0).abs() < 1e-12),
            other => panic!("expected bar, got {other:?}"),
        }

        let plane = ElementResult::Plane { gauss: vec![PlaneStress { sxx: 1.0, syy: 2.0, sxy: 3.0, von_mises: 4.0 }] };
        let zero_plane = zero_like(&plane);
        match &zero_plane {
            ElementResult::Plane { gauss } => assert_eq!(gauss[0], PlaneStress { sxx: 0.0, syy: 0.0, sxy: 0.0, von_mises: 0.0 }),
            other => panic!("expected plane, got {other:?}"),
        }
        match add_scaled_element_result(&zero_plane, &plane, factor) {
            ElementResult::Plane { gauss } => {
                assert!((gauss[0].sxx - factor * 1.0).abs() < 1e-12);
                assert!((gauss[0].syy - factor * 2.0).abs() < 1e-12);
                assert!((gauss[0].sxy - factor * 3.0).abs() < 1e-12);
            }
            other => panic!("expected plane, got {other:?}"),
        }

        let plate = ElementResult::Plate { gauss: vec![PlateMoments { mx: 1.0, my: 2.0, mxy: 3.0 }] };
        let zero_plate = zero_like(&plate);
        match add_scaled_element_result(&zero_plate, &plate, factor) {
            ElementResult::Plate { gauss } => {
                assert!((gauss[0].mx - factor * 1.0).abs() < 1e-12);
                assert!((gauss[0].my - factor * 2.0).abs() < 1e-12);
                assert!((gauss[0].mxy - factor * 3.0).abs() < 1e-12);
            }
            other => panic!("expected plate, got {other:?}"),
        }

        let solid = ElementResult::Solid { gauss: vec![SolidStress { sxx: 1.0, syy: 2.0, szz: 3.0, sxy: 4.0, syz: 5.0, sxz: 6.0, von_mises: 7.0 }] };
        let zero_solid = zero_like(&solid);
        match add_scaled_element_result(&zero_solid, &solid, factor) {
            ElementResult::Solid { gauss } => {
                assert!((gauss[0].sxx - factor * 1.0).abs() < 1e-12);
                assert!((gauss[0].szz - factor * 3.0).abs() < 1e-12);
                assert!((gauss[0].syz - factor * 5.0).abs() < 1e-12);
            }
            other => panic!("expected solid, got {other:?}"),
        }

        let shell = ElementResult::Shell { gauss: vec![ShellState { nxx: 1.0, nyy: 2.0, nxy: 3.0, mxx: 4.0, myy: 5.0, mxy: 6.0, von_mises_top: 7.0, von_mises_bottom: 8.0 }] };
        let zero_shell = zero_like(&shell);
        match add_scaled_element_result(&zero_shell, &shell, factor) {
            ElementResult::Shell { gauss } => {
                assert!((gauss[0].nxx - factor * 1.0).abs() < 1e-12);
                assert!((gauss[0].mxy - factor * 6.0).abs() < 1e-12);
                assert!((gauss[0].von_mises_bottom - factor * 8.0).abs() < 1e-12);
            }
            other => panic!("expected shell, got {other:?}"),
        }
    }

    /// 📊️ `element_scalar_average` covers every element-kind/scalar combination it recognizes (`Some`)
    /// and every mismatched combination (`None`) — arms `nodal_averaged_scalar`'s own patch tests never
    /// happen to exercise (those only touch `Plane`/`VonMises`).
    #[test]
    fn element_scalar_average_covers_every_variant_and_scalar_combination() {
        let plane = ElementResult::Plane { gauss: vec![PlaneStress { sxx: 1.0, syy: 2.0, sxy: 3.0, von_mises: 4.0 }] };
        assert_eq!(element_scalar_average(&plane, StressScalar::VonMises), Some(4.0));
        assert_eq!(element_scalar_average(&plane, StressScalar::Sxx), Some(1.0));
        assert_eq!(element_scalar_average(&plane, StressScalar::Syy), Some(2.0));
        assert_eq!(element_scalar_average(&plane, StressScalar::Sxy), Some(3.0));
        assert_eq!(element_scalar_average(&plane, StressScalar::Szz), None);
        assert_eq!(element_scalar_average(&plane, StressScalar::VonMisesTop), None);

        let solid = ElementResult::Solid { gauss: vec![SolidStress { sxx: 1.0, syy: 2.0, szz: 3.0, sxy: 4.0, syz: 5.0, sxz: 6.0, von_mises: 7.0 }] };
        assert_eq!(element_scalar_average(&solid, StressScalar::VonMises), Some(7.0));
        assert_eq!(element_scalar_average(&solid, StressScalar::Sxx), Some(1.0));
        assert_eq!(element_scalar_average(&solid, StressScalar::Syy), Some(2.0));
        assert_eq!(element_scalar_average(&solid, StressScalar::Szz), Some(3.0));
        assert_eq!(element_scalar_average(&solid, StressScalar::Sxy), Some(4.0));
        assert_eq!(element_scalar_average(&solid, StressScalar::Syz), Some(5.0));
        assert_eq!(element_scalar_average(&solid, StressScalar::Sxz), Some(6.0));
        assert_eq!(element_scalar_average(&solid, StressScalar::VonMisesTop), None);

        let shell = ElementResult::Shell { gauss: vec![ShellState { nxx: 0.0, nyy: 0.0, nxy: 0.0, mxx: 0.0, myy: 0.0, mxy: 0.0, von_mises_top: 8.0, von_mises_bottom: 9.0 }] };
        assert_eq!(element_scalar_average(&shell, StressScalar::VonMisesTop), Some(8.0));
        assert_eq!(element_scalar_average(&shell, StressScalar::VonMisesBottom), Some(9.0));
        assert_eq!(element_scalar_average(&shell, StressScalar::VonMises), None);

        let bar = ElementResult::Bar { n: 42.0 };
        assert_eq!(element_scalar_average(&bar, StressScalar::VonMises), None);
    }

    fn graph_operation(id: u64) -> Operation {
        Operation::new(semio_framework_job::OperationId(id), semio_framework_job::RevisionId(4), semio_framework_job::Generation(2), 9)
    }

    fn graph_plan() -> Vec<FemStagePlan> {
        vec![
            FemStagePlan { stage: FemJobStage::ValidateReferences, units: 1 },
            FemStagePlan { stage: FemJobStage::BuildDofMap, units: 2 },
            FemStagePlan { stage: FemJobStage::OrderEquations, units: 3 },
            FemStagePlan { stage: FemJobStage::Assemble, units: 4 },
            FemStagePlan { stage: FemJobStage::Factor, units: 5 },
            FemStagePlan { stage: FemJobStage::Solve, units: 6 },
            FemStagePlan { stage: FemJobStage::Recover, units: 7 },
            FemStagePlan { stage: FemJobStage::Finalize, units: 8 },
        ]
    }

    #[test]
    fn fem_job_graph_checkpoint_resume_preserves_stage_order() {
        let operation = graph_operation(201);
        let mut graph = FemJobGraph::new(operation, graph_plan(), 2);
        let mut sequence = 0;
        let checkpoint = loop {
            let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(2, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            if let StepOutcome::CheckpointReady(checkpoint) = graph.step(&mut context) {
                break checkpoint.state;
            }
        };
        let mut resumed = FemJobGraph::from_checkpoint(operation, &checkpoint).expect("graph checkpoint restores");
        assert_eq!(resumed.checkpoint_bytes(), checkpoint);
        let mut seen = Vec::new();
        loop {
            if let Some(stage) = resumed.progress().stage {
                if seen.last() != Some(&stage) {
                    seen.push(stage);
                }
            }
            let mut context = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(3, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            if matches!(resumed.step(&mut context), StepOutcome::Complete(_)) {
                break;
            }
        }
        assert_eq!(resumed.progress().completed_units, 36);
        assert_eq!(seen, graph_plan().into_iter().skip(1).map(|plan| plan.stage).collect::<Vec<_>>());
    }

    #[test]
    fn fem_job_graph_rejects_stale_and_cancelled_steps_without_mutation() {
        let operation = graph_operation(202);
        let mut graph = FemJobGraph::new(operation, graph_plan(), 2);
        let before = graph.checkpoint_bytes();
        let mut sequence = 0;
        let mut stale =
            semio_framework_job::StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(2, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
        assert!(matches!(graph.step(&mut stale), StepOutcome::Fault(_)));
        assert_eq!(graph.checkpoint_bytes(), before);

        let token = semio_framework_job::root_cancel_token();
        semio_framework_async::block_on(token.cancel());
        let mut cancelled = semio_framework_job::StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(2, u64::MAX), token, || 0, &mut sequence);
        assert_eq!(graph.step(&mut cancelled), StepOutcome::Cancelled);
        assert_eq!(graph.checkpoint_bytes(), before);
    }
}
// #endregion 🔖️Tests
