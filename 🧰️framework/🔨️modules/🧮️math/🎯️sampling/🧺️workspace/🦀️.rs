//! 🧺️ Token-sampling step contract: the reusable logits workspace, the per-step view and distribution, processor kinds, undo marks and processor stats.

use semio_framework_math_sampling_vocabulary::{Accum, SamplingError, SanitizePolicy, SequenceId, StepIndex, TokenBitset, TokenId, TokenTextAdapters, Vocabulary, sanitize_logits, softmax_live};
use semio_framework_math_sampling_config::{Candidate, ScheduleInput, TieBreak};

// #region 🔖️Workspace
/// 🧰️ Reinterprets a `&[u32]` as `&[TokenId]` without copying.
///
/// SAFETY: `TokenId` is `#[repr(transparent)]` over `u32` (see its definition in `🔖️Ids`), so the
/// two types share identical size, alignment, and bit-pattern validity — every `u32` is a valid
/// `TokenId`.
pub async fn cast_u32_slice_to_token_ids(ids: &[u32]) -> &[TokenId] {
    unsafe { core::slice::from_raw_parts(ids.as_ptr().cast::<TokenId>(), ids.len()) }
}

async fn argmax_token(logits: &[f32], tie_break: TieBreak) -> TokenId {
    let mut best_idx = 0u32;
    let mut best_val = f32::NEG_INFINITY;
    for (i, &v) in logits.iter().enumerate() {
        let better = match tie_break {
            TieBreak::HighestTokenId => v >= best_val,
            TieBreak::LowestTokenId | TieBreak::FirstSeen => v > best_val,
        };
        if better {
            best_val = v;
            best_idx = i as u32;
        }
    }
    TokenId::new(best_idx)
}

async fn argmax_index_in_slice(logits: &[f32], indices: &[u32]) -> u32 {
    let mut best = indices[0];
    let mut best_val = logits[best as usize];
    for &idx in &indices[1..] {
        let v = logits[idx as usize];
        if v > best_val {
            best_val = v;
            best = idx;
        }
    }
    best
}

/// 🧰️ Per-step scratch state for one sequence's logits: raw/processed vocab-sized arrays, the
/// hard-mask bitset, the shrinking `live` candidate-index list every warper operates on, and the
/// buffers the shared prob-sort (`sort_live_by_prob_desc`) reuses so steady-state stepping never
/// allocates once capacities have grown to the vocabulary size.
pub struct LogitsWorkspace {
    vocab_size: usize,
    accum: Accum,
    raw: Vec<f32>,
    processed: Vec<f32>,
    mask: TokenBitset,
    live: Vec<u32>,
    probs: Vec<f32>,
    sort_order: Vec<u32>,
    sorted_live_buf: Vec<u32>,
    sorted_probs_buf: Vec<f32>,
    saved_argmax: TokenId,
}

impl LogitsWorkspace {
    pub async fn new(vocab_size: usize) -> Self {
        Self {
            vocab_size,
            accum: Accum::default(),
            raw: vec![0.0; vocab_size],
            processed: vec![0.0; vocab_size],
            mask: TokenBitset::new_full(vocab_size).await,
            live: (0..vocab_size as u32).collect(),
            probs: Vec::with_capacity(vocab_size),
            sort_order: Vec::with_capacity(vocab_size),
            sorted_live_buf: Vec::with_capacity(vocab_size),
            sorted_probs_buf: Vec::with_capacity(vocab_size),
            saved_argmax: TokenId::new(0),
        }
    }

    /// 🧰️ Grows every buffer to `vocab_size` if it is larger than the workspace's current
    /// capacity; a no-operation (never shrinks) otherwise — the basis of pool reuse across batch slots.
    pub async fn ensure_capacity(&mut self, vocab_size: usize) {
        if vocab_size <= self.vocab_size {
            return;
        }
        self.vocab_size = vocab_size;
        self.raw.resize(vocab_size, 0.0);
        self.processed.resize(vocab_size, 0.0);
        self.mask = TokenBitset::new_full(vocab_size).await;
    }

    pub async fn set_accum(&mut self, accum: Accum) {
        self.accum = accum;
    }

    pub async fn vocab_size(&self) -> usize {
        self.vocab_size
    }

    pub async fn live(&self) -> &[u32] {
        &self.live
    }

    pub async fn set_live(&mut self, live: Vec<u32>) {
        self.live = live;
    }

    pub async fn processed(&self) -> &[f32] {
        &self.processed
    }

    pub async fn raw(&self) -> &[f32] {
        &self.raw
    }

    pub async fn probs(&self) -> &[f32] {
        &self.probs
    }

    pub async fn mask_mut(&mut self) -> &mut TokenBitset {
        &mut self.mask
    }

    pub async fn mask(&self) -> &TokenBitset {
        &self.mask
    }

    pub async fn saved_argmax(&self) -> TokenId {
        self.saved_argmax
    }

    /// 🧰️ Resets the workspace for a fresh step: copies `raw_logits` into `raw`, sanitizes it in
    /// place per `policy`, mirrors the sanitized values into `processed`, fills the mask, resets
    /// `live` to every vocabulary index, and records the (post-sanitize) argmax — the anchor the
    /// fallback ladder uses once every other option is exhausted. Returns the count of altered
    /// (NaN/Inf-resolved) entries.
    pub async fn reset_for_step(&mut self, raw_logits: &[f32], policy: SanitizePolicy) -> Result<usize, SamplingError> {
        debug_assert_eq!(raw_logits.len(), self.vocab_size);
        self.raw.copy_from_slice(raw_logits);
        let altered = sanitize_logits(&mut self.raw, policy).await?;
        self.processed.copy_from_slice(&self.raw);
        self.mask.fill().await;
        self.live.clear();
        self.live.extend(0..self.vocab_size as u32);
        self.saved_argmax = argmax_token(&self.raw, TieBreak::LowestTokenId).await;
        self.probs.clear();
        Ok(altered)
    }

    /// 🧰️ Removes every `live` entry whose mask bit is unset — call once after all hard-mask
    /// processors have run, before any soft-penalty or truncation processor.
    pub async fn sync_live_with_mask(&mut self) {
        let mut kept = Vec::with_capacity(self.live.len());
        for &idx in &self.live {
            if self.mask.get(TokenId::new(idx)).await {
                kept.push(idx);
            }
        }
        self.live = kept;
    }

    pub async fn scale_processed_over_live(&mut self, factor: f32) {
        for &idx in &self.live {
            self.processed[idx as usize] *= factor;
        }
    }

    pub async fn add_bias_over_live(&mut self, mut bias: impl FnMut(TokenId) -> f32) {
        for &idx in &self.live {
            self.processed[idx as usize] += bias(TokenId::new(idx));
        }
    }

    /// 🧰️ Replaces each live token's processed logit with `f(token, current_logit)` — the general
    /// form multiplicative penalties (sign-aware repetition penalty, per-class factors) need,
    /// since those aren't expressible as a pure additive bias.
    pub async fn transform_processed_over_live(&mut self, mut f: impl FnMut(TokenId, f32) -> f32) {
        for &idx in &self.live {
            let token = TokenId::new(idx);
            let current = self.processed[idx as usize];
            self.processed[idx as usize] = f(token, current);
        }
    }

    /// 🧰️ Adds `delta` to one token's processed logit directly by index, regardless of current
    /// `live` membership (harmless if the token is already masked out — later phases only ever
    /// read `processed` through `live`).
    pub async fn bias_processed(&mut self, token: TokenId, delta: f32) {
        if let Some(v) = self.processed.get_mut(token.get() as usize) {
            *v += delta;
        }
    }

    /// 🧰️ Collapses `live` to a single entry: the current argmax of `processed` restricted to
    /// `live` — how `Temperature`/`DynamicTemperature` implement "temperature 0 == greedy".
    pub async fn collapse_live_to_argmax(&mut self) {
        if self.live.is_empty() {
            return;
        }
        let best = argmax_index_in_slice(&self.processed, &self.live).await;
        self.live.clear();
        self.live.push(best);
    }

    /// 🧰️ Softmax over the *current* `live` order (does not sort); used by processors that need
    /// this step's entropy without disturbing candidate order (e.g. [`DynamicTemperature`]).
    pub async fn softmax_over_live(&mut self) -> f64 {
        let n = self.live.len();
        self.probs.resize(n, 0.0);
        softmax_live(&self.processed, &self.live, &mut self.probs, self.accum).await
    }

    /// 🧰️ Softmax over `live`, then reorders `live`/`probs` in lockstep by probability descending
    /// (ties ascending token id) — the shared, allocation-amortized sort every truncation warper
    /// (and the final distribution build) starts from.
    pub async fn sort_live_by_prob_desc(&mut self) {
        self.softmax_over_live().await;
        let n = self.live.len();
        let Self { live, probs, sort_order, sorted_live_buf, sorted_probs_buf, .. } = self;
        if sort_order.len() < n {
            sort_order.resize(n, 0);
        }
        for (i, slot) in sort_order[..n].iter_mut().enumerate() {
            *slot = i as u32;
        }
        sort_order[..n].sort_unstable_by(|&a, &b| probs[b as usize].partial_cmp(&probs[a as usize]).unwrap_or(core::cmp::Ordering::Equal).then_with(|| live[a as usize].cmp(&live[b as usize])));
        if sorted_live_buf.len() < n {
            sorted_live_buf.resize(n, 0);
        }
        if sorted_probs_buf.len() < n {
            sorted_probs_buf.resize(n, 0.0);
        }
        for (slot, &idx) in sort_order[..n].iter().enumerate() {
            sorted_live_buf[slot] = live[idx as usize];
            sorted_probs_buf[slot] = probs[idx as usize];
        }
        core::mem::swap(live, sorted_live_buf);
        core::mem::swap(probs, sorted_probs_buf);
        live.truncate(n);
        probs.truncate(n);
    }

    /// 🧰️ Keeps the top `keep.max(min_keep)` entries of an already-[`LogitsWorkspace::sort_live_by_prob_desc`]-sorted
    /// `live`/`probs` pair — every truncation warper's shared "never drop below `min_keep`" guarantee.
    pub async fn truncate_live_to(&mut self, keep: usize, min_keep: usize) {
        let keep = keep.max(min_keep.min(self.live.len())).min(self.live.len());
        self.live.truncate(keep);
        self.probs.truncate(keep);
    }
}

/// 🧰️ Output of one [`TokenSampler::sample`] call; cleared (not deallocated) between steps.
#[derive(Clone, Debug, Default)]
pub struct SelectionBuffer {
    pub chosen: Vec<Candidate>,
}

impl SelectionBuffer {
    pub async fn clear(&mut self) {
        self.chosen.clear();
    }
}

/// 🧰️ Reusable pool of [`LogitsWorkspace`]s for batch/continuous-batching use, so per-slot state
/// survives across steps instead of being reallocated.
#[derive(Default)]
pub struct WorkspacePool {
    slots: Vec<LogitsWorkspace>,
}

impl WorkspacePool {
    pub async fn new() -> Self {
        Self::default()
    }

    pub async fn acquire(&mut self, vocab_size: usize) -> LogitsWorkspace {
        match self.slots.pop() {
            Some(mut workspace) => {
                workspace.ensure_capacity(vocab_size).await;
                workspace
            }
            None => LogitsWorkspace::new(vocab_size).await,
        }
    }

    pub async fn release(&mut self, workspace: LogitsWorkspace) {
        self.slots.push(workspace);
    }
}
// #endregion 🔖️Workspace

/// 🔌️ Read-only view of one sequence at one step, borrowed for the duration of a pipeline phase.
pub struct StepView<'a> {
    pub sequence: SequenceId,
    pub step: StepIndex,
    pub prompt: &'a [TokenId],
    pub generated: &'a [TokenId],
    pub vocab: &'a Vocabulary,
    pub adapter: Option<&'a TokenTextAdapters>,
    pub last_entropy: Option<f64>,
}

pub async fn schedule_input(view: &StepView<'_>) -> ScheduleInput {
    ScheduleInput { step: view.step, generated_len: view.generated.len(), last_entropy: view.last_entropy }
}

/// 🔌️ The renormalized live distribution handed to samplers: every slice shares indexing, sorted
/// by probability descending with ties broken by ascending token id.
pub struct Distribution<'a> {
    pub tokens: &'a [TokenId],
    pub probs: &'a [f32],
    pub logprobs: &'a [f32],
    pub cdf: &'a [f64],
    pub entropy: f64,
}

/// 🔌️ Which pipeline phase a [`LogitsProcessor`] belongs to; the (forthcoming, wave-4) engine
/// dispatches hard masks before soft penalties before truncation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProcessorKind {
    HardMask,
    SoftPenalty,
    Truncation,
}

/// 🔌️ Opaque undo-log position returned by `LogitsProcessor::save`/`Constraint::save`/`StopCondition::save`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct StateMark(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ConstraintMark(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct StopMark(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ProcessorStats {
    pub masked: u32,
    pub timing_ns: u64,
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
