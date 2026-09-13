//! 📈️ Multi-case/combination linear-static analysis, self-weight load generation, modal analysis
//! (frequencies/shapes), linear buckling, and nodal-averaged stress recovery for contour rendering
//! (`nodal_averaged_scalar`) — all sparse-backed (RCM-ordered, single LDLT factorization shared
//! across every load case / eigen-solve).

use crate::algebra::{MatD, VecD};
use crate::model::{BeamStation, Dof, Element, ElementContext, ElementResult, Elements, FemError, MemberUdl, NodalLoad, Node, NodeDisplacement, NodeReaction, PlaneStress, PlateMoments, ShellState, SolidStress, SolutionChecks, StaticResult, Support};
use crate::sparse::{ldlt_factor, rcm_order, subspace_iteration, Coo, Csr, EigenPairs, LdltFactor};
use replication::value::list::PagedList;
use semio_framework_job::{CommitCandidate, InteractiveJob, JobFault, JobPayloadStream, Operation, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

const MOUNTED_OWNER_PAGE_BYTES: usize = 4_096;
const ASSEMBLY_TRIPLET_INDEX_SPACE: usize = usize::MAX;

fn encode_value<T: dsl::ToValue>(value: &T) -> Vec<u8> {
    store::pack_rt::encode_wire_value(&value.to_value())
}

fn decode_value<T: dsl::FromValue>(bytes: &[u8]) -> Result<T, String> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| error.to_string())?;
    T::from_value(value).map_err(|error| error.to_string())
}

fn reserve_exact_owner_page<T>(owner: &mut Vec<T>, additional: usize) -> bool {
    owner.try_reserve_exact(additional).is_ok() && owner.capacity().checked_mul(size_of::<T>()).is_some_and(|bytes| bytes <= MOUNTED_OWNER_PAGE_BYTES)
}

fn close_vec_owner_step<T>(owner: &mut Vec<T>, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
    if owner.pop().is_some() {
        return Ok(Some((1, 0)));
    }
    let bytes = owner.capacity().checked_mul(size_of::<T>()).ok_or(())?;
    if bytes == 0 {
        return Ok(None);
    }
    if bytes > maximum_bytes {
        return Err(());
    }
    *owner = Vec::new();
    Ok(Some((1, bytes)))
}

fn close_paged_owner_step<T, const N: usize>(owner: &mut PagedList<T, N>, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
    if owner.pop().is_some() {
        return Ok(Some((1, 0)));
    }
    if owner.terminal_is_empty() {
        return Ok(None);
    }
    let progress = owner.release_empty_page(maximum_bytes).map_err(|_| ())?;
    progress.progressed.then_some(Some((1, progress.released_allocation_bytes))).ok_or(())
}

fn cold_paged_owner<T, const N: usize>(capacity: usize) -> Result<PagedList<T, N>, FemError> {
    let mut owner = PagedList::default();
    while owner.capacity() < capacity {
        if !owner.reserve_capacity_one(capacity, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::Singular)?.progressed {
            return Err(FemError::Singular);
        }
    }
    Ok(owner)
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

pub const MOUNTED_ANALYSIS_NODE_SLOTS: usize = 128;
pub const MOUNTED_ANALYSIS_ELEMENT_SLOTS: usize = 128;
pub const MOUNTED_ANALYSIS_SUPPORT_SLOTS: usize = 64;
pub const MOUNTED_ANALYSIS_BACKING_BYTES: usize = MOUNTED_OWNER_PAGE_BYTES;

pub struct MountedAnalysisSupport {
    node_id: String,
    fixed: [Option<Dof>; 6],
    fixed_len: usize,
}

impl MountedAnalysisSupport {
    pub fn new(node_id: String) -> Self {
        Self { node_id, fixed: [None; 6], fixed_len: 0 }
    }

    pub fn push_fixed(&mut self, dof: Dof) -> Result<(), Dof> {
        if self.fixed_len == self.fixed.len() {
            return Err(dof);
        }
        self.fixed[self.fixed_len] = Some(dof);
        self.fixed_len += 1;
        Ok(())
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if self.node_id.capacity() != 0 {
            let bytes = self.node_id.capacity();
            if bytes > maximum_bytes {
                return (false, 0, 0);
            }
            self.node_id = String::new();
            return (false, 1, bytes);
        }
        if self.fixed_len != 0 {
            self.fixed_len -= 1;
            self.fixed[self.fixed_len] = None;
            return (false, 1, 0);
        }
        (true, 0, 0)
    }
}

/// 🚧️ A mounted model retains allocation failures separately from logical admission limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MountedAnalysisFault {
    Capacity { requested: usize, maximum: usize },
    Allocation { allocated_bytes: usize, reason: &'static str },
    Closing,
}

/// 🎟️ One model admission opportunity reports only its actual newly retained backing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MountedAnalysisAdmission {
    pub complete: bool,
    pub allocated_bytes: usize,
}

struct MountedModelItems<T, const N: usize> {
    values: PagedList<T, N>,
    admitted: usize,
}

impl<T, const N: usize> Default for MountedModelItems<T, N> {
    fn default() -> Self {
        Self { values: PagedList::default(), admitted: 0 }
    }
}

impl<T, const N: usize> MountedModelItems<T, N> {
    fn next_allocation_bytes(&self, target: usize) -> Result<Option<usize>, MountedAnalysisFault> {
        if target > N { return Err(MountedAnalysisFault::Capacity { requested: target, maximum: N }); }
        self.values.next_capacity_allocation_bytes(target).map_err(|reason| MountedAnalysisFault::Allocation { allocated_bytes: 0, reason })
    }

    fn admit_one(&mut self, target: usize, maximum_bytes: usize) -> Result<MountedAnalysisAdmission, MountedAnalysisFault> {
        if self.next_allocation_bytes(target)?.is_some() {
            let progress = self.values.reserve_capacity_one(target, maximum_bytes.min(MOUNTED_OWNER_PAGE_BYTES)).map_err(|error| MountedAnalysisFault::Allocation { allocated_bytes: error.allocated_bytes, reason: error.reason })?;
            return Ok(MountedAnalysisAdmission { complete: false, allocated_bytes: progress.allocated_bytes });
        }
        if self.admitted < target {
            self.admitted += 1;
            return Ok(MountedAnalysisAdmission::default());
        }
        Ok(MountedAnalysisAdmission { complete: true, allocated_bytes: 0 })
    }

    fn push(&mut self, value: T) -> Result<(), T> {
        if self.values.len() >= self.admitted { return Err(value); }
        self.values.push_reserved(value)
    }
}

trait MountedAnalysisItem {
    fn close_item_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize);
}

impl MountedAnalysisItem for Node {
    fn close_item_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        let bytes = self.id.capacity();
        if bytes == 0 { return (true, 0, 0); }
        if bytes > maximum_bytes { return (false, 0, 0); }
        self.id = String::new();
        (false, 1, bytes)
    }
}

impl MountedAnalysisItem for Elements {
    fn close_item_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(bytes) = self.mounted_next_string_bytes() {
            if bytes > maximum_bytes { return (false, 0, 0); }
            return (false, 1, self.close_mounted_string_step().expect("admitted mounted element string"));
        }
        (self.mounted_strings_terminal_is_empty(), 0, 0)
    }
}

impl MountedAnalysisItem for MountedAnalysisSupport {
    fn close_item_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        self.close_step(maximum_bytes)
    }
}

impl<T: MountedAnalysisItem, const N: usize> MountedModelItems<T, N> {
    fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(value) = self.values.len().checked_sub(1).and_then(|index| self.values.get_mut(index)) {
            let progress = value.close_item_step(maximum_bytes);
            if !progress.0 { return progress; }
            self.values.pop();
            return (false, 1, 0);
        }
        if self.admitted != 0 {
            self.admitted -= 1;
            return (false, 1, 0);
        }
        match close_paged_owner_step(&mut self.values, maximum_bytes) {
            Ok(Some((items, bytes))) => (false, items, bytes),
            Ok(None) => (true, 0, 0),
            Err(()) => (false, 0, 0),
        }
    }
}

/// 🧱 Mounted model metadata moves directly while each payload page retains its own byte grant.
pub struct MountedAnalysisModel {
    nodes: MountedModelItems<Node, MOUNTED_ANALYSIS_NODE_SLOTS>,
    elements: MountedModelItems<Elements, MOUNTED_ANALYSIS_ELEMENT_SLOTS>,
    supports: MountedModelItems<MountedAnalysisSupport, MOUNTED_ANALYSIS_SUPPORT_SLOTS>,
    fault: Option<MountedAnalysisFault>,
    closing: bool,
    close_lane: u8,
}

impl MountedAnalysisModel {
    pub fn new() -> Self {
        Self { nodes: MountedModelItems::default(), elements: MountedModelItems::default(), supports: MountedModelItems::default(), fault: None, closing: false, close_lane: 0 }
    }

    fn admission_ready(&self) -> Result<(), MountedAnalysisFault> {
        if let Some(fault) = self.fault { return Err(fault); }
        if self.closing { return Err(MountedAnalysisFault::Closing); }
        Ok(())
    }

    fn retain_admission(&mut self, result: Result<MountedAnalysisAdmission, MountedAnalysisFault>) -> Result<MountedAnalysisAdmission, MountedAnalysisFault> {
        if let Err(fault) = result { self.fault = Some(fault); }
        result
    }

    pub fn next_node_allocation_bytes(&self, target: usize) -> Result<Option<usize>, MountedAnalysisFault> {
        self.admission_ready()?;
        self.nodes.next_allocation_bytes(target)
    }

    pub fn next_element_allocation_bytes(&self, target: usize) -> Result<Option<usize>, MountedAnalysisFault> {
        self.admission_ready()?;
        self.elements.next_allocation_bytes(target)
    }

    pub fn next_support_allocation_bytes(&self, target: usize) -> Result<Option<usize>, MountedAnalysisFault> {
        self.admission_ready()?;
        self.supports.next_allocation_bytes(target)
    }

    pub fn admit_node_one(&mut self, target: usize, maximum_bytes: usize) -> Result<MountedAnalysisAdmission, MountedAnalysisFault> {
        self.admission_ready()?;
        let result = self.nodes.admit_one(target, maximum_bytes);
        self.retain_admission(result)
    }

    pub fn admit_element_one(&mut self, target: usize, maximum_bytes: usize) -> Result<MountedAnalysisAdmission, MountedAnalysisFault> {
        self.admission_ready()?;
        let result = self.elements.admit_one(target, maximum_bytes);
        self.retain_admission(result)
    }

    pub fn admit_support_one(&mut self, target: usize, maximum_bytes: usize) -> Result<MountedAnalysisAdmission, MountedAnalysisFault> {
        self.admission_ready()?;
        let result = self.supports.admit_one(target, maximum_bytes);
        self.retain_admission(result)
    }

    pub fn push_node(&mut self, node: Node) -> Result<(), Node> {
        if self.admission_ready().is_err() { return Err(node); }
        self.nodes.push(node)
    }

    pub fn push_element(&mut self, element: Elements) -> Result<(), Elements> {
        if self.admission_ready().is_err() { return Err(element); }
        self.elements.push(element)
    }

    pub fn push_support(&mut self, support: MountedAnalysisSupport) -> Result<(), MountedAnalysisSupport> {
        if self.admission_ready().is_err() { return Err(support); }
        self.supports.push(support)
    }

    pub fn nodes_len(&self) -> usize { self.nodes.values.len() }
    pub fn elements_len(&self) -> usize { self.elements.values.len() }
    pub fn node(&self, index: usize) -> Option<&Node> { self.nodes.values.get(index) }
    pub fn element(&self, index: usize) -> Option<&Elements> { self.elements.values.get(index) }
    fn support(&self, index: usize) -> Option<&MountedAnalysisSupport> { self.supports.values.get(index) }

    pub fn physical_backing_bytes(&self) -> [usize; 3] {
        [self.nodes.values.allocated_bytes(), self.elements.values.allocated_bytes(), self.supports.values.allocated_bytes()]
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.nodes.values.terminal_is_empty() && self.elements.values.terminal_is_empty() && self.supports.values.terminal_is_empty() && self.nodes.admitted == 0 && self.elements.admitted == 0 && self.supports.admitted == 0
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        self.closing = true;
        loop {
            let progress = match self.close_lane {
                0 => self.nodes.close_step(maximum_bytes),
                1 => self.elements.close_step(maximum_bytes),
                2 => self.supports.close_step(maximum_bytes),
                _ => return (self.terminal_is_empty(), 0, 0),
            };
            if !progress.0 { return progress; }
            self.close_lane += 1;
        }
    }
}

impl Default for MountedAnalysisModel {
    fn default() -> Self {
        Self::new()
    }
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
                let bytes = model.nodes.capacity() * size_of::<Node>();
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
                let bytes = model.elements.capacity() * size_of::<Elements>();
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
                let bytes = support.fixed.capacity() * size_of::<Dof>();
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
                let bytes = model.supports.capacity() * size_of::<Support>();
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
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

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
pub struct FemStagePlan {
    pub stage: FemJobStage,
    pub units: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
pub struct FemJobProgress {
    pub stage: Option<FemJobStage>,
    pub completed_stages: usize,
    pub total_stages: usize,
    pub completed_units: u64,
    pub total_units: u64,
}

#[derive(Clone, ToValue, FromValue)]
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

    pub fn from_checkpoint(operation: Operation, bytes: &[u8]) -> Result<Self, String> {
        Ok(Self { operation, state: decode_value(bytes)? })
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
    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
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

impl InteractiveJob for FemJobGraph {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
        }
        if self.state.checkpoint_due {
            self.state.checkpoint_due = false;
            let bytes = self.checkpoint_bytes();
            return match context.payload_from_bytes(JobPayloadStream::CheckpointState, &bytes) {
                Ok(state) => StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state, applied_progress: self.state.completed_units }),
                Err(_) => StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
            };
        }
        if self.state.stage_cursor == self.state.plans.len() {
            let progress = self.progress();
            let bytes = encode_value(&progress);
            return match context.payload_from_bytes(JobPayloadStream::CommitOutput, &bytes) {
                Ok(output) => StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output }),
                Err(_) => StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
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
                return StepOutcome::Cancelled;
            }
        }
        if self.state.stage_cursor == self.state.plans.len() {
            let progress = self.progress();
            let bytes = encode_value(&progress);
            return match context.payload_from_bytes(JobPayloadStream::CommitOutput, &bytes) {
                Ok(output) => StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output }),
                Err(_) => StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
            };
        }
        let bytes = encode_value(&self.progress());
        match context.payload_from_bytes(JobPayloadStream::Preview, &bytes) {
            Ok(preview) => StepOutcome::PreviewReady(preview),
            Err(_) => StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
        }
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(tag = "kind")]
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
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
pub struct AssemblyPreview {
    pub stage: AssemblyJobStage,
    pub completed_elements: usize,
    pub total_elements: usize,
    pub full_triplets: usize,
    pub free_triplets: usize,
    pub assembled_element_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct AssemblyTriplet {
    sequence: u64,
    row: u32,
    col: u32,
    value: f64,
}

#[derive(Clone, Default)]
struct AssemblyPartitionBuffer {
    full: PagedList<AssemblyTriplet, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    free: PagedList<AssemblyTriplet, ASSEMBLY_TRIPLET_INDEX_SPACE>,
}

#[derive(Clone)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum PendingElementBuildStage {
    ReserveIndices,
    Indices,
    PublishIndex,
    ReservePositions,
    Positions,
    PublishPosition,
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
            Self::PublishIndex => "fem.element.publish-global-index",
            Self::ReservePositions => "fem.element.reserve-positions",
            Self::Positions => "fem.element.position",
            Self::PublishPosition => "fem.element.publish-position",
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

#[derive(Clone)]
struct PendingElementBuild {
    element_index: usize,
    node_count: usize,
    dof_count: usize,
    scalar_cursor: usize,
    lookup_cursor: usize,
    lookup_match: Option<usize>,
    stage: PendingElementBuildStage,
    indices_new: Vec<usize>,
    positions: Vec<[f64; 3]>,
    stiffness: Vec<f64>,
    stiffness_dimensions: [usize; 2],
    stiffness_observed_bytes: usize,
    stiffness_credit_reserved: bool,
    stiffness_admitted: bool,
}

#[derive(Clone)]
struct AssemblyCheckpoint {
    stage: AssemblyJobStage,
    total_elements: usize,
    element_cursor: usize,
    pending_build: Option<PendingElementBuild>,
    pending: Option<PendingElementAssembly>,
    partitions: Vec<AssemblyPartitionBuffer>,
    full_merge_cursors: Vec<usize>,
    free_merge_cursors: Vec<usize>,
    merged_full: PagedList<AssemblyTriplet, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    merged_free: PagedList<AssemblyTriplet, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    checkpoint_due: bool,
    preview_due: bool,
    resume_target: usize,
    merge_scan_partition: usize,
    merge_candidate: Option<(usize, AssemblyTriplet)>,
}

#[derive(ToValue, FromValue)]
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
    ReservePartitionTripletCounts,
    InitializePartitionTripletCounts,
    ReserveDofs,
    ValidateNodePairs,
    ValidateElementReferences,
    ValidateSupportReferences,
    DiscoverDofs,
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
    ReserveMergedFull,
    ReserveMergedFree,
    ReserveFullMergeCursors,
    ReserveFreeMergeCursors,
    BuildMergeCursors,
    Complete,
}

/// 🧵️ Retained mounted-session assembly-plan construction. Each call performs one reference
/// comparison, DOF insertion, scalar initialization or fixed-capacity allocation opportunity.
enum AssemblyConstructionModel {
    Dynamic(Arc<AnalysisModel>),
    Mounted(MountedAnalysisModel),
}

impl AssemblyConstructionModel {
    fn nodes_len(&self) -> usize {
        match self {
            Self::Dynamic(model) => model.nodes.len(),
            Self::Mounted(model) => model.nodes_len(),
        }
    }

    fn elements_len(&self) -> usize {
        match self {
            Self::Dynamic(model) => model.elements.len(),
            Self::Mounted(model) => model.elements_len(),
        }
    }

    fn supports_len(&self) -> usize {
        match self {
            Self::Dynamic(model) => model.supports.len(),
            Self::Mounted(model) => model.supports.values.len(),
        }
    }

    fn node(&self, index: usize) -> Option<&Node> {
        match self {
            Self::Dynamic(model) => model.nodes.get(index),
            Self::Mounted(model) => model.node(index),
        }
    }

    fn element(&self, index: usize) -> Option<&Elements> {
        match self {
            Self::Dynamic(model) => model.elements.get(index),
            Self::Mounted(model) => model.element(index),
        }
    }

    fn support_node_id(&self, index: usize) -> Option<&str> {
        match self {
            Self::Dynamic(model) => model.supports.get(index).map(|support| support.node_id.as_str()),
            Self::Mounted(model) => model.support(index).map(|support| support.node_id.as_str()),
        }
    }

    fn support_fixed_len(&self, index: usize) -> Option<usize> {
        match self {
            Self::Dynamic(model) => model.supports.get(index).map(|support| support.fixed.len()),
            Self::Mounted(model) => model.support(index).map(|support| support.fixed_len),
        }
    }

    fn support_fixed(&self, support: usize, index: usize) -> Option<Dof> {
        match self {
            Self::Dynamic(model) => model.supports.get(support)?.fixed.get(index).copied(),
            Self::Mounted(model) => model.support(support)?.fixed.get(index).copied().flatten(),
        }
    }
}

pub struct AssemblyJobConstruction {
    model: Option<AssemblyConstructionModel>,
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
    partition_triplet_counts: Vec<usize>,
    partition_reserve_cursor: usize,
    partition_reserve_lane: u8,
    plan: AssemblyPlan,
    constrained_old: Vec<bool>,
    partitions: Vec<AssemblyPartitionBuffer>,
    merged_full: PagedList<AssemblyTriplet, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    merged_free: PagedList<AssemblyTriplet, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    full_merge_cursors: Vec<usize>,
    free_merge_cursors: Vec<usize>,
    job: Option<AssemblyJob<'static>>,
}

impl AssemblyJobConstruction {
    pub fn new_owned(model: Arc<AnalysisModel>, operation: Operation, partition_count: usize) -> Self {
        Self::from_model(AssemblyConstructionModel::Dynamic(model), operation, partition_count)
    }

    fn from_model(model: AssemblyConstructionModel, operation: Operation, partition_count: usize) -> Self {
        Self {
            model: Some(model),
            model_close: AnalysisModelCloseCursor::default(),
            operation,
            partition_count,
            stage: AssemblyConstructionStage::ReservePartitionTripletCounts,
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
            partition_triplet_counts: Vec::new(),
            partition_reserve_cursor: 0,
            partition_reserve_lane: 0,
            plan: AssemblyPlan { dof_map: DofMap { order: Vec::new() }, inv_perm: Vec::new(), ndof: 0, free_new: Vec::new(), compact_of_new: Vec::new() },
            constrained_old: Vec::new(),
            partitions: Vec::new(),
            merged_full: PagedList::default(),
            merged_free: PagedList::default(),
            full_merge_cursors: Vec::new(),
            free_merge_cursors: Vec::new(),
            job: None,
        }
    }

    /// 🧱 Retains an already-admitted fixed mounted model without contiguous materialization.
    pub fn new_mounted(model: MountedAnalysisModel, operation: Operation, partition_count: usize) -> Self {
        Self::from_model(AssemblyConstructionModel::Mounted(model), operation, partition_count)
    }

    pub fn step_one(&mut self) -> Result<bool, FemError> {
        match self.stage {
            AssemblyConstructionStage::ReservePartitionTripletCounts => {
                if self.partition_count == 0 {
                    return Err(FemError::EmptyModel);
                }
                if !reserve_exact_owner_page(&mut self.partition_triplet_counts, self.partition_count) {
                    return Err(FemError::Singular);
                }
                self.stage = AssemblyConstructionStage::InitializePartitionTripletCounts;
            }
            AssemblyConstructionStage::InitializePartitionTripletCounts => {
                if self.partition_triplet_counts.len() < self.partition_count {
                    self.partition_triplet_counts.push(0);
                } else {
                    self.stage = AssemblyConstructionStage::ReserveDofs;
                }
            }
            AssemblyConstructionStage::ReserveDofs => {
                if self.model.as_ref().is_none_or(|model| model.nodes_len() == 0) {
                    return Err(FemError::EmptyModel);
                }
                let maximum_dofs = self.model.as_ref().ok_or(FemError::EmptyModel)?.nodes_len().checked_mul(6).ok_or(FemError::Singular)?;
                if !reserve_exact_owner_page(&mut self.plan.dof_map.order, maximum_dofs) {
                    return Err(FemError::Singular);
                }
                self.stage = AssemblyConstructionStage::ValidateNodePairs;
            }
            AssemblyConstructionStage::ValidateNodePairs => {
                let model = self.model.as_ref().ok_or(FemError::EmptyModel)?;
                if self.node_outer >= model.nodes_len() {
                    self.stage = AssemblyConstructionStage::ValidateElementReferences;
                } else if self.node_inner >= model.nodes_len() {
                    self.node_outer += 1;
                    self.node_inner = self.node_outer + 1;
                } else {
                    let outer = model.node(self.node_outer).ok_or(FemError::EmptyModel)?;
                    let inner = model.node(self.node_inner).ok_or(FemError::EmptyModel)?;
                    if outer.id == inner.id {
                        return Err(FemError::DuplicateNodeId(outer.id.clone()));
                    }
                    self.node_inner += 1;
                }
            }
            AssemblyConstructionStage::ValidateElementReferences => {
                let model = self.model.as_ref().ok_or(FemError::EmptyModel)?;
                if self.element_cursor >= model.elements_len() {
                    self.stage = AssemblyConstructionStage::ValidateSupportReferences;
                } else if self.reference_cursor >= model.element(self.element_cursor).and_then(Elements::mounted_node_id_count).ok_or(FemError::Singular)? {
                    let element = model.element(self.element_cursor).ok_or(FemError::Singular)?;
                    let side = element.mounted_node_id_count().and_then(|nodes| nodes.checked_mul(element.dofs_per_node().len())).ok_or(FemError::Singular)?;
                    let triplets = side.checked_mul(side).ok_or(FemError::Singular)?;
                    self.maximum_triplets = self.maximum_triplets.checked_add(triplets).ok_or(FemError::Singular)?;
                    let partition = self.element_cursor % self.partition_count;
                    self.partition_triplet_counts[partition] = self.partition_triplet_counts[partition].checked_add(triplets).ok_or(FemError::Singular)?;
                    self.element_cursor += 1;
                    self.reference_cursor = 0;
                    self.reference_node_cursor = 0;
                } else if self.reference_node_cursor >= model.nodes_len() {
                    return Err(FemError::DanglingNodeRef(model.element(self.element_cursor).and_then(|element| element.mounted_node_id(self.reference_cursor)).ok_or(FemError::Singular)?.to_owned()));
                } else if model.node(self.reference_node_cursor).ok_or(FemError::EmptyModel)?.id == model.element(self.element_cursor).and_then(|element| element.mounted_node_id(self.reference_cursor)).ok_or(FemError::Singular)? {
                    self.reference_cursor += 1;
                    self.reference_node_cursor = 0;
                } else {
                    self.reference_node_cursor += 1;
                }
            }
            AssemblyConstructionStage::ValidateSupportReferences => {
                let model = self.model.as_ref().ok_or(FemError::EmptyModel)?;
                if self.support_cursor >= model.supports_len() {
                    self.element_cursor = 0;
                    self.stage = AssemblyConstructionStage::DiscoverDofs;
                } else if self.support_node_cursor >= model.nodes_len() {
                    return Err(FemError::DanglingNodeRef(model.support_node_id(self.support_cursor).ok_or(FemError::Singular)?.to_owned()));
                } else if model.node(self.support_node_cursor).ok_or(FemError::EmptyModel)?.id == model.support_node_id(self.support_cursor).ok_or(FemError::Singular)? {
                    self.support_cursor += 1;
                    self.support_node_cursor = 0;
                } else {
                    self.support_node_cursor += 1;
                }
            }
            AssemblyConstructionStage::DiscoverDofs => {
                let model = self.model.as_ref().ok_or(FemError::EmptyModel)?;
                if self.dof_node_cursor >= model.nodes_len() {
                    self.stage = AssemblyConstructionStage::ReservePermutation;
                } else if self.dof_element_cursor >= model.elements_len() {
                    self.dof_emit_cursor = 0;
                    self.stage = AssemblyConstructionStage::EmitDofs;
                } else if self.dof_reference_cursor >= model.element(self.dof_element_cursor).and_then(Elements::mounted_node_id_count).ok_or(FemError::Singular)? {
                    self.dof_element_cursor += 1;
                    self.dof_reference_cursor = 0;
                } else {
                    let element = model.element(self.dof_element_cursor).ok_or(FemError::Singular)?;
                    if element.mounted_node_id(self.dof_reference_cursor).ok_or(FemError::Singular)? == model.node(self.dof_node_cursor).ok_or(FemError::EmptyModel)?.id {
                        for dof in element.dofs_per_node() {
                            self.active_dofs[dof.index()] = true;
                        }
                    }
                    self.dof_reference_cursor += 1;
                }
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
                        let node_id = self.model.as_ref().and_then(|model| model.node(self.dof_node_cursor)).ok_or(FemError::EmptyModel)?.id.clone();
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
                let model = self.model.as_ref().ok_or(FemError::EmptyModel)?;
                if self.constraint_support_cursor >= model.supports_len() {
                    self.stage = AssemblyConstructionStage::ReserveFree;
                } else if self.constraint_dof_cursor >= model.support_fixed_len(self.constraint_support_cursor).ok_or(FemError::Singular)? {
                    self.constraint_support_cursor += 1;
                    self.constraint_dof_cursor = 0;
                    self.constraint_order_cursor = 0;
                } else if self.constraint_order_cursor >= self.plan.dof_map.order.len() {
                    self.constraint_dof_cursor += 1;
                    self.constraint_order_cursor = 0;
                } else {
                    let (node_id, dof) = &self.plan.dof_map.order[self.constraint_order_cursor];
                    if node_id == model.support_node_id(self.constraint_support_cursor).ok_or(FemError::Singular)? && *dof == model.support_fixed(self.constraint_support_cursor, self.constraint_dof_cursor).ok_or(FemError::Singular)? {
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
                    let per_partition = self.partition_triplet_counts[self.partition_reserve_cursor];
                    if self.partition_reserve_lane == 0 {
                        partition.full.reserve_capacity_one(per_partition, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::Singular)?;
                        if partition.full.capacity() < per_partition {
                            return Ok(false);
                        }
                        self.partition_reserve_lane = 1;
                    } else {
                        partition.free.reserve_capacity_one(per_partition, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::Singular)?;
                        if partition.free.capacity() < per_partition {
                            return Ok(false);
                        }
                        self.partition_reserve_lane = 0;
                        self.partition_reserve_cursor += 1;
                    }
                } else {
                    self.stage = AssemblyConstructionStage::ReserveMergedFull;
                }
            }
            AssemblyConstructionStage::ReserveMergedFull => {
                self.merged_full.reserve_capacity_one(self.maximum_triplets, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::Singular)?;
                if self.merged_full.capacity() >= self.maximum_triplets {
                    self.stage = AssemblyConstructionStage::ReserveMergedFree;
                }
            }
            AssemblyConstructionStage::ReserveMergedFree => {
                self.merged_free.reserve_capacity_one(self.maximum_triplets, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| FemError::Singular)?;
                if self.merged_free.capacity() >= self.maximum_triplets {
                    self.scalar_cursor = 0;
                    self.stage = AssemblyConstructionStage::ReserveFullMergeCursors;
                }
            }
            AssemblyConstructionStage::ReserveFullMergeCursors => {
                if self.full_merge_cursors.is_empty() && !self.partition_triplet_counts.is_empty() {
                    self.full_merge_cursors = std::mem::take(&mut self.partition_triplet_counts);
                }
                if self.scalar_cursor < self.full_merge_cursors.len() {
                    self.full_merge_cursors[self.scalar_cursor] = 0;
                    self.scalar_cursor += 1;
                } else {
                    self.stage = AssemblyConstructionStage::ReserveFreeMergeCursors;
                }
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
                    let total_elements = model.elements_len();
                    let model_signature = self.operation.operation.0 ^ self.operation.base_revision.0.rotate_left(17) ^ self.operation.generation.0.rotate_left(33);
                    let plan = std::mem::replace(&mut self.plan, AssemblyPlan { dof_map: DofMap { order: Vec::new() }, inv_perm: Vec::new(), ndof: 0, free_new: Vec::new(), compact_of_new: Vec::new() });
                    self.job = Some(AssemblyJob {
                        state: AssemblyCheckpoint {
                            stage: AssemblyJobStage::ElementTriplets,
                            total_elements,
                            element_cursor: 0,
                            pending_build: None,
                            pending: None,
                            partitions: std::mem::take(&mut self.partitions),
                            full_merge_cursors: std::mem::take(&mut self.full_merge_cursors),
                            free_merge_cursors: std::mem::take(&mut self.free_merge_cursors),
                            merged_full: std::mem::take(&mut self.merged_full),
                            merged_free: std::mem::take(&mut self.merged_free),
                            checkpoint_due: false,
                            preview_due: false,
                            resume_target: 0,
                            merge_scan_partition: 0,
                            merge_candidate: None,
                        },
                        model: match model {
                            AssemblyConstructionModel::Dynamic(model) => AnalysisModelOwner::Owned(model),
                            AssemblyConstructionModel::Mounted(model) => AnalysisModelOwner::Mounted(model),
                        },
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
        match close_vec_owner_step(&mut self.partition_triplet_counts, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        for owner in [&mut self.merged_full, &mut self.merged_free] {
            match close_paged_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        for owner in [&mut self.full_merge_cursors, &mut self.free_merge_cursors] {
            match close_vec_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        if let Some(partition) = self.partitions.last_mut() {
            match close_paged_owner_step(&mut partition.full, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
            match close_paged_owner_step(&mut partition.free, maximum_bytes) {
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
            let (terminal, items, bytes) = match model {
                AssemblyConstructionModel::Dynamic(model) => close_analysis_model_step(model, &mut self.model_close, maximum_bytes),
                AssemblyConstructionModel::Mounted(model) => model.close_step(maximum_bytes),
            };
            if !terminal {
                return (false, items, bytes);
            }
            self.model = None;
            return (false, 1, 0);
        }
        (true, 0, 0)
    }
}

impl AssemblyJob<'static> {
    /// 🧭 Resolves one old node/DOF identity to its retained full and free equation indices.
    pub fn visual_equation_indices(&self, node_id: &str, dof: Dof) -> Option<(usize, Option<usize>)> {
        let old = self.plan.dof_map.get(node_id, dof)?;
        let new = *self.plan.inv_perm.get(old)?;
        Some((new, self.plan.compact_of_new.get(new).copied().flatten()))
    }

    pub fn visual_free_order(&self) -> usize {
        self.plan.free_new.len()
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
    Mounted(MountedAnalysisModel),
}

impl AnalysisModelOwner<'_> {
    fn dynamic(&self) -> Option<&AnalysisModel> {
        match self {
            Self::Borrowed(model) => Some(model),
            Self::Owned(model) => Some(model.as_ref()),
            Self::Mounted(_) => None,
        }
    }

    fn elements_len(&self) -> usize {
        match self {
            Self::Borrowed(model) => model.elements.len(),
            Self::Owned(model) => model.elements.len(),
            Self::Mounted(model) => model.elements_len(),
        }
    }

    fn node(&self, index: usize) -> Option<&Node> {
        match self {
            Self::Borrowed(model) => model.nodes.get(index),
            Self::Owned(model) => model.nodes.get(index),
            Self::Mounted(model) => model.node(index),
        }
    }

    fn element(&self, index: usize) -> Option<&Elements> {
        match self {
            Self::Borrowed(model) => model.elements.get(index),
            Self::Owned(model) => model.elements.get(index),
            Self::Mounted(model) => model.element(index),
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
        let dynamic = model.dynamic().ok_or(FemError::EmptyModel)?;
        let plan = AssemblyPlan::prepare(dynamic)?;
        let model_signature = assembly_model_signature(dynamic);
        let total_elements = model.elements_len();
        let mut partition_triplet_counts = vec![0usize; partition_count];
        let maximum_triplets = dynamic.elements.iter().enumerate().try_fold(0usize, |total, (index, element)| {
            let side = element.node_ids().len().checked_mul(element.dofs_per_node().len()).ok_or(FemError::Singular)?;
            let triplets = side.checked_mul(side).ok_or(FemError::Singular)?;
            let partition = index % partition_count;
            partition_triplet_counts[partition] = partition_triplet_counts[partition].checked_add(triplets).ok_or(FemError::Singular)?;
            total.checked_add(triplets).ok_or(FemError::Singular)
        })?;
        let mut partitions = Vec::with_capacity(partition_count);
        for capacity in partition_triplet_counts {
            partitions.push(AssemblyPartitionBuffer { full: cold_paged_owner(capacity)?, free: cold_paged_owner(capacity)? });
        }
        Ok(Self {
            model,
            operation,
            model_signature,
            plan,
            state: AssemblyCheckpoint {
                stage: AssemblyJobStage::ElementTriplets,
                total_elements,
                element_cursor: 0,
                pending_build: None,
                pending: None,
                partitions,
                full_merge_cursors: vec![0; partition_count],
                free_merge_cursors: vec![0; partition_count],
                merged_full: cold_paged_owner(maximum_triplets)?,
                merged_free: cold_paged_owner(maximum_triplets)?,
                checkpoint_due: false,
                preview_due: false,
                resume_target: 0,
                merge_scan_partition: 0,
                merge_candidate: None,
            },
            close_lane: 0,
            model_close: AnalysisModelCloseCursor::default(),
        })
    }

    pub fn from_checkpoint(model: &'model AnalysisModel, operation: Operation, bytes: &[u8]) -> Result<Self, String> {
        let checkpoint: AssemblyResumeCheckpoint = decode_value(bytes)?;
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
        encode_value(&checkpoint)
    }

    pub fn preview(&self) -> AssemblyPreview {
        AssemblyPreview {
            stage: self.state.stage,
            completed_elements: self.state.element_cursor,
            total_elements: self.state.total_elements,
            full_triplets: self.state.partitions.iter().map(|partition| partition.full.len()).sum(),
            free_triplets: self.state.partitions.iter().map(|partition| partition.free.len()).sum(),
            assembled_element_ids: (0..self.state.element_cursor).filter_map(|index| self.model.element(index)).map(|element| element.id().to_string()).collect(),
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
                        match close_paged_owner_step(&mut partition.full, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        match close_paged_owner_step(&mut partition.free, maximum_bytes) {
                            Ok(Some(step)) => return (false, step.0, step.1),
                            Err(()) => return (false, 0, 0),
                            Ok(None) => {}
                        }
                        self.state.partitions.pop();
                        (1, 0)
                    } else {
                        let bytes = self.state.partitions.capacity() * size_of::<AssemblyPartitionBuffer>();
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
                4 => match close_paged_owner_step(&mut self.state.merged_full, maximum_bytes) {
                    Ok(Some(step)) => step,
                    Err(()) => return (false, 0, 0),
                    Ok(None) => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                5 => match close_paged_owner_step(&mut self.state.merged_free, maximum_bytes) {
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
                    AnalysisModelOwner::Mounted(model) => {
                        let (terminal, items, bytes) = model.close_step(maximum_bytes);
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
            let element = self.model.element(element_index).ok_or(FemError::Singular)?;
            let node_count = element.mounted_node_id_count().ok_or(FemError::Singular)?;
            let dof_count = element.dofs_per_node().len();
            node_count.checked_mul(dof_count).ok_or(FemError::Singular)?;
            self.state.pending_build = Some(PendingElementBuild {
                element_index,
                node_count,
                dof_count,
                scalar_cursor: 0,
                lookup_cursor: 0,
                lookup_match: None,
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
        let element = self.model.element(build.element_index).ok_or(FemError::Singular)?;
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
                    let Some((current, current_dof)) = self.plan.dof_map.order.get(build.lookup_cursor) else { return Err(FemError::Singular) };
                    if current == node_id && *current_dof == element.dofs_per_node()[dof_index] {
                        build.lookup_match = Some(build.lookup_cursor);
                        build.stage = PendingElementBuildStage::PublishIndex;
                    } else {
                        build.lookup_cursor += 1;
                    }
                } else {
                    build.scalar_cursor = 0;
                    build.lookup_cursor = 0;
                    build.stage = PendingElementBuildStage::ReservePositions;
                }
            }
            PendingElementBuildStage::PublishIndex => {
                let old = build.lookup_match.take().ok_or(FemError::Singular)?;
                build.indices_new.push(self.plan.inv_perm[old]);
                build.scalar_cursor += 1;
                build.lookup_cursor = 0;
                build.stage = PendingElementBuildStage::Indices;
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
                    let Some(node) = self.model.node(build.lookup_cursor) else { return Err(FemError::Singular) };
                    if node.id == node_id {
                        build.lookup_match = Some(build.lookup_cursor);
                        build.stage = PendingElementBuildStage::PublishPosition;
                    } else {
                        build.lookup_cursor += 1;
                    }
                } else {
                    build.stage = PendingElementBuildStage::ReserveStiffnessCredit;
                }
            }
            PendingElementBuildStage::PublishPosition => {
                let node = build.lookup_match.take().and_then(|index| self.model.node(index)).ok_or(FemError::Singular)?;
                build.positions.push(node.pos);
                build.scalar_cursor += 1;
                build.lookup_cursor = 0;
                build.stage = PendingElementBuildStage::Positions;
            }
            PendingElementBuildStage::ReserveStiffnessCredit => {
                let side = build.indices_new.len();
                let requested_bytes = side.checked_mul(side).and_then(|cells| cells.checked_mul(size_of::<f64>())).ok_or(FemError::Singular)?;
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
                let Some(observed_bytes) = build.stiffness.capacity().checked_mul(size_of::<f64>()) else { return Err(FemError::Singular) };
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
        let dynamic = self.model.dynamic().ok_or(FemError::Singular)?;
        let element = dynamic.elements.get(element_index).ok_or(FemError::Singular)?;
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let indices_old = element_global_indices(&self.plan.dof_map, &node_ids, dofs).ok_or(FemError::Singular)?;
        let indices_new = indices_old.iter().map(|&old| self.plan.inv_perm[old]).collect::<Vec<_>>();
        let context = ElementContext { positions: positions_of(&dynamic.nodes, &node_ids) };
        let stiffness = element.stiffness_global(&context);
        let side = indices_new.len();
        self.state.pending = Some(PendingElementAssembly { element_index, side, cell_cursor: 0, reclaim_lane: 0, complete: false, indices_new, positions: context.positions, stiffness: stiffness.data });
        Ok(())
    }

    fn assemble_cell(&mut self) -> Result<(), FemError> {
        let pending = self.state.pending.as_mut().expect("pending element exists");
        if pending.side == 0 {
            pending.complete = true;
            self.state.element_cursor += 1;
            return Ok(());
        }
        let local_row = pending.cell_cursor / pending.side;
        let local_col = pending.cell_cursor % pending.side;
        let value = pending.stiffness[pending.cell_cursor];
        if value != 0.0 {
            let new_row = pending.indices_new[local_row];
            let new_col = pending.indices_new[local_col];
            let sequence = ((pending.element_index as u64) << 32) | pending.cell_cursor as u64;
            let partition = pending.element_index % self.state.partitions.len();
            let full = AssemblyTriplet { sequence, row: new_row as u32, col: new_col as u32, value };
            let free =
                if let (Some(compact_row), Some(compact_col)) = (self.plan.compact_of_new[new_row], self.plan.compact_of_new[new_col]) { Some(AssemblyTriplet { sequence, row: compact_row as u32, col: compact_col as u32, value }) } else { None };
            let owner = &mut self.state.partitions[partition];
            if !owner.full.has_reserved_slot() || free.is_some() && !owner.free.has_reserved_slot() {
                return Err(FemError::Singular);
            }
            owner.full.push_reserved(full).map_err(|_| FemError::Singular)?;
            if let Some(free) = free {
                owner.free.push_reserved(free).map_err(|_| FemError::Singular)?;
            }
        }
        pending.cell_cursor += 1;
        if pending.cell_cursor == pending.side * pending.side {
            self.state.element_cursor += 1;
            pending.complete = true;
        }
        Ok(())
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
                self.state.checkpoint_due = self.state.element_cursor.is_multiple_of(16) || self.state.element_cursor == self.state.total_elements;
            }
        }
        true
    }

    fn advance_partition_merge(&mut self, full: bool) -> Result<Option<bool>, FemError> {
        if self.state.merge_scan_partition < self.state.partitions.len() {
            let partition_index = self.state.merge_scan_partition;
            let partition = &self.state.partitions[partition_index];
            let cursor = if full { self.state.full_merge_cursors[partition_index] } else { self.state.free_merge_cursors[partition_index] };
            let entry = if full { partition.full.get(cursor) } else { partition.free.get(cursor) }.copied();
            if let Some(entry) = entry {
                if self.state.merge_candidate.is_none_or(|(_, candidate)| entry.sequence < candidate.sequence) {
                    self.state.merge_candidate = Some((partition_index, entry));
                }
            }
            self.state.merge_scan_partition += 1;
            return Ok(Some(false));
        }
        let Some((partition_index, entry)) = self.state.merge_candidate else {
            return Ok(None);
        };
        let placement = if full { self.state.merged_full.push_reserved(entry) } else { self.state.merged_free.push_reserved(entry) };
        if let Err(entry) = placement {
            self.state.merge_candidate = Some((partition_index, entry));
            return Err(FemError::Singular);
        }
        if full {
            self.state.full_merge_cursors[partition_index] += 1;
        } else {
            self.state.free_merge_cursors[partition_index] += 1;
        }
        self.state.merge_candidate = None;
        self.state.merge_scan_partition = 0;
        Ok(Some(true))
    }

    fn finish(self) -> Option<UnfactoredSystem> {
        if self.state.stage != AssemblyJobStage::Complete {
            return None;
        }
        let mut k_full_coo = Coo::new(self.plan.ndof);
        for entry in self.state.merged_full.iter() {
            k_full_coo.add(entry.row as usize, entry.col as usize, entry.value);
        }
        let mut k_ff_coo = Coo::new(self.plan.free_new.len());
        for entry in self.state.merged_free.iter() {
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
    CountUnique,
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
    entries: PagedList<AssemblyTriplet, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    stage: AssemblyCsrBuildStage,
    sort_outer: usize,
    sort_inner: usize,
    count_cursor: usize,
    unique_count: usize,
    count_key: Option<(u32, u32)>,
    merge_cursor: usize,
    row_cursor: usize,
    row_counts: PagedList<u32, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    indptr: PagedList<u32, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    indices: PagedList<u32, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    values: PagedList<f64, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    last_key: Option<(u32, u32)>,
    matrix: Option<Csr>,
    n: usize,
}

impl AssemblyCsrBuild {
    pub fn new(mut assembly: AssemblyJob<'static>) -> Result<Self, AssemblyJob<'static>> {
        if assembly.state.stage != AssemblyJobStage::Complete {
            return Err(assembly);
        }
        let entries = std::mem::take(&mut assembly.state.merged_full);
        let n = assembly.plan.ndof;
        Ok(Self {
            assembly: Some(assembly),
            entries,
            stage: AssemblyCsrBuildStage::ReserveRows,
            sort_outer: 1,
            sort_inner: 1,
            count_cursor: 0,
            unique_count: 0,
            count_key: None,
            merge_cursor: 0,
            row_cursor: 0,
            row_counts: PagedList::default(),
            indptr: PagedList::default(),
            indices: PagedList::default(),
            values: PagedList::default(),
            last_key: None,
            matrix: None,
            n,
        })
    }

    /// 🧮 Transfers only the unconstrained free×free owner for a physical retained solve.
    pub fn new_free(mut assembly: AssemblyJob<'static>) -> Result<Self, AssemblyJob<'static>> {
        if assembly.state.stage != AssemblyJobStage::Complete {
            return Err(assembly);
        }
        let entries = std::mem::take(&mut assembly.state.merged_free);
        let n = assembly.plan.free_new.len();
        Ok(Self {
            assembly: Some(assembly),
            entries,
            stage: AssemblyCsrBuildStage::ReserveRows,
            sort_outer: 1,
            sort_inner: 1,
            count_cursor: 0,
            unique_count: 0,
            count_key: None,
            merge_cursor: 0,
            row_cursor: 0,
            row_counts: PagedList::default(),
            indptr: PagedList::default(),
            indices: PagedList::default(),
            values: PagedList::default(),
            last_key: None,
            matrix: None,
            n,
        })
    }

    pub fn step_one(&mut self) -> Result<bool, &'static [u8]> {
        let n = self.n;
        match self.stage {
            AssemblyCsrBuildStage::ReserveRows => {
                self.row_counts.reserve_capacity_one(n, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| b"fem.assembly-csr-row-allocation" as &'static [u8])?;
                if self.row_counts.capacity() >= n {
                    self.stage = AssemblyCsrBuildStage::InitializeRows;
                }
            }
            AssemblyCsrBuildStage::InitializeRows => {
                if self.row_counts.len() < n {
                    self.row_counts.push_reserved(0).map_err(|_| b"fem.assembly-csr-row-placement" as &'static [u8])?;
                } else {
                    self.stage = AssemblyCsrBuildStage::Sort;
                }
            }
            AssemblyCsrBuildStage::Sort => {
                if self.sort_outer >= self.entries.len() {
                    self.stage = AssemblyCsrBuildStage::CountUnique;
                } else if self.sort_inner > 0 {
                    let left = self.sort_inner - 1;
                    let right = self.sort_inner;
                    let left_entry = self.entries.get(left).copied().ok_or(b"fem.assembly-csr-left-entry" as &'static [u8])?;
                    let right_entry = self.entries.get(right).copied().ok_or(b"fem.assembly-csr-right-entry" as &'static [u8])?;
                    let left_key = (left_entry.row, left_entry.col, left_entry.sequence);
                    let right_key = (right_entry.row, right_entry.col, right_entry.sequence);
                    if right_key < left_key {
                        *self.entries.get_mut(left).ok_or(b"fem.assembly-csr-left-entry" as &'static [u8])? = right_entry;
                        *self.entries.get_mut(right).ok_or(b"fem.assembly-csr-right-entry" as &'static [u8])? = left_entry;
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
            AssemblyCsrBuildStage::CountUnique => {
                if let Some(entry) = self.entries.get(self.count_cursor).copied() {
                    let key = (entry.row, entry.col);
                    if self.count_key != Some(key) {
                        self.unique_count = self.unique_count.checked_add(1).ok_or(b"fem.assembly-csr-unique-overflow" as &'static [u8])?;
                        self.count_key = Some(key);
                    }
                    self.count_cursor += 1;
                } else {
                    self.stage = AssemblyCsrBuildStage::ReserveIndices;
                }
            }
            AssemblyCsrBuildStage::ReserveIndices => {
                self.indices.reserve_capacity_one(self.unique_count, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| b"fem.assembly-csr-index-allocation" as &'static [u8])?;
                if self.indices.capacity() >= self.unique_count {
                    self.stage = AssemblyCsrBuildStage::ReserveValues;
                }
            }
            AssemblyCsrBuildStage::ReserveValues => {
                self.values.reserve_capacity_one(self.unique_count, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| b"fem.assembly-csr-value-allocation" as &'static [u8])?;
                if self.values.capacity() >= self.unique_count {
                    self.stage = AssemblyCsrBuildStage::Merge;
                }
            }
            AssemblyCsrBuildStage::Merge => {
                if let Some(entry) = self.entries.get(self.merge_cursor).copied() {
                    let key = (entry.row, entry.col);
                    if self.last_key == Some(key) {
                        let last = self.values.len().checked_sub(1).ok_or(b"fem.assembly-csr-missing-value" as &'static [u8])?;
                        *self.values.get_mut(last).ok_or(b"fem.assembly-csr-missing-value" as &'static [u8])? += entry.value;
                    } else {
                        if !self.indices.has_reserved_slot() || !self.values.has_reserved_slot() {
                            return Err(b"fem.assembly-csr-output-capacity");
                        }
                        self.indices.push_reserved(entry.col).map_err(|_| b"fem.assembly-csr-index-placement" as &'static [u8])?;
                        self.values.push_reserved(entry.value).map_err(|_| b"fem.assembly-csr-value-placement" as &'static [u8])?;
                        let count = self.row_counts.get_mut(entry.row as usize).ok_or(b"fem.assembly-csr-row-bound" as &'static [u8])?;
                        *count = count.checked_add(1).ok_or(b"fem.assembly-csr-row-overflow" as &'static [u8])?;
                        self.last_key = Some(key);
                    }
                    self.merge_cursor += 1;
                } else {
                    self.stage = AssemblyCsrBuildStage::ReserveIndptr;
                }
            }
            AssemblyCsrBuildStage::ReserveIndptr => {
                let required = n.checked_add(1).ok_or(b"fem.assembly-csr-indptr-overflow" as &'static [u8])?;
                self.indptr.reserve_capacity_one(required, MOUNTED_OWNER_PAGE_BYTES).map_err(|_| b"fem.assembly-csr-indptr-allocation" as &'static [u8])?;
                if self.indptr.capacity() >= required {
                    self.indptr.push_reserved(0).map_err(|_| b"fem.assembly-csr-indptr-placement" as &'static [u8])?;
                    self.stage = AssemblyCsrBuildStage::Indptr;
                }
            }
            AssemblyCsrBuildStage::Indptr => {
                if let Some(count) = self.row_counts.get(self.row_cursor).copied() {
                    let previous = self.indptr.get(self.indptr.len() - 1).copied().unwrap_or(0);
                    let next = previous.checked_add(count).ok_or(b"fem.assembly-csr-indptr-overflow" as &'static [u8])?;
                    self.indptr.push_reserved(next).map_err(|_| b"fem.assembly-csr-indptr-placement" as &'static [u8])?;
                    self.row_cursor += 1;
                } else {
                    self.matrix = Some(Csr::from_paged_parts(n, std::mem::take(&mut self.indptr), std::mem::take(&mut self.indices), std::mem::take(&mut self.values)));
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

    /// ⚖️ Borrows one retained full-system triplet for bounded constrained-reaction recovery.
    pub fn visual_full_entry(&self, index: usize) -> Option<(usize, usize, f64)> {
        let entry = self.assembly.as_ref()?.state.merged_full.get(index)?;
        Some((entry.row as usize, entry.col as usize, entry.value))
    }

    pub fn visual_compact_index(&self, new_index: usize) -> Option<usize> {
        self.assembly.as_ref()?.plan.compact_of_new.get(new_index).copied().flatten()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(assembly) = self.assembly.as_mut() {
            let (terminal, items, bytes) = assembly.close_step(maximum_bytes);
            if !terminal {
                return (false, items, bytes);
            }
            self.assembly = None;
            return (false, 1, 0);
        }
        match close_paged_owner_step(&mut self.entries, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        for owner in [&mut self.row_counts, &mut self.indptr, &mut self.indices] {
            match close_paged_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        match close_paged_owner_step(&mut self.values, maximum_bytes) {
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
            return (false, 1, 0);
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
            return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
        }
        context.set_stage(self.state.pending_build.as_ref().map_or_else(|| self.state.stage.label(), |build| build.stage.label()));
        if context.should_yield() {
            return StepOutcome::Yield;
        }
        context.consume_fuel(1);
        if self.state.checkpoint_due {
            self.state.checkpoint_due = false;
            if matches!(&self.model, AnalysisModelOwner::Owned(_) | AnalysisModelOwner::Mounted(_)) {
                return StepOutcome::Yield;
            }
            let bytes = self.checkpoint_bytes();
            return match context.payload_from_bytes(JobPayloadStream::CheckpointState, &bytes) {
                Ok(state) => StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state, applied_progress: self.state.element_cursor as u64 }),
                Err(_) => StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
            };
        }
        if self.state.preview_due {
            self.state.preview_due = false;
            if matches!(&self.model, AnalysisModelOwner::Owned(_) | AnalysisModelOwner::Mounted(_)) {
                return StepOutcome::Yield;
            }
            let bytes = encode_value(&self.preview());
            return match context.payload_from_bytes(JobPayloadStream::Preview, &bytes) {
                Ok(preview) => StepOutcome::PreviewReady(preview),
                Err(_) => StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
            };
        }
        if self.state.stage == AssemblyJobStage::Complete {
            if matches!(&self.model, AnalysisModelOwner::Owned(_) | AnalysisModelOwner::Mounted(_)) {
                return StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) });
            }
            let bytes = encode_value(&self.preview());
            return match context.payload_from_bytes(JobPayloadStream::CommitOutput, &bytes) {
                Ok(output) => StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output }),
                Err(_) => StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
            };
        }
        match self.state.stage {
            AssemblyJobStage::ElementTriplets => {
                if !self.reclaim_element_owner() {
                    if self.state.pending.is_none() {
                        if self.state.element_cursor == self.state.total_elements {
                            self.state.stage = AssemblyJobStage::MergeFull;
                        } else {
                            let result = if matches!(&self.model, AnalysisModelOwner::Owned(_) | AnalysisModelOwner::Mounted(_)) { self.advance_element_build().map(|_| ()) } else { self.begin_borrowed_element() };
                            if result.is_err() {
                                return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
                            }
                        }
                    } else {
                        if self.assemble_cell().is_err() {
                            return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
                        }
                    }
                }
            }
            AssemblyJobStage::MergeFull => match self.advance_partition_merge(true) {
                Ok(None) => {
                    self.state.merge_scan_partition = 0;
                    self.state.merge_candidate = None;
                    self.state.stage = AssemblyJobStage::MergeFree;
                }
                Ok(Some(_)) => {}
                Err(_) => return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
            },
            AssemblyJobStage::MergeFree => match self.advance_partition_merge(false) {
                Ok(None) => self.state.stage = AssemblyJobStage::Complete,
                Ok(Some(_)) => {}
                Err(_) => return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
            },
            AssemblyJobStage::Complete => {}
        }
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if self.state.preview_due {
            self.state.preview_due = false;
            let bytes = encode_value(&self.preview());
            match context.payload_from_bytes(JobPayloadStream::Preview, &bytes) {
                Ok(preview) => StepOutcome::PreviewReady(preview),
                Err(_) => StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
            }
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
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(4_096, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut preview_sequence);
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
        for (sum, reaction) in reaction_sum.iter_mut().zip(cr.checks.reaction_sum) {
            *sum += factor * reaction;
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
    let max_diag = diag_estimate.iter().copied().fold(0.0_f64, f64::max);
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
