//! 🪜️ PCG statistics, checkpoint and publication state, and the checkpointed job with its bounded steps.

use super::*;

// #region 🔖️Pcg
/// 📈️ Convergence outcome of a `pcg` call.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PcgStats {
    pub iterations: usize,
    pub residual_norm: f64,
    pub converged: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[value(tag = "kind")]
pub enum PcgStage {
    InitializeDiagonal,
    InitialSpmv,
    InitialResidual,
    InitialPrecondition,
    IterationSpmv,
    IterationUpdate,
    IterationPrecondition,
    IterationDirection,
    Complete,
}

pub(super) struct PcgCheckpoint {
    pub(super) a: Csr,
    pub(super) b: VecD,
    pub(super) x: VecD,
    pub(super) tol_rel: f64,
    pub(super) max_iter: usize,
    pub(super) batch_units: usize,
    pub(super) stage: PcgStage,
    pub(super) diag: VecD,
    pub(super) r: VecD,
    pub(super) z: VecD,
    pub(super) p: VecD,
    pub(super) ap: VecD,
    pub(super) b_norm: f64,
    pub(super) residual_norm: f64,
    pub(super) residual_sq: f64,
    pub(super) rz_old: f64,
    pub(super) rz_new: f64,
    pub(super) dot_accum: f64,
    pub(super) alpha: f64,
    pub(super) beta: f64,
    pub(super) iteration: usize,
    pub(super) cursor: usize,
    pub(super) row_cursor: usize,
    pub(super) entry_cursor: usize,
    pub(super) row_sum: f64,
    pub(super) converged: bool,
    pub(super) coarse_published: bool,
    pub(super) preview_due: bool,
    pub(super) checkpoint_due: bool,
}

pub(super) const PCG_SCALAR_BACKING_BYTES: usize = SPARSE_PAGE_BYTES;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum PcgPublicationKind {
    Checkpoint,
    Preview,
    Complete,
}

pub(super) struct PcgPublication {
    pub(super) kind: PcgPublicationKind,
    pub(super) cursor: NumericalPageCursor,
    pub(super) writer: RetainedJobPayloadWriter,
    pub(super) complete: bool,
}

impl PcgPublication {
    pub(super) fn new(kind: PcgPublicationKind) -> Self {
        let stream = match kind {
            PcgPublicationKind::Checkpoint => JobPayloadStream::CheckpointState,
            PcgPublicationKind::Preview => JobPayloadStream::Preview,
            PcgPublicationKind::Complete => JobPayloadStream::CommitOutput,
        };
        Self { kind, cursor: NumericalPageCursor::new(), writer: RetainedJobPayloadWriter::new(stream), complete: false }
    }

    pub(super) fn advance(&mut self, operation: Operation, state: &PcgCheckpoint, context: &mut StepContext<'_>) -> Result<bool, ValueError> {
        let (magic, kind, fields) = match self.kind {
            PcgPublicationKind::Checkpoint => (b"FEMPCP1\0", 13, 11),
            _ => (b"FEMPCG1\0", 3, 5),
        };
        if advance_numerical_page_header(&mut self.writer, magic, kind, self.cursor, context)? { return Ok(false); }
        let complete = if self.kind == PcgPublicationKind::Checkpoint {
            match self.cursor.field {
                0 => advance_u64_values(&mut self.writer, &state.checkpoint_control(operation), &mut self.cursor, context)?,
                1 => advance_paged_u32_owner(&mut self.writer, &state.a.indptr, &mut self.cursor, context)?,
                2 => advance_paged_u32_owner(&mut self.writer, &state.a.indices, &mut self.cursor, context)?,
                3 => advance_paged_f64_owner(&mut self.writer, &state.a.vals, &mut self.cursor, context)?,
                4 => advance_f64_owner(&mut self.writer, &state.b.0, &mut self.cursor, context)?,
                5 => advance_f64_owner(&mut self.writer, &state.x.0, &mut self.cursor, context)?,
                6 => advance_f64_owner(&mut self.writer, &state.diag.0, &mut self.cursor, context)?,
                7 => advance_f64_owner(&mut self.writer, &state.r.0, &mut self.cursor, context)?,
                8 => advance_f64_owner(&mut self.writer, &state.z.0, &mut self.cursor, context)?,
                9 => advance_f64_owner(&mut self.writer, &state.p.0, &mut self.cursor, context)?,
                10 => advance_f64_owner(&mut self.writer, &state.ap.0, &mut self.cursor, context)?,
                _ => true,
            }
        } else if self.cursor.field == 0 {
            advance_u64_values(&mut self.writer, &state.output_control(operation), &mut self.cursor, context)?
        } else {
            let values = if self.cursor.field == 2 || self.cursor.field == 3 { &state.r.0 } else { &state.x.0 };
            if !advance_owner_length(&mut self.writer, values.len(), &mut self.cursor, context)? { return Ok(false); }
            if let Some(value) = values.get(self.cursor.item) {
                let value = match self.cursor.field { 3 => -*value, 4 => value.abs(), _ => *value };
                if write_numerical_entry(&mut self.writer, &value.to_bits().to_le_bytes(), context)? { self.cursor.item += 1; }
                false
            } else {
                true
            }
        };
        if complete {
            if !self.writer.commit_staged_page_funded(context, size_of::<NumericalPageCursor>() + size_of::<bool>())? { return Ok(false); }
            self.cursor.field += 1;
            self.cursor.owner = 0;
            self.cursor.item = 0;
        }
        Ok(self.cursor.field == fields)
    }
}

impl PcgCheckpoint {
    pub(super) fn checkpoint_control(&self, operation: Operation) -> [u64; 26] {
        [
            operation.operation.0, operation.base_revision.0, operation.generation.0, operation.seed,
            self.a.n as u64, self.tol_rel.to_bits(), self.max_iter as u64, self.batch_units as u64,
            self.stage as u64, self.b_norm.to_bits(), self.residual_norm.to_bits(), self.residual_sq.to_bits(),
            self.rz_old.to_bits(), self.rz_new.to_bits(), self.dot_accum.to_bits(), self.alpha.to_bits(),
            self.beta.to_bits(), self.iteration as u64, self.cursor as u64, self.row_cursor as u64,
            self.entry_cursor as u64, self.row_sum.to_bits(), self.converged as u64, self.coarse_published as u64,
            self.preview_due as u64, 0,
        ]
    }

    fn output_control(&self, operation: Operation) -> [u64; 10] {
        [
            operation.operation.0, operation.base_revision.0, operation.generation.0, operation.seed,
            self.a.n as u64, self.stage as u64,
            if self.converged { 2 } else if self.coarse_published { 1 } else { 0 },
            self.iteration as u64, self.residual_norm.to_bits(), self.converged as u64,
        ]
    }
}

pub struct PcgJob {
    pub(super) operation: Operation,
    pub(super) state: PcgCheckpoint,
    pub(super) close_lane: u8,
    pub(super) publication: Option<PcgPublication>,
    pub(super) publication_fault: Option<NumericalCheckpointFault>,
    pub(super) terminal_published: bool,
    pub(super) closing: bool,
    pub(super) desk: NumericalOutcomeDesk,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PcgVisualScalar {
    pub displacement: f64,
    pub residual: f64,
    pub reaction: f64,
    pub contour: f64,
    pub mode_estimate: f64,
}

impl PcgJob {
    pub(super) fn from_state(operation: Operation, state: PcgCheckpoint) -> Self {
        Self { operation, state, close_lane: 0, publication: None, publication_fault: None, terminal_published: false, closing: false, desk: NumericalOutcomeDesk::default() }
    }

    pub(super) fn step_publication(&mut self, context: &mut StepContext<'_>, kind: PcgPublicationKind) -> Result<NumericalRun, ValueError> {
        if context.should_yield() { return Ok(NumericalRun::Yield); }
        let result = self.advance_publication(context, kind);
        context.consume_fuel(1);
        result
    }

    fn advance_publication(&mut self, context: &mut StepContext<'_>, kind: PcgPublicationKind) -> Result<NumericalRun, ValueError> {
        if self.publication.is_none() {
            let copy_bytes = 2 * size_of::<Option<PcgPublication>>();
            let demand = RetirementDemand { copy_bytes, depth: 1, ..Default::default() };
            if !semio_s_fem_sparse::numerical_grant_covers(context.retained_grant(), demand) { return Ok(NumericalRun::Yield); }
            self.publication = Some(PcgPublication::new(kind));
            context.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() })?;
            return Ok(NumericalRun::Yield);
        }
        let publication = self.publication.as_mut().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "PCG publication lost its original frame"))?;
        if !publication.complete {
            if publication.writer.staged_page_len().is_none() {
                publication.writer.begin_staged_page_funded(context)?;
                return Ok(NumericalRun::Yield);
            }
            if publication.advance(self.operation, &self.state, context)? { publication.complete = true; }
            return Ok(NumericalRun::Yield);
        }
        if !publication.writer.take_finished_funded(&mut self.desk.lent, context, 2 * size_of::<Option<PcgPublication>>() + 3 * size_of::<bool>())? { return Ok(NumericalRun::Yield); }
        let kind = publication.kind;
        self.publication = None;
        let payload = self.desk.lent.take().expect("finished PCG publication retained its actual payload");
        Ok(match kind {
            PcgPublicationKind::Checkpoint => { self.state.checkpoint_due = false; self.desk.lend_checkpoint(payload, self.state.iteration as u64) }
            PcgPublicationKind::Preview => { self.state.preview_due = false; self.desk.lend_preview(payload) }
            PcgPublicationKind::Complete => { self.terminal_published = true; self.desk.lend_complete(payload) }
        })
    }

    pub fn new(operation: Operation, a: Csr, b: VecD, x: VecD, tol_rel: f64, max_iter: usize, batch_units: usize) -> Self {
        Self {
            operation,
            state: PcgCheckpoint::new(a, b, x, tol_rel, max_iter, batch_units),
            close_lane: 0,
            publication: None,
            publication_fault: None,
            terminal_published: false,
            closing: false,
            desk: NumericalOutcomeDesk::default(),
        }
    }

    pub fn solution(&self) -> (&VecD, PcgStats) {
        (&self.state.x, PcgStats { iterations: self.state.iteration, residual_norm: self.state.residual_norm, converged: self.state.converged })
    }

    /// 👁️ Borrows one numerical-to-visual scalar without cloning a solver vector.
    pub fn visual_scalar(&self, index: usize) -> Option<PcgVisualScalar> {
        let displacement = *self.state.x.0.get(index)?;
        let residual = *self.state.r.0.get(index)?;
        Some(PcgVisualScalar { displacement, residual, reaction: -residual, contour: displacement.abs(), mode_estimate: displacement })
    }

    /// 📈️ Exposes generation-local convergence scalars for one retained visual page.
    pub fn visual_progress(&self) -> (usize, usize, f64, f64, bool) {
        (self.state.iteration, self.state.max_iter, self.state.residual_norm, self.state.tol_rel, self.state.converged)
    }

    /// 🎼 Transfers the converged stiffness owner into the retained modal child without cloning.
    pub fn take_completed_matrix(&mut self) -> Option<Csr> {
        (self.state.stage == PcgStage::Complete).then(|| std::mem::replace(&mut self.state.a, Csr::from_owned_parts(0, Vec::new(), Vec::new(), Vec::new())))
    }

    /// 🧹️ Retires one matrix/vector scalar owner per governed close opportunity.
    pub(super) fn close_retained_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        self.closing = true;
        loop {
            if self.close_lane == 0 {
                let (terminal, items, bytes) = self.state.a.close_step(maximum_bytes);
                if !terminal {
                    return (false, items, bytes);
                }
                self.close_lane = 1;
                continue;
            }
            let step = match self.close_lane {
                1 => close_vec_owner_step(&mut self.state.b.0, maximum_bytes),
                2 => close_vec_owner_step(&mut self.state.x.0, maximum_bytes),
                3 => close_vec_owner_step(&mut self.state.diag.0, maximum_bytes),
                4 => close_vec_owner_step(&mut self.state.r.0, maximum_bytes),
                5 => close_vec_owner_step(&mut self.state.z.0, maximum_bytes),
                6 => close_vec_owner_step(&mut self.state.p.0, maximum_bytes),
                7 => close_vec_owner_step(&mut self.state.ap.0, maximum_bytes),
                _ => return (true, 0, 0),
            };
            match step {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Ok(None) => self.close_lane += 1,
                Err(()) => return (false, 0, 0),
            }
        }
    }















}
// #endregion 🔖️Pcg

impl PcgCheckpoint {
    pub(super) fn new(a: Csr, b: VecD, x: VecD, tol_rel: f64, max_iter: usize, batch_units: usize) -> Self {
        assert_eq!(a.n, b.len(), "pcg rhs dimension mismatch");
        assert_eq!(a.n, x.len(), "pcg initial guess dimension mismatch");
        assert!(batch_units > 0, "pcg batch must contain work");
        let n = a.n;
        let b_norm = b.norm2().max(1e-300);
        Self {
                a,
                b,
                x,
                tol_rel,
                max_iter,
                batch_units,
                stage: PcgStage::InitializeDiagonal,
                diag: VecD::zeros(n),
                r: VecD::zeros(n),
                z: VecD::zeros(n),
                p: VecD::zeros(n),
                ap: VecD::zeros(n),
                b_norm,
                residual_norm: 0.0,
                residual_sq: 0.0,
                rz_old: 0.0,
                rz_new: 0.0,
                dot_accum: 0.0,
                alpha: 0.0,
                beta: 0.0,
                iteration: 0,
                cursor: 0,
                row_cursor: 0,
                entry_cursor: 0,
                row_sum: 0.0,
                converged: false,
                coarse_published: false,
                preview_due: false,
                checkpoint_due: false,
        }
    }

    pub(super) fn reset_spmv(&mut self, stage: PcgStage) {
        self.stage = stage;
        self.row_cursor = 0;
        self.entry_cursor = self.a.indptr(0).unwrap_or(0) as usize;
        self.row_sum = 0.0;
        self.dot_accum = 0.0;
    }

    pub(super) fn finish(&mut self, converged: bool) {
        self.converged = converged;
        self.stage = PcgStage::Complete;
    }

    pub(super) fn step_diagonal(&mut self, units: &mut usize, maximum_units: usize) {
        while self.row_cursor < self.a.n && *units < maximum_units {
            let row = self.row_cursor;
            let end = self.a.indptr(row + 1).expect("PCG diagonal row end") as usize;
            if self.entry_cursor < end {
                let entry = self.entry_cursor;
                if self.a.index(entry).expect("PCG diagonal column") as usize == row {
                    self.diag.set(row, self.a.value(entry).expect("PCG diagonal value"));
                }
                self.entry_cursor += 1;
            } else {
                self.row_cursor += 1;
                self.entry_cursor = self.a.indptr(self.row_cursor.min(self.a.n)).expect("PCG diagonal next row") as usize;
            }
            *units += 1;
        }
        if self.row_cursor == self.a.n {
            self.reset_spmv(PcgStage::InitialSpmv);
        }
    }

    pub(super) fn step_spmv(&mut self, units: &mut usize, maximum_units: usize, initial: bool) {
        while self.row_cursor < self.a.n && *units < maximum_units {
            let row = self.row_cursor;
            let end = self.a.indptr(row + 1).expect("PCG SpMV row end") as usize;
            if self.entry_cursor < end {
                let entry = self.entry_cursor;
                let col = self.a.index(entry).expect("PCG SpMV column") as usize;
                let value = if initial { self.x.get(col) } else { self.p.get(col) };
                self.row_sum += self.a.value(entry).expect("PCG SpMV value") * value;
                self.entry_cursor += 1;
            } else {
                self.ap.set(row, self.row_sum);
                if !initial {
                    self.dot_accum += self.p.get(row) * self.row_sum;
                }
                self.row_sum = 0.0;
                self.row_cursor += 1;
                self.entry_cursor = self.a.indptr(self.row_cursor.min(self.a.n)).expect("PCG SpMV next row") as usize;
            }
            *units += 1;
        }
        if self.row_cursor == self.a.n {
            self.cursor = 0;
            if initial {
                self.residual_sq = 0.0;
                self.stage = PcgStage::InitialResidual;
            } else if self.dot_accum.abs() < 1e-300 {
                self.finish(false);
            } else {
                self.alpha = self.rz_old / self.dot_accum;
                self.residual_sq = 0.0;
                self.stage = PcgStage::IterationUpdate;
            }
        }
    }

    pub(super) fn step_initial_residual(&mut self, units: &mut usize, maximum_units: usize) {
        while self.cursor < self.a.n && *units < maximum_units {
            let i = self.cursor;
            let value = self.b.get(i) - self.ap.get(i);
            self.r.set(i, value);
            self.residual_sq += value * value;
            self.cursor += 1;
            *units += 1;
        }
        if self.cursor == self.a.n {
            self.residual_norm = self.residual_sq.sqrt() / self.b_norm;
            if self.residual_norm < self.tol_rel {
                self.finish(true);
            } else if self.max_iter == 0 {
                self.finish(false);
            } else {
                self.cursor = 0;
                self.rz_new = 0.0;
                self.stage = PcgStage::InitialPrecondition;
            }
        }
    }

    pub(super) fn step_precondition(&mut self, units: &mut usize, maximum_units: usize, initial: bool) {
        while self.cursor < self.a.n && *units < maximum_units {
            let i = self.cursor;
            let d = self.diag.get(i);
            let value = if d.abs() > 1e-300 { self.r.get(i) / d } else { self.r.get(i) };
            self.z.set(i, value);
            if initial {
                self.p.set(i, value);
            }
            self.rz_new += self.r.get(i) * value;
            self.cursor += 1;
            *units += 1;
        }
        if self.cursor == self.a.n {
            self.cursor = 0;
            if initial {
                self.rz_old = self.rz_new;
                self.iteration = 1;
                self.reset_spmv(PcgStage::IterationSpmv);
            } else {
                self.beta = self.rz_new / self.rz_old;
                self.stage = PcgStage::IterationDirection;
            }
        }
    }

    pub(super) fn step_update(&mut self, units: &mut usize, maximum_units: usize) {
        while self.cursor < self.a.n && *units < maximum_units {
            let i = self.cursor;
            self.x.set(i, self.x.get(i) + self.alpha * self.p.get(i));
            self.r.set(i, self.r.get(i) - self.alpha * self.ap.get(i));
            self.residual_sq += self.r.get(i) * self.r.get(i);
            self.cursor += 1;
            *units += 1;
        }
        if self.cursor == self.a.n {
            self.residual_norm = self.residual_sq.sqrt() / self.b_norm;
            if self.residual_norm < self.tol_rel {
                self.finish(true);
            } else if self.iteration >= self.max_iter {
                self.finish(false);
            } else {
                if !self.coarse_published && self.residual_norm < self.tol_rel.max(1e-3) {
                    self.coarse_published = true;
                    self.preview_due = true;
                }
                self.cursor = 0;
                self.rz_new = 0.0;
                self.stage = PcgStage::IterationPrecondition;
            }
            self.checkpoint_due = true;
        }
    }

    pub(super) fn step_direction(&mut self, units: &mut usize, maximum_units: usize) {
        while self.cursor < self.a.n && *units < maximum_units {
            let i = self.cursor;
            self.p.set(i, self.z.get(i) + self.beta * self.p.get(i));
            self.cursor += 1;
            *units += 1;
        }
        if self.cursor == self.a.n {
            self.rz_old = self.rz_new;
            self.iteration += 1;
            self.reset_spmv(PcgStage::IterationSpmv);
        }
    }

    pub(super) fn advance_one(&mut self) -> usize {
        let mut units = 0;
        match self.stage {
            PcgStage::InitializeDiagonal => self.step_diagonal(&mut units, 1),
            PcgStage::InitialSpmv => self.step_spmv(&mut units, 1, true),
            PcgStage::InitialResidual => self.step_initial_residual(&mut units, 1),
            PcgStage::InitialPrecondition => self.step_precondition(&mut units, 1, true),
            PcgStage::IterationSpmv => self.step_spmv(&mut units, 1, false),
            PcgStage::IterationUpdate => self.step_update(&mut units, 1),
            PcgStage::IterationPrecondition => self.step_precondition(&mut units, 1, false),
            PcgStage::IterationDirection => self.step_direction(&mut units, 1),
            PcgStage::Complete => {}
        }
        units
    }
}
