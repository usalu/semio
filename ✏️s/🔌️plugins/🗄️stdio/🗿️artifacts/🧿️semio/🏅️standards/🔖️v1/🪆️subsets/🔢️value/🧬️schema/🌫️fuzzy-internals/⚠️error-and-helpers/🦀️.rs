//! ⚠️ Fuzzy error and helpers: the fuzzy error type with its display and error impls, the result alias and the clamp, linspace and argmax helpers. See [`super`].

// #region 🔖️FuzzyError
/// ⚠️ Recoverable fuzzy-logic failures: invalid domains, empty rule bases, singular least-squares systems.
#[derive(Debug, PartialEq)]
pub enum FuzzyError {
    InvalidDomain(String),
    EmptyRuleBase,
    EmptyUniverse,
    SingularSystem,
    DimensionMismatch(String),
    InvalidIntuitionistic,
    NoFiredRules,
    InvalidParameterCount { expected: usize, got: usize },
}

impl std::fmt::Display for FuzzyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDomain(message) => write!(formatter, "invalid domain: {message}"),
            Self::EmptyRuleBase => formatter.write_str("empty rule base"),
            Self::EmptyUniverse => formatter.write_str("empty universe"),
            Self::SingularSystem => formatter.write_str("singular linear system"),
            Self::DimensionMismatch(message) => write!(formatter, "dimension mismatch: {message}"),
            Self::InvalidIntuitionistic => formatter.write_str("invalid intuitionistic set: membership + non-membership exceeds 1"),
            Self::NoFiredRules => formatter.write_str("no fired rules"),
            Self::InvalidParameterCount { expected, got } => write!(formatter, "invalid parameter count: expected {expected}, got {got}"),
        }
    }
}

impl std::error::Error for FuzzyError {}

pub type FuzzyResult<T> = Result<T, FuzzyError>;
// #endregion 🔖️FuzzyError

// #region 🔖️Helpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn clamp01(x: f64) -> f64 {
    x.clamp(0.0, 1.0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn linspace(min: f64, max: f64, n: usize) -> Vec<f64> {
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![min];
    }
    let step = (max - min) / (n - 1) as f64;
    (0..n).map(|i| min + step * i as f64).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn argmax(values: &[f64]) -> usize {
    values.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).map_or(0, |(i, _)| i)
}
// #endregion 🔖️Helpers
