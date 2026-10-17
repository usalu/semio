//! 🏗️ Sampling config builder: the builder that assembles and validates a sampling config. See [`super`].

use super::*;

// #region 🔖️Config
/// ⚙️ Chainable constructor for [`SamplingConfig`]; `build()` runs full validation once.
#[derive(Clone, Debug)]
#[derive(Default)]
pub struct SamplingConfigBuilder {
    pub(super) config: SamplingConfig,
}

impl SamplingConfigBuilder {
    pub async fn new() -> Self {
        Self { config: SamplingConfig::default() }
    }

    pub async fn method(mut self, method: SamplingMethod) -> Self {
        self.config.method = method;
        self
    }

    pub async fn processor(mut self, processor: ProcessorSpec) -> Self {
        self.config.processors.push(processor);
        self
    }

    pub async fn error_mode(mut self, mode: ErrorMode) -> Self {
        self.config.error_mode = mode;
        self
    }

    pub async fn sanitize(mut self, policy: SanitizePolicy) -> Self {
        self.config.sanitize = policy;
        self
    }

    pub async fn accum(mut self, accum: Accum) -> Self {
        self.config.accum = accum;
        self
    }

    pub async fn seed(mut self, seed: u64) -> Self {
        self.config.seed = seed;
        self
    }

    pub async fn candidate_count(mut self, count: usize) -> Self {
        self.config.candidate_count = count;
        self
    }

    pub async fn min_tokens(mut self, min_tokens: usize) -> Self {
        self.config.min_tokens = min_tokens;
        self
    }

    pub async fn max_tokens(mut self, max_tokens: usize) -> Self {
        self.config.max_tokens = max_tokens;
        self
    }

    pub async fn forced(mut self, forced: ForcedSpec) -> Self {
        self.config.forced = forced;
        self
    }

    pub async fn stops(mut self, stops: StopSpec) -> Self {
        self.config.stops = stops;
        self
    }

    pub async fn constraint(mut self, constraint: ConstraintSpec) -> Self {
        self.config.constraints.push(constraint);
        self
    }

    pub async fn logprobs(mut self, logprobs: LogprobsSpec) -> Self {
        self.config.logprobs = logprobs;
        self
    }

    pub async fn limits(mut self, limits: SamplingLimits) -> Self {
        self.config.limits = limits;
        self
    }

    pub async fn diagnostics(mut self, diagnostics: DiagnosticsSpec) -> Self {
        self.config.diagnostics = diagnostics;
        self
    }

    /// ⚙️ Validates and returns the finished config.
    pub async fn build(self) -> Result<SamplingConfig, SamplingError> {
        self.config.validate().await?;
        Ok(self.config)
    }
}
// #endregion 🔖️Config
