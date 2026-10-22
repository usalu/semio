//! 🌀️ Dense Jacobi eigen kernel, subspace state types and the job constructor, accessors and checkpoint owners.

use super::*;

// #region 🔖️DenseEigen
/// 🎯️ Cyclic Jacobi eigenvalue algorithm for a small dense symmetric matrix — returns eigenvalues
/// (ascending) and the matching eigenvectors as columns of the returned `MatD`. Used internally to
/// solve the small (`p×p`, `p ≤ ~40`) projected eigenproblem inside `subspace_iteration`.
#[cfg(test)]
pub(super) fn dense_symmetric_eigen_jacobi(a: &MatD) -> (Vec<f64>, MatD) {
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
                let t = if theta >= 0.0 { 1.0 / (theta + theta.hypot(1.0)) } else { -1.0 / (-theta + theta.hypot(1.0)) };
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
pub(super) fn frobenius_norm(m: &MatD) -> f64 {
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

pub(super) fn mat_col(m: &semio_s_fem_algebra::PagedMatD, col: usize) -> VecD {
    VecD::from_vec((0..m.rows).map(|row| m.get(row, col)).collect())
}

#[derive(Clone, PartialEq)]
pub(super) struct SubspaceCheckpoint {
    pub(super) identity: NumericalCheckpointIdentity,
    pub(super) k_factor: LdltFactor,
    pub(super) b: Csr,
    pub(super) n: usize,
    pub(super) p: usize,
    pub(super) max_iter: usize,
    pub(super) m: usize,
    pub(super) shift:Option<f64>,
    pub(super) eigensystem_certified:bool,
    pub(super) x: semio_s_fem_algebra::PagedMatD,
    pub(super) prev_theta: PagedList<f64,SPARSE_INDEX_SPACE>,
    pub(super) final_theta: PagedList<f64,SPARSE_INDEX_SPACE>,
    pub(super) iteration: usize,
    pub(super) residuals: PagedList<f64,SPARSE_INDEX_SPACE>,
    pub(super) converged_count: usize,
    pub(super) converged: bool,
    pub(super) checkpoint_due: bool,
    pub(super) preview_due: bool,
    pub(super) admission_fault: bool,
    pub(super) factor_validation_cursor: usize,
    pub(super) factor_validation_complete: bool,
    pub(super) initialization_cursor: usize,
    pub(super) publication_stage: u8,
    pub(super) publication_first: usize,
    pub(super) publication_second: usize,
    pub(super) work: SubspaceWork,
    pub(super) retiring_work: Option<SubspaceWork>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SubspaceStage {
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
    NormInverseInitialize,
    NormInverseAccumulate,
    NormRightRows,
    ShiftOperatorColumnRow,
    CertifyNonPositive,
}

#[derive(Clone, PartialEq)]
pub(super) struct SubspaceWork {
    pub(super) stage: SubspaceStage,
    pub(super) reserve: usize,
    pub(super) first: usize,
    pub(super) second: usize,
    pub(super) third: usize,
    pub(super) phase: usize,
    pub(super) sweep: usize,
    pub(super) scalar: f64,
    pub(super) coefficient: f64,
    pub(super) cosine: f64,
    pub(super) sine: f64,
    pub(super) tangent: f64,
    pub(super) rhs: semio_s_fem_algebra::PagedMatD,
    pub(super) solved: semio_s_fem_algebra::PagedMatD,
    pub(super) b_basis: semio_s_fem_algebra::PagedMatD,
    pub(super) projected: semio_s_fem_algebra::PagedMatD,
    pub(super) jacobi: semio_s_fem_algebra::PagedMatD,
    pub(super) jacobi_vectors: semio_s_fem_algebra::PagedMatD,
    pub(super) ordered_vectors: semio_s_fem_algebra::PagedMatD,
    pub(super) candidate_x: semio_s_fem_algebra::PagedMatD,
    pub(super) mu: PagedList<f64,SPARSE_INDEX_SPACE>,
    pub(super) theta: PagedList<f64,SPARSE_INDEX_SPACE>,
    pub(super) order: PagedList<usize,SPARSE_INDEX_SPACE>,
    pub(super) close_lane: u8,
}

impl SubspaceWork {
    pub(super) fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{
        for lane in self.close_lane..=10{let demand=match lane{
            0 if !self.rhs.data.terminal_is_empty()=>Some(paged_owner_retirement_demand(&self.rhs.data)?),
            1 if !self.solved.data.terminal_is_empty()=>Some(paged_owner_retirement_demand(&self.solved.data)?),
            2 if !self.b_basis.data.terminal_is_empty()=>Some(paged_owner_retirement_demand(&self.b_basis.data)?),
            3 if !self.projected.data.terminal_is_empty()=>Some(paged_owner_retirement_demand(&self.projected.data)?),
            4 if !self.jacobi.data.terminal_is_empty()=>Some(paged_owner_retirement_demand(&self.jacobi.data)?),
            5 if !self.jacobi_vectors.data.terminal_is_empty()=>Some(paged_owner_retirement_demand(&self.jacobi_vectors.data)?),
            6 if !self.ordered_vectors.data.terminal_is_empty()=>Some(paged_owner_retirement_demand(&self.ordered_vectors.data)?),
            7 if !self.candidate_x.data.terminal_is_empty()=>Some(paged_owner_retirement_demand(&self.candidate_x.data)?),
            8 if !self.mu.terminal_is_empty()=>Some(paged_owner_retirement_demand(&self.mu)?),
            9 if !self.theta.terminal_is_empty()=>Some(paged_owner_retirement_demand(&self.theta)?),
            10 if !self.order.terminal_is_empty()=>Some(paged_owner_retirement_demand(&self.order)?),_=>None};if let Some(mut demand)=demand{demand.depth+=1;return Ok(demand)}}Ok(RetirementDemand{depth:1,..Default::default()})
    }
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

    pub(super) fn empty() -> Self {
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
            rhs: semio_s_fem_algebra::PagedMatD::empty(),
            solved: semio_s_fem_algebra::PagedMatD::empty(),
            b_basis: semio_s_fem_algebra::PagedMatD::empty(),
            projected: semio_s_fem_algebra::PagedMatD::empty(),
            jacobi: semio_s_fem_algebra::PagedMatD::empty(),
            jacobi_vectors: semio_s_fem_algebra::PagedMatD::empty(),
            ordered_vectors: semio_s_fem_algebra::PagedMatD::empty(),
            candidate_x: semio_s_fem_algebra::PagedMatD::empty(),
            mu: PagedList::empty(),
            theta: PagedList::empty(),
            order: PagedList::empty(),
            close_lane: 0,
        }
    }

    pub(super) fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        loop {
            let step = match self.close_lane {
                0 => close_paged_owner_step(&mut self.rhs.data, maximum_bytes),
                1 => close_paged_owner_step(&mut self.solved.data, maximum_bytes),
                2 => close_paged_owner_step(&mut self.b_basis.data, maximum_bytes),
                3 => close_paged_owner_step(&mut self.projected.data, maximum_bytes),
                4 => close_paged_owner_step(&mut self.jacobi.data, maximum_bytes),
                5 => close_paged_owner_step(&mut self.jacobi_vectors.data, maximum_bytes),
                6 => close_paged_owner_step(&mut self.ordered_vectors.data, maximum_bytes),
                7 => close_paged_owner_step(&mut self.candidate_x.data, maximum_bytes),
                8 => close_paged_owner_step(&mut self.mu, maximum_bytes),
                9 => close_paged_owner_step(&mut self.theta, maximum_bytes),
                10 => close_paged_owner_step(&mut self.order, maximum_bytes),
                _ => return (true, 0, 0),
            };
            match step {
                Ok(Some((items, bytes))) => return (false, items, bytes),
                Ok(None) => self.close_lane += 1,
                Err(()) => return (false, 0, 0),
            }
        }
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.close_lane > 10
    }
}

pub struct SubspaceIterationJob {
    pub(super) operation: Operation,
    pub(super) state: SubspaceCheckpoint,
    pub(super) preview_writer: Option<RetainedJobPayloadWriter>,
    pub(super) preview_page_cursor: usize,
    pub(super) terminal_writer: Option<RetainedJobPayloadWriter>,
    pub(super) terminal_page_cursor: usize,
    pub(super) checkpoint_writer: Option<RetainedJobPayloadWriter>,
    pub(super) checkpoint_cursor: NumericalPageCursor,
    pub(super) desk: NumericalOutcomeDesk,
    pub(super) terminal_published: bool,
}

pub(super) fn close_subspace_checkpoint_step(state: &mut SubspaceCheckpoint, maximum_bytes: usize) -> (bool, usize, usize) {
    let(terminal,items,bytes)=state.k_factor.close_step(maximum_bytes);if !terminal{return(false,items,bytes)}
    for owner in [&mut state.prev_theta, &mut state.final_theta, &mut state.residuals] {
        match close_paged_owner_step(owner, maximum_bytes) {
            Ok(Some((items, bytes))) => return (false, items, bytes),
            Err(()) => return (false, 0, 0),
            Ok(None) => {}
        }
    }
    match close_paged_owner_step(&mut state.x.data,maximum_bytes){Ok(Some((items,bytes)))=>return(false,items,bytes),Err(())=>return(false,0,0),Ok(None)=>{}}
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
// #endregion 🔖️SubspaceIteration

impl SubspaceCheckpoint {
    pub(super) fn new(operation: Operation, k_factor: LdltFactor, b: Csr, n: usize, p: usize, max_iter: usize) -> Self {
        let admission_fault = !SubspaceIterationJob::admits_shape(n, p) || b.n != n || k_factor.order() != n || !b.ownership_is_valid();
        let m = if admission_fault { 0 } else { p.saturating_add(8).max(p.saturating_mul(2)).min(n).max(1) };
        Self {
                identity: NumericalCheckpointIdentity::from_operation(operation),
                k_factor,
                b,
                n,
                p,
                max_iter,
                m,
                shift:None,
                eigensystem_certified:false,
                x: semio_s_fem_algebra::PagedMatD::empty(),
                prev_theta: PagedList::empty(),
                final_theta: PagedList::empty(),
                iteration: 0,
                residuals: PagedList::empty(),
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
        }
    }

    pub(super) fn reset_cursor(&mut self, stage: SubspaceStage) {
        self.work.stage = stage;
        self.work.first = 0;
        self.work.second = 0;
        self.work.third = 0;
        self.work.phase = 0;
        self.work.scalar = 0.0;
    }

    pub(super) fn advance_inverse_norm_initialize(&mut self){
        let work=&mut self.work;
        if work.first<self.n{work.rhs.set(work.first,0,u8::from(work.first==work.reserve)as f64);if work.reserve==0{work.candidate_x.set(work.first,0,0.0);}work.first+=1;}else{self.reset_cursor(SubspaceStage::FactorForwardEntry);}
    }

    pub(super) fn advance_inverse_norm_accumulate(&mut self){
        let work=&mut self.work;
        if work.phase==0{
            if work.first<self.n{work.candidate_x.add_at(work.first,0,work.solved.get(work.first,0).abs());work.first+=1;}
            else if work.reserve+1<self.n{work.reserve+=1;self.reset_cursor(SubspaceStage::NormInverseInitialize);}
            else{work.first=0;work.coefficient=0.0;work.phase=1;}
        }else if work.first<self.n{work.coefficient=work.coefficient.max(work.candidate_x.get(work.first,0));work.first+=1;}else{self.shift=Some(work.coefficient);self.reset_cursor(SubspaceStage::NormRightRows);self.work.coefficient=0.0;}
    }

    pub(super) fn advance_right_norm(&mut self)->Result<(),()>{
        let work=&mut self.work;
        if work.first==self.n{
            let shift=self.shift.ok_or(())?*work.coefficient*(1.0+64.0*f64::EPSILON);
            if !shift.is_finite(){return Err(())}
            self.shift=Some(shift);
            self.reset_cursor(if self.m==self.n{SubspaceStage::ShiftOperatorColumnRow}else{SubspaceStage::ApplyOperatorColumnRow});
            return Ok(())
        }
        let start=self.b.indptr(work.first).ok_or(())?as usize;
        let end=self.b.indptr(work.first+1).ok_or(())?as usize;
        if start>end||end>self.b.indices.len(){return Err(())}
        if start+work.third==end{work.coefficient=work.coefficient.max(work.scalar);work.scalar=0.0;work.third=0;work.first+=1;return Ok(())}
        let index=start+work.third;
        let row=self.b.index(index).ok_or(())?as usize;
        let value=self.b.value(index).ok_or(())?;
        if row>=self.n||!value.is_finite()||work.third>0&&self.b.index(index-1).ok_or(())?as usize>=row{return Err(())}
        if work.phase==0{work.second=self.b.indptr(row).ok_or(())?as usize;work.phase=1;return Ok(())}
        let counterpart_end=self.b.indptr(row+1).ok_or(())?as usize;
        if work.second>=counterpart_end{return Err(())}
        let counterpart_row=self.b.index(work.second).ok_or(())?as usize;
        if counterpart_row<work.first{work.second+=1;return Ok(())}
        let counterpart=self.b.value(work.second).ok_or(())?;
        if counterpart_row!=work.first||!counterpart.is_finite()||(value-counterpart).abs()>8.0*f64::EPSILON*value.abs().max(counterpart.abs()){return Err(())}
        work.scalar+=value.abs();work.third+=1;work.phase=0;Ok(())
    }
}

impl SubspaceIterationJob {
    pub fn admits_shape(order:usize,modes:usize)->bool{order!=0&&modes!=0&&modes<=order&&order<=u32::MAX as usize&&order.checked_mul(order).and_then(|cells|cells.checked_mul(size_of::<f64>())).is_some()}

    pub fn new(operation: Operation, k_factor: LdltFactor, b: Csr, n: usize, p: usize, max_iter: usize) -> Self {
        Self {
            operation,
            state: SubspaceCheckpoint::new(operation, k_factor, b, n, p, max_iter),
            preview_writer: None,
            preview_page_cursor: 0,
            terminal_writer: None,
            terminal_page_cursor: 0,
            checkpoint_writer: None,
            checkpoint_cursor: NumericalPageCursor::new(),
            terminal_published: false, desk: NumericalOutcomeDesk::default(),
        }
    }

    pub fn preview(&self) -> SubspacePreview {
        SubspacePreview {
            iteration: self.state.iteration,
            eigenvalues: self.state.final_theta.iter().take(self.state.p).copied().collect(),
            mode_shapes: (0..self.state.p.min(self.state.final_theta.len()).min(self.state.x.cols)).map(|column| mat_col(&self.state.x, column).0).collect(),
            residuals: self.state.residuals.iter().copied().collect(),
            converged_count: self.state.converged_count,
            converged: self.state.converged,
        }
    }

    pub fn solution(&self) -> EigenPairs {
        EigenPairs { values: self.state.final_theta.iter().take(self.state.p).copied().collect(), vectors: (0..self.state.p.min(self.state.final_theta.len()).min(self.state.x.cols)).map(|column| mat_col(&self.state.x, column)).collect() }
    }

    pub fn spectrum(&self)->SubspaceSpectrum{if !self.state.converged{SubspaceSpectrum::Unresolved}else if self.state.final_theta.is_empty(){SubspaceSpectrum::NoPositive}else{SubspaceSpectrum::Positive{available:self.state.final_theta.len()}}}

    pub fn visual_mode_scalar(&self, mode: usize, index: usize) -> Option<(f64, f64)> {
        let eigenvalue = *self.state.final_theta.get(mode)?;
        (mode < self.state.x.cols && index < self.state.x.rows).then(|| (self.state.x.get(index, mode), eigenvalue))
    }

    pub fn visual_progress(&self) -> (usize, usize, f64, bool) {
        let residual = self.state.residuals.first().copied().unwrap_or(f64::INFINITY);
        (self.state.iteration, self.state.max_iter, residual, self.state.converged)
    }

    pub(super) fn advance_work_checkpoint_entry(work: &SubspaceWork, cursor: &mut NumericalPageCursor, writer: &mut RetainedJobPayloadWriter, base: u16, context: &mut StepContext<'_>) -> Result<bool, ValueError> {
        Ok(match cursor.field - base {
            0 => {
                let values = work.checkpoint_control();
                advance_u64_values(writer, &values, cursor, context)?
            }
            1 => advance_matrix_owner(writer, &work.rhs, cursor, context)?,
            2 => advance_matrix_owner(writer, &work.solved, cursor, context)?,
            3 => advance_matrix_owner(writer, &work.b_basis, cursor, context)?,
            4 => advance_matrix_owner(writer, &work.projected, cursor, context)?,
            5 => advance_matrix_owner(writer, &work.jacobi, cursor, context)?,
            6 => advance_matrix_owner(writer, &work.jacobi_vectors, cursor, context)?,
            7 => advance_matrix_owner(writer, &work.ordered_vectors, cursor, context)?,
            8 => advance_matrix_owner(writer, &work.candidate_x, cursor, context)?,
            9 => advance_paged_f64_owner(writer, &work.mu, cursor, context)?,
            10 => advance_paged_f64_owner(writer, &work.theta, cursor, context)?,
            11 => advance_u64_owner(writer, &work.order, cursor, context)?,
            _ => true,
        })
    }

    pub(super) fn advance_checkpoint_entry(state: &SubspaceCheckpoint, cursor: &mut NumericalPageCursor, writer: &mut RetainedJobPayloadWriter, context: &mut StepContext<'_>) -> Result<bool, ValueError> {
        if advance_numerical_page_header(writer, b"FEMSCP1\0", 12, *cursor, context)? {
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
                    u64::from(state.shift.is_some()),
                    state.shift.unwrap_or(0.0).to_bits(),
                    u64::from(state.eigensystem_certified),
                ];
                advance_u64_values(writer, &values, cursor, context)?
            }
            1 => advance_u64_owner(writer, &state.k_factor.columns.offsets, cursor, context)?,
            2 => advance_paged_pair_owner(writer, &state.k_factor.columns.entries, cursor, context)?,
            3 => advance_paged_f64_owner(writer, &state.k_factor.d, cursor, context)?,
            4 => advance_paged_u32_owner(writer, &state.b.indptr, cursor, context)?,
            5 => advance_paged_u32_owner(writer, &state.b.indices, cursor, context)?,
            6 => advance_paged_f64_owner(writer, &state.b.vals, cursor, context)?,
            7 => advance_matrix_owner(writer, &state.x, cursor, context)?,
            8 => advance_paged_f64_owner(writer, &state.prev_theta, cursor, context)?,
            9 => advance_paged_f64_owner(writer, &state.final_theta, cursor, context)?,
            10 => advance_paged_f64_owner(writer, &state.residuals, cursor, context)?,
            field if (11..=22).contains(&field) => Self::advance_work_checkpoint_entry(&state.work, cursor, writer, 11, context)?,            23 => {
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
            field if (24..=34).contains(&field) => {
                let Some(work) = state.retiring_work.as_ref() else { return Ok(true) };
                Self::advance_work_checkpoint_entry(work, cursor, writer, 23, context)?
            }
            _ => return Ok(true),
        };
        if complete {
            if !writer.commit_staged_page_funded(context, size_of::<NumericalPageCursor>() + 3 * size_of::<usize>())? { return Ok(false); }
            cursor.item = 0;
            cursor.owner = 0;
            cursor.field = match cursor.field {
                field if field <= 22 => field + 1,
                23 if state.retiring_work.is_some() => 24,
                23 => u16::MAX,
                field if (24..34).contains(&field) => field + 1,
                _ => u16::MAX,
            };
        }
        Ok(cursor.field == u16::MAX)
    }

    pub(super) fn reserve_matrix_owner(matrix:&mut semio_s_fem_algebra::PagedMatD,rows:usize,columns:usize,context:&mut StepContext<'_>)->Result<bool,()>{
        let cells=rows.checked_mul(columns).ok_or(())?;
        if matrix.data.capacity()<cells{
            let progress=matrix.data.reserve_capacity_one_funded(cells,context.retained_grant()).map_err(|_|())?;
            context.consume_retained(progress).map_err(|_|())?;
            if progress.copied_items!=0{matrix.rows=rows;matrix.cols=columns;}
            return Ok(false);
        }
        matrix.rows=rows;matrix.cols=columns;Ok(true)
    }

    pub(super) fn initialize_matrix_owner(matrix:&mut semio_s_fem_algebra::PagedMatD,context:&mut StepContext<'_>)->Result<bool,()>{
        let cells=matrix.rows.checked_mul(matrix.cols).ok_or(())?;
        if matrix.data.len()==cells{return Ok(true)}
        let grant=context.retained_grant();let depth=matrix.data.next_push_depth_demand().map_err(|_|())?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<f64>()||grant.maximum_depth<depth{return Ok(false)}
        matrix.data.push_reserved(0.0).map_err(|_|())?;
        context.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<f64>(),..Default::default()}).map_err(|_|())?;Ok(false)
    }

    pub(super) fn reserve_scalar_owner<T>(owner:&mut PagedList<T,SPARSE_INDEX_SPACE>,items:usize,context:&mut StepContext<'_>)->Result<bool,()>{
        if owner.capacity()>=items{return Ok(true)}
        let progress=owner.reserve_capacity_one_funded(items,context.retained_grant()).map_err(|_|())?;
        context.consume_retained(progress).map_err(|_|())?;Ok(false)
    }

    pub(super) fn push_scalar<T>(owner:&mut PagedList<T,SPARSE_INDEX_SPACE>,value:T,context:&mut StepContext<'_>)->Result<bool,()>{
        let grant=context.retained_grant();let depth=owner.next_push_depth_demand().map_err(|_|())?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<T>()||grant.maximum_depth<depth{return Ok(false)}
        owner.push_reserved(value).map_err(|_|())?;
        context.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<T>(),..Default::default()}).map_err(|_|())?;Ok(true)
    }

    pub(super) fn reserve_iteration_owner(&mut self,context:&mut StepContext<'_>) -> Result<(), ()> {
        if let Some(retiring) = self.state.retiring_work.as_mut() {
            let demand=retiring.retirement_demands().map_err(|_|())?;let grant=context.retained_grant();
            if !numerical_grant_covers(grant,demand){return Ok(())}
            let(terminal,items,bytes)=retiring.close_step(grant.maximum_release_bytes);
            context.consume_retained(RetainedCloneProgress{copied_items:items.max(1),released_bytes:bytes,..Default::default()}).map_err(|_|())?;
            if terminal {
                self.state.retiring_work = None;
            }
            return Ok(());
        }
        let n = self.state.n;
        let m = self.state.m;
        let work = &mut self.state.work;
        match work.reserve {
            0 => {if !Self::reserve_matrix_owner(&mut self.state.x, n, m,context)?{return Ok(())}},
            1 => {
                if !Self::initialize_matrix_owner(&mut self.state.x,context)? {
                    return Ok(());
                }
            }
            2 => {
                if self.state.initialization_cursor < n*m {
                    let index=self.state.initialization_cursor;
                    let mut bits=(index as u64).wrapping_add(self.operation.seed).wrapping_add(0x9e3779b97f4a7c15);
                    bits=(bits^(bits>>30)).wrapping_mul(0xbf58476d1ce4e5b9);
                    bits=(bits^(bits>>27)).wrapping_mul(0x94d049bb133111eb);
                    bits^=bits>>31;
                    let grant=context.retained_grant();let depth=self.state.x.data.next_get_depth_demand(index).map_err(|_|())?;
                    if grant.maximum_copy_bytes<size_of::<f64>()||grant.maximum_depth<depth{return Ok(())}                    self.state.x.set(index/m,index%m,if m==n{u8::from(index/m==index%m)as f64}else{((bits>>11)as f64/9007199254740992.0)-0.5});
                    context.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<f64>(),..Default::default()}).map_err(|_|())?;
                    self.state.initialization_cursor += 1;
                    return Ok(());
                }
            }
            3 => {
                if self.state.prev_theta.capacity() < self.state.p {
                    if !Self::reserve_scalar_owner(&mut self.state.prev_theta, self.state.p,context)?{return Ok(())}
                }
            }
            4 => {
                if self.state.prev_theta.len() < self.state.p {
                    Self::push_scalar(&mut self.state.prev_theta,0.0,context)?;
                    return Ok(());
                }
            }
            5 => {
                if self.state.residuals.capacity() < self.state.p {
                    if !Self::reserve_scalar_owner(&mut self.state.residuals, self.state.p,context)?{return Ok(())}
                }
            }
            6 => {
                if self.state.residuals.len() < self.state.p {
                    Self::push_scalar(&mut self.state.residuals,0.0,context)?;
                    return Ok(());
                }
            }
            7 => {
                if self.state.final_theta.capacity() < m {
                    if !Self::reserve_scalar_owner(&mut self.state.final_theta, m,context)?{return Ok(())}
                }
            }
            8 => {}
            9 => {if !Self::reserve_matrix_owner(&mut work.rhs, n, m,context)?{return Ok(())}},
            10 => {
                if !Self::initialize_matrix_owner(&mut work.rhs,context)? {
                    return Ok(());
                }
            }
            11 => {if !Self::reserve_matrix_owner(&mut work.solved, n, m,context)?{return Ok(())}},
            12 => {
                if !Self::initialize_matrix_owner(&mut work.solved,context)? {
                    return Ok(());
                }
            }
            13 => {if !Self::reserve_matrix_owner(&mut work.b_basis, n, m,context)?{return Ok(())}},
            14 => {
                if !Self::initialize_matrix_owner(&mut work.b_basis,context)? {
                    return Ok(());
                }
            }
            15 => {if !Self::reserve_matrix_owner(&mut work.projected, m, m,context)?{return Ok(())}},
            16 => {
                if !Self::initialize_matrix_owner(&mut work.projected,context)? {
                    return Ok(());
                }
            }
            17 => {if !Self::reserve_matrix_owner(&mut work.jacobi, m, m,context)?{return Ok(())}},
            18 => {
                if !Self::initialize_matrix_owner(&mut work.jacobi,context)? {
                    return Ok(());
                }
            }
            19 => {if !Self::reserve_matrix_owner(&mut work.jacobi_vectors, m, m,context)?{return Ok(())}},
            20 => {
                if !Self::initialize_matrix_owner(&mut work.jacobi_vectors,context)? {
                    return Ok(());
                }
            }
            21 => {
                if work.first < m {
                    let grant=context.retained_grant();let depth=work.jacobi_vectors.data.next_get_depth_demand(work.first*m+work.first).map_err(|_|())?;
                    if grant.maximum_copy_bytes<size_of::<f64>()||grant.maximum_depth<depth{return Ok(())}
                    context.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<f64>(),..Default::default()}).map_err(|_|())?;                    work.jacobi_vectors.set(work.first, work.first, 1.0);
                    work.first += 1;
                    return Ok(());
                }
                work.first = 0;
            }
            22 => {if !Self::reserve_matrix_owner(&mut work.ordered_vectors, m, m,context)?{return Ok(())}},
            23 => {
                if !Self::initialize_matrix_owner(&mut work.ordered_vectors,context)? {
                    return Ok(());
                }
            }
            24 => {if !Self::reserve_matrix_owner(&mut work.candidate_x, n, m,context)?{return Ok(())}},
            25 => {
                if !Self::initialize_matrix_owner(&mut work.candidate_x,context)? {
                    return Ok(());
                }
            }
            26 => {if !Self::reserve_scalar_owner(&mut work.mu, m,context)?{return Ok(())}},
            27 => {
                if work.mu.len() < m {
                    Self::push_scalar(&mut work.mu,0.0,context)?;
                    return Ok(());
                }
            }
            28 => {if !Self::reserve_scalar_owner(&mut work.theta, m,context)?{return Ok(())}},
            29 => {
                if work.theta.len() < m {
                    Self::push_scalar(&mut work.theta,0.0,context)?;
                    return Ok(());
                }
            }
            30 => {if !Self::reserve_scalar_owner(&mut work.order, m,context)?{return Ok(())}},
            31 => {
                if work.order.len() < m {
                    let value=work.order.len();Self::push_scalar(&mut work.order,value,context)?;
                    return Ok(());
                }
            }
            32 => {
                if m==n{self.state.shift=Some(0.0);self.state.reset_cursor(SubspaceStage::NormRightRows);}else if self.state.shift.is_none(){self.state.reset_cursor(SubspaceStage::NormInverseInitialize);self.state.work.reserve=0;}else{self.state.reset_cursor(SubspaceStage::ApplyOperatorColumnRow);}
                return Ok(());
            }
            _ => return Err(()),
        }
        work.reserve += 1;
        Ok(())
    }

    pub(super) fn reset_cursor(&mut self, stage: SubspaceStage) {
        self.state.work.stage = stage;
        self.state.work.first = 0;
        self.state.work.second = 0;
        self.state.work.third = 0;
        self.state.work.phase = 0;
        self.state.work.scalar = 0.0;
    }
}
