//! 🌱️ Bounded PCG job construction and the retained checkpoint restore cursor.

use super::*;

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
    pub(super) operation: Operation,
    pub(super) matrix: Option<Csr>,
    pub(super) stage: PcgConstructionStage,
    pub(super) cursor: usize,
    pub(super) b: VecD,
    pub(super) mounted_b: Option<MountedScalarSlots>,
    pub(super) rhs_norm: f64,
    pub(super) x: VecD,
    pub(super) diag: VecD,
    pub(super) r: VecD,
    pub(super) z: VecD,
    pub(super) p: VecD,
    pub(super) ap: VecD,
    pub(super) complete: Option<PcgJob>,
    fault: Option<&'static [u8]>,
    pub(super) closing: bool,
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
    pub(super) operation: Operation,
    pub(super) payload: Option<RetainedJobPayload>,
    total_pages: usize,
    pub(super) page_slot: usize,
    close_due: bool,
    pub(super) expected_field: u16,
    pub(super) page_entry: usize,
    pub(super) owner_cursor: NumericalOwnerRestoreCursor,
    control: [u64; 26],
    pub(super) job: Option<PcgJob>,
    fault: Option<NumericalCheckpointFault>,
    pub(super) closing: bool,
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

    pub(super) fn advance(&mut self, context: &mut StepContext<'_>) -> Result<Option<PcgJob>, NumericalCheckpointFault> {
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

    pub(super) fn close_frontier(&self) -> Result<Option<RetirementDemand>, ValueError> {
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
