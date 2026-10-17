//! 🔶️ The CSR build, the assembly run loop and the one-shot assembled system.

use super::*;

// #region 🔖️Assembly
impl AssemblyJob<'static> {
    /// 🧵️ Worker-session constructor retaining an immutable model root across bounded turns.
    pub fn new_owned(model: Arc<AnalysisModel>, operation: Operation, partition_count: usize) -> Result<Self, FemError> {
        Self::from_owner(AnalysisModelOwner::Owned(model), operation, partition_count)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum AssemblyCsrBuildStage {
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
    pub(super) assembly: Option<AssemblyJob<'static>>,
    pub(super) entries: PagedList<AssemblyTriplet, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    pub(super) stage: AssemblyCsrBuildStage,
    pub(super) sort_outer: usize,
    pub(super) sort_inner: usize,
    pub(super) count_cursor: usize,
    pub(super) unique_count: usize,
    pub(super) count_key: Option<(u32, u32)>,
    pub(super) merge_cursor: usize,
    pub(super) row_cursor: usize,
    pub(super) row_counts: PagedList<u32, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    pub(super) indptr: PagedList<u32, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    pub(super) indices: PagedList<u32, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    pub(super) values: PagedList<f64, ASSEMBLY_TRIPLET_INDEX_SPACE>,
    pub(super) last_key: Option<(u32, u32)>,
    pub(super) matrix: Option<Csr>,
    pub(super) n: usize,
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
            let (terminal, items, bytes) = assembly.close_retained_step(maximum_bytes);
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

impl AssemblyJob<'_> {
    /// 🧹️ Pays back the one lent completion payload so the finished matrices can be taken.
    pub fn retire_outcome(&mut self) {
        self.desk.retire_all();
    }

    fn run(&mut self, context: &mut StepContext<'_>) -> NumericalRun {
        if context.is_cancelled() {
            return NumericalRun::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return NumericalRun::Fault;
        }
        context.set_stage(self.state.pending_build.as_ref().map_or_else(|| self.state.stage.label(), |build| build.stage.label()));
        if context.should_yield() {
            return NumericalRun::Yield;
        }
        context.consume_fuel(1);
        if self.state.checkpoint_due {
            self.state.checkpoint_due = false;
            if matches!(&self.model, AnalysisModelOwner::Owned(_) | AnalysisModelOwner::Mounted(_)) {
                return NumericalRun::Yield;
            }
            let bytes = self.checkpoint_bytes();
            return match context.payload_from_bytes(JobPayloadStream::CheckpointState, &bytes) {
                Ok(state) => self.desk.lend_checkpoint(state, self.state.element_cursor as u64),
                Err(_) => NumericalRun::Fault,
            };
        }
        if self.state.preview_due {
            self.state.preview_due = false;
            if matches!(&self.model, AnalysisModelOwner::Owned(_) | AnalysisModelOwner::Mounted(_)) {
                return NumericalRun::Yield;
            }
            let bytes = encode_value(&self.preview());
            return match context.payload_from_bytes(JobPayloadStream::Preview, &bytes) {
                Ok(preview) => {
                    self.state.preview_cursor = self.state.element_cursor;
                    self.desk.lend_preview(preview)
                }
                Err(_) => NumericalRun::Fault,
            };
        }
        if self.state.stage == AssemblyJobStage::Complete {
            if matches!(&self.model, AnalysisModelOwner::Owned(_) | AnalysisModelOwner::Mounted(_)) {
                return NumericalRun::Complete;
            }
            let bytes = encode_value(&self.preview());
            return match context.payload_from_bytes(JobPayloadStream::CommitOutput, &bytes) {
                Ok(output) => self.desk.lend_complete(output),
                Err(_) => NumericalRun::Fault,
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
                                return NumericalRun::Fault;
                            }
                        }
                    } else {
                        if self.assemble_cell().is_err() {
                            return NumericalRun::Fault;
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
                Err(_) => return NumericalRun::Fault,
            },
            AssemblyJobStage::MergeFree => match self.advance_partition_merge(false) {
                Ok(None) => self.state.stage = AssemblyJobStage::Complete,
                Ok(Some(_)) => {}
                Err(_) => return NumericalRun::Fault,
            },
            AssemblyJobStage::Complete => {}
        }
        if context.is_cancelled() {
            return NumericalRun::Cancelled;
        }
        if self.state.preview_due {
            self.state.preview_due = false;
            let bytes = encode_value(&self.preview());
            match context.payload_from_bytes(JobPayloadStream::Preview, &bytes) {
                Ok(preview) => {
                    self.state.preview_cursor = self.state.element_cursor;
                    self.desk.lend_preview(preview)
                }
                Err(_) => NumericalRun::Fault,
            }
        } else {
            NumericalRun::Yield
        }
    }

    fn close_frontier(&self) -> Result<Option<RetirementDemand>, ValueError> {
        self.desk.retirement_demands().transpose()
    }

    fn close_demand(&self) -> Result<RetirementDemand, ValueError> {
        Ok(self.close_frontier()?.unwrap_or_else(|| numerical_rung_demand(self.close_lane > 11)))
    }
}

impl InteractiveJob for AssemblyJob<'_> {
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
        self.close_lane > 11 && self.desk.terminal_is_empty()
    }
}

impl AssemblyJob<'_> {
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

/// 🧮️ The shared, once-per-model assembly: DOF map, RCM permutation, free/constrained partition
/// (partitioned BEFORE assembly per the design — only free×free entries feed the LDLT factor), and
/// both the free-free `LdltFactor` (for solves) and the full `Csr` (for reactions/residuals).
pub struct AssembledSystem {
    pub dof_map: DofMap,
    pub inv_perm: Vec<usize>,
    pub ndof: usize,
    pub free_new: Vec<usize>,
    pub compact_of_new: Vec<Option<usize>>,
    pub k_factor: LdltFactor,
    pub k_full: Csr,
}

impl AssembledSystem {
    pub fn n_free(&self) -> usize {
        self.free_new.len()
    }
}

pub fn assemble_system(model: &AnalysisModel) -> Result<AssembledSystem, FemError> {
    let operation = Operation::new(semio_framework_job::OperationId(u64::MAX - 5), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), 0);
    let mut job = AssemblyJob::new(model, operation, 1)?;
    let mut preview_sequence = 0;
    loop {
        match numerical_batch_step(&mut job, operation, &mut preview_sequence) {
            NumericalBatchEnd::Complete => {
                job.retire_outcome();
                break;
            }
            NumericalBatchEnd::Fault | NumericalBatchEnd::Cancelled => {
                numerical_batch_close(&mut job);
                return Err(FemError::Singular);
            }
            NumericalBatchEnd::Continue => {}
        }
    }
    let unfactored = job.finish().expect("completed assembly owns its matrices");
    let k_full = unfactored.k_full_coo.to_csr();
    let k_factor = ldlt_factor(&unfactored.k_ff_coo.to_csc_sym_upper()).map_err(|_| FemError::Singular)?;
    let AssemblyPlan { dof_map, inv_perm, ndof, free_new, compact_of_new, node_slot: _ } = unfactored.plan;
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
pub fn case_rhs_old(model: &AnalysisModel, dof_map: &DofMap, case: &LoadCase, gravity: [f64; 3]) -> VecD {
    let ndof = dof_map.len();
    let mut f = VecD::zeros(ndof);

    let node_slot = node_index_of(&model.nodes);
    for element in &model.elements {
        let node_ids = element.node_ids();
        let dofs = element.dofs_per_node();
        let Some(indices) = element_global_indices(dof_map, &node_ids, dofs) else { continue };
        let ctx = ElementContext { positions: positions_of(&node_slot, &model.nodes, &node_ids) };

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
