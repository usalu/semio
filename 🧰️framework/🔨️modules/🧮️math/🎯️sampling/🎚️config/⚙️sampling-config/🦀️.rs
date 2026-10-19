//! ⚙️ Sampling config: the configuration record, its defaults and its validation, accessors and update methods. See [`super`].

use super::*;

// #region 🔖️Config
/// ⚙️ Everything needed to sample one sequence, independent of any particular model or backend.
#[derive(Clone, PartialEq, Debug)]
pub struct SamplingConfig {
    pub method: SamplingMethod,
    pub processors: Vec<ProcessorSpec>,
    pub error_mode: ErrorMode,
    pub sanitize: SanitizePolicy,
    pub accum: Accum,
    pub seed: u64,
    pub candidate_count: usize,
    pub min_tokens: usize,
    pub max_tokens: usize,
    pub forced: ForcedSpec,
    pub stops: StopSpec,
    pub constraints: Vec<ConstraintSpec>,
    pub logprobs: LogprobsSpec,
    pub limits: SamplingLimits,
    pub diagnostics: DiagnosticsSpec,
}

impl Default for SamplingConfig {
    fn default() -> Self {
        Self {
            method: SamplingMethod::default(),
            processors: Vec::new(),
            error_mode: ErrorMode::default(),
            sanitize: SanitizePolicy::default(),
            accum: Accum::default(),
            seed: 0,
            candidate_count: 1,
            min_tokens: 0,
            max_tokens: 4_096,
            forced: ForcedSpec::default(),
            stops: StopSpec::default(),
            constraints: Vec::new(),
            logprobs: LogprobsSpec::default(),
            limits: SamplingLimits::default(),
            diagnostics: DiagnosticsSpec::default(),
        }
    }
}

impl SamplingConfig {
    /// ⚙️ Full structural validation: resource limits, cross-field consistency, and per-processor
    /// sanity — run before generation starts and again whenever an override merges into a base.
    pub async fn validate(&self) -> Result<(), SamplingError> {
        self.limits.validate().await?;
        if self.candidate_count == 0 {
            return Err(SamplingError::InvalidConfig { field: "candidate_count", reason: "must be >= 1" });
        }
        if self.candidate_count > self.limits.max_candidates {
            return Err(SamplingError::LimitExceeded { limit: "max_candidates" });
        }
        if self.min_tokens > self.max_tokens {
            return Err(SamplingError::InvalidConfig { field: "min_tokens", reason: "must not exceed max_tokens" });
        }
        if self.stops.sequences.len() > self.limits.max_stop_sequences {
            return Err(SamplingError::LimitExceeded { limit: "max_stop_sequences" });
        }
        let stop_bytes: usize = self.stops.sequences.iter().map(Vec::len).sum();
        if stop_bytes > self.limits.max_stop_bytes {
            return Err(SamplingError::LimitExceeded { limit: "max_stop_bytes" });
        }
        if self.forced.at_position.len() > self.limits.max_forced_tokens {
            return Err(SamplingError::LimitExceeded { limit: "max_forced_tokens" });
        }
        if let SamplingMethod::GumbelTopK { k } = self.method {
            if k == 0 {
                return Err(SamplingError::InvalidConfig { field: "method.k", reason: "gumbel top-k requires k >= 1" });
            }
        }
        for processor in &self.processors {
            validate_processor_spec(processor, &self.limits).await?;
        }
        for constraint in &self.constraints {
            if let ConstraintSpec::Ebnf(text) | ConstraintSpec::Regex(text) = constraint {
                if text.len() > self.limits.max_grammar_bytes {
                    return Err(SamplingError::LimitExceeded { limit: "max_grammar_bytes" });
                }
            }
        }
        Ok(())
    }

    /// ⚙️ FNV-1a fingerprint of the canonical JSON form, stamped into serialized sequence state so
    /// a resumed sequence can detect it was checkpointed under a different configuration.
    pub async fn fingerprint(&self) -> u64 {
        let text = write_json(&self.to_json().await).await;
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for &b in text.as_bytes() {
            hash = (hash ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    /// ⚙️ Greedy decoding, no penalties or truncation — the deterministic baseline preset.
    pub async fn precise() -> Self {
        Self { method: SamplingMethod::Greedy { tie_break: TieBreak::LowestTokenId }, ..Self::default() }
    }

    /// ⚙️ Temperature 0.7, top-p 0.9, min-p 0.05, mild repetition penalty — a general-purpose
    /// preset balancing coherence and variety.
    pub async fn balanced() -> Self {
        Self {
            method: SamplingMethod::Multinomial { strategy: MultinomialStrategy::CdfBinarySearch },
            processors: vec![
                ProcessorSpec::Temperature { value: Schedule::Constant(0.7) },
                ProcessorSpec::TopP { p: Schedule::Constant(0.9), min_keep: 1 },
                ProcessorSpec::MinP { p: Schedule::Constant(0.05), min_keep: 1 },
                ProcessorSpec::RepetitionPenalty { penalty: 1.1, scope: PenaltyScope::GeneratedOnly },
            ],
            ..Self::default()
        }
    }

    /// ⚙️ Higher temperature and wider top-p/top-k for more diverse output.
    pub async fn creative() -> Self {
        Self {
            method: SamplingMethod::Multinomial { strategy: MultinomialStrategy::CdfBinarySearch },
            processors: vec![ProcessorSpec::Temperature { value: Schedule::Constant(1.0) }, ProcessorSpec::TopK { k: Schedule::Constant(100.0), min_keep: 1 }, ProcessorSpec::TopP { p: Schedule::Constant(0.95), min_keep: 1 }],
            ..Self::default()
        }
    }

    /// ⚙️ Fixed seed, greedy, tiny token budget — for reproducible tests, not production use.
    pub async fn deterministic_test() -> Self {
        Self { method: SamplingMethod::Greedy { tie_break: TieBreak::LowestTokenId }, seed: 42, max_tokens: 64, ..Self::default() }
    }

    /// ⚙️ Structured JSON form (versioned: top-level `"version": 1`).
    pub async fn to_json(&self) -> JsonValue {
        let mut processors = Vec::with_capacity(self.processors.len());
        for p in &self.processors {
            processors.push(processor_spec_to_json(p).await);
        }
        JsonValue::Object(vec![
            ("version".into(), JsonValue::Num(1.0)),
            ("method".into(), sampling_method_to_json(&self.method).await),
            ("processors".into(), JsonValue::Array(processors)),
            (
                "error_mode".into(),
                JsonValue::Str(match self.error_mode {
                    ErrorMode::Strict => "strict".into(),
                    ErrorMode::Permissive => "permissive".into(),
                }),
            ),
            ("seed".into(), JsonValue::Num(self.seed as f64)),
            ("candidate_count".into(), JsonValue::Num(self.candidate_count as f64)),
            ("min_tokens".into(), JsonValue::Num(self.min_tokens as f64)),
            ("max_tokens".into(), JsonValue::Num(self.max_tokens as f64)),
        ])
    }

    /// ⚙️ Parses the `"version": 1` JSON form back into a config, tolerating unknown top-level
    /// fields (forward compatibility) but rejecting a version it does not recognize.
    pub async fn from_json(value: &JsonValue) -> Result<Self, SamplingError> {
        async fn num(value: &JsonValue, key: &'static str, default: f64) -> f64 {
            match value.get(key).await {
                Some(v) => v.as_f64().await,
                None => None,
            }
            .unwrap_or(default)
        }
        let version = match value.get("version").await {
            Some(v) => v.as_f64().await,
            None => None,
        }
        .ok_or(SamplingError::Corrupted { reason: "config missing version" })?;
        if version as u32 != 1 {
            return Err(SamplingError::SerializationVersion { expected: 1, actual: version as u32 });
        }
        let method = match value.get("method").await {
            Some(v) => sampling_method_from_json(v).await,
            None => Err(SamplingError::Corrupted { reason: "config missing method" }),
        }?;
        let processor_specs = match value.get("processors").await {
            Some(v) => v.as_array().await,
            None => None,
        }
        .ok_or(SamplingError::Corrupted { reason: "config missing processors" })?;
        let mut processors = Vec::with_capacity(processor_specs.len());
        for spec in processor_specs {
            processors.push(processor_spec_from_json(spec).await?);
        }
        let error_mode_str = match value.get("error_mode").await {
            Some(v) => v.as_str().await,
            None => None,
        };
        let error_mode = match error_mode_str {
            Some("strict") => ErrorMode::Strict,
            Some("permissive") | None => ErrorMode::Permissive,
            Some(_) => return Err(SamplingError::Corrupted { reason: "unknown error_mode" }),
        };
        Ok(Self {
            method,
            processors,
            error_mode,
            seed: num(value, "seed", 0.0).await as u64,
            candidate_count: num(value, "candidate_count", 1.0).await as usize,
            min_tokens: num(value, "min_tokens", 0.0).await as usize,
            max_tokens: num(value, "max_tokens", 4_096.0).await as usize,
            ..Self::default()
        })
    }
}
// #endregion 🔖️Config
