//! 🎵️ Subspace-iteration eigensolver for the generalized problem `Kφ=λBφ` (modal and buckling), with the modal
//! input construction that feeds it and the dense cyclic-Jacobi oracle of its tests.

use semio_framework_job::{OriginalTurnFailure, InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeView, JobPayloadAdmissionFault, JobPayloadStream, Operation, RetainedCloneGrant, RetainedCloneProgress, RetainedJobPayload, RetainedJobPayloadWriter, StepContext};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_s_fem_sparse::{numerical_close_original, CscSym, Csr, MountedScalarSlots, NUMERICAL_OWNER_PAGE_BYTES, NumericalBatchEnd, NumericalCheckpointFault, NumericalCheckpointIdentity, NumericalOutcomeDesk, NumericalOwnerRestoreCursor, NumericalPageCursor, NumericalPageView, NumericalRun, advance_f64_owner, advance_matrix_owner, advance_numerical_page_header, advance_owner_length, advance_paged_f64_owner, advance_paged_u32_owner, advance_pair_owner, advance_u64_owner, advance_u64_values, close_nested_vec_owner_step, close_vec_owner_step, declared_owner_length, numerical_batch_close, numerical_batch_step, numerical_close_gate, numerical_ladder_turn, numerical_restore_close_due, numerical_restore_frontier, numerical_restore_payload_close, numerical_rung_demand, numerical_writer_close, numerical_writer_demand, parse_numerical_page, read_checkpoint_u64, read_checkpoint_usize, restore_f64_entry, restore_numerical_owner, restore_paged_f64_entry, restore_paged_u32_entry, restore_pair_entry, restore_usize_entry, validate_restored_owner};
use semio_s_fem_sparse_ldlt::{LdltFactor};
use semio_s_fem_algebra::{MatD, VecD};

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
        if owner.capacity().checked_mul(size_of::<T>()).is_none_or(|bytes| bytes > NUMERICAL_OWNER_PAGE_BYTES) {
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
                    let value = self.mass.as_ref().map(|mass| mass.get(self.row)).or_else(|| self.mounted_mass.as_ref().and_then(|mass| mass.get(self.row))).ok_or(b"modal-input-mass-owner" as &'static [u8])?;
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
                    let end = matrix.indptr(self.row + 1).ok_or(b"modal-input-row-end" as &'static [u8])? as usize;
                    if self.entry < end {
                        if matrix.index(self.entry).ok_or(b"modal-input-column" as &'static [u8])? as usize >= self.row {
                            self.upper_count = self.upper_count.checked_add(1).ok_or(b"modal-input-upper-overflow" as &'static [u8])?;
                        }
                        self.entry += 1;
                    } else {
                        self.row += 1;
                        self.entry = matrix.indptr(self.row).unwrap_or(0) as usize;
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
                    let end = matrix.indptr(self.row + 1).ok_or(b"modal-input-row-end" as &'static [u8])? as usize;
                    if self.entry < end {
                        let column = matrix.index(self.entry).ok_or(b"modal-input-column" as &'static [u8])? as usize;
                        if column >= self.row {
                            self.rowind.push(column as u32);
                            self.values.push(matrix.value(self.entry).ok_or(b"modal-input-value" as &'static [u8])?);
                        }
                        self.entry += 1;
                    } else {
                        self.colptr.push(self.rowind.len() as u32);
                        self.row += 1;
                        self.entry = matrix.indptr(self.row).unwrap_or(0) as usize;
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
                    let value = self.mounted_mass.as_ref().and_then(|mass| mass.get(self.row)).ok_or(b"modal-input-mounted-mass-owner" as &'static [u8])?;
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
            let (terminal, items, bytes) = stiffness.close_step(maximum_bytes);
            if !terminal {
                return (false, items, bytes);
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
// #region 🔖️DenseEigen
/// 🎯️ Cyclic Jacobi eigenvalue algorithm for a small dense symmetric matrix — returns eigenvalues
/// (ascending) and the matching eigenvectors as columns of the returned `MatD`. Used internally to
/// solve the small (`p×p`, `p ≤ ~40`) projected eigenproblem inside `subspace_iteration`.
#[cfg(test)]
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

#[cfg(test)]
fn frobenius_norm(m: &MatD) -> f64 {
    let mut sum = 0.0;
    for row in 0..m.rows {
        for col in 0..m.cols {
            sum += m.get(row, col) * m.get(row, col);
        }
    }
    sum.sqrt()
}

// #endregion 🔖️DenseEigen

// #region 🔖️SubspaceIteration
/// 📐️ The lowest `p` eigenpairs of a generalized eigenproblem, ascending by value.
#[derive(Clone, Debug, PartialEq)]
pub struct EigenPairs {
    pub values: Vec<f64>,
    pub vectors: Vec<VecD>,
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, PartialEq)]
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
    fn checkpoint_control(&self) -> [u64; 12] {
        [
            self.stage as u64,
            self.reserve as u64,
            self.first as u64,
            self.second as u64,
            self.third as u64,
            self.phase as u64,
            self.sweep as u64,
            self.scalar.to_bits(),
            self.coefficient.to_bits(),
            self.cosine.to_bits(),
            self.sine.to_bits(),
            self.tangent.to_bits(),
        ]
    }

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

/// 📏️ Every dense subspace owner (`x`, `rhs`, `solved`, `b_basis`, `candidate_x`) is one
/// `n × m` page of `f64`, and the narrowest admitted subspace is `m = p + 8 ≥ 9` columns, so this
/// is the largest order any admitted problem can page — `reserve_matrix_owner` still prices the
/// exact `n × m` product per owner. The fem3d demo hall alone is 132 free DOFs.
const SUBSPACE_MINIMUM_COLUMNS: usize = 9;
const SUBSPACE_MAXIMUM_ORDER: usize = NUMERICAL_OWNER_PAGE_BYTES / size_of::<f64>() / SUBSPACE_MINIMUM_COLUMNS;
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
    desk: NumericalOutcomeDesk,
}

fn close_subspace_checkpoint_step(state: &mut SubspaceCheckpoint, maximum_bytes: usize) -> (bool, usize, usize) {
    match close_nested_vec_owner_step(&mut state.k_factor.l_cols, maximum_bytes) {
        Ok(Some((items, bytes))) => return (false, items, bytes),
        Err(()) => return (false, 0, 0),
        Ok(None) => {}
    }
    for owner in [&mut state.k_factor.d, &mut state.x.data, &mut state.prev_theta, &mut state.final_theta, &mut state.residuals] {
        match close_vec_owner_step(owner, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
    }
    let (matrix_terminal, matrix_items, matrix_bytes) = state.b.close_step(maximum_bytes);
    if !matrix_terminal {
        return (false, matrix_items, matrix_bytes);
    }
    let (terminal, items, bytes) = state.work.close_step(maximum_bytes);
    if !terminal {
        return (false, items, bytes);
    }
    if let Some(retiring) = state.retiring_work.as_mut() {
        let (terminal, items, bytes) = retiring.close_step(maximum_bytes);
        if !terminal {
            return (false, items, bytes);
        }
        state.retiring_work = None;
        return (false, 1, 0);
    }
    (true, 0, 0)
}

impl SubspaceIterationJob {
    pub fn new(operation: Operation, k_factor: LdltFactor, b: Csr, n: usize, p: usize, max_iter: usize) -> Self {
        let factor_pages_valid = k_factor.l_cols.capacity().saturating_mul(size_of::<Vec<(u32, f64)>>()) <= NUMERICAL_OWNER_PAGE_BYTES;
        let admission_fault = n == 0 || p == 0 || p > n || b.n != n || k_factor.n != n || n > SUBSPACE_MAXIMUM_ORDER || !factor_pages_valid || !b.ownership_is_valid();
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
            desk: NumericalOutcomeDesk::default(),
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
                let values = work.checkpoint_control();
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
            515 => advance_paged_u32_owner(writer, &state.b.indptr, cursor)?,
            516 => advance_paged_u32_owner(writer, &state.b.indices, cursor)?,
            517 => advance_paged_f64_owner(writer, &state.b.vals, cursor)?,
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
                    let values = work.checkpoint_control();
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
        if matrix.data.capacity().checked_mul(size_of::<f64>()).ok_or(())? > NUMERICAL_OWNER_PAGE_BYTES {
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
        if owner.capacity().checked_mul(size_of::<T>()).ok_or(())? > NUMERICAL_OWNER_PAGE_BYTES {
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
        let start = self.state.b.indptr(row).expect("subspace row start") as usize;
        let end = self.state.b.indptr(row + 1).expect("subspace row end") as usize;
        if work.third < end.saturating_sub(start) {
            let index = start + work.third;
            let source_row = self.state.b.index(index).expect("subspace column") as usize;
            let source = if using_basis { work.solved.get(source_row, work.first) } else { self.state.x.get(source_row, work.first) };
            work.scalar += self.state.b.value(index).expect("subspace value") * source;
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
            work.third = 0;
            work.stage = SubspaceStage::FactorDiagonalEntry;
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
            work.second += 1;
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

    fn advance_orthogonalize(&mut self) {
        let n = self.state.n;
        let m = self.state.m;
        let work = &mut self.state.work;
        if work.first >= m {
            work.phase = 1;
            self.reset_cursor(SubspaceStage::ApplyOperatorColumnRow);
            self.state.work.phase = 1;
            return;
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
            return;
        }
        work.stage = SubspaceStage::NormalizeColumnElement;
        work.phase = 0;
        work.third = 0;
        work.scalar = 0.0;
    }

    fn advance_normalize(&mut self) {
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
                    return;
                }
                work.coefficient = work.scalar.max(1e-300).sqrt().recip();
                work.third = 0;
                work.phase = 1;
            }
            return;
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
            return;
        }
        if work.phase == 3 {
            if work.second == n {
                work.second = 0;
                work.phase = 4;
                return;
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
            return;
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
            return;
        }
        if work.phase == 5 {
            if work.second == n {
                work.second = 0;
                work.third = 0;
                work.coefficient = 0.0;
                work.phase = 6;
                return;
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
            return;
        }
        if work.phase == 6 {
            if work.second == work.first {
                work.third = 0;
                work.scalar = 0.0;
                work.phase = 8;
                return;
            }
            work.coefficient += work.solved.get(work.third, work.first) * work.rhs.get(work.third, work.second);
            work.third += 1;
            if work.third == n {
                work.third = 0;
                work.phase = 7;
            }
            return;
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
            return;
        }
        if work.phase == 8 {
            work.scalar += work.solved.get(work.third, work.first) * work.rhs.get(work.third, work.first);
            work.third += 1;
            if work.third == n {
                work.coefficient = work.scalar.max(1e-300).sqrt().recip();
                work.third = 0;
                work.phase = 1;
            }
            return;
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
            self.reset_cursor(SubspaceStage::PublishIteration);
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
        self.state.converged = self.state.converged_count == self.state.p;
        self.state.checkpoint_due = true;
        self.state.preview_due = true;
        self.state.publication_stage = 0;
        self.state.publication_first = 0;
        self.state.publication_second = 0;
        let displaced = std::mem::replace(&mut self.state.work, SubspaceWork::empty());
        self.state.retiring_work = Some(displaced);
    }

    fn close_retained_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        close_subspace_checkpoint_step(&mut self.state, maximum_bytes)
    }

    fn close_terminal_is_empty(&self) -> bool {
        self.state.k_factor.l_cols.capacity() == 0
            && self.state.k_factor.d.capacity() == 0
            && self.state.b.terminal_is_empty()
            && self.state.x.data.capacity() == 0
            && self.state.prev_theta.capacity() == 0
            && self.state.final_theta.capacity() == 0
            && self.state.residuals.capacity() == 0
            && self.state.work.terminal_is_empty()
            && self.state.retiring_work.is_none()
    }
}

fn restore_matrix_entry(matrix: &mut MatD, page: &NumericalPageView<'_>, maximum: usize, entry: &mut usize, cursor: &mut NumericalOwnerRestoreCursor) -> Result<bool, NumericalCheckpointFault> {
    let complete = restore_numerical_owner(&mut matrix.data, page, maximum, entry, cursor, true)?;
    if let Some(shape) = cursor.shape { matrix.rows = shape.rows; matrix.cols = shape.cols; }
    Ok(complete)
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
    work.close_lane = 0;
    Ok(())
}

fn restore_work_entry(work: &mut SubspaceWork, page: &NumericalPageView<'_>, base: u16, n: usize, m: usize, entry: &mut usize, control: &mut [u64; 24], cursor: &mut NumericalOwnerRestoreCursor) -> Result<bool, NumericalCheckpointFault> {
    match page.field - base {
        0 => {
            let count = read_checkpoint_usize(page.bytes, 0)?;
            if count != 12 || page.bytes.len() != 8 + count * 8 {
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
        1 => restore_matrix_entry(&mut work.rhs, page, n.saturating_mul(m), entry, cursor),
        2 => restore_matrix_entry(&mut work.solved, page, n.saturating_mul(m), entry, cursor),
        3 => restore_matrix_entry(&mut work.b_basis, page, n.saturating_mul(m), entry, cursor),
        4 => restore_matrix_entry(&mut work.projected, page, m.saturating_mul(m), entry, cursor),
        5 => restore_matrix_entry(&mut work.jacobi, page, m.saturating_mul(m), entry, cursor),
        6 => restore_matrix_entry(&mut work.jacobi_vectors, page, m.saturating_mul(m), entry, cursor),
        7 => restore_matrix_entry(&mut work.ordered_vectors, page, m.saturating_mul(m), entry, cursor),
        8 => restore_matrix_entry(&mut work.candidate_x, page, n.saturating_mul(m), entry, cursor),
        9 => restore_f64_entry(&mut work.mu, page, m, entry, cursor),
        10 => restore_f64_entry(&mut work.theta, page, m, entry, cursor),
        11 => restore_usize_entry(&mut work.order, page, m, entry, cursor),
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
    owner_cursor: NumericalOwnerRestoreCursor,
    control: [u64; 24],
    state: Option<SubspaceCheckpoint>,
    fault: Option<NumericalCheckpointFault>,
}

impl SubspaceRestoreCursor {
    pub fn new(operation: Operation, payload: RetainedJobPayload) -> Self {
        let total_pages = payload.page_count();
        Self { operation, payload: Some(payload), total_pages, page_slot: 0, close_due: false, expected_field: 0, page_entry: 0, owner_cursor: NumericalOwnerRestoreCursor::default(), control: [0; 24], state: None, fault: None }
    }

    fn decode_page_entry(&mut self, bytes: &[u8]) -> Result<bool, NumericalCheckpointFault> {
        let page = parse_numerical_page(bytes, b"FEMSCP1\0")?;
        if page.kind != 12 || page.field != self.expected_field || (self.owner_cursor.shape.is_none() && (page.owner != 0 || page.item != 0)) {
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
                b: Csr::from_owned_parts(n, Vec::new(), Vec::new(), Vec::new()),
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
            field if field >= 2 && field < 2 + n as u16 => restore_pair_entry(&mut state.k_factor.l_cols[(field - 2) as usize], &page, n, &mut self.page_entry, &mut self.owner_cursor)?,
            514 => restore_f64_entry(&mut state.k_factor.d, &page, n, &mut self.page_entry, &mut self.owner_cursor)?,
            515 => restore_paged_u32_entry(&mut state.b.indptr, &page, n + 1, &mut self.page_entry, &mut self.owner_cursor)?,
            516 => restore_paged_u32_entry(&mut state.b.indices, &page, n.saturating_mul(n), &mut self.page_entry, &mut self.owner_cursor)?,
            517 => restore_paged_f64_entry(&mut state.b.vals, &page, n.saturating_mul(n), &mut self.page_entry, &mut self.owner_cursor)?,
            518 => restore_matrix_entry(&mut state.x, &page, n.saturating_mul(m), &mut self.page_entry, &mut self.owner_cursor)?,
            519 => restore_f64_entry(&mut state.prev_theta, &page, state.p, &mut self.page_entry, &mut self.owner_cursor)?,
            520 => restore_f64_entry(&mut state.final_theta, &page, m, &mut self.page_entry, &mut self.owner_cursor)?,
            521 => restore_f64_entry(&mut state.residuals, &page, state.p, &mut self.page_entry, &mut self.owner_cursor)?,
            field if (522..=533).contains(&field) => restore_work_entry(&mut state.work, &page, 522, n, m, &mut self.page_entry, &mut self.control, &mut self.owner_cursor)?,
            534 => {
                let present = read_checkpoint_u64(page.bytes, 0)?;
                if self.page_entry == 0 {
                    self.page_entry = 1;
                    return Ok(false);
                }
                match present {
                    0 if page.bytes.len() == 8 => true,
                    1 => {
                        let count = read_checkpoint_usize(page.bytes, 8)?;
                        if count != 12 || page.bytes.len() != 16 + count * 8 {
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
                restore_work_entry(work, &page, 534, n, m, &mut self.page_entry, &mut self.control, &mut self.owner_cursor)?
            }
            _ => return Err(NumericalCheckpointFault::Field),
        };
        if complete && self.owner_cursor.shape.is_none() {
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
        if let Some(fault) = self.fault { return Err(fault); }
        let outcome = self.advance(context);
        if let Err(fault) = outcome { self.fault = Some(fault); }
        outcome
    }

    fn advance(&mut self, context: &mut StepContext<'_>) -> Result<Option<SubspaceIterationJob>, NumericalCheckpointFault> {
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
                return Ok(Some(SubspaceIterationJob {
                    operation: self.operation,
                    state,
                    preview_writer: None,
                    preview_page_cursor: 0,
                    terminal_writer: None,
                    terminal_page_cursor: 0,
                    checkpoint_writer: None,
                    checkpoint_cursor: NumericalPageCursor::new(),
                    desk: NumericalOutcomeDesk::default(),
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
        let (terminal, items, bytes) = close_subspace_checkpoint_step(state, grant.maximum_release_bytes);
        let step = if terminal {
            self.state = None;
            InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }
        } else {
            InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: items, released_bytes: bytes, ..Default::default() } }
        };
        step.admit(grant, self.terminal_is_empty())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.payload.is_none() && self.state.is_none()
    }
}

impl SubspaceIterationJob {
    fn run(&mut self, context: &mut StepContext<'_>) -> NumericalRun {
        if context.is_cancelled() {
            return NumericalRun::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return NumericalRun::Fault;
        }
        if !self.state.factor_validation_complete {
            context.set_stage("fem.subspace.validate-factor-owner");
            if context.should_yield() {
                return NumericalRun::Yield;
            }
            context.consume_fuel(1);
            if let Some(column) = self.state.k_factor.l_cols.get(self.state.factor_validation_cursor) {
                if column.capacity().saturating_mul(size_of::<(u32, f64)>()) > NUMERICAL_OWNER_PAGE_BYTES {
                    self.state.admission_fault = true;
                    self.state.factor_validation_complete = true;
                    return NumericalRun::Fault;
                }
                self.state.factor_validation_cursor += 1;
                return NumericalRun::Yield;
            }
            self.state.factor_validation_complete = true;
            return NumericalRun::Yield;
        }
        if self.state.admission_fault || self.state.n > SUBSPACE_MAXIMUM_ORDER || self.state.m > SUBSPACE_MAXIMUM_COLUMNS {
            return NumericalRun::Fault;
        }
        if self.state.checkpoint_due || self.checkpoint_writer.is_some() {
            context.set_stage("fem.subspace.checkpoint-page");
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
            return self.desk.lend_checkpoint(state, self.state.iteration as u64);
        }
        if self.state.preview_due {
            context.set_stage("fem.subspace.preview-entry");
            if context.should_yield() {
                return NumericalRun::Yield;
            }
            if self.preview_writer.is_none() {
                context.consume_fuel(1);
                self.preview_writer = Some(RetainedJobPayloadWriter::new(JobPayloadStream::Preview));
                self.preview_page_cursor = 0;
                return NumericalRun::Yield;
            }
            context.consume_fuel(1);
            let Some(writer) = self.preview_writer.as_mut() else {
                return NumericalRun::Fault;
            };
            if writer.staged_page_len().is_none() && self.state.publication_stage < 4 {
                return match writer.begin_staged_page(context) {
                    Ok(()) => NumericalRun::Yield,
                    Err(_) => NumericalRun::Fault,
                };
            }
            let complete = match Self::advance_preview_entry(&mut self.state, self.operation, writer) {
                Ok(complete) => complete,
                Err(_) => return NumericalRun::Fault,
            };
            if !complete {
                return NumericalRun::Yield;
            }
            let Some(writer) = self.preview_writer.take() else {
                return NumericalRun::Fault;
            };
            let preview = match writer.finish() {
                Ok(preview) => preview,
                Err(writer) => {
                    self.preview_writer = Some(writer);
                    return NumericalRun::Fault;
                }
            };
            self.preview_page_cursor = 0;
            self.state.preview_due = false;
            return self.desk.lend_preview(preview);
        }
        if self.state.iteration >= self.state.max_iter || self.state.converged {
            context.set_stage("fem.subspace.terminal-page");
            if context.should_yield() {
                return NumericalRun::Yield;
            }
            context.consume_fuel(1);
            if self.terminal_writer.is_none() {
                self.state.publication_stage = 0;
                self.state.publication_first = 0;
                self.state.publication_second = 0;
                self.terminal_writer = Some(RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput));
                self.terminal_page_cursor = 0;
                return NumericalRun::Yield;
            }
            let Some(writer) = self.terminal_writer.as_mut() else {
                return NumericalRun::Fault;
            };
            if writer.staged_page_len().is_none() && self.state.publication_stage < 4 {
                return match writer.begin_staged_page(context) {
                    Ok(()) => NumericalRun::Yield,
                    Err(_) => NumericalRun::Fault,
                };
            }
            let complete = match Self::advance_preview_entry(&mut self.state, self.operation, writer) {
                Ok(complete) => complete,
                Err(_) => return NumericalRun::Fault,
            };
            if !complete {
                return NumericalRun::Yield;
            }
            let Some(writer) = self.terminal_writer.take() else {
                return NumericalRun::Fault;
            };
            let output = match writer.finish() {
                Ok(output) => output,
                Err(writer) => {
                    self.terminal_writer = Some(writer);
                    return NumericalRun::Fault;
                }
            };
            return self.desk.lend_complete(output);
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
            return NumericalRun::Yield;
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
            SubspaceStage::OrthogonalizePairElement => {
                self.advance_orthogonalize();
                Ok(())
            }
            SubspaceStage::NormalizeColumnElement => {
                self.advance_normalize();
                Ok(())
            }
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
            return NumericalRun::Fault;
        }
        if context.is_cancelled() {
            return NumericalRun::Cancelled;
        }
        NumericalRun::Yield
    }

    fn largest_owner_bytes(&self) -> usize {
        let state = &self.state;
        [state.k_factor.d.capacity() * size_of::<f64>(), state.x.data.capacity() * size_of::<f64>(), state.prev_theta.capacity() * size_of::<f64>(), state.final_theta.capacity() * size_of::<f64>(), state.residuals.capacity() * size_of::<f64>()]
            .into_iter()
            .chain(state.k_factor.l_cols.iter().map(|column| column.capacity() * size_of::<(u32, f64)>()))
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
        for slot in [&self.checkpoint_writer, &self.preview_writer, &self.terminal_writer] {
            if let Some(demand) = numerical_writer_demand(slot) {
                return demand.map(Some);
            }
        }
        Ok(None)
    }

    fn close_demand(&self) -> Result<RetirementDemand, ValueError> {
        Ok(self.close_frontier()?.unwrap_or_else(|| self.rung_demand()))
    }

    fn close_turn(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        if let Some(step) = self.desk.close_step(grant) {
            return step;
        }
        if let Some(step) = numerical_writer_close(&mut self.checkpoint_writer, grant) {
            return step;
        }
        if let Some(step) = numerical_writer_close(&mut self.preview_writer, grant) {
            return step;
        }
        if let Some(step) = numerical_writer_close(&mut self.terminal_writer, grant) {
            return step;
        }
        Ok(numerical_ladder_turn(self.close_retained_step(grant.maximum_release_bytes)))
    }
}

impl InteractiveJob for SubspaceIterationJob {
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
        if let Some(writer) = self.preview_writer.as_mut() {
            writer.begin_close();
        }
        if let Some(writer) = self.terminal_writer.as_mut() {
            writer.begin_close();
        }
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if self.terminal_is_empty() {
            return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
        }
        let demand = self.close_demand()?;
        numerical_close_original(cx, self, demand, Self::close_granted, Self::terminal_is_empty)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand()?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> { Ok(self.close_demand()?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand()?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand()?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.checkpoint_writer.is_none() && self.preview_writer.is_none() && self.terminal_writer.is_none() && self.desk.terminal_is_empty() && self.close_terminal_is_empty()
    }
}

impl SubspaceIterationJob {
    /// 🎟️ One close turn under an explicit grant: the owners of a child job and the original-context `close_step`
    /// both admit it through the job's own quote.
    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        let frontier = self.close_frontier()?;
        if self.terminal_is_empty() {
            return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
        }
        if !numerical_close_gate(grant, frontier) {
            return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
        }
        self.close_turn(grant)?.admit(grant, self.terminal_is_empty())
    }
}

/// 🎯️ Finds the lowest `p` generalized eigenpairs through the retained subspace cursor.
pub fn subspace_iteration(k_factor: &LdltFactor, b: &Csr, n: usize, p: usize, max_iter: usize) -> EigenPairs {
    let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), 0);
    let mut job = SubspaceIterationJob::new(operation, k_factor.clone(), b.clone(), n, p, max_iter);
    let mut preview_sequence = 0;
    loop {
        match numerical_batch_step(&mut job, operation, &mut preview_sequence) {
            NumericalBatchEnd::Complete => {
                let solution = job.solution();
                numerical_batch_close(&mut job);
                return solution;
            }
            NumericalBatchEnd::Fault => {
                numerical_batch_close(&mut job);
                panic!("subspace batch adapter faulted")
            }
            NumericalBatchEnd::Cancelled | NumericalBatchEnd::Continue => {}
        }
    }
}
// #endregion 🔖️SubspaceIteration

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
