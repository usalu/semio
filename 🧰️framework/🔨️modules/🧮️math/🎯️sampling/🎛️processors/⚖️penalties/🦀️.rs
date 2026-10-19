//! ⚖️ Repetition, presence, frequency, decaying, class, n-gram and phrase penalties.

use semio_framework_math_sampling_vocabulary::{SamplingError, TokenId};
use semio_framework_math_sampling_config::PenaltyScope;
use semio_framework_math_sampling_workspace::{LogitsWorkspace, ProcessorKind, StateMark, StepView};
use super::{LogitsProcessor, LogitsProcessors};

// #region 🔖️Penalties
/// ⚖️ Dense-enough token→count map for repetition-style penalties. Backed by a `HashMap` (not a
/// vocab-sized `Vec`) so processor construction never needs to know the vocabulary size.
#[derive(Clone, Default)]
pub struct FreqTable {
    counts: std::collections::HashMap<TokenId, u32>,
}

impl FreqTable {
    pub async fn new() -> Self {
        Self::default()
    }

    pub async fn count(&self, token: TokenId) -> u32 {
        self.counts.get(&token).copied().unwrap_or(0)
    }

    pub async fn increment(&mut self, token: TokenId) {
        *self.counts.entry(token).or_insert(0) += 1;
    }

    /// ⚖️ Exact inverse of [`FreqTable::increment`] — removes the map entry entirely once its
    /// count reaches zero, so `count()` and `HashMap` iteration agree on "seen at all".
    pub async fn decrement(&mut self, token: TokenId) {
        if let Some(c) = self.counts.get_mut(&token) {
            if *c <= 1 {
                self.counts.remove(&token);
            } else {
                *c -= 1;
            }
        }
    }

    pub async fn reset(&mut self) {
        self.counts.clear();
    }
}

/// ⚖️ Open-addressed (via `HashMap`) rolling-context index: maps a hash of the last `order - 1`
/// tokens to every token observed to follow that context, for [`NoRepeatNgram`]. `undo` records
/// exactly the key touched by each [`NgramIndex::record`] call (or `None` for a no-operation call), so
/// [`NgramIndex::rollback_last_n`] can undo precisely `n` prior commits.
#[derive(Clone, Default)]
pub struct NgramIndex {
    order: usize,
    table: std::collections::HashMap<u64, Vec<TokenId>>,
    undo: Vec<Option<u64>>,
}

impl NgramIndex {
    pub async fn new(order: usize) -> Self {
        Self { order, table: std::collections::HashMap::new(), undo: Vec::new() }
    }

    async fn context_hash(context: &[TokenId]) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for token in context {
            hash = (hash ^ token.get() as u64).wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    /// ⚖️ Records that `next` followed the last `order - 1` tokens of `history` (the history as it
    /// existed *before* `next`).
    pub async fn record(&mut self, history: &[TokenId], next: TokenId) {
        if self.order < 2 || history.len() + 1 < self.order {
            self.undo.push(None);
            return;
        }
        let ctx_len = self.order - 1;
        let context = &history[history.len() - ctx_len..];
        let key = Self::context_hash(context).await;
        self.table.entry(key).or_default().push(next);
        self.undo.push(Some(key));
    }

    /// ⚖️ Tokens that would recreate an already-seen `order`-gram if selected next.
    pub async fn forbidden_next(&self, history: &[TokenId]) -> &[TokenId] {
        if self.order < 2 || history.len() + 1 < self.order {
            return &[];
        }
        let ctx_len = self.order - 1;
        let context = &history[history.len() - ctx_len..];
        let key = Self::context_hash(context).await;
        self.table.get(&key).map_or(&[][..], Vec::as_slice)
    }

    pub async fn commit_count(&self) -> u64 {
        self.undo.len() as u64
    }

    pub async fn rollback_last_n(&mut self, n: usize) {
        for _ in 0..n {
            let Some(entry) = self.undo.pop() else { break };
            if let Some(key) = entry {
                if let Some(list) = self.table.get_mut(&key) {
                    list.pop();
                    if list.is_empty() {
                        self.table.remove(&key);
                    }
                }
            }
        }
    }

    pub async fn reset(&mut self) {
        self.table.clear();
        self.undo.clear();
    }
}

/// ⚖️ Sign-aware multiplicative repetition penalty: positive logits are divided by `penalty`,
/// negative logits multiplied by it — so `penalty > 1` always pushes a seen token's logit down
/// regardless of its sign.
pub struct RepetitionPenalty {
    pub penalty: f32,
    pub scope: PenaltyScope,
    pub(super) counts: FreqTable,
    prompt_included: bool,
    commit_log: Vec<TokenId>,
}

impl RepetitionPenalty {
    pub async fn new(penalty: f32, scope: PenaltyScope) -> Self {
        Self { penalty, scope, counts: FreqTable::new().await, prompt_included: false, commit_log: Vec::new() }
    }

    async fn include_prompt_if_needed(&mut self, prompt: &[TokenId]) {
        if !self.prompt_included && matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::PromptOnly) {
            for &t in prompt {
                self.counts.increment(t).await;
            }
            self.prompt_included = true;
        }
    }
}

impl LogitsProcessor for RepetitionPenalty {
    async fn name(&self) -> &'static str {
        "repetition_penalty"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        self.include_prompt_if_needed(view.prompt).await;
        let penalty = self.penalty;
        let mut seen: std::collections::HashSet<TokenId> = std::collections::HashSet::new();
        for &idx in ws.live().await {
            let token = TokenId::new(idx);
            if self.counts.count(token).await > 0 {
                seen.insert(token);
            }
        }
        ws.transform_processed_over_live(|token, logit| {
            if seen.contains(&token) {
                if logit > 0.0 {
                    logit / penalty
                } else {
                    logit * penalty
                }
            } else {
                logit
            }
        })
        .await;
        Ok(())
    }
    async fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
            self.counts.increment(token).await;
        }
        self.commit_log.push(token);
    }
    async fn save(&mut self) -> StateMark {
        StateMark(self.commit_log.len() as u64)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        while self.commit_log.len() as u64 > mark.0 {
            if let Some(token) = self.commit_log.pop() {
                if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
                    self.counts.decrement(token).await;
                }
            }
        }
    }
    async fn reset(&mut self) {
        self.counts.reset().await;
        self.prompt_included = false;
        self.commit_log.clear();
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { penalty: self.penalty, scope: self.scope, counts: self.counts.clone(), prompt_included: self.prompt_included, commit_log: self.commit_log.clone() }).into()
    }
}

/// ⚖️ Fixed additive penalty applied once per distinct token that has appeared.
pub struct PresencePenalty {
    pub penalty: f32,
    pub scope: PenaltyScope,
    pub(super) counts: FreqTable,
    prompt_included: bool,
    commit_log: Vec<TokenId>,
}

impl PresencePenalty {
    pub async fn new(penalty: f32, scope: PenaltyScope) -> Self {
        Self { penalty, scope, counts: FreqTable::new().await, prompt_included: false, commit_log: Vec::new() }
    }

    async fn include_prompt_if_needed(&mut self, prompt: &[TokenId]) {
        if !self.prompt_included && matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::PromptOnly) {
            for &t in prompt {
                self.counts.increment(t).await;
            }
            self.prompt_included = true;
        }
    }
}

impl LogitsProcessor for PresencePenalty {
    async fn name(&self) -> &'static str {
        "presence_penalty"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        self.include_prompt_if_needed(view.prompt).await;
        let penalty = self.penalty;
        let mut seen: std::collections::HashSet<TokenId> = std::collections::HashSet::new();
        for &idx in ws.live().await {
            let token = TokenId::new(idx);
            if self.counts.count(token).await > 0 {
                seen.insert(token);
            }
        }
        ws.add_bias_over_live(|token| if seen.contains(&token) { -penalty } else { 0.0 }).await;
        Ok(())
    }
    async fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
            self.counts.increment(token).await;
        }
        self.commit_log.push(token);
    }
    async fn save(&mut self) -> StateMark {
        StateMark(self.commit_log.len() as u64)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        while self.commit_log.len() as u64 > mark.0 {
            if let Some(token) = self.commit_log.pop() {
                if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
                    self.counts.decrement(token).await;
                }
            }
        }
    }
    async fn reset(&mut self) {
        self.counts.reset().await;
        self.prompt_included = false;
        self.commit_log.clear();
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { penalty: self.penalty, scope: self.scope, counts: self.counts.clone(), prompt_included: self.prompt_included, commit_log: self.commit_log.clone() }).into()
    }
}

/// ⚖️ Additive penalty proportional to a token's occurrence count.
pub struct FrequencyPenalty {
    pub penalty: f32,
    pub scope: PenaltyScope,
    pub(super) counts: FreqTable,
    prompt_included: bool,
    commit_log: Vec<TokenId>,
}

impl FrequencyPenalty {
    pub async fn new(penalty: f32, scope: PenaltyScope) -> Self {
        Self { penalty, scope, counts: FreqTable::new().await, prompt_included: false, commit_log: Vec::new() }
    }

    async fn include_prompt_if_needed(&mut self, prompt: &[TokenId]) {
        if !self.prompt_included && matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::PromptOnly) {
            for &t in prompt {
                self.counts.increment(t).await;
            }
            self.prompt_included = true;
        }
    }
}

impl LogitsProcessor for FrequencyPenalty {
    async fn name(&self) -> &'static str {
        "frequency_penalty"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        self.include_prompt_if_needed(view.prompt).await;
        let penalty = self.penalty;
        let mut counts_by_token: std::collections::HashMap<TokenId, u32> = std::collections::HashMap::new();
        for &idx in ws.live().await {
            let token = TokenId::new(idx);
            counts_by_token.insert(token, self.counts.count(token).await);
        }
        ws.add_bias_over_live(|token| -penalty * counts_by_token.get(&token).copied().unwrap_or(0) as f32).await;
        Ok(())
    }
    async fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
            self.counts.increment(token).await;
        }
        self.commit_log.push(token);
    }
    async fn save(&mut self) -> StateMark {
        StateMark(self.commit_log.len() as u64)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        while self.commit_log.len() as u64 > mark.0 {
            if let Some(token) = self.commit_log.pop() {
                if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
                    self.counts.decrement(token).await;
                }
            }
        }
    }
    async fn reset(&mut self) {
        self.counts.reset().await;
        self.prompt_included = false;
        self.commit_log.clear();
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { penalty: self.penalty, scope: self.scope, counts: self.counts.clone(), prompt_included: self.prompt_included, commit_log: self.commit_log.clone() }).into()
    }
}

/// ⚖️ Repetition penalty that decays exponentially with distance from the most recent occurrence,
/// over a bounded trailing `window` of generated tokens.
pub struct DecayingPenalty {
    pub penalty: f32,
    pub window: usize,
    pub half_life: f64,
    pub scope: PenaltyScope,
    history: std::collections::VecDeque<TokenId>,
    snapshots: Vec<std::collections::VecDeque<TokenId>>,
}

impl DecayingPenalty {
    pub async fn new(penalty: f32, window: usize, half_life: f64, scope: PenaltyScope) -> Self {
        Self { penalty, window, half_life, scope, history: std::collections::VecDeque::new(), snapshots: Vec::new() }
    }
}

impl LogitsProcessor for DecayingPenalty {
    async fn name(&self) -> &'static str {
        "decaying_penalty"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let half_life = self.half_life.max(1e-6);
        let penalty = self.penalty as f64;
        let mut bias: std::collections::HashMap<TokenId, f32> = std::collections::HashMap::new();
        for (i, &token) in self.history.iter().rev().enumerate() {
            let distance = (i + 1) as f64;
            let decay = 0.5f64.powf(distance / half_life);
            *bias.entry(token).or_insert(0.0) += (penalty * decay) as f32;
        }
        ws.add_bias_over_live(|token| -bias.get(&token).copied().unwrap_or(0.0)).await;
        Ok(())
    }
    async fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        self.history.push_back(token);
        if self.history.len() > self.window {
            self.history.pop_front();
        }
    }
    async fn save(&mut self) -> StateMark {
        self.snapshots.push(self.history.clone());
        StateMark((self.snapshots.len() - 1) as u64)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        if let Some(history) = self.snapshots.get(mark.0 as usize) {
            self.history = history.clone();
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    async fn reset(&mut self) {
        self.history.clear();
        self.snapshots.clear();
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { penalty: self.penalty, window: self.window, half_life: self.half_life, scope: self.scope, history: self.history.clone(), snapshots: self.snapshots.clone() }).into()
    }
}

/// ⚖️ Static multiplicative factor applied per token class (`class_tokens[c]` lists the tokens in
/// class `c`, `factors[c]` its factor). No per-sequence state, so `commit`/`save`/`rollback_to`
/// use their trait defaults.
pub struct TokenClassPenalty {
    class_of: std::collections::HashMap<TokenId, u16>,
    factors: Vec<f32>,
}

impl TokenClassPenalty {
    pub async fn new(class_tokens: Vec<Vec<TokenId>>, factors: Vec<f32>) -> Self {
        let mut class_of = std::collections::HashMap::new();
        for (class, tokens) in class_tokens.into_iter().enumerate() {
            for token in tokens {
                class_of.insert(token, class as u16);
            }
        }
        Self { class_of, factors }
    }
}

impl LogitsProcessor for TokenClassPenalty {
    async fn name(&self) -> &'static str {
        "token_class_penalty"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let class_of = &self.class_of;
        let factors = &self.factors;
        ws.transform_processed_over_live(|token, logit| match class_of.get(&token).and_then(|&c| factors.get(c as usize)) {
            Some(&factor) => logit * factor,
            None => logit,
        })
        .await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { class_of: self.class_of.clone(), factors: self.factors.clone() }).into()
    }
}

/// ⚖️ Hard-masks any token that would recreate an `n`-gram already present in the sequence's
/// history (prompt + generated).
pub struct NoRepeatNgram {
    pub n: usize,
    index: NgramIndex,
}

impl NoRepeatNgram {
    pub async fn new(n: usize) -> Self {
        Self { n, index: NgramIndex::new(n).await }
    }

    async fn full_history(view: &StepView<'_>) -> Vec<TokenId> {
        let mut history = Vec::with_capacity(view.prompt.len() + view.generated.len());
        history.extend_from_slice(view.prompt);
        history.extend_from_slice(view.generated);
        history
    }
}

impl LogitsProcessor for NoRepeatNgram {
    async fn name(&self) -> &'static str {
        "no_repeat_ngram"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let history = Self::full_history(view).await;
        for &token in self.index.forbidden_next(&history).await {
            ws.mask_mut().await.set(token, false).await;
        }
        Ok(())
    }
    async fn commit(&mut self, view: &StepView<'_>, token: TokenId) {
        let history = Self::full_history(view).await;
        self.index.record(&history, token).await;
    }
    async fn save(&mut self) -> StateMark {
        StateMark(self.index.commit_count().await)
    }
    async fn rollback_to(&mut self, mark: StateMark) {
        let current = self.index.commit_count().await;
        if current > mark.0 {
            self.index.rollback_last_n((current - mark.0) as usize).await;
        }
    }
    async fn reset(&mut self) {
        self.index.reset().await;
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { n: self.n, index: self.index.clone() }).into()
    }
}

/// ⚖️ Penalizes the final token of any phrase whose proper prefix the generated text currently
/// ends with (discourages completing that exact phrase). Stateless: reads `view.generated`
/// directly each step.
pub struct PhrasePenalty {
    pub phrases: Vec<Vec<TokenId>>,
    pub penalty: f32,
}

impl LogitsProcessor for PhrasePenalty {
    async fn name(&self) -> &'static str {
        "phrase_penalty"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let generated = view.generated;
        let penalty = self.penalty;
        let mut biased: std::collections::HashMap<TokenId, f32> = std::collections::HashMap::new();
        for phrase in &self.phrases {
            let Some((&last, prefix)) = phrase.split_last() else { continue };
            if generated.len() >= prefix.len() && &generated[generated.len() - prefix.len()..] == prefix {
                *biased.entry(last).or_insert(0.0) += penalty;
            }
        }
        ws.add_bias_over_live(|token| -biased.get(&token).copied().unwrap_or(0.0)).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { phrases: self.phrases.clone(), penalty: self.penalty }).into()
    }
}
// #endregion 🔖️Penalties
