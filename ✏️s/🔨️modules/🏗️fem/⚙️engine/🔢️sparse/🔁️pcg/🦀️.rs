//! 🔁️ Jacobi-preconditioned conjugate-gradient iterative solver: construction, checkpointed job and restore.

use replication::value::list::PagedList;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, OriginalTurnFailure, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeView, JobPayloadAdmissionFault, JobPayloadStream, Operation, RetainedCloneGrant, RetainedCloneProgress, RetainedJobPayload, RetainedJobPayloadWriter, StepContext};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_fem_sparse::{numerical_close_original, Csr, MountedScalarSlots, NumericalBatchEnd, NumericalCheckpointFault, NumericalCheckpointIdentity, NumericalOutcomeDesk, NumericalOwnerRestoreCursor, NumericalPageCursor, NumericalRun, SPARSE_PAGE_BYTES, advance_f64_owner, advance_numerical_page_header, advance_owner_length, advance_paged_f64_owner, advance_paged_u32_owner, advance_u64_values, close_vec_owner_step, declared_owner_length, numerical_batch_close, numerical_batch_step, numerical_child_close, numerical_close_gate, numerical_close_result, numerical_ladder_turn, numerical_owner_turn, numerical_restore_close_due, numerical_restore_frontier, numerical_restore_payload_close, numerical_rung_demand, parse_numerical_page, read_checkpoint_u64, restore_f64_entry, restore_paged_f64_entry, restore_paged_u32_entry, write_numerical_entry};
use semio_s_fem_algebra::{VecD};

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

struct PcgCheckpoint {
    a: Csr,
    b: VecD,
    x: VecD,
    tol_rel: f64,
    max_iter: usize,
    batch_units: usize,
    stage: PcgStage,
    diag: VecD,
    r: VecD,
    z: VecD,
    p: VecD,
    ap: VecD,
    b_norm: f64,
    residual_norm: f64,
    residual_sq: f64,
    rz_old: f64,
    rz_new: f64,
    dot_accum: f64,
    alpha: f64,
    beta: f64,
    iteration: usize,
    cursor: usize,
    row_cursor: usize,
    entry_cursor: usize,
    row_sum: f64,
    converged: bool,
    coarse_published: bool,
    preview_due: bool,
    checkpoint_due: bool,
}

const PCG_SCALAR_BACKING_BYTES: usize = SPARSE_PAGE_BYTES;

#[derive(Clone, Copy, PartialEq, Eq)]
enum PcgPublicationKind {
    Checkpoint,
    Preview,
    Complete,
}

struct PcgPublication {
    kind: PcgPublicationKind,
    cursor: NumericalPageCursor,
    writer: RetainedJobPayloadWriter,
    complete: bool,
}

impl PcgPublication {
    fn new(kind: PcgPublicationKind) -> Self {
        let stream = match kind {
            PcgPublicationKind::Checkpoint => JobPayloadStream::CheckpointState,
            PcgPublicationKind::Preview => JobPayloadStream::Preview,
            PcgPublicationKind::Complete => JobPayloadStream::CommitOutput,
        };
        Self { kind, cursor: NumericalPageCursor::new(), writer: RetainedJobPayloadWriter::new(stream), complete: false }
    }

    fn advance(&mut self, operation: Operation, state: &PcgCheckpoint) -> Result<bool, JobPayloadAdmissionFault> {
        let (magic, kind, fields) = match self.kind {
            PcgPublicationKind::Checkpoint => (b"FEMPCP1\0", 13, 11),
            _ => (b"FEMPCG1\0", 3, 5),
        };
        if advance_numerical_page_header(&mut self.writer, magic, kind, self.cursor)? { return Ok(false); }
        let complete = if self.kind == PcgPublicationKind::Checkpoint {
            match self.cursor.field {
                0 => advance_u64_values(&mut self.writer, &state.checkpoint_control(operation), &mut self.cursor)?,
                1 => advance_paged_u32_owner(&mut self.writer, &state.a.indptr, &mut self.cursor)?,
                2 => advance_paged_u32_owner(&mut self.writer, &state.a.indices, &mut self.cursor)?,
                3 => advance_paged_f64_owner(&mut self.writer, &state.a.vals, &mut self.cursor)?,
                4 => advance_f64_owner(&mut self.writer, &state.b.0, &mut self.cursor)?,
                5 => advance_f64_owner(&mut self.writer, &state.x.0, &mut self.cursor)?,
                6 => advance_f64_owner(&mut self.writer, &state.diag.0, &mut self.cursor)?,
                7 => advance_f64_owner(&mut self.writer, &state.r.0, &mut self.cursor)?,
                8 => advance_f64_owner(&mut self.writer, &state.z.0, &mut self.cursor)?,
                9 => advance_f64_owner(&mut self.writer, &state.p.0, &mut self.cursor)?,
                10 => advance_f64_owner(&mut self.writer, &state.ap.0, &mut self.cursor)?,
                _ => true,
            }
        } else if self.cursor.field == 0 {
            advance_u64_values(&mut self.writer, &state.output_control(operation), &mut self.cursor)?
        } else {
            let values = if self.cursor.field == 2 || self.cursor.field == 3 { &state.r.0 } else { &state.x.0 };
            if !advance_owner_length(&mut self.writer, values.len(), &mut self.cursor)? { return Ok(false); }
            if let Some(value) = values.get(self.cursor.item) {
                let value = match self.cursor.field { 3 => -*value, 4 => value.abs(), _ => *value };
                if write_numerical_entry(&mut self.writer, &value.to_bits().to_le_bytes())? { self.cursor.item += 1; }
                false
            } else {
                true
            }
        };
        if complete {
            self.writer.commit_staged_page()?;
            self.cursor.field += 1;
            self.cursor.owner = 0;
            self.cursor.item = 0;
        }
        Ok(self.cursor.field == fields)
    }
}

impl PcgCheckpoint {
    fn checkpoint_control(&self, operation: Operation) -> [u64; 26] {
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
    operation: Operation,
    state: PcgCheckpoint,
    close_lane: u8,
    publication: Option<PcgPublication>,
    publication_fault: Option<NumericalCheckpointFault>,
    terminal_published: bool,
    closing: bool,
    desk: NumericalOutcomeDesk,
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
    fn from_state(operation: Operation, state: PcgCheckpoint) -> Self {
        Self { operation, state, close_lane: 0, publication: None, publication_fault: None, terminal_published: false, closing: false, desk: NumericalOutcomeDesk::default() }
    }

    fn step_publication(&mut self, context: &mut StepContext<'_>, kind: PcgPublicationKind) -> Result<NumericalRun, NumericalCheckpointFault> {
        if context.should_yield() { return Ok(NumericalRun::Yield); }
        context.consume_fuel(1);
        if self.publication.is_none() {
            self.publication = Some(PcgPublication::new(kind));
            return Ok(NumericalRun::Yield);
        }
        let publication = self.publication.as_mut().ok_or(NumericalCheckpointFault::Field)?;
        if !publication.complete {
            if publication.writer.staged_page_len().is_none() {
                publication.writer.begin_staged_page(context).map_err(|_| NumericalCheckpointFault::Admission)?;
                return Ok(NumericalRun::Yield);
            }
            publication.complete = publication.advance(self.operation, &self.state).map_err(|_| NumericalCheckpointFault::Admission)?;
            return Ok(NumericalRun::Yield);
        }
        let publication = self.publication.take().ok_or(NumericalCheckpointFault::Field)?;
        let payload = match publication.writer.finish() {
            Ok(payload) => payload,
            Err(writer) => {
                self.publication = Some(PcgPublication { kind: publication.kind, cursor: publication.cursor, writer, complete: publication.complete });
                return Err(NumericalCheckpointFault::Admission);
            }
        };
        Ok(match publication.kind {
            PcgPublicationKind::Checkpoint => {
                self.state.checkpoint_due = false;
                self.desk.lend_checkpoint(payload, self.state.iteration as u64)
            }
            PcgPublicationKind::Preview => {
                self.state.preview_due = false;
                self.desk.lend_preview(payload)
            }
            PcgPublicationKind::Complete => {
                self.terminal_published = true;
                self.state.preview_due = false;
                self.state.checkpoint_due = false;
                self.desk.lend_complete(payload)
            }
        })
    }

    pub fn new(operation: Operation, a: Csr, b: VecD, x: VecD, tol_rel: f64, max_iter: usize, batch_units: usize) -> Self {
        assert_eq!(a.n, b.len(), "pcg rhs dimension mismatch");
        assert_eq!(a.n, x.len(), "pcg initial guess dimension mismatch");
        assert!(batch_units > 0, "pcg batch must contain work");
        let n = a.n;
        let b_norm = b.norm2().max(1e-300);
        Self {
            operation,
            state: PcgCheckpoint {
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
            },
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
    fn close_retained_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
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

    fn reset_spmv(&mut self, stage: PcgStage) {
        self.state.stage = stage;
        self.state.row_cursor = 0;
        self.state.entry_cursor = self.state.a.indptr(0).unwrap_or(0) as usize;
        self.state.row_sum = 0.0;
        self.state.dot_accum = 0.0;
    }

    fn finish(&mut self, converged: bool) {
        self.state.converged = converged;
        self.state.stage = PcgStage::Complete;
    }

    fn step_diagonal(&mut self, context: &mut StepContext<'_>, units: &mut usize) {
        while self.state.row_cursor < self.state.a.n && *units < self.state.batch_units && !context.should_yield() && !context.is_cancelled() {
            let row = self.state.row_cursor;
            let end = self.state.a.indptr(row + 1).expect("PCG diagonal row end") as usize;
            if self.state.entry_cursor < end {
                let entry = self.state.entry_cursor;
                if self.state.a.index(entry).expect("PCG diagonal column") as usize == row {
                    self.state.diag.set(row, self.state.a.value(entry).expect("PCG diagonal value"));
                }
                self.state.entry_cursor += 1;
            } else {
                self.state.row_cursor += 1;
                self.state.entry_cursor = self.state.a.indptr(self.state.row_cursor.min(self.state.a.n)).expect("PCG diagonal next row") as usize;
            }
            *units += 1;
            context.consume_fuel(1);
        }
        if self.state.row_cursor == self.state.a.n {
            self.reset_spmv(PcgStage::InitialSpmv);
        }
    }

    fn step_spmv(&mut self, context: &mut StepContext<'_>, units: &mut usize, initial: bool) {
        while self.state.row_cursor < self.state.a.n && *units < self.state.batch_units && !context.should_yield() && !context.is_cancelled() {
            let row = self.state.row_cursor;
            let end = self.state.a.indptr(row + 1).expect("PCG SpMV row end") as usize;
            if self.state.entry_cursor < end {
                let entry = self.state.entry_cursor;
                let col = self.state.a.index(entry).expect("PCG SpMV column") as usize;
                let value = if initial { self.state.x.get(col) } else { self.state.p.get(col) };
                self.state.row_sum += self.state.a.value(entry).expect("PCG SpMV value") * value;
                self.state.entry_cursor += 1;
            } else {
                self.state.ap.set(row, self.state.row_sum);
                if !initial {
                    self.state.dot_accum += self.state.p.get(row) * self.state.row_sum;
                }
                self.state.row_sum = 0.0;
                self.state.row_cursor += 1;
                self.state.entry_cursor = self.state.a.indptr(self.state.row_cursor.min(self.state.a.n)).expect("PCG SpMV next row") as usize;
            }
            *units += 1;
            context.consume_fuel(1);
        }
        if self.state.row_cursor == self.state.a.n {
            self.state.cursor = 0;
            if initial {
                self.state.residual_sq = 0.0;
                self.state.stage = PcgStage::InitialResidual;
            } else if self.state.dot_accum.abs() < 1e-300 {
                self.finish(false);
            } else {
                self.state.alpha = self.state.rz_old / self.state.dot_accum;
                self.state.residual_sq = 0.0;
                self.state.stage = PcgStage::IterationUpdate;
            }
        }
    }

    fn step_initial_residual(&mut self, context: &mut StepContext<'_>, units: &mut usize) {
        while self.state.cursor < self.state.a.n && *units < self.state.batch_units && !context.should_yield() && !context.is_cancelled() {
            let i = self.state.cursor;
            let value = self.state.b.get(i) - self.state.ap.get(i);
            self.state.r.set(i, value);
            self.state.residual_sq += value * value;
            self.state.cursor += 1;
            *units += 1;
            context.consume_fuel(1);
        }
        if self.state.cursor == self.state.a.n {
            self.state.residual_norm = self.state.residual_sq.sqrt() / self.state.b_norm;
            if self.state.residual_norm < self.state.tol_rel {
                self.finish(true);
            } else if self.state.max_iter == 0 {
                self.finish(false);
            } else {
                self.state.cursor = 0;
                self.state.rz_new = 0.0;
                self.state.stage = PcgStage::InitialPrecondition;
            }
        }
    }

    fn step_precondition(&mut self, context: &mut StepContext<'_>, units: &mut usize, initial: bool) {
        while self.state.cursor < self.state.a.n && *units < self.state.batch_units && !context.should_yield() && !context.is_cancelled() {
            let i = self.state.cursor;
            let d = self.state.diag.get(i);
            let value = if d.abs() > 1e-300 { self.state.r.get(i) / d } else { self.state.r.get(i) };
            self.state.z.set(i, value);
            if initial {
                self.state.p.set(i, value);
            }
            self.state.rz_new += self.state.r.get(i) * value;
            self.state.cursor += 1;
            *units += 1;
            context.consume_fuel(1);
        }
        if self.state.cursor == self.state.a.n {
            self.state.cursor = 0;
            if initial {
                self.state.rz_old = self.state.rz_new;
                self.state.iteration = 1;
                self.reset_spmv(PcgStage::IterationSpmv);
            } else {
                self.state.beta = self.state.rz_new / self.state.rz_old;
                self.state.stage = PcgStage::IterationDirection;
            }
        }
    }

    fn step_update(&mut self, context: &mut StepContext<'_>, units: &mut usize) {
        while self.state.cursor < self.state.a.n && *units < self.state.batch_units && !context.should_yield() && !context.is_cancelled() {
            let i = self.state.cursor;
            self.state.x.set(i, self.state.x.get(i) + self.state.alpha * self.state.p.get(i));
            self.state.r.set(i, self.state.r.get(i) - self.state.alpha * self.state.ap.get(i));
            self.state.residual_sq += self.state.r.get(i) * self.state.r.get(i);
            self.state.cursor += 1;
            *units += 1;
            context.consume_fuel(1);
        }
        if self.state.cursor == self.state.a.n {
            self.state.residual_norm = self.state.residual_sq.sqrt() / self.state.b_norm;
            if self.state.residual_norm < self.state.tol_rel {
                self.finish(true);
            } else if self.state.iteration >= self.state.max_iter {
                self.finish(false);
            } else {
                if !self.state.coarse_published && self.state.residual_norm < self.state.tol_rel.max(1e-3) {
                    self.state.coarse_published = true;
                    self.state.preview_due = true;
                }
                self.state.cursor = 0;
                self.state.rz_new = 0.0;
                self.state.stage = PcgStage::IterationPrecondition;
            }
            self.state.checkpoint_due = true;
        }
    }

    fn step_direction(&mut self, context: &mut StepContext<'_>, units: &mut usize) {
        while self.state.cursor < self.state.a.n && *units < self.state.batch_units && !context.should_yield() && !context.is_cancelled() {
            let i = self.state.cursor;
            self.state.p.set(i, self.state.z.get(i) + self.state.beta * self.state.p.get(i));
            self.state.cursor += 1;
            *units += 1;
            context.consume_fuel(1);
        }
        if self.state.cursor == self.state.a.n {
            self.state.rz_old = self.state.rz_new;
            self.state.iteration += 1;
            self.reset_spmv(PcgStage::IterationSpmv);
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PcgConstructionStage {
    ReserveB,
    InitializeB,
    RetireMountedB,
    NormalizeB,
    ReserveX,
    InitializeX,
    ReserveDiagonal,
    InitializeDiagonal,
    ReserveR,
    InitializeR,
    ReserveZ,
    InitializeZ,
    ReserveP,
    InitializeP,
    ReserveAp,
    InitializeAp,
    Complete,
}

/// 🧵️ Retained PCG constructor. Each vector allocation and each initialized scalar owns
/// a distinct worker opportunity; no full RHS or zero-vector materialization occurs in its caller.
pub struct PcgJobConstruction {
    operation: Operation,
    matrix: Option<Csr>,
    stage: PcgConstructionStage,
    cursor: usize,
    b: VecD,
    mounted_b: Option<MountedScalarSlots>,
    rhs_norm: f64,
    x: VecD,
    diag: VecD,
    r: VecD,
    z: VecD,
    p: VecD,
    ap: VecD,
    complete: Option<PcgJob>,
    fault: Option<&'static [u8]>,
    closing: bool,
}

impl PcgJobConstruction {
    pub fn new(operation: Operation, matrix: Csr) -> Self {
        Self {
            operation,
            matrix: Some(matrix),
            stage: PcgConstructionStage::ReserveB,
            cursor: 0,
            b: VecD::from_vec(Vec::new()),
            mounted_b: None,
            rhs_norm: 0.0,
            x: VecD::from_vec(Vec::new()),
            diag: VecD::from_vec(Vec::new()),
            r: VecD::from_vec(Vec::new()),
            z: VecD::from_vec(Vec::new()),
            p: VecD::from_vec(Vec::new()),
            ap: VecD::from_vec(Vec::new()),
            complete: None,
            fault: None,
            closing: false,
        }
    }

    /// 🌬️ Retains the generation's assembled RHS and derives its convergence norm one scalar per opportunity.
    pub fn new_with_rhs(operation: Operation, matrix: Csr, rhs: VecD) -> Result<Self, (Csr, VecD)> {
        if matrix.n != rhs.len() || rhs.0.capacity().saturating_mul(size_of::<f64>()) > PCG_SCALAR_BACKING_BYTES {
            return Err((matrix, rhs));
        }
        Ok(Self {
            operation,
            matrix: Some(matrix),
            stage: PcgConstructionStage::NormalizeB,
            cursor: 0,
            b: rhs,
            mounted_b: None,
            rhs_norm: 0.0,
            x: VecD::from_vec(Vec::new()),
            diag: VecD::from_vec(Vec::new()),
            r: VecD::from_vec(Vec::new()),
            z: VecD::from_vec(Vec::new()),
            p: VecD::from_vec(Vec::new()),
            ap: VecD::from_vec(Vec::new()),
            complete: None,
            fault: None,
            closing: false,
        })
    }

    pub fn new_with_mounted_rhs(operation: Operation, matrix: Csr, rhs: MountedScalarSlots) -> Result<Self, (Csr, MountedScalarSlots)> {
        if matrix.n != rhs.len() {
            return Err((matrix, rhs));
        }
        let mut construction = Self::new(operation, matrix);
        construction.mounted_b = Some(rhs);
        Ok(construction)
    }

    pub fn step_one(&mut self) -> Result<bool, &'static [u8]> {
        if let Some(fault) = self.fault { return Err(fault); }
        if self.closing { return Err(b"pcg-construction-closing"); }
        let result = self.advance_one();
        if let Err(fault) = result { self.fault = Some(fault); }
        result
    }

    fn advance_one(&mut self) -> Result<bool, &'static [u8]> {
        let n = self.matrix.as_ref().ok_or(b"pcg-construction-matrix-missing" as &'static [u8])?.n;
        if n.checked_mul(size_of::<f64>()).is_none_or(|bytes| bytes > PCG_SCALAR_BACKING_BYTES) {
            return Err(b"pcg-construction-owner-page-capacity");
        }
        macro_rules! reserve {
            ($owner:expr, $next:expr, $fault:expr) => {{
                $owner.0.try_reserve_exact(n).map_err(|_| $fault as &'static [u8])?;
                if $owner.0.capacity().checked_mul(std::mem::size_of::<f64>()).is_none_or(|bytes| bytes > PCG_SCALAR_BACKING_BYTES) {
                    return Err(b"pcg-construction-owner-page-capacity");
                }
                self.cursor = 0;
                self.stage = $next;
            }};
        }
        macro_rules! initialize {
            ($owner:expr, $value:expr, $next:expr) => {{
                if self.cursor < n {
                    $owner.0.push($value);
                    self.cursor += 1;
                } else {
                    self.stage = $next;
                }
            }};
        }
        match self.stage {
            PcgConstructionStage::ReserveB => reserve!(self.b, PcgConstructionStage::InitializeB, b"pcg-construction-b-allocation"),
            PcgConstructionStage::InitializeB => {
                if self.cursor < n {
                    let value = self.mounted_b.as_ref().and_then(|rhs| rhs.get(self.cursor)).unwrap_or(1.0);
                    self.b.0.push(value);
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                    self.stage = if self.mounted_b.is_some() { PcgConstructionStage::RetireMountedB } else { PcgConstructionStage::NormalizeB };
                }
            }
            PcgConstructionStage::RetireMountedB => {
                if self.mounted_b.as_mut().is_some_and(MountedScalarSlots::close_step) {
                    self.mounted_b = None;
                    self.stage = PcgConstructionStage::NormalizeB;
                }
            }
            PcgConstructionStage::NormalizeB => {
                if self.cursor < n {
                    let value = *self.b.0.get(self.cursor).ok_or(b"pcg-construction-rhs-missing" as &'static [u8])?;
                    if !value.is_finite() { return Err(b"pcg-construction-rhs-scalar"); }
                    let norm = self.rhs_norm.hypot(value);
                    if !norm.is_finite() { return Err(b"pcg-construction-rhs-norm"); }
                    self.rhs_norm = norm;
                    self.cursor += 1;
                } else {
                    self.stage = PcgConstructionStage::ReserveX;
                }
            }
            PcgConstructionStage::ReserveX => reserve!(self.x, PcgConstructionStage::InitializeX, b"pcg-construction-x-allocation"),
            PcgConstructionStage::InitializeX => initialize!(self.x, 0.0, PcgConstructionStage::ReserveDiagonal),
            PcgConstructionStage::ReserveDiagonal => reserve!(self.diag, PcgConstructionStage::InitializeDiagonal, b"pcg-construction-diagonal-allocation"),
            PcgConstructionStage::InitializeDiagonal => initialize!(self.diag, 0.0, PcgConstructionStage::ReserveR),
            PcgConstructionStage::ReserveR => reserve!(self.r, PcgConstructionStage::InitializeR, b"pcg-construction-r-allocation"),
            PcgConstructionStage::InitializeR => initialize!(self.r, 0.0, PcgConstructionStage::ReserveZ),
            PcgConstructionStage::ReserveZ => reserve!(self.z, PcgConstructionStage::InitializeZ, b"pcg-construction-z-allocation"),
            PcgConstructionStage::InitializeZ => initialize!(self.z, 0.0, PcgConstructionStage::ReserveP),
            PcgConstructionStage::ReserveP => reserve!(self.p, PcgConstructionStage::InitializeP, b"pcg-construction-p-allocation"),
            PcgConstructionStage::InitializeP => initialize!(self.p, 0.0, PcgConstructionStage::ReserveAp),
            PcgConstructionStage::ReserveAp => reserve!(self.ap, PcgConstructionStage::InitializeAp, b"pcg-construction-ap-allocation"),
            PcgConstructionStage::InitializeAp => initialize!(self.ap, 0.0, PcgConstructionStage::Complete),
            PcgConstructionStage::Complete => {
                if self.complete.is_none() {
                    self.complete = Some(PcgJob {
                        operation: self.operation,
                        state: PcgCheckpoint {
                            a: self.matrix.take().expect("matrix retained through construction"),
                            b: std::mem::replace(&mut self.b, VecD::from_vec(Vec::new())),
                            x: std::mem::replace(&mut self.x, VecD::from_vec(Vec::new())),
                            tol_rel: 1.0e-8,
                            max_iter: 512,
                            batch_units: 1,
                            stage: PcgStage::InitializeDiagonal,
                            diag: std::mem::replace(&mut self.diag, VecD::from_vec(Vec::new())),
                            r: std::mem::replace(&mut self.r, VecD::from_vec(Vec::new())),
                            z: std::mem::replace(&mut self.z, VecD::from_vec(Vec::new())),
                            p: std::mem::replace(&mut self.p, VecD::from_vec(Vec::new())),
                            ap: std::mem::replace(&mut self.ap, VecD::from_vec(Vec::new())),
                            b_norm: self.rhs_norm.max(1e-300),
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
                        },
                        close_lane: 0,
                        publication: None,
                        publication_fault: None,
                        terminal_published: false,
                        closing: false,
                        desk: NumericalOutcomeDesk::default(),
                    });
                }
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn take_complete(&mut self) -> Option<PcgJob> {
        (self.stage == PcgConstructionStage::Complete).then(|| self.complete.take()).flatten()
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        self.closing = true;
        if grant.maximum_items == 0 {
            return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
        }
        let maximum_bytes = grant.maximum_release_bytes;
        if let Some(complete) = self.complete.as_mut() {
            let step = complete.close_granted(grant)?;
            let terminal = complete.terminal_is_empty();
            if terminal {
                self.complete = None;
            }
            return Ok(numerical_child_close(step, terminal));
        }
        if let Some(matrix) = self.matrix.as_mut() {
            let (terminal, items, bytes) = matrix.close_step(maximum_bytes);
            if !terminal {
                return Ok(numerical_ladder_turn((false, items, bytes)));
            }
            self.matrix = None;
            return Ok(numerical_ladder_turn((false, 1, 0)));
        }
        if let Some(rhs) = self.mounted_b.as_mut() {
            if !rhs.close_step() {
                return Ok(numerical_ladder_turn((false, 1, 0)));
            }
            self.mounted_b = None;
            return Ok(numerical_ladder_turn((false, 1, 0)));
        }
        for vector in [&mut self.b, &mut self.x, &mut self.diag, &mut self.r, &mut self.z, &mut self.p, &mut self.ap] {
            if let Some(step) = numerical_owner_turn(close_vec_owner_step(&mut vector.0, maximum_bytes)) {
                return Ok(step);
            }
        }
        Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() })
    }

    pub fn close_frontier(&self) -> Result<Option<RetirementDemand>, ValueError> {
        match self.complete.as_ref() {
            Some(complete) => complete.close_frontier(),
            None => Ok(None),
        }
    }

    pub fn close_demand(&self) -> Result<RetirementDemand, ValueError> {
        Ok(self.close_frontier()?.unwrap_or_else(|| numerical_rung_demand(false)))
    }
}

/// 🧬 Restores one exact retained PCG generation with bounded page and scalar ownership.
pub struct PcgRestoreCursor {
    operation: Operation,
    payload: Option<RetainedJobPayload>,
    total_pages: usize,
    page_slot: usize,
    close_due: bool,
    expected_field: u16,
    page_entry: usize,
    owner_cursor: NumericalOwnerRestoreCursor,
    control: [u64; 26],
    job: Option<PcgJob>,
    fault: Option<NumericalCheckpointFault>,
    closing: bool,
}

impl PcgRestoreCursor {
    pub fn new(operation: Operation, payload: RetainedJobPayload) -> Self {
        let total_pages = payload.page_count();
        Self { operation, payload: Some(payload), total_pages, page_slot: 0, close_due: false, expected_field: 0, page_entry: 0, owner_cursor: NumericalOwnerRestoreCursor::default(), control: [0; 26], job: None, fault: None, closing: false }
    }

    fn decode_control(&mut self) -> Result<(), NumericalCheckpointFault> {
        let c = self.control;
        if !(NumericalCheckpointIdentity { operation: c[0], revision: c[1], generation: c[2], seed: c[3] }).matches(self.operation) {
            return Err(NumericalCheckpointFault::Stale);
        }
        let integer = |index| usize::try_from(c[index]).map_err(|_| NumericalCheckpointFault::Envelope);
        let boolean = |index| match c[index] { 0 => Ok(false), 1 => Ok(true), _ => Err(NumericalCheckpointFault::Field) };
        let n = integer(4)?;
        if n.checked_mul(size_of::<f64>()).is_none_or(|bytes| bytes > PCG_SCALAR_BACKING_BYTES) || integer(7)? == 0 || integer(18)? > n || integer(19)? > n {
            return Err(NumericalCheckpointFault::Envelope);
        }
        let stage = match c[8] {
            0 => PcgStage::InitializeDiagonal,
            1 => PcgStage::InitialSpmv,
            2 => PcgStage::InitialResidual,
            3 => PcgStage::InitialPrecondition,
            4 => PcgStage::IterationSpmv,
            5 => PcgStage::IterationUpdate,
            6 => PcgStage::IterationPrecondition,
            7 => PcgStage::IterationDirection,
            8 => PcgStage::Complete,
            _ => return Err(NumericalCheckpointFault::Field),
        };
        let state = PcgCheckpoint {
            a: Csr::from_paged_parts(n, PagedList::default(), PagedList::default(), PagedList::default()),
            b: VecD::from_vec(Vec::new()), x: VecD::from_vec(Vec::new()), diag: VecD::from_vec(Vec::new()),
            r: VecD::from_vec(Vec::new()), z: VecD::from_vec(Vec::new()), p: VecD::from_vec(Vec::new()), ap: VecD::from_vec(Vec::new()),
            tol_rel: f64::from_bits(c[5]), max_iter: integer(6)?, batch_units: integer(7)?, stage,
            b_norm: f64::from_bits(c[9]), residual_norm: f64::from_bits(c[10]), residual_sq: f64::from_bits(c[11]),
            rz_old: f64::from_bits(c[12]), rz_new: f64::from_bits(c[13]), dot_accum: f64::from_bits(c[14]),
            alpha: f64::from_bits(c[15]), beta: f64::from_bits(c[16]), iteration: integer(17)?, cursor: integer(18)?,
            row_cursor: integer(19)?, entry_cursor: integer(20)?, row_sum: f64::from_bits(c[21]),
            converged: boolean(22)?, coarse_published: boolean(23)?, preview_due: boolean(24)?, checkpoint_due: boolean(25)?,
        };
        if !state.tol_rel.is_finite() || state.tol_rel <= 0.0 || !state.b_norm.is_finite() || state.b_norm <= 0.0 || state.iteration > state.max_iter
            || [state.residual_norm, state.residual_sq, state.rz_old, state.rz_new, state.dot_accum, state.alpha, state.beta, state.row_sum].iter().any(|value| !value.is_finite())
            || state.residual_norm < 0.0 || state.residual_sq < 0.0 {
            return Err(NumericalCheckpointFault::Envelope);
        }
        self.job = Some(PcgJob::from_state(self.operation, state));
        Ok(())
    }

    fn decode_page_entry(&mut self, bytes: &[u8]) -> Result<bool, NumericalCheckpointFault> {
        let page = parse_numerical_page(bytes, b"FEMPCP1\0")?;
        if bytes[14..16] != [0, 0] || page.kind != 13 || page.field != self.expected_field || (self.owner_cursor.shape.is_none() && (page.owner != 0 || page.item != 0)) {
            return Err(NumericalCheckpointFault::Field);
        }
        if page.field == 0 {
            if declared_owner_length(&page, 26)? != 26 || page.bytes.len() != 216 { return Err(NumericalCheckpointFault::Truncated); }
            if self.page_entry == 0 { self.page_entry = 1; return Ok(false); }
            let item = self.page_entry - 1;
            if item < 26 {
                self.control[item] = read_checkpoint_u64(page.bytes, 8 + item * 8)?;
                self.page_entry += 1;
                return Ok(false);
            }
            self.decode_control()?;
            self.expected_field = 1;
            return Ok(true);
        }
        let state = &mut self.job.as_mut().ok_or(NumericalCheckpointFault::Field)?.state;
        let n = state.a.n;
        let expected = match page.field {
            1 => n + 1,
            2 | 3 => state.a.indptr(n).ok_or(NumericalCheckpointFault::Field)? as usize,
            4..=10 => n,
            _ => return Err(NumericalCheckpointFault::Field),
        };
        if self.owner_cursor.shape.is_none() && declared_owner_length(&page, expected)? != expected { return Err(NumericalCheckpointFault::Envelope); }
        let before = match page.field { 1 => state.a.indptr.len(), 2 => state.a.indices.len(), _ => 0 };
        let complete = match page.field {
            1 => restore_paged_u32_entry(&mut state.a.indptr, &page, expected, &mut self.page_entry, &mut self.owner_cursor)?,
            2 => restore_paged_u32_entry(&mut state.a.indices, &page, expected, &mut self.page_entry, &mut self.owner_cursor)?,
            3 => restore_paged_f64_entry(&mut state.a.vals, &page, expected, &mut self.page_entry, &mut self.owner_cursor)?,
            field => {
                let owner = match field {
                    4 => &mut state.b.0, 5 => &mut state.x.0, 6 => &mut state.diag.0, 7 => &mut state.r.0,
                    8 => &mut state.z.0, 9 => &mut state.p.0, 10 => &mut state.ap.0, _ => unreachable!(),
                };
                let complete = restore_f64_entry(owner, &page, expected, &mut self.page_entry, &mut self.owner_cursor)?;
                if owner.capacity().checked_mul(size_of::<f64>()).is_none_or(|bytes| bytes > PCG_SCALAR_BACKING_BYTES) { return Err(NumericalCheckpointFault::Admission); }
                if owner.last().is_some_and(|value| !value.is_finite()) { return Err(NumericalCheckpointFault::Envelope); }
                complete
            }
        };
        if page.field == 3 && state.a.vals.len() != 0 && state.a.value(state.a.vals.len() - 1).is_some_and(|value| !value.is_finite()) {
            return Err(NumericalCheckpointFault::Envelope);
        }
        if page.field == 1 && state.a.indptr.len() > before {
            let value = state.a.indptr(before).ok_or(NumericalCheckpointFault::Field)? as usize;
            if value > n * n || (before == 0 && value != 0) || (before > 0 && value < state.a.indptr(before - 1).ok_or(NumericalCheckpointFault::Field)? as usize) {
                return Err(NumericalCheckpointFault::Envelope);
            }
        }
        if page.field == 2 && state.a.indices.len() > before && state.a.index(before).is_none_or(|value| value as usize >= n) {
            return Err(NumericalCheckpointFault::Envelope);
        }
        if complete && self.owner_cursor.shape.is_none() { self.expected_field = if page.field == 10 { u16::MAX } else { page.field + 1 }; }
        Ok(complete)
    }

    pub fn step(&mut self, context: &mut StepContext<'_>) -> Result<Option<PcgJob>, NumericalCheckpointFault> {
        if let Some(fault) = self.fault { return Err(fault); }
        let result = self.advance(context);
        if let Err(fault) = result { self.fault = Some(fault); }
        result
    }

    fn advance(&mut self, context: &mut StepContext<'_>) -> Result<Option<PcgJob>, NumericalCheckpointFault> {
        if self.closing || context.is_cancelled() { return Err(NumericalCheckpointFault::Cancelled); }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation { return Err(NumericalCheckpointFault::Stale); }
        if context.should_yield() { return Ok(None); }
        context.consume_fuel(1);
        if self.close_due {
            let payload = self.payload.as_mut().ok_or(NumericalCheckpointFault::Truncated)?;
            if numerical_restore_close_due(payload, context, &mut self.page_slot, &mut self.page_entry, &mut self.close_due, self.total_pages)? {
                if self.expected_field != u16::MAX { return Err(NumericalCheckpointFault::Truncated); }
                let job = self.job.as_ref().ok_or(NumericalCheckpointFault::Truncated)?;
                let state = &job.state;
                if state.entry_cursor > state.a.vals.len() || state.a.indices.len() != state.a.vals.len() { return Err(NumericalCheckpointFault::Envelope); }
                self.payload = None;
                return Ok(self.job.take());
            }
            return Ok(None);
        }
        let payload = self.payload.take().ok_or(NumericalCheckpointFault::Truncated)?;
        let decoded = payload.page(self.page_slot).ok_or(NumericalCheckpointFault::Truncated).and_then(|source| self.decode_page_entry(source));
        self.payload = Some(payload);
        if decoded? { self.close_due = true; }
        Ok(None)
    }

    fn close_frontier(&self) -> Result<Option<RetirementDemand>, ValueError> {
        if self.payload.is_some() {
            return numerical_restore_frontier(&self.payload);
        }
        match self.job.as_ref() {
            Some(job) => job.close_frontier(),
            None => Ok(None),
        }
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        self.closing = true;
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
        let Some(job) = self.job.as_mut() else { return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }) };
        let step = job.close_granted(grant)?;
        let terminal = job.terminal_is_empty();
        if terminal {
            self.job = None;
        }
        Ok(numerical_child_close(step, terminal))
    }

    pub fn terminal_is_empty(&self) -> bool { self.payload.is_none() && self.job.is_none() }
}


impl PcgJob {
    fn run(&mut self, context: &mut StepContext<'_>) -> NumericalRun {
        if context.is_cancelled() {
            return NumericalRun::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return NumericalRun::Fault;
        }
        if self.closing || self.publication_fault.is_some() {
            return NumericalRun::Fault;
        }
        if self.terminal_published { return NumericalRun::Yield; }
        let publication_kind = self.publication.as_ref().map(|publication| publication.kind).or_else(|| {
            if self.state.stage == PcgStage::Complete { Some(PcgPublicationKind::Complete) }
            else if self.state.preview_due { Some(PcgPublicationKind::Preview) }
            else if self.state.checkpoint_due { Some(PcgPublicationKind::Checkpoint) }
            else { None }
        });
        if let Some(kind) = publication_kind {
            context.set_stage(match kind {
                PcgPublicationKind::Checkpoint => "fem.pcg.checkpoint-page",
                PcgPublicationKind::Preview => "fem.pcg.preview-page",
                PcgPublicationKind::Complete => "fem.pcg.complete-page",
            });
            return match self.step_publication(context, kind) {
                Ok(outcome) => outcome,
                Err(fault) => {
                    self.publication_fault = Some(fault);
                    NumericalRun::Fault
                }
            };
        }
        context.set_stage("fem.pcg");
        let mut units = 0;
        while units < self.state.batch_units && !context.should_yield() && self.state.stage != PcgStage::Complete && !self.state.preview_due && !self.state.checkpoint_due {
            match self.state.stage {
                PcgStage::InitializeDiagonal => self.step_diagonal(context, &mut units),
                PcgStage::InitialSpmv => self.step_spmv(context, &mut units, true),
                PcgStage::InitialResidual => self.step_initial_residual(context, &mut units),
                PcgStage::InitialPrecondition => self.step_precondition(context, &mut units, true),
                PcgStage::IterationSpmv => self.step_spmv(context, &mut units, false),
                PcgStage::IterationUpdate => self.step_update(context, &mut units),
                PcgStage::IterationPrecondition => self.step_precondition(context, &mut units, false),
                PcgStage::IterationDirection => self.step_direction(context, &mut units),
                PcgStage::Complete => {}
            }
            if context.is_cancelled() {
                return NumericalRun::Cancelled;
            }
        }
        NumericalRun::Yield
    }

    fn close_frontier(&self) -> Result<Option<RetirementDemand>, ValueError> {
        if let Some(demand) = self.desk.retirement_demands() {
            return demand.map(Some);
        }
        if let Some(publication) = self.publication.as_ref() {
            return publication.writer.retirement_demands().map(Some);
        }
        Ok(None)
    }

    fn close_demand(&self) -> Result<RetirementDemand, ValueError> {
        Ok(self.close_frontier()?.unwrap_or_else(|| numerical_rung_demand(self.close_lane >= 8)))
    }

    fn close_turn(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        if let Some(step) = self.desk.close_step(grant) {
            return step;
        }
        if let Some(publication) = self.publication.as_mut() {
            publication.writer.begin_close();
            let step = numerical_close_result(publication.writer.close_step(grant))?;
            if publication.writer.terminal_is_empty() {
                self.publication = None;
            }
            return Ok(step);
        }
        Ok(numerical_ladder_turn(self.close_retained_step(grant.maximum_release_bytes)))
    }

    /// 🎟️ One close turn under an explicit grant; the parents that own a job (construction, restore) and the
    /// original-context `close_step` both admit it through the job's own quote.
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

impl InteractiveJob for PcgJob {
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

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        self.desk.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(publication) = self.publication.as_mut() { publication.writer.begin_close(); }
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
        self.close_lane == 8 && self.publication.is_none() && self.desk.terminal_is_empty()
    }
}

/// ➰️ Jacobi-preconditioned conjugate gradient — mutates `x0` in place, converges when
/// `‖r‖ / ‖b‖ < tol_rel` or `max_iter` is reached.
pub fn pcg(a: &Csr, b: &VecD, x0: &mut VecD, tol_rel: f64, max_iter: usize) -> PcgStats {
    let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), 0);
    let mut job = PcgJob::new(operation, a.clone(), b.clone(), x0.clone(), tol_rel, max_iter, 1_024);
    let mut preview_sequence = 0;
    loop {
        match numerical_batch_step(&mut job, operation, &mut preview_sequence) {
            NumericalBatchEnd::Complete => {
                let (solution, stats) = job.solution();
                *x0 = solution.clone();
                numerical_batch_close(&mut job);
                return stats;
            }
            NumericalBatchEnd::Fault => {
                numerical_batch_close(&mut job);
                panic!("PCG batch execution faulted");
            }
            NumericalBatchEnd::Cancelled => {
                numerical_batch_close(&mut job);
                panic!("PCG batch execution cancelled");
            }
            NumericalBatchEnd::Continue => {}
        }
    }
}
// #endregion 🔖️Pcg

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
