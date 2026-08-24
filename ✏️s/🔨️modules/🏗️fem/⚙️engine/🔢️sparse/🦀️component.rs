//! 🧮️ Sparse linear algebra: COO/CSR/CSC assembly, a left-looking sparse LDLT direct solver, a
//! Jacobi-preconditioned conjugate-gradient iterative solver, a subspace-iteration eigensolver
//! (modal/buckling `Kφ=λBφ`) backed by a dense cyclic-Jacobi eigensolver for its small projected
//! subproblem, and reverse-Cuthill-McKee bandwidth-reduction ordering. No dependency beyond
//! `crate::algebra`'s dense `MatD`/`VecD`, used here as both scratch storage for small
//! projected problems and as the correctness oracle in this module's tests.

use crate::algebra::{MatD, VecD};
use semio_framework_job::{CommitCandidate, InteractiveJob, JobFault, JobPayloadAdmissionFault, JobPayloadStream, Operation, RetainedJobPayload, RetainedJobPayloadWriter, StepBudget, StepContext, StepOutcome};
use std::collections::{BTreeMap, VecDeque};

fn close_vec_owner_step<T>(owner: &mut Vec<T>, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
    if owner.pop().is_some() {
        return Ok(Some((1, 0)));
    }
    let bytes = owner.capacity().checked_mul(std::mem::size_of::<T>()).ok_or(())?;
    if bytes == 0 {
        return Ok(None);
    }
    if bytes > maximum_bytes {
        return Err(());
    }
    *owner = Vec::new();
    Ok(Some((1, bytes)))
}

fn close_nested_vec_owner_step<T>(owner: &mut Vec<Vec<T>>, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
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
        Csr { n, indptr, indices, vals }
    }

    /// 🔺️ Keeps only entries where `col >= row` (upper triangle), grouped by the SMALLER index `j`
    /// so that storage-column `j` directly holds `A[j][c]` for every `c >= j` (via symmetry
    /// `A[j][c] = A[c][j]`) — the layout the left-looking LDLT column loop needs without a scan.
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
/// 🧮️ General compressed-sparse-row matrix — used for SpMV (PCG, residual checks).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Csr {
    pub n: usize,
    indptr: Vec<u32>,
    indices: Vec<u32>,
    vals: Vec<f64>,
}

impl Csr {
    /// 🧵️ Adopts arrays prepared by the retained assembly cursor without another scan.
    pub(crate) fn from_owned_parts(n: usize, indptr: Vec<u32>, indices: Vec<u32>, vals: Vec<f64>) -> Self {
        Self { n, indptr, indices, vals }
    }

    pub(crate) fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        match close_vec_owner_step(&mut self.vals, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        for owner in [&mut self.indices, &mut self.indptr] {
            match close_vec_owner_step(owner, maximum_bytes) {
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
            let start = self.indptr[row] as usize;
            let end = self.indptr[row + 1] as usize;
            let mut sum = 0.0;
            for idx in start..end {
                sum += self.vals[idx] * x.get(self.indices[idx] as usize);
            }
            out.set(row, sum);
        }
        out
    }

    pub fn diag(&self) -> VecD {
        let mut out = VecD::zeros(self.n);
        for row in 0..self.n {
            let start = self.indptr[row] as usize;
            let end = self.indptr[row + 1] as usize;
            for idx in start..end {
                if self.indices[idx] as usize == row {
                    out.set(row, self.vals[idx]);
                }
            }
        }
        out
    }
}
// #endregion 🔖️Csr

// #region 🔖️CscSym
/// 🔺️ Symmetric matrix, upper-triangle entries only (`col >= row`), grouped by the smaller index —
/// storage-column `j` holds `A[j][c]` for every `c >= j`, the LDLT input format.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CscSym {
    pub n: usize,
    colptr: Vec<u32>,
    rowind: Vec<u32>,
    vals: Vec<f64>,
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

    pub(crate) fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
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

// #region 🔖️Ldlt
/// ⚠️ Everything that can go wrong factoring a `CscSym`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SparseError {
    ZeroPivot { column: usize },
    DimensionMismatch,
}

/// 🧊️ A sparse left-looking LDLT factorization (unit lower `L`, diagonal `D`), permutation-agnostic
/// — a caller applying `rcm_order` reorders the matrix/RHS/solution indices itself before/after
/// calling into this module.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LdltFactor {
    n: usize,
    l_cols: Vec<Vec<(u32, f64)>>,
    d: Vec<f64>,
}

fn ldlt_column(a: &CscSym, l_cols: &mut [BTreeMap<u32, f64>], d: &mut [f64], row_lists: &mut [Vec<usize>], j: usize) -> Result<(), SparseError> {
    let mut accum: BTreeMap<usize, f64> = BTreeMap::new();
    let start = a.colptr[j] as usize;
    let end = a.colptr[j + 1] as usize;
    for idx in start..end {
        let row = a.rowind[idx] as usize;
        *accum.entry(row).or_insert(0.0) += a.vals[idx];
    }

    for &k in &row_lists[j] {
        let ljk = *l_cols[k].get(&(j as u32)).unwrap_or(&0.0);
        if ljk == 0.0 {
            continue;
        }
        let factor = ljk * d[k];
        for (&row_u32, &lik) in &l_cols[k] {
            let row = row_u32 as usize;
            if row >= j {
                *accum.entry(row).or_insert(0.0) -= factor * lik;
            }
        }
    }

    let djj = *accum.get(&j).unwrap_or(&0.0);
    if djj.abs() < 1e-12 {
        return Err(SparseError::ZeroPivot { column: j });
    }
    d[j] = djj;
    for (&row, &value) in &accum {
        if row > j && value != 0.0 {
            l_cols[j].insert(row as u32, value / djj);
            row_lists[row].push(j);
        }
    }
    Ok(())
}

/// 🧮️ Left-looking sparse LDLT: for each column `j`, seeds an accumulator from `A`'s column `j`
/// (rows `>= j`), then for every earlier column `k` with `L[j][k] != 0` (tracked via each row's
/// list of contributing earlier columns) subtracts `L[j][k] * L[i][k] * D[k]` at every row `i`
/// where `L[i][k] != 0` — this is where fill-in appears. Symbolic and numeric phases are combined
/// in one pass, per Davis's "Direct Methods for Sparse Linear Systems".
pub fn ldlt_factor(a: &CscSym) -> Result<LdltFactor, SparseError> {
    let n = a.n;
    let mut map_cols: Vec<BTreeMap<u32, f64>> = vec![BTreeMap::new(); n];
    let mut d = vec![0.0; n];
    let mut row_lists: Vec<Vec<usize>> = vec![Vec::new(); n];

    for j in 0..n {
        ldlt_column(a, &mut map_cols, &mut d, &mut row_lists, j)?;
    }

    let l_cols = map_cols.into_iter().map(BTreeMap::into_iter).collect();
    Ok(LdltFactor { n, l_cols, d })
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LdltPreview {
    pub completed_columns: usize,
    pub total_columns: usize,
    pub negative_pivots: usize,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct LdltCheckpoint {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct NumericalCheckpointIdentity {
    operation: u64,
    revision: u64,
    generation: u64,
    seed: u64,
}

impl NumericalCheckpointIdentity {
    fn from_operation(operation: Operation) -> Self {
        Self { operation: operation.operation.0, revision: operation.base_revision.0, generation: operation.generation.0, seed: operation.seed }
    }

    fn matches(self, operation: Operation) -> bool {
        self == Self::from_operation(operation)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct LdltColumnWorkspace {
    values: Vec<f64>,
    marks: Vec<u32>,
    generation: u32,
    candidate: Vec<(u32, f64)>,
}

const LDLT_MAXIMUM_ORDER: usize = 40;
const NUMERICAL_OWNER_PAGE_BYTES: usize = 16 * 1024;
pub const MOUNTED_SCALAR_SLOTS: usize = 768;

/// 🧮 Fixed mounted scalar owner with one admission, write, update, or close action per turn.
pub struct MountedScalarSlots {
    values: [f64; MOUNTED_SCALAR_SLOTS],
    admitted: usize,
    len: usize,
}

impl MountedScalarSlots {
    pub fn new() -> Self {
        Self { values: [0.0; MOUNTED_SCALAR_SLOTS], admitted: 0, len: 0 }
    }

    pub fn admit_one(&mut self, target: usize) -> Result<bool, ()> {
        if target > MOUNTED_SCALAR_SLOTS {
            return Err(());
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

    pub fn add_at(&mut self, index: usize, value: f64) -> Result<(), ()> {
        if index >= self.len {
            return Err(());
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
const NUMERICAL_CHECKPOINT_VERSION: u16 = 1;
const NUMERICAL_CHECKPOINT_HEADER_BYTES: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct NumericalPageCursor {
    field: u16,
    owner: usize,
    item: usize,
}

impl NumericalPageCursor {
    fn new() -> Self {
        Self { field: 0, owner: 0, item: 0 }
    }
}

fn numerical_page_header(magic: &[u8; 8], kind: u16, cursor: NumericalPageCursor) -> [u8; NUMERICAL_CHECKPOINT_HEADER_BYTES] {
    let mut bytes = [0; NUMERICAL_CHECKPOINT_HEADER_BYTES];
    bytes[..8].copy_from_slice(magic);
    bytes[8..10].copy_from_slice(&NUMERICAL_CHECKPOINT_VERSION.to_le_bytes());
    bytes[10..12].copy_from_slice(&kind.to_le_bytes());
    bytes[12..14].copy_from_slice(&cursor.field.to_le_bytes());
    bytes[16..24].copy_from_slice(&(cursor.owner as u64).to_le_bytes());
    bytes[24..32].copy_from_slice(&(cursor.item as u64).to_le_bytes());
    bytes
}

fn advance_numerical_page_header(writer: &mut RetainedJobPayloadWriter, magic: &[u8; 8], kind: u16, cursor: NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
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

fn advance_owner_length(writer: &mut RetainedJobPayloadWriter, length: usize, cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if cursor.owner == 0 {
        writer.write_staged(&(length as u64).to_le_bytes())?;
        cursor.owner = 1;
        return Ok(false);
    }
    Ok(true)
}

fn advance_u32_owner(writer: &mut RetainedJobPayloadWriter, values: &[u32], cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        writer.write_staged(&value.to_le_bytes())?;
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

fn advance_u64_owner(writer: &mut RetainedJobPayloadWriter, values: &[usize], cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        writer.write_staged(&(*value as u64).to_le_bytes())?;
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

fn advance_u64_values(writer: &mut RetainedJobPayloadWriter, values: &[u64], cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        writer.write_staged(&value.to_le_bytes())?;
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

fn advance_f64_owner(writer: &mut RetainedJobPayloadWriter, values: &[f64], cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some(value) = values.get(cursor.item) {
        writer.write_staged(&value.to_bits().to_le_bytes())?;
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

fn advance_pair_owner(writer: &mut RetainedJobPayloadWriter, values: &[(u32, f64)], cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if !advance_owner_length(writer, values.len(), cursor)? {
        return Ok(false);
    }
    if let Some((index, value)) = values.get(cursor.item) {
        let mut bytes = [0; 12];
        bytes[..4].copy_from_slice(&index.to_le_bytes());
        bytes[4..].copy_from_slice(&value.to_bits().to_le_bytes());
        writer.write_staged(&bytes)?;
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

fn advance_matrix_owner(writer: &mut RetainedJobPayloadWriter, matrix: &MatD, cursor: &mut NumericalPageCursor) -> Result<bool, JobPayloadAdmissionFault> {
    if cursor.owner == 0 {
        let mut bytes = [0; 24];
        bytes[..8].copy_from_slice(&(matrix.rows as u64).to_le_bytes());
        bytes[8..16].copy_from_slice(&(matrix.cols as u64).to_le_bytes());
        bytes[16..].copy_from_slice(&(matrix.data.len() as u64).to_le_bytes());
        writer.write_staged(&bytes)?;
        cursor.owner = 1;
        return Ok(false);
    }
    if let Some(value) = matrix.data.get(cursor.item) {
        writer.write_staged(&value.to_bits().to_le_bytes())?;
        cursor.item += 1;
        return Ok(false);
    }
    Ok(true)
}

pub struct LdltJob {
    operation: Operation,
    state: LdltCheckpoint,
    output_writer: Option<RetainedJobPayloadWriter>,
    output_page_cursor: usize,
    checkpoint_writer: Option<RetainedJobPayloadWriter>,
    checkpoint_cursor: NumericalPageCursor,
}

impl LdltJob {
    pub fn new(operation: Operation, a: CscSym, columns_per_step: usize) -> Self {
        assert!(columns_per_step > 0, "ldlt batch must contain work");
        let n = a.n;
        let input_pages_valid = a.colptr.capacity().saturating_mul(std::mem::size_of::<u32>()) <= NUMERICAL_OWNER_PAGE_BYTES
            && a.rowind.capacity().saturating_mul(std::mem::size_of::<u32>()) <= NUMERICAL_OWNER_PAGE_BYTES
            && a.vals.capacity().saturating_mul(std::mem::size_of::<f64>()) <= NUMERICAL_OWNER_PAGE_BYTES;
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
        let bytes = owner.capacity().checked_mul(std::mem::size_of::<T>()).ok_or(SparseError::DimensionMismatch)?;
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
        for step in [close_nested_vec_owner_step(&mut self.state.row_lists, maximum_bytes), close_nested_vec_owner_step(&mut self.state.l_cols, maximum_bytes)] {
            match step {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        for owner in [&mut self.state.workspace.candidate] {
            match close_vec_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        for owner in [&mut self.state.d, &mut self.state.a.vals, &mut self.state.workspace.values] {
            match close_vec_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        for owner in [&mut self.state.a.rowind, &mut self.state.a.colptr] {
            match close_vec_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        match close_vec_owner_step(&mut self.state.workspace.marks, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        (true, 0, 0)
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

struct NumericalPageView<'a> {
    kind: u16,
    field: u16,
    owner: usize,
    item: usize,
    bytes: &'a [u8],
}

fn read_checkpoint_u16(bytes: &[u8], offset: usize) -> Result<u16, NumericalCheckpointFault> {
    let value = bytes.get(offset..offset + 2).ok_or(NumericalCheckpointFault::Truncated)?;
    Ok(u16::from_le_bytes([value[0], value[1]]))
}

fn read_checkpoint_u32(bytes: &[u8], offset: usize) -> Result<u32, NumericalCheckpointFault> {
    let value = bytes.get(offset..offset + 4).ok_or(NumericalCheckpointFault::Truncated)?;
    Ok(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

fn read_checkpoint_u64(bytes: &[u8], offset: usize) -> Result<u64, NumericalCheckpointFault> {
    let value = bytes.get(offset..offset + 8).ok_or(NumericalCheckpointFault::Truncated)?;
    Ok(u64::from_le_bytes([value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7]]))
}

fn parse_numerical_page<'a>(bytes: &'a [u8], magic: &[u8; 8]) -> Result<NumericalPageView<'a>, NumericalCheckpointFault> {
    if bytes.len() < NUMERICAL_CHECKPOINT_HEADER_BYTES || bytes.get(..8) != Some(magic.as_slice()) {
        return Err(NumericalCheckpointFault::Truncated);
    }
    if read_checkpoint_u16(bytes, 8)? != NUMERICAL_CHECKPOINT_VERSION {
        return Err(NumericalCheckpointFault::Version);
    }
    Ok(NumericalPageView {
        kind: read_checkpoint_u16(bytes, 10)?,
        field: read_checkpoint_u16(bytes, 12)?,
        owner: read_checkpoint_u64(bytes, 16)? as usize,
        item: read_checkpoint_u64(bytes, 24)? as usize,
        bytes: &bytes[NUMERICAL_CHECKPOINT_HEADER_BYTES..],
    })
}

fn declared_owner_length(page: &NumericalPageView<'_>, maximum: usize) -> Result<usize, NumericalCheckpointFault> {
    let length = read_checkpoint_u64(page.bytes, 0)? as usize;
    (length <= maximum).then_some(length).ok_or(NumericalCheckpointFault::Envelope)
}

fn validate_restored_owner<T>(owner: &Vec<T>) -> Result<(), NumericalCheckpointFault> {
    let bytes = owner.capacity().checked_mul(std::mem::size_of::<T>()).ok_or(NumericalCheckpointFault::Envelope)?;
    (bytes <= NUMERICAL_OWNER_PAGE_BYTES).then_some(()).ok_or(NumericalCheckpointFault::Envelope)
}

fn restore_u32_entry(owner: &mut Vec<u32>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize) -> Result<bool, NumericalCheckpointFault> {
    let length = declared_owner_length(page, maximum)?;
    if page.item != 0 || page.bytes.len() != 8usize.saturating_add(length.saturating_mul(4)) {
        return Err(NumericalCheckpointFault::Truncated);
    }
    if *entry == 0 {
        if !owner.is_empty() {
            return Err(NumericalCheckpointFault::Field);
        }
        owner.try_reserve_exact(length).map_err(|_| NumericalCheckpointFault::Admission)?;
        validate_restored_owner(owner)?;
        *entry = 1;
        return Ok(false);
    }
    let item = *entry - 1;
    if owner.len() != item {
        return Err(NumericalCheckpointFault::Field);
    }
    if item < length {
        owner.push(read_checkpoint_u32(page.bytes, 8 + item * 4)?);
        *entry += 1;
        return Ok(false);
    }
    Ok(true)
}

fn restore_usize_entry(owner: &mut Vec<usize>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize) -> Result<bool, NumericalCheckpointFault> {
    let length = declared_owner_length(page, maximum)?;
    if page.item != 0 || page.bytes.len() != 8usize.saturating_add(length.saturating_mul(8)) {
        return Err(NumericalCheckpointFault::Truncated);
    }
    if *entry == 0 {
        if !owner.is_empty() {
            return Err(NumericalCheckpointFault::Field);
        }
        owner.try_reserve_exact(length).map_err(|_| NumericalCheckpointFault::Admission)?;
        validate_restored_owner(owner)?;
        *entry = 1;
        return Ok(false);
    }
    let item = *entry - 1;
    if owner.len() != item {
        return Err(NumericalCheckpointFault::Field);
    }
    if item < length {
        owner.push(read_checkpoint_u64(page.bytes, 8 + item * 8)? as usize);
        *entry += 1;
        return Ok(false);
    }
    Ok(true)
}

fn restore_f64_entry(owner: &mut Vec<f64>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize) -> Result<bool, NumericalCheckpointFault> {
    let length = declared_owner_length(page, maximum)?;
    if page.item != 0 || page.bytes.len() != 8usize.saturating_add(length.saturating_mul(8)) {
        return Err(NumericalCheckpointFault::Truncated);
    }
    if *entry == 0 {
        if !owner.is_empty() {
            return Err(NumericalCheckpointFault::Field);
        }
        owner.try_reserve_exact(length).map_err(|_| NumericalCheckpointFault::Admission)?;
        validate_restored_owner(owner)?;
        *entry = 1;
        return Ok(false);
    }
    let item = *entry - 1;
    if owner.len() != item {
        return Err(NumericalCheckpointFault::Field);
    }
    if item < length {
        owner.push(f64::from_bits(read_checkpoint_u64(page.bytes, 8 + item * 8)?));
        *entry += 1;
        return Ok(false);
    }
    Ok(true)
}

fn restore_pair_entry(owner: &mut Vec<(u32, f64)>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize) -> Result<bool, NumericalCheckpointFault> {
    let length = declared_owner_length(page, maximum)?;
    if page.item != 0 || page.bytes.len() != 8usize.saturating_add(length.saturating_mul(12)) {
        return Err(NumericalCheckpointFault::Truncated);
    }
    if *entry == 0 {
        if !owner.is_empty() {
            return Err(NumericalCheckpointFault::Field);
        }
        owner.try_reserve_exact(length).map_err(|_| NumericalCheckpointFault::Admission)?;
        validate_restored_owner(owner)?;
        *entry = 1;
        return Ok(false);
    }
    let item = *entry - 1;
    if owner.len() != item {
        return Err(NumericalCheckpointFault::Field);
    }
    if item < length {
        let offset = 8 + item * 12;
        owner.push((read_checkpoint_u32(page.bytes, offset)?, f64::from_bits(read_checkpoint_u64(page.bytes, offset + 4)?)));
        *entry += 1;
        return Ok(false);
    }
    Ok(true)
}

pub struct LdltRestoreCursor {
    operation: Operation,
    payload: Option<RetainedJobPayload>,
    total_pages: usize,
    page_slot: usize,
    close_due: bool,
    expected_field: u16,
    page_entry: usize,
    control: [u64; 32],
    state: Option<LdltCheckpoint>,
    fault: Option<NumericalCheckpointFault>,
}

impl LdltRestoreCursor {
    pub fn new(operation: Operation, payload: RetainedJobPayload) -> Self {
        let total_pages = payload.page_count();
        Self { operation, payload: Some(payload), total_pages, page_slot: 0, close_due: false, expected_field: 0, page_entry: 0, control: [0; 32], state: None, fault: None }
    }

    fn decode_page_entry(&mut self, bytes: &[u8]) -> Result<bool, NumericalCheckpointFault> {
        let page = parse_numerical_page(bytes, b"FEMLCP1\0")?;
        if page.kind != 11 || page.field != self.expected_field || page.owner != 0 || page.item != 0 {
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
            1 => restore_u32_entry(&mut state.a.colptr, &page, n + 1, &mut self.page_entry)?,
            2 => restore_u32_entry(&mut state.a.rowind, &page, n.saturating_mul(n), &mut self.page_entry)?,
            3 => restore_f64_entry(&mut state.a.vals, &page, n.saturating_mul(n), &mut self.page_entry)?,
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
            field if field >= 5 && field < 5 + state.l_cols.len() as u16 => restore_pair_entry(&mut state.l_cols[(field - 5) as usize], &page, n, &mut self.page_entry)?,
            517 => restore_f64_entry(&mut state.d, &page, n, &mut self.page_entry)?,
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
            field if field >= row_base && field < row_base + state.row_lists.len() as u16 => restore_usize_entry(&mut state.row_lists[(field - row_base) as usize], &page, n, &mut self.page_entry)?,
            1032 => restore_f64_entry(&mut state.workspace.values, &page, n, &mut self.page_entry)?,
            1033 => restore_u32_entry(&mut state.workspace.marks, &page, n, &mut self.page_entry)?,
            1034 => restore_pair_entry(&mut state.workspace.candidate, &page, n, &mut self.page_entry)?,
            _ => return Err(NumericalCheckpointFault::Field),
        };
        if complete {
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
            let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            self.page_slot += 1;
            self.close_due = false;
            self.page_entry = 0;
            if self.page_slot == self.total_pages {
                if self.expected_field != u16::MAX || !payload.terminal_is_empty() {
                    return Err(NumericalCheckpointFault::Truncated);
                }
                self.payload = None;
                let state = self.state.take().ok_or(NumericalCheckpointFault::Truncated)?;
                return Ok(Some(LdltJob { operation: self.operation, state, output_writer: None, output_page_cursor: 0, checkpoint_writer: None, checkpoint_cursor: NumericalPageCursor::new() }));
            }
            return Ok(None);
        }
        let payload = self.payload.take().ok_or(NumericalCheckpointFault::Truncated)?;
        let decoded = payload.page(self.page_slot).ok_or(NumericalCheckpointFault::Truncated).and_then(|source| self.decode_page_entry(source));
        self.payload = Some(payload);
        match decoded {
            Ok(true) => self.close_due = true,
            Ok(false) => {}
            Err(fault) => {
                self.fault = Some(fault);
                return Err(fault);
            }
        }
        Ok(None)
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(payload) = self.payload.as_mut() {
            if !payload.terminal_is_empty() {
                return match payload.close_step(1, maximum_bytes) {
                    semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    semio_framework_job::JobPayloadCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 },
                };
            }
            self.payload = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let Some(state) = self.state.as_mut() else { return semio_framework_job::InteractiveJobCloseStep::Complete };
        for step in [close_nested_vec_owner_step(&mut state.row_lists, maximum_bytes), close_nested_vec_owner_step(&mut state.l_cols, maximum_bytes)] {
            if let Ok(Some((released_items, released_bytes))) = step {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
            }
        }
        if let Ok(Some((released_items, released_bytes))) = close_vec_owner_step(&mut state.workspace.candidate, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        for owner in [&mut state.d, &mut state.a.vals, &mut state.workspace.values] {
            if let Ok(Some((released_items, released_bytes))) = close_vec_owner_step(owner, maximum_bytes) {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
            }
        }
        for owner in [&mut state.a.rowind, &mut state.a.colptr, &mut state.workspace.marks] {
            if let Ok(Some((released_items, released_bytes))) = close_vec_owner_step(owner, maximum_bytes) {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
            }
        }
        self.state = None;
        semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.payload.is_none() && self.state.is_none()
    }
}

impl InteractiveJob for LdltJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
        }
        if self.state.admission_fault {
            return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
        }
        if self.state.checkpoint_due || self.checkpoint_writer.is_some() {
            context.set_stage("fem.ldlt.checkpoint-page");
            if context.should_yield() {
                return StepOutcome::Yield;
            }
            context.consume_fuel(1);
            if self.checkpoint_writer.is_none() {
                self.state.checkpoint_due = false;
                self.checkpoint_cursor = NumericalPageCursor::new();
                self.checkpoint_writer = Some(RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState));
                return StepOutcome::Yield;
            }
            let Some(writer) = self.checkpoint_writer.as_mut() else {
                return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
            };
            if writer.staged_page_len().is_none() {
                return match writer.begin_staged_page(context) {
                    Ok(()) => StepOutcome::Yield,
                    Err(_) => StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
                };
            }
            let complete = match Self::advance_checkpoint_entry(&self.state, &mut self.checkpoint_cursor, writer) {
                Ok(complete) => complete,
                Err(_) => return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
            };
            if !complete {
                return StepOutcome::Yield;
            }
            let Some(writer) = self.checkpoint_writer.take() else {
                return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
            };
            let state = match writer.finish() {
                Ok(state) => state,
                Err(writer) => {
                    self.checkpoint_writer = Some(writer);
                    return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
                }
            };
            return StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state, applied_progress: self.state.column as u64 });
        }
        if self.state.column == self.state.a.n {
            context.set_stage("fem.ldlt.output-entry");
            if context.should_yield() {
                return StepOutcome::Yield;
            }
            if self.output_writer.is_none() {
                context.consume_fuel(1);
                self.output_writer = Some(RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput));
                self.output_page_cursor = 0;
                return StepOutcome::Yield;
            }
            context.consume_fuel(1);
            let Some(writer) = self.output_writer.as_mut() else {
                return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
            };
            if writer.staged_page_len().is_none() {
                return match writer.begin_staged_page(context) {
                    Ok(()) => StepOutcome::Yield,
                    Err(_) => StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
                };
            }
            let complete = match Self::advance_output_entry(&mut self.state, self.operation, writer) {
                Ok(complete) => complete,
                Err(_) => return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
            };
            if !complete {
                return StepOutcome::Yield;
            }
            let Some(writer) = self.output_writer.take() else {
                return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
            };
            let output = match writer.finish() {
                Ok(output) => output,
                Err(writer) => {
                    self.output_writer = Some(writer);
                    return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
                }
            };
            return StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output });
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
            return StepOutcome::Yield;
        }
        context.consume_fuel(1);
        if self.advance_column_microcursor().is_err() {
            return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
        }
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        StepOutcome::Yield
    }

    fn begin_close(&mut self) {
        if let Some(writer) = self.checkpoint_writer.as_mut() {
            writer.begin_close();
        }
        if let Some(writer) = self.output_writer.as_mut() {
            writer.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(writer) = self.checkpoint_writer.as_mut() {
            return match writer.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    self.checkpoint_writer = None;
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
            };
        }
        if let Some(writer) = self.output_writer.as_mut() {
            return match writer.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    self.output_writer = None;
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
            };
        }
        let (complete, released_items, released_bytes) = self.close_retained_step(maximum_bytes);
        if complete {
            semio_framework_job::InteractiveJobCloseStep::Complete
        } else {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.checkpoint_writer.is_none() && self.output_writer.is_none() && self.close_terminal_is_empty()
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

// #region 🔖️Pcg
/// 📈️ Convergence outcome of a `pcg` call.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PcgStats {
    pub iterations: usize,
    pub residual_norm: f64,
    pub converged: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PcgQuality {
    Initializing,
    Coarse,
    Final,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PcgPreview {
    pub stage: PcgStage,
    pub quality: PcgQuality,
    pub iteration: usize,
    pub residual_norm: f64,
    pub displacement: Vec<f64>,
    pub residual: Vec<f64>,
    pub reactions: Vec<f64>,
    pub approximate_contours: Vec<f64>,
    pub converged: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
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

pub struct PcgJob {
    operation: Operation,
    state: PcgCheckpoint,
    close_lane: u8,
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
        }
    }

    pub fn from_checkpoint(operation: Operation, bytes: &[u8]) -> Result<Self, serde_json::Error> {
        Ok(Self { operation, state: serde_json::from_slice(bytes)?, close_lane: 0 })
    }

    pub fn checkpoint_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&self.state).expect("pcg checkpoint is serializable")
    }

    pub fn preview(&self) -> PcgPreview {
        let reactions = self.state.r.0.iter().map(|value| -*value).collect();
        PcgPreview {
            stage: self.state.stage,
            quality: if self.state.converged {
                PcgQuality::Final
            } else if self.state.coarse_published {
                PcgQuality::Coarse
            } else {
                PcgQuality::Initializing
            },
            iteration: self.state.iteration,
            residual_norm: self.state.residual_norm,
            displacement: self.state.x.0.clone(),
            residual: self.state.r.0.clone(),
            reactions,
            approximate_contours: self.state.x.0.iter().map(|value| value.abs()).collect(),
            converged: self.state.converged,
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
    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        loop {
            let step = match self.close_lane {
                0 => close_vec_owner_step(&mut self.state.a.vals, maximum_bytes),
                1 => close_vec_owner_step(&mut self.state.a.indices, maximum_bytes),
                2 => close_vec_owner_step(&mut self.state.a.indptr, maximum_bytes),
                3 => close_vec_owner_step(&mut self.state.b.0, maximum_bytes),
                4 => close_vec_owner_step(&mut self.state.x.0, maximum_bytes),
                5 => close_vec_owner_step(&mut self.state.diag.0, maximum_bytes),
                6 => close_vec_owner_step(&mut self.state.r.0, maximum_bytes),
                7 => close_vec_owner_step(&mut self.state.z.0, maximum_bytes),
                8 => close_vec_owner_step(&mut self.state.p.0, maximum_bytes),
                9 => close_vec_owner_step(&mut self.state.ap.0, maximum_bytes),
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
        self.state.entry_cursor = self.state.a.indptr.first().copied().unwrap_or(0) as usize;
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
            let end = self.state.a.indptr[row + 1] as usize;
            if self.state.entry_cursor < end {
                let entry = self.state.entry_cursor;
                if self.state.a.indices[entry] as usize == row {
                    self.state.diag.set(row, self.state.a.vals[entry]);
                }
                self.state.entry_cursor += 1;
            } else {
                self.state.row_cursor += 1;
                self.state.entry_cursor = self.state.a.indptr[self.state.row_cursor.min(self.state.a.n)] as usize;
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
            let end = self.state.a.indptr[row + 1] as usize;
            if self.state.entry_cursor < end {
                let entry = self.state.entry_cursor;
                let col = self.state.a.indices[entry] as usize;
                let value = if initial { self.state.x.get(col) } else { self.state.p.get(col) };
                self.state.row_sum += self.state.a.vals[entry] * value;
                self.state.entry_cursor += 1;
            } else {
                self.state.ap.set(row, self.state.row_sum);
                if !initial {
                    self.state.dot_accum += self.state.p.get(row) * self.state.row_sum;
                }
                self.state.row_sum = 0.0;
                self.state.row_cursor += 1;
                self.state.entry_cursor = self.state.a.indptr[self.state.row_cursor.min(self.state.a.n)] as usize;
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
            self.state.rz_new += self.state.r.get(i) * value;
            self.state.cursor += 1;
            *units += 1;
            context.consume_fuel(1);
        }
        if self.state.cursor == self.state.a.n {
            self.state.cursor = 0;
            if initial {
                self.state.p = self.state.z.clone();
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
enum ModalInputStage {
    ValidateMass,
    CountUpper,
    ReserveColptr,
    ReserveRowind,
    ReserveValues,
    CopyUpper,
    ReserveMassIndptr,
    ReserveMassIndices,
    ReserveMassValues,
    CopyMountedMass,
    RetireMountedMass,
    BuildMass,
    Complete,
}

/// 🎼 Cursorized converged-stiffness and physical lumped-mass transfer into generalized modes.
pub struct ModalInputConstruction {
    matrix: Option<Csr>,
    mass: Option<VecD>,
    mounted_mass: Option<MountedScalarSlots>,
    stage: ModalInputStage,
    row: usize,
    entry: usize,
    upper_count: usize,
    colptr: Vec<u32>,
    rowind: Vec<u32>,
    values: Vec<f64>,
    mass_indptr: Vec<u32>,
    mass_indices: Vec<u32>,
    mass_values: Vec<f64>,
    complete: Option<(CscSym, Csr)>,
}

impl ModalInputConstruction {
    pub fn new(matrix: Csr, mass: VecD) -> Self {
        Self {
            matrix: Some(matrix),
            mass: Some(mass),
            mounted_mass: None,
            stage: ModalInputStage::ValidateMass,
            row: 0,
            entry: 0,
            upper_count: 0,
            colptr: Vec::new(),
            rowind: Vec::new(),
            values: Vec::new(),
            mass_indptr: Vec::new(),
            mass_indices: Vec::new(),
            mass_values: Vec::new(),
            complete: None,
        }
    }

    pub fn new_mounted(matrix: Csr, mass: MountedScalarSlots) -> Self {
        let mut construction = Self::new(matrix, VecD::from_vec(Vec::new()));
        construction.mass = None;
        construction.mounted_mass = Some(mass);
        construction
    }

    fn reserve<T>(owner: &mut Vec<T>, count: usize) -> Result<(), &'static [u8]> {
        owner.try_reserve_exact(count).map_err(|_| b"modal-input-owner-allocation" as &'static [u8])?;
        if owner.capacity().checked_mul(std::mem::size_of::<T>()).is_none_or(|bytes| bytes > NUMERICAL_OWNER_PAGE_BYTES) {
            return Err(b"modal-input-owner-page");
        }
        Ok(())
    }

    pub fn step_one(&mut self) -> Result<bool, &'static [u8]> {
        let matrix = self.matrix.as_ref().ok_or(b"modal-input-matrix-owner" as &'static [u8])?;
        let n = matrix.n;
        match self.stage {
            ModalInputStage::ValidateMass => {
                let mass_len = self.mass.as_ref().map(VecD::len).or_else(|| self.mounted_mass.as_ref().map(MountedScalarSlots::len)).ok_or(b"modal-input-mass-owner" as &'static [u8])?;
                if mass_len != n {
                    return Err(b"modal-input-mass-order");
                }
                if self.row < n {
                    let value = self.mass.as_ref().map(|mass| mass.get(self.row)).or_else(|| self.mounted_mass.as_ref().and_then(|mass| mass.get(self.row))).ok_or(b"modal-input-mass-owner")?;
                    if !value.is_finite() || value <= 0.0 {
                        return Err(b"modal-input-mass-value");
                    }
                    self.row += 1;
                } else {
                    self.row = 0;
                    self.entry = 0;
                    self.stage = ModalInputStage::CountUpper;
                }
            }
            ModalInputStage::CountUpper => {
                if self.row == n {
                    self.stage = ModalInputStage::ReserveColptr;
                } else {
                    let end = matrix.indptr[self.row + 1] as usize;
                    if self.entry < end {
                        if matrix.indices[self.entry] as usize >= self.row {
                            self.upper_count = self.upper_count.checked_add(1).ok_or(b"modal-input-upper-overflow")?;
                        }
                        self.entry += 1;
                    } else {
                        self.row += 1;
                        self.entry = matrix.indptr.get(self.row).copied().unwrap_or(0) as usize;
                    }
                }
            }
            ModalInputStage::ReserveColptr => {
                Self::reserve(&mut self.colptr, n + 1)?;
                self.colptr.push(0);
                self.stage = ModalInputStage::ReserveRowind;
            }
            ModalInputStage::ReserveRowind => {
                Self::reserve(&mut self.rowind, self.upper_count)?;
                self.stage = ModalInputStage::ReserveValues;
            }
            ModalInputStage::ReserveValues => {
                Self::reserve(&mut self.values, self.upper_count)?;
                self.row = 0;
                self.entry = 0;
                self.stage = ModalInputStage::CopyUpper;
            }
            ModalInputStage::CopyUpper => {
                if self.row == n {
                    self.stage = ModalInputStage::ReserveMassIndptr;
                } else {
                    let end = matrix.indptr[self.row + 1] as usize;
                    if self.entry < end {
                        let column = matrix.indices[self.entry] as usize;
                        if column >= self.row {
                            self.rowind.push(column as u32);
                            self.values.push(matrix.vals[self.entry]);
                        }
                        self.entry += 1;
                    } else {
                        self.colptr.push(self.rowind.len() as u32);
                        self.row += 1;
                        self.entry = matrix.indptr.get(self.row).copied().unwrap_or(0) as usize;
                    }
                }
            }
            ModalInputStage::ReserveMassIndptr => {
                Self::reserve(&mut self.mass_indptr, n + 1)?;
                self.mass_indptr.push(0);
                self.stage = ModalInputStage::ReserveMassIndices;
            }
            ModalInputStage::ReserveMassIndices => {
                Self::reserve(&mut self.mass_indices, n)?;
                self.stage = ModalInputStage::ReserveMassValues;
            }
            ModalInputStage::ReserveMassValues => {
                if let Some(mut mass) = self.mass.take() {
                    self.mass_values = std::mem::take(&mut mass.0);
                    self.row = 0;
                    self.stage = ModalInputStage::BuildMass;
                } else {
                    Self::reserve(&mut self.mass_values, n)?;
                    self.row = 0;
                    self.stage = ModalInputStage::CopyMountedMass;
                }
            }
            ModalInputStage::CopyMountedMass => {
                if self.row < n {
                    let value = self.mounted_mass.as_ref().and_then(|mass| mass.get(self.row)).ok_or(b"modal-input-mounted-mass-owner")?;
                    self.mass_values.push(value);
                    self.row += 1;
                } else {
                    self.stage = ModalInputStage::RetireMountedMass;
                }
            }
            ModalInputStage::RetireMountedMass => {
                if self.mounted_mass.as_mut().is_some_and(|mass| mass.close_step()) {
                    self.mounted_mass = None;
                    self.row = 0;
                    self.stage = ModalInputStage::BuildMass;
                }
            }
            ModalInputStage::BuildMass => {
                if self.row < n {
                    self.mass_indices.push(self.row as u32);
                    self.mass_indptr.push((self.row + 1) as u32);
                    self.row += 1;
                } else {
                    let stiffness = CscSym { n, colptr: std::mem::take(&mut self.colptr), rowind: std::mem::take(&mut self.rowind), vals: std::mem::take(&mut self.values) };
                    let mass = Csr::from_owned_parts(n, std::mem::take(&mut self.mass_indptr), std::mem::take(&mut self.mass_indices), std::mem::take(&mut self.mass_values));
                    self.complete = Some((stiffness, mass));
                    self.stage = ModalInputStage::Complete;
                }
            }
            ModalInputStage::Complete => return Ok(true),
        }
        Ok(self.stage == ModalInputStage::Complete)
    }

    pub fn take_complete(&mut self) -> Option<(CscSym, Csr)> {
        (self.stage == ModalInputStage::Complete).then(|| self.complete.take()).flatten()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some((stiffness, mass)) = self.complete.as_mut() {
            if !stiffness.vals.is_empty() || !stiffness.rowind.is_empty() || !stiffness.colptr.is_empty() {
                return stiffness.close_step(maximum_bytes);
            }
            let (terminal, items, bytes) = mass.close_step(maximum_bytes);
            if !terminal {
                return (false, items, bytes);
            }
            self.complete = None;
            return (false, 1, 0);
        }
        if let Some(matrix) = self.matrix.as_mut() {
            let (terminal, items, bytes) = matrix.close_step(maximum_bytes);
            if !terminal {
                return (false, items, bytes);
            }
            self.matrix = None;
            return (false, 1, 0);
        }
        if let Some(mass) = self.mass.as_mut() {
            match close_vec_owner_step(&mut mass.0, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
            self.mass = None;
            return (false, 1, 0);
        }
        if let Some(mass) = self.mounted_mass.as_mut() {
            if !mass.close_step() {
                return (false, 1, 0);
            }
            self.mounted_mass = None;
            return (false, 1, 0);
        }
        for owner in [&mut self.values, &mut self.mass_values] {
            match close_vec_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        for owner in [&mut self.colptr, &mut self.rowind, &mut self.mass_indptr, &mut self.mass_indices] {
            match close_vec_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        (true, 0, 0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PcgConstructionStage {
    ReserveB,
    InitializeB,
    RetireMountedB,
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
    x: VecD,
    diag: VecD,
    r: VecD,
    z: VecD,
    p: VecD,
    ap: VecD,
    complete: Option<PcgJob>,
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
            x: VecD::from_vec(Vec::new()),
            diag: VecD::from_vec(Vec::new()),
            r: VecD::from_vec(Vec::new()),
            z: VecD::from_vec(Vec::new()),
            p: VecD::from_vec(Vec::new()),
            ap: VecD::from_vec(Vec::new()),
            complete: None,
        }
    }

    /// 🌬️ Retains a generation-local assembled RHS instead of fabricating the compatibility unit vector.
    pub fn new_with_rhs(operation: Operation, matrix: Csr, rhs: VecD) -> Result<Self, (Csr, VecD)> {
        if matrix.n != rhs.len() || rhs.0.capacity().saturating_mul(std::mem::size_of::<f64>()) > NUMERICAL_OWNER_PAGE_BYTES {
            return Err((matrix, rhs));
        }
        Ok(Self {
            operation,
            matrix: Some(matrix),
            stage: PcgConstructionStage::ReserveX,
            cursor: 0,
            b: rhs,
            mounted_b: None,
            x: VecD::from_vec(Vec::new()),
            diag: VecD::from_vec(Vec::new()),
            r: VecD::from_vec(Vec::new()),
            z: VecD::from_vec(Vec::new()),
            p: VecD::from_vec(Vec::new()),
            ap: VecD::from_vec(Vec::new()),
            complete: None,
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
        let n = self.matrix.as_ref().ok_or(b"pcg-construction-matrix-missing" as &'static [u8])?.n;
        macro_rules! reserve {
            ($owner:expr, $next:expr, $fault:expr) => {{
                $owner.0.try_reserve_exact(n).map_err(|_| $fault as &'static [u8])?;
                if $owner.0.capacity().checked_mul(std::mem::size_of::<f64>()).is_none_or(|bytes| bytes > 4_096) {
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
                    self.stage = if self.mounted_b.is_some() { PcgConstructionStage::RetireMountedB } else { PcgConstructionStage::ReserveX };
                }
            }
            PcgConstructionStage::RetireMountedB => {
                if self.mounted_b.as_mut().is_some_and(MountedScalarSlots::close_step) {
                    self.mounted_b = None;
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
                            b_norm: (n as f64).sqrt().max(1e-300),
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

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some(complete) = self.complete.as_mut() {
            let (terminal, items, bytes) = complete.close_step(maximum_bytes);
            if !terminal {
                return (false, items, bytes);
            }
            self.complete = None;
            return (false, 1, 0);
        }
        if let Some(matrix) = self.matrix.as_mut() {
            let (terminal, items, bytes) = matrix.close_step(maximum_bytes);
            if !terminal {
                return (false, items, bytes);
            }
            self.matrix = None;
            return (false, 1, 0);
        }
        if let Some(rhs) = self.mounted_b.as_mut() {
            if !rhs.close_step() {
                return (false, 1, 0);
            }
            self.mounted_b = None;
            return (false, 1, 0);
        }
        for vector in [&mut self.b, &mut self.x, &mut self.diag, &mut self.r, &mut self.z, &mut self.p, &mut self.ap] {
            match close_vec_owner_step(&mut vector.0, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        (true, 0, 0)
    }
}

impl InteractiveJob for PcgJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return StepOutcome::Fault(JobFault { detail: b"stale-fem-pcg-operation".to_vec() });
        }
        context.set_stage("fem.pcg");
        let mut units = 0;
        while units < self.state.batch_units && !context.should_yield() && self.state.stage != PcgStage::Complete {
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
                return StepOutcome::Cancelled;
            }
        }
        if self.state.stage == PcgStage::Complete {
            return StepOutcome::Complete(CommitCandidate { state: self.checkpoint_bytes(), output: serde_json::to_vec(&self.preview()).expect("pcg output is serializable") });
        }
        if self.state.preview_due {
            self.state.preview_due = false;
            return StepOutcome::PreviewReady(serde_json::to_vec(&self.preview()).expect("pcg preview is serializable"));
        }
        if self.state.checkpoint_due {
            self.state.checkpoint_due = false;
            return StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: self.checkpoint_bytes(), applied_progress: self.state.iteration as u64 });
        }
        StepOutcome::Yield
    }

    fn begin_close(&mut self) {}

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let (complete, released_items, released_bytes) = PcgJob::close_step(self, maximum_bytes);
        if complete {
            semio_framework_job::InteractiveJobCloseStep::Complete
        } else {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.close_lane > 9
    }
}

/// ➰️ Jacobi-preconditioned conjugate gradient — mutates `x0` in place, converges when
/// `‖r‖ / ‖b‖ < tol_rel` or `max_iter` is reached.
pub fn pcg(a: &Csr, b: &VecD, x0: &mut VecD, tol_rel: f64, max_iter: usize) -> PcgStats {
    let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), 0);
    let mut job = PcgJob::new(operation, a.clone(), b.clone(), x0.clone(), tol_rel, max_iter, 1_024);
    let mut preview_sequence = 0;
    loop {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut preview_sequence);
        if matches!(job.step(&mut context), StepOutcome::Complete(_)) {
            let (solution, stats) = job.solution();
            *x0 = solution.clone();
            return stats;
        }
    }
}
// #endregion 🔖️Pcg

// #region 🔖️DenseEigen
/// 🎯️ Cyclic Jacobi eigenvalue algorithm for a small dense symmetric matrix — returns eigenvalues
/// (ascending) and the matching eigenvectors as columns of the returned `MatD`. Used internally to
/// solve the small (`p×p`, `p ≤ ~40`) projected eigenproblem inside `subspace_iteration`.
fn dense_symmetric_eigen_jacobi(a: &MatD) -> (Vec<f64>, MatD) {
    let n = a.rows;
    let mut m = a.clone();
    let mut v = MatD::identity(n);
    if n == 0 {
        return (Vec::new(), v);
    }

    for _sweep in 0..100 {
        let mut off_sq = 0.0;
        for p in 0..n {
            for q in (p + 1)..n {
                off_sq += m.get(p, q) * m.get(p, q);
            }
        }
        if off_sq.sqrt() < 1e-12 * (frobenius_norm(&m) + 1.0) {
            break;
        }

        for p in 0..n {
            for q in (p + 1)..n {
                let apq = m.get(p, q);
                if apq.abs() < 1e-300 {
                    continue;
                }
                let app = m.get(p, p);
                let aqq = m.get(q, q);
                let theta = (aqq - app) / (2.0 * apq);
                let t = if theta >= 0.0 { 1.0 / (theta + (theta * theta + 1.0).sqrt()) } else { -1.0 / (-theta + (theta * theta + 1.0).sqrt()) };
                let c = 1.0 / (t * t + 1.0).sqrt();
                let s = t * c;

                m.set(p, p, app - t * apq);
                m.set(q, q, aqq + t * apq);
                m.set(p, q, 0.0);
                m.set(q, p, 0.0);

                for i in 0..n {
                    if i == p || i == q {
                        continue;
                    }
                    let aip = m.get(i, p);
                    let aiq = m.get(i, q);
                    let new_aip = c * aip - s * aiq;
                    let new_aiq = s * aip + c * aiq;
                    m.set(i, p, new_aip);
                    m.set(p, i, new_aip);
                    m.set(i, q, new_aiq);
                    m.set(q, i, new_aiq);
                }

                for i in 0..n {
                    let vip = v.get(i, p);
                    let viq = v.get(i, q);
                    v.set(i, p, c * vip - s * viq);
                    v.set(i, q, s * vip + c * viq);
                }
            }
        }
    }

    let raw_vals: Vec<f64> = (0..n).map(|i| m.get(i, i)).collect();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a_idx, &b_idx| raw_vals[a_idx].total_cmp(&raw_vals[b_idx]));
    let mut vals = vec![0.0; n];
    let mut vecs = MatD::zeros(n, n);
    for (new_idx, &old_idx) in order.iter().enumerate() {
        vals[new_idx] = raw_vals[old_idx];
        for row in 0..n {
            vecs.set(row, new_idx, v.get(row, old_idx));
        }
    }
    (vals, vecs)
}

fn frobenius_norm(m: &MatD) -> f64 {
    let mut sum = 0.0;
    for row in 0..m.rows {
        for col in 0..m.cols {
            sum += m.get(row, col) * m.get(row, col);
        }
    }
    sum.sqrt()
}

fn symmetrize(a: &MatD) -> MatD {
    let n = a.rows;
    let mut out = MatD::zeros(n, n);
    for i in 0..n {
        for j in 0..n {
            out.set(i, j, 0.5 * (a.get(i, j) + a.get(j, i)));
        }
    }
    out
}
// #endregion 🔖️DenseEigen

// #region 🔖️SubspaceIteration
/// 📐️ The lowest `p` eigenpairs of a generalized eigenproblem, ascending by value.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EigenPairs {
    pub values: Vec<f64>,
    pub vectors: Vec<VecD>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubspacePreview {
    pub iteration: usize,
    pub eigenvalues: Vec<f64>,
    pub mode_shapes: Vec<Vec<f64>>,
    pub residuals: Vec<f64>,
    pub converged_count: usize,
    pub converged: bool,
}

fn mat_col(m: &MatD, col: usize) -> VecD {
    VecD::from_vec((0..m.rows).map(|row| m.get(row, col)).collect())
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct SubspaceCheckpoint {
    identity: NumericalCheckpointIdentity,
    k_factor: LdltFactor,
    b: Csr,
    n: usize,
    p: usize,
    max_iter: usize,
    m: usize,
    x: MatD,
    prev_theta: Vec<f64>,
    final_theta: Vec<f64>,
    iteration: usize,
    residuals: Vec<f64>,
    converged_count: usize,
    converged: bool,
    checkpoint_due: bool,
    preview_due: bool,
    admission_fault: bool,
    factor_validation_cursor: usize,
    factor_validation_complete: bool,
    initialization_cursor: usize,
    publication_stage: u8,
    publication_first: usize,
    publication_second: usize,
    work: SubspaceWork,
    retiring_work: Option<SubspaceWork>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum SubspaceStage {
    ReserveIteration,
    ApplyOperatorColumnRow,
    FactorForwardEntry,
    FactorDiagonalEntry,
    FactorBackwardEntry,
    OrthogonalizePairElement,
    NormalizeColumnElement,
    ProjectedMatrixCellEntry,
    JacobiFindPairCell,
    JacobiRotateCell,
    JacobiConvergenceCell,
    ModeSortCompare,
    ModePermuteElement,
    ResidualColumnRow,
    ConvergenceMode,
    PublishIteration,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct SubspaceWork {
    stage: SubspaceStage,
    reserve: usize,
    first: usize,
    second: usize,
    third: usize,
    phase: usize,
    sweep: usize,
    scalar: f64,
    coefficient: f64,
    cosine: f64,
    sine: f64,
    tangent: f64,
    rhs: MatD,
    solved: MatD,
    b_basis: MatD,
    projected: MatD,
    jacobi: MatD,
    jacobi_vectors: MatD,
    ordered_vectors: MatD,
    candidate_x: MatD,
    mu: Vec<f64>,
    theta: Vec<f64>,
    order: Vec<usize>,
    close_lane: u8,
}

impl SubspaceWork {
    fn empty() -> Self {
        Self {
            stage: SubspaceStage::ReserveIteration,
            reserve: 0,
            first: 0,
            second: 0,
            third: 0,
            phase: 0,
            sweep: 0,
            scalar: 0.0,
            coefficient: 0.0,
            cosine: 0.0,
            sine: 0.0,
            tangent: 0.0,
            rhs: MatD::zeros(0, 0),
            solved: MatD::zeros(0, 0),
            b_basis: MatD::zeros(0, 0),
            projected: MatD::zeros(0, 0),
            jacobi: MatD::zeros(0, 0),
            jacobi_vectors: MatD::zeros(0, 0),
            ordered_vectors: MatD::zeros(0, 0),
            candidate_x: MatD::zeros(0, 0),
            mu: Vec::new(),
            theta: Vec::new(),
            order: Vec::new(),
            close_lane: 0,
        }
    }

    fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        loop {
            let step = match self.close_lane {
                0 => close_vec_owner_step(&mut self.rhs.data, maximum_bytes),
                1 => close_vec_owner_step(&mut self.solved.data, maximum_bytes),
                2 => close_vec_owner_step(&mut self.b_basis.data, maximum_bytes),
                3 => close_vec_owner_step(&mut self.projected.data, maximum_bytes),
                4 => close_vec_owner_step(&mut self.jacobi.data, maximum_bytes),
                5 => close_vec_owner_step(&mut self.jacobi_vectors.data, maximum_bytes),
                6 => close_vec_owner_step(&mut self.ordered_vectors.data, maximum_bytes),
                7 => close_vec_owner_step(&mut self.candidate_x.data, maximum_bytes),
                8 => close_vec_owner_step(&mut self.mu, maximum_bytes),
                9 => close_vec_owner_step(&mut self.theta, maximum_bytes),
                10 => close_vec_owner_step(&mut self.order, maximum_bytes),
                _ => return (true, 0, 0),
            };
            match step {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Ok(None) => self.close_lane += 1,
                Err(()) => return (false, 0, 0),
            }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.close_lane > 10
    }
}

const SUBSPACE_MAXIMUM_ORDER: usize = 40;
const SUBSPACE_MAXIMUM_COLUMNS: usize = 40;

pub struct SubspaceIterationJob {
    operation: Operation,
    state: SubspaceCheckpoint,
    preview_writer: Option<RetainedJobPayloadWriter>,
    preview_page_cursor: usize,
    terminal_writer: Option<RetainedJobPayloadWriter>,
    terminal_page_cursor: usize,
    checkpoint_writer: Option<RetainedJobPayloadWriter>,
    checkpoint_cursor: NumericalPageCursor,
}

impl SubspaceIterationJob {
    pub fn new(operation: Operation, k_factor: LdltFactor, b: Csr, n: usize, p: usize, max_iter: usize) -> Self {
        let factor_pages_valid = k_factor.l_cols.capacity().saturating_mul(std::mem::size_of::<Vec<(u32, f64)>>()) <= NUMERICAL_OWNER_PAGE_BYTES;
        let sparse_pages_valid = b.indptr.capacity().saturating_mul(std::mem::size_of::<u32>()) <= NUMERICAL_OWNER_PAGE_BYTES
            && b.indices.capacity().saturating_mul(std::mem::size_of::<u32>()) <= NUMERICAL_OWNER_PAGE_BYTES
            && b.vals.capacity().saturating_mul(std::mem::size_of::<f64>()) <= NUMERICAL_OWNER_PAGE_BYTES;
        let admission_fault = n == 0 || p == 0 || p > n || b.n != n || k_factor.n != n || n > SUBSPACE_MAXIMUM_ORDER || !factor_pages_valid || !sparse_pages_valid;
        let m = if admission_fault { 0 } else { (p + 8).max(2 * p).min(n).max(1) };
        Self {
            operation,
            state: SubspaceCheckpoint {
                identity: NumericalCheckpointIdentity::from_operation(operation),
                k_factor,
                b,
                n,
                p,
                max_iter,
                m,
                x: MatD::zeros(0, 0),
                prev_theta: Vec::new(),
                final_theta: Vec::new(),
                iteration: 0,
                residuals: Vec::new(),
                converged_count: 0,
                converged: false,
                checkpoint_due: false,
                preview_due: false,
                admission_fault,
                factor_validation_cursor: 0,
                factor_validation_complete: admission_fault,
                initialization_cursor: 0,
                publication_stage: 0,
                publication_first: 0,
                publication_second: 0,
                work: SubspaceWork::empty(),
                retiring_work: None,
            },
            preview_writer: None,
            preview_page_cursor: 0,
            terminal_writer: None,
            terminal_page_cursor: 0,
            checkpoint_writer: None,
            checkpoint_cursor: NumericalPageCursor::new(),
        }
    }

    pub fn preview(&self) -> SubspacePreview {
        SubspacePreview {
            iteration: self.state.iteration,
            eigenvalues: self.state.final_theta.iter().take(self.state.p).copied().collect(),
            mode_shapes: (0..self.state.p.min(self.state.x.cols)).map(|column| mat_col(&self.state.x, column).0).collect(),
            residuals: self.state.residuals.clone(),
            converged_count: self.state.converged_count,
            converged: self.state.converged,
        }
    }

    pub fn solution(&self) -> EigenPairs {
        EigenPairs { values: self.state.final_theta.iter().take(self.state.p).copied().collect(), vectors: (0..self.state.p.min(self.state.x.cols)).map(|column| mat_col(&self.state.x, column)).collect() }
    }

    /// 🎼 Borrows one converged genuine subspace component and its eigenvalue without cloning a mode.
    pub fn visual_mode_scalar(&self, mode: usize, index: usize) -> Option<(f64, f64)> {
        let eigenvalue = *self.state.final_theta.get(mode)?;
        (mode < self.state.x.cols && index < self.state.x.rows).then(|| (self.state.x.get(index, mode), eigenvalue))
    }

    /// 🎼 Exposes the retained subspace convergence authority without cloning eigen owners.
    pub fn visual_progress(&self) -> (usize, usize, f64, bool) {
        let residual = self.state.residuals.first().copied().unwrap_or(f64::INFINITY);
        (self.state.iteration, self.state.max_iter, residual, self.state.converged)
    }

    fn reset_cursor(&mut self, stage: SubspaceStage) {
        self.state.work.stage = stage;
        self.state.work.first = 0;
        self.state.work.second = 0;
        self.state.work.third = 0;
        self.state.work.phase = 0;
        self.state.work.scalar = 0.0;
    }

    fn advance_work_checkpoint_entry(work: &SubspaceWork, cursor: &mut NumericalPageCursor, writer: &mut RetainedJobPayloadWriter, base: u16) -> Result<bool, JobPayloadAdmissionFault> {
        Ok(match cursor.field - base {
            0 => {
                let values = [
                    work.stage as u64,
                    work.reserve as u64,
                    work.first as u64,
                    work.second as u64,
                    work.third as u64,
                    work.phase as u64,
                    work.sweep as u64,
                    work.scalar.to_bits(),
                    work.coefficient.to_bits(),
                    work.cosine.to_bits(),
                    work.sine.to_bits(),
                    work.tangent.to_bits(),
                    work.close_lane as u64,
                ];
                advance_u64_values(writer, &values, cursor)?
            }
            1 => advance_matrix_owner(writer, &work.rhs, cursor)?,
            2 => advance_matrix_owner(writer, &work.solved, cursor)?,
            3 => advance_matrix_owner(writer, &work.b_basis, cursor)?,
            4 => advance_matrix_owner(writer, &work.projected, cursor)?,
            5 => advance_matrix_owner(writer, &work.jacobi, cursor)?,
            6 => advance_matrix_owner(writer, &work.jacobi_vectors, cursor)?,
            7 => advance_matrix_owner(writer, &work.ordered_vectors, cursor)?,
            8 => advance_matrix_owner(writer, &work.candidate_x, cursor)?,
            9 => advance_f64_owner(writer, &work.mu, cursor)?,
            10 => advance_f64_owner(writer, &work.theta, cursor)?,
            11 => advance_u64_owner(writer, &work.order, cursor)?,
            _ => true,
        })
    }

    fn advance_checkpoint_entry(state: &SubspaceCheckpoint, cursor: &mut NumericalPageCursor, writer: &mut RetainedJobPayloadWriter) -> Result<bool, JobPayloadAdmissionFault> {
        if advance_numerical_page_header(writer, b"FEMSCP1\0", 12, *cursor)? {
            return Ok(false);
        }
        let complete = match cursor.field {
            0 => {
                let values = [
                    state.identity.operation,
                    state.identity.revision,
                    state.identity.generation,
                    state.identity.seed,
                    state.n as u64,
                    state.p as u64,
                    state.max_iter as u64,
                    state.m as u64,
                    state.iteration as u64,
                    state.converged_count as u64,
                    state.converged as u64,
                    state.checkpoint_due as u64,
                    state.preview_due as u64,
                    state.admission_fault as u64,
                    state.factor_validation_cursor as u64,
                    state.factor_validation_complete as u64,
                    state.initialization_cursor as u64,
                    state.publication_stage as u64,
                    state.publication_first as u64,
                    state.publication_second as u64,
                ];
                advance_u64_values(writer, &values, cursor)?
            }
            1 => advance_owner_length(writer, state.k_factor.l_cols.len(), cursor)?,
            field if field >= 2 && field < 2 + state.n as u16 => advance_pair_owner(writer, &state.k_factor.l_cols[(field - 2) as usize], cursor)?,
            514 => advance_f64_owner(writer, &state.k_factor.d, cursor)?,
            515 => advance_u32_owner(writer, &state.b.indptr, cursor)?,
            516 => advance_u32_owner(writer, &state.b.indices, cursor)?,
            517 => advance_f64_owner(writer, &state.b.vals, cursor)?,
            518 => advance_matrix_owner(writer, &state.x, cursor)?,
            519 => advance_f64_owner(writer, &state.prev_theta, cursor)?,
            520 => advance_f64_owner(writer, &state.final_theta, cursor)?,
            521 => advance_f64_owner(writer, &state.residuals, cursor)?,
            field if (522..=533).contains(&field) => Self::advance_work_checkpoint_entry(&state.work, cursor, writer, 522)?,
            534 => {
                if cursor.owner == 0 {
                    writer.write_staged(&(state.retiring_work.is_some() as u64).to_le_bytes())?;
                    cursor.owner = 1;
                    false
                } else if let Some(work) = state.retiring_work.as_ref() {
                    let values = [
                        work.stage as u64,
                        work.reserve as u64,
                        work.first as u64,
                        work.second as u64,
                        work.third as u64,
                        work.phase as u64,
                        work.sweep as u64,
                        work.scalar.to_bits(),
                        work.coefficient.to_bits(),
                        work.cosine.to_bits(),
                        work.sine.to_bits(),
                        work.tangent.to_bits(),
                        work.close_lane as u64,
                    ];
                    if cursor.owner == 1 {
                        writer.write_staged(&(values.len() as u64).to_le_bytes())?;
                        cursor.owner = 2;
                        false
                    } else if let Some(value) = values.get(cursor.item) {
                        writer.write_staged(&value.to_le_bytes())?;
                        cursor.item += 1;
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
            field if (535..=545).contains(&field) => {
                let Some(work) = state.retiring_work.as_ref() else { return Ok(true) };
                Self::advance_work_checkpoint_entry(work, cursor, writer, 534)?
            }
            _ => return Ok(true),
        };
        if complete {
            writer.commit_staged_page()?;
            cursor.item = 0;
            cursor.owner = 0;
            cursor.field = match cursor.field {
                0 => 1,
                1 if state.n == 0 => 514,
                1 => 2,
                field if field >= 2 && field + 1 < 2 + state.n as u16 => field + 1,
                field if field >= 2 && field < 2 + state.n as u16 => 514,
                field if (514..=533).contains(&field) => field + 1,
                534 if state.retiring_work.is_some() => 535,
                534 => u16::MAX,
                field if (535..545).contains(&field) => field + 1,
                545 => u16::MAX,
                _ => u16::MAX,
            };
        }
        Ok(cursor.field == u16::MAX)
    }

    fn reserve_matrix_owner(matrix: &mut MatD, rows: usize, columns: usize) -> Result<(), ()> {
        let cells = rows.checked_mul(columns).ok_or(())?;
        if matrix.data.capacity() == 0 {
            matrix.data.try_reserve_exact(cells).map_err(|_| ())?;
        }
        if matrix.data.capacity().checked_mul(std::mem::size_of::<f64>()).ok_or(())? > NUMERICAL_OWNER_PAGE_BYTES {
            return Err(());
        }
        matrix.rows = rows;
        matrix.cols = columns;
        Ok(())
    }

    fn initialize_matrix_owner(matrix: &mut MatD) -> bool {
        let cells = matrix.rows.saturating_mul(matrix.cols);
        if matrix.data.len() < cells {
            matrix.data.push(0.0);
            false
        } else {
            true
        }
    }

    fn reserve_scalar_owner<T>(owner: &mut Vec<T>, items: usize) -> Result<(), ()> {
        owner.try_reserve_exact(items).map_err(|_| ())?;
        if owner.capacity().checked_mul(std::mem::size_of::<T>()).ok_or(())? > NUMERICAL_OWNER_PAGE_BYTES {
            return Err(());
        }
        Ok(())
    }

    fn reserve_iteration_owner(&mut self) -> Result<(), ()> {
        if let Some(retiring) = self.state.retiring_work.as_mut() {
            if retiring.close_step(usize::MAX).0 {
                self.state.retiring_work = None;
            }
            return Ok(());
        }
        let n = self.state.n;
        let m = self.state.m;
        let work = &mut self.state.work;
        match work.reserve {
            0 => Self::reserve_matrix_owner(&mut self.state.x, n, m)?,
            1 => {
                if !Self::initialize_matrix_owner(&mut self.state.x) {
                    return Ok(());
                }
            }
            2 => {
                if self.state.initialization_cursor < m {
                    let column = self.state.initialization_cursor;
                    self.state.x.set(column, column, 1.0);
                    if column + 1 < n {
                        self.state.x.add_at(column + 1, column, 0.3);
                    }
                    if column > 0 {
                        self.state.x.add_at(column - 1, column, 0.3);
                    }
                    self.state.initialization_cursor += 1;
                    return Ok(());
                }
            }
            3 => {
                if self.state.prev_theta.capacity() == 0 {
                    Self::reserve_scalar_owner(&mut self.state.prev_theta, self.state.p)?;
                }
            }
            4 => {
                if self.state.prev_theta.len() < self.state.p {
                    self.state.prev_theta.push(f64::MAX);
                    return Ok(());
                }
            }
            5 => {
                if self.state.residuals.capacity() == 0 {
                    Self::reserve_scalar_owner(&mut self.state.residuals, self.state.p)?;
                }
            }
            6 => {
                if self.state.residuals.len() < self.state.p {
                    self.state.residuals.push(f64::MAX);
                    return Ok(());
                }
            }
            7 => {
                if self.state.final_theta.capacity() == 0 {
                    Self::reserve_scalar_owner(&mut self.state.final_theta, m)?;
                }
            }
            8 => {}
            9 => Self::reserve_matrix_owner(&mut work.rhs, n, m)?,
            10 => {
                if !Self::initialize_matrix_owner(&mut work.rhs) {
                    return Ok(());
                }
            }
            11 => Self::reserve_matrix_owner(&mut work.solved, n, m)?,
            12 => {
                if !Self::initialize_matrix_owner(&mut work.solved) {
                    return Ok(());
                }
            }
            13 => Self::reserve_matrix_owner(&mut work.b_basis, n, m)?,
            14 => {
                if !Self::initialize_matrix_owner(&mut work.b_basis) {
                    return Ok(());
                }
            }
            15 => Self::reserve_matrix_owner(&mut work.projected, m, m)?,
            16 => {
                if !Self::initialize_matrix_owner(&mut work.projected) {
                    return Ok(());
                }
            }
            17 => Self::reserve_matrix_owner(&mut work.jacobi, m, m)?,
            18 => {
                if !Self::initialize_matrix_owner(&mut work.jacobi) {
                    return Ok(());
                }
            }
            19 => Self::reserve_matrix_owner(&mut work.jacobi_vectors, m, m)?,
            20 => {
                if !Self::initialize_matrix_owner(&mut work.jacobi_vectors) {
                    return Ok(());
                }
            }
            21 => {
                if work.first < m {
                    work.jacobi_vectors.set(work.first, work.first, 1.0);
                    work.first += 1;
                    return Ok(());
                }
                work.first = 0;
            }
            22 => Self::reserve_matrix_owner(&mut work.ordered_vectors, m, m)?,
            23 => {
                if !Self::initialize_matrix_owner(&mut work.ordered_vectors) {
                    return Ok(());
                }
            }
            24 => Self::reserve_matrix_owner(&mut work.candidate_x, n, m)?,
            25 => {
                if !Self::initialize_matrix_owner(&mut work.candidate_x) {
                    return Ok(());
                }
            }
            26 => Self::reserve_scalar_owner(&mut work.mu, m)?,
            27 => {
                if work.mu.len() < m {
                    work.mu.push(0.0);
                    return Ok(());
                }
            }
            28 => Self::reserve_scalar_owner(&mut work.theta, m)?,
            29 => {
                if work.theta.len() < m {
                    work.theta.push(f64::MAX);
                    return Ok(());
                }
            }
            30 => Self::reserve_scalar_owner(&mut work.order, m)?,
            31 => {
                if work.order.len() < m {
                    work.order.push(work.order.len());
                    return Ok(());
                }
            }
            32 => {
                self.reset_cursor(SubspaceStage::ApplyOperatorColumnRow);
                return Ok(());
            }
            _ => return Err(()),
        }
        work.reserve += 1;
        Ok(())
    }

    fn advance_apply_operator(&mut self) {
        let n = self.state.n;
        let m = self.state.m;
        let work = &mut self.state.work;
        let using_basis = work.phase == 1;
        if work.first >= m {
            if using_basis {
                self.reset_cursor(SubspaceStage::ProjectedMatrixCellEntry);
            } else {
                self.reset_cursor(SubspaceStage::FactorForwardEntry);
            }
            return;
        }
        let row = work.second;
        let start = self.state.b.indptr[row] as usize;
        let end = self.state.b.indptr[row + 1] as usize;
        if work.third < end.saturating_sub(start) {
            let index = start + work.third;
            let source_row = self.state.b.indices[index] as usize;
            let source = if using_basis { work.solved.get(source_row, work.first) } else { self.state.x.get(source_row, work.first) };
            work.scalar += self.state.b.vals[index] * source;
            work.third += 1;
            return;
        }
        if using_basis {
            work.b_basis.set(row, work.first, work.scalar);
        } else {
            work.rhs.set(row, work.first, work.scalar);
        }
        work.scalar = 0.0;
        work.third = 0;
        work.second += 1;
        if work.second == n {
            work.second = 0;
            work.first += 1;
        }
    }

    fn advance_factor_forward(&mut self) {
        let n = self.state.n;
        let m = self.state.m;
        let work = &mut self.state.work;
        if work.first >= m {
            self.reset_cursor(SubspaceStage::OrthogonalizePairElement);
            return;
        }
        if work.phase == 0 {
            work.solved.set(work.second, work.first, work.rhs.get(work.second, work.first));
            work.second += 1;
            if work.second == n {
                work.second = 0;
                work.phase = 1;
            }
            return;
        }
        if work.second == n {
            work.second = 0;
            work.phase = 0;
            work.first += 1;
            return;
        }
        let entries = &self.state.k_factor.l_cols[work.second];
        if work.third < entries.len() {
            let (row, value) = entries[work.third];
            let delta = value * work.solved.get(work.second, work.first);
            work.solved.add_at(row as usize, work.first, -delta);
            work.third += 1;
        } else {
            work.third = 0;
            work.stage = SubspaceStage::FactorDiagonalEntry;
        }
    }

    fn advance_factor_diagonal(&mut self) {
        let work = &mut self.state.work;
        let value = work.solved.get(work.second, work.first) / self.state.k_factor.d[work.second];
        work.solved.set(work.second, work.first, value);
        work.second += 1;
        if work.second == self.state.n {
            work.second = self.state.n;
            work.third = 0;
            work.stage = SubspaceStage::FactorBackwardEntry;
        }
    }

    fn advance_factor_backward(&mut self) {
        let work = &mut self.state.work;
        if work.second == 0 {
            work.second = 0;
            work.third = 0;
            work.phase = 0;
            work.first += 1;
            work.stage = SubspaceStage::FactorForwardEntry;
            return;
        }
        let column = work.second - 1;
        let entries = &self.state.k_factor.l_cols[column];
        if work.third < entries.len() {
            let (row, value) = entries[work.third];
            let delta = value * work.solved.get(row as usize, work.first);
            work.solved.add_at(column, work.first, -delta);
            work.third += 1;
        } else {
            work.second -= 1;
            work.third = 0;
        }
    }

    fn advance_orthogonalize(&mut self) -> Result<(), ()> {
        let n = self.state.n;
        let m = self.state.m;
        let work = &mut self.state.work;
        if work.first >= m {
            work.phase = 1;
            self.reset_cursor(SubspaceStage::ApplyOperatorColumnRow);
            self.state.work.phase = 1;
            return Ok(());
        }
        if work.second < work.first {
            if work.phase == 0 {
                work.coefficient += work.solved.get(work.third, work.first) * work.rhs.get(work.third, work.second);
                work.third += 1;
                if work.third == n {
                    work.third = 0;
                    work.phase = 1;
                }
            } else {
                let row = work.third;
                let coefficient = work.coefficient;
                work.solved.add_at(row, work.first, -coefficient * work.solved.get(row, work.second));
                work.rhs.add_at(row, work.first, -coefficient * work.rhs.get(row, work.second));
                work.third += 1;
                if work.third == n {
                    work.third = 0;
                    work.phase = 0;
                    work.coefficient = 0.0;
                    work.second += 1;
                }
            }
            return Ok(());
        }
        work.stage = SubspaceStage::NormalizeColumnElement;
        work.phase = 0;
        work.third = 0;
        work.scalar = 0.0;
        Ok(())
    }

    fn advance_normalize(&mut self) -> Result<(), ()> {
        let n = self.state.n;
        let work = &mut self.state.work;
        if work.phase == 0 {
            work.scalar += work.solved.get(work.third, work.first) * work.rhs.get(work.third, work.first);
            work.third += 1;
            if work.third == n {
                if !work.scalar.is_finite() || work.scalar <= 1e-24 {
                    work.second = 0;
                    work.third = 0;
                    work.scalar = 0.0;
                    work.phase = 2;
                    return Ok(());
                }
                work.coefficient = work.scalar.max(1e-300).sqrt().recip();
                work.third = 0;
                work.phase = 1;
            }
            return Ok(());
        }
        if work.phase == 2 {
            let value = self.state.x.get(work.third, work.first);
            work.solved.set(work.third, work.first, value);
            work.b_basis.set(work.third, work.first, value);
            work.third += 1;
            if work.third == n {
                work.second = 0;
                work.third = 0;
                work.phase = 3;
            }
            return Ok(());
        }
        if work.phase == 3 {
            if work.second == n {
                work.second = 0;
                work.phase = 4;
                return Ok(());
            }
            let entries = &self.state.k_factor.l_cols[work.second];
            if work.third < entries.len() {
                let (row, value) = entries[work.third];
                let delta = value * self.state.x.get(row as usize, work.first);
                work.b_basis.add_at(work.second, work.first, delta);
                work.third += 1;
            } else {
                work.second += 1;
                work.third = 0;
            }
            return Ok(());
        }
        if work.phase == 4 {
            let value = work.b_basis.get(work.second, work.first) * self.state.k_factor.d[work.second];
            work.b_basis.set(work.second, work.first, value);
            work.rhs.set(work.second, work.first, value);
            work.second += 1;
            if work.second == n {
                work.second = 0;
                work.third = 0;
                work.phase = 5;
            }
            return Ok(());
        }
        if work.phase == 5 {
            if work.second == n {
                work.second = 0;
                work.third = 0;
                work.coefficient = 0.0;
                work.phase = 6;
                return Ok(());
            }
            let entries = &self.state.k_factor.l_cols[work.second];
            if work.third < entries.len() {
                let (row, value) = entries[work.third];
                let delta = value * work.b_basis.get(work.second, work.first);
                work.rhs.add_at(row as usize, work.first, delta);
                work.third += 1;
            } else {
                work.second += 1;
                work.third = 0;
            }
            return Ok(());
        }
        if work.phase == 6 {
            if work.second == work.first {
                work.third = 0;
                work.scalar = 0.0;
                work.phase = 8;
                return Ok(());
            }
            work.coefficient += work.solved.get(work.third, work.first) * work.rhs.get(work.third, work.second);
            work.third += 1;
            if work.third == n {
                work.third = 0;
                work.phase = 7;
            }
            return Ok(());
        }
        if work.phase == 7 {
            let coefficient = work.coefficient;
            work.solved.add_at(work.third, work.first, -coefficient * work.solved.get(work.third, work.second));
            work.rhs.add_at(work.third, work.first, -coefficient * work.rhs.get(work.third, work.second));
            work.third += 1;
            if work.third == n {
                work.second += 1;
                work.third = 0;
                work.coefficient = 0.0;
                work.phase = 6;
            }
            return Ok(());
        }
        if work.phase == 8 {
            work.scalar += work.solved.get(work.third, work.first) * work.rhs.get(work.third, work.first);
            work.third += 1;
            if work.third == n {
                work.coefficient = work.scalar.max(1e-300).sqrt().recip();
                work.third = 0;
                work.phase = 1;
            }
            return Ok(());
        }
        let scale = work.coefficient;
        let row = work.third;
        work.solved.set(row, work.first, work.solved.get(row, work.first) * scale);
        work.rhs.set(row, work.first, work.rhs.get(row, work.first) * scale);
        work.third += 1;
        if work.third == n {
            work.first += 1;
            work.second = 0;
            work.third = 0;
            work.phase = 0;
            work.scalar = 0.0;
            work.coefficient = 0.0;
            work.stage = SubspaceStage::OrthogonalizePairElement;
        }
        Ok(())
    }

    fn advance_projected(&mut self) {
        let m = self.state.m;
        let n = self.state.n;
        let work = &mut self.state.work;
        if work.phase == 1 {
            let row = work.first / m;
            let column = work.first % m;
            work.jacobi.set(row, column, 0.5 * (work.projected.get(row, column) + work.projected.get(column, row)));
            work.first += 1;
            if work.first == m * m {
                self.reset_cursor(SubspaceStage::JacobiConvergenceCell);
            }
            return;
        }
        work.scalar += work.solved.get(work.third, work.first) * work.b_basis.get(work.third, work.second);
        work.third += 1;
        if work.third < n {
            return;
        }
        work.projected.set(work.first, work.second, work.scalar);
        work.scalar = 0.0;
        work.third = 0;
        work.second += 1;
        if work.second == m {
            work.second = 0;
            work.first += 1;
        }
        if work.first == m {
            work.first = 0;
            work.second = 0;
            work.phase = 1;
        }
    }

    fn advance_jacobi_convergence(&mut self) {
        let m = self.state.m;
        let work = &mut self.state.work;
        if work.first < m * m {
            let row = work.first / m;
            let column = work.first % m;
            let value = work.jacobi.get(row, column);
            work.scalar += value * value;
            if row < column {
                work.coefficient += value * value;
            }
            work.first += 1;
            return;
        }
        if work.coefficient.sqrt() < 1e-12 * (work.scalar.sqrt() + 1.0) || work.sweep == 100 {
            work.first = 0;
            work.second = 0;
            work.phase = 0;
            work.stage = SubspaceStage::ModeSortCompare;
        } else {
            work.first = 0;
            work.second = 1;
            work.third = 0;
            work.stage = SubspaceStage::JacobiFindPairCell;
        }
        work.scalar = 0.0;
        work.coefficient = 0.0;
    }

    fn advance_jacobi_pair(&mut self) {
        let m = self.state.m;
        let work = &mut self.state.work;
        if work.first + 1 >= m {
            work.sweep += 1;
            self.reset_cursor(SubspaceStage::JacobiConvergenceCell);
            return;
        }
        if work.second >= m {
            work.first += 1;
            work.second = work.first + 1;
            return;
        }
        let p = work.first;
        let q = work.second;
        let apq = work.jacobi.get(p, q);
        if apq.abs() < 1e-300 {
            work.second += 1;
            return;
        }
        let theta = (work.jacobi.get(q, q) - work.jacobi.get(p, p)) / (2.0 * apq);
        work.tangent = if theta >= 0.0 { 1.0 / (theta + (theta * theta + 1.0).sqrt()) } else { -1.0 / (-theta + (theta * theta + 1.0).sqrt()) };
        work.cosine = 1.0 / (work.tangent * work.tangent + 1.0).sqrt();
        work.sine = work.tangent * work.cosine;
        work.phase = 0;
        work.third = 0;
        work.stage = SubspaceStage::JacobiRotateCell;
    }

    fn advance_jacobi_rotation(&mut self) {
        let m = self.state.m;
        let work = &mut self.state.work;
        let p = work.first;
        let q = work.second;
        match work.phase {
            0 => {
                let apq = work.jacobi.get(p, q);
                let app = work.jacobi.get(p, p);
                work.jacobi.set(p, p, app - work.tangent * apq);
                work.phase = 1;
            }
            1 => {
                let apq = work.jacobi.get(q, p);
                let aqq = work.jacobi.get(q, q);
                work.jacobi.set(q, q, aqq + work.tangent * apq);
                work.phase = 2;
            }
            2 => {
                work.jacobi.set(p, q, 0.0);
                work.phase = 3;
            }
            3 => {
                work.jacobi.set(q, p, 0.0);
                work.phase = 4;
            }
            4 => {
                if work.third == p || work.third == q {
                    work.third += 1;
                    return;
                }
                if work.third < m {
                    let row = work.third;
                    work.scalar = work.jacobi.get(row, p);
                    work.coefficient = work.jacobi.get(row, q);
                    work.phase = 5;
                } else {
                    work.third = 0;
                    work.phase = 9;
                }
            }
            5 => {
                let value = work.cosine * work.scalar - work.sine * work.coefficient;
                work.jacobi.set(work.third, p, value);
                work.phase = 6;
            }
            6 => {
                let value = work.cosine * work.scalar - work.sine * work.coefficient;
                work.jacobi.set(p, work.third, value);
                work.phase = 7;
            }
            7 => {
                let value = work.sine * work.scalar + work.cosine * work.coefficient;
                work.jacobi.set(work.third, q, value);
                work.phase = 8;
            }
            8 => {
                let value = work.sine * work.scalar + work.cosine * work.coefficient;
                work.jacobi.set(q, work.third, value);
                work.third += 1;
                work.phase = 4;
            }
            9 => {
                if work.third < m {
                    let row = work.third;
                    work.scalar = work.jacobi_vectors.get(row, p);
                    work.coefficient = work.jacobi_vectors.get(row, q);
                    work.phase = 10;
                } else {
                    work.second += 1;
                    work.third = 0;
                    work.stage = SubspaceStage::JacobiFindPairCell;
                }
            }
            10 => {
                work.jacobi_vectors.set(work.third, p, work.cosine * work.scalar - work.sine * work.coefficient);
                work.phase = 11;
            }
            _ => {
                work.jacobi_vectors.set(work.third, q, work.sine * work.scalar + work.cosine * work.coefficient);
                work.third += 1;
                work.phase = 9;
            }
        }
    }

    fn mode_key(&self, index: usize) -> f64 {
        if self.state.work.mu[index] > 1e-14 {
            self.state.work.mu[index].recip()
        } else {
            f64::MAX
        }
    }

    fn advance_mode_sort(&mut self) {
        let m = self.state.m;
        if self.state.work.phase == 0 {
            let index = self.state.work.first;
            self.state.work.mu[index] = self.state.work.jacobi.get(index, index);
            self.state.work.order[index] = index;
            self.state.work.first += 1;
            if self.state.work.first == m {
                self.state.work.first = 1;
                self.state.work.second = 1;
                self.state.work.phase = 1;
            }
            return;
        }
        if self.state.work.first >= m {
            self.reset_cursor(SubspaceStage::ModePermuteElement);
            return;
        }
        let position = self.state.work.second;
        if position == 0 {
            self.state.work.first += 1;
            self.state.work.second = self.state.work.first;
            return;
        }
        let left = self.state.work.order[position - 1];
        let right = self.state.work.order[position];
        let comparison = self.mode_key(left).total_cmp(&self.mode_key(right)).then_with(|| left.cmp(&right));
        if comparison.is_gt() {
            self.state.work.order.swap(position - 1, position);
            self.state.work.second -= 1;
        } else {
            self.state.work.first += 1;
            self.state.work.second = self.state.work.first;
        }
    }

    fn advance_mode_permute(&mut self) {
        let m = self.state.m;
        let n = self.state.n;
        let work = &mut self.state.work;
        if work.phase == 0 {
            if work.first < m {
                let source = work.order[work.first];
                work.theta[work.first] = if work.mu[source] > 1e-14 { work.mu[source].recip() } else { f64::MAX };
                work.first += 1;
            } else {
                work.first = 0;
                work.second = 0;
                work.phase = 1;
            }
            return;
        }
        if work.phase == 1 {
            let source = work.order[work.first];
            work.ordered_vectors.set(work.second, work.first, work.jacobi_vectors.get(work.second, source));
            work.second += 1;
            if work.second == m {
                work.second = 0;
                work.first += 1;
            }
            if work.first == m {
                work.first = 0;
                work.second = 0;
                work.third = 0;
                work.scalar = 0.0;
                work.phase = 2;
            }
            return;
        }
        work.scalar += work.solved.get(work.first, work.third) * work.ordered_vectors.get(work.third, work.second);
        work.third += 1;
        if work.third == m {
            work.candidate_x.set(work.first, work.second, work.scalar);
            work.third = 0;
            work.scalar = 0.0;
            work.second += 1;
            if work.second == m {
                work.second = 0;
                work.first += 1;
            }
        }
        if work.first == n {
            self.reset_cursor(SubspaceStage::ResidualColumnRow);
        }
    }

    fn advance_residual(&mut self) {
        let work = &mut self.state.work;
        if work.first < self.state.p {
            let value = work.theta[work.first];
            let previous = self.state.prev_theta[work.first];
            self.state.residuals[work.first] = if previous < f64::MAX { ((value - previous) / previous.abs().max(1e-12)).abs() } else { f64::MAX };
            work.first += 1;
        } else {
            work.first = 0;
            self.state.converged_count = 0;
            work.stage = SubspaceStage::ConvergenceMode;
        }
    }

    fn advance_convergence(&mut self) {
        let work = &mut self.state.work;
        if work.first < self.state.p {
            if self.state.residuals[work.first] < 1e-6 {
                self.state.converged_count += 1;
            }
            self.state.prev_theta[work.first] = work.theta[work.first];
            work.first += 1;
        } else {
            self.state.converged = self.state.converged_count == self.state.p;
            work.stage = SubspaceStage::PublishIteration;
        }
    }

    fn advance_preview_entry(state: &mut SubspaceCheckpoint, operation: Operation, writer: &mut RetainedJobPayloadWriter) -> Result<bool, JobPayloadAdmissionFault> {
        let cursor = NumericalPageCursor { field: state.publication_stage as u16, owner: state.publication_first, item: state.publication_second };
        if advance_numerical_page_header(writer, b"FEMSUB2\0", 2, cursor)? {
            return Ok(false);
        }
        match state.publication_stage {
            0 => {
                let values = [operation.operation.0, operation.base_revision.0, operation.generation.0, operation.seed, state.iteration as u64, state.converged_count as u64, state.converged as u64, state.n as u64, state.p as u64];
                if let Some(value) = values.get(state.publication_first) {
                    writer.write_staged(&value.to_le_bytes())?;
                    state.publication_first += 1;
                } else {
                    state.publication_first = 0;
                    state.publication_stage = 1;
                }
            }
            1 => {
                if let Some(value) = state.final_theta.get(state.publication_first).filter(|_| state.publication_first < state.p) {
                    writer.write_staged(&value.to_bits().to_le_bytes())?;
                    state.publication_first += 1;
                } else {
                    state.publication_first = 0;
                    state.publication_stage = 2;
                }
            }
            2 => {
                if state.publication_first < state.p {
                    writer.write_staged(&state.x.get(state.publication_second, state.publication_first).to_bits().to_le_bytes())?;
                    state.publication_second += 1;
                    if state.publication_second == state.n {
                        state.publication_second = 0;
                        state.publication_first += 1;
                    }
                } else {
                    state.publication_first = 0;
                    state.publication_stage = 3;
                }
            }
            3 => {
                if let Some(value) = state.residuals.get(state.publication_first).filter(|_| state.publication_first < state.p) {
                    writer.write_staged(&value.to_bits().to_le_bytes())?;
                    state.publication_first += 1;
                } else {
                    state.publication_stage = 4;
                }
            }
            4 => writer.commit_staged_page()?,
            _ => {}
        }
        Ok(state.publication_stage == 4 && writer.staged_page_len().is_none())
    }

    fn publish_iteration(&mut self) {
        if self.state.work.phase == 0 {
            if self.state.work.first == 0 {
                self.state.final_theta.clear();
            }
            if self.state.work.first < self.state.m {
                self.state.final_theta.push(self.state.work.theta[self.state.work.first]);
                self.state.work.first += 1;
                return;
            }
            self.state.work.phase = 1;
            return;
        }
        std::mem::swap(&mut self.state.x, &mut self.state.work.candidate_x);
        self.state.iteration += 1;
        self.state.checkpoint_due = true;
        self.state.preview_due = true;
        self.state.publication_stage = 0;
        self.state.publication_first = 0;
        self.state.publication_second = 0;
        let displaced = std::mem::replace(&mut self.state.work, SubspaceWork::empty());
        self.state.retiring_work = Some(displaced);
    }

    fn close_retained_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        match close_nested_vec_owner_step(&mut self.state.k_factor.l_cols, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        for owner in [&mut self.state.k_factor.d, &mut self.state.b.vals, &mut self.state.x.data, &mut self.state.prev_theta, &mut self.state.final_theta, &mut self.state.residuals] {
            match close_vec_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        for owner in [&mut self.state.b.indices, &mut self.state.b.indptr] {
            match close_vec_owner_step(owner, maximum_bytes) {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Err(()) => return (false, 0, 0),
                Ok(None) => {}
            }
        }
        let (terminal, items, bytes) = self.state.work.close_step(maximum_bytes);
        if !terminal {
            return (false, items, bytes);
        }
        if let Some(retiring) = self.state.retiring_work.as_mut() {
            let (terminal, items, bytes) = retiring.close_step(maximum_bytes);
            if !terminal {
                return (false, items, bytes);
            }
            self.state.retiring_work = None;
            return (false, 1, 0);
        }
        (true, 0, 0)
    }

    fn close_terminal_is_empty(&self) -> bool {
        self.state.k_factor.l_cols.capacity() == 0
            && self.state.k_factor.d.capacity() == 0
            && self.state.b.vals.capacity() == 0
            && self.state.b.indices.capacity() == 0
            && self.state.b.indptr.capacity() == 0
            && self.state.x.data.capacity() == 0
            && self.state.prev_theta.capacity() == 0
            && self.state.final_theta.capacity() == 0
            && self.state.residuals.capacity() == 0
            && self.state.work.terminal_is_empty()
            && self.state.retiring_work.is_none()
    }
}

fn restore_matrix_entry(matrix: &mut MatD, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize) -> Result<bool, NumericalCheckpointFault> {
    let rows = read_checkpoint_u64(page.bytes, 0)? as usize;
    let cols = read_checkpoint_u64(page.bytes, 8)? as usize;
    let capacity = rows.checked_mul(cols).ok_or(NumericalCheckpointFault::Envelope)?;
    let length = read_checkpoint_u64(page.bytes, 16)? as usize;
    if capacity > maximum || length > capacity || page.item != 0 || page.bytes.len() != 24usize.saturating_add(length.saturating_mul(8)) {
        return Err(NumericalCheckpointFault::Envelope);
    }
    if *entry == 0 {
        if !matrix.data.is_empty() {
            return Err(NumericalCheckpointFault::Field);
        }
        matrix.data.try_reserve_exact(capacity).map_err(|_| NumericalCheckpointFault::Admission)?;
        validate_restored_owner(&matrix.data)?;
        matrix.rows = rows;
        matrix.cols = cols;
        *entry = 1;
        return Ok(false);
    }
    if matrix.rows != rows || matrix.cols != cols {
        return Err(NumericalCheckpointFault::Field);
    }
    let item = *entry - 1;
    if matrix.data.len() != item {
        return Err(NumericalCheckpointFault::Field);
    }
    if item < length {
        matrix.data.push(f64::from_bits(read_checkpoint_u64(page.bytes, 24 + item * 8)?));
        *entry += 1;
        return Ok(false);
    }
    Ok(true)
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
    work.close_lane = values[12] as u8;
    Ok(())
}

fn restore_work_entry(work: &mut SubspaceWork, page: &NumericalPageView<'_>, base: u16, n: usize, m: usize, entry: &mut usize, control: &mut [u64; 24]) -> Result<bool, NumericalCheckpointFault> {
    match page.field - base {
        0 => {
            let count = declared_owner_length(page, 13)?;
            if count != 13 || page.bytes.len() != 8 + count * 8 {
                return Err(NumericalCheckpointFault::Truncated);
            }
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
        1 => restore_matrix_entry(&mut work.rhs, page, n.saturating_mul(m), entry),
        2 => restore_matrix_entry(&mut work.solved, page, n.saturating_mul(m), entry),
        3 => restore_matrix_entry(&mut work.b_basis, page, n.saturating_mul(m), entry),
        4 => restore_matrix_entry(&mut work.projected, page, m.saturating_mul(m), entry),
        5 => restore_matrix_entry(&mut work.jacobi, page, m.saturating_mul(m), entry),
        6 => restore_matrix_entry(&mut work.jacobi_vectors, page, m.saturating_mul(m), entry),
        7 => restore_matrix_entry(&mut work.ordered_vectors, page, m.saturating_mul(m), entry),
        8 => restore_matrix_entry(&mut work.candidate_x, page, n.saturating_mul(m), entry),
        9 => restore_f64_entry(&mut work.mu, page, m, entry),
        10 => restore_f64_entry(&mut work.theta, page, m, entry),
        11 => restore_usize_entry(&mut work.order, page, m, entry),
        _ => Err(NumericalCheckpointFault::Field),
    }
}

pub struct SubspaceRestoreCursor {
    operation: Operation,
    payload: Option<RetainedJobPayload>,
    total_pages: usize,
    page_slot: usize,
    close_due: bool,
    expected_field: u16,
    page_entry: usize,
    control: [u64; 24],
    state: Option<SubspaceCheckpoint>,
    fault: Option<NumericalCheckpointFault>,
}

impl SubspaceRestoreCursor {
    pub fn new(operation: Operation, payload: RetainedJobPayload) -> Self {
        let total_pages = payload.page_count();
        Self { operation, payload: Some(payload), total_pages, page_slot: 0, close_due: false, expected_field: 0, page_entry: 0, control: [0; 24], state: None, fault: None }
    }

    fn decode_page_entry(&mut self, bytes: &[u8]) -> Result<bool, NumericalCheckpointFault> {
        let page = parse_numerical_page(bytes, b"FEMSCP1\0")?;
        if page.kind != 12 || page.field != self.expected_field || page.owner != 0 || page.item != 0 {
            return Err(NumericalCheckpointFault::Field);
        }
        if page.field == 0 {
            let count = declared_owner_length(&page, 24)?;
            if count != 20 || page.bytes.len() != 8 + count * 8 {
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
            let p = value(5) as usize;
            let m = value(7) as usize;
            if n == 0 || n > SUBSPACE_MAXIMUM_ORDER || p == 0 || p > n || m == 0 || m > SUBSPACE_MAXIMUM_COLUMNS || m > n {
                return Err(NumericalCheckpointFault::Envelope);
            }
            self.state = Some(SubspaceCheckpoint {
                identity,
                k_factor: LdltFactor { n, l_cols: Vec::new(), d: Vec::new() },
                b: Csr { n, indptr: Vec::new(), indices: Vec::new(), vals: Vec::new() },
                n,
                p,
                max_iter: value(6) as usize,
                m,
                x: MatD::zeros(0, 0),
                prev_theta: Vec::new(),
                final_theta: Vec::new(),
                iteration: value(8) as usize,
                residuals: Vec::new(),
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
            1 => {
                let length = declared_owner_length(&page, n)?;
                if length != n || page.bytes.len() != 8 {
                    return Err(NumericalCheckpointFault::Field);
                }
                if self.page_entry == 0 {
                    state.k_factor.l_cols.try_reserve_exact(n).map_err(|_| NumericalCheckpointFault::Admission)?;
                    validate_restored_owner(&state.k_factor.l_cols)?;
                    self.page_entry = 1;
                    return Ok(false);
                }
                if state.k_factor.l_cols.len() < n {
                    state.k_factor.l_cols.push(Vec::new());
                    self.page_entry += 1;
                    return Ok(false);
                }
                true
            }
            field if field >= 2 && field < 2 + n as u16 => restore_pair_entry(&mut state.k_factor.l_cols[(field - 2) as usize], &page, n, &mut self.page_entry)?,
            514 => restore_f64_entry(&mut state.k_factor.d, &page, n, &mut self.page_entry)?,
            515 => restore_u32_entry(&mut state.b.indptr, &page, n + 1, &mut self.page_entry)?,
            516 => restore_u32_entry(&mut state.b.indices, &page, n.saturating_mul(n), &mut self.page_entry)?,
            517 => restore_f64_entry(&mut state.b.vals, &page, n.saturating_mul(n), &mut self.page_entry)?,
            518 => restore_matrix_entry(&mut state.x, &page, n.saturating_mul(m), &mut self.page_entry)?,
            519 => restore_f64_entry(&mut state.prev_theta, &page, state.p, &mut self.page_entry)?,
            520 => restore_f64_entry(&mut state.final_theta, &page, m, &mut self.page_entry)?,
            521 => restore_f64_entry(&mut state.residuals, &page, state.p, &mut self.page_entry)?,
            field if (522..=533).contains(&field) => restore_work_entry(&mut state.work, &page, 522, n, m, &mut self.page_entry, &mut self.control)?,
            534 => {
                let present = read_checkpoint_u64(page.bytes, 0)?;
                if self.page_entry == 0 {
                    self.page_entry = 1;
                    return Ok(false);
                }
                match present {
                    0 if page.bytes.len() == 8 => true,
                    1 => {
                        let count = read_checkpoint_u64(page.bytes, 8)? as usize;
                        if count != 13 || page.bytes.len() != 16 + count * 8 {
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
            field if (535..=545).contains(&field) => {
                let work = state.retiring_work.as_mut().ok_or(NumericalCheckpointFault::Field)?;
                restore_work_entry(work, &page, 534, n, m, &mut self.page_entry, &mut self.control)?
            }
            _ => return Err(NumericalCheckpointFault::Field),
        };
        if complete {
            self.expected_field = match page.field {
                1 if n == 0 => 514,
                1 => 2,
                field if field >= 2 && field + 1 < 2 + n as u16 => field + 1,
                field if field >= 2 && field < 2 + n as u16 => 514,
                field if (514..=533).contains(&field) => field + 1,
                534 if state.retiring_work.is_some() => 535,
                534 => u16::MAX,
                field if (535..545).contains(&field) => field + 1,
                545 => u16::MAX,
                _ => return Err(NumericalCheckpointFault::Field),
            };
        }
        Ok(complete)
    }

    pub fn step(&mut self, context: &mut StepContext<'_>) -> Result<Option<SubspaceIterationJob>, NumericalCheckpointFault> {
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
            let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            self.page_slot += 1;
            self.close_due = false;
            self.page_entry = 0;
            if self.page_slot == self.total_pages {
                if self.expected_field != u16::MAX || !payload.terminal_is_empty() {
                    return Err(NumericalCheckpointFault::Truncated);
                }
                self.payload = None;
                let state = self.state.take().ok_or(NumericalCheckpointFault::Truncated)?;
                return Ok(Some(SubspaceIterationJob {
                    operation: self.operation,
                    state,
                    preview_writer: None,
                    preview_page_cursor: 0,
                    terminal_writer: None,
                    terminal_page_cursor: 0,
                    checkpoint_writer: None,
                    checkpoint_cursor: NumericalPageCursor::new(),
                }));
            }
            return Ok(None);
        }
        let payload = self.payload.take().ok_or(NumericalCheckpointFault::Truncated)?;
        let decoded = payload.page(self.page_slot).ok_or(NumericalCheckpointFault::Truncated).and_then(|source| self.decode_page_entry(source));
        self.payload = Some(payload);
        match decoded {
            Ok(true) => self.close_due = true,
            Ok(false) => {}
            Err(fault) => {
                self.fault = Some(fault);
                return Err(fault);
            }
        }
        Ok(None)
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(payload) = self.payload.as_mut() {
            if !payload.terminal_is_empty() {
                return match payload.close_step(1, maximum_bytes) {
                    semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    semio_framework_job::JobPayloadCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 },
                };
            }
            self.payload = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        let Some(state) = self.state.as_mut() else { return semio_framework_job::InteractiveJobCloseStep::Complete };
        if let Some(work) = state.retiring_work.as_mut() {
            let (complete, released_items, released_bytes) = work.close_step(maximum_bytes);
            if complete {
                state.retiring_work = None;
            }
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        let (work_complete, released_items, released_bytes) = state.work.close_step(maximum_bytes);
        if !work_complete {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        if let Ok(Some((released_items, released_bytes))) = close_nested_vec_owner_step(&mut state.k_factor.l_cols, maximum_bytes) {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
        }
        for owner in [&mut state.k_factor.d, &mut state.b.vals, &mut state.x.data, &mut state.prev_theta, &mut state.final_theta, &mut state.residuals] {
            if let Ok(Some((released_items, released_bytes))) = close_vec_owner_step(owner, maximum_bytes) {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
            }
        }
        for owner in [&mut state.b.indices, &mut state.b.indptr] {
            if let Ok(Some((released_items, released_bytes))) = close_vec_owner_step(owner, maximum_bytes) {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes };
            }
        }
        self.state = None;
        semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.payload.is_none() && self.state.is_none()
    }
}

impl InteractiveJob for SubspaceIterationJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
        }
        if !self.state.factor_validation_complete {
            context.set_stage("fem.subspace.validate-factor-owner");
            if context.should_yield() {
                return StepOutcome::Yield;
            }
            context.consume_fuel(1);
            if let Some(column) = self.state.k_factor.l_cols.get(self.state.factor_validation_cursor) {
                if column.capacity().saturating_mul(std::mem::size_of::<(u32, f64)>()) > NUMERICAL_OWNER_PAGE_BYTES {
                    self.state.admission_fault = true;
                    self.state.factor_validation_complete = true;
                    return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
                }
                self.state.factor_validation_cursor += 1;
                return StepOutcome::Yield;
            }
            self.state.factor_validation_complete = true;
            return StepOutcome::Yield;
        }
        if self.state.admission_fault || self.state.n > SUBSPACE_MAXIMUM_ORDER || self.state.m > SUBSPACE_MAXIMUM_COLUMNS {
            return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
        }
        if self.state.checkpoint_due || self.checkpoint_writer.is_some() {
            context.set_stage("fem.subspace.checkpoint-page");
            if context.should_yield() {
                return StepOutcome::Yield;
            }
            context.consume_fuel(1);
            if self.checkpoint_writer.is_none() {
                self.state.checkpoint_due = false;
                self.checkpoint_cursor = NumericalPageCursor::new();
                self.checkpoint_writer = Some(RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState));
                return StepOutcome::Yield;
            }
            let Some(writer) = self.checkpoint_writer.as_mut() else {
                return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
            };
            if writer.staged_page_len().is_none() {
                return match writer.begin_staged_page(context) {
                    Ok(()) => StepOutcome::Yield,
                    Err(_) => StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
                };
            }
            let complete = match Self::advance_checkpoint_entry(&self.state, &mut self.checkpoint_cursor, writer) {
                Ok(complete) => complete,
                Err(_) => return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
            };
            if !complete {
                return StepOutcome::Yield;
            }
            let Some(writer) = self.checkpoint_writer.take() else {
                return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
            };
            let state = match writer.finish() {
                Ok(state) => state,
                Err(writer) => {
                    self.checkpoint_writer = Some(writer);
                    return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
                }
            };
            return StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state, applied_progress: self.state.iteration as u64 });
        }
        if self.state.preview_due {
            context.set_stage("fem.subspace.preview-entry");
            if context.should_yield() {
                return StepOutcome::Yield;
            }
            if self.preview_writer.is_none() {
                context.consume_fuel(1);
                self.preview_writer = Some(RetainedJobPayloadWriter::new(JobPayloadStream::Preview));
                self.preview_page_cursor = 0;
                return StepOutcome::Yield;
            }
            context.consume_fuel(1);
            let Some(writer) = self.preview_writer.as_mut() else {
                return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
            };
            if writer.staged_page_len().is_none() && self.state.publication_stage < 4 {
                return match writer.begin_staged_page(context) {
                    Ok(()) => StepOutcome::Yield,
                    Err(_) => StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
                };
            }
            let complete = match Self::advance_preview_entry(&mut self.state, self.operation, writer) {
                Ok(complete) => complete,
                Err(_) => return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
            };
            if !complete {
                return StepOutcome::Yield;
            }
            let Some(writer) = self.preview_writer.take() else {
                return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
            };
            let preview = match writer.finish() {
                Ok(preview) => preview,
                Err(writer) => {
                    self.preview_writer = Some(writer);
                    return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
                }
            };
            self.preview_page_cursor = 0;
            self.state.preview_due = false;
            return StepOutcome::PreviewReady(preview);
        }
        if self.state.iteration >= self.state.max_iter || self.state.converged {
            context.set_stage("fem.subspace.terminal-page");
            if context.should_yield() {
                return StepOutcome::Yield;
            }
            context.consume_fuel(1);
            if self.terminal_writer.is_none() {
                self.state.publication_stage = 0;
                self.state.publication_first = 0;
                self.state.publication_second = 0;
                self.terminal_writer = Some(RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput));
                self.terminal_page_cursor = 0;
                return StepOutcome::Yield;
            }
            let Some(writer) = self.terminal_writer.as_mut() else {
                return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
            };
            if writer.staged_page_len().is_none() && self.state.publication_stage < 4 {
                return match writer.begin_staged_page(context) {
                    Ok(()) => StepOutcome::Yield,
                    Err(_) => StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
                };
            }
            let complete = match Self::advance_preview_entry(&mut self.state, self.operation, writer) {
                Ok(complete) => complete,
                Err(_) => return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) }),
            };
            if !complete {
                return StepOutcome::Yield;
            }
            let Some(writer) = self.terminal_writer.take() else {
                return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
            };
            let output = match writer.finish() {
                Ok(output) => output,
                Err(writer) => {
                    self.terminal_writer = Some(writer);
                    return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
                }
            };
            return StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output });
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
        });
        if context.should_yield() {
            return StepOutcome::Yield;
        }
        context.consume_fuel(1);
        let result = match self.state.work.stage {
            SubspaceStage::ReserveIteration => self.reserve_iteration_owner(),
            SubspaceStage::ApplyOperatorColumnRow => {
                self.advance_apply_operator();
                Ok(())
            }
            SubspaceStage::FactorForwardEntry => {
                self.advance_factor_forward();
                Ok(())
            }
            SubspaceStage::FactorDiagonalEntry => {
                self.advance_factor_diagonal();
                Ok(())
            }
            SubspaceStage::FactorBackwardEntry => {
                self.advance_factor_backward();
                Ok(())
            }
            SubspaceStage::OrthogonalizePairElement => self.advance_orthogonalize(),
            SubspaceStage::NormalizeColumnElement => self.advance_normalize(),
            SubspaceStage::ProjectedMatrixCellEntry => {
                self.advance_projected();
                Ok(())
            }
            SubspaceStage::JacobiFindPairCell => {
                self.advance_jacobi_pair();
                Ok(())
            }
            SubspaceStage::JacobiRotateCell => {
                self.advance_jacobi_rotation();
                Ok(())
            }
            SubspaceStage::JacobiConvergenceCell => {
                self.advance_jacobi_convergence();
                Ok(())
            }
            SubspaceStage::ModeSortCompare => {
                self.advance_mode_sort();
                Ok(())
            }
            SubspaceStage::ModePermuteElement => {
                self.advance_mode_permute();
                Ok(())
            }
            SubspaceStage::ResidualColumnRow => {
                self.advance_residual();
                Ok(())
            }
            SubspaceStage::ConvergenceMode => {
                self.advance_convergence();
                Ok(())
            }
            SubspaceStage::PublishIteration => {
                self.publish_iteration();
                Ok(())
            }
        };
        if result.is_err() {
            return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
        }
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        StepOutcome::Yield
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

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(writer) = self.checkpoint_writer.as_mut() {
            return match writer.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    self.checkpoint_writer = None;
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
            };
        }
        if let Some(writer) = self.preview_writer.as_mut() {
            return match writer.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    self.preview_writer = None;
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
            };
        }
        if let Some(writer) = self.terminal_writer.as_mut() {
            return match writer.close_step(maximum_items, maximum_bytes) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                semio_framework_job::JobPayloadCloseStep::Complete => {
                    self.terminal_writer = None;
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
            };
        }
        let (complete, released_items, released_bytes) = self.close_retained_step(maximum_bytes);
        if complete {
            semio_framework_job::InteractiveJobCloseStep::Complete
        } else {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.checkpoint_writer.is_none() && self.preview_writer.is_none() && self.terminal_writer.is_none() && self.close_terminal_is_empty()
    }
}

/// 🧹️ Closes retained batch-adapter payload pages before the next solver grant.
fn close_batch_payload(mut payload: RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
}

/// 🎯️ Finds the lowest `p` generalized eigenpairs through the retained subspace cursor.
pub fn subspace_iteration(k_factor: &LdltFactor, b: &Csr, n: usize, p: usize, max_iter: usize) -> EigenPairs {
    let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), 0);
    let mut job = SubspaceIterationJob::new(operation, k_factor.clone(), b.clone(), n, p, max_iter);
    let mut preview_sequence = 0;
    loop {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut preview_sequence);
        match job.step(&mut context) {
            StepOutcome::Complete(candidate) => {
                close_batch_payload(candidate.state);
                close_batch_payload(candidate.output);
                let solution = job.solution();
                while !job.terminal_is_empty() {
                    let _ = job.close_step(1, usize::MAX);
                }
                return solution;
            }
            StepOutcome::CheckpointReady(checkpoint) => close_batch_payload(checkpoint.state),
            StepOutcome::PreviewReady(preview) => close_batch_payload(preview),
            StepOutcome::Fault(fault) => {
                close_batch_payload(fault.detail);
                panic!("subspace batch adapter faulted")
            }
            _ => {}
        }
    }
}
// #endregion 🔖️SubspaceIteration

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
mod tests {
    use super::*;

    fn graph_laplacian_plus_identity(n: usize, edges: &[(usize, usize)]) -> Coo {
        let mut degree = vec![0usize; n];
        for &(u, v) in edges {
            degree[u] += 1;
            degree[v] += 1;
        }
        let mut coo = Coo::new(n);
        for i in 0..n {
            coo.add(i, i, degree[i] as f64 + 1.0);
        }
        for &(u, v) in edges {
            coo.add(u, v, -1.0);
            coo.add(v, u, -1.0);
        }
        coo
    }

    #[test]
    fn ldlt_matches_dense_lu_on_random_spd() {
        let n = 8;
        let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 0), (0, 4), (2, 6)];
        let coo = graph_laplacian_plus_identity(n, &edges);
        let factor = ldlt_factor(&coo.to_csc_sym_upper()).expect("factors");
        let x_expected = VecD::from_vec((0..n).map(|i| (i as f64) * 0.5 + 1.0).collect());
        let dense = coo.to_dense();
        let b = dense.mul_vec(&x_expected);
        let x_ldlt = factor.solve(&b);
        let x_lu = dense.lu_solve(&b).expect("dense solvable");
        for i in 0..n {
            assert!((x_ldlt.get(i) - x_expected.get(i)).abs() < 1e-8);
            assert!((x_ldlt.get(i) - x_lu.get(i)).abs() < 1e-8);
        }
    }

    #[test]
    fn ldlt_matches_dense_lu_on_1d_laplacian() {
        let n = 20;
        let mut coo = Coo::new(n);
        for i in 0..n {
            coo.add(i, i, 2.0);
            if i + 1 < n {
                coo.add(i, i + 1, -1.0);
                coo.add(i + 1, i, -1.0);
            }
        }
        let factor = ldlt_factor(&coo.to_csc_sym_upper()).expect("factors");
        let x_expected = VecD::from_vec((0..n).map(|i| ((i % 5) as f64) - 1.5).collect());
        let dense = coo.to_dense();
        let b = dense.mul_vec(&x_expected);
        let x_ldlt = factor.solve(&b);
        let x_lu = dense.lu_solve(&b).expect("dense solvable");
        for i in 0..n {
            assert!((x_ldlt.get(i) - x_lu.get(i)).abs() < 1e-8);
        }
    }

    #[test]
    fn ldlt_solve_many_matches_solve_per_column() {
        let n = 8;
        let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 0), (0, 4), (2, 6)];
        let coo = graph_laplacian_plus_identity(n, &edges);
        let factor = ldlt_factor(&coo.to_csc_sym_upper()).expect("factors");
        let mut rhs = MatD::zeros(n, 3);
        for r in 0..n {
            rhs.set(r, 0, r as f64 + 1.0);
            rhs.set(r, 1, (n - r) as f64);
            rhs.set(r, 2, if r % 2 == 0 { 1.0 } else { -1.0 });
        }
        let combined = factor.solve_many(&rhs);
        for c in 0..3 {
            let col = VecD::from_vec((0..n).map(|r| rhs.get(r, c)).collect());
            let single = factor.solve(&col);
            for r in 0..n {
                assert!((combined.get(r, c) - single.get(r)).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn ldlt_reports_zero_pivot_on_singular_matrix() {
        let n = 5;
        let edges = [(0, 1), (1, 2), (2, 3)];
        let mut degree = vec![0usize; n];
        for &(u, v) in &edges {
            degree[u] += 1;
            degree[v] += 1;
        }
        let mut coo = Coo::new(n);
        for i in 0..4 {
            coo.add(i, i, degree[i] as f64 + 1.0);
        }
        for &(u, v) in &edges {
            coo.add(u, v, -1.0);
            coo.add(v, u, -1.0);
        }
        match ldlt_factor(&coo.to_csc_sym_upper()) {
            Err(SparseError::ZeroPivot { column }) => assert_eq!(column, 4),
            other => panic!("expected zero pivot error, got {other:?}"),
        }
    }

    #[test]
    fn pcg_matches_ldlt_and_dense_lu() {
        let n = 8;
        let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 0), (0, 4), (2, 6)];
        let coo = graph_laplacian_plus_identity(n, &edges);
        let dense = coo.to_dense();
        let x_expected = VecD::from_vec((0..n).map(|i| (i as f64) * 0.3 - 1.0).collect());
        let b = dense.mul_vec(&x_expected);
        let csr = coo.to_csr();
        let mut x0 = VecD::zeros(n);
        let stats = pcg(&csr, &b, &mut x0, 1e-10, 500);
        assert!(stats.converged);
        let lu = dense.lu_solve(&b).expect("dense solvable");
        for i in 0..n {
            assert!((x0.get(i) - lu.get(i)).abs() < 1e-6);
        }
    }

    #[test]
    fn rcm_reduces_bandwidth_on_scattered_path_graph() {
        let shuffle = [9usize, 0, 8, 1, 7, 2, 6, 3, 5, 4];
        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); 10];
        let mut edges = Vec::new();
        for i in 0..9 {
            let (u, v) = (shuffle[i], shuffle[i + 1]);
            adjacency[u].push(v);
            adjacency[v].push(u);
            edges.push((u, v));
        }
        let bandwidth = |index_of: &dyn Fn(usize) -> usize| -> usize { edges.iter().map(|&(u, v)| (index_of(u) as i64 - index_of(v) as i64).unsigned_abs() as usize).max().unwrap() };
        let before = bandwidth(&|x| x);
        let perm = rcm_order(&adjacency);
        let mut new_index = vec![0usize; 10];
        for (new_idx, &old_idx) in perm.iter().enumerate() {
            new_index[old_idx] = new_idx;
        }
        let after = bandwidth(&|x| new_index[x]);
        assert!(after <= before);
    }

    #[test]
    fn dense_symmetric_eigen_jacobi_matches_known_eigenvalues() {
        let mut a = MatD::zeros(3, 3);
        a.set(0, 0, 3.0);
        a.set(1, 1, 1.0);
        a.set(2, 2, 2.0);
        let (vals, _vecs) = dense_symmetric_eigen_jacobi(&a);
        assert!((vals[0] - 1.0).abs() < 1e-9);
        assert!((vals[1] - 2.0).abs() < 1e-9);
        assert!((vals[2] - 3.0).abs() < 1e-9);
    }

    #[test]
    fn subspace_iteration_matches_diagonal_analytic_case() {
        let n = 10;
        let mut k_coo = Coo::new(n);
        let mut b_coo = Coo::new(n);
        for i in 0..n {
            k_coo.add(i, i, (i + 1) as f64);
            b_coo.add(i, i, 1.0);
        }
        let k_factor = ldlt_factor(&k_coo.to_csc_sym_upper()).expect("factors");
        let b_csr = b_coo.to_csr();
        let pairs = subspace_iteration(&k_factor, &b_csr, n, 4, 30);
        let expected = [1.0, 2.0, 3.0, 4.0];
        for i in 0..4 {
            assert!((pairs.values[i] - expected[i]).abs() / expected[i] < 1e-4, "eigenvalue {} = {} expected {}", i, pairs.values[i], expected[i]);
        }
    }

    #[test]
    fn subspace_iteration_matches_dense_jacobi_on_small_nondiagonal_case() {
        let n = 7;
        let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 0), (0, 3)];
        let k_coo = graph_laplacian_plus_identity(n, &edges);
        let dense_k = k_coo.to_dense();
        let (dense_vals, _) = dense_symmetric_eigen_jacobi(&dense_k);

        let mut b_coo = Coo::new(n);
        for i in 0..n {
            b_coo.add(i, i, 1.0);
        }
        let k_factor = ldlt_factor(&k_coo.to_csc_sym_upper()).expect("factors");
        let b_csr = b_coo.to_csr();
        let pairs = subspace_iteration(&k_factor, &b_csr, n, 3, 30);

        for i in 0..3 {
            assert!((pairs.values[i] - dense_vals[i]).abs() / dense_vals[i].abs().max(1e-9) < 1e-3);
        }
    }

    /// 🕳️ Indefinite and null generalized modes sort behind physical positive modes using the same
    /// finite sentinel on every replay, so checkpoints remain valid JSON and ordering is total.
    #[test]
    fn subspace_iteration_uses_a_deterministic_finite_null_mode_sentinel() {
        let n = 3;
        let mut k_coo = Coo::new(n);
        let mut b_coo = Coo::new(n);
        for (index, stiffness) in [1.0, 2.0, 3.0].into_iter().enumerate() {
            k_coo.add(index, index, stiffness);
        }
        b_coo.add(0, 0, 1.0);
        b_coo.add(2, 2, -1.0);
        let k_factor = ldlt_factor(&k_coo.to_csc_sym_upper()).expect("factors");
        let b_csr = b_coo.to_csr();

        let first = subspace_iteration(&k_factor, &b_csr, n, 3, 30);
        let replay = subspace_iteration(&k_factor, &b_csr, n, 3, 30);

        assert_eq!(first, replay);
        assert_eq!(first.values, vec![1.0, f64::MAX, f64::MAX]);
        assert!(first.values.iter().all(|value| value.is_finite() && *value > 0.0));
        assert!(first.values.windows(2).all(|pair| pair[0] <= pair[1]));
    }

    /// 🔍️ `CscSym::get` reads back every entry of a symmetric matrix (both `row<=col` and `row>col`
    /// orderings resolve to the same stored upper-triangle slot) and returns `0.0` for an absent entry;
    /// `to_csr_full` mirrors the SAME matrix into a full (both triangles materialized) `Csr`.
    #[test]
    fn csc_sym_get_and_to_csr_full_match_dense() {
        let mut coo = Coo::new(3);
        coo.add(0, 0, 4.0);
        coo.add(1, 1, 5.0);
        coo.add(2, 2, 6.0);
        coo.add(0, 1, 2.0);
        coo.add(1, 0, 2.0);
        coo.add(1, 2, 3.0);
        coo.add(2, 1, 3.0);
        let dense = coo.to_dense();
        let csc = coo.to_csc_sym_upper();

        for r in 0..3 {
            for c in 0..3 {
                assert!((csc.get(r, c) - dense.get(r, c)).abs() < 1e-12, "get({r},{c}) = {} vs dense {}", csc.get(r, c), dense.get(r, c));
            }
        }
        assert_eq!(csc.get(0, 2), 0.0, "no (0,2) entry was ever added");

        let full = csc.to_csr_full();
        let x = VecD::from_vec(vec![1.0, 2.0, 3.0]);
        let expected = dense.mul_vec(&x);
        let actual = full.mul_vec(&x);
        for i in 0..3 {
            assert!((actual.get(i) - expected.get(i)).abs() < 1e-9, "mul_vec[{i}] = {} vs {}", actual.get(i), expected.get(i));
        }
    }

    /// 🔢️ `negative_pivot_count` counts `D[j] < 0` — a diagonal (already-factored-trivially) indefinite
    /// matrix with one negative entry must report exactly one negative pivot.
    #[test]
    fn negative_pivot_count_counts_negative_diagonal_entries() {
        let mut coo = Coo::new(3);
        coo.add(0, 0, 1.0);
        coo.add(1, 1, -2.0);
        coo.add(2, 2, 3.0);
        let factor = ldlt_factor(&coo.to_csc_sym_upper()).expect("diagonal matrix factors trivially");
        assert_eq!(factor.negative_pivot_count(), 1);
    }

    /// ⏱️ `pcg` returns immediately (zero iterations, `converged: true`) when the initial guess `x0`
    /// already satisfies the residual tolerance.
    #[test]
    fn pcg_converges_immediately_when_initial_guess_is_already_exact() {
        let mut coo = Coo::new(3);
        coo.add(0, 0, 2.0);
        coo.add(1, 1, 3.0);
        coo.add(2, 2, 4.0);
        let csr = coo.to_csr();
        let mut x0 = VecD::from_vec(vec![1.0, 2.0, 3.0]);
        let b = csr.mul_vec(&x0);
        let stats = pcg(&csr, &b, &mut x0, 1e-8, 100);
        assert_eq!(stats.iterations, 0);
        assert!(stats.converged);
    }

    /// ⏱️ `pcg` with `max_iter: 0` never enters its iteration loop and reports `converged: false`.
    #[test]
    fn pcg_reports_not_converged_when_max_iter_is_zero() {
        let mut coo = Coo::new(3);
        coo.add(0, 0, 2.0);
        coo.add(1, 1, 3.0);
        coo.add(2, 2, 4.0);
        let csr = coo.to_csr();
        let b = VecD::from_vec(vec![1.0, 1.0, 1.0]);
        let mut x0 = VecD::zeros(3);
        let stats = pcg(&csr, &b, &mut x0, 1e-12, 0);
        assert_eq!(stats.iterations, 0);
        assert!(!stats.converged);
    }

    /// ⏱️ `pcg` against an all-zero operator has zero search-direction curvature (`pᵀAp = 0`) on its
    /// very first step, hitting the early `break` guard against dividing by zero — reported as
    /// `converged: false` after exactly 1 iteration.
    #[test]
    fn pcg_breaks_on_zero_curvature_direction() {
        let coo = Coo::new(3); // no entries added: A is the zero operator
        let csr = coo.to_csr();
        let b = VecD::from_vec(vec![1.0, 1.0, 1.0]);
        let mut x0 = VecD::zeros(3);
        let stats = pcg(&csr, &b, &mut x0, 1e-12, 50);
        assert_eq!(stats.iterations, 1);
        assert!(!stats.converged);
    }

    /// 🎯️ `dense_symmetric_eigen_jacobi` on a 0x0 matrix returns empty eigenvalues/eigenvectors instead
    /// of looping — the degenerate size `subspace_iteration`'s own `.max(1)` guard against normally
    /// avoids, but the helper itself must still handle directly.
    #[test]
    fn dense_symmetric_eigen_jacobi_handles_zero_size_matrix() {
        let a = MatD::zeros(0, 0);
        let (vals, vecs) = dense_symmetric_eigen_jacobi(&a);
        assert!(vals.is_empty());
        assert_eq!(vecs.rows, 0);
        assert_eq!(vecs.cols, 0);
    }

    fn test_operation(id: u64) -> Operation {
        Operation::new(semio_framework_job::OperationId(id), semio_framework_job::RevisionId(7), semio_framework_job::Generation(3), 11)
    }

    fn drive_pcg_job(mut job: PcgJob, operation: Operation) -> (VecD, PcgStats) {
        let mut sequence = 0;
        loop {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            match job.step(&mut context) {
                StepOutcome::Complete(_) => {
                    let (solution, stats) = job.solution();
                    return (solution.clone(), stats);
                }
                StepOutcome::Fault(fault) => panic!("pcg fault: {}", String::from_utf8_lossy(&fault.detail)),
                StepOutcome::Cancelled => panic!("pcg unexpectedly cancelled"),
                _ => {}
            }
        }
    }

    fn restore_ldlt(operation: Operation, payload: RetainedJobPayload) -> Result<LdltJob, NumericalCheckpointFault> {
        let mut restore = LdltRestoreCursor::new(operation, payload);
        let mut sequence = 0;
        for _ in 0..200_000 {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            match restore.step(&mut context) {
                Ok(Some(job)) => return Ok(job),
                Ok(None) => {}
                Err(fault) => {
                    while !restore.terminal_is_empty() {
                        let _ = restore.close_step(1, usize::MAX);
                    }
                    return Err(fault);
                }
            }
        }
        Err(NumericalCheckpointFault::Truncated)
    }

    fn restore_subspace(operation: Operation, payload: RetainedJobPayload) -> Result<SubspaceIterationJob, NumericalCheckpointFault> {
        let mut restore = SubspaceRestoreCursor::new(operation, payload);
        let mut sequence = 0;
        for _ in 0..400_000 {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            match restore.step(&mut context) {
                Ok(Some(job)) => return Ok(job),
                Ok(None) => {}
                Err(fault) => {
                    while !restore.terminal_is_empty() {
                        let _ = restore.close_step(1, usize::MAX);
                    }
                    return Err(fault);
                }
            }
        }
        Err(NumericalCheckpointFault::Truncated)
    }

    fn close_payload(mut payload: RetainedJobPayload) {
        while !payload.terminal_is_empty() {
            let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
    }

    #[test]
    fn pcg_job_is_batch_deterministic_and_matches_reference() {
        let n = 24;
        let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
        let csr = graph_laplacian_plus_identity(n, &edges).to_csr();
        let b = VecD::from_vec((0..n).map(|i| (i + 1) as f64).collect());
        let operation = test_operation(101);
        let (one, one_stats) = drive_pcg_job(PcgJob::new(operation, csr.clone(), b.clone(), VecD::zeros(n), 1e-11, 200, 1), operation);
        let (wide, wide_stats) = drive_pcg_job(PcgJob::new(operation, csr.clone(), b.clone(), VecD::zeros(n), 1e-11, 200, 97), operation);
        let mut reference = VecD::zeros(n);
        let reference_stats = pcg(&csr, &b, &mut reference, 1e-11, 200);
        assert_eq!(one, wide);
        assert_eq!(one_stats, wide_stats);
        assert_eq!(one, reference);
        assert_eq!(one_stats, reference_stats);
    }

    #[test]
    fn pcg_job_checkpoint_resume_is_exact() {
        let n = 24;
        let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
        let csr = graph_laplacian_plus_identity(n, &edges).to_csr();
        let b = VecD::from_vec((0..n).map(|i| (i + 1) as f64).collect());
        let operation = test_operation(102);
        let expected = drive_pcg_job(PcgJob::new(operation, csr.clone(), b.clone(), VecD::zeros(n), 1e-12, 200, 7), operation);
        let mut job = PcgJob::new(operation, csr, b, VecD::zeros(n), 1e-12, 200, 7);
        let mut sequence = 0;
        let checkpoint = loop {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            if let StepOutcome::CheckpointReady(checkpoint) = job.step(&mut context) {
                break checkpoint.state;
            }
        };
        let resumed = PcgJob::from_checkpoint(operation, &checkpoint).expect("pcg checkpoint restores");
        assert_eq!(resumed.checkpoint_bytes(), checkpoint);
        assert_eq!(drive_pcg_job(resumed, operation), expected);
    }

    #[test]
    fn pcg_job_publishes_coarse_preview_before_final_tolerance() {
        let n = 40;
        let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
        let csr = graph_laplacian_plus_identity(n, &edges).to_csr();
        let b = VecD::from_vec((0..n).map(|i| (i + 1) as f64).collect());
        let operation = test_operation(107);
        let mut job = PcgJob::new(operation, csr, b, VecD::zeros(n), 1e-12, 200, 512);
        let mut sequence = 0;
        loop {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            match job.step(&mut context) {
                StepOutcome::PreviewReady(bytes) => {
                    let preview: PcgPreview = serde_json::from_slice(&bytes).expect("pcg preview decodes");
                    assert_eq!(preview.quality, PcgQuality::Coarse);
                    assert!(preview.residual_norm < 1e-3);
                    assert!(preview.residual_norm >= 1e-12);
                    break;
                }
                StepOutcome::Complete(_) => panic!("pcg reached final tolerance before publishing coarse quality"),
                _ => {}
            }
        }
    }

    #[test]
    fn solver_jobs_reject_stale_and_cancelled_steps_without_mutation() {
        let mut coo = Coo::new(8);
        for i in 0..8 {
            coo.add(i, i, 2.0 + i as f64);
        }
        let csr = coo.to_csr();
        let operation = test_operation(103);
        let mut stale = PcgJob::new(operation, csr.clone(), VecD::from_vec(vec![1.0; 8]), VecD::zeros(8), 1e-9, 20, 8);
        let before = stale.checkpoint_bytes();
        let mut sequence = 0;
        let mut context = StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), StepBudget::new(100, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
        assert!(matches!(stale.step(&mut context), StepOutcome::Fault(_)));
        assert_eq!(stale.checkpoint_bytes(), before);

        let mut cancelled = PcgJob::new(operation, csr, VecD::from_vec(vec![1.0; 8]), VecD::zeros(8), 1e-9, 20, 8);
        let before = cancelled.checkpoint_bytes();
        let token = semio_framework_job::root_cancel_token();
        semio_framework_async::block_on(token.cancel());
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(100, u64::MAX), token, || 0, &mut sequence);
        assert_eq!(cancelled.step(&mut context), StepOutcome::Cancelled);
        assert_eq!(cancelled.checkpoint_bytes(), before);
    }

    #[test]
    fn ldlt_job_checkpoint_resume_matches_reference() {
        let n = 30;
        let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
        let matrix = graph_laplacian_plus_identity(n, &edges).to_csc_sym_upper();
        let expected = ldlt_factor(&matrix).expect("reference factors");
        let operation = test_operation(104);
        let mut job = LdltJob::new(operation, matrix, 3);
        let mut sequence = 0;
        let checkpoint = loop {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            match job.step(&mut context) {
                StepOutcome::CheckpointReady(checkpoint) => break checkpoint.state,
                StepOutcome::Yield => {}
                outcome => panic!("unexpected LDLT checkpoint outcome: {outcome:?}"),
            }
        };
        let mut resumed = restore_ldlt(operation, checkpoint).expect("retained LDLT checkpoint restores");
        loop {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(2, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            match resumed.step(&mut context) {
                StepOutcome::Complete(candidate) => {
                    close_payload(candidate.state);
                    close_payload(candidate.output);
                    break;
                }
                StepOutcome::CheckpointReady(checkpoint) => close_payload(checkpoint.state),
                _ => {}
            }
        }
        assert_eq!(resumed.factor(), Some(expected));
        while !InteractiveJob::terminal_is_empty(&job) {
            let _ = InteractiveJob::close_step(&mut job, 1, usize::MAX);
        }
        while !InteractiveJob::terminal_is_empty(&resumed) {
            let _ = InteractiveJob::close_step(&mut resumed, 1, usize::MAX);
        }
    }

    #[test]
    fn p6h_ldlt_microcursor_max_plus_one_cancel_deadline_stale_replay_and_numerical_parity() {
        let n = 18;
        let edges: Vec<(usize, usize)> = (0..n - 1).map(|index| (index, index + 1)).collect();
        let matrix = graph_laplacian_plus_identity(n, &edges).to_csc_sym_upper();
        let reference = ldlt_factor(&matrix).expect("batch reference factor");
        let operation = test_operation(121);
        let drive = |fuel: u64| {
            let mut job = LdltJob::new(operation, matrix.clone(), 1);
            let mut sequence = 0;
            loop {
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(fuel, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
                match job.step(&mut context) {
                    StepOutcome::Complete(mut candidate) => {
                        close_payload(candidate.state);
                        close_payload(candidate.output);
                        let factor = job.factor().expect("completed factor owner");
                        while !InteractiveJob::terminal_is_empty(&job) {
                            let _ = InteractiveJob::close_step(&mut job, 1, usize::MAX);
                        }
                        return factor;
                    }
                    StepOutcome::CheckpointReady(checkpoint) => close_payload(checkpoint.state),
                    StepOutcome::PreviewReady(preview) => close_payload(preview),
                    _ => {}
                }
            }
        };
        assert_eq!(drive(1), reference);
        assert_eq!(drive(2), reference);
        assert_eq!(drive(4), reference);

        let mut zero_fuel = LdltJob::new(operation, matrix.clone(), 1);
        let before = zero_fuel.state.clone();
        let mut sequence = 0;
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(0, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
        assert_eq!(zero_fuel.step(&mut context), StepOutcome::Yield);
        assert!(zero_fuel.state == before);

        let mut deadline = LdltJob::new(operation, matrix.clone(), 1);
        let before = deadline.state.clone();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, 0), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
        assert_eq!(deadline.step(&mut context), StepOutcome::Yield);
        assert!(deadline.state == before);

        let mut stale = LdltJob::new(operation, matrix.clone(), 1);
        let before = stale.state.clone();
        let mut context = StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
        assert!(matches!(stale.step(&mut context), StepOutcome::Fault(_)));
        assert!(stale.state == before);

        let mut cancelled = LdltJob::new(operation, matrix, 1);
        let before = cancelled.state.clone();
        let token = semio_framework_job::root_cancel_token();
        semio_framework_async::block_on(token.cancel());
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), token, || 0, &mut sequence);
        assert_eq!(cancelled.step(&mut context), StepOutcome::Cancelled);
        assert!(cancelled.state == before);

        let mut lookup = LdltJob::new(operation, deadline.state.a.clone(), 1);
        let mut observed_lookup = false;
        for _ in 0..200_000 {
            if lookup.state.cursor.stage == LdltColumnStage::ContributorLookup && lookup.state.cursor.lookup_initialized && lookup.state.cursor.lookup_lower < lookup.state.cursor.lookup_upper {
                let before = lookup.state.clone();
                let mut expired = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, 0), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
                assert_eq!(lookup.step(&mut expired), StepOutcome::Yield);
                assert!(lookup.state == before);
                let mut one = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
                assert_eq!(lookup.step(&mut one), StepOutcome::Yield);
                assert_eq!(lookup.state.cursor.contributor, before.cursor.contributor);
                assert!(lookup.state.cursor.lookup_lower != before.cursor.lookup_lower || lookup.state.cursor.lookup_upper != before.cursor.lookup_upper);
                observed_lookup = true;
                break;
            }
            let mut one = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            if let StepOutcome::CheckpointReady(checkpoint) = lookup.step(&mut one) {
                close_payload(checkpoint.state);
            }
        }
        assert!(observed_lookup, "adversarial LDLT reaches retained contributor comparison");
        while !InteractiveJob::terminal_is_empty(&lookup) {
            let _ = InteractiveJob::close_step(&mut lookup, 1, usize::MAX);
        }

        let refused = CscSym { n: LDLT_MAXIMUM_ORDER + 1, colptr: vec![0; LDLT_MAXIMUM_ORDER + 2], rowind: Vec::new(), vals: Vec::new() };
        let mut maximum_plus_one = LdltJob::new(operation, refused, 1);
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
        assert!(matches!(maximum_plus_one.step(&mut context), StepOutcome::Fault(_)));

        let mut publishing = LdltJob::new(operation, deadline.state.a.clone(), 1);
        for _ in 0..200_000 {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            if let StepOutcome::CheckpointReady(checkpoint) = publishing.step(&mut context) {
                close_payload(checkpoint.state);
            }
            if publishing.output_writer.is_some() {
                break;
            }
        }
        assert!(publishing.output_writer.is_some(), "retained LDLT result writer becomes interruptible before publication");
        InteractiveJob::begin_close(&mut publishing);
        for _ in 0..200_000 {
            if matches!(InteractiveJob::close_step(&mut publishing, 1, usize::MAX), semio_framework_job::InteractiveJobCloseStep::Complete) {
                break;
            }
        }
        assert!(InteractiveJob::terminal_is_empty(&publishing));

        let checkpoint = loop {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            if let StepOutcome::CheckpointReady(checkpoint) = zero_fuel.step(&mut context) {
                break checkpoint.state;
            }
        };
        let mut roundtrip = restore_ldlt(operation, checkpoint).expect("LDLT retained checkpoint roundtrip");
        assert_eq!(roundtrip.state.identity, zero_fuel.state.identity);
        while !InteractiveJob::terminal_is_empty(&roundtrip) {
            let _ = InteractiveJob::close_step(&mut roundtrip, 1, usize::MAX);
        }
        assert!(matches!(restore_ldlt(operation, RetainedJobPayload::empty(JobPayloadStream::CheckpointState)), Err(NumericalCheckpointFault::Truncated)));
        let wrong_revision = Operation::new(operation.operation, semio_framework_job::RevisionId(operation.base_revision.0 + 1), operation.generation, operation.seed);
        let wrong_seed = Operation::new(operation.operation, operation.base_revision, operation.generation, operation.seed + 1);
        let fresh_checkpoint = |id: u64| {
            let mut source = LdltJob::new(operation, graph_laplacian_plus_identity(6, &[(0, 1), (1, 2)]).to_csc_sym_upper(), 1);
            let mut local_sequence = id;
            loop {
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut local_sequence);
                if let StepOutcome::CheckpointReady(checkpoint) = source.step(&mut context) {
                    break checkpoint.state;
                }
            }
        };
        assert!(matches!(restore_ldlt(wrong_revision, fresh_checkpoint(1)), Err(NumericalCheckpointFault::Stale)));
        assert!(matches!(restore_ldlt(wrong_seed, fresh_checkpoint(2)), Err(NumericalCheckpointFault::Stale)));
        let mut interrupted_restore = LdltRestoreCursor::new(operation, fresh_checkpoint(3));
        let mut restore_context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
        assert!(matches!(interrupted_restore.step(&mut restore_context), Ok(None)));
        while !interrupted_restore.terminal_is_empty() {
            match interrupted_restore.close_step(1, usize::MAX) {
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items, .. } => assert!(released_items <= 1),
                semio_framework_job::InteractiveJobCloseStep::Complete => {}
                semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("LDLT restore close cannot block"),
            }
        }

        for owner in [&mut deadline, &mut stale, &mut cancelled, &mut maximum_plus_one] {
            while !InteractiveJob::terminal_is_empty(owner) {
                let _ = InteractiveJob::close_step(owner, 1, usize::MAX);
            }
        }

        let mut closing = zero_fuel;
        let mut close_turns = 0;
        loop {
            close_turns += 1;
            match InteractiveJob::close_step(&mut closing, 1, usize::MAX) {
                semio_framework_job::InteractiveJobCloseStep::Complete => break,
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items, .. } => assert!(released_items <= 1),
                semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("fixed LDLT close cannot block"),
            }
            assert!(close_turns < 20_000);
        }
        assert!(InteractiveJob::terminal_is_empty(&closing));
    }

    #[test]
    fn subspace_job_resume_and_scheduling_are_deterministic() {
        let n = 12;
        let mut k = Coo::new(n);
        let mut b = Coo::new(n);
        for i in 0..n {
            k.add(i, i, (i + 1) as f64);
            b.add(i, i, 1.0);
        }
        let factor = ldlt_factor(&k.to_csc_sym_upper()).expect("factors");
        let mass = b.to_csr();
        let operation = test_operation(105);
        let mut uninterrupted = SubspaceIterationJob::new(operation, factor.clone(), mass.clone(), n, 4, 30);
        let mut sequence = 0;
        let expected = loop {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            match uninterrupted.step(&mut context) {
                StepOutcome::Complete(candidate) => {
                    close_payload(candidate.state);
                    close_payload(candidate.output);
                    break uninterrupted.solution();
                }
                StepOutcome::CheckpointReady(checkpoint) => close_payload(checkpoint.state),
                StepOutcome::PreviewReady(preview) => close_payload(preview),
                _ => {}
            }
        };

        let mut interrupted = SubspaceIterationJob::new(operation, factor, mass, n, 4, 30);
        let checkpoint = loop {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            match interrupted.step(&mut context) {
                StepOutcome::CheckpointReady(checkpoint) => break checkpoint.state,
                StepOutcome::Yield | StepOutcome::PreviewReady(_) => {}
                outcome => panic!("unexpected subspace checkpoint outcome: {outcome:?}"),
            }
        };
        let mut resumed = restore_subspace(operation, checkpoint).expect("subspace retained checkpoint restores");
        while !InteractiveJob::terminal_is_empty(&interrupted) {
            let _ = InteractiveJob::close_step(&mut interrupted, 1, usize::MAX);
        }
        loop {
            let mut yielded = StepContext::new(operation.operation, operation.generation, StepBudget::new(0, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            assert!(matches!(resumed.step(&mut yielded), StepOutcome::Yield));
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            match resumed.step(&mut context) {
                StepOutcome::Complete(candidate) => {
                    close_payload(candidate.state);
                    close_payload(candidate.output);
                    break;
                }
                StepOutcome::CheckpointReady(checkpoint) => close_payload(checkpoint.state),
                StepOutcome::PreviewReady(preview) => close_payload(preview),
                _ => {}
            }
        }
        assert_eq!(resumed.solution(), expected);
        assert_eq!(resumed.preview().converged_count, 4);
        while !InteractiveJob::terminal_is_empty(&uninterrupted) {
            let _ = InteractiveJob::close_step(&mut uninterrupted, 1, usize::MAX);
        }
        while !InteractiveJob::terminal_is_empty(&resumed) {
            let _ = InteractiveJob::close_step(&mut resumed, 1, usize::MAX);
        }
    }

    #[test]
    fn p6h_subspace_cancellation_is_observed_at_every_nested_stage_and_worker_replay_is_exact() {
        let n = 8;
        let mut stiffness = Coo::new(n);
        let mut mass = Coo::new(n);
        for index in 0..n {
            stiffness.add(index, index, (index + 1) as f64);
            mass.add(index, index, 1.0);
        }
        let factor = ldlt_factor(&stiffness.to_csc_sym_upper()).expect("factor");
        let operation = test_operation(122);
        let mass = mass.to_csr();
        let drive = |fuel: u64| {
            let mut replay = SubspaceIterationJob::new(operation, factor.clone(), mass.clone(), n, 3, 3);
            let mut replay_sequence = 0;
            for _ in 0..200_000 {
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(fuel, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut replay_sequence);
                match replay.step(&mut context) {
                    StepOutcome::Complete(candidate) => {
                        close_payload(candidate.state);
                        close_payload(candidate.output);
                        let solution = replay.solution();
                        while !InteractiveJob::terminal_is_empty(&replay) {
                            let _ = InteractiveJob::close_step(&mut replay, 1, usize::MAX);
                        }
                        return solution;
                    }
                    StepOutcome::CheckpointReady(checkpoint) => close_payload(checkpoint.state),
                    StepOutcome::PreviewReady(preview) => close_payload(preview),
                    StepOutcome::Fault(fault) => panic!("subspace replay fault: {:?}", fault.detail),
                    _ => {}
                }
            }
            panic!("subspace replay did not reach a terminal state")
        };
        let single = drive(1);
        assert_eq!(drive(2), single);
        assert_eq!(drive(4), single);

        let mut validating = SubspaceIterationJob::new(operation, factor.clone(), mass.clone(), n, 3, 1);
        let before = validating.state.clone();
        let mut validation_sequence = 0;
        let mut expired = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, 0), semio_framework_job::root_cancel_token(), || 0, &mut validation_sequence);
        assert_eq!(validating.step(&mut expired), StepOutcome::Yield);
        assert!(validating.state == before);
        let mut one = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut validation_sequence);
        assert_eq!(validating.step(&mut one), StepOutcome::Yield);
        assert_eq!(validating.state.factor_validation_cursor, 1, "one construction grant validates one factor owner");
        while !InteractiveJob::terminal_is_empty(&validating) {
            let _ = InteractiveJob::close_step(&mut validating, 1, usize::MAX);
        }

        let mut oversized = Vec::<(u32, f64)>::new();
        oversized.try_reserve_exact(NUMERICAL_OWNER_PAGE_BYTES / std::mem::size_of::<(u32, f64)>() + 1).expect("hostile factor backing");
        assert!(oversized.capacity() * std::mem::size_of::<(u32, f64)>() > NUMERICAL_OWNER_PAGE_BYTES);
        let mut columns = vec![Vec::new(); n];
        columns[0] = oversized;
        let mut refused_owner = SubspaceIterationJob::new(operation, LdltFactor { n, l_cols: columns, d: vec![1.0; n] }, mass.clone(), n, 3, 1);
        let mut refused_sequence = 0;
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut refused_sequence);
        assert!(matches!(refused_owner.step(&mut context), StepOutcome::Fault(_)));
        while !InteractiveJob::terminal_is_empty(&refused_owner) {
            let _ = InteractiveJob::close_step(&mut refused_owner, 1, usize::MAX);
        }

        for refused_order in [0, SUBSPACE_MAXIMUM_ORDER + 1] {
            let factor = LdltFactor { n: refused_order, l_cols: vec![Vec::new(); refused_order], d: vec![1.0; refused_order] };
            let mass = Csr::from_owned_parts(refused_order, vec![0; refused_order + 1], Vec::new(), Vec::new());
            let mut refused = SubspaceIterationJob::new(operation, factor, mass, refused_order, usize::from(refused_order != 0), 1);
            let mut refused_sequence = 0;
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut refused_sequence);
            assert!(matches!(refused.step(&mut context), StepOutcome::Fault(_)));
            while !InteractiveJob::terminal_is_empty(&refused) {
                let _ = InteractiveJob::close_step(&mut refused, 1, usize::MAX);
            }
        }

        let mut publishing = SubspaceIterationJob::new(operation, factor.clone(), mass.clone(), n, 3, 1);
        let mut publishing_sequence = 0;
        for _ in 0..200_000 {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut publishing_sequence);
            if let StepOutcome::CheckpointReady(checkpoint) = publishing.step(&mut context) {
                close_payload(checkpoint.state);
            }
            if publishing.preview_writer.is_some() {
                break;
            }
        }
        assert!(publishing.preview_writer.is_some(), "retained preview page writer becomes interruptible before publication");
        InteractiveJob::begin_close(&mut publishing);
        for _ in 0..200_000 {
            if matches!(InteractiveJob::close_step(&mut publishing, 1, usize::MAX), semio_framework_job::InteractiveJobCloseStep::Complete) {
                break;
            }
        }
        assert!(InteractiveJob::terminal_is_empty(&publishing));

        let mut job = SubspaceIterationJob::new(operation, factor, mass, n, 3, 3);
        let mut seen = std::collections::BTreeSet::new();
        let mut sequence = 0;
        for _ in 0..200_000 {
            if seen.len() == 16 {
                break;
            }
            seen.insert(job.state.work.stage as u8);
            job.state.checkpoint_due = true;
            let checkpoint = loop {
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
                if let StepOutcome::CheckpointReady(checkpoint) = job.step(&mut context) {
                    break checkpoint.state;
                }
            };
            let mut cancelled = restore_subspace(operation, checkpoint).expect("stage retained checkpoint");
            let before = cancelled.state.clone();
            let token = semio_framework_job::root_cancel_token();
            semio_framework_async::block_on(token.cancel());
            let mut cancelled_context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), token, || 0, &mut sequence);
            assert_eq!(cancelled.step(&mut cancelled_context), StepOutcome::Cancelled);
            assert!(cancelled.state == before);
            while !InteractiveJob::terminal_is_empty(&cancelled) {
                let _ = InteractiveJob::close_step(&mut cancelled, 1, usize::MAX);
            }
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            if let StepOutcome::Fault(fault) = job.step(&mut context) {
                panic!("subspace stage walk fault: {:?}", fault.detail);
            }
        }
        assert_eq!(seen.len(), 16);

        assert!(matches!(restore_subspace(operation, RetainedJobPayload::empty(JobPayloadStream::CheckpointState)), Err(NumericalCheckpointFault::Truncated)));
        let wrong_generation = Operation::new(operation.operation, operation.base_revision, semio_framework_job::Generation(operation.generation.0 + 1), operation.seed);
        job.state.checkpoint_due = true;
        let checkpoint = loop {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            if let StepOutcome::CheckpointReady(checkpoint) = job.step(&mut context) {
                break checkpoint.state;
            }
        };
        assert!(matches!(restore_subspace(wrong_generation, checkpoint), Err(NumericalCheckpointFault::Stale)));
        job.state.checkpoint_due = true;
        let checkpoint = loop {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            if let StepOutcome::CheckpointReady(checkpoint) = job.step(&mut context) {
                break checkpoint.state;
            }
        };
        let mut interrupted_restore = SubspaceRestoreCursor::new(operation, checkpoint);
        let mut restore_context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
        assert!(matches!(interrupted_restore.step(&mut restore_context), Ok(None)));
        while !interrupted_restore.terminal_is_empty() {
            match interrupted_restore.close_step(1, usize::MAX) {
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items, .. } => assert!(released_items <= 1),
                semio_framework_job::InteractiveJobCloseStep::Complete => {}
                semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("subspace restore close cannot block"),
            }
        }
        let mut closing = job;
        let mut close_turns = 0;
        loop {
            close_turns += 1;
            match InteractiveJob::close_step(&mut closing, 1, usize::MAX) {
                semio_framework_job::InteractiveJobCloseStep::Complete => break,
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items, .. } => assert!(released_items <= 1),
                semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("fixed subspace close cannot block"),
            }
            assert!(close_turns < 200_000);
        }
        assert!(InteractiveJob::terminal_is_empty(&closing));
    }

    #[test]
    fn adversarial_solver_steps_stay_below_eight_milliseconds() {
        let n = 20_000;
        let mut coo = Coo::new(n);
        for i in 0..n {
            coo.add(i, i, 2.0);
        }
        let operation = test_operation(106);
        let mut pcg = PcgJob::new(operation, coo.to_csr(), VecD::from_vec(vec![1.0; n]), VecD::zeros(n), 1e-9, 20, 1);
        let mut sequence = 0;
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
        let started = std::time::Instant::now();
        let _ = pcg.step(&mut context);
        assert!(started.elapsed() < std::time::Duration::from_millis(8));
    }

    #[test]
    fn pcg_construction_initializes_one_scalar_per_opportunity_and_closes_interruptibly() {
        let matrix = Csr::from_owned_parts(3, vec![0, 1, 2, 3], vec![0, 1, 2], vec![2.0, 3.0, 4.0]);
        let mut construction = PcgJobConstruction::new(test_operation(107), matrix);
        let mut opportunities = 0;
        while !construction.step_one().expect("fixed PCG construction") {
            opportunities += 1;
            assert!(opportunities < 128);
        }
        assert!(opportunities > 18, "six retained vectors cannot be initialized in one constructor turn");
        let job = construction.take_complete().expect("terminal construction transfers once");
        assert_eq!(job.a.n, 3);
        assert!(construction.take_complete().is_none());

        let matrix = Csr::from_owned_parts(3, vec![0, 1, 2, 3], vec![0, 1, 2], vec![2.0, 3.0, 4.0]);
        let mut interrupted = PcgJobConstruction::new(test_operation(108), matrix);
        assert!(!interrupted.step_one().expect("one reservation"));
        let before = interrupted.matrix.as_ref().expect("matrix retained").vals.len();
        let (terminal, _, _) = interrupted.close_step(4_096);
        assert!(!terminal);
        assert_eq!(interrupted.matrix.as_ref().expect("matrix shell retained").vals.len() + 1, before);
    }
}
// #endregion 🔖️Tests
