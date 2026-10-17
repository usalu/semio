//! ➗️ T-norms, t-conorms and hedges: the triangular norm and conorm families and the complement, concentration and dilation hedges. See [`super`].

use super::*;

// #region 🔖️TNormTConorm
/// 🔗️ T-norm for fuzzy AND aggregation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum TNorm {
    Min,
    Product,
    Lukasiewicz,
    Drastic,
}

impl TNorm {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(self, a: f64, b: f64) -> f64 {
        let a = clamp01(a);
        let b = clamp01(b);
        match self {
            Self::Min => a.min(b),
            Self::Product => a * b,
            Self::Lukasiewicz => (a + b - 1.0).max(0.0),
            Self::Drastic => {
                if b == 1.0 {
                    a
                } else if a == 1.0 {
                    b
                } else {
                    0.0
                }
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fold<I: Iterator<Item = f64>>(self, values: I) -> f64 {
        values.fold(1.0, |acc, v| self.apply(acc, v))
    }
}

/// 🔗️ T-conorm for fuzzy OR aggregation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum TConorm {
    Max,
    ProbSum,
    Lukasiewicz,
    NilpotentMax,
}

impl TConorm {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(self, a: f64, b: f64) -> f64 {
        let a = clamp01(a);
        let b = clamp01(b);
        match self {
            Self::Max => a.max(b),
            Self::ProbSum => a + b - a * b,
            Self::Lukasiewicz => (a + b).min(1.0),
            Self::NilpotentMax => {
                if a + b < 1.0 {
                    0.0
                } else {
                    a.max(b)
                }
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fold<I: Iterator<Item = f64>>(self, values: I) -> f64 {
        values.fold(0.0, |acc, v| self.apply(acc, v))
    }
}

/// 🌫️ Linguistic hedge modifiers.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Hedge {
    Very,
    Somewhat,
    MoreOrLess,
    Extremely,
}

impl Hedge {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn apply(self, mu: f64) -> f64 {
        let mu = clamp01(mu);
        match self {
            Self::Very => mu * mu,
            Self::Somewhat => mu.sqrt(),
            Self::MoreOrLess => 0.5 + 0.5 * (20.0 * (mu - 0.5)).sin(),
            Self::Extremely => mu * mu * mu,
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn complement(mu: f64) -> f64 {
    1.0 - clamp01(mu)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn concentration(mu: f64) -> f64 {
    clamp01(mu).powi(2)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dilation(mu: f64) -> f64 {
    clamp01(mu).sqrt()
}
// #endregion 🔖️TNormTConorm
