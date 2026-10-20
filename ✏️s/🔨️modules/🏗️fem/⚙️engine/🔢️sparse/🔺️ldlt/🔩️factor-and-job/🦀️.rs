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
    pub columns: SparseColumns,
    pub d: PagedList<f64, SPARSE_INDEX_SPACE>,
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
    Ok(LdltFactor::from_owned_parts(n, l_cols, d))
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
    pub(super) columns: SparseColumns,
    pub(super) d: PagedList<f64, SPARSE_INDEX_SPACE>,
    pub(super) contributors: RowContributors,
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
    SortRows,
}

#[derive(Clone, PartialEq)]
pub(super) struct LdltColumnCursor {
    pub(super) stage: LdltColumnStage,
    pub(super) source: usize,
    pub(super) contributor: u64,
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
    pub(super) values: PagedList<f64, SPARSE_INDEX_SPACE>,
    pub(super) marks: PagedList<u32, SPARSE_INDEX_SPACE>,
    pub(super) generation: u32,
    pub(super) touched: PagedList<u32, SPARSE_INDEX_SPACE>,
    pub(super) candidate: PagedList<(u32, f64), SPARSE_INDEX_SPACE>,
    pub(super) sort: HeapSortCursor,
}

pub(super) const LDLT_MAXIMUM_ORDER: usize = u32::MAX as usize;

pub struct LdltJob {
    pub(super) operation: Operation,
    pub(super) state: LdltCheckpoint,
    pub(super) output_writer: Option<RetainedJobPayloadWriter>,
    pub(super) output_page_cursor: usize,
    pub(super) checkpoint_writer: Option<RetainedJobPayloadWriter>,
    pub(super) checkpoint_cursor: NumericalPageCursor,
    pub(super) desk: NumericalOutcomeDesk,
    pub(super) terminal_published: bool,
}

pub(super) fn close_ldlt_checkpoint_step(state: &mut LdltCheckpoint, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
    if let Some(step) = state.contributors.close_step(maximum_bytes)? { return Ok(Some(step)); }
    if let Some(step) = state.columns.close_step(maximum_bytes)? { return Ok(Some(step)); }
    if let Some(step) = close_paged_owner_step(&mut state.workspace.candidate, maximum_bytes)? { return Ok(Some(step)); }
    for owner in [&mut state.workspace.touched, &mut state.workspace.marks, &mut state.a.rowind, &mut state.a.colptr] { if let Some(step) = close_paged_owner_step(owner, maximum_bytes)? { return Ok(Some(step)); } }
    for owner in [&mut state.d, &mut state.a.vals, &mut state.workspace.values] { if let Some(step) = close_paged_owner_step(owner, maximum_bytes)? { return Ok(Some(step)); } }
    Ok(None)
}

impl LdltJob {
    pub fn new(operation: Operation, a: CscSym, columns_per_step: usize) -> Self {
        assert!(columns_per_step > 0, "ldlt batch must contain work");
        let n = a.n;
        let admission_fault = n > LDLT_MAXIMUM_ORDER || a.colptr.len() != n.saturating_add(1) || a.rowind.len() != a.vals.len() || a.colptr.get(0).copied() != Some(0) || a.colptr.get(n).copied().map(|value| value as usize) != Some(a.vals.len());
        let workspace = LdltColumnWorkspace { values: PagedList::empty(), marks: PagedList::empty(), generation: 0, touched: PagedList::empty(), candidate: PagedList::empty(), sort: HeapSortCursor::new(0) };
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
                columns: SparseColumns::default(),
                d: PagedList::empty(),
                contributors: RowContributors::default(),
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
            terminal_published: false, desk: NumericalOutcomeDesk::default(),
        }
    }

    pub fn preview(&self) -> LdltPreview {
        LdltPreview { completed_columns: self.state.column, total_columns: self.state.a.n, negative_pivots: self.state.d.iter().take(self.state.column).filter(|&&value| value < 0.0).count() }
    }

    #[cfg(test)]
    fn factor(&self) -> Option<LdltFactor> {
        (self.state.column == self.state.a.n).then(|| LdltFactor { n: self.state.a.n, columns: self.state.columns.clone(), d: self.state.d.clone() })
    }

    /// 🎼 Transfers the completed factor into the retained subspace child without cloning.
    pub fn take_factor(&mut self, grant: RetainedCloneGrant) -> Option<(LdltFactor, RetainedCloneProgress)> {
        let copied_bytes = size_of::<LdltFactor>() * 2;
        if self.state.column != self.state.a.n || !numerical_grant_covers(grant, RetirementDemand { copy_bytes: copied_bytes, depth: 1, ..Default::default() }) { return None; }
        Some((LdltFactor { n: self.state.a.n, columns: std::mem::take(&mut self.state.columns), d: std::mem::take(&mut self.state.d) }, RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() }))
    }

    fn reserve_owner<T>(owner: &mut PagedList<T, SPARSE_INDEX_SPACE>, target: usize, context: &mut StepContext<'_>) -> Result<bool, SparseError> {
        if owner.capacity() >= target { return Ok(true); }
        let progress = owner.reserve_capacity_one_funded(target, context.retained_grant()).map_err(|_| SparseError::DimensionMismatch)?;
        context.consume_retained(progress).map_err(|_| SparseError::DimensionMismatch)?;
        Ok(false)
    }

    fn admit_work(context: &mut StepContext<'_>, depth: usize, bytes: usize) -> Result<bool, SparseError> {
        let copied_bytes = size_of::<LdltColumnCursor>() * 2 + bytes;
        if !numerical_grant_covers(context.retained_grant(), RetirementDemand { copy_bytes: copied_bytes, depth: depth.max(1), ..Default::default() }) { return Ok(false); }
        context.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() }).map_err(|_| SparseError::DimensionMismatch)?;
        Ok(true)
    }

    fn owner_depth<T>(owner: &PagedList<T, SPARSE_INDEX_SPACE>, index: usize) -> Result<usize, SparseError> { owner.next_get_depth_demand(index).map_err(|_| SparseError::DimensionMismatch) }

    fn initialize_owner<T: Copy>(owner: &mut PagedList<T, SPARSE_INDEX_SPACE>, target: usize, value: T, context: &mut StepContext<'_>) -> Result<bool, SparseError> {
        if !Self::reserve_owner(owner, target, context)? { return Ok(false); }
        if owner.len() == target { return Ok(true); }
        let depth = owner.next_push_depth_demand().map_err(|_| SparseError::DimensionMismatch)?;
        if !Self::admit_work(context, depth, size_of::<T>())? { return Ok(false); }
        owner.push_reserved(value).map_err(|_| SparseError::DimensionMismatch)?;
        Ok(false)
    }

    fn advance_reservation(&mut self, context: &mut StepContext<'_>) -> Result<(), SparseError> {
        let n = self.state.a.n;
        let complete = match self.state.reserve_lane {
            0 => Self::initialize_owner(&mut self.state.contributors.heads, n, u64::MAX, context)?,
            1 => Self::initialize_owner(&mut self.state.contributors.tails, n, u64::MAX, context)?,
            2 => Self::initialize_owner(&mut self.state.d, n, 0.0, context)?,
            3 => Self::initialize_owner(&mut self.state.workspace.values, n, 0.0, context)?,
            4 => Self::initialize_owner(&mut self.state.workspace.marks, n, 0, context)?,
            5 => Self::initialize_owner(&mut self.state.columns.offsets, 1, 0, context)?,
            _ => true,
        };
        if complete && Self::admit_work(context, 1, 1)? { self.state.reserve_lane = if self.state.reserve_lane < 5 { self.state.reserve_lane + 1 } else { 9 }; }
        Ok(())
    }

    fn reserve_workspace_row(&mut self, row: usize, context: &mut StepContext<'_>) -> Result<bool, SparseError> {
        if row >= self.state.a.n { return Err(SparseError::DimensionMismatch); }
        if self.state.workspace.marks[row] == self.state.workspace.generation { return Ok(true); }
        let target = self.state.workspace.touched.len().checked_add(1).ok_or(SparseError::DimensionMismatch)?;
        Self::reserve_owner(&mut self.state.workspace.touched, target, context)
    }

    pub(super) fn workspace_depth(&self, row: usize) -> Result<usize, SparseError> {
        let mut depth = Self::owner_depth(&self.state.workspace.marks, row)? + Self::owner_depth(&self.state.workspace.values, row)?;
        if self.state.workspace.marks[row] != self.state.workspace.generation { depth += self.state.workspace.touched.next_push_depth_demand().map_err(|_| SparseError::DimensionMismatch)?; }
        Ok(depth)
    }

    fn workspace_add(&mut self, row: usize, value: f64) -> Result<(), SparseError> {
        if !value.is_finite() { return Err(SparseError::DimensionMismatch); }
        let fresh = self.state.workspace.marks[row] != self.state.workspace.generation;
        let result = if fresh { value } else { self.state.workspace.values[row] + value };
        if !result.is_finite() { return Err(SparseError::DimensionMismatch); }
        if fresh {
            self.state.workspace.touched.push_reserved(row as u32).map_err(|_| SparseError::DimensionMismatch)?;
            self.state.workspace.marks[row] = self.state.workspace.generation;
        }
        self.state.workspace.values[row] = result;
        Ok(())
    }

    fn workspace_get(&self, row: usize) -> f64 { if self.state.workspace.marks[row] == self.state.workspace.generation { self.state.workspace.values[row] } else { 0.0 } }

    fn advance_column_microcursor(&mut self, context: &mut StepContext<'_>) -> Result<(), SparseError> {
        if self.state.reserve_lane < 9 { return self.advance_reservation(context); }
        let column = self.state.column;
        match self.state.cursor.stage {
            LdltColumnStage::ReserveColumn => {
                if self.state.workspace.sort.input_len() != 0 || self.state.cursor.emit_row != 0 {
                    if Self::admit_work(context, 1, size_of::<HeapSortCursor>() * 2 + size_of::<usize>())? { self.state.workspace.sort = HeapSortCursor::new(0); self.state.cursor.emit_row = 0; }
                    return Ok(());
                }
                if !self.state.workspace.candidate.is_empty() {
                    let depth = self.state.workspace.candidate.next_pop_depth_demand().map_err(|_| SparseError::DimensionMismatch)?;
                    if Self::admit_work(context, depth, size_of::<(u32, f64)>())? { self.state.workspace.candidate.pop(); }
                    return Ok(());
                }
                if !self.state.workspace.touched.is_empty() {
                    let depth = self.state.workspace.touched.next_pop_depth_demand().map_err(|_| SparseError::DimensionMismatch)?;
                    if Self::admit_work(context, depth, size_of::<u32>())? { self.state.workspace.touched.pop(); }
                    return Ok(());
                }
                let generation = self.state.workspace.generation.checked_add(1).ok_or(SparseError::DimensionMismatch)?;
                let source = *self.state.a.colptr.get(column).ok_or(SparseError::DimensionMismatch)? as usize;
                let depth = Self::owner_depth(&self.state.a.colptr, column)? + Self::owner_depth(&self.state.contributors.heads, column)?;
                if !Self::admit_work(context, depth, 24)? { return Ok(()); }
                self.state.workspace.generation = generation;
                self.state.workspace.sort = HeapSortCursor::new(0);
                self.state.cursor.source = source;
                self.state.cursor.contributor = self.state.contributors.heads[column];
                self.state.cursor.entry = 0;
                self.state.cursor.active_column = 0;
                self.state.cursor.lookup_lower = 0;
                self.state.cursor.lookup_upper = 0;
                self.state.cursor.lookup_mid = 0;
                self.state.cursor.lookup_comparison = 0;
                self.state.cursor.lookup_initialized = false;
                self.state.cursor.emit_row = 0;
                self.state.cursor.stage = LdltColumnStage::SourceEntry;
            }
            LdltColumnStage::SourceEntry => {
                let end = *self.state.a.colptr.get(column + 1).ok_or(SparseError::DimensionMismatch)? as usize;
                if end > self.state.a.vals.len() || self.state.cursor.source > end { return Err(SparseError::DimensionMismatch); }
                if self.state.cursor.source < end {
                    let index = self.state.cursor.source;
                    let row = *self.state.a.rowind.get(index).ok_or(SparseError::DimensionMismatch)? as usize;
                    let value = *self.state.a.vals.get(index).ok_or(SparseError::DimensionMismatch)?;
                    if row < column || !value.is_finite() { return Err(SparseError::DimensionMismatch); }
                    if !self.reserve_workspace_row(row, context)? { return Ok(()); }
                    let depth = Self::owner_depth(&self.state.a.colptr, column + 1)? + Self::owner_depth(&self.state.a.rowind, index)? + Self::owner_depth(&self.state.a.vals, index)? + self.workspace_depth(row)?;
                    if !Self::admit_work(context, depth, 32)? { return Ok(()); }
                    self.workspace_add(row, value)?;
                    self.state.cursor.source += 1;
                } else if Self::admit_work(context, Self::owner_depth(&self.state.a.colptr, column + 1)?, 8)? { self.state.cursor.stage = LdltColumnStage::ContributorLookup; }
            }
            LdltColumnStage::ContributorLookup => {
                if self.state.cursor.contributor == u64::MAX {
                    if Self::admit_work(context, 1, 1)? { self.state.cursor.stage = LdltColumnStage::PivotRead; }
                    return Ok(());
                }
                let link_index = usize::try_from(self.state.cursor.contributor).map_err(|_| SparseError::DimensionMismatch)?;
                let link = *self.state.contributors.links.get(link_index).ok_or(SparseError::DimensionMismatch)?;
                let active = link.column as usize;
                if active >= column { return Err(SparseError::DimensionMismatch); }
                let length = self.state.columns.column_len(active).ok_or(SparseError::DimensionMismatch)?;
                let depth = Self::owner_depth(&self.state.contributors.links, link_index)? + Self::owner_depth(&self.state.columns.offsets, active)? + Self::owner_depth(&self.state.columns.offsets, active + 1)?;
                if !self.state.cursor.lookup_initialized {
                    if !Self::admit_work(context, depth, 24)? { return Ok(()); }
                    self.state.cursor.active_column = active;
                    self.state.cursor.lookup_lower = 0;
                    self.state.cursor.lookup_upper = length;
                    self.state.cursor.lookup_mid = 0;
                    self.state.cursor.lookup_comparison = 0;
                    self.state.cursor.lookup_initialized = true;
                } else if self.state.cursor.lookup_lower < self.state.cursor.lookup_upper {
                    let mid = self.state.cursor.lookup_lower + (self.state.cursor.lookup_upper - self.state.cursor.lookup_lower) / 2;
                    let pair = self.state.columns.entry(active, mid).ok_or(SparseError::DimensionMismatch)?;
                    let entry_depth = self.state.columns.entry_depth_demand(active, mid).map_err(|_| SparseError::DimensionMismatch)?;
                    if !Self::admit_work(context, depth + entry_depth, size_of::<(u32, f64)>())? { return Ok(()); }
                    let comparison = pair.0.cmp(&(column as u32));
                    self.state.cursor.lookup_mid = mid;
                    self.state.cursor.lookup_comparison = match comparison { std::cmp::Ordering::Less => -1, std::cmp::Ordering::Equal => 0, std::cmp::Ordering::Greater => 1 };
                    if comparison == std::cmp::Ordering::Less { self.state.cursor.lookup_lower = mid + 1; } else { self.state.cursor.lookup_upper = mid; }
                } else {
                    let index = self.state.cursor.lookup_lower;
                    let entry = self.state.columns.entry(active, index).filter(|entry| entry.0 == column as u32).ok_or(SparseError::DimensionMismatch)?;
                    let entry_depth = self.state.columns.entry_depth_demand(active, index).map_err(|_| SparseError::DimensionMismatch)?;
                    if !Self::admit_work(context, depth + entry_depth + Self::owner_depth(&self.state.d, active)?, 32)? { return Ok(()); }
                    self.state.cursor.factor = entry.1 * self.state.d[active];
                    self.state.cursor.entry = index;
                    self.state.cursor.lookup_lower = 0;
                    self.state.cursor.lookup_upper = 0;
                    self.state.cursor.lookup_mid = 0;
                    self.state.cursor.lookup_comparison = 0;
                    self.state.cursor.lookup_initialized = false;
                    self.state.cursor.contributor = link.next;
                    self.state.cursor.stage = LdltColumnStage::ContributorEntry;
                }
            }
            LdltColumnStage::ContributorEntry => {
                let active = self.state.cursor.active_column;
                let length = self.state.columns.column_len(active).ok_or(SparseError::DimensionMismatch)?;
                if self.state.cursor.factor == 0.0 || self.state.cursor.entry >= length {
                    if Self::admit_work(context, Self::owner_depth(&self.state.columns.offsets, active + 1)?, 8)? { self.state.cursor.stage = LdltColumnStage::ContributorLookup; }
                } else {
                    let entry_index = self.state.cursor.entry;
                    let (row, value) = self.state.columns.entry(active, entry_index).ok_or(SparseError::DimensionMismatch)?;
                    if (row as usize) < column || !self.state.cursor.factor.is_finite() { return Err(SparseError::DimensionMismatch); }
                    if !self.reserve_workspace_row(row as usize, context)? { return Ok(()); }
                    let depth = self.state.columns.entry_depth_demand(active, entry_index).map_err(|_| SparseError::DimensionMismatch)? + self.workspace_depth(row as usize)?;
                    if !Self::admit_work(context, depth, 40)? { return Ok(()); }
                    self.workspace_add(row as usize, -self.state.cursor.factor * value)?;
                    self.state.cursor.entry += 1;
                }
            }
            LdltColumnStage::PivotRead => {
                let pivot = self.workspace_get(column);
                if !Self::admit_work(context, self.workspace_depth(column)?, 16)? { return Ok(()); }
                if !pivot.is_finite() || pivot.abs() < 1e-12 { return Err(SparseError::ZeroPivot { column }); }
                self.state.cursor.pivot = pivot;
                self.state.cursor.stage = LdltColumnStage::DiagonalCommit;
            }
            LdltColumnStage::DiagonalCommit => {
                if !Self::admit_work(context, Self::owner_depth(&self.state.d, column)?, 8)? { return Ok(()); }
                self.state.d[column] = self.state.cursor.pivot;
                self.state.workspace.sort = HeapSortCursor::new(self.state.workspace.touched.len());
                self.state.cursor.stage = LdltColumnStage::SortRows;
            }
            LdltColumnStage::SortRows => {
                let work = self.state.workspace.sort.next();
                let mut comparison = None;
                let depth = match work {
                    HeapSortWork::Compare { left, right } | HeapSortWork::Swap { left, right } => Self::owner_depth(&self.state.workspace.touched, left)? + Self::owner_depth(&self.state.workspace.touched, right)?,
                    _ => 1,
                };
                if !Self::admit_work(context, depth, size_of::<HeapSortCursor>() * 2 + 16)? { return Ok(()); }
                match work {
                    HeapSortWork::Compare { left, right } => comparison = Some(self.state.workspace.touched[left].cmp(&self.state.workspace.touched[right])),
                    HeapSortWork::Swap { left, right } => { let first = self.state.workspace.touched[left]; let second = self.state.workspace.touched[right]; self.state.workspace.touched[left] = second; self.state.workspace.touched[right] = first; },
                    HeapSortWork::Complete => { self.state.cursor.emit_row = 0; self.state.cursor.stage = LdltColumnStage::EmitRow; return Ok(()); },
                    HeapSortWork::Advance => {},
                }
                self.state.workspace.sort.advance(comparison).map_err(|_| SparseError::DimensionMismatch)?;
            }
            LdltColumnStage::EmitRow => {
                if self.state.cursor.emit_row < self.state.workspace.touched.len() {
                    let touched_index = self.state.cursor.emit_row;
                    let row = self.state.workspace.touched[touched_index] as usize;
                    let value = self.workspace_get(row);
                    let target = self.state.workspace.candidate.len() + 1;
                    if row > column && value != 0.0 && !Self::reserve_owner(&mut self.state.workspace.candidate, target, context)? { return Ok(()); }
                    let mut depth = Self::owner_depth(&self.state.workspace.touched, touched_index)? + self.workspace_depth(row)?;
                    if row > column && value != 0.0 { depth += self.state.workspace.candidate.next_push_depth_demand().map_err(|_| SparseError::DimensionMismatch)?; }
                    if !Self::admit_work(context, depth, 32)? { return Ok(()); }
                    if row > column && value != 0.0 {
                        let entry = value / self.state.cursor.pivot;
                        if !entry.is_finite() { return Err(SparseError::DimensionMismatch); }
                        self.state.workspace.candidate.push_reserved((row as u32, entry)).map_err(|_| SparseError::DimensionMismatch)?;
                    }
                    self.state.cursor.emit_row += 1;
                } else if Self::admit_work(context, 1, 8)? { self.state.cursor.entry = 0; self.state.cursor.stage = LdltColumnStage::PublishColumn; }
            }
            LdltColumnStage::PublishColumn => {
                if self.state.cursor.entry < self.state.workspace.candidate.len() {
                    let pair = self.state.workspace.candidate[self.state.cursor.entry];
                    let target = self.state.columns.entries.len() + 1;
                    if !Self::reserve_owner(&mut self.state.columns.entries, target, context)? { return Ok(()); }
                    let target = self.state.contributors.links.len() + 1;
                    if !Self::reserve_owner(&mut self.state.contributors.links, target, context)? { return Ok(()); }
                    let row = pair.0 as usize;
                    let tail = self.state.contributors.tails[row];
                    let mut depth = Self::owner_depth(&self.state.workspace.candidate, self.state.cursor.entry)? + Self::owner_depth(&self.state.contributors.tails, row)? + self.state.columns.entries.next_push_depth_demand().map_err(|_| SparseError::DimensionMismatch)? + self.state.contributors.links.next_push_depth_demand().map_err(|_| SparseError::DimensionMismatch)?;
                    depth += if tail == u64::MAX { Self::owner_depth(&self.state.contributors.heads, row)? } else { Self::owner_depth(&self.state.contributors.links, usize::try_from(tail).map_err(|_| SparseError::DimensionMismatch)?)? };
                    if !Self::admit_work(context, depth, 64)? { return Ok(()); }
                    let link = self.state.contributors.links.len() as u64;
                    self.state.columns.entries.push_reserved(pair).map_err(|_| SparseError::DimensionMismatch)?;
                    self.state.contributors.links.push_reserved(RowContributor { column: column as u32, next: u64::MAX }).map_err(|_| SparseError::DimensionMismatch)?;
                    if tail == u64::MAX { self.state.contributors.heads[row] = link; } else { self.state.contributors.links[tail as usize].next = link; }
                    self.state.contributors.tails[row] = link;
                    self.state.cursor.entry += 1;
                } else if Self::admit_work(context, 1, 1)? { self.state.cursor.stage = LdltColumnStage::CompleteColumn; }
            }
            LdltColumnStage::CompleteColumn => {
                if self.state.columns.offsets.len() == column + 1 {
                    if !Self::reserve_owner(&mut self.state.columns.offsets, column + 2, context)? { return Ok(()); }
                    let depth = self.state.columns.offsets.next_push_depth_demand().map_err(|_| SparseError::DimensionMismatch)?;
                    if !Self::admit_work(context, depth, size_of::<usize>())? { return Ok(()); }
                    self.state.columns.offsets.push_reserved(self.state.columns.entries.len()).map_err(|_| SparseError::DimensionMismatch)?;
                } else if Self::admit_work(context, 1, 16)? {
                    self.state.column += 1;
                    self.state.cursor.stage = LdltColumnStage::ReserveColumn;
                    self.state.checkpoint_due = self.state.column < self.state.a.n;
                }
            }
        }
        Ok(())
    }

    pub(super) fn advance_checkpoint_entry(state: &LdltCheckpoint, cursor: &mut NumericalPageCursor, writer: &mut RetainedJobPayloadWriter, context: &mut StepContext<'_>) -> Result<bool, ValueError> {
        if advance_numerical_page_header(writer, b"FEMLCP1\0", 11, *cursor, context)? { return Ok(false); }
        let complete = match cursor.field {
            0 => {
                let values = [state.identity.operation, state.identity.revision, state.identity.generation, state.identity.seed, state.a.n as u64, state.column as u64, state.cursor.stage as u64, state.cursor.source as u64, state.cursor.contributor, state.cursor.entry as u64, state.cursor.emit_row as u64, state.cursor.active_column as u64, state.cursor.factor.to_bits(), state.cursor.pivot.to_bits(), state.admission_fault as u64, state.reserve_lane as u64, state.reserve_cursor as u64, state.publication_stage as u64, state.publication_outer as u64, state.publication_inner as u64, state.workspace.generation as u64, state.cursor.lookup_lower as u64, state.cursor.lookup_upper as u64, state.cursor.lookup_mid as u64, state.cursor.lookup_comparison as i64 as u64, state.cursor.lookup_initialized as u64];
                advance_u64_values(writer, &values, cursor, context)?
            }
            1 => advance_paged_u32_owner(writer, &state.a.colptr, cursor, context)?,
            2 => advance_paged_u32_owner(writer, &state.a.rowind, cursor, context)?,
            3 => advance_paged_f64_owner(writer, &state.a.vals, cursor, context)?,
            4 => advance_u64_owner(writer, &state.columns.offsets, cursor, context)?,
            5 => advance_paged_pair_owner(writer, &state.columns.entries, cursor, context)?,
            6 => advance_paged_f64_owner(writer, &state.d, cursor, context)?,
            7 => advance_paged_u64_owner(writer, &state.contributors.heads, cursor, context)?,
            8 => advance_paged_u64_owner(writer, &state.contributors.tails, cursor, context)?,
            9 => advance_contributor_owner(writer, &state.contributors.links, cursor, context)?,
            10 => advance_paged_f64_owner(writer, &state.workspace.values, cursor, context)?,
            11 => advance_paged_u32_owner(writer, &state.workspace.marks, cursor, context)?,
            12 => advance_paged_u32_owner(writer, &state.workspace.touched, cursor, context)?,
            13 => advance_paged_pair_owner(writer, &state.workspace.candidate, cursor, context)?,
            14 => advance_u64_values(writer, &state.workspace.sort.snapshot(), cursor, context)?,
            _ => return Ok(true),
        };
        if complete { if !writer.commit_staged_page_funded(context, size_of::<NumericalPageCursor>() + 3 * size_of::<usize>())? { return Ok(false); } cursor.item = 0; cursor.owner = 0; cursor.field = if cursor.field == 14 { u16::MAX } else { cursor.field + 1 }; }
        Ok(cursor.field == u16::MAX)
    }

    pub(super) fn advance_output_entry(state: &mut LdltCheckpoint, operation: Operation, writer: &mut RetainedJobPayloadWriter, context: &mut StepContext<'_>) -> Result<bool, ValueError> {
        let cursor = NumericalPageCursor { field: state.publication_stage as u16, owner: state.publication_outer, item: state.publication_inner };
        if advance_numerical_page_header(writer, b"FEMLDL2\0", 1, cursor, context)? { return Ok(false); }
        match state.publication_stage {
            0 => {
                let values = [operation.operation.0, operation.base_revision.0, operation.generation.0, operation.seed, state.a.n as u64];
                if let Some(value) = values.get(state.publication_outer) { if write_numerical_entry(writer, &value.to_le_bytes(), context)? { state.publication_outer += 1; } }
                else { if !writer.commit_staged_page_funded(context, size_of::<NumericalPageCursor>() + 3 * size_of::<usize>())? { return Ok(false); } state.publication_outer = 0; state.publication_stage = 1; }
            }
            1 => {
                if let Some(value) = state.d.get(state.publication_outer) { if write_numerical_entry(writer, &value.to_bits().to_le_bytes(), context)? { state.publication_outer += 1; } }
                else { if !writer.commit_staged_page_funded(context, size_of::<NumericalPageCursor>() + 3 * size_of::<usize>())? { return Ok(false); } state.publication_outer = 0; state.publication_stage = 2; }
            }
            2 => {
                if state.publication_outer == state.columns.len() { if !writer.commit_staged_page_funded(context, size_of::<NumericalPageCursor>() + 3 * size_of::<usize>())? { return Ok(false); } state.publication_stage = 3; }
                else if state.publication_inner == 0 {
                    let length = state.columns.column_len(state.publication_outer).expect("published factor column");
                    if write_numerical_entry(writer, &(length as u64).to_le_bytes(), context)? { state.publication_inner = 1; }
                } else if let Some((row, value)) = state.columns.entry(state.publication_outer, state.publication_inner - 1) {
                    let mut bytes = [0; 12]; bytes[..4].copy_from_slice(&row.to_le_bytes()); bytes[4..].copy_from_slice(&value.to_bits().to_le_bytes());
                    if write_numerical_entry(writer, &bytes, context)? { state.publication_inner += 1; }
                } else { state.publication_inner = 0; state.publication_outer += 1; }
            }
            _ => {},
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

    pub(super) fn close_terminal_is_empty(&self) -> bool { self.state.contributors.terminal_is_empty() && self.state.columns.terminal_is_empty() && self.state.d.terminal_is_empty() && self.state.a.terminal_is_empty() && self.state.workspace.values.terminal_is_empty() && self.state.workspace.marks.terminal_is_empty() && self.state.workspace.touched.terminal_is_empty() && self.state.workspace.candidate.terminal_is_empty() }
}

impl LdltFactor {
    /// 🧊️ Cold factor inputs become one compact first-party paged column stream.
    pub fn from_owned_parts(n: usize, columns: Vec<Vec<(u32, f64)>>, diagonal: Vec<f64>) -> Self { Self { n, columns: SparseColumns::from_owned(columns), d: paged_from_values(diagonal) } }
    pub fn order(&self) -> usize { self.n }
    pub fn diagonal(&self, index: usize) -> Option<f64> { self.d.get(index).copied() }
    pub fn column_entries_len(&self, column: usize) -> Option<usize> { self.columns.column_len(column) }
    pub fn column_entry(&self, column: usize, entry: usize) -> Option<(u32, f64)> { self.columns.entry(column, entry) }
    pub fn diagonal_depth_demand(&self, index: usize) -> Result<usize, ValueError> { self.d.next_get_depth_demand(index).map_err(|error| ValueError::literal(ValueRefusalKind::OwnershipLimit, error.reason)) }
    pub fn column_entry_depth_demand(&self, column: usize, entry: usize) -> Result<usize, ValueError> { self.columns.entry_depth_demand(column, entry) }
    pub fn terminal_is_empty(&self) -> bool { self.columns.terminal_is_empty() && self.d.terminal_is_empty() }
    pub fn allocated_bytes(&self) -> usize { self.columns.offsets.allocated_bytes() + self.columns.entries.allocated_bytes() + self.d.allocated_bytes() }
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        let mut demand = if !self.columns.terminal_is_empty() { self.columns.next_retirement_demands()? } else { paged_owner_retirement_demand(&self.d)? };
        if !self.terminal_is_empty() { demand.copy_bytes = size_of::<Self>() * 2 + size_of::<(u32, f64)>(); }
        Ok(demand)
    }
    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        match self.columns.close_step(maximum_bytes) { Ok(Some((items, bytes))) => return (false, items, bytes), Err(()) => return (false, 0, 0), Ok(None) => {}, }
        match close_paged_owner_step(&mut self.d, maximum_bytes) { Ok(Some((items, bytes))) => (false, items, bytes), Err(()) => (false, 0, 0), Ok(None) => (true, 0, 0), }
    }
    /// 🧮️ Solve through the compact column stream and diagonal.
    pub fn solve(&self, b: &VecD) -> VecD {
        let n = self.n;
        let mut y = b.0.clone();
        for j in 0..n {
            let yj = y[j];
            if yj == 0.0 {
                continue;
            }
            for entry in 0..self.column_entries_len(j).expect("factor column") {
                let (row, lij) = self.column_entry(j, entry).expect("factor entry");
                y[row as usize] -= lij * yj;
            }
        }
        for (j, value) in y.iter_mut().enumerate().take(n) {
            *value /= self.d[j];
        }
        for j in (0..n).rev() {
            let mut sum = y[j];
            for entry in 0..self.column_entries_len(j).expect("factor column") {
                let (row, lij) = self.column_entry(j, entry).expect("factor entry");
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
