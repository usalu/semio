//! 📐️ Linear-Gaussian structural causal model: fitting, sampling, interventional and counterfactual queries and the what-if evaluation. See [`super`].

use super::*;

// #region 🔖️ScmLinear
/// 🧮️ Linear-Gaussian SCM: `v = intercept_v + Σ_p weights[v][p]·p + ε_v`, `ε_v ~ N(0, noise_var[v])`.
#[derive(Clone, Debug)]
pub struct LinearGaussianScm {
    pub dag: CausalDag,
    pub weights: semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD,
    pub intercepts: semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::VecD,
    pub noise_var: semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::VecD,
}

impl LinearGaussianScm {
    /// 📈️ Per-node OLS on parents (roots get their marginal mean/variance).
    #[allow(clippy::needless_range_loop, reason = "row indexes both the MatD design matrix by (row, col) and the pv slice — enumerate() would only remove one of the two")]
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fit(dag: &CausalDag, data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table) -> Result<Self, CausalError> {
        let n = dag.n();
        let mut weights = semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD::zeros(n, n);
        let mut intercepts = semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::VecD::zeros(n);
        let mut noise_var = semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::VecD::zeros(n);
        for v in 0..n {
            let y = data.continuous(v)?;
            let parents = dag.parents(v);
            if parents.is_empty() {
                intercepts.set(v, semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::mean(y)?);
                noise_var.set(v, semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::variance(y).unwrap_or(1e-12).max(1e-12));
                continue;
            }
            let n_rows = y.len();
            let mut design = semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD::zeros(n_rows, parents.len());
            for (col, &p) in parents.iter().enumerate() {
                let pv = data.continuous(p)?;
                for row in 0..n_rows {
                    design.set(row, col, pv[row]);
                }
            }
            let fit = semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::ols(&design, y, true)?;
            intercepts.set(v, fit.coefficients[0]);
            for (col, &p) in parents.iter().enumerate() {
                weights.set(v, p, fit.coefficients[col + 1]);
            }
            noise_var.set(v, fit.sigma2.max(1e-12));
        }
        Ok(Self { dag: dag.clone(), weights, intercepts, noise_var })
    }

    /// 🎲️ Ancestral sampling in topological order.
    #[allow(clippy::needless_range_loop, reason = "row indexes a per-variable column selected by the inner topological-order loop, not a single Vec — no single iterator covers both loop levels")]
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn simulate(&self, n_samples: usize, rng: &mut semio_framework_geometry::random::Rng) -> Result<semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, CausalError> {
        let n = self.dag.n();
        let mut columns = vec![vec![0.0f64; n_samples]; n];
        for row in 0..n_samples {
            for &v in self.dag.topological_order() {
                let mut val = self.intercepts.get(v);
                for &p in self.dag.parents(v) {
                    val += self.weights.get(v, p) * columns[p][row];
                }
                let sd = self.noise_var.get(v).sqrt().max(1e-12);
                val += semio_s_artifact_stdio_semio_table_probability::schema::probability_internals::Normal::new(0.0, sd)?.sample(rng);
                columns[v][row] = val;
            }
        }
        Ok(semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table::from_f64_columns(self.dag.names().to_vec(), columns)?)
    }

    /// ✂️ `do(v := value)` for each entry: cuts `v`'s incoming edges, fixes `intercept = value`, `noise_var = 0`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn intervened(&self, interventions: &[(usize, f64)]) -> Result<Self, CausalError> {
        let intervened_set: HashSet<usize> = interventions.iter().map(|&(v, _)| v).collect();
        let remaining_edges: Vec<(usize, usize)> = self.dag.edges().into_iter().filter(|&(_, child)| !intervened_set.contains(&child)).collect();
        let new_dag = CausalDag::new(self.dag.names().to_vec(), &remaining_edges)?;
        let n = self.dag.n();
        let mut weights = semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD::zeros(n, n);
        let mut intercepts = self.intercepts.clone();
        let mut noise_var = self.noise_var.clone();
        for v in 0..n {
            if intervened_set.contains(&v) {
                continue;
            }
            for &p in self.dag.parents(v) {
                weights.set(v, p, self.weights.get(v, p));
            }
        }
        for &(v, value) in interventions {
            intercepts.set(v, value);
            noise_var.set(v, 0.0);
        }
        Ok(Self { dag: new_dag, weights, intercepts, noise_var })
    }

    /// 🧮️ `E[v]` for every `v`, via forward substitution in topological order.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn mean(&self) -> semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::VecD {
        let mut mu = semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::VecD::zeros(self.dag.n());
        for &v in self.dag.topological_order() {
            let mut val = self.intercepts.get(v);
            for &p in self.dag.parents(v) {
                val += self.weights.get(v, p) * mu.get(p);
            }
            mu.set(v, val);
        }
        mu
    }

    /// 🧮️ Implied covariance `(I−B)⁻¹ D (I−B)⁻ᵀ`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn implied_covariance(&self) -> Result<semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD, CausalError> {
        let n = self.dag.n();
        let mut i_minus_b = semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD::identity(n);
        for v in 0..n {
            for &p in self.dag.parents(v) {
                i_minus_b.set(v, p, -self.weights.get(v, p));
            }
        }
        let inv = semio_s_artifact_stdio_semio_table_statistics::schema::statistics_internals::invert(&i_minus_b)?;
        let mut d = semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::MatD::zeros(n, n);
        for v in 0..n {
            d.set(v, v, self.noise_var.get(v));
        }
        Ok(inv.matmul(&d).matmul(&inv.transpose()))
    }

    /// 🎯️ `∂E[y]/∂(do x)`: sum over directed `x -> ... -> y` paths of the product of edge
    /// weights, via topological dynamic programming.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn total_effect(&self, x: usize, y: usize) -> f64 {
        let mut sensitivity = vec![0.0f64; self.dag.n()];
        sensitivity[x] = 1.0;
        for &v in self.dag.topological_order() {
            if v == x {
                continue;
            }
            let mut acc = 0.0;
            for &p in self.dag.parents(v) {
                acc += self.weights.get(v, p) * sensitivity[p];
            }
            sensitivity[v] = acc;
        }
        sensitivity[y]
    }

    /// 🎯️ `E[y | do(x=1)] − E[y | do(x=0)]`; equals [`LinearGaussianScm::total_effect`] for a
    /// linear model, kept as a distinct name for symmetry with the potential-outcomes estimators.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn ate(&self, x: usize, y: usize) -> f64 {
        self.total_effect(x, y)
    }

    /// 🔮️ Abduction–action–prediction (Pearl 2009 ch. 7, Thm 7.1.7): recovers every exogenous
    /// noise term from a fully observed row, applies `interventions`, and re-propagates the same
    /// noise — exact for a linear-Gaussian model.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn counterfactual(&self, observed: &[f64], interventions: &[(usize, f64)]) -> Result<semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::VecD, CausalError> {
        let n = self.dag.n();
        if observed.len() != n {
            return Err(CausalError::DimensionMismatch(format!("observed row has {} entries, expected {n}", observed.len())));
        }
        let mut noise = vec![0.0f64; n];
        for v in 0..n {
            let mut predicted = self.intercepts.get(v);
            for &p in self.dag.parents(v) {
                predicted += self.weights.get(v, p) * observed[p];
            }
            noise[v] = observed[v] - predicted;
        }
        let intervened_map: HashMap<usize, f64> = interventions.iter().copied().collect();
        let mut out = semio_s_artifact_stdio_semio_value_algebra::schema::algebra_internals::VecD::zeros(n);
        for &v in self.dag.topological_order() {
            if let Some(&value) = intervened_map.get(&v) {
                out.set(v, value);
                continue;
            }
            let mut val = self.intercepts.get(v);
            for &p in self.dag.parents(v) {
                val += self.weights.get(v, p) * out.get(p);
            }
            val += noise[v];
            out.set(v, val);
        }
        Ok(out)
    }

    /// 🔮️ `E[target | do(interventions)]`: the mean of the intervened model at `target`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn interventional_mean(&self, target: usize, interventions: &[(usize, f64)]) -> Result<f64, CausalError> {
        Ok(self.intervened(interventions)?.mean().get(target))
    }
}
// #endregion 🔖️ScmLinear

// #region 🔖️Query
impl LinearGaussianScm {
    /// 🔮️ Level 2 (empty evidence): interventional mean of `target`. Level 3 (evidence covering
    /// every variable): exact counterfactual via abduction–action–prediction. Partial evidence is
    /// out of scope for v1 (needs conditional-MVN abduction) and returns `InvalidQuery`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn query(&self, target: usize, what_if: &WhatIf) -> Result<f64, CausalError> {
        if what_if.evidence.is_empty() {
            return self.interventional_mean(target, &what_if.interventions);
        }
        if what_if.evidence.len() != self.dag.n() {
            return Err(CausalError::InvalidQuery("counterfactual queries need evidence for every variable in v1 (partial-evidence abduction is out of scope)".to_string()));
        }
        let mut observed = vec![0.0; self.dag.n()];
        for &(v, value) in &what_if.evidence {
            observed[v] = value;
        }
        Ok(self.counterfactual(&observed, &what_if.interventions)?.get(target))
    }
}
// #endregion 🔖️Query
