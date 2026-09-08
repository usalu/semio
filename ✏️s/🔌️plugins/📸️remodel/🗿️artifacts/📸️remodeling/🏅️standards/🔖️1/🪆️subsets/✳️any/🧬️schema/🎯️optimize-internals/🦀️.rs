//! 🎯️ Nonlinear least squares and robust estimation: Gauss-Newton, Levenberg-Marquardt, Schur-complement bundle solvers, robust losses and RANSAC consensus.
//! Moved wholesale from `🧮️math/🎯️optimize` — 📸️remodeling is its sole repo-wide consumer (verified: symbol-level grep of every exported type/fn across the whole tree outside math and remodeling returned zero hits), per `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` wave M3d.

use crate::algebra::{cholesky, cholesky_solve, weighted_normal_equations, MatD, VecD};
use geometry::random::Rng;
use std::collections::HashMap;

// #region 🔖️Problem
/// 🧩️ A nonlinear least-squares problem: `parameter_count()` unknowns, `residual_count()` residuals,
/// and their Jacobian. [`plus`](LeastSquaresProblem::plus) is the manifold retraction hook — problems
/// whose parameters live on a manifold (e.g. `SE3`-parameterized poses in a later camera-geometry
/// crate) override it; everything else gets ordinary Euclidean addition for free.
pub trait LeastSquaresProblem {
    fn residual_count(&self) -> usize;
    fn parameter_count(&self) -> usize;
    fn residuals(&self, x: &VecD, out: &mut VecD);
    fn jacobian(&self, x: &VecD, out: &mut MatD);

    /// 🧭️ Retracts a local update `dx` onto the parameter manifold at `x`; ordinary vector addition
    /// for Euclidean parameterizations.
    fn plus(&self, x: &VecD, dx: &VecD) -> VecD {
        x.add(dx)
    }
}

/// 🔬️ Central-difference Jacobian via [`LeastSquaresProblem::plus`], for problems where an analytic
/// Jacobian is unavailable or for cross-checking one that is.
pub fn numeric_jacobian(problem: &impl LeastSquaresProblem, x: &VecD, eps: f64, out: &mut MatD) {
    let n = problem.parameter_count();
    let m = problem.residual_count();
    for j in 0..n {
        let mut dx = VecD::zeros(n);
        dx.set(j, eps);
        let x_plus = problem.plus(x, &dx);
        dx.set(j, -eps);
        let x_minus = problem.plus(x, &dx);
        let mut r_plus = VecD::zeros(m);
        let mut r_minus = VecD::zeros(m);
        problem.residuals(&x_plus, &mut r_plus);
        problem.residuals(&x_minus, &mut r_minus);
        for (row, (rp, rm)) in r_plus.0.iter().zip(r_minus.0.iter()).enumerate() {
            out.set(row, j, (rp - rm) / (2.0 * eps));
        }
    }
}
// #endregion 🔖️Problem

// #region 🔖️RobustLoss
/// 🛡️ M-estimator loss for iteratively-reweighted least squares: [`RobustLoss::weight`] gives the
/// per-residual IRLS weight `rho'(r)/r`, [`RobustLoss::rho`] the robust cost itself, both parameterized
/// by `r2 = r*r` since every caller already has the squared residual on hand.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RobustLoss {
    Trivial,
    Huber(f64),
    Cauchy(f64),
    Tukey(f64),
}

impl RobustLoss {
    /// ⚖️ IRLS reweighting factor for a squared residual `r2`.
    pub fn weight(&self, r2: f64) -> f64 {
        match *self {
            Self::Trivial => 1.0,
            Self::Huber(delta) => {
                let delta2 = delta * delta;
                if r2 <= delta2 {
                    1.0
                } else {
                    delta / r2.sqrt()
                }
            }
            Self::Cauchy(c) => 1.0 / (1.0 + r2 / (c * c)),
            Self::Tukey(c) => {
                let c2 = c * c;
                if r2 < c2 {
                    let t = 1.0 - r2 / c2;
                    t * t
                } else {
                    0.0
                }
            }
        }
    }

    /// 📉️ Robust cost `rho(r)` for a squared residual `r2`, for reporting total cost under the loss.
    pub fn rho(&self, r2: f64) -> f64 {
        match *self {
            Self::Trivial => 0.5 * r2,
            Self::Huber(delta) => {
                let delta2 = delta * delta;
                if r2 <= delta2 {
                    0.5 * r2
                } else {
                    delta * (r2.sqrt() - 0.5 * delta)
                }
            }
            Self::Cauchy(c) => {
                let c2 = c * c;
                0.5 * c2 * (1.0 + r2 / c2).ln()
            }
            Self::Tukey(c) => {
                let c2 = c * c;
                if r2 < c2 {
                    let t = 1.0 - r2 / c2;
                    (c2 / 6.0) * (1.0 - t * t * t)
                } else {
                    c2 / 6.0
                }
            }
        }
    }
}
// #endregion 🔖️RobustLoss

// #region 🔖️GaussNewton
/// 🧮️ Assembles the IRLS-weighted `(JᵀWJ, JᵀWr)` normal-equation pieces and the robustified cost at
/// `x` — the Gauss-Newton machinery that [`levenberg_marquardt`] damps and iterates.
fn evaluate_weighted(problem: &impl LeastSquaresProblem, x: &VecD, loss: &RobustLoss) -> (MatD, VecD, f64) {
    let m = problem.residual_count();
    let n = problem.parameter_count();
    let mut r = VecD::zeros(m);
    let mut j = MatD::zeros(m, n);
    problem.residuals(x, &mut r);
    problem.jacobian(x, &mut j);
    let weights: Vec<f64> = r.0.iter().map(|&ri| loss.weight(ri * ri)).collect();
    let cost: f64 = r.0.iter().map(|&ri| loss.rho(ri * ri)).sum();
    let (jtj, jtr) = weighted_normal_equations(&j, &r, &weights);
    (jtj, jtr, cost)
}

/// 🧮️ Solves the (possibly damped) normal-equation system `A dx = b` via Cholesky, falling back to
/// partial-pivoted LU when `A` isn't positive definite (indefinite damping regime, near-singular Jacobian).
fn solve_normal_equations(a: &MatD, b: &VecD) -> Option<VecD> {
    match cholesky(a) {
        Ok(l) => Some(cholesky_solve(&l, b)),
        Err(_) => a.lu_solve(b),
    }
}

/// 📐️ Local quadratic model's predicted cost reduction `0.5 * dx . (lambda * diag(H) * dx - g)`
/// (Madsen/Nielsen/Tingleff eq. 3.16), used by the Nielsen gain-ratio accept/reject rule.
fn quadratic_gain(h: &MatD, g: &VecD, d: &VecD, lambda: f64) -> f64 {
    (0..d.len()).map(|i| d.get(i) * (lambda * h.get(i, i).max(1e-12) * d.get(i) - g.get(i))).sum()
}

/// 🚶️ Undamped Gauss-Newton: repeatedly solves the IRLS-reweighted normal equations `JᵀWJ dx = -JᵀWr`
/// (`cfg.loss`, `cfg.max_iters`, `cfg.tol_grad`, `cfg.tol_dx` from [`LmConfig`] — `initial_lambda` is
/// unused here) and always accepts the step; exact in one iteration on linear residuals, but has no
/// trust-region fallback, so a rank-deficient Jacobian simply halts iteration early rather than damping.
pub fn gauss_newton(problem: &impl LeastSquaresProblem, x0: VecD, cfg: &LmConfig) -> LmResult {
    let mut x = x0;
    let (mut jtj, mut jtr, mut cost) = evaluate_weighted(problem, &x, &cfg.loss);
    let mut iterations = 0usize;
    let mut converged = jtr.norm_inf() <= cfg.tol_grad;
    while !converged && iterations < cfg.max_iters {
        iterations += 1;
        let rhs = jtr.scale(-1.0);
        let Some(dx) = solve_normal_equations(&jtj, &rhs) else {
            break;
        };
        if dx.norm2() <= cfg.tol_dx {
            converged = true;
            break;
        }
        x = problem.plus(&x, &dx);
        let (jtj_new, jtr_new, cost_new) = evaluate_weighted(problem, &x, &cfg.loss);
        jtj = jtj_new;
        jtr = jtr_new;
        cost = cost_new;
        if jtr.norm_inf() <= cfg.tol_grad {
            converged = true;
        }
    }
    LmResult { x, cost, iterations, converged, jtj }
}
// #endregion 🔖️GaussNewton

// #region 🔖️LevenbergMarquardt
/// 🎛️ Levenberg-Marquardt tuning: damping schedule, convergence tolerances, and the robust loss
/// applied per residual (IRLS).
#[derive(Clone, Debug)]
pub struct LmConfig {
    pub max_iters: usize,
    pub initial_lambda: f64,
    pub tol_grad: f64,
    pub tol_dx: f64,
    pub loss: RobustLoss,
}

impl Default for LmConfig {
    fn default() -> Self {
        Self { max_iters: 100, initial_lambda: 1e-3, tol_grad: 1e-10, tol_dx: 1e-14, loss: RobustLoss::Trivial }
    }
}

/// 📦️ Outcome of [`levenberg_marquardt`]: the solution, its cost, and the final (undamped) `JᵀWJ`
/// Hessian approximation for downstream covariance estimation.
#[derive(Clone, Debug)]
pub struct LmResult {
    pub x: VecD,
    pub cost: f64,
    pub iterations: usize,
    pub converged: bool,
    pub jtj: MatD,
}

/// 🎯️ Damped Gauss-Newton (Levenberg-Marquardt) minimization of `problem`, robustified via IRLS
/// reweighting per [`LmConfig::loss`] and stepped via [`LeastSquaresProblem::plus`]. Damping follows
/// Nielsen's classic update rule: accepted steps shrink `lambda` by the gain ratio's cube law, rejected
/// steps grow it by a doubling factor `nu`.
pub fn levenberg_marquardt(problem: &impl LeastSquaresProblem, x0: VecD, cfg: &LmConfig) -> LmResult {
    let n = problem.parameter_count();
    let mut x = x0;
    let (mut jtj, mut jtr, mut cost) = evaluate_weighted(problem, &x, &cfg.loss);
    let mut lambda = cfg.initial_lambda;
    let mut nu = 2.0_f64;
    let mut iterations = 0usize;
    let mut converged = jtr.norm_inf() <= cfg.tol_grad;
    while !converged && iterations < cfg.max_iters {
        iterations += 1;
        let mut damped = jtj.clone();
        for i in 0..n {
            damped.add_at(i, i, lambda * jtj.get(i, i).max(1e-12));
        }
        let rhs = jtr.scale(-1.0);
        let Some(dx) = solve_normal_equations(&damped, &rhs) else {
            lambda *= nu;
            nu *= 2.0;
            if lambda > 1e15 {
                break;
            }
            continue;
        };
        if dx.norm2() <= cfg.tol_dx {
            converged = true;
            break;
        }
        let x_new = problem.plus(&x, &dx);
        let (jtj_new, jtr_new, cost_new) = evaluate_weighted(problem, &x_new, &cfg.loss);
        let predicted = 0.5 * quadratic_gain(&jtj, &jtr, &dx, lambda);
        let actual = cost - cost_new;
        let rho = if predicted.abs() < 1e-300 { 0.0 } else { actual / predicted };
        if rho > 0.0 {
            x = x_new;
            jtj = jtj_new;
            jtr = jtr_new;
            cost = cost_new;
            lambda *= (1.0 - (2.0 * rho - 1.0).powi(3)).max(1.0 / 3.0);
            nu = 2.0;
            if jtr.norm_inf() <= cfg.tol_grad {
                converged = true;
            }
        } else {
            lambda *= nu;
            nu *= 2.0;
        }
    }
    LmResult { x, cost, iterations, converged, jtj }
}
// #endregion 🔖️LevenbergMarquardt

// #region 🔖️SchurBundle
/// 🌉️ One residual term in a bipartite least-squares problem: touches an "A" block, a "B" block, or
/// both (a pure prior on one side is expressed by leaving the other index `None`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    SchurResult { a_params, b_params, cost, iterations, converged, a_block_covariance_diagonals: covariances }
}

/// 📊️ The marginal covariance diagonal blocks computed during `result`'s final accepted iteration.
pub fn camera_covariances(result: &SchurResult) -> &[MatD] {
    &result.a_block_covariance_diagonals
}
// #endregion 🔖️SchurBundle

// #region 🔖️Consensus
/// 🎲️ A minimal-sample model solver for RANSAC-family consensus: fits a model (possibly several
/// hypotheses) from exactly `SAMPLE_SIZE` data points, and scores any datum's fit via [`residual`](MinimalSolver::residual).
pub trait MinimalSolver {
    type Datum: Clone;
    type Model: Clone;
    const SAMPLE_SIZE: usize;
    fn solve(&self, sample: &[Self::Datum]) -> Vec<Self::Model>;
    fn residual(&self, model: &Self::Model, datum: &Self::Datum) -> f64;
}

/// 📏️ How a candidate model's fit to the full dataset is scored.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RansacScoring {
    InlierCount,
    Msac,
}

/// 🎛️ RANSAC tuning: inlier threshold, target confidence for the adaptive iteration cap, the
/// iteration ceiling itself, and the seed for reproducible sampling.
#[derive(Clone, Debug)]
pub struct RansacConfig {
    pub threshold: f64,
    pub confidence: f64,
    pub max_iters: usize,
    pub seed: u64,
    pub scoring: RansacScoring,
}

impl Default for RansacConfig {
    fn default() -> Self {
        Self { threshold: 1.0, confidence: 0.99, max_iters: 1000, seed: 0, scoring: RansacScoring::Msac }
    }
}

/// 📦️ The winning model, its full inlier index set (re-evaluated over all of `data`, not just the
/// minimal sample), and its score under [`RansacConfig::scoring`].
#[derive(Clone, Debug)]
pub struct RansacResult<M> {
    pub model: M,
    pub inliers: Vec<usize>,
    pub score: f64,
}

/// 📏️ `(msac_cost, inlier_count)` for `model` against every datum — both tracked together so
/// [`is_better`] can compare consistently regardless of [`RansacScoring`] mode.
fn score_model<S: MinimalSolver>(solver: &S, data: &[S::Datum], model: &S::Model, threshold: f64) -> (f64, usize) {
    let t2 = threshold * threshold;
    let mut inlier_count = 0usize;
    let mut msac_cost = 0.0;
    for datum in data {
        let r = solver.residual(model, datum);
        let r2 = r * r;
        if r2 <= t2 {
            inlier_count += 1;
        }
        msac_cost += r2.min(t2);
    }
    (msac_cost, inlier_count)
}

/// ⚖️ Whether `candidate` beats `current_best` under `scoring` (higher inlier count wins for
/// [`RansacScoring::InlierCount`], lower truncated cost wins for [`RansacScoring::Msac`]).
fn is_better(scoring: RansacScoring, candidate: (f64, usize), current_best: (f64, usize)) -> bool {
    match scoring {
        RansacScoring::InlierCount => candidate.1 > current_best.1,
        RansacScoring::Msac => candidate.0 < current_best.0,
    }
}

fn collect_inliers<S: MinimalSolver>(solver: &S, data: &[S::Datum], model: &S::Model, threshold: f64) -> Vec<usize> {
    data.iter().enumerate().filter(|(_, datum)| solver.residual(model, datum).abs() <= threshold).map(|(i, _)| i).collect()
}

/// 🎲️ Shared RANSAC sampling loop: draws minimal samples, scores every hypothesis `solver.solve`
/// returns, and adaptively shrinks the iteration cap via the standard confidence formula whenever a
/// better model is found. `on_new_best` lets [`lo_ransac`] hook in a local-optimization refinement
/// without duplicating the sampling/scoring/adaptive-cap machinery.
fn ransac_core<S: MinimalSolver>(solver: &S, data: &[S::Datum], cfg: &RansacConfig, mut on_new_best: impl FnMut(S::Model, (f64, usize)) -> (S::Model, (f64, usize))) -> Option<(S::Model, (f64, usize))> {
    if data.len() < S::SAMPLE_SIZE {
        return None;
    }
    let mut rng = Rng::from_seed(cfg.seed);
    let mut best: Option<(S::Model, (f64, usize))> = None;
    let mut max_iters = cfg.max_iters;
    let mut iter = 0usize;
    while iter < max_iters {
        iter += 1;
        let idx = rng.sample_without_replacement(data.len(), S::SAMPLE_SIZE);
        let sample: Vec<S::Datum> = idx.iter().map(|&i| data[i].clone()).collect();
        for model in solver.solve(&sample) {
            let candidate = score_model(solver, data, &model, cfg.threshold);
            let better = best.as_ref().is_none_or(|(_, best_score)| is_better(cfg.scoring, candidate, *best_score));
            if !better {
                continue;
            }
            let (final_model, final_score) = on_new_best(model, candidate);
            best = Some((final_model, final_score));
            let inlier_count = final_score.1;
            if inlier_count < S::SAMPLE_SIZE {
                continue;
            }
            let inlier_ratio = inlier_count as f64 / data.len() as f64;
            let denom = (1.0 - inlier_ratio.powi(S::SAMPLE_SIZE as i32)).ln();
            if denom >= 0.0 {
                continue;
            }
            let needed = ((1.0 - cfg.confidence).ln() / denom).ceil();
            if needed.is_finite() && needed >= 0.0 {
                max_iters = max_iters.min((needed as usize).max(iter));
            }
        }
    }
    best
}

fn finish<S: MinimalSolver>(solver: &S, data: &[S::Datum], cfg: &RansacConfig, model: S::Model, score: (f64, usize)) -> RansacResult<S::Model> {
    let inliers = collect_inliers(solver, data, &model, cfg.threshold);
    let score = match cfg.scoring {
        RansacScoring::InlierCount => score.1 as f64,
        RansacScoring::Msac => score.0,
    };
    RansacResult { model, inliers, score }
}

/// 🎲️ Standard RANSAC: repeatedly fits a minimal-sample model and keeps the best-scoring hypothesis,
/// adaptively shrinking the iteration budget as the estimated inlier ratio improves. `None` if `data`
/// is smaller than the minimal sample size or no hypothesis ever fit.
pub fn ransac<S: MinimalSolver>(solver: &S, data: &[S::Datum], cfg: &RansacConfig) -> Option<RansacResult<S::Model>> {
    let (model, score) = ransac_core(solver, data, cfg, |model, score| (model, score))?;
    Some(finish(solver, data, cfg, model, score))
}

/// 🎲️ Locally-optimized RANSAC: identical to [`ransac`], except every time a new best model is found,
/// `local_opt` is offered that model's current inlier set and may return a refined model, which
/// replaces the candidate whenever it scores strictly better.
pub fn lo_ransac<S: MinimalSolver>(solver: &S, data: &[S::Datum], cfg: &RansacConfig, local_opt: impl Fn(&[S::Datum], &S::Model) -> Option<S::Model>) -> Option<RansacResult<S::Model>> {
    let (model, score) = ransac_core(solver, data, cfg, |model, score| {
        let inlier_idx = collect_inliers(solver, data, &model, cfg.threshold);
        let subset: Vec<S::Datum> = inlier_idx.iter().map(|&i| data[i].clone()).collect();
        match local_opt(&subset, &model) {
            Some(refined) => {
                let refined_score = score_model(solver, data, &refined, cfg.threshold);
                if is_better(cfg.scoring, refined_score, score) {
                    (refined, refined_score)
                } else {
                    (model, score)
                }
            }
            None => (model, score),
        }
    })?;
    Some(finish(solver, data, cfg, model, score))
}
// #endregion 🔖️Consensus

// #region 🔖️Scalar
fn sign_or_one(v: f64) -> f64 {
    if v > 0.0 {
        1.0
    } else if v < 0.0 {
        -1.0
    } else {
        1.0
    }
}

/// 🔍️ Golden-section search for the minimum of a unimodal `f` on `[a, b]`, shrinking the bracket by
/// the golden ratio each step until its width drops below `tol` (bounded to 200 steps).
pub fn golden_section(f: impl Fn(f64) -> f64, a: f64, b: f64, tol: f64) -> (f64, f64) {
    let gr = (5f64.sqrt() - 1.0) / 2.0;
    let (mut lo, mut hi) = (a.min(b), a.max(b));
    let mut c = hi - gr * (hi - lo);
    let mut d = lo + gr * (hi - lo);
    let mut fc = f(c);
    let mut fd = f(d);
    for _ in 0..200 {
        if (hi - lo).abs() < tol {
            break;
        }
        if fc < fd {
            hi = d;
            d = c;
            fd = fc;
            c = hi - gr * (hi - lo);
            fc = f(c);
        } else {
            lo = c;
            c = d;
            fc = fd;
            d = lo + gr * (hi - lo);
            fd = f(d);
        }
    }
    let x = 0.5 * (lo + hi);
    (x, f(x))
}

/// 🔍️ Brent's method on a bounded interval `[a, b]`: combines golden-section steps with safeguarded
/// inverse-parabolic interpolation for superlinear convergence near the minimum (the classic
/// Netlib/Brent `fmin` algorithm), bounded to 100 function evaluations.
pub fn brent_minimize(f: impl Fn(f64) -> f64, a: f64, b: f64, tol: f64) -> (f64, f64) {
    let sqrt_eps = 2.220_446_049_250_313e-16_f64.sqrt();
    let golden_mean = 0.5 * (3.0 - 5f64.sqrt());
    let (mut lo, mut hi) = (a.min(b), a.max(b));
    let mut fulc = lo + golden_mean * (hi - lo);
    let (mut nfc, mut xf) = (fulc, fulc);
    let mut rat = 0.0_f64;
    let mut e = 0.0_f64;
    let mut fx = f(xf);
    let mut num_evals = 1usize;
    let (mut ffulc, mut fnfc) = (fx, fx);
    let mut xm = 0.5 * (lo + hi);
    let mut tol1 = sqrt_eps * xf.abs() + tol / 3.0;
    let mut tol2 = 2.0 * tol1;
    let max_iters = 100;
    while (xf - xm).abs() > (tol2 - 0.5 * (hi - lo)) && num_evals < max_iters {
        let mut golden = true;
        if e.abs() > tol1 {
            let r0 = (xf - nfc) * (fx - ffulc);
            let mut q = (xf - fulc) * (fx - fnfc);
            let mut p = (xf - fulc) * q - (xf - nfc) * r0;
            q = 2.0 * (q - r0);
            if q > 0.0 {
                p = -p;
            }
            let q = q.abs();
            let r_prev = e;
            e = rat;
            if p.abs() < (0.5 * q * r_prev).abs() && p > q * (lo - xf) && p < q * (hi - xf) {
                rat = p / q;
                let x = xf + rat;
                if (x - lo) < tol2 || (hi - x) < tol2 {
                    rat = tol1 * sign_or_one(xm - xf);
                }
                golden = false;
            }
        }
        if golden {
            e = if xf >= xm { lo - xf } else { hi - xf };
            rat = golden_mean * e;
        }
        let si = sign_or_one(rat);
        let x = xf + si * rat.abs().max(tol1);
        let fu = f(x);
        num_evals += 1;
        if fu <= fx {
            if x >= xf {
                lo = xf;
            } else {
                hi = xf;
            }
            fulc = nfc;
            ffulc = fnfc;
            nfc = xf;
            fnfc = fx;
            xf = x;
            fx = fu;
        } else {
            if x < xf {
                lo = x;
            } else {
                hi = x;
            }
            if fu <= fnfc || nfc == xf {
                fulc = nfc;
                ffulc = fnfc;
                nfc = x;
                fnfc = fu;
            } else if fu <= ffulc || fulc == xf || fulc == nfc {
                fulc = x;
                ffulc = fu;
            }
        }
        xm = 0.5 * (lo + hi);
        tol1 = sqrt_eps * xf.abs() + tol / 3.0;
        tol2 = 2.0 * tol1;
    }
    (xf, fx)
}
// #endregion 🔖️Scalar

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
