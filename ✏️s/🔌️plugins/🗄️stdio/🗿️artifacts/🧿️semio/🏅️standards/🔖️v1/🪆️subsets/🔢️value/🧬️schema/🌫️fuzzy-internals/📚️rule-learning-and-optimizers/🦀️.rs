//! 📚️ Rule learning and optimizers: Wang-Mendel rule generation, pruning and weighting, subtractive clustering, the genetic and particle swarm optimizers and the evolving fuzzy system. See [`super`].

use super::*;

// #region 🔖️RuleLearning
/// 🌱️ Wang–Mendel rule induction from input-output samples.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn wang_mendel_rules(inputs: &[LinguisticVariable], output: &LinguisticVariable, data: &[(Vec<f64>, f64)], model: InferenceModel) -> RuleBase {
    let mut rules = Vec::new();
    for (idx, (x, y)) in data.iter().enumerate() {
        let mut antecedents = Vec::new();
        for (i, var) in inputs.iter().enumerate() {
            let grades = var.fuzzify(x[i]);
            let term = argmax(&grades);
            antecedents.push(AntecedentClause { input: i, term, hedge: None });
        }
        let out_grades = output.fuzzify(*y);
        let out_term = argmax(&out_grades);
        let consequent = match model {
            InferenceModel::Sugeno => Consequent::SugenoConstant { output: 0, value: *y },
            InferenceModel::Tsukamoto => Consequent::Tsukamoto { output: 0, term: out_term },
            _ => Consequent::Mamdani { output: 0, term: out_term },
        };
        rules.push(Rule { id: idx, antecedents, consequent, weight: 1.0, confidence: 1.0 });
    }
    RuleBase::new(rules)
}

/// 🧹️ Prune rules with activation support or weight below thresholds.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn prune_rules(rules: RuleBase, min_weight: f64, min_confidence: f64) -> RuleBase {
    RuleBase::new(rules.rules.into_iter().filter(|r| r.weight >= min_weight && r.confidence >= min_confidence).collect())
}

/// 🎯️ Re-weight rules by empirical fit quality on a dataset.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn weight_rules_by_fit(system: &mut MimoSystem, data: &[(Vec<f64>, Vec<f64>)]) {
    let mut confidences = Vec::new();
    for rule in &system.rules.rules {
        let mut err = 0.0;
        let mut n = 0.0;
        for (x, y) in data {
            if let Ok((pred, _)) = system.infer(x) {
                for (p, t) in pred.iter().zip(y.iter()) {
                    err += (p - t).abs();
                    n += 1.0;
                }
            }
        }
        confidences.push(if n > 0.0 { (1.0 / (1.0 + err / n)).clamp(0.0, 1.0) } else { rule.confidence });
    }
    for (rule, confidence) in system.rules.rules.iter_mut().zip(confidences) {
        rule.confidence = confidence;
    }
}

/// 🌱️ Subtractive clustering seed centers for rule generation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn subtractive_cluster_centers(data: &[Vec<f64>], ra: f64, accept_ratio: f64, reject_ratio: f64) -> Vec<Vec<f64>> {
    if data.is_empty() {
        return Vec::new();
    }
    let dim = data[0].len();
    let mut potentials: Vec<f64> = data.iter().map(|_| 0.0).collect();
    for i in 0..data.len() {
        for point in data.iter() {
            let dist2: f64 = (0..dim).map(|d| (data[i][d] - point[d]).powi(2)).sum();
            potentials[i] += (-dist2 / (2.0 * ra * ra)).exp();
        }
    }
    let mut centers = Vec::new();
    let max_p = potentials.iter().copied().fold(0.0, f64::max);
    let accept = accept_ratio * max_p;
    let reject = reject_ratio * max_p;
    while let Some((idx, p)) = potentials.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).map(|(i, &p)| (i, p)) {
        if p < reject {
            break;
        }
        centers.push(data[idx].clone());
        for j in 0..data.len() {
            let dist2: f64 = (0..dim).map(|d| (data[idx][d] - data[j][d]).powi(2)).sum();
            potentials[j] -= (-dist2 / (2.0 * ra * ra)).exp();
            potentials[j] = potentials[j].max(0.0);
        }
        if p < accept {
            break;
        }
    }
    centers
}
// #endregion 🔖️RuleLearning

// #region 🔖️Optimizers
/// 🧬️ Genetic algorithm optimizer for fuzzy parameter vectors.
pub struct GeneticOptimizer {
    pub population_size: usize,
    pub generations: usize,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub bounds: Vec<(f64, f64)>,
    pub seed: u64,
}

impl GeneticOptimizer {
    pub fn optimize<F: Fn(&[f64]) -> f64>(&self, fitness: F) -> (Vec<f64>, f64) {
        let mut rng = Rng::from_seed(self.seed);
        let dim = self.bounds.len();
        let mut population: Vec<Vec<f64>> = (0..self.population_size).map(|_| (0..dim).map(|d| self.bounds[d].0 + rng.next_f64() * (self.bounds[d].1 - self.bounds[d].0)).collect()).collect();
        let mut best = population[0].clone();
        let mut best_fit = fitness(&best);
        for _ in 0..self.generations {
            let mut scored: Vec<(f64, Vec<f64>)> = population.drain(..).map(|ind| (fitness(&ind), ind)).collect();
            scored.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            if scored[0].0 < best_fit {
                best_fit = scored[0].0;
                best = scored[0].1.clone();
            }
            let elite = scored.len() / 5;
            population = scored.into_iter().take(elite).map(|(_, ind)| ind).collect();
            while population.len() < self.population_size {
                let p1 = &population[rng.next_range(0, population.len() as u64) as usize];
                let p2 = &population[rng.next_range(0, population.len() as u64) as usize];
                let mut child: Vec<f64> = if rng.next_bool(self.crossover_rate) { (0..dim).map(|d| if rng.next_bool(0.5) { p1[d] } else { p2[d] }).collect() } else { p1.clone() };
                for d in 0..dim {
                    if rng.next_bool(self.mutation_rate) {
                        child[d] = self.bounds[d].0 + rng.next_f64() * (self.bounds[d].1 - self.bounds[d].0);
                    }
                }
                population.push(child);
            }
        }
        (best, best_fit)
    }
}

/// 🐝️ Particle swarm optimizer for fuzzy parameter tuning.
pub struct PsoOptimizer {
    pub swarm_size: usize,
    pub iterations: usize,
    pub inertia: f64,
    pub cognitive: f64,
    pub social: f64,
    pub bounds: Vec<(f64, f64)>,
    pub seed: u64,
}

impl PsoOptimizer {
    pub fn optimize<F: Fn(&[f64]) -> f64>(&self, fitness: F) -> (Vec<f64>, f64) {
        let mut rng = Rng::from_seed(self.seed);
        let dim = self.bounds.len();
        let mut positions: Vec<Vec<f64>> = (0..self.swarm_size).map(|_| (0..dim).map(|d| self.bounds[d].0 + rng.next_f64() * (self.bounds[d].1 - self.bounds[d].0)).collect()).collect();
        let mut velocities = vec![vec![0.0; dim]; self.swarm_size];
        let mut personal_best = positions.clone();
        let mut personal_fit: Vec<f64> = positions.iter().map(|p| fitness(p)).collect();
        let gbest_idx = personal_fit.iter().enumerate().min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).map_or(0, |(i, _)| i);
        let mut gbest = personal_best[gbest_idx].clone();
        let mut gbest_fit = personal_fit[gbest_idx];
        for _ in 0..self.iterations {
            for i in 0..self.swarm_size {
                for d in 0..dim {
                    let r1 = rng.next_f64();
                    let r2 = rng.next_f64();
                    velocities[i][d] = self.inertia * velocities[i][d] + self.cognitive * r1 * (personal_best[i][d] - positions[i][d]) + self.social * r2 * (gbest[d] - positions[i][d]);
                    positions[i][d] = (positions[i][d] + velocities[i][d]).clamp(self.bounds[d].0, self.bounds[d].1);
                }
                let fit = fitness(&positions[i]);
                if fit < personal_fit[i] {
                    personal_fit[i] = fit;
                    personal_best[i] = positions[i].clone();
                    if fit < gbest_fit {
                        gbest_fit = fit;
                        gbest = positions[i].clone();
                    }
                }
            }
        }
        (gbest, gbest_fit)
    }
}
// #endregion 🔖️Optimizers

// #region 🔖️EvolvingFuzzySystem
/// 🔄️ Evolving fuzzy system that adds, prunes, and adapts rules from streaming samples.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct EvolvingFuzzySystem {
    pub system: MimoSystem,
    pub learning_rate: f64,
    pub prune_threshold: f64,
    pub max_rules: usize,
}

impl EvolvingFuzzySystem {
    pub fn new(system: MimoSystem, learning_rate: f64, prune_threshold: f64, max_rules: usize) -> Self {
        Self { system, learning_rate, prune_threshold, max_rules }
    }

    pub fn observe(&mut self, x: &[f64], y: &[f64]) -> FuzzyResult<Explanation> {
        let (pred, mut explanation) = self.system.infer(x)?;
        for (rule, trace) in self.system.rules.rules.iter_mut().zip(explanation.traces.iter()) {
            rule.weight = (rule.weight + self.learning_rate * trace.activation).clamp(0.0, 1.0);
            for (p, t) in pred.iter().zip(y.iter()) {
                let err = (p - t).abs();
                rule.confidence = (rule.confidence + self.learning_rate * (1.0 - err)).clamp(0.0, 1.0);
            }
        }
        self.system.rules = prune_rules(self.system.rules.clone(), self.prune_threshold, self.prune_threshold);
        if self.system.rules.rules.len() < self.max_rules {
            if let Some(output) = y.first().copied() {
                let rule = Rule {
                    id: self.system.rules.rules.len(),
                    antecedents: (0..self.system.inputs.len())
                        .map(|i| {
                            let grades = self.system.inputs[i].fuzzify(x[i]);
                            AntecedentClause { input: i, term: argmax(&grades), hedge: None }
                        })
                        .collect(),
                    consequent: Consequent::SugenoConstant { output: 0, value: output },
                    weight: 0.5,
                    confidence: 0.5,
                };
                self.system.rules.rules.push(rule);
            }
        }
        explanation.rationale = format!("evolving update: {} active rules", self.system.rules.rules.len());
        Ok(explanation)
    }
}
// #endregion 🔖️EvolvingFuzzySystem
