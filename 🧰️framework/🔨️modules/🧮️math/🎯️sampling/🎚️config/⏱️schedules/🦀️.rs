//! ⏱️ Schedules of the sampling configuration: the schedule input, the schedule curves with their evaluation and the schedule progress. See [`super`].

use super::*;

// #region 🔖️Schedules
/// 📅️ What a [`Schedule`] is evaluated against at one sampling step.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ScheduleInput {
    pub step: StepIndex,
    pub generated_len: usize,
    pub last_entropy: Option<f64>,
}

/// 📅️ A parameter value that may vary over the course of generation. Every warper/penalty knob
/// that the feature tree calls out as schedulable is `Schedule`-typed in [`ProcessorSpec`].
///
/// `PartialEq` compares `Callback` variants by function-pointer identity — good enough to
/// distinguish "the same fn item" from "a different one" in tests/config diffing, even though two
/// pointers to the same fn aren't guaranteed unique across monomorphizations; hence the allow.
#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Clone, PartialEq, Debug)]
pub enum Schedule {
    Constant(f64),
    Linear {
        from: f64,
        to: f64,
        over_steps: u32,
    },
    Exponential {
        from: f64,
        to: f64,
        over_steps: u32,
    },
    Cosine {
        from: f64,
        to: f64,
        over_steps: u32,
    },
    /// 📅️ Step-indexed breakpoints; the value holds at the most recent breakpoint `<= step`.
    Piecewise(Vec<(StepIndex, f64)>),
    /// 📅️ One value per generated-token position, clamped to the last entry past its length.
    ByPosition(Vec<f64>),
    EntropyScaled {
        base: f64,
        gain: f64,
        min: f64,
        max: f64,
    },
    /// 📅️ Escape hatch for host-defined logic; not text-serializable (see [`Schedule::to_json`]).
    Callback(fn(ScheduleInput) -> f64),
}

impl Schedule {
    /// 📅️ Evaluates the schedule at `input`.
    pub async fn eval(&self, input: ScheduleInput) -> f64 {
        match self {
            Self::Constant(v) => *v,
            Self::Linear { from, to, over_steps } => {
                let t = schedule_progress(input.step, *over_steps).await;
                from + (to - from) * t
            }
            Self::Exponential { from, to, over_steps } => {
                let t = schedule_progress(input.step, *over_steps).await;
                if *from > 0.0 && *to > 0.0 {
                    from * (to / from).powf(t)
                } else {
                    from + (to - from) * t
                }
            }
            Self::Cosine { from, to, over_steps } => {
                let t = schedule_progress(input.step, *over_steps).await;
                let cos_t = 0.5 * (1.0 - (core::f64::consts::PI * t).cos());
                from + (to - from) * cos_t
            }
            Self::Piecewise(pieces) => {
                let mut value = pieces.first().map_or(0.0, |(_, v)| *v);
                for (step, v) in pieces {
                    if step.get() <= input.step.get() {
                        value = *v;
                    } else {
                        break;
                    }
                }
                value
            }
            Self::ByPosition(values) => {
                if values.is_empty() {
                    0.0
                } else {
                    values[input.generated_len.min(values.len() - 1)]
                }
            }
            Self::EntropyScaled { base, gain, min, max } => {
                let entropy = input.last_entropy.unwrap_or(0.0);
                (base + gain * entropy).clamp(*min, *max)
            }
            Self::Callback(f) => f(input),
        }
    }

    /// 📅️ Structured form for config serialization; `Callback` encodes as a marker that
    /// deliberately fails to round-trip (see [`Schedule::from_json`]).
    pub async fn to_json(&self) -> JsonValue {
        let obj = |pairs: Vec<(&str, JsonValue)>| JsonValue::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
        match self {
            Self::Constant(v) => obj(vec![("kind", JsonValue::Str("constant".into())), ("value", JsonValue::Num(*v))]),
            Self::Linear { from, to, over_steps } => obj(vec![("kind", JsonValue::Str("linear".into())), ("from", JsonValue::Num(*from)), ("to", JsonValue::Num(*to)), ("over_steps", JsonValue::Num(*over_steps as f64))]),
            Self::Exponential { from, to, over_steps } => obj(vec![("kind", JsonValue::Str("exponential".into())), ("from", JsonValue::Num(*from)), ("to", JsonValue::Num(*to)), ("over_steps", JsonValue::Num(*over_steps as f64))]),
            Self::Cosine { from, to, over_steps } => obj(vec![("kind", JsonValue::Str("cosine".into())), ("from", JsonValue::Num(*from)), ("to", JsonValue::Num(*to)), ("over_steps", JsonValue::Num(*over_steps as f64))]),
            Self::Piecewise(pieces) => obj(vec![("kind", JsonValue::Str("piecewise".into())), ("pieces", JsonValue::Array(pieces.iter().map(|(s, v)| JsonValue::Array(vec![JsonValue::Num(s.get() as f64), JsonValue::Num(*v)])).collect()))]),
            Self::ByPosition(values) => obj(vec![("kind", JsonValue::Str("by_position".into())), ("values", JsonValue::Array(values.iter().map(|v| JsonValue::Num(*v)).collect()))]),
            Self::EntropyScaled { base, gain, min, max } => obj(vec![("kind", JsonValue::Str("entropy_scaled".into())), ("base", JsonValue::Num(*base)), ("gain", JsonValue::Num(*gain)), ("min", JsonValue::Num(*min)), ("max", JsonValue::Num(*max))]),
            Self::Callback(_) => obj(vec![("kind", JsonValue::Str("callback".into()))]),
        }
    }

    /// 📅️ Inverse of [`Schedule::to_json`]; rejects `"callback"` since function pointers are not
    /// recoverable from serialized data.
    pub async fn from_json(value: &JsonValue) -> Result<Self, SamplingError> {
        // 🔀️ `let num = |key| value.get(key).and_then(JsonValue::as_f64)` no longer type-checks
        // once `JsonValue::get`/`as_f64` are `async fn` — `Option::and_then` takes a plain
        // `FnOnce`, not an async one, and a captured-`value` sync closure cannot itself `.await`.
        // A nested `async fn` taking `value` explicitly replaces it (O1 de-dyn, math-dedyn).
        async fn num(value: &JsonValue, key: &'static str) -> Result<f64, SamplingError> {
            let found = match value.get(key).await {
                Some(v) => v.as_f64().await,
                None => None,
            };
            found.ok_or(SamplingError::Corrupted { reason: "schedule missing numeric field" })
        }
        let kind = match value.get("kind").await {
            Some(v) => v.as_str().await,
            None => None,
        }
        .ok_or(SamplingError::Corrupted { reason: "schedule missing kind" })?;
        match kind {
            "constant" => Ok(Self::Constant(num(value, "value").await?)),
            "linear" => Ok(Self::Linear { from: num(value, "from").await?, to: num(value, "to").await?, over_steps: num(value, "over_steps").await? as u32 }),
            "exponential" => Ok(Self::Exponential { from: num(value, "from").await?, to: num(value, "to").await?, over_steps: num(value, "over_steps").await? as u32 }),
            "cosine" => Ok(Self::Cosine { from: num(value, "from").await?, to: num(value, "to").await?, over_steps: num(value, "over_steps").await? as u32 }),
            "piecewise" => {
                let pieces = match value.get("pieces").await {
                    Some(v) => v.as_array().await,
                    None => None,
                }
                .ok_or(SamplingError::Corrupted { reason: "piecewise schedule missing pieces" })?;
                let mut out = Vec::with_capacity(pieces.len());
                for piece in pieces {
                    let pair = piece.as_array().await.ok_or(SamplingError::Corrupted { reason: "piecewise entry must be a pair" })?;
                    let step = match pair.first() {
                        Some(v) => v.as_f64().await,
                        None => None,
                    };
                    let v = match pair.get(1) {
                        Some(v) => v.as_f64().await,
                        None => None,
                    };
                    let (Some(step), Some(v)) = (step, v) else {
                        return Err(SamplingError::Corrupted { reason: "piecewise entry must be [step, value]" });
                    };
                    out.push((StepIndex::new(step as u32), v));
                }
                Ok(Self::Piecewise(out))
            }
            "by_position" => {
                let values = match value.get("values").await {
                    Some(v) => v.as_array().await,
                    None => None,
                }
                .ok_or(SamplingError::Corrupted { reason: "by_position schedule missing values" })?;
                let mut out = Vec::with_capacity(values.len());
                for v in values {
                    out.push(v.as_f64().await.ok_or(SamplingError::Corrupted { reason: "by_position value must be numeric" })?);
                }
                Ok(Self::ByPosition(out))
            }
            "entropy_scaled" => Ok(Self::EntropyScaled { base: num(value, "base").await?, gain: num(value, "gain").await?, min: num(value, "min").await?, max: num(value, "max").await? }),
            "callback" => Err(SamplingError::Corrupted { reason: "callback schedules cannot be deserialized" }),
            _ => Err(SamplingError::Corrupted { reason: "unknown schedule kind" }),
        }
    }
}

async fn schedule_progress(step: StepIndex, over_steps: u32) -> f64 {
    if over_steps == 0 {
        1.0
    } else {
        (step.get() as f64 / over_steps as f64).min(1.0)
    }
}
// #endregion 🔖️Schedules
