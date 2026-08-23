//! 🧮️ Sparse linear algebra: COO/CSR/CSC assembly, a left-looking sparse LDLT direct solver, a
//! Jacobi-preconditioned conjugate-gradient iterative solver, a subspace-iteration eigensolver
//! (modal/buckling `Kφ=λBφ`) backed by a dense cyclic-Jacobi eigensolver for its small projected
//! subproblem, and reverse-Cuthill-McKee bandwidth-reduction ordering. No dependency beyond
//! `crate::algebra`'s dense `MatD`/`VecD`, used here as both scratch storage for small
//! projected problems and as the correctness oracle in this module's tests.

use crate::algebra::{MatD, VecD};
use semio_framework_job::{CommitCandidate, InteractiveJob, JobFault, Operation, StepBudget, StepContext, StepOutcome};
use std::collections::{BTreeMap, VecDeque};

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

    pub(crate) fn close_step(&mut self) -> (bool, usize) {
        if self.vals.pop().is_some() {
            return (false, std::mem::size_of::<f64>());
        }
        if self.indices.pop().is_some() || self.indptr.pop().is_some() {
            return (false, std::mem::size_of::<u32>());
        }
        (true, 0)
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
    l_cols: Vec<BTreeMap<u32, f64>>,
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
    let mut l_cols: Vec<BTreeMap<u32, f64>> = vec![BTreeMap::new(); n];
    let mut d = vec![0.0; n];
    let mut row_lists: Vec<Vec<usize>> = vec![Vec::new(); n];

    for j in 0..n {
        ldlt_column(a, &mut l_cols, &mut d, &mut row_lists, j)?;
    }

    Ok(LdltFactor { n, l_cols, d })
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LdltPreview {
    pub completed_columns: usize,
    pub total_columns: usize,
    pub negative_pivots: usize,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct LdltCheckpoint {
    a: CscSym,
    l_cols: Vec<BTreeMap<u32, f64>>,
    d: Vec<f64>,
    row_lists: Vec<Vec<usize>>,
    column: usize,
    columns_per_step: usize,
}

pub struct LdltJob {
    operation: Operation,
    state: LdltCheckpoint,
}

impl LdltJob {
    pub fn new(operation: Operation, a: CscSym, columns_per_step: usize) -> Self {
        assert!(columns_per_step > 0, "ldlt batch must contain work");
        let n = a.n;
        Self { operation, state: LdltCheckpoint { a, l_cols: vec![BTreeMap::new(); n], d: vec![0.0; n], row_lists: vec![Vec::new(); n], column: 0, columns_per_step } }
    }

    pub fn from_checkpoint(operation: Operation, bytes: &[u8]) -> Result<Self, serde_json::Error> {
        Ok(Self { operation, state: serde_json::from_slice(bytes)? })
    }

    pub fn checkpoint_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&self.state).expect("ldlt checkpoint is serializable")
    }

    pub fn preview(&self) -> LdltPreview {
        LdltPreview { completed_columns: self.state.column, total_columns: self.state.a.n, negative_pivots: self.state.d[..self.state.column].iter().filter(|&&value| value < 0.0).count() }
    }

    pub fn factor(&self) -> Option<LdltFactor> {
        (self.state.column == self.state.a.n).then(|| LdltFactor { n: self.state.a.n, l_cols: self.state.l_cols.clone(), d: self.state.d.clone() })
    }
}

impl InteractiveJob for LdltJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return StepOutcome::Fault(JobFault { detail: b"stale-fem-ldlt-operation".to_vec() });
        }
        context.set_stage("fem.ldlt.column");
        let stop = (self.state.column + self.state.columns_per_step).min(self.state.a.n);
        while self.state.column < stop && !context.should_yield() {
            let column = self.state.column;
            if let Err(SparseError::ZeroPivot { column }) = ldlt_column(&self.state.a, &mut self.state.l_cols, &mut self.state.d, &mut self.state.row_lists, column) {
                return StepOutcome::Fault(JobFault { detail: format!("zero-pivot:{column}").into_bytes() });
            }
            self.state.column += 1;
            context.consume_fuel(1);
            if context.is_cancelled() {
                return StepOutcome::Cancelled;
            }
        }
        if self.state.column == self.state.a.n {
            let factor = self.factor().expect("complete ldlt has a factor");
            return StepOutcome::Complete(CommitCandidate { state: self.checkpoint_bytes(), output: serde_json::to_vec(&factor).expect("ldlt output is serializable") });
        }
        StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: self.checkpoint_bytes(), applied_progress: self.state.column as u64 })
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
            for (&row, &lij) in self.l_cols[j].iter() {
                y[row as usize] -= lij * yj;
            }
        }
        for (j, value) in y.iter_mut().enumerate().take(n) {
            *value /= self.d[j];
        }
        for j in (0..n).rev() {
            let mut sum = y[j];
            for (&row, &lij) in self.l_cols[j].iter() {
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

    fn multiply(&self, x: &VecD) -> VecD {
        let mut upper = x.0.clone();
        for (column, entries) in self.l_cols.iter().enumerate() {
            for (&row, &value) in entries {
                upper[column] += value * x.get(row as usize);
            }
        }
        for (index, value) in upper.iter_mut().enumerate() {
            *value *= self.d[index];
        }
        let mut output = upper.clone();
        for (column, entries) in self.l_cols.iter().enumerate() {
            for (&row, &value) in entries {
                output[row as usize] += value * upper[column];
            }
        }
        VecD::from_vec(output)
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

    /// 🧹️ Retires one matrix/vector scalar owner per governed close opportunity.
    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if maximum_bytes < std::mem::size_of::<f64>().max(std::mem::size_of::<u32>()) {
            return (false, 0, 0);
        }
        loop {
            let released_bytes = match self.close_lane {
                0 => match self.state.a.vals.pop() {
                    Some(_) => std::mem::size_of::<f64>(),
                    None => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                1 => match self.state.a.indices.pop() {
                    Some(_) => std::mem::size_of::<u32>(),
                    None => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                2 => match self.state.a.indptr.pop() {
                    Some(_) => std::mem::size_of::<u32>(),
                    None => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                3 => match self.state.b.0.pop() {
                    Some(_) => std::mem::size_of::<f64>(),
                    None => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                4 => match self.state.x.0.pop() {
                    Some(_) => std::mem::size_of::<f64>(),
                    None => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                5 => match self.state.diag.0.pop() {
                    Some(_) => std::mem::size_of::<f64>(),
                    None => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                6 => match self.state.r.0.pop() {
                    Some(_) => std::mem::size_of::<f64>(),
                    None => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                7 => match self.state.z.0.pop() {
                    Some(_) => std::mem::size_of::<f64>(),
                    None => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                8 => match self.state.p.0.pop() {
                    Some(_) => std::mem::size_of::<f64>(),
                    None => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                9 => match self.state.ap.0.pop() {
                    Some(_) => std::mem::size_of::<f64>(),
                    None => {
                        self.close_lane += 1;
                        continue;
                    }
                },
                _ => return (true, 0, 0),
            };
            return (false, 1, released_bytes);
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
enum PcgConstructionStage {
    ReserveB,
    InitializeB,
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
            x: VecD::from_vec(Vec::new()),
            diag: VecD::from_vec(Vec::new()),
            r: VecD::from_vec(Vec::new()),
            z: VecD::from_vec(Vec::new()),
            p: VecD::from_vec(Vec::new()),
            ap: VecD::from_vec(Vec::new()),
            complete: None,
        }
    }

    pub fn step_one(&mut self) -> Result<bool, &'static [u8]> {
        let n = self.matrix.as_ref().ok_or(b"pcg-construction-matrix-missing" as &'static [u8])?.n;
        macro_rules! reserve {
            ($owner:expr, $next:expr, $fault:expr) => {{
                $owner.0.try_reserve_exact(n).map_err(|_| $fault as &'static [u8])?;
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
            PcgConstructionStage::InitializeB => initialize!(self.b, 1.0, PcgConstructionStage::ReserveX),
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

    pub fn close_step(&mut self) -> (bool, usize) {
        if let Some(matrix) = self.matrix.as_mut() {
            if matrix.vals.pop().is_some() {
                return (false, std::mem::size_of::<f64>());
            }
            if matrix.indices.pop().is_some() || matrix.indptr.pop().is_some() {
                return (false, std::mem::size_of::<u32>());
            }
            self.matrix = None;
            return (false, std::mem::size_of::<Csr>());
        }
        for vector in [&mut self.b, &mut self.x, &mut self.diag, &mut self.r, &mut self.z, &mut self.p, &mut self.ap] {
            if vector.0.pop().is_some() {
                return (false, std::mem::size_of::<f64>());
            }
        }
        (self.complete.is_none(), 0)
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

fn set_col(m: &mut MatD, col: usize, v: &VecD) {
    for row in 0..m.rows {
        m.set(row, col, v.get(row));
    }
}

fn apply_b(b: &Csr, m: &MatD) -> MatD {
    let mut out = MatD::zeros(m.rows, m.cols);
    for col in 0..m.cols {
        let bv = b.mul_vec(&mat_col(m, col));
        set_col(&mut out, col, &bv);
    }
    out
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct SubspaceCheckpoint {
    k_factor: LdltFactor,
    b: Csr,
    n: usize,
    p: usize,
    max_iter: usize,
    m: usize,
    x: MatD,
    prev_theta: Vec<f64>,
    final_theta: Vec<f64>,
    final_x: MatD,
    iteration: usize,
    residuals: Vec<f64>,
    converged_count: usize,
    converged: bool,
    checkpoint_due: bool,
}

pub struct SubspaceIterationJob {
    operation: Operation,
    state: SubspaceCheckpoint,
}

impl SubspaceIterationJob {
    pub fn new(operation: Operation, k_factor: LdltFactor, b: Csr, n: usize, p: usize, max_iter: usize) -> Self {
        assert!(n > 0, "subspace iteration needs at least one equation");
        assert!(p > 0 && p <= n, "subspace mode count must be within the equation count");
        assert_eq!(b.n, n, "subspace mass matrix dimension mismatch");
        assert_eq!(k_factor.n, n, "subspace stiffness factor dimension mismatch");
        let m = (p + 8).max(2 * p).min(n).max(1);
        let mut x = MatD::zeros(n, m);
        for j in 0..m {
            x.set(j, j, 1.0);
            if j + 1 < n {
                x.add_at(j + 1, j, 0.3);
            }
            if j >= 1 {
                x.add_at(j - 1, j, 0.3);
            }
        }
        Self {
            operation,
            state: SubspaceCheckpoint {
                k_factor,
                b,
                n,
                p,
                max_iter,
                m,
                final_x: x.clone(),
                x,
                prev_theta: vec![f64::MAX; p],
                final_theta: Vec::new(),
                iteration: 0,
                residuals: vec![f64::MAX; p],
                converged_count: 0,
                converged: false,
                checkpoint_due: false,
            },
        }
    }

    pub fn from_checkpoint(operation: Operation, bytes: &[u8]) -> Result<Self, serde_json::Error> {
        Ok(Self { operation, state: serde_json::from_slice(bytes)? })
    }

    pub fn checkpoint_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&self.state).expect("subspace checkpoint is serializable")
    }

    pub fn preview(&self) -> SubspacePreview {
        SubspacePreview {
            iteration: self.state.iteration,
            eigenvalues: self.state.final_theta.iter().take(self.state.p).copied().collect(),
            mode_shapes: (0..self.state.p.min(self.state.final_x.cols)).map(|column| mat_col(&self.state.final_x, column).0).collect(),
            residuals: self.state.residuals.clone(),
            converged_count: self.state.converged_count,
            converged: self.state.converged,
        }
    }

    pub fn solution(&self) -> EigenPairs {
        EigenPairs { values: self.state.final_theta.iter().take(self.state.p).copied().collect(), vectors: (0..self.state.p.min(self.state.final_x.cols)).map(|column| mat_col(&self.state.final_x, column)).collect() }
    }

    fn iterate(&mut self) {
        let rhs = apply_b(&self.state.b, &self.state.x);
        let y = self.state.k_factor.solve_many(&rhs);
        let mut cols: Vec<VecD> = (0..self.state.m).map(|j| mat_col(&y, j)).collect();
        let mut kcols: Vec<VecD> = (0..self.state.m).map(|j| mat_col(&rhs, j)).collect();
        for j in 0..self.state.m {
            for k in 0..j {
                let coeff = cols[j].dot(&kcols[k]);
                cols[j] = cols[j].sub(&cols[k].scale(coeff));
                kcols[j] = kcols[j].sub(&kcols[k].scale(coeff));
            }
            let mut norm_sq = cols[j].dot(&kcols[j]);
            if !norm_sq.is_finite() || norm_sq <= 1e-24 {
                cols[j] = mat_col(&self.state.x, j);
                kcols[j] = self.state.k_factor.multiply(&cols[j]);
                for k in 0..j {
                    let coeff = cols[j].dot(&kcols[k]);
                    cols[j] = cols[j].sub(&cols[k].scale(coeff));
                    kcols[j] = kcols[j].sub(&kcols[k].scale(coeff));
                }
                norm_sq = cols[j].dot(&kcols[j]);
            }
            let norm = norm_sq.max(1e-300).sqrt();
            cols[j] = cols[j].scale(1.0 / norm);
            kcols[j] = kcols[j].scale(1.0 / norm);
        }
        let mut basis = MatD::zeros(self.state.n, self.state.m);
        for j in 0..self.state.m {
            set_col(&mut basis, j, &cols[j]);
        }

        let projected = symmetrize(&basis.transpose().matmul(&apply_b(&self.state.b, &basis)));
        let (mu, vectors) = dense_symmetric_eigen_jacobi(&projected);
        let mut order: Vec<usize> = (0..mu.len()).collect();
        order.sort_by(|&left, &right| {
            let left_lambda = if mu[left] > 1e-14 { mu[left].recip() } else { f64::MAX };
            let right_lambda = if mu[right] > 1e-14 { mu[right].recip() } else { f64::MAX };
            left_lambda.total_cmp(&right_lambda).then_with(|| left.cmp(&right))
        });
        let theta: Vec<f64> = order.iter().map(|&index| if mu[index] > 1e-14 { mu[index].recip() } else { f64::MAX }).collect();
        let mut ordered_vectors = MatD::zeros(vectors.rows, vectors.cols);
        for (column, &source) in order.iter().enumerate() {
            for row in 0..vectors.rows {
                ordered_vectors.set(row, column, vectors.get(row, source));
            }
        }
        let x_new = basis.matmul(&ordered_vectors);

        let current: Vec<f64> = theta.iter().take(self.state.p).copied().collect();
        self.state.residuals = current.iter().zip(&self.state.prev_theta).map(|(&value, &previous)| if previous < f64::MAX { ((value - previous) / previous.abs().max(1e-12)).abs() } else { f64::MAX }).collect();
        self.state.converged_count = self.state.residuals.iter().filter(|&&residual| residual < 1e-6).count();
        self.state.converged = self.state.converged_count == self.state.p;
        self.state.prev_theta = current;
        self.state.final_theta = theta;
        self.state.final_x = x_new.clone();
        self.state.x = x_new;
        self.state.iteration += 1;
        self.state.checkpoint_due = true;
    }
}

impl InteractiveJob for SubspaceIterationJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return StepOutcome::Fault(JobFault { detail: b"stale-fem-subspace-operation".to_vec() });
        }
        if self.state.checkpoint_due {
            self.state.checkpoint_due = false;
            return StepOutcome::CheckpointReady(semio_framework_job::Checkpoint { state: self.checkpoint_bytes(), applied_progress: self.state.iteration as u64 });
        }
        if self.state.iteration >= self.state.max_iter || self.state.converged {
            return StepOutcome::Complete(CommitCandidate { state: self.checkpoint_bytes(), output: serde_json::to_vec(&self.preview()).expect("subspace output is serializable") });
        }
        context.set_stage("fem.subspace.iteration");
        if context.should_yield() {
            return StepOutcome::Yield;
        }
        self.iterate();
        context.consume_fuel(1);
        if context.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        StepOutcome::PreviewReady(serde_json::to_vec(&self.preview()).expect("subspace preview is serializable"))
    }
}

/// 🎯️ Finds the lowest `p` eigenpairs of `K x = λ B x`, given `K`'s LDLT factorization and `B` as a
/// `Csr` (mass matrix, or `−Kg` for buckling — sign convention is the caller's responsibility).
/// Bathe-style subspace iteration: B-orthonormalize the current subspace via modified Gram-Schmidt,
/// solve `K Y = B X`, project both `K` and `B` onto `Y` (using `Yᵀ K Y = Yᵀ (K Y) = Yᵀ (B X)` so the
/// raw `K` operator is never needed — only its factorization), solve the small dense generalized
/// eigenproblem via a Cholesky-of-`B_proj` transform to a standard eigenproblem, rotate the subspace
/// by the recovered eigenvectors, and repeat until the lowest `p` eigenvalues stop changing.
pub fn subspace_iteration(k_factor: &LdltFactor, b: &Csr, n: usize, p: usize, max_iter: usize) -> EigenPairs {
    let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), 0);
    let mut job = SubspaceIterationJob::new(operation, k_factor.clone(), b.clone(), n, p, max_iter);
    let mut preview_sequence = 0;
    loop {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut preview_sequence);
        if matches!(job.step(&mut context), StepOutcome::Complete(_)) {
            return job.solution();
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
            if let StepOutcome::CheckpointReady(checkpoint) = job.step(&mut context) {
                break checkpoint.state;
            }
        };
        let mut resumed = LdltJob::from_checkpoint(operation, &checkpoint).expect("ldlt checkpoint restores");
        loop {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(2, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            if matches!(resumed.step(&mut context), StepOutcome::Complete(_)) {
                break;
            }
        }
        assert_eq!(resumed.factor(), Some(expected));
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
            if matches!(uninterrupted.step(&mut context), StepOutcome::Complete(_)) {
                break uninterrupted.solution();
            }
        };

        let mut interrupted = SubspaceIterationJob::new(operation, factor, mass, n, 4, 30);
        let checkpoint = loop {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            if let StepOutcome::CheckpointReady(checkpoint) = interrupted.step(&mut context) {
                break checkpoint.state;
            }
        };
        let mut resumed = SubspaceIterationJob::from_checkpoint(operation, &checkpoint).expect("subspace checkpoint restores");
        assert_eq!(resumed.checkpoint_bytes(), checkpoint);
        loop {
            let mut yielded = StepContext::new(operation.operation, operation.generation, StepBudget::new(0, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            assert!(matches!(resumed.step(&mut yielded), StepOutcome::Yield | StepOutcome::CheckpointReady(_)));
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || 0, &mut sequence);
            if matches!(resumed.step(&mut context), StepOutcome::Complete(_)) {
                break;
            }
        }
        assert_eq!(resumed.solution(), expected);
        assert_eq!(resumed.preview().converged_count, 4);
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
}
// #endregion 🔖️Tests
