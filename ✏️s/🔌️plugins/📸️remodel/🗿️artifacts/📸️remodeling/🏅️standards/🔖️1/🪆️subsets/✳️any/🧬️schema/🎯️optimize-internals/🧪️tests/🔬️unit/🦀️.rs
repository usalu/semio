
use super::*;
use crate::algebra::solve_llsq;

// #region 🔖️RosenbrockTests
struct Rosenbrock;

impl LeastSquaresProblem for Rosenbrock {
    fn residual_count(&self) -> usize {
        2
    }

    fn parameter_count(&self) -> usize {
        2
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        let (x0, x1) = (x.get(0), x.get(1));
        out.set(0, 10.0 * (x1 - x0 * x0));
        out.set(1, 1.0 - x0);
    }

    fn jacobian(&self, x: &VecD, out: &mut MatD) {
        let x0 = x.get(0);
        out.set(0, 0, -20.0 * x0);
        out.set(0, 1, 10.0);
        out.set(1, 0, -1.0);
        out.set(1, 1, 0.0);
    }
}

#[test]
fn levenberg_marquardt_solves_rosenbrock() {
    let cfg = LmConfig::default();
    let x0 = VecD::from_vec(vec![-1.2, 1.0]);
    let result = levenberg_marquardt(&Rosenbrock, x0, &cfg);
    assert!(result.converged);
    assert!((result.x.get(0) - 1.0).abs() < 1e-4, "x0 = {}", result.x.get(0));
    assert!((result.x.get(1) - 1.0).abs() < 1e-4, "x1 = {}", result.x.get(1));
    assert!(result.cost < 1e-8, "cost = {}", result.cost);
}

#[test]
fn numeric_jacobian_matches_analytic_jacobian_for_rosenbrock() {
    let x = VecD::from_vec(vec![0.7, -0.3]);
    let mut analytic = MatD::zeros(2, 2);
    Rosenbrock.jacobian(&x, &mut analytic);
    let mut numeric = MatD::zeros(2, 2);
    numeric_jacobian(&Rosenbrock, &x, 1e-6, &mut numeric);
    for row in 0..2 {
        for col in 0..2 {
            assert!((analytic.get(row, col) - numeric.get(row, col)).abs() < 1e-4);
        }
    }
}
// #endregion 🔖️RosenbrockTests

// #region 🔖️GaussNewtonTests
struct ConsistentLinearSystem;

impl LeastSquaresProblem for ConsistentLinearSystem {
    fn residual_count(&self) -> usize {
        3
    }

    fn parameter_count(&self) -> usize {
        2
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        let (a, b) = (x.get(0), x.get(1));
        out.set(0, 2.0 * a + b - 5.0);
        out.set(1, a - b - 1.0);
        out.set(2, 3.0 * a + 2.0 * b - 8.0);
    }

    fn jacobian(&self, _x: &VecD, out: &mut MatD) {
        out.set(0, 0, 2.0);
        out.set(0, 1, 1.0);
        out.set(1, 0, 1.0);
        out.set(1, 1, -1.0);
        out.set(2, 0, 3.0);
        out.set(2, 1, 2.0);
    }
}

#[test]
fn gauss_newton_solves_consistent_linear_system_in_one_step() {
    let cfg = LmConfig { max_iters: 20, tol_grad: 1e-14, tol_dx: 1e-14, ..LmConfig::default() };
    let x0 = VecD::from_vec(vec![0.0, 0.0]);
    let result = gauss_newton(&ConsistentLinearSystem, x0, &cfg);
    assert!(result.converged);
    assert_eq!(result.iterations, 1, "linear residuals should be exact after one Gauss-Newton step");
    assert!((result.x.get(0) - 2.0).abs() < 1e-8, "a = {}", result.x.get(0));
    assert!((result.x.get(1) - 1.0).abs() < 1e-8, "b = {}", result.x.get(1));
    assert!(result.cost < 1e-16, "cost = {}", result.cost);
}
// #endregion 🔖️GaussNewtonTests

// #region 🔖️RobustRegressionTests
struct LinearRegressionProblem {
    xs: Vec<f64>,
    ys: Vec<f64>,
}

impl LeastSquaresProblem for LinearRegressionProblem {
    fn residual_count(&self) -> usize {
        self.xs.len()
    }

    fn parameter_count(&self) -> usize {
        2
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        let (m, b) = (x.get(0), x.get(1));
        for (i, (&xi, &yi)) in self.xs.iter().zip(self.ys.iter()).enumerate() {
            out.set(i, m * xi + b - yi);
        }
    }

    fn jacobian(&self, _x: &VecD, out: &mut MatD) {
        for (i, &xi) in self.xs.iter().enumerate() {
            out.set(i, 0, xi);
            out.set(i, 1, 1.0);
        }
    }
}

#[test]
fn huber_loss_recovers_line_that_trivial_loss_misses() {
    let true_m = 2.0;
    let true_b = -1.0;
    let n = 40;
    let mut rng = Rng::from_seed(20260719);
    let mut xs = Vec::with_capacity(n);
    let mut ys = Vec::with_capacity(n);
    for i in 0..n {
        let x = i as f64 * 0.25;
        let clean = true_m * x + true_b;
        let y = if i % 3 == 0 { clean + 8.0 + rng.next_f64() * 4.0 } else { clean + (rng.next_f64() - 0.5) * 0.05 };
        xs.push(x);
        ys.push(y);
    }
    let problem = LinearRegressionProblem { xs, ys };
    let x0 = VecD::from_vec(vec![0.0, 0.0]);

    let trivial_cfg = LmConfig { loss: RobustLoss::Trivial, ..LmConfig::default() };
    let trivial_result = levenberg_marquardt(&problem, x0.clone(), &trivial_cfg);

    let huber_cfg = LmConfig { loss: RobustLoss::Huber(1.0), ..LmConfig::default() };
    let huber_result = levenberg_marquardt(&problem, x0, &huber_cfg);

    let trivial_slope_error = (trivial_result.x.get(0) - true_m).abs();
    let huber_slope_error = (huber_result.x.get(0) - true_m).abs();
    let trivial_intercept_error = (trivial_result.x.get(1) - true_b).abs();
    let huber_intercept_error = (huber_result.x.get(1) - true_b).abs();
    assert!(huber_slope_error < 0.1, "huber slope error {huber_slope_error}");
    assert!(huber_slope_error < trivial_slope_error, "huber slope error {huber_slope_error} should beat trivial slope error {trivial_slope_error}");
    assert!(huber_intercept_error < 0.6, "huber intercept error {huber_intercept_error}");
    assert!(huber_intercept_error < trivial_intercept_error, "huber intercept error {huber_intercept_error} should beat trivial intercept error {trivial_intercept_error}");
}
// #endregion 🔖️RobustRegressionTests

// #region 🔖️SchurConsistencyTests
struct ToyObservations {
    observed: Vec<Vec<f64>>,
}

fn toy_predict(a: &VecD, b: &VecD) -> f64 {
    let scale = a.get(0);
    let offset = a.get(1);
    let value = b.get(0);
    scale * value + offset + 0.01 * scale * value * value
}

struct FlatToyProblem<'a> {
    model: &'a ToyObservations,
    num_cameras: usize,
    num_points: usize,
}

impl FlatToyProblem<'_> {
    fn unpack(&self, x: &VecD) -> (Vec<VecD>, Vec<VecD>) {
        let a = (0..self.num_cameras).map(|i| VecD::from_vec(vec![x.get(2 * i), x.get(2 * i + 1)])).collect();
        let base = 2 * self.num_cameras;
        let b = (0..self.num_points).map(|j| VecD::from_vec(vec![x.get(base + j)])).collect();
        (a, b)
    }
}

impl LeastSquaresProblem for FlatToyProblem<'_> {
    fn residual_count(&self) -> usize {
        self.num_cameras * self.num_points
    }

    fn parameter_count(&self) -> usize {
        2 * self.num_cameras + self.num_points
    }

    fn residuals(&self, x: &VecD, out: &mut VecD) {
        let (a, b) = self.unpack(x);
        for (i, ai) in a.iter().enumerate() {
            for (j, bj) in b.iter().enumerate() {
                out.set(i * self.num_points + j, toy_predict(ai, bj) - self.model.observed[i][j]);
            }
        }
    }

    fn jacobian(&self, x: &VecD, out: &mut MatD) {
        let (a, b) = self.unpack(x);
        for (i, ai) in a.iter().enumerate() {
            for (j, bj) in b.iter().enumerate() {
                let row = i * self.num_points + j;
                let scale = ai.get(0);
                let value = bj.get(0);
                out.set(row, 2 * i, value + 0.01 * value * value);
                out.set(row, 2 * i + 1, 1.0);
                out.set(row, 2 * self.num_cameras + j, scale + 0.02 * scale * value);
            }
        }
    }
}

struct SchurToyProblem<'a> {
    model: &'a ToyObservations,
    terms: Vec<ResidualTerm>,
    num_points: usize,
}

impl BipartiteResiduals for SchurToyProblem<'_> {
    fn num_a_blocks(&self) -> usize {
        self.model.observed.len()
    }

    fn num_b_blocks(&self) -> usize {
        self.num_points
    }

    fn a_block_dim(&self) -> usize {
        2
    }

    fn b_block_dim(&self) -> usize {
        1
    }

    fn residual_terms(&self) -> &[ResidualTerm] {
        &self.terms
    }

    fn evaluate(&self, a_params: &[VecD], b_params: &[VecD], term: &ResidualTerm) -> (VecD, MatD, MatD) {
        let i = term.a_index.expect("toy terms always touch a camera");
        let j = term.b_index.expect("toy terms always touch a point");
        let scale = a_params[i].get(0);
        let value = b_params[j].get(0);
        let r = VecD::from_vec(vec![toy_predict(&a_params[i], &b_params[j]) - self.model.observed[i][j]]);
        let mut ja = MatD::zeros(1, 2);
        ja.set(0, 0, value + 0.01 * value * value);
        ja.set(0, 1, 1.0);
        let mut jb = MatD::zeros(1, 1);
        jb.set(0, 0, scale + 0.02 * scale * value);
        (r, ja, jb)
    }
}

#[test]
fn schur_lm_matches_flat_levenberg_marquardt() {
    let num_cameras = 2;
    let num_points = 8;
    let a_true: Vec<VecD> = (0..num_cameras).map(|i| VecD::from_vec(vec![1.0 + 0.3 * i as f64, 0.1 * i as f64])).collect();
    let b_true: Vec<VecD> = (0..num_points).map(|j| VecD::from_vec(vec![0.5 + 0.2 * j as f64])).collect();
    let observed: Vec<Vec<f64>> = a_true.iter().map(|a| b_true.iter().map(|b| toy_predict(a, b)).collect()).collect();
    let model = ToyObservations { observed };

    let a0: Vec<VecD> = (0..num_cameras).map(|_| VecD::from_vec(vec![0.8, 0.0])).collect();
    let b0: Vec<VecD> = (0..num_points).map(|_| VecD::from_vec(vec![0.4])).collect();

    let terms: Vec<ResidualTerm> = (0..num_cameras).flat_map(|i| (0..num_points).map(move |j| ResidualTerm { a_index: Some(i), b_index: Some(j), dim: 1 })).collect();

    let flat_problem = FlatToyProblem { model: &model, num_cameras, num_points };
    let mut x0 = Vec::with_capacity(2 * num_cameras + num_points);
    for a in &a0 {
        x0.push(a.get(0));
        x0.push(a.get(1));
    }
    for b in &b0 {
        x0.push(b.get(0));
    }
    let cfg = LmConfig { max_iters: 200, tol_grad: 1e-14, tol_dx: 1e-16, ..LmConfig::default() };
    let flat_result = levenberg_marquardt(&flat_problem, VecD::from_vec(x0), &cfg);

    let schur_problem = SchurToyProblem { model: &model, terms, num_points };
    let schur_result = schur_lm(&schur_problem, a0, b0, &cfg);

    assert!((flat_result.cost - schur_result.cost).abs() < 1e-6, "flat cost {} vs schur cost {}", flat_result.cost, schur_result.cost);

    let mut flat_residuals = VecD::zeros(flat_problem.residual_count());
    flat_problem.residuals(&flat_result.x, &mut flat_residuals);
    let flat_norm = flat_residuals.norm2();
    let schur_norm = (2.0 * schur_result.cost).sqrt();
    assert!((flat_norm - schur_norm).abs() < 1e-6, "flat residual norm {flat_norm} vs schur residual norm {schur_norm}");

    assert_eq!(camera_covariances(&schur_result).len(), num_cameras);
}
// #endregion 🔖️SchurConsistencyTests

// #region 🔖️RansacTests
struct LineSolver;

impl MinimalSolver for LineSolver {
    type Datum = (f64, f64);
    type Model = (f64, f64);
    const SAMPLE_SIZE: usize = 2;

    fn solve(&self, sample: &[Self::Datum]) -> Vec<Self::Model> {
        let (x1, y1) = sample[0];
        let (x2, y2) = sample[1];
        if (x2 - x1).abs() < 1e-12 {
            return Vec::new();
        }
        let m = (y2 - y1) / (x2 - x1);
        let b = y1 - m * x1;
        vec![(m, b)]
    }

    fn residual(&self, model: &Self::Model, datum: &Self::Datum) -> f64 {
        model.0 * datum.0 + model.1 - datum.1
    }
}

fn synthetic_line_data(seed: u64) -> (Vec<(f64, f64)>, f64, f64, usize) {
    let true_m = 2.0;
    let true_b = 1.0;
    let n = 100;
    let outlier_count = 40;
    let mut rng = Rng::from_seed(seed);
    let data: Vec<(f64, f64)> = (0..n)
        .map(|i| {
            let x = i as f64 * 0.1;
            let clean = true_m * x + true_b;
            if i < outlier_count { (x, clean + 10.0 + rng.next_f64() * 5.0) } else { (x, clean + (rng.next_f64() - 0.5) * 0.02) }
        })
        .collect();
    (data, true_m, true_b, n - outlier_count)
}

#[test]
fn ransac_recovers_line_and_planted_inlier_count() {
    let (data, true_m, true_b, planted_inliers) = synthetic_line_data(2026);
    let cfg = RansacConfig { threshold: 0.2, confidence: 0.999, max_iters: 500, seed: 7, scoring: RansacScoring::Msac };
    let result = ransac(&LineSolver, &data, &cfg).expect("line is fittable");
    assert_eq!(result.inliers.len(), planted_inliers);
    assert!((result.model.0 - true_m).abs() < 0.05, "m = {}", result.model.0);
    assert!((result.model.1 - true_b).abs() < 0.05, "b = {}", result.model.1);
}

#[test]
fn lo_ransac_improves_on_plain_ransac_accuracy() {
    let (data, true_m, true_b, _) = synthetic_line_data(2026);
    let cfg = RansacConfig { threshold: 0.2, confidence: 0.999, max_iters: 500, seed: 7, scoring: RansacScoring::Msac };
    let plain = ransac(&LineSolver, &data, &cfg).expect("line is fittable");
    let local_opt = |subset: &[(f64, f64)], _model: &(f64, f64)| -> Option<(f64, f64)> {
        if subset.len() < 2 {
            return None;
        }
        let mut a = MatD::zeros(subset.len(), 2);
        let mut y = VecD::zeros(subset.len());
        for (row, &(x, yy)) in subset.iter().enumerate() {
            a.set(row, 0, x);
            a.set(row, 1, 1.0);
            y.set(row, yy);
        }
        solve_llsq(&a, &y).ok().map(|v| (v.get(0), v.get(1)))
    };
    let refined = lo_ransac(&LineSolver, &data, &cfg, local_opt).expect("line is fittable");

    let plain_error = (plain.model.0 - true_m).abs() + (plain.model.1 - true_b).abs();
    let refined_error = (refined.model.0 - true_m).abs() + (refined.model.1 - true_b).abs();
    assert!(refined_error < 1e-2, "refined error {refined_error}");
    assert!(refined_error <= plain_error + 1e-9, "refined error {refined_error} should not exceed plain error {plain_error}");
}
// #endregion 🔖️RansacTests

// #region 🔖️ScalarTests
#[test]
fn brent_minimize_finds_convex_minimum() {
    let (x, fx) = brent_minimize(|x| (x - 2.0).powi(2) + 1.0, 0.0, 5.0, 1e-10);
    assert!((x - 2.0).abs() < 1e-6, "x = {x}");
    assert!((fx - 1.0).abs() < 1e-6, "fx = {fx}");
}

#[test]
fn golden_section_finds_convex_minimum() {
    let (x, fx) = golden_section(|x| (x - 2.0).powi(2) + 1.0, 0.0, 5.0, 1e-8);
    assert!((x - 2.0).abs() < 1e-4, "x = {x}");
    assert!((fx - 1.0).abs() < 1e-4, "fx = {fx}");
}
// #endregion 🔖️ScalarTests
