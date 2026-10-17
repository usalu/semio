//! 🎛️ Sampling method and spec vocabulary: the tie break, the multinomial strategy, the sampling method, the penalty scope, the processor and constraint specs and the stop, forced, logprobs and diagnostics specs. See [`super`].

use super::*;

// #region 🔖️Config
/// ⚙️ Deterministic tie-breaking policy for greedy selection and every truncation warper's
/// boundary decision.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TieBreak {
    #[default]
    LowestTokenId,
    HighestTokenId,
    FirstSeen,
}

/// ⚙️ Which concrete algorithm backs [`SamplingMethod::Multinomial`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum MultinomialStrategy {
    #[default]
    CdfBinarySearch,
    LinearScan,
    Alias,
}

/// ⚙️ How a token is finally chosen from the (already-warped) live distribution.
#[derive(Clone, PartialEq, Debug)]
pub enum SamplingMethod {
    Greedy { tie_break: TieBreak },
    Multinomial { strategy: MultinomialStrategy },
    GumbelMax,
    GumbelTopK { k: usize },
}

impl Default for SamplingMethod {
    fn default() -> Self {
        Self::Greedy { tie_break: TieBreak::default() }
    }
}

/// ⚙️ Whether a penalty/bias considers the prompt, the generated continuation, or both, when
/// scanning a sequence's history.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PenaltyScope {
    #[default]
    PromptAndGenerated,
    GeneratedOnly,
    PromptOnly,
}

/// ⚙️ Which Mirostat control law an adaptive processor runs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MirostatVersion {
    V1,
    V2,
}

/// ⚙️ One entry in the ordered processor pipeline. Every schedulable numeric parameter is
/// [`Schedule`]-typed so config presets and per-step dynamics share one representation.
#[derive(Clone, PartialEq, Debug)]
pub enum ProcessorSpec {
    Temperature {
        value: Schedule,
    },
    DynamicTemperature {
        base: Schedule,
        entropy_gain: f64,
        min: f64,
        max: f64,
    },
    TopK {
        k: Schedule,
        min_keep: usize,
    },
    TopP {
        p: Schedule,
        min_keep: usize,
    },
    MinP {
        p: Schedule,
        min_keep: usize,
    },
    Typical {
        mass: Schedule,
        min_keep: usize,
    },
    LocallyTypical {
        mass: Schedule,
        min_keep: usize,
    },
    TailFree {
        z: Schedule,
        min_keep: usize,
    },
    Epsilon {
        cutoff: Schedule,
        min_keep: usize,
    },
    Eta {
        cutoff: Schedule,
        min_keep: usize,
    },
    TopA {
        power: Schedule,
        min_keep: usize,
    },
    RankTruncation {
        max_rank: usize,
    },
    AdaptiveTruncation {
        target_entropy: Option<f64>,
        target_effective_count: Option<f64>,
    },
    RepetitionPenalty {
        penalty: f32,
        scope: PenaltyScope,
    },
    PresencePenalty {
        penalty: f32,
        scope: PenaltyScope,
    },
    FrequencyPenalty {
        penalty: f32,
        scope: PenaltyScope,
    },
    DecayingPenalty {
        penalty: f32,
        window: usize,
        half_life: f64,
        scope: PenaltyScope,
    },
    /// ⚙️ `class_tokens[c]` lists the token ids in class `c`; `factors[c]` is that class's
    /// multiplicative penalty factor (same lengths, index-aligned).
    TokenClassPenalty {
        class_tokens: Vec<Vec<TokenId>>,
        factors: Vec<f32>,
    },
    NoRepeatNgram {
        n: usize,
    },
    PhrasePenalty {
        phrases: Vec<Vec<TokenId>>,
        penalty: f32,
    },
    LogitBiasSparse {
        entries: Vec<(TokenId, f32)>,
    },
    LogitBiasDense {
        values: Vec<f32>,
    },
    AllowTokens {
        tokens: Vec<TokenId>,
    },
    ForbidTokens {
        tokens: Vec<TokenId>,
    },
    SuppressSpecial,
    BadWords {
        phrases: Vec<Vec<TokenId>>,
    },
    SequenceEncouragement {
        phrases: Vec<Vec<TokenId>>,
        bonus: f32,
    },
    Mirostat {
        version: MirostatVersion,
        target_surprise: f64,
        learning_rate: f64,
    },
    EntropyPid {
        target: f64,
        kp: f64,
        ki: f64,
        kd: f64,
    },
    RepetitionController {
        window: usize,
        threshold: f64,
        boost: f64,
    },
    ConfidenceController {
        low_entropy: f64,
        high_entropy: f64,
        low_temp: f64,
        high_temp: f64,
    },
}

/// ⚙️ One hand-rolled-constraint source. Compiled into a [`Constraint`] impl by the engine.
#[derive(Clone, PartialEq, Debug)]
pub enum ConstraintSpec {
    Regex(String),
    Trie(Vec<Vec<TokenId>>),
    MustInclude(Vec<Vec<TokenId>>),
    JsonMode,
    Ebnf(String),
    JsonSchema(JsonValue),
}

/// ⚙️ How matched stop text is reflected in the returned generation.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum StopTextMode {
    #[default]
    Include,
    Exclude,
    Separate,
}

/// ⚙️ Stop-condition configuration: token-level, text-sequence, and time-based stops.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct StopSpec {
    pub tokens: Vec<TokenId>,
    pub sequences: Vec<Vec<u8>>,
    pub mode: StopTextMode,
    pub max_time_ms: Option<u64>,
}

/// ⚙️ Forced-token configuration: an initial BOS/prefix and/or exact tokens at fixed positions.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ForcedSpec {
    pub bos: Option<TokenId>,
    pub prefix: Vec<TokenId>,
    pub at_position: Vec<(StepIndex, TokenId)>,
}

/// ⚙️ How many alternative log-probabilities to report alongside the selected token.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct LogprobsSpec {
    pub top_n: usize,
    pub include_pre_truncation: bool,
}

/// ⚙️ Which optional per-step diagnostics to collect (all zero-cost when disabled).
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct DiagnosticsSpec {
    pub enabled: bool,
    pub timing: bool,
}
// #endregion 🔖️Config
