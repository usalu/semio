//! 🔍️ Token-sampling decoding strategies: continuous batching, beam search, best-of-n, speculative decoding and sharded-vocabulary sampling.

#![allow(async_fn_in_trait)]

use semio_framework_math_sampling_vocabulary::{RandomSource, RandomSources, SamplingError, SequenceId, StreamKey, StreamPurpose, TokenId, TokenTextAdapters, Vocabulary, cdf_binary_search, cumulative_from_probs};
use semio_framework_math_sampling_config::{SamplingConfig, SamplingResult};
use semio_framework_math_sampling_workspace::{LogitsWorkspace, WorkspacePool};
use semio_framework_math_sampling_engine::{SamplingObserver, SamplingObservers, SequenceState, StatelessStepInput, commit_token_to_state, sample_step, sample_step_stateless};

/// 🗂️ One rank's local top candidate, for sharded-vocabulary all-gather.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ShardCandidate {
    pub token: TokenId,
    pub logit: f32,
}

/// 🗂️ Deterministic collective-communication primitives sharded-vocabulary sampling needs.
///
/// 🔀️ Not `#[dyn_enum]`'d (O1 de-dyn, math-dedyn): [`LocalCollective`] is this crate's only
/// implementor, and every call site already used a short-lived `&mut dyn Collective` borrow, never
/// a stored trait object — the sanctioned single-impl answer is the concrete type directly, not a
/// one-variant enum (see `📓️terra-math-dedyn-report.md`).
pub trait Collective {
    async fn rank(&self) -> usize;
    async fn world_size(&self) -> usize;
    async fn all_reduce_max_f32(&mut self, values: &mut [f32]) -> Result<(), SamplingError>;
    async fn all_reduce_sum_f64(&mut self, values: &mut [f64]) -> Result<(), SamplingError>;
    /// 🗂️ Gathers every rank's local candidates into `out`, in rank order (deterministic).
    async fn all_gather_candidates(&mut self, local: &[ShardCandidate], out: &mut Vec<ShardCandidate>) -> Result<(), SamplingError>;
}

// #region 🔖️Batch
/// 📦️ One sequence's raw logits for a batch step.
pub struct BatchEntry<'a> {
    pub id: SequenceId,
    pub logits: &'a [f32],
}

/// 📦️ A batch of per-sequence logits to step together.
pub struct BatchSamplingRequest<'a> {
    pub entries: Vec<BatchEntry<'a>>,
}

/// 📦️ Per-sequence results from one [`ContinuousBatcher::step`] call, in the same order as the
/// request's entries.
pub struct BatchSamplingResult {
    pub results: Vec<(SequenceId, Result<SamplingResult, SamplingError>)>,
}

/// 📦️ Owns a dynamic set of [`SequenceState`]s addressed by [`SequenceId`] (never by slot index),
/// so sequences can be added/removed between steps and the batch's *processing* order can vary
/// freely — every sequence's own [`SequenceState::rng`] stream is keyed by its id, not by
/// position, so outputs never depend on batch order. Reuses [`LogitsWorkspace`]s via
/// [`WorkspacePool`] so steady-state stepping doesn't reallocate.
pub struct ContinuousBatcher {
    sequences: std::collections::HashMap<SequenceId, SequenceState>,
    pool: WorkspacePool,
    vocab_size: usize,
}

impl ContinuousBatcher {
    pub async fn new(vocab_size: usize) -> Self {
        Self { sequences: std::collections::HashMap::new(), pool: WorkspacePool::new().await, vocab_size }
    }

    pub async fn add_sequence(&mut self, state: SequenceState) {
        self.sequences.insert(state.id().await, state);
    }

    pub async fn remove_sequence(&mut self, id: SequenceId) -> Option<SequenceState> {
        self.sequences.remove(&id)
    }

    pub async fn contains(&self, id: SequenceId) -> bool {
        self.sequences.contains_key(&id)
    }

    pub async fn len(&self) -> usize {
        self.sequences.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.sequences.is_empty()
    }

    pub async fn get(&self, id: SequenceId) -> Option<&SequenceState> {
        self.sequences.get(&id)
    }

    /// 📦️ Steps every sequence named in `request.entries`, in that (arbitrary) order.
    pub async fn step(&mut self, config: &SamplingConfig, vocab: &Vocabulary, adapter: Option<&TokenTextAdapters>, request: &BatchSamplingRequest<'_>, observer: &mut SamplingObservers) -> BatchSamplingResult {
        let mut results = Vec::with_capacity(request.entries.len());
        for entry in &request.entries {
            let Some(state) = self.sequences.get_mut(&entry.id) else {
                results.push((entry.id, Err(SamplingError::InvalidConfig { field: "batch", reason: "unknown sequence id" })));
                continue;
            };
            let mut ws = self.pool.acquire(self.vocab_size).await;
            let result = sample_step(config, state, &mut ws, vocab, adapter, entry.logits, observer).await;
            self.pool.release(ws).await;
            results.push((entry.id, result));
        }
        BatchSamplingResult { results }
    }
}
// #endregion 🔖️Batch

// #region 🔖️Search
/// 🌳️ One beam search hypothesis: its own independent [`SequenceState`] plus cumulative
/// log-probability score (unnormalized — apply [`gnmt_length_penalty`] for ranking).
pub struct BeamHypothesis {
    pub state: SequenceState,
    pub score: f64,
}

/// 🌳️ GNMT-style length penalty: `((5 + len) / 6) ^ alpha`, dividing the raw score to avoid
/// favoring short hypotheses.
pub async fn gnmt_length_penalty(len: usize, alpha: f64) -> f64 {
    ((5.0 + len as f64) / 6.0).powf(alpha)
}

pub struct BeamSearchConfig {
    pub width: usize,
    pub length_penalty: f64,
    pub max_steps: usize,
}

/// 🌳️ Beam search: `next_logits` supplies raw model logits for a hypothesis's current state (the
/// caller owns model inference, per § 1's non-responsibilities). At each step, every active
/// hypothesis contributes its top-`width` next-token candidates (by probability); all
/// `hypotheses × candidates` are pooled, ranked by cumulative log-probability (ties by ascending
/// token id), and the top `width` become the next round's hypotheses — each an independent
/// [`SequenceState::fork`] so per-sequence state (penalties, constraints, RNG) never aliases
/// across beams. Finished hypotheses are set aside and never re-expanded. Returns every
/// hypothesis (finished or step-exhausted), sorted best-first by length-normalized score.
pub async fn beam_search(
    config: &SamplingConfig,
    beam_config: &BeamSearchConfig,
    vocab: &Vocabulary,
    adapter: Option<&TokenTextAdapters>,
    initial: SequenceState,
    mut next_logits: impl AsyncFnMut(&SequenceState) -> Vec<f32>,
) -> Result<Vec<BeamHypothesis>, SamplingError> {
    let mut ws = LogitsWorkspace::new(vocab.size).await;
    let mut beams = vec![BeamHypothesis { state: initial, score: 0.0 }];
    let mut finished: Vec<BeamHypothesis> = Vec::new();

    for _ in 0..beam_config.max_steps {
        if beams.is_empty() {
            break;
        }
        let mut candidates: Vec<(usize, TokenId, f64)> = Vec::new();
        for (bi, beam) in beams.iter().enumerate() {
            let raw_logits = next_logits(&beam.state).await;
            vocab.validate_logits_len(raw_logits.len()).await?;
            ws.set_accum(config.accum).await;
            ws.reset_for_step(&raw_logits, config.sanitize).await?;
            ws.sort_live_by_prob_desc().await;
            let k = beam_config.width.min(ws.live().await.len());
            for i in 0..k {
                candidates.push((bi, TokenId::new(ws.live().await[i]), (ws.probs().await[i] as f64).ln()));
            }
        }
        if candidates.is_empty() {
            break;
        }
        candidates.sort_by(|a, b| {
            let score_a = beams[a.0].score + a.2;
            let score_b = beams[b.0].score + b.2;
            score_b.partial_cmp(&score_a).unwrap_or(core::cmp::Ordering::Equal).then_with(|| a.1.cmp(&b.1))
        });

        let mut next_round = Vec::with_capacity(beam_config.width);
        for (rank, &(bi, token, logprob)) in candidates.iter().take(beam_config.width).enumerate() {
            let key = StreamKey { request: 0, sequence: beams[bi].state.id().await.get(), beam: rank as u32, candidate: 0, purpose: StreamPurpose::Beam };
            let mut child = beams[bi].state.fork(beams[bi].state.id().await, key).await;
            commit_token_to_state(&mut child, vocab, adapter, token, logprob).await;
            let hyp = BeamHypothesis { state: child, score: beams[bi].score + logprob };
            if hyp.state.is_finished().await {
                finished.push(hyp);
            } else {
                next_round.push(hyp);
            }
        }
        beams = next_round;
    }

    finished.extend(beams);
    // 🔀️ `Vec::sort_by`'s comparator is a sync `FnMut` — cannot `.await` inside it (E0728). The
    // normalized score is precomputed per hypothesis (decorate-sort-undecorate) instead (O1
    // de-dyn, math-dedyn).
    let mut scored: Vec<(f64, BeamHypothesis)> = Vec::with_capacity(finished.len());
    for hyp in finished {
        let normalized = hyp.score / gnmt_length_penalty(hyp.state.generated().await.len(), beam_config.length_penalty).await;
        scored.push((normalized, hyp));
    }
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(core::cmp::Ordering::Equal));
    Ok(scored.into_iter().map(|(_, hyp)| hyp).collect())
}

/// 🌳️ Generates `n` complete candidates by running ordinary [`sample_step`] to completion for
/// each, then ranks them by mean log-probability (best first). `make_initial` builds a fresh
/// [`SequenceState`] per candidate index (so each gets its own RNG stream, e.g. via a distinct
/// [`StreamKey::candidate`]).
pub struct BestOfN {
    pub n: usize,
}

impl BestOfN {
    #[allow(clippy::too_many_arguments)]
    pub async fn run(
        &self,
        config: &SamplingConfig,
        vocab: &Vocabulary,
        adapter: Option<&TokenTextAdapters>,
        max_steps: usize,
        observer: &mut SamplingObservers,
        // 🔀️ `impl Fn(usize) -> Result<SequenceState, SamplingError>` no longer type-checks now
        // that `SequenceState::new` is `async fn` — `AsyncFn` (stable since 1.85) is the direct
        // async-closure replacement, not a `Pin<Box<dyn Future>>` factory (R1 bans `dyn Future` in
        // this kind of position too; O1 de-dyn, math-dedyn).
        mut make_initial: impl AsyncFnMut(usize) -> Result<SequenceState, SamplingError>,
        mut next_logits: impl AsyncFnMut(&SequenceState) -> Vec<f32>,
    ) -> Result<Vec<(SequenceState, f64)>, SamplingError> {
        let mut candidates = Vec::with_capacity(self.n);
        for i in 0..self.n {
            let mut state = make_initial(i).await?;
            let mut ws = LogitsWorkspace::new(vocab.size).await;
            for _ in 0..max_steps {
                if state.is_finished().await {
                    break;
                }
                let raw_logits = next_logits(&state).await;
                sample_step(config, &mut state, &mut ws, vocab, adapter, &raw_logits, observer).await?;
            }
            let mean_logprob = if state.generated().await.is_empty() { 0.0 } else { state.cumulative_logprob().await / state.generated().await.len() as f64 };
            candidates.push((state, mean_logprob));
        }
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(core::cmp::Ordering::Equal));
        Ok(candidates)
    }
}

/// 🌳️ Retries [`sample_step_stateless`] up to `max_attempts` times, keeping the first result
/// `accept` approves.
pub struct RejectionSampler {
    pub max_attempts: usize,
}

impl RejectionSampler {
    pub async fn sample(&self, config: &SamplingConfig, ws: &mut LogitsWorkspace, rng: &mut RandomSources, raw_logits: &[f32], input: StatelessStepInput<'_>, mut accept: impl FnMut(&SamplingResult) -> bool) -> Result<SamplingResult, SamplingError> {
        let mut last_err = None;
        for _ in 0..self.max_attempts {
            match sample_step_stateless(config, ws, rng, raw_logits, input).await {
                Ok(result) if accept(&result) => return Ok(result),
                Ok(_) => continue,
                Err(e) => last_err = Some(e),
            }
        }
        Err(last_err.unwrap_or(SamplingError::LimitExceeded { limit: "rejection_sampler.max_attempts" }))
    }
}
// #endregion 🔖️Search

// #region 🔖️Speculative
/// ⚡️ Outcome counters for one [`speculative_decode`] call.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct SpecMetrics {
    pub proposed: usize,
    pub accepted: usize,
    pub bonus_taken: bool,
}

/// ⚡️ Exact speculative decoding: verifies `draft_tokens` against `target_logits` (the target
/// model's logits for the state as it would be after accepting the prefix so far), accepting
/// position `i` with probability `min(1, p_target(draft_i) / p_draft(draft_i))`. On the first
/// rejection, resamples from the clamped residual `max(0, p_target(t) - p_draft(t))` (renormalized
/// over *every* token `t`, not just the drafted one) and stops — this is what makes the scheme
/// *exact*: the resulting token distribution is identical to sampling from `target_logits`
/// directly at every position. `draft_distributions[i]` must therefore be the draft model's
/// **full** per-token probability vector for position `i` (same vocabulary shape as the target
/// logits) — a residual computed from only the drafted token's own probability (treating every
/// other token as if the draft model assigned it zero mass) is a different, *biased* algorithm,
/// not exact speculative decoding. If every draft is accepted, draws one bonus token from the
/// final target row. All verification draws come from a dedicated [`StreamPurpose::Speculative`]
/// sub-stream (split from `state.rng` once, up front), so speculation never perturbs the
/// sequence's ordinary selection stream. Accepted/resampled/bonus tokens are committed through
/// [`commit_token_to_state`] one at a time (not as a single all-or-nothing transaction), so
/// constraint/penalty/stop state stays exactly what non-speculative decoding would have produced
/// for the same accepted prefix.
#[allow(clippy::too_many_arguments)]
pub async fn speculative_decode(
    config: &SamplingConfig,
    state: &mut SequenceState,
    ws: &mut LogitsWorkspace,
    vocab: &Vocabulary,
    adapter: Option<&TokenTextAdapters>,
    draft_tokens: &[TokenId],
    draft_distributions: &[Vec<f32>],
    mut target_logits: impl AsyncFnMut(&SequenceState) -> Vec<f32>,
    observer: &mut SamplingObservers,
) -> Result<(Vec<SamplingResult>, SpecMetrics), SamplingError> {
    let mut results = Vec::new();
    let mut metrics = SpecMetrics { proposed: draft_tokens.len(), accepted: 0, bonus_taken: false };
    let stream_key = StreamKey { request: 0, sequence: state.id().await.get(), beam: 0, candidate: 0, purpose: StreamPurpose::Speculative };
    let mut spec_rng = state.split_rng(stream_key).await;

    for i in 0..draft_tokens.len() {
        if state.is_finished().await {
            break;
        }
        let raw_logits = target_logits(&*state).await;
        vocab.validate_logits_len(raw_logits.len()).await?;
        ws.set_accum(config.accum).await;
        ws.reset_for_step(&raw_logits, config.sanitize).await?;
        ws.sort_live_by_prob_desc().await;
        let live_idx = ws.live().await.iter().position(|&t| t == draft_tokens[i].get());
        let probs_now = ws.probs().await;
        let target_prob = live_idx.map_or(0.0, |idx| probs_now[idx]);
        let draft_prob = draft_distributions[i].get(draft_tokens[i].get() as usize).copied().unwrap_or(0.0);
        let accept_prob = (target_prob as f64 / (draft_prob as f64).max(1e-12)).min(1.0);

        if spec_rng.next_f64().await < accept_prob {
            let logprob = (target_prob.max(f32::MIN_POSITIVE) as f64).ln();
            let finish = commit_token_to_state(state, vocab, adapter, draft_tokens[i], logprob).await;
            metrics.accepted += 1;
            let result = SamplingResult { token: draft_tokens[i], logprob: logprob as f32, finish, alternatives: Vec::new(), top_logprobs: None, rng_stream: stream_key, diagnostics: None };
            observer.on_token(state.id().await, &result).await;
            results.push(result);
        } else {
            let residual: Vec<f32> = ws.live().await.iter().zip(ws.probs().await.iter()).map(|(&t, &p)| (p - draft_distributions[i].get(t as usize).copied().unwrap_or(0.0)).max(0.0)).collect();
            let sum: f32 = residual.iter().sum();
            let normalized: Vec<f32> = if sum > 0.0 { residual.iter().map(|&r| r / sum).collect() } else { ws.probs().await.to_vec() };
            let cdf = cumulative_from_probs(&normalized).await;
            let idx = cdf_binary_search(&cdf, spec_rng.next_f64().await).await;
            let token = TokenId::new(ws.live().await[idx]);
            let logprob = (normalized[idx].max(f32::MIN_POSITIVE) as f64).ln();
            let finish = commit_token_to_state(state, vocab, adapter, token, logprob).await;
            let result = SamplingResult { token, logprob: logprob as f32, finish, alternatives: Vec::new(), top_logprobs: None, rng_stream: stream_key, diagnostics: None };
            observer.on_token(state.id().await, &result).await;
            results.push(result);
            return Ok((results, metrics));
        }
    }

    if !state.is_finished().await {
        let raw_logits = target_logits(&*state).await;
        vocab.validate_logits_len(raw_logits.len()).await?;
        ws.set_accum(config.accum).await;
        ws.reset_for_step(&raw_logits, config.sanitize).await?;
        ws.sort_live_by_prob_desc().await;
        let cdf = cumulative_from_probs(ws.probs().await).await;
        let idx = cdf_binary_search(&cdf, spec_rng.next_f64().await).await;
        let token = TokenId::new(ws.live().await[idx]);
        let logprob = (ws.probs().await[idx].max(f32::MIN_POSITIVE) as f64).ln();
        let finish = commit_token_to_state(state, vocab, adapter, token, logprob).await;
        metrics.bonus_taken = true;
        let result = SamplingResult { token, logprob: logprob as f32, finish, alternatives: Vec::new(), top_logprobs: None, rng_stream: stream_key, diagnostics: None };
        observer.on_token(state.id().await, &result).await;
        results.push(result);
    }
    Ok((results, metrics))
}
// #endregion 🔖️Speculative

// #region 🔖️Sharded
struct LocalCollectiveMailbox {
    f32_slots: Vec<Vec<f32>>,
    f64_slots: Vec<Vec<f64>>,
    candidate_slots: Vec<Vec<ShardCandidate>>,
}

/// 🗂️ Same-process reference [`Collective`]: every rank's handle shares one mailbox. This is
/// **not** a real network protocol — each call stages this rank's contribution and returns the
/// reduction over whatever has been staged *so far*, so callers must call every rank once (any
/// order) per logical collective operation, then call once more (or read the last call's result) to see
/// every rank's contribution reflected. Good enough for testing the sharded sampling math against
/// its unsharded equivalent in a single process; not a substitute for a real collective library.
pub struct LocalCollective {
    rank: usize,
    world_size: usize,
    mailbox: std::rc::Rc<std::cell::RefCell<LocalCollectiveMailbox>>,
}

impl LocalCollective {
    /// 🗂️ Builds `world_size` linked rank handles sharing one mailbox.
    pub async fn new_group(world_size: usize) -> Vec<Self> {
        let mailbox = std::rc::Rc::new(std::cell::RefCell::new(LocalCollectiveMailbox { f32_slots: vec![Vec::new(); world_size], f64_slots: vec![Vec::new(); world_size], candidate_slots: vec![Vec::new(); world_size] }));
        (0..world_size).map(|rank| Self { rank, world_size, mailbox: mailbox.clone() }).collect()
    }
}

impl Collective for LocalCollective {
    async fn rank(&self) -> usize {
        self.rank
    }
    async fn world_size(&self) -> usize {
        self.world_size
    }
    async fn all_reduce_max_f32(&mut self, values: &mut [f32]) -> Result<(), SamplingError> {
        let mut mailbox = self.mailbox.borrow_mut();
        mailbox.f32_slots[self.rank] = values.to_vec();
        let len = values.len();
        for slot in values.iter_mut() {
            *slot = f32::NEG_INFINITY;
        }
        for staged in &mailbox.f32_slots {
            for (i, &v) in staged.iter().enumerate().take(len) {
                if v > values[i] {
                    values[i] = v;
                }
            }
        }
        Ok(())
    }
    async fn all_reduce_sum_f64(&mut self, values: &mut [f64]) -> Result<(), SamplingError> {
        let mut mailbox = self.mailbox.borrow_mut();
        mailbox.f64_slots[self.rank] = values.to_vec();
        let len = values.len();
        for slot in values.iter_mut() {
            *slot = 0.0;
        }
        for staged in &mailbox.f64_slots {
            for (i, &v) in staged.iter().enumerate().take(len) {
                values[i] += v;
            }
        }
        Ok(())
    }
    async fn all_gather_candidates(&mut self, local: &[ShardCandidate], out: &mut Vec<ShardCandidate>) -> Result<(), SamplingError> {
        let mut mailbox = self.mailbox.borrow_mut();
        mailbox.candidate_slots[self.rank] = local.to_vec();
        out.clear();
        for slot in &mailbox.candidate_slots {
            out.extend_from_slice(slot);
        }
        Ok(())
    }
}

/// 🗂️ Sharded softmax: each rank supplies its local shard's raw logits; returns this rank's
/// locally-normalized probabilities under the *global* normalization constant (global max via
/// `all_reduce_max_f32`, then global `sum(exp(l - max))` via `all_reduce_sum_f64`) — matching
/// softmax over the concatenation of every shard's logits.
pub async fn sharded_softmax(collective: &mut LocalCollective, local_logits: &[f32]) -> Vec<f32> {
    let local_max = local_logits.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let mut max_buf = [local_max];
    let _ = collective.all_reduce_max_f32(&mut max_buf).await;
    let global_max = max_buf[0];
    let local_sum: f64 = local_logits.iter().map(|&l| ((l - global_max) as f64).exp()).sum();
    let mut sum_buf = [local_sum];
    let _ = collective.all_reduce_sum_f64(&mut sum_buf).await;
    let global_sum = sum_buf[0];
    local_logits.iter().map(|&l| (((l - global_max) as f64).exp() / global_sum) as f32).collect()
}

/// 🗂️ Sharded top-k: each rank contributes its local top-`k` (by logit), merged via
/// `all_gather_candidates` and re-truncated to the global top-`k`.
pub async fn sharded_top_k(collective: &mut LocalCollective, local_logits: &[f32], local_token_offset: u32, k: usize) -> Vec<ShardCandidate> {
    let mut indexed: Vec<(u32, f32)> = local_logits.iter().enumerate().map(|(i, &l)| (local_token_offset + i as u32, l)).collect();
    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(core::cmp::Ordering::Equal));
    let local: Vec<ShardCandidate> = indexed.into_iter().take(k).map(|(t, l)| ShardCandidate { token: TokenId::new(t), logit: l }).collect();
    let mut gathered = Vec::new();
    let _ = collective.all_gather_candidates(&local, &mut gathered).await;
    gathered.sort_by(|a, b| b.logit.partial_cmp(&a.logit).unwrap_or(core::cmp::Ordering::Equal).then_with(|| a.token.cmp(&b.token)));
    gathered.truncate(k);
    gathered
}

/// 🗂️ Sharded categorical sample: gathers every token across all shards (via [`sharded_top_k`]
/// with `k = usize::MAX`), builds the global CDF, and draws one token — exactly matching a plain
/// (unsharded) multinomial draw over the concatenated vocabulary given the same `u`.
pub async fn sharded_sample(collective: &mut LocalCollective, local_logits: &[f32], local_token_offset: u32, rng: &mut RandomSources) -> Option<TokenId> {
    let candidates = sharded_top_k(collective, local_logits, local_token_offset, usize::MAX).await;
    if candidates.is_empty() {
        return None;
    }
    let max_logit = candidates.iter().map(|c| c.logit).fold(f32::NEG_INFINITY, f32::max);
    let weights: Vec<f64> = candidates.iter().map(|c| ((c.logit - max_logit) as f64).exp()).collect();
    let total: f64 = weights.iter().sum();
    let probs: Vec<f32> = weights.iter().map(|&w| (w / total) as f32).collect();
    let cdf = cumulative_from_probs(&probs).await;
    let idx = cdf_binary_search(&cdf, rng.next_f64().await).await;
    Some(candidates[idx].token)
}
// #endregion 🔖️Sharded

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
