//! 🧮️ The least-squares problem trait, the numeric Jacobian and the robust loss functions.

use super::*;

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
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::RetireOwned)]
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
