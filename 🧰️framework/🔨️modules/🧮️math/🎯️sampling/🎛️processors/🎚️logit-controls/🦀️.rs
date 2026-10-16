//! 🎚️ Logit biases, token allow and forbid lists, bad words, and length controls.

use semio_framework_math_sampling_vocabulary::{SamplingError, TokenBitset, TokenId};
use semio_framework_math_sampling_config::ForcedSpec;
use semio_framework_math_sampling_workspace::{LogitsWorkspace, ProcessorKind, StepView};
use super::LogitsProcessor;

// #region 🔖️Biases
/// 🧲️ Sparse per-token additive logit bias.
pub struct LogitBiasSparse {
    pub entries: Vec<(TokenId, f32)>,
}

impl LogitsProcessor for LogitBiasSparse {
    async fn name(&self) -> &'static str {
        "logit_bias_sparse"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        for &(token, bias) in &self.entries {
            ws.bias_processed(token, bias).await;
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { entries: self.entries.clone() }).into()
    }
}

/// 🧲️ Dense per-token additive logit bias (`values[i]` biases token `i`).
pub struct LogitBiasDense {
    pub values: Vec<f32>,
}

impl LogitsProcessor for LogitBiasDense {
    async fn name(&self) -> &'static str {
        "logit_bias_dense"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        for (i, &delta) in self.values.iter().enumerate() {
            ws.bias_processed(TokenId::new(i as u32), delta).await;
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { values: self.values.clone() }).into()
    }
}

/// 🧲️ Hard-restricts the live set to exactly `tokens` (intersected with whatever survives earlier
/// hard-mask processors).
pub struct AllowTokens {
    pub tokens: Vec<TokenId>,
}

impl LogitsProcessor for AllowTokens {
    async fn name(&self) -> &'static str {
        "allow_tokens"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let mut allow = TokenBitset::new_empty(ws.vocab_size().await).await;
        for &token in &self.tokens {
            allow.set(token, true).await;
        }
        ws.mask_mut().await.and_with(&allow).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { tokens: self.tokens.clone() }).into()
    }
}

/// 🧲️ Hard-excludes `tokens` from the live set.
pub struct ForbidTokens {
    pub tokens: Vec<TokenId>,
}

impl LogitsProcessor for ForbidTokens {
    async fn name(&self) -> &'static str {
        "forbid_tokens"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        for &token in &self.tokens {
            ws.mask_mut().await.set(token, false).await;
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { tokens: self.tokens.clone() }).into()
    }
}

/// 🧲️ Hard-excludes every token in the vocabulary's [`Vocabulary::special`] set.
pub struct SuppressSpecial;

impl LogitsProcessor for SuppressSpecial {
    async fn name(&self) -> &'static str {
        "suppress_special"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.mask_mut().await.and_not_with(&view.vocab.special).await;
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self).into()
    }
}

/// 🧲️ Hard-excludes the final token of any bad-word phrase whose proper prefix the generated text
/// currently ends with (prefix-sensitive multi-token bad-word suppression).
pub struct BadWords {
    pub phrases: Vec<Vec<TokenId>>,
}

impl LogitsProcessor for BadWords {
    async fn name(&self) -> &'static str {
        "bad_words"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let generated = view.generated;
        for phrase in &self.phrases {
            let Some((&last, prefix)) = phrase.split_last() else { continue };
            if generated.len() >= prefix.len() && &generated[generated.len() - prefix.len()..] == prefix {
                ws.mask_mut().await.set(last, false).await;
            }
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { phrases: self.phrases.clone() }).into()
    }
}

/// 🧲️ Positive bias toward completing a configured phrase once the generated text ends with its
/// proper prefix (the encouragement counterpart to [`BadWords`]/[`PhrasePenalty`]).
pub struct SequenceEncouragement {
    pub phrases: Vec<Vec<TokenId>>,
    pub bonus: f32,
}

impl LogitsProcessor for SequenceEncouragement {
    async fn name(&self) -> &'static str {
        "sequence_encouragement"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let generated = view.generated;
        let bonus = self.bonus;
        for phrase in &self.phrases {
            let Some((&last, prefix)) = phrase.split_last() else { continue };
            if generated.len() >= prefix.len() && &generated[generated.len() - prefix.len()..] == prefix {
                ws.bias_processed(last, bonus).await;
            }
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { phrases: self.phrases.clone(), bonus: self.bonus }).into()
    }
}
// #endregion 🔖️Biases

// #region 🔖️Length
/// 📏️ Hard-suppresses every EOS token until `min_tokens` generated tokens have been produced.
pub struct MinLengthEosSuppression {
    pub min_tokens: usize,
}

impl LogitsProcessor for MinLengthEosSuppression {
    async fn name(&self) -> &'static str {
        "min_length_eos_suppression"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        if view.generated.len() < self.min_tokens {
            for &eos in &view.vocab.eos {
                ws.mask_mut().await.set(eos, false).await;
            }
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { min_tokens: self.min_tokens }).into()
    }
}

/// 📏️ Hard-restricts the live set to (the first configured) EOS token once the *next* token would
/// reach `max_tokens`, guaranteeing generation stops cleanly at the length cap.
pub struct MaxLengthForceEos {
    pub max_tokens: usize,
}

impl LogitsProcessor for MaxLengthForceEos {
    async fn name(&self) -> &'static str {
        "max_length_force_eos"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        if view.generated.len() + 1 >= self.max_tokens {
            if let Some(&eos) = view.vocab.eos.first() {
                let mut only_eos = TokenBitset::new_empty(ws.vocab_size().await).await;
                only_eos.set(eos, true).await;
                ws.mask_mut().await.and_with(&only_eos).await;
            }
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { max_tokens: self.max_tokens }).into()
    }
}

/// 📏️ Hard-forces an exact token at step 0 (`bos`), steps `1..=prefix.len()` (`prefix`), or any
/// explicitly listed `(step, token)` pair thereafter.
pub struct ForcedTokens {
    pub spec: ForcedSpec,
}

impl ForcedTokens {
    async fn forced_token_at(&self, step: u32) -> Option<TokenId> {
        if step == 0 {
            if let Some(bos) = self.spec.bos {
                return Some(bos);
            }
        }
        if step >= 1 && (step as usize) <= self.spec.prefix.len() {
            return self.spec.prefix.get(step as usize - 1).copied();
        }
        self.spec.at_position.iter().find(|(s, _)| s.get() == step).map(|(_, t)| *t)
    }
}

impl LogitsProcessor for ForcedTokens {
    async fn name(&self) -> &'static str {
        "forced_tokens"
    }
    async fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        if let Some(token) = self.forced_token_at(view.step.get()).await {
            let mut only = TokenBitset::new_empty(ws.vocab_size().await).await;
            only.set(token, true).await;
            ws.mask_mut().await.and_with(&only).await;
        }
        Ok(())
    }
    async fn fork(&self) -> LogitsProcessors {
        (Self { spec: self.spec.clone() }).into()
    }
}
// #endregion 🔖️Length
