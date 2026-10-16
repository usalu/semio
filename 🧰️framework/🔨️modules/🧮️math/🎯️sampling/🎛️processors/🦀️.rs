//! 🎛️ Token-sampling logits processors and samplers: warpers, penalties, biases, length controls, selection samplers and adaptive controllers.

#![allow(async_fn_in_trait)]

use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
use semio_framework_math_sampling_vocabulary::{RandomSources, SamplingError, TokenId};
use semio_framework_math_sampling_config::{ProcessorSpec, SamplingMethod};
use semio_framework_math_sampling_workspace::{Distribution, LogitsWorkspace, ProcessorKind, SelectionBuffer, StateMark, StepView};
#[cfg(test)] use semio_framework_math_sampling_vocabulary::entropy_nats;
#[cfg(test)] use semio_framework_math_sampling_config::{ForcedSpec, MirostatVersion, MultinomialStrategy, PenaltyScope, Schedule, TieBreak};

#[path = "🌡️warpers/🦀️.rs"]
mod warpers;
pub use warpers::*;
#[path = "⚖️penalties/🦀️.rs"]
mod penalties;
pub use penalties::*;
#[path = "🎚️logit-controls/🦀️.rs"]
mod logit_controls;
pub use logit_controls::*;
#[path = "🎯️selection/🦀️.rs"]
mod selection;
pub use selection::*;
#[path = "🌀️adaptive/🦀️.rs"]
mod adaptive;
pub use adaptive::*;

/// 🔌️ One step of the logits pipeline: transforms `ws` in place (mask, soft penalty, or
/// truncation), and optionally commits per-sequence effects once a token is definitively accepted.
#[dyn_enum]
pub trait LogitsProcessor {
    async fn name(&self) -> &'static str;
    async fn kind(&self) -> ProcessorKind;
    async fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError>;
    async fn commit(&mut self, _view: &StepView<'_>, _token: TokenId) {}
    async fn save(&mut self) -> StateMark {
        StateMark::default()
    }
    async fn rollback_to(&mut self, _mark: StateMark) {}
    async fn reset(&mut self) {}
    async fn fork(&self) -> LogitsProcessors;
}

/// 🔌️ Selects one or more tokens from the final [`Distribution`]; must write at least one
/// candidate into `out` or return an error.
#[dyn_enum]
pub trait TokenSampler {
    async fn name(&self) -> &'static str;
    async fn sample(&mut self, view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut RandomSources, out: &mut SelectionBuffer) -> Result<(), SamplingError>;
    async fn fork(&self) -> TokenSamplers;
}

// #region 🔖️Warpers
/// 🌡️ Builds a [`LogitsProcessor`] from a [`ProcessorSpec`]; exhaustive over every variant once
/// the warpers (`🔖️Warpers`), penalties (`🔖️Penalties`), biases (`🔖️Biases`), and adaptive
/// samplers (`🔖️Adaptive`) regions have all landed.
pub async fn build_processor(spec: &ProcessorSpec) -> Result<LogitsProcessors, SamplingError> {
    match spec {
        ProcessorSpec::Temperature { value } => Ok((Temperature { value: value.clone() }).into()),
        ProcessorSpec::DynamicTemperature { base, entropy_gain, min, max } => Ok((DynamicTemperature { base: base.clone(), entropy_gain: *entropy_gain, min: *min, max: *max }).into()),
        ProcessorSpec::TopK { k, min_keep } => Ok((TopK { k: k.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::TopP { p, min_keep } => Ok((TopP { p: p.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::MinP { p, min_keep } => Ok((MinP { p: p.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::Typical { mass, min_keep } => Ok((Typical { mass: mass.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::LocallyTypical { mass, min_keep } => Ok((LocallyTypical { mass: mass.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::TailFree { z, min_keep } => Ok((TailFree { z: z.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::Epsilon { cutoff, min_keep } => Ok((EpsilonCutoff { cutoff: cutoff.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::Eta { cutoff, min_keep } => Ok((EtaCutoff { cutoff: cutoff.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::TopA { power, min_keep } => Ok((TopA { power: power.clone(), min_keep: *min_keep }).into()),
        ProcessorSpec::RankTruncation { max_rank } => Ok((RankTruncation { max_rank: *max_rank }).into()),
        ProcessorSpec::AdaptiveTruncation { target_entropy, target_effective_count } => Ok((AdaptiveTruncation { target_entropy: *target_entropy, target_effective_count: *target_effective_count }).into()),
        ProcessorSpec::RepetitionPenalty { penalty, scope } => Ok((RepetitionPenalty::new(*penalty, *scope).await).into()),
        ProcessorSpec::PresencePenalty { penalty, scope } => Ok((PresencePenalty::new(*penalty, *scope).await).into()),
        ProcessorSpec::FrequencyPenalty { penalty, scope } => Ok((FrequencyPenalty::new(*penalty, *scope).await).into()),
        ProcessorSpec::DecayingPenalty { penalty, window, half_life, scope } => Ok((DecayingPenalty::new(*penalty, *window, *half_life, *scope).await).into()),
        ProcessorSpec::TokenClassPenalty { class_tokens, factors } => Ok((TokenClassPenalty::new(class_tokens.clone(), factors.clone()).await).into()),
        ProcessorSpec::NoRepeatNgram { n } => Ok((NoRepeatNgram::new(*n).await).into()),
        ProcessorSpec::PhrasePenalty { phrases, penalty } => Ok((PhrasePenalty { phrases: phrases.clone(), penalty: *penalty }).into()),
        ProcessorSpec::LogitBiasSparse { entries } => Ok((LogitBiasSparse { entries: entries.clone() }).into()),
        ProcessorSpec::LogitBiasDense { values } => Ok((LogitBiasDense { values: values.clone() }).into()),
        ProcessorSpec::AllowTokens { tokens } => Ok((AllowTokens { tokens: tokens.clone() }).into()),
        ProcessorSpec::ForbidTokens { tokens } => Ok((ForbidTokens { tokens: tokens.clone() }).into()),
        ProcessorSpec::SuppressSpecial => Ok((SuppressSpecial).into()),
        ProcessorSpec::BadWords { phrases } => Ok((BadWords { phrases: phrases.clone() }).into()),
        ProcessorSpec::SequenceEncouragement { phrases, bonus } => Ok((SequenceEncouragement { phrases: phrases.clone(), bonus: *bonus }).into()),
        ProcessorSpec::Mirostat { version, target_surprise, learning_rate } => Ok((Mirostat::new(*version, *target_surprise, *learning_rate).await).into()),
        ProcessorSpec::EntropyPid { target, kp, ki, kd } => Ok((EntropyPid::new(*target, *kp, *ki, *kd).await).into()),
        ProcessorSpec::RepetitionController { window, threshold, boost } => Ok((RepetitionController::new(*window, *threshold, *boost).await).into()),
        ProcessorSpec::ConfidenceController { low_entropy, high_entropy, low_temp, high_temp } => Ok((ConfidenceController { low_entropy: *low_entropy, high_entropy: *high_entropy, low_temp: *low_temp, high_temp: *high_temp }).into()),
    }
}
// #endregion 🔖️Warpers

// #region 🔖️Selection

/// 🎯️ Builds a [`TokenSampler`] from a [`SamplingMethod`].
pub async fn build_sampler(method: &SamplingMethod) -> TokenSamplers {
    match method {
        SamplingMethod::Greedy { tie_break } => (GreedySampler { tie_break: *tie_break }).into(),
        SamplingMethod::Multinomial { strategy } => (MultinomialSampler { strategy: *strategy }).into(),
        SamplingMethod::GumbelMax => (GumbelMaxSampler).into(),
        SamplingMethod::GumbelTopK { k } => (GumbelTopKSampler { k: *k }).into(),
    }
}

dyn_enum_close! {
    pub enum TokenSamplers: TokenSampler {
        Greedy(GreedySampler),
        Multinomial(MultinomialSampler),
        GumbelMax(GumbelMaxSampler),
        GumbelTopK(GumbelTopKSampler),
    }
}
// #endregion 🔖️Selection

// #region 🔖️Adaptive

dyn_enum_close! {
    pub enum LogitsProcessors: LogitsProcessor {
        Temperature(Temperature),
        DynamicTemperature(DynamicTemperature),
        TopK(TopK),
        TopP(TopP),
        MinP(MinP),
        Typical(Typical),
        LocallyTypical(LocallyTypical),
        TailFree(TailFree),
        EpsilonCutoff(EpsilonCutoff),
        EtaCutoff(EtaCutoff),
        TopA(TopA),
        RankTruncation(RankTruncation),
        AdaptiveTruncation(AdaptiveTruncation),
        RepetitionPenalty(RepetitionPenalty),
        PresencePenalty(PresencePenalty),
        FrequencyPenalty(FrequencyPenalty),
        DecayingPenalty(DecayingPenalty),
        TokenClassPenalty(TokenClassPenalty),
        NoRepeatNgram(NoRepeatNgram),
        PhrasePenalty(PhrasePenalty),
        LogitBiasSparse(LogitBiasSparse),
        LogitBiasDense(LogitBiasDense),
        AllowTokens(AllowTokens),
        ForbidTokens(ForbidTokens),
        SuppressSpecial(SuppressSpecial),
        BadWords(BadWords),
        SequenceEncouragement(SequenceEncouragement),
        MinLengthEosSuppression(MinLengthEosSuppression),
        MaxLengthForceEos(MaxLengthForceEos),
        ForcedTokens(ForcedTokens),
        Mirostat(Mirostat),
        EntropyPid(EntropyPid),
        RepetitionController(RepetitionController),
        ConfidenceController(ConfidenceController),
    }
}
// #endregion 🔖️Adaptive

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
