//! 🔢️ Fuzzy arithmetic, relations and possibility: fuzzy numbers, fuzzy relations and the possibility measure. See [`super`].

use super::*;

// #region 🔖️FuzzyArithmetic
/// 🔢️ Triangular fuzzy number for α-cut arithmetic.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct FuzzyNumber {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl FuzzyNumber {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn triangular(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn alpha_cut(&self, alpha: f64) -> (f64, f64) {
        let alpha = clamp01(alpha);
        if alpha <= 0.0 {
            return (self.b, self.b);
        }
        let left = self.a + alpha * (self.b - self.a);
        let right = self.c - alpha * (self.c - self.b);
        (left.min(right), left.max(right))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn defuzzify_centroid(&self, samples: usize) -> f64 {
        let mut num = 0.0;
        let mut den = 0.0;
        for i in 0..samples {
            let x = self.a + (self.c - self.a) * i as f64 / (samples - 1).max(1) as f64;
            let mu = MembershipFunction::triangular(self.a, self.b, self.c).eval(x);
            num += x * mu;
            den += mu;
        }
        if den.abs() < 1e-12 {
            self.b
        } else {
            num / den
        }
    }

    #[allow(clippy::should_implement_trait, reason = "value-semantics add used pervasively as a plain method by fuzzy arithmetic callers")]
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add(self, other: Self) -> Self {
        Self { a: self.a + other.a, b: self.b + other.b, c: self.c + other.c }
    }

    #[allow(clippy::should_implement_trait, reason = "value-semantics sub used pervasively as a plain method by fuzzy arithmetic callers")]
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sub(self, other: Self) -> Self {
        Self { a: self.a - other.c, b: self.b - other.b, c: self.c - other.a }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn scale(self, k: f64) -> Self {
        if k >= 0.0 {
            Self { a: self.a * k, b: self.b * k, c: self.c * k }
        } else {
            Self { a: self.c * k, b: self.b * k, c: self.a * k }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn fuzzy_add(a: FuzzyNumber, b: FuzzyNumber) -> FuzzyNumber {
    a.add(b)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn fuzzy_mul_interval(a: FuzzyNumber, b: FuzzyNumber, alpha: f64) -> (f64, f64) {
    let (al, ar) = a.alpha_cut(alpha);
    let (bl, br) = b.alpha_cut(alpha);
    let candidates = [al * bl, al * br, ar * bl, ar * br];
    (candidates.iter().copied().fold(f64::INFINITY, f64::min), candidates.iter().copied().fold(f64::NEG_INFINITY, f64::max))
}
// #endregion 🔖️FuzzyArithmetic

// #region 🔖️FuzzyRelation
/// 🔗️ Graded fuzzy relation R: X → Y stored as a matrix.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct FuzzyRelation {
    pub values: Vec<Vec<f64>>,
}

impl FuzzyRelation {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { values: vec![vec![0.0; cols]; rows] }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.values[row][col] = clamp01(value);
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.values[row][col]
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose_max_min(&self, other: &Self) -> FuzzyResult<Self> {
        if self.values[0].len() != other.values.len() {
            return Err(FuzzyError::DimensionMismatch("relation composition".into()));
        }
        let rows = self.values.len();
        let cols = other.values[0].len();
        let inner = other.values.len();
        let mut out = Self::new(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                let mut best = 0.0_f64;
                for k in 0..inner {
                    best = best.max(self.values[i][k].min(other.values[k][j]));
                }
                out.values[i][j] = best;
            }
        }
        Ok(out)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose_max_product(&self, other: &Self) -> FuzzyResult<Self> {
        if self.values[0].len() != other.values.len() {
            return Err(FuzzyError::DimensionMismatch("relation composition".into()));
        }
        let rows = self.values.len();
        let cols = other.values[0].len();
        let inner = other.values.len();
        let mut out = Self::new(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                let mut best = 0.0_f64;
                for k in 0..inner {
                    best = best.max(self.values[i][k] * other.values[k][j]);
                }
                out.values[i][j] = best;
            }
        }
        Ok(out)
    }
}
// #endregion 🔖️FuzzyRelation

// #region 🔖️PossibilityTheory
/// 🎭️ Possibility measure Π(A) = sup_{x∈A} μ(x) on a discrete universe.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct PossibilityMeasure {
    pub universe: Vec<f64>,
    pub membership: Vec<f64>,
}

impl PossibilityMeasure {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(universe: Vec<f64>, membership: Vec<f64>) -> FuzzyResult<Self> {
        if universe.len() != membership.len() || universe.is_empty() {
            return Err(FuzzyError::InvalidDomain("possibility universe".into()));
        }
        Ok(Self { universe, membership: membership.into_iter().map(clamp01).collect() })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn possibility(&self, predicate: impl Fn(f64) -> bool) -> f64 {
        self.universe.iter().zip(self.membership.iter()).filter(|(x, _)| predicate(**x)).map(|(_, mu)| *mu).fold(0.0, f64::max)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn necessity(&self, predicate: impl Fn(f64) -> bool) -> f64 {
        1.0 - self.possibility(|x| !predicate(x))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_scores(universe: Vec<f64>, scores: Vec<f64>) -> FuzzyResult<Self> {
        let max_score = scores.iter().copied().fold(0.0_f64, f64::max).max(1e-12);
        Self::new(universe, scores.into_iter().map(|s| s / max_score).collect())
    }
}
// #endregion 🔖️PossibilityTheory
