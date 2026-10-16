//! 🌡️ Warpers: temperature, top-k, top-p, min-p, typical, tail-free, cutoffs and truncations.

use semio_framework_math_sampling_vocabulary::{SamplingError, entropy_nats};
use semio_framework_math_sampling_config::Schedule;
use semio_framework_math_sampling_workspace::{LogitsWorkspace, ProcessorKind, StepView, schedule_input};
use super::LogitsProcessor;

// #region 🔖️Warpers
/// 🌡️ `logit / temperature`; `temperature <= 0` collapses to greedy via [`LogitsWorkspace::collapse_live_to_argmax`].
pub struct Temperature {
    pub value: Schedule,
}

impl LogitsProcessor for Temperature {
    async fn name(&self) -> &'static str {
        "temperature"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let value = self.value.eval(schedule_input(view).await).await;
        if value <= 0.0 {
            ws.collapse_live_to_argmax().await;
        } else {
            ws.scale_processed_over_live(1.0 / value as f32).await;
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { value: self.value.clone() }).into()
    }
}

/// 🌡️ Temperature whose value tracks the live set's current entropy: `clamp(base + gain *
/// entropy, min, max)`.
pub struct DynamicTemperature {
    pub base: Schedule,
    pub entropy_gain: f64,
    pub min: f64,
    pub max: f64,
}

impl LogitsProcessor for DynamicTemperature {
    async fn name(&self) -> &'static str {
        "dynamic_temperature"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.softmax_over_live().await;
        let entropy = entropy_nats(ws.probs().await).await;
        let temp = (self.base.eval(schedule_input(view).await).await + self.entropy_gain * entropy).clamp(self.min, self.max);
        if temp <= 0.0 {
            ws.collapse_live_to_argmax().await;
        } else {
            ws.scale_processed_over_live(1.0 / temp as f32).await;
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { base: self.base.clone(), entropy_gain: self.entropy_gain, min: self.min, max: self.max }).into()
    }
}

/// 🌡️ Keeps exactly the top `k` live tokens by probability.
pub struct TopK {
    pub k: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TopK {
    async fn name(&self) -> &'static str {
        "top_k"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let k = self.k.eval(schedule_input(view).await).await.round().max(1.0) as usize;
        ws.sort_live_by_prob_desc().await;
        ws.truncate_live_to(k, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { k: self.k.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Nucleus sampling: retains the smallest prefix (by descending probability) whose cumulative
/// mass reaches `p`.
pub struct TopP {
    pub p: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TopP {
    async fn name(&self) -> &'static str {
        "top_p"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let p = self.p.eval(schedule_input(view).await).await.clamp(0.0, 1.0);
        ws.sort_live_by_prob_desc().await;
        let probs = ws.probs().await;
        let mut cumulative = 0.0f64;
        let mut keep = probs.len();
        for (i, &pr) in probs.iter().enumerate() {
            cumulative += pr as f64;
            if cumulative >= p {
                keep = i + 1;
                break;
            }
        }
        ws.truncate_live_to(keep, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { p: self.p.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Retains tokens whose probability is at least `p` times the live set's maximum probability.
pub struct MinP {
    pub p: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for MinP {
    async fn name(&self) -> &'static str {
        "min_p"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let p = self.p.eval(schedule_input(view).await).await.clamp(0.0, 1.0) as f32;
        ws.sort_live_by_prob_desc().await;
        let probs = ws.probs().await;
        let threshold = probs.first().copied().unwrap_or(0.0) * p;
        let keep = probs.iter().take_while(|&&pr| pr >= threshold).count().max(1);
        ws.truncate_live_to(keep, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { p: self.p.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Shared implementation for [`Typical`] and [`LocallyTypical`]: ranks tokens by absolute
/// deviation of their surprisal (`-ln p`) from the live set's entropy and retains the smallest
/// such prefix whose cumulative mass reaches `mass`.
async fn apply_typical_truncation(ws: &mut LogitsWorkspace, mass: f64, min_keep: usize) {
    ws.softmax_over_live().await;
    let n = ws.live().await.len();
    if n == 0 {
        return;
    }
    let probs = ws.probs().await.to_vec();
    let entropy = entropy_nats(&probs).await;
    let live = ws.live().await.to_vec();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| {
        let da = (-(probs[a] as f64).ln() - entropy).abs();
        let db = (-(probs[b] as f64).ln() - entropy).abs();
        da.partial_cmp(&db).unwrap_or(core::cmp::Ordering::Equal).then_with(|| live[a].cmp(&live[b]))
    });
    let mut cumulative = 0.0f64;
    let mut keep = 0usize;
    for &i in &order {
        cumulative += probs[i] as f64;
        keep += 1;
        if cumulative >= mass {
            break;
        }
    }
    keep = keep.max(min_keep.min(n));
    let kept: Vec<u32> = order[..keep].iter().map(|&i| live[i]).collect();
    ws.set_live(kept).await;
}

/// 🌡️ Locally typical sampling (global variant): see [`apply_typical_truncation`].
pub struct Typical {
    pub mass: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for Typical {
    async fn name(&self) -> &'static str {
        "typical"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let mass = self.mass.eval(schedule_input(view).await).await.clamp(0.0, 1.0);
        apply_typical_truncation(ws, mass, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { mass: self.mass.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Locally typical sampling; this single-step engine applies the same rule as [`Typical`]
/// (the "local" distinction only matters across multiple denoising passes, not one logits step).
pub struct LocallyTypical {
    pub mass: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for LocallyTypical {
    async fn name(&self) -> &'static str {
        "locally_typical"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let mass = self.mass.eval(schedule_input(view).await).await.clamp(0.0, 1.0);
        apply_typical_truncation(ws, mass, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { mass: self.mass.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Tail-free sampling: cuts where the cumulative normalized second derivative of the sorted
/// probability curve reaches `z`.
pub struct TailFree {
    pub z: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TailFree {
    async fn name(&self) -> &'static str {
        "tail_free"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let z = self.z.eval(schedule_input(view).await).await.clamp(0.0, 1.0);
        ws.sort_live_by_prob_desc().await;
        let probs = ws.probs().await.to_vec();
        let n = probs.len();
        if n < 3 {
            return Ok(());
        }
        let first_deriv: Vec<f32> = probs.windows(2).map(|w| (w[0] - w[1]).abs()).collect();
        let second_deriv: Vec<f32> = first_deriv.windows(2).map(|w| (w[0] - w[1]).abs()).collect();
        let total: f32 = second_deriv.iter().sum();
        let mut keep = n;
        if total > 0.0 {
            let mut cumulative = 0.0f32;
            for (i, &d) in second_deriv.iter().enumerate() {
                cumulative += d / total;
                if cumulative as f64 >= z {
                    keep = i + 2;
                    break;
                }
            }
        }
        ws.truncate_live_to(keep, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { z: self.z.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Drops any token with probability below an absolute `cutoff`.
pub struct EpsilonCutoff {
    pub cutoff: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for EpsilonCutoff {
    async fn name(&self) -> &'static str {
        "epsilon"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let cutoff = self.cutoff.eval(schedule_input(view).await).await.max(0.0);
        ws.sort_live_by_prob_desc().await;
        let probs = ws.probs().await;
        let keep = probs.iter().take_while(|&&p| p as f64 >= cutoff).count().max(1);
        ws.truncate_live_to(keep, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { cutoff: self.cutoff.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Entropy-adaptive cutoff: `eta = min(epsilon, sqrt(epsilon) * exp(-entropy))`.
pub struct EtaCutoff {
    pub cutoff: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for EtaCutoff {
    async fn name(&self) -> &'static str {
        "eta"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let epsilon = self.cutoff.eval(schedule_input(view).await).await.max(1e-12);
        ws.sort_live_by_prob_desc().await;
        let probs = ws.probs().await.to_vec();
        let entropy = entropy_nats(&probs).await;
        let eta = epsilon.min(epsilon.sqrt() * (-entropy).exp());
        let keep = probs.iter().take_while(|&&p| p as f64 >= eta).count().max(1);
        ws.truncate_live_to(keep, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { cutoff: self.cutoff.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Keeps tokens with `prob >= power * max_prob^2`.
pub struct TopA {
    pub power: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TopA {
    async fn name(&self) -> &'static str {
        "top_a"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let a = self.power.eval(schedule_input(view).await).await.max(0.0);
        ws.sort_live_by_prob_desc().await;
        let probs = ws.probs().await;
        let max_p = probs.first().copied().unwrap_or(0.0) as f64;
        let threshold = a * max_p * max_p;
        let keep = probs.iter().take_while(|&&p| p as f64 >= threshold).count().max(1);
        ws.truncate_live_to(keep, self.min_keep).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { power: self.power.clone(), min_keep: self.min_keep }).into()
    }
}

/// 🌡️ Keeps only the top `max_rank` tokens by probability (a fixed-count variant of [`TopK`]
/// used when rank alone, not a schedule, determines the cutoff).
pub struct RankTruncation {
    pub max_rank: usize,
}

impl LogitsProcessor for RankTruncation {
    async fn name(&self) -> &'static str {
        "rank_truncation"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.sort_live_by_prob_desc().await;
        ws.truncate_live_to(self.max_rank, 1).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { max_rank: self.max_rank }).into()
    }
}

/// 🌡️ Grows the kept prefix (by descending probability) until the renormalized subset's entropy
/// drops to `target_entropy`, or its effective candidate count reaches `target_effective_count`.
pub struct AdaptiveTruncation {
    pub target_entropy: Option<f64>,
    pub target_effective_count: Option<f64>,
}

impl LogitsProcessor for AdaptiveTruncation {
    async fn name(&self) -> &'static str {
        "adaptive_truncation"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.sort_live_by_prob_desc().await;
        let probs = ws.probs().await.to_vec();
        let n = probs.len();
        let mut keep = n;
        let mut cumulative = 0.0f64;
        'search: for (i, &p) in probs.iter().enumerate() {
            cumulative += p as f64;
            let mut renorm_entropy = 0.0f64;
            for &q in &probs[..=i] {
                let r = q as f64 / cumulative;
                if r > 0.0 {
                    renorm_entropy -= r * r.ln();
                }
            }
            if let Some(target) = self.target_effective_count {
                if renorm_entropy.exp() >= target {
                    keep = i + 1;
                    break 'search;
                }
            }
            if let Some(target) = self.target_entropy {
                if renorm_entropy <= target {
                    keep = i + 1;
                    break 'search;
                }
            }
        }
        ws.truncate_live_to(keep, 1).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { target_entropy: self.target_entropy, target_effective_count: self.target_effective_count }).into()
    }
}

// #endregion 🔖️Warpers
