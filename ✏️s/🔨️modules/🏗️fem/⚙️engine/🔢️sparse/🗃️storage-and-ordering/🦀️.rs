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
        CscSym::from_owned_parts(n, colptr, rowind, vals)
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

    /// 📦️ Queries the actual retained physical backing without traversing matrix values.
    pub fn allocated_bytes(&self) -> usize {
        self.indptr.allocated_bytes() + self.indices.allocated_bytes() + self.vals.allocated_bytes()
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
    pub colptr: PagedList<u32, SPARSE_INDEX_SPACE>,
    pub rowind: PagedList<u32, SPARSE_INDEX_SPACE>,
    pub vals: PagedList<f64, SPARSE_INDEX_SPACE>,
}

impl CscSym {
    /// 📚️ Cold callers consume contiguous input once; mounted producers transfer paged owners directly.
    pub fn from_owned_parts(n: usize, colptr: Vec<u32>, rowind: Vec<u32>, vals: Vec<f64>) -> Self {
        Self { n, colptr: paged_from_values(colptr), rowind: paged_from_values(rowind), vals: paged_from_values(vals) }
    }
    pub fn from_paged_parts(n: usize, colptr: PagedList<u32, SPARSE_INDEX_SPACE>, rowind: PagedList<u32, SPARSE_INDEX_SPACE>, vals: PagedList<f64, SPARSE_INDEX_SPACE>) -> Self { Self { n, colptr, rowind, vals } }
    pub fn terminal_is_empty(&self) -> bool { self.colptr.terminal_is_empty() && self.rowind.terminal_is_empty() && self.vals.terminal_is_empty() }
    pub fn allocated_bytes(&self) -> usize { self.colptr.allocated_bytes() + self.rowind.allocated_bytes() + self.vals.allocated_bytes() }

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
        match close_paged_owner_step(&mut self.vals, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
        for owner in [&mut self.rowind, &mut self.colptr] {
            match close_paged_owner_step(owner, maximum_bytes) {
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

pub fn restore_paged_usize_entry<const N: usize>(owner: &mut PagedList<usize, N>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor, context: &mut StepContext<'_>) -> Result<bool, NumericalCheckpointFault> { restore_numerical_owner(owner, page, maximum, entry, cursor, false, context) }
pub fn restore_paged_pair_entry<const N: usize>(owner: &mut PagedList<(u32, f64), N>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor, context: &mut StepContext<'_>) -> Result<bool, NumericalCheckpointFault> { restore_numerical_owner(owner, page, maximum, entry, cursor, false, context) }
pub fn restore_paged_u64_entry<const N: usize>(owner: &mut PagedList<u64, N>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor, context: &mut StepContext<'_>) -> Result<bool, NumericalCheckpointFault> { restore_numerical_owner(owner, page, maximum, entry, cursor, false, context) }
pub fn restore_contributor_entry(owner: &mut PagedList<RowContributor, SPARSE_INDEX_SPACE>, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor, context: &mut StepContext<'_>) -> Result<bool, NumericalCheckpointFault> { restore_numerical_owner(owner, page, maximum, entry, cursor, false, context) }


// #endregion 🔖️Ldlt

// #region 🔖️Pcg


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PencilConstructionStage {
    ValidateMass,
    ReserveColptr,
    CopyUpper,
    ReserveMassIndptr,
    ReserveMassIndices,
    ReserveMassValues,
    CopyMountedMass,
    RetireMountedMass,
    BuildMass,
    Complete,
}

/// 🎼️ Original paged symmetric pencils support physical modal and signed geometric analyses.
pub struct SymmetricPencilConstruction {
    pub(super) matrix: Option<Csr>,
    pub(super) mass: Option<VecD>,
    mounted_mass: Option<MountedScalarSlots>,
    second: Option<Csr>,
    stage: PencilConstructionStage,
    row: usize,
    entry: usize,
    colptr: PagedList<u32, SPARSE_INDEX_SPACE>,
    rowind: PagedList<u32, SPARSE_INDEX_SPACE>,
    values: PagedList<f64, SPARSE_INDEX_SPACE>,
    mass_indptr: PagedList<u32, SPARSE_INDEX_SPACE>,
    mass_indices: PagedList<u32, SPARSE_INDEX_SPACE>,
    mass_values: PagedList<f64, SPARSE_INDEX_SPACE>,
    pub(super) complete: Option<(CscSym, Csr)>,
}

impl SymmetricPencilConstruction {
    /// 📏️ Lend the completed order while retaining both original matrices.
    pub fn completed_order(&self) -> Option<usize> { self.complete.as_ref().map(|(matrix, _)| matrix.n) }

    pub fn new(matrix: Csr, mass: VecD) -> Self {
        Self { matrix: Some(matrix), mass: Some(mass), mounted_mass: None, second: None, stage: PencilConstructionStage::ValidateMass, row: 0, entry: 0, colptr: PagedList::empty(), rowind: PagedList::empty(), values: PagedList::empty(), mass_indptr: PagedList::empty(), mass_indices: PagedList::empty(), mass_values: PagedList::empty(), complete: None }
    }

    pub fn new_mounted(matrix: Csr, mass: MountedScalarSlots) -> Self { let mut owner = Self::new(matrix, VecD::from_vec(Vec::new())); owner.mass = None; owner.mounted_mass = Some(mass); owner }

    /// 🎼️ Retain an original symmetric pencil for modal or signed geometric analysis.
    pub fn new_pair(matrix: Csr, second: Csr) -> Self { let mut owner = Self::new(matrix, VecD::from_vec(Vec::new())); owner.mass = None; owner.second = Some(second); owner }

    fn reserve<T>(owner: &mut PagedList<T, SPARSE_INDEX_SPACE>, count: usize, context: &mut StepContext<'_>) -> Result<bool, &'static [u8]> {
        if owner.capacity() >= count { return Ok(true); }
        let progress = owner.reserve_capacity_one_funded(count, context.retained_grant()).map_err(|_| b"pencil-capacity" as &'static [u8])?;
        context.consume_retained(progress).map_err(|_| b"pencil-capacity-receipt" as &'static [u8])?;
        Ok(false)
    }
    fn admit(context: &mut StepContext<'_>, depth: usize, bytes: usize) -> Result<bool, &'static [u8]> {
        let demand = RetirementDemand { copy_bytes: size_of::<Self>() * 2 + bytes, depth: depth.max(1), ..Default::default() };
        if !numerical_grant_covers(context.retained_grant(), demand) { return Ok(false); }
        context.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }).map_err(|_| b"pencil-copy" as &'static [u8])?;
        Ok(true)
    }
    fn depth<T>(owner: &PagedList<T, SPARSE_INDEX_SPACE>, index: usize) -> Result<usize, &'static [u8]> { owner.next_get_depth_demand(index).map_err(|_| b"pencil-path" as &'static [u8]) }
    fn push_depth<T>(owner: &PagedList<T, SPARSE_INDEX_SPACE>) -> Result<usize, &'static [u8]> { owner.next_push_depth_demand().map_err(|_| b"pencil-path" as &'static [u8]) }

    pub fn step_one(&mut self, context: &mut StepContext<'_>) -> Result<bool, &'static [u8]> {
        if context.is_cancelled() || context.should_yield() { return Ok(false); }
        let matrix = self.matrix.as_ref().ok_or(b"pencil-matrix-owner" as &'static [u8])?;
        let n = matrix.n;
        match self.stage {
            PencilConstructionStage::ValidateMass => {
                let length = self.second.as_ref().map(|second| second.n).or_else(|| self.mass.as_ref().map(VecD::len)).or_else(|| self.mounted_mass.as_ref().map(MountedScalarSlots::len)).ok_or(b"pencil-second-owner" as &'static [u8])?;
                if length != n || n > u32::MAX as usize { return Err(b"pencil-order"); }
                if !Self::admit(context, 1, 16)? { return Ok(false); }
                if self.second.is_some() || self.row == n { self.row = 0; self.entry = 0; self.stage = PencilConstructionStage::ReserveColptr; }
                else {
                    let value = self.mass.as_ref().map(|mass| mass.get(self.row)).or_else(|| self.mounted_mass.as_ref().and_then(|mass| mass.get(self.row))).ok_or(b"pencil-mass-owner" as &'static [u8])?;
                    if !value.is_finite() || value <= 0.0 { return Err(b"pencil-mass-value"); }
                    self.row += 1;
                }
            }
            PencilConstructionStage::ReserveColptr => {
                if !Self::reserve(&mut self.colptr, n.saturating_add(1), context)? { return Ok(false); }
                if !Self::admit(context, Self::push_depth(&self.colptr)?, 4)? { return Ok(false); }
                self.colptr.push_reserved(0).map_err(|_| b"pencil-offset" as &'static [u8])?;
                self.stage = PencilConstructionStage::CopyUpper;
            }
            PencilConstructionStage::CopyUpper => {
                if self.row == n {
                    if !Self::admit(context, 1, 1)? { return Ok(false); }
                    self.stage = PencilConstructionStage::ReserveMassIndptr;
                } else {
                    let end = matrix.indptr(self.row + 1).ok_or(b"pencil-row-end" as &'static [u8])? as usize;
                    if end > matrix.vals.len() || self.entry > end { return Err(b"pencil-row-shape"); }
                    if self.entry < end {
                        let column = matrix.index(self.entry).ok_or(b"pencil-column" as &'static [u8])? as usize;
                        let value = matrix.value(self.entry).ok_or(b"pencil-value" as &'static [u8])?;
                        if column >= n || !value.is_finite() { return Err(b"pencil-entry"); }
                        if column >= self.row {
                            let target = self.rowind.len() + 1;
                            if target > u32::MAX as usize { return Err(b"pencil-offset-overflow"); }
                            if !Self::reserve(&mut self.rowind, target, context)? || !Self::reserve(&mut self.values, target, context)? { return Ok(false); }
                        }
                        let mut depth = Self::depth(&matrix.indptr, self.row + 1)? + Self::depth(&matrix.indices, self.entry)? + Self::depth(&matrix.vals, self.entry)?;
                        if column >= self.row { depth += Self::push_depth(&self.rowind)? + Self::push_depth(&self.values)?; }
                        if !Self::admit(context, depth, 32)? { return Ok(false); }
                        if column >= self.row { self.rowind.push_reserved(column as u32).map_err(|_| b"pencil-row" as &'static [u8])?; self.values.push_reserved(value).map_err(|_| b"pencil-value" as &'static [u8])?; }
                        self.entry += 1;
                    } else {
                        let depth = Self::depth(&matrix.indptr, self.row + 1)? + Self::push_depth(&self.colptr)?;
                        if !Self::admit(context, depth, 16)? { return Ok(false); }
                        self.colptr.push_reserved(self.rowind.len() as u32).map_err(|_| b"pencil-offset" as &'static [u8])?;
                        self.row += 1;
                        self.entry = end;
                    }
                }
            }
            PencilConstructionStage::ReserveMassIndptr => {
                if self.second.is_some() {
                    if !Self::admit(context, 1, size_of::<CscSym>() + size_of::<Csr>())? { return Ok(false); }
                    let first = CscSym::from_paged_parts(n, std::mem::take(&mut self.colptr), std::mem::take(&mut self.rowind), std::mem::take(&mut self.values));
                    self.complete = Some((first, self.second.take().expect("original pencil"))); self.stage = PencilConstructionStage::Complete;
                } else {
                    if !Self::reserve(&mut self.mass_indptr, n.saturating_add(1), context)? { return Ok(false); }
                    if !Self::admit(context, Self::push_depth(&self.mass_indptr)?, 4)? { return Ok(false); }
                    self.mass_indptr.push_reserved(0).map_err(|_| b"pencil-mass-offset" as &'static [u8])?; self.stage = PencilConstructionStage::ReserveMassIndices;
                }
            }
            PencilConstructionStage::ReserveMassIndices => {
                if !Self::reserve(&mut self.mass_indices, n, context)? { return Ok(false); }
                if !Self::admit(context, 1, 1)? { return Ok(false); } self.stage = PencilConstructionStage::ReserveMassValues;
            }
            PencilConstructionStage::ReserveMassValues => {
                if !Self::reserve(&mut self.mass_values, n, context)? { return Ok(false); }
                if !Self::admit(context, 1, 16)? { return Ok(false); } self.row = 0; self.stage = PencilConstructionStage::CopyMountedMass;
            }
            PencilConstructionStage::CopyMountedMass => {
                if self.row < n {
                    let value = self.mass.as_ref().map(|mass| mass.get(self.row)).or_else(|| self.mounted_mass.as_ref().and_then(|mass| mass.get(self.row))).ok_or(b"pencil-mass-owner" as &'static [u8])?;
                    if !Self::admit(context, Self::push_depth(&self.mass_values)?, 16)? { return Ok(false); }
                    self.mass_values.push_reserved(value).map_err(|_| b"pencil-mass-value" as &'static [u8])?; self.row += 1;
                } else if Self::admit(context, 1, 1)? { self.stage = PencilConstructionStage::RetireMountedMass; }
            }
            PencilConstructionStage::RetireMountedMass => {
                if let Some(mass) = self.mass.as_mut() {
                    let demand = if mass.0.is_empty() { mass.0.capacity().saturating_mul(size_of::<f64>()) } else { 0 };
                    if !numerical_grant_covers(context.retained_grant(), RetirementDemand { copy_bytes: size_of::<Self>() * 2 + 8, release_bytes: demand, depth: 1, ..Default::default() }) { return Ok(false); }
                    match close_vec_owner_step(&mut mass.0, context.retained_grant().maximum_release_bytes) {
                        Ok(Some((items, bytes))) => { context.consume_retained(RetainedCloneProgress { copied_items: items.max(1), copied_bytes: size_of::<Self>() * 2 + 8, released_bytes: bytes, ..Default::default() }).map_err(|_| b"pencil-mass-close" as &'static [u8])?; return Ok(false); },
                        Err(()) => return Ok(false), Ok(None) => {},
                    }
                    if !Self::admit(context, 1, size_of::<VecD>())? { return Ok(false); } self.mass = None;
                } else if let Some(mass) = self.mounted_mass.as_mut() {
                    if !Self::admit(context, 1, 8)? { return Ok(false); }
                    if !mass.close_step() { return Ok(false); } self.mounted_mass = None;
                } else {
                    if !Self::admit(context, 1, 16)? { return Ok(false); } self.row = 0; self.stage = PencilConstructionStage::BuildMass;
                }
            }
            PencilConstructionStage::BuildMass => {
                if self.row < n {
                    if !Self::admit(context, Self::push_depth(&self.mass_indices)? + Self::push_depth(&self.mass_indptr)?, 16)? { return Ok(false); }
                    self.mass_indices.push_reserved(self.row as u32).map_err(|_| b"pencil-mass-index" as &'static [u8])?;
                    self.mass_indptr.push_reserved((self.row + 1) as u32).map_err(|_| b"pencil-mass-offset" as &'static [u8])?; self.row += 1;
                } else {
                    if !Self::admit(context, 1, size_of::<CscSym>() + size_of::<Csr>())? { return Ok(false); }
                    let first = CscSym::from_paged_parts(n, std::mem::take(&mut self.colptr), std::mem::take(&mut self.rowind), std::mem::take(&mut self.values));
                    let second = Csr::from_paged_parts(n, std::mem::take(&mut self.mass_indptr), std::mem::take(&mut self.mass_indices), std::mem::take(&mut self.mass_values));
                    self.complete = Some((first, second)); self.stage = PencilConstructionStage::Complete;
                }
            }
            PencilConstructionStage::Complete => return Ok(true),
        }
        Ok(self.stage == PencilConstructionStage::Complete)
    }

    pub fn take_complete(&mut self, grant: RetainedCloneGrant) -> Option<((CscSym, Csr), RetainedCloneProgress)> {
        let copied_bytes = (size_of::<CscSym>() + size_of::<Csr>()) * 2;
        if self.stage != PencilConstructionStage::Complete || !numerical_grant_covers(grant, RetirementDemand { copy_bytes: copied_bytes, depth: 1, ..Default::default() }) { return None; }
        self.complete.take().map(|owners| (owners, RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() }))
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if let Some((first, second)) = self.complete.as_mut() {
            let step = first.close_step(maximum_bytes); if !step.0 { return step; }
            let step = second.close_step(maximum_bytes); if !step.0 { return step; }
            self.complete = None; return (false, 1, 0);
        }
        for slot in [&mut self.matrix, &mut self.second] {
            if let Some(matrix) = slot.as_mut() { let step = matrix.close_step(maximum_bytes); if !step.0 { return step; } *slot = None; return (false, 1, 0); }
        }
        if let Some(mass) = self.mass.as_mut() {
            match close_vec_owner_step(&mut mass.0, maximum_bytes) { Ok(Some((items, bytes))) => return (false, items, bytes), Err(()) => return (false, 0, 0), Ok(None) => {} }
            self.mass = None; return (false, 1, 0);
        }
        if let Some(mass) = self.mounted_mass.as_mut() { if !mass.close_step() { return (false, 1, 0); } self.mounted_mass = None; return (false, 1, 0); }
        for owner in [&mut self.values, &mut self.mass_values] { match close_paged_owner_step(owner, maximum_bytes) { Ok(Some((items, bytes))) => return (false, items, bytes), Err(()) => return (false, 0, 0), Ok(None) => {} } }
        for owner in [&mut self.colptr, &mut self.rowind, &mut self.mass_indptr, &mut self.mass_indices] { match close_paged_owner_step(owner, maximum_bytes) { Ok(Some((items, bytes))) => return (false, items, bytes), Err(()) => return (false, 0, 0), Ok(None) => {} } }
        (true, 0, 0)
    }
}


// #endregion 🔖️Pcg

// #region 🔖️DenseEigen




// #endregion 🔖️DenseEigen

// #region 🔖️SubspaceIteration



/// 🎯️ Only certified physical positive eigenpairs or explicit full-span absence are publishable.


pub trait NumericalMatrixRestore{type Owner:NumericalRestoreOwner<f64>;fn owner(&mut self)->&mut Self::Owner;fn shape(&mut self,rows:usize,columns:usize);}
impl NumericalMatrixRestore for MatD{type Owner=Vec<f64>;fn owner(&mut self)->&mut Self::Owner{&mut self.data}fn shape(&mut self,rows:usize,columns:usize){self.rows=rows;self.cols=columns;}}
impl NumericalMatrixRestore for semio_s_fem_algebra::PagedMatD{type Owner=PagedList<f64,SPARSE_INDEX_SPACE>;fn owner(&mut self)->&mut Self::Owner{&mut self.data}fn shape(&mut self,rows:usize,columns:usize){self.rows=rows;self.cols=columns;}}


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
