//! ⚠️ Errors, fallbacks and limits of sampling: the sampling error, the finish reason, the error mode, the fallback action and its resolution and the sampling limits. See [`super`].

use super::*;

// #region 🔖️Errors
/// 🚨️ Every way a sampling step can fail to produce a token. Kept flat (no nested error types)
/// so callers can match exhaustively without chasing a `source()` chain for common cases.
#[derive(Clone, PartialEq, Debug)]
pub enum SamplingError {
    /// 🚨️ A configuration value failed validation (`field` names the offending knob).
    InvalidConfig { field: &'static str, reason: &'static str },
    /// 🚨️ Logits length does not match the configured vocabulary size.
    VocabMismatch { expected: usize, actual: usize },
    /// 🚨️ Logits contained NaN/Inf and the active [`SanitizePolicy`] is [`SanitizePolicy::Error`].
    NonFiniteLogits { index: usize },
    /// 🚨️ Every token was masked or truncated away with no fallback available.
    EmptyDistribution,
    /// 🚨️ A constraint reports no valid continuation exists (dead automaton state).
    ConstraintDead { constraint: &'static str },
    /// 🚨️ A configured resource cap (§ [`SamplingLimits`]) was exceeded.
    LimitExceeded { limit: &'static str },
    /// 🚨️ EBNF grammar text failed to parse at the given byte offset.
    GrammarParse { offset: usize, reason: &'static str },
    /// 🚨️ Regex pattern text failed to parse at the given byte offset.
    RegexParse { offset: usize, reason: &'static str },
    /// 🚨️ An automaton (DFA/NFA/Earley chart) exceeded its state/size budget mid-construction.
    AutomatonBudget { budget: &'static str },
    /// 🚨️ Serialized config/state carries an unsupported or mismatched format version.
    SerializationVersion { expected: u32, actual: u32 },
    /// 🚨️ A config/state fingerprint did not match the fingerprint recorded at serialization time.
    FingerprintMismatch,
    /// 🚨️ Serialized data is truncated, malformed, or fails an integrity check.
    Corrupted { reason: &'static str },
    /// 🚨️ A sharded-vocabulary collective operation failed (timeout, rank mismatch, ...).
    Collective { reason: &'static str },
    /// 🚨️ A user-supplied callback (rerank, similarity, denoiser, ...) reported failure.
    Callback { reason: &'static str },
    /// 🚨️ Generation was cancelled via an external cancellation signal.
    Cancelled,
}

impl core::fmt::Display for SamplingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidConfig { field, reason } => write!(f, "invalid config field `{field}`: {reason}"),
            Self::VocabMismatch { expected, actual } => {
                write!(f, "logits length {actual} does not match vocabulary size {expected}")
            }
            Self::NonFiniteLogits { index } => write!(f, "non-finite logit at index {index}"),
            Self::EmptyDistribution => write!(f, "no valid token remains after masking/truncation"),
            Self::ConstraintDead { constraint } => write!(f, "constraint `{constraint}` has no valid continuation"),
            Self::LimitExceeded { limit } => write!(f, "resource limit exceeded: {limit}"),
            Self::GrammarParse { offset, reason } => write!(f, "grammar parse error at byte {offset}: {reason}"),
            Self::RegexParse { offset, reason } => write!(f, "regex parse error at byte {offset}: {reason}"),
            Self::AutomatonBudget { budget } => write!(f, "automaton exceeded budget: {budget}"),
            Self::SerializationVersion { expected, actual } => {
                write!(f, "serialization version mismatch: expected {expected}, found {actual}")
            }
            Self::FingerprintMismatch => write!(f, "config/state fingerprint mismatch"),
            Self::Corrupted { reason } => write!(f, "corrupted serialized data: {reason}"),
            Self::Collective { reason } => write!(f, "collective operation failed: {reason}"),
            Self::Callback { reason } => write!(f, "callback failed: {reason}"),
            Self::Cancelled => write!(f, "generation cancelled"),
        }
    }
}

impl std::error::Error for SamplingError {}

/// 🚨️ Why a sequence stopped generating.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FinishReason {
    /// 🚨️ An end-of-sequence token was selected.
    EosToken,
    /// 🚨️ A single stop token (outside the EOS set) was selected.
    StopToken,
    /// 🚨️ A configured stop text sequence matched, by index into `StopSpec::sequences`.
    StopSequence { index: usize },
    /// 🚨️ The per-sequence maximum generated-token count was reached.
    MaxTokens,
    /// 🚨️ The maximum wall-clock duration was reached.
    MaxTimeMs,
    /// 🚨️ A constraint (grammar/JSON/schema) reports completion.
    ConstraintComplete,
    /// 🚨️ An external cancellation signal was observed.
    Cancelled,
    /// 🚨️ No valid token existed and no fallback resolved (only reachable in permissive mode).
    Dead,
    /// 🚨️ A user-supplied stop callback returned `true`.
    Callback,
}

/// 🚨️ Whether a failed step surfaces an error or resolves through the fallback ladder.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ErrorMode {
    /// 🚨️ Any [`SamplingError`] is returned to the caller.
    Strict,
    /// 🚨️ Recoverable failures resolve via [`resolve_fallback`] instead of erroring.
    #[default]
    Permissive,
}

/// 🚨️ Which rung of the fallback ladder resolved an otherwise-empty distribution.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FallbackAction {
    /// 🚨️ A configured forced token was substituted.
    ForcedToken,
    /// 🚨️ The vocabulary's (first) EOS token was substituted.
    Eos,
    /// 🚨️ The pre-mask, pre-truncation argmax of the raw logits was substituted.
    ArgmaxRaw,
    /// 🚨️ No rung resolved; the caller's [`ErrorMode`] decides between `Dead` and an error.
    Error,
}

/// 🚨️ Walks the fallback ladder `ForcedToken -> Eos -> ArgmaxRaw -> Error` for an empty
/// distribution, in that fixed priority order — forced tokens are an explicit user instruction so
/// they always win, EOS is the next-safest "stop cleanly" choice, and only then does raw argmax
/// (which may violate active masks) get used as a last resort before giving up.
pub async fn resolve_fallback(forced: Option<TokenId>, eos: Option<TokenId>, argmax_raw: Option<TokenId>) -> (FallbackAction, Option<TokenId>) {
    if let Some(token) = forced {
        return (FallbackAction::ForcedToken, Some(token));
    }
    if let Some(token) = eos {
        return (FallbackAction::Eos, Some(token));
    }
    if let Some(token) = argmax_raw {
        return (FallbackAction::ArgmaxRaw, Some(token));
    }
    (FallbackAction::Error, None)
}
// #endregion 🔖️Errors

// #region 🔖️Limits
/// 🛡️ Resource caps applied to untrusted configuration, so a hostile or malformed request cannot
/// force unbounded allocation or CPU time (bounded grammars, bounded automata, bounded beams).
#[derive(Clone, PartialEq, Debug)]
pub struct SamplingLimits {
    pub max_stop_sequences: usize,
    pub max_stop_bytes: usize,
    pub max_grammar_bytes: usize,
    pub max_automaton_states: usize,
    pub max_dfa_cache_entries: usize,
    pub max_beam_width: usize,
    pub max_candidates: usize,
    pub max_ngram_order: usize,
    pub max_forced_tokens: usize,
    pub max_schedule_pieces: usize,
    pub max_json_depth: usize,
}

impl Default for SamplingLimits {
    fn default() -> Self {
        Self {
            max_stop_sequences: 64,
            max_stop_bytes: 4_096,
            max_grammar_bytes: 65_536,
            max_automaton_states: 100_000,
            max_dfa_cache_entries: 16_384,
            max_beam_width: 64,
            max_candidates: 256,
            max_ngram_order: 8,
            max_forced_tokens: 4_096,
            max_schedule_pieces: 256,
            max_json_depth: 64,
        }
    }
}

impl SamplingLimits {
    /// 🛡️ Rejects a manifestly broken limits set (zero caps would make the engine unusable).
    pub async fn validate(&self) -> Result<(), SamplingError> {
        let fields: [(&'static str, usize); 11] = [
            ("max_stop_sequences", self.max_stop_sequences),
            ("max_stop_bytes", self.max_stop_bytes),
            ("max_grammar_bytes", self.max_grammar_bytes),
            ("max_automaton_states", self.max_automaton_states),
            ("max_dfa_cache_entries", self.max_dfa_cache_entries),
            ("max_beam_width", self.max_beam_width),
            ("max_candidates", self.max_candidates),
            ("max_ngram_order", self.max_ngram_order),
            ("max_forced_tokens", self.max_forced_tokens),
            ("max_schedule_pieces", self.max_schedule_pieces),
            ("max_json_depth", self.max_json_depth),
        ];
        for (field, value) in fields {
            if value == 0 {
                return Err(SamplingError::InvalidConfig { field, reason: "limit must be non-zero" });
            }
        }
        Ok(())
    }
}
// #endregion 🔖️Limits
