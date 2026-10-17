//! 📐️ The Gauss-Newton step and the Levenberg-Marquardt damped solver with its configuration and result.

use super::*;

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
pub(super) fn solve_normal_equations(a: &MatD, b: &VecD) -> Option<VecD> {
    match cholesky(a) {
        Ok(l) => Some(cholesky_solve(&l, b)),
        Err(_) => a.lu_solve(b),
    }
}

/// 📐️ Local quadratic model's predicted cost reduction `0.5 * dx . (lambda * diag(H) * dx - g)`
/// (Madsen/Nielsen/Tingleff eq. 3.16), used by the Nielsen gain-ratio accept/reject rule.
pub(super) fn quadratic_gain(h: &MatD, g: &VecD, d: &VecD, lambda: f64) -> f64 {
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
#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
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
