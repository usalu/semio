//! 🚶️ The bounded iteration steps of the subspace job: operator, factor solves, orthogonalization, Jacobi, sorting, residuals.

use super::*;

impl SubspaceIterationJob {
    pub(super) fn advance_apply_operator(&mut self) {
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

    pub(super) fn advance_factor_forward(&mut self) {
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

    pub(super) fn advance_factor_diagonal(&mut self) {
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

    pub(super) fn advance_factor_backward(&mut self) {
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

    pub(super) fn advance_orthogonalize(&mut self) {
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

    pub(super) fn advance_normalize(&mut self) {
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

    pub(super) fn advance_projected(&mut self) {
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

    pub(super) fn advance_jacobi_convergence(&mut self) {
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

    pub(super) fn advance_jacobi_pair(&mut self) {
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

    pub(super) fn advance_jacobi_rotation(&mut self) {
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

    pub(super) fn advance_mode_sort(&mut self) {
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

    pub(super) fn advance_mode_permute(&mut self) {
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

    pub(super) fn advance_residual(&mut self) {
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

    pub(super) fn advance_convergence(&mut self) {
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

    pub(super) fn advance_preview_entry(state: &mut SubspaceCheckpoint, operation: Operation, writer: &mut RetainedJobPayloadWriter) -> Result<bool, JobPayloadAdmissionFault> {
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

    pub(super) fn publish_iteration(&mut self) {
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

    pub(super) fn close_retained_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        close_subspace_checkpoint_step(&mut self.state, maximum_bytes)
    }

    pub(super) fn close_terminal_is_empty(&self) -> bool {
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
