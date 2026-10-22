//! 🏃️ The subspace job run loop, its interactive step, close turns and the one-shot `subspace_iteration`.

use super::*;

// #region 🔖️SubspaceIteration
impl SubspaceIterationJob {
    fn run(&mut self, context: &mut StepContext<'_>) -> Result<NumericalRun, OriginalTurnFailure> {
        if context.is_cancelled() { return Ok(NumericalRun::Cancelled); }
        if context.should_yield() { return Ok(NumericalRun::Yield); }
        let result = self.run_admitted(context);
        context.consume_fuel(1);
        result
    }

    fn run_admitted(&mut self, context: &mut StepContext<'_>) -> Result<NumericalRun, OriginalTurnFailure> {
        if self.terminal_published { return Ok(NumericalRun::Yield); }
        if context.is_cancelled() {
            return Ok(NumericalRun::Cancelled);
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return Ok(NumericalRun::Fault);
        }
        if context.retained_grant().maximum_items==0{return Ok(NumericalRun::Yield)}
        if self.state.admission_fault {
            return Ok(NumericalRun::Fault);
        }
        if !self.state.factor_validation_complete{
            context.set_stage("fem.subspace.validate-factor-owner");if context.should_yield(){return Ok(NumericalRun::Yield)}
            let column=self.state.factor_validation_cursor;let entry=self.state.work.second;
            if column==self.state.n{self.state.factor_validation_complete=true;return Ok(NumericalRun::Yield)}
            let depth=match self.state.k_factor.column_entry_depth_demand(column,entry).and_then(|path|self.state.k_factor.diagonal_depth_demand(column).map(|diagonal|path.max(diagonal)+1)){Ok(depth)=>depth,Err(_)=>return Ok(NumericalRun::Fault)};
            if context.retained_grant().maximum_depth<depth{return Ok(NumericalRun::Yield)}
            if self.state.k_factor.diagonal(column).is_none_or(|value|!value.is_finite()||value<=0.0){return Ok(NumericalRun::Fault)}
            if let Some((row,value))=self.state.k_factor.column_entry(column,entry){if !value.is_finite()||row as usize<=column||row as usize>=self.state.n{return Ok(NumericalRun::Fault)}self.state.work.second+=1;}else{self.state.work.second=0;self.state.factor_validation_cursor+=1;}
            return Ok(NumericalRun::Yield);
        }
        if self.state.checkpoint_due || self.checkpoint_writer.is_some() {
            context.set_stage("fem.subspace.checkpoint-page");
            if self.checkpoint_writer.is_some() && self.checkpoint_cursor.field == u16::MAX {
                if !RetainedJobPayloadWriter::finish_funded(&mut self.checkpoint_writer, &mut self.desk.lent, context)? { return Ok(NumericalRun::Yield); }
                let payload = self.desk.lent.take().expect("finished numerical writer retained its original payload");
                return Ok(self.desk.lend_checkpoint(payload, self.state.iteration as u64));
            }

            if context.should_yield() {
                return Ok(NumericalRun::Yield);
            }
            if self.checkpoint_writer.is_none() {
                self.checkpoint_writer = RetainedJobPayloadWriter::new_funded(JobPayloadStream::CheckpointState, context)?;
                if self.checkpoint_writer.is_some() { self.state.checkpoint_due = false; self.checkpoint_cursor = NumericalPageCursor::new(); }
                return Ok(NumericalRun::Yield);
            }
            let Some(writer) = self.checkpoint_writer.as_mut() else {
                return Ok(NumericalRun::Fault);
            };
            if writer.staged_page_len().is_none() {
                writer.begin_staged_page_funded(context)?;
                return Ok(NumericalRun::Yield);
            }
            let complete = Self::advance_checkpoint_entry(&self.state, &mut self.checkpoint_cursor, writer, context)?;
            if !complete {
                return Ok(NumericalRun::Yield);
            }
            return Ok(NumericalRun::Yield);
        }
        if self.state.preview_due {
            context.set_stage("fem.subspace.preview-entry");
            if self.preview_writer.as_ref().is_some_and(|writer| writer.staged_page_len().is_none()) && self.state.publication_stage == 4 {
                if !RetainedJobPayloadWriter::finish_funded(&mut self.preview_writer, &mut self.desk.lent, context)? { return Ok(NumericalRun::Yield); }
                self.state.preview_due = false;
                let payload = self.desk.lent.take().expect("finished numerical writer retained its original payload");
                return Ok(self.desk.lend_preview(payload));
            }

            if context.should_yield() {
                return Ok(NumericalRun::Yield);
            }
            if self.preview_writer.is_none() {
                self.preview_writer = RetainedJobPayloadWriter::new_funded(JobPayloadStream::Preview, context)?;
                self.preview_page_cursor = 0;
                return Ok(NumericalRun::Yield);
            }
            let Some(writer) = self.preview_writer.as_mut() else {
                return Ok(NumericalRun::Fault);
            };
            if writer.staged_page_len().is_none() && self.state.publication_stage < 4 {
                writer.begin_staged_page_funded(context)?;
                return Ok(NumericalRun::Yield);
            }
            let complete = Self::advance_preview_entry(&mut self.state, self.operation, writer, context)?;
            if !complete {
                return Ok(NumericalRun::Yield);
            }
            return Ok(NumericalRun::Yield);
        }
        let boundary=self.state.work.stage==SubspaceStage::ReserveIteration&&self.state.work.reserve==0&&self.state.retiring_work.is_none();
        if boundary&&!self.state.converged&&(self.state.iteration>=self.state.max_iter||(self.state.iteration>0&&self.state.final_theta.len()<self.state.p&&self.state.m<self.state.n)){
            if self.state.m==self.state.n{return Ok(NumericalRun::Fault)}
            let copy=2*size_of::<semio_s_fem_algebra::PagedMatD>()+size_of::<PagedList<f64,SPARSE_INDEX_SPACE>>()+size_of::<SubspaceWork>();let grant=context.retained_grant();
            if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(NumericalRun::Yield)}
            let mut displaced=SubspaceWork::empty();
            displaced.candidate_x=std::mem::replace(&mut self.state.x,semio_s_fem_algebra::PagedMatD::empty());
            displaced.theta=std::mem::replace(&mut self.state.final_theta,PagedList::empty());
            self.state.retiring_work=Some(displaced);
            self.state.m=self.state.m.saturating_mul(2).min(self.state.n);
            self.state.iteration=0;self.state.initialization_cursor=0;
            context.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;
            return Ok(NumericalRun::Yield);
        }
        if self.state.iteration >= self.state.max_iter || self.state.converged {
            context.set_stage("fem.subspace.terminal-page");
            if self.terminal_writer.as_ref().is_some_and(|writer| writer.staged_page_len().is_none()) && self.state.publication_stage == 4 {
                let mut demand = RetainedJobPayloadWriter::finish_demand();
                demand.copy_bytes = demand.copy_bytes.checked_add(size_of::<bool>()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "numerical terminal metadata overflow"))?;
                if !semio_s_fem_sparse::numerical_grant_covers(context.retained_grant(), demand) { return Ok(NumericalRun::Yield); }
                if !RetainedJobPayloadWriter::finish_funded(&mut self.terminal_writer, &mut self.desk.lent, context)? { return Ok(NumericalRun::Yield); }
                self.terminal_published = true;
                context.consume_retained(RetainedCloneProgress { copied_bytes: size_of::<bool>(), ..Default::default() })?;
                let payload = self.desk.lent.take().expect("finished numerical writer retained its original payload");
                return Ok(self.desk.lend_complete(payload));
            }

            if context.should_yield() {
                return Ok(NumericalRun::Yield);
            }
            if self.terminal_writer.is_none() {
                self.terminal_writer = RetainedJobPayloadWriter::new_funded(JobPayloadStream::CommitOutput, context)?;
                if self.terminal_writer.is_some() { self.state.publication_stage = 0; self.state.publication_first = 0; self.state.publication_second = 0; }
                self.terminal_page_cursor = 0;
                return Ok(NumericalRun::Yield);
            }
            let Some(writer) = self.terminal_writer.as_mut() else {
                return Ok(NumericalRun::Fault);
            };
            if writer.staged_page_len().is_none() && self.state.publication_stage < 4 {
                writer.begin_staged_page_funded(context)?;
                return Ok(NumericalRun::Yield);
            }
            let complete = Self::advance_preview_entry(&mut self.state, self.operation, writer, context)?;
            if !complete {
                return Ok(NumericalRun::Yield);
            }
            return Ok(NumericalRun::Yield);
        }
        context.set_stage(match self.state.work.stage {
            SubspaceStage::ReserveIteration => "fem.subspace.reserve-iteration",
            SubspaceStage::ApplyOperatorColumnRow => "fem.subspace.apply-operator-column-row",
            SubspaceStage::FactorForwardEntry => "fem.subspace.factor-forward-entry",
            SubspaceStage::FactorDiagonalEntry => "fem.subspace.factor-diagonal-entry",
            SubspaceStage::FactorBackwardEntry => "fem.subspace.factor-backward-entry",
            SubspaceStage::OrthogonalizePairElement => "fem.subspace.orthogonalize-pair-element",
            SubspaceStage::NormalizeColumnElement => "fem.subspace.normalize-column-element",
            SubspaceStage::ProjectedMatrixCellEntry => "fem.subspace.projected-matrix-cell-entry",
            SubspaceStage::JacobiFindPairCell => "fem.subspace.jacobi-find-pair-cell",
            SubspaceStage::JacobiRotateCell => "fem.subspace.jacobi-rotate-cell",
            SubspaceStage::JacobiConvergenceCell => "fem.subspace.jacobi-convergence-cell",
            SubspaceStage::ModeSortCompare => "fem.subspace.mode-sort-compare",
            SubspaceStage::ModePermuteElement => "fem.subspace.mode-permute-element",
            SubspaceStage::ResidualColumnRow => "fem.subspace.residual-column-row",
            SubspaceStage::ConvergenceMode => "fem.subspace.convergence-mode",
            SubspaceStage::PublishIteration => "fem.subspace.publish-iteration",
            SubspaceStage::NormInverseInitialize=>"fem.subspace.norm-inverse-initialize",
            SubspaceStage::NormInverseAccumulate=>"fem.subspace.norm-inverse-accumulate",
            SubspaceStage::NormRightRows=>"fem.subspace.norm-right-rows",
            SubspaceStage::ShiftOperatorColumnRow=>"fem.subspace.shift-operator-column-row",
            SubspaceStage::CertifyNonPositive=>"fem.subspace.certify-non-positive",
        });
        if context.should_yield() {
            return Ok(NumericalRun::Yield);
        }
        let arithmetic=!matches!(self.state.work.stage,SubspaceStage::ReserveIteration|SubspaceStage::PublishIteration);
        if arithmetic{
            let depth=match self.arithmetic_depth_demand(){Ok(depth)=>depth,Err(_)=>return Ok(NumericalRun::Fault)};
            let grant=context.retained_grant();
            if grant.maximum_depth<depth||grant.maximum_copy_bytes<2*size_of::<f64>(){return Ok(NumericalRun::Yield)}
        }
        let result = match self.state.work.stage {
            SubspaceStage::ReserveIteration => self.reserve_iteration_owner(context),
            SubspaceStage::ApplyOperatorColumnRow => {
                self.state.advance_apply_operator();
                Ok(())
            }
            SubspaceStage::FactorForwardEntry => {
                self.state.advance_factor_forward();
                Ok(())
            }
            SubspaceStage::FactorDiagonalEntry => {
                self.state.advance_factor_diagonal();
                Ok(())
            }
            SubspaceStage::FactorBackwardEntry => {
                self.state.advance_factor_backward();
                Ok(())
            }
            SubspaceStage::OrthogonalizePairElement => {
                self.state.advance_orthogonalize();
                Ok(())
            }
            SubspaceStage::NormalizeColumnElement => {
                self.state.advance_normalize()
            }
            SubspaceStage::ProjectedMatrixCellEntry => {
                self.state.advance_projected();
                Ok(())
            }
            SubspaceStage::JacobiFindPairCell => {
                self.state.advance_jacobi_pair();
                Ok(())
            }
            SubspaceStage::JacobiRotateCell => {
                self.state.advance_jacobi_rotation();
                Ok(())
            }
            SubspaceStage::JacobiConvergenceCell => {
                self.state.advance_jacobi_convergence();
                Ok(())
            }
            SubspaceStage::ModeSortCompare => {
                self.state.advance_mode_sort()
            }
            SubspaceStage::ModePermuteElement => {
                self.state.advance_mode_permute()
            }
            SubspaceStage::ResidualColumnRow => {
                self.state.advance_residual();
                Ok(())
            }
            SubspaceStage::ConvergenceMode => {
                self.state.advance_convergence();
                Ok(())
            }
            SubspaceStage::PublishIteration => self.publish_iteration(context),
            SubspaceStage::NormInverseInitialize=>{self.state.advance_inverse_norm_initialize();Ok(())},
            SubspaceStage::NormInverseAccumulate=>{self.state.advance_inverse_norm_accumulate();Ok(())},
            SubspaceStage::NormRightRows=>self.state.advance_right_norm(),
            SubspaceStage::ShiftOperatorColumnRow=>{self.state.advance_shift_operator();Ok(())},
            SubspaceStage::CertifyNonPositive=>self.state.advance_non_positive_certificate(),
        };
        if result.is_err() {
            return Ok(NumericalRun::Fault);
        }
        if arithmetic&&context.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<f64>(),..Default::default()}).is_err(){return Ok(NumericalRun::Fault)}        if context.is_cancelled() {
            return Ok(NumericalRun::Cancelled);
        }
        Ok(NumericalRun::Yield)
    }



    fn rung_demand(&self)->Result<RetirementDemand,ValueError>{subspace_checkpoint_retirement_demands(&self.state)}

    fn close_frontier(&self) -> Result<Option<RetirementDemand>, ValueError> {
        if let Some(demand) = self.desk.retirement_demands() {
            return demand.map(Some);
        }
        for slot in [&self.checkpoint_writer, &self.preview_writer, &self.terminal_writer] {
            if let Some(demand) = numerical_writer_demand(slot) {
                return demand.map(Some);
            }
        }
        Ok(None)
    }

    fn close_demand(&self) -> Result<RetirementDemand, ValueError> {
        match self.close_frontier()?{Some(demand)=>Ok(demand),None=>self.rung_demand()}
    }

    fn close_turn(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        if let Some(step) = self.desk.close_step(grant) {
            return step;
        }
        if let Some(step) = numerical_writer_close(&mut self.checkpoint_writer, grant) {
            return step;
        }
        if let Some(step) = numerical_writer_close(&mut self.preview_writer, grant) {
            return step;
        }
        if let Some(step) = numerical_writer_close(&mut self.terminal_writer, grant) {
            return step;
        }
        Ok(numerical_ladder_turn(self.close_retained_step(grant.maximum_release_bytes)))
    }
}

impl InteractiveJob for SubspaceIterationJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        if self.desk.retire_due(cx, self.operation) {
            self.desk.retire_delivered(cx)?;
            return Ok(None);
        }
        let run = match self.desk.take_pending() {
            Some(run) => run,
            None => self.run(cx)?,
        };
        Ok(self.desk.admit(cx, run)?)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        self.desk.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if let Some(writer) = self.checkpoint_writer.as_mut() {
            writer.begin_close();
        }
        if let Some(writer) = self.preview_writer.as_mut() {
            writer.begin_close();
        }
        if let Some(writer) = self.terminal_writer.as_mut() {
            writer.begin_close();
        }
    }

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
        self.checkpoint_writer.is_none() && self.preview_writer.is_none() && self.terminal_writer.is_none() && self.desk.terminal_is_empty() && self.close_terminal_is_empty()
    }
}

impl SubspaceIterationJob {
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
        self.close_turn(grant)?.admit(grant, self.terminal_is_empty())
    }
}

/// ❄️ Finds generalized positive eigenpairs through the shared mathematical state without native authority.
pub fn subspace_iteration(k_factor: &LdltFactor, b: &Csr, n: usize, p: usize, max_iter: usize) -> EigenPairs {
    let operation = Operation::new(semio_framework_job::OperationId(0), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), 0);
    let mut state = SubspaceCheckpoint::new(operation, k_factor.clone(), b.clone(), n, p, max_iter);
    assert!(!state.admission_fault, "subspace mathematical shape refused");
    for column in 0..n {
        assert!(state.k_factor.diagonal(column).is_some_and(|value| value.is_finite() && value > 0.0), "subspace elastic diagonal refused");
        for entry in 0..state.k_factor.column_entries_len(column).expect("subspace factor column") {
            let (row, value) = state.k_factor.column_entry(column, entry).expect("subspace factor entry");
            assert!(value.is_finite() && row as usize > column && (row as usize) < n, "subspace elastic column refused");
        }
    }
    while !state.converged {
        if state.iteration >= state.max_iter || state.iteration > 0 && state.final_theta.len() < state.p && state.m < state.n {
            assert!(state.m < state.n, "subspace mathematical convergence exhausted");
            state.m = state.m.saturating_mul(2).min(state.n);
            state.iteration = 0;
            state.initialization_cursor = 0;
        }
        state.advance_mathematical().expect("subspace mathematical transition refused");
    }
    EigenPairs { values: state.final_theta.iter().take(p).copied().collect(), vectors: (0..p.min(state.final_theta.len()).min(state.x.cols)).map(|column| mat_col(&state.x, column)).collect() }
}
// #endregion 🔖️SubspaceIteration
