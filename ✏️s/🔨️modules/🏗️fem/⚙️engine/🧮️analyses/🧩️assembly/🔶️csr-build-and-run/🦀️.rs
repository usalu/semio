//! 🔶️ The CSR build, the assembly run loop and the one-shot assembled system.

use super::*;

// #region 🔖️Assembly


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
    pub(super) csr: CsrTripletBuild,
}

impl AssemblyCsrBuild {
    /// 🏗️ Borrows a retained analysis node for bounded result recovery.
    pub fn model_node(&self, index: usize) -> Option<&Node> { self.assembly.as_ref()?.model_node(index) }
    /// 🧱️ Borrows a retained analysis element for bounded result recovery.
    pub fn model_element(&self, index: usize) -> Option<&Elements> { self.assembly.as_ref()?.model_element(index) }
    /// 🧭️ Borrows equation indices from the original retained assembly owner.
    pub fn visual_equation_indices(&self, node_id: &str, dof: Dof) -> Option<(usize, Option<usize>)> { self.assembly.as_ref()?.visual_equation_indices(node_id, dof) }
    pub fn new(mut assembly: AssemblyJob<'static>) -> Result<Self, AssemblyJob<'static>> {
        if assembly.state.stage != AssemblyJobStage::Complete { return Err(assembly); }
        let entries = std::mem::take(&mut assembly.state.merged_full);
        let csr = CsrTripletBuild::new(assembly.plan.ndof, entries);
        Ok(Self { assembly: Some(assembly), csr })
    }
    /// 🧮️ Transfers the original unconstrained free×free triplets into the same conversion cursor.
    pub fn new_free(mut assembly: AssemblyJob<'static>) -> Result<Self, AssemblyJob<'static>> {
        if assembly.state.stage != AssemblyJobStage::Complete { return Err(assembly); }
        let entries = std::mem::take(&mut assembly.state.merged_free);
        let csr = CsrTripletBuild::new(assembly.plan.free_new.len(), entries);
        Ok(Self { assembly: Some(assembly), csr })
    }
    pub fn next_demands(&self) -> Result<RetirementDemand, &'static [u8]> { self.csr.next_demands() }
    pub fn step_one_funded(&mut self, grant: RetainedCloneGrant) -> Result<(bool, RetainedCloneProgress), &'static [u8]> { self.csr.step_one_funded(grant) }
    pub fn take_demands(&self) -> Option<RetirementDemand> { self.csr.take_demands() }
    pub fn take_complete(&mut self, grant: RetainedCloneGrant) -> Option<(Csr, RetainedCloneProgress)> { self.csr.take_complete(grant) }
    /// ⚖️ Borrows one original full-system triplet for constrained-reaction recovery.
    pub fn visual_full_entry(&self, index: usize) -> Option<(usize, usize, f64)> {
        let entry = self.assembly.as_ref()?.state.merged_full.get(index)?;
        Some((entry.row as usize, entry.col as usize, entry.value))
    }
    pub fn visual_compact_index(&self, new_index: usize) -> Option<usize> { self.assembly.as_ref()?.plan.compact_of_new.get(new_index).copied().flatten() }
    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(assembly) = self.assembly.as_mut() {
            let step = assembly.close_retained_step(maximum_bytes);
            if !step.0 { return step; }
            self.assembly = None;
            return (false, 1, 0);
        }
        self.csr.close_step(maximum_bytes)
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
    pub inv_perm: AssemblyOrder<usize>,
    pub ndof: usize,
    pub free_new: AssemblyOrder<usize>,
    pub compact_of_new: AssemblyOrder<Option<usize>>,
    pub k_factor: LdltFactor,
    pub k_full: Csr,
}

impl AssembledSystem {
    pub fn n_free(&self) -> usize {
        self.free_new.len()
    }
}

fn element_contribution(plan: &AssemblyPlan, model: &AnalysisModel, element_index: usize) -> Result<PendingElementAssembly, FemError> {
    let element = model.elements.get(element_index).ok_or(FemError::Singular)?;
    let node_ids = element.node_ids();
    let dofs = element.dofs_per_node();
    let indices_old = element_global_indices(&plan.dof_map, &node_ids, dofs).ok_or(FemError::Singular)?;
    let indices_new = indices_old.iter().map(|&old| plan.inv_perm[old]).collect::<Vec<_>>();
    let context = ElementContext { positions: positions_of(&plan.node_slot, &model.nodes, &node_ids) };
    let stiffness = element.stiffness_global(&context);
    let side = indices_new.len();
    Ok(PendingElementAssembly { element_index, side, cell_cursor: 0, reclaim_lane: 0, complete: false, indices_new, positions: context.positions, stiffness: stiffness.data })
}

/// ❄️ Assembles mathematical stiffness contributions in deterministic element/cell order without native execution authority.
pub fn assemble_system(model: &AnalysisModel) -> Result<AssembledSystem, FemError> {
    let plan = AssemblyPlan::prepare(model)?;
    let mut full = Coo::new(plan.ndof);
    let mut free = Coo::new(plan.free_new.len());
    for element_index in 0..model.elements.len() {
        let contribution = element_contribution(&plan, model, element_index)?;
        for (cell, &value) in contribution.stiffness.iter().enumerate() {
            if value == 0.0 { continue; }
            let row = contribution.indices_new[cell / contribution.side];
            let column = contribution.indices_new[cell % contribution.side];
            full.add(row, column, value);
            if let (Some(row), Some(column)) = (plan.compact_of_new[row], plan.compact_of_new[column]) { free.add(row, column, value); }
        }
    }
    let k_full = full.to_csr();
    let k_factor = ldlt_factor(&free.to_csc_sym_upper()).map_err(|_| FemError::Singular)?;
    let AssemblyPlan { dof_map, inv_perm, ndof, free_new, compact_of_new, node_slot: _ } = plan;
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
