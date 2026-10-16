//! 🌀️ Adaptive controllers: Mirostat, entropy PID, repetition and confidence control.

use semio_framework_math_sampling_vocabulary::{SamplingError, TokenId, entropy_nats};
use semio_framework_math_sampling_config::MirostatVersion;
use semio_framework_math_sampling_workspace::{LogitsWorkspace, ProcessorKind, StateMark, StepView};
use super::LogitsProcessor;

// #region 🔖️Adaptive
/// 🌀️ Mirostat v1/v2: adapts the truncation cutoff step by step so the *observed* surprise of
/// selected tokens tracks `target_surprise`. `last_probs` caches the just-processed live
/// distribution so [`Mirostat::commit`] can look up the selected token's probability without the
/// engine needing to pass it explicitly (the [`LogitsProcessor::commit`] signature only carries
/// the token id).
pub struct Mirostat {
    pub version: MirostatVersion,
    pub target_surprise: f64,
    pub learning_rate: f64,
    pub(super) mu: f64,
    last_probs: std::collections::HashMap<TokenId, f32>,
    mu_snapshots: Vec<f64>,
}

impl Mirostat {
    pub async fn new(version: MirostatVersion, target_surprise: f64, learning_rate: f64) -> Self {
        Self { version, target_surprise, learning_rate, mu: target_surprise * 2.0, last_probs: std::collections::HashMap::new(), mu_snapshots: Vec::new() }
    }
}

impl LogitsProcessor for Mirostat {
    async fn name(&self) -> &'static str {
        "mirostat"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.sort_live_by_prob_desc().await;
        let keep = match self.version {
            // 🌀️ v1: candidate-set size grows/shrinks as `2^mu` (a power-law-fit proxy).
            MirostatVersion::V1 => (2.0f64).powf(self.mu).round().max(1.0) as usize,
            // 🌀️ v2: keep every token whose surprise (`-log2 p`) stays within `mu` of the target.
            MirostatVersion::V2 => {
                let probs = ws.probs().await;
                probs.iter().take_while(|&&p| -(p as f64).log2() <= self.mu).count().max(1)
            }
        };
        ws.truncate_live_to(keep, 1).await;
        self.last_probs.clear();
        for (&token_idx, &p) in ws.live().await.iter().zip(ws.probs().await.iter()) {
            self.last_probs.insert(TokenId::new(token_idx), p);
        }
        Ok(())
    }
    async fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        self.mu_snapshots.push(self.mu);
        if let Some(&p) = self.last_probs.get(&token) {
            let surprise = -(p as f64).log2();
            self.mu -= self.learning_rate * (surprise - self.target_surprise);
        }
    }
    async fn save(&mut self) -> StateMark {
        StateMark(self.mu_snapshots.len() as u64)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        self.mu_snapshots.truncate(mark.0 as usize);
        self.mu = self.mu_snapshots.last().copied().unwrap_or(self.target_surprise * 2.0);
    }
    async fn reset(&mut self) {
        self.mu = self.target_surprise * 2.0;
        self.mu_snapshots.clear();
        self.last_probs.clear();
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { version: self.version, target_surprise: self.target_surprise, learning_rate: self.learning_rate, mu: self.mu, last_probs: self.last_probs.clone(), mu_snapshots: self.mu_snapshots.clone() }).into()
    }
}

/// 🌀️ PID controller driving temperature toward a target entropy.
pub struct EntropyPid {
    pub target: f64,
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    integral: f64,
    last_error: f64,
    pending_error: f64,
    history: Vec<(f64, f64)>,
}

impl EntropyPid {
    pub async fn new(target: f64, kp: f64, ki: f64, kd: f64) -> Self {
        Self { target, kp, ki, kd, integral: 0.0, last_error: 0.0, pending_error: 0.0, history: Vec::new() }
    }
}

impl LogitsProcessor for EntropyPid {
    async fn name(&self) -> &'static str {
        "entropy_pid"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.softmax_over_live().await;
        let entropy = entropy_nats(ws.probs().await).await;
        self.pending_error = self.target - entropy;
        let temp = (1.0 + self.kp * self.pending_error + self.ki * self.integral + self.kd * (self.pending_error - self.last_error)).max(0.05);
        ws.scale_processed_over_live(1.0 / temp as f32).await;
        Ok(())
    }
    async fn commit(&mut self, _view: &StepView<'_>, _token: TokenId) {
        self.history.push((self.integral, self.last_error));
        self.integral += self.pending_error;
        self.last_error = self.pending_error;
    }
    async fn save(&mut self) -> StateMark {
        StateMark(self.history.len() as u64)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        self.history.truncate(mark.0 as usize);
        let (integral, last_error) = self.history.last().copied().unwrap_or((0.0, 0.0));
        self.integral = integral;
        self.last_error = last_error;
    }
    async fn reset(&mut self) {
        self.integral = 0.0;
        self.last_error = 0.0;
        self.pending_error = 0.0;
        self.history.clear();
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { target: self.target, kp: self.kp, ki: self.ki, kd: self.kd, integral: self.integral, last_error: self.last_error, pending_error: self.pending_error, history: self.history.clone() }).into()
    }
}

/// 🌀️ Flattens the distribution (raises effective temperature) once the fraction of repeated
/// tokens in the trailing `window` exceeds `threshold`.
pub struct RepetitionController {
    pub window: usize,
    pub threshold: f64,
    pub boost: f64,
    recent: std::collections::VecDeque<TokenId>,
    snapshots: Vec<std::collections::VecDeque<TokenId>>,
}

impl RepetitionController {
    pub async fn new(window: usize, threshold: f64, boost: f64) -> Self {
        Self { window, threshold, boost, recent: std::collections::VecDeque::new(), snapshots: Vec::new() }
    }

    async fn repetition_ratio(&self) -> f64 {
        if self.recent.is_empty() {
            return 0.0;
        }
        let unique: std::collections::HashSet<&TokenId> = self.recent.iter().collect();
        1.0 - (unique.len() as f64 / self.recent.len() as f64)
    }
}

impl LogitsProcessor for RepetitionController {
    async fn name(&self) -> &'static str {
        "repetition_controller"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        if self.repetition_ratio().await > self.threshold {
            ws.scale_processed_over_live(1.0 / (1.0 + self.boost as f32)).await;
        }
        Ok(())
    }
    async fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        self.recent.push_back(token);
        if self.recent.len() > self.window {
            self.recent.pop_front();
        }
    }
    async fn save(&mut self) -> StateMark {
        self.snapshots.push(self.recent.clone());
        StateMark((self.snapshots.len() - 1) as u64)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        if let Some(recent) = self.snapshots.get(mark.0 as usize) {
            self.recent = recent.clone();
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    async fn reset(&mut self) {
        self.recent.clear();
        self.snapshots.clear();
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { window: self.window, threshold: self.threshold, boost: self.boost, recent: self.recent.clone(), snapshots: self.snapshots.clone() }).into()
    }
}

/// 🌀️ Interpolates temperature between `low_temp` (near-greedy, at or below `low_entropy`) and
/// `high_temp` (broad, at or above `high_entropy`) based on the live set's current entropy.
pub struct ConfidenceController {
    pub low_entropy: f64,
    pub high_entropy: f64,
    pub low_temp: f64,
    pub high_temp: f64,
}

impl LogitsProcessor for ConfidenceController {
    async fn name(&self) -> &'static str {
        "confidence_controller"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.softmax_over_live().await;
        let entropy = entropy_nats(ws.probs().await).await;
        let span = (self.high_entropy - self.low_entropy).max(1e-9);
        let t = ((entropy - self.low_entropy) / span).clamp(0.0, 1.0);
        let temp = self.low_temp + (self.high_temp - self.low_temp) * t;
        if temp <= 0.0 {
            ws.collapse_live_to_argmax().await;
        } else {
            ws.scale_processed_over_live(1.0 / temp as f32).await;
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { low_entropy: self.low_entropy, high_entropy: self.high_entropy, low_temp: self.low_temp, high_temp: self.high_temp }).into()
    }
}
// #endregion 🔖️Adaptive
