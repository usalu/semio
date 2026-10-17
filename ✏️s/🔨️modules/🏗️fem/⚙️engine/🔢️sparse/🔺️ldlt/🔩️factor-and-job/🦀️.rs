//! 🔩️ The LDLT factor, its one-shot factorization, the checkpointed job and the factor solve.

use super::*;

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
    pub(super) identity: NumericalCheckpointIdentity,
    pub(super) a: CscSym,
    pub(super) l_cols: Vec<Vec<(u32, f64)>>,
    pub(super) d: Vec<f64>,
    pub(super) row_lists: Vec<Vec<usize>>,
    pub(super) column: usize,
    pub(super) cursor: LdltColumnCursor,
    pub(super) workspace: LdltColumnWorkspace,
    pub(super) admission_fault: bool,
    pub(super) reserve_lane: u8,
    pub(super) reserve_cursor: usize,
    pub(super) checkpoint_due: bool,
    pub(super) publication_stage: u8,
    pub(super) publication_outer: usize,
    pub(super) publication_inner: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum LdltColumnStage {
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
pub(super) struct LdltColumnCursor {
    pub(super) stage: LdltColumnStage,
    pub(super) source: usize,
    pub(super) contributor: usize,
    pub(super) entry: usize,
    pub(super) emit_row: usize,
    pub(super) active_column: usize,
    pub(super) lookup_lower: usize,
    pub(super) lookup_upper: usize,
    pub(super) lookup_mid: usize,
    pub(super) lookup_comparison: i8,
    pub(super) lookup_initialized: bool,
    pub(super) factor: f64,
    pub(super) pivot: f64,
}

#[derive(Clone, PartialEq)]
pub(super) struct LdltColumnWorkspace {
    pub(super) values: Vec<f64>,
    pub(super) marks: Vec<u32>,
    pub(super) generation: u32,
    pub(super) candidate: Vec<(u32, f64)>,
}

pub(super) const LDLT_MAXIMUM_ORDER: usize = 40;

pub struct LdltJob {
    pub(super) operation: Operation,
    pub(super) state: LdltCheckpoint,
    pub(super) output_writer: Option<RetainedJobPayloadWriter>,
    pub(super) output_page_cursor: usize,
    pub(super) checkpoint_writer: Option<RetainedJobPayloadWriter>,
    pub(super) checkpoint_cursor: NumericalPageCursor,
    pub(super) desk: NumericalOutcomeDesk,
}

pub(super) fn close_ldlt_checkpoint_step(state: &mut LdltCheckpoint, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
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

    pub(super) fn advance_column_microcursor(&mut self) -> Result<(), SparseError> {
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

    pub(super) fn advance_checkpoint_entry(state: &LdltCheckpoint, cursor: &mut NumericalPageCursor, writer: &mut RetainedJobPayloadWriter) -> Result<bool, JobPayloadAdmissionFault> {
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

    pub(super) fn advance_output_entry(state: &mut LdltCheckpoint, operation: Operation, writer: &mut RetainedJobPayloadWriter) -> Result<bool, JobPayloadAdmissionFault> {
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

    pub(super) fn close_retained_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        match close_ldlt_checkpoint_step(&mut self.state, maximum_bytes) {
            Ok(Some((items, bytes))) => (false, items, bytes),
            Ok(None) => (true, 0, 0),
            Err(()) => (false, 0, 0),
        }
    }

    pub(super) fn close_terminal_is_empty(&self) -> bool {
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
