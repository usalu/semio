//! 🧾️ Enum JSON and spec validation: the processor spec validation and the JSON forms of the sampling method, tie break, multinomial strategy and penalty scope. See [`super`].

use super::*;

// #region 🔖️Config
pub(super) async fn validate_processor_spec(spec: &ProcessorSpec, limits: &SamplingLimits) -> Result<(), SamplingError> {
    match spec {
        ProcessorSpec::NoRepeatNgram { n } if *n == 0 || *n > limits.max_ngram_order => {
            return Err(SamplingError::LimitExceeded { limit: "max_ngram_order" });
        }
        ProcessorSpec::TokenClassPenalty { class_tokens, factors } if class_tokens.len() != factors.len() => {
            return Err(SamplingError::InvalidConfig { field: "token_class_penalty", reason: "class_tokens and factors must have equal length" });
        }
        ProcessorSpec::RankTruncation { max_rank } if *max_rank == 0 => {
            return Err(SamplingError::InvalidConfig { field: "rank_truncation.max_rank", reason: "must be >= 1" });
        }
        _ => {}
    }
    Ok(())
}

pub(super) async fn sampling_method_to_json(method: &SamplingMethod) -> JsonValue {
    let obj = |pairs: Vec<(&str, JsonValue)>| JsonValue::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    match method {
        SamplingMethod::Greedy { tie_break } => obj(vec![("kind", JsonValue::Str("greedy".into())), ("tie_break", tie_break_to_json(*tie_break).await)]),
        SamplingMethod::Multinomial { strategy } => obj(vec![("kind", JsonValue::Str("multinomial".into())), ("strategy", multinomial_strategy_to_json(*strategy).await)]),
        SamplingMethod::GumbelMax => obj(vec![("kind", JsonValue::Str("gumbel_max".into()))]),
        SamplingMethod::GumbelTopK { k } => obj(vec![("kind", JsonValue::Str("gumbel_top_k".into())), ("k", JsonValue::Num(*k as f64))]),
    }
}

pub(super) async fn sampling_method_from_json(value: &JsonValue) -> Result<SamplingMethod, SamplingError> {
    let kind = match value.get("kind").await {
        Some(v) => v.as_str().await,
        None => None,
    }
    .ok_or(SamplingError::Corrupted { reason: "method missing kind" })?;
    match kind {
        "greedy" => Ok(SamplingMethod::Greedy { tie_break: tie_break_from_json(value.get("tie_break").await).await? }),
        "multinomial" => Ok(SamplingMethod::Multinomial { strategy: multinomial_strategy_from_json(value.get("strategy").await).await? }),
        "gumbel_max" => Ok(SamplingMethod::GumbelMax),
        "gumbel_top_k" => {
            let k = match value.get("k").await {
                Some(v) => v.as_f64().await,
                None => None,
            };
            Ok(SamplingMethod::GumbelTopK { k: k.unwrap_or(1.0) as usize })
        }
        _ => Err(SamplingError::Corrupted { reason: "unknown sampling method kind" }),
    }
}

async fn tie_break_to_json(tie_break: TieBreak) -> JsonValue {
    JsonValue::Str(
        match tie_break {
            TieBreak::LowestTokenId => "lowest_token_id",
            TieBreak::HighestTokenId => "highest_token_id",
            TieBreak::FirstSeen => "first_seen",
        }
        .into(),
    )
}

async fn tie_break_from_json(value: Option<&JsonValue>) -> Result<TieBreak, SamplingError> {
    let as_str = match value {
        Some(v) => v.as_str().await,
        None => None,
    };
    match as_str {
        Some("lowest_token_id") | None => Ok(TieBreak::LowestTokenId),
        Some("highest_token_id") => Ok(TieBreak::HighestTokenId),
        Some("first_seen") => Ok(TieBreak::FirstSeen),
        Some(_) => Err(SamplingError::Corrupted { reason: "unknown tie_break" }),
    }
}

async fn multinomial_strategy_to_json(strategy: MultinomialStrategy) -> JsonValue {
    JsonValue::Str(
        match strategy {
            MultinomialStrategy::CdfBinarySearch => "cdf_binary_search",
            MultinomialStrategy::LinearScan => "linear_scan",
            MultinomialStrategy::Alias => "alias",
        }
        .into(),
    )
}

async fn multinomial_strategy_from_json(value: Option<&JsonValue>) -> Result<MultinomialStrategy, SamplingError> {
    let as_str = match value {
        Some(v) => v.as_str().await,
        None => None,
    };
    match as_str {
        Some("cdf_binary_search") | None => Ok(MultinomialStrategy::CdfBinarySearch),
        Some("linear_scan") => Ok(MultinomialStrategy::LinearScan),
        Some("alias") => Ok(MultinomialStrategy::Alias),
        Some(_) => Err(SamplingError::Corrupted { reason: "unknown multinomial strategy" }),
    }
}

pub(super) async fn penalty_scope_to_json(scope: PenaltyScope) -> JsonValue {
    JsonValue::Str(
        match scope {
            PenaltyScope::PromptAndGenerated => "prompt_and_generated",
            PenaltyScope::GeneratedOnly => "generated_only",
            PenaltyScope::PromptOnly => "prompt_only",
        }
        .into(),
    )
}

pub(super) async fn penalty_scope_from_json(value: Option<&JsonValue>) -> Result<PenaltyScope, SamplingError> {
    let as_str = match value {
        Some(v) => v.as_str().await,
        None => None,
    };
    match as_str {
        Some("prompt_and_generated") | None => Ok(PenaltyScope::PromptAndGenerated),
        Some("generated_only") => Ok(PenaltyScope::GeneratedOnly),
        Some("prompt_only") => Ok(PenaltyScope::PromptOnly),
        Some(_) => Err(SamplingError::Corrupted { reason: "unknown penalty scope" }),
    }
}
// #endregion 🔖️Config
