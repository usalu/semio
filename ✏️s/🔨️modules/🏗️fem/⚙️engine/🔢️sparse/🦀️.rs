//! 🧮️ Sparse linear algebra core: COO/CSR/CSC storage, reverse-Cuthill-McKee ordering, and the numerical job
//! protocol every sparse solver job shares (outcome desk, close ladder, checkpoint page writer and restore cursor).

use replication::value::list::PagedList;
use semio_framework_job::{close_original_retained_step, InteractiveJob, InteractiveJobCloseStep, OriginalTurnFailure, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPayloadAdmissionFault, JobPayloadStream, Operation, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedJobPayload, RetainedJobPayloadWriter, StepBudget, StepContext};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::VecDeque;
use semio_s_fem_algebra::{MatD, VecD};

pub const SPARSE_INDEX_SPACE: usize = usize::MAX;
pub const SPARSE_PAGE_BYTES: usize = 4096;

/// 🧭️ What one internal step of a numerical job decided before any outcome is lent to the caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumericalRun {
    Yield,
    Cancelled,
    Fault,
    Preview,
    Checkpoint(u64),
    Complete,
}

/// 🎁️ Owns the one finished payload a numerical job lends at a time, retries a refused admission and pays the lent payload back from the next call's wallet.
pub struct NumericalOutcomeDesk {
    pub lent: Option<RetainedJobPayload>,
    pub delivered: bool,
    pub pending: Option<NumericalRun>,
    pub state_marker: RetainedJobPayload,
    pub output_marker: RetainedJobPayload,
    pub fault_marker: RetainedJobPayload,
}

impl Default for NumericalOutcomeDesk {
    fn default() -> Self {
        Self {
            lent: None,
            delivered: false,
            pending: None,
            state_marker: RetainedJobPayload::empty(JobPayloadStream::CommitState),
            output_marker: RetainedJobPayload::empty(JobPayloadStream::CommitOutput),
            fault_marker: RetainedJobPayload::empty(JobPayloadStream::Fault),
        }
    }
}

/// 🎟️ Whether one grant covers one quoted demand with at least one item.
pub fn numerical_grant_covers(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items != 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}

/// 🚪️ Whether a grant admits one close turn: a payload or writer frontier must be covered by its own quote, an owner rung is bounded by its own release limit.
pub fn numerical_close_gate(grant: RetainedCloneGrant, frontier: Option<RetirementDemand>) -> bool {
    match frontier {
        Some(demand) => numerical_grant_covers(grant, demand),
        None => grant.maximum_items != 0 && grant.maximum_depth != 0,
    }
}

/// 📏️ The largest single owner release one numerical close turn performs: every admitted owner is at most one owner page.
pub const NUMERICAL_CLOSE_RELEASE_BYTES: usize = NUMERICAL_OWNER_PAGE_BYTES;

/// 🪜️ Quotes one ladder rung of a numerical job's retained owners.
pub fn numerical_rung_demand(terminal: bool) -> RetirementDemand {
    if terminal { RetirementDemand::default() } else { RetirementDemand { release_bytes: NUMERICAL_CLOSE_RELEASE_BYTES, depth: 1, ..Default::default() } }
}

/// 🧯️ Maps one payload or writer close turn onto the job close vocabulary.
pub fn numerical_close_result(step: Result<RetainedCloneStep, ValueError>) -> Result<InteractiveJobCloseStep, ValueError> {
    let (RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) = step?;
    Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } })
}

/// 📏️ A job's own quote for its next close turn, read from its four declared demand currencies.
pub fn numerical_close_demand(job: &impl InteractiveJob) -> Result<RetirementDemand, ValueError> {
    let copy_bytes = job.next_close_copy_byte_demand()?;
    Ok(RetirementDemand { copy_bytes, capacity_bytes: job.next_close_capacity_byte_demand(copy_bytes)?, release_bytes: job.next_close_release_byte_demand()?, depth: job.next_close_depth_demand()?.max(1) })
}

/// 🪙️ Runs one grant-shaped numerical close turn of `owner` under the caller's original wallet: the turn only runs
/// when that wallet covers the job's quote, its effects are debited once, and a terminal owner reports `Complete`.
pub fn numerical_close_original<J>(cx: &mut StepContext<'_>, owner: &mut J, demand: RetirementDemand, turn: impl FnOnce(&mut J, RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError>, terminal: impl FnOnce(&J) -> bool) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
    let step = close_original_retained_step(cx, demand, |grant| {
        Ok(match turn(owner, grant)? {
            InteractiveJobCloseStep::Complete { progress } => RetainedCloneStep::Complete(progress),
            InteractiveJobCloseStep::Pending { progress } => RetainedCloneStep::Progress(progress),
            InteractiveJobCloseStep::Blocked => RetainedCloneStep::Progress(RetainedCloneProgress::default()),
        })
    })?;
    Ok(if terminal(owner) { InteractiveJobCloseStep::Complete { progress: step.progress() } } else { step })
}

/// 🎟️ One close turn of a locally owned job under exactly its own quoted grant (batch adapters and contract tests).
pub fn numerical_owned_close_turn<J: InteractiveJob>(job: &mut J, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
    let (mut sequence, mut receipt) = (0, RetainedCloneProgress::default());
    let mut context = StepContext::new(semio_framework_job::OperationId(0), semio_framework_job::Generation(0), StepBudget::new(u64::MAX, u64::MAX, grant), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
    job.close_step(&mut context)
}

/// 🪜️ Maps one tuple-shaped owner ladder rung onto the job close vocabulary; `None` once the ladder is exhausted.
pub fn numerical_owner_turn(rung: Result<Option<(usize, usize)>, ()>) -> Option<InteractiveJobCloseStep> {
    match rung {
        Ok(Some((items, bytes))) => Some(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: items, released_bytes: bytes, ..Default::default() } }),
        Err(()) => Some(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }),
        Ok(None) => None,
    }
}

/// 🪜️ Maps a `(complete, items, bytes)` ladder turn onto the job close vocabulary.
pub fn numerical_ladder_turn(turn: (bool, usize, usize)) -> InteractiveJobCloseStep {
    let (complete, items, bytes) = turn;
    let progress = RetainedCloneProgress { copied_items: items, released_bytes: bytes, ..Default::default() };
    if complete { InteractiveJobCloseStep::Complete { progress } } else { InteractiveJobCloseStep::Pending { progress } }
}

/// 🎟️ The wallet one batch-adapter step and one batch-adapter close turn may spend.
pub const NUMERICAL_BATCH_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };

/// 🧭️ What one batch-adapter step learned from its borrowed outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumericalBatchEnd {
    Continue,
    Complete,
    Fault,
    Cancelled,
}

/// ▶️ Runs one batch-adapter step with a fresh receipt; delivered payloads are paid back by the job's next step.
pub fn numerical_batch_step<J: InteractiveJob>(job: &mut J, operation: Operation, preview_sequence: &mut u64) -> NumericalBatchEnd {
    let mut receipt = RetainedCloneProgress::default();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX, NUMERICAL_BATCH_GRANT), semio_framework_job::root_cancel_token(), || Some(0), preview_sequence, &mut receipt);
    match job.step(&mut context).expect("numerical batch step admission") {
        None | Some(JobOutcomeBorrow::Yield { .. } | JobOutcomeBorrow::PreviewReady { .. } | JobOutcomeBorrow::CheckpointReady { .. }) => NumericalBatchEnd::Continue,
        Some(JobOutcomeBorrow::Complete { .. }) => NumericalBatchEnd::Complete,
        Some(JobOutcomeBorrow::Fault { .. }) => NumericalBatchEnd::Fault,
        Some(JobOutcomeBorrow::Cancelled { .. }) => NumericalBatchEnd::Cancelled,
    }
}

/// 🧹️ Closes a batch-adapter job through its own quoted close grants until it is terminal-empty.
pub fn numerical_batch_close<J: InteractiveJob>(job: &mut J) {
    job.begin_close();
    while !job.terminal_is_empty() {
        let copy_bytes = job.next_close_copy_byte_demand().expect("numerical batch job quotes its copy demand");
        let grant = RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: copy_bytes,
            maximum_capacity_bytes: job.next_close_capacity_byte_demand(copy_bytes).expect("numerical batch job quotes its capacity demand"),
            maximum_release_bytes: job.next_close_release_byte_demand().expect("numerical batch job quotes its release demand"),
            maximum_depth: job.next_close_depth_demand().expect("numerical batch job quotes its depth demand").max(1),
        };
        let step = numerical_owned_close_turn(job, grant);
        assert!(matches!(step, Ok(InteractiveJobCloseStep::Pending { .. } | InteractiveJobCloseStep::Complete { .. })), "a locally owned numerical batch close has no external owner and no refusal");
    }
}

/// 🪺️ Maps a child job's close turn onto the parent's; a terminal child is removed by the caller after this turn.
pub fn numerical_child_close(step: InteractiveJobCloseStep, terminal: bool) -> InteractiveJobCloseStep {
    let progress = step.progress();
    match step {
        InteractiveJobCloseStep::Blocked => step,
        _ if terminal => InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } },
        _ => InteractiveJobCloseStep::Pending { progress },
    }
}

/// 🧯️ Closes one retained payload under a grant that covers its own quote.
pub fn numerical_payload_close(payload: &mut RetainedJobPayload, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
    let demand = payload.retirement_demands()?;
    if !numerical_grant_covers(grant, demand) {
        return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
    }
    numerical_close_result(payload.close_step(grant))
}

/// 🧯️ Closes one half-built writer slot under a grant that covers its own quote; `None` when the slot is empty.
pub fn numerical_writer_close(slot: &mut Option<RetainedJobPayloadWriter>, grant: RetainedCloneGrant) -> Option<Result<InteractiveJobCloseStep, ValueError>> {
    let writer = slot.as_mut()?;
    writer.begin_close();
    let demand = match writer.retirement_demands() {
        Ok(demand) => demand,
        Err(error) => return Some(Err(error)),
    };
    if !numerical_grant_covers(grant, demand) {
        return Some(Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }));
    }
    let step = numerical_close_result(writer.close_step(grant));
    if writer.terminal_is_empty() {
        *slot = None;
    }
    Some(step)
}

/// 📏️ Quotes a half-built writer slot's next close turn; `None` when the slot is empty.
pub fn numerical_writer_demand(slot: &Option<RetainedJobPayloadWriter>) -> Option<Result<RetirementDemand, ValueError>> {
    slot.as_ref().map(RetainedJobPayloadWriter::retirement_demands)
}

/// 🔁️ Closes one restored checkpoint page, then the operation ledger, from the calling step's own wallet; true once the payload is terminal-empty.
pub fn numerical_restore_close_due(payload: &mut RetainedJobPayload, context: &mut StepContext<'_>, page_slot: &mut usize, page_entry: &mut usize, close_due: &mut bool, total_pages: usize) -> Result<bool, NumericalCheckpointFault> {
    let demand = payload.retirement_demands().map_err(|_| NumericalCheckpointFault::Admission)?;
    let grant = context.retained_grant();
    if !numerical_grant_covers(grant, demand) {
        return Ok(false);
    }
    let pages = payload.page_count();
    let step = payload.close_step(grant).map_err(|_| NumericalCheckpointFault::Admission)?;
    context.consume_retained(step.progress()).map_err(|_| NumericalCheckpointFault::Admission)?;
    if payload.page_count() < pages {
        *page_slot += 1;
        *page_entry = 0;
        if *page_slot < total_pages {
            *close_due = false;
            return Ok(false);
        }
    }
    Ok(payload.terminal_is_empty())
}

/// 🧯️ Closes a restore cursor's raw payload under a grant that covers its own quote; `None` once the payload is gone.
pub fn numerical_restore_payload_close(slot: &mut Option<RetainedJobPayload>, grant: RetainedCloneGrant) -> Option<Result<InteractiveJobCloseStep, ValueError>> {
    let payload = slot.as_mut()?;
    if payload.terminal_is_empty() {
        *slot = None;
        return Some(Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }));
    }
    Some(numerical_payload_close(payload, grant))
}

/// 📏️ Quotes a restore cursor's raw payload frontier; `None` once only owner rungs remain.
pub fn numerical_restore_frontier(payload: &Option<RetainedJobPayload>) -> Result<Option<RetirementDemand>, ValueError> {
    match payload {
        Some(payload) if !payload.terminal_is_empty() => payload.retirement_demands().map(Some),
        Some(_) => Ok(Some(RetirementDemand { depth: 1, ..Default::default() })),
        None => Ok(None),
    }
}

impl NumericalOutcomeDesk {
    pub fn lend_preview(&mut self, payload: RetainedJobPayload) -> NumericalRun {
        self.lent = Some(payload);
        NumericalRun::Preview
    }

    pub fn lend_checkpoint(&mut self, payload: RetainedJobPayload, applied_progress: u64) -> NumericalRun {
        self.lent = Some(payload);
        NumericalRun::Checkpoint(applied_progress)
    }

    pub fn lend_complete(&mut self, output: RetainedJobPayload) -> NumericalRun {
        self.lent = Some(output);
        NumericalRun::Complete
    }

    /// 🤝️ True when the previous call delivered a payload that this call must pay back before the job resumes.
    pub fn retire_due(&self, cx: &StepContext<'_>, operation: Operation) -> bool {
        self.delivered && !cx.is_cancelled() && cx.operation() == operation.operation && cx.generation() == operation.generation
    }

    pub fn take_pending(&mut self) -> Option<NumericalRun> {
        self.pending.take()
    }

    /// 💸️ Pays the delivered payload back turn by turn from the calling step's own wallet.
    pub fn retire_delivered(&mut self, cx: &mut StepContext<'_>) -> Result<(), ValueError> {
        let Some(payload) = self.lent.as_mut() else {
            self.delivered = false;
            return Ok(());
        };
        let demand = payload.retirement_demands()?;
        let grant = cx.retained_grant();
        if numerical_grant_covers(grant, demand) {
            let progress = match payload.close_step(grant)? {
                RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress) => progress,
            };
            cx.consume_retained(progress)?;
        }
        if payload.terminal_is_empty() {
            self.lent = None;
            self.delivered = false;
        }
        Ok(())
    }

    /// 📬️ Lends the decided outcome, or keeps it pending when the caller's grant cannot admit it yet.
    pub fn admit<'a>(&'a mut self, cx: &mut StepContext<'_>, run: NumericalRun) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let admitted = match run {
            NumericalRun::Yield => return Ok(None),
            NumericalRun::Cancelled => JobOutcomeBorrow::admit_cancelled(cx)?,
            NumericalRun::Fault => JobOutcomeBorrow::admit_fault(cx, &self.fault_marker)?,
            NumericalRun::Preview => JobOutcomeBorrow::admit_preview(cx, self.lent.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "numerical preview has no staged payload"))?)?,
            NumericalRun::Checkpoint(applied_progress) => JobOutcomeBorrow::admit_checkpoint(cx, self.lent.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "numerical checkpoint has no staged payload"))?, applied_progress)?,
            NumericalRun::Complete => JobOutcomeBorrow::admit_complete(cx, Some(&self.state_marker), Some(self.lent.as_ref().unwrap_or(&self.output_marker)))?,
        };
        if admitted.is_none() {
            self.pending = Some(run);
        } else if matches!(run, NumericalRun::Preview | NumericalRun::Checkpoint(_) | NumericalRun::Complete) {
            self.delivered = true;
        }
        Ok(admitted)
    }

    pub fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        let lent = || self.lent.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "numerical outcome has no staged payload"));
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Fault => descriptor.fault(&self.fault_marker),
            JobOutcomeKind::PreviewReady => descriptor.preview(lent()?),
            JobOutcomeKind::CheckpointReady { .. } => descriptor.checkpoint(lent()?),
            JobOutcomeKind::Complete => descriptor.complete(Some(&self.state_marker), Some(self.lent.as_ref().unwrap_or(&self.output_marker))),
        }
    }

    pub fn retirement_demands(&self) -> Option<Result<RetirementDemand, ValueError>> {
        self.lent.as_ref().map(RetainedJobPayload::retirement_demands)
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Option<Result<InteractiveJobCloseStep, ValueError>> {
        let payload = self.lent.as_mut()?;
        if payload.terminal_is_empty() {
            self.lent = None;
            self.delivered = false;
            return Some(Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }));
        }
        let step = numerical_payload_close(payload, grant);
        if payload.terminal_is_empty() {
            self.lent = None;
            self.delivered = false;
        }
        Some(step)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.lent.is_none() && self.pending.is_none()
    }

    /// 🧹️ Pays back the lent payload through its own quoted grants; for owners that keep their job after completion.
    pub fn retire_all(&mut self) {
        while let Some(demand) = self.retirement_demands() {
            let demand = demand.expect("a locally owned lent payload quotes its close demand");
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            let step = self.close_step(grant);
            assert!(matches!(step, Some(Ok(InteractiveJobCloseStep::Pending { .. } | InteractiveJobCloseStep::Complete { .. }))), "a locally owned lent payload close has no external owner and no refusal");
        }
        self.pending = None;
    }
}

pub fn close_vec_owner_step<T>(owner: &mut Vec<T>, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
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
    let progress = owner.release_empty_page(maximum_bytes).map_err(|_| ())?;
    if progress.progressed {
        return Ok(Some((0, progress.released_allocation_bytes)));
    }
    if owner.terminal_is_empty() {
        Ok(None)
    } else {
        Err(())
    }
}

fn paged_from_values<T>(values: Vec<T>) -> PagedList<T, SPARSE_INDEX_SPACE> {
    let target = values.len();
    let mut owner = PagedList::default();
    while owner.capacity() < target {
        owner.reserve_capacity_one(target, SPARSE_PAGE_BYTES).expect("cold CSR page allocation");
    }
    for value in values {
        owner.push_reserved(value).unwrap_or_else(|_| unreachable!("cold CSR reserved exact slot"));
    }
    owner
}

pub fn close_nested_vec_owner_step<T>(owner: &mut Vec<Vec<T>>, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
    if let Some(child) = owner.last_mut() {
        if let Some(step) = close_vec_owner_step(child, maximum_bytes)? {
            return Ok(Some(step));
        }
        owner.pop();
        return Ok(Some((1, 0)));
    }
    close_vec_owner_step(owner, maximum_bytes)
}

// #region 🔖️Coo
/// 🧱️ Triplet (row, col, value) accumulator for FEM-style assembly — duplicate `(row, col)`
/// entries are summed lazily by whichever `to_*` conversion reads them.
pub struct Coo {
    pub n: usize,
    rows: Vec<u32>,
    cols: Vec<u32>,
    vals: Vec<f64>,
}

impl Coo {
    /// 🕸️ The standard SPD reference matrix of the solver contract tests and benchmarks: the Laplacian of an undirected
    /// graph on `n` vertices plus the identity, so it stays positive definite for any edge set.
    pub fn graph_laplacian_plus_identity(n: usize, edges: &[(usize, usize)]) -> Coo {
        let mut degree = vec![0usize; n];
        for &(u, v) in edges {
            degree[u] += 1;
            degree[v] += 1;
        }
        let mut coo = Coo::new(n);
        for (i, d) in degree.iter().enumerate() {
            coo.add(i, i, *d as f64 + 1.0);
        }
        for &(u, v) in edges {
            coo.add(u, v, -1.0);
            coo.add(v, u, -1.0);
        }
        coo
    }

    pub fn new(n: usize) -> Self {
        Self { n, rows: Vec::new(), cols: Vec::new(), vals: Vec::new() }
    }

    pub fn add(&mut self, row: usize, col: usize, value: f64) {
        self.rows.push(row as u32);
        self.cols.push(col as u32);
        self.vals.push(value);
    }

    /// 🧩️ Scatters a small dense element block (e.g. from `Element::stiffness_global`) at global indices.
    pub fn add_block(&mut self, indices: &[usize], block: &MatD) {
        for (local_row, &global_row) in indices.iter().enumerate() {
            for (local_col, &global_col) in indices.iter().enumerate() {
                let value = block.get(local_row, local_col);
                if value != 0.0 {
                    self.add(global_row, global_col, value);
                }
            }
        }
    }

    fn merge_sorted(mut entries: Vec<(u32, f64)>) -> Vec<(u32, f64)> {
        entries.sort_by_key(|&(k, _)| k);
        let mut merged: Vec<(u32, f64)> = Vec::with_capacity(entries.len());
        for (k, v) in entries {
            if let Some(last) = merged.last_mut() {
                if last.0 == k {
                    last.1 += v;
                    continue;
                }
            }
            merged.push((k, v));
        }
        merged
    }

    /// 🧮️ General CSR (both triangles present or not, caller's choice) — used for SpMV.
    pub fn to_csr(&self) -> Csr {
        let n = self.n;
        let mut by_row: Vec<Vec<(u32, f64)>> = vec![Vec::new(); n];
        for i in 0..self.rows.len() {
            by_row[self.rows[i] as usize].push((self.cols[i], self.vals[i]));
        }
        let mut indptr = vec![0u32; n + 1];
        let mut indices = Vec::new();
        let mut vals = Vec::new();
        for row in 0..n {
            let merged = Self::merge_sorted(std::mem::take(&mut by_row[row]));
            indptr[row + 1] = indptr[row] + merged.len() as u32;
            for (c, v) in merged {
                indices.push(c);
                vals.push(v);
            }
        }
        Csr::from_owned_parts(n, indptr, indices, vals)
    }

    /// 🔺️ Keeps only entries where `col >= row` (upper triangle), grouped by the SMALLER index `j`
    /// so that storage-column `j` directly holds `A[j][c]` for every `c >= j` (via symmetry
    /// `A[j][c] = A[c][j]`) — the layout the LDLT factorization scatters into its row envelope without a scan.
    pub fn to_csc_sym_upper(&self) -> CscSym {
        let n = self.n;
        let mut by_col: Vec<Vec<(u32, f64)>> = vec![Vec::new(); n];
        for i in 0..self.rows.len() {
            let row = self.rows[i];
            let col = self.cols[i];
            if col >= row {
                by_col[row as usize].push((col, self.vals[i]));
            }
        }
        let mut colptr = vec![0u32; n + 1];
        let mut rowind = Vec::new();
        let mut vals = Vec::new();
        for col in 0..n {
            let merged = Self::merge_sorted(std::mem::take(&mut by_col[col]));
            colptr[col + 1] = colptr[col] + merged.len() as u32;
            for (r, v) in merged {
                rowind.push(r);
                vals.push(v);
            }
        }
        CscSym { n, colptr, rowind, vals }
    }

    /// 🪞️ Dense form for testing/cross-validation against `MatD::lu_solve`.
    pub fn to_dense(&self) -> MatD {
        let mut m = MatD::zeros(self.n, self.n);
        for i in 0..self.rows.len() {
            m.add_at(self.rows[i] as usize, self.cols[i] as usize, self.vals[i]);
        }
        m
    }
}
// #endregion 🔖️Coo

// #region 🔖️Csr
/// 🧮️ General compressed-sparse-row matrix with independent fixed physical pages for SpMV.
#[derive(Clone, ToValue, FromValue, semio_framework_value::RetireOwned)]
struct CsrWire {
    n: usize,
    indptr: Vec<u32>,
    indices: Vec<u32>,
    vals: Vec<f64>,
}

#[derive(Clone)]
pub struct Csr {
    pub n: usize,
    pub indptr: PagedList<u32, SPARSE_INDEX_SPACE>,
    pub indices: PagedList<u32, SPARSE_INDEX_SPACE>,
    pub vals: PagedList<f64, SPARSE_INDEX_SPACE>,
}

impl Csr {
    /// 🧊️ Moves cold contiguous inputs into fixed physical pages.
    pub fn from_owned_parts(n: usize, indptr: Vec<u32>, indices: Vec<u32>, vals: Vec<f64>) -> Self {
        Self { n, indptr: paged_from_values(indptr), indices: paged_from_values(indices), vals: paged_from_values(vals) }
    }

    /// 📚️ Transfers retained paged arrays without a contiguous intermediate owner.
    pub fn from_paged_parts(n: usize, indptr: PagedList<u32, SPARSE_INDEX_SPACE>, indices: PagedList<u32, SPARSE_INDEX_SPACE>, vals: PagedList<f64, SPARSE_INDEX_SPACE>) -> Self {
        Self { n, indptr, indices, vals }
    }

    pub fn indptr(&self, index: usize) -> Option<u32> {
        self.indptr.get(index).copied()
    }

    pub fn index(&self, index: usize) -> Option<u32> {
        self.indices.get(index).copied()
    }

    pub fn value(&self, index: usize) -> Option<f64> {
        self.vals.get(index).copied()
    }

    /// 🧮️ Returns the exact initialized compressed-entry count.
    pub fn entries_len(&self) -> usize {
        self.vals.len()
    }

    /// 🧱️ Reports row-pointer, column-index, and scalar physical backing independently.
    pub fn physical_backing_bytes(&self) -> [usize; 3] {
        [self.indptr.allocated_bytes(), self.indices.allocated_bytes(), self.vals.allocated_bytes()]
    }

    pub fn ownership_is_valid(&self) -> bool {
        self.n.checked_add(1).is_some_and(|rows| self.indptr.len() == rows) && self.indices.len() == self.vals.len() && self.indptr(self.n) == u32::try_from(self.entries_len()).ok()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.indptr.terminal_is_empty() && self.indices.terminal_is_empty() && self.vals.terminal_is_empty()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        match close_paged_owner_step(&mut self.vals, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        for owner in [&mut self.indices, &mut self.indptr] {
            match close_paged_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        (true, 0, 0)
    }

    pub fn mul_vec(&self, x: &VecD) -> VecD {
        let mut out = VecD::zeros(self.n);
        for row in 0..self.n {
            let start = self.indptr(row).expect("CSR row start") as usize;
            let end = self.indptr(row + 1).expect("CSR row end") as usize;
            let mut sum = 0.0;
            for idx in start..end {
                sum += self.value(idx).expect("CSR value") * x.get(self.index(idx).expect("CSR column") as usize);
            }
            out.set(row, sum);
        }
        out
    }

    pub fn diag(&self) -> VecD {
        let mut out = VecD::zeros(self.n);
        for row in 0..self.n {
            let start = self.indptr(row).expect("CSR row start") as usize;
            let end = self.indptr(row + 1).expect("CSR row end") as usize;
            for idx in start..end {
                if self.index(idx).expect("CSR column") as usize == row {
                    out.set(row, self.value(idx).expect("CSR value"));
                }
            }
        }
        out
    }
}

impl std::fmt::Debug for Csr {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Csr")
            .field("n", &self.n)
            .field("indptr", &self.indptr.iter().copied().collect::<Vec<_>>())
            .field("indices", &self.indices.iter().copied().collect::<Vec<_>>())
            .field("vals", &self.vals.iter().copied().collect::<Vec<_>>())
            .finish()
    }
}

impl PartialEq for Csr {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n && self.indptr.iter().eq(other.indptr.iter()) && self.indices.iter().eq(other.indices.iter()) && self.vals.iter().eq(other.vals.iter())
    }
}

impl semio_framework_value::ToValue for Csr {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::ToValue::to_value(&CsrWire { n: self.n, indptr: self.indptr.iter().copied().collect(), indices: self.indices.iter().copied().collect(), vals: self.vals.iter().copied().collect() })
    }
}

impl semio_framework_value::FromValue for Csr {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, ValueError> {
        let wire = <CsrWire as semio_framework_value::FromValue>::from_value(value)?;
        Ok(Self::from_owned_parts(wire.n, wire.indptr, wire.indices, wire.vals))
    }
}
// #endregion 🔖️Csr

// #region 🔖️CscSym
/// 🔺️ Symmetric matrix, upper-triangle entries only (`col >= row`), grouped by the smaller index —
/// storage-column `j` holds `A[j][c]` for every `c >= j`, the LDLT input format.
#[derive(Clone, Debug, PartialEq)]
pub struct CscSym {
    pub n: usize,
    pub colptr: Vec<u32>,
    pub rowind: Vec<u32>,
    pub vals: Vec<f64>,
}

impl CscSym {
    /// 🔍️ Reads `(row, col)` — storage-column is the smaller index, stored-row the larger.
    pub fn get(&self, row: usize, col: usize) -> f64 {
        let (lo, hi) = if row <= col { (row, col) } else { (col, row) };
        let start = self.colptr[lo] as usize;
        let end = self.colptr[lo + 1] as usize;
        for idx in start..end {
            if self.rowind[idx] as usize == hi {
                return self.vals[idx];
            }
        }
        0.0
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        match close_vec_owner_step(&mut self.vals, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        for owner in [&mut self.rowind, &mut self.colptr] {
            match close_vec_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        (true, 0, 0)
    }

    /// 🪟️ Mirrors into a full general CSR (for SpMV/PCG/residual use).
    pub fn to_csr_full(&self) -> Csr {
        let mut coo = Coo::new(self.n);
        for col in 0..self.n {
            let start = self.colptr[col] as usize;
            let end = self.colptr[col + 1] as usize;
            for idx in start..end {
                let row = self.rowind[idx] as usize;
                let value = self.vals[idx];
                coo.add(row, col, value);
                if row != col {
                    coo.add(col, row, value);
                }
            }
        }
        coo.to_csr()
    }
}
// #endregion 🔖️CscSym


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumericalCheckpointIdentity {
    pub operation: u64,
    pub revision: u64,
    pub generation: u64,
    pub seed: u64,
}

impl NumericalCheckpointIdentity {
    pub fn from_operation(operation: Operation) -> Self {
        Self { operation: operation.operation.0, revision: operation.base_revision.0, generation: operation.generation.0, seed: operation.seed }
    }

    pub fn matches(self, operation: Operation) -> bool {
        self == Self::from_operation(operation)
    }
}
pub const NUMERICAL_OWNER_PAGE_BYTES: usize = 16 * 1024;
pub const MOUNTED_SCALAR_SLOTS: usize = 768;

/// 📊️ A mounted scalar admission or update exceeds its available slots.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MountedScalarFault {
    Capacity { requested: usize, maximum: usize },
    Index { index: usize, len: usize },
}

/// 🧮 Fixed mounted scalar owner with one admission, write, update, or close action per turn.
pub struct MountedScalarSlots {
    pub values: [f64; MOUNTED_SCALAR_SLOTS],
    pub admitted: usize,
    pub len: usize,
}

impl MountedScalarSlots {
    pub fn new() -> Self {
        Self { values: [0.0; MOUNTED_SCALAR_SLOTS], admitted: 0, len: 0 }
    }

    pub fn admit_one(&mut self, target: usize) -> Result<bool, MountedScalarFault> {
        if target > MOUNTED_SCALAR_SLOTS {
            return Err(MountedScalarFault::Capacity { requested: target, maximum: MOUNTED_SCALAR_SLOTS });
        }
        if self.admitted < target {
            self.admitted += 1;
            return Ok(false);
        }
        Ok(true)
    }

    pub fn push(&mut self, value: f64) -> Result<(), f64> {
        if self.len == self.admitted {
            return Err(value);
        }
        self.values[self.len] = value;
        self.len += 1;
        Ok(())
    }

    pub fn add_at(&mut self, index: usize, value: f64) -> Result<(), MountedScalarFault> {
        if index >= self.len {
            return Err(MountedScalarFault::Index { index, len: self.len });
        }
        self.values[index] += value;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<f64> {
        (index < self.len).then(|| self.values[index])
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.admitted == 0
    }

    pub fn close_step(&mut self) -> bool {
        if self.len != 0 {
            self.len -= 1;
            self.values[self.len] = 0.0;
            return false;
        }
        if self.admitted != 0 {
            self.admitted -= 1;
            return false;
        }
        true
    }
}

impl Default for MountedScalarSlots {
    fn default() -> Self {
        Self::new()
    }
}
pub const NUMERICAL_CHECKPOINT_VERSION: u16 = 1;
pub const NUMERICAL_CHECKPOINT_HEADER_BYTES: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumericalPageCursor {
    pub field: u16,
    pub owner: usize,
    pub item: usize,
}

impl NumericalPageCursor {
    pub fn new() -> Self {
        Self { field: 0, owner: 0, item: 0 }
    }
}

pub fn numerical_page_header(magic: &[u8; 8], kind: u16, cursor: NumericalPageCursor) -> [u8; NUMERICAL_CHECKPOINT_HEADER_BYTES] {
    let mut bytes = [0; NUMERICAL_CHECKPOINT_HEADER_BYTES];
    bytes[..8].copy_from_slice(magic);
    bytes[8..10].copy_from_slice(&NUMERICAL_CHECKPOINT_VERSION.to_le_bytes());
    bytes[10..12].copy_from_slice(&kind.to_le_bytes());
    bytes[12..14].copy_from_slice(&cursor.field.to_le_bytes());
    bytes[16..24].copy_from_slice(&(cursor.owner as u64).to_le_bytes());
    bytes[24..32].copy_from_slice(&(cursor.item as u64).to_le_bytes());
    bytes
}

pub fn advance_numerical_page_header(writer: &mut RetainedJobPayloadWriter, magic: &[u8; 8], kind: u16, cursor: NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if writer.staged_page_len() == Some(0) {
        writer.write_staged(&numerical_page_header(magic, kind, cursor))?;
        return Ok(true);
    }
    Ok(false)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumericalCheckpointFault {
    Cancelled,
    Stale,
    Version,
    Truncated,
    Field,
    Envelope,
    Admission,
}

pub fn write_numerical_entry(writer: &mut RetainedJobPayloadWriter, bytes: &[u8]) -> Result<bool, JobPayloadAdmissionFault> {
    if writer.staged_page_remaining() < bytes.len() {
        writer.commit_staged_page()?;
        return Ok(false);
    }
    writer.write_staged(bytes)?;
    Ok(true)
}

pub fn advance_owner_length(writer: &mut RetainedJobPayloadWriter, length: usize, cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if cursor.owner == 0 {
        if !write_numerical_entry(writer, &(length as u64).to_le_bytes())? {
            return Ok(false);
        }
        cursor.owner = 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_u32_owner(writer: &mut RetainedJobPayloadWriter, values: &[u32], cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        if !write_numerical_entry(writer, &value.to_le_bytes())? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_u64_owner(writer: &mut RetainedJobPayloadWriter, values: &[usize], cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        if !write_numerical_entry(writer, &(*value as u64).to_le_bytes())? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_u64_values(writer: &mut RetainedJobPayloadWriter, values: &[u64], cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        if !write_numerical_entry(writer, &value.to_le_bytes())? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_f64_owner(writer: &mut RetainedJobPayloadWriter, values: &[f64], cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        if !write_numerical_entry(writer, &value.to_bits().to_le_bytes())? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_paged_u32_owner<const N: usize>(writer: &mut RetainedJobPayloadWriter, values: &PagedList<u32, N>, cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        if !write_numerical_entry(writer, &value.to_le_bytes())? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_paged_f64_owner<const N: usize>(writer: &mut RetainedJobPayloadWriter, values: &PagedList<f64, N>, cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        if !write_numerical_entry(writer, &value.to_bits().to_le_bytes())? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_pair_owner(writer: &mut RetainedJobPayloadWriter, values: &[(u32, f64)], cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some((index, value)) = values.get(cursor.item) {
        let mut bytes = [0; 12];
        bytes[..4].copy_from_slice(&index.to_le_bytes());
        bytes[4..].copy_from_slice(&value.to_bits().to_le_bytes());
        if !write_numerical_entry(writer, &bytes)? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

pub fn advance_matrix_owner(writer: &mut RetainedJobPayloadWriter, matrix: &MatD, cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if cursor.owner == 0 {
        let mut bytes = [0; 24];
        bytes[..8].copy_from_slice(&(matrix.rows as u64).to_le_bytes());
        bytes[8..16].copy_from_slice(&(matrix.cols as u64).to_le_bytes());
        bytes[16..].copy_from_slice(&(matrix.data.len() as u64).to_le_bytes());
        if !write_numerical_entry(writer, &bytes)? {
            return Ok(false);
        }
        cursor.owner = 1;
        return Ok(false);
    }
    if let Some(value) = matrix.data.get(cursor.item) {
        if !write_numerical_entry(writer, &value.to_bits().to_le_bytes())? {
            return Ok(false);
        }
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}
pub struct NumericalPageView<'a> {
    pub kind: u16,
    pub field: u16,
    pub owner: usize,
    pub item: usize,
    pub bytes: &'a [u8],
}

fn read_checkpoint_u16(bytes: &[u8], offset: usize) -> Result<u16, NumericalCheckpointFault> {
    let value = bytes.get(offset..offset.checked_add(2).ok_or(NumericalCheckpointFault::Truncated)?).ok_or(NumericalCheckpointFault::Truncated)?;
    Ok(u16::from_le_bytes([value[0], value[1]]))
}

fn read_checkpoint_u32(bytes: &[u8], offset: usize) -> Result<u32, NumericalCheckpointFault> {
    let value = bytes.get(offset..offset.checked_add(4).ok_or(NumericalCheckpointFault::Truncated)?).ok_or(NumericalCheckpointFault::Truncated)?;
    Ok(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

pub fn read_checkpoint_u64(bytes: &[u8], offset: usize) -> Result<u64, NumericalCheckpointFault> {
    let value = bytes.get(offset..offset.checked_add(8).ok_or(NumericalCheckpointFault::Truncated)?).ok_or(NumericalCheckpointFault::Truncated)?;
    Ok(u64::from_le_bytes([value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7]]))
}

pub fn read_checkpoint_usize(bytes: &[u8], offset: usize) -> Result<usize, NumericalCheckpointFault> {
    usize::try_from(read_checkpoint_u64(bytes, offset)?).map_err(|_| NumericalCheckpointFault::Envelope)
}

pub fn parse_numerical_page<'a>(bytes: &'a [u8], magic: &[u8; 8]) -> Result<NumericalPageView<'a>, NumericalCheckpointFault> {
    if bytes.len() < NUMERICAL_CHECKPOINT_HEADER_BYTES || bytes.get(..8) != Some(magic.as_slice()) {
        return Err(NumericalCheckpointFault::Truncated);
    }
    if read_checkpoint_u16(bytes, 8)? != NUMERICAL_CHECKPOINT_VERSION {
        return Err(NumericalCheckpointFault::Version);
    }
    Ok(NumericalPageView {
        kind: read_checkpoint_u16(bytes, 10)?,
        field: read_checkpoint_u16(bytes, 12)?,
        owner: read_checkpoint_usize(bytes, 16)?,
        item: read_checkpoint_usize(bytes, 24)?,
        bytes: &bytes[NUMERICAL_CHECKPOINT_HEADER_BYTES..],
    })
}

pub fn declared_owner_length(page: &NumericalPageView<'_>, maximum: usize) -> Result<usize, NumericalCheckpointFault> {
    let length = read_checkpoint_usize(page.bytes, 0)?;
    (length <= maximum).then_some(length).ok_or(NumericalCheckpointFault::Envelope)
}

pub fn validate_restored_owner<T>(owner: &Vec<T>) -> Result<(), NumericalCheckpointFault> {
    let bytes = owner.capacity().checked_mul(size_of::<T>()).ok_or(NumericalCheckpointFault::Envelope)?;
    (bytes <= NUMERICAL_OWNER_PAGE_BYTES).then_some(()).ok_or(NumericalCheckpointFault::Envelope)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumericalOwnerShape {
    pub length: usize,
    pub capacity: usize,
    pub rows: usize,
    pub cols: usize,
}

#[derive(Default, Debug)]
pub struct NumericalOwnerRestoreCursor {
    pub shape: Option<NumericalOwnerShape>,
    pub next_item: usize,
    pub page_item: usize,
    pub page_owner: usize,
    pub page_values: usize,
    pub page_open: bool,
}

impl NumericalOwnerRestoreCursor {
    pub fn begin_page(&mut self, page: &NumericalPageView<'_>, maximum: usize, width: usize, matrix: bool) -> Result<NumericalOwnerShape, NumericalCheckpointFault> {
        if self.page_open {
            if page.item != self.page_item || page.owner != self.page_owner { return Err(NumericalCheckpointFault::Field); }
            return self.shape.ok_or(NumericalCheckpointFault::Field);
        }
        let prefix = if page.owner == 0 { if matrix { 24 } else { 8 } } else { 0 };
        let shape = match self.shape {
            Some(shape) if page.owner == 1 && page.item == self.next_item => shape,
            Some(_) => return Err(NumericalCheckpointFault::Field),
            None if page.owner == 0 && page.item == 0 => {
                if matrix {
                    let rows = read_checkpoint_usize(page.bytes, 0)?;
                    let cols = read_checkpoint_usize(page.bytes, 8)?;
                    let capacity = rows.checked_mul(cols).ok_or(NumericalCheckpointFault::Envelope)?;
                    let length = read_checkpoint_usize(page.bytes, 16)?;
                    if capacity > maximum || length > capacity { return Err(NumericalCheckpointFault::Envelope); }
                    NumericalOwnerShape { length, capacity, rows, cols }
                } else {
                    let length = declared_owner_length(page, maximum)?;
                    NumericalOwnerShape { length, capacity: length, rows: 0, cols: 0 }
                }
            }
            None => return Err(NumericalCheckpointFault::Field),
        };
        let remaining = shape.length.checked_sub(self.next_item).ok_or(NumericalCheckpointFault::Field)?;
        let page_values = remaining.min((semio_framework_job::JOB_PAYLOAD_PAGE_BYTES - NUMERICAL_CHECKPOINT_HEADER_BYTES - prefix) / width);
        if page.bytes.len() != prefix + page_values * width { return Err(NumericalCheckpointFault::Truncated); }
        self.shape = Some(shape);
        self.page_item = page.item;
        self.page_owner = page.owner;
        self.page_values = page_values;
        self.page_open = true;
        Ok(shape)
    }

    pub fn finish_page(&mut self) {
        if self.shape.is_some_and(|shape| self.next_item == shape.length) { *self = Self::default(); }
        else { self.page_open = false; }
    }
}

pub trait NumericalRestoreScalar: Sized {
    const WIDTH: usize;
    fn decode(bytes: &[u8], offset: usize) -> Result<Self, NumericalCheckpointFault>;
}

impl NumericalRestoreScalar for u32 {
    const WIDTH: usize = 4;
    fn decode(bytes: &[u8], offset: usize) -> Result<Self, NumericalCheckpointFault> { read_checkpoint_u32(bytes, offset) }
}

impl NumericalRestoreScalar for usize {
    const WIDTH: usize = 8;
    fn decode(bytes: &[u8], offset: usize) -> Result<Self, NumericalCheckpointFault> { read_checkpoint_usize(bytes, offset) }
}

impl NumericalRestoreScalar for f64 {
    const WIDTH: usize = 8;
    fn decode(bytes: &[u8], offset: usize) -> Result<Self, NumericalCheckpointFault> { Ok(f64::from_bits(read_checkpoint_u64(bytes, offset)?)) }
}

impl NumericalRestoreScalar for (u32, f64) {
    const WIDTH: usize = 12;
    fn decode(bytes: &[u8], offset: usize) -> Result<Self, NumericalCheckpointFault> {
        Ok((read_checkpoint_u32(bytes, offset)?, f64::from_bits(read_checkpoint_u64(bytes, offset.checked_add(4).ok_or(NumericalCheckpointFault::Truncated)?)?)))
    }
}

pub trait NumericalRestoreOwner<T> {
    fn restored_len(&self) -> usize;
    fn reserve_restore(&mut self, capacity: usize) -> Result<bool, NumericalCheckpointFault>;
    fn append_restore(&mut self, value: T) -> Result<(), NumericalCheckpointFault>;
}

impl<T> NumericalRestoreOwner<T> for Vec<T> {
    fn restored_len(&self) -> usize { self.len() }
    fn reserve_restore(&mut self, capacity: usize) -> Result<bool, NumericalCheckpointFault> {
        let bytes = capacity.checked_mul(size_of::<T>()).ok_or(NumericalCheckpointFault::Envelope)?;
        if bytes > NUMERICAL_OWNER_PAGE_BYTES { return Err(NumericalCheckpointFault::Envelope); }
        if self.capacity() < capacity { self.try_reserve_exact(capacity - self.len()).map_err(|_| NumericalCheckpointFault::Admission)?; }
        validate_restored_owner(self)?;
        Ok(true)
    }
    fn append_restore(&mut self, value: T) -> Result<(), NumericalCheckpointFault> {
        if self.len() == self.capacity() { return Err(NumericalCheckpointFault::Admission); }
        self.push(value);
        Ok(())
    }
}

impl<T, const N: usize> NumericalRestoreOwner<T> for PagedList<T, N> {
    fn restored_len(&self) -> usize { self.len() }
    fn reserve_restore(&mut self, capacity: usize) -> Result<bool, NumericalCheckpointFault> {
        if self.capacity() < capacity { self.reserve_capacity_one(capacity, SPARSE_PAGE_BYTES).map_err(|_| NumericalCheckpointFault::Admission)?; }
        Ok(self.capacity() >= capacity)
    }
    fn append_restore(&mut self, value: T) -> Result<(), NumericalCheckpointFault> { self.push_reserved(value).map_err(|_| NumericalCheckpointFault::Admission) }
}

pub fn restore_numerical_owner<T: NumericalRestoreScalar>(
    owner: &mut impl NumericalRestoreOwner<T>,
    page: &NumericalPageView<'_>,
    maximum: usize,
    entry: &mut usize,
    cursor: &mut NumericalOwnerRestoreCursor,
    matrix: bool,
) -> Result<bool, NumericalCheckpointFault> {
    let shape = cursor.begin_page(page, maximum, T::WIDTH, matrix)?;
    if owner.restored_len() != cursor.next_item { return Err(NumericalCheckpointFault::Field); }
    if *entry == 0 {
        if owner.reserve_restore(shape.capacity)? { *entry = 1; }
        return Ok(false);
    }
    let item = entry.checked_sub(1).ok_or(NumericalCheckpointFault::Field)?;
    if item > cursor.page_values || cursor.page_item.checked_add(item) != Some(cursor.next_item) { return Err(NumericalCheckpointFault::Field); }
    if item < cursor.page_values {
        let prefix = if page.owner == 0 { if matrix { 24 } else { 8 } } else { 0 };
        owner.append_restore(T::decode(page.bytes, prefix + item * T::WIDTH)?)?;
        cursor.next_item += 1;
        *entry += 1;
        return Ok(false);
    }
    cursor.finish_page();
    Ok(true)
}

pub fn restore_u32_entry(owner: &mut Vec<u32>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false)
}

pub fn restore_usize_entry(owner: &mut Vec<usize>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false)
}

pub fn restore_f64_entry(owner: &mut Vec<f64>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false)
}

pub fn restore_paged_u32_entry<const N: usize>(owner: &mut PagedList<u32, N>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false)
}

pub fn restore_paged_f64_entry<const N: usize>(owner: &mut PagedList<f64, N>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false)
}

pub fn restore_pair_entry(owner: &mut Vec<(u32, f64)>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor) -> Result<bool, NumericalCheckpointFault> {
    restore_numerical_owner(owner, page, maximum, entry, cursor, false)
}

// #region 🔖️Rcm
fn bfs_distances(start: usize, adjacency: &[Vec<usize>]) -> Vec<i64> {
    let n = adjacency.len();
    let mut dist = vec![-1i64; n];
    dist[start] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(u) = queue.pop_front() {
        for &v in &adjacency[u] {
            if dist[v] == -1 {
                dist[v] = dist[u] + 1;
                queue.push_back(v);
            }
        }
    }
    dist
}

fn farthest_node(start: usize, adjacency: &[Vec<usize>]) -> usize {
    let dist = bfs_distances(start, adjacency);
    (0..adjacency.len()).filter(|&i| dist[i] >= 0).max_by_key(|&i| dist[i]).unwrap_or(start)
}

/// 🧭️ George-Liu pseudo-peripheral heuristic: BFS to the farthest node, repeat twice more.
fn pseudo_peripheral(start: usize, adjacency: &[Vec<usize>]) -> usize {
    let a = farthest_node(start, adjacency);
    let b = farthest_node(a, adjacency);
    farthest_node(b, adjacency)
}

/// 🌀️ Reverse Cuthill-McKee ordering of an adjacency list (undirected graph: `adjacency[i]` =
/// neighbors of node `i`). Returns a permutation `perm` such that `perm[new_index] = old_index`.
/// Disconnected graphs are processed one component at a time, in order of first unvisited node;
/// each component is seeded from its pseudo-peripheral node, BFS'd with each level's nodes emitted
/// sorted by ascending degree, and the whole resulting order is reversed at the end.
pub fn rcm_order(adjacency: &[Vec<usize>]) -> Vec<usize> {
    let n = adjacency.len();
    let mut visited = vec![false; n];
    let mut order: Vec<usize> = Vec::with_capacity(n);

    for seed in 0..n {
        if visited[seed] {
            continue;
        }
        let root = pseudo_peripheral(seed, adjacency);
        visited[root] = true;
        let mut queue = VecDeque::new();
        queue.push_back(root);
        order.push(root);
        while let Some(u) = queue.pop_front() {
            let mut neighbors: Vec<usize> = adjacency[u].iter().copied().filter(|&v| !visited[v]).collect();
            neighbors.sort_by_key(|&v| adjacency[v].len());
            for v in neighbors {
                if !visited[v] {
                    visited[v] = true;
                    order.push(v);
                    queue.push_back(v);
                }
            }
        }
    }

    order.reverse();
    order
}
// #endregion 🔖️Rcm

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
