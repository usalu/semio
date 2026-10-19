//! 🚶️ The bounded iteration steps of the subspace job: operator, factor solves, orthogonalization, Jacobi, sorting, residuals.

use super::*;

impl SubspaceCheckpoint {
    pub(super) fn advance_apply_operator(&mut self) {
        let n = self.n;
        let m = self.m;
        let work = &mut self.work;
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
        let start = self.b.indptr(row).expect("subspace row start") as usize;
        let end = self.b.indptr(row + 1).expect("subspace row end") as usize;
        if work.third < end.saturating_sub(start) {
            let index = start + work.third;
            let source_row = self.b.index(index).expect("subspace column") as usize;
            let source = if using_basis { work.solved.get(source_row, work.first) } else { self.x.get(source_row, work.first) };
            work.scalar += self.b.value(index).expect("subspace value") * source;
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

    pub(super) fn advance_factor_forward(&mut self) {
        let n = self.n;
        let m = self.m;
        let work = &mut self.work;
        if work.first >= if self.shift.is_none(){1}else{m} {
            self.reset_cursor(if self.shift.is_none(){SubspaceStage::NormInverseAccumulate}else{SubspaceStage::ShiftOperatorColumnRow});
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
        if let Some((row,value))=self.k_factor.column_entry(work.second,work.third){
            let delta = value * work.solved.get(work.second, work.first);
            work.solved.add_at(row as usize, work.first, -delta);
            work.third += 1;
        } else {
            work.third = 0;
            work.second += 1;
        }
    }

    pub(super) fn advance_factor_diagonal(&mut self) {
        let work = &mut self.work;
        let value = work.solved.get(work.second, work.first) / self.k_factor.diagonal(work.second).expect("retained elastic diagonal");
        work.solved.set(work.second, work.first, value);
        work.second += 1;
        if work.second == self.n {
            work.second = self.n;
            work.third = 0;
            work.stage = SubspaceStage::FactorBackwardEntry;
        }
    }

    pub(super) fn advance_factor_backward(&mut self) {
        let work = &mut self.work;
        if work.second == 0 {
            work.second = 0;
            work.third = 0;
            work.phase = 0;
            work.first += 1;
            work.stage = SubspaceStage::FactorForwardEntry;
            return;
        }
        let column = work.second - 1;
        if let Some((row,value))=self.k_factor.column_entry(column,work.third){
            let delta = value * work.solved.get(row as usize, work.first);
            work.solved.add_at(column, work.first, -delta);
            work.third += 1;
        } else {
            work.second -= 1;
            work.third = 0;
        }
    }

    pub(super) fn advance_orthogonalize(&mut self) {
        let n = self.n;
        let m = self.m;
        let work = &mut self.work;
        if work.first >= m {
            work.phase = 1;
            self.reset_cursor(SubspaceStage::ApplyOperatorColumnRow);
            self.work.phase = 1;
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

    pub(super) fn advance_normalize(&mut self)->Result<(),()> {
        let n = self.n;
        let work = &mut self.work;
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
                if !work.scalar.is_finite()||work.scalar<=0.0{return Err(())}
                work.coefficient = work.scalar.sqrt().recip();
                work.third = 0;
                work.phase = 1;
            }
            return Ok(());
        }
        if work.phase == 2 {
            let value = self.x.get(work.third, work.first);
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
            if let Some((row,value))=self.k_factor.column_entry(work.second,work.third){
                let delta = value * self.x.get(row as usize, work.first);
                work.b_basis.add_at(work.second, work.first, delta);
                work.third += 1;
            } else {
                work.second += 1;
                work.third = 0;
            }
            return Ok(());
        }
        if work.phase == 4 {
            let value = work.b_basis.get(work.second, work.first) * self.k_factor.diagonal(work.second).expect("retained elastic diagonal");
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
            if let Some((row,value))=self.k_factor.column_entry(work.second,work.third){
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
                if !work.scalar.is_finite()||work.scalar<=0.0{return Err(())}
                work.coefficient = work.scalar.sqrt().recip();
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

    pub(super) fn advance_projected(&mut self) {
        let m = self.m;
        let n = self.n;
        let work = &mut self.work;
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

    pub(super) fn advance_jacobi_convergence(&mut self) {
        let m = self.m;
        let work = &mut self.work;
        if work.first < m * m {
            let row = work.first / m;
            let column = work.first % m;
            let value = work.jacobi.get(row, column);
            work.scalar = work.scalar.hypot(value);
            if row < column {
                work.coefficient = work.coefficient.hypot(value);
            }
            work.first += 1;
            return;
        }
        if work.coefficient <= 1e-12 * work.scalar || work.sweep == 100 {
            self.eigensystem_certified=work.coefficient<=1e-12*work.scalar;
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

    pub(super) fn advance_jacobi_pair(&mut self) {
        let m = self.m;
        let work = &mut self.work;
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
        work.tangent = if theta >= 0.0 { 1.0 / (theta + theta.hypot(1.0)) } else { -1.0 / (-theta + theta.hypot(1.0)) };
        work.cosine = 1.0 / (work.tangent * work.tangent + 1.0).sqrt();
        work.sine = work.tangent * work.cosine;
        work.phase = 0;
        work.third = 0;
        work.stage = SubspaceStage::JacobiRotateCell;
    }

    pub(super) fn advance_jacobi_rotation(&mut self) {
        let m = self.m;
        let work = &mut self.work;
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

    pub(super) fn mode_key(&self, index: usize) -> f64 {
        if self.work.mu[index] > self.work.coefficient {
            self.work.mu[index].recip()
        } else {
            f64::INFINITY
        }
    }

    pub(super) fn advance_mode_sort(&mut self)->Result<(),()> {
        let m = self.m;
        if self.work.phase == 0 {
            let index = self.work.first;
            self.work.mu[index] = self.work.jacobi.get(index, index);
            if !self.work.mu[index].is_finite(){return Err(())}
            self.work.scalar=self.work.scalar.max(self.work.mu[index].abs());
            self.work.order[index] = index;
            self.work.first += 1;
            if self.work.first == m {
                self.work.first = 1;
                self.work.second = 1;
                self.work.phase = 1;
                self.work.coefficient=if self.m==self.n{0.0}else{self.work.scalar*64.0*f64::EPSILON};
            }
            return Ok(());
        }
        if self.work.first >= m {
            self.reset_cursor(SubspaceStage::ModePermuteElement);
            return Ok(());
        }
        let position = self.work.second;
        if position == 0 {
            self.work.first += 1;
            self.work.second = self.work.first;
            return Ok(());
        }
        let left = self.work.order[position - 1];
        let right = self.work.order[position];
        let comparison = self.mode_key(left).total_cmp(&self.mode_key(right)).then_with(|| left.cmp(&right));
        if comparison.is_gt() {
            self.work.order.swap(position - 1, position);
            self.work.second -= 1;
        } else {
            self.work.first += 1;
            self.work.second = self.work.first;
        }
        Ok(())
    }

    pub(super) fn advance_mode_permute(&mut self)->Result<(),()> {
        let m = self.m;
        let n = self.n;
        let work = &mut self.work;
        if work.phase == 0 {
            if work.first < m {
                let source = work.order[work.first];
                work.theta[work.first] = if work.mu[source] > work.coefficient { work.mu[source].recip() } else { 0.0 };
                if !work.theta[work.first].is_finite(){return Err(())}
                work.first += 1;
            } else {
                work.first = 0;
                work.second = 0;
                work.phase = 1;
            }
            return Ok(());
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
            return Ok(());
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
        Ok(())
    }

    pub(super) fn advance_residual(&mut self){
        let n=self.n;let work=&mut self.work;
        if work.first==self.p{work.first=0;self.converged_count=0;work.stage=SubspaceStage::ConvergenceMode;return}
        let mode=work.first;let eigenvalue=work.theta[mode];
        if eigenvalue<=0.0{self.residuals[mode]=0.0;work.first+=1;return}
        match work.phase{
            0=>{work.rhs.set(work.second,mode,work.candidate_x.get(work.second,mode));work.second+=1;if work.second==n{work.second=0;work.third=0;work.phase=1;}},
            1=>{if work.second==n{work.second=0;work.phase=2;return}if let Some((row,value))=self.k_factor.column_entry(work.second,work.third){work.rhs.add_at(work.second,mode,value*work.candidate_x.get(row as usize,mode));work.third+=1;}else{work.second+=1;work.third=0;}},
            2=>{let value=work.rhs.get(work.second,mode)*self.k_factor.diagonal(work.second).expect("retained elastic diagonal");work.rhs.set(work.second,mode,value);work.solved.set(work.second,mode,value);work.second+=1;if work.second==n{work.second=0;work.third=0;work.phase=3;}},
            3=>{if work.second==n{work.second=0;work.phase=4;return}if let Some((row,value))=self.k_factor.column_entry(work.second,work.third){work.solved.add_at(row as usize,mode,value*work.rhs.get(work.second,mode));work.third+=1;}else{work.second+=1;work.third=0;}},
            4=>{if work.second==n{work.second=0;work.scalar=0.0;work.coefficient=0.0;work.cosine=0.0;work.phase=5;return}let start=self.b.indptr(work.second).unwrap()as usize;let end=self.b.indptr(work.second+1).unwrap()as usize;if start+work.third<end{let index=start+work.third;let row=self.b.index(index).unwrap()as usize;work.scalar+=self.b.value(index).unwrap()*work.candidate_x.get(row,mode);work.third+=1;}else{work.b_basis.set(work.second,mode,work.scalar);work.second+=1;work.third=0;work.scalar=0.0;}},
            _=>{if work.second<n{let kx=work.solved.get(work.second,mode);let bx=work.b_basis.get(work.second,mode);work.scalar=work.scalar.hypot(kx-eigenvalue*bx);work.coefficient=work.coefficient.hypot(kx);work.cosine=work.cosine.hypot(bx);work.second+=1;}else{let scale=work.coefficient+eigenvalue.abs()*work.cosine;self.residuals[mode]=if scale>0.0{work.scalar/scale}else{0.0};work.first+=1;work.second=0;work.third=0;work.phase=0;work.scalar=0.0;}}
        }
    }

    pub(super) fn advance_convergence(&mut self) {
        let work = &mut self.work;
        if work.first < self.p {
            if work.theta[work.first]>0.0&&self.residuals[work.first] < 1e-6 {
                self.converged_count += 1;
            }
            self.prev_theta[work.first] = work.theta[work.first];
            work.first += 1;
        } else {
            self.reset_cursor(if self.m==self.n&&self.work.theta[0]<=0.0&&self.eigensystem_certified{SubspaceStage::CertifyNonPositive}else{SubspaceStage::PublishIteration});
        }
    }

    pub(super) fn advance_non_positive_certificate(&mut self)->Result<(),()>{
        let n=self.n;let work=&mut self.work;
        match work.phase{
            0=>{if work.first<n*n{work.projected.set(work.first/n,work.first%n,0.0);work.first+=1;}else{work.first=0;work.phase=1;}},
            1=>{if work.first==n{work.first=0;work.second=0;work.third=0;work.phase=2;return Ok(())}let start=self.b.indptr(work.first).ok_or(())?as usize;let end=self.b.indptr(work.first+1).ok_or(())?as usize;if start+work.third<end{let index=start+work.third;let row=self.b.index(index).ok_or(())?as usize;work.projected.set(work.first,row,-self.b.value(index).ok_or(())?);work.third+=1;}else{work.first+=1;work.third=0;}},
            2=>{if work.first==n{self.reset_cursor(SubspaceStage::PublishIteration);return Ok(())}work.scalar=work.projected.get(work.first,work.first);work.third=0;work.phase=3;},
            3=>{if work.third<work.first{let l=work.projected.get(work.first,work.third);work.scalar-=l*l*work.projected.get(work.third,work.third);work.third+=1;}else{if !work.scalar.is_finite()||work.scalar<0.0{return Err(())}work.projected.set(work.first,work.first,work.scalar);work.second=work.first+1;work.phase=4;}},
            4=>{if work.second==n{work.first+=1;work.phase=2;}else{work.scalar=work.projected.get(work.second,work.first);work.third=0;work.phase=5;}},
            _=>{if work.third<work.first{work.scalar-=work.projected.get(work.second,work.third)*work.projected.get(work.third,work.third)*work.projected.get(work.first,work.third);work.third+=1;}else{let pivot=work.projected.get(work.first,work.first);if !work.scalar.is_finite()||pivot==0.0&&work.scalar!=0.0{return Err(())}let value=if pivot==0.0{0.0}else{work.scalar/pivot};if !value.is_finite(){return Err(())}work.projected.set(work.second,work.first,value);work.second+=1;work.phase=4;}}
        }Ok(())
    }

    pub(super) fn reserve_mathematical(&mut self) {
        use semio_s_fem_algebra::PagedMatD;
        let n = self.n;
        let m = self.m;
        if self.x.rows != n || self.x.cols != m { self.x = PagedMatD::zeros(n, m); }
        while self.initialization_cursor < n * m {
            let index = self.initialization_cursor;
            let mut bits = (index as u64).wrapping_add(self.identity.seed).wrapping_add(0x9e3779b97f4a7c15);
            bits = (bits ^ (bits >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            bits = (bits ^ (bits >> 27)).wrapping_mul(0x94d049bb133111eb);
            bits ^= bits >> 31;
            self.x.set(index / m, index % m, if m == n { u8::from(index / m == index % m) as f64 } else { ((bits >> 11) as f64 / 9007199254740992.0) - 0.5 });
            self.initialization_cursor += 1;
        }
        while self.prev_theta.len() < self.p { self.prev_theta.push(0.0); }
        while self.residuals.len() < self.p { self.residuals.push(0.0); }
        self.work.rhs = PagedMatD::zeros(n, m);
        self.work.solved = PagedMatD::zeros(n, m);
        self.work.b_basis = PagedMatD::zeros(n, m);
        self.work.projected = PagedMatD::zeros(m, m);
        self.work.jacobi = PagedMatD::zeros(m, m);
        self.work.jacobi_vectors = PagedMatD::zeros(m, m);
        self.work.ordered_vectors = PagedMatD::zeros(m, m);
        self.work.candidate_x = PagedMatD::zeros(n, m);
        for column in 0..m { self.work.jacobi_vectors.set(column, column, 1.0); }
        self.work.mu = (0..m).map(|_| 0.0).collect();
        self.work.theta = (0..m).map(|_| 0.0).collect();
        self.work.order = (0..m).collect();
        if m == n { self.shift = Some(0.0); self.reset_cursor(SubspaceStage::NormRightRows); }
        else if self.shift.is_none() { self.reset_cursor(SubspaceStage::NormInverseInitialize); self.work.reserve = 0; }
        else { self.reset_cursor(SubspaceStage::ApplyOperatorColumnRow); }
    }

    pub(super) fn publish_mathematical(&mut self) {
        self.final_theta = self.work.theta.iter().copied().filter(|value| *value > 0.0 && value.is_finite()).collect();
        std::mem::swap(&mut self.x, &mut self.work.candidate_x);
        self.iteration += 1;
        self.converged = self.eigensystem_certified && self.converged_count == self.p.min(self.final_theta.len()) && (self.final_theta.len() >= self.p || self.m == self.n);
        self.work = SubspaceWork::empty();
    }

    pub(super) fn advance_mathematical(&mut self) -> Result<(), ()> {
        match self.work.stage {
            SubspaceStage::ReserveIteration => self.reserve_mathematical(),
            SubspaceStage::ApplyOperatorColumnRow => self.advance_apply_operator(),
            SubspaceStage::FactorForwardEntry => self.advance_factor_forward(),
            SubspaceStage::FactorDiagonalEntry => self.advance_factor_diagonal(),
            SubspaceStage::FactorBackwardEntry => self.advance_factor_backward(),
            SubspaceStage::OrthogonalizePairElement => self.advance_orthogonalize(),
            SubspaceStage::NormalizeColumnElement => self.advance_normalize()?,
            SubspaceStage::ProjectedMatrixCellEntry => self.advance_projected(),
            SubspaceStage::JacobiFindPairCell => self.advance_jacobi_pair(),
            SubspaceStage::JacobiRotateCell => self.advance_jacobi_rotation(),
            SubspaceStage::JacobiConvergenceCell => self.advance_jacobi_convergence(),
            SubspaceStage::ModeSortCompare => self.advance_mode_sort()?,
            SubspaceStage::ModePermuteElement => self.advance_mode_permute()?,
            SubspaceStage::ResidualColumnRow => self.advance_residual(),
            SubspaceStage::ConvergenceMode => self.advance_convergence(),
            SubspaceStage::PublishIteration => self.publish_mathematical(),
            SubspaceStage::NormInverseInitialize => self.advance_inverse_norm_initialize(),
            SubspaceStage::NormInverseAccumulate => self.advance_inverse_norm_accumulate(),
            SubspaceStage::NormRightRows => self.advance_right_norm()?,
            SubspaceStage::ShiftOperatorColumnRow => self.advance_shift_operator(),
            SubspaceStage::CertifyNonPositive => self.advance_non_positive_certificate()?,
        }
        Ok(())
    }
}

impl SubspaceIterationJob {
    pub(super) fn arithmetic_depth_demand(&self)->Result<usize,()> {
        fn depth<T,const N:usize>(owner:&PagedList<T,N>)->Result<usize,()>{owner.next_get_depth_demand(owner.len().saturating_sub(1)).map_err(|_|())}
        let state=&self.state;let work=&state.work;let mut result=1;
        for owner in [&state.x.data,&work.rhs.data,&work.solved.data,&work.b_basis.data,&work.projected.data,&work.jacobi.data,&work.jacobi_vectors.data,&work.ordered_vectors.data,&work.candidate_x.data]{result=result.max(depth(owner)?+1)}
        for owner in [&state.prev_theta,&state.final_theta,&state.residuals,&work.mu,&work.theta]{result=result.max(depth(owner)?+1)}
        result=result.max(depth(&work.order)?+1).max(depth(&state.b.indptr)?+1).max(depth(&state.b.indices)?+1).max(depth(&state.b.vals)?+1);
        let column=if work.stage==SubspaceStage::FactorBackwardEntry{work.second.saturating_sub(1)}else{work.second}.min(state.n-1);
        result=result.max(state.k_factor.diagonal_depth_demand(column).map_err(|_|())?+1);
        result=result.max(state.k_factor.column_entry_depth_demand(column,work.third.min(state.k_factor.column_entries_len(column).ok_or(())?)).map_err(|_|())?+1);
        Ok(result)
    }



    pub(super) fn advance_preview_entry(state: &mut SubspaceCheckpoint, operation: Operation, writer: &mut RetainedJobPayloadWriter, context: &mut StepContext<'_>) -> Result<bool, ValueError> {
        let published=state.p.min(state.final_theta.len());
        let cursor = NumericalPageCursor { field: state.publication_stage as u16, owner: state.publication_first, item: state.publication_second };
        if advance_numerical_page_header(writer, b"FEMSUB2\0", 2, cursor, context)? {
            return Ok(false);
        }
        match state.publication_stage {
            0 => {
                let values = [operation.operation.0, operation.base_revision.0, operation.generation.0, operation.seed, state.iteration as u64, state.converged_count as u64, state.converged as u64, state.n as u64, published as u64];
                if let Some(value) = values.get(state.publication_first) {
                    writer.write_staged(&value.to_le_bytes())?;
                    state.publication_first += 1;
                } else {
                    state.publication_first = 0;
                    state.publication_stage = 1;
                }
            }
            1 => {
                if let Some(value) = state.final_theta.get(state.publication_first).filter(|_| state.publication_first < published) {
                    writer.write_staged(&value.to_bits().to_le_bytes())?;
                    state.publication_first += 1;
                } else {
                    state.publication_first = 0;
                    state.publication_stage = 2;
                }
            }
            2 => {
                if state.publication_first < published {
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
                if let Some(value) = state.residuals.get(state.publication_first).filter(|_| state.publication_first < published) {
                    writer.write_staged(&value.to_bits().to_le_bytes())?;
                    state.publication_first += 1;
                } else {
                    state.publication_stage = 4;
                }
            }
            4 => { if !writer.commit_staged_page_funded(context, 3 * size_of::<usize>())? { return Ok(false); } },
            _ => {}
        }
        Ok(state.publication_stage == 4 && writer.staged_page_len().is_none())
    }

    pub(super) fn publish_iteration(&mut self,context:&mut StepContext<'_>)->Result<(),()> {
        if self.state.work.phase == 0 {
            if self.state.work.first == 0&&!self.state.final_theta.is_empty() {
                let grant=context.retained_grant();let depth=self.state.final_theta.next_pop_depth_demand().map_err(|_|())?;
                if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<f64>()||grant.maximum_depth<depth{return Ok(())}
                self.state.final_theta.pop();context.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<f64>(),..Default::default()}).map_err(|_|())?;
                return Ok(());
            }
            if self.state.work.first < self.state.m {
                let value=self.state.work.theta[self.state.work.first];
                if value>0.0&&value.is_finite()&&!Self::push_scalar(&mut self.state.final_theta,value,context)?{return Ok(())}
                self.state.work.first += 1;
                return Ok(());
            }
            self.state.work.phase = 1;
            return Ok(());
        }
        let copy=2*size_of::<semio_s_fem_algebra::PagedMatD>()+size_of::<SubspaceWork>();let grant=context.retained_grant();
        if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(())}
        std::mem::swap(&mut self.state.x, &mut self.state.work.candidate_x);
        self.state.iteration += 1;
        self.state.converged = self.state.eigensystem_certified&&self.state.converged_count==self.state.p.min(self.state.final_theta.len())&&(self.state.final_theta.len()>=self.state.p||self.state.m==self.state.n);
        self.state.checkpoint_due = true;
        self.state.preview_due = true;
        self.state.publication_stage = 0;
        self.state.publication_first = 0;
        self.state.publication_second = 0;
        let displaced = std::mem::replace(&mut self.state.work, SubspaceWork::empty());
        self.state.retiring_work = Some(displaced);
        context.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}).map_err(|_|())?;Ok(())
    }

    pub(super) fn close_retained_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        close_subspace_checkpoint_step(&mut self.state, maximum_bytes)
    }

    pub(super) fn close_terminal_is_empty(&self) -> bool {
        self.state.k_factor.terminal_is_empty()
            && self.state.b.terminal_is_empty()
            && self.state.x.data.terminal_is_empty()
            && self.state.prev_theta.terminal_is_empty()
            && self.state.final_theta.terminal_is_empty()
            && self.state.residuals.terminal_is_empty()
            && self.state.work.terminal_is_empty()
            && self.state.retiring_work.is_none()
    }
}
