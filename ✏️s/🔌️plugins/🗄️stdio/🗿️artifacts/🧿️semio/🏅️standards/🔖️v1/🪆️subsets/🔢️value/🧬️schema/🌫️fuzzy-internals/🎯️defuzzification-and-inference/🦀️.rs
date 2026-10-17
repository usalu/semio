//! 🎯️ Defuzzification and inference: the defuzzifiers, the inference models, rule traces, explanations, the multiple input multiple output system and the hierarchical system. See [`super`].

use super::*;

// #region 🔖️Defuzzification
/// 🎯️ Defuzzification method applied to a discrete aggregated membership vector.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum Defuzzifier {
    Centroid,
    Bisector,
    Mom,
    Som,
    Lom,
    WeightedAverage,
    Height,
}

impl Defuzzifier {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(&self, universe: &Universe, membership: &[f64], rule_heights: Option<&[f64]>, rule_values: Option<&[f64]>) -> FuzzyResult<f64> {
        if universe.samples.is_empty() || membership.is_empty() {
            return Err(FuzzyError::EmptyUniverse);
        }
        match self {
            Self::Centroid => {
                let mut num = 0.0;
                let mut den = 0.0;
                for (x, mu) in universe.samples.iter().zip(membership.iter()) {
                    num += x * mu;
                    den += mu;
                }
                Ok(if den.abs() < 1e-12 { universe.samples[universe.len() / 2] } else { num / den })
            }
            Self::Bisector => {
                let total: f64 = membership.iter().sum();
                if total.abs() < 1e-12 {
                    return Ok(universe.samples[universe.len() / 2]);
                }
                let half = 0.5 * total;
                let mut cumulative = 0.0;
                for (x, mu) in universe.samples.iter().zip(membership.iter()) {
                    cumulative += mu;
                    if cumulative >= half {
                        return Ok(*x);
                    }
                }
                Ok(*universe.samples.last().unwrap())
            }
            Self::Mom => {
                let max_mu = membership.iter().copied().fold(0.0, f64::max);
                let peaks: Vec<f64> = universe.samples.iter().zip(membership.iter()).filter(|(_, mu)| (**mu - max_mu).abs() < 1e-9).map(|(x, _)| *x).collect();
                Ok(peaks.iter().sum::<f64>() / peaks.len().max(1) as f64)
            }
            Self::Som => Ok(universe.samples[argmax(membership)]),
            Self::Lom => {
                let max_mu = membership.iter().copied().fold(0.0, f64::max);
                let last = universe.samples.iter().zip(membership.iter()).filter(|(_, mu)| (**mu - max_mu).abs() < 1e-9).map(|(x, _)| *x).next_back();
                Ok(last.unwrap_or(universe.samples[universe.len() / 2]))
            }
            Self::WeightedAverage => {
                let heights = rule_heights.ok_or_else(|| FuzzyError::DimensionMismatch("weighted average needs rule heights".into()))?;
                let values = rule_values.ok_or_else(|| FuzzyError::DimensionMismatch("weighted average needs rule values".into()))?;
                let mut num = 0.0;
                let mut den = 0.0;
                for (h, v) in heights.iter().zip(values.iter()) {
                    num += h * v;
                    den += h;
                }
                Ok(if den.abs() < 1e-12 { values[0] } else { num / den })
            }
            Self::Height => {
                let heights = rule_heights.ok_or_else(|| FuzzyError::DimensionMismatch("height defuzz needs rule heights".into()))?;
                let values = rule_values.ok_or_else(|| FuzzyError::DimensionMismatch("height defuzz needs rule values".into()))?;
                Self::WeightedAverage.apply(universe, membership, Some(heights), Some(values))
            }
        }
    }
}
// #endregion 🔖️Defuzzification

// #region 🔖️InferenceEngine
/// ⚙️ Fuzzy inference engine type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum InferenceModel {
    Mamdani,
    Sugeno,
    Tsukamoto,
    Larsen,
    Hybrid,
}

/// 🔥️ Single rule firing trace for explainability.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct RuleTrace {
    pub rule_id: usize,
    pub activation: f64,
    pub contribution: f64,
    pub description: String,
}

/// 🧾️ Explanation of an inference step.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Explanation {
    pub model: InferenceModel,
    pub input_values: Vec<f64>,
    pub output_values: Vec<f64>,
    pub traces: Vec<RuleTrace>,
    pub defuzzifier: Defuzzifier,
    pub rationale: String,
}

/// 🎛️ Multi-input multi-output fuzzy inference system.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct MimoSystem {
    pub inputs: Vec<LinguisticVariable>,
    pub outputs: Vec<LinguisticVariable>,
    pub rules: RuleBase,
    pub model: InferenceModel,
    pub tnorm: TNorm,
    pub tconorm: TConorm,
    pub defuzzifier: Defuzzifier,
}

impl MimoSystem {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn infer(&self, values: &[f64]) -> FuzzyResult<(Vec<f64>, Explanation)> {
        if values.len() != self.inputs.len() {
            return Err(FuzzyError::DimensionMismatch("input count".into()));
        }
        if self.rules.is_empty() {
            return Err(FuzzyError::EmptyRuleBase);
        }
        let mut outputs = vec![0.0; self.outputs.len()];
        let mut traces = Vec::new();
        for (out_idx, output_var) in self.outputs.iter().enumerate() {
            let (value, out_traces) = self.infer_output(out_idx, output_var, values)?;
            outputs[out_idx] = value;
            traces.extend(out_traces);
        }
        let rationale = format!("{} engine produced {} output(s) from {} rule(s)", format!("{:?}", self.model).to_lowercase(), outputs.len(), self.rules.rules.len());
        Ok((outputs.clone(), Explanation { model: self.model, input_values: values.to_vec(), output_values: outputs, traces, defuzzifier: self.defuzzifier, rationale }))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn infer_output(&self, out_idx: usize, output_var: &LinguisticVariable, values: &[f64]) -> FuzzyResult<(f64, Vec<RuleTrace>)> {
        let mut aggregated = vec![0.0; output_var.universe.len()];
        let mut traces = Vec::new();
        let mut sugeno_num = 0.0;
        let mut sugeno_den = 0.0;
        let mut heights = Vec::new();
        let mut rule_values = Vec::new();
        for rule in &self.rules.rules {
            let activation = rule.firing_strength(&self.inputs, values, self.tnorm);
            if activation <= 1e-12 {
                continue;
            }
            match (&self.model, &rule.consequent) {
                (InferenceModel::Mamdani | InferenceModel::Hybrid, Consequent::Mamdani { output, term }) if *output == out_idx => {
                    for (i, x) in output_var.universe.samples.iter().enumerate() {
                        let mu = output_var.terms[*term].grade(*x).min(activation);
                        aggregated[i] = self.tconorm.apply(aggregated[i], mu);
                    }
                    traces.push(RuleTrace { rule_id: rule.id, activation, contribution: activation, description: format!("Mamdani rule {} clipped consequent", rule.id) });
                }
                (InferenceModel::Larsen | InferenceModel::Hybrid, Consequent::Mamdani { output, term }) if *output == out_idx => {
                    for (i, x) in output_var.universe.samples.iter().enumerate() {
                        let base = output_var.terms[*term].grade(*x);
                        let mu = if base <= 1e-12 { 0.0 } else { (activation * base / base.max(activation)).min(1.0) };
                        aggregated[i] = self.tconorm.apply(aggregated[i], mu.min(base * activation));
                    }
                    traces.push(RuleTrace { rule_id: rule.id, activation, contribution: activation, description: format!("Larsen rule {} scaled consequent", rule.id) });
                }
                (InferenceModel::Sugeno | InferenceModel::Hybrid, Consequent::SugenoConstant { output, value }) if *output == out_idx => {
                    sugeno_num += activation * value;
                    sugeno_den += activation;
                    heights.push(activation);
                    rule_values.push(*value);
                    traces.push(RuleTrace { rule_id: rule.id, activation, contribution: activation * value, description: format!("Sugeno constant {:.3}", value) });
                }
                (InferenceModel::Sugeno | InferenceModel::Hybrid, Consequent::SugenoLinear { output, coeffs }) if *output == out_idx => {
                    let mut y = coeffs.first().copied().unwrap_or(0.0);
                    for (i, &v) in values.iter().enumerate() {
                        if i + 1 < coeffs.len() {
                            y += coeffs[i + 1] * v;
                        }
                    }
                    sugeno_num += activation * y;
                    sugeno_den += activation;
                    heights.push(activation);
                    rule_values.push(y);
                    traces.push(RuleTrace { rule_id: rule.id, activation, contribution: activation * y, description: format!("Sugeno linear -> {:.3}", y) });
                }
                (InferenceModel::Tsukamoto | InferenceModel::Hybrid, Consequent::Tsukamoto { output, term }) if *output == out_idx => {
                    let term_mf = &output_var.terms[*term].mf;
                    let crisp = tsukamoto_inverse(term_mf, activation);
                    sugeno_num += activation * crisp;
                    sugeno_den += activation;
                    heights.push(activation);
                    rule_values.push(crisp);
                    traces.push(RuleTrace { rule_id: rule.id, activation, contribution: activation * crisp, description: format!("Tsukamoto inverse -> {:.3}", crisp) });
                }
                (_, Consequent::SoftConstraint { output, term, preference }) if *output == out_idx => {
                    let pref = preference.max(0.0);
                    for (i, x) in output_var.universe.samples.iter().enumerate() {
                        let mu = output_var.terms[*term].grade(*x).min(activation * pref);
                        aggregated[i] = self.tconorm.apply(aggregated[i], mu);
                    }
                    traces.push(RuleTrace { rule_id: rule.id, activation, contribution: activation * pref, description: format!("soft constraint preference {:.3}", pref) });
                }
                _ => {}
            }
        }
        let value = match self.model {
            InferenceModel::Sugeno | InferenceModel::Tsukamoto => {
                if sugeno_den.abs() < 1e-12 {
                    return Err(FuzzyError::NoFiredRules);
                }
                Defuzzifier::WeightedAverage.apply(&output_var.universe, &aggregated, Some(&heights), Some(&rule_values))?
            }
            InferenceModel::Mamdani | InferenceModel::Larsen | InferenceModel::Hybrid => {
                if aggregated.iter().sum::<f64>() < 1e-12 && sugeno_den > 1e-12 {
                    sugeno_num / sugeno_den
                } else {
                    self.defuzzifier.apply(&output_var.universe, &aggregated, None, None)?
                }
            }
        };
        Ok((value, traces))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn tsukamoto_inverse(mf: &MembershipFunction, alpha: f64) -> f64 {
    let alpha = clamp01(alpha);
    if alpha <= 0.0 {
        return mf.support_min();
    }
    let min = mf.support_min();
    let max = mf.support_max();
    let mut lo = min;
    let mut hi = max;
    for _ in 0..64 {
        let mid = 0.5 * (lo + hi);
        if mf.eval(mid) >= alpha {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    0.5 * (lo + hi)
}

/// 🏗️ Hierarchical fuzzy system: layer outputs become next-layer inputs.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct HierarchicalSystem {
    pub layers: Vec<MimoSystem>,
}

impl HierarchicalSystem {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(layers: Vec<MimoSystem>) -> Self {
        Self { layers }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn infer(&self, values: &[f64]) -> FuzzyResult<(Vec<f64>, Vec<Explanation>)> {
        let mut current = values.to_vec();
        let mut explanations = Vec::new();
        for layer in &self.layers {
            let (out, explanation) = layer.infer(&current)?;
            explanations.push(explanation);
            current = out;
        }
        Ok((current, explanations))
    }
}
// #endregion 🔖️InferenceEngine
