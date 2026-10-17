//! 📊️ Step results of a sampling step: the candidate, the top logprobs, the step diagnostics and the sampling result. See [`super`].

use super::*;

// #region 🔖️Candidates
/// 🏅️ One scored token, at whatever pipeline stage produced it (pre- or post-truncation, or the
/// final selected token).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Candidate {
    pub token: TokenId,
    pub raw_logit: f32,
    pub processed_logit: f32,
    pub prob: f32,
    pub logprob: f32,
    pub rank: u32,
}

/// 🏅️ Alternative log-probabilities reported alongside a selection, before and/or after
/// truncation warpers ran (per [`LogprobsSpec`]).
#[derive(Clone, PartialEq, Debug, Default)]
pub struct TopLogprobs {
    pub pre_truncation: Vec<Candidate>,
    pub post_truncation: Vec<Candidate>,
}

/// 🏅️ Optional per-step numerical/pipeline diagnostics (only populated when
/// [`DiagnosticsSpec::enabled`]).
#[derive(Clone, PartialEq, Debug, Default)]
pub struct StepDiagnostics {
    pub entropy: f64,
    pub effective_count: f64,
    pub truncation_mass: f64,
    pub masked_by: Vec<(&'static str, u32)>,
    pub timings_ns: Vec<(&'static str, u64)>,
    pub fallback: Option<FallbackAction>,
    pub health: Option<DistributionHealth>,
}

/// 🏅️ Everything one [`sample_step_stateless`] call (or, later, a stateful engine step) returns.
#[derive(Clone, PartialEq, Debug)]
pub struct SamplingResult {
    pub token: TokenId,
    pub logprob: f32,
    pub finish: Option<FinishReason>,
    pub alternatives: Vec<Candidate>,
    pub top_logprobs: Option<TopLogprobs>,
    pub rng_stream: StreamKey,
    pub diagnostics: Option<StepDiagnostics>,
}
// #endregion 🔖️Candidates
