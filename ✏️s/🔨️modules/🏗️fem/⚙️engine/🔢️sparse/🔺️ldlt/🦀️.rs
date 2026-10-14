//! 🔺️ Envelope (skyline) sparse LDLT direct solver: the one-shot factor and its checkpointed interactive job.

use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, OriginalTurnFailure, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeView, JobPayloadAdmissionFault, JobPayloadStream, Operation, RetainedCloneGrant, RetainedCloneProgress, RetainedJobPayload, RetainedJobPayloadWriter, StepContext};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_s_fem_sparse::{numerical_close_original, CscSym, NUMERICAL_OWNER_PAGE_BYTES, NumericalCheckpointFault, NumericalCheckpointIdentity, NumericalOutcomeDesk, NumericalOwnerRestoreCursor, NumericalPageCursor, NumericalRun, advance_f64_owner, advance_numerical_page_header, advance_owner_length, advance_pair_owner, advance_u32_owner, advance_u64_owner, advance_u64_values, close_nested_vec_owner_step, close_vec_owner_step, declared_owner_length, numerical_close_gate, numerical_ladder_turn, numerical_owner_turn, numerical_restore_close_due, numerical_restore_frontier, numerical_restore_payload_close, numerical_rung_demand, numerical_writer_close, numerical_writer_demand, parse_numerical_page, read_checkpoint_u64, restore_f64_entry, restore_pair_entry, restore_u32_entry, restore_usize_entry, validate_restored_owner};
use semio_s_fem_algebra::{MatD, VecD};

// #region 🔖️Ldlt
/// ⚠️ Everything that can go wrong factoring a `CscSym`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SparseError {
    ZeroPivot { column: usize },
    DimensionMismatch,
}

/// 🧊️ A sparse LDLT factorization (unit lower `L`, diagonal `D`), permutation-agnostic
/// — a caller applying `rcm_order` reorders the matrix/RHS/solution indices itself before/after
/// calling into this module.
#[derive(Clone, Debug, PartialEq)]
pub struct LdltFactor {
    pub n: usize,
    pub l_cols: Vec<Vec<(u32, f64)>>,
    pub d: Vec<f64>,
}

/// 🧭️ The row envelope of a `CscSym`: `first[i]` is the leftmost column holding a stored entry of row
/// `i` (the diagonal when the row holds nothing to its left). Fill-in of an `LDLᵀ` factorization never
/// leaves the envelope, so `first` is also the exact sparsity every column of `L` lands in.
fn row_envelope(a: &CscSym) -> Vec<usize> {
    let mut first: Vec<usize> = (0..a.n).collect();
    for column in 0..a.n {
        for idx in a.colptr[column] as usize..a.colptr[column + 1] as usize {
            let row = a.rowind[idx] as usize;
            if column < first[row] {
                first[row] = column;
            }
        }
    }
    first
}

/// 🧮️ Envelope (skyline) LDLT, row-Crout order: `L` and `D` are computed row by row inside the row
/// envelope of `A`, where every fill-in entry of an `LDLᵀ` factorization provably lands, so the
/// factorization touches `Σᵢ (i − first[i])²/2` flops on a contiguous per-row buffer instead of a map
/// per column. Under `rcm_order` a finite-element mesh's envelope is its (small) bandwidth, which is
/// what turns a two-thousand-node continuum model from minutes into a fraction of a second. Every
/// entry is formed by the SAME floating-point operations, in the same order, as the checkpointed
/// left-looking `LdltJob` — `A[i][k] − Σₘ (L[k][m]·D[m])·L[i][m]`, ascending `m`, the scaled column
/// factor `L[k][m]·D[m]` kept beside `L` — so the batch factor and the resumable job stay
/// bit-identical (the job's parity tests hold them to `==`). The result is the same unit-lower `L` /
/// diagonal `D` pair in the same column-sorted storage as before, explicit zeros dropped.
pub fn ldlt_factor(a: &CscSym) -> Result<LdltFactor, SparseError> {
    let n = a.n;
    let first = row_envelope(a);
    let mut offset = vec![0usize; n + 1];
    for row in 0..n {
        offset[row + 1] = offset[row] + (row - first[row] + 1);
    }
    let mut envelope = vec![0.0f64; offset[n]];
    for column in 0..n {
        for idx in a.colptr[column] as usize..a.colptr[column + 1] as usize {
            let row = a.rowind[idx] as usize;
            envelope[offset[row] + (column - first[row])] += a.vals[idx];
        }
    }
    let mut scaled = vec![0.0f64; offset[n]];
    let mut d = vec![0.0f64; n];
    for row in 0..n {
        let base = offset[row];
        let start = first[row];
        for column in start..row {
            let mut g = envelope[base + (column - start)];
            let column_base = offset[column];
            let column_start = first[column];
            for m in start.max(column_start)..column {
                g -= scaled[column_base + (m - column_start)] * envelope[base + (m - start)];
            }
            let l = g / d[column];
            envelope[base + (column - start)] = l;
            scaled[base + (column - start)] = l * d[column];
        }
        let mut pivot = envelope[base + (row - start)];
        for column in start..row {
            pivot -= scaled[base + (column - start)] * envelope[base + (column - start)];
        }
        if pivot.abs() < 1e-12 {
            return Err(SparseError::ZeroPivot { column: row });
        }
        d[row] = pivot;
    }
    let mut l_cols: Vec<Vec<(u32, f64)>> = vec![Vec::new(); n];
    for row in 0..n {
        for column in first[row]..row {
            let value = envelope[offset[row] + (column - first[row])];
            if value != 0.0 {
                l_cols[column].push((row as u32, value));
            }
        }
    }
    Ok(LdltFactor { n, l_cols, d })
}

#[derive(Clone, Debug, PartialEq)]
pub struct LdltPreview {
    pub completed_columns: usize,
    pub total_columns: usize,
    pub negative_pivots: usize,
}

#[derive(Clone, PartialEq)]
pub struct LdltCheckpoint {
    identity: NumericalCheckpointIdentity,
    a: CscSym,
    l_cols: Vec<Vec<(u32, f64)>>,
    d: Vec<f64>,
    row_lists: Vec<Vec<usize>>,
    column: usize,
    cursor: LdltColumnCursor,
    workspace: LdltColumnWorkspace,
    admission_fault: bool,
    reserve_lane: u8,
    reserve_cursor: usize,
    checkpoint_due: bool,
    publication_stage: u8,
    publication_outer: usize,
    publication_inner: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LdltColumnStage {
    ReserveColumn,
    SourceEntry,
    ContributorLookup,
    ContributorEntry,
    PivotRead,
    DiagonalCommit,
    EmitRow,
    PublishColumn,
    CompleteColumn,
}

#[derive(Clone, PartialEq)]
struct LdltColumnCursor {
    stage: LdltColumnStage,
    source: usize,
    contributor: usize,
    entry: usize,
    emit_row: usize,
    active_column: usize,
    lookup_lower: usize,
    lookup_upper: usize,
    lookup_mid: usize,
    lookup_comparison: i8,
    lookup_initialized: bool,
    factor: f64,
    pivot: f64,
}

#[derive(Clone, PartialEq)]
struct LdltColumnWorkspace {
    values: Vec<f64>,
    marks: Vec<u32>,
    generation: u32,
    candidate: Vec<(u32, f64)>,
}

const LDLT_MAXIMUM_ORDER: usize = 40;

pub struct LdltJob {
    operation: Operation,
    state: LdltCheckpoint,
    output_writer: Option<RetainedJobPayloadWriter>,
    output_page_cursor: usize,
    checkpoint_writer: Option<RetainedJobPayloadWriter>,
    checkpoint_cursor: NumericalPageCursor,
    desk: NumericalOutcomeDesk,
}

fn close_ldlt_checkpoint_step(state: &mut LdltCheckpoint, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
    if let Some(step) = close_nested_vec_owner_step(&mut state.row_lists, maximum_bytes)? { return Ok(Some(step)); }
    if let Some(step) = close_nested_vec_owner_step(&mut state.l_cols, maximum_bytes)? { return Ok(Some(step)); }
    if let Some(step) = close_vec_owner_step(&mut state.workspace.candidate, maximum_bytes)? { return Ok(Some(step)); }
    for owner in [&mut state.d, &mut state.a.vals, &mut state.workspace.values] {
        if let Some(step) = close_vec_owner_step(owner, maximum_bytes)? { return Ok(Some(step)); }
    }
    for owner in [&mut state.a.rowind, &mut state.a.colptr, &mut state.workspace.marks] {
        if let Some(step) = close_vec_owner_step(owner, maximum_bytes)? { return Ok(Some(step)); }
    }
    Ok(None)
}

impl LdltJob {
    pub fn new(operation: Operation, a: CscSym, columns_per_step: usize) -> Self {
        assert!(columns_per_step > 0, "ldlt batch must contain work");
        let n = a.n;
        let input_pages_valid = a.colptr.capacity().saturating_mul(size_of::<u32>()) <= NUMERICAL_OWNER_PAGE_BYTES
            && a.rowind.capacity().saturating_mul(size_of::<u32>()) <= NUMERICAL_OWNER_PAGE_BYTES
            && a.vals.capacity().saturating_mul(size_of::<f64>()) <= NUMERICAL_OWNER_PAGE_BYTES;
        let admission_fault = n > LDLT_MAXIMUM_ORDER || a.colptr.len() != n.saturating_add(1) || a.rowind.len() != a.vals.len() || !input_pages_valid;
        let workspace = LdltColumnWorkspace { values: Vec::new(), marks: Vec::new(), generation: 0, candidate: Vec::new() };
        let cursor = LdltColumnCursor {
            stage: LdltColumnStage::ReserveColumn,
            source: 0,
            contributor: 0,
            entry: 0,
            emit_row: 0,
            active_column: 0,
            lookup_lower: 0,
            lookup_upper: 0,
            lookup_mid: 0,
            lookup_comparison: 0,
            lookup_initialized: false,
            factor: 0.0,
            pivot: 0.0,
        };
        Self {
            operation,
            state: LdltCheckpoint {
                identity: NumericalCheckpointIdentity::from_operation(operation),
                a,
                l_cols: Vec::new(),
                d: Vec::new(),
                row_lists: Vec::new(),
                column: 0,
                cursor,
                workspace,
                admission_fault,
                reserve_lane: 0,
                reserve_cursor: 0,
                checkpoint_due: false,
                publication_stage: 0,
                publication_outer: 0,
                publication_inner: 0,
            },
            output_writer: None,
            output_page_cursor: 0,
            checkpoint_writer: None,
            checkpoint_cursor: NumericalPageCursor::new(),
            desk: NumericalOutcomeDesk::default(),
        }
    }

    pub fn preview(&self) -> LdltPreview {
        LdltPreview { completed_columns: self.state.column, total_columns: self.state.a.n, negative_pivots: self.state.d[..self.state.column].iter().filter(|&&value| value < 0.0).count() }
    }

    pub fn factor(&self) -> Option<LdltFactor> {
        (self.state.column == self.state.a.n).then(|| LdltFactor { n: self.state.a.n, l_cols: self.state.l_cols.clone(), d: self.state.d.clone() })
    }

    /// 🎼 Transfers the completed factor into the retained subspace child without cloning.
    pub fn take_factor(&mut self) -> Option<LdltFactor> {
        (self.state.column == self.state.a.n).then(|| LdltFactor { n: self.state.a.n, l_cols: std::mem::take(&mut self.state.l_cols), d: std::mem::take(&mut self.state.d) })
    }

    fn workspace_add(&mut self, row: usize, value: f64) {
        if self.state.workspace.marks[row] != self.state.workspace.generation {
            self.state.workspace.marks[row] = self.state.workspace.generation;
            self.state.workspace.values[row] = 0.0;
        }
        self.state.workspace.values[row] += value;
    }

    fn workspace_get(&self, row: usize) -> f64 {
        if self.state.workspace.marks[row] == self.state.workspace.generation {
            self.state.workspace.values[row]
        } else {
            0.0
        }
    }

    fn reserve_fixed_owner<T>(owner: &mut Vec<T>, items: usize) -> Result<(), SparseError> {
        owner.try_reserve_exact(items).map_err(|_| SparseError::DimensionMismatch)?;
        let bytes = owner.capacity().checked_mul(size_of::<T>()).ok_or(SparseError::DimensionMismatch)?;
        if bytes > NUMERICAL_OWNER_PAGE_BYTES {
            return Err(SparseError::DimensionMismatch);
        }
        Ok(())
    }

    fn advance_reservation(&mut self) -> Result<bool, SparseError> {
        let n = self.state.a.n;
        match self.state.reserve_lane {
            0 => {
                Self::reserve_fixed_owner(&mut self.state.row_lists, n)?;
                self.state.reserve_lane = 1;
            }
            1 => {
                if self.state.reserve_cursor < n {
                    let mut owner = Vec::new();
                    Self::reserve_fixed_owner(&mut owner, self.state.reserve_cursor)?;
                    self.state.row_lists.push(owner);
                    self.state.reserve_cursor += 1;
                } else {
                    self.state.reserve_cursor = 0;
                    self.state.reserve_lane = 2;
                }
            }
            2 => {
                Self::reserve_fixed_owner(&mut self.state.l_cols, n)?;
                self.state.reserve_lane = 3;
            }
            3 => {
                if self.state.reserve_cursor < n {
                    let mut owner = Vec::new();
                    Self::reserve_fixed_owner(&mut owner, n.saturating_sub(self.state.reserve_cursor + 1))?;
                    self.state.l_cols.push(owner);
                    self.state.reserve_cursor += 1;
                } else {
                    self.state.reserve_cursor = 0;
                    self.state.reserve_lane = 4;
                }
            }
            4 => {
                if self.state.d.capacity() == 0 {
                    Self::reserve_fixed_owner(&mut self.state.d, n)?;
                } else if self.state.d.len() < n {
                    self.state.d.push(0.0);
                } else {
                    self.state.reserve_lane = 5;
                }
            }
            5 => {
                if self.state.workspace.values.capacity() == 0 {
                    Self::reserve_fixed_owner(&mut self.state.workspace.values, n)?;
                } else if self.state.workspace.values.len() < n {
                    self.state.workspace.values.push(0.0);
                } else {
                    self.state.reserve_lane = 6;
                }
            }
            6 => {
                if self.state.workspace.marks.capacity() == 0 {
                    Self::reserve_fixed_owner(&mut self.state.workspace.marks, n)?;
                } else if self.state.workspace.marks.len() < n {
                    self.state.workspace.marks.push(0);
                } else {
                    self.state.reserve_lane = 7;
                }
            }
            7 => {
                Self::reserve_fixed_owner(&mut self.state.workspace.candidate, n)?;
                self.state.reserve_lane = 8;
            }
            8 => {
                self.state.reserve_lane = 9;
            }
            _ => return Ok(true),
        }
        Ok(self.state.reserve_lane == 9)
    }

    fn advance_column_microcursor(&mut self) -> Result<(), SparseError> {
        if self.state.reserve_lane < 9 {
            self.advance_reservation()?;
            return Ok(());
        }
        let column = self.state.column;
        match self.state.cursor.stage {
            LdltColumnStage::ReserveColumn => {
                let Some(generation) = self.state.workspace.generation.checked_add(1) else {
                    return Err(SparseError::DimensionMismatch);
                };
                self.state.workspace.generation = generation;
                self.state.workspace.candidate.clear();
                self.state.cursor.source = self.state.a.colptr[column] as usize;
                self.state.cursor.contributor = 0;
                self.state.cursor.entry = 0;
                self.state.cursor.active_column = 0;
                self.state.cursor.lookup_lower = 0;
                self.state.cursor.lookup_upper = 0;
                self.state.cursor.lookup_mid = 0;
                self.state.cursor.lookup_comparison = 0;
                self.state.cursor.lookup_initialized = false;
                self.state.cursor.emit_row = column.saturating_add(1);
                self.state.cursor.stage = LdltColumnStage::SourceEntry;
            }
            LdltColumnStage::SourceEntry => {
                let end = self.state.a.colptr[column + 1] as usize;
                if self.state.cursor.source < end {
                    let index = self.state.cursor.source;
                    let row = self.state.a.rowind[index] as usize;
                    self.workspace_add(row, self.state.a.vals[index]);
                    self.state.cursor.source += 1;
                } else {
                    self.state.cursor.stage = LdltColumnStage::ContributorLookup;
                }
            }
            LdltColumnStage::ContributorLookup => {
                if self.state.cursor.contributor < self.state.row_lists[column].len() {
                    let active = self.state.row_lists[column][self.state.cursor.contributor];
                    if !self.state.cursor.lookup_initialized {
                        self.state.cursor.active_column = active;
                        self.state.cursor.lookup_upper = self.state.l_cols[active].len();
                        self.state.cursor.lookup_mid = 0;
                        self.state.cursor.lookup_comparison = 0;
                        self.state.cursor.lookup_initialized = true;
                    } else if self.state.cursor.lookup_lower < self.state.cursor.lookup_upper {
                        let mid = self.state.cursor.lookup_lower + (self.state.cursor.lookup_upper - self.state.cursor.lookup_lower) / 2;
                        let comparison = self.state.l_cols[active][mid].0.cmp(&(column as u32));
                        self.state.cursor.lookup_mid = mid;
                        self.state.cursor.lookup_comparison = match comparison {
                            std::cmp::Ordering::Less => -1,
                            std::cmp::Ordering::Equal => 0,
                            std::cmp::Ordering::Greater => 1,
                        };
                        if comparison == std::cmp::Ordering::Less {
                            self.state.cursor.lookup_lower = mid + 1;
                        } else {
                            self.state.cursor.lookup_upper = mid;
                        }
                    } else {
                        let index = self.state.cursor.lookup_lower;
                        self.state.cursor.factor = self.state.l_cols[active].get(index).filter(|entry| entry.0 == column as u32).map_or(0.0, |entry| entry.1 * self.state.d[active]);
                        self.state.cursor.entry = 0;
                        self.state.cursor.lookup_lower = 0;
                        self.state.cursor.lookup_upper = 0;
                        self.state.cursor.lookup_mid = 0;
                        self.state.cursor.lookup_comparison = 0;
                        self.state.cursor.lookup_initialized = false;
                        self.state.cursor.contributor += 1;
                        self.state.cursor.stage = LdltColumnStage::ContributorEntry;
                    }
                } else {
                    self.state.cursor.stage = LdltColumnStage::PivotRead;
                }
            }
            LdltColumnStage::ContributorEntry => {
                let active = self.state.cursor.active_column;
                if self.state.cursor.factor == 0.0 || self.state.cursor.entry >= self.state.l_cols[active].len() {
                    self.state.cursor.stage = LdltColumnStage::ContributorLookup;
                } else {
                    let (row, value) = self.state.l_cols[active][self.state.cursor.entry];
                    if row as usize >= column {
                        self.workspace_add(row as usize, -self.state.cursor.factor * value);
                    }
                    self.state.cursor.entry += 1;
                }
            }
            LdltColumnStage::PivotRead => {
                self.state.cursor.pivot = self.workspace_get(column);
                if self.state.cursor.pivot.abs() < 1e-12 {
                    return Err(SparseError::ZeroPivot { column });
                }
                self.state.cursor.stage = LdltColumnStage::DiagonalCommit;
            }
            LdltColumnStage::DiagonalCommit => {
                self.state.d[column] = self.state.cursor.pivot;
                self.state.cursor.stage = LdltColumnStage::EmitRow;
            }
            LdltColumnStage::EmitRow => {
                if self.state.cursor.emit_row < self.state.a.n {
                    let row = self.state.cursor.emit_row;
                    let value = self.workspace_get(row);
                    if value != 0.0 {
                        self.state.workspace.candidate.push((row as u32, value / self.state.cursor.pivot));
                    }
                    self.state.cursor.emit_row += 1;
                } else {
                    self.state.cursor.entry = 0;
                    self.state.cursor.stage = LdltColumnStage::PublishColumn;
                }
            }
            LdltColumnStage::PublishColumn => {
                if self.state.cursor.entry < self.state.workspace.candidate.len() {
                    let entry = self.state.workspace.candidate[self.state.cursor.entry];
                    self.state.l_cols[column].push(entry);
                    self.state.row_lists[entry.0 as usize].push(column);
                    self.state.cursor.entry += 1;
                } else {
                    self.state.cursor.stage = LdltColumnStage::CompleteColumn;
                }
            }
            LdltColumnStage::CompleteColumn => {
                self.state.column += 1;
                self.state.cursor.stage = LdltColumnStage::ReserveColumn;
                self.state.checkpoint_due = self.state.column < self.state.a.n;
            }
        }
        Ok(())
    }

    fn advance_checkpoint_entry(state: &LdltCheckpoint, cursor: &mut NumericalPageCursor, writer: &mut RetainedJobPayloadWriter) -> Result<bool, JobPayloadAdmissionFault> {
        let n = state.a.n;
        let row_base = 520u16;
        if advance_numerical_page_header(writer, b"FEMLCP1\0", 11, *cursor)? {
            return Ok(false);
        }
        let complete = match cursor.field {
            0 => {
                let values = [
                    state.identity.operation,
                    state.identity.revision,
                    state.identity.generation,
                    state.identity.seed,
                    n as u64,
                    state.column as u64,
                    state.cursor.stage as u64,
                    state.cursor.source as u64,
                    state.cursor.contributor as u64,
                    state.cursor.entry as u64,
                    state.cursor.emit_row as u64,
                    state.cursor.active_column as u64,
                    state.cursor.factor.to_bits(),
                    state.cursor.pivot.to_bits(),
                    state.admission_fault as u64,
                    state.reserve_lane as u64,
                    state.reserve_cursor as u64,
                    state.publication_stage as u64,
                    state.publication_outer as u64,
                    state.publication_inner as u64,
                    state.workspace.generation as u64,
                    state.cursor.lookup_lower as u64,
                    state.cursor.lookup_upper as u64,
                    state.cursor.lookup_mid as u64,
                    state.cursor.lookup_comparison as i64 as u64,
                    state.cursor.lookup_initialized as u64,
                ];
                advance_u64_values(writer, &values, cursor)?
            }
            1 => advance_u32_owner(writer, &state.a.colptr, cursor)?,
            2 => advance_u32_owner(writer, &state.a.rowind, cursor)?,
            3 => advance_f64_owner(writer, &state.a.vals, cursor)?,
            4 => advance_owner_length(writer, state.l_cols.len(), cursor)?,
            field if field >= 5 && field < 5 + state.l_cols.len() as u16 => advance_pair_owner(writer, &state.l_cols[(field - 5) as usize], cursor)?,
            517 => advance_f64_owner(writer, &state.d, cursor)?,
            518 => advance_owner_length(writer, state.row_lists.len(), cursor)?,
            field if field >= row_base && field < row_base + state.row_lists.len() as u16 => advance_u64_owner(writer, &state.row_lists[(field - row_base) as usize], cursor)?,
            1032 => advance_f64_owner(writer, &state.workspace.values, cursor)?,
            1033 => advance_u32_owner(writer, &state.workspace.marks, cursor)?,
            1034 => advance_pair_owner(writer, &state.workspace.candidate, cursor)?,
            _ => return Ok(true),
        };
        if complete {
            writer.commit_staged_page()?;
            cursor.item = 0;
            cursor.owner = 0;
            cursor.field = match cursor.field {
                4 if state.l_cols.is_empty() => 517,
                4 => 5,
                field if field >= 5 && field + 1 < 5 + state.l_cols.len() as u16 => field + 1,
                field if field >= 5 && field < 5 + state.l_cols.len() as u16 => 517,
                517 => 518,
                518 if state.row_lists.is_empty() => 1032,
                518 => row_base,
                field if field >= row_base && field + 1 < row_base + state.row_lists.len() as u16 => field + 1,
                field if field >= row_base && field < row_base + state.row_lists.len() as u16 => 1032,
                1032 | 1033 => cursor.field + 1,
                1034 => u16::MAX,
                field => field + 1,
            };
        }
        Ok(cursor.field == u16::MAX)
    }

    fn advance_output_entry(state: &mut LdltCheckpoint, operation: Operation, writer: &mut RetainedJobPayloadWriter) -> Result<bool, JobPayloadAdmissionFault> {
        let cursor = NumericalPageCursor { field: state.publication_stage as u16, owner: state.publication_outer, item: state.publication_inner };
        if advance_numerical_page_header(writer, b"FEMLDL2\0", 1, cursor)? {
            return Ok(false);
        }
        match state.publication_stage {
            0 => {
                let values = [operation.operation.0, operation.base_revision.0, operation.generation.0, operation.seed, state.a.n as u64];
                if let Some(value) = values.get(state.publication_outer) {
                    writer.write_staged(&value.to_le_bytes())?;
                    state.publication_outer += 1;
                } else {
                    writer.commit_staged_page()?;
                    state.publication_outer = 0;
                    state.publication_stage = 1;
                }
            }
            1 => {
                if let Some(value) = state.d.get(state.publication_outer) {
                    writer.write_staged(&value.to_bits().to_le_bytes())?;
                    state.publication_outer += 1;
                } else {
                    writer.commit_staged_page()?;
                    state.publication_outer = 0;
                    state.publication_stage = 2;
                }
            }
            2 => {
                if state.publication_outer == state.l_cols.len() {
                    writer.commit_staged_page()?;
                    state.publication_stage = 3;
                } else {
                    let column = &state.l_cols[state.publication_outer];
                    if state.publication_inner == 0 {
                        writer.write_staged(&(column.len() as u64).to_le_bytes())?;
                        state.publication_inner = 1;
                    } else if let Some((row, value)) = column.get(state.publication_inner - 1) {
                        let mut bytes = [0; 12];
                        bytes[..4].copy_from_slice(&row.to_le_bytes());
                        bytes[4..].copy_from_slice(&value.to_bits().to_le_bytes());
                        writer.write_staged(&bytes)?;
                        state.publication_inner += 1;
                    } else {
                        state.publication_inner = 0;
                        state.publication_outer += 1;
                    }
                }
            }
            _ => {}
        }
        Ok(state.publication_stage == 3)
    }

    fn close_retained_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        match close_ldlt_checkpoint_step(&mut self.state, maximum_bytes) {
            Ok(Some((items, bytes))) => (false, items, bytes),
            Ok(None) => (true, 0, 0),
            Err(()) => (false, 0, 0),
        }
    }

    fn close_terminal_is_empty(&self) -> bool {
        self.state.row_lists.is_empty()
            && self.state.row_lists.capacity() == 0
            && self.state.l_cols.is_empty()
            && self.state.l_cols.capacity() == 0
            && self.state.d.capacity() == 0
            && self.state.a.vals.capacity() == 0
            && self.state.a.rowind.capacity() == 0
            && self.state.a.colptr.capacity() == 0
            && self.state.workspace.values.capacity() == 0
            && self.state.workspace.marks.capacity() == 0
            && self.state.workspace.candidate.capacity() == 0
    }
}

pub struct LdltRestoreCursor {
    pub operation: Operation,
    pub payload: Option<RetainedJobPayload>,
    pub total_pages: usize,
    pub page_slot: usize,
    pub close_due: bool,
    pub expected_field: u16,
    pub page_entry: usize,
    pub owner_cursor: NumericalOwnerRestoreCursor,
    pub control: [u64; 32],
    pub state: Option<LdltCheckpoint>,
    pub fault: Option<NumericalCheckpointFault>,
}

impl LdltRestoreCursor {
    pub fn new(operation: Operation, payload: RetainedJobPayload) -> Self {
        let total_pages = payload.page_count();
        Self { operation, payload: Some(payload), total_pages, page_slot: 0, close_due: false, expected_field: 0, page_entry: 0, owner_cursor: NumericalOwnerRestoreCursor::default(), control: [0; 32], state: None, fault: None }
    }

    fn decode_page_entry(&mut self, bytes: &[u8]) -> Result<bool, NumericalCheckpointFault> {
        let page = parse_numerical_page(bytes, b"FEMLCP1\0")?;
        if page.kind != 11 || page.field != self.expected_field || (self.owner_cursor.shape.is_none() && (page.owner != 0 || page.item != 0)) {
            return Err(NumericalCheckpointFault::Field);
        }
        if page.field == 0 {
            let count = declared_owner_length(&page, 32)?;
            if count != 26 || page.bytes.len() != 8 + count * 8 {
                return Err(NumericalCheckpointFault::Truncated);
            }
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
            if n > LDLT_MAXIMUM_ORDER {
                return Err(NumericalCheckpointFault::Envelope);
            }
            let stage = match value(6) {
                0 => LdltColumnStage::ReserveColumn,
                1 => LdltColumnStage::SourceEntry,
                2 => LdltColumnStage::ContributorLookup,
                3 => LdltColumnStage::ContributorEntry,
                4 => LdltColumnStage::PivotRead,
                5 => LdltColumnStage::DiagonalCommit,
                6 => LdltColumnStage::EmitRow,
                7 => LdltColumnStage::PublishColumn,
                8 => LdltColumnStage::CompleteColumn,
                _ => return Err(NumericalCheckpointFault::Field),
            };
            self.state = Some(LdltCheckpoint {
                identity,
                a: CscSym { n, colptr: Vec::new(), rowind: Vec::new(), vals: Vec::new() },
                l_cols: Vec::new(),
                d: Vec::new(),
                row_lists: Vec::new(),
                column: value(5) as usize,
                cursor: LdltColumnCursor {
                    stage,
                    source: value(7) as usize,
                    contributor: value(8) as usize,
                    entry: value(9) as usize,
                    emit_row: value(10) as usize,
                    active_column: value(11) as usize,
                    lookup_lower: value(21) as usize,
                    lookup_upper: value(22) as usize,
                    lookup_mid: value(23) as usize,
                    lookup_comparison: value(24) as i64 as i8,
                    lookup_initialized: value(25) != 0,
                    factor: f64::from_bits(value(12)),
                    pivot: f64::from_bits(value(13)),
                },
                workspace: LdltColumnWorkspace { values: Vec::new(), marks: Vec::new(), generation: value(20) as u32, candidate: Vec::new() },
                admission_fault: value(14) != 0,
                reserve_lane: value(15) as u8,
                reserve_cursor: value(16) as usize,
                checkpoint_due: false,
                publication_stage: value(17) as u8,
                publication_outer: value(18) as usize,
                publication_inner: value(19) as usize,
            });
            self.expected_field = 1;
            return Ok(true);
        }
        let state = self.state.as_mut().ok_or(NumericalCheckpointFault::Field)?;
        let n = state.a.n;
        let row_base = 520u16;
        let complete = match page.field {
            1 => restore_u32_entry(&mut state.a.colptr, &page, n + 1, &mut self.page_entry, &mut self.owner_cursor)?,
            2 => restore_u32_entry(&mut state.a.rowind, &page, n.saturating_mul(n), &mut self.page_entry, &mut self.owner_cursor)?,
            3 => restore_f64_entry(&mut state.a.vals, &page, n.saturating_mul(n), &mut self.page_entry, &mut self.owner_cursor)?,
            4 => {
                let length = declared_owner_length(&page, n)?;
                if page.bytes.len() != 8 {
                    return Err(NumericalCheckpointFault::Field);
                }
                if self.page_entry == 0 {
                    state.l_cols.try_reserve_exact(length).map_err(|_| NumericalCheckpointFault::Admission)?;
                    validate_restored_owner(&state.l_cols)?;
                    self.page_entry = 1;
                    return Ok(false);
                }
                if state.l_cols.len() < length {
                    state.l_cols.push(Vec::new());
                    self.page_entry += 1;
                    return Ok(false);
                }
                true
            }
            field if field >= 5 && field < 5 + state.l_cols.len() as u16 => restore_pair_entry(&mut state.l_cols[(field - 5) as usize], &page, n, &mut self.page_entry, &mut self.owner_cursor)?,
            517 => restore_f64_entry(&mut state.d, &page, n, &mut self.page_entry, &mut self.owner_cursor)?,
            518 => {
                let length = declared_owner_length(&page, n)?;
                if page.bytes.len() != 8 {
                    return Err(NumericalCheckpointFault::Field);
                }
                if self.page_entry == 0 {
                    state.row_lists.try_reserve_exact(length).map_err(|_| NumericalCheckpointFault::Admission)?;
                    validate_restored_owner(&state.row_lists)?;
                    self.page_entry = 1;
                    return Ok(false);
                }
                if state.row_lists.len() < length {
                    state.row_lists.push(Vec::new());
                    self.page_entry += 1;
                    return Ok(false);
                }
                true
            }
            field if field >= row_base && field < row_base + state.row_lists.len() as u16 => restore_usize_entry(&mut state.row_lists[(field - row_base) as usize], &page, n, &mut self.page_entry, &mut self.owner_cursor)?,
            1032 => restore_f64_entry(&mut state.workspace.values, &page, n, &mut self.page_entry, &mut self.owner_cursor)?,
            1033 => restore_u32_entry(&mut state.workspace.marks, &page, n, &mut self.page_entry, &mut self.owner_cursor)?,
            1034 => restore_pair_entry(&mut state.workspace.candidate, &page, n, &mut self.page_entry, &mut self.owner_cursor)?,
            _ => return Err(NumericalCheckpointFault::Field),
        };
        if complete && self.owner_cursor.shape.is_none() {
            self.expected_field = match page.field {
                4 if state.l_cols.is_empty() => 517,
                4 => 5,
                field if field >= 5 && field + 1 < 5 + state.l_cols.len() as u16 => field + 1,
                field if field >= 5 && field < 5 + state.l_cols.len() as u16 => 517,
                517 => 518,
                518 if state.row_lists.is_empty() => 1032,
                518 => row_base,
                field if field >= row_base && field + 1 < row_base + state.row_lists.len() as u16 => field + 1,
                field if field >= row_base && field < row_base + state.row_lists.len() as u16 => 1032,
                1032 | 1033 => page.field + 1,
                1034 => u16::MAX,
                field => field + 1,
            };
        }
        Ok(complete)
    }

    pub fn step(&mut self, context: &mut StepContext<'_>) -> Result<Option<LdltJob>, NumericalCheckpointFault> {
        if let Some(fault) = self.fault { return Err(fault); }
        let outcome = self.advance(context);
        if let Err(fault) = outcome { self.fault = Some(fault); }
        outcome
    }

    fn advance(&mut self, context: &mut StepContext<'_>) -> Result<Option<LdltJob>, NumericalCheckpointFault> {
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
        if self.close_due {
            let payload = self.payload.as_mut().ok_or(NumericalCheckpointFault::Truncated)?;
            if numerical_restore_close_due(payload, context, &mut self.page_slot, &mut self.page_entry, &mut self.close_due, self.total_pages)? {
                if self.expected_field != u16::MAX {
                    return Err(NumericalCheckpointFault::Truncated);
                }
                self.payload = None;
                let state = self.state.take().ok_or(NumericalCheckpointFault::Truncated)?;
                return Ok(Some(LdltJob { operation: self.operation, state, output_writer: None, output_page_cursor: 0, checkpoint_writer: None, checkpoint_cursor: NumericalPageCursor::new(), desk: NumericalOutcomeDesk::default() }));
            }
            return Ok(None);
        }
        let payload = self.payload.take().ok_or(NumericalCheckpointFault::Truncated)?;
        let decoded = payload.page(self.page_slot).ok_or(NumericalCheckpointFault::Truncated).and_then(|source| self.decode_page_entry(source));
        self.payload = Some(payload);
        match decoded {
            Ok(true) => self.close_due = true,
            Ok(false) => {}
            Err(fault) => return Err(fault),
        }
        Ok(None)
    }

    fn close_frontier(&self) -> Result<Option<RetirementDemand>, ValueError> {
        numerical_restore_frontier(&self.payload)
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
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
        let step = match numerical_owner_turn(close_ldlt_checkpoint_step(state, grant.maximum_release_bytes)) {
            Some(step) => step,
            None => {
                self.state = None;
                InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }
            }
        };
        step.admit(grant, self.terminal_is_empty())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.payload.is_none() && self.state.is_none()
    }
}

impl LdltJob {
    fn run(&mut self, context: &mut StepContext<'_>) -> NumericalRun {
        if context.is_cancelled() {
            return NumericalRun::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return NumericalRun::Fault;
        }
        if self.state.admission_fault {
            return NumericalRun::Fault;
        }
        if self.state.checkpoint_due || self.checkpoint_writer.is_some() {
            context.set_stage("fem.ldlt.checkpoint-page");
            if context.should_yield() {
                return NumericalRun::Yield;
            }
            context.consume_fuel(1);
            if self.checkpoint_writer.is_none() {
                self.state.checkpoint_due = false;
                self.checkpoint_cursor = NumericalPageCursor::new();
                self.checkpoint_writer = Some(RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState));
                return NumericalRun::Yield;
            }
            let Some(writer) = self.checkpoint_writer.as_mut() else {
                return NumericalRun::Fault;
            };
            if writer.staged_page_len().is_none() {
                return match writer.begin_staged_page(context) {
                    Ok(()) => NumericalRun::Yield,
                    Err(_) => NumericalRun::Fault,
                };
            }
            let complete = match Self::advance_checkpoint_entry(&self.state, &mut self.checkpoint_cursor, writer) {
                Ok(complete) => complete,
                Err(_) => return NumericalRun::Fault,
            };
            if !complete {
                return NumericalRun::Yield;
            }
            let Some(writer) = self.checkpoint_writer.take() else {
                return NumericalRun::Fault;
            };
            let state = match writer.finish() {
                Ok(state) => state,
                Err(writer) => {
                    self.checkpoint_writer = Some(writer);
                    return NumericalRun::Fault;
                }
            };
            return self.desk.lend_checkpoint(state, self.state.column as u64);
        }
        if self.state.column == self.state.a.n {
            context.set_stage("fem.ldlt.output-entry");
            if context.should_yield() {
                return NumericalRun::Yield;
            }
            if self.output_writer.is_none() {
                context.consume_fuel(1);
                self.output_writer = Some(RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput));
                self.output_page_cursor = 0;
                return NumericalRun::Yield;
            }
            context.consume_fuel(1);
            let Some(writer) = self.output_writer.as_mut() else {
                return NumericalRun::Fault;
            };
            if writer.staged_page_len().is_none() {
                return match writer.begin_staged_page(context) {
                    Ok(()) => NumericalRun::Yield,
                    Err(_) => NumericalRun::Fault,
                };
            }
            let complete = match Self::advance_output_entry(&mut self.state, self.operation, writer) {
                Ok(complete) => complete,
                Err(_) => return NumericalRun::Fault,
            };
            if !complete {
                return NumericalRun::Yield;
            }
            let Some(writer) = self.output_writer.take() else {
                return NumericalRun::Fault;
            };
            let output = match writer.finish() {
                Ok(output) => output,
                Err(writer) => {
                    self.output_writer = Some(writer);
                    return NumericalRun::Fault;
                }
            };
            return self.desk.lend_complete(output);
        }
        context.set_stage(match self.state.cursor.stage {
            LdltColumnStage::ReserveColumn => "fem.ldlt.reserve-column",
            LdltColumnStage::SourceEntry => "fem.ldlt.source-entry",
            LdltColumnStage::ContributorLookup => "fem.ldlt.contributor-lookup",
            LdltColumnStage::ContributorEntry => "fem.ldlt.contributor-entry",
            LdltColumnStage::PivotRead => "fem.ldlt.pivot-read",
            LdltColumnStage::DiagonalCommit => "fem.ldlt.diagonal-commit",
            LdltColumnStage::EmitRow => "fem.ldlt.emit-row",
            LdltColumnStage::PublishColumn => "fem.ldlt.publish-column",
            LdltColumnStage::CompleteColumn => "fem.ldlt.complete-column",
        });
        if context.should_yield() {
            return NumericalRun::Yield;
        }
        context.consume_fuel(1);
        if self.advance_column_microcursor().is_err() {
            return NumericalRun::Fault;
        }
        if context.is_cancelled() {
            return NumericalRun::Cancelled;
        }
        NumericalRun::Yield
    }

    fn largest_owner_bytes(&self) -> usize {
        let state = &self.state;
        [state.a.colptr.capacity() * size_of::<u32>(), state.a.rowind.capacity() * size_of::<u32>(), state.a.vals.capacity() * size_of::<f64>(), state.d.capacity() * size_of::<f64>()]
            .into_iter()
            .chain(state.l_cols.iter().map(|column| column.capacity() * size_of::<(u32, f64)>()))
            .chain(state.row_lists.iter().map(|row| row.capacity() * size_of::<usize>()))
            .max()
            .unwrap_or(0)
    }

    fn rung_demand(&self) -> RetirementDemand {
        let mut demand = numerical_rung_demand(self.close_terminal_is_empty());
        if self.state.admission_fault {
            demand.release_bytes = demand.release_bytes.max(self.largest_owner_bytes());
        }
        demand
    }

    fn close_frontier(&self) -> Result<Option<RetirementDemand>, ValueError> {
        if let Some(demand) = self.desk.retirement_demands() {
            return demand.map(Some);
        }
        if let Some(demand) = numerical_writer_demand(&self.checkpoint_writer) {
            return demand.map(Some);
        }
        if let Some(demand) = numerical_writer_demand(&self.output_writer) {
            return demand.map(Some);
        }
        Ok(None)
    }

    fn close_demand(&self) -> Result<RetirementDemand, ValueError> {
        Ok(self.close_frontier()?.unwrap_or_else(|| self.rung_demand()))
    }

    fn close_turn(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        let frontier = self.close_frontier()?;
        if !numerical_close_gate(grant, frontier) {
            return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
        }
        let step = if let Some(step) = self.desk.close_step(grant) {
            step?
        } else if let Some(step) = numerical_writer_close(&mut self.checkpoint_writer, grant) {
            step?
        } else if let Some(step) = numerical_writer_close(&mut self.output_writer, grant) {
            step?
        } else {
            numerical_ladder_turn(self.close_retained_step(grant.maximum_release_bytes))
        };
        step.admit(grant, self.terminal_is_empty())
    }
}

impl InteractiveJob for LdltJob {
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
        if let Some(writer) = self.checkpoint_writer.as_mut() {
            writer.begin_close();
        }
        if let Some(writer) = self.output_writer.as_mut() {
            writer.begin_close();
        }
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if self.terminal_is_empty() {
            return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
        }
        let demand = self.close_demand()?;
        numerical_close_original(cx, self, demand, Self::close_turn, Self::terminal_is_empty)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand()?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> { Ok(self.close_demand()?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand()?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand()?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.checkpoint_writer.is_none() && self.output_writer.is_none() && self.desk.terminal_is_empty() && self.close_terminal_is_empty()
    }
}

impl LdltFactor {
    /// 🧭️ Forward (`Ly=b`) → diagonal (`z=y/D`) → backward (`Lᵀx=z`) substitution, column-oriented
    /// so no separate row-major structure of `L` is needed.
    pub fn solve(&self, b: &VecD) -> VecD {
        let n = self.n;
        let mut y = b.0.clone();
        for j in 0..n {
            let yj = y[j];
            if yj == 0.0 {
                continue;
            }
            for &(row, lij) in self.l_cols[j].iter() {
                y[row as usize] -= lij * yj;
            }
        }
        for (j, value) in y.iter_mut().enumerate().take(n) {
            *value /= self.d[j];
        }
        for j in (0..n).rev() {
            let mut sum = y[j];
            for &(row, lij) in self.l_cols[j].iter() {
                sum -= lij * y[row as usize];
            }
            y[j] = sum;
        }
        VecD::from_vec(y)
    }

    pub fn solve_many(&self, b: &MatD) -> MatD {
        let mut out = MatD::zeros(b.rows, b.cols);
        for col in 0..b.cols {
            let rhs = VecD::from_vec((0..b.rows).map(|row| b.get(row, col)).collect());
            let x = self.solve(&rhs);
            for row in 0..b.rows {
                out.set(row, col, x.get(row));
            }
        }
        out
    }

    /// 🔢️ Count of `D[j] < 0` — a Sturm-sequence inertia count, used later for eigenvalue-count checks.
    pub fn negative_pivot_count(&self) -> usize {
        self.d.iter().filter(|&&value| value < 0.0).count()
    }
}
// #endregion 🔖️Ldlt

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
