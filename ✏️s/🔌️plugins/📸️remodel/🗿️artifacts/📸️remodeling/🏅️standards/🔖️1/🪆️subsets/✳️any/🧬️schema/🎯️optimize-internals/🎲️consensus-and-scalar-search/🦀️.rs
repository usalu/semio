//! 🎲️ RANSAC and LO-RANSAC consensus estimation and the golden-section and Brent scalar minimisers.

use super::*;

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
