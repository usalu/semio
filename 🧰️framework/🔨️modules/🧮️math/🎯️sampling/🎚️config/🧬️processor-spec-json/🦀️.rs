//! 🧬️ Processor spec JSON: the JSON form of every processor spec and its parser. See [`super`].

use super::*;

// #region 🔖️Config
pub(super) async fn processor_spec_to_json(spec: &ProcessorSpec) -> JsonValue {
    let obj = |pairs: Vec<(&str, JsonValue)>| JsonValue::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    let tokens_json = |tokens: &[TokenId]| JsonValue::Array(tokens.iter().map(|t| JsonValue::Num(t.get() as f64)).collect());
    let phrases_json = |phrases: &[Vec<TokenId>]| JsonValue::Array(phrases.iter().map(|p| tokens_json(p)).collect());
    match spec {
        ProcessorSpec::Temperature { value } => obj(vec![("kind", JsonValue::Str("temperature".into())), ("value", value.to_json().await)]),
        ProcessorSpec::DynamicTemperature { base, entropy_gain, min, max } => {
            obj(vec![("kind", JsonValue::Str("dynamic_temperature".into())), ("base", base.to_json().await), ("entropy_gain", JsonValue::Num(*entropy_gain)), ("min", JsonValue::Num(*min)), ("max", JsonValue::Num(*max))])
        }
        ProcessorSpec::TopK { k, min_keep } => obj(vec![("kind", JsonValue::Str("top_k".into())), ("k", k.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::TopP { p, min_keep } => obj(vec![("kind", JsonValue::Str("top_p".into())), ("p", p.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::MinP { p, min_keep } => obj(vec![("kind", JsonValue::Str("min_p".into())), ("p", p.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::Typical { mass, min_keep } => obj(vec![("kind", JsonValue::Str("typical".into())), ("mass", mass.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::LocallyTypical { mass, min_keep } => obj(vec![("kind", JsonValue::Str("locally_typical".into())), ("mass", mass.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::TailFree { z, min_keep } => obj(vec![("kind", JsonValue::Str("tail_free".into())), ("z", z.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::Epsilon { cutoff, min_keep } => obj(vec![("kind", JsonValue::Str("epsilon".into())), ("cutoff", cutoff.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::Eta { cutoff, min_keep } => obj(vec![("kind", JsonValue::Str("eta".into())), ("cutoff", cutoff.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::TopA { power, min_keep } => obj(vec![("kind", JsonValue::Str("top_a".into())), ("power", power.to_json().await), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::RankTruncation { max_rank } => obj(vec![("kind", JsonValue::Str("rank_truncation".into())), ("max_rank", JsonValue::Num(*max_rank as f64))]),
        ProcessorSpec::AdaptiveTruncation { target_entropy, target_effective_count } => {
            obj(vec![("kind", JsonValue::Str("adaptive_truncation".into())), ("target_entropy", target_entropy.map_or(JsonValue::Null, JsonValue::Num)), ("target_effective_count", target_effective_count.map_or(JsonValue::Null, JsonValue::Num))])
        }
        ProcessorSpec::RepetitionPenalty { penalty, scope } => obj(vec![("kind", JsonValue::Str("repetition_penalty".into())), ("penalty", JsonValue::Num(*penalty as f64)), ("scope", penalty_scope_to_json(*scope).await)]),
        ProcessorSpec::PresencePenalty { penalty, scope } => obj(vec![("kind", JsonValue::Str("presence_penalty".into())), ("penalty", JsonValue::Num(*penalty as f64)), ("scope", penalty_scope_to_json(*scope).await)]),
        ProcessorSpec::FrequencyPenalty { penalty, scope } => obj(vec![("kind", JsonValue::Str("frequency_penalty".into())), ("penalty", JsonValue::Num(*penalty as f64)), ("scope", penalty_scope_to_json(*scope).await)]),
        ProcessorSpec::DecayingPenalty { penalty, window, half_life, scope } => obj(vec![
            ("kind", JsonValue::Str("decaying_penalty".into())),
            ("penalty", JsonValue::Num(*penalty as f64)),
            ("window", JsonValue::Num(*window as f64)),
            ("half_life", JsonValue::Num(*half_life)),
            ("scope", penalty_scope_to_json(*scope).await),
        ]),
        ProcessorSpec::TokenClassPenalty { class_tokens, factors } => {
            obj(vec![("kind", JsonValue::Str("token_class_penalty".into())), ("class_tokens", phrases_json(class_tokens)), ("factors", JsonValue::Array(factors.iter().map(|f| JsonValue::Num(*f as f64)).collect()))])
        }
        ProcessorSpec::NoRepeatNgram { n } => obj(vec![("kind", JsonValue::Str("no_repeat_ngram".into())), ("n", JsonValue::Num(*n as f64))]),
        ProcessorSpec::PhrasePenalty { phrases, penalty } => obj(vec![("kind", JsonValue::Str("phrase_penalty".into())), ("phrases", phrases_json(phrases)), ("penalty", JsonValue::Num(*penalty as f64))]),
        ProcessorSpec::LogitBiasSparse { entries } => {
            obj(vec![("kind", JsonValue::Str("logit_bias_sparse".into())), ("entries", JsonValue::Array(entries.iter().map(|(t, b)| JsonValue::Array(vec![JsonValue::Num(t.get() as f64), JsonValue::Num(*b as f64)])).collect()))])
        }
        ProcessorSpec::LogitBiasDense { values } => obj(vec![("kind", JsonValue::Str("logit_bias_dense".into())), ("values", JsonValue::Array(values.iter().map(|v| JsonValue::Num(*v as f64)).collect()))]),
        ProcessorSpec::AllowTokens { tokens } => obj(vec![("kind", JsonValue::Str("allow_tokens".into())), ("tokens", tokens_json(tokens))]),
        ProcessorSpec::ForbidTokens { tokens } => obj(vec![("kind", JsonValue::Str("forbid_tokens".into())), ("tokens", tokens_json(tokens))]),
        ProcessorSpec::SuppressSpecial => obj(vec![("kind", JsonValue::Str("suppress_special".into()))]),
        ProcessorSpec::BadWords { phrases } => obj(vec![("kind", JsonValue::Str("bad_words".into())), ("phrases", phrases_json(phrases))]),
        ProcessorSpec::SequenceEncouragement { phrases, bonus } => obj(vec![("kind", JsonValue::Str("sequence_encouragement".into())), ("phrases", phrases_json(phrases)), ("bonus", JsonValue::Num(*bonus as f64))]),
        ProcessorSpec::Mirostat { version, target_surprise, learning_rate } => obj(vec![
            ("kind", JsonValue::Str("mirostat".into())),
            ("version", JsonValue::Str(if *version == MirostatVersion::V1 { "v1".into() } else { "v2".into() })),
            ("target_surprise", JsonValue::Num(*target_surprise)),
            ("learning_rate", JsonValue::Num(*learning_rate)),
        ]),
        ProcessorSpec::EntropyPid { target, kp, ki, kd } => obj(vec![("kind", JsonValue::Str("entropy_pid".into())), ("target", JsonValue::Num(*target)), ("kp", JsonValue::Num(*kp)), ("ki", JsonValue::Num(*ki)), ("kd", JsonValue::Num(*kd))]),
        ProcessorSpec::RepetitionController { window, threshold, boost } => {
            obj(vec![("kind", JsonValue::Str("repetition_controller".into())), ("window", JsonValue::Num(*window as f64)), ("threshold", JsonValue::Num(*threshold)), ("boost", JsonValue::Num(*boost))])
        }
        ProcessorSpec::ConfidenceController { low_entropy, high_entropy, low_temp, high_temp } => obj(vec![
            ("kind", JsonValue::Str("confidence_controller".into())),
            ("low_entropy", JsonValue::Num(*low_entropy)),
            ("high_entropy", JsonValue::Num(*high_entropy)),
            ("low_temp", JsonValue::Num(*low_temp)),
            ("high_temp", JsonValue::Num(*high_temp)),
        ]),
    }
}

pub(super) async fn processor_spec_from_json(value: &JsonValue) -> Result<ProcessorSpec, SamplingError> {
    async fn opt_str<'a>(value: &'a JsonValue, key: &'static str) -> Option<&'a str> {
        match value.get(key).await {
            Some(v) => v.as_str().await,
            None => None,
        }
    }
    async fn opt_num(value: &JsonValue, key: &'static str) -> Option<f64> {
        match value.get(key).await {
            Some(v) => v.as_f64().await,
            None => None,
        }
    }
    async fn num(value: &JsonValue, key: &'static str, default: f64) -> f64 {
        opt_num(value, key).await.unwrap_or(default)
    }
    async fn opt_array<'a>(value: &'a JsonValue, key: &'static str) -> Option<&'a [JsonValue]> {
        match value.get(key).await {
            Some(v) => v.as_array().await,
            None => None,
        }
    }
    async fn schedule(value: &JsonValue, key: &'static str) -> Result<Schedule, SamplingError> {
        let field = value.get(key).await.ok_or(SamplingError::Corrupted { reason: "processor missing schedule field" })?;
        Schedule::from_json(field).await
    }
    async fn f32_array(value: &JsonValue, key: &'static str) -> Vec<f32> {
        let Some(arr) = opt_array(value, key).await else { return Vec::new() };
        let mut out = Vec::with_capacity(arr.len());
        for item in arr {
            if let Some(n) = item.as_f64().await {
                out.push(n as f32);
            }
        }
        out
    }
    async fn tokens(value: &JsonValue, key: &'static str) -> Vec<TokenId> {
        let Some(arr) = opt_array(value, key).await else { return Vec::new() };
        let mut out = Vec::with_capacity(arr.len());
        for item in arr {
            if let Some(n) = item.as_f64().await {
                out.push(TokenId::new(n as u32));
            }
        }
        out
    }
    async fn phrases(value: &JsonValue, key: &'static str) -> Vec<Vec<TokenId>> {
        let Some(arr) = opt_array(value, key).await else { return Vec::new() };
        let mut out = Vec::with_capacity(arr.len());
        for item in arr {
            let Some(inner) = item.as_array().await else { continue };
            let mut phrase = Vec::with_capacity(inner.len());
            for n in inner {
                if let Some(f) = n.as_f64().await {
                    phrase.push(TokenId::new(f as u32));
                }
            }
            out.push(phrase);
        }
        out
    }

    let kind = opt_str(value, "kind").await.ok_or(SamplingError::Corrupted { reason: "processor missing kind" })?;
    match kind {
        "temperature" => Ok(ProcessorSpec::Temperature { value: schedule(value, "value").await? }),
        "dynamic_temperature" => Ok(ProcessorSpec::DynamicTemperature { base: schedule(value, "base").await?, entropy_gain: num(value, "entropy_gain", 0.0).await, min: num(value, "min", 0.0).await, max: num(value, "max", 2.0).await }),
        "top_k" => Ok(ProcessorSpec::TopK { k: schedule(value, "k").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "top_p" => Ok(ProcessorSpec::TopP { p: schedule(value, "p").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "min_p" => Ok(ProcessorSpec::MinP { p: schedule(value, "p").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "typical" => Ok(ProcessorSpec::Typical { mass: schedule(value, "mass").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "locally_typical" => Ok(ProcessorSpec::LocallyTypical { mass: schedule(value, "mass").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "tail_free" => Ok(ProcessorSpec::TailFree { z: schedule(value, "z").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "epsilon" => Ok(ProcessorSpec::Epsilon { cutoff: schedule(value, "cutoff").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "eta" => Ok(ProcessorSpec::Eta { cutoff: schedule(value, "cutoff").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "top_a" => Ok(ProcessorSpec::TopA { power: schedule(value, "power").await?, min_keep: num(value, "min_keep", 1.0).await as usize }),
        "rank_truncation" => Ok(ProcessorSpec::RankTruncation { max_rank: num(value, "max_rank", 1.0).await as usize }),
        "adaptive_truncation" => Ok(ProcessorSpec::AdaptiveTruncation { target_entropy: opt_num(value, "target_entropy").await, target_effective_count: opt_num(value, "target_effective_count").await }),
        "repetition_penalty" => Ok(ProcessorSpec::RepetitionPenalty { penalty: num(value, "penalty", 1.0).await as f32, scope: penalty_scope_from_json(value.get("scope").await).await? }),
        "presence_penalty" => Ok(ProcessorSpec::PresencePenalty { penalty: num(value, "penalty", 0.0).await as f32, scope: penalty_scope_from_json(value.get("scope").await).await? }),
        "frequency_penalty" => Ok(ProcessorSpec::FrequencyPenalty { penalty: num(value, "penalty", 0.0).await as f32, scope: penalty_scope_from_json(value.get("scope").await).await? }),
        "decaying_penalty" => Ok(ProcessorSpec::DecayingPenalty {
            penalty: num(value, "penalty", 0.0).await as f32,
            window: num(value, "window", 16.0).await as usize,
            half_life: num(value, "half_life", 1.0).await,
            scope: penalty_scope_from_json(value.get("scope").await).await?,
        }),
        "token_class_penalty" => Ok(ProcessorSpec::TokenClassPenalty { class_tokens: phrases(value, "class_tokens").await, factors: f32_array(value, "factors").await }),
        "no_repeat_ngram" => Ok(ProcessorSpec::NoRepeatNgram { n: num(value, "n", 3.0).await as usize }),
        "phrase_penalty" => Ok(ProcessorSpec::PhrasePenalty { phrases: phrases(value, "phrases").await, penalty: num(value, "penalty", 0.0).await as f32 }),
        "logit_bias_sparse" => {
            let mut entries = Vec::new();
            if let Some(arr) = opt_array(value, "entries").await {
                for pair_value in arr {
                    let Some(pair) = pair_value.as_array().await else { continue };
                    let (Some(first), Some(second)) = (pair.first(), pair.get(1)) else { continue };
                    let (Some(token), Some(bias)) = (first.as_f64().await, second.as_f64().await) else { continue };
                    entries.push((TokenId::new(token as u32), bias as f32));
                }
            }
            Ok(ProcessorSpec::LogitBiasSparse { entries })
        }
        "logit_bias_dense" => Ok(ProcessorSpec::LogitBiasDense { values: f32_array(value, "values").await }),
        "allow_tokens" => Ok(ProcessorSpec::AllowTokens { tokens: tokens(value, "tokens").await }),
        "forbid_tokens" => Ok(ProcessorSpec::ForbidTokens { tokens: tokens(value, "tokens").await }),
        "suppress_special" => Ok(ProcessorSpec::SuppressSpecial),
        "bad_words" => Ok(ProcessorSpec::BadWords { phrases: phrases(value, "phrases").await }),
        "sequence_encouragement" => Ok(ProcessorSpec::SequenceEncouragement { phrases: phrases(value, "phrases").await, bonus: num(value, "bonus", 0.0).await as f32 }),
        "mirostat" => Ok(ProcessorSpec::Mirostat {
            version: if opt_str(value, "version").await == Some("v1") { MirostatVersion::V1 } else { MirostatVersion::V2 },
            target_surprise: num(value, "target_surprise", 5.0).await,
            learning_rate: num(value, "learning_rate", 0.1).await,
        }),
        "entropy_pid" => Ok(ProcessorSpec::EntropyPid { target: num(value, "target", 2.0).await, kp: num(value, "kp", 0.1).await, ki: num(value, "ki", 0.0).await, kd: num(value, "kd", 0.0).await }),
        "repetition_controller" => Ok(ProcessorSpec::RepetitionController { window: num(value, "window", 16.0).await as usize, threshold: num(value, "threshold", 0.5).await, boost: num(value, "boost", 0.2).await }),
        "confidence_controller" => {
            Ok(ProcessorSpec::ConfidenceController { low_entropy: num(value, "low_entropy", 0.5).await, high_entropy: num(value, "high_entropy", 3.0).await, low_temp: num(value, "low_temp", 0.5).await, high_temp: num(value, "high_temp", 1.2).await })
        }
        _ => Err(SamplingError::Corrupted { reason: "unknown processor kind" }),
    }
}
// #endregion 🔖️Config
