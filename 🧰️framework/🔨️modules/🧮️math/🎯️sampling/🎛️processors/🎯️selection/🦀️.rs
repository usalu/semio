//! 🎯️ Selection samplers: greedy, multinomial, Gumbel-max and Gumbel top-k over an alias table.

use semio_framework_math_sampling_vocabulary::{RandomSource, RandomSources, SamplingError, cdf_binary_search};
use semio_framework_math_sampling_config::{Candidate, MultinomialStrategy, TieBreak};
use semio_framework_math_sampling_workspace::{Distribution, SelectionBuffer, StepView};
use super::{TokenSampler, TokenSamplers};

// #region 🔖️Selection
async fn candidate_from(dist: &Distribution<'_>, index: usize) -> Candidate {
    Candidate { token: dist.tokens[index], raw_logit: 0.0, processed_logit: 0.0, prob: dist.probs[index], logprob: dist.logprobs[index], rank: index as u32 }
}

/// 🎯️ Walker's alias method, reimplemented locally (rather than reusing
/// [`semio_framework_geometry::random::AliasTable`]) because that type's `sample` is hard-wired to the
/// concrete `semio_framework_geometry::random::Rng` and cannot accept our `RandomSources` enum.
struct AliasTable {
    prob: Vec<f32>,
    alias: Vec<u32>,
}

impl AliasTable {
    async fn new(weights: &[f32]) -> Self {
        let n = weights.len();
        if n == 0 {
            return Self { prob: vec![1.0], alias: vec![0] };
        }
        let sum: f32 = weights.iter().sum();
        if sum <= 0.0 {
            let mut prob = vec![0.0; n];
            prob[0] = 1.0;
            return Self { prob, alias: vec![0; n] };
        }
        let mut scaled: Vec<f32> = weights.iter().map(|w| w / sum * n as f32).collect();
        let mut small: Vec<usize> = Vec::new();
        let mut large: Vec<usize> = Vec::new();
        for (i, &p) in scaled.iter().enumerate() {
            if p < 1.0 {
                small.push(i);
            } else {
                large.push(i);
            }
        }
        let mut prob = vec![0.0f32; n];
        let mut alias = vec![0u32; n];
        while !small.is_empty() && !large.is_empty() {
            let s = small.pop().expect("checked non-empty above");
            let l = large.pop().expect("checked non-empty above");
            prob[s] = scaled[s];
            alias[s] = l as u32;
            scaled[l] = scaled[l] + scaled[s] - 1.0;
            if scaled[l] < 1.0 {
                small.push(l);
            } else {
                large.push(l);
            }
        }
        for l in large {
            prob[l] = 1.0;
        }
        for s in small {
            prob[s] = 1.0;
        }
        Self { prob, alias }
    }

    async fn sample(&self, rng: &mut RandomSources) -> usize {
        let n = self.prob.len();
        let i = rng.next_range(0, n as u64).await as usize;
        if rng.next_f64().await < self.prob[i] as f64 {
            i
        } else {
            self.alias[i] as usize
        }
    }
}

/// 🎯️ Deterministic argmax selection over the (already prob-sorted) [`Distribution`].
pub struct GreedySampler {
    pub tie_break: TieBreak,
}

impl TokenSampler for GreedySampler {
    async fn name(&self) -> &'static str {
        "greedy"
    }
    async fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, _rng: &mut RandomSources, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
        if dist.tokens.is_empty() {
            return Err(SamplingError::EmptyDistribution);
        }
        let max_prob = dist.probs[0];
        let tie_epsilon = f32::EPSILON.max(max_prob.abs() * 1e-6);
        let mut end = 0usize;
        while end + 1 < dist.probs.len() && (dist.probs[end + 1] - max_prob).abs() <= tie_epsilon {
            end += 1;
        }
        let idx = match self.tie_break {
            TieBreak::HighestTokenId => end,
            TieBreak::LowestTokenId | TieBreak::FirstSeen => 0,
        };
        out.chosen.push(candidate_from(dist, idx).await);
        Ok(())
    }
    async fn fork(&self) -> TokenSamplers {
        (Self { tie_break: self.tie_break }).into()
    }
}

/// 🎯️ Samples one token proportional to the live distribution via the configured strategy.
pub struct MultinomialSampler {
    pub strategy: MultinomialStrategy,
}

impl TokenSampler for MultinomialSampler {
    async fn name(&self) -> &'static str {
        "multinomial"
    }
    async fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut RandomSources, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
        if dist.tokens.is_empty() {
            return Err(SamplingError::EmptyDistribution);
        }
        let idx = match self.strategy {
            MultinomialStrategy::CdfBinarySearch => cdf_binary_search(dist.cdf, rng.next_f64().await).await,
            MultinomialStrategy::LinearScan => {
                let u = rng.next_f64().await;
                let mut cumulative = 0.0f64;
                let mut idx = dist.probs.len() - 1;
                for (i, &p) in dist.probs.iter().enumerate() {
                    cumulative += p as f64;
                    if cumulative >= u {
                        idx = i;
                        break;
                    }
                }
                idx
            }
            MultinomialStrategy::Alias => {
                let table = AliasTable::new(dist.probs);
                table.await.sample(rng).await
            }
        };
        out.chosen.push(candidate_from(dist, idx).await);
        Ok(())
    }
    async fn fork(&self) -> TokenSamplers {
        (Self { strategy: self.strategy }).into()
    }
}

/// 🎯️ Gumbel-max trick: `argmax(logprob_i + Gumbel_i)`, statistically equivalent to multinomial
/// sampling but usable when only a perturb-then-argmax primitive is available.
pub struct GumbelMaxSampler;

impl TokenSampler for GumbelMaxSampler {
    async fn name(&self) -> &'static str {
        "gumbel_max"
    }
    async fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut RandomSources, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
        if dist.tokens.is_empty() {
            return Err(SamplingError::EmptyDistribution);
        }
        let mut best = 0usize;
        let mut best_score = f64::NEG_INFINITY;
        for i in 0..dist.tokens.len() {
            let score = dist.logprobs[i] as f64 + rng.gumbel().await;
            if score > best_score {
                best_score = score;
                best = i;
            }
        }
        out.chosen.push(candidate_from(dist, best).await);
        Ok(())
    }
    async fn fork(&self) -> TokenSamplers {
        (Self).into()
    }
}

/// 🎯️ Gumbel-top-k: `k` tokens without replacement, drawn by taking the top `k` of `logprob_i +
/// Gumbel_i`.
pub struct GumbelTopKSampler {
    pub k: usize,
}

impl TokenSampler for GumbelTopKSampler {
    async fn name(&self) -> &'static str {
        "gumbel_top_k"
    }
    async fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut RandomSources, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
        if dist.tokens.is_empty() {
            return Err(SamplingError::EmptyDistribution);
        }
        let k = self.k.min(dist.tokens.len());
        let mut scored: Vec<(f64, usize)> = Vec::with_capacity(dist.tokens.len());
        for i in 0..dist.tokens.len() {
            scored.push((dist.logprobs[i] as f64 + rng.gumbel().await, i));
        }
        scored.sort_unstable_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(core::cmp::Ordering::Equal).then_with(|| dist.tokens[a.1].cmp(&dist.tokens[b.1])));
        for &(_, i) in scored.iter().take(k) {
            out.chosen.push(candidate_from(dist, i).await);
        }
        Ok(())
    }
    async fn fork(&self) -> TokenSamplers {
        (Self { k: self.k }).into()
    }
}
// #endregion 🔖️Selection
