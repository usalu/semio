//! 🛰️ Temporal, spatial, hybrid and fan: the temporal and spatial evaluators, the probabilistic and possibilistic conversions, the hybrid fusion and the fan controller. See [`super`].

use super::*;

// #region 🔖️TemporalSpatial
/// ⏱️ Temporal fuzzy evaluator for vague time concepts.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct TemporalEvaluator {
    pub now: f64,
    pub recent_window: f64,
    pub frequent_window: f64,
}

impl TemporalEvaluator {
    pub fn recently(&self, timestamp: f64) -> f64 {
        let dt = (self.now - timestamp).max(0.0);
        clamp01(1.0 - dt / self.recent_window.max(1e-6))
    }

    pub fn frequently(&self, timestamps: &[f64]) -> f64 {
        let count = timestamps.iter().filter(|t| (self.now - **t).abs() <= self.frequent_window).count() as f64;
        clamp01(count / 5.0)
    }
}

/// 📍️ Spatial fuzzy evaluator for proximity concepts.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct SpatialEvaluator {
    pub anchor: Vec<f64>,
    pub near_radius: f64,
}

impl SpatialEvaluator {
    pub fn near(&self, point: &[f64]) -> f64 {
        let dist2: f64 = self.anchor.iter().zip(point.iter()).map(|(a, p)| (a - p).powi(2)).sum();
        clamp01(1.0 - dist2.sqrt() / self.near_radius.max(1e-6))
    }

    pub fn slowly(&self, velocity: f64, max_speed: f64) -> f64 {
        clamp01(1.0 - velocity.abs() / max_speed.max(1e-6))
    }
}
// #endregion 🔖️TemporalSpatial

// #region 🔖️HybridUncertainty
/// 🔀️ Map probabilistic scores to fuzzy membership without external Bayesian engines.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn probabilistic_to_membership(probability: f64, certainty: f64) -> f64 {
    clamp01(probability * certainty + 0.5 * (1.0 - certainty))
}

/// 🔀️ Bridge possibility and fuzzy membership on a shared universe.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn possibility_to_membership(possibility: &PossibilityMeasure) -> Vec<f64> {
    possibility.membership.clone()
}

/// 🔀️ Combine fuzzy and possibility evidence via weighted fusion.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn hybrid_fuse(membership: f64, possibility: f64, fuzzy_weight: f64) -> f64 {
    let w = clamp01(fuzzy_weight);
    clamp01(w * membership + (1.0 - w) * possibility)
}
// #endregion 🔖️HybridUncertainty

// #region 🔖️FanController
/// 🌀️ Fan-speed controller builder demonstrating the full advanced fuzzy pipeline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct FanController {
    pub temperature_var: LinguisticVariable,
    pub speed_var: LinguisticVariable,
    pub sensor_uncertainty: IntervalType2Set,
    pub evolving: EvolvingFuzzySystem,
}

impl FanController {
    pub fn from_sensor_data(temps: &[f64], fan_speeds: &[f64]) -> FuzzyResult<Self> {
        let t_min = temps.iter().copied().fold(f64::INFINITY, f64::min);
        let t_max = temps.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let s_max = fan_speeds.iter().copied().fold(0.0_f64, f64::max).max(1.0);
        let temp_univ = Universe::new(t_min - 1.0, t_max + 1.0, 101)?;
        let speed_univ = Universe::new(0.0, s_max * 1.1, 101)?;
        let mut low_mf = MembershipFunction::triangular(t_min, t_min, (t_min + t_max) * 0.5);
        let mut high_mf = MembershipFunction::triangular((t_min + t_max) * 0.5, t_max, t_max);
        let low_samples: Vec<(f64, f64)> = temps.iter().map(|&t| (t, if t < (t_min + t_max) * 0.5 { 1.0 } else { 0.0 })).collect();
        let high_samples: Vec<(f64, f64)> = temps.iter().map(|&t| (t, if t >= (t_min + t_max) * 0.5 { 1.0 } else { 0.0 })).collect();
        let mut low_adapt = AdaptiveMembership::new(low_mf, 0.01);
        low_adapt.fit(&low_samples, 20);
        low_mf = low_adapt.mf;
        let mut high_adapt = AdaptiveMembership::new(high_mf, 0.01);
        high_adapt.fit(&high_samples, 20);
        high_mf = high_adapt.mf;
        let temperature_var = LinguisticVariable::new("temperature", temp_univ, vec![FuzzySet::new("low", low_mf), FuzzySet::new("high", high_mf)]);
        let speed_var = LinguisticVariable::new("fan_speed", speed_univ, vec![FuzzySet::new("slow", MembershipFunction::triangular(0.0, 0.0, s_max * 0.5)), FuzzySet::new("fast", MembershipFunction::triangular(s_max * 0.4, s_max, s_max))]);
        let sensor_uncertainty = IntervalType2Set::new("temperature_sensor", MembershipFunction::gaussian((t_min + t_max) * 0.5, (t_max - t_min) * 0.05), MembershipFunction::gaussian((t_min + t_max) * 0.5, (t_max - t_min) * 0.12));
        let rules = RuleBase::new(vec![
            Rule { id: 0, antecedents: vec![AntecedentClause { input: 0, term: 1, hedge: None }], consequent: Consequent::Mamdani { output: 0, term: 1 }, weight: 0.9, confidence: 0.85 },
            Rule { id: 1, antecedents: vec![AntecedentClause { input: 0, term: 0, hedge: None }], consequent: Consequent::Mamdani { output: 0, term: 0 }, weight: 0.8, confidence: 0.8 },
        ]);
        let system = MimoSystem { inputs: vec![temperature_var.clone()], outputs: vec![speed_var.clone()], rules, model: InferenceModel::Mamdani, tnorm: TNorm::Min, tconorm: TConorm::Max, defuzzifier: Defuzzifier::Centroid };
        let evolving = EvolvingFuzzySystem::new(system, 0.05, 0.1, 8);
        Ok(Self { temperature_var, speed_var, sensor_uncertainty, evolving })
    }

    pub fn decide(&mut self, temperature: f64) -> FuzzyResult<(f64, Explanation)> {
        let (lo, hi) = self.sensor_uncertainty.grade_interval(temperature);
        let adjusted = 0.5 * (temperature + self.sensor_uncertainty.type_reduced_centroid(&[lo, temperature, hi]));
        self.evolving.observe(&[adjusted], &[0.0])?;
        let (outputs, explanation) = self.evolving.system.infer(&[adjusted])?;
        Ok((outputs[0], explanation))
    }
}
// #endregion 🔖️FanController
