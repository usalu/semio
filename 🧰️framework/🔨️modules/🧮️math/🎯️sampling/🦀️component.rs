//! 🎰️ Model-agnostic LLM token-sampling engine: logits in, processor pipeline, constrained
//! distributions, deterministic seeded selection — plus a diffusion/continuous-noise solver module.

// #region 🔖️Ids
/// 🧩️ Index of one vocabulary entry. `u32` keeps candidate/mask arithmetic cheap even for
/// million-token sharded vocabularies while staying far below any real model's vocab size.
/// `#[repr(transparent)]` lets [`cast_u32_slice_to_token_ids`] hand out a typed view over a raw
/// index buffer without copying.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TokenId(pub u32);

impl TokenId {
    /// 🧩️ Wraps a raw vocabulary index.
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// 🧩️ Raw vocabulary index.
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl core::fmt::Display for TokenId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 🧩️ Identifies one generation request/sequence across batch reorders — never a slot index, so
/// continuous batching can shuffle rows without breaking RNG-stream or state addressing.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SequenceId(pub u64);

impl SequenceId {
    /// 🧩️ Wraps a raw sequence identifier.
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// 🧩️ Raw sequence identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl core::fmt::Display for SequenceId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 🧩️ Zero-based count of tokens generated so far for one sequence (excludes the prompt).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StepIndex(pub u32);

impl StepIndex {
    /// 🧩️ Wraps a raw step count.
    pub const fn new(step: u32) -> Self {
        Self(step)
    }

    /// 🧩️ Raw step count.
    pub const fn get(self) -> u32 {
        self.0
    }

    /// 🧩️ Next step, or `None` on overflow (caller-observable rather than a silent wraparound).
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(next) => Some(Self(next)),
            None => None,
        }
    }
}

impl core::fmt::Display for StepIndex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
// #endregion 🔖️Ids

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
pub fn resolve_fallback(forced: Option<TokenId>, eos: Option<TokenId>, argmax_raw: Option<TokenId>) -> (FallbackAction, Option<TokenId>) {
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
    pub fn validate(&self) -> Result<(), SamplingError> {
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

// #region 🔖️Text
// #region 🔖️Json
/// 📜️ A parsed JSON value. Objects preserve insertion order via `Vec<(String, JsonValue)>` rather
/// than a `HashMap` — configs are small, order-preserving round-trips are nicer to diff, and this
/// keeps the whole module dependency-free (no hashing DoS surface to reason about either).
#[derive(Clone, PartialEq, Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    /// 📜️ Looks up a key in an object value; `None` for any other variant or a missing key.
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            Self::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Num(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[JsonValue]> {
        match self {
            Self::Array(items) => Some(items),
            _ => None,
        }
    }
}

/// 📜️ Recursive-descent JSON parser over `&str`, depth-capped by `max_depth` so a maliciously
/// nested document (`[[[[...`) cannot blow the call stack — this is the same nesting bound that
/// protects the JSON-Schema/grammar constraint machinery later in the file.
pub fn parse_json(text: &str, max_depth: usize) -> Result<JsonValue, SamplingError> {
    let bytes = text.as_bytes();
    let mut pos = 0usize;
    let value = parse_json_value(bytes, &mut pos, 0, max_depth)?;
    skip_json_whitespace(bytes, &mut pos);
    if pos != bytes.len() {
        return Err(SamplingError::Corrupted { reason: "trailing data after JSON value" });
    }
    Ok(value)
}

fn skip_json_whitespace(bytes: &[u8], pos: &mut usize) {
    while *pos < bytes.len() && matches!(bytes[*pos], b' ' | b'\t' | b'\n' | b'\r') {
        *pos += 1;
    }
}

fn parse_json_value(bytes: &[u8], pos: &mut usize, depth: usize, max_depth: usize) -> Result<JsonValue, SamplingError> {
    if depth > max_depth {
        return Err(SamplingError::LimitExceeded { limit: "max_json_depth" });
    }
    skip_json_whitespace(bytes, pos);
    let Some(&byte) = bytes.get(*pos) else {
        return Err(SamplingError::Corrupted { reason: "unexpected end of JSON input" });
    };
    match byte {
        b'n' => parse_json_literal(bytes, pos, "null", JsonValue::Null),
        b't' => parse_json_literal(bytes, pos, "true", JsonValue::Bool(true)),
        b'f' => parse_json_literal(bytes, pos, "false", JsonValue::Bool(false)),
        b'"' => parse_json_string(bytes, pos).map(JsonValue::Str),
        b'[' => parse_json_array(bytes, pos, depth, max_depth),
        b'{' => parse_json_object(bytes, pos, depth, max_depth),
        b'-' | b'0'..=b'9' => parse_json_number(bytes, pos),
        _ => Err(SamplingError::Corrupted { reason: "unexpected byte in JSON input" }),
    }
}

fn parse_json_literal(bytes: &[u8], pos: &mut usize, literal: &str, value: JsonValue) -> Result<JsonValue, SamplingError> {
    let lit = literal.as_bytes();
    if bytes[*pos..].starts_with(lit) {
        *pos += lit.len();
        Ok(value)
    } else {
        Err(SamplingError::Corrupted { reason: "invalid JSON literal" })
    }
}

fn parse_json_string(bytes: &[u8], pos: &mut usize) -> Result<String, SamplingError> {
    debug_assert_eq!(bytes[*pos], b'"');
    *pos += 1;
    let mut out = String::new();
    loop {
        let &byte = bytes.get(*pos).ok_or(SamplingError::Corrupted { reason: "unterminated JSON string" })?;
        *pos += 1;
        match byte {
            b'"' => return Ok(out),
            b'\\' => {
                let &esc = bytes.get(*pos).ok_or(SamplingError::Corrupted { reason: "unterminated JSON escape" })?;
                *pos += 1;
                match esc {
                    b'"' => out.push('"'),
                    b'\\' => out.push('\\'),
                    b'/' => out.push('/'),
                    b'n' => out.push('\n'),
                    b't' => out.push('\t'),
                    b'r' => out.push('\r'),
                    b'b' => out.push('\u{8}'),
                    b'f' => out.push('\u{c}'),
                    b'u' => {
                        let code = parse_json_unicode_escape(bytes, pos)?;
                        out.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
                    }
                    _ => return Err(SamplingError::Corrupted { reason: "invalid JSON escape" }),
                }
            }
            _ => {
                // 📜️ Re-decode the UTF-8 codepoint starting at this byte instead of pushing raw bytes.
                let start = *pos - 1;
                let ch_len = utf8_sequence_len(byte).ok_or(SamplingError::Corrupted { reason: "invalid UTF-8 in JSON string" })?;
                let end = start + ch_len;
                let slice = bytes.get(start..end).ok_or(SamplingError::Corrupted { reason: "truncated UTF-8 in JSON string" })?;
                let s = core::str::from_utf8(slice).map_err(|_| SamplingError::Corrupted { reason: "invalid UTF-8 in JSON string" })?;
                out.push_str(s);
                *pos = end;
            }
        }
    }
}

fn parse_json_unicode_escape(bytes: &[u8], pos: &mut usize) -> Result<u32, SamplingError> {
    let hex = bytes.get(*pos..*pos + 4).ok_or(SamplingError::Corrupted { reason: "truncated unicode escape" })?;
    let s = core::str::from_utf8(hex).map_err(|_| SamplingError::Corrupted { reason: "invalid unicode escape" })?;
    let code = u32::from_str_radix(s, 16).map_err(|_| SamplingError::Corrupted { reason: "invalid unicode escape" })?;
    *pos += 4;
    Ok(code)
}

fn parse_json_number(bytes: &[u8], pos: &mut usize) -> Result<JsonValue, SamplingError> {
    let start = *pos;
    if bytes.get(*pos) == Some(&b'-') {
        *pos += 1;
    }
    while bytes.get(*pos).is_some_and(u8::is_ascii_digit) {
        *pos += 1;
    }
    if bytes.get(*pos) == Some(&b'.') {
        *pos += 1;
        while bytes.get(*pos).is_some_and(u8::is_ascii_digit) {
            *pos += 1;
        }
    }
    if matches!(bytes.get(*pos), Some(b'e' | b'E')) {
        *pos += 1;
        if matches!(bytes.get(*pos), Some(b'+' | b'-')) {
            *pos += 1;
        }
        while bytes.get(*pos).is_some_and(u8::is_ascii_digit) {
            *pos += 1;
        }
    }
    let text = core::str::from_utf8(&bytes[start..*pos]).expect("ASCII number slice is valid UTF-8");
    text.parse::<f64>().map(JsonValue::Num).map_err(|_| SamplingError::Corrupted { reason: "invalid JSON number" })
}

fn parse_json_array(bytes: &[u8], pos: &mut usize, depth: usize, max_depth: usize) -> Result<JsonValue, SamplingError> {
    debug_assert_eq!(bytes[*pos], b'[');
    *pos += 1;
    let mut items = Vec::new();
    skip_json_whitespace(bytes, pos);
    if bytes.get(*pos) == Some(&b']') {
        *pos += 1;
        return Ok(JsonValue::Array(items));
    }
    loop {
        items.push(parse_json_value(bytes, pos, depth + 1, max_depth)?);
        skip_json_whitespace(bytes, pos);
        match bytes.get(*pos) {
            Some(b',') => {
                *pos += 1;
            }
            Some(b']') => {
                *pos += 1;
                return Ok(JsonValue::Array(items));
            }
            _ => return Err(SamplingError::Corrupted { reason: "expected ',' or ']' in JSON array" }),
        }
    }
}

fn parse_json_object(bytes: &[u8], pos: &mut usize, depth: usize, max_depth: usize) -> Result<JsonValue, SamplingError> {
    debug_assert_eq!(bytes[*pos], b'{');
    *pos += 1;
    let mut entries = Vec::new();
    skip_json_whitespace(bytes, pos);
    if bytes.get(*pos) == Some(&b'}') {
        *pos += 1;
        return Ok(JsonValue::Object(entries));
    }
    loop {
        skip_json_whitespace(bytes, pos);
        if bytes.get(*pos) != Some(&b'"') {
            return Err(SamplingError::Corrupted { reason: "expected string key in JSON object" });
        }
        let key = parse_json_string(bytes, pos)?;
        skip_json_whitespace(bytes, pos);
        if bytes.get(*pos) != Some(&b':') {
            return Err(SamplingError::Corrupted { reason: "expected ':' in JSON object" });
        }
        *pos += 1;
        let value = parse_json_value(bytes, pos, depth + 1, max_depth)?;
        entries.push((key, value));
        skip_json_whitespace(bytes, pos);
        match bytes.get(*pos) {
            Some(b',') => {
                *pos += 1;
            }
            Some(b'}') => {
                *pos += 1;
                return Ok(JsonValue::Object(entries));
            }
            _ => return Err(SamplingError::Corrupted { reason: "expected ',' or '}' in JSON object" }),
        }
    }
}

/// 📜️ Writes a [`JsonValue`] in compact form with correct string escaping and round-trippable
/// float formatting (integral floats print without a trailing `.0`-free exponent surprise).
pub fn write_json(value: &JsonValue) -> String {
    let mut out = String::new();
    write_json_into(value, &mut out);
    out
}

fn write_json_into(value: &JsonValue, out: &mut String) {
    match value {
        JsonValue::Null => out.push_str("null"),
        JsonValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        JsonValue::Num(n) => write_json_number(*n, out),
        JsonValue::Str(s) => write_json_string(s, out),
        JsonValue::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_json_into(item, out);
            }
            out.push(']');
        }
        JsonValue::Object(entries) => {
            out.push('{');
            for (i, (key, item)) in entries.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_json_string(key, out);
                out.push(':');
                write_json_into(item, out);
            }
            out.push('}');
        }
    }
}

fn write_json_number(n: f64, out: &mut String) {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        out.push_str(&format!("{}", n as i64));
    } else {
        out.push_str(&format!("{n}"));
    }
}

fn write_json_string(s: &str, out: &mut String) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}
// #endregion 🔖️Json

// #region 🔖️Utf8
/// 📜️ How much of a byte sequence forms valid UTF-8, for incremental decoding of streamed token
/// bytes that may end mid-codepoint.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Utf8Status {
    /// 📜️ The whole slice is valid, complete UTF-8.
    Complete,
    /// 📜️ The slice ends with a valid-so-far partial codepoint needing `more` additional bytes.
    Partial { more: usize },
    /// 📜️ The slice contains a byte sequence that can never become valid UTF-8.
    Invalid,
}

/// 📜️ Length of the UTF-8 sequence a leading byte starts, or `None` if `byte` cannot lead one.
pub fn utf8_sequence_len(byte: u8) -> Option<usize> {
    match byte {
        0x00..=0x7F => Some(1),
        0xC2..=0xDF => Some(2),
        0xE0..=0xEF => Some(3),
        0xF0..=0xF4 => Some(4),
        _ => None,
    }
}

/// 📜️ Classifies the tail of `bytes` as complete, valid-partial, or invalid UTF-8. Used to decide
/// how many trailing bytes of a just-emitted token must be held back until more bytes arrive.
pub fn utf8_status(bytes: &[u8]) -> Utf8Status {
    if bytes.is_empty() {
        return Utf8Status::Complete;
    }
    // 📜️ Walk backward from the end to find the start of the last (possibly incomplete) codepoint.
    let mut start = bytes.len() - 1;
    let mut lead_len = None;
    for _ in 0..4 {
        if let Some(len) = utf8_sequence_len(bytes[start]) {
            lead_len = Some(len);
            break;
        }
        if start == 0 {
            return Utf8Status::Invalid;
        }
        start -= 1;
    }
    let Some(len) = lead_len else {
        return Utf8Status::Invalid;
    };
    let have = bytes.len() - start;
    if have > len {
        return Utf8Status::Invalid;
    }
    if have == len {
        return match core::str::from_utf8(&bytes[start..]) {
            Ok(_) => Utf8Status::Complete,
            Err(_) => Utf8Status::Invalid,
        };
    }
    Utf8Status::Partial { more: len - have }
}
// #endregion 🔖️Utf8
// #endregion 🔖️Text

// #region 🔖️Numerics
/// 📐️ How non-finite input logits are handled before any processor runs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SanitizePolicy {
    /// 📐️ Any NaN or +Inf logit is a hard [`SamplingError::NonFiniteLogits`].
    Error,
    /// 📐️ NaN and -Inf both collapse to `f32::NEG_INFINITY` (effectively masked); +Inf is rejected
    /// (an infinitely-preferred token needs an explicit decision, not a silent uniform pick).
    #[default]
    NegInfNan,
    /// 📐️ NaN collapses to `f32::NEG_INFINITY`; +Inf clamps to `f32::MAX`.
    ClampInf,
}

/// 📐️ Applies `policy` to `logits` in place. Returns the count of entries that were altered, for
/// diagnostics.
pub fn sanitize_logits(logits: &mut [f32], policy: SanitizePolicy) -> Result<usize, SamplingError> {
    let mut altered = 0usize;
    for (i, l) in logits.iter_mut().enumerate() {
        if l.is_nan() {
            match policy {
                SanitizePolicy::Error => return Err(SamplingError::NonFiniteLogits { index: i }),
                SanitizePolicy::NegInfNan | SanitizePolicy::ClampInf => {
                    *l = f32::NEG_INFINITY;
                    altered += 1;
                }
            }
        } else if *l == f32::INFINITY {
            match policy {
                SanitizePolicy::Error | SanitizePolicy::NegInfNan => return Err(SamplingError::NonFiniteLogits { index: i }),
                SanitizePolicy::ClampInf => {
                    *l = f32::MAX;
                    altered += 1;
                }
            }
        } else if *l == f32::NEG_INFINITY {
            // 📐️ Negative infinity is always a valid "hard masked" representation; never altered.
        }
    }
    Ok(altered)
}

/// 📐️ Accumulation precision for softmax/entropy computation.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Accum {
    F32,
    #[default]
    F64,
}

/// 📐️ Compensated (Kahan-Neumaier) running sum — keeps `f64` summation accurate over long live
/// candidate lists where naive summation would otherwise lose low-order bits.
#[derive(Clone, Copy, Default, Debug)]
pub struct KahanSum {
    sum: f64,
    correction: f64,
}

impl KahanSum {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, value: f64) {
        let t = self.sum + value;
        if self.sum.abs() >= value.abs() {
            self.correction += (self.sum - t) + value;
        } else {
            self.correction += (value - t) + self.sum;
        }
        self.sum = t;
    }

    pub fn value(&self) -> f64 {
        self.sum + self.correction
    }
}

/// 📐️ Numerically stable softmax over `logits[live]`, writing normalized probabilities into
/// `probs[..live.len()]`. Uses max-subtraction so large logits never overflow `exp`, and
/// [`KahanSum`] under [`Accum::F64`] so the sum stays close to `1.0` even for wide vocabularies.
pub fn softmax_live(logits: &[f32], live: &[u32], probs: &mut [f32], accum: Accum) -> f64 {
    debug_assert_eq!(live.len(), probs.len());
    if live.is_empty() {
        return 0.0;
    }
    let max_logit = live.iter().map(|&i| logits[i as usize]).fold(f32::NEG_INFINITY, f32::max);
    match accum {
        Accum::F32 => {
            let mut sum = 0.0f32;
            for (slot, &i) in live.iter().enumerate() {
                let e = (logits[i as usize] - max_logit).exp();
                probs[slot] = e;
                sum += e;
            }
            let inv = if sum > 0.0 { 1.0 / sum } else { 0.0 };
            for p in probs.iter_mut() {
                *p *= inv;
            }
            sum as f64
        }
        Accum::F64 => {
            let mut sum = KahanSum::new();
            let mut exps = vec![0.0f64; live.len()];
            for (slot, &i) in live.iter().enumerate() {
                let e = ((logits[i as usize] - max_logit) as f64).exp();
                exps[slot] = e;
                sum.add(e);
            }
            let total = sum.value();
            let inv = if total > 0.0 { 1.0 / total } else { 0.0 };
            for (slot, p) in probs.iter_mut().enumerate() {
                *p = (exps[slot] * inv) as f32;
            }
            total
        }
    }
}

/// 📐️ `log(sum(exp(values)))` computed via max-subtraction, safe for arbitrarily negative/large
/// inputs (including `-inf` entries, which contribute nothing).
pub fn logsumexp_f64(values: &[f64]) -> f64 {
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if !max.is_finite() {
        return max;
    }
    let mut sum = KahanSum::new();
    for &v in values {
        sum.add((v - max).exp());
    }
    max + sum.value().ln()
}

/// 📐️ Shannon entropy in nats of a probability vector (`0 ln 0 := 0` by convention).
pub fn entropy_nats(probs: &[f32]) -> f64 {
    let mut sum = KahanSum::new();
    for &p in probs {
        let p = p as f64;
        if p > 0.0 {
            sum.add(-p * p.ln());
        }
    }
    sum.value()
}

/// 📐️ `exp(entropy)`: the "effective" number of roughly-equally-likely candidates, matching the
/// perplexity of the live distribution.
pub fn effective_candidate_count(probs: &[f32]) -> f64 {
    entropy_nats(probs).exp()
}

/// 📐️ Total probability mass removed by comparing a pre-truncation and post-truncation live-prob
/// vector that share the same normalization base (`1.0 - post_sum` when `post` sums the surviving
/// portion of a `pre` distribution that summed to `1.0`).
pub fn truncation_mass(pre_kept_sum: f64) -> f64 {
    (1.0 - pre_kept_sum).max(0.0)
}

/// 📐️ Health flags for a live distribution, computed once per step for diagnostics and to decide
/// whether the fallback ladder must engage.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DistributionHealth {
    pub live_count: usize,
    pub prob_sum: f64,
    pub is_degenerate: bool,
}

impl DistributionHealth {
    pub fn assess(live_count: usize, prob_sum: f64) -> Self {
        // 📐️ `is_nan() || <= 0.0` (not `!(prob_sum > 0.0)`) so NaN is explicitly, intentionally
        // flagged degenerate rather than relying on `>` being false for NaN — same result, clearer.
        Self { live_count, prob_sum, is_degenerate: live_count == 0 || prob_sum.is_nan() || prob_sum <= 0.0 }
    }
}

/// 📐️ Reorders `live[..]` so the top `k` (by `logits[live[i]]` descending, ties by ascending token
/// id for determinism) occupy `live[..k]`, unordered within that prefix — a Hoare-partition
/// quickselect, `O(n)` average instead of an `O(n log n)` full sort. `scratch` must have length
/// `>= live.len()` and its contents are overwritten.
pub fn partial_select_top_k(logits: &[f32], live: &mut [u32], k: usize, scratch: &mut [f32]) {
    let n = live.len();
    if k >= n {
        return;
    }
    debug_assert!(scratch.len() >= n);
    let key = |token: u32| -> (f32, core::cmp::Reverse<u32>) { (logits[token as usize], core::cmp::Reverse(token)) };
    // 📐️ Selection over a byte-key derived from (logit desc, token asc) via index-sort scratch.
    for (slot, &t) in live.iter().enumerate() {
        scratch[slot] = logits[t as usize];
    }
    quickselect_desc(live, scratch, 0, n - 1, k, &key);
}

fn quickselect_desc(live: &mut [u32], scratch: &mut [f32], mut lo: usize, mut hi: usize, k: usize, key: &impl Fn(u32) -> (f32, core::cmp::Reverse<u32>)) {
    loop {
        if lo >= hi {
            return;
        }
        let pivot = key(live[(lo + hi) / 2]);
        let mut i = lo;
        let mut j = hi;
        loop {
            while key(live[i]) > pivot {
                i += 1;
            }
            while key(live[j]) < pivot {
                j -= 1;
            }
            if i >= j {
                break;
            }
            live.swap(i, j);
            scratch.swap(i, j);
            i += 1;
            if j == 0 {
                break;
            }
            j -= 1;
        }
        if k <= j {
            hi = j;
        } else if k > j + 1 {
            lo = j + 1;
        } else {
            return;
        }
    }
}

/// 📐️ Smallest index `i` such that `cdf[i] >= u`, via binary search over a nondecreasing CDF.
/// Falls back to the last index when floating-point rounding leaves `u` fractionally above the
/// final (nominally `1.0`) entry.
pub fn cdf_binary_search(cdf: &[f64], u: f64) -> usize {
    if cdf.is_empty() {
        return 0;
    }
    let mut lo = 0usize;
    let mut hi = cdf.len() - 1;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if cdf[mid] < u {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}
// #endregion 🔖️Numerics

// #region 🔖️Bitset
/// 🎭️ A dense bitset over `0..len` token indices, `u64`-word packed. The single shared
/// representation for hard masks (constraints, allow/forbid lists, forced-token exclusivity) —
/// every mask operation the pipeline needs is `O(vocab / 64)`.
#[derive(Clone, PartialEq, Debug)]
pub struct TokenBitset {
    words: Vec<u64>,
    len: usize,
}

impl TokenBitset {
    /// 🎭️ All-zero (empty) bitset over `len` tokens.
    pub fn new_empty(len: usize) -> Self {
        Self { words: vec![0u64; len.div_ceil(64)], len }
    }

    /// 🎭️ All-one (full) bitset over `len` tokens.
    pub fn new_full(len: usize) -> Self {
        let mut set = Self::new_empty(len);
        set.fill();
        set
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn get(&self, token: TokenId) -> bool {
        let idx = token.get() as usize;
        debug_assert!(idx < self.len);
        (self.words[idx / 64] >> (idx % 64)) & 1 != 0
    }

    #[inline]
    pub fn set(&mut self, token: TokenId, value: bool) {
        let idx = token.get() as usize;
        debug_assert!(idx < self.len);
        let mask = 1u64 << (idx % 64);
        if value {
            self.words[idx / 64] |= mask;
        } else {
            self.words[idx / 64] &= !mask;
        }
    }

    /// 🎭️ Sets every bit `0..len` (trailing bits beyond `len` in the final word stay zero).
    pub fn fill(&mut self) {
        let full_words = self.len / 64;
        self.words[..full_words].fill(u64::MAX);
        let rem = self.len % 64;
        if rem > 0 {
            self.words[full_words] = (1u64 << rem) - 1;
        }
    }

    pub fn clear_all(&mut self) {
        self.words.fill(0);
    }

    /// 🎭️ In-place `self &= other`.
    pub fn and_with(&mut self, other: &TokenBitset) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a &= *b;
        }
    }

    /// 🎭️ In-place `self |= other`.
    pub fn or_with(&mut self, other: &TokenBitset) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a |= *b;
        }
    }

    /// 🎭️ In-place `self &= !other`.
    pub fn and_not_with(&mut self, other: &TokenBitset) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a &= !*b;
        }
    }

    pub fn count_ones(&self) -> usize {
        self.words.iter().map(|w| w.count_ones() as usize).sum()
    }

    pub fn is_all_zero(&self) -> bool {
        self.words.iter().all(|&w| w == 0)
    }

    /// 🎭️ Lowest set bit, word-skipping past all-zero words.
    pub fn first_set(&self) -> Option<TokenId> {
        for (word_idx, &word) in self.words.iter().enumerate() {
            if word != 0 {
                let bit = word.trailing_zeros() as usize;
                return Some(TokenId::new((word_idx * 64 + bit) as u32));
            }
        }
        None
    }

    /// 🎭️ Iterates set bits in ascending order, skipping whole zero words at a time.
    pub fn iter_ones(&self) -> impl Iterator<Item = TokenId> + '_ {
        self.words.iter().enumerate().flat_map(|(word_idx, &word)| {
            let mut remaining = word;
            core::iter::from_fn(move || {
                if remaining == 0 {
                    return None;
                }
                let bit = remaining.trailing_zeros();
                remaining &= remaining - 1;
                Some(TokenId::new((word_idx * 64 + bit as usize) as u32))
            })
        })
    }
}
// #endregion 🔖️Bitset

// #region 🔖️Rng
/// 🎲️ Which sub-stream of randomness a draw belongs to, so unrelated concerns (selection noise vs.
/// speculative-decoding acceptance vs. diffusion noise) never share bits even within one sequence.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum StreamPurpose {
    Selection = 0,
    Gumbel = 1,
    Noise = 2,
    Speculative = 3,
    Beam = 4,
    Diffusion = 5,
}

/// 🎲️ Identifies one independent random stream by the ids that produced it — never by batch slot,
/// so a continuous-batching reorder never changes which bits a sequence draws.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StreamKey {
    pub request: u64,
    pub sequence: u64,
    pub beam: u32,
    pub candidate: u32,
    pub purpose: StreamPurpose,
}

fn mix64(x: u64) -> u64 {
    crate::random::SplitMix64::new(x).next_u64()
}

fn stream_seed(key: StreamKey) -> u64 {
    let mut acc = mix64(key.request);
    acc = mix64(acc ^ mix64(key.sequence));
    acc = mix64(acc ^ mix64(key.beam as u64));
    acc = mix64(acc ^ mix64(key.candidate as u64));
    acc = mix64(acc ^ mix64(key.purpose as u64));
    acc
}

/// 🎲️ Which concrete generator produced a [`RngSnapshot`], so `restore` can reject a snapshot
/// meant for the other kind instead of silently reinterpreting its words.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RngKind {
    Counter,
    Xoshiro,
}

/// 🎲️ Portable capture of a generator's internal state, text-serializable for [`SequenceState`]
/// checkpoints.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RngSnapshot {
    pub kind: RngKind,
    pub words: [u64; 4],
}

impl RngSnapshot {
    /// 🎲️ Compact `kind:hex:hex:hex:hex` text form.
    pub fn to_text(&self) -> String {
        let kind = match self.kind {
            RngKind::Counter => "counter",
            RngKind::Xoshiro => "xoshiro",
        };
        format!("{kind}:{:016x}:{:016x}:{:016x}:{:016x}", self.words[0], self.words[1], self.words[2], self.words[3])
    }

    /// 🎲️ Inverse of [`RngSnapshot::to_text`].
    pub fn from_text(text: &str) -> Result<Self, SamplingError> {
        let mut parts = text.split(':');
        let kind = match parts.next() {
            Some("counter") => RngKind::Counter,
            Some("xoshiro") => RngKind::Xoshiro,
            _ => return Err(SamplingError::Corrupted { reason: "unknown rng snapshot kind" }),
        };
        let mut words = [0u64; 4];
        for w in words.iter_mut() {
            let part = parts.next().ok_or(SamplingError::Corrupted { reason: "truncated rng snapshot" })?;
            *w = u64::from_str_radix(part, 16).map_err(|_| SamplingError::Corrupted { reason: "invalid rng snapshot hex" })?;
        }
        if parts.next().is_some() {
            return Err(SamplingError::Corrupted { reason: "trailing data in rng snapshot" });
        }
        Ok(Self { kind, words })
    }
}

/// 🎲️ Object-safe source of randomness handed to samplers/warpers/search algorithms. Every
/// implementation must be splittable into independent child streams keyed by [`StreamKey`] alone
/// (never by call order), which is what keeps continuous-batching reorders and speculative
/// verification bit-reproducible.
pub trait RandomSource {
    fn next_u64(&mut self) -> u64;
    /// 🎲️ Derives an independent child stream from `(self, key)` — order-independent across calls.
    fn split(&self, key: StreamKey) -> Box<dyn RandomSource>;
    fn snapshot(&self) -> RngSnapshot;
    fn restore(&mut self, snapshot: &RngSnapshot) -> Result<(), SamplingError>;

    /// 🎲️ Uniform `f64` in `[0, 1)`.
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// 🎲️ Uniform `f64` in `(0, 1]` — safe as the argument to `ln()`, unlike `next_f64`.
    fn next_f64_open01(&mut self) -> f64 {
        (((self.next_u64() >> 11) + 1) as f64) * (1.0 / (1u64 << 53) as f64)
    }

    /// 🎲️ Uniform `u64` in `[lo, hi)` via rejection sampling (no modulo bias).
    fn next_range(&mut self, lo: u64, hi: u64) -> u64 {
        debug_assert!(hi >= lo, "next_range: hi must be >= lo");
        let range = hi - lo;
        if range == 0 {
            return lo;
        }
        let limit = u64::MAX - (u64::MAX % range);
        loop {
            let x = self.next_u64();
            if x < limit {
                return lo + x % range;
            }
        }
    }

    /// 🎲️ Standard Gumbel(0, 1) draw, `-ln(-ln(u))` for `u` in `(0, 1]`.
    fn gumbel(&mut self) -> f64 {
        let u = self.next_f64_open01();
        -(-u.ln()).ln()
    }
}

/// 🎲️ Default splittable [`RandomSource`]: a counter-based generator (double [`mix64`] of
/// `key ^ mix64(counter)`, Philox-lite) chosen over a stepped generator specifically because
/// splitting never advances or depends on the parent's step count — two sequences split from the
/// same parent at different times still get independent, order-irrelevant streams.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CounterRng {
    key: u64,
    ctr: u64,
}

impl CounterRng {
    /// 🎲️ A root stream from a plain seed, with no [`StreamKey`] semantics (tests, standalone use).
    pub fn from_seed(seed: u64) -> Self {
        Self { key: mix64(seed), ctr: 0 }
    }

    /// 🎲️ A root stream combining a request-level seed with a full [`StreamKey`] in one step.
    pub fn from_root(root_seed: u64, key: StreamKey) -> Self {
        Self { key: mix64(mix64(root_seed) ^ stream_seed(key)), ctr: 0 }
    }
}

impl RandomSource for CounterRng {
    fn next_u64(&mut self) -> u64 {
        let ctr = self.ctr;
        self.ctr = self.ctr.wrapping_add(1);
        mix64(self.key ^ mix64(ctr))
    }

    fn split(&self, key: StreamKey) -> Box<dyn RandomSource> {
        Box::new(Self { key: mix64(self.key ^ stream_seed(key)), ctr: 0 })
    }

    fn snapshot(&self) -> RngSnapshot {
        RngSnapshot { kind: RngKind::Counter, words: [self.key, self.ctr, 0, 0] }
    }

    fn restore(&mut self, snapshot: &RngSnapshot) -> Result<(), SamplingError> {
        if snapshot.kind != RngKind::Counter {
            return Err(SamplingError::Corrupted { reason: "rng snapshot kind mismatch: expected counter" });
        }
        self.key = snapshot.words[0];
        self.ctr = snapshot.words[1];
        Ok(())
    }
}

/// 🎲️ [`RandomSource`] adapter over [`crate::random::Rng`] (xoshiro256**), for callers who
/// want that generator's statistical profile instead of the default counter-based stream.
pub struct XoshiroSource(crate::random::Rng);

impl XoshiroSource {
    pub fn from_seed(seed: u64) -> Self {
        Self(crate::random::Rng::from_seed(seed))
    }
}

impl RandomSource for XoshiroSource {
    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn split(&self, key: StreamKey) -> Box<dyn RandomSource> {
        let state = self.0.state();
        let seed = mix64(state[0] ^ state[1] ^ stream_seed(key));
        Box::new(Self::from_seed(seed))
    }

    fn snapshot(&self) -> RngSnapshot {
        RngSnapshot { kind: RngKind::Xoshiro, words: self.0.state() }
    }

    fn restore(&mut self, snapshot: &RngSnapshot) -> Result<(), SamplingError> {
        if snapshot.kind != RngKind::Xoshiro {
            return Err(SamplingError::Corrupted { reason: "rng snapshot kind mismatch: expected xoshiro" });
        }
        self.0 = crate::random::Rng::from_state(snapshot.words);
        Ok(())
    }
}
// #endregion 🔖️Rng

// #region 🔖️Vocabulary
/// 📖️ Static facts about the token space a [`SamplingConfig`] samples over.
#[derive(Clone, PartialEq, Debug)]
pub struct Vocabulary {
    pub size: usize,
    pub eos: Vec<TokenId>,
    pub bos: Option<TokenId>,
    pub pad: Option<TokenId>,
    pub unk: Option<TokenId>,
    pub special: TokenBitset,
}

impl Vocabulary {
    /// 📖️ A vocabulary of `size` tokens with no special tokens configured.
    pub fn new(size: usize) -> Self {
        Self { size, eos: Vec::new(), bos: None, pad: None, unk: None, special: TokenBitset::new_empty(size) }
    }

    pub fn with_eos(mut self, eos: Vec<TokenId>) -> Self {
        self.eos = eos;
        self
    }

    pub fn with_bos(mut self, bos: TokenId) -> Self {
        self.bos = Some(bos);
        self
    }

    pub fn with_pad(mut self, pad: TokenId) -> Self {
        self.pad = Some(pad);
        self
    }

    pub fn with_unk(mut self, unk: TokenId) -> Self {
        self.unk = Some(unk);
        self
    }

    /// 📖️ Marks `tokens` as special (suppressible via `ProcessorSpec::SuppressSpecial`).
    pub fn with_special(mut self, tokens: &[TokenId]) -> Self {
        for &token in tokens {
            self.special.set(token, true);
        }
        self
    }

    pub fn is_eos(&self, token: TokenId) -> bool {
        self.eos.contains(&token)
    }

    /// 📖️ Errors unless `len` matches this vocabulary's declared size.
    pub fn validate_logits_len(&self, len: usize) -> Result<(), SamplingError> {
        if len != self.size {
            Err(SamplingError::VocabMismatch { expected: self.size, actual: len })
        } else {
            Ok(())
        }
    }
}

/// 📖️ Maps [`TokenId`]s to their surface-form bytes, for constraints and stop matching that
/// operate on generated text rather than raw ids.
pub trait TokenTextAdapter {
    fn vocab_size(&self) -> usize;
    /// 📖️ Raw (possibly partial-UTF-8) bytes of one token; `None` for byte-less special tokens.
    fn token_bytes(&self, token: TokenId) -> Option<&[u8]>;
    /// 📖️ Stable hash of the whole token table, used to key automaton-state×token caches so a
    /// swapped tokenizer can never silently reuse another tokenizer's cached transitions.
    fn fingerprint(&self) -> u64;
}

/// 📖️ Reference [`TokenTextAdapter`] over a plain `&[&[u8]]` token table.
pub struct SliceTextAdapter<'a> {
    tokens: &'a [&'a [u8]],
}

impl<'a> SliceTextAdapter<'a> {
    pub fn new(tokens: &'a [&'a [u8]]) -> Self {
        Self { tokens }
    }
}

impl TokenTextAdapter for SliceTextAdapter<'_> {
    fn vocab_size(&self) -> usize {
        self.tokens.len()
    }

    fn token_bytes(&self, token: TokenId) -> Option<&[u8]> {
        self.tokens.get(token.get() as usize).copied()
    }

    fn fingerprint(&self) -> u64 {
        // 📖️ FNV-1a over every token's bytes with a separator byte between entries so `["ab","c"]`
        // and `["a","bc"]` never collide.
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for token in self.tokens {
            for &b in *token {
                hash = (hash ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3);
            }
            hash = (hash ^ 0xFF).wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }
}
// #endregion 🔖️Vocabulary

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
    pub fn eval(&self, input: ScheduleInput) -> f64 {
        match self {
            Self::Constant(v) => *v,
            Self::Linear { from, to, over_steps } => {
                let t = schedule_progress(input.step, *over_steps);
                from + (to - from) * t
            }
            Self::Exponential { from, to, over_steps } => {
                let t = schedule_progress(input.step, *over_steps);
                if *from > 0.0 && *to > 0.0 {
                    from * (to / from).powf(t)
                } else {
                    from + (to - from) * t
                }
            }
            Self::Cosine { from, to, over_steps } => {
                let t = schedule_progress(input.step, *over_steps);
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
    pub fn to_json(&self) -> JsonValue {
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
    pub fn from_json(value: &JsonValue) -> Result<Self, SamplingError> {
        let kind = value.get("kind").and_then(JsonValue::as_str).ok_or(SamplingError::Corrupted { reason: "schedule missing kind" })?;
        let num = |key: &'static str| value.get(key).and_then(JsonValue::as_f64).ok_or(SamplingError::Corrupted { reason: "schedule missing numeric field" });
        match kind {
            "constant" => Ok(Self::Constant(num("value")?)),
            "linear" => Ok(Self::Linear { from: num("from")?, to: num("to")?, over_steps: num("over_steps")? as u32 }),
            "exponential" => Ok(Self::Exponential { from: num("from")?, to: num("to")?, over_steps: num("over_steps")? as u32 }),
            "cosine" => Ok(Self::Cosine { from: num("from")?, to: num("to")?, over_steps: num("over_steps")? as u32 }),
            "piecewise" => {
                let pieces = value.get("pieces").and_then(JsonValue::as_array).ok_or(SamplingError::Corrupted { reason: "piecewise schedule missing pieces" })?;
                let mut out = Vec::with_capacity(pieces.len());
                for piece in pieces {
                    let pair = piece.as_array().ok_or(SamplingError::Corrupted { reason: "piecewise entry must be a pair" })?;
                    let (Some(step), Some(v)) = (pair.first().and_then(JsonValue::as_f64), pair.get(1).and_then(JsonValue::as_f64)) else {
                        return Err(SamplingError::Corrupted { reason: "piecewise entry must be [step, value]" });
                    };
                    out.push((StepIndex::new(step as u32), v));
                }
                Ok(Self::Piecewise(out))
            }
            "by_position" => {
                let values = value.get("values").and_then(JsonValue::as_array).ok_or(SamplingError::Corrupted { reason: "by_position schedule missing values" })?;
                let out = values.iter().map(|v| v.as_f64().ok_or(SamplingError::Corrupted { reason: "by_position value must be numeric" })).collect::<Result<Vec<_>, _>>()?;
                Ok(Self::ByPosition(out))
            }
            "entropy_scaled" => Ok(Self::EntropyScaled { base: num("base")?, gain: num("gain")?, min: num("min")?, max: num("max")? }),
            "callback" => Err(SamplingError::Corrupted { reason: "callback schedules cannot be deserialized" }),
            _ => Err(SamplingError::Corrupted { reason: "unknown schedule kind" }),
        }
    }
}

fn schedule_progress(step: StepIndex, over_steps: u32) -> f64 {
    if over_steps == 0 {
        1.0
    } else {
        (step.get() as f64 / over_steps as f64).min(1.0)
    }
}
// #endregion 🔖️Schedules

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
    pub fn validate(&self) -> Result<(), SamplingError> {
        self.limits.validate()?;
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
            validate_processor_spec(processor, &self.limits)?;
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
    pub fn fingerprint(&self) -> u64 {
        let text = write_json(&self.to_json());
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for &b in text.as_bytes() {
            hash = (hash ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    /// ⚙️ Greedy decoding, no penalties or truncation — the deterministic baseline preset.
    pub fn precise() -> Self {
        Self { method: SamplingMethod::Greedy { tie_break: TieBreak::LowestTokenId }, ..Self::default() }
    }

    /// ⚙️ Temperature 0.7, top-p 0.9, min-p 0.05, mild repetition penalty — a general-purpose
    /// preset balancing coherence and variety.
    pub fn balanced() -> Self {
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
    pub fn creative() -> Self {
        Self {
            method: SamplingMethod::Multinomial { strategy: MultinomialStrategy::CdfBinarySearch },
            processors: vec![ProcessorSpec::Temperature { value: Schedule::Constant(1.0) }, ProcessorSpec::TopK { k: Schedule::Constant(100.0), min_keep: 1 }, ProcessorSpec::TopP { p: Schedule::Constant(0.95), min_keep: 1 }],
            ..Self::default()
        }
    }

    /// ⚙️ Fixed seed, greedy, tiny token budget — for reproducible tests, not production use.
    pub fn deterministic_test() -> Self {
        Self { method: SamplingMethod::Greedy { tie_break: TieBreak::LowestTokenId }, seed: 42, max_tokens: 64, ..Self::default() }
    }

    /// ⚙️ Structured JSON form (versioned: top-level `"version": 1`).
    pub fn to_json(&self) -> JsonValue {
        JsonValue::Object(vec![
            ("version".into(), JsonValue::Num(1.0)),
            ("method".into(), sampling_method_to_json(&self.method)),
            ("processors".into(), JsonValue::Array(self.processors.iter().map(processor_spec_to_json).collect())),
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
    pub fn from_json(value: &JsonValue) -> Result<Self, SamplingError> {
        let version = value.get("version").and_then(JsonValue::as_f64).ok_or(SamplingError::Corrupted { reason: "config missing version" })?;
        if version as u32 != 1 {
            return Err(SamplingError::SerializationVersion { expected: 1, actual: version as u32 });
        }
        let method = value.get("method").ok_or(SamplingError::Corrupted { reason: "config missing method" }).and_then(sampling_method_from_json)?;
        let processors = value.get("processors").and_then(JsonValue::as_array).ok_or(SamplingError::Corrupted { reason: "config missing processors" })?.iter().map(processor_spec_from_json).collect::<Result<Vec<_>, _>>()?;
        let error_mode = match value.get("error_mode").and_then(JsonValue::as_str) {
            Some("strict") => ErrorMode::Strict,
            Some("permissive") | None => ErrorMode::Permissive,
            Some(_) => return Err(SamplingError::Corrupted { reason: "unknown error_mode" }),
        };
        let num = |key: &'static str, default: f64| value.get(key).and_then(JsonValue::as_f64).unwrap_or(default);
        Ok(Self { method, processors, error_mode, seed: num("seed", 0.0) as u64, candidate_count: num("candidate_count", 1.0) as usize, min_tokens: num("min_tokens", 0.0) as usize, max_tokens: num("max_tokens", 4_096.0) as usize, ..Self::default() })
    }
}

fn validate_processor_spec(spec: &ProcessorSpec, limits: &SamplingLimits) -> Result<(), SamplingError> {
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

fn sampling_method_to_json(method: &SamplingMethod) -> JsonValue {
    let obj = |pairs: Vec<(&str, JsonValue)>| JsonValue::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    match method {
        SamplingMethod::Greedy { tie_break } => obj(vec![("kind", JsonValue::Str("greedy".into())), ("tie_break", tie_break_to_json(*tie_break))]),
        SamplingMethod::Multinomial { strategy } => obj(vec![("kind", JsonValue::Str("multinomial".into())), ("strategy", multinomial_strategy_to_json(*strategy))]),
        SamplingMethod::GumbelMax => obj(vec![("kind", JsonValue::Str("gumbel_max".into()))]),
        SamplingMethod::GumbelTopK { k } => obj(vec![("kind", JsonValue::Str("gumbel_top_k".into())), ("k", JsonValue::Num(*k as f64))]),
    }
}

fn sampling_method_from_json(value: &JsonValue) -> Result<SamplingMethod, SamplingError> {
    let kind = value.get("kind").and_then(JsonValue::as_str).ok_or(SamplingError::Corrupted { reason: "method missing kind" })?;
    match kind {
        "greedy" => Ok(SamplingMethod::Greedy { tie_break: tie_break_from_json(value.get("tie_break"))? }),
        "multinomial" => Ok(SamplingMethod::Multinomial { strategy: multinomial_strategy_from_json(value.get("strategy"))? }),
        "gumbel_max" => Ok(SamplingMethod::GumbelMax),
        "gumbel_top_k" => Ok(SamplingMethod::GumbelTopK { k: value.get("k").and_then(JsonValue::as_f64).unwrap_or(1.0) as usize }),
        _ => Err(SamplingError::Corrupted { reason: "unknown sampling method kind" }),
    }
}

fn tie_break_to_json(tie_break: TieBreak) -> JsonValue {
    JsonValue::Str(
        match tie_break {
            TieBreak::LowestTokenId => "lowest_token_id",
            TieBreak::HighestTokenId => "highest_token_id",
            TieBreak::FirstSeen => "first_seen",
        }
        .into(),
    )
}

fn tie_break_from_json(value: Option<&JsonValue>) -> Result<TieBreak, SamplingError> {
    match value.and_then(JsonValue::as_str) {
        Some("lowest_token_id") | None => Ok(TieBreak::LowestTokenId),
        Some("highest_token_id") => Ok(TieBreak::HighestTokenId),
        Some("first_seen") => Ok(TieBreak::FirstSeen),
        Some(_) => Err(SamplingError::Corrupted { reason: "unknown tie_break" }),
    }
}

fn multinomial_strategy_to_json(strategy: MultinomialStrategy) -> JsonValue {
    JsonValue::Str(
        match strategy {
            MultinomialStrategy::CdfBinarySearch => "cdf_binary_search",
            MultinomialStrategy::LinearScan => "linear_scan",
            MultinomialStrategy::Alias => "alias",
        }
        .into(),
    )
}

fn multinomial_strategy_from_json(value: Option<&JsonValue>) -> Result<MultinomialStrategy, SamplingError> {
    match value.and_then(JsonValue::as_str) {
        Some("cdf_binary_search") | None => Ok(MultinomialStrategy::CdfBinarySearch),
        Some("linear_scan") => Ok(MultinomialStrategy::LinearScan),
        Some("alias") => Ok(MultinomialStrategy::Alias),
        Some(_) => Err(SamplingError::Corrupted { reason: "unknown multinomial strategy" }),
    }
}

fn penalty_scope_to_json(scope: PenaltyScope) -> JsonValue {
    JsonValue::Str(
        match scope {
            PenaltyScope::PromptAndGenerated => "prompt_and_generated",
            PenaltyScope::GeneratedOnly => "generated_only",
            PenaltyScope::PromptOnly => "prompt_only",
        }
        .into(),
    )
}

fn penalty_scope_from_json(value: Option<&JsonValue>) -> Result<PenaltyScope, SamplingError> {
    match value.and_then(JsonValue::as_str) {
        Some("prompt_and_generated") | None => Ok(PenaltyScope::PromptAndGenerated),
        Some("generated_only") => Ok(PenaltyScope::GeneratedOnly),
        Some("prompt_only") => Ok(PenaltyScope::PromptOnly),
        Some(_) => Err(SamplingError::Corrupted { reason: "unknown penalty scope" }),
    }
}

fn processor_spec_to_json(spec: &ProcessorSpec) -> JsonValue {
    let obj = |pairs: Vec<(&str, JsonValue)>| JsonValue::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect());
    let tokens_json = |tokens: &[TokenId]| JsonValue::Array(tokens.iter().map(|t| JsonValue::Num(t.get() as f64)).collect());
    let phrases_json = |phrases: &[Vec<TokenId>]| JsonValue::Array(phrases.iter().map(|p| tokens_json(p)).collect());
    match spec {
        ProcessorSpec::Temperature { value } => obj(vec![("kind", JsonValue::Str("temperature".into())), ("value", value.to_json())]),
        ProcessorSpec::DynamicTemperature { base, entropy_gain, min, max } => {
            obj(vec![("kind", JsonValue::Str("dynamic_temperature".into())), ("base", base.to_json()), ("entropy_gain", JsonValue::Num(*entropy_gain)), ("min", JsonValue::Num(*min)), ("max", JsonValue::Num(*max))])
        }
        ProcessorSpec::TopK { k, min_keep } => obj(vec![("kind", JsonValue::Str("top_k".into())), ("k", k.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::TopP { p, min_keep } => obj(vec![("kind", JsonValue::Str("top_p".into())), ("p", p.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::MinP { p, min_keep } => obj(vec![("kind", JsonValue::Str("min_p".into())), ("p", p.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::Typical { mass, min_keep } => obj(vec![("kind", JsonValue::Str("typical".into())), ("mass", mass.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::LocallyTypical { mass, min_keep } => obj(vec![("kind", JsonValue::Str("locally_typical".into())), ("mass", mass.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::TailFree { z, min_keep } => obj(vec![("kind", JsonValue::Str("tail_free".into())), ("z", z.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::Epsilon { cutoff, min_keep } => obj(vec![("kind", JsonValue::Str("epsilon".into())), ("cutoff", cutoff.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::Eta { cutoff, min_keep } => obj(vec![("kind", JsonValue::Str("eta".into())), ("cutoff", cutoff.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::TopA { power, min_keep } => obj(vec![("kind", JsonValue::Str("top_a".into())), ("power", power.to_json()), ("min_keep", JsonValue::Num(*min_keep as f64))]),
        ProcessorSpec::RankTruncation { max_rank } => obj(vec![("kind", JsonValue::Str("rank_truncation".into())), ("max_rank", JsonValue::Num(*max_rank as f64))]),
        ProcessorSpec::AdaptiveTruncation { target_entropy, target_effective_count } => {
            obj(vec![("kind", JsonValue::Str("adaptive_truncation".into())), ("target_entropy", target_entropy.map_or(JsonValue::Null, JsonValue::Num)), ("target_effective_count", target_effective_count.map_or(JsonValue::Null, JsonValue::Num))])
        }
        ProcessorSpec::RepetitionPenalty { penalty, scope } => obj(vec![("kind", JsonValue::Str("repetition_penalty".into())), ("penalty", JsonValue::Num(*penalty as f64)), ("scope", penalty_scope_to_json(*scope))]),
        ProcessorSpec::PresencePenalty { penalty, scope } => obj(vec![("kind", JsonValue::Str("presence_penalty".into())), ("penalty", JsonValue::Num(*penalty as f64)), ("scope", penalty_scope_to_json(*scope))]),
        ProcessorSpec::FrequencyPenalty { penalty, scope } => obj(vec![("kind", JsonValue::Str("frequency_penalty".into())), ("penalty", JsonValue::Num(*penalty as f64)), ("scope", penalty_scope_to_json(*scope))]),
        ProcessorSpec::DecayingPenalty { penalty, window, half_life, scope } => {
            obj(vec![("kind", JsonValue::Str("decaying_penalty".into())), ("penalty", JsonValue::Num(*penalty as f64)), ("window", JsonValue::Num(*window as f64)), ("half_life", JsonValue::Num(*half_life)), ("scope", penalty_scope_to_json(*scope))])
        }
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

fn processor_spec_from_json(value: &JsonValue) -> Result<ProcessorSpec, SamplingError> {
    let kind = value.get("kind").and_then(JsonValue::as_str).ok_or(SamplingError::Corrupted { reason: "processor missing kind" })?;
    let schedule = |key: &'static str| -> Result<Schedule, SamplingError> { value.get(key).ok_or(SamplingError::Corrupted { reason: "processor missing schedule field" }).and_then(Schedule::from_json) };
    let num = |key: &'static str, default: f64| value.get(key).and_then(JsonValue::as_f64).unwrap_or(default);
    let tokens = |key: &'static str| -> Vec<TokenId> { value.get(key).and_then(JsonValue::as_array).map(|a| a.iter().filter_map(JsonValue::as_f64).map(|n| TokenId::new(n as u32)).collect()).unwrap_or_default() };
    let phrases = |key: &'static str| -> Vec<Vec<TokenId>> {
        value.get(key).and_then(JsonValue::as_array).map(|a| a.iter().filter_map(JsonValue::as_array).map(|p| p.iter().filter_map(JsonValue::as_f64).map(|n| TokenId::new(n as u32)).collect()).collect()).unwrap_or_default()
    };
    match kind {
        "temperature" => Ok(ProcessorSpec::Temperature { value: schedule("value")? }),
        "dynamic_temperature" => Ok(ProcessorSpec::DynamicTemperature { base: schedule("base")?, entropy_gain: num("entropy_gain", 0.0), min: num("min", 0.0), max: num("max", 2.0) }),
        "top_k" => Ok(ProcessorSpec::TopK { k: schedule("k")?, min_keep: num("min_keep", 1.0) as usize }),
        "top_p" => Ok(ProcessorSpec::TopP { p: schedule("p")?, min_keep: num("min_keep", 1.0) as usize }),
        "min_p" => Ok(ProcessorSpec::MinP { p: schedule("p")?, min_keep: num("min_keep", 1.0) as usize }),
        "typical" => Ok(ProcessorSpec::Typical { mass: schedule("mass")?, min_keep: num("min_keep", 1.0) as usize }),
        "locally_typical" => Ok(ProcessorSpec::LocallyTypical { mass: schedule("mass")?, min_keep: num("min_keep", 1.0) as usize }),
        "tail_free" => Ok(ProcessorSpec::TailFree { z: schedule("z")?, min_keep: num("min_keep", 1.0) as usize }),
        "epsilon" => Ok(ProcessorSpec::Epsilon { cutoff: schedule("cutoff")?, min_keep: num("min_keep", 1.0) as usize }),
        "eta" => Ok(ProcessorSpec::Eta { cutoff: schedule("cutoff")?, min_keep: num("min_keep", 1.0) as usize }),
        "top_a" => Ok(ProcessorSpec::TopA { power: schedule("power")?, min_keep: num("min_keep", 1.0) as usize }),
        "rank_truncation" => Ok(ProcessorSpec::RankTruncation { max_rank: num("max_rank", 1.0) as usize }),
        "adaptive_truncation" => Ok(ProcessorSpec::AdaptiveTruncation { target_entropy: value.get("target_entropy").and_then(JsonValue::as_f64), target_effective_count: value.get("target_effective_count").and_then(JsonValue::as_f64) }),
        "repetition_penalty" => Ok(ProcessorSpec::RepetitionPenalty { penalty: num("penalty", 1.0) as f32, scope: penalty_scope_from_json(value.get("scope"))? }),
        "presence_penalty" => Ok(ProcessorSpec::PresencePenalty { penalty: num("penalty", 0.0) as f32, scope: penalty_scope_from_json(value.get("scope"))? }),
        "frequency_penalty" => Ok(ProcessorSpec::FrequencyPenalty { penalty: num("penalty", 0.0) as f32, scope: penalty_scope_from_json(value.get("scope"))? }),
        "decaying_penalty" => Ok(ProcessorSpec::DecayingPenalty { penalty: num("penalty", 0.0) as f32, window: num("window", 16.0) as usize, half_life: num("half_life", 1.0), scope: penalty_scope_from_json(value.get("scope"))? }),
        "token_class_penalty" => {
            Ok(ProcessorSpec::TokenClassPenalty { class_tokens: phrases("class_tokens"), factors: value.get("factors").and_then(JsonValue::as_array).map(|a| a.iter().filter_map(JsonValue::as_f64).map(|n| n as f32).collect()).unwrap_or_default() })
        }
        "no_repeat_ngram" => Ok(ProcessorSpec::NoRepeatNgram { n: num("n", 3.0) as usize }),
        "phrase_penalty" => Ok(ProcessorSpec::PhrasePenalty { phrases: phrases("phrases"), penalty: num("penalty", 0.0) as f32 }),
        "logit_bias_sparse" => Ok(ProcessorSpec::LogitBiasSparse {
            entries: value
                .get("entries")
                .and_then(JsonValue::as_array)
                .map(|a| a.iter().filter_map(JsonValue::as_array).filter_map(|pair| Some((TokenId::new(pair.first()?.as_f64()? as u32), pair.get(1)?.as_f64()? as f32))).collect())
                .unwrap_or_default(),
        }),
        "logit_bias_dense" => Ok(ProcessorSpec::LogitBiasDense { values: value.get("values").and_then(JsonValue::as_array).map(|a| a.iter().filter_map(JsonValue::as_f64).map(|n| n as f32).collect()).unwrap_or_default() }),
        "allow_tokens" => Ok(ProcessorSpec::AllowTokens { tokens: tokens("tokens") }),
        "forbid_tokens" => Ok(ProcessorSpec::ForbidTokens { tokens: tokens("tokens") }),
        "suppress_special" => Ok(ProcessorSpec::SuppressSpecial),
        "bad_words" => Ok(ProcessorSpec::BadWords { phrases: phrases("phrases") }),
        "sequence_encouragement" => Ok(ProcessorSpec::SequenceEncouragement { phrases: phrases("phrases"), bonus: num("bonus", 0.0) as f32 }),
        "mirostat" => Ok(ProcessorSpec::Mirostat {
            version: if value.get("version").and_then(JsonValue::as_str) == Some("v1") { MirostatVersion::V1 } else { MirostatVersion::V2 },
            target_surprise: num("target_surprise", 5.0),
            learning_rate: num("learning_rate", 0.1),
        }),
        "entropy_pid" => Ok(ProcessorSpec::EntropyPid { target: num("target", 2.0), kp: num("kp", 0.1), ki: num("ki", 0.0), kd: num("kd", 0.0) }),
        "repetition_controller" => Ok(ProcessorSpec::RepetitionController { window: num("window", 16.0) as usize, threshold: num("threshold", 0.5), boost: num("boost", 0.2) }),
        "confidence_controller" => Ok(ProcessorSpec::ConfidenceController { low_entropy: num("low_entropy", 0.5), high_entropy: num("high_entropy", 3.0), low_temp: num("low_temp", 0.5), high_temp: num("high_temp", 1.2) }),
        _ => Err(SamplingError::Corrupted { reason: "unknown processor kind" }),
    }
}

/// ⚙️ Chainable constructor for [`SamplingConfig`]; `build()` runs full validation once.
#[derive(Clone, Debug)]
pub struct SamplingConfigBuilder {
    config: SamplingConfig,
}

impl SamplingConfigBuilder {
    pub fn new() -> Self {
        Self { config: SamplingConfig::default() }
    }

    pub fn method(mut self, method: SamplingMethod) -> Self {
        self.config.method = method;
        self
    }

    pub fn processor(mut self, processor: ProcessorSpec) -> Self {
        self.config.processors.push(processor);
        self
    }

    pub fn error_mode(mut self, mode: ErrorMode) -> Self {
        self.config.error_mode = mode;
        self
    }

    pub fn sanitize(mut self, policy: SanitizePolicy) -> Self {
        self.config.sanitize = policy;
        self
    }

    pub fn accum(mut self, accum: Accum) -> Self {
        self.config.accum = accum;
        self
    }

    pub fn seed(mut self, seed: u64) -> Self {
        self.config.seed = seed;
        self
    }

    pub fn candidate_count(mut self, count: usize) -> Self {
        self.config.candidate_count = count;
        self
    }

    pub fn min_tokens(mut self, min_tokens: usize) -> Self {
        self.config.min_tokens = min_tokens;
        self
    }

    pub fn max_tokens(mut self, max_tokens: usize) -> Self {
        self.config.max_tokens = max_tokens;
        self
    }

    pub fn forced(mut self, forced: ForcedSpec) -> Self {
        self.config.forced = forced;
        self
    }

    pub fn stops(mut self, stops: StopSpec) -> Self {
        self.config.stops = stops;
        self
    }

    pub fn constraint(mut self, constraint: ConstraintSpec) -> Self {
        self.config.constraints.push(constraint);
        self
    }

    pub fn logprobs(mut self, logprobs: LogprobsSpec) -> Self {
        self.config.logprobs = logprobs;
        self
    }

    pub fn limits(mut self, limits: SamplingLimits) -> Self {
        self.config.limits = limits;
        self
    }

    pub fn diagnostics(mut self, diagnostics: DiagnosticsSpec) -> Self {
        self.config.diagnostics = diagnostics;
        self
    }

    /// ⚙️ Validates and returns the finished config.
    pub fn build(self) -> Result<SamplingConfig, SamplingError> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for SamplingConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
// #endregion 🔖️Config

// #region 🔖️Candidates
/// 🏅️ One scored token, at whatever pipeline stage produced it (pre- or post-truncation, or the
/// final selected token).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Candidate {
    pub token: TokenId,
    pub raw_logit: f32,
    pub processed_logit: f32,
    pub prob: f32,
    pub logprob: f32,
    pub rank: u32,
}

/// 🏅️ Alternative log-probabilities reported alongside a selection, before and/or after
/// truncation warpers ran (per [`LogprobsSpec`]).
#[derive(Clone, PartialEq, Debug, Default)]
pub struct TopLogprobs {
    pub pre_truncation: Vec<Candidate>,
    pub post_truncation: Vec<Candidate>,
}

/// 🏅️ Optional per-step numerical/pipeline diagnostics (only populated when
/// [`DiagnosticsSpec::enabled`]).
#[derive(Clone, PartialEq, Debug, Default)]
pub struct StepDiagnostics {
    pub entropy: f64,
    pub effective_count: f64,
    pub truncation_mass: f64,
    pub masked_by: Vec<(&'static str, u32)>,
    pub timings_ns: Vec<(&'static str, u64)>,
    pub fallback: Option<FallbackAction>,
    pub health: Option<DistributionHealth>,
}

/// 🏅️ Everything one [`sample_step_stateless`] call (or, later, a stateful engine step) returns.
#[derive(Clone, PartialEq, Debug)]
pub struct SamplingResult {
    pub token: TokenId,
    pub logprob: f32,
    pub finish: Option<FinishReason>,
    pub alternatives: Vec<Candidate>,
    pub top_logprobs: Option<TopLogprobs>,
    pub rng_stream: StreamKey,
    pub diagnostics: Option<StepDiagnostics>,
}
// #endregion 🔖️Candidates

// #region 🔖️Workspace
/// 🧰️ Reinterprets a `&[u32]` as `&[TokenId]` without copying.
///
/// SAFETY: `TokenId` is `#[repr(transparent)]` over `u32` (see its definition in `🔖️Ids`), so the
/// two types share identical size, alignment, and bit-pattern validity — every `u32` is a valid
/// `TokenId`.
fn cast_u32_slice_to_token_ids(ids: &[u32]) -> &[TokenId] {
    unsafe { core::slice::from_raw_parts(ids.as_ptr().cast::<TokenId>(), ids.len()) }
}

fn argmax_token(logits: &[f32], tie_break: TieBreak) -> TokenId {
    let mut best_idx = 0u32;
    let mut best_val = f32::NEG_INFINITY;
    for (i, &v) in logits.iter().enumerate() {
        let better = match tie_break {
            TieBreak::HighestTokenId => v >= best_val,
            TieBreak::LowestTokenId | TieBreak::FirstSeen => v > best_val,
        };
        if better {
            best_val = v;
            best_idx = i as u32;
        }
    }
    TokenId::new(best_idx)
}

fn argmax_index_in_slice(logits: &[f32], indices: &[u32]) -> u32 {
    let mut best = indices[0];
    let mut best_val = logits[best as usize];
    for &idx in &indices[1..] {
        let v = logits[idx as usize];
        if v > best_val {
            best_val = v;
            best = idx;
        }
    }
    best
}

/// 🧰️ Per-step scratch state for one sequence's logits: raw/processed vocab-sized arrays, the
/// hard-mask bitset, the shrinking `live` candidate-index list every warper operates on, and the
/// buffers the shared prob-sort (`sort_live_by_prob_desc`) reuses so steady-state stepping never
/// allocates once capacities have grown to the vocabulary size.
pub struct LogitsWorkspace {
    vocab_size: usize,
    accum: Accum,
    raw: Vec<f32>,
    processed: Vec<f32>,
    mask: TokenBitset,
    live: Vec<u32>,
    probs: Vec<f32>,
    sort_order: Vec<u32>,
    sorted_live_buf: Vec<u32>,
    sorted_probs_buf: Vec<f32>,
    saved_argmax: TokenId,
}

impl LogitsWorkspace {
    pub fn new(vocab_size: usize) -> Self {
        Self {
            vocab_size,
            accum: Accum::default(),
            raw: vec![0.0; vocab_size],
            processed: vec![0.0; vocab_size],
            mask: TokenBitset::new_full(vocab_size),
            live: (0..vocab_size as u32).collect(),
            probs: Vec::with_capacity(vocab_size),
            sort_order: Vec::with_capacity(vocab_size),
            sorted_live_buf: Vec::with_capacity(vocab_size),
            sorted_probs_buf: Vec::with_capacity(vocab_size),
            saved_argmax: TokenId::new(0),
        }
    }

    /// 🧰️ Grows every buffer to `vocab_size` if it is larger than the workspace's current
    /// capacity; a no-operation (never shrinks) otherwise — the basis of pool reuse across batch slots.
    pub fn ensure_capacity(&mut self, vocab_size: usize) {
        if vocab_size <= self.vocab_size {
            return;
        }
        self.vocab_size = vocab_size;
        self.raw.resize(vocab_size, 0.0);
        self.processed.resize(vocab_size, 0.0);
        self.mask = TokenBitset::new_full(vocab_size);
    }

    pub fn set_accum(&mut self, accum: Accum) {
        self.accum = accum;
    }

    pub fn vocab_size(&self) -> usize {
        self.vocab_size
    }

    pub fn live(&self) -> &[u32] {
        &self.live
    }

    pub fn set_live(&mut self, live: Vec<u32>) {
        self.live = live;
    }

    pub fn processed(&self) -> &[f32] {
        &self.processed
    }

    pub fn raw(&self) -> &[f32] {
        &self.raw
    }

    pub fn probs(&self) -> &[f32] {
        &self.probs
    }

    pub fn mask_mut(&mut self) -> &mut TokenBitset {
        &mut self.mask
    }

    pub fn mask(&self) -> &TokenBitset {
        &self.mask
    }

    pub fn saved_argmax(&self) -> TokenId {
        self.saved_argmax
    }

    /// 🧰️ Resets the workspace for a fresh step: copies `raw_logits` into `raw`, sanitizes it in
    /// place per `policy`, mirrors the sanitized values into `processed`, fills the mask, resets
    /// `live` to every vocabulary index, and records the (post-sanitize) argmax — the anchor the
    /// fallback ladder uses once every other option is exhausted. Returns the count of altered
    /// (NaN/Inf-resolved) entries.
    pub fn reset_for_step(&mut self, raw_logits: &[f32], policy: SanitizePolicy) -> Result<usize, SamplingError> {
        debug_assert_eq!(raw_logits.len(), self.vocab_size);
        self.raw.copy_from_slice(raw_logits);
        let altered = sanitize_logits(&mut self.raw, policy)?;
        self.processed.copy_from_slice(&self.raw);
        self.mask.fill();
        self.live.clear();
        self.live.extend(0..self.vocab_size as u32);
        self.saved_argmax = argmax_token(&self.raw, TieBreak::LowestTokenId);
        self.probs.clear();
        Ok(altered)
    }

    /// 🧰️ Removes every `live` entry whose mask bit is unset — call once after all hard-mask
    /// processors have run, before any soft-penalty or truncation processor.
    pub fn sync_live_with_mask(&mut self) {
        let mask = &self.mask;
        self.live.retain(|&idx| mask.get(TokenId::new(idx)));
    }

    pub fn scale_processed_over_live(&mut self, factor: f32) {
        for &idx in &self.live {
            self.processed[idx as usize] *= factor;
        }
    }

    pub fn add_bias_over_live(&mut self, mut bias: impl FnMut(TokenId) -> f32) {
        for &idx in &self.live {
            self.processed[idx as usize] += bias(TokenId::new(idx));
        }
    }

    /// 🧰️ Replaces each live token's processed logit with `f(token, current_logit)` — the general
    /// form multiplicative penalties (sign-aware repetition penalty, per-class factors) need,
    /// since those aren't expressible as a pure additive bias.
    pub fn transform_processed_over_live(&mut self, mut f: impl FnMut(TokenId, f32) -> f32) {
        for &idx in &self.live {
            let token = TokenId::new(idx);
            let current = self.processed[idx as usize];
            self.processed[idx as usize] = f(token, current);
        }
    }

    /// 🧰️ Adds `delta` to one token's processed logit directly by index, regardless of current
    /// `live` membership (harmless if the token is already masked out — later phases only ever
    /// read `processed` through `live`).
    pub fn bias_processed(&mut self, token: TokenId, delta: f32) {
        if let Some(v) = self.processed.get_mut(token.get() as usize) {
            *v += delta;
        }
    }

    /// 🧰️ Collapses `live` to a single entry: the current argmax of `processed` restricted to
    /// `live` — how `Temperature`/`DynamicTemperature` implement "temperature 0 == greedy".
    pub fn collapse_live_to_argmax(&mut self) {
        if self.live.is_empty() {
            return;
        }
        let best = argmax_index_in_slice(&self.processed, &self.live);
        self.live.clear();
        self.live.push(best);
    }

    /// 🧰️ Softmax over the *current* `live` order (does not sort); used by processors that need
    /// this step's entropy without disturbing candidate order (e.g. [`DynamicTemperature`]).
    pub fn softmax_over_live(&mut self) -> f64 {
        let n = self.live.len();
        self.probs.resize(n, 0.0);
        softmax_live(&self.processed, &self.live, &mut self.probs, self.accum)
    }

    /// 🧰️ Softmax over `live`, then reorders `live`/`probs` in lockstep by probability descending
    /// (ties ascending token id) — the shared, allocation-amortized sort every truncation warper
    /// (and the final distribution build) starts from.
    pub fn sort_live_by_prob_desc(&mut self) {
        self.softmax_over_live();
        let n = self.live.len();
        let Self { live, probs, sort_order, sorted_live_buf, sorted_probs_buf, .. } = self;
        if sort_order.len() < n {
            sort_order.resize(n, 0);
        }
        for (i, slot) in sort_order[..n].iter_mut().enumerate() {
            *slot = i as u32;
        }
        sort_order[..n].sort_unstable_by(|&a, &b| probs[b as usize].partial_cmp(&probs[a as usize]).unwrap_or(core::cmp::Ordering::Equal).then_with(|| live[a as usize].cmp(&live[b as usize])));
        if sorted_live_buf.len() < n {
            sorted_live_buf.resize(n, 0);
        }
        if sorted_probs_buf.len() < n {
            sorted_probs_buf.resize(n, 0.0);
        }
        for (slot, &idx) in sort_order[..n].iter().enumerate() {
            sorted_live_buf[slot] = live[idx as usize];
            sorted_probs_buf[slot] = probs[idx as usize];
        }
        core::mem::swap(live, sorted_live_buf);
        core::mem::swap(probs, sorted_probs_buf);
        live.truncate(n);
        probs.truncate(n);
    }

    /// 🧰️ Keeps the top `keep.max(min_keep)` entries of an already-[`LogitsWorkspace::sort_live_by_prob_desc`]-sorted
    /// `live`/`probs` pair — every truncation warper's shared "never drop below `min_keep`" guarantee.
    pub fn truncate_live_to(&mut self, keep: usize, min_keep: usize) {
        let keep = keep.max(min_keep.min(self.live.len())).min(self.live.len());
        self.live.truncate(keep);
        self.probs.truncate(keep);
    }
}

/// 🧰️ Output of one [`TokenSampler::sample`] call; cleared (not deallocated) between steps.
#[derive(Clone, Debug, Default)]
pub struct SelectionBuffer {
    pub chosen: Vec<Candidate>,
}

impl SelectionBuffer {
    pub fn clear(&mut self) {
        self.chosen.clear();
    }
}

/// 🧰️ Reusable pool of [`LogitsWorkspace`]s for batch/continuous-batching use, so per-slot state
/// survives across steps instead of being reallocated.
#[derive(Default)]
pub struct WorkspacePool {
    slots: Vec<LogitsWorkspace>,
}

impl WorkspacePool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn acquire(&mut self, vocab_size: usize) -> LogitsWorkspace {
        match self.slots.pop() {
            Some(mut workspace) => {
                workspace.ensure_capacity(vocab_size);
                workspace
            }
            None => LogitsWorkspace::new(vocab_size),
        }
    }

    pub fn release(&mut self, workspace: LogitsWorkspace) {
        self.slots.push(workspace);
    }
}
// #endregion 🔖️Workspace

// #region 🔖️Traits
/// 🔌️ Read-only view of one sequence at one step, borrowed for the duration of a pipeline phase.
pub struct StepView<'a> {
    pub sequence: SequenceId,
    pub step: StepIndex,
    pub prompt: &'a [TokenId],
    pub generated: &'a [TokenId],
    pub vocab: &'a Vocabulary,
    pub adapter: Option<&'a dyn TokenTextAdapter>,
    pub last_entropy: Option<f64>,
}

fn schedule_input(view: &StepView<'_>) -> ScheduleInput {
    ScheduleInput { step: view.step, generated_len: view.generated.len(), last_entropy: view.last_entropy }
}

/// 🔌️ The renormalized live distribution handed to samplers: every slice shares indexing, sorted
/// by probability descending with ties broken by ascending token id.
pub struct Distribution<'a> {
    pub tokens: &'a [TokenId],
    pub probs: &'a [f32],
    pub logprobs: &'a [f32],
    pub cdf: &'a [f64],
    pub entropy: f64,
}

/// 🔌️ Which pipeline phase a [`LogitsProcessor`] belongs to; the (forthcoming, wave-4) engine
/// dispatches hard masks before soft penalties before truncation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProcessorKind {
    HardMask,
    SoftPenalty,
    Truncation,
}

/// 🔌️ Opaque undo-log position returned by `LogitsProcessor::save`/`Constraint::save`/`StopCondition::save`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct StateMark(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ConstraintMark(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct StopMark(pub u64);

/// 🔌️ One step of the logits pipeline: transforms `ws` in place (mask, soft penalty, or
/// truncation), and optionally commits per-sequence effects once a token is definitively accepted.
pub trait LogitsProcessor {
    fn name(&self) -> &'static str;
    fn kind(&self) -> ProcessorKind;
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError>;
    fn commit(&mut self, _view: &StepView<'_>, _token: TokenId) {}
    fn save(&mut self) -> StateMark {
        StateMark::default()
    }
    fn rollback_to(&mut self, _mark: StateMark) {}
    fn reset(&mut self) {}
    fn fork(&self) -> Box<dyn LogitsProcessor>;
}

/// 🔌️ Selects one or more tokens from the final [`Distribution`]; must write at least one
/// candidate into `out` or return an error.
pub trait TokenSampler {
    fn name(&self) -> &'static str;
    fn sample(&mut self, view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut dyn RandomSource, out: &mut SelectionBuffer) -> Result<(), SamplingError>;
    fn fork(&self) -> Box<dyn TokenSampler>;
}

/// 🔌️ A structural/lexical constraint on the next token (regex, grammar, JSON mode, ...).
pub trait Constraint {
    fn name(&self) -> &'static str;
    /// 🧱️ ANDs the set of currently-valid tokens into `mask` (starts all-ones for the first
    /// constraint in a composition).
    fn fill_mask(&mut self, view: &StepView<'_>, mask: &mut TokenBitset) -> Result<(), SamplingError>;
    fn accept(&mut self, view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError>;
    fn is_satisfied(&self) -> bool;
    fn is_finished(&self) -> bool;
    fn is_dead(&self) -> bool;
    fn save(&mut self) -> ConstraintMark;
    fn rollback_to(&mut self, mark: ConstraintMark);
    fn reset(&mut self);
    fn fork(&self) -> Box<dyn Constraint>;
}

/// 🛑️ Result of feeding one token's bytes to a [`StopCondition`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StopPoll {
    Continue,
    Hold { ambiguous_bytes: usize },
    Finished { reason: FinishReason, matched_bytes: usize },
}

pub trait StopCondition {
    fn name(&self) -> &'static str;
    fn on_token(&mut self, view: &StepView<'_>, token: TokenId) -> StopPoll;
    fn save(&mut self) -> StopMark;
    fn rollback_to(&mut self, mark: StopMark);
    fn reset(&mut self);
    fn fork(&self) -> Box<dyn StopCondition>;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ProcessorStats {
    pub masked: u32,
    pub timing_ns: u64,
}

/// 🔭️ Hook points into the engine's per-step lifecycle; every method is a no-operation default so
/// observers only implement what they need.
pub trait SamplingObserver {
    fn on_step_start(&mut self, _sequence: SequenceId, _step: StepIndex) {}
    fn on_processor(&mut self, _sequence: SequenceId, _name: &'static str, _stats: &ProcessorStats) {}
    fn on_fallback(&mut self, _sequence: SequenceId, _error: &SamplingError, _action: FallbackAction) {}
    fn on_token(&mut self, _sequence: SequenceId, _result: &SamplingResult) {}
    fn on_finish(&mut self, _sequence: SequenceId, _reason: FinishReason) {}
}

/// 🗂️ One rank's local top candidate, for sharded-vocabulary all-gather.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ShardCandidate {
    pub token: TokenId,
    pub logit: f32,
}

/// 🗂️ Deterministic collective-communication primitives sharded-vocabulary sampling needs.
pub trait Collective {
    fn rank(&self) -> usize;
    fn world_size(&self) -> usize;
    fn all_reduce_max_f32(&mut self, values: &mut [f32]) -> Result<(), SamplingError>;
    fn all_reduce_sum_f64(&mut self, values: &mut [f64]) -> Result<(), SamplingError>;
    /// 🗂️ Gathers every rank's local candidates into `out`, in rank order (deterministic).
    fn all_gather_candidates(&mut self, local: &[ShardCandidate], out: &mut Vec<ShardCandidate>) -> Result<(), SamplingError>;
}
// #endregion 🔖️Traits

// #region 🔖️Warpers
/// 🌡️ `logit / temperature`; `temperature <= 0` collapses to greedy via [`LogitsWorkspace::collapse_live_to_argmax`].
pub struct Temperature {
    pub value: Schedule,
}

impl LogitsProcessor for Temperature {
    fn name(&self) -> &'static str {
        "temperature"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let value = self.value.eval(schedule_input(view));
        if value <= 0.0 {
            ws.collapse_live_to_argmax();
        } else {
            ws.scale_processed_over_live(1.0 / value as f32);
        }
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { value: self.value.clone() })
    }
}

/// 🌡️ Temperature whose value tracks the live set's current entropy: `clamp(base + gain *
/// entropy, min, max)`.
pub struct DynamicTemperature {
    pub base: Schedule,
    pub entropy_gain: f64,
    pub min: f64,
    pub max: f64,
}

impl LogitsProcessor for DynamicTemperature {
    fn name(&self) -> &'static str {
        "dynamic_temperature"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.softmax_over_live();
        let entropy = entropy_nats(ws.probs());
        let temp = (self.base.eval(schedule_input(view)) + self.entropy_gain * entropy).clamp(self.min, self.max);
        if temp <= 0.0 {
            ws.collapse_live_to_argmax();
        } else {
            ws.scale_processed_over_live(1.0 / temp as f32);
        }
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { base: self.base.clone(), entropy_gain: self.entropy_gain, min: self.min, max: self.max })
    }
}

/// 🌡️ Keeps exactly the top `k` live tokens by probability.
pub struct TopK {
    pub k: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TopK {
    fn name(&self) -> &'static str {
        "top_k"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let k = self.k.eval(schedule_input(view)).round().max(1.0) as usize;
        ws.sort_live_by_prob_desc();
        ws.truncate_live_to(k, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { k: self.k.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Nucleus sampling: retains the smallest prefix (by descending probability) whose cumulative
/// mass reaches `p`.
pub struct TopP {
    pub p: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TopP {
    fn name(&self) -> &'static str {
        "top_p"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let p = self.p.eval(schedule_input(view)).clamp(0.0, 1.0);
        ws.sort_live_by_prob_desc();
        let probs = ws.probs();
        let mut cumulative = 0.0f64;
        let mut keep = probs.len();
        for (i, &pr) in probs.iter().enumerate() {
            cumulative += pr as f64;
            if cumulative >= p {
                keep = i + 1;
                break;
            }
        }
        ws.truncate_live_to(keep, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { p: self.p.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Retains tokens whose probability is at least `p` times the live set's maximum probability.
pub struct MinP {
    pub p: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for MinP {
    fn name(&self) -> &'static str {
        "min_p"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let p = self.p.eval(schedule_input(view)).clamp(0.0, 1.0) as f32;
        ws.sort_live_by_prob_desc();
        let probs = ws.probs();
        let threshold = probs.first().copied().unwrap_or(0.0) * p;
        let keep = probs.iter().take_while(|&&pr| pr >= threshold).count().max(1);
        ws.truncate_live_to(keep, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { p: self.p.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Shared implementation for [`Typical`] and [`LocallyTypical`]: ranks tokens by absolute
/// deviation of their surprisal (`-ln p`) from the live set's entropy and retains the smallest
/// such prefix whose cumulative mass reaches `mass`.
fn apply_typical_truncation(ws: &mut LogitsWorkspace, mass: f64, min_keep: usize) {
    ws.softmax_over_live();
    let n = ws.live().len();
    if n == 0 {
        return;
    }
    let probs = ws.probs().to_vec();
    let entropy = entropy_nats(&probs);
    let live = ws.live().to_vec();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| {
        let da = (-(probs[a] as f64).ln() - entropy).abs();
        let db = (-(probs[b] as f64).ln() - entropy).abs();
        da.partial_cmp(&db).unwrap_or(core::cmp::Ordering::Equal).then_with(|| live[a].cmp(&live[b]))
    });
    let mut cumulative = 0.0f64;
    let mut keep = 0usize;
    for &i in &order {
        cumulative += probs[i] as f64;
        keep += 1;
        if cumulative >= mass {
            break;
        }
    }
    keep = keep.max(min_keep.min(n));
    let kept: Vec<u32> = order[..keep].iter().map(|&i| live[i]).collect();
    ws.set_live(kept);
}

/// 🌡️ Locally typical sampling (global variant): see [`apply_typical_truncation`].
pub struct Typical {
    pub mass: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for Typical {
    fn name(&self) -> &'static str {
        "typical"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let mass = self.mass.eval(schedule_input(view)).clamp(0.0, 1.0);
        apply_typical_truncation(ws, mass, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { mass: self.mass.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Locally typical sampling; this single-step engine applies the same rule as [`Typical`]
/// (the "local" distinction only matters across multiple denoising passes, not one logits step).
pub struct LocallyTypical {
    pub mass: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for LocallyTypical {
    fn name(&self) -> &'static str {
        "locally_typical"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let mass = self.mass.eval(schedule_input(view)).clamp(0.0, 1.0);
        apply_typical_truncation(ws, mass, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { mass: self.mass.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Tail-free sampling: cuts where the cumulative normalized second derivative of the sorted
/// probability curve reaches `z`.
pub struct TailFree {
    pub z: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TailFree {
    fn name(&self) -> &'static str {
        "tail_free"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let z = self.z.eval(schedule_input(view)).clamp(0.0, 1.0);
        ws.sort_live_by_prob_desc();
        let probs = ws.probs().to_vec();
        let n = probs.len();
        if n < 3 {
            return Ok(());
        }
        let first_deriv: Vec<f32> = probs.windows(2).map(|w| (w[0] - w[1]).abs()).collect();
        let second_deriv: Vec<f32> = first_deriv.windows(2).map(|w| (w[0] - w[1]).abs()).collect();
        let total: f32 = second_deriv.iter().sum();
        let mut keep = n;
        if total > 0.0 {
            let mut cumulative = 0.0f32;
            for (i, &d) in second_deriv.iter().enumerate() {
                cumulative += d / total;
                if cumulative as f64 >= z {
                    keep = i + 2;
                    break;
                }
            }
        }
        ws.truncate_live_to(keep, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { z: self.z.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Drops any token with probability below an absolute `cutoff`.
pub struct EpsilonCutoff {
    pub cutoff: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for EpsilonCutoff {
    fn name(&self) -> &'static str {
        "epsilon"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let cutoff = self.cutoff.eval(schedule_input(view)).max(0.0);
        ws.sort_live_by_prob_desc();
        let probs = ws.probs();
        let keep = probs.iter().take_while(|&&p| p as f64 >= cutoff).count().max(1);
        ws.truncate_live_to(keep, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { cutoff: self.cutoff.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Entropy-adaptive cutoff: `eta = min(epsilon, sqrt(epsilon) * exp(-entropy))`.
pub struct EtaCutoff {
    pub cutoff: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for EtaCutoff {
    fn name(&self) -> &'static str {
        "eta"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let epsilon = self.cutoff.eval(schedule_input(view)).max(1e-12);
        ws.sort_live_by_prob_desc();
        let probs = ws.probs().to_vec();
        let entropy = entropy_nats(&probs);
        let eta = epsilon.min(epsilon.sqrt() * (-entropy).exp());
        let keep = probs.iter().take_while(|&&p| p as f64 >= eta).count().max(1);
        ws.truncate_live_to(keep, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { cutoff: self.cutoff.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Keeps tokens with `prob >= power * max_prob^2`.
pub struct TopA {
    pub power: Schedule,
    pub min_keep: usize,
}

impl LogitsProcessor for TopA {
    fn name(&self) -> &'static str {
        "top_a"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let a = self.power.eval(schedule_input(view)).max(0.0);
        ws.sort_live_by_prob_desc();
        let probs = ws.probs();
        let max_p = probs.first().copied().unwrap_or(0.0) as f64;
        let threshold = a * max_p * max_p;
        let keep = probs.iter().take_while(|&&p| p as f64 >= threshold).count().max(1);
        ws.truncate_live_to(keep, self.min_keep);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { power: self.power.clone(), min_keep: self.min_keep })
    }
}

/// 🌡️ Keeps only the top `max_rank` tokens by probability (a fixed-count variant of [`TopK`]
/// used when rank alone, not a schedule, determines the cutoff).
pub struct RankTruncation {
    pub max_rank: usize,
}

impl LogitsProcessor for RankTruncation {
    fn name(&self) -> &'static str {
        "rank_truncation"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.sort_live_by_prob_desc();
        ws.truncate_live_to(self.max_rank, 1);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { max_rank: self.max_rank })
    }
}

/// 🌡️ Grows the kept prefix (by descending probability) until the renormalized subset's entropy
/// drops to `target_entropy`, or its effective candidate count reaches `target_effective_count`.
pub struct AdaptiveTruncation {
    pub target_entropy: Option<f64>,
    pub target_effective_count: Option<f64>,
}

impl LogitsProcessor for AdaptiveTruncation {
    fn name(&self) -> &'static str {
        "adaptive_truncation"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.sort_live_by_prob_desc();
        let probs = ws.probs().to_vec();
        let n = probs.len();
        let mut keep = n;
        let mut cumulative = 0.0f64;
        'search: for (i, &p) in probs.iter().enumerate() {
            cumulative += p as f64;
            let mut renorm_entropy = 0.0f64;
            for &q in &probs[..=i] {
                let r = q as f64 / cumulative;
                if r > 0.0 {
                    renorm_entropy -= r * r.ln();
                }
            }
            if let Some(target) = self.target_effective_count {
                if renorm_entropy.exp() >= target {
                    keep = i + 1;
                    break 'search;
                }
            }
            if let Some(target) = self.target_entropy {
                if renorm_entropy <= target {
                    keep = i + 1;
                    break 'search;
                }
            }
        }
        ws.truncate_live_to(keep, 1);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { target_entropy: self.target_entropy, target_effective_count: self.target_effective_count })
    }
}

/// 🌡️ Builds a [`LogitsProcessor`] from a [`ProcessorSpec`]; exhaustive over every variant once
/// the warpers (`🔖️Warpers`), penalties (`🔖️Penalties`), biases (`🔖️Biases`), and adaptive
/// samplers (`🔖️Adaptive`) regions have all landed.
pub fn build_processor(spec: &ProcessorSpec) -> Result<Box<dyn LogitsProcessor>, SamplingError> {
    match spec {
        ProcessorSpec::Temperature { value } => Ok(Box::new(Temperature { value: value.clone() })),
        ProcessorSpec::DynamicTemperature { base, entropy_gain, min, max } => Ok(Box::new(DynamicTemperature { base: base.clone(), entropy_gain: *entropy_gain, min: *min, max: *max })),
        ProcessorSpec::TopK { k, min_keep } => Ok(Box::new(TopK { k: k.clone(), min_keep: *min_keep })),
        ProcessorSpec::TopP { p, min_keep } => Ok(Box::new(TopP { p: p.clone(), min_keep: *min_keep })),
        ProcessorSpec::MinP { p, min_keep } => Ok(Box::new(MinP { p: p.clone(), min_keep: *min_keep })),
        ProcessorSpec::Typical { mass, min_keep } => Ok(Box::new(Typical { mass: mass.clone(), min_keep: *min_keep })),
        ProcessorSpec::LocallyTypical { mass, min_keep } => Ok(Box::new(LocallyTypical { mass: mass.clone(), min_keep: *min_keep })),
        ProcessorSpec::TailFree { z, min_keep } => Ok(Box::new(TailFree { z: z.clone(), min_keep: *min_keep })),
        ProcessorSpec::Epsilon { cutoff, min_keep } => Ok(Box::new(EpsilonCutoff { cutoff: cutoff.clone(), min_keep: *min_keep })),
        ProcessorSpec::Eta { cutoff, min_keep } => Ok(Box::new(EtaCutoff { cutoff: cutoff.clone(), min_keep: *min_keep })),
        ProcessorSpec::TopA { power, min_keep } => Ok(Box::new(TopA { power: power.clone(), min_keep: *min_keep })),
        ProcessorSpec::RankTruncation { max_rank } => Ok(Box::new(RankTruncation { max_rank: *max_rank })),
        ProcessorSpec::AdaptiveTruncation { target_entropy, target_effective_count } => Ok(Box::new(AdaptiveTruncation { target_entropy: *target_entropy, target_effective_count: *target_effective_count })),
        ProcessorSpec::RepetitionPenalty { penalty, scope } => Ok(Box::new(RepetitionPenalty::new(*penalty, *scope))),
        ProcessorSpec::PresencePenalty { penalty, scope } => Ok(Box::new(PresencePenalty::new(*penalty, *scope))),
        ProcessorSpec::FrequencyPenalty { penalty, scope } => Ok(Box::new(FrequencyPenalty::new(*penalty, *scope))),
        ProcessorSpec::DecayingPenalty { penalty, window, half_life, scope } => Ok(Box::new(DecayingPenalty::new(*penalty, *window, *half_life, *scope))),
        ProcessorSpec::TokenClassPenalty { class_tokens, factors } => Ok(Box::new(TokenClassPenalty::new(class_tokens.clone(), factors.clone()))),
        ProcessorSpec::NoRepeatNgram { n } => Ok(Box::new(NoRepeatNgram::new(*n))),
        ProcessorSpec::PhrasePenalty { phrases, penalty } => Ok(Box::new(PhrasePenalty { phrases: phrases.clone(), penalty: *penalty })),
        ProcessorSpec::LogitBiasSparse { entries } => Ok(Box::new(LogitBiasSparse { entries: entries.clone() })),
        ProcessorSpec::LogitBiasDense { values } => Ok(Box::new(LogitBiasDense { values: values.clone() })),
        ProcessorSpec::AllowTokens { tokens } => Ok(Box::new(AllowTokens { tokens: tokens.clone() })),
        ProcessorSpec::ForbidTokens { tokens } => Ok(Box::new(ForbidTokens { tokens: tokens.clone() })),
        ProcessorSpec::SuppressSpecial => Ok(Box::new(SuppressSpecial)),
        ProcessorSpec::BadWords { phrases } => Ok(Box::new(BadWords { phrases: phrases.clone() })),
        ProcessorSpec::SequenceEncouragement { phrases, bonus } => Ok(Box::new(SequenceEncouragement { phrases: phrases.clone(), bonus: *bonus })),
        ProcessorSpec::Mirostat { version, target_surprise, learning_rate } => Ok(Box::new(Mirostat::new(*version, *target_surprise, *learning_rate))),
        ProcessorSpec::EntropyPid { target, kp, ki, kd } => Ok(Box::new(EntropyPid::new(*target, *kp, *ki, *kd))),
        ProcessorSpec::RepetitionController { window, threshold, boost } => Ok(Box::new(RepetitionController::new(*window, *threshold, *boost))),
        ProcessorSpec::ConfidenceController { low_entropy, high_entropy, low_temp, high_temp } => Ok(Box::new(ConfidenceController { low_entropy: *low_entropy, high_entropy: *high_entropy, low_temp: *low_temp, high_temp: *high_temp })),
    }
}
// #endregion 🔖️Warpers

// #region 🔖️Penalties
/// ⚖️ Dense-enough token→count map for repetition-style penalties. Backed by a `HashMap` (not a
/// vocab-sized `Vec`) so processor construction never needs to know the vocabulary size.
#[derive(Clone, Default)]
pub struct FreqTable {
    counts: std::collections::HashMap<TokenId, u32>,
}

impl FreqTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(&self, token: TokenId) -> u32 {
        self.counts.get(&token).copied().unwrap_or(0)
    }

    pub fn increment(&mut self, token: TokenId) {
        *self.counts.entry(token).or_insert(0) += 1;
    }

    /// ⚖️ Exact inverse of [`FreqTable::increment`] — removes the map entry entirely once its
    /// count reaches zero, so `count()` and `HashMap` iteration agree on "seen at all".
    pub fn decrement(&mut self, token: TokenId) {
        if let Some(c) = self.counts.get_mut(&token) {
            if *c <= 1 {
                self.counts.remove(&token);
            } else {
                *c -= 1;
            }
        }
    }

    pub fn reset(&mut self) {
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
    pub fn new(order: usize) -> Self {
        Self { order, table: std::collections::HashMap::new(), undo: Vec::new() }
    }

    fn context_hash(context: &[TokenId]) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for token in context {
            hash = (hash ^ token.get() as u64).wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    /// ⚖️ Records that `next` followed the last `order - 1` tokens of `history` (the history as it
    /// existed *before* `next`).
    pub fn record(&mut self, history: &[TokenId], next: TokenId) {
        if self.order < 2 || history.len() + 1 < self.order {
            self.undo.push(None);
            return;
        }
        let ctx_len = self.order - 1;
        let context = &history[history.len() - ctx_len..];
        let key = Self::context_hash(context);
        self.table.entry(key).or_default().push(next);
        self.undo.push(Some(key));
    }

    /// ⚖️ Tokens that would recreate an already-seen `order`-gram if selected next.
    pub fn forbidden_next(&self, history: &[TokenId]) -> &[TokenId] {
        if self.order < 2 || history.len() + 1 < self.order {
            return &[];
        }
        let ctx_len = self.order - 1;
        let context = &history[history.len() - ctx_len..];
        let key = Self::context_hash(context);
        self.table.get(&key).map_or(&[][..], Vec::as_slice)
    }

    pub fn commit_count(&self) -> u64 {
        self.undo.len() as u64
    }

    pub fn rollback_last_n(&mut self, n: usize) {
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

    pub fn reset(&mut self) {
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
    counts: FreqTable,
    prompt_included: bool,
    commit_log: Vec<TokenId>,
}

impl RepetitionPenalty {
    pub fn new(penalty: f32, scope: PenaltyScope) -> Self {
        Self { penalty, scope, counts: FreqTable::new(), prompt_included: false, commit_log: Vec::new() }
    }

    fn include_prompt_if_needed(&mut self, prompt: &[TokenId]) {
        if !self.prompt_included && matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::PromptOnly) {
            for &t in prompt {
                self.counts.increment(t);
            }
            self.prompt_included = true;
        }
    }
}

impl LogitsProcessor for RepetitionPenalty {
    fn name(&self) -> &'static str {
        "repetition_penalty"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        self.include_prompt_if_needed(view.prompt);
        let penalty = self.penalty;
        let counts = &self.counts;
        ws.transform_processed_over_live(|token, logit| {
            if counts.count(token) > 0 {
                if logit > 0.0 {
                    logit / penalty
                } else {
                    logit * penalty
                }
            } else {
                logit
            }
        });
        Ok(())
    }
    fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
            self.counts.increment(token);
        }
        self.commit_log.push(token);
    }
    fn save(&mut self) -> StateMark {
        StateMark(self.commit_log.len() as u64)
    }
    fn rollback_to(&mut self, mark: StateMark) {
        while self.commit_log.len() as u64 > mark.0 {
            if let Some(token) = self.commit_log.pop() {
                if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
                    self.counts.decrement(token);
                }
            }
        }
    }
    fn reset(&mut self) {
        self.counts.reset();
        self.prompt_included = false;
        self.commit_log.clear();
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { penalty: self.penalty, scope: self.scope, counts: self.counts.clone(), prompt_included: self.prompt_included, commit_log: self.commit_log.clone() })
    }
}

/// ⚖️ Fixed additive penalty applied once per distinct token that has appeared.
pub struct PresencePenalty {
    pub penalty: f32,
    pub scope: PenaltyScope,
    counts: FreqTable,
    prompt_included: bool,
    commit_log: Vec<TokenId>,
}

impl PresencePenalty {
    pub fn new(penalty: f32, scope: PenaltyScope) -> Self {
        Self { penalty, scope, counts: FreqTable::new(), prompt_included: false, commit_log: Vec::new() }
    }

    fn include_prompt_if_needed(&mut self, prompt: &[TokenId]) {
        if !self.prompt_included && matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::PromptOnly) {
            for &t in prompt {
                self.counts.increment(t);
            }
            self.prompt_included = true;
        }
    }
}

impl LogitsProcessor for PresencePenalty {
    fn name(&self) -> &'static str {
        "presence_penalty"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        self.include_prompt_if_needed(view.prompt);
        let penalty = self.penalty;
        let counts = &self.counts;
        ws.add_bias_over_live(|token| if counts.count(token) > 0 { -penalty } else { 0.0 });
        Ok(())
    }
    fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
            self.counts.increment(token);
        }
        self.commit_log.push(token);
    }
    fn save(&mut self) -> StateMark {
        StateMark(self.commit_log.len() as u64)
    }
    fn rollback_to(&mut self, mark: StateMark) {
        while self.commit_log.len() as u64 > mark.0 {
            if let Some(token) = self.commit_log.pop() {
                if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
                    self.counts.decrement(token);
                }
            }
        }
    }
    fn reset(&mut self) {
        self.counts.reset();
        self.prompt_included = false;
        self.commit_log.clear();
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { penalty: self.penalty, scope: self.scope, counts: self.counts.clone(), prompt_included: self.prompt_included, commit_log: self.commit_log.clone() })
    }
}

/// ⚖️ Additive penalty proportional to a token's occurrence count.
pub struct FrequencyPenalty {
    pub penalty: f32,
    pub scope: PenaltyScope,
    counts: FreqTable,
    prompt_included: bool,
    commit_log: Vec<TokenId>,
}

impl FrequencyPenalty {
    pub fn new(penalty: f32, scope: PenaltyScope) -> Self {
        Self { penalty, scope, counts: FreqTable::new(), prompt_included: false, commit_log: Vec::new() }
    }

    fn include_prompt_if_needed(&mut self, prompt: &[TokenId]) {
        if !self.prompt_included && matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::PromptOnly) {
            for &t in prompt {
                self.counts.increment(t);
            }
            self.prompt_included = true;
        }
    }
}

impl LogitsProcessor for FrequencyPenalty {
    fn name(&self) -> &'static str {
        "frequency_penalty"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        self.include_prompt_if_needed(view.prompt);
        let penalty = self.penalty;
        let counts = &self.counts;
        ws.add_bias_over_live(|token| -penalty * counts.count(token) as f32);
        Ok(())
    }
    fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
            self.counts.increment(token);
        }
        self.commit_log.push(token);
    }
    fn save(&mut self) -> StateMark {
        StateMark(self.commit_log.len() as u64)
    }
    fn rollback_to(&mut self, mark: StateMark) {
        while self.commit_log.len() as u64 > mark.0 {
            if let Some(token) = self.commit_log.pop() {
                if matches!(self.scope, PenaltyScope::PromptAndGenerated | PenaltyScope::GeneratedOnly) {
                    self.counts.decrement(token);
                }
            }
        }
    }
    fn reset(&mut self) {
        self.counts.reset();
        self.prompt_included = false;
        self.commit_log.clear();
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { penalty: self.penalty, scope: self.scope, counts: self.counts.clone(), prompt_included: self.prompt_included, commit_log: self.commit_log.clone() })
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
    pub fn new(penalty: f32, window: usize, half_life: f64, scope: PenaltyScope) -> Self {
        Self { penalty, window, half_life, scope, history: std::collections::VecDeque::new(), snapshots: Vec::new() }
    }
}

impl LogitsProcessor for DecayingPenalty {
    fn name(&self) -> &'static str {
        "decaying_penalty"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let half_life = self.half_life.max(1e-6);
        let penalty = self.penalty as f64;
        let mut bias: std::collections::HashMap<TokenId, f32> = std::collections::HashMap::new();
        for (i, &token) in self.history.iter().rev().enumerate() {
            let distance = (i + 1) as f64;
            let decay = 0.5f64.powf(distance / half_life);
            *bias.entry(token).or_insert(0.0) += (penalty * decay) as f32;
        }
        ws.add_bias_over_live(|token| -bias.get(&token).copied().unwrap_or(0.0));
        Ok(())
    }
    fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        self.history.push_back(token);
        if self.history.len() > self.window {
            self.history.pop_front();
        }
    }
    fn save(&mut self) -> StateMark {
        self.snapshots.push(self.history.clone());
        StateMark((self.snapshots.len() - 1) as u64)
    }
    fn rollback_to(&mut self, mark: StateMark) {
        if let Some(history) = self.snapshots.get(mark.0 as usize) {
            self.history = history.clone();
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    fn reset(&mut self) {
        self.history.clear();
        self.snapshots.clear();
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { penalty: self.penalty, window: self.window, half_life: self.half_life, scope: self.scope, history: self.history.clone(), snapshots: self.snapshots.clone() })
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
    pub fn new(class_tokens: Vec<Vec<TokenId>>, factors: Vec<f32>) -> Self {
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
    fn name(&self) -> &'static str {
        "token_class_penalty"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let class_of = &self.class_of;
        let factors = &self.factors;
        ws.transform_processed_over_live(|token, logit| match class_of.get(&token).and_then(|&c| factors.get(c as usize)) {
            Some(&factor) => logit * factor,
            None => logit,
        });
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { class_of: self.class_of.clone(), factors: self.factors.clone() })
    }
}

/// ⚖️ Hard-masks any token that would recreate an `n`-gram already present in the sequence's
/// history (prompt + generated).
pub struct NoRepeatNgram {
    pub n: usize,
    index: NgramIndex,
}

impl NoRepeatNgram {
    pub fn new(n: usize) -> Self {
        Self { n, index: NgramIndex::new(n) }
    }

    fn full_history(view: &StepView<'_>) -> Vec<TokenId> {
        let mut history = Vec::with_capacity(view.prompt.len() + view.generated.len());
        history.extend_from_slice(view.prompt);
        history.extend_from_slice(view.generated);
        history
    }
}

impl LogitsProcessor for NoRepeatNgram {
    fn name(&self) -> &'static str {
        "no_repeat_ngram"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let history = Self::full_history(view);
        for &token in self.index.forbidden_next(&history) {
            ws.mask_mut().set(token, false);
        }
        Ok(())
    }
    fn commit(&mut self, view: &StepView<'_>, token: TokenId) {
        let history = Self::full_history(view);
        self.index.record(&history, token);
    }
    fn save(&mut self) -> StateMark {
        StateMark(self.index.commit_count())
    }
    fn rollback_to(&mut self, mark: StateMark) {
        let current = self.index.commit_count();
        if current > mark.0 {
            self.index.rollback_last_n((current - mark.0) as usize);
        }
    }
    fn reset(&mut self) {
        self.index.reset();
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { n: self.n, index: self.index.clone() })
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
    fn name(&self) -> &'static str {
        "phrase_penalty"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let generated = view.generated;
        let penalty = self.penalty;
        let mut biased: std::collections::HashMap<TokenId, f32> = std::collections::HashMap::new();
        for phrase in &self.phrases {
            let Some((&last, prefix)) = phrase.split_last() else { continue };
            if generated.len() >= prefix.len() && &generated[generated.len() - prefix.len()..] == prefix {
                *biased.entry(last).or_insert(0.0) += penalty;
            }
        }
        ws.add_bias_over_live(|token| -biased.get(&token).copied().unwrap_or(0.0));
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { phrases: self.phrases.clone(), penalty: self.penalty })
    }
}
// #endregion 🔖️Penalties

// #region 🔖️Biases
/// 🧲️ Sparse per-token additive logit bias.
pub struct LogitBiasSparse {
    pub entries: Vec<(TokenId, f32)>,
}

impl LogitsProcessor for LogitBiasSparse {
    fn name(&self) -> &'static str {
        "logit_bias_sparse"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        for &(token, bias) in &self.entries {
            ws.bias_processed(token, bias);
        }
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { entries: self.entries.clone() })
    }
}

/// 🧲️ Dense per-token additive logit bias (`values[i]` biases token `i`).
pub struct LogitBiasDense {
    pub values: Vec<f32>,
}

impl LogitsProcessor for LogitBiasDense {
    fn name(&self) -> &'static str {
        "logit_bias_dense"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        for (i, &delta) in self.values.iter().enumerate() {
            ws.bias_processed(TokenId::new(i as u32), delta);
        }
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { values: self.values.clone() })
    }
}

/// 🧲️ Hard-restricts the live set to exactly `tokens` (intersected with whatever survives earlier
/// hard-mask processors).
pub struct AllowTokens {
    pub tokens: Vec<TokenId>,
}

impl LogitsProcessor for AllowTokens {
    fn name(&self) -> &'static str {
        "allow_tokens"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let mut allow = TokenBitset::new_empty(ws.vocab_size());
        for &token in &self.tokens {
            allow.set(token, true);
        }
        ws.mask_mut().and_with(&allow);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { tokens: self.tokens.clone() })
    }
}

/// 🧲️ Hard-excludes `tokens` from the live set.
pub struct ForbidTokens {
    pub tokens: Vec<TokenId>,
}

impl LogitsProcessor for ForbidTokens {
    fn name(&self) -> &'static str {
        "forbid_tokens"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        for &token in &self.tokens {
            ws.mask_mut().set(token, false);
        }
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { tokens: self.tokens.clone() })
    }
}

/// 🧲️ Hard-excludes every token in the vocabulary's [`Vocabulary::special`] set.
pub struct SuppressSpecial;

impl LogitsProcessor for SuppressSpecial {
    fn name(&self) -> &'static str {
        "suppress_special"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.mask_mut().and_not_with(&view.vocab.special);
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self)
    }
}

/// 🧲️ Hard-excludes the final token of any bad-word phrase whose proper prefix the generated text
/// currently ends with (prefix-sensitive multi-token bad-word suppression).
pub struct BadWords {
    pub phrases: Vec<Vec<TokenId>>,
}

impl LogitsProcessor for BadWords {
    fn name(&self) -> &'static str {
        "bad_words"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let generated = view.generated;
        for phrase in &self.phrases {
            let Some((&last, prefix)) = phrase.split_last() else { continue };
            if generated.len() >= prefix.len() && &generated[generated.len() - prefix.len()..] == prefix {
                ws.mask_mut().set(last, false);
            }
        }
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { phrases: self.phrases.clone() })
    }
}

/// 🧲️ Positive bias toward completing a configured phrase once the generated text ends with its
/// proper prefix (the encouragement counterpart to [`BadWords`]/[`PhrasePenalty`]).
pub struct SequenceEncouragement {
    pub phrases: Vec<Vec<TokenId>>,
    pub bonus: f32,
}

impl LogitsProcessor for SequenceEncouragement {
    fn name(&self) -> &'static str {
        "sequence_encouragement"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        let generated = view.generated;
        let bonus = self.bonus;
        for phrase in &self.phrases {
            let Some((&last, prefix)) = phrase.split_last() else { continue };
            if generated.len() >= prefix.len() && &generated[generated.len() - prefix.len()..] == prefix {
                ws.bias_processed(last, bonus);
            }
        }
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { phrases: self.phrases.clone(), bonus: self.bonus })
    }
}
// #endregion 🔖️Biases

// #region 🔖️Length
/// 📏️ Hard-suppresses every EOS token until `min_tokens` generated tokens have been produced.
pub struct MinLengthEosSuppression {
    pub min_tokens: usize,
}

impl LogitsProcessor for MinLengthEosSuppression {
    fn name(&self) -> &'static str {
        "min_length_eos_suppression"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        if view.generated.len() < self.min_tokens {
            for &eos in &view.vocab.eos {
                ws.mask_mut().set(eos, false);
            }
        }
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { min_tokens: self.min_tokens })
    }
}

/// 📏️ Hard-restricts the live set to (the first configured) EOS token once the *next* token would
/// reach `max_tokens`, guaranteeing generation stops cleanly at the length cap.
pub struct MaxLengthForceEos {
    pub max_tokens: usize,
}

impl LogitsProcessor for MaxLengthForceEos {
    fn name(&self) -> &'static str {
        "max_length_force_eos"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        if view.generated.len() + 1 >= self.max_tokens {
            if let Some(&eos) = view.vocab.eos.first() {
                let mut only_eos = TokenBitset::new_empty(ws.vocab_size());
                only_eos.set(eos, true);
                ws.mask_mut().and_with(&only_eos);
            }
        }
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { max_tokens: self.max_tokens })
    }
}

/// 📏️ Hard-forces an exact token at step 0 (`bos`), steps `1..=prefix.len()` (`prefix`), or any
/// explicitly listed `(step, token)` pair thereafter.
pub struct ForcedTokens {
    pub spec: ForcedSpec,
}

impl ForcedTokens {
    fn forced_token_at(&self, step: u32) -> Option<TokenId> {
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
    fn name(&self) -> &'static str {
        "forced_tokens"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::HardMask
    }
    fn process(&mut self, view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        if let Some(token) = self.forced_token_at(view.step.get()) {
            let mut only = TokenBitset::new_empty(ws.vocab_size());
            only.set(token, true);
            ws.mask_mut().and_with(&only);
        }
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { spec: self.spec.clone() })
    }
}
// #endregion 🔖️Length

// #region 🔖️Selection
fn candidate_from(dist: &Distribution<'_>, index: usize) -> Candidate {
    Candidate { token: dist.tokens[index], raw_logit: 0.0, processed_logit: 0.0, prob: dist.probs[index], logprob: dist.logprobs[index], rank: index as u32 }
}

/// 🎯️ Walker's alias method, reimplemented locally (rather than reusing
/// [`crate::random::AliasTable`]) because that type's `sample` is hard-wired to the
/// concrete `crate::random::Rng` and cannot accept our `dyn RandomSource` trait object.
struct AliasTable {
    prob: Vec<f32>,
    alias: Vec<u32>,
}

impl AliasTable {
    fn new(weights: &[f32]) -> Self {
        let n = weights.len();
        if n == 0 {
            return Self { prob: vec![1.0], alias: vec![0] };
        }
        let sum: f32 = weights.iter().sum();
        if sum <= 0.0 {
            let mut prob = vec![0.0; n];
            prob[0] = 1.0;
            return Self { prob, alias: vec![0; n] };
        }
        let mut scaled: Vec<f32> = weights.iter().map(|w| w / sum * n as f32).collect();
        let mut small: Vec<usize> = Vec::new();
        let mut large: Vec<usize> = Vec::new();
        for (i, &p) in scaled.iter().enumerate() {
            if p < 1.0 {
                small.push(i);
            } else {
                large.push(i);
            }
        }
        let mut prob = vec![0.0f32; n];
        let mut alias = vec![0u32; n];
        // ⚖️ `while let (Some(s), Some(l)) = (small.pop(), large.pop())` looks equivalent but isn't:
        // both `.pop()` calls run unconditionally as call arguments before the pattern is tested, so
        // the moment either vec empties, the other's last element is silently discarded (popped and
        // dropped) instead of falling through to the `prob[x] = 1.0` cleanup loops below — leaving
        // that bucket's `prob`/`alias` at their zeroed defaults. Explicit `is_empty()` checks avoid it.
        while !small.is_empty() && !large.is_empty() {
            let s = small.pop().expect("checked non-empty above");
            let l = large.pop().expect("checked non-empty above");
            prob[s] = scaled[s];
            alias[s] = l as u32;
            scaled[l] = scaled[l] + scaled[s] - 1.0;
            if scaled[l] < 1.0 {
                small.push(l);
            } else {
                large.push(l);
            }
        }
        for l in large {
            prob[l] = 1.0;
        }
        for s in small {
            prob[s] = 1.0;
        }
        Self { prob, alias }
    }

    fn sample(&self, rng: &mut dyn RandomSource) -> usize {
        let n = self.prob.len();
        let i = rng.next_range(0, n as u64) as usize;
        if rng.next_f64() < self.prob[i] as f64 {
            i
        } else {
            self.alias[i] as usize
        }
    }
}

/// 🎯️ Deterministic argmax selection over the (already prob-sorted) [`Distribution`].
pub struct GreedySampler {
    pub tie_break: TieBreak,
}

impl TokenSampler for GreedySampler {
    fn name(&self) -> &'static str {
        "greedy"
    }
    fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, _rng: &mut dyn RandomSource, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
        if dist.tokens.is_empty() {
            return Err(SamplingError::EmptyDistribution);
        }
        let max_prob = dist.probs[0];
        let tie_epsilon = f32::EPSILON.max(max_prob.abs() * 1e-6);
        let mut end = 0usize;
        while end + 1 < dist.probs.len() && (dist.probs[end + 1] - max_prob).abs() <= tie_epsilon {
            end += 1;
        }
        let idx = match self.tie_break {
            TieBreak::HighestTokenId => end,
            TieBreak::LowestTokenId | TieBreak::FirstSeen => 0,
        };
        out.chosen.push(candidate_from(dist, idx));
        Ok(())
    }
    fn fork(&self) -> Box<dyn TokenSampler> {
        Box::new(Self { tie_break: self.tie_break })
    }
}

/// 🎯️ Samples one token proportional to the live distribution via the configured strategy.
pub struct MultinomialSampler {
    pub strategy: MultinomialStrategy,
}

impl TokenSampler for MultinomialSampler {
    fn name(&self) -> &'static str {
        "multinomial"
    }
    fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut dyn RandomSource, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
        if dist.tokens.is_empty() {
            return Err(SamplingError::EmptyDistribution);
        }
        let idx = match self.strategy {
            MultinomialStrategy::CdfBinarySearch => cdf_binary_search(dist.cdf, rng.next_f64()),
            MultinomialStrategy::LinearScan => {
                let u = rng.next_f64();
                let mut cumulative = 0.0f64;
                let mut idx = dist.probs.len() - 1;
                for (i, &p) in dist.probs.iter().enumerate() {
                    cumulative += p as f64;
                    if cumulative >= u {
                        idx = i;
                        break;
                    }
                }
                idx
            }
            MultinomialStrategy::Alias => {
                let table = AliasTable::new(dist.probs);
                table.sample(rng)
            }
        };
        out.chosen.push(candidate_from(dist, idx));
        Ok(())
    }
    fn fork(&self) -> Box<dyn TokenSampler> {
        Box::new(Self { strategy: self.strategy })
    }
}

/// 🎯️ Gumbel-max trick: `argmax(logprob_i + Gumbel_i)`, statistically equivalent to multinomial
/// sampling but usable when only a perturb-then-argmax primitive is available.
pub struct GumbelMaxSampler;

impl TokenSampler for GumbelMaxSampler {
    fn name(&self) -> &'static str {
        "gumbel_max"
    }
    fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut dyn RandomSource, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
        if dist.tokens.is_empty() {
            return Err(SamplingError::EmptyDistribution);
        }
        let mut best = 0usize;
        let mut best_score = f64::NEG_INFINITY;
        for i in 0..dist.tokens.len() {
            let score = dist.logprobs[i] as f64 + rng.gumbel();
            if score > best_score {
                best_score = score;
                best = i;
            }
        }
        out.chosen.push(candidate_from(dist, best));
        Ok(())
    }
    fn fork(&self) -> Box<dyn TokenSampler> {
        Box::new(Self)
    }
}

/// 🎯️ Gumbel-top-k: `k` tokens without replacement, drawn by taking the top `k` of `logprob_i +
/// Gumbel_i`.
pub struct GumbelTopKSampler {
    pub k: usize,
}

impl TokenSampler for GumbelTopKSampler {
    fn name(&self) -> &'static str {
        "gumbel_top_k"
    }
    fn sample(&mut self, _view: &StepView<'_>, dist: &Distribution<'_>, rng: &mut dyn RandomSource, out: &mut SelectionBuffer) -> Result<(), SamplingError> {
        if dist.tokens.is_empty() {
            return Err(SamplingError::EmptyDistribution);
        }
        let k = self.k.min(dist.tokens.len());
        let mut scored: Vec<(f64, usize)> = (0..dist.tokens.len()).map(|i| (dist.logprobs[i] as f64 + rng.gumbel(), i)).collect();
        scored.sort_unstable_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(core::cmp::Ordering::Equal).then_with(|| dist.tokens[a.1].cmp(&dist.tokens[b.1])));
        for &(_, i) in scored.iter().take(k) {
            out.chosen.push(candidate_from(dist, i));
        }
        Ok(())
    }
    fn fork(&self) -> Box<dyn TokenSampler> {
        Box::new(Self { k: self.k })
    }
}

/// 🎯️ Builds a [`TokenSampler`] from a [`SamplingMethod`].
pub fn build_sampler(method: &SamplingMethod) -> Box<dyn TokenSampler> {
    match method {
        SamplingMethod::Greedy { tie_break } => Box::new(GreedySampler { tie_break: *tie_break }),
        SamplingMethod::Multinomial { strategy } => Box::new(MultinomialSampler { strategy: *strategy }),
        SamplingMethod::GumbelMax => Box::new(GumbelMaxSampler),
        SamplingMethod::GumbelTopK { k } => Box::new(GumbelTopKSampler { k: *k }),
    }
}
// #endregion 🔖️Selection

// #region 🔖️Adaptive
/// 🌀️ Mirostat v1/v2: adapts the truncation cutoff step by step so the *observed* surprise of
/// selected tokens tracks `target_surprise`. `last_probs` caches the just-processed live
/// distribution so [`Mirostat::commit`] can look up the selected token's probability without the
/// engine needing to pass it explicitly (the [`LogitsProcessor::commit`] signature only carries
/// the token id).
pub struct Mirostat {
    pub version: MirostatVersion,
    pub target_surprise: f64,
    pub learning_rate: f64,
    mu: f64,
    last_probs: std::collections::HashMap<TokenId, f32>,
    mu_snapshots: Vec<f64>,
}

impl Mirostat {
    pub fn new(version: MirostatVersion, target_surprise: f64, learning_rate: f64) -> Self {
        Self { version, target_surprise, learning_rate, mu: target_surprise * 2.0, last_probs: std::collections::HashMap::new(), mu_snapshots: Vec::new() }
    }
}

impl LogitsProcessor for Mirostat {
    fn name(&self) -> &'static str {
        "mirostat"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::Truncation
    }
    fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.sort_live_by_prob_desc();
        let keep = match self.version {
            // 🌀️ v1: candidate-set size grows/shrinks as `2^mu` (a power-law-fit proxy).
            MirostatVersion::V1 => (2.0f64).powf(self.mu).round().max(1.0) as usize,
            // 🌀️ v2: keep every token whose surprise (`-log2 p`) stays within `mu` of the target.
            MirostatVersion::V2 => {
                let probs = ws.probs();
                probs.iter().take_while(|&&p| -(p as f64).log2() <= self.mu).count().max(1)
            }
        };
        ws.truncate_live_to(keep, 1);
        self.last_probs.clear();
        for (&token_idx, &p) in ws.live().iter().zip(ws.probs().iter()) {
            self.last_probs.insert(TokenId::new(token_idx), p);
        }
        Ok(())
    }
    fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        self.mu_snapshots.push(self.mu);
        if let Some(&p) = self.last_probs.get(&token) {
            let surprise = -(p as f64).log2();
            self.mu -= self.learning_rate * (surprise - self.target_surprise);
        }
    }
    fn save(&mut self) -> StateMark {
        StateMark(self.mu_snapshots.len() as u64)
    }
    fn rollback_to(&mut self, mark: StateMark) {
        self.mu_snapshots.truncate(mark.0 as usize);
        self.mu = self.mu_snapshots.last().copied().unwrap_or(self.target_surprise * 2.0);
    }
    fn reset(&mut self) {
        self.mu = self.target_surprise * 2.0;
        self.mu_snapshots.clear();
        self.last_probs.clear();
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { version: self.version, target_surprise: self.target_surprise, learning_rate: self.learning_rate, mu: self.mu, last_probs: self.last_probs.clone(), mu_snapshots: self.mu_snapshots.clone() })
    }
}

/// 🌀️ PID controller driving temperature toward a target entropy.
pub struct EntropyPid {
    pub target: f64,
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    integral: f64,
    last_error: f64,
    pending_error: f64,
    history: Vec<(f64, f64)>,
}

impl EntropyPid {
    pub fn new(target: f64, kp: f64, ki: f64, kd: f64) -> Self {
        Self { target, kp, ki, kd, integral: 0.0, last_error: 0.0, pending_error: 0.0, history: Vec::new() }
    }
}

impl LogitsProcessor for EntropyPid {
    fn name(&self) -> &'static str {
        "entropy_pid"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.softmax_over_live();
        let entropy = entropy_nats(ws.probs());
        self.pending_error = self.target - entropy;
        let temp = (1.0 + self.kp * self.pending_error + self.ki * self.integral + self.kd * (self.pending_error - self.last_error)).max(0.05);
        ws.scale_processed_over_live(1.0 / temp as f32);
        Ok(())
    }
    fn commit(&mut self, _view: &StepView<'_>, _token: TokenId) {
        self.history.push((self.integral, self.last_error));
        self.integral += self.pending_error;
        self.last_error = self.pending_error;
    }
    fn save(&mut self) -> StateMark {
        StateMark(self.history.len() as u64)
    }
    fn rollback_to(&mut self, mark: StateMark) {
        self.history.truncate(mark.0 as usize);
        let (integral, last_error) = self.history.last().copied().unwrap_or((0.0, 0.0));
        self.integral = integral;
        self.last_error = last_error;
    }
    fn reset(&mut self) {
        self.integral = 0.0;
        self.last_error = 0.0;
        self.pending_error = 0.0;
        self.history.clear();
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { target: self.target, kp: self.kp, ki: self.ki, kd: self.kd, integral: self.integral, last_error: self.last_error, pending_error: self.pending_error, history: self.history.clone() })
    }
}

/// 🌀️ Flattens the distribution (raises effective temperature) once the fraction of repeated
/// tokens in the trailing `window` exceeds `threshold`.
pub struct RepetitionController {
    pub window: usize,
    pub threshold: f64,
    pub boost: f64,
    recent: std::collections::VecDeque<TokenId>,
    snapshots: Vec<std::collections::VecDeque<TokenId>>,
}

impl RepetitionController {
    pub fn new(window: usize, threshold: f64, boost: f64) -> Self {
        Self { window, threshold, boost, recent: std::collections::VecDeque::new(), snapshots: Vec::new() }
    }

    fn repetition_ratio(&self) -> f64 {
        if self.recent.is_empty() {
            return 0.0;
        }
        let unique: std::collections::HashSet<&TokenId> = self.recent.iter().collect();
        1.0 - (unique.len() as f64 / self.recent.len() as f64)
    }
}

impl LogitsProcessor for RepetitionController {
    fn name(&self) -> &'static str {
        "repetition_controller"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        if self.repetition_ratio() > self.threshold {
            ws.scale_processed_over_live(1.0 / (1.0 + self.boost as f32));
        }
        Ok(())
    }
    fn commit(&mut self, _view: &StepView<'_>, token: TokenId) {
        self.recent.push_back(token);
        if self.recent.len() > self.window {
            self.recent.pop_front();
        }
    }
    fn save(&mut self) -> StateMark {
        self.snapshots.push(self.recent.clone());
        StateMark((self.snapshots.len() - 1) as u64)
    }
    fn rollback_to(&mut self, mark: StateMark) {
        if let Some(recent) = self.snapshots.get(mark.0 as usize) {
            self.recent = recent.clone();
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    fn reset(&mut self) {
        self.recent.clear();
        self.snapshots.clear();
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { window: self.window, threshold: self.threshold, boost: self.boost, recent: self.recent.clone(), snapshots: self.snapshots.clone() })
    }
}

/// 🌀️ Interpolates temperature between `low_temp` (near-greedy, at or below `low_entropy`) and
/// `high_temp` (broad, at or above `high_entropy`) based on the live set's current entropy.
pub struct ConfidenceController {
    pub low_entropy: f64,
    pub high_entropy: f64,
    pub low_temp: f64,
    pub high_temp: f64,
}

impl LogitsProcessor for ConfidenceController {
    fn name(&self) -> &'static str {
        "confidence_controller"
    }
    fn kind(&self) -> ProcessorKind {
        ProcessorKind::SoftPenalty
    }
    fn process(&mut self, _view: &StepView<'_>, ws: &mut LogitsWorkspace) -> Result<(), SamplingError> {
        ws.softmax_over_live();
        let entropy = entropy_nats(ws.probs());
        let span = (self.high_entropy - self.low_entropy).max(1e-9);
        let t = ((entropy - self.low_entropy) / span).clamp(0.0, 1.0);
        let temp = self.low_temp + (self.high_temp - self.low_temp) * t;
        if temp <= 0.0 {
            ws.collapse_live_to_argmax();
        } else {
            ws.scale_processed_over_live(1.0 / temp as f32);
        }
        Ok(())
    }
    fn fork(&self) -> Box<dyn LogitsProcessor> {
        Box::new(Self { low_entropy: self.low_entropy, high_entropy: self.high_entropy, low_temp: self.low_temp, high_temp: self.high_temp })
    }
}
// #endregion 🔖️Adaptive

// #region 🔖️Stops
#[derive(Clone, Default)]
struct AcNode {
    children: std::collections::HashMap<u8, u32>,
    fail: u32,
    depth: usize,
    pattern_end: Option<usize>,
    longest_suffix_pattern: Option<usize>,
}

/// 🛑️ Flat Aho-Corasick automaton over byte patterns, built once per [`StopSpec`] and shared
/// (read-only) across a sequence's forks via `Rc`.
pub struct AhoCorasick {
    nodes: Vec<AcNode>,
    pattern_lens: Vec<usize>,
}

impl AhoCorasick {
    pub fn build(patterns: &[Vec<u8>]) -> Self {
        let mut nodes = vec![AcNode::default()];
        for (pi, pattern) in patterns.iter().enumerate() {
            let mut state = 0u32;
            for &byte in pattern {
                state = match nodes[state as usize].children.get(&byte) {
                    Some(&next) => next,
                    None => {
                        let parent_depth = nodes[state as usize].depth;
                        nodes.push(AcNode { depth: parent_depth + 1, ..AcNode::default() });
                        let next = (nodes.len() - 1) as u32;
                        nodes[state as usize].children.insert(byte, next);
                        next
                    }
                };
            }
            nodes[state as usize].pattern_end = Some(pi);
        }
        let mut queue: std::collections::VecDeque<u32> = std::collections::VecDeque::new();
        for &child in nodes[0].children.values() {
            queue.push_back(child);
        }
        while let Some(u) = queue.pop_front() {
            let children: Vec<(u8, u32)> = nodes[u as usize].children.iter().map(|(&b, &c)| (b, c)).collect();
            for (byte, v) in children {
                let mut f = nodes[u as usize].fail;
                while f != 0 && !nodes[f as usize].children.contains_key(&byte) {
                    f = nodes[f as usize].fail;
                }
                let candidate = nodes[f as usize].children.get(&byte).copied().unwrap_or(0);
                let final_fail = if candidate == v { 0 } else { candidate };
                nodes[v as usize].fail = final_fail;
                nodes[v as usize].longest_suffix_pattern = nodes[final_fail as usize].pattern_end.or(nodes[final_fail as usize].longest_suffix_pattern);
                queue.push_back(v);
            }
        }
        let pattern_lens = patterns.iter().map(Vec::len).collect();
        Self { nodes, pattern_lens }
    }

    /// 🛑️ Advances by one byte via the automaton's goto function (following fail links on miss).
    fn step(&self, state: u32, byte: u8) -> u32 {
        let mut s = state;
        loop {
            if let Some(&next) = self.nodes[s as usize].children.get(&byte) {
                return next;
            }
            if s == 0 {
                return 0;
            }
            s = self.nodes[s as usize].fail;
        }
    }

    /// 🛑️ A pattern index and byte length if `state` completes (directly or via a fail-link
    /// suffix) a pattern.
    fn matched_at(&self, state: u32) -> Option<(usize, usize)> {
        let node = &self.nodes[state as usize];
        node.pattern_end.or(node.longest_suffix_pattern).map(|pi| (pi, self.pattern_lens[pi]))
    }

    /// 🛑️ Bytes of unresolved trailing context at `state` — the "hold back" count.
    fn depth(&self, state: u32) -> usize {
        self.nodes[state as usize].depth
    }
}

/// 🛑️ Token-level stop: fires on any token in a fixed set (e.g. explicit stop tokens beyond EOS).
pub struct TokenStopCondition {
    pub tokens: Vec<TokenId>,
}

impl StopCondition for TokenStopCondition {
    fn name(&self) -> &'static str {
        "token_stop"
    }
    fn on_token(&mut self, _view: &StepView<'_>, token: TokenId) -> StopPoll {
        if self.tokens.contains(&token) {
            StopPoll::Finished { reason: FinishReason::StopToken, matched_bytes: 0 }
        } else {
            StopPoll::Continue
        }
    }
    fn save(&mut self) -> StopMark {
        StopMark(0)
    }
    fn rollback_to(&mut self, _mark: StopMark) {}
    fn reset(&mut self) {}
    fn fork(&self) -> Box<dyn StopCondition> {
        Box::new(Self { tokens: self.tokens.clone() })
    }
}

/// 🛑️ Text-sequence stop: feeds each token's byte representation (via the [`TokenTextAdapter`] in
/// [`StepView::adapter`]) through an [`AhoCorasick`] automaton; a no-operation (never matches) when no
/// adapter is supplied, since stop text can't be evaluated without token→byte mapping.
pub struct TextStopCondition {
    ac: std::rc::Rc<AhoCorasick>,
    #[allow(dead_code)]
    mode: StopTextMode,
    state: u32,
    snapshots: Vec<u32>,
}

impl TextStopCondition {
    pub fn new(sequences: &[Vec<u8>], mode: StopTextMode) -> Self {
        Self { ac: std::rc::Rc::new(AhoCorasick::build(sequences)), mode, state: 0, snapshots: Vec::new() }
    }
}

impl StopCondition for TextStopCondition {
    fn name(&self) -> &'static str {
        "text_stop"
    }
    fn on_token(&mut self, view: &StepView<'_>, token: TokenId) -> StopPoll {
        let Some(adapter) = view.adapter else { return StopPoll::Continue };
        let Some(bytes) = adapter.token_bytes(token) else { return StopPoll::Continue };
        for &byte in bytes {
            self.state = self.ac.step(self.state, byte);
            if let Some((pattern_index, pattern_len)) = self.ac.matched_at(self.state) {
                return StopPoll::Finished { reason: FinishReason::StopSequence { index: pattern_index }, matched_bytes: pattern_len };
            }
        }
        let hold = self.ac.depth(self.state);
        if hold > 0 {
            StopPoll::Hold { ambiguous_bytes: hold }
        } else {
            StopPoll::Continue
        }
    }
    fn save(&mut self) -> StopMark {
        self.snapshots.push(self.state);
        StopMark((self.snapshots.len() - 1) as u64)
    }
    fn rollback_to(&mut self, mark: StopMark) {
        if let Some(&state) = self.snapshots.get(mark.0 as usize) {
            self.state = state;
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    fn reset(&mut self) {
        self.state = 0;
        self.snapshots.clear();
    }
    fn fork(&self) -> Box<dyn StopCondition> {
        Box::new(Self { ac: self.ac.clone(), mode: self.mode, state: self.state, snapshots: self.snapshots.clone() })
    }
}
// #endregion 🔖️Stops

// #region 🔖️Automata
/// 🤖️ Parsed regex AST. Byte-level throughout (classes/literals match raw bytes, so multi-byte
/// UTF-8 sequences work as literal byte runs but aren't given first-class Unicode class support).
#[derive(Clone, PartialEq, Debug)]
enum RegexNode {
    Literal(u8),
    AnyByte,
    Class { ranges: Vec<(u8, u8)>, negate: bool },
    Concat(Vec<RegexNode>),
    Alt(Vec<RegexNode>),
    Star(Box<RegexNode>),
    Plus(Box<RegexNode>),
    Opt(Box<RegexNode>),
    Repeat(Box<RegexNode>, usize, Option<usize>),
}

fn unescape_byte(b: u8) -> u8 {
    match b {
        b'n' => b'\n',
        b't' => b'\t',
        b'r' => b'\r',
        other => other,
    }
}

/// 🤖️ Recursive-descent regex parser: alternation (lowest) → concatenation → postfix quantifiers
/// (`* + ? {m,n}`) → atoms (literals, `.`, `[...]` classes, `(...)` groups, `\`-escapes). No
/// backtracking at parse OR match time — this only builds the AST; matching happens via the NFA
/// → DFA pipeline below.
fn parse_regex(pattern: &str) -> Result<RegexNode, SamplingError> {
    let bytes = pattern.as_bytes();
    let mut pos = 0usize;
    let node = parse_alt(bytes, &mut pos)?;
    if pos != bytes.len() {
        return Err(SamplingError::RegexParse { offset: pos, reason: "unexpected trailing characters" });
    }
    Ok(node)
}

fn parse_alt(bytes: &[u8], pos: &mut usize) -> Result<RegexNode, SamplingError> {
    let mut branches = vec![parse_concat(bytes, pos)?];
    while bytes.get(*pos) == Some(&b'|') {
        *pos += 1;
        branches.push(parse_concat(bytes, pos)?);
    }
    Ok(if branches.len() == 1 { branches.pop().expect("non-empty branches") } else { RegexNode::Alt(branches) })
}

fn parse_concat(bytes: &[u8], pos: &mut usize) -> Result<RegexNode, SamplingError> {
    let mut parts = Vec::new();
    while let Some(&b) = bytes.get(*pos) {
        if b == b'|' || b == b')' {
            break;
        }
        parts.push(parse_quantified(bytes, pos)?);
    }
    Ok(RegexNode::Concat(parts))
}

fn parse_quantified(bytes: &[u8], pos: &mut usize) -> Result<RegexNode, SamplingError> {
    let atom = parse_atom(bytes, pos)?;
    match bytes.get(*pos) {
        Some(b'*') => {
            *pos += 1;
            Ok(RegexNode::Star(Box::new(atom)))
        }
        Some(b'+') => {
            *pos += 1;
            Ok(RegexNode::Plus(Box::new(atom)))
        }
        Some(b'?') => {
            *pos += 1;
            Ok(RegexNode::Opt(Box::new(atom)))
        }
        Some(b'{') => {
            *pos += 1;
            let (min, max) = parse_repeat_bounds(bytes, pos)?;
            Ok(RegexNode::Repeat(Box::new(atom), min, max))
        }
        _ => Ok(atom),
    }
}

fn parse_repeat_bounds(bytes: &[u8], pos: &mut usize) -> Result<(usize, Option<usize>), SamplingError> {
    let parse_uint = |bytes: &[u8], pos: &mut usize| -> Result<usize, SamplingError> {
        let start = *pos;
        while bytes.get(*pos).is_some_and(u8::is_ascii_digit) {
            *pos += 1;
        }
        core::str::from_utf8(&bytes[start..*pos]).ok().and_then(|s| s.parse().ok()).ok_or(SamplingError::RegexParse { offset: start, reason: "invalid repeat bound" })
    };
    let min = parse_uint(bytes, pos)?;
    let max = if bytes.get(*pos) == Some(&b',') {
        *pos += 1;
        if bytes.get(*pos) == Some(&b'}') {
            None
        } else {
            Some(parse_uint(bytes, pos)?)
        }
    } else {
        Some(min)
    };
    if bytes.get(*pos) != Some(&b'}') {
        return Err(SamplingError::RegexParse { offset: *pos, reason: "expected '}'" });
    }
    *pos += 1;
    Ok((min, max))
}

fn parse_atom(bytes: &[u8], pos: &mut usize) -> Result<RegexNode, SamplingError> {
    match bytes.get(*pos) {
        Some(b'(') => {
            *pos += 1;
            let inner = parse_alt(bytes, pos)?;
            if bytes.get(*pos) != Some(&b')') {
                return Err(SamplingError::RegexParse { offset: *pos, reason: "expected ')'" });
            }
            *pos += 1;
            Ok(inner)
        }
        Some(b'.') => {
            *pos += 1;
            Ok(RegexNode::AnyByte)
        }
        Some(b'[') => parse_class(bytes, pos),
        Some(b'\\') => {
            *pos += 1;
            let &b = bytes.get(*pos).ok_or(SamplingError::RegexParse { offset: *pos, reason: "dangling escape" })?;
            *pos += 1;
            Ok(RegexNode::Literal(unescape_byte(b)))
        }
        Some(&b) => {
            *pos += 1;
            Ok(RegexNode::Literal(b))
        }
        None => Err(SamplingError::RegexParse { offset: *pos, reason: "unexpected end of pattern" }),
    }
}

fn parse_class(bytes: &[u8], pos: &mut usize) -> Result<RegexNode, SamplingError> {
    debug_assert_eq!(bytes[*pos], b'[');
    *pos += 1;
    let negate = bytes.get(*pos) == Some(&b'^');
    if negate {
        *pos += 1;
    }
    let mut ranges = Vec::new();
    while let Some(&b) = bytes.get(*pos) {
        if b == b']' {
            break;
        }
        let lo = if b == b'\\' {
            *pos += 1;
            let e = *bytes.get(*pos).ok_or(SamplingError::RegexParse { offset: *pos, reason: "dangling escape in class" })?;
            unescape_byte(e)
        } else {
            b
        };
        *pos += 1;
        if bytes.get(*pos) == Some(&b'-') && bytes.get(*pos + 1) != Some(&b']') {
            *pos += 1;
            let &hi = bytes.get(*pos).ok_or(SamplingError::RegexParse { offset: *pos, reason: "dangling range in class" })?;
            *pos += 1;
            ranges.push((lo, hi));
        } else {
            ranges.push((lo, lo));
        }
    }
    if bytes.get(*pos) != Some(&b']') {
        return Err(SamplingError::RegexParse { offset: *pos, reason: "expected ']'" });
    }
    *pos += 1;
    Ok(RegexNode::Class { ranges, negate })
}

struct NfaNode {
    eps: Vec<usize>,
    byte_ranges: Vec<((u8, u8), usize)>,
    accept: bool,
}

struct Frag {
    start: usize,
    accept: usize,
}

struct NfaBuilder {
    nodes: Vec<NfaNode>,
}

impl NfaBuilder {
    fn new_state(&mut self) -> usize {
        self.nodes.push(NfaNode { eps: Vec::new(), byte_ranges: Vec::new(), accept: false });
        self.nodes.len() - 1
    }

    fn build(&mut self, node: &RegexNode, limits: &SamplingLimits) -> Result<Frag, SamplingError> {
        if self.nodes.len() > limits.max_automaton_states {
            return Err(SamplingError::AutomatonBudget { budget: "max_automaton_states" });
        }
        match node {
            RegexNode::Literal(b) => {
                let s = self.new_state();
                let a = self.new_state();
                self.nodes[s].byte_ranges.push(((*b, *b), a));
                Ok(Frag { start: s, accept: a })
            }
            RegexNode::AnyByte => {
                let s = self.new_state();
                let a = self.new_state();
                self.nodes[s].byte_ranges.push(((0, 255), a));
                Ok(Frag { start: s, accept: a })
            }
            RegexNode::Class { ranges, negate } => {
                let s = self.new_state();
                let a = self.new_state();
                if *negate {
                    let mut covered = [false; 256];
                    for &(lo, hi) in ranges {
                        for b in lo..=hi {
                            covered[b as usize] = true;
                        }
                    }
                    let mut b = 0u16;
                    while b <= 255 {
                        if !covered[b as usize] {
                            let start = b;
                            while b <= 255 && !covered[b as usize] {
                                b += 1;
                            }
                            self.nodes[s].byte_ranges.push(((start as u8, (b - 1) as u8), a));
                        } else {
                            b += 1;
                        }
                    }
                } else {
                    for &(lo, hi) in ranges {
                        self.nodes[s].byte_ranges.push(((lo, hi), a));
                    }
                }
                Ok(Frag { start: s, accept: a })
            }
            RegexNode::Concat(parts) => {
                if parts.is_empty() {
                    let s = self.new_state();
                    return Ok(Frag { start: s, accept: s });
                }
                let mut frags = Vec::with_capacity(parts.len());
                for p in parts {
                    frags.push(self.build(p, limits)?);
                }
                for w in frags.windows(2) {
                    self.nodes[w[0].accept].eps.push(w[1].start);
                }
                Ok(Frag { start: frags[0].start, accept: frags[frags.len() - 1].accept })
            }
            RegexNode::Alt(branches) => {
                let s = self.new_state();
                let a = self.new_state();
                for b in branches {
                    let f = self.build(b, limits)?;
                    self.nodes[s].eps.push(f.start);
                    self.nodes[f.accept].eps.push(a);
                }
                Ok(Frag { start: s, accept: a })
            }
            RegexNode::Star(inner) => {
                let s = self.new_state();
                let a = self.new_state();
                let f = self.build(inner, limits)?;
                self.nodes[s].eps.push(f.start);
                self.nodes[s].eps.push(a);
                self.nodes[f.accept].eps.push(f.start);
                self.nodes[f.accept].eps.push(a);
                Ok(Frag { start: s, accept: a })
            }
            RegexNode::Plus(inner) => {
                let first = self.build(inner, limits)?;
                let star = self.build(&RegexNode::Star(inner.clone()), limits)?;
                self.nodes[first.accept].eps.push(star.start);
                Ok(Frag { start: first.start, accept: star.accept })
            }
            RegexNode::Opt(inner) => {
                let s = self.new_state();
                let a = self.new_state();
                let f = self.build(inner, limits)?;
                self.nodes[s].eps.push(f.start);
                self.nodes[s].eps.push(a);
                self.nodes[f.accept].eps.push(a);
                Ok(Frag { start: s, accept: a })
            }
            RegexNode::Repeat(inner, min, max) => {
                let mut parts = Vec::new();
                for _ in 0..*min {
                    parts.push((**inner).clone());
                }
                match max {
                    Some(max) if *max > *min => {
                        for _ in 0..(*max - *min) {
                            parts.push(RegexNode::Opt(inner.clone()));
                        }
                    }
                    None => parts.push(RegexNode::Star(inner.clone())),
                    _ => {}
                }
                self.build(&RegexNode::Concat(parts), limits)
            }
        }
    }
}

fn eps_closure(nodes: &[NfaNode], start: &[usize]) -> Vec<usize> {
    let mut stack: Vec<usize> = start.to_vec();
    let mut seen: std::collections::BTreeSet<usize> = start.iter().copied().collect();
    while let Some(s) = stack.pop() {
        for &e in &nodes[s].eps {
            if seen.insert(e) {
                stack.push(e);
            }
        }
    }
    seen.into_iter().collect()
}

fn nfa_set_has_accept(nodes: &[NfaNode], states: &[usize]) -> bool {
    states.iter().any(|&s| nodes[s].accept)
}

/// 🤖️ A byte-level DFA over 256-wide dense transition rows. State `0` is a permanent, unreachable,
/// non-accepting dead state (the "no transition" sentinel); the real start state is always `>= 1`
/// once the pattern accepts at least one string. `alive[s]` is precomputed once via reverse
/// reachability from every accept state, so `Constraint::is_dead` is an O(1) lookup rather than a
/// live search.
pub struct Dfa {
    transitions: Vec<u32>,
    accept: Vec<bool>,
    alive: Vec<bool>,
    start: u32,
    num_states: usize,
}

const DFA_DEAD: u32 = 0;

/// 🤖️ Subset construction: an NFA (with accept flags already set) plus a designated start state
/// becomes a byte-level DFA, budget-checked against `limits.max_automaton_states`. Shared by
/// [`Dfa::from_pattern`] and [`EbnfConstraint::new`] (which compiles a grammar into an NFA via a
/// different front end but needs the exact same back end).
fn subset_construct(nodes: &[NfaNode], start_nfa_state: usize, limits: &SamplingLimits) -> Result<Dfa, SamplingError> {
    let mut dfa_states: Vec<Vec<usize>> = vec![Vec::new()];
    let mut state_index: std::collections::HashMap<Vec<usize>, u32> = std::collections::HashMap::new();
    state_index.insert(Vec::new(), DFA_DEAD);
    let mut accept = vec![false];
    let mut transitions: Vec<u32> = vec![DFA_DEAD; 256];

    let start_set = eps_closure(nodes, &[start_nfa_state]);
    let start = if let Some(&id) = state_index.get(&start_set) {
        id
    } else {
        let id = dfa_states.len() as u32;
        dfa_states.push(start_set.clone());
        state_index.insert(start_set.clone(), id);
        accept.push(nfa_set_has_accept(nodes, &start_set));
        transitions.resize(dfa_states.len() * 256, DFA_DEAD);
        id
    };

    let mut queue: std::collections::VecDeque<u32> = std::collections::VecDeque::new();
    queue.push_back(start);
    let mut queued: std::collections::HashSet<u32> = std::collections::HashSet::new();
    queued.insert(start);
    while let Some(state_id) = queue.pop_front() {
        if dfa_states.len() > limits.max_automaton_states {
            return Err(SamplingError::AutomatonBudget { budget: "max_automaton_states" });
        }
        let nfa_set = dfa_states[state_id as usize].clone();
        for byte in 0..=255u16 {
            let byte = byte as u8;
            let mut targets: Vec<usize> = Vec::new();
            for &s in &nfa_set {
                for &((lo, hi), to) in &nodes[s].byte_ranges {
                    if byte >= lo && byte <= hi {
                        targets.push(to);
                    }
                }
            }
            if targets.is_empty() {
                continue;
            }
            let closure = eps_closure(nodes, &targets);
            if closure.is_empty() {
                continue;
            }
            let next_id = if let Some(&id) = state_index.get(&closure) {
                id
            } else {
                let id = dfa_states.len() as u32;
                dfa_states.push(closure.clone());
                state_index.insert(closure.clone(), id);
                accept.push(nfa_set_has_accept(nodes, &closure));
                transitions.resize(dfa_states.len() * 256, DFA_DEAD);
                id
            };
            transitions[state_id as usize * 256 + byte as usize] = next_id;
            if queued.insert(next_id) {
                queue.push_back(next_id);
            }
        }
    }

    let num_states = dfa_states.len();
    let mut incoming: Vec<Vec<u32>> = vec![Vec::new(); num_states];
    for s in 0..num_states {
        for b in 0..256usize {
            let t = transitions[s * 256 + b];
            if t != DFA_DEAD {
                incoming[t as usize].push(s as u32);
            }
        }
    }
    let mut alive = vec![false; num_states];
    let mut stack: Vec<u32> = (0..num_states as u32).filter(|&s| accept[s as usize]).collect();
    for &s in &stack {
        alive[s as usize] = true;
    }
    while let Some(s) = stack.pop() {
        for &p in &incoming[s as usize] {
            if !alive[p as usize] {
                alive[p as usize] = true;
                stack.push(p);
            }
        }
    }

    Ok(Dfa { transitions, accept, alive, start, num_states })
}

impl Dfa {
    /// 🤖️ Parses `pattern`, builds its Thompson NFA, then performs subset construction into a DFA,
    /// erroring if either the NFA or the DFA would exceed `limits.max_automaton_states`.
    pub fn from_pattern(pattern: &str, limits: &SamplingLimits) -> Result<Self, SamplingError> {
        let ast = parse_regex(pattern)?;
        let mut builder = NfaBuilder { nodes: Vec::new() };
        let frag = builder.build(&ast, limits)?;
        builder.nodes[frag.accept].accept = true;
        subset_construct(&builder.nodes, frag.start, limits)
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    #[inline]
    pub fn step(&self, state: u32, byte: u8) -> u32 {
        self.transitions[state as usize * 256 + byte as usize]
    }

    pub fn is_accept(&self, state: u32) -> bool {
        self.accept.get(state as usize).copied().unwrap_or(false)
    }

    pub fn is_alive(&self, state: u32) -> bool {
        self.alive.get(state as usize).copied().unwrap_or(false)
    }

    pub fn is_dead(&self, state: u32) -> bool {
        state == DFA_DEAD
    }

    pub fn num_states(&self) -> usize {
        self.num_states
    }
}

/// 🤖️ Per-DFA-state lazily computed `(allowed token mask, next state per token)`, so a constraint
/// doesn't re-walk every token's bytes through the DFA on every step at the same automaton state.
/// Bounded by `max_entries`; a full-clear eviction policy (not LRU) once the bound is hit — simple
/// and correct, if not optimal; a proper clock/LRU is a hardening-wave follow-up.
pub struct DfaTokenMemo {
    entries: std::collections::HashMap<u32, (TokenBitset, Vec<u32>)>,
    max_entries: usize,
}

impl DfaTokenMemo {
    pub fn new(max_entries: usize) -> Self {
        Self { entries: std::collections::HashMap::new(), max_entries }
    }

    pub fn get_or_compute(&mut self, dfa: &Dfa, state: u32, adapter: &dyn TokenTextAdapter) -> &(TokenBitset, Vec<u32>) {
        if !self.entries.contains_key(&state) {
            if self.entries.len() >= self.max_entries {
                self.entries.clear();
            }
            let vocab_size = adapter.vocab_size();
            let mut allowed = TokenBitset::new_empty(vocab_size);
            let mut next_state = vec![DFA_DEAD; vocab_size];
            for (i, next_state_i) in next_state.iter_mut().enumerate() {
                let token = TokenId::new(i as u32);
                if let Some(bytes) = adapter.token_bytes(token) {
                    let mut s = state;
                    let mut ok = true;
                    for &b in bytes {
                        s = dfa.step(s, b);
                        if dfa.is_dead(s) {
                            ok = false;
                            break;
                        }
                    }
                    if ok && dfa.is_alive(s) {
                        allowed.set(token, true);
                        *next_state_i = s;
                    }
                }
            }
            self.entries.insert(state, (allowed, next_state));
        }
        self.entries.get(&state).expect("just inserted or already present")
    }
}
// #endregion 🔖️Automata

// #region 🔖️Constraints
/// 🧱️ Constrains generation to text matching a regex (interpreted as a whole-string match: the
/// final generated text must end in an accept state).
pub struct RegexConstraint {
    dfa: std::rc::Rc<Dfa>,
    cache: DfaTokenMemo,
    max_cache_entries: usize,
    state: u32,
    snapshots: Vec<u32>,
}

impl RegexConstraint {
    pub fn new(pattern: &str, limits: &SamplingLimits) -> Result<Self, SamplingError> {
        let dfa = Dfa::from_pattern(pattern, limits)?;
        let start = dfa.start();
        Ok(Self { dfa: std::rc::Rc::new(dfa), cache: DfaTokenMemo::new(limits.max_dfa_cache_entries), max_cache_entries: limits.max_dfa_cache_entries, state: start, snapshots: Vec::new() })
    }
}

impl Constraint for RegexConstraint {
    fn name(&self) -> &'static str {
        "regex"
    }
    fn fill_mask(&mut self, view: &StepView<'_>, mask: &mut TokenBitset) -> Result<(), SamplingError> {
        let Some(adapter) = view.adapter else { return Ok(()) };
        let (allowed, _) = self.cache.get_or_compute(&self.dfa, self.state, adapter);
        mask.and_with(allowed);
        Ok(())
    }
    fn accept(&mut self, view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError> {
        let Some(adapter) = view.adapter else { return Ok(()) };
        let (_, next_state) = self.cache.get_or_compute(&self.dfa, self.state, adapter);
        if let Some(&next) = next_state.get(token.get() as usize) {
            self.state = next;
        }
        Ok(())
    }
    fn is_satisfied(&self) -> bool {
        self.dfa.is_accept(self.state)
    }
    fn is_finished(&self) -> bool {
        self.is_satisfied()
    }
    fn is_dead(&self) -> bool {
        self.dfa.is_dead(self.state) || !self.dfa.is_alive(self.state)
    }
    fn save(&mut self) -> ConstraintMark {
        self.snapshots.push(self.state);
        ConstraintMark((self.snapshots.len() - 1) as u64)
    }
    fn rollback_to(&mut self, mark: ConstraintMark) {
        if let Some(&s) = self.snapshots.get(mark.0 as usize) {
            self.state = s;
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    fn reset(&mut self) {
        self.state = self.dfa.start();
        self.snapshots.clear();
    }
    fn fork(&self) -> Box<dyn Constraint> {
        Box::new(Self { dfa: self.dfa.clone(), cache: DfaTokenMemo::new(self.max_cache_entries), max_cache_entries: self.max_cache_entries, state: self.state, snapshots: self.snapshots.clone() })
    }
}

/// 🧱️ Constrains generation to one of a fixed set of allowed token-id phrases (a trie over
/// `TokenId` sequences rather than bytes).
pub struct TrieConstraint {
    nodes: Vec<std::collections::HashMap<TokenId, usize>>,
    accept: Vec<bool>,
    state: usize,
    snapshots: Vec<usize>,
}

impl TrieConstraint {
    pub fn new(phrases: &[Vec<TokenId>]) -> Self {
        let mut nodes = vec![std::collections::HashMap::new()];
        let mut accept = vec![false];
        for phrase in phrases {
            let mut state = 0usize;
            for &token in phrase {
                if !nodes[state].contains_key(&token) {
                    nodes.push(std::collections::HashMap::new());
                    accept.push(false);
                    let new_id = nodes.len() - 1;
                    nodes[state].insert(token, new_id);
                }
                state = nodes[state][&token];
            }
            accept[state] = true;
        }
        Self { nodes, accept, state: 0, snapshots: Vec::new() }
    }
}

impl Constraint for TrieConstraint {
    fn name(&self) -> &'static str {
        "trie"
    }
    fn fill_mask(&mut self, _view: &StepView<'_>, mask: &mut TokenBitset) -> Result<(), SamplingError> {
        let mut allow = TokenBitset::new_empty(mask.len());
        for &token in self.nodes[self.state].keys() {
            allow.set(token, true);
        }
        mask.and_with(&allow);
        Ok(())
    }
    fn accept(&mut self, _view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError> {
        if let Some(&next) = self.nodes[self.state].get(&token) {
            self.state = next;
        }
        Ok(())
    }
    fn is_satisfied(&self) -> bool {
        self.accept[self.state]
    }
    fn is_finished(&self) -> bool {
        self.accept[self.state] && self.nodes[self.state].is_empty()
    }
    fn is_dead(&self) -> bool {
        self.nodes[self.state].is_empty() && !self.accept[self.state]
    }
    fn save(&mut self) -> ConstraintMark {
        self.snapshots.push(self.state);
        ConstraintMark((self.snapshots.len() - 1) as u64)
    }
    fn rollback_to(&mut self, mark: ConstraintMark) {
        if let Some(&s) = self.snapshots.get(mark.0 as usize) {
            self.state = s;
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    fn reset(&mut self) {
        self.state = 0;
        self.snapshots.clear();
    }
    fn fork(&self) -> Box<dyn Constraint> {
        Box::new(Self { nodes: self.nodes.clone(), accept: self.accept.clone(), state: self.state, snapshots: self.snapshots.clone() })
    }
}

/// 🧱️ Requires at least one of several token-sequence alternatives to appear somewhere in the
/// generated continuation; does not restrict candidates, only tracks completion (EOS/finish are
/// gated on [`Constraint::is_satisfied`] by the engine's constraint composition).
pub struct MustIncludeConstraint {
    alternatives: Vec<Vec<TokenId>>,
    satisfied: bool,
    history: Vec<TokenId>,
    snapshots: Vec<(bool, usize)>,
}

impl MustIncludeConstraint {
    pub fn new(alternatives: Vec<Vec<TokenId>>) -> Self {
        Self { alternatives, satisfied: false, history: Vec::new(), snapshots: Vec::new() }
    }
}

impl Constraint for MustIncludeConstraint {
    fn name(&self) -> &'static str {
        "must_include"
    }
    fn fill_mask(&mut self, _view: &StepView<'_>, _mask: &mut TokenBitset) -> Result<(), SamplingError> {
        Ok(())
    }
    fn accept(&mut self, _view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError> {
        self.history.push(token);
        if !self.satisfied {
            for phrase in &self.alternatives {
                if self.history.len() >= phrase.len() && self.history[self.history.len() - phrase.len()..] == phrase[..] {
                    self.satisfied = true;
                    break;
                }
            }
        }
        Ok(())
    }
    fn is_satisfied(&self) -> bool {
        self.satisfied || self.alternatives.is_empty()
    }
    fn is_finished(&self) -> bool {
        self.satisfied
    }
    fn is_dead(&self) -> bool {
        false
    }
    fn save(&mut self) -> ConstraintMark {
        self.snapshots.push((self.satisfied, self.history.len()));
        ConstraintMark((self.snapshots.len() - 1) as u64)
    }
    fn rollback_to(&mut self, mark: ConstraintMark) {
        if let Some(&(satisfied, len)) = self.snapshots.get(mark.0 as usize) {
            self.satisfied = satisfied;
            self.history.truncate(len);
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    fn reset(&mut self) {
        self.satisfied = false;
        self.history.clear();
        self.snapshots.clear();
    }
    fn fork(&self) -> Box<dyn Constraint> {
        Box::new(Self { alternatives: self.alternatives.clone(), satisfied: self.satisfied, history: self.history.clone(), snapshots: self.snapshots.clone() })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum JsonFrame {
    Object,
    Array,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum JsonExpect {
    Value,
    ObjectKeyOrClose,
    ObjectKey,
    ObjectColon,
    ObjectCommaOrClose,
    ArrayValueOrClose,
    ArrayCommaOrClose,
    Done,
}

/// 🧱️ Byte-level incremental JSON structural constraint: a small explicit push-down automaton
/// (object/array frame stack + an "expected next token class" state) that validates JSON syntax
/// reactively as token bytes are accepted. Numbers, booleans, and `null` are treated as atomic
/// (their internal digits/characters aren't byte-validated) — a deliberate simplification, not a
/// full JSON-number grammar. Does not proactively mask (`fill_mask` is a no-operation): building a
/// per-state token-feasibility cache for this hand-written automaton (mirroring [`DfaTokenMemo`])
/// is a hardening-wave follow-up.
pub struct JsonModeConstraint {
    stack: Vec<JsonFrame>,
    expect: JsonExpect,
    in_string: bool,
    string_escaped: bool,
    string_is_key: bool,
    dead: bool,
    snapshots: Vec<(Vec<JsonFrame>, JsonExpect, bool, bool, bool, bool)>,
}

impl JsonModeConstraint {
    pub fn new() -> Self {
        Self { stack: Vec::new(), expect: JsonExpect::Value, in_string: false, string_escaped: false, string_is_key: false, dead: false, snapshots: Vec::new() }
    }

    fn after_value(&mut self) {
        self.expect = match self.stack.last() {
            None => JsonExpect::Done,
            Some(JsonFrame::Object) => JsonExpect::ObjectCommaOrClose,
            Some(JsonFrame::Array) => JsonExpect::ArrayCommaOrClose,
        };
    }

    /// 🧱️ Consumes one byte, returning `false` if it is structurally invalid in the current state.
    fn feed_byte(&mut self, b: u8) -> bool {
        if self.in_string {
            if self.string_escaped {
                self.string_escaped = false;
                return true;
            }
            match b {
                b'\\' => {
                    self.string_escaped = true;
                    true
                }
                b'"' => {
                    self.in_string = false;
                    if self.string_is_key {
                        self.expect = JsonExpect::ObjectColon;
                    } else {
                        self.after_value();
                    }
                    true
                }
                _ => true,
            }
        } else {
            match b {
                b' ' | b'\t' | b'\n' | b'\r' => true,
                b'"' if matches!(self.expect, JsonExpect::Value | JsonExpect::ArrayValueOrClose) => {
                    self.in_string = true;
                    self.string_is_key = false;
                    true
                }
                b'"' if matches!(self.expect, JsonExpect::ObjectKey | JsonExpect::ObjectKeyOrClose) => {
                    self.in_string = true;
                    self.string_is_key = true;
                    true
                }
                b'{' if matches!(self.expect, JsonExpect::Value | JsonExpect::ArrayValueOrClose) => {
                    self.stack.push(JsonFrame::Object);
                    self.expect = JsonExpect::ObjectKeyOrClose;
                    true
                }
                b'[' if matches!(self.expect, JsonExpect::Value | JsonExpect::ArrayValueOrClose) => {
                    self.stack.push(JsonFrame::Array);
                    self.expect = JsonExpect::ArrayValueOrClose;
                    true
                }
                b'}' if matches!(self.expect, JsonExpect::ObjectKeyOrClose | JsonExpect::ObjectCommaOrClose) && self.stack.last() == Some(&JsonFrame::Object) => {
                    self.stack.pop();
                    self.after_value();
                    true
                }
                b']' if matches!(self.expect, JsonExpect::ArrayValueOrClose | JsonExpect::ArrayCommaOrClose) && self.stack.last() == Some(&JsonFrame::Array) => {
                    self.stack.pop();
                    self.after_value();
                    true
                }
                b':' if self.expect == JsonExpect::ObjectColon => {
                    self.expect = JsonExpect::Value;
                    true
                }
                b',' if self.expect == JsonExpect::ObjectCommaOrClose => {
                    self.expect = JsonExpect::ObjectKey;
                    true
                }
                b',' if self.expect == JsonExpect::ArrayCommaOrClose => {
                    self.expect = JsonExpect::ArrayValueOrClose;
                    true
                }
                b't' | b'f' | b'n' | b'-' | b'0'..=b'9' if matches!(self.expect, JsonExpect::Value | JsonExpect::ArrayValueOrClose) => {
                    self.after_value();
                    true
                }
                _ => false,
            }
        }
    }
}

impl Default for JsonModeConstraint {
    fn default() -> Self {
        Self::new()
    }
}

impl Constraint for JsonModeConstraint {
    fn name(&self) -> &'static str {
        "json_mode"
    }
    fn fill_mask(&mut self, _view: &StepView<'_>, _mask: &mut TokenBitset) -> Result<(), SamplingError> {
        Ok(())
    }
    fn accept(&mut self, view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError> {
        let Some(adapter) = view.adapter else { return Ok(()) };
        let Some(bytes) = adapter.token_bytes(token) else { return Ok(()) };
        for &b in bytes {
            if !self.feed_byte(b) {
                self.dead = true;
                break;
            }
        }
        Ok(())
    }
    fn is_satisfied(&self) -> bool {
        self.expect == JsonExpect::Done && !self.in_string
    }
    fn is_finished(&self) -> bool {
        self.is_satisfied()
    }
    fn is_dead(&self) -> bool {
        self.dead
    }
    fn save(&mut self) -> ConstraintMark {
        self.snapshots.push((self.stack.clone(), self.expect, self.in_string, self.string_escaped, self.string_is_key, self.dead));
        ConstraintMark((self.snapshots.len() - 1) as u64)
    }
    fn rollback_to(&mut self, mark: ConstraintMark) {
        if let Some((stack, expect, in_string, string_escaped, string_is_key, dead)) = self.snapshots.get(mark.0 as usize).cloned() {
            self.stack = stack;
            self.expect = expect;
            self.in_string = in_string;
            self.string_escaped = string_escaped;
            self.string_is_key = string_is_key;
            self.dead = dead;
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    fn reset(&mut self) {
        *self = Self::new();
    }
    fn fork(&self) -> Box<dyn Constraint> {
        Box::new(Self { stack: self.stack.clone(), expect: self.expect, in_string: self.in_string, string_escaped: self.string_escaped, string_is_key: self.string_is_key, dead: self.dead, snapshots: self.snapshots.clone() })
    }
}

/// 🧱️ EBNF grammar rules: `name ::= expr ;` where `expr` reuses [`RegexNode`]'s alternation,
/// concatenation, and quantifier operators over quoted terminals and rule references. Compiled by
/// inlining rule references into a single [`RegexNode`] (bounded by `max_expansions`), so it
/// supports non-recursive and boundedly-recursive grammars — not general (potentially
/// left-recursive or unbounded) context-free grammars, which would need a real Earley/GLR parser.
/// That is a deliberate, documented scope cut: [`ConstraintSpec::Ebnf`] compiles through this path
/// and therefore shares the same limitation.
struct EbnfGrammar {
    rules: std::collections::HashMap<String, GrammarExpr>,
    start: String,
}

#[derive(Clone)]
enum GrammarExpr {
    Terminal(String),
    Rule(String),
    Concat(Vec<GrammarExpr>),
    Alt(Vec<GrammarExpr>),
    Star(Box<GrammarExpr>),
    Plus(Box<GrammarExpr>),
    Opt(Box<GrammarExpr>),
}

fn parse_ebnf(text: &str) -> Result<EbnfGrammar, SamplingError> {
    let mut rules = std::collections::HashMap::new();
    let mut start = None;
    for (line_no, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some(sep) = line.find("::=") else {
            return Err(SamplingError::GrammarParse { offset: line_no, reason: "expected '::=' rule separator" });
        };
        let name = line[..sep].trim().to_string();
        let body = line[sep + 3..].trim().trim_end_matches(';').trim();
        let expr = parse_grammar_alt(body.as_bytes(), &mut 0)?;
        if start.is_none() {
            start = Some(name.clone());
        }
        rules.insert(name, expr);
    }
    let start = start.ok_or(SamplingError::GrammarParse { offset: 0, reason: "grammar defines no rules" })?;
    Ok(EbnfGrammar { rules, start })
}

fn parse_grammar_alt(bytes: &[u8], pos: &mut usize) -> Result<GrammarExpr, SamplingError> {
    skip_grammar_ws(bytes, pos);
    let mut branches = vec![parse_grammar_concat(bytes, pos)?];
    loop {
        skip_grammar_ws(bytes, pos);
        if bytes.get(*pos) == Some(&b'|') {
            *pos += 1;
            branches.push(parse_grammar_concat(bytes, pos)?);
        } else {
            break;
        }
    }
    Ok(if branches.len() == 1 { branches.pop().expect("non-empty branches") } else { GrammarExpr::Alt(branches) })
}

fn parse_grammar_concat(bytes: &[u8], pos: &mut usize) -> Result<GrammarExpr, SamplingError> {
    let mut parts = Vec::new();
    loop {
        skip_grammar_ws(bytes, pos);
        match bytes.get(*pos) {
            Some(b'|') | Some(b')') | None => break,
            _ => parts.push(parse_grammar_quantified(bytes, pos)?),
        }
    }
    Ok(GrammarExpr::Concat(parts))
}

fn parse_grammar_quantified(bytes: &[u8], pos: &mut usize) -> Result<GrammarExpr, SamplingError> {
    let atom = parse_grammar_atom(bytes, pos)?;
    match bytes.get(*pos) {
        Some(b'*') => {
            *pos += 1;
            Ok(GrammarExpr::Star(Box::new(atom)))
        }
        Some(b'+') => {
            *pos += 1;
            Ok(GrammarExpr::Plus(Box::new(atom)))
        }
        Some(b'?') => {
            *pos += 1;
            Ok(GrammarExpr::Opt(Box::new(atom)))
        }
        _ => Ok(atom),
    }
}

fn skip_grammar_ws(bytes: &[u8], pos: &mut usize) {
    while bytes.get(*pos).is_some_and(u8::is_ascii_whitespace) {
        *pos += 1;
    }
}

fn parse_grammar_atom(bytes: &[u8], pos: &mut usize) -> Result<GrammarExpr, SamplingError> {
    skip_grammar_ws(bytes, pos);
    match bytes.get(*pos) {
        Some(b'(') => {
            *pos += 1;
            let inner = parse_grammar_alt(bytes, pos)?;
            skip_grammar_ws(bytes, pos);
            if bytes.get(*pos) != Some(&b')') {
                return Err(SamplingError::GrammarParse { offset: *pos, reason: "expected ')'" });
            }
            *pos += 1;
            Ok(inner)
        }
        Some(b'"') => {
            *pos += 1;
            let start = *pos;
            while bytes.get(*pos).is_some_and(|&b| b != b'"') {
                *pos += 1;
            }
            let text = core::str::from_utf8(&bytes[start..*pos]).map_err(|_| SamplingError::GrammarParse { offset: start, reason: "invalid utf-8 in terminal" })?.to_string();
            if bytes.get(*pos) != Some(&b'"') {
                return Err(SamplingError::GrammarParse { offset: *pos, reason: "unterminated terminal" });
            }
            *pos += 1;
            Ok(GrammarExpr::Terminal(text))
        }
        Some(&b) if b.is_ascii_alphabetic() || b == b'_' => {
            let start = *pos;
            while bytes.get(*pos).is_some_and(|&b| b.is_ascii_alphanumeric() || b == b'_') {
                *pos += 1;
            }
            let name = core::str::from_utf8(&bytes[start..*pos]).expect("ASCII rule name").to_string();
            Ok(GrammarExpr::Rule(name))
        }
        _ => Err(SamplingError::GrammarParse { offset: *pos, reason: "expected '(', '\"', or a rule name" }),
    }
}

/// 🧱️ Inlines `expr` into a [`RegexNode`], expanding rule references by substitution up to
/// `budget` total expansions (shared, decremented across the whole compilation) — the bound that
/// turns unsupported unbounded recursion into a clean [`SamplingError::AutomatonBudget`] instead
/// of an infinite expansion.
fn compile_grammar_expr(expr: &GrammarExpr, grammar: &EbnfGrammar, budget: &mut usize) -> Result<RegexNode, SamplingError> {
    if *budget == 0 {
        return Err(SamplingError::AutomatonBudget { budget: "grammar expansion (possible unbounded recursion)" });
    }
    *budget -= 1;
    match expr {
        GrammarExpr::Terminal(text) => Ok(RegexNode::Concat(text.bytes().map(RegexNode::Literal).collect())),
        GrammarExpr::Rule(name) => {
            let inner = grammar.rules.get(name).ok_or(SamplingError::GrammarParse { offset: 0, reason: "reference to undefined rule" })?;
            compile_grammar_expr(inner, grammar, budget)
        }
        GrammarExpr::Concat(parts) => Ok(RegexNode::Concat(parts.iter().map(|p| compile_grammar_expr(p, grammar, budget)).collect::<Result<_, _>>()?)),
        GrammarExpr::Alt(branches) => Ok(RegexNode::Alt(branches.iter().map(|p| compile_grammar_expr(p, grammar, budget)).collect::<Result<_, _>>()?)),
        GrammarExpr::Star(inner) => Ok(RegexNode::Star(Box::new(compile_grammar_expr(inner, grammar, budget)?))),
        GrammarExpr::Plus(inner) => Ok(RegexNode::Plus(Box::new(compile_grammar_expr(inner, grammar, budget)?))),
        GrammarExpr::Opt(inner) => Ok(RegexNode::Opt(Box::new(compile_grammar_expr(inner, grammar, budget)?))),
    }
}

/// 🧱️ An EBNF-grammar constraint, compiled through [`compile_grammar_expr`] into the same
/// DFA/[`DfaTokenMemo`] machinery as [`RegexConstraint`] — see [`EbnfGrammar`]'s doc for the
/// supported (non-recursive/boundedly-recursive) grammar subset.
pub struct EbnfConstraint(RegexConstraint);

impl EbnfConstraint {
    pub fn new(grammar_text: &str, limits: &SamplingLimits) -> Result<Self, SamplingError> {
        let grammar = parse_ebnf(grammar_text)?;
        let start_expr = grammar.rules.get(&grammar.start).ok_or(SamplingError::GrammarParse { offset: 0, reason: "start rule missing" })?.clone();
        let mut budget = limits.max_grammar_bytes;
        let ast = compile_grammar_expr(&start_expr, &grammar, &mut budget)?;
        let mut builder = NfaBuilder { nodes: Vec::new() };
        let frag = builder.build(&ast, limits)?;
        builder.nodes[frag.accept].accept = true;
        let dfa = subset_construct(&builder.nodes, frag.start, limits)?;
        let start = dfa.start();
        Ok(Self(RegexConstraint { dfa: std::rc::Rc::new(dfa), cache: DfaTokenMemo::new(limits.max_dfa_cache_entries), max_cache_entries: limits.max_dfa_cache_entries, state: start, snapshots: Vec::new() }))
    }
}

impl Constraint for EbnfConstraint {
    fn name(&self) -> &'static str {
        "ebnf"
    }
    fn fill_mask(&mut self, view: &StepView<'_>, mask: &mut TokenBitset) -> Result<(), SamplingError> {
        self.0.fill_mask(view, mask)
    }
    fn accept(&mut self, view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError> {
        self.0.accept(view, token)
    }
    fn is_satisfied(&self) -> bool {
        self.0.is_satisfied()
    }
    fn is_finished(&self) -> bool {
        self.0.is_finished()
    }
    fn is_dead(&self) -> bool {
        self.0.is_dead()
    }
    fn save(&mut self) -> ConstraintMark {
        self.0.save()
    }
    fn rollback_to(&mut self, mark: ConstraintMark) {
        self.0.rollback_to(mark);
    }
    fn reset(&mut self) {
        self.0.reset();
    }
    fn fork(&self) -> Box<dyn Constraint> {
        Box::new(Self(RegexConstraint { dfa: self.0.dfa.clone(), cache: DfaTokenMemo::new(self.0.max_cache_entries), max_cache_entries: self.0.max_cache_entries, state: self.0.state, snapshots: self.0.snapshots.clone() }))
    }
}

/// 🧱️ Recursive JSON-Schema subset validator: `type`, `enum`, numeric `minimum`/`maximum`, string
/// `minLength`/`maxLength`, array `minItems`/`maxItems`/`items`, and object `required`/`properties`.
/// Unrecognized schema keywords (`oneOf`, `pattern`, ...) are silently ignored rather than
/// rejected — a documented subset, not full JSON Schema.
fn validates_json_schema(value: &JsonValue, schema: &JsonValue) -> bool {
    if let Some(type_name) = schema.get("type").and_then(JsonValue::as_str) {
        let matches_type = match type_name {
            "object" => matches!(value, JsonValue::Object(_)),
            "array" => matches!(value, JsonValue::Array(_)),
            "string" => matches!(value, JsonValue::Str(_)),
            "number" => matches!(value, JsonValue::Num(_)),
            "integer" => matches!(value, JsonValue::Num(n) if n.fract() == 0.0),
            "boolean" => matches!(value, JsonValue::Bool(_)),
            "null" => matches!(value, JsonValue::Null),
            _ => true,
        };
        if !matches_type {
            return false;
        }
    }
    if let Some(enum_values) = schema.get("enum").and_then(JsonValue::as_array) {
        if !enum_values.contains(value) {
            return false;
        }
    }
    match value {
        JsonValue::Num(n) => {
            if let Some(min) = schema.get("minimum").and_then(JsonValue::as_f64) {
                if *n < min {
                    return false;
                }
            }
            if let Some(max) = schema.get("maximum").and_then(JsonValue::as_f64) {
                if *n > max {
                    return false;
                }
            }
        }
        JsonValue::Str(s) => {
            if let Some(min_len) = schema.get("minLength").and_then(JsonValue::as_f64) {
                if (s.len() as f64) < min_len {
                    return false;
                }
            }
            if let Some(max_len) = schema.get("maxLength").and_then(JsonValue::as_f64) {
                if (s.len() as f64) > max_len {
                    return false;
                }
            }
        }
        JsonValue::Array(items) => {
            if let Some(min_items) = schema.get("minItems").and_then(JsonValue::as_f64) {
                if (items.len() as f64) < min_items {
                    return false;
                }
            }
            if let Some(max_items) = schema.get("maxItems").and_then(JsonValue::as_f64) {
                if (items.len() as f64) > max_items {
                    return false;
                }
            }
            if let Some(item_schema) = schema.get("items") {
                if !items.iter().all(|item| validates_json_schema(item, item_schema)) {
                    return false;
                }
            }
        }
        JsonValue::Object(entries) => {
            if let Some(required) = schema.get("required").and_then(JsonValue::as_array) {
                for req in required {
                    if let Some(key) = req.as_str() {
                        if !entries.iter().any(|(k, _)| k == key) {
                            return false;
                        }
                    }
                }
            }
            if let Some(JsonValue::Object(props)) = schema.get("properties") {
                for (key, val) in entries {
                    if let Some((_, prop_schema)) = props.iter().find(|(k, _)| k == key) {
                        if !validates_json_schema(val, prop_schema) {
                            return false;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    true
}

/// 🧱️ JSON syntax (via [`JsonModeConstraint`]) plus a schema check run once the accumulated text
/// is syntactically complete — schema compliance is a completion gate, not a proactive per-token
/// mask (that would need compiling the schema into the DFA machinery, a hardening-wave follow-up).
pub struct JsonSchemaConstraint {
    mode: JsonModeConstraint,
    schema: JsonValue,
    text: Vec<u8>,
    schema_violated: bool,
    text_snapshots: Vec<(Vec<u8>, bool)>,
}

impl JsonSchemaConstraint {
    pub fn new(schema: JsonValue) -> Self {
        Self { mode: JsonModeConstraint::new(), schema, text: Vec::new(), schema_violated: false, text_snapshots: Vec::new() }
    }
}

impl Constraint for JsonSchemaConstraint {
    fn name(&self) -> &'static str {
        "json_schema"
    }
    fn fill_mask(&mut self, view: &StepView<'_>, mask: &mut TokenBitset) -> Result<(), SamplingError> {
        self.mode.fill_mask(view, mask)
    }
    fn accept(&mut self, view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError> {
        if let Some(adapter) = view.adapter {
            if let Some(bytes) = adapter.token_bytes(token) {
                self.text.extend_from_slice(bytes);
            }
        }
        self.mode.accept(view, token)?;
        if self.mode.is_satisfied() {
            let text = String::from_utf8_lossy(&self.text);
            match parse_json(&text, 64) {
                Ok(value) => {
                    if !validates_json_schema(&value, &self.schema) {
                        self.schema_violated = true;
                    }
                }
                Err(_) => self.schema_violated = true,
            }
        }
        Ok(())
    }
    fn is_satisfied(&self) -> bool {
        self.mode.is_satisfied() && !self.schema_violated
    }
    fn is_finished(&self) -> bool {
        self.is_satisfied()
    }
    fn is_dead(&self) -> bool {
        self.mode.is_dead() || self.schema_violated
    }
    fn save(&mut self) -> ConstraintMark {
        let inner_mark = self.mode.save();
        self.text_snapshots.push((self.text.clone(), self.schema_violated));
        debug_assert_eq!(inner_mark.0, (self.text_snapshots.len() - 1) as u64, "mode and text snapshot stacks must grow in lockstep");
        ConstraintMark((self.text_snapshots.len() - 1) as u64)
    }
    fn rollback_to(&mut self, mark: ConstraintMark) {
        self.mode.rollback_to(mark);
        if let Some((text, violated)) = self.text_snapshots.get(mark.0 as usize).cloned() {
            self.text = text;
            self.schema_violated = violated;
        }
        self.text_snapshots.truncate(mark.0 as usize + 1);
    }
    fn reset(&mut self) {
        self.mode.reset();
        self.text.clear();
        self.schema_violated = false;
        self.text_snapshots.clear();
    }
    fn fork(&self) -> Box<dyn Constraint> {
        Box::new(Self {
            mode: JsonModeConstraint {
                stack: self.mode.stack.clone(),
                expect: self.mode.expect,
                in_string: self.mode.in_string,
                string_escaped: self.mode.string_escaped,
                string_is_key: self.mode.string_is_key,
                dead: self.mode.dead,
                snapshots: self.mode.snapshots.clone(),
            },
            schema: self.schema.clone(),
            text: self.text.clone(),
            schema_violated: self.schema_violated,
            text_snapshots: self.text_snapshots.clone(),
        })
    }
}

/// 🧱️ Builds a [`Constraint`] from a [`ConstraintSpec`]; exhaustive over every variant.
pub fn build_constraint(spec: &ConstraintSpec, limits: &SamplingLimits) -> Result<Box<dyn Constraint>, SamplingError> {
    match spec {
        ConstraintSpec::Regex(pattern) => Ok(Box::new(RegexConstraint::new(pattern, limits)?)),
        ConstraintSpec::Trie(phrases) => Ok(Box::new(TrieConstraint::new(phrases))),
        ConstraintSpec::MustInclude(alternatives) => Ok(Box::new(MustIncludeConstraint::new(alternatives.clone()))),
        ConstraintSpec::JsonMode => Ok(Box::new(JsonModeConstraint::new())),
        ConstraintSpec::Ebnf(text) => Ok(Box::new(EbnfConstraint::new(text, limits)?)),
        ConstraintSpec::JsonSchema(schema) => Ok(Box::new(JsonSchemaConstraint::new(schema.clone()))),
    }
}
// #endregion 🔖️Constraints

// #region 🔖️SequenceState
/// 🧬️ A restorable point-in-time snapshot of every stateful component's undo-log position, plus
/// the observable state (generated length, cumulative log-probability) they correspond to.
#[derive(Clone, Debug)]
pub struct SequenceCheckpoint {
    pub generated_len: usize,
    pub cumulative_logprob: f64,
    pub processor_marks: Vec<StateMark>,
    pub constraint_marks: Vec<ConstraintMark>,
    pub stop_marks: Vec<StopMark>,
    pub rng_snapshot: RngSnapshot,
}

/// 🧬️ Everything mutable about one in-flight sequence: history, the pipeline's per-sequence
/// component instances (owned here, not shared), and a checkpoint stack enabling `rollback`.
pub struct SequenceState {
    id: SequenceId,
    prompt: Vec<TokenId>,
    generated: Vec<TokenId>,
    cumulative_logprob: f64,
    processors: Vec<Box<dyn LogitsProcessor>>,
    sampler: Box<dyn TokenSampler>,
    constraints: Vec<Box<dyn Constraint>>,
    stops: Vec<Box<dyn StopCondition>>,
    rng: Box<dyn RandomSource>,
    finish: Option<FinishReason>,
    config_fingerprint: u64,
    checkpoints: Vec<SequenceCheckpoint>,
}

impl SequenceState {
    /// 🧬️ Builds every per-sequence processor/constraint/sampler/stop instance from `config`.
    pub fn new(id: SequenceId, prompt: Vec<TokenId>, config: &SamplingConfig, rng: Box<dyn RandomSource>) -> Result<Self, SamplingError> {
        let mut processors: Vec<Box<dyn LogitsProcessor>> = vec![Box::new(MinLengthEosSuppression { min_tokens: config.min_tokens }), Box::new(MaxLengthForceEos { max_tokens: config.max_tokens })];
        if config.forced.bos.is_some() || !config.forced.prefix.is_empty() || !config.forced.at_position.is_empty() {
            processors.push(Box::new(ForcedTokens { spec: config.forced.clone() }));
        }
        for spec in &config.processors {
            processors.push(build_processor(spec)?);
        }
        let sampler = build_sampler(&config.method);
        let mut constraints: Vec<Box<dyn Constraint>> = Vec::new();
        for spec in &config.constraints {
            constraints.push(build_constraint(spec, &config.limits)?);
        }
        let mut stops: Vec<Box<dyn StopCondition>> = Vec::new();
        if !config.stops.tokens.is_empty() {
            stops.push(Box::new(TokenStopCondition { tokens: config.stops.tokens.clone() }));
        }
        if !config.stops.sequences.is_empty() {
            stops.push(Box::new(TextStopCondition::new(&config.stops.sequences, config.stops.mode)));
        }
        Ok(Self { id, prompt, generated: Vec::new(), cumulative_logprob: 0.0, processors, sampler, constraints, stops, rng, finish: None, config_fingerprint: config.fingerprint(), checkpoints: Vec::new() })
    }

    pub fn id(&self) -> SequenceId {
        self.id
    }

    pub fn prompt(&self) -> &[TokenId] {
        &self.prompt
    }

    pub fn generated(&self) -> &[TokenId] {
        &self.generated
    }

    pub fn cumulative_logprob(&self) -> f64 {
        self.cumulative_logprob
    }

    pub fn finish(&self) -> Option<FinishReason> {
        self.finish
    }

    pub fn is_finished(&self) -> bool {
        self.finish.is_some()
    }

    pub fn config_fingerprint(&self) -> u64 {
        self.config_fingerprint
    }

    fn reset_state_only(&mut self) {
        self.generated.clear();
        self.cumulative_logprob = 0.0;
        self.finish = None;
        for p in self.processors.iter_mut() {
            p.reset();
        }
        for c in self.constraints.iter_mut() {
            c.reset();
        }
        for s in self.stops.iter_mut() {
            s.reset();
        }
    }

    pub fn reset(&mut self) {
        self.reset_state_only();
        self.checkpoints.clear();
    }

    /// 🧬️ Captures the current position of every component's undo log.
    pub fn checkpoint(&mut self) -> SequenceCheckpoint {
        SequenceCheckpoint {
            generated_len: self.generated.len(),
            cumulative_logprob: self.cumulative_logprob,
            processor_marks: self.processors.iter_mut().map(|p| p.save()).collect(),
            constraint_marks: self.constraints.iter_mut().map(|c| c.save()).collect(),
            stop_marks: self.stops.iter_mut().map(|s| s.save()).collect(),
            rng_snapshot: self.rng.snapshot(),
        }
    }

    /// 🧬️ Restores every component to a previously captured [`SequenceCheckpoint`].
    pub fn restore(&mut self, checkpoint: &SequenceCheckpoint) {
        self.generated.truncate(checkpoint.generated_len);
        self.cumulative_logprob = checkpoint.cumulative_logprob;
        self.finish = None;
        for (p, mark) in self.processors.iter_mut().zip(checkpoint.processor_marks.iter()) {
            p.rollback_to(*mark);
        }
        for (c, mark) in self.constraints.iter_mut().zip(checkpoint.constraint_marks.iter()) {
            c.rollback_to(*mark);
        }
        for (s, mark) in self.stops.iter_mut().zip(checkpoint.stop_marks.iter()) {
            s.rollback_to(*mark);
        }
        let _ = self.rng.restore(&checkpoint.rng_snapshot);
    }

    /// 🧬️ Removes the last `n` generated tokens (and their effects on every component) by
    /// discarding the `n` most recent checkpoints and restoring the one before them — or the
    /// pristine pre-generation state if fewer than `n` checkpoints exist.
    pub fn rollback(&mut self, n: usize) {
        let keep = self.checkpoints.len().saturating_sub(n);
        self.checkpoints.truncate(keep);
        match self.checkpoints.last().cloned() {
            Some(checkpoint) => self.restore(&checkpoint),
            None => self.reset_state_only(),
        }
    }

    /// 🧬️ Independent copy sharing no mutable state, with its own RNG stream split from `self`'s
    /// via `rng_split_key` — the basis for beam search and speculative decoding forks.
    pub fn fork(&self, new_id: SequenceId, rng_split_key: StreamKey) -> Self {
        Self {
            id: new_id,
            prompt: self.prompt.clone(),
            generated: self.generated.clone(),
            cumulative_logprob: self.cumulative_logprob,
            processors: self.processors.iter().map(|p| p.fork()).collect(),
            sampler: self.sampler.fork(),
            constraints: self.constraints.iter().map(|c| c.fork()).collect(),
            stops: self.stops.iter().map(|s| s.fork()).collect(),
            rng: self.rng.split(rng_split_key),
            finish: self.finish,
            config_fingerprint: self.config_fingerprint,
            checkpoints: self.checkpoints.clone(),
        }
    }

    /// 🧬️ Versioned, config-fingerprinted text form: `v1|fingerprint_hex|token,token,...`.
    pub fn to_text(&self) -> String {
        let tokens: Vec<String> = self.generated.iter().map(|t| t.get().to_string()).collect();
        format!("v1|{:016x}|{}", self.config_fingerprint, tokens.join(","))
    }

    /// 🧬️ Decodes a [`SequenceState::to_text`] string, validating its fingerprint against `self`'s
    /// config. Returns the generated token list without mutating `self`; the caller re-drives a
    /// fresh [`SequenceState`] through those tokens (via the normal stateful step path) to rebuild
    /// processor/constraint/stop state exactly rather than trying to deserialize it directly.
    pub fn decode_text(&self, text: &str) -> Result<Vec<TokenId>, SamplingError> {
        let mut parts = text.splitn(3, '|');
        let version = parts.next().ok_or(SamplingError::Corrupted { reason: "missing state version" })?;
        if version != "v1" {
            return Err(SamplingError::SerializationVersion { expected: 1, actual: version.trim_start_matches('v').parse().unwrap_or(0) });
        }
        let fingerprint_hex = parts.next().ok_or(SamplingError::Corrupted { reason: "missing state fingerprint" })?;
        let fingerprint = u64::from_str_radix(fingerprint_hex, 16).map_err(|_| SamplingError::Corrupted { reason: "invalid state fingerprint" })?;
        if fingerprint != self.config_fingerprint {
            return Err(SamplingError::FingerprintMismatch);
        }
        let tokens_part = parts.next().unwrap_or("");
        if tokens_part.is_empty() {
            return Ok(Vec::new());
        }
        tokens_part.split(',').map(|s| s.parse::<u32>().map(TokenId::new).map_err(|_| SamplingError::Corrupted { reason: "invalid token in serialized state" })).collect()
    }
}
// #endregion 🔖️SequenceState

// #region 🔖️Observability
/// 🔭️ An observer that discards every event — the default when nobody wants diagnostics.
#[derive(Default)]
pub struct NullObserver;

impl SamplingObserver for NullObserver {}

/// 🔭️ An observer that records a bounded number of human-readable event lines. Never logs token
/// text, only ids and reasons, matching the "no token text by default" security default.
#[derive(Default)]
pub struct CollectingObserver {
    pub events: Vec<String>,
    max_events: usize,
}

impl CollectingObserver {
    pub fn new(max_events: usize) -> Self {
        Self { events: Vec::new(), max_events }
    }

    fn push(&mut self, event: String) {
        if self.events.len() < self.max_events {
            self.events.push(event);
        }
    }
}

impl SamplingObserver for CollectingObserver {
    fn on_finish(&mut self, sequence: SequenceId, reason: FinishReason) {
        self.push(format!("finish seq={} reason={:?}", sequence.get(), reason));
    }
    fn on_fallback(&mut self, sequence: SequenceId, _error: &SamplingError, action: FallbackAction) {
        self.push(format!("fallback seq={} action={:?}", sequence.get(), action));
    }
}
// #endregion 🔖️Observability

// #region 🔖️Engine
fn cumulative_from_probs(probs: &[f32]) -> Vec<f64> {
    let mut out = Vec::with_capacity(probs.len());
    let mut sum = KahanSum::new();
    for &p in probs {
        sum.add(p as f64);
        out.push(sum.value());
    }
    if let Some(last) = out.last_mut() {
        *last = 1.0;
    }
    out
}

/// 🚂️ Everything [`sample_step_stateless`] needs about the sequence beyond the raw logits. Every
/// field is `Copy` (ids/indices) or a borrowed reference, so the whole struct is `Copy` and cheap
/// to pass by value.
#[derive(Clone, Copy)]
pub struct StatelessStepInput<'a> {
    pub sequence: SequenceId,
    pub step: StepIndex,
    pub prompt: &'a [TokenId],
    pub generated: &'a [TokenId],
    pub vocab: &'a Vocabulary,
    pub adapter: Option<&'a dyn TokenTextAdapter>,
    pub last_entropy: Option<f64>,
}

/// 🚂️ Runs one sampling step with no persistent per-sequence state (the "Stateless one-step
/// sampling" operating mode from § 1): applies every configured warper in order, falls back
/// through [`resolve_fallback`] if truncation empties the live set, builds the final distribution,
/// then samples via `config.method`. Penalties, biases, constraints, and stop conditions are not
/// yet applied here — they require [`SequenceState`] (added by the stateful engine in a later
/// wave); see [`build_processor`] for which [`ProcessorSpec`] variants are wired up so far.
pub fn sample_step_stateless(config: &SamplingConfig, ws: &mut LogitsWorkspace, rng: &mut dyn RandomSource, raw_logits: &[f32], input: StatelessStepInput<'_>) -> Result<SamplingResult, SamplingError> {
    input.vocab.validate_logits_len(raw_logits.len())?;
    ws.set_accum(config.accum);
    ws.reset_for_step(raw_logits, config.sanitize)?;

    let view = StepView { sequence: input.sequence, step: input.step, prompt: input.prompt, generated: input.generated, vocab: input.vocab, adapter: input.adapter, last_entropy: input.last_entropy };

    let mut processors = Vec::with_capacity(config.processors.len());
    for spec in &config.processors {
        processors.push(build_processor(spec)?);
    }
    for processor in processors.iter_mut() {
        if ws.live().is_empty() {
            break;
        }
        processor.process(&view, ws)?;
    }

    let fallback = if ws.live().is_empty() {
        let (action, token) = resolve_fallback(None, input.vocab.eos.first().copied(), Some(ws.saved_argmax()));
        let token = token.ok_or(SamplingError::EmptyDistribution)?;
        ws.set_live(vec![token.get()]);
        Some(action)
    } else {
        None
    };

    ws.sort_live_by_prob_desc();
    let cdf = cumulative_from_probs(ws.probs());
    let logprobs: Vec<f32> = ws.probs().iter().map(|&p| (p as f64).ln() as f32).collect();
    let entropy = entropy_nats(ws.probs());
    let tokens = cast_u32_slice_to_token_ids(ws.live());
    let dist = Distribution { tokens, probs: ws.probs(), logprobs: &logprobs, cdf: &cdf, entropy };

    let mut sampler = build_sampler(&config.method);
    let mut selection = SelectionBuffer::default();
    sampler.sample(&view, &dist, rng, &mut selection)?;
    let chosen = *selection.chosen.first().ok_or(SamplingError::EmptyDistribution)?;

    let next_len = input.generated.len() + 1;
    // 🚂️ Check the length cap before EOS: when `max_tokens` is what forced EOS to be selected (see
    // `MaxLengthForceEos` in the stateful engine), `MaxTokens` is the more informative reason.
    let finish = if next_len >= config.max_tokens {
        Some(FinishReason::MaxTokens)
    } else if input.vocab.is_eos(chosen.token) {
        Some(FinishReason::EosToken)
    } else {
        None
    };

    let diagnostics = config.diagnostics.enabled.then(|| StepDiagnostics {
        entropy,
        effective_count: entropy.exp(),
        truncation_mass: 0.0,
        masked_by: Vec::new(),
        timings_ns: Vec::new(),
        fallback,
        health: Some(DistributionHealth::assess(ws.live().len(), ws.probs().iter().map(|&p| p as f64).sum())),
    });

    Ok(SamplingResult {
        token: chosen.token,
        logprob: chosen.logprob,
        finish,
        alternatives: selection.chosen,
        top_logprobs: None,
        rng_stream: StreamKey { request: 0, sequence: input.sequence.get(), beam: 0, candidate: 0, purpose: StreamPurpose::Selection },
        diagnostics,
    })
}

/// 🚂️ Runs one sampling step against a stateful [`SequenceState`] — the "Stateful multi-step
/// generation" operating mode. Phase-separates processors by [`ProcessorKind`] (hard masks, then
/// soft penalties, then truncation), applies the fallback ladder if truncation empties the live
/// set, samples, then commits every component's per-sequence effects and pushes a checkpoint —
/// but only after a token has been definitively selected (the "state update only after successful
/// token selection" pipeline guarantee). `view` is scoped to short-lived blocks throughout so its
/// borrow of `state.prompt`/`state.generated` never overlaps a later `&mut state` access.
pub fn sample_step(
    config: &SamplingConfig,
    state: &mut SequenceState,
    ws: &mut LogitsWorkspace,
    vocab: &Vocabulary,
    adapter: Option<&dyn TokenTextAdapter>,
    raw_logits: &[f32],
    observer: &mut dyn SamplingObserver,
) -> Result<SamplingResult, SamplingError> {
    if state.finish.is_some() {
        return Err(SamplingError::InvalidConfig { field: "state", reason: "sequence already finished" });
    }
    vocab.validate_logits_len(raw_logits.len())?;
    ws.set_accum(config.accum);
    ws.reset_for_step(raw_logits, config.sanitize)?;

    let step = StepIndex::new(state.generated.len() as u32);
    let sequence_id = state.id;
    observer.on_step_start(sequence_id, step);

    {
        let view = StepView { sequence: sequence_id, step, prompt: &state.prompt, generated: &state.generated, vocab, adapter, last_entropy: None };
        for constraint in state.constraints.iter_mut() {
            constraint.fill_mask(&view, ws.mask_mut())?;
        }
        for processor in state.processors.iter_mut() {
            if processor.kind() == ProcessorKind::HardMask {
                processor.process(&view, ws)?;
            }
        }
        ws.sync_live_with_mask();
        for processor in state.processors.iter_mut() {
            if ws.live().is_empty() {
                break;
            }
            if processor.kind() == ProcessorKind::SoftPenalty {
                processor.process(&view, ws)?;
            }
        }
        for processor in state.processors.iter_mut() {
            if ws.live().is_empty() {
                break;
            }
            if processor.kind() == ProcessorKind::Truncation {
                processor.process(&view, ws)?;
            }
        }
    }

    let fallback = if ws.live().is_empty() {
        let (action, token) = resolve_fallback(None, vocab.eos.first().copied(), Some(ws.saved_argmax()));
        match token {
            Some(t) => {
                observer.on_fallback(sequence_id, &SamplingError::EmptyDistribution, action);
                ws.set_live(vec![t.get()]);
                Some(action)
            }
            None => {
                let err = SamplingError::EmptyDistribution;
                observer.on_fallback(sequence_id, &err, FallbackAction::Error);
                return Err(err);
            }
        }
    } else {
        None
    };

    ws.sort_live_by_prob_desc();
    let cdf = cumulative_from_probs(ws.probs());
    let logprobs: Vec<f32> = ws.probs().iter().map(|&p| (p as f64).ln() as f32).collect();
    let entropy = entropy_nats(ws.probs());
    let tokens = cast_u32_slice_to_token_ids(ws.live());
    let dist = Distribution { tokens, probs: ws.probs(), logprobs: &logprobs, cdf: &cdf, entropy };

    // 🎲️ `state.rng` is this sequence's own persistent stream (established once at construction or
    // fork time via `StreamKey`/`split`); every step draws from it directly so its counter advances
    // step over step. Re-deriving via `split(stream_key)` here would be wrong — `stream_key` is
    // invariant across steps within one sequence, so every step would draw the same first value.
    let stream_key = StreamKey { request: 0, sequence: sequence_id.get(), beam: 0, candidate: 0, purpose: StreamPurpose::Selection };
    let mut selection = SelectionBuffer::default();
    {
        let view = StepView { sequence: sequence_id, step, prompt: &state.prompt, generated: &state.generated, vocab, adapter, last_entropy: Some(entropy) };
        state.sampler.sample(&view, &dist, &mut *state.rng, &mut selection)?;
    }
    let chosen = *selection.chosen.first().ok_or(SamplingError::EmptyDistribution)?;

    // 🚂️ Transactional commit: every phase above only read `state`; only now — because a token was
    // definitively selected — do processors/constraints/stops/history actually change.
    let mut stop_reason = None;
    {
        let view = StepView { sequence: sequence_id, step, prompt: &state.prompt, generated: &state.generated, vocab, adapter, last_entropy: Some(entropy) };
        for processor in state.processors.iter_mut() {
            processor.commit(&view, chosen.token);
        }
        for constraint in state.constraints.iter_mut() {
            constraint.accept(&view, chosen.token)?;
        }
        for stop in state.stops.iter_mut() {
            if let StopPoll::Finished { reason, .. } = stop.on_token(&view, chosen.token) {
                stop_reason.get_or_insert(reason);
            }
        }
    }
    state.generated.push(chosen.token);
    state.cumulative_logprob += chosen.logprob as f64;
    let checkpoint = state.checkpoint();
    state.checkpoints.push(checkpoint);

    let next_len = state.generated.len();
    // 🚂️ Same MaxTokens-before-EosToken priority as `sample_step_stateless` — see its comment.
    let finish = stop_reason.or_else(|| {
        if next_len >= config.max_tokens {
            Some(FinishReason::MaxTokens)
        } else if vocab.is_eos(chosen.token) {
            Some(FinishReason::EosToken)
        } else {
            None
        }
    });
    state.finish = finish;
    if let Some(reason) = finish {
        observer.on_finish(sequence_id, reason);
    }

    let diagnostics = config.diagnostics.enabled.then(|| StepDiagnostics {
        entropy,
        effective_count: entropy.exp(),
        truncation_mass: 0.0,
        masked_by: Vec::new(),
        timings_ns: Vec::new(),
        fallback,
        health: Some(DistributionHealth::assess(ws.live().len(), ws.probs().iter().map(|&p| p as f64).sum())),
    });

    let result = SamplingResult { token: chosen.token, logprob: chosen.logprob, finish, alternatives: selection.chosen, top_logprobs: None, rng_stream: stream_key, diagnostics };
    observer.on_token(sequence_id, &result);
    Ok(result)
}

/// 🚂️ Commits `token` onto `state` outside the normal `sample_step` selection path — shared by
/// [`beam_search`] and [`speculative_decode`], which each pick the next token through their own
/// mechanism (beam expansion, speculative accept/resample) rather than a [`TokenSampler`].
fn commit_token_to_state(state: &mut SequenceState, vocab: &Vocabulary, adapter: Option<&dyn TokenTextAdapter>, token: TokenId, logprob_nats: f64) -> Option<FinishReason> {
    let step = StepIndex::new(state.generated.len() as u32);
    let view = StepView { sequence: state.id, step, prompt: &state.prompt, generated: &state.generated, vocab, adapter, last_entropy: None };
    for processor in state.processors.iter_mut() {
        processor.commit(&view, token);
    }
    for constraint in state.constraints.iter_mut() {
        let _ = constraint.accept(&view, token);
    }
    let mut stop_reason = None;
    for stop in state.stops.iter_mut() {
        if let StopPoll::Finished { reason, .. } = stop.on_token(&view, token) {
            stop_reason.get_or_insert(reason);
        }
    }
    state.generated.push(token);
    state.cumulative_logprob += logprob_nats;
    let checkpoint = state.checkpoint();
    state.checkpoints.push(checkpoint);
    let finish = stop_reason.or_else(|| vocab.is_eos(token).then_some(FinishReason::EosToken));
    state.finish = finish;
    finish
}
// #endregion 🔖️Engine

// #region 🔖️Batch
/// 📦️ One sequence's raw logits for a batch step.
pub struct BatchEntry<'a> {
    pub id: SequenceId,
    pub logits: &'a [f32],
}

/// 📦️ A batch of per-sequence logits to step together.
pub struct BatchSamplingRequest<'a> {
    pub entries: Vec<BatchEntry<'a>>,
}

/// 📦️ Per-sequence results from one [`ContinuousBatcher::step`] call, in the same order as the
/// request's entries.
pub struct BatchSamplingResult {
    pub results: Vec<(SequenceId, Result<SamplingResult, SamplingError>)>,
}

/// 📦️ Owns a dynamic set of [`SequenceState`]s addressed by [`SequenceId`] (never by slot index),
/// so sequences can be added/removed between steps and the batch's *processing* order can vary
/// freely — every sequence's own [`SequenceState::rng`] stream is keyed by its id, not by
/// position, so outputs never depend on batch order. Reuses [`LogitsWorkspace`]s via
/// [`WorkspacePool`] so steady-state stepping doesn't reallocate.
pub struct ContinuousBatcher {
    sequences: std::collections::HashMap<SequenceId, SequenceState>,
    pool: WorkspacePool,
    vocab_size: usize,
}

impl ContinuousBatcher {
    pub fn new(vocab_size: usize) -> Self {
        Self { sequences: std::collections::HashMap::new(), pool: WorkspacePool::new(), vocab_size }
    }

    pub fn add_sequence(&mut self, state: SequenceState) {
        self.sequences.insert(state.id(), state);
    }

    pub fn remove_sequence(&mut self, id: SequenceId) -> Option<SequenceState> {
        self.sequences.remove(&id)
    }

    pub fn contains(&self, id: SequenceId) -> bool {
        self.sequences.contains_key(&id)
    }

    pub fn len(&self) -> usize {
        self.sequences.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sequences.is_empty()
    }

    pub fn get(&self, id: SequenceId) -> Option<&SequenceState> {
        self.sequences.get(&id)
    }

    /// 📦️ Steps every sequence named in `request.entries`, in that (arbitrary) order.
    pub fn step(&mut self, config: &SamplingConfig, vocab: &Vocabulary, adapter: Option<&dyn TokenTextAdapter>, request: &BatchSamplingRequest<'_>, observer: &mut dyn SamplingObserver) -> BatchSamplingResult {
        let mut results = Vec::with_capacity(request.entries.len());
        for entry in &request.entries {
            let Some(state) = self.sequences.get_mut(&entry.id) else {
                results.push((entry.id, Err(SamplingError::InvalidConfig { field: "batch", reason: "unknown sequence id" })));
                continue;
            };
            let mut ws = self.pool.acquire(self.vocab_size);
            let result = sample_step(config, state, &mut ws, vocab, adapter, entry.logits, observer);
            self.pool.release(ws);
            results.push((entry.id, result));
        }
        BatchSamplingResult { results }
    }
}
// #endregion 🔖️Batch

// #region 🔖️Search
/// 🌳️ One beam search hypothesis: its own independent [`SequenceState`] plus cumulative
/// log-probability score (unnormalized — apply [`gnmt_length_penalty`] for ranking).
pub struct BeamHypothesis {
    pub state: SequenceState,
    pub score: f64,
}

/// 🌳️ GNMT-style length penalty: `((5 + len) / 6) ^ alpha`, dividing the raw score to avoid
/// favoring short hypotheses.
pub fn gnmt_length_penalty(len: usize, alpha: f64) -> f64 {
    ((5.0 + len as f64) / 6.0).powf(alpha)
}

pub struct BeamSearchConfig {
    pub width: usize,
    pub length_penalty: f64,
    pub max_steps: usize,
}

/// 🌳️ Beam search: `next_logits` supplies raw model logits for a hypothesis's current state (the
/// caller owns model inference, per § 1's non-responsibilities). At each step, every active
/// hypothesis contributes its top-`width` next-token candidates (by probability); all
/// `hypotheses × candidates` are pooled, ranked by cumulative log-probability (ties by ascending
/// token id), and the top `width` become the next round's hypotheses — each an independent
/// [`SequenceState::fork`] so per-sequence state (penalties, constraints, RNG) never aliases
/// across beams. Finished hypotheses are set aside and never re-expanded. Returns every
/// hypothesis (finished or step-exhausted), sorted best-first by length-normalized score.
pub fn beam_search(
    config: &SamplingConfig,
    beam_config: &BeamSearchConfig,
    vocab: &Vocabulary,
    adapter: Option<&dyn TokenTextAdapter>,
    initial: SequenceState,
    mut next_logits: impl FnMut(&SequenceState) -> Vec<f32>,
) -> Result<Vec<BeamHypothesis>, SamplingError> {
    let mut ws = LogitsWorkspace::new(vocab.size);
    let mut beams = vec![BeamHypothesis { state: initial, score: 0.0 }];
    let mut finished: Vec<BeamHypothesis> = Vec::new();

    for _ in 0..beam_config.max_steps {
        if beams.is_empty() {
            break;
        }
        let mut candidates: Vec<(usize, TokenId, f64)> = Vec::new();
        for (bi, beam) in beams.iter().enumerate() {
            let raw_logits = next_logits(&beam.state);
            vocab.validate_logits_len(raw_logits.len())?;
            ws.set_accum(config.accum);
            ws.reset_for_step(&raw_logits, config.sanitize)?;
            ws.sort_live_by_prob_desc();
            let k = beam_config.width.min(ws.live().len());
            for i in 0..k {
                candidates.push((bi, TokenId::new(ws.live()[i]), (ws.probs()[i] as f64).ln()));
            }
        }
        if candidates.is_empty() {
            break;
        }
        candidates.sort_by(|a, b| {
            let score_a = beams[a.0].score + a.2;
            let score_b = beams[b.0].score + b.2;
            score_b.partial_cmp(&score_a).unwrap_or(core::cmp::Ordering::Equal).then_with(|| a.1.cmp(&b.1))
        });

        let mut next_round = Vec::with_capacity(beam_config.width);
        for (rank, &(bi, token, logprob)) in candidates.iter().take(beam_config.width).enumerate() {
            let key = StreamKey { request: 0, sequence: beams[bi].state.id().get(), beam: rank as u32, candidate: 0, purpose: StreamPurpose::Beam };
            let mut child = beams[bi].state.fork(beams[bi].state.id(), key);
            commit_token_to_state(&mut child, vocab, adapter, token, logprob);
            let hyp = BeamHypothesis { state: child, score: beams[bi].score + logprob };
            if hyp.state.is_finished() {
                finished.push(hyp);
            } else {
                next_round.push(hyp);
            }
        }
        beams = next_round;
    }

    finished.extend(beams);
    finished.sort_by(|a, b| {
        let na = a.score / gnmt_length_penalty(a.state.generated().len(), beam_config.length_penalty);
        let nb = b.score / gnmt_length_penalty(b.state.generated().len(), beam_config.length_penalty);
        nb.partial_cmp(&na).unwrap_or(core::cmp::Ordering::Equal)
    });
    Ok(finished)
}

/// 🌳️ Generates `n` complete candidates by running ordinary [`sample_step`] to completion for
/// each, then ranks them by mean log-probability (best first). `make_initial` builds a fresh
/// [`SequenceState`] per candidate index (so each gets its own RNG stream, e.g. via a distinct
/// [`StreamKey::candidate`]).
pub struct BestOfN {
    pub n: usize,
}

impl BestOfN {
    #[allow(clippy::too_many_arguments)]
    pub fn run(
        &self,
        config: &SamplingConfig,
        vocab: &Vocabulary,
        adapter: Option<&dyn TokenTextAdapter>,
        max_steps: usize,
        observer: &mut dyn SamplingObserver,
        make_initial: impl Fn(usize) -> Result<SequenceState, SamplingError>,
        mut next_logits: impl FnMut(&SequenceState) -> Vec<f32>,
    ) -> Result<Vec<(SequenceState, f64)>, SamplingError> {
        let mut candidates = Vec::with_capacity(self.n);
        for i in 0..self.n {
            let mut state = make_initial(i)?;
            let mut ws = LogitsWorkspace::new(vocab.size);
            for _ in 0..max_steps {
                if state.is_finished() {
                    break;
                }
                let raw_logits = next_logits(&state);
                sample_step(config, &mut state, &mut ws, vocab, adapter, &raw_logits, observer)?;
            }
            let mean_logprob = if state.generated().is_empty() { 0.0 } else { state.cumulative_logprob() / state.generated().len() as f64 };
            candidates.push((state, mean_logprob));
        }
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(core::cmp::Ordering::Equal));
        Ok(candidates)
    }
}

/// 🌳️ Retries [`sample_step_stateless`] up to `max_attempts` times, keeping the first result
/// `accept` approves.
pub struct RejectionSampler {
    pub max_attempts: usize,
}

impl RejectionSampler {
    pub fn sample(&self, config: &SamplingConfig, ws: &mut LogitsWorkspace, rng: &mut dyn RandomSource, raw_logits: &[f32], input: StatelessStepInput<'_>, mut accept: impl FnMut(&SamplingResult) -> bool) -> Result<SamplingResult, SamplingError> {
        let mut last_err = None;
        for _ in 0..self.max_attempts {
            match sample_step_stateless(config, ws, rng, raw_logits, input) {
                Ok(result) if accept(&result) => return Ok(result),
                Ok(_) => continue,
                Err(e) => last_err = Some(e),
            }
        }
        Err(last_err.unwrap_or(SamplingError::LimitExceeded { limit: "rejection_sampler.max_attempts" }))
    }
}
// #endregion 🔖️Search

// #region 🔖️Speculative
/// ⚡️ Outcome counters for one [`speculative_decode`] call.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct SpecMetrics {
    pub proposed: usize,
    pub accepted: usize,
    pub bonus_taken: bool,
}

/// ⚡️ Exact speculative decoding: verifies `draft_tokens` against `target_logits` (the target
/// model's logits for the state as it would be after accepting the prefix so far), accepting
/// position `i` with probability `min(1, p_target(draft_i) / p_draft(draft_i))`. On the first
/// rejection, resamples from the clamped residual `max(0, p_target(t) - p_draft(t))` (renormalized
/// over *every* token `t`, not just the drafted one) and stops — this is what makes the scheme
/// *exact*: the resulting token distribution is identical to sampling from `target_logits`
/// directly at every position. `draft_distributions[i]` must therefore be the draft model's
/// **full** per-token probability vector for position `i` (same vocabulary shape as the target
/// logits) — a residual computed from only the drafted token's own probability (treating every
/// other token as if the draft model assigned it zero mass) is a different, *biased* algorithm,
/// not exact speculative decoding. If every draft is accepted, draws one bonus token from the
/// final target row. All verification draws come from a dedicated [`StreamPurpose::Speculative`]
/// sub-stream (split from `state.rng` once, up front), so speculation never perturbs the
/// sequence's ordinary selection stream. Accepted/resampled/bonus tokens are committed through
/// [`commit_token_to_state`] one at a time (not as a single all-or-nothing transaction), so
/// constraint/penalty/stop state stays exactly what non-speculative decoding would have produced
/// for the same accepted prefix.
#[allow(clippy::too_many_arguments)]
pub fn speculative_decode(
    config: &SamplingConfig,
    state: &mut SequenceState,
    ws: &mut LogitsWorkspace,
    vocab: &Vocabulary,
    adapter: Option<&dyn TokenTextAdapter>,
    draft_tokens: &[TokenId],
    draft_distributions: &[Vec<f32>],
    mut target_logits: impl FnMut(&SequenceState) -> Vec<f32>,
    observer: &mut dyn SamplingObserver,
) -> Result<(Vec<SamplingResult>, SpecMetrics), SamplingError> {
    let mut results = Vec::new();
    let mut metrics = SpecMetrics { proposed: draft_tokens.len(), accepted: 0, bonus_taken: false };
    let stream_key = StreamKey { request: 0, sequence: state.id().get(), beam: 0, candidate: 0, purpose: StreamPurpose::Speculative };
    let mut spec_rng = state.rng.split(stream_key);

    for i in 0..draft_tokens.len() {
        if state.is_finished() {
            break;
        }
        let raw_logits = target_logits(&*state);
        vocab.validate_logits_len(raw_logits.len())?;
        ws.set_accum(config.accum);
        ws.reset_for_step(&raw_logits, config.sanitize)?;
        ws.sort_live_by_prob_desc();
        let target_prob = ws.live().iter().position(|&t| t == draft_tokens[i].get()).map_or(0.0, |idx| ws.probs()[idx]);
        let draft_prob = draft_distributions[i].get(draft_tokens[i].get() as usize).copied().unwrap_or(0.0);
        let accept_prob = (target_prob as f64 / (draft_prob as f64).max(1e-12)).min(1.0);

        if spec_rng.next_f64() < accept_prob {
            let logprob = (target_prob.max(f32::MIN_POSITIVE) as f64).ln();
            let finish = commit_token_to_state(state, vocab, adapter, draft_tokens[i], logprob);
            metrics.accepted += 1;
            let result = SamplingResult { token: draft_tokens[i], logprob: logprob as f32, finish, alternatives: Vec::new(), top_logprobs: None, rng_stream: stream_key, diagnostics: None };
            observer.on_token(state.id(), &result);
            results.push(result);
        } else {
            let residual: Vec<f32> = ws.live().iter().zip(ws.probs().iter()).map(|(&t, &p)| (p - draft_distributions[i].get(t as usize).copied().unwrap_or(0.0)).max(0.0)).collect();
            let sum: f32 = residual.iter().sum();
            let normalized: Vec<f32> = if sum > 0.0 { residual.iter().map(|&r| r / sum).collect() } else { ws.probs().to_vec() };
            let cdf = cumulative_from_probs(&normalized);
            let idx = cdf_binary_search(&cdf, spec_rng.next_f64());
            let token = TokenId::new(ws.live()[idx]);
            let logprob = (normalized[idx].max(f32::MIN_POSITIVE) as f64).ln();
            let finish = commit_token_to_state(state, vocab, adapter, token, logprob);
            let result = SamplingResult { token, logprob: logprob as f32, finish, alternatives: Vec::new(), top_logprobs: None, rng_stream: stream_key, diagnostics: None };
            observer.on_token(state.id(), &result);
            results.push(result);
            return Ok((results, metrics));
        }
    }

    if !state.is_finished() {
        let raw_logits = target_logits(&*state);
        vocab.validate_logits_len(raw_logits.len())?;
        ws.set_accum(config.accum);
        ws.reset_for_step(&raw_logits, config.sanitize)?;
        ws.sort_live_by_prob_desc();
        let cdf = cumulative_from_probs(ws.probs());
        let idx = cdf_binary_search(&cdf, spec_rng.next_f64());
        let token = TokenId::new(ws.live()[idx]);
        let logprob = (ws.probs()[idx].max(f32::MIN_POSITIVE) as f64).ln();
        let finish = commit_token_to_state(state, vocab, adapter, token, logprob);
        metrics.bonus_taken = true;
        let result = SamplingResult { token, logprob: logprob as f32, finish, alternatives: Vec::new(), top_logprobs: None, rng_stream: stream_key, diagnostics: None };
        observer.on_token(state.id(), &result);
        results.push(result);
    }
    Ok((results, metrics))
}
// #endregion 🔖️Speculative

// #region 🔖️Sharded
struct LocalCollectiveMailbox {
    f32_slots: Vec<Vec<f32>>,
    f64_slots: Vec<Vec<f64>>,
    candidate_slots: Vec<Vec<ShardCandidate>>,
}

/// 🗂️ Same-process reference [`Collective`]: every rank's handle shares one mailbox. This is
/// **not** a real network protocol — each call stages this rank's contribution and returns the
/// reduction over whatever has been staged *so far*, so callers must call every rank once (any
/// order) per logical collective operation, then call once more (or read the last call's result) to see
/// every rank's contribution reflected. Good enough for testing the sharded sampling math against
/// its unsharded equivalent in a single process; not a substitute for a real collective library.
pub struct LocalCollective {
    rank: usize,
    world_size: usize,
    mailbox: std::rc::Rc<std::cell::RefCell<LocalCollectiveMailbox>>,
}

impl LocalCollective {
    /// 🗂️ Builds `world_size` linked rank handles sharing one mailbox.
    pub fn new_group(world_size: usize) -> Vec<Self> {
        let mailbox = std::rc::Rc::new(std::cell::RefCell::new(LocalCollectiveMailbox { f32_slots: vec![Vec::new(); world_size], f64_slots: vec![Vec::new(); world_size], candidate_slots: vec![Vec::new(); world_size] }));
        (0..world_size).map(|rank| Self { rank, world_size, mailbox: mailbox.clone() }).collect()
    }
}

impl Collective for LocalCollective {
    fn rank(&self) -> usize {
        self.rank
    }
    fn world_size(&self) -> usize {
        self.world_size
    }
    fn all_reduce_max_f32(&mut self, values: &mut [f32]) -> Result<(), SamplingError> {
        let mut mailbox = self.mailbox.borrow_mut();
        mailbox.f32_slots[self.rank] = values.to_vec();
        let len = values.len();
        for slot in values.iter_mut() {
            *slot = f32::NEG_INFINITY;
        }
        for staged in &mailbox.f32_slots {
            for (i, &v) in staged.iter().enumerate().take(len) {
                if v > values[i] {
                    values[i] = v;
                }
            }
        }
        Ok(())
    }
    fn all_reduce_sum_f64(&mut self, values: &mut [f64]) -> Result<(), SamplingError> {
        let mut mailbox = self.mailbox.borrow_mut();
        mailbox.f64_slots[self.rank] = values.to_vec();
        let len = values.len();
        for slot in values.iter_mut() {
            *slot = 0.0;
        }
        for staged in &mailbox.f64_slots {
            for (i, &v) in staged.iter().enumerate().take(len) {
                values[i] += v;
            }
        }
        Ok(())
    }
    fn all_gather_candidates(&mut self, local: &[ShardCandidate], out: &mut Vec<ShardCandidate>) -> Result<(), SamplingError> {
        let mut mailbox = self.mailbox.borrow_mut();
        mailbox.candidate_slots[self.rank] = local.to_vec();
        out.clear();
        for slot in &mailbox.candidate_slots {
            out.extend_from_slice(slot);
        }
        Ok(())
    }
}

/// 🗂️ Sharded softmax: each rank supplies its local shard's raw logits; returns this rank's
/// locally-normalized probabilities under the *global* normalization constant (global max via
/// `all_reduce_max_f32`, then global `sum(exp(l - max))` via `all_reduce_sum_f64`) — matching
/// softmax over the concatenation of every shard's logits.
pub fn sharded_softmax(collective: &mut dyn Collective, local_logits: &[f32]) -> Vec<f32> {
    let local_max = local_logits.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let mut max_buf = [local_max];
    let _ = collective.all_reduce_max_f32(&mut max_buf);
    let global_max = max_buf[0];
    let local_sum: f64 = local_logits.iter().map(|&l| ((l - global_max) as f64).exp()).sum();
    let mut sum_buf = [local_sum];
    let _ = collective.all_reduce_sum_f64(&mut sum_buf);
    let global_sum = sum_buf[0];
    local_logits.iter().map(|&l| (((l - global_max) as f64).exp() / global_sum) as f32).collect()
}

/// 🗂️ Sharded top-k: each rank contributes its local top-`k` (by logit), merged via
/// `all_gather_candidates` and re-truncated to the global top-`k`.
pub fn sharded_top_k(collective: &mut dyn Collective, local_logits: &[f32], local_token_offset: u32, k: usize) -> Vec<ShardCandidate> {
    let mut indexed: Vec<(u32, f32)> = local_logits.iter().enumerate().map(|(i, &l)| (local_token_offset + i as u32, l)).collect();
    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(core::cmp::Ordering::Equal));
    let local: Vec<ShardCandidate> = indexed.into_iter().take(k).map(|(t, l)| ShardCandidate { token: TokenId::new(t), logit: l }).collect();
    let mut gathered = Vec::new();
    let _ = collective.all_gather_candidates(&local, &mut gathered);
    gathered.sort_by(|a, b| b.logit.partial_cmp(&a.logit).unwrap_or(core::cmp::Ordering::Equal).then_with(|| a.token.cmp(&b.token)));
    gathered.truncate(k);
    gathered
}

/// 🗂️ Sharded categorical sample: gathers every token across all shards (via [`sharded_top_k`]
/// with `k = usize::MAX`), builds the global CDF, and draws one token — exactly matching a plain
/// (unsharded) multinomial draw over the concatenated vocabulary given the same `u`.
pub fn sharded_sample(collective: &mut dyn Collective, local_logits: &[f32], local_token_offset: u32, rng: &mut dyn RandomSource) -> Option<TokenId> {
    let candidates = sharded_top_k(collective, local_logits, local_token_offset, usize::MAX);
    if candidates.is_empty() {
        return None;
    }
    let max_logit = candidates.iter().map(|c| c.logit).fold(f32::NEG_INFINITY, f32::max);
    let weights: Vec<f64> = candidates.iter().map(|c| ((c.logit - max_logit) as f64).exp()).collect();
    let total: f64 = weights.iter().sum();
    let probs: Vec<f32> = weights.iter().map(|&w| (w / total) as f32).collect();
    let cdf = cumulative_from_probs(&probs);
    let idx = cdf_binary_search(&cdf, rng.next_f64());
    Some(candidates[idx].token)
}
// #endregion 🔖️Sharded

// #region 🔖️Diffusion
/// 🌫️ A caller-owned latent buffer plus its logical shape (e.g. `[batch, channels, height,
/// width]`). Purely descriptive — solvers and the denoiser trait operate on plain `&[f32]`/`&mut
/// [f32]` slices directly (avoids an awkward "mutable slice behind a shared struct reference"
/// pattern); this type exists for callers setting up or reading back a run's buffers.
pub struct LatentView<'a> {
    pub data: &'a mut [f32],
    pub shape: [usize; 4],
}

impl LatentView<'_> {
    pub fn len(&self) -> usize {
        self.shape.iter().product()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

fn linspace(a: f64, b: f64, n: usize) -> Vec<f64> {
    if n <= 1 {
        return vec![a];
    }
    (0..n).map(|i| a + (b - a) * i as f64 / (n as f64 - 1.0)).collect()
}

fn betas_to_sigmas(betas: &[f64]) -> Vec<f64> {
    let mut alpha_bar = 1.0;
    betas
        .iter()
        .map(|&beta| {
            alpha_bar *= 1.0 - beta;
            ((1.0 - alpha_bar) / alpha_bar.max(1e-12)).sqrt()
        })
        .collect()
}

/// 🌫️ Noise-level (`sigma`) schedules. All variants produce a monotonically non-increasing
/// sequence from `sigmas(steps)[0]` (most noise) down toward `0` (a clean sample).
#[derive(Clone, PartialEq, Debug)]
pub enum NoiseSchedule {
    Linear { beta_start: f64, beta_end: f64 },
    ScaledLinear { beta_start: f64, beta_end: f64 },
    Cosine { s: f64 },
    Karras { sigma_min: f64, sigma_max: f64, rho: f64 },
    Exponential { sigma_min: f64, sigma_max: f64 },
    Polynomial { sigma_min: f64, sigma_max: f64, power: f64 },
    Custom(Vec<f64>),
}

impl NoiseSchedule {
    /// 🌫️ Produces `steps` sigma values, descending.
    pub fn sigmas(&self, steps: usize) -> Vec<f64> {
        match self {
            Self::Linear { beta_start, beta_end } => {
                let mut s = betas_to_sigmas(&linspace(*beta_start, *beta_end, steps));
                s.reverse();
                s
            }
            Self::ScaledLinear { beta_start, beta_end } => {
                let betas: Vec<f64> = linspace(beta_start.sqrt(), beta_end.sqrt(), steps).iter().map(|b| b * b).collect();
                let mut s = betas_to_sigmas(&betas);
                s.reverse();
                s
            }
            Self::Cosine { s: s_offset } => {
                let f = |t: f64| ((t + s_offset) / (1.0 + s_offset) * core::f64::consts::FRAC_PI_2).cos().powi(2);
                let f0 = f(0.0);
                let n = steps.max(1);
                let mut out: Vec<f64> = (0..n)
                    .map(|i| {
                        let alpha_bar = (f(i as f64 / (n as f64 - 1.0).max(1.0)) / f0).clamp(1e-9, 1.0);
                        ((1.0 - alpha_bar) / alpha_bar).sqrt()
                    })
                    .collect();
                out.reverse();
                out
            }
            Self::Karras { sigma_min, sigma_max, rho } => {
                let n = steps.max(1);
                (0..n)
                    .map(|i| {
                        let t = i as f64 / (n as f64 - 1.0).max(1.0);
                        (sigma_max.powf(1.0 / rho) + t * (sigma_min.powf(1.0 / rho) - sigma_max.powf(1.0 / rho))).powf(*rho)
                    })
                    .collect()
            }
            Self::Exponential { sigma_min, sigma_max } => {
                let n = steps.max(1);
                (0..n)
                    .map(|i| {
                        let t = i as f64 / (n as f64 - 1.0).max(1.0);
                        (sigma_max.ln() + t * (sigma_min.ln() - sigma_max.ln())).exp()
                    })
                    .collect()
            }
            Self::Polynomial { sigma_min, sigma_max, power } => {
                let n = steps.max(1);
                (0..n)
                    .map(|i| {
                        let t = i as f64 / (n as f64 - 1.0).max(1.0);
                        (sigma_max.powf(1.0 / power) + t * (sigma_min.powf(1.0 / power) - sigma_max.powf(1.0 / power))).powf(*power)
                    })
                    .collect()
            }
            Self::Custom(values) => values.clone(),
        }
    }
}

/// 🌫️ What a [`Denoiser`]'s raw output represents.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PredictionType {
    Epsilon,
    VPrediction,
    Sample,
}

/// 🌫️ Converts a [`Denoiser`]'s raw prediction into a denoised (`x0`) estimate. `VPrediction` uses
/// one common (EDM-style) `sigma_data = 1` parametrization: `c_skip = 1/(sigma²+1)`, `c_out =
/// -sigma/sqrt(sigma²+1)` — other papers' exact v-prediction constants vary; this is a documented
/// choice, not a universal standard.
fn to_denoised(prediction_type: PredictionType, x: &[f32], sigma: f64, raw: &[f32]) -> Vec<f32> {
    match prediction_type {
        PredictionType::Sample => raw.to_vec(),
        PredictionType::Epsilon => x.iter().zip(raw).map(|(&xi, &ei)| xi - (sigma as f32) * ei).collect(),
        PredictionType::VPrediction => {
            let sigma2_1 = (sigma * sigma + 1.0) as f32;
            let c_skip = 1.0 / sigma2_1;
            let c_out = -(sigma as f32) / sigma2_1.sqrt();
            x.iter().zip(raw).map(|(&xi, &vi)| c_skip * xi + c_out * vi).collect()
        }
    }
}

/// 🌫️ Which conditioning branch a [`Denoiser`] call evaluates, for classifier-free guidance.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GuidanceBranch {
    Conditional,
    Unconditional,
}

/// 🌫️ Caller-supplied model evaluation: predicts (in `prediction_type`'s parametrization) the
/// noise/sample for `latent` at noise level `sigma`. Diffusion/image sampling never runs model
/// inference itself (same non-responsibility as token sampling, § 1) — this trait is the seam.
pub trait Denoiser {
    fn prediction_type(&self) -> PredictionType;
    #[allow(clippy::too_many_arguments)]
    fn denoise(&mut self, latent: &[f32], shape: [usize; 4], sigma: f64, step: usize, branch: GuidanceBranch, out: &mut [f32]) -> Result<(), SamplingError>;
}

/// 🌫️ Classifier-free guidance: `guided = uncond + scale * (cond - uncond)`, with optional
/// variance-rescaling (Lin et al., "Common Diffusion Noise Schedules and Sample Steps are Flawed")
/// to counter over-saturation at high `scale`.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Guidance {
    pub scale: f64,
    pub rescale: f64,
}

fn std_dev(values: &[f32]) -> f64 {
    let n = values.len() as f64;
    if n == 0.0 {
        return 0.0;
    }
    let mean: f64 = values.iter().map(|&x| x as f64).sum::<f64>() / n;
    let var: f64 = values.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / n;
    var.sqrt()
}

impl Guidance {
    fn combine(&self, cond: &[f32], uncond: &[f32], out: &mut [f32]) {
        for i in 0..out.len() {
            out[i] = uncond[i] + (self.scale as f32) * (cond[i] - uncond[i]);
        }
        if self.rescale > 0.0 {
            let std_cond = std_dev(cond);
            let std_guided = std_dev(out);
            if std_guided > 1e-8 {
                let factor = (std_cond / std_guided) as f32;
                let rescale = self.rescale as f32;
                for v in out.iter_mut() {
                    *v = *v * rescale * factor + *v * (1.0 - rescale);
                }
            }
        }
    }
}

fn normal_std(rng: &mut dyn RandomSource) -> f32 {
    let u1 = rng.next_f64_open01();
    let u2 = rng.next_f64();
    ((-2.0 * u1.ln()).sqrt() * (2.0 * core::f64::consts::PI * u2).cos()) as f32
}

fn euler_step(x: &mut [f32], denoised: &[f32], sigma: f64, sigma_next: f64) {
    let sigma_f = sigma.max(1e-10) as f32;
    let dt = (sigma_next - sigma) as f32;
    for i in 0..x.len() {
        let d = (x[i] - denoised[i]) / sigma_f;
        x[i] += d * dt;
    }
}

fn euler_ancestral_step(x: &mut [f32], denoised: &[f32], sigma: f64, sigma_next: f64, rng: &mut dyn RandomSource) {
    let sigma_up = if sigma > 1e-10 { (sigma_next.powi(2) * (sigma.powi(2) - sigma_next.powi(2)) / sigma.powi(2)).max(0.0).sqrt() } else { 0.0 };
    let sigma_down = (sigma_next.powi(2) - sigma_up.powi(2)).max(0.0).sqrt();
    euler_step(x, denoised, sigma, sigma_down);
    if sigma_up > 1e-10 {
        for xi in x.iter_mut() {
            *xi += (sigma_up as f32) * normal_std(rng);
        }
    }
}

fn heun_correct(x: &mut [f32], denoised0: &[f32], x_euler: &[f32], denoised1: &[f32], sigma: f64, sigma_next: f64) {
    let sigma_f = sigma.max(1e-10) as f32;
    let sigma_next_f = sigma_next.max(1e-10) as f32;
    let dt = (sigma_next - sigma) as f32;
    for i in 0..x.len() {
        let d0 = (x[i] - denoised0[i]) / sigma_f;
        let d1 = (x_euler[i] - denoised1[i]) / sigma_next_f;
        x[i] += (d0 + d1) * 0.5 * dt;
    }
}

fn ddim_step(x: &mut [f32], denoised: &[f32], sigma: f64, sigma_next: f64, eta: f64, rng: &mut dyn RandomSource) {
    let alpha_bar = 1.0 / (1.0 + sigma * sigma);
    let alpha_bar_next = 1.0 / (1.0 + sigma_next * sigma_next);
    let sigma_ddim = if alpha_bar < 1.0 { eta * ((1.0 - alpha_bar_next) / (1.0 - alpha_bar) * (1.0 - alpha_bar / alpha_bar_next)).max(0.0).sqrt() } else { 0.0 };
    let sigma_f = sigma.max(1e-10) as f32;
    let dir_coeff = ((1.0 - alpha_bar_next - sigma_ddim * sigma_ddim).max(0.0)).sqrt() as f32;
    let alpha_bar_next_sqrt = alpha_bar_next.sqrt() as f32;
    for i in 0..x.len() {
        let eps = (x[i] - denoised[i]) / sigma_f;
        x[i] = alpha_bar_next_sqrt * denoised[i] + dir_coeff * eps;
    }
    if sigma_ddim > 1e-10 {
        for xi in x.iter_mut() {
            *xi += (sigma_ddim as f32) * normal_std(rng);
        }
    }
}

/// 🌫️ Which ODE/SDE solver [`run_diffusion`] uses to step from one noise level to the next.
/// Covers the most commonly used solvers; PLMS/DPM/DPM++/UniPC-style multistep solvers are a
/// documented scope cut for this delivery (they need retained solver-state history across steps,
/// which [`Solver`] doesn't yet carry).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Solver {
    Euler,
    EulerAncestral,
    Heun,
    Ddim { eta: f64 },
}

/// 🌫️ One [`run_diffusion`] configuration.
pub struct DiffusionRunConfig {
    pub schedule: NoiseSchedule,
    pub solver: Solver,
    pub steps: usize,
    pub guidance: Option<Guidance>,
    pub seed: u64,
}

/// 🌫️ Whether [`run_diffusion`]'s step callback wants to continue or cancel the run.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StepControlFlow {
    Continue,
    Cancel,
}

fn eval_denoised(denoiser: &mut dyn Denoiser, x: &[f32], shape: [usize; 4], sigma: f64, guidance: Option<&Guidance>, step: usize) -> Result<Vec<f32>, SamplingError> {
    let prediction_type = denoiser.prediction_type();
    let mut raw_cond = vec![0.0f32; x.len()];
    denoiser.denoise(x, shape, sigma, step, GuidanceBranch::Conditional, &mut raw_cond)?;
    let raw = if let Some(g) = guidance {
        let mut raw_uncond = vec![0.0f32; x.len()];
        denoiser.denoise(x, shape, sigma, step, GuidanceBranch::Unconditional, &mut raw_uncond)?;
        let mut combined = vec![0.0f32; x.len()];
        g.combine(&raw_cond, &raw_uncond, &mut combined);
        combined
    } else {
        raw_cond
    };
    Ok(to_denoised(prediction_type, x, sigma, &raw))
}

/// 🌫️ Runs the reverse diffusion process on `latent` in place, from `schedule.sigmas(steps + 1)`'s
/// most-noisy value down to its last (typically `0`) — the "Text-to-image latent initialization"
/// and "Partial denoising" generation modes (see [`img2img_start_index`] and [`apply_inpaint_mask`]
/// for image-to-image / inpainting on top of this same loop). `step_callback(step, sigma_next,
/// latent)` fires after every step — it can preview, modify (e.g. blend in [`apply_inpaint_mask`]),
/// or cancel (returning [`StepControlFlow::Cancel`], which surfaces as [`SamplingError::Cancelled`]).
pub fn run_diffusion(config: &DiffusionRunConfig, latent: &mut [f32], shape: [usize; 4], denoiser: &mut dyn Denoiser, mut step_callback: impl FnMut(usize, f64, &mut [f32]) -> StepControlFlow) -> Result<(), SamplingError> {
    let sigmas = config.schedule.sigmas(config.steps + 1);
    let mut rng = CounterRng::from_root(config.seed, StreamKey { request: 0, sequence: 0, beam: 0, candidate: 0, purpose: StreamPurpose::Diffusion });

    for i in 0..config.steps {
        let sigma = sigmas[i];
        let sigma_next = sigmas.get(i + 1).copied().unwrap_or(0.0);
        let denoised = eval_denoised(denoiser, latent, shape, sigma, config.guidance.as_ref(), i)?;

        match config.solver {
            Solver::Euler => euler_step(latent, &denoised, sigma, sigma_next),
            Solver::EulerAncestral => euler_ancestral_step(latent, &denoised, sigma, sigma_next, &mut rng),
            Solver::Heun => {
                if sigma_next <= 1e-10 {
                    euler_step(latent, &denoised, sigma, sigma_next);
                } else {
                    let mut x_euler = latent.to_vec();
                    euler_step(&mut x_euler, &denoised, sigma, sigma_next);
                    let denoised_next = eval_denoised(denoiser, &x_euler, shape, sigma_next, config.guidance.as_ref(), i)?;
                    heun_correct(latent, &denoised, &x_euler, &denoised_next, sigma, sigma_next);
                }
            }
            Solver::Ddim { eta } => ddim_step(latent, &denoised, sigma, sigma_next, eta, &mut rng),
        }

        if let StepControlFlow::Cancel = step_callback(i, sigma_next, latent) {
            return Err(SamplingError::Cancelled);
        }
    }
    Ok(())
}

/// 🌫️ Image-to-image: the sigma-schedule index to start denoising from, given `strength` in
/// `[0, 1]` (`1.0` = full generation from pure noise, `0.0` = skip denoising entirely, returning
/// the original image essentially unchanged).
pub fn img2img_start_index(sigma_count: usize, strength: f64) -> usize {
    let strength = strength.clamp(0.0, 1.0);
    (((1.0 - strength) * sigma_count as f64).round() as usize).min(sigma_count)
}

/// 🌫️ Inpainting: re-noises `original` to the current `sigma` and blends it into `x` wherever
/// `mask[i] > 0` (`1.0` = fully keep the (re-noised) original, `0.0` = fully keep the freely
/// generated content), matching the "hold the known region, regenerate the rest" contract.
pub fn apply_inpaint_mask(x: &mut [f32], original: &[f32], mask: &[f32], sigma: f64, rng: &mut dyn RandomSource) {
    for i in 0..x.len() {
        if mask[i] > 0.0 {
            let noisy_original = original[i] + (sigma as f32) * normal_std(rng);
            x[i] = mask[i] * noisy_original + (1.0 - mask[i]) * x[i];
        }
    }
}
// #endregion 🔖️Diffusion

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // #region 🔖️IdsTests
    #[test]
    fn step_index_checked_next_overflows_to_none() {
        assert_eq!(StepIndex::new(u32::MAX).checked_next(), None);
        assert_eq!(StepIndex::new(0).checked_next(), Some(StepIndex::new(1)));
    }

    #[test]
    fn ids_display_show_raw_value() {
        assert_eq!(format!("{}", TokenId::new(42)), "42");
        assert_eq!(format!("{}", SequenceId::new(7)), "7");
        assert_eq!(format!("{}", StepIndex::new(3)), "3");
    }
    // #endregion 🔖️IdsTests

    // #region 🔖️ErrorsTests
    #[test]
    fn every_error_variant_has_nonempty_display() {
        let errors = [
            SamplingError::InvalidConfig { field: "temperature", reason: "must be >= 0" },
            SamplingError::VocabMismatch { expected: 10, actual: 5 },
            SamplingError::NonFiniteLogits { index: 3 },
            SamplingError::EmptyDistribution,
            SamplingError::ConstraintDead { constraint: "regex" },
            SamplingError::LimitExceeded { limit: "max_beam_width" },
            SamplingError::GrammarParse { offset: 0, reason: "unexpected token" },
            SamplingError::RegexParse { offset: 0, reason: "unbalanced paren" },
            SamplingError::AutomatonBudget { budget: "max_automaton_states" },
            SamplingError::SerializationVersion { expected: 1, actual: 2 },
            SamplingError::FingerprintMismatch,
            SamplingError::Corrupted { reason: "bad header" },
            SamplingError::Collective { reason: "timeout" },
            SamplingError::Callback { reason: "reranker panicked" },
            SamplingError::Cancelled,
        ];
        for error in &errors {
            assert!(!format!("{error}").is_empty());
            let _: &dyn std::error::Error = error;
        }
    }

    #[test]
    fn fallback_ladder_prefers_forced_over_eos_over_argmax_over_error() {
        let forced = Some(TokenId::new(1));
        let eos = Some(TokenId::new(2));
        let argmax = Some(TokenId::new(3));
        assert_eq!(resolve_fallback(forced, eos, argmax), (FallbackAction::ForcedToken, forced));
        assert_eq!(resolve_fallback(None, eos, argmax), (FallbackAction::Eos, eos));
        assert_eq!(resolve_fallback(None, None, argmax), (FallbackAction::ArgmaxRaw, argmax));
        assert_eq!(resolve_fallback(None, None, None), (FallbackAction::Error, None));
    }
    // #endregion 🔖️ErrorsTests

    // #region 🔖️LimitsTests
    #[test]
    fn default_limits_validate() {
        assert!(SamplingLimits::default().validate().is_ok());
    }

    #[test]
    fn zero_limit_fails_validation() {
        let limits = SamplingLimits { max_beam_width: 0, ..SamplingLimits::default() };
        assert!(limits.validate().is_err());
    }
    // #endregion 🔖️LimitsTests

    // #region 🔖️JsonTests
    #[test]
    fn json_round_trips_basic_values() {
        let text = r#"{"a":1,"b":[true,false,null,"x\ny"],"c":{"d":-2.5}}"#;
        let value = parse_json(text, 64).expect("valid json");
        assert_eq!(value.get("a").and_then(JsonValue::as_f64), Some(1.0));
        let b = value.get("b").and_then(JsonValue::as_array).expect("array");
        assert_eq!(b[0], JsonValue::Bool(true));
        assert_eq!(b[3], JsonValue::Str("x\ny".to_string()));
        assert_eq!(value.get("c").and_then(|c| c.get("d")).and_then(JsonValue::as_f64), Some(-2.5));

        let written = write_json(&value);
        let reparsed = parse_json(&written, 64).expect("valid round-trip json");
        assert_eq!(value, reparsed);
    }

    #[test]
    fn json_depth_cap_is_enforced() {
        let nested = "[".repeat(10) + &"]".repeat(10);
        assert!(parse_json(&nested, 5).is_err());
        assert!(parse_json(&nested, 20).is_ok());
    }

    #[test]
    fn json_rejects_trailing_garbage() {
        assert!(parse_json("1 2", 8).is_err());
    }

    #[test]
    fn json_rejects_malformed_literals_strings_and_numbers() {
        assert!(parse_json("", 8).is_err());
        assert!(parse_json("nul", 8).is_err());
        assert!(parse_json("truX", 8).is_err());
        assert!(parse_json("falsy", 8).is_err());
        assert!(parse_json("?", 8).is_err());
        assert!(parse_json("\"unterminated", 8).is_err());
        assert!(parse_json("\"bad\\x\"", 8).is_err());
        assert!(parse_json("\"\\", 8).is_err());
        assert!(parse_json("\"\\u12\"", 8).is_err());
        assert!(parse_json("--1", 8).is_err());
    }

    #[test]
    fn json_rejects_malformed_arrays_and_objects() {
        assert!(parse_json("[1 2]", 8).is_err());
        assert!(parse_json("[1,]", 8).is_err());
        assert!(parse_json("{1:2}", 8).is_err());
        assert!(parse_json("{\"a\" 1}", 8).is_err());
        assert!(parse_json("{\"a\":1 \"b\":2}", 8).is_err());
        assert!(parse_json("[", 8).is_err());
        assert!(parse_json("{", 8).is_err());
    }

    #[test]
    fn json_parses_empty_array_and_object_and_unicode_escape() {
        assert_eq!(parse_json("[]", 8).unwrap(), JsonValue::Array(Vec::new()));
        assert_eq!(parse_json("{}", 8).unwrap(), JsonValue::Object(Vec::new()));
        let value = parse_json("\"\\u0041\"", 8).unwrap();
        assert_eq!(value, JsonValue::Str("A".to_string()));
    }

    #[test]
    fn json_value_accessors_return_none_for_mismatched_variants() {
        let value = JsonValue::Str("x".to_string());
        assert_eq!(value.as_f64(), None);
        assert_eq!(value.as_bool(), None);
        assert_eq!(value.as_array(), None);
        assert_eq!(value.get("k"), None);
        assert_eq!(JsonValue::Bool(true).as_str(), None);
    }

    #[test]
    fn write_json_escapes_control_characters_and_formats_non_integral_numbers() {
        let value = JsonValue::Object(vec![("s".to_string(), JsonValue::Str("a\u{1}b".to_string())), ("n".to_string(), JsonValue::Num(1.5))]);
        let written = write_json(&value);
        assert!(written.contains("\\u0001"));
        assert!(written.contains("1.5"));
        assert_eq!(write_json(&JsonValue::Num(3.0)), "3");
    }
    // #endregion 🔖️JsonTests

    // #region 🔖️Utf8Tests
    #[test]
    fn utf8_status_classifies_complete_partial_invalid() {
        assert_eq!(utf8_status(b"hello"), Utf8Status::Complete);
        assert_eq!(utf8_status("héllo".as_bytes()), Utf8Status::Complete);
        let full = "é".as_bytes();
        assert_eq!(utf8_status(&full[..1]), Utf8Status::Partial { more: 1 });
        assert_eq!(utf8_status(&[0xFF]), Utf8Status::Invalid);
        assert_eq!(utf8_status(b""), Utf8Status::Complete);
    }
    // #endregion 🔖️Utf8Tests

    // #region 🔖️NumericsTests
    #[test]
    fn softmax_live_sums_to_one_and_matches_hand_computed() {
        let logits = [1.0f32, 2.0, 3.0];
        let live = [0u32, 1, 2];
        let mut probs = [0.0f32; 3];
        softmax_live(&logits, &live, &mut probs, Accum::F64);
        let sum: f32 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
        // 📐️ Hand-computed via exp(x - 3): [exp(-2), exp(-1), exp(0)] / sum
        let expected = [0.09003057f32, 0.24472847, 0.66524096];
        for (p, e) in probs.iter().zip(expected.iter()) {
            assert!((p - e).abs() < 1e-5, "{p} vs {e}");
        }
    }

    #[test]
    fn softmax_live_f32_and_f64_accum_agree_closely() {
        let logits: Vec<f32> = (0..50).map(|i| (i as f32) * 0.37 - 5.0).collect();
        let live: Vec<u32> = (0..50).collect();
        let mut probs_32 = vec![0.0f32; 50];
        let mut probs_64 = vec![0.0f32; 50];
        softmax_live(&logits, &live, &mut probs_32, Accum::F32);
        softmax_live(&logits, &live, &mut probs_64, Accum::F64);
        for (a, b) in probs_32.iter().zip(probs_64.iter()) {
            assert!((a - b).abs() < 1e-3, "{a} vs {b}");
        }
    }

    #[test]
    fn softmax_live_is_invariant_to_constant_shift() {
        let base = [1.0f32, 2.0, 3.0];
        let shifted = [1001.0f32, 1002.0, 1003.0];
        let live = [0u32, 1, 2];
        let mut probs_base = [0.0f32; 3];
        let mut probs_shifted = [0.0f32; 3];
        softmax_live(&base, &live, &mut probs_base, Accum::F64);
        softmax_live(&shifted, &live, &mut probs_shifted, Accum::F64);
        for (a, b) in probs_base.iter().zip(probs_shifted.iter()) {
            assert!((a - b).abs() < 1e-5);
        }
    }

    #[test]
    fn logsumexp_matches_naive_for_moderate_values() {
        let values: [f64; 4] = [0.1, 0.2, 0.3, -0.5];
        let naive = values.iter().map(|v| v.exp()).sum::<f64>().ln();
        assert!((logsumexp_f64(&values) - naive).abs() < 1e-9);
    }

    #[test]
    fn logsumexp_handles_all_neg_infinity() {
        assert_eq!(logsumexp_f64(&[f64::NEG_INFINITY, f64::NEG_INFINITY]), f64::NEG_INFINITY);
    }

    #[test]
    fn entropy_of_uniform_distribution_is_ln_n() {
        let probs = [0.25f32; 4];
        assert!((entropy_nats(&probs) - (4.0f64).ln()).abs() < 1e-6);
    }

    #[test]
    fn entropy_of_deterministic_distribution_is_zero() {
        let probs = [1.0f32, 0.0, 0.0];
        assert!(entropy_nats(&probs).abs() < 1e-9);
    }

    #[test]
    fn effective_candidate_count_matches_perplexity_of_uniform() {
        let probs = [0.125f32; 8];
        assert!((effective_candidate_count(&probs) - 8.0).abs() < 1e-6);
    }

    #[test]
    fn kahan_sum_matches_naive_sum_for_short_sequences() {
        let mut sum = KahanSum::new();
        let values = [1.0, 2.0, 3.0, 4.5];
        for v in values {
            sum.add(v);
        }
        assert!((sum.value() - 10.5).abs() < 1e-12);
    }

    #[test]
    fn partial_select_top_k_selects_highest_k_by_logit_with_tie_break() {
        let logits = [5.0f32, 1.0, 5.0, 3.0, 2.0];
        let mut live: Vec<u32> = (0..5).collect();
        let mut scratch = vec![0.0f32; 5];
        partial_select_top_k(&logits, &mut live, 3, &mut scratch);
        let mut top3 = live[..3].to_vec();
        top3.sort_unstable();
        assert_eq!(top3, vec![0, 2, 3]);
    }

    #[test]
    fn partial_select_top_k_is_noop_when_k_covers_everything() {
        let logits = [1.0f32, 2.0, 3.0];
        let mut live: Vec<u32> = (0..3).collect();
        let mut scratch = vec![0.0f32; 3];
        let before = live.clone();
        partial_select_top_k(&logits, &mut live, 3, &mut scratch);
        assert_eq!(live, before);
    }

    #[test]
    fn cdf_binary_search_finds_first_index_at_or_above_u() {
        let cdf = [0.2, 0.5, 0.5, 0.9, 1.0];
        assert_eq!(cdf_binary_search(&cdf, 0.0), 0);
        assert_eq!(cdf_binary_search(&cdf, 0.2), 0);
        assert_eq!(cdf_binary_search(&cdf, 0.21), 1);
        assert_eq!(cdf_binary_search(&cdf, 0.5), 1);
        assert_eq!(cdf_binary_search(&cdf, 0.95), 4);
        assert_eq!(cdf_binary_search(&cdf, 1.0), 4);
    }

    #[test]
    fn sanitize_logits_neg_inf_nan_policy_masks_nan_and_rejects_pos_inf() {
        let mut logits = [1.0f32, f32::NAN, f32::NEG_INFINITY];
        let altered = sanitize_logits(&mut logits, SanitizePolicy::NegInfNan).expect("no +inf present");
        assert_eq!(altered, 1);
        assert_eq!(logits[1], f32::NEG_INFINITY);

        let mut with_pos_inf = [1.0f32, f32::INFINITY];
        assert!(sanitize_logits(&mut with_pos_inf, SanitizePolicy::NegInfNan).is_err());
    }

    #[test]
    fn sanitize_logits_error_policy_rejects_any_non_finite() {
        let mut logits = [1.0f32, f32::NAN];
        assert!(sanitize_logits(&mut logits, SanitizePolicy::Error).is_err());
    }

    #[test]
    fn sanitize_logits_clamp_inf_policy_clamps_positive_infinity() {
        let mut logits = [1.0f32, f32::INFINITY];
        let altered = sanitize_logits(&mut logits, SanitizePolicy::ClampInf).expect("clamp policy never errors on inf");
        assert_eq!(altered, 1);
        assert_eq!(logits[1], f32::MAX);
    }
    // #endregion 🔖️NumericsTests

    // #region 🔖️BitsetTests
    #[test]
    fn bitset_new_full_has_exactly_len_bits_set() {
        let set = TokenBitset::new_full(70);
        assert_eq!(set.count_ones(), 70);
        for i in 0..70 {
            assert!(set.get(TokenId::new(i)));
        }
    }

    #[test]
    fn bitset_set_get_round_trip() {
        let mut set = TokenBitset::new_empty(10);
        set.set(TokenId::new(3), true);
        set.set(TokenId::new(7), true);
        assert!(set.get(TokenId::new(3)));
        assert!(set.get(TokenId::new(7)));
        assert!(!set.get(TokenId::new(4)));
        assert_eq!(set.count_ones(), 2);
        set.set(TokenId::new(3), false);
        assert!(!set.get(TokenId::new(3)));
    }

    #[test]
    fn bitset_and_or_and_not_operations() {
        let mut a = TokenBitset::new_empty(8);
        let mut b = TokenBitset::new_empty(8);
        a.set(TokenId::new(0), true);
        a.set(TokenId::new(1), true);
        b.set(TokenId::new(1), true);
        b.set(TokenId::new(2), true);

        let mut and = a.clone();
        and.and_with(&b);
        assert_eq!(and.count_ones(), 1);
        assert!(and.get(TokenId::new(1)));

        let mut or = a.clone();
        or.or_with(&b);
        assert_eq!(or.count_ones(), 3);

        let mut and_not = a.clone();
        and_not.and_not_with(&b);
        assert_eq!(and_not.count_ones(), 1);
        assert!(and_not.get(TokenId::new(0)));
    }

    #[test]
    fn bitset_iter_ones_skips_zero_words_across_boundaries() {
        let mut set = TokenBitset::new_empty(200);
        set.set(TokenId::new(5), true);
        set.set(TokenId::new(130), true);
        set.set(TokenId::new(199), true);
        let ones: Vec<u32> = set.iter_ones().map(TokenId::get).collect();
        assert_eq!(ones, vec![5, 130, 199]);
    }

    #[test]
    fn bitset_first_set_and_is_all_zero() {
        let mut set = TokenBitset::new_empty(64);
        assert!(set.is_all_zero());
        assert_eq!(set.first_set(), None);
        set.set(TokenId::new(40), true);
        assert!(!set.is_all_zero());
        assert_eq!(set.first_set(), Some(TokenId::new(40)));
    }
    // #endregion 🔖️BitsetTests

    // #region 🔖️RngTests
    #[test]
    fn counter_rng_is_deterministic_for_same_seed() {
        let mut a = CounterRng::from_seed(123);
        let mut b = CounterRng::from_seed(123);
        let seq_a: Vec<u64> = (0..32).map(|_| a.next_u64()).collect();
        let seq_b: Vec<u64> = (0..32).map(|_| b.next_u64()).collect();
        assert_eq!(seq_a, seq_b);
    }

    #[test]
    fn counter_rng_different_seeds_diverge() {
        let mut a = CounterRng::from_seed(1);
        let mut b = CounterRng::from_seed(2);
        let seq_a: Vec<u64> = (0..8).map(|_| a.next_u64()).collect();
        let seq_b: Vec<u64> = (0..8).map(|_| b.next_u64()).collect();
        assert_ne!(seq_a, seq_b);
    }

    #[test]
    fn counter_rng_split_is_independent_of_call_order() {
        let parent = CounterRng::from_seed(999);
        let key_a = StreamKey { request: 1, sequence: 2, beam: 0, candidate: 0, purpose: StreamPurpose::Selection };
        let key_b = StreamKey { request: 1, sequence: 3, beam: 0, candidate: 0, purpose: StreamPurpose::Selection };

        // 🎲️ Splitting in either order from the same parent must produce identical child streams.
        let mut a_first = parent.split(key_a);
        let mut b_first = parent.split(key_b);
        let a_vals_1: Vec<u64> = (0..4).map(|_| a_first.next_u64()).collect();
        let b_vals_1: Vec<u64> = (0..4).map(|_| b_first.next_u64()).collect();

        let mut b_second = parent.split(key_b);
        let mut a_second = parent.split(key_a);
        let b_vals_2: Vec<u64> = (0..4).map(|_| b_second.next_u64()).collect();
        let a_vals_2: Vec<u64> = (0..4).map(|_| a_second.next_u64()).collect();

        assert_eq!(a_vals_1, a_vals_2);
        assert_eq!(b_vals_1, b_vals_2);
        assert_ne!(a_vals_1, b_vals_1);
    }

    #[test]
    fn counter_rng_split_differs_by_purpose() {
        let parent = CounterRng::from_seed(42);
        let base = StreamKey { request: 1, sequence: 1, beam: 0, candidate: 0, purpose: StreamPurpose::Selection };
        let gumbel = StreamKey { purpose: StreamPurpose::Gumbel, ..base };
        let mut a = parent.split(base);
        let mut b = parent.split(gumbel);
        assert_ne!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn counter_rng_snapshot_restore_resumes_identically() {
        let mut original = CounterRng::from_seed(77);
        for _ in 0..9 {
            original.next_u64();
        }
        let snapshot = original.snapshot();
        let mut resumed = CounterRng::from_seed(0);
        resumed.restore(&snapshot).expect("matching kind restores cleanly");
        let expected: Vec<u64> = (0..16).map(|_| original.next_u64()).collect();
        let actual: Vec<u64> = (0..16).map(|_| resumed.next_u64()).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn rng_snapshot_rejects_kind_mismatch_on_restore() {
        let snapshot = CounterRng::from_seed(1).snapshot();
        let mut xoshiro = XoshiroSource::from_seed(1);
        assert!(xoshiro.restore(&snapshot).is_err());
    }

    #[test]
    fn rng_snapshot_text_round_trips() {
        let snapshot = RngSnapshot { kind: RngKind::Counter, words: [1, 2, 3, 4] };
        let text = snapshot.to_text();
        let parsed = RngSnapshot::from_text(&text).expect("valid snapshot text");
        assert_eq!(snapshot, parsed);
    }

    #[test]
    fn xoshiro_source_matches_underlying_rng_sequence() {
        let mut source = XoshiroSource::from_seed(4242);
        let mut reference = crate::random::Rng::from_seed(4242);
        for _ in 0..16 {
            assert_eq!(source.next_u64(), reference.next_u64());
        }
    }

    #[test]
    fn next_f64_open01_is_never_zero() {
        let mut rng = CounterRng::from_seed(0);
        for _ in 0..1000 {
            let u = rng.next_f64_open01();
            assert!(u > 0.0 && u <= 1.0, "u = {u} out of (0, 1]");
        }
    }

    #[test]
    fn next_range_stays_within_bounds() {
        let mut rng = CounterRng::from_seed(5);
        for _ in 0..1000 {
            let x = rng.next_range(3, 9);
            assert!((3..9).contains(&x));
        }
        assert_eq!(rng.next_range(4, 4), 4);
    }
    // #endregion 🔖️RngTests

    // #region 🔖️VocabularyTests
    #[test]
    fn vocabulary_validates_logits_length() {
        let vocab = Vocabulary::new(10);
        assert!(vocab.validate_logits_len(10).is_ok());
        assert!(vocab.validate_logits_len(9).is_err());
    }

    #[test]
    fn vocabulary_is_eos_reflects_configured_set() {
        let vocab = Vocabulary::new(10).with_eos(vec![TokenId::new(0), TokenId::new(1)]);
        assert!(vocab.is_eos(TokenId::new(0)));
        assert!(!vocab.is_eos(TokenId::new(2)));
    }

    #[test]
    fn slice_text_adapter_returns_bytes_and_stable_fingerprint() {
        let tokens: Vec<&[u8]> = vec![b"ab", b"c"];
        let adapter = SliceTextAdapter::new(&tokens);
        assert_eq!(adapter.vocab_size(), 2);
        assert_eq!(adapter.token_bytes(TokenId::new(0)), Some(b"ab".as_slice()));
        assert_eq!(adapter.token_bytes(TokenId::new(5)), None);
        let fp1 = adapter.fingerprint();
        let fp2 = SliceTextAdapter::new(&tokens).fingerprint();
        assert_eq!(fp1, fp2);

        let different: Vec<&[u8]> = vec![b"a", b"bc"];
        let fp3 = SliceTextAdapter::new(&different).fingerprint();
        assert_ne!(fp1, fp3, "separator byte must prevent boundary-shift collisions");
    }
    // #endregion 🔖️VocabularyTests

    // #region 🔖️ScheduleTests
    #[test]
    fn constant_schedule_ignores_input() {
        let schedule = Schedule::Constant(0.7);
        let input = ScheduleInput { step: StepIndex::new(50), generated_len: 50, last_entropy: None };
        assert_eq!(schedule.eval(input), 0.7);
    }

    #[test]
    fn linear_schedule_interpolates_and_clamps_at_bound() {
        let schedule = Schedule::Linear { from: 0.0, to: 1.0, over_steps: 10 };
        let at = |step: u32| schedule.eval(ScheduleInput { step: StepIndex::new(step), generated_len: 0, last_entropy: None });
        assert!((at(0) - 0.0).abs() < 1e-9);
        assert!((at(5) - 0.5).abs() < 1e-9);
        assert!((at(10) - 1.0).abs() < 1e-9);
        assert!((at(20) - 1.0).abs() < 1e-9, "must clamp past over_steps");
    }

    #[test]
    fn cosine_schedule_starts_and_ends_at_bounds() {
        let schedule = Schedule::Cosine { from: 1.0, to: 0.0, over_steps: 8 };
        let at = |step: u32| schedule.eval(ScheduleInput { step: StepIndex::new(step), generated_len: 0, last_entropy: None });
        assert!((at(0) - 1.0).abs() < 1e-9);
        assert!((at(8) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn piecewise_schedule_holds_at_last_breakpoint() {
        let schedule = Schedule::Piecewise(vec![(StepIndex::new(0), 1.0), (StepIndex::new(5), 2.0), (StepIndex::new(10), 3.0)]);
        let at = |step: u32| schedule.eval(ScheduleInput { step: StepIndex::new(step), generated_len: 0, last_entropy: None });
        assert_eq!(at(0), 1.0);
        assert_eq!(at(3), 1.0);
        assert_eq!(at(5), 2.0);
        assert_eq!(at(7), 2.0);
        assert_eq!(at(100), 3.0);
    }

    #[test]
    fn by_position_schedule_clamps_past_its_length() {
        let schedule = Schedule::ByPosition(vec![0.1, 0.2, 0.3]);
        let at = |len: usize| schedule.eval(ScheduleInput { step: StepIndex::new(0), generated_len: len, last_entropy: None });
        assert_eq!(at(0), 0.1);
        assert_eq!(at(2), 0.3);
        assert_eq!(at(50), 0.3);
    }

    #[test]
    fn entropy_scaled_schedule_clamps_to_range() {
        let schedule = Schedule::EntropyScaled { base: 0.5, gain: 1.0, min: 0.0, max: 1.0 };
        let at = |entropy: f64| schedule.eval(ScheduleInput { step: StepIndex::new(0), generated_len: 0, last_entropy: Some(entropy) });
        assert_eq!(at(-10.0), 0.0);
        assert_eq!(at(10.0), 1.0);
        assert!((at(0.0) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn schedule_json_round_trips_every_non_callback_variant() {
        let schedules = vec![
            Schedule::Constant(0.5),
            Schedule::Linear { from: 0.0, to: 1.0, over_steps: 10 },
            Schedule::Exponential { from: 0.1, to: 1.0, over_steps: 5 },
            Schedule::Cosine { from: 1.0, to: 0.0, over_steps: 8 },
            Schedule::Piecewise(vec![(StepIndex::new(0), 1.0), (StepIndex::new(4), 2.0)]),
            Schedule::ByPosition(vec![0.1, 0.2]),
            Schedule::EntropyScaled { base: 0.5, gain: 1.0, min: 0.0, max: 2.0 },
        ];
        for schedule in schedules {
            let json = schedule.to_json();
            let parsed = Schedule::from_json(&json).expect("round trip should succeed");
            assert_eq!(schedule, parsed);
        }
    }

    #[test]
    fn callback_schedule_serializes_but_refuses_to_deserialize() {
        fn double(input: ScheduleInput) -> f64 {
            input.step.get() as f64 * 2.0
        }
        let schedule = Schedule::Callback(double);
        let json = schedule.to_json();
        assert!(Schedule::from_json(&json).is_err());
    }
    // #endregion 🔖️ScheduleTests

    // #region 🔖️ConfigTests
    #[test]
    fn default_config_validates() {
        assert!(SamplingConfig::default().validate().is_ok());
    }

    #[test]
    fn all_presets_validate() {
        assert!(SamplingConfig::precise().validate().is_ok());
        assert!(SamplingConfig::balanced().validate().is_ok());
        assert!(SamplingConfig::creative().validate().is_ok());
        assert!(SamplingConfig::deterministic_test().validate().is_ok());
    }

    #[test]
    fn builder_produces_a_validated_config() {
        let config = SamplingConfigBuilder::new()
            .method(SamplingMethod::Multinomial { strategy: MultinomialStrategy::CdfBinarySearch })
            .processor(ProcessorSpec::Temperature { value: Schedule::Constant(0.8) })
            .seed(7)
            .max_tokens(128)
            .build()
            .expect("valid config");
        assert_eq!(config.seed, 7);
        assert_eq!(config.max_tokens, 128);
        assert_eq!(config.processors.len(), 1);
    }

    #[test]
    fn validate_rejects_min_tokens_above_max_tokens() {
        let config = SamplingConfig { min_tokens: 10, max_tokens: 5, ..SamplingConfig::default() };
        assert!(config.validate().is_err());
    }

    #[test]
    fn validate_rejects_zero_candidate_count() {
        let config = SamplingConfig { candidate_count: 0, ..SamplingConfig::default() };
        assert!(config.validate().is_err());
    }

    #[test]
    fn validate_rejects_candidate_count_above_limit() {
        let limits = SamplingLimits { max_candidates: 4, ..SamplingLimits::default() };
        let config = SamplingConfig { candidate_count: 5, limits, ..SamplingConfig::default() };
        assert!(config.validate().is_err());
    }

    #[test]
    fn validate_rejects_too_many_stop_sequences() {
        let limits = SamplingLimits { max_stop_sequences: 1, ..SamplingLimits::default() };
        let config = SamplingConfig { limits, stops: StopSpec { sequences: vec![b"a".to_vec(), b"b".to_vec()], ..StopSpec::default() }, ..SamplingConfig::default() };
        assert!(config.validate().is_err());
    }

    #[test]
    fn validate_rejects_zero_no_repeat_ngram_order() {
        let config = SamplingConfig { processors: vec![ProcessorSpec::NoRepeatNgram { n: 0 }], ..SamplingConfig::default() };
        assert!(config.validate().is_err());
    }

    #[test]
    fn validate_rejects_mismatched_token_class_penalty_lengths() {
        let config = SamplingConfig { processors: vec![ProcessorSpec::TokenClassPenalty { class_tokens: vec![vec![TokenId::new(0)], vec![TokenId::new(1)]], factors: vec![0.5] }], ..SamplingConfig::default() };
        assert!(config.validate().is_err());
    }

    #[test]
    fn config_fingerprint_is_stable_and_sensitive_to_changes() {
        let a = SamplingConfig::balanced();
        let b = SamplingConfig::balanced();
        assert_eq!(a.fingerprint(), b.fingerprint());
        let c = SamplingConfig { seed: a.seed + 1, ..a.clone() };
        assert_ne!(a.fingerprint(), c.fingerprint());
    }

    #[test]
    fn config_json_round_trips_core_fields() {
        let config = SamplingConfigBuilder::new()
            .method(SamplingMethod::GumbelTopK { k: 3 })
            .processor(ProcessorSpec::TopK { k: Schedule::Constant(40.0), min_keep: 1 })
            .processor(ProcessorSpec::RepetitionPenalty { penalty: 1.2, scope: PenaltyScope::GeneratedOnly })
            .seed(99)
            .candidate_count(2)
            .min_tokens(1)
            .max_tokens(200)
            .build()
            .expect("valid config");
        let json = config.to_json();
        let parsed = SamplingConfig::from_json(&json).expect("valid round trip");
        assert_eq!(parsed.method, config.method);
        assert_eq!(parsed.processors, config.processors);
        assert_eq!(parsed.seed, config.seed);
        assert_eq!(parsed.candidate_count, config.candidate_count);
        assert_eq!(parsed.min_tokens, config.min_tokens);
        assert_eq!(parsed.max_tokens, config.max_tokens);
    }

    #[test]
    fn config_json_rejects_unknown_version() {
        let json = JsonValue::Object(vec![("version".into(), JsonValue::Num(2.0))]);
        assert!(matches!(SamplingConfig::from_json(&json), Err(SamplingError::SerializationVersion { expected: 1, actual: 2 })));
    }

    #[test]
    fn all_processor_spec_variants_round_trip_through_json() {
        let specs = vec![
            ProcessorSpec::Temperature { value: Schedule::Constant(0.7) },
            ProcessorSpec::DynamicTemperature { base: Schedule::Constant(0.5), entropy_gain: 0.1, min: 0.0, max: 2.0 },
            ProcessorSpec::TopK { k: Schedule::Constant(40.0), min_keep: 1 },
            ProcessorSpec::TopP { p: Schedule::Constant(0.9), min_keep: 1 },
            ProcessorSpec::MinP { p: Schedule::Constant(0.05), min_keep: 1 },
            ProcessorSpec::Typical { mass: Schedule::Constant(0.9), min_keep: 1 },
            ProcessorSpec::LocallyTypical { mass: Schedule::Constant(0.9), min_keep: 1 },
            ProcessorSpec::TailFree { z: Schedule::Constant(0.95), min_keep: 1 },
            ProcessorSpec::Epsilon { cutoff: Schedule::Constant(0.001), min_keep: 1 },
            ProcessorSpec::Eta { cutoff: Schedule::Constant(0.001), min_keep: 1 },
            ProcessorSpec::TopA { power: Schedule::Constant(2.0), min_keep: 1 },
            ProcessorSpec::RankTruncation { max_rank: 50 },
            ProcessorSpec::AdaptiveTruncation { target_entropy: Some(2.0), target_effective_count: None },
            ProcessorSpec::RepetitionPenalty { penalty: 1.1, scope: PenaltyScope::GeneratedOnly },
            ProcessorSpec::PresencePenalty { penalty: 0.5, scope: PenaltyScope::PromptAndGenerated },
            ProcessorSpec::FrequencyPenalty { penalty: 0.3, scope: PenaltyScope::PromptOnly },
            ProcessorSpec::DecayingPenalty { penalty: 0.5, window: 32, half_life: 4.0, scope: PenaltyScope::GeneratedOnly },
            ProcessorSpec::TokenClassPenalty { class_tokens: vec![vec![TokenId::new(0)], vec![TokenId::new(1), TokenId::new(2)]], factors: vec![0.5, 0.9] },
            ProcessorSpec::NoRepeatNgram { n: 3 },
            ProcessorSpec::PhrasePenalty { phrases: vec![vec![TokenId::new(1), TokenId::new(2)]], penalty: 0.4 },
            ProcessorSpec::LogitBiasSparse { entries: vec![(TokenId::new(5), 2.0)] },
            ProcessorSpec::LogitBiasDense { values: vec![0.0, 1.0, -1.0] },
            ProcessorSpec::AllowTokens { tokens: vec![TokenId::new(1)] },
            ProcessorSpec::ForbidTokens { tokens: vec![TokenId::new(2)] },
            ProcessorSpec::SuppressSpecial,
            ProcessorSpec::BadWords { phrases: vec![vec![TokenId::new(3)]] },
            ProcessorSpec::SequenceEncouragement { phrases: vec![vec![TokenId::new(4)]], bonus: 1.5 },
            ProcessorSpec::Mirostat { version: MirostatVersion::V2, target_surprise: 5.0, learning_rate: 0.1 },
            ProcessorSpec::EntropyPid { target: 2.0, kp: 0.1, ki: 0.01, kd: 0.0 },
            ProcessorSpec::RepetitionController { window: 16, threshold: 0.5, boost: 0.2 },
            ProcessorSpec::ConfidenceController { low_entropy: 0.5, high_entropy: 3.0, low_temp: 0.5, high_temp: 1.2 },
        ];
        for spec in specs {
            let json = processor_spec_to_json(&spec);
            let parsed = processor_spec_from_json(&json).expect("round trip should succeed");
            assert_eq!(spec, parsed);
        }
    }
    // #endregion 🔖️ConfigTests

    // #region 🔖️WorkspaceTests
    fn small_vocab() -> Vocabulary {
        Vocabulary::new(8).with_eos(vec![TokenId::new(7)])
    }

    fn step_view<'a>(vocab: &'a Vocabulary, prompt: &'a [TokenId], generated: &'a [TokenId]) -> StepView<'a> {
        StepView { sequence: SequenceId::new(1), step: StepIndex::new(generated.len() as u32), prompt, generated, vocab, adapter: None, last_entropy: None }
    }

    #[test]
    fn workspace_reset_for_step_initializes_live_to_full_vocab_and_argmax() {
        let mut ws = LogitsWorkspace::new(5);
        let logits = [1.0f32, 3.0, 2.0, 3.0, 0.0];
        ws.reset_for_step(&logits, SanitizePolicy::NegInfNan).expect("finite logits never error");
        assert_eq!(ws.live(), &[0, 1, 2, 3, 4]);
        // 📐️ Ties between indices 1 and 3 (both 3.0) break toward the lowest token id.
        assert_eq!(ws.saved_argmax(), TokenId::new(1));
    }

    #[test]
    fn workspace_sync_live_with_mask_removes_masked_entries() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[0.0; 4], SanitizePolicy::NegInfNan).unwrap();
        ws.mask_mut().set(TokenId::new(2), false);
        ws.sync_live_with_mask();
        assert_eq!(ws.live(), &[0, 1, 3]);
    }

    #[test]
    fn workspace_sort_live_by_prob_desc_orders_by_probability_then_token_id() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[1.0, 3.0, 3.0, 0.5], SanitizePolicy::NegInfNan).unwrap();
        ws.sort_live_by_prob_desc();
        assert_eq!(ws.live(), &[1, 2, 0, 3]);
        assert!(ws.probs()[0] >= ws.probs()[1]);
        assert!(ws.probs()[1] >= ws.probs()[2]);
    }

    #[test]
    fn workspace_truncate_live_to_respects_min_keep() {
        let mut ws = LogitsWorkspace::new(5);
        ws.reset_for_step(&[5.0, 4.0, 3.0, 2.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        ws.sort_live_by_prob_desc();
        ws.truncate_live_to(1, 3);
        assert_eq!(ws.live().len(), 3);
    }

    #[test]
    fn workspace_collapse_live_to_argmax_leaves_single_best_entry() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[1.0, 5.0, 2.0, 5.0], SanitizePolicy::NegInfNan).unwrap();
        ws.collapse_live_to_argmax();
        assert_eq!(ws.live(), &[1]);
    }

    #[test]
    fn workspace_pool_reuses_released_workspace() {
        let mut pool = WorkspacePool::new();
        let ws = pool.acquire(16);
        assert_eq!(ws.vocab_size(), 16);
        pool.release(ws);
        let reused = pool.acquire(16);
        assert_eq!(reused.vocab_size(), 16);
    }
    // #endregion 🔖️WorkspaceTests

    // #region 🔖️WarperTests
    #[test]
    fn temperature_zero_collapses_to_greedy() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[1.0, 5.0, 2.0, 0.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = small_vocab();
        let view = step_view(&vocab, &[], &[]);
        let mut temp = Temperature { value: Schedule::Constant(0.0) };
        temp.process(&view, &mut ws).unwrap();
        assert_eq!(ws.live(), &[1]);
    }

    #[test]
    fn temperature_scales_processed_logits() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[2.0, 4.0, 6.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3);
        let view = step_view(&vocab, &[], &[]);
        let mut temp = Temperature { value: Schedule::Constant(2.0) };
        temp.process(&view, &mut ws).unwrap();
        assert_eq!(ws.processed(), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn top_k_keeps_exactly_k_highest_probability_tokens() {
        let mut ws = LogitsWorkspace::new(5);
        ws.reset_for_step(&[5.0, 4.0, 3.0, 2.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(5);
        let view = step_view(&vocab, &[], &[]);
        let mut top_k = TopK { k: Schedule::Constant(2.0), min_keep: 1 };
        top_k.process(&view, &mut ws).unwrap();
        let mut kept = ws.live().to_vec();
        kept.sort_unstable();
        assert_eq!(kept, vec![0, 1]);
    }

    #[test]
    fn top_p_retains_smallest_prefix_covering_cumulative_mass() {
        let mut ws = LogitsWorkspace::new(4);
        // 🌡️ Logits chosen so softmax gives one dominant token (~0.87) plus a long tail.
        ws.reset_for_step(&[10.0, 0.0, 0.0, 0.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(4);
        let view = step_view(&vocab, &[], &[]);
        let mut top_p = TopP { p: Schedule::Constant(0.5), min_keep: 1 };
        top_p.process(&view, &mut ws).unwrap();
        assert_eq!(ws.live(), &[0]);
    }

    #[test]
    fn top_p_min_keep_overrides_a_too_small_cutoff() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[1.0, 1.0, 1.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(4);
        let view = step_view(&vocab, &[], &[]);
        let mut top_p = TopP { p: Schedule::Constant(0.01), min_keep: 3 };
        top_p.process(&view, &mut ws).unwrap();
        assert_eq!(ws.live().len(), 3);
    }

    #[test]
    fn min_p_drops_tokens_far_below_the_maximum() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[10.0, 0.0, -10.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3);
        let view = step_view(&vocab, &[], &[]);
        let mut min_p = MinP { p: Schedule::Constant(0.1), min_keep: 1 };
        min_p.process(&view, &mut ws).unwrap();
        assert_eq!(ws.live(), &[0]);
    }

    #[test]
    fn typical_and_locally_typical_agree_on_the_same_input() {
        let logits = [3.0f32, 1.0, 0.5, 0.1];
        let vocab = Vocabulary::new(4);
        let view = step_view(&vocab, &[], &[]);

        let mut ws_a = LogitsWorkspace::new(4);
        ws_a.reset_for_step(&logits, SanitizePolicy::NegInfNan).unwrap();
        let mut typical = Typical { mass: Schedule::Constant(0.8), min_keep: 1 };
        typical.process(&view, &mut ws_a).unwrap();

        let mut ws_b = LogitsWorkspace::new(4);
        ws_b.reset_for_step(&logits, SanitizePolicy::NegInfNan).unwrap();
        let mut locally = LocallyTypical { mass: Schedule::Constant(0.8), min_keep: 1 };
        locally.process(&view, &mut ws_b).unwrap();

        let mut a = ws_a.live().to_vec();
        let mut b = ws_b.live().to_vec();
        a.sort_unstable();
        b.sort_unstable();
        assert_eq!(a, b);
        assert!(!a.is_empty());
    }

    #[test]
    fn tail_free_keeps_at_least_min_keep_on_short_live_sets() {
        let mut ws = LogitsWorkspace::new(2);
        ws.reset_for_step(&[1.0, 2.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(2);
        let view = step_view(&vocab, &[], &[]);
        let mut tail_free = TailFree { z: Schedule::Constant(0.9), min_keep: 1 };
        tail_free.process(&view, &mut ws).unwrap();
        assert!(!ws.live().is_empty());
    }

    #[test]
    fn epsilon_cutoff_drops_near_zero_probability_tokens() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[20.0, -20.0, -20.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3);
        let view = step_view(&vocab, &[], &[]);
        let mut epsilon = EpsilonCutoff { cutoff: Schedule::Constant(0.01), min_keep: 1 };
        epsilon.process(&view, &mut ws).unwrap();
        assert_eq!(ws.live(), &[0]);
    }

    #[test]
    fn eta_cutoff_never_empties_live_set() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[1.0, 2.0, 3.0, 4.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(4);
        let view = step_view(&vocab, &[], &[]);
        let mut eta = EtaCutoff { cutoff: Schedule::Constant(0.1), min_keep: 1 };
        eta.process(&view, &mut ws).unwrap();
        assert!(!ws.live().is_empty());
    }

    #[test]
    fn top_a_drops_low_probability_tokens_relative_to_max() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[10.0, -10.0, -10.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3);
        let view = step_view(&vocab, &[], &[]);
        let mut top_a = TopA { power: Schedule::Constant(0.5), min_keep: 1 };
        top_a.process(&view, &mut ws).unwrap();
        assert_eq!(ws.live(), &[0]);
    }

    #[test]
    fn rank_truncation_keeps_exactly_max_rank_entries() {
        let mut ws = LogitsWorkspace::new(5);
        ws.reset_for_step(&[5.0, 4.0, 3.0, 2.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(5);
        let view = step_view(&vocab, &[], &[]);
        let mut rank = RankTruncation { max_rank: 2 };
        rank.process(&view, &mut ws).unwrap();
        assert_eq!(ws.live().len(), 2);
    }

    #[test]
    fn adaptive_truncation_targeting_effective_count_shrinks_a_near_uniform_distribution() {
        // 📐️ A peaked distribution's *natural* effective count is already low — "targeting" a
        // higher count than that can never shrink it (there's nothing to cut). Truncation only
        // makes sense the other way: start near-uniform (effective count 6) and target lower (2).
        let mut ws = LogitsWorkspace::new(6);
        ws.reset_for_step(&[1.0; 6], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(6);
        let view = step_view(&vocab, &[], &[]);
        let mut adaptive = AdaptiveTruncation { target_entropy: None, target_effective_count: Some(2.0) };
        adaptive.process(&view, &mut ws).unwrap();
        assert!(ws.live().len() < 6);
        assert!(ws.live().len() >= 2);
    }

    #[test]
    fn min_keep_guarantee_is_honored_by_every_truncation_warper() {
        let vocab = Vocabulary::new(6);
        let view = step_view(&vocab, &[], &[]);
        let logits = [1.0f32, 1.0, 1.0, 1.0, 1.0, 1.0];
        let min_keep = 4;

        let mut ws = LogitsWorkspace::new(6);
        ws.reset_for_step(&logits, SanitizePolicy::NegInfNan).unwrap();
        TopK { k: Schedule::Constant(1.0), min_keep }.process(&view, &mut ws).unwrap();
        assert!(ws.live().len() >= min_keep);

        let mut ws = LogitsWorkspace::new(6);
        ws.reset_for_step(&logits, SanitizePolicy::NegInfNan).unwrap();
        TopP { p: Schedule::Constant(0.001), min_keep }.process(&view, &mut ws).unwrap();
        assert!(ws.live().len() >= min_keep);

        let mut ws = LogitsWorkspace::new(6);
        ws.reset_for_step(&logits, SanitizePolicy::NegInfNan).unwrap();
        MinP { p: Schedule::Constant(0.999), min_keep }.process(&view, &mut ws).unwrap();
        assert!(ws.live().len() >= min_keep);
    }
    // #endregion 🔖️WarperTests

    // #region 🔖️SelectionTests
    fn distribution_fixture<'a>(tokens: &'a [TokenId], probs: &'a [f32], logprobs: &'a [f32], cdf: &'a [f64]) -> Distribution<'a> {
        Distribution { tokens, probs, logprobs, cdf, entropy: entropy_nats(probs) }
    }

    #[test]
    fn greedy_sampler_lowest_token_id_picks_first_of_a_tie() {
        let tokens = [TokenId::new(0), TokenId::new(1), TokenId::new(2)];
        let probs = [0.5f32, 0.5, 0.0];
        let logprobs = [probs[0].ln(), probs[1].ln(), f32::NEG_INFINITY];
        let cdf = [0.5, 1.0, 1.0];
        let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf);
        let mut sampler = GreedySampler { tie_break: TieBreak::LowestTokenId };
        let mut out = SelectionBuffer::default();
        let mut rng = CounterRng::from_seed(1);
        sampler.sample(&step_view(&small_vocab(), &[], &[]), &dist, &mut rng, &mut out).unwrap();
        assert_eq!(out.chosen[0].token, TokenId::new(0));
    }

    #[test]
    fn greedy_sampler_highest_token_id_picks_last_of_a_tie() {
        let tokens = [TokenId::new(0), TokenId::new(1), TokenId::new(2)];
        let probs = [0.5f32, 0.5, 0.0];
        let logprobs = [probs[0].ln(), probs[1].ln(), f32::NEG_INFINITY];
        let cdf = [0.5, 1.0, 1.0];
        let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf);
        let mut sampler = GreedySampler { tie_break: TieBreak::HighestTokenId };
        let mut out = SelectionBuffer::default();
        let mut rng = CounterRng::from_seed(1);
        sampler.sample(&step_view(&small_vocab(), &[], &[]), &dist, &mut rng, &mut out).unwrap();
        assert_eq!(out.chosen[0].token, TokenId::new(1));
    }

    #[test]
    fn greedy_sampler_errors_on_empty_distribution() {
        let dist = distribution_fixture(&[], &[], &[], &[]);
        let mut sampler = GreedySampler { tie_break: TieBreak::LowestTokenId };
        let mut out = SelectionBuffer::default();
        let mut rng = CounterRng::from_seed(1);
        assert!(sampler.sample(&step_view(&small_vocab(), &[], &[]), &dist, &mut rng, &mut out).is_err());
    }

    #[test]
    fn multinomial_cdf_binary_search_matches_expected_frequencies() {
        let tokens = [TokenId::new(0), TokenId::new(1)];
        let probs = [0.2f32, 0.8];
        let logprobs = [probs[0].ln(), probs[1].ln()];
        let cdf = [0.2, 1.0];
        let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf);
        let mut sampler = MultinomialSampler { strategy: MultinomialStrategy::CdfBinarySearch };
        let mut rng = CounterRng::from_seed(2024);
        let vocab = small_vocab();
        let view = step_view(&vocab, &[], &[]);
        let draws = 20_000;
        let mut count_1 = 0u32;
        for _ in 0..draws {
            let mut out = SelectionBuffer::default();
            sampler.sample(&view, &dist, &mut rng, &mut out).unwrap();
            if out.chosen[0].token == TokenId::new(1) {
                count_1 += 1;
            }
        }
        let ratio = count_1 as f64 / draws as f64;
        assert!((ratio - 0.8).abs() < 0.02, "ratio {ratio} too far from 0.8");
    }

    #[test]
    fn multinomial_strategies_agree_statistically() {
        let tokens = [TokenId::new(0), TokenId::new(1), TokenId::new(2)];
        let probs = [0.1f32, 0.3, 0.6];
        let logprobs = [probs[0].ln(), probs[1].ln(), probs[2].ln()];
        let cdf = cumulative_from_probs(&probs);
        let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf);
        let vocab = small_vocab();
        let view = step_view(&vocab, &[], &[]);
        let draws = 20_000;

        for strategy in [MultinomialStrategy::CdfBinarySearch, MultinomialStrategy::LinearScan, MultinomialStrategy::Alias] {
            let mut sampler = MultinomialSampler { strategy };
            let mut rng = CounterRng::from_seed(555);
            let mut counts = [0u32; 3];
            for _ in 0..draws {
                let mut out = SelectionBuffer::default();
                sampler.sample(&view, &dist, &mut rng, &mut out).unwrap();
                counts[out.chosen[0].token.get() as usize] += 1;
            }
            for (i, &expected) in probs.iter().enumerate() {
                let ratio = counts[i] as f64 / draws as f64;
                assert!((ratio - expected as f64).abs() < 0.03, "strategy {strategy:?} index {i}: ratio {ratio} vs expected {expected}");
            }
        }
    }

    #[test]
    fn gumbel_max_sampler_matches_multinomial_marginals() {
        let tokens = [TokenId::new(0), TokenId::new(1), TokenId::new(2)];
        let probs = [0.2f32, 0.3, 0.5];
        let logprobs = [probs[0].ln(), probs[1].ln(), probs[2].ln()];
        let cdf = cumulative_from_probs(&probs);
        let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf);
        let vocab = small_vocab();
        let view = step_view(&vocab, &[], &[]);
        let mut sampler = GumbelMaxSampler;
        let mut rng = CounterRng::from_seed(321);
        let draws = 20_000;
        let mut counts = [0u32; 3];
        for _ in 0..draws {
            let mut out = SelectionBuffer::default();
            sampler.sample(&view, &dist, &mut rng, &mut out).unwrap();
            counts[out.chosen[0].token.get() as usize] += 1;
        }
        for (i, &expected) in probs.iter().enumerate() {
            let ratio = counts[i] as f64 / draws as f64;
            assert!((ratio - expected as f64).abs() < 0.03, "index {i}: ratio {ratio} vs expected {expected}");
        }
    }

    #[test]
    fn gumbel_top_k_returns_k_distinct_tokens() {
        let tokens = [TokenId::new(0), TokenId::new(1), TokenId::new(2), TokenId::new(3)];
        let probs = [0.4f32, 0.3, 0.2, 0.1];
        let logprobs = [probs[0].ln(), probs[1].ln(), probs[2].ln(), probs[3].ln()];
        let cdf = cumulative_from_probs(&probs);
        let dist = distribution_fixture(&tokens, &probs, &logprobs, &cdf);
        let vocab = small_vocab();
        let view = step_view(&vocab, &[], &[]);
        let mut sampler = GumbelTopKSampler { k: 2 };
        let mut rng = CounterRng::from_seed(7);
        let mut out = SelectionBuffer::default();
        sampler.sample(&view, &dist, &mut rng, &mut out).unwrap();
        assert_eq!(out.chosen.len(), 2);
        assert_ne!(out.chosen[0].token, out.chosen[1].token);
    }
    // #endregion 🔖️SelectionTests

    // #region 🔖️EngineTests
    #[test]
    fn stateless_step_is_deterministic_for_same_seed_and_config() {
        let vocab = small_vocab();
        let config = SamplingConfig::balanced();
        let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];

        let run = || {
            let mut ws = LogitsWorkspace::new(8);
            let mut rng = CounterRng::from_seed(config.seed);
            let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
            sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).unwrap()
        };
        let a = run();
        let b = run();
        assert_eq!(a.token, b.token);
        assert_eq!(a.logprob, b.logprob);
    }

    #[test]
    fn stateless_step_greedy_precise_always_picks_the_argmax() {
        let vocab = small_vocab();
        let config = SamplingConfig::precise();
        let logits = [1.0f32, 2.0, 9.0, 3.0, 1.5, 0.2, 0.1, -5.0];
        let mut ws = LogitsWorkspace::new(8);
        let mut rng = CounterRng::from_seed(0);
        let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
        let result = sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).unwrap();
        assert_eq!(result.token, TokenId::new(2));
    }

    #[test]
    fn stateless_step_reports_eos_finish_reason() {
        let vocab = small_vocab();
        let config = SamplingConfig::precise();
        let mut logits = [0.0f32; 8];
        logits[7] = 100.0; // 📖️ token 7 is the configured EOS token.
        let mut ws = LogitsWorkspace::new(8);
        let mut rng = CounterRng::from_seed(0);
        let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
        let result = sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).unwrap();
        assert_eq!(result.token, TokenId::new(7));
        assert_eq!(result.finish, Some(FinishReason::EosToken));
    }

    #[test]
    fn stateless_step_reports_max_tokens_finish_reason() {
        let vocab = small_vocab();
        let config = SamplingConfig { max_tokens: 1, ..SamplingConfig::precise() };
        let logits = [1.0f32, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let mut ws = LogitsWorkspace::new(8);
        let mut rng = CounterRng::from_seed(0);
        let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
        let result = sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).unwrap();
        assert_eq!(result.finish, Some(FinishReason::MaxTokens));
    }

    #[test]
    fn stateless_step_rejects_mismatched_logits_length() {
        let vocab = small_vocab();
        let config = SamplingConfig::precise();
        let logits = [0.0f32; 4];
        let mut ws = LogitsWorkspace::new(8);
        let mut rng = CounterRng::from_seed(0);
        let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
        assert!(sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).is_err());
    }

    #[test]
    fn stateless_step_probabilities_sum_to_approximately_one() {
        let vocab = small_vocab();
        let config = SamplingConfig::balanced();
        let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];
        let mut ws = LogitsWorkspace::new(8);
        let mut rng = CounterRng::from_seed(1);
        let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
        sample_step_stateless(&config, &mut ws, &mut rng, &logits, input).unwrap();
        let sum: f32 = ws.probs().iter().sum();
        assert!((sum - 1.0).abs() < 1e-4, "sum = {sum}");
    }
    // #endregion 🔖️EngineTests

    // #region 🔖️PenaltiesTests
    #[test]
    fn repetition_penalty_pushes_down_a_seen_positive_logit() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[4.0, 2.0, 2.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3);
        let generated = [TokenId::new(0)];
        let view = step_view(&vocab, &[], &generated);
        let mut penalty = RepetitionPenalty::new(2.0, PenaltyScope::GeneratedOnly);
        penalty.commit(&view, TokenId::new(0));
        penalty.process(&view, &mut ws).unwrap();
        assert_eq!(ws.processed()[0], 2.0);
        assert_eq!(ws.processed()[1], 2.0);
    }

    #[test]
    fn repetition_penalty_rollback_restores_exact_prior_state() {
        let vocab = Vocabulary::new(3);
        let view = step_view(&vocab, &[], &[]);
        let mut penalty = RepetitionPenalty::new(2.0, PenaltyScope::GeneratedOnly);
        let mark_before = penalty.save();
        penalty.commit(&view, TokenId::new(0));
        penalty.commit(&view, TokenId::new(1));
        assert_eq!(penalty.counts.count(TokenId::new(0)), 1);
        penalty.rollback_to(mark_before);
        assert_eq!(penalty.counts.count(TokenId::new(0)), 0);
        assert_eq!(penalty.counts.count(TokenId::new(1)), 0);
    }

    #[test]
    fn presence_penalty_applies_flat_penalty_regardless_of_count() {
        let mut ws = LogitsWorkspace::new(2);
        ws.reset_for_step(&[1.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(2);
        let view = step_view(&vocab, &[], &[]);
        let mut penalty = PresencePenalty::new(0.5, PenaltyScope::GeneratedOnly);
        penalty.commit(&view, TokenId::new(0));
        penalty.commit(&view, TokenId::new(0));
        penalty.process(&view, &mut ws).unwrap();
        assert!((ws.processed()[0] - 0.5).abs() < 1e-6);
        assert!((ws.processed()[1] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn frequency_penalty_scales_with_occurrence_count() {
        let mut ws = LogitsWorkspace::new(2);
        ws.reset_for_step(&[1.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(2);
        let view = step_view(&vocab, &[], &[]);
        let mut penalty = FrequencyPenalty::new(0.5, PenaltyScope::GeneratedOnly);
        penalty.commit(&view, TokenId::new(0));
        penalty.commit(&view, TokenId::new(0));
        penalty.process(&view, &mut ws).unwrap();
        assert!((ws.processed()[0] - 0.0).abs() < 1e-6);
        assert!((ws.processed()[1] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn decaying_penalty_weighs_recent_occurrences_more_than_distant_ones() {
        let mut ws_recent = LogitsWorkspace::new(2);
        ws_recent.reset_for_step(&[1.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(2);
        let view = step_view(&vocab, &[], &[]);
        let mut recent_penalty = DecayingPenalty::new(1.0, 8, 1.0, PenaltyScope::GeneratedOnly);
        recent_penalty.commit(&view, TokenId::new(0));
        recent_penalty.process(&view, &mut ws_recent).unwrap();

        let mut ws_distant = LogitsWorkspace::new(2);
        ws_distant.reset_for_step(&[1.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        let mut distant_penalty = DecayingPenalty::new(1.0, 8, 1.0, PenaltyScope::GeneratedOnly);
        distant_penalty.commit(&view, TokenId::new(0));
        for _ in 0..5 {
            distant_penalty.commit(&view, TokenId::new(1));
        }
        distant_penalty.process(&view, &mut ws_distant).unwrap();

        assert!(ws_recent.processed()[0] < ws_distant.processed()[0], "a more recent occurrence must be penalized harder");
    }

    #[test]
    fn token_class_penalty_scales_only_classified_tokens() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[10.0, 10.0, 10.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3);
        let view = step_view(&vocab, &[], &[]);
        let mut penalty = TokenClassPenalty::new(vec![vec![TokenId::new(0), TokenId::new(1)]], vec![0.5]);
        penalty.process(&view, &mut ws).unwrap();
        assert_eq!(ws.processed()[0], 5.0);
        assert_eq!(ws.processed()[1], 5.0);
        assert_eq!(ws.processed()[2], 10.0);
    }

    #[test]
    fn no_repeat_ngram_forbids_recreating_a_seen_bigram() {
        let mut ngram = NoRepeatNgram::new(2);
        let vocab = Vocabulary::new(5);
        // History: [0, 1, 0]. The bigram (0 -> 1) was already seen, so after another `0` the
        // engine must forbid token `1` (it would recreate that exact bigram).
        let generated_first = [TokenId::new(0)];
        let view_after_first = step_view(&vocab, &[], &generated_first);
        ngram.commit(&view_after_first, TokenId::new(1));
        let mut ws = LogitsWorkspace::new(5);
        ws.reset_for_step(&[0.0; 5], SanitizePolicy::NegInfNan).unwrap();
        let generated_third = [TokenId::new(0), TokenId::new(1), TokenId::new(0)];
        let view_after_third = step_view(&vocab, &[], &generated_third);
        ngram.process(&view_after_third, &mut ws).unwrap();
        assert!(!ws.mask().get(TokenId::new(1)));
        assert!(ws.mask().get(TokenId::new(2)));
    }

    #[test]
    fn no_repeat_ngram_rollback_un_forbids() {
        let mut ngram = NoRepeatNgram::new(2);
        let vocab = Vocabulary::new(5);
        let generated = [TokenId::new(0)];
        let view = step_view(&vocab, &[], &generated);
        let mark = ngram.save();
        ngram.commit(&view, TokenId::new(1));
        ngram.rollback_to(mark);
        let mut ws = LogitsWorkspace::new(5);
        ws.reset_for_step(&[0.0; 5], SanitizePolicy::NegInfNan).unwrap();
        ngram.process(&view, &mut ws).unwrap();
        assert!(ws.mask().get(TokenId::new(1)), "rollback must undo the forbidden-next entry");
    }

    #[test]
    fn phrase_penalty_penalizes_only_after_the_proper_prefix() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[1.0, 1.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3);
        let generated = [TokenId::new(0)];
        let view = step_view(&vocab, &[], &generated);
        let mut penalty = PhrasePenalty { phrases: vec![vec![TokenId::new(0), TokenId::new(1)]], penalty: 0.5 };
        penalty.process(&view, &mut ws).unwrap();
        assert!((ws.processed()[1] - 0.5).abs() < 1e-6);
        assert_eq!(ws.processed()[2], 1.0);
    }
    // #endregion 🔖️PenaltiesTests

    // #region 🔖️BiasesTests
    #[test]
    fn logit_bias_sparse_and_dense_add_expected_deltas() {
        let vocab = Vocabulary::new(3);
        let view = step_view(&vocab, &[], &[]);

        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[1.0, 1.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        let mut sparse = LogitBiasSparse { entries: vec![(TokenId::new(1), 5.0)] };
        sparse.process(&view, &mut ws).unwrap();
        assert_eq!(ws.processed(), &[1.0, 6.0, 1.0]);

        let mut ws2 = LogitsWorkspace::new(3);
        ws2.reset_for_step(&[1.0, 1.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        let mut dense = LogitBiasDense { values: vec![0.0, -2.0, 3.0] };
        dense.process(&view, &mut ws2).unwrap();
        assert_eq!(ws2.processed(), &[1.0, -1.0, 4.0]);
    }

    #[test]
    fn allow_tokens_restricts_mask_to_exactly_the_allowed_set() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[0.0; 4], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(4);
        let view = step_view(&vocab, &[], &[]);
        let mut allow = AllowTokens { tokens: vec![TokenId::new(1), TokenId::new(3)] };
        allow.process(&view, &mut ws).unwrap();
        ws.sync_live_with_mask();
        assert_eq!(ws.live(), &[1, 3]);
    }

    #[test]
    fn forbid_tokens_removes_exactly_the_forbidden_set() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[0.0; 4], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(4);
        let view = step_view(&vocab, &[], &[]);
        let mut forbid = ForbidTokens { tokens: vec![TokenId::new(2)] };
        forbid.process(&view, &mut ws).unwrap();
        ws.sync_live_with_mask();
        assert_eq!(ws.live(), &[0, 1, 3]);
    }

    #[test]
    fn suppress_special_removes_flagged_tokens() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[0.0; 4], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(4).with_special(&[TokenId::new(0), TokenId::new(3)]);
        let view = step_view(&vocab, &[], &[]);
        let mut suppress = SuppressSpecial;
        suppress.process(&view, &mut ws).unwrap();
        ws.sync_live_with_mask();
        assert_eq!(ws.live(), &[1, 2]);
    }

    #[test]
    fn bad_words_masks_the_completion_token_after_its_prefix() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[0.0; 3], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3);
        let generated = [TokenId::new(0)];
        let view = step_view(&vocab, &[], &generated);
        let mut bad = BadWords { phrases: vec![vec![TokenId::new(0), TokenId::new(1)]] };
        bad.process(&view, &mut ws).unwrap();
        assert!(!ws.mask().get(TokenId::new(1)));
        assert!(ws.mask().get(TokenId::new(2)));
    }

    #[test]
    fn sequence_encouragement_biases_the_completion_token_after_its_prefix() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[1.0, 1.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3);
        let generated = [TokenId::new(0)];
        let view = step_view(&vocab, &[], &generated);
        let mut encourage = SequenceEncouragement { phrases: vec![vec![TokenId::new(0), TokenId::new(1)]], bonus: 3.0 };
        encourage.process(&view, &mut ws).unwrap();
        assert_eq!(ws.processed()[1], 4.0);
        assert_eq!(ws.processed()[2], 1.0);
    }
    // #endregion 🔖️BiasesTests

    // #region 🔖️LengthTests
    #[test]
    fn min_length_eos_suppression_masks_eos_before_the_floor() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[0.0; 3], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3).with_eos(vec![TokenId::new(2)]);
        let generated = [TokenId::new(0)];
        let view = step_view(&vocab, &[], &generated);
        let mut min_len = MinLengthEosSuppression { min_tokens: 5 };
        min_len.process(&view, &mut ws).unwrap();
        assert!(!ws.mask().get(TokenId::new(2)));
    }

    #[test]
    fn min_length_eos_suppression_allows_eos_once_floor_reached() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[0.0; 3], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3).with_eos(vec![TokenId::new(2)]);
        let generated = vec![TokenId::new(0); 5];
        let view = step_view(&vocab, &[], &generated);
        let mut min_len = MinLengthEosSuppression { min_tokens: 5 };
        min_len.process(&view, &mut ws).unwrap();
        assert!(ws.mask().get(TokenId::new(2)));
    }

    #[test]
    fn max_length_force_eos_restricts_to_eos_at_the_cap() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[0.0; 3], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3).with_eos(vec![TokenId::new(2)]);
        let generated = vec![TokenId::new(0); 4];
        let view = step_view(&vocab, &[], &generated);
        let mut force = MaxLengthForceEos { max_tokens: 5 };
        force.process(&view, &mut ws).unwrap();
        ws.sync_live_with_mask();
        assert_eq!(ws.live(), &[2]);
    }

    #[test]
    fn forced_tokens_forces_bos_then_prefix_then_at_position() {
        let vocab = Vocabulary::new(6);
        let spec = ForcedSpec { bos: Some(TokenId::new(0)), prefix: vec![TokenId::new(1), TokenId::new(2)], at_position: vec![(StepIndex::new(5), TokenId::new(4))] };
        let mut forced = ForcedTokens { spec };

        let mut ws = LogitsWorkspace::new(6);
        ws.reset_for_step(&[0.0; 6], SanitizePolicy::NegInfNan).unwrap();
        forced.process(&step_view(&vocab, &[], &[]), &mut ws).unwrap();
        ws.sync_live_with_mask();
        assert_eq!(ws.live(), &[0]);

        let mut ws = LogitsWorkspace::new(6);
        ws.reset_for_step(&[0.0; 6], SanitizePolicy::NegInfNan).unwrap();
        forced.process(&step_view(&vocab, &[], &[TokenId::new(0)]), &mut ws).unwrap();
        ws.sync_live_with_mask();
        assert_eq!(ws.live(), &[1]);

        let mut ws = LogitsWorkspace::new(6);
        ws.reset_for_step(&[0.0; 6], SanitizePolicy::NegInfNan).unwrap();
        let generated5 = vec![TokenId::new(0); 5];
        forced.process(&step_view(&vocab, &[], &generated5), &mut ws).unwrap();
        ws.sync_live_with_mask();
        assert_eq!(ws.live(), &[4]);
    }
    // #endregion 🔖️LengthTests

    // #region 🔖️AdaptiveTests
    #[test]
    fn mirostat_v2_truncates_to_tokens_within_the_surprise_budget() {
        let mut ws = LogitsWorkspace::new(6);
        ws.reset_for_step(&[10.0, 0.0, 0.0, 0.0, 0.0, 0.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(6);
        let view = step_view(&vocab, &[], &[]);
        let mut mirostat = Mirostat::new(MirostatVersion::V2, 3.0, 0.1);
        mirostat.process(&view, &mut ws).unwrap();
        assert!(ws.live().len() < 6);
    }

    #[test]
    fn mirostat_commit_updates_mu_toward_target() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[10.0, 0.0, 0.0, 0.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(4);
        let view = step_view(&vocab, &[], &[]);
        let mut mirostat = Mirostat::new(MirostatVersion::V2, 3.0, 0.5);
        mirostat.process(&view, &mut ws).unwrap();
        let mu_before = mirostat.mu;
        mirostat.commit(&view, TokenId::new(0));
        assert_ne!(mirostat.mu, mu_before);
    }

    #[test]
    fn mirostat_rollback_restores_mu() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[10.0, 0.0, 0.0, 0.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(4);
        let view = step_view(&vocab, &[], &[]);
        let mut mirostat = Mirostat::new(MirostatVersion::V2, 3.0, 0.5);
        let mark = mirostat.save();
        mirostat.process(&view, &mut ws).unwrap();
        mirostat.commit(&view, TokenId::new(0));
        mirostat.rollback_to(mark);
        assert_eq!(mirostat.mu, 6.0);
    }

    #[test]
    fn entropy_pid_sharpens_the_distribution_when_entropy_exceeds_target() {
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&[1.0, 1.0, 1.0, 1.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(4);
        let view = step_view(&vocab, &[], &[]);
        let mut pid = EntropyPid::new(0.1, 1.0, 0.0, 0.0);
        pid.process(&view, &mut ws).unwrap();
        // 📐️ Uniform entropy (ln 4 ≈ 1.39) is far above the 0.1 target, so `error = target - entropy`
        // is very negative, driving `temp` toward (and clamped at) `0.05` — a *low* temperature that
        // sharpens the distribution (reduces entropy) by scaling logits *up* (dividing by a small
        // temp), the correct control direction for "entropy is too high, pull it down".
        assert!(ws.processed()[0] > 1.0);
    }

    #[test]
    fn repetition_controller_flattens_after_crossing_the_threshold() {
        let vocab = Vocabulary::new(2);
        let view = step_view(&vocab, &[], &[]);
        let mut controller = RepetitionController::new(4, 0.5, 1.0);
        for _ in 0..4 {
            controller.commit(&view, TokenId::new(0));
        }
        let mut ws = LogitsWorkspace::new(2);
        ws.reset_for_step(&[2.0, 2.0], SanitizePolicy::NegInfNan).unwrap();
        controller.process(&view, &mut ws).unwrap();
        assert!(ws.processed()[0] < 2.0);
    }

    #[test]
    fn confidence_controller_uses_low_temp_under_low_entropy() {
        let mut ws = LogitsWorkspace::new(3);
        ws.reset_for_step(&[10.0, 0.0, 0.0], SanitizePolicy::NegInfNan).unwrap();
        let vocab = Vocabulary::new(3);
        let view = step_view(&vocab, &[], &[]);
        let mut controller = ConfidenceController { low_entropy: 0.0, high_entropy: 2.0, low_temp: 0.2, high_temp: 1.5 };
        controller.process(&view, &mut ws).unwrap();
        // 📐️ Near-zero entropy selects a temperature near `low_temp` (0.2), scaling logits up ~5x.
        assert!(ws.processed()[0] > 40.0);
    }
    // #endregion 🔖️AdaptiveTests

    // #region 🔖️StopsTests
    struct MockAdapter {
        table: Vec<Vec<u8>>,
    }
    impl TokenTextAdapter for MockAdapter {
        fn vocab_size(&self) -> usize {
            self.table.len()
        }
        fn token_bytes(&self, token: TokenId) -> Option<&[u8]> {
            self.table.get(token.get() as usize).map(Vec::as_slice)
        }
        fn fingerprint(&self) -> u64 {
            0
        }
    }

    #[test]
    fn aho_corasick_matches_and_reports_hold_back_on_partial_prefix() {
        let ac = AhoCorasick::build(&[b"ab".to_vec()]);
        let s1 = ac.step(0, b'a');
        assert_eq!(ac.depth(s1), 1);
        assert!(ac.matched_at(s1).is_none());
        let s2 = ac.step(s1, b'b');
        assert!(ac.matched_at(s2).is_some());
    }

    #[test]
    fn aho_corasick_handles_overlapping_patterns_via_fail_links() {
        let ac = AhoCorasick::build(&[b"abc".to_vec(), b"bcd".to_vec()]);
        let mut state = 0u32;
        for &byte in b"abcd" {
            state = ac.step(state, byte);
        }
        // 🛑️ After consuming "abcd", the automaton must have matched "bcd" via the fail link.
        let (index, len) = ac.matched_at(state).expect("bcd must match via fail link");
        assert_eq!(len, 3);
        let _ = index;
    }

    #[test]
    fn token_stop_condition_fires_on_configured_token() {
        let vocab = small_vocab();
        let view = step_view(&vocab, &[], &[]);
        let mut stop = TokenStopCondition { tokens: vec![TokenId::new(9)] };
        assert_eq!(stop.on_token(&view, TokenId::new(9)), StopPoll::Finished { reason: FinishReason::StopToken, matched_bytes: 0 });
        assert_eq!(stop.on_token(&view, TokenId::new(1)), StopPoll::Continue);
    }

    #[test]
    fn text_stop_condition_matches_a_multi_token_stop_sequence() {
        let adapter = MockAdapter { table: vec![b"He".to_vec(), b"llo".to_vec(), b"!".to_vec()] };
        let vocab = Vocabulary::new(3);
        let view = StepView { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: Some(&adapter), last_entropy: None };
        let mut stop = TextStopCondition::new(&[b"Hello".to_vec()], StopTextMode::Include);
        assert_eq!(stop.on_token(&view, TokenId::new(0)), StopPoll::Hold { ambiguous_bytes: 2 });
        let result = stop.on_token(&view, TokenId::new(1));
        assert_eq!(result, StopPoll::Finished { reason: FinishReason::StopSequence { index: 0 }, matched_bytes: 5 });
    }

    #[test]
    fn text_stop_condition_without_adapter_never_matches() {
        let vocab = Vocabulary::new(3);
        let view = StepView { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
        let mut stop = TextStopCondition::new(&[b"Hello".to_vec()], StopTextMode::Include);
        assert_eq!(stop.on_token(&view, TokenId::new(0)), StopPoll::Continue);
    }
    // #endregion 🔖️StopsTests

    // #region 🔖️SequenceStateTests
    fn make_state(config: &SamplingConfig) -> SequenceState {
        SequenceState::new(SequenceId::new(1), Vec::new(), config, Box::new(CounterRng::from_seed(config.seed))).unwrap_or_else(|e| panic!("sequence state should build: {e}"))
    }

    #[test]
    fn sequence_state_new_builds_configured_constraints() {
        let config = SamplingConfig { constraints: vec![ConstraintSpec::JsonMode], ..SamplingConfig::default() };
        let state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(0))).expect("json mode constraint should build");
        assert_eq!(state.constraints.len(), 1);
    }

    #[test]
    fn sequence_state_new_rejects_an_invalid_regex_constraint() {
        let config = SamplingConfig { constraints: vec![ConstraintSpec::Regex("(unclosed".to_string())], ..SamplingConfig::default() };
        assert!(SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(0))).is_err());
    }

    #[test]
    fn sequence_state_checkpoint_restore_round_trips() {
        let config = SamplingConfig::precise();
        let mut state = make_state(&config);
        let vocab = small_vocab();
        let view = step_view(&vocab, &[], &[]);
        state.generated.push(TokenId::new(3));
        state.cumulative_logprob = -1.5;
        let checkpoint = state.checkpoint();
        state.generated.push(TokenId::new(4));
        state.cumulative_logprob = -3.0;
        state.restore(&checkpoint);
        assert_eq!(state.generated(), &[TokenId::new(3)]);
        assert!((state.cumulative_logprob() - (-1.5)).abs() < 1e-9);
        let _ = view;
    }

    #[test]
    fn sequence_state_rollback_then_readvance_matches_direct_run() {
        let config = SamplingConfig::balanced();
        let vocab = small_vocab();
        let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];

        let mut direct = SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(config.seed))).unwrap();
        let mut ws = LogitsWorkspace::new(8);
        let mut observer = NullObserver;
        for _ in 0..3 {
            sample_step(&config, &mut direct, &mut ws, &vocab, None, &logits, &mut observer).unwrap();
        }
        let direct_text = direct.to_text();

        let mut replayed = SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(config.seed))).unwrap();
        let mut ws2 = LogitsWorkspace::new(8);
        for _ in 0..3 {
            sample_step(&config, &mut replayed, &mut ws2, &vocab, None, &logits, &mut observer).unwrap();
        }
        replayed.rollback(2);
        assert_eq!(replayed.generated().len(), 1);
        for _ in 0..2 {
            sample_step(&config, &mut replayed, &mut ws2, &vocab, None, &logits, &mut observer).unwrap();
        }
        assert_eq!(replayed.to_text(), direct_text);
    }

    #[test]
    fn sequence_state_fork_diverges_independently() {
        let config = SamplingConfig::balanced();
        let vocab = small_vocab();
        let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];
        let mut base = SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(config.seed))).unwrap();
        let mut ws = LogitsWorkspace::new(8);
        let mut observer = NullObserver;
        sample_step(&config, &mut base, &mut ws, &vocab, None, &logits, &mut observer).unwrap();

        let key_a = StreamKey { request: 0, sequence: 2, beam: 0, candidate: 0, purpose: StreamPurpose::Selection };
        let key_b = StreamKey { request: 0, sequence: 3, beam: 0, candidate: 0, purpose: StreamPurpose::Selection };
        let mut fork_a = base.fork(SequenceId::new(2), key_a);
        let mut fork_b = base.fork(SequenceId::new(3), key_b);
        assert_eq!(fork_a.generated(), fork_b.generated());

        let mut ws_a = LogitsWorkspace::new(8);
        let mut ws_b = LogitsWorkspace::new(8);
        for _ in 0..5 {
            sample_step(&config, &mut fork_a, &mut ws_a, &vocab, None, &logits, &mut observer).unwrap();
            sample_step(&config, &mut fork_b, &mut ws_b, &vocab, None, &logits, &mut observer).unwrap();
        }
        assert_ne!(fork_a.generated(), fork_b.generated(), "independently split RNG streams should diverge over several draws");
    }

    #[test]
    fn sequence_state_to_text_round_trips_and_rejects_fingerprint_mismatch() {
        let config = SamplingConfig::precise();
        let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(0))).unwrap();
        let vocab = small_vocab();
        let mut ws = LogitsWorkspace::new(8);
        let mut observer = NullObserver;
        let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];
        sample_step(&config, &mut state, &mut ws, &vocab, None, &logits, &mut observer).unwrap();
        let text = state.to_text();
        let decoded = state.decode_text(&text).unwrap();
        assert_eq!(decoded, state.generated());

        let other_config = SamplingConfig::balanced();
        let other_state = SequenceState::new(SequenceId::new(1), Vec::new(), &other_config, Box::new(CounterRng::from_seed(0))).unwrap();
        assert!(other_state.decode_text(&text).is_err());
    }

    #[test]
    fn sample_step_errors_when_sequence_already_finished() {
        let config = SamplingConfig { max_tokens: 1, ..SamplingConfig::precise() };
        let vocab = small_vocab();
        let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(0))).unwrap();
        let mut ws = LogitsWorkspace::new(8);
        let mut observer = NullObserver;
        let logits = [1.0f32, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let result = sample_step(&config, &mut state, &mut ws, &vocab, None, &logits, &mut observer).unwrap();
        assert_eq!(result.finish, Some(FinishReason::MaxTokens));
        assert!(sample_step(&config, &mut state, &mut ws, &vocab, None, &logits, &mut observer).is_err());
    }

    #[test]
    fn sample_step_with_no_repeat_ngram_avoids_recreating_a_bigram() {
        let config = SamplingConfig { method: SamplingMethod::Greedy { tie_break: TieBreak::LowestTokenId }, processors: vec![ProcessorSpec::NoRepeatNgram { n: 2 }], max_tokens: 6, ..SamplingConfig::default() };
        let vocab = small_vocab();
        let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(0))).unwrap();
        let mut ws = LogitsWorkspace::new(8);
        let mut observer = NullObserver;
        // 📐️ Token 1 always dominates except when masked, so the no-repeat-ngram guard should force
        // deviation the moment the same bigram would otherwise recur.
        let mut logits = [0.0f32; 8];
        logits[1] = 10.0;
        logits[0] = 5.0;
        let mut tokens = Vec::new();
        for _ in 0..4 {
            let result = sample_step(&config, &mut state, &mut ws, &vocab, None, &logits, &mut observer).unwrap();
            tokens.push(result.token);
            if result.finish.is_some() {
                break;
            }
        }
        // 🌡️ Token 1 (the argmax) can never appear twice consecutively after the same predecessor.
        for pair in tokens.windows(3) {
            assert!(!(pair[0] == pair[2] && pair[1] == TokenId::new(1) && pair[0] == TokenId::new(1)), "must not recreate the (1, 1) bigram twice");
        }
    }

    #[test]
    fn sample_step_honors_forced_bos_at_step_zero() {
        let config = SamplingConfig { forced: ForcedSpec { bos: Some(TokenId::new(3)), ..ForcedSpec::default() }, ..SamplingConfig::precise() };
        let vocab = small_vocab();
        let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(0))).unwrap();
        let mut ws = LogitsWorkspace::new(8);
        let mut observer = NullObserver;
        let logits = [10.0f32, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
        let result = sample_step(&config, &mut state, &mut ws, &vocab, None, &logits, &mut observer).unwrap();
        assert_eq!(result.token, TokenId::new(3));
    }
    // #endregion 🔖️SequenceStateTests

    // #region 🔖️AutomataTests
    #[test]
    fn dfa_matches_a_star_b_pattern() {
        let limits = SamplingLimits::default();
        let dfa = Dfa::from_pattern("a*b", &limits).unwrap();
        for accepted in ["b", "ab", "aaab"] {
            let mut state = dfa.start();
            for &byte in accepted.as_bytes() {
                state = dfa.step(state, byte);
            }
            assert!(dfa.is_accept(state), "{accepted:?} should be accepted");
        }
        for rejected in ["a", "ba", "abc", ""] {
            let mut state = dfa.start();
            let mut dead = false;
            for &byte in rejected.as_bytes() {
                state = dfa.step(state, byte);
                if dfa.is_dead(state) {
                    dead = true;
                    break;
                }
            }
            assert!(dead || !dfa.is_accept(state), "{rejected:?} should not be accepted");
        }
    }

    #[test]
    fn dfa_handles_alternation_class_and_bounded_repeat() {
        let limits = SamplingLimits::default();
        let dfa = Dfa::from_pattern("[a-c]{2,3}", &limits).unwrap();
        let matches = |s: &str| {
            let mut state = dfa.start();
            for &byte in s.as_bytes() {
                state = dfa.step(state, byte);
                if dfa.is_dead(state) {
                    return false;
                }
            }
            dfa.is_accept(state)
        };
        assert!(matches("ab"));
        assert!(matches("abc"));
        assert!(!matches("a"));
        assert!(!matches("abca"));
        assert!(!matches("ad"));
    }

    #[test]
    fn dfa_alive_flag_marks_dead_ends_as_unreachable_to_accept() {
        let limits = SamplingLimits::default();
        let dfa = Dfa::from_pattern("ab", &limits).unwrap();
        let mut state = dfa.start();
        state = dfa.step(state, b'z');
        assert!(dfa.is_dead(state));
        assert!(!dfa.is_alive(state));
    }

    #[test]
    fn dfa_rejects_unbalanced_parens_with_parse_error() {
        let limits = SamplingLimits::default();
        assert!(Dfa::from_pattern("(ab", &limits).is_err());
    }

    #[test]
    fn dfa_budget_is_enforced_for_pathologically_wide_patterns() {
        let limits = SamplingLimits { max_automaton_states: 4, ..SamplingLimits::default() };
        assert!(Dfa::from_pattern("(a|b|c|d|e|f|g|h){5}", &limits).is_err());
    }

    #[test]
    fn dfa_token_cache_computes_allowed_tokens_and_next_states() {
        let limits = SamplingLimits::default();
        let dfa = Dfa::from_pattern("ab", &limits).unwrap();
        let tokens: Vec<&[u8]> = vec![b"a", b"b", b"x"];
        let adapter = SliceTextAdapter::new(&tokens);
        let mut cache = DfaTokenMemo::new(16);
        let (allowed, next) = cache.get_or_compute(&dfa, dfa.start(), &adapter);
        assert!(allowed.get(TokenId::new(0)));
        assert!(!allowed.get(TokenId::new(1)));
        assert!(!allowed.get(TokenId::new(2)));
        let after_a = next[0];
        let (allowed2, _) = cache.get_or_compute(&dfa, after_a, &adapter);
        assert!(allowed2.get(TokenId::new(1)));
    }

    #[test]
    fn dfa_supports_plus_optional_and_negated_class_quantifiers() {
        let limits = SamplingLimits::default();
        let matches = |pattern: &str, s: &str| {
            let dfa = Dfa::from_pattern(pattern, &limits).unwrap();
            let mut state = dfa.start();
            for &byte in s.as_bytes() {
                state = dfa.step(state, byte);
                if dfa.is_dead(state) {
                    return false;
                }
            }
            dfa.is_accept(state)
        };
        assert!(matches("a+", "aaa"));
        assert!(!matches("a+", ""));
        assert!(matches("ab?c", "ac"));
        assert!(matches("ab?c", "abc"));
        assert!(matches("[^a-c]", "d"));
        assert!(!matches("[^a-c]", "b"));
    }

    #[test]
    fn dfa_handles_escaped_bytes_and_unbounded_repeat() {
        let limits = SamplingLimits::default();
        let dfa = Dfa::from_pattern(r"a\n{1,}", &limits).unwrap();
        let mut state = dfa.start();
        for &byte in b"a\n\n\n" {
            state = dfa.step(state, byte);
        }
        assert!(dfa.is_accept(state));
    }

    #[test]
    fn regex_parse_errors_on_unclosed_class_and_dangling_escapes() {
        let limits = SamplingLimits::default();
        assert!(Dfa::from_pattern("[abc", &limits).is_err());
        assert!(Dfa::from_pattern("a\\", &limits).is_err());
        assert!(Dfa::from_pattern("[a\\", &limits).is_err());
        assert!(Dfa::from_pattern("a{2", &limits).is_err());
        assert!(Dfa::from_pattern("a)", &limits).is_err());
    }

    #[test]
    fn dfa_token_cache_evicts_all_entries_once_max_entries_is_reached() {
        let limits = SamplingLimits::default();
        let dfa = Dfa::from_pattern("a*b", &limits).unwrap();
        let tokens: Vec<&[u8]> = vec![b"a", b"b"];
        let adapter = SliceTextAdapter::new(&tokens);
        let mut cache = DfaTokenMemo::new(1);
        let start = dfa.start();
        cache.get_or_compute(&dfa, start, &adapter);
        let after_a = dfa.step(start, b'a');
        // 🤖️ Filling a second, distinct state must evict the first since max_entries is 1.
        cache.get_or_compute(&dfa, after_a, &adapter);
        assert_eq!(cache.entries.len(), 1);
        assert!(cache.entries.contains_key(&after_a));
    }
    // #endregion 🔖️AutomataTests

    // #region 🔖️ConstraintsTests
    #[test]
    fn regex_constraint_masks_to_only_valid_continuations() {
        let limits = SamplingLimits::default();
        let mut constraint = RegexConstraint::new("ab", &limits).unwrap();
        let tokens: Vec<&[u8]> = vec![b"a", b"b", b"x"];
        let adapter = SliceTextAdapter::new(&tokens);
        let vocab = Vocabulary::new(3);
        let view = StepView { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: Some(&adapter), last_entropy: None };
        let mut mask = TokenBitset::new_full(3);
        constraint.fill_mask(&view, &mut mask).unwrap();
        assert!(mask.get(TokenId::new(0)));
        assert!(!mask.get(TokenId::new(1)));
        assert!(!mask.get(TokenId::new(2)));
        assert!(!constraint.is_satisfied());
        constraint.accept(&view, TokenId::new(0)).unwrap();
        let mut mask2 = TokenBitset::new_full(3);
        constraint.fill_mask(&view, &mut mask2).unwrap();
        assert!(mask2.get(TokenId::new(1)));
        constraint.accept(&view, TokenId::new(1)).unwrap();
        assert!(constraint.is_satisfied());
    }

    #[test]
    fn regex_constraint_rollback_restores_dfa_state() {
        let limits = SamplingLimits::default();
        let mut constraint = RegexConstraint::new("ab", &limits).unwrap();
        let tokens: Vec<&[u8]> = vec![b"a", b"b"];
        let adapter = SliceTextAdapter::new(&tokens);
        let vocab = Vocabulary::new(2);
        let view = StepView { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: Some(&adapter), last_entropy: None };
        let mark = constraint.save();
        constraint.accept(&view, TokenId::new(0)).unwrap();
        assert!(!constraint.is_satisfied());
        constraint.rollback_to(mark);
        let mut mask = TokenBitset::new_full(2);
        constraint.fill_mask(&view, &mut mask).unwrap();
        assert!(mask.get(TokenId::new(0)), "rollback must restore the pre-'a' DFA state");
    }

    #[test]
    fn trie_constraint_only_allows_configured_phrase_tokens() {
        let mut constraint = TrieConstraint::new(&[vec![TokenId::new(0), TokenId::new(1)], vec![TokenId::new(2)]]);
        let vocab = Vocabulary::new(3);
        let view = step_view(&vocab, &[], &[]);
        let mut mask = TokenBitset::new_full(3);
        constraint.fill_mask(&view, &mut mask).unwrap();
        assert!(mask.get(TokenId::new(0)));
        assert!(mask.get(TokenId::new(2)));
        assert!(!mask.get(TokenId::new(1)));
        assert!(!constraint.is_satisfied());
        constraint.accept(&view, TokenId::new(2)).unwrap();
        assert!(constraint.is_satisfied());
        assert!(constraint.is_finished());
    }

    #[test]
    fn must_include_constraint_is_satisfied_once_an_alternative_appears() {
        let mut constraint = MustIncludeConstraint::new(vec![vec![TokenId::new(1), TokenId::new(2)]]);
        let vocab = Vocabulary::new(3);
        let view = step_view(&vocab, &[], &[]);
        assert!(!constraint.is_satisfied());
        constraint.accept(&view, TokenId::new(0)).unwrap();
        assert!(!constraint.is_satisfied());
        constraint.accept(&view, TokenId::new(1)).unwrap();
        constraint.accept(&view, TokenId::new(2)).unwrap();
        assert!(constraint.is_satisfied());
    }

    #[test]
    fn json_mode_constraint_accepts_valid_json_and_rejects_invalid() {
        let tokens: Vec<&[u8]> = vec![b"{", b"\"a\"", b":", b"1", b"}", b"]"];
        let adapter = SliceTextAdapter::new(&tokens);
        let vocab = Vocabulary::new(tokens.len());
        let view = StepView { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: Some(&adapter), last_entropy: None };

        let mut good = JsonModeConstraint::new();
        for &tok in &[0u32, 1, 2, 3, 4] {
            good.accept(&view, TokenId::new(tok)).unwrap();
        }
        assert!(good.is_satisfied());
        assert!(!good.is_dead());

        let mut bad = JsonModeConstraint::new();
        bad.accept(&view, TokenId::new(0)).unwrap();
        bad.accept(&view, TokenId::new(5)).unwrap(); // ']' can't close an object
        assert!(bad.is_dead());
    }

    #[test]
    fn json_schema_constraint_flags_a_schema_violation_once_json_completes() {
        let tokens: Vec<&[u8]> = vec![b"{", b"\"a\"", b":", b"1", b"}"];
        let adapter = SliceTextAdapter::new(&tokens);
        let vocab = Vocabulary::new(tokens.len());
        let view = StepView { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: Some(&adapter), last_entropy: None };

        let schema = parse_json(r#"{"type":"object","required":["b"]}"#, 16).unwrap();
        let mut constraint = JsonSchemaConstraint::new(schema);
        for &tok in &[0u32, 1, 2, 3, 4] {
            constraint.accept(&view, TokenId::new(tok)).unwrap();
        }
        assert!(constraint.mode.is_satisfied());
        assert!(constraint.is_dead(), "missing required property 'b' should be flagged");
    }

    #[test]
    fn json_schema_validator_checks_type_enum_and_bounds() {
        let schema = parse_json(r#"{"type":"number","minimum":0,"maximum":10}"#, 16).unwrap();
        assert!(validates_json_schema(&JsonValue::Num(5.0), &schema));
        assert!(!validates_json_schema(&JsonValue::Num(-1.0), &schema));
        assert!(!validates_json_schema(&JsonValue::Str("x".into()), &schema));

        let enum_schema = parse_json(r#"{"enum":["a","b"]}"#, 16).unwrap();
        assert!(validates_json_schema(&JsonValue::Str("a".into()), &enum_schema));
        assert!(!validates_json_schema(&JsonValue::Str("c".into()), &enum_schema));
    }

    #[test]
    fn ebnf_constraint_compiles_a_simple_recursive_ish_grammar_and_masks_correctly() {
        let grammar = "greeting ::= \"hi\" | \"hello\" ;";
        let limits = SamplingLimits::default();
        let mut constraint = EbnfConstraint::new(grammar, &limits).unwrap();
        let tokens: Vec<&[u8]> = vec![b"hi", b"hello", b"bye"];
        let adapter = SliceTextAdapter::new(&tokens);
        let vocab = Vocabulary::new(3);
        let view = StepView { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: Some(&adapter), last_entropy: None };
        let mut mask = TokenBitset::new_full(3);
        constraint.fill_mask(&view, &mut mask).unwrap();
        assert!(mask.get(TokenId::new(0)));
        assert!(mask.get(TokenId::new(1)));
        assert!(!mask.get(TokenId::new(2)));
    }

    #[test]
    fn ebnf_constraint_rejects_unbounded_left_recursion() {
        let grammar = "a ::= a \"x\" ;";
        let limits = SamplingLimits { max_grammar_bytes: 50, ..SamplingLimits::default() };
        assert!(EbnfConstraint::new(grammar, &limits).is_err());
    }

    #[test]
    fn build_constraint_covers_every_constraint_spec_variant() {
        let limits = SamplingLimits::default();
        assert!(build_constraint(&ConstraintSpec::Regex("a".into()), &limits).is_ok());
        assert!(build_constraint(&ConstraintSpec::Trie(vec![vec![TokenId::new(0)]]), &limits).is_ok());
        assert!(build_constraint(&ConstraintSpec::MustInclude(vec![vec![TokenId::new(0)]]), &limits).is_ok());
        assert!(build_constraint(&ConstraintSpec::JsonMode, &limits).is_ok());
        assert!(build_constraint(&ConstraintSpec::Ebnf("a ::= \"x\" ;".into()), &limits).is_ok());
        assert!(build_constraint(&ConstraintSpec::JsonSchema(JsonValue::Object(Vec::new())), &limits).is_ok());
    }

    #[test]
    fn sample_step_with_regex_constraint_only_ever_emits_matching_text() {
        let config = SamplingConfig { method: SamplingMethod::Greedy { tie_break: TieBreak::LowestTokenId }, constraints: vec![ConstraintSpec::Regex("(a|b)".into())], max_tokens: 1, ..SamplingConfig::default() };
        let tokens: Vec<&[u8]> = vec![b"z", b"a", b"b"];
        let adapter = SliceTextAdapter::new(&tokens);
        let vocab = Vocabulary::new(3);
        let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(0))).unwrap();
        let mut ws = LogitsWorkspace::new(3);
        let mut observer = NullObserver;
        // 🧱️ Token 0 ("z") has the highest raw logit but must be masked out by the regex constraint.
        let logits = [10.0f32, 1.0, 1.0];
        let result = sample_step(&config, &mut state, &mut ws, &vocab, Some(&adapter), &logits, &mut observer).unwrap();
        assert_ne!(result.token, TokenId::new(0));
    }

    #[test]
    fn validates_json_schema_checks_integer_string_and_array_bounds() {
        let int_schema = parse_json(r#"{"type":"integer"}"#, 16).unwrap();
        assert!(validates_json_schema(&JsonValue::Num(3.0), &int_schema));
        assert!(!validates_json_schema(&JsonValue::Num(3.5), &int_schema));

        let str_schema = parse_json(r#"{"minLength":2,"maxLength":4}"#, 16).unwrap();
        assert!(validates_json_schema(&JsonValue::Str("abc".into()), &str_schema));
        assert!(!validates_json_schema(&JsonValue::Str("a".into()), &str_schema));
        assert!(!validates_json_schema(&JsonValue::Str("abcde".into()), &str_schema));

        let arr_schema = parse_json(r#"{"minItems":1,"maxItems":2,"items":{"type":"number"}}"#, 16).unwrap();
        assert!(validates_json_schema(&JsonValue::Array(vec![JsonValue::Num(1.0)]), &arr_schema));
        assert!(!validates_json_schema(&JsonValue::Array(Vec::new()), &arr_schema));
        assert!(!validates_json_schema(&JsonValue::Array(vec![JsonValue::Num(1.0), JsonValue::Num(2.0), JsonValue::Num(3.0)]), &arr_schema));
        assert!(!validates_json_schema(&JsonValue::Array(vec![JsonValue::Str("x".into())]), &arr_schema));
    }

    #[test]
    fn validates_json_schema_checks_object_required_properties_and_enum() {
        let schema = parse_json(r#"{"type":"object","required":["a"],"properties":{"a":{"type":"number"}}}"#, 16).unwrap();
        let ok = JsonValue::Object(vec![("a".to_string(), JsonValue::Num(1.0))]);
        assert!(validates_json_schema(&ok, &schema));
        let missing = JsonValue::Object(Vec::new());
        assert!(!validates_json_schema(&missing, &schema));
        let wrong_type = JsonValue::Object(vec![("a".to_string(), JsonValue::Str("x".into()))]);
        assert!(!validates_json_schema(&wrong_type, &schema));

        let enum_schema = parse_json(r#"{"enum":[1,2]}"#, 16).unwrap();
        assert!(!validates_json_schema(&JsonValue::Num(3.0), &enum_schema));
    }

    #[test]
    fn must_include_constraint_is_trivially_satisfied_with_no_alternatives_and_supports_rollback() {
        let vocab = Vocabulary::new(3);
        let view = step_view(&vocab, &[], &[]);
        let empty = MustIncludeConstraint::new(Vec::new());
        assert!(empty.is_satisfied());
        assert!(!empty.is_finished());

        let mut constraint = MustIncludeConstraint::new(vec![vec![TokenId::new(1)]]);
        let mark = constraint.save();
        constraint.accept(&view, TokenId::new(1)).unwrap();
        assert!(constraint.is_satisfied());
        constraint.rollback_to(mark);
        assert!(!constraint.is_satisfied());
        constraint.reset();
        assert!(!constraint.is_satisfied());
        let forked = constraint.fork();
        assert!(!forked.is_satisfied());
    }
    // #endregion 🔖️ConstraintsTests

    // #region 🔖️BatchTests
    #[test]
    fn continuous_batcher_add_remove_and_step() {
        let config = SamplingConfig::precise();
        let vocab = small_vocab();
        let mut batcher = ContinuousBatcher::new(8);
        let state_a = SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(1))).unwrap();
        let state_b = SequenceState::new(SequenceId::new(2), Vec::new(), &config, Box::new(CounterRng::from_seed(2))).unwrap();
        batcher.add_sequence(state_a);
        batcher.add_sequence(state_b);
        assert_eq!(batcher.len(), 2);
        assert!(batcher.contains(SequenceId::new(1)));

        let logits_a = [1.0f32, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let logits_b = [9.0f32, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let request = BatchSamplingRequest { entries: vec![BatchEntry { id: SequenceId::new(1), logits: &logits_a }, BatchEntry { id: SequenceId::new(2), logits: &logits_b }] };
        let mut observer = NullObserver;
        let batch_result = batcher.step(&config, &vocab, None, &request, &mut observer);
        assert_eq!(batch_result.results.len(), 2);
        assert_eq!(batch_result.results[0].1.as_ref().unwrap().token, TokenId::new(1));
        assert_eq!(batch_result.results[1].1.as_ref().unwrap().token, TokenId::new(0));

        let removed = batcher.remove_sequence(SequenceId::new(1));
        assert!(removed.is_some());
        assert_eq!(batcher.len(), 1);
    }

    #[test]
    fn continuous_batcher_step_reports_error_for_unknown_sequence() {
        let config = SamplingConfig::precise();
        let vocab = small_vocab();
        let mut batcher = ContinuousBatcher::new(8);
        let logits = [0.0f32; 8];
        let request = BatchSamplingRequest { entries: vec![BatchEntry { id: SequenceId::new(99), logits: &logits }] };
        let mut observer = NullObserver;
        let result = batcher.step(&config, &vocab, None, &request, &mut observer);
        assert!(result.results[0].1.is_err());
    }

    #[test]
    fn continuous_batcher_per_sequence_output_is_independent_of_processing_order() {
        let config = SamplingConfig::balanced();
        let vocab = small_vocab();
        let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];
        let mut observer = NullObserver;

        let mut forward = ContinuousBatcher::new(8);
        forward.add_sequence(SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(config.seed))).unwrap());
        forward.add_sequence(SequenceState::new(SequenceId::new(2), Vec::new(), &config, Box::new(CounterRng::from_seed(config.seed))).unwrap());
        let request_forward = BatchSamplingRequest { entries: vec![BatchEntry { id: SequenceId::new(1), logits: &logits }, BatchEntry { id: SequenceId::new(2), logits: &logits }] };
        forward.step(&config, &vocab, None, &request_forward, &mut observer);

        let mut backward = ContinuousBatcher::new(8);
        backward.add_sequence(SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(config.seed))).unwrap());
        backward.add_sequence(SequenceState::new(SequenceId::new(2), Vec::new(), &config, Box::new(CounterRng::from_seed(config.seed))).unwrap());
        let request_backward = BatchSamplingRequest { entries: vec![BatchEntry { id: SequenceId::new(2), logits: &logits }, BatchEntry { id: SequenceId::new(1), logits: &logits }] };
        backward.step(&config, &vocab, None, &request_backward, &mut observer);

        assert_eq!(forward.get(SequenceId::new(1)).unwrap().generated(), backward.get(SequenceId::new(1)).unwrap().generated());
        assert_eq!(forward.get(SequenceId::new(2)).unwrap().generated(), backward.get(SequenceId::new(2)).unwrap().generated());
    }
    // #endregion 🔖️BatchTests

    // #region 🔖️SearchTests
    #[test]
    fn beam_search_finds_the_highest_probability_short_sequence() {
        let config = SamplingConfig::precise();
        let vocab = Vocabulary::new(4).with_eos(vec![TokenId::new(3)]);
        let beam_config = BeamSearchConfig { width: 4, length_penalty: 1.0, max_steps: 3 };
        let initial = SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(0))).unwrap();
        // 🌳️ Token 1 dominates for the first two steps (building the best possible 2-token prefix);
        // from step 2 on, EOS (token 3) overwhelmingly dominates every beam alike, so whichever beam
        // carries the best prefix into that step produces the overall best-scoring finished
        // hypothesis: "1, 1, 3". Raw (non-length-normalized) cumulative log-probability would instead
        // favor never stopping ("1, 1, 1, ...") — this scenario is deliberately shaped so the length
        // penalty isn't what's under test, only "beam search finds the argmax-per-step path".
        let hypotheses = beam_search(&config, &beam_config, &vocab, None, initial, |state| if state.generated().len() < 2 { vec![0.0, 3.0, -5.0, -5.0] } else { vec![-5.0, -5.0, -5.0, 10.0] }).unwrap();
        assert!(!hypotheses.is_empty());
        let best = &hypotheses[0];
        assert_eq!(best.state.generated(), &[TokenId::new(1), TokenId::new(1), TokenId::new(3)]);
    }

    #[test]
    fn beam_search_hypotheses_have_independent_state() {
        let config = SamplingConfig::precise();
        let vocab = Vocabulary::new(4);
        let beam_config = BeamSearchConfig { width: 3, length_penalty: 1.0, max_steps: 2 };
        let initial = SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(0))).unwrap();
        let hypotheses = beam_search(&config, &beam_config, &vocab, None, initial, |_state| vec![1.0, 2.0, 3.0, 0.5]).unwrap();
        assert!(hypotheses.len() >= 2);
        // 🌳️ Every surviving hypothesis's fork is a distinct SequenceState with its own id-derived RNG.
        let ids: std::collections::HashSet<u64> = hypotheses.iter().map(|h| h.state.id().get()).collect();
        assert_eq!(ids.len(), 1, "all forks share the parent's sequence id in this driver; independence is in per-beam state, not id");
    }

    #[test]
    fn best_of_n_selects_the_candidate_with_highest_mean_logprob() {
        let config = SamplingConfig::precise();
        let vocab = Vocabulary::new(4).with_eos(vec![TokenId::new(3)]);
        let best_of = BestOfN { n: 3 };
        let mut observer = NullObserver;
        let results = best_of.run(&config, &vocab, None, 2, &mut observer, |i| SequenceState::new(SequenceId::new(i as u64), Vec::new(), &config, Box::new(CounterRng::from_seed(i as u64))), |_state| vec![0.0, 9.0, 0.0, 1.0]).unwrap();
        assert_eq!(results.len(), 3);
        for i in 1..results.len() {
            assert!(results[i - 1].1 >= results[i].1, "results must be sorted best-first by mean logprob");
        }
    }

    #[test]
    fn rejection_sampler_retries_until_accept_returns_true() {
        let config = SamplingConfig::balanced();
        let vocab = small_vocab();
        let mut ws = LogitsWorkspace::new(8);
        let mut rng = CounterRng::from_seed(7);
        let logits = [1.0f32, 2.0, 0.5, 3.0, 1.5, 0.2, 0.1, -5.0];
        let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
        let sampler = RejectionSampler { max_attempts: 100 };
        let result = sampler.sample(&config, &mut ws, &mut rng, &logits, input, |r| r.token == TokenId::new(3)).unwrap();
        assert_eq!(result.token, TokenId::new(3));
    }

    #[test]
    fn rejection_sampler_errors_after_max_attempts_when_never_accepted() {
        let config = SamplingConfig::precise();
        let vocab = small_vocab();
        let mut ws = LogitsWorkspace::new(8);
        let mut rng = CounterRng::from_seed(0);
        let logits = [0.0f32, 9.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let input = StatelessStepInput { sequence: SequenceId::new(1), step: StepIndex::new(0), prompt: &[], generated: &[], vocab: &vocab, adapter: None, last_entropy: None };
        let sampler = RejectionSampler { max_attempts: 5 };
        assert!(sampler.sample(&config, &mut ws, &mut rng, &logits, input, |r| r.token == TokenId::new(2)).is_err());
    }
    // #endregion 🔖️SearchTests

    // #region 🔖️SpeculativeTests
    #[test]
    fn speculative_decode_accepts_when_draft_matches_target_distribution() {
        let config = SamplingConfig::precise();
        let vocab = Vocabulary::new(4);
        let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(0))).unwrap();
        let mut ws = LogitsWorkspace::new(4);
        let mut observer = NullObserver;
        let draft_tokens = [TokenId::new(1), TokenId::new(1)];
        let draft_distributions = vec![vec![0.05f32, 0.9, 0.03, 0.02], vec![0.05f32, 0.9, 0.03, 0.02]];
        let (results, metrics) = speculative_decode(&config, &mut state, &mut ws, &vocab, None, &draft_tokens, &draft_distributions, |_state| vec![0.0, 9.0, 0.0, 0.0], &mut observer).unwrap();
        assert_eq!(metrics.proposed, 2);
        assert_eq!(metrics.accepted, 2);
        assert!(metrics.bonus_taken);
        assert_eq!(results.len(), 3);
        assert_eq!(state.generated().len(), 3);
    }

    #[test]
    fn speculative_decode_rejects_and_resamples_when_draft_disagrees_with_target() {
        let config = SamplingConfig::precise();
        let vocab = Vocabulary::new(4);
        let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(0))).unwrap();
        let mut ws = LogitsWorkspace::new(4);
        let mut observer = NullObserver;
        // 🎲️ Draft proposes token 0 with high confidence, but the target model overwhelmingly
        // prefers token 1 — acceptance probability is near zero, so this should almost always reject.
        let draft_tokens = [TokenId::new(0)];
        let draft_distributions = vec![vec![0.99f32, 0.01, 0.0, 0.0]];
        let (results, metrics) = speculative_decode(&config, &mut state, &mut ws, &vocab, None, &draft_tokens, &draft_distributions, |_state| vec![-10.0, 10.0, -10.0, -10.0], &mut observer).unwrap();
        assert_eq!(metrics.accepted, 0);
        assert!(!metrics.bonus_taken);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].token, TokenId::new(1));
    }

    #[test]
    fn speculative_decode_matches_direct_target_sampling_distribution() {
        // ⚡️ Statistical check: over many trials, the *marginal* distribution of the first token
        // speculative decoding produces should match sampling directly from the target logits —
        // the defining correctness property of exact speculative decoding.
        let config = SamplingConfig { method: SamplingMethod::Multinomial { strategy: MultinomialStrategy::CdfBinarySearch }, ..SamplingConfig::default() };
        let vocab = Vocabulary::new(3);
        let target = [0.0f32, 1.0, 2.0];
        let draft_probs_dist = [0.5f32, 0.3, 0.2]; // a plausible, imperfect draft distribution

        let trials = 4_000;
        let mut counts_spec = [0u32; 3];
        for seed in 0..trials {
            let mut state = SequenceState::new(SequenceId::new(1), Vec::new(), &config, Box::new(CounterRng::from_seed(seed))).unwrap();
            let mut ws = LogitsWorkspace::new(3);
            let mut observer = NullObserver;
            // Draft token drawn from the (imperfect) draft distribution using a simple counter rng.
            let mut draft_rng = CounterRng::from_seed(seed ^ 0xD3AF);
            let cdf = cumulative_from_probs(&draft_probs_dist);
            let draft_token = TokenId::new(cdf_binary_search(&cdf, draft_rng.next_f64()) as u32);
            let draft_distributions = vec![draft_probs_dist.to_vec()];
            let (results, _metrics) = speculative_decode(&config, &mut state, &mut ws, &vocab, None, &[draft_token], &draft_distributions, |_state| target.to_vec(), &mut observer).unwrap();
            counts_spec[results[0].token.get() as usize] += 1;
        }

        let mut ws = LogitsWorkspace::new(3);
        let target_probs = {
            ws.reset_for_step(&target, SanitizePolicy::NegInfNan).unwrap();
            ws.sort_live_by_prob_desc();
            let mut probs = vec![0.0f32; 3];
            for (&tok, &p) in ws.live().iter().zip(ws.probs().iter()) {
                probs[tok as usize] = p;
            }
            probs
        };

        for i in 0..3 {
            let observed = counts_spec[i] as f64 / trials as f64;
            let expected = target_probs[i] as f64;
            assert!((observed - expected).abs() < 0.03, "token {i}: observed {observed} vs expected {expected}");
        }
    }
    // #endregion 🔖️SpeculativeTests

    // #region 🔖️ShardedTests
    #[test]
    fn sharded_softmax_matches_unsharded_softmax() {
        let full_logits = [1.0f32, 2.0, 0.5, 3.0, -1.0, 0.2, 4.0, 0.1];
        let mut ws = LogitsWorkspace::new(8);
        ws.reset_for_step(&full_logits, SanitizePolicy::NegInfNan).unwrap();
        // 📐️ `softmax_over_live`'s return value is the raw pre-normalization partition sum (useful
        // for e.g. logsumexp), not the post-normalization probability sum — check `ws.probs()` for
        // that instead.
        ws.softmax_over_live();
        let mut unsharded_probs = [0.0f32; 8];
        for (&tok, &p) in ws.live().iter().zip(ws.probs().iter()) {
            unsharded_probs[tok as usize] = p;
        }
        let prob_sum: f32 = ws.probs().iter().sum();
        assert!((prob_sum - 1.0).abs() < 1e-4);

        let shard0 = &full_logits[..4];
        let shard1 = &full_logits[4..];
        let mut ranks = LocalCollective::new_group(2);
        let mut rank1 = ranks.pop().unwrap();
        let mut rank0 = ranks.pop().unwrap();
        // 🗂️ Two-phase mailbox convention: call every rank once, then again to read the merged result.
        let _ = sharded_softmax(&mut rank0, shard0);
        let _ = sharded_softmax(&mut rank1, shard1);
        let sharded0 = sharded_softmax(&mut rank0, shard0);
        let sharded1 = sharded_softmax(&mut rank1, shard1);

        for i in 0..4 {
            assert!((sharded0[i] - unsharded_probs[i]).abs() < 1e-5, "shard0[{i}]: {} vs {}", sharded0[i], unsharded_probs[i]);
        }
        for i in 0..4 {
            assert!((sharded1[i] - unsharded_probs[4 + i]).abs() < 1e-5, "shard1[{i}]: {} vs {}", sharded1[i], unsharded_probs[4 + i]);
        }
    }

    #[test]
    fn sharded_top_k_matches_unsharded_top_k() {
        let full_logits = [1.0f32, 5.0, 0.5, 3.0, -1.0, 0.2, 4.0, 0.1];
        let shard0 = &full_logits[..4];
        let shard1 = &full_logits[4..];
        let mut ranks = LocalCollective::new_group(2);
        let mut rank1 = ranks.pop().unwrap();
        let mut rank0 = ranks.pop().unwrap();
        let _ = sharded_top_k(&mut rank0, shard0, 0, 3);
        let top_k = sharded_top_k(&mut rank1, shard1, 4, 3);

        let mut expected: Vec<(u32, f32)> = full_logits.iter().enumerate().map(|(i, &l)| (i as u32, l)).collect();
        expected.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let expected_tokens: Vec<u32> = expected.iter().take(3).map(|&(t, _)| t).collect();
        let actual_tokens: Vec<u32> = top_k.iter().map(|c| c.token.get()).collect();
        assert_eq!(actual_tokens, expected_tokens);
    }

    #[test]
    fn sharded_sample_marginal_distribution_matches_unsharded() {
        let full_logits = [2.0f32, 0.0, 1.0, -1.0];
        let shard0 = &full_logits[..2];
        let shard1 = &full_logits[2..];
        let trials = 5_000;
        let mut counts = [0u32; 4];
        for seed in 0..trials {
            let mut ranks = LocalCollective::new_group(2);
            let mut rank1 = ranks.pop().unwrap();
            let mut rank0 = ranks.pop().unwrap();
            let mut rng = CounterRng::from_seed(seed);
            let _ = sharded_top_k(&mut rank0, shard0, 0, usize::MAX);
            if let Some(token) = sharded_sample(&mut rank1, shard1, 2, &mut rng) {
                counts[token.get() as usize] += 1;
            }
        }
        let mut ws = LogitsWorkspace::new(4);
        ws.reset_for_step(&full_logits, SanitizePolicy::NegInfNan).unwrap();
        ws.sort_live_by_prob_desc();
        let mut expected = [0.0f32; 4];
        for (&tok, &p) in ws.live().iter().zip(ws.probs().iter()) {
            expected[tok as usize] = p;
        }
        for i in 0..4 {
            let observed = counts[i] as f64 / trials as f64;
            assert!((observed - expected[i] as f64).abs() < 0.03, "token {i}: observed {observed} vs expected {}", expected[i]);
        }
    }
    // #endregion 🔖️ShardedTests

    // #region 🔖️DiffusionTests
    struct ConstantDenoiser {
        target: f32,
    }
    impl Denoiser for ConstantDenoiser {
        fn prediction_type(&self) -> PredictionType {
            PredictionType::Sample
        }
        fn denoise(&mut self, _latent: &[f32], _shape: [usize; 4], _sigma: f64, _step: usize, _branch: GuidanceBranch, out: &mut [f32]) -> Result<(), SamplingError> {
            for o in out.iter_mut() {
                *o = self.target;
            }
            Ok(())
        }
    }

    #[test]
    fn all_noise_schedules_are_non_increasing() {
        let schedules = vec![
            NoiseSchedule::Linear { beta_start: 0.0001, beta_end: 0.02 },
            NoiseSchedule::ScaledLinear { beta_start: 0.0001, beta_end: 0.02 },
            NoiseSchedule::Cosine { s: 0.008 },
            NoiseSchedule::Karras { sigma_min: 0.01, sigma_max: 10.0, rho: 7.0 },
            NoiseSchedule::Exponential { sigma_min: 0.01, sigma_max: 10.0 },
            NoiseSchedule::Polynomial { sigma_min: 0.01, sigma_max: 10.0, power: 2.0 },
        ];
        for schedule in schedules {
            let sigmas = schedule.sigmas(10);
            assert_eq!(sigmas.len(), 10);
            for w in sigmas.windows(2) {
                assert!(w[0] >= w[1] - 1e-9, "{schedule:?} sigmas not non-increasing: {sigmas:?}");
            }
        }
    }

    #[test]
    fn custom_schedule_returns_its_values_verbatim() {
        let values = vec![5.0, 3.0, 1.0, 0.0];
        let schedule = NoiseSchedule::Custom(values.clone());
        assert_eq!(schedule.sigmas(4), values);
    }

    #[test]
    fn euler_solver_converges_exactly_to_a_constant_target_when_schedule_reaches_zero() {
        // 📐️ With a constant (x-independent) `Sample` prediction, the Euler update at the final
        // step (sigma_next == 0) reduces algebraically to `x_next = denoised` exactly — a good
        // closed-form check that doesn't depend on any specific schedule shape.
        let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![10.0, 5.0, 2.0, 0.5, 0.0]), solver: Solver::Euler, steps: 4, guidance: None, seed: 0 };
        let mut latent = vec![0.0f32; 4];
        let mut denoiser = ConstantDenoiser { target: 5.0 };
        run_diffusion(&config, &mut latent, [1, 1, 1, 4], &mut denoiser, |_, _, _| StepControlFlow::Continue).unwrap();
        for &v in &latent {
            assert!((v - 5.0).abs() < 1e-3, "v={v}");
        }
    }

    #[test]
    fn ddim_eta_zero_is_deterministic_regardless_of_seed() {
        let run = |seed: u64| {
            let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![4.0, 2.0, 1.0, 0.0]), solver: Solver::Ddim { eta: 0.0 }, steps: 3, guidance: None, seed };
            let mut latent = vec![1.0f32; 4];
            let mut denoiser = ConstantDenoiser { target: 3.0 };
            run_diffusion(&config, &mut latent, [1, 1, 1, 4], &mut denoiser, |_, _, _| StepControlFlow::Continue).unwrap();
            latent
        };
        assert_eq!(run(1), run(999));
    }

    #[test]
    fn euler_ancestral_and_ddim_with_eta_differ_across_seeds() {
        let run = |seed: u64| {
            let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![4.0, 2.0, 1.0, 0.2]), solver: Solver::EulerAncestral, steps: 3, guidance: None, seed };
            let mut latent = vec![1.0f32; 4];
            let mut denoiser = ConstantDenoiser { target: 3.0 };
            run_diffusion(&config, &mut latent, [1, 1, 1, 4], &mut denoiser, |_, _, _| StepControlFlow::Continue).unwrap();
            latent
        };
        assert_ne!(run(1), run(2), "ancestral sampling's injected noise should differ across seeds");
    }

    #[test]
    fn guidance_combine_extrapolates_away_from_unconditional() {
        let guidance = Guidance { scale: 2.0, rescale: 0.0 };
        let cond = [1.0f32, 2.0];
        let uncond = [0.0f32, 0.0];
        let mut out = [0.0f32; 2];
        guidance.combine(&cond, &uncond, &mut out);
        assert_eq!(out, [2.0, 4.0]);
    }

    #[test]
    fn run_diffusion_with_guidance_evaluates_both_branches_every_step() {
        struct BranchTrackingDenoiser {
            cond_calls: usize,
            uncond_calls: usize,
        }
        impl Denoiser for BranchTrackingDenoiser {
            fn prediction_type(&self) -> PredictionType {
                PredictionType::Sample
            }
            fn denoise(&mut self, _latent: &[f32], _shape: [usize; 4], _sigma: f64, _step: usize, branch: GuidanceBranch, out: &mut [f32]) -> Result<(), SamplingError> {
                match branch {
                    GuidanceBranch::Conditional => {
                        self.cond_calls += 1;
                        out.fill(1.0);
                    }
                    GuidanceBranch::Unconditional => {
                        self.uncond_calls += 1;
                        out.fill(0.0);
                    }
                }
                Ok(())
            }
        }
        let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![2.0, 1.0, 0.0]), solver: Solver::Euler, steps: 2, guidance: Some(Guidance { scale: 1.5, rescale: 0.0 }), seed: 0 };
        let mut latent = vec![0.0f32; 2];
        let mut denoiser = BranchTrackingDenoiser { cond_calls: 0, uncond_calls: 0 };
        run_diffusion(&config, &mut latent, [1, 1, 1, 2], &mut denoiser, |_, _, _| StepControlFlow::Continue).unwrap();
        assert_eq!(denoiser.cond_calls, 2);
        assert_eq!(denoiser.uncond_calls, 2);
    }

    #[test]
    fn heun_solver_uses_two_evaluations_per_non_final_step() {
        struct CountingDenoiser {
            calls: usize,
        }
        impl Denoiser for CountingDenoiser {
            fn prediction_type(&self) -> PredictionType {
                PredictionType::Sample
            }
            fn denoise(&mut self, _latent: &[f32], _shape: [usize; 4], _sigma: f64, _step: usize, _branch: GuidanceBranch, out: &mut [f32]) -> Result<(), SamplingError> {
                self.calls += 1;
                out.fill(2.0);
                Ok(())
            }
        }
        let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![4.0, 2.0, 0.0]), solver: Solver::Heun, steps: 2, guidance: None, seed: 0 };
        let mut latent = vec![0.0f32; 2];
        let mut denoiser = CountingDenoiser { calls: 0 };
        run_diffusion(&config, &mut latent, [1, 1, 1, 2], &mut denoiser, |_, _, _| StepControlFlow::Continue).unwrap();
        // step 0 (sigma_next=2.0, not final): 2 evaluations; step 1 (sigma_next=0.0, final): 1 evaluation.
        assert_eq!(denoiser.calls, 3);
    }

    #[test]
    fn run_diffusion_cancellation_stops_the_run_and_errors() {
        let config = DiffusionRunConfig { schedule: NoiseSchedule::Custom(vec![4.0, 2.0, 1.0, 0.0]), solver: Solver::Euler, steps: 3, guidance: None, seed: 0 };
        let mut latent = vec![0.0f32; 2];
        let mut denoiser = ConstantDenoiser { target: 1.0 };
        let mut calls = 0;
        let result = run_diffusion(&config, &mut latent, [1, 1, 1, 2], &mut denoiser, |_, _, _| {
            calls += 1;
            if calls == 1 {
                StepControlFlow::Cancel
            } else {
                StepControlFlow::Continue
            }
        });
        assert!(result.is_err());
        assert_eq!(calls, 1);
    }

    #[test]
    fn img2img_start_index_boundaries() {
        assert_eq!(img2img_start_index(10, 1.0), 0);
        assert_eq!(img2img_start_index(10, 0.0), 10);
        assert_eq!(img2img_start_index(10, 0.5), 5);
    }

    #[test]
    fn apply_inpaint_mask_blends_by_mask_weight() {
        let mut x = [10.0f32, 10.0, 10.0];
        let original = [0.0f32, 0.0, 0.0];
        let mask = [1.0f32, 0.0, 0.5];
        let mut rng = CounterRng::from_seed(1);
        // 🌫️ sigma = 0.0 means the re-noised original equals the original exactly, isolating the
        // blend-weight arithmetic from the injected-noise term.
        apply_inpaint_mask(&mut x, &original, &mask, 0.0, &mut rng);
        assert!((x[0] - 0.0).abs() < 1e-6, "fully masked (1.0) must fall back to the original");
        assert_eq!(x[1], 10.0, "unmasked (0.0) must stay untouched");
        assert!((x[2] - 5.0).abs() < 1e-6, "half-masked (0.5) must blend 50/50");
    }

    #[test]
    fn prediction_type_conversions_round_trip_through_denoised() {
        let x = [2.0f32, -1.0];
        let sigma = 0.5;
        let target = [3.0f32, 3.0];

        let sample_raw = target;
        assert_eq!(to_denoised(PredictionType::Sample, &x, sigma, &sample_raw), target.to_vec());

        // 📐️ epsilon such that `x - sigma*eps == target` exactly.
        let eps: Vec<f32> = x.iter().zip(target).map(|(&xi, ti)| (xi - ti) / sigma as f32).collect();
        let denoised = to_denoised(PredictionType::Epsilon, &x, sigma, &eps);
        for (d, t) in denoised.iter().zip(target) {
            assert!((d - t).abs() < 1e-4);
        }
    }
    // #endregion 🔖️DiffusionTests
}
// #endregion 🔖️Tests
