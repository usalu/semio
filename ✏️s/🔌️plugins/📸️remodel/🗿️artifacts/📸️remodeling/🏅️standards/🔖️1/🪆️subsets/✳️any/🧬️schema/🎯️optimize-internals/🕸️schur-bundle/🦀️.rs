//! 🕸️ The Schur-complement bundle solver: bipartite residual terms, the accumulator, the damped step, covariances and the resumable preparation.

use super::*;

// #region 🔖️SchurBundle
/// 🌉️ One residual term in a bipartite least-squares problem: touches an "A" block, a "B" block, or
/// both (a pure prior on one side is expressed by leaving the other index `None`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct ResidualTerm {
    pub a_index: Option<usize>,
    pub b_index: Option<usize>,
    pub dim: usize,
}

/// 🌉️ A bundle-adjustment-shaped bipartite least-squares problem: many small "A" blocks (e.g. cameras)
/// and many small "B" blocks (e.g. points), where every residual term touches at most one A block and
/// one B block — the structure [`schur_lm`] exploits to eliminate the B blocks analytically. Generic
/// over what A/B actually are; camera- or `SE3`-specific parameterizations belong in a dependent crate.
pub trait BipartiteResiduals {
    fn num_a_blocks(&self) -> usize;
    fn num_b_blocks(&self) -> usize;
    fn a_block_dim(&self) -> usize;
    fn b_block_dim(&self) -> usize;
    fn residual_terms(&self) -> &[ResidualTerm];
    fn evaluate(&self, a_params: &[VecD], b_params: &[VecD], term: &ResidualTerm) -> (VecD, MatD, MatD);
}

/// 📦️ Outcome of [`schur_lm`]: refined A/B blocks, final cost, and the marginal covariance diagonal
/// (one block per A-index) recoverable via [`camera_covariances`].
#[derive(Clone, Debug)]
pub struct SchurResult {
    pub a_params: Vec<VecD>,
    pub b_params: Vec<VecD>,
    pub cost: f64,
    pub iterations: usize,
    pub converged: bool,
    pub a_block_covariance_diagonals: Vec<MatD>,
    /// 🎚️ The damping the solver ended on, so a caller that spends one iteration per call can
    /// resume the schedule where it left off instead of restarting from [`LmConfig::initial_lambda`].
    pub lambda: f64,
}

/// 🧮️ Accumulates `Jᵀ W J` into `target` (`target` sized `j.cols x j.cols`) for a single term's block Jacobian.
fn add_weighted_gram(target: &mut MatD, j: &MatD, w: f64) {
    for row in 0..j.rows {
        for a in 0..j.cols {
            let ja = j.get(row, a);
            if ja == 0.0 {
                continue;
            }
            for b in 0..j.cols {
                target.add_at(a, b, w * ja * j.get(row, b));
            }
        }
    }
}

/// 🧮️ Accumulates the `ja^T W jb` cross block (sized `ja.cols x jb.cols`) for a term touching both sides.
fn add_weighted_cross(target: &mut MatD, ja: &MatD, jb: &MatD, w: f64) {
    for row in 0..ja.rows {
        for a in 0..ja.cols {
            let va = ja.get(row, a);
            if va == 0.0 {
                continue;
            }
            for b in 0..jb.cols {
                target.add_at(a, b, w * va * jb.get(row, b));
            }
        }
    }
}

/// 🧮️ Accumulates `Jᵀ W r` into `target`.
fn add_weighted_jt_r(target: &mut VecD, j: &MatD, r: &VecD, w: f64) {
    for row in 0..j.rows {
        let rw = w * r.get(row);
        for a in 0..j.cols {
            target.add_at(a, rw * j.get(row, a));
        }
    }
}

/// 🧮️ One robustified pass over every residual term, accumulating the block-diagonal `Haa`/`Hbb`
/// Hessian pieces, the sparse `Hab` cross blocks, the gradient pieces, and the total cost.
#[allow(clippy::type_complexity, reason = "the block Hessian pieces are each a distinct accumulator with their own shape; bundling them into a struct would just rename the same five return values without reducing what the caller needs to unpack")]
fn accumulate_bipartite(problem: &impl BipartiteResiduals, a_params: &[VecD], b_params: &[VecD], loss: &RobustLoss) -> (Vec<MatD>, Vec<MatD>, HashMap<(usize, usize), MatD>, Vec<VecD>, Vec<VecD>, f64) {
    let ad = problem.a_block_dim();
    let bd = problem.b_block_dim();
    let num_a = problem.num_a_blocks();
    let num_b = problem.num_b_blocks();
    let mut haa: Vec<MatD> = (0..num_a).map(|_| MatD::zeros(ad, ad)).collect();
    let mut hbb: Vec<MatD> = (0..num_b).map(|_| MatD::zeros(bd, bd)).collect();
    let mut hab: HashMap<(usize, usize), MatD> = HashMap::new();
    let mut ga: Vec<VecD> = (0..num_a).map(|_| VecD::zeros(ad)).collect();
    let mut gb: Vec<VecD> = (0..num_b).map(|_| VecD::zeros(bd)).collect();
    let mut cost = 0.0;
    for term in problem.residual_terms() {
        let (r, ja, jb) = problem.evaluate(a_params, b_params, term);
        let r2 = r.dot(&r);
        let w = loss.weight(r2);
        cost += loss.rho(r2);
        if let Some(ai) = term.a_index {
            add_weighted_gram(&mut haa[ai], &ja, w);
            add_weighted_jt_r(&mut ga[ai], &ja, &r, w);
        }
        if let Some(bi) = term.b_index {
            add_weighted_gram(&mut hbb[bi], &jb, w);
            add_weighted_jt_r(&mut gb[bi], &jb, &r, w);
        }
        if let (Some(ai), Some(bi)) = (term.a_index, term.b_index) {
            let entry = hab.entry((ai, bi)).or_insert_with(|| MatD::zeros(ad, bd));
            add_weighted_cross(entry, &ja, &jb, w);
        }
    }
    (haa, hbb, hab, ga, gb, cost)
}

/// 🧮️ One damped Schur-complement LM step: eliminates every B block analytically (each `Hbb` is small
/// and block-diagonal, coupling only to the A blocks it touches), solves the dense reduced camera-only
/// system, then back-substitutes for the B-block updates. Returns `None` if any linear solve fails.
#[allow(
    clippy::too_many_arguments,
    reason = "each argument is a distinct accumulator (per-block Hessian/gradient pieces plus the damping scalar) produced by accumulate_bipartite; bundling them into a struct would just rename this same data without reducing what the function needs"
)]
fn schur_step(ad: usize, num_a: usize, haa: &[MatD], hbb: &[MatD], hab: &HashMap<(usize, usize), MatD>, ga: &[VecD], gb: &[VecD], lambda: f64) -> Option<(Vec<VecD>, Vec<VecD>, MatD)> {
    let num_b = hbb.len();
    let total_a = num_a * ad;
    let mut reduced_h = MatD::zeros(total_a, total_a);
    let mut reduced_g = VecD::zeros(total_a);
    for ai in 0..num_a {
        for row in 0..ad {
            for col in 0..ad {
                let mut value = haa[ai].get(row, col);
                if row == col {
                    value += lambda * haa[ai].get(row, row).max(1e-12);
                }
                reduced_h.add_at(ai * ad + row, ai * ad + col, value);
            }
            reduced_g.add_at(ai * ad + row, -ga[ai].get(row));
        }
    }
    let mut touching: Vec<Vec<usize>> = vec![Vec::new(); num_b];
    for &(ai, bi) in hab.keys() {
        touching[bi].push(ai);
    }
    // 🎲️ The hash map's key order is random per process; the reduction below accumulates in this
    // order, so without sorting two runs of one problem differ at rounding level — and a
    // reconstruction that adjusts its bundle then meshes differently from one run to the next.
    for cameras in &mut touching {
        cameras.sort_unstable();
    }
    let mut hbb_damped: Vec<MatD> = Vec::with_capacity(num_b);
    for bi in 0..num_b {
        let bd = hbb[bi].rows;
        let mut damped = hbb[bi].clone();
        for k in 0..bd {
            let diag = hbb[bi].get(k, k).max(1e-12);
            damped.add_at(k, k, lambda * diag);
        }
        let solved_gb = solve_normal_equations(&damped, &gb[bi])?;
        for &ai in &touching[bi] {
            let hab_ai = hab.get(&(ai, bi)).expect("ai gathered from this hab's own keys");
            let contrib = hab_ai.mul_vec(&solved_gb);
            for row in 0..ad {
                reduced_g.add_at(ai * ad + row, contrib.get(row));
            }
        }
        let mut m_by_a: Vec<(usize, MatD)> = Vec::with_capacity(touching[bi].len());
        for &aj in &touching[bi] {
            let hab_aj_t = hab.get(&(aj, bi)).expect("aj gathered from this hab's own keys").transpose();
            let mut m = MatD::zeros(bd, ad);
            for col in 0..ad {
                let rhs: Vec<f64> = (0..bd).map(|r| hab_aj_t.get(r, col)).collect();
                let solved = solve_normal_equations(&damped, &VecD::from_vec(rhs))?;
                for r in 0..bd {
                    m.set(r, col, solved.get(r));
                }
            }
            m_by_a.push((aj, m));
        }
        for &ai in &touching[bi] {
            let hab_ai = hab.get(&(ai, bi)).expect("ai gathered from this hab's own keys");
            for (aj, m) in &m_by_a {
                let block = hab_ai.matmul(m);
                for row in 0..ad {
                    for col in 0..ad {
                        reduced_h.add_at(ai * ad + row, aj * ad + col, -block.get(row, col));
                    }
                }
            }
        }
        hbb_damped.push(damped);
    }
    let dx_full = solve_normal_equations(&reduced_h, &reduced_g)?;
    let da: Vec<VecD> = (0..num_a).map(|ai| VecD::from_vec((0..ad).map(|row| dx_full.get(ai * ad + row)).collect())).collect();
    let mut db: Vec<VecD> = Vec::with_capacity(num_b);
    for bi in 0..num_b {
        let mut rhs = gb[bi].scale(-1.0);
        for &ai in &touching[bi] {
            let hab_ai_t = hab.get(&(ai, bi)).expect("ai gathered from this hab's own keys").transpose();
            rhs = rhs.sub(&hab_ai_t.mul_vec(&da[ai]));
        }
        db.push(solve_normal_equations(&hbb_damped[bi], &rhs)?);
    }
    Some((da, db, reduced_h))
}

/// 🧮️ Marginal covariance diagonal blocks: inverts the dense reduced system once (column-by-column
/// solves) and slices out each A block's own `ad x ad` diagonal submatrix.
fn covariance_diagonals(h: &MatD, ad: usize, num_a: usize) -> Vec<MatD> {
    let total = h.rows;
    let mut inv = MatD::zeros(total, total);
    for col in 0..total {
        let mut e = VecD::zeros(total);
        e.set(col, 1.0);
        if let Some(x) = solve_normal_equations(h, &e) {
            for row in 0..total {
                inv.set(row, col, x.get(row));
            }
        }
    }
    (0..num_a)
        .map(|ai| {
            let mut block = MatD::zeros(ad, ad);
            for row in 0..ad {
                for col in 0..ad {
                    block.set(row, col, inv.get(ai * ad + row, ai * ad + col));
                }
            }
            block
        })
        .collect()
}

/// 🎯️ Levenberg-Marquardt over a [`BipartiteResiduals`] problem via Schur-complement elimination of
/// the B blocks each iteration; algebraically exact (not an approximation of the dense normal
/// equations) and Nielsen accept/reject exactly as [`levenberg_marquardt`]. Both A and B updates are
/// applied via plain elementwise addition — this generic solver has no notion of a manifold retraction.
pub fn schur_lm(problem: &impl BipartiteResiduals, a0: Vec<VecD>, b0: Vec<VecD>, cfg: &LmConfig) -> SchurResult {
    let ad = problem.a_block_dim();
    let num_a = a0.len();
    let mut a_params = a0;
    let mut b_params = b0;
    let (mut haa, mut hbb, mut hab, mut ga, mut gb, mut cost) = accumulate_bipartite(problem, &a_params, &b_params, &cfg.loss);
    let mut lambda = cfg.initial_lambda;
    let mut nu = 2.0_f64;
    let mut iterations = 0usize;
    let grad_inf = |ga: &[VecD], gb: &[VecD]| -> f64 { ga.iter().map(VecD::norm_inf).fold(0.0_f64, f64::max).max(gb.iter().map(VecD::norm_inf).fold(0.0_f64, f64::max)) };
    let mut converged = grad_inf(&ga, &gb) <= cfg.tol_grad;
    let mut last_reduced_h: Option<MatD> = None;
    while !converged && iterations < cfg.max_iters {
        iterations += 1;
        let Some((da, db, reduced_h)) = schur_step(ad, num_a, &haa, &hbb, &hab, &ga, &gb, lambda) else {
            lambda *= nu;
            nu *= 2.0;
            if lambda > 1e15 {
                break;
            }
            continue;
        };
        let dx_norm = (da.iter().map(|v| v.dot(v)).sum::<f64>() + db.iter().map(|v| v.dot(v)).sum::<f64>()).sqrt();
        if dx_norm <= cfg.tol_dx {
            converged = true;
            last_reduced_h = Some(reduced_h);
            break;
        }
        let a_new: Vec<VecD> = a_params.iter().zip(da.iter()).map(|(a, d)| a.add(d)).collect();
        let b_new: Vec<VecD> = b_params.iter().zip(db.iter()).map(|(b, d)| b.add(d)).collect();
        let (haa_new, hbb_new, hab_new, ga_new, gb_new, cost_new) = accumulate_bipartite(problem, &a_new, &b_new, &cfg.loss);
        let predicted = 0.5 * (haa.iter().zip(da.iter()).zip(ga.iter()).map(|((h, d), g)| quadratic_gain(h, g, d, lambda)).sum::<f64>() + hbb.iter().zip(db.iter()).zip(gb.iter()).map(|((h, d), g)| quadratic_gain(h, g, d, lambda)).sum::<f64>());
        let actual = cost - cost_new;
        let rho = if predicted.abs() < 1e-300 { 0.0 } else { actual / predicted };
        if rho > 0.0 {
            a_params = a_new;
            b_params = b_new;
            haa = haa_new;
            hbb = hbb_new;
            hab = hab_new;
            ga = ga_new;
            gb = gb_new;
            cost = cost_new;
            lambda *= (1.0 - (2.0 * rho - 1.0).powi(3)).max(1.0 / 3.0);
            nu = 2.0;
            last_reduced_h = Some(reduced_h);
            if grad_inf(&ga, &gb) <= cfg.tol_grad {
                converged = true;
            }
        } else {
            lambda *= nu;
            nu *= 2.0;
        }
    }
    let final_reduced_h = last_reduced_h.or_else(|| schur_step(ad, num_a, &haa, &hbb, &hab, &ga, &gb, 0.0).map(|(_, _, h)| h));
    let covariances = final_reduced_h.map_or_else(Vec::new, |h| covariance_diagonals(&h, ad, num_a));
    SchurResult { a_params, b_params, cost, iterations, converged, a_block_covariance_diagonals: covariances, lambda }
}

/// 📊️ The marginal covariance diagonal blocks computed during `result`'s final accepted iteration.
pub fn camera_covariances(result: &SchurResult) -> &[MatD] {
    &result.a_block_covariance_diagonals
}

/// 🧮️ [`accumulate_bipartite`] spread over calls: `Jᵀ W J` and `Jᵀ W r` per block plus the robust
/// cost, `term_budget` residual terms at a time, so a worker step never evaluates more terms than
/// its budget however large the problem.
#[derive(semio_framework_value::RetireOwned)]
pub struct BipartiteAccumulator {
    haa: Vec<MatD>,
    hbb: Vec<MatD>,
    hab: HashMap<(usize, usize), MatD>,
    ga: Vec<VecD>,
    gb: Vec<VecD>,
    pub(super) cost: f64,
    cursor: usize,
}

impl BipartiteAccumulator {
    pub(super) fn new(problem: &impl BipartiteResiduals) -> Self {
        let (ad, bd) = (problem.a_block_dim(), problem.b_block_dim());
        Self {
            haa: (0..problem.num_a_blocks()).map(|_| MatD::zeros(ad, ad)).collect(),
            hbb: (0..problem.num_b_blocks()).map(|_| MatD::zeros(bd, bd)).collect(),
            hab: HashMap::new(),
            ga: (0..problem.num_a_blocks()).map(|_| VecD::zeros(ad)).collect(),
            gb: (0..problem.num_b_blocks()).map(|_| VecD::zeros(bd)).collect(),
            cost: 0.0,
            cursor: 0,
        }
    }

    /// `true` once every term is accumulated.
    fn advance(&mut self, problem: &impl BipartiteResiduals, a_params: &[VecD], b_params: &[VecD], loss: &RobustLoss, term_budget: usize) -> bool {
        let (ad, bd) = (problem.a_block_dim(), problem.b_block_dim());
        let terms = problem.residual_terms();
        let end = self.cursor.saturating_add(term_budget.max(1)).min(terms.len());
        for term in &terms[self.cursor..end] {
            let (r, ja, jb) = problem.evaluate(a_params, b_params, term);
            let r2 = r.dot(&r);
            let w = loss.weight(r2);
            self.cost += loss.rho(r2);
            if let Some(ai) = term.a_index {
                add_weighted_gram(&mut self.haa[ai], &ja, w);
                add_weighted_jt_r(&mut self.ga[ai], &ja, &r, w);
            }
            if let Some(bi) = term.b_index {
                add_weighted_gram(&mut self.hbb[bi], &jb, w);
                add_weighted_jt_r(&mut self.gb[bi], &jb, &r, w);
            }
            if let (Some(ai), Some(bi)) = (term.a_index, term.b_index) {
                let entry = self.hab.entry((ai, bi)).or_insert_with(|| MatD::zeros(ad, bd));
                add_weighted_cross(entry, &ja, &jb, w);
            }
        }
        self.cursor = end;
        end == terms.len()
    }

    fn gradient_infinity_norm(&self) -> f64 {
        self.ga.iter().map(VecD::norm_inf).fold(0.0_f64, f64::max).max(self.gb.iter().map(VecD::norm_inf).fold(0.0_f64, f64::max))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum SchurLmPhase {
    /// Accumulating the normal equations at the current parameters.
    Base,
    /// One damped Schur step at the current damping.
    Solve,
    /// Accumulating the normal equations at the candidate parameters, to accept or reject the step.
    Candidate,
    Done,
}

/// 🎯️ [`schur_lm`] spread over worker steps: the same Nielsen-damped Gauss–Newton iterations, with
/// the two full passes over the residual terms that each iteration needs (the normal equations at
/// the current and at the candidate parameters) accumulated `term_budget` terms per call and the
/// reduced camera solve as its own call. An accepted candidate's accumulation becomes the next
/// iteration's base, so an accepted iteration costs one pass over the terms, a rejected one the
/// candidate pass only. A 36-camera / 1 800-point bundle whose single iteration ran 5 s in one
/// unit of a debug build now spends ~90 units of a few milliseconds.
#[derive(semio_framework_value::RetireOwned)]
pub struct SchurLmPreparation {
    pub a_params: Vec<VecD>,
    pub b_params: Vec<VecD>,
    pub(super) lambda: f64,
    nu: f64,
    pub(super) iterations: usize,
    pub(super) converged: bool,
    phase: SchurLmPhase,
    base: BipartiteAccumulator,
    pub(super) candidate: Option<(Vec<VecD>, Vec<VecD>, Vec<VecD>, Vec<VecD>, BipartiteAccumulator)>,
}

impl SchurLmPreparation {
    pub fn new(problem: &impl BipartiteResiduals, a0: Vec<VecD>, b0: Vec<VecD>, cfg: &LmConfig) -> Self {
        Self { a_params: a0, b_params: b0, lambda: cfg.initial_lambda, nu: 2.0, iterations: 0, converged: false, phase: SchurLmPhase::Base, base: BipartiteAccumulator::new(problem), candidate: None }
    }

    /// 🎚️ The damping the schedule stands at.
    pub fn lambda(&self) -> f64 {
        self.lambda
    }

    pub fn converged(&self) -> bool {
        self.converged
    }

    pub fn iterations(&self) -> usize {
        self.iterations
    }

    /// One bounded unit; `true` once the adjustment is done (converged, out of iterations, or the
    /// damping exhausted).
    pub fn advance(&mut self, problem: &impl BipartiteResiduals, cfg: &LmConfig, term_budget: usize) -> bool {
        let ad = problem.a_block_dim();
        let num_a = self.a_params.len();
        match self.phase {
            SchurLmPhase::Base => {
                if self.base.advance(problem, &self.a_params, &self.b_params, &cfg.loss, term_budget) {
                    if self.base.gradient_infinity_norm() <= cfg.tol_grad {
                        self.converged = true;
                        self.phase = SchurLmPhase::Done;
                    } else {
                        self.phase = SchurLmPhase::Solve;
                    }
                }
            }
            SchurLmPhase::Solve => {
                if self.iterations >= cfg.max_iters {
                    self.phase = SchurLmPhase::Done;
                    return true;
                }
                self.iterations += 1;
                let Some((da, db, _)) = schur_step(ad, num_a, &self.base.haa, &self.base.hbb, &self.base.hab, &self.base.ga, &self.base.gb, self.lambda) else {
                    self.lambda *= self.nu;
                    self.nu *= 2.0;
                    if self.lambda > 1e15 {
                        self.phase = SchurLmPhase::Done;
                    }
                    return self.phase == SchurLmPhase::Done;
                };
                let dx_norm = (da.iter().map(|v| v.dot(v)).sum::<f64>() + db.iter().map(|v| v.dot(v)).sum::<f64>()).sqrt();
                if dx_norm <= cfg.tol_dx {
                    self.converged = true;
                    self.phase = SchurLmPhase::Done;
                    return true;
                }
                let a_new: Vec<VecD> = self.a_params.iter().zip(da.iter()).map(|(a, d)| a.add(d)).collect();
                let b_new: Vec<VecD> = self.b_params.iter().zip(db.iter()).map(|(b, d)| b.add(d)).collect();
                self.candidate = Some((a_new, b_new, da, db, BipartiteAccumulator::new(problem)));
                self.phase = SchurLmPhase::Candidate;
            }
            SchurLmPhase::Candidate => {
                let (a_new, b_new, _, _, accumulator) = self.candidate.as_mut().expect("a candidate step");
                if !accumulator.advance(problem, a_new, b_new, &cfg.loss, term_budget) {
                    return false;
                }
                let (a_new, b_new, da, db, accumulator) = self.candidate.take().expect("a candidate step");
                let predicted = 0.5 * (self.base.haa.iter().zip(da.iter()).zip(self.base.ga.iter()).map(|((h, d), g)| quadratic_gain(h, g, d, self.lambda)).sum::<f64>() + self.base.hbb.iter().zip(db.iter()).zip(self.base.gb.iter()).map(|((h, d), g)| quadratic_gain(h, g, d, self.lambda)).sum::<f64>());
                let actual = self.base.cost - accumulator.cost;
                let rho = if predicted.abs() < 1e-300 { 0.0 } else { actual / predicted };
                if rho > 0.0 {
                    self.a_params = a_new;
                    self.b_params = b_new;
                    self.base = accumulator;
                    self.lambda *= (1.0 - (2.0 * rho - 1.0).powi(3)).max(1.0 / 3.0);
                    self.nu = 2.0;
                    if self.base.gradient_infinity_norm() <= cfg.tol_grad {
                        self.converged = true;
                        self.phase = SchurLmPhase::Done;
                        return true;
                    }
                } else {
                    self.lambda *= self.nu;
                    self.nu *= 2.0;
                }
                self.phase = if self.iterations >= cfg.max_iters { SchurLmPhase::Done } else { SchurLmPhase::Solve };
            }
            SchurLmPhase::Done => {}
        }
        self.phase == SchurLmPhase::Done
    }
}
// #endregion 🔖️SchurBundle
