//! 🧮️ Sparse linear algebra: COO/CSR/CSC assembly, a left-looking sparse LDLT direct solver, a
//! Jacobi-preconditioned conjugate-gradient iterative solver, a subspace-iteration eigensolver
//! (modal/buckling `Kφ=λBφ`) backed by a dense cyclic-Jacobi eigensolver for its small projected
//! subproblem, and reverse-Cuthill-McKee bandwidth-reduction ordering. No dependency beyond
//! `crate::algebra`'s dense `MatD`/`VecD`, used here as both scratch storage for small
//! projected problems and as the correctness oracle in this module's tests.

use crate::algebra::{MatD, VecD};
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
pub struct Csr {
    pub n: usize,
    indptr: Vec<u32>,
    indices: Vec<u32>,
    vals: Vec<f64>,
}

impl Csr {
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
#[derive(Debug)]
pub enum SparseError {
    ZeroPivot { column: usize },
    DimensionMismatch,
}

/// 🧊️ A sparse left-looking LDLT factorization (unit lower `L`, diagonal `D`), permutation-agnostic
/// — a caller applying `rcm_order` reorders the matrix/RHS/solution indices itself before/after
/// calling into this module.
#[derive(Debug)]
pub struct LdltFactor {
    n: usize,
    l_cols: Vec<BTreeMap<u32, f64>>,
    d: Vec<f64>,
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
            for (&row_u32, &lik) in l_cols[k].iter() {
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

        for (&row, &value) in accum.iter() {
            if row > j && value != 0.0 {
                l_cols[j].insert(row as u32, value / djj);
                row_lists[row].push(j);
            }
        }
    }

    Ok(LdltFactor { n, l_cols, d })
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

    /// 🔢️ Count of `D[j] < 0` — a Sturm-sequence inertia count, used later for eigenvalue-count checks.
    pub fn negative_pivot_count(&self) -> usize {
        self.d.iter().filter(|&&value| value < 0.0).count()
    }
}
// #endregion 🔖️Ldlt

// #region 🔖️Pcg
/// 📈️ Convergence outcome of a `pcg` call.
#[derive(Debug, Clone, Copy)]
pub struct PcgStats {
    pub iterations: usize,
    pub residual_norm: f64,
    pub converged: bool,
}

/// ➰️ Jacobi-preconditioned conjugate gradient — mutates `x0` in place, converges when
/// `‖r‖ / ‖b‖ < tol_rel` or `max_iter` is reached.
pub fn pcg(a: &Csr, b: &VecD, x0: &mut VecD, tol_rel: f64, max_iter: usize) -> PcgStats {
    let n = a.n;
    let diag = a.diag();
    let precondition = |r: &VecD| -> VecD {
        let mut z = VecD::zeros(n);
        for i in 0..n {
            let d = diag.get(i);
            z.set(i, if d.abs() > 1e-300 { r.get(i) / d } else { r.get(i) });
        }
        z
    };

    let b_norm = b.norm2().max(1e-300);
    let mut r = b.sub(&a.mul_vec(x0));
    let mut residual_norm = r.norm2() / b_norm;
    if residual_norm < tol_rel {
        return PcgStats { iterations: 0, residual_norm, converged: true };
    }

    let mut z = precondition(&r);
    let mut p = z.clone();
    let mut rz_old = r.dot(&z);
    let mut iterations = 0;

    for iter in 0..max_iter {
        iterations = iter + 1;
        let ap = a.mul_vec(&p);
        let pap = p.dot(&ap);
        if pap.abs() < 1e-300 {
            break;
        }
        let alpha = rz_old / pap;
        for i in 0..n {
            x0.set(i, x0.get(i) + alpha * p.get(i));
        }
        r = r.sub(&ap.scale(alpha));
        residual_norm = r.norm2() / b_norm;
        if residual_norm < tol_rel {
            return PcgStats { iterations, residual_norm, converged: true };
        }
        z = precondition(&r);
        let rz_new = r.dot(&z);
        let beta = rz_new / rz_old;
        p = z.add(&p.scale(beta));
        rz_old = rz_new;
    }

    PcgStats { iterations, residual_norm, converged: false }
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
    order.sort_by(|&a_idx, &b_idx| raw_vals[a_idx].partial_cmp(&raw_vals[b_idx]).unwrap());
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

/// 🪜️ Lower-triangular Cholesky `A = L Lᵀ` of a small dense SPD matrix (Cholesky-Banachiewicz).
fn cholesky_lower(a: &MatD) -> MatD {
    let n = a.rows;
    let mut l = MatD::zeros(n, n);
    for i in 0..n {
        for j in 0..=i {
            let mut sum = a.get(i, j);
            for k in 0..j {
                sum -= l.get(i, k) * l.get(j, k);
            }
            if i == j {
                l.set(i, j, sum.max(1e-300).sqrt());
            } else {
                l.set(i, j, sum / l.get(j, j));
            }
        }
    }
    l
}

/// 🔁️ Inverse of a lower-triangular matrix via forward substitution, one identity column at a time.
fn invert_lower_triangular(l: &MatD) -> MatD {
    let n = l.rows;
    let mut inv = MatD::zeros(n, n);
    for col in 0..n {
        let mut x = vec![0.0; n];
        for i in 0..n {
            let mut sum = if i == col { 1.0 } else { 0.0 };
            for k in 0..i {
                sum -= l.get(i, k) * x[k];
            }
            x[i] = sum / l.get(i, i);
        }
        for i in 0..n {
            inv.set(i, col, x[i]);
        }
    }
    inv
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
pub struct EigenPairs {
    pub values: Vec<f64>,
    pub vectors: Vec<VecD>,
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

/// 🎯️ Finds the lowest `p` eigenpairs of `K x = λ B x`, given `K`'s LDLT factorization and `B` as a
/// `Csr` (mass matrix, or `−Kg` for buckling — sign convention is the caller's responsibility).
/// Bathe-style subspace iteration: B-orthonormalize the current subspace via modified Gram-Schmidt,
/// solve `K Y = B X`, project both `K` and `B` onto `Y` (using `Yᵀ K Y = Yᵀ (K Y) = Yᵀ (B X)` so the
/// raw `K` operator is never needed — only its factorization), solve the small dense generalized
/// eigenproblem via a Cholesky-of-`B_proj` transform to a standard eigenproblem, rotate the subspace
/// by the recovered eigenvectors, and repeat until the lowest `p` eigenvalues stop changing.
pub fn subspace_iteration(k_factor: &LdltFactor, b: &Csr, n: usize, p: usize, max_iter: usize) -> EigenPairs {
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

    let mut prev_theta: Vec<f64> = vec![f64::INFINITY; p];
    let mut final_theta: Vec<f64> = Vec::new();
    let mut final_x = x.clone();

    for _iter in 0..max_iter {
        let bx = apply_b(b, &x);
        let mut cols: Vec<VecD> = (0..m).map(|j| mat_col(&x, j)).collect();
        let mut bcols: Vec<VecD> = (0..m).map(|j| mat_col(&bx, j)).collect();
        for j in 0..m {
            for k in 0..j {
                let coeff = cols[j].dot(&bcols[k]);
                cols[j] = cols[j].sub(&cols[k].scale(coeff));
                bcols[j] = bcols[j].sub(&bcols[k].scale(coeff));
            }
            let norm = cols[j].dot(&bcols[j]).max(1e-300).sqrt();
            cols[j] = cols[j].scale(1.0 / norm);
            bcols[j] = bcols[j].scale(1.0 / norm);
        }
        for j in 0..m {
            set_col(&mut x, j, &cols[j]);
        }

        let rhs = apply_b(b, &x);
        let y = k_factor.solve_many(&rhs);

        let k_proj = symmetrize(&y.transpose().matmul(&rhs));
        let by = apply_b(b, &y);
        let b_proj = symmetrize(&y.transpose().matmul(&by));

        let l = cholesky_lower(&b_proj);
        let l_inv = invert_lower_triangular(&l);
        let a_hat = symmetrize(&l_inv.matmul(&k_proj).matmul(&l_inv.transpose()));
        let (theta, w) = dense_symmetric_eigen_jacobi(&a_hat);
        let z = l_inv.transpose().matmul(&w);

        let x_new = y.matmul(&z);

        let current_p: Vec<f64> = theta.iter().take(p).cloned().collect();
        let converged = current_p.iter().zip(prev_theta.iter()).all(|(&cur, &prev)| if prev.is_infinite() { false } else { ((cur - prev) / prev.abs().max(1e-12)).abs() < 1e-6 });

        prev_theta = current_p;
        final_theta = theta;
        final_x = x_new.clone();
        x = x_new;

        if converged {
            break;
        }
    }

    let values: Vec<f64> = final_theta.into_iter().take(p).collect();
    let vectors: Vec<VecD> = (0..p.min(final_x.cols)).map(|j| mat_col(&final_x, j)).collect();
    EigenPairs { values, vectors }
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
}
// #endregion 🔖️Tests
