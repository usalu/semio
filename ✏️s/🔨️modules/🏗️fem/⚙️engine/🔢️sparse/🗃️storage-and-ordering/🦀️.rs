//! 🗃️ COO, CSR and symmetric CSC storage and the reverse-Cuthill-McKee ordering.

use super::*;

// #region 🔖️Coo
/// 🧱️ Triplet (row, col, value) accumulator for FEM-style assembly — duplicate `(row, col)`
/// entries are summed lazily by whichever `to_*` conversion reads them.
pub struct Coo {
    pub n: usize,
    pub(super) rows: Vec<u32>,
    pub(super) cols: Vec<u32>,
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
