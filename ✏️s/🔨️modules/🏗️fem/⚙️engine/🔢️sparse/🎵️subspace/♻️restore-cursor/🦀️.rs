//! ♻️ Checkpoint restore cursor of the subspace job.

use super::*;

pub(super) fn restore_matrix_entry(matrix: &mut impl NumericalMatrixRestore, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor, context: &mut StepContext<'_>) -> Result<bool, NumericalCheckpointFault> {
    let install_shape = !cursor.page_open && page.owner == 0;
    let complete = restore_numerical_owner(matrix.owner(), page, maximum, entry, cursor, true, context)?;
    if install_shape && cursor.page_open { if let Some(shape) = cursor.shape { matrix.shape(shape.rows,shape.cols); } }
    Ok(complete)
}

fn decode_subspace_stage(value: u64) -> Result<SubspaceStage, NumericalCheckpointFault> {
    match value {
        0 => Ok(SubspaceStage::ReserveIteration),
        1 => Ok(SubspaceStage::ApplyOperatorColumnRow),
        2 => Ok(SubspaceStage::FactorForwardEntry),
        3 => Ok(SubspaceStage::FactorDiagonalEntry),
        4 => Ok(SubspaceStage::FactorBackwardEntry),
        5 => Ok(SubspaceStage::OrthogonalizePairElement),
        6 => Ok(SubspaceStage::NormalizeColumnElement),
        7 => Ok(SubspaceStage::ProjectedMatrixCellEntry),
        8 => Ok(SubspaceStage::JacobiFindPairCell),
        9 => Ok(SubspaceStage::JacobiRotateCell),
        10 => Ok(SubspaceStage::JacobiConvergenceCell),
        11 => Ok(SubspaceStage::ModeSortCompare),
        12 => Ok(SubspaceStage::ModePermuteElement),
        13 => Ok(SubspaceStage::ResidualColumnRow),
        14 => Ok(SubspaceStage::ConvergenceMode),
        15 => Ok(SubspaceStage::PublishIteration),
        16 => Ok(SubspaceStage::NormInverseInitialize),
        17 => Ok(SubspaceStage::NormInverseAccumulate),
        18 => Ok(SubspaceStage::NormRightRows),
        19 => Ok(SubspaceStage::ShiftOperatorColumnRow),
        20 => Ok(SubspaceStage::CertifyNonPositive),
        _ => Err(NumericalCheckpointFault::Field),
    }
}

fn apply_work_control(work: &mut SubspaceWork, values: &[u64; 24]) -> Result<(), NumericalCheckpointFault> {
    work.stage = decode_subspace_stage(values[0])?;
    work.reserve = values[1] as usize;
    work.first = values[2] as usize;
    work.second = values[3] as usize;
    work.third = values[4] as usize;
    work.phase = values[5] as usize;
    work.sweep = values[6] as usize;
    work.scalar = f64::from_bits(values[7]);
    work.coefficient = f64::from_bits(values[8]);
    work.cosine = f64::from_bits(values[9]);
    work.sine = f64::from_bits(values[10]);
    work.tangent = f64::from_bits(values[11]);
    work.close_lane = 0;
    Ok(())
}

pub(super) fn restore_work_entry(work: &mut SubspaceWork, page: &NumericalPageView<'_>, base: u16, n: usize, m: usize, entry: &mut usize, control: &mut [u64; 24], cursor: &mut NumericalOwnerRestoreCursor, context: &mut StepContext<'_>) -> Result<bool, NumericalCheckpointFault> {
    match page.field - base {
        0 => {
            let count = read_checkpoint_usize(page.bytes, 0)?;
            if count != 12 || page.bytes.len() != 8 + count * 8 {
                return Err(NumericalCheckpointFault::Truncated);
            }
            if !numerical_restore_control(context, *entry, count, 12 * 8, 1)? { return Ok(false); }
            if *entry == 0 {
                *entry = 1;
                return Ok(false);
            }
            let item = *entry - 1;
            if item < count {
                control[item] = read_checkpoint_u64(page.bytes, 8 + item * 8)?;
                *entry += 1;
                return Ok(false);
            }
            apply_work_control(work, control)?;
            Ok(true)
        }
        1 => restore_matrix_entry(&mut work.rhs, page, n.saturating_mul(m), entry, cursor, context),
        2 => restore_matrix_entry(&mut work.solved, page, n.saturating_mul(m), entry, cursor, context),
        3 => restore_matrix_entry(&mut work.b_basis, page, n.saturating_mul(m), entry, cursor, context),
        4 => restore_matrix_entry(&mut work.projected, page, m.saturating_mul(m), entry, cursor, context),
        5 => restore_matrix_entry(&mut work.jacobi, page, m.saturating_mul(m), entry, cursor, context),
        6 => restore_matrix_entry(&mut work.jacobi_vectors, page, m.saturating_mul(m), entry, cursor, context),
        7 => restore_matrix_entry(&mut work.ordered_vectors, page, m.saturating_mul(m), entry, cursor, context),
        8 => restore_matrix_entry(&mut work.candidate_x, page, n.saturating_mul(m), entry, cursor, context),
        9 => restore_paged_f64_entry(&mut work.mu, page, m, entry, cursor, context),
        10 => restore_paged_f64_entry(&mut work.theta, page, m, entry, cursor, context),
        11 => restore_numerical_owner(&mut work.order,page,m,entry,cursor,false, context),
        _ => Err(NumericalCheckpointFault::Field),
    }
}

pub struct SubspaceRestoreCursor {
    pub(super) operation: Operation,
    pub(super) payload: Option<RetainedJobPayload>,
    total_pages: usize,
    pub(super) page_slot: usize,
    close_due: bool,
    expected_field: u16,
    pub(super) page_entry: usize,
    owner_cursor: NumericalOwnerRestoreCursor,
    control: [u64; 24],
    pub(super) state: Option<SubspaceCheckpoint>,
    pub(super) fault: Option<NumericalCheckpointFault>,
}

impl SubspaceRestoreCursor {
    pub fn new(operation: Operation, payload: RetainedJobPayload) -> Self {
        let total_pages = payload.page_count();
        Self { operation, payload: Some(payload), total_pages, page_slot: 0, close_due: false, expected_field: 0, page_entry: 0, owner_cursor: NumericalOwnerRestoreCursor::default(), control: [0; 24], state: None, fault: None }
    }

    fn decode_page_entry(&mut self, bytes: &[u8], context: &mut StepContext<'_>) -> Result<bool, NumericalCheckpointFault> {
        let page = parse_numerical_page(bytes, b"FEMSCP1\0")?;
        if page.kind != 12 || page.field != self.expected_field || (self.owner_cursor.shape.is_none() && (page.owner != 0 || page.item != 0)) {
            return Err(NumericalCheckpointFault::Field);
        }
        if page.field == 0 {
            let count = declared_owner_length(&page, 24)?;
            if count != 23 || page.bytes.len() != 8 + count * 8 {
                return Err(NumericalCheckpointFault::Truncated);
            }
            if !numerical_restore_control(context, self.page_entry, count, size_of::<SubspaceCheckpoint>(), 1)? { return Ok(false); }
            if self.page_entry == 0 {
                self.page_entry = 1;
                return Ok(false);
            }
            let item = self.page_entry - 1;
            if item < count {
                self.control[item] = read_checkpoint_u64(page.bytes, 8 + item * 8)?;
                self.page_entry += 1;
                return Ok(false);
            }
            let value = |index| self.control[index];
            let identity = NumericalCheckpointIdentity { operation: value(0), revision: value(1), generation: value(2), seed: value(3) };
            if !identity.matches(self.operation) {
                return Err(NumericalCheckpointFault::Stale);
            }
            let n = value(4) as usize;
            let p = value(5) as usize;
            let m = value(7) as usize;
            if !SubspaceIterationJob::admits_shape(n,p) || m == 0 || m > n {
                return Err(NumericalCheckpointFault::Envelope);
            }
            self.state = Some(SubspaceCheckpoint {
                identity,
                k_factor: LdltFactor { n, columns: SparseColumns::default(), d: PagedList::empty() },
                b: Csr::from_owned_parts(n, Vec::new(), Vec::new(), Vec::new()),
                n,
                p,
                max_iter: value(6) as usize,
                m,
                shift:if value(20)==0{None}else{Some(f64::from_bits(value(21)))},
                eigensystem_certified:value(22)!=0,
                x: semio_s_fem_algebra::PagedMatD::empty(),
                prev_theta: PagedList::empty(),
                final_theta: PagedList::empty(),
                iteration: value(8) as usize,
                residuals: PagedList::empty(),
                converged_count: value(9) as usize,
                converged: value(10) != 0,
                checkpoint_due: false,
                preview_due: value(12) != 0,
                admission_fault: value(13) != 0,
                factor_validation_cursor: value(14) as usize,
                factor_validation_complete: value(15) != 0,
                initialization_cursor: value(16) as usize,
                publication_stage: value(17) as u8,
                publication_first: value(18) as usize,
                publication_second: value(19) as usize,
                work: SubspaceWork::empty(),
                retiring_work: None,
            });
            self.expected_field = 1;
            return Ok(true);
        }
        let state = self.state.as_mut().ok_or(NumericalCheckpointFault::Field)?;
        let n = state.n;
        let m = state.m;
        let complete = match page.field {
            1 => restore_paged_usize_entry(&mut state.k_factor.columns.offsets, &page, n + 1, &mut self.page_entry, &mut self.owner_cursor, context)?,
            2 => restore_paged_pair_entry(&mut state.k_factor.columns.entries, &page, n.saturating_mul(n), &mut self.page_entry, &mut self.owner_cursor, context)?,
            3 => restore_paged_f64_entry(&mut state.k_factor.d, &page, n, &mut self.page_entry, &mut self.owner_cursor, context)?,
            4 => restore_paged_u32_entry(&mut state.b.indptr, &page, n + 1, &mut self.page_entry, &mut self.owner_cursor, context)?,
            5 => restore_paged_u32_entry(&mut state.b.indices, &page, n.saturating_mul(n), &mut self.page_entry, &mut self.owner_cursor, context)?,
            6 => restore_paged_f64_entry(&mut state.b.vals, &page, n.saturating_mul(n), &mut self.page_entry, &mut self.owner_cursor, context)?,
            7 => restore_matrix_entry(&mut state.x, &page, n.saturating_mul(m), &mut self.page_entry, &mut self.owner_cursor, context)?,
            8 => restore_paged_f64_entry(&mut state.prev_theta, &page, m, &mut self.page_entry, &mut self.owner_cursor, context)?,
            9 => restore_paged_f64_entry(&mut state.final_theta, &page, m, &mut self.page_entry, &mut self.owner_cursor, context)?,
            10 => restore_paged_f64_entry(&mut state.residuals, &page, m, &mut self.page_entry, &mut self.owner_cursor, context)?,
            field if (11..=22).contains(&field) => restore_work_entry(&mut state.work, &page, 11, n, m, &mut self.page_entry, &mut self.control, &mut self.owner_cursor, context)?,            23 => {
                let present = read_checkpoint_u64(page.bytes, 0)?;
                let copy_bytes = if self.page_entry <= 1 { NUMERICAL_CHECKPOINT_HEADER_BYTES + 16 + size_of::<usize>() } else if self.page_entry < 14 { 16 + size_of::<usize>() } else { 12 * 8 + size_of::<SubspaceWork>() };
                if !numerical_restore_admit(context, RetirementDemand { copy_bytes, depth: 1, ..Default::default() })? { return Ok(false); }
                if self.page_entry == 0 {
                    self.page_entry = 1;
                    return Ok(false);
                }
                match present {
                    0 if page.bytes.len() == 8 => true,
                    1 => {
                        let count = read_checkpoint_usize(page.bytes, 8)?;
                        if count != 12 || page.bytes.len() != 16 + count * 8 {
                            return Err(NumericalCheckpointFault::Truncated);
                        }
                        if self.page_entry == 1 {
                            self.page_entry = 2;
                            return Ok(false);
                        }
                        let item = self.page_entry - 2;
                        if item < count {
                            self.control[item] = read_checkpoint_u64(page.bytes, 16 + item * 8)?;
                            self.page_entry += 1;
                            return Ok(false);
                        }
                        let mut work = SubspaceWork::empty();
                        apply_work_control(&mut work, &self.control)?;
                        state.retiring_work = Some(work);
                        true
                    }
                    _ => return Err(NumericalCheckpointFault::Field),
                }
            }
            field if (24..=34).contains(&field) => {
                let work = state.retiring_work.as_mut().ok_or(NumericalCheckpointFault::Field)?;
                restore_work_entry(work, &page, 23, n, m, &mut self.page_entry, &mut self.control, &mut self.owner_cursor, context)?
            }
            _ => return Err(NumericalCheckpointFault::Field),
        };
        if complete && self.owner_cursor.shape.is_none() {
            self.expected_field = match page.field {
                field if (1..=22).contains(&field) => field + 1,
                23 if state.retiring_work.is_some() => 24,
                23 => u16::MAX,
                field if (24..34).contains(&field) => field + 1,
                34 => u16::MAX,
                _ => return Err(NumericalCheckpointFault::Field),
            };
        }
        Ok(complete)
    }

    pub fn step(&mut self, context: &mut StepContext<'_>) -> Result<Option<SubspaceIterationJob>, NumericalCheckpointFault> {
        if let Some(fault) = self.fault { return Err(fault); }
        let outcome = self.advance(context);
        if let Err(fault) = outcome { self.fault = Some(fault); }
        outcome
    }

    fn advance(&mut self, context: &mut StepContext<'_>) -> Result<Option<SubspaceIterationJob>, NumericalCheckpointFault> {
        if context.is_cancelled() {
            return Err(NumericalCheckpointFault::Cancelled);
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return Err(NumericalCheckpointFault::Stale);
        }
        if context.should_yield() {
            return Ok(None);
        }
        context.consume_fuel(1);
        if self.payload.is_none() {
            if !numerical_restore_admit(context, RetirementDemand { copy_bytes: size_of::<SubspaceCheckpoint>() + size_of::<SubspaceIterationJob>(), depth: 1, ..Default::default() })? { return Ok(None); }
            let state = self.state.take().ok_or(NumericalCheckpointFault::Truncated)?;
            return Ok(Some(SubspaceIterationJob { operation: self.operation, state, preview_writer: None, preview_page_cursor: 0, terminal_writer: None, terminal_page_cursor: 0, checkpoint_writer: None, checkpoint_cursor: NumericalPageCursor::new(), terminal_published: false, desk: NumericalOutcomeDesk::default() }));
        }
        if self.close_due {
            let payload = self.payload.as_mut().ok_or(NumericalCheckpointFault::Truncated)?;
            if numerical_restore_close_due(payload, context, &mut self.page_slot, &mut self.page_entry, &mut self.close_due, self.total_pages)? {
                if self.expected_field != u16::MAX {
                    return Err(NumericalCheckpointFault::Truncated);
                }
                self.payload = None;
                return Ok(None);
            }
            return Ok(None);
        }
        let payload = self.payload.take().ok_or(NumericalCheckpointFault::Truncated)?;
        let decoded = payload.page(self.page_slot).ok_or(NumericalCheckpointFault::Truncated).and_then(|source| self.decode_page_entry(source, context));
        self.payload = Some(payload);
        match decoded {
            Ok(true) => self.close_due = true,
            Ok(false) => {}
            Err(fault) => return Err(fault),
        }
        Ok(None)
    }

    pub(super) fn close_frontier(&self) -> Result<Option<RetirementDemand>, ValueError> {
        if let Some(demand)=numerical_restore_frontier(&self.payload)?{return Ok(Some(demand))}
        self.state.as_ref().map(subspace_checkpoint_retirement_demands).transpose()
    }
    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        let demand = self.close_frontier()?.unwrap_or_else(|| numerical_rung_demand(self.terminal_is_empty()));
        numerical_close_original(cx, self, demand, Self::close_granted, Self::terminal_is_empty)
    }

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        let frontier = self.close_frontier()?;
        if self.terminal_is_empty() {
            return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
        }
        if !numerical_close_gate(grant, frontier) {
            return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
        }
        if let Some(step) = numerical_restore_payload_close(&mut self.payload, grant) {
            return step?.admit(grant, self.terminal_is_empty());
        }
        let Some(state) = self.state.as_mut() else { return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }) };
        let (terminal, items, bytes) = close_subspace_checkpoint_step(state, grant.maximum_release_bytes);
        let step = if terminal {
            self.state = None;
            InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }
        } else {
            InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: items, released_bytes: bytes, ..Default::default() } }
        };
        step.admit(grant, self.terminal_is_empty())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.payload.is_none() && self.state.is_none()
    }
}
