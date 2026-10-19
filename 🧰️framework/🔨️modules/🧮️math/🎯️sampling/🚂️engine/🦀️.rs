//! 🚂️ Token-sampling engine: sequence state with checkpoints, observers and the stateless and stateful sampling step.

#![allow(async_fn_in_trait)]

use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
use semio_framework_math_sampling_vocabulary::{DistributionHealth, FallbackAction, FinishReason, RandomSource, RandomSources, RngSnapshot, SamplingError, SequenceId, StepIndex, StreamKey, StreamPurpose, TokenId, TokenTextAdapters, Vocabulary, cumulative_from_probs, entropy_nats, resolve_fallback};
use semio_framework_math_sampling_config::{SamplingConfig, SamplingResult, StepDiagnostics};
use semio_framework_math_sampling_workspace::{ConstraintMark, Distribution, LogitsWorkspace, ProcessorKind, ProcessorStats, SelectionBuffer, StateMark, StepView, StopMark, cast_u32_slice_to_token_ids};
use semio_framework_math_sampling_processors::{ForcedTokens, LogitsProcessor, LogitsProcessors, MaxLengthForceEos, MinLengthEosSuppression, TokenSampler, TokenSamplers, build_processor, build_sampler};
use semio_framework_math_sampling_constraints::{Constraint, Constraints, StopCondition, StopConditions, StopPoll, TextStopCondition, TokenStopCondition, build_constraint};

/// 🔭️ Hook points into the engine's per-step lifecycle; every method is a no-operation default so
/// observers only implement what they need.
#[dyn_enum]
pub trait SamplingObserver {
    async fn on_step_start(&mut self, _sequence: SequenceId, _step: StepIndex) {}
    async fn on_processor(&mut self, _sequence: SequenceId, _name: &'static str, _stats: &ProcessorStats) {}
    async fn on_fallback(&mut self, _sequence: SequenceId, _error: &SamplingError, _action: FallbackAction) {}
    async fn on_token(&mut self, _sequence: SequenceId, _result: &SamplingResult) {}
    async fn on_finish(&mut self, _sequence: SequenceId, _reason: FinishReason) {}
}

// #region 🔖️SequenceState
/// 🧬️ A restorable point-in-time snapshot of every stateful component's undo-log position, plus
/// the observable state (generated length, cumulative log-probability) they correspond to.
#[derive(Clone, Debug)]
pub struct SequenceCheckpoint {
    pub generated_len: usize,
    pub cumulative_logprob: f64,
    pub processor_marks: Vec<StateMark>,
    pub constraint_marks: Vec<ConstraintMark>,
    pub stop_marks: Vec<StopMark>,
    pub rng_snapshot: RngSnapshot,
}

/// 🧬️ Everything mutable about one in-flight sequence: history, the pipeline's per-sequence
/// component instances (owned here, not shared), and a checkpoint stack enabling `rollback`.
pub struct SequenceState {
    id: SequenceId,
    prompt: Vec<TokenId>,
    generated: Vec<TokenId>,
    cumulative_logprob: f64,
    processors: Vec<LogitsProcessors>,
    sampler: TokenSamplers,
    constraints: Vec<Constraints>,
    stops: Vec<StopConditions>,
    rng: RandomSources,
    finish: Option<FinishReason>,
    config_fingerprint: u64,
    checkpoints: Vec<SequenceCheckpoint>,
}

impl SequenceState {
    /// 🧬️ Builds every per-sequence processor/constraint/sampler/stop instance from `config`.
    pub async fn new(id: SequenceId, prompt: Vec<TokenId>, config: &SamplingConfig, rng: RandomSources) -> Result<Self, SamplingError> {
        let mut processors: Vec<LogitsProcessors> = vec![(MinLengthEosSuppression { min_tokens: config.min_tokens }).into(), (MaxLengthForceEos { max_tokens: config.max_tokens }).into()];
        if config.forced.bos.is_some() || !config.forced.prefix.is_empty() || !config.forced.at_position.is_empty() {
            processors.push((ForcedTokens { spec: config.forced.clone() }).into());
        }
        for spec in &config.processors {
            processors.push(build_processor(spec).await?);
        }
        let sampler = build_sampler(&config.method).await;
        let mut constraints: Vec<Constraints> = Vec::new();
        for spec in &config.constraints {
            constraints.push(build_constraint(spec, &config.limits).await?);
        }
        let mut stops: Vec<StopConditions> = Vec::new();
        if !config.stops.tokens.is_empty() {
            stops.push((TokenStopCondition { tokens: config.stops.tokens.clone() }).into());
        }
        if !config.stops.sequences.is_empty() {
            stops.push((TextStopCondition::new(&config.stops.sequences, config.stops.mode).await).into());
        }
        Ok(Self { id, prompt, generated: Vec::new(), cumulative_logprob: 0.0, processors, sampler, constraints, stops, rng, finish: None, config_fingerprint: config.fingerprint().await, checkpoints: Vec::new() })
    }

    pub async fn id(&self) -> SequenceId {
        self.id
    }

    pub async fn prompt(&self) -> &[TokenId] {
        &self.prompt
    }

    pub async fn generated(&self) -> &[TokenId] {
        &self.generated
    }

    /// 🎲️ Splits an independent sub-stream off this sequence's random source without advancing it.
    pub async fn split_rng(&self, key: StreamKey) -> RandomSources {
        self.rng.split(key).await
    }

    pub async fn cumulative_logprob(&self) -> f64 {
        self.cumulative_logprob
    }

    pub async fn finish(&self) -> Option<FinishReason> {
        self.finish
    }

    pub async fn is_finished(&self) -> bool {
        self.finish.is_some()
    }

    pub async fn config_fingerprint(&self) -> u64 {
        self.config_fingerprint
    }

    async fn reset_state_only(&mut self) {
        self.generated.clear();
        self.cumulative_logprob = 0.0;
        self.finish = None;
        for p in self.processors.iter_mut() {
            p.reset().await;
        }
        for c in self.constraints.iter_mut() {
            c.reset().await;
        }
        for s in self.stops.iter_mut() {
            s.reset().await;
        }
    }

    pub async fn reset(&mut self) {
        self.reset_state_only().await;
        self.checkpoints.clear();
    }

    /// 🧬️ Captures the current position of every component's undo log.
    pub async fn checkpoint(&mut self) -> SequenceCheckpoint {
        let mut processor_marks = Vec::with_capacity(self.processors.len());
        for p in self.processors.iter_mut() {
            processor_marks.push(p.save().await);
        }
        let mut constraint_marks = Vec::with_capacity(self.constraints.len());
        for c in self.constraints.iter_mut() {
            constraint_marks.push(c.save().await);
        }
        let mut stop_marks = Vec::with_capacity(self.stops.len());
        for s in self.stops.iter_mut() {
            stop_marks.push(s.save().await);
        }
        SequenceCheckpoint { generated_len: self.generated.len(), cumulative_logprob: self.cumulative_logprob, processor_marks, constraint_marks, stop_marks, rng_snapshot: self.rng.snapshot().await }
    }

    /// 🧬️ Restores every component to a previously captured [`SequenceCheckpoint`].
    pub async fn restore(&mut self, checkpoint: &SequenceCheckpoint) {
        self.generated.truncate(checkpoint.generated_len);
        self.cumulative_logprob = checkpoint.cumulative_logprob;
        self.finish = None;
        for (p, mark) in self.processors.iter_mut().zip(checkpoint.processor_marks.iter()) {
            p.rollback_to(*mark).await;
        }
        for (c, mark) in self.constraints.iter_mut().zip(checkpoint.constraint_marks.iter()) {
            c.rollback_to(*mark).await;
        }
        for (s, mark) in self.stops.iter_mut().zip(checkpoint.stop_marks.iter()) {
            s.rollback_to(*mark).await;
        }
        let _ = self.rng.restore(&checkpoint.rng_snapshot).await;
    }

    /// 🧬️ Removes the last `n` generated tokens (and their effects on every component) by
    /// discarding the `n` most recent checkpoints and restoring the one before them — or the
    /// pristine pre-generation state if fewer than `n` checkpoints exist.
    pub async fn rollback(&mut self, n: usize) {
        let keep = self.checkpoints.len().saturating_sub(n);
        self.checkpoints.truncate(keep);
        match self.checkpoints.last().cloned() {
            Some(checkpoint) => self.restore(&checkpoint).await,
            None => self.reset_state_only().await,
        }
    }

    /// 🧬️ Independent copy sharing no mutable state, with its own RNG stream split from `self`'s
    /// via `rng_split_key` — the basis for beam search and speculative decoding forks.
    pub async fn fork(&self, new_id: SequenceId, rng_split_key: StreamKey) -> Self {
        let mut processors = Vec::with_capacity(self.processors.len());
        for p in &self.processors {
            processors.push(p.fork().await);
        }
        let mut constraints = Vec::with_capacity(self.constraints.len());
        for c in &self.constraints {
            constraints.push(c.fork().await);
        }
        let mut stops = Vec::with_capacity(self.stops.len());
        for s in &self.stops {
            stops.push(s.fork().await);
        }
        Self {
            id: new_id,
            prompt: self.prompt.clone(),
            generated: self.generated.clone(),
            cumulative_logprob: self.cumulative_logprob,
            processors,
            sampler: self.sampler.fork().await,
            constraints,
            stops,
            rng: self.rng.split(rng_split_key).await,
            finish: self.finish,
            config_fingerprint: self.config_fingerprint,
            checkpoints: self.checkpoints.clone(),
        }
    }

    /// 🧬️ Versioned, config-fingerprinted text form: `v1|fingerprint_hex|token,token,...`.
    pub async fn to_text(&self) -> String {
        let tokens: Vec<String> = self.generated.iter().map(|t| t.get().to_string()).collect();
        format!("v1|{:016x}|{}", self.config_fingerprint, tokens.join(","))
    }

    /// 🧬️ Decodes a [`SequenceState::to_text`] string, validating its fingerprint against `self`'s
    /// config. Returns the generated token list without mutating `self`; the caller re-drives a
    /// fresh [`SequenceState`] through those tokens (via the normal stateful step path) to rebuild
    /// processor/constraint/stop state exactly rather than trying to deserialize it directly.
    pub async fn decode_text(&self, text: &str) -> Result<Vec<TokenId>, SamplingError> {
        let mut parts = text.splitn(3, '|');
        let version = parts.next().ok_or(SamplingError::Corrupted { reason: "missing state version" })?;
        if version != "v1" {
            return Err(SamplingError::SerializationVersion { expected: 1, actual: version.trim_start_matches('v').parse().unwrap_or(0) });
        }
        let fingerprint_hex = parts.next().ok_or(SamplingError::Corrupted { reason: "missing state fingerprint" })?;
        let fingerprint = u64::from_str_radix(fingerprint_hex, 16).map_err(|_| SamplingError::Corrupted { reason: "invalid state fingerprint" })?;
        if fingerprint != self.config_fingerprint {
            return Err(SamplingError::FingerprintMismatch);
        }
        let tokens_part = parts.next().unwrap_or("");
        if tokens_part.is_empty() {
            return Ok(Vec::new());
        }
        tokens_part.split(',').map(|s| s.parse::<u32>().map(TokenId::new).map_err(|_| SamplingError::Corrupted { reason: "invalid token in serialized state" })).collect()
    }
}
// #endregion 🔖️SequenceState

// #region 🔖️Observability
/// 🔭️ An observer that discards every event — the default when nobody wants diagnostics.
#[derive(Default)]
pub struct NullObserver;

impl SamplingObserver for NullObserver {}

/// 🔭️ An observer that records a bounded number of human-readable event lines. Never logs token
/// text, only ids and reasons, matching the "no token text by default" security default.
#[derive(Default)]
pub struct CollectingObserver {
    pub events: Vec<String>,
    max_events: usize,
}

impl CollectingObserver {
    pub async fn new(max_events: usize) -> Self {
        Self { events: Vec::new(), max_events }
    }

    async fn push(&mut self, event: String) {
        if self.events.len() < self.max_events {
            self.events.push(event);
        }
    }
}

impl SamplingObserver for CollectingObserver {
    async fn on_finish(&mut self, sequence: SequenceId, reason: FinishReason) {
        self.push(format!("finish seq={} reason={:?}", sequence.get(), reason)).await;
    }
    async fn on_fallback(&mut self, sequence: SequenceId, _error: &SamplingError, action: FallbackAction) {
        self.push(format!("fallback seq={} action={:?}", sequence.get(), action)).await;
    }
}

dyn_enum_close! {
    pub enum SamplingObservers: SamplingObserver {
        Null(NullObserver),
        Collecting(CollectingObserver),
    }
}
// #endregion 🔖️Observability

// #region 🔖️Engine

/// 🚂️ Everything [`sample_step_stateless`] needs about the sequence beyond the raw logits. Every
/// field is `Copy` (ids/indices) or a borrowed reference, so the whole struct is `Copy` and cheap
/// to pass by value.
#[derive(Clone, Copy)]
pub struct StatelessStepInput<'a> {
    pub sequence: SequenceId,
    pub step: StepIndex,
    pub prompt: &'a [TokenId],
    pub generated: &'a [TokenId],
    pub vocab: &'a Vocabulary,
    pub adapter: Option<&'a TokenTextAdapters>,
    pub last_entropy: Option<f64>,
}

/// 🚂️ Runs one sampling step with no persistent per-sequence state (the "Stateless one-step
/// sampling" operating mode from § 1): applies every configured warper in order, falls back
/// through [`resolve_fallback`] if truncation empties the live set, builds the final distribution,
/// then samples via `config.method`. Penalties, biases, constraints, and stop conditions are not
/// yet applied here — they require [`SequenceState`] (added by the stateful engine in a later
/// wave); see [`build_processor`] for which [`ProcessorSpec`] variants are wired up so far.
pub async fn sample_step_stateless(config: &SamplingConfig, ws: &mut LogitsWorkspace, rng: &mut RandomSources, raw_logits: &[f32], input: StatelessStepInput<'_>) -> Result<SamplingResult, SamplingError> {
    input.vocab.validate_logits_len(raw_logits.len()).await?;
    ws.set_accum(config.accum).await;
    ws.reset_for_step(raw_logits, config.sanitize).await?;

    let view = StepView { sequence: input.sequence, step: input.step, prompt: input.prompt, generated: input.generated, vocab: input.vocab, adapter: input.adapter, last_entropy: input.last_entropy };

    let mut processors = Vec::with_capacity(config.processors.len());
    for spec in &config.processors {
        processors.push(build_processor(spec).await?);
    }
    for processor in processors.iter_mut() {
        if ws.live().await.is_empty() {
            break;
        }
        processor.process(&view, ws).await?;
    }

    let fallback = if ws.live().await.is_empty() {
        let (action, token) = resolve_fallback(None, input.vocab.eos.first().copied(), Some(ws.saved_argmax().await)).await;
        let token = token.ok_or(SamplingError::EmptyDistribution)?;
        ws.set_live(vec![token.get()]).await;
        Some(action)
    } else {
        None
    };

    ws.sort_live_by_prob_desc().await;
    let cdf = cumulative_from_probs(ws.probs().await).await;
    let logprobs: Vec<f32> = ws.probs().await.iter().map(|&p| (p as f64).ln() as f32).collect();
    let entropy = entropy_nats(ws.probs().await).await;
    let tokens = cast_u32_slice_to_token_ids(ws.live().await).await;
    let dist = Distribution { tokens, probs: ws.probs().await, logprobs: &logprobs, cdf: &cdf, entropy };

    let mut sampler = build_sampler(&config.method).await;
    let mut selection = SelectionBuffer::default();
    sampler.sample(&view, &dist, rng, &mut selection).await?;
    let chosen = *selection.chosen.first().ok_or(SamplingError::EmptyDistribution)?;

    let next_len = input.generated.len() + 1;
    let finish = if next_len >= config.max_tokens {
        Some(FinishReason::MaxTokens)
    } else if input.vocab.is_eos(chosen.token).await {
        Some(FinishReason::EosToken)
    } else {
        None
    };

    let diagnostics = if config.diagnostics.enabled {
        let prob_sum: f64 = ws.probs().await.iter().map(|&p| p as f64).sum();
        Some(StepDiagnostics { entropy, effective_count: entropy.exp(), truncation_mass: 0.0, masked_by: Vec::new(), timings_ns: Vec::new(), fallback, health: Some(DistributionHealth::assess(ws.live().await.len(), prob_sum).await) })
    } else {
        None
    };

    Ok(SamplingResult {
        token: chosen.token,
        logprob: chosen.logprob,
        finish,
        alternatives: selection.chosen,
        top_logprobs: None,
        rng_stream: StreamKey { request: 0, sequence: input.sequence.get(), beam: 0, candidate: 0, purpose: StreamPurpose::Selection },
        diagnostics,
    })
}

/// 🚂️ Runs one sampling step against a stateful [`SequenceState`] — the "Stateful multi-step
/// generation" operating mode. Phase-separates processors by [`ProcessorKind`] (hard masks, then
/// soft penalties, then truncation), applies the fallback ladder if truncation empties the live
/// set, samples, then commits every component's per-sequence effects and pushes a checkpoint —
/// but only after a token has been definitively selected (the "state update only after successful
/// token selection" pipeline guarantee). `view` is scoped to short-lived blocks throughout so its
/// borrow of `state.prompt`/`state.generated` never overlaps a later `&mut state` access.
pub async fn sample_step(
    config: &SamplingConfig,
    state: &mut SequenceState,
    ws: &mut LogitsWorkspace,
    vocab: &Vocabulary,
    adapter: Option<&TokenTextAdapters>,
    raw_logits: &[f32],
    observer: &mut SamplingObservers,
) -> Result<SamplingResult, SamplingError> {
    if state.finish.is_some() {
        return Err(SamplingError::InvalidConfig { field: "state", reason: "sequence already finished" });
    }
    vocab.validate_logits_len(raw_logits.len()).await?;
    ws.set_accum(config.accum).await;
    ws.reset_for_step(raw_logits, config.sanitize).await?;

    let step = StepIndex::new(state.generated.len() as u32);
    let sequence_id = state.id;
    observer.on_step_start(sequence_id, step).await;

    {
        let view = StepView { sequence: sequence_id, step, prompt: &state.prompt, generated: &state.generated, vocab, adapter, last_entropy: None };
        for constraint in state.constraints.iter_mut() {
            constraint.fill_mask(&view, ws.mask_mut().await).await?;
        }
        for processor in state.processors.iter_mut() {
            if processor.kind().await == ProcessorKind::HardMask {
                processor.process(&view, ws).await?;
            }
        }
        ws.sync_live_with_mask().await;
        for processor in state.processors.iter_mut() {
            if ws.live().await.is_empty() {
                break;
            }
            if processor.kind().await == ProcessorKind::SoftPenalty {
                processor.process(&view, ws).await?;
            }
        }
        for processor in state.processors.iter_mut() {
            if ws.live().await.is_empty() {
                break;
            }
            if processor.kind().await == ProcessorKind::Truncation {
                processor.process(&view, ws).await?;
            }
        }
    }

    let fallback = if ws.live().await.is_empty() {
        let (action, token) = resolve_fallback(None, vocab.eos.first().copied(), Some(ws.saved_argmax().await)).await;
        match token {
            Some(t) => {
                observer.on_fallback(sequence_id, &SamplingError::EmptyDistribution, action).await;
                ws.set_live(vec![t.get()]).await;
                Some(action)
            }
            None => {
                let err = SamplingError::EmptyDistribution;
                observer.on_fallback(sequence_id, &err, FallbackAction::Error).await;
                return Err(err);
            }
        }
    } else {
        None
    };

    ws.sort_live_by_prob_desc().await;
    let cdf = cumulative_from_probs(ws.probs().await).await;
    let logprobs: Vec<f32> = ws.probs().await.iter().map(|&p| (p as f64).ln() as f32).collect();
    let entropy = entropy_nats(ws.probs().await).await;
    let tokens = cast_u32_slice_to_token_ids(ws.live().await).await;
    let dist = Distribution { tokens, probs: ws.probs().await, logprobs: &logprobs, cdf: &cdf, entropy };

    let stream_key = StreamKey { request: 0, sequence: sequence_id.get(), beam: 0, candidate: 0, purpose: StreamPurpose::Selection };
    let mut selection = SelectionBuffer::default();
    {
        let view = StepView { sequence: sequence_id, step, prompt: &state.prompt, generated: &state.generated, vocab, adapter, last_entropy: Some(entropy) };
        state.sampler.sample(&view, &dist, &mut state.rng, &mut selection).await?;
    }
    let chosen = *selection.chosen.first().ok_or(SamplingError::EmptyDistribution)?;

    let mut stop_reason = None;
    {
        let view = StepView { sequence: sequence_id, step, prompt: &state.prompt, generated: &state.generated, vocab, adapter, last_entropy: Some(entropy) };
        for processor in state.processors.iter_mut() {
            processor.commit(&view, chosen.token).await;
        }
        for constraint in state.constraints.iter_mut() {
            constraint.accept(&view, chosen.token).await?;
        }
        for stop in state.stops.iter_mut() {
            if let StopPoll::Finished { reason, .. } = stop.on_token(&view, chosen.token).await {
                stop_reason.get_or_insert(reason);
            }
        }
    }
    state.generated.push(chosen.token);
    state.cumulative_logprob += chosen.logprob as f64;
    let checkpoint = state.checkpoint().await;
    state.checkpoints.push(checkpoint);

    let next_len = state.generated.len();
    let finish = match stop_reason {
        Some(reason) => Some(reason),
        None => {
            if next_len >= config.max_tokens {
                Some(FinishReason::MaxTokens)
            } else if vocab.is_eos(chosen.token).await {
                Some(FinishReason::EosToken)
            } else {
                None
            }
        }
    };
    state.finish = finish;
    if let Some(reason) = finish {
        observer.on_finish(sequence_id, reason).await;
    }

    let diagnostics = if config.diagnostics.enabled {
        let prob_sum: f64 = ws.probs().await.iter().map(|&p| p as f64).sum();
        Some(StepDiagnostics { entropy, effective_count: entropy.exp(), truncation_mass: 0.0, masked_by: Vec::new(), timings_ns: Vec::new(), fallback, health: Some(DistributionHealth::assess(ws.live().await.len(), prob_sum).await) })
    } else {
        None
    };

    let result = SamplingResult { token: chosen.token, logprob: chosen.logprob, finish, alternatives: selection.chosen, top_logprobs: None, rng_stream: stream_key, diagnostics };
    observer.on_token(sequence_id, &result).await;
    Ok(result)
}

/// 🚂️ Commits `token` onto `state` outside the normal `sample_step` selection path — shared by
/// [`beam_search`] and [`speculative_decode`], which each pick the next token through their own
/// mechanism (beam expansion, speculative accept/resample) rather than a [`TokenSampler`].
pub async fn commit_token_to_state(state: &mut SequenceState, vocab: &Vocabulary, adapter: Option<&TokenTextAdapters>, token: TokenId, logprob_nats: f64) -> Option<FinishReason> {
    let step = StepIndex::new(state.generated.len() as u32);
    let view = StepView { sequence: state.id, step, prompt: &state.prompt, generated: &state.generated, vocab, adapter, last_entropy: None };
    for processor in state.processors.iter_mut() {
        processor.commit(&view, token).await;
    }
    for constraint in state.constraints.iter_mut() {
        let _ = constraint.accept(&view, token).await;
    }
    let mut stop_reason = None;
    for stop in state.stops.iter_mut() {
        if let StopPoll::Finished { reason, .. } = stop.on_token(&view, token).await {
            stop_reason.get_or_insert(reason);
        }
    }
    state.generated.push(token);
    state.cumulative_logprob += logprob_nats;
    let checkpoint = state.checkpoint().await;
    state.checkpoints.push(checkpoint);
    let finish = match stop_reason {
        Some(reason) => Some(reason),
        None => vocab.is_eos(token).await.then_some(FinishReason::EosToken),
    };
    state.finish = finish;
    finish
}
// #endregion 🔖️Engine

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
