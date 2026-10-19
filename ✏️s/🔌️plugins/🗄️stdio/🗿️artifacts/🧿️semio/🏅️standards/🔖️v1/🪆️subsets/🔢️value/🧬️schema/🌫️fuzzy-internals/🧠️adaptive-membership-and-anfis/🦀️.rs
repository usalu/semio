//! 🧠️ Adaptive membership and ANFIS: trainable membership functions and the adaptive neuro-fuzzy inference system. See [`super`].

use super::*;

// #region 🔖️AdaptiveMembership
/// 🎯️ Adaptive membership-function fitting from labeled (x, μ) samples via gradient descent.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct AdaptiveMembership {
    pub mf: MembershipFunction,
    pub learning_rate: f64,
}

impl AdaptiveMembership {
    pub fn new(mf: MembershipFunction, learning_rate: f64) -> Self {
        Self { mf, learning_rate }
    }

    pub fn fit(&mut self, samples: &[(f64, f64)], epochs: usize) -> f64 {
        let mut loss = 0.0;
        for _ in 0..epochs {
            loss = 0.0;
            let mut grads = vec![0.0; self.mf.parameters().len()];
            for &(x, target) in samples {
                let pred = self.mf.eval(x);
                let err = pred - clamp01(target);
                loss += err * err;
                let eps = 1e-4;
                let params = self.mf.parameters();
                for (i, g) in grads.iter_mut().enumerate().take(params.len()) {
                    let mut p = params.clone();
                    p[i] += eps;
                    let _ = self.mf.set_parameters(&p);
                    let plus = self.mf.eval(x);
                    let _ = self.mf.set_parameters(&params);
                    *g += err * (plus - pred) / eps;
                }
            }
            let mut params = self.mf.parameters();
            for (p, g) in params.iter_mut().zip(grads.iter()) {
                *p -= self.learning_rate * g / samples.len().max(1) as f64;
            }
            let _ = self.mf.set_parameters(&params);
        }
        loss / samples.len().max(1) as f64
    }
}
// #endregion 🔖️AdaptiveMembership

// #region 🔖️Anfis
/// 🧠️ ANFIS: adaptive neuro-fuzzy inference system with hybrid least-squares + gradient learning.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Anfis {
    pub input_count: usize,
    pub rules_per_input: usize,
    pub premise_centers: Vec<Vec<f64>>,
    pub premise_widths: Vec<Vec<f64>>,
    pub consequent_coeffs: Vec<Vec<f64>>,
}

impl Anfis {
    pub fn new(input_count: usize, rules_per_input: usize, input_ranges: &[(f64, f64)]) -> Self {
        let rule_count = rules_per_input.pow(input_count as u32);
        let mut premise_centers = vec![vec![0.0; input_count]; rule_count];
        let mut premise_widths = vec![vec![1.0; input_count]; rule_count];
        let mut idx = 0usize;
        let steps: Vec<Vec<f64>> = input_ranges.iter().map(|(lo, hi)| linspace(*lo, *hi, rules_per_input)).collect();
        fn recurse(input_count: usize, steps: &[Vec<f64>], path: &mut Vec<usize>, idx: &mut usize, centers: &mut [Vec<f64>], widths: &mut [Vec<f64>], ranges: &[(f64, f64)]) {
            if path.len() == input_count {
                for (j, &s) in path.iter().enumerate() {
                    centers[*idx][j] = steps[j][s];
                    widths[*idx][j] = (ranges[j].1 - ranges[j].0) / steps[j].len().max(1) as f64;
                }
                *idx += 1;
                return;
            }
            for s in 0..steps[path.len()].len() {
                path.push(s);
                recurse(input_count, steps, path, idx, centers, widths, ranges);
                path.pop();
            }
        }
        let mut path = Vec::new();
        recurse(input_count, &steps, &mut path, &mut idx, &mut premise_centers, &mut premise_widths, input_ranges);
        let consequent_coeffs = vec![vec![0.0; input_count + 1]; rule_count];
        Self { input_count, rules_per_input, premise_centers, premise_widths, consequent_coeffs }
    }

    pub fn rule_count(&self) -> usize {
        self.consequent_coeffs.len()
    }

    fn firing_strengths(&self, x: &[f64]) -> Vec<f64> {
        (0..self.rule_count())
            .map(|r| {
                (0..self.input_count)
                    .map(|j| {
                        let sigma = self.premise_widths[r][j].max(1e-6);
                        let z = (x[j] - self.premise_centers[r][j]) / sigma;
                        (-0.5 * z * z).exp()
                    })
                    .product()
            })
            .collect()
    }

    pub fn forward(&self, x: &[f64]) -> f64 {
        let w = self.firing_strengths(x);
        let den: f64 = w.iter().sum();
        if den < 1e-12 {
            return 0.0;
        }
        let mut num = 0.0;
        for (r, wr) in w.iter().enumerate() {
            let mut y = self.consequent_coeffs[r][0];
            for (j, &xj) in x.iter().enumerate() {
                y += self.consequent_coeffs[r][j + 1] * xj;
            }
            num += wr * y;
        }
        num / den
    }

    pub fn fit_hybrid(&mut self, data: &[(Vec<f64>, f64)], epochs: usize) -> f64 {
        let mut loss = 0.0;
        for _ in 0..epochs {
            self.fit_consequents(data);
            loss = self.fit_premises(data);
        }
        loss
    }

    fn fit_consequents(&mut self, data: &[(Vec<f64>, f64)]) {
        let n = data.len();
        let r = self.rule_count();
        let p = self.input_count + 1;
        let mut ata = MatD::zeros(r * p, r * p);
        let mut atb = VecD::zeros(r * p);
        for (x, y) in data {
            let w = self.firing_strengths(x);
            let den: f64 = w.iter().sum();
            if den < 1e-12 {
                continue;
            }
            for i in 0..r {
                let wi = w[i] / den;
                let mut row = vec![wi];
                row.extend(x.iter().map(|&xj| wi * xj));
                for a in 0..p {
                    for b in 0..p {
                        ata.add_at(i * p + a, i * p + b, row[a] * row[b]);
                    }
                    atb.add_at(i * p + a, row[a] * *y);
                }
            }
        }
        if let Some(sol) = ata.lu_solve(&atb) {
            for i in 0..r {
                for j in 0..p {
                    self.consequent_coeffs[i][j] = sol.get(i * p + j);
                }
            }
        }
        let _ = n;
    }

    fn fit_premises(&mut self, data: &[(Vec<f64>, f64)]) -> f64 {
        let mut loss = 0.0;
        let lr = 0.01;
        for (x, y) in data {
            let pred = self.forward(x);
            let err = pred - y;
            loss += err * err;
            for r in 0..self.rule_count() {
                for j in 0..self.input_count {
                    let sigma = self.premise_widths[r][j].max(1e-6);
                    let z = (x[j] - self.premise_centers[r][j]) / sigma;
                    let g = err * (-z / sigma) * (-0.5 * z * z).exp();
                    self.premise_centers[r][j] -= lr * g;
                    self.premise_widths[r][j] = (self.premise_widths[r][j] - lr * g.abs()).max(1e-3);
                }
            }
        }
        loss / data.len().max(1) as f64
    }
}
// #endregion 🔖️Anfis
