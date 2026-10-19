//! 🧮️ Numerics of sampling: the logit sanitation, the Kahan sum, the live softmax, log-sum-exp, entropy, candidate count, truncation mass and distribution health. See [`super`].

use super::*;

// #region 🔖️Numerics
/// 📐️ How non-finite input logits are handled before any processor runs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SanitizePolicy {
    /// 📐️ Any NaN or +Inf logit is a hard [`SamplingError::NonFiniteLogits`].
    Error,
    /// 📐️ NaN and -Inf both collapse to `f32::NEG_INFINITY` (effectively masked); +Inf is rejected
    /// (an infinitely-preferred token needs an explicit decision, not a silent uniform pick).
    #[default]
    NegInfNan,
    /// 📐️ NaN collapses to `f32::NEG_INFINITY`; +Inf clamps to `f32::MAX`.
    ClampInf,
}

/// 📐️ Applies `policy` to `logits` in place. Returns the count of entries that were altered, for
/// diagnostics.
pub async fn sanitize_logits(logits: &mut [f32], policy: SanitizePolicy) -> Result<usize, SamplingError> {
    let mut altered = 0usize;
    for (i, l) in logits.iter_mut().enumerate() {
        if l.is_nan() {
            match policy {
                SanitizePolicy::Error => return Err(SamplingError::NonFiniteLogits { index: i }),
                SanitizePolicy::NegInfNan | SanitizePolicy::ClampInf => {
                    *l = f32::NEG_INFINITY;
                    altered += 1;
                }
            }
        } else if *l == f32::INFINITY {
            match policy {
                SanitizePolicy::Error | SanitizePolicy::NegInfNan => return Err(SamplingError::NonFiniteLogits { index: i }),
                SanitizePolicy::ClampInf => {
                    *l = f32::MAX;
                    altered += 1;
                }
            }
        } else if *l == f32::NEG_INFINITY {}
    }
    Ok(altered)
}

/// 📐️ Accumulation precision for softmax/entropy computation.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Accum {
    F32,
    #[default]
    F64,
}

/// 📐️ Compensated (Kahan-Neumaier) running sum — keeps `f64` summation accurate over long live
/// candidate lists where naive summation would otherwise lose low-order bits.
#[derive(Clone, Copy, Default, Debug)]
pub struct KahanSum {
    pub(super) sum: f64,
    correction: f64,
}

impl KahanSum {
    pub async fn new() -> Self {
        Self::default()
    }

    pub async fn add(&mut self, value: f64) {
        let t = self.sum + value;
        if self.sum.abs() >= value.abs() {
            self.correction += (self.sum - t) + value;
        } else {
            self.correction += (value - t) + self.sum;
        }
        self.sum = t;
    }

    pub async fn value(&self) -> f64 {
        self.sum + self.correction
    }
}

/// 📐️ Numerically stable softmax over `logits[live]`, writing normalized probabilities into
/// `probs[..live.len()]`. Uses max-subtraction so large logits never overflow `exp`, and
/// [`KahanSum`] under [`Accum::F64`] so the sum stays close to `1.0` even for wide vocabularies.
pub async fn softmax_live(logits: &[f32], live: &[u32], probs: &mut [f32], accum: Accum) -> f64 {
    debug_assert_eq!(live.len(), probs.len());
    if live.is_empty() {
        return 0.0;
    }
    let max_logit = live.iter().map(|&i| logits[i as usize]).fold(f32::NEG_INFINITY, f32::max);
    match accum {
        Accum::F32 => {
            let mut sum = 0.0f32;
            for (slot, &i) in live.iter().enumerate() {
                let e = (logits[i as usize] - max_logit).exp();
                probs[slot] = e;
                sum += e;
            }
            let inv = if sum > 0.0 { 1.0 / sum } else { 0.0 };
            for p in probs.iter_mut() {
                *p *= inv;
            }
            sum as f64
        }
        Accum::F64 => {
            let mut sum = KahanSum::new().await;
            let mut exps = vec![0.0f64; live.len()];
            for (slot, &i) in live.iter().enumerate() {
                let e = ((logits[i as usize] - max_logit) as f64).exp();
                exps[slot] = e;
                sum.add(e).await;
            }
            let total = sum.value().await;
            let inv = if total > 0.0 { 1.0 / total } else { 0.0 };
            for (slot, p) in probs.iter_mut().enumerate() {
                *p = (exps[slot] * inv) as f32;
            }
            total
        }
    }
}

/// 📐️ `log(sum(exp(values)))` computed via max-subtraction, safe for arbitrarily negative/large
/// inputs (including `-inf` entries, which contribute nothing).
pub async fn logsumexp_f64(values: &[f64]) -> f64 {
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !max.is_finite() {
        return max;
    }
    let mut sum = KahanSum::new().await;
    for &v in values {
        sum.add((v - max).exp()).await;
    }
    max + sum.value().await.ln()
}

/// 📐️ Shannon entropy in nats of a probability vector (`0 ln 0 := 0` by convention).
pub async fn entropy_nats(probs: &[f32]) -> f64 {
    let mut sum = KahanSum::new().await;
    for &p in probs {
        let p = p as f64;
        if p > 0.0 {
            sum.add(-p * p.ln()).await;
        }
    }
    sum.value().await
}

/// 📐️ `exp(entropy)`: the "effective" number of roughly-equally-likely candidates, matching the
/// perplexity of the live distribution.
pub async fn effective_candidate_count(probs: &[f32]) -> f64 {
    entropy_nats(probs).await.exp()
}

/// 📐️ Total probability mass removed by comparing a pre-truncation and post-truncation live-prob
/// vector that share the same normalization base (`1.0 - post_sum` when `post` sums the surviving
/// portion of a `pre` distribution that summed to `1.0`).
pub async fn truncation_mass(pre_kept_sum: f64) -> f64 {
    (1.0 - pre_kept_sum).max(0.0)
}

/// 📐️ Health flags for a live distribution, computed once per step for diagnostics and to decide
/// whether the fallback ladder must engage.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DistributionHealth {
    pub live_count: usize,
    pub prob_sum: f64,
    pub is_degenerate: bool,
}

impl DistributionHealth {
    pub async fn assess(live_count: usize, prob_sum: f64) -> Self {
        Self { live_count, prob_sum, is_degenerate: live_count == 0 || prob_sum.is_nan() || prob_sum <= 0.0 }
    }
}
// #endregion 🔖️Numerics
