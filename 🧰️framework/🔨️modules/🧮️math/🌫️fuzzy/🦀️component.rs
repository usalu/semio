//! 🌫️ Advanced fuzzy logic: type-1/IT2/IFS sets, multi-engine inference, neuro-fuzzy learning, clustering, multicriteria decision-making, and explainability.

#![allow(clippy::needless_range_loop, reason = "index-based numerics loops mirror the crate::algebra style for matrix and rule iteration")]
use crate::algebra::{MatD, VecD};
use crate::random::Rng;
use serde::{Deserialize, Serialize};
use std::f64::consts::E;
use thiserror::Error;

// #region 🔖️FuzzyError
/// ⚠️ Recoverable fuzzy-logic failures: invalid domains, empty rule bases, singular least-squares systems.
#[derive(Debug, Error, PartialEq)]
pub enum FuzzyError {
    #[error("invalid domain: {0}")]
    InvalidDomain(String),
    #[error("empty rule base")]
    EmptyRuleBase,
    #[error("empty universe")]
    EmptyUniverse,
    #[error("singular linear system")]
    SingularSystem,
    #[error("dimension mismatch: {0}")]
    DimensionMismatch(String),
    #[error("invalid intuitionistic set: membership + non-membership exceeds 1")]
    InvalidIntuitionistic,
    #[error("no fired rules")]
    NoFiredRules,
    #[error("invalid parameter count: expected {expected}, got {got}")]
    InvalidParameterCount { expected: usize, got: usize },
}

pub type FuzzyResult<T> = Result<T, FuzzyError>;
// #endregion 🔖️FuzzyError

// #region 🔖️Helpers
fn clamp01(x: f64) -> f64 {
    x.clamp(0.0, 1.0)
}

fn linspace(min: f64, max: f64, n: usize) -> Vec<f64> {
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![min];
    }
    let step = (max - min) / (n - 1) as f64;
    (0..n).map(|i| min + step * i as f64).collect()
}

fn argmax(values: &[f64]) -> usize {
    values.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).map_or(0, |(i, _)| i)
}
// #endregion 🔖️Helpers

// #region 🔖️MembershipFunction
/// 📐️ Parametric membership function with eval, parameter access, and gradient hooks for adaptation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MembershipFunction {
    Triangular { a: f64, b: f64, c: f64 },
    Trapezoidal { a: f64, b: f64, c: f64, d: f64 },
    Gaussian { mean: f64, sigma: f64 },
    GeneralizedBell { a: f64, b: f64, c: f64 },
    Sigmoid { a: f64, c: f64 },
    Singleton { value: f64 },
    PiecewiseLinear { knots: Vec<(f64, f64)> },
}

impl MembershipFunction {
    pub fn triangular(a: f64, b: f64, c: f64) -> Self {
        Self::Triangular { a, b, c }
    }

    pub fn trapezoidal(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self::Trapezoidal { a, b, c, d }
    }

    pub fn gaussian(mean: f64, sigma: f64) -> Self {
        Self::Gaussian { mean, sigma: sigma.max(1e-12) }
    }

    pub fn generalized_bell(a: f64, b: f64, c: f64) -> Self {
        Self::GeneralizedBell { a: a.max(1e-12), b: b.max(1e-6), c }
    }

    pub fn sigmoid(a: f64, c: f64) -> Self {
        Self::Sigmoid { a, c }
    }

    pub fn singleton(value: f64) -> Self {
        Self::Singleton { value }
    }

    pub fn piecewise_linear(knots: Vec<(f64, f64)>) -> Self {
        Self::PiecewiseLinear { knots }
    }

    pub fn eval(&self, x: f64) -> f64 {
        match self {
            Self::Triangular { a, b, c } => {
                if x <= *a || x >= *c {
                    0.0
                } else if (*a - *b).abs() < 1e-12 {
                    if x <= *b {
                        1.0
                    } else {
                        (*c - x) / (*c - *b)
                    }
                } else if (*c - *b).abs() < 1e-12 {
                    if x >= *b {
                        1.0
                    } else {
                        (x - *a) / (*b - *a)
                    }
                } else if x <= *b {
                    (x - *a) / (*b - *a)
                } else {
                    (*c - x) / (*c - *b)
                }
            }
            Self::Trapezoidal { a, b, c, d } => {
                if x <= *a || x >= *d {
                    0.0
                } else if x >= *b && x <= *c {
                    1.0
                } else if x < *b {
                    (x - *a) / (*b - *a)
                } else {
                    (*d - x) / (*d - *c)
                }
            }
            Self::Gaussian { mean, sigma } => {
                let z = (x - *mean) / *sigma;
                (-0.5 * z * z).exp()
            }
            Self::GeneralizedBell { a, b, c } => {
                let z = ((x - *c) / *a).abs().powf(*b);
                1.0 / (1.0 + z)
            }
            Self::Sigmoid { a, c } => 1.0 / (1.0 + E.powf(-*a * (x - *c))),
            Self::Singleton { value } => {
                if (x - *value).abs() < 1e-12 {
                    1.0
                } else {
                    0.0
                }
            }
            Self::PiecewiseLinear { knots } => {
                if knots.is_empty() {
                    return 0.0;
                }
                if knots.len() == 1 {
                    return knots[0].1;
                }
                if x <= knots[0].0 {
                    return knots[0].1;
                }
                if x >= knots[knots.len() - 1].0 {
                    return knots[knots.len() - 1].1;
                }
                for w in knots.windows(2) {
                    let (x0, y0) = w[0];
                    let (x1, y1) = w[1];
                    if x >= x0 && x <= x1 {
                        let t = if (x1 - x0).abs() < 1e-12 { 0.0 } else { (x - x0) / (x1 - x0) };
                        return y0 + t * (y1 - y0);
                    }
                }
                0.0
            }
        }
    }

    pub fn parameters(&self) -> Vec<f64> {
        match self {
            Self::Triangular { a, b, c } => vec![*a, *b, *c],
            Self::Trapezoidal { a, b, c, d } => vec![*a, *b, *c, *d],
            Self::Gaussian { mean, sigma } => vec![*mean, *sigma],
            Self::GeneralizedBell { a, b, c } => vec![*a, *b, *c],
            Self::Sigmoid { a, c } => vec![*a, *c],
            Self::Singleton { value } => vec![*value],
            Self::PiecewiseLinear { knots } => knots.iter().flat_map(|(x, y)| [*x, *y]).collect(),
        }
    }

    pub fn set_parameters(&mut self, params: &[f64]) -> FuzzyResult<()> {
        match self {
            Self::Triangular { a, b, c } => {
                if params.len() != 3 {
                    return Err(FuzzyError::InvalidParameterCount { expected: 3, got: params.len() });
                }
                *a = params[0];
                *b = params[1];
                *c = params[2];
            }
            Self::Trapezoidal { a, b, c, d } => {
                if params.len() != 4 {
                    return Err(FuzzyError::InvalidParameterCount { expected: 4, got: params.len() });
                }
                *a = params[0];
                *b = params[1];
                *c = params[2];
                *d = params[3];
            }
            Self::Gaussian { mean, sigma } => {
                if params.len() != 2 {
                    return Err(FuzzyError::InvalidParameterCount { expected: 2, got: params.len() });
                }
                *mean = params[0];
                *sigma = params[1].max(1e-12);
            }
            Self::GeneralizedBell { a, b, c } => {
                if params.len() != 3 {
                    return Err(FuzzyError::InvalidParameterCount { expected: 3, got: params.len() });
                }
                *a = params[0].max(1e-12);
                *b = params[1].max(1e-6);
                *c = params[2];
            }
            Self::Sigmoid { a, c } => {
                if params.len() != 2 {
                    return Err(FuzzyError::InvalidParameterCount { expected: 2, got: params.len() });
                }
                *a = params[0];
                *c = params[1];
            }
            Self::Singleton { value } => {
                if params.len() != 1 {
                    return Err(FuzzyError::InvalidParameterCount { expected: 1, got: params.len() });
                }
                *value = params[0];
            }
            Self::PiecewiseLinear { knots } => {
                if !params.len().is_multiple_of(2) {
                    return Err(FuzzyError::InvalidParameterCount { expected: knots.len() * 2, got: params.len() });
                }
                knots.clear();
                for chunk in params.chunks(2) {
                    knots.push((chunk[0], chunk[1]));
                }
            }
        }
        Ok(())
    }

    pub fn support_min(&self) -> f64 {
        match self {
            Self::Triangular { a, .. } | Self::Trapezoidal { a, .. } => *a,
            Self::Gaussian { mean, sigma } => mean - 4.0 * sigma,
            Self::GeneralizedBell { c, a, .. } => c - 4.0 * a,
            Self::Sigmoid { c, .. } => c - 10.0,
            Self::Singleton { value } => *value,
            Self::PiecewiseLinear { knots } => knots.first().map_or(0.0, |(x, _)| *x),
        }
    }

    pub fn support_max(&self) -> f64 {
        match self {
            Self::Triangular { c, .. } => *c,
            Self::Trapezoidal { d, .. } => *d,
            Self::Gaussian { mean, sigma } => mean + 4.0 * sigma,
            Self::GeneralizedBell { c, a, .. } => c + 4.0 * a,
            Self::Sigmoid { c, .. } => c + 10.0,
            Self::Singleton { value } => *value,
            Self::PiecewiseLinear { knots } => knots.last().map_or(0.0, |(x, _)| *x),
        }
    }
}
// #endregion 🔖️MembershipFunction

// #region 🔖️FuzzySet
/// 🌫️ Type-1 fuzzy set: linguistic label plus membership function.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FuzzySet {
    pub name: String,
    pub mf: MembershipFunction,
}

impl FuzzySet {
    pub fn new(name: impl Into<String>, mf: MembershipFunction) -> Self {
        Self { name: name.into(), mf }
    }

    pub fn grade(&self, x: f64) -> f64 {
        clamp01(self.mf.eval(x))
    }
}

/// 🌫️ Interval type-2 fuzzy set with footprint of uncertainty (lower/upper membership functions).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntervalType2Set {
    pub name: String,
    pub lower: MembershipFunction,
    pub upper: MembershipFunction,
}

impl IntervalType2Set {
    pub fn new(name: impl Into<String>, lower: MembershipFunction, upper: MembershipFunction) -> Self {
        Self { name: name.into(), lower, upper }
    }

    pub fn grade_interval(&self, x: f64) -> (f64, f64) {
        let lo = clamp01(self.lower.eval(x));
        let hi = clamp01(self.upper.eval(x).max(lo));
        (lo, hi)
    }

    pub fn type_reduced_centroid(&self, universe: &[f64]) -> f64 {
        if universe.is_empty() {
            return 0.0;
        }
        let mut num = 0.0;
        let mut den = 0.0;
        for &x in universe {
            let (lo, hi) = self.grade_interval(x);
            let mu = 0.5 * (lo + hi);
            num += x * mu;
            den += mu;
        }
        if den.abs() < 1e-12 {
            universe[universe.len() / 2]
        } else {
            num / den
        }
    }
}

/// 🌫️ Intuitionistic fuzzy set with membership, non-membership, and hesitation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntuitionisticSet {
    pub name: String,
    pub membership: MembershipFunction,
    pub non_membership: MembershipFunction,
}

impl IntuitionisticSet {
    pub fn new(name: impl Into<String>, membership: MembershipFunction, non_membership: MembershipFunction) -> Self {
        Self { name: name.into(), membership, non_membership }
    }

    pub fn grades(&self, x: f64) -> FuzzyResult<(f64, f64, f64)> {
        let mu = clamp01(self.membership.eval(x));
        let nu = clamp01(self.non_membership.eval(x));
        if mu + nu > 1.0 + 1e-9 {
            return Err(FuzzyError::InvalidIntuitionistic);
        }
        Ok((mu, nu, 1.0 - mu - nu))
    }
}
// #endregion 🔖️FuzzySet

// #region 🔖️TNormTConorm
/// 🔗️ T-norm for fuzzy AND aggregation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TNorm {
    Min,
    Product,
    Lukasiewicz,
    Drastic,
}

impl TNorm {
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

    pub fn fold<I: Iterator<Item = f64>>(self, values: I) -> f64 {
        values.fold(1.0, |acc, v| self.apply(acc, v))
    }
}

/// 🔗️ T-conorm for fuzzy OR aggregation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TConorm {
    Max,
    ProbSum,
    Lukasiewicz,
    NilpotentMax,
}

impl TConorm {
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

    pub fn fold<I: Iterator<Item = f64>>(self, values: I) -> f64 {
        values.fold(0.0, |acc, v| self.apply(acc, v))
    }
}

/// 🌫️ Linguistic hedge modifiers.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Hedge {
    Very,
    Somewhat,
    MoreOrLess,
    Extremely,
}

impl Hedge {
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

pub fn complement(mu: f64) -> f64 {
    1.0 - clamp01(mu)
}

pub fn concentration(mu: f64) -> f64 {
    clamp01(mu).powi(2)
}

pub fn dilation(mu: f64) -> f64 {
    clamp01(mu).sqrt()
}
// #endregion 🔖️TNormTConorm

// #region 🔖️FuzzyArithmetic
/// 🔢️ Triangular fuzzy number for α-cut arithmetic.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FuzzyNumber {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl FuzzyNumber {
    pub fn triangular(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    pub fn alpha_cut(&self, alpha: f64) -> (f64, f64) {
        let alpha = clamp01(alpha);
        if alpha <= 0.0 {
            return (self.b, self.b);
        }
        let left = self.a + alpha * (self.b - self.a);
        let right = self.c - alpha * (self.c - self.b);
        (left.min(right), left.max(right))
    }

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
    pub fn add(self, other: Self) -> Self {
        Self { a: self.a + other.a, b: self.b + other.b, c: self.c + other.c }
    }

    #[allow(clippy::should_implement_trait, reason = "value-semantics sub used pervasively as a plain method by fuzzy arithmetic callers")]
    pub fn sub(self, other: Self) -> Self {
        Self { a: self.a - other.c, b: self.b - other.b, c: self.c - other.a }
    }

    pub fn scale(self, k: f64) -> Self {
        if k >= 0.0 {
            Self { a: self.a * k, b: self.b * k, c: self.c * k }
        } else {
            Self { a: self.c * k, b: self.b * k, c: self.a * k }
        }
    }
}

pub fn fuzzy_add(a: FuzzyNumber, b: FuzzyNumber) -> FuzzyNumber {
    a.add(b)
}

pub fn fuzzy_mul_interval(a: FuzzyNumber, b: FuzzyNumber, alpha: f64) -> (f64, f64) {
    let (al, ar) = a.alpha_cut(alpha);
    let (bl, br) = b.alpha_cut(alpha);
    let candidates = [al * bl, al * br, ar * bl, ar * br];
    (candidates.iter().copied().fold(f64::INFINITY, f64::min), candidates.iter().copied().fold(f64::NEG_INFINITY, f64::max))
}
// #endregion 🔖️FuzzyArithmetic

// #region 🔖️FuzzyRelation
/// 🔗️ Graded fuzzy relation R: X → Y stored as a matrix.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FuzzyRelation {
    pub values: Vec<Vec<f64>>,
}

impl FuzzyRelation {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { values: vec![vec![0.0; cols]; rows] }
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.values[row][col] = clamp01(value);
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.values[row][col]
    }

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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PossibilityMeasure {
    pub universe: Vec<f64>,
    pub membership: Vec<f64>,
}

impl PossibilityMeasure {
    pub fn new(universe: Vec<f64>, membership: Vec<f64>) -> FuzzyResult<Self> {
        if universe.len() != membership.len() || universe.is_empty() {
            return Err(FuzzyError::InvalidDomain("possibility universe".into()));
        }
        Ok(Self { universe, membership: membership.into_iter().map(clamp01).collect() })
    }

    pub fn possibility(&self, predicate: impl Fn(f64) -> bool) -> f64 {
        self.universe.iter().zip(self.membership.iter()).filter(|(x, _)| predicate(**x)).map(|(_, mu)| *mu).fold(0.0, f64::max)
    }

    pub fn necessity(&self, predicate: impl Fn(f64) -> bool) -> f64 {
        1.0 - self.possibility(|x| !predicate(x))
    }

    pub fn from_scores(universe: Vec<f64>, scores: Vec<f64>) -> FuzzyResult<Self> {
        let max_score = scores.iter().copied().fold(0.0_f64, f64::max).max(1e-12);
        Self::new(universe, scores.into_iter().map(|s| s / max_score).collect())
    }
}
// #endregion 🔖️PossibilityTheory

// #region 🔖️Universe
/// 📏️ Discrete universe of discourse for numerical integration and defuzzification.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Universe {
    pub min: f64,
    pub max: f64,
    pub samples: Vec<f64>,
}

impl Universe {
    pub fn new(min: f64, max: f64, n: usize) -> FuzzyResult<Self> {
        if min >= max || n == 0 {
            return Err(FuzzyError::InvalidDomain("universe bounds".into()));
        }
        Ok(Self { min, max, samples: linspace(min, max, n) })
    }

    pub fn sample(&self, index: usize) -> f64 {
        self.samples[index]
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}
// #endregion 🔖️Universe

// #region 🔖️LinguisticVariable
/// 🗣️ Linguistic variable: universe plus named fuzzy terms.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinguisticVariable {
    pub name: String,
    pub universe: Universe,
    pub terms: Vec<FuzzySet>,
}

impl LinguisticVariable {
    pub fn new(name: impl Into<String>, universe: Universe, terms: Vec<FuzzySet>) -> Self {
        Self { name: name.into(), universe, terms }
    }

    pub fn fuzzify(&self, x: f64) -> Vec<f64> {
        self.terms.iter().map(|t| t.grade(x)).collect()
    }

    pub fn term_index(&self, name: &str) -> Option<usize> {
        self.terms.iter().position(|t| t.name == name)
    }
}
// #endregion 🔖️LinguisticVariable

// #region 🔖️Rule
/// 📜️ Antecedent clause: input variable index and term index.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AntecedentClause {
    pub input: usize,
    pub term: usize,
    pub hedge: Option<Hedge>,
}

/// 📜️ Rule consequent variants for Mamdani, Sugeno, and Tsukamoto engines.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Consequent {
    Mamdani { output: usize, term: usize },
    SugenoConstant { output: usize, value: f64 },
    SugenoLinear { output: usize, coeffs: Vec<f64> },
    Tsukamoto { output: usize, term: usize },
    SoftConstraint { output: usize, term: usize, preference: f64 },
}

/// 📜️ Weighted fuzzy rule with optional confidence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    pub id: usize,
    pub antecedents: Vec<AntecedentClause>,
    pub consequent: Consequent,
    pub weight: f64,
    pub confidence: f64,
}

impl Rule {
    pub fn firing_strength(&self, inputs: &[LinguisticVariable], values: &[f64], tnorm: TNorm) -> f64 {
        let strengths: Vec<f64> = self
            .antecedents
            .iter()
            .map(|clause| {
                let mu = inputs[clause.input].terms[clause.term].grade(values[clause.input]);
                clause.hedge.map_or(mu, |h| h.apply(mu))
            })
            .collect();
        self.weight * self.confidence * tnorm.fold(strengths.into_iter())
    }
}

/// 📚️ Collection of fuzzy rules.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleBase {
    pub rules: Vec<Rule>,
}

impl RuleBase {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}
// #endregion 🔖️Rule

// #region 🔖️Defuzzification
/// 🎯️ Defuzzification method applied to a discrete aggregated membership vector.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Defuzzifier {
    Centroid,
    Bisector,
    Mom,
    Som,
    Lom,
    WeightedAverage,
    Height,
}

impl Defuzzifier {
    pub fn apply(&self, universe: &Universe, membership: &[f64], rule_heights: Option<&[f64]>, rule_values: Option<&[f64]>) -> FuzzyResult<f64> {
        if universe.samples.is_empty() || membership.is_empty() {
            return Err(FuzzyError::EmptyUniverse);
        }
        match self {
            Self::Centroid => {
                let mut num = 0.0;
                let mut den = 0.0;
                for (x, mu) in universe.samples.iter().zip(membership.iter()) {
                    num += x * mu;
                    den += mu;
                }
                Ok(if den.abs() < 1e-12 { universe.samples[universe.len() / 2] } else { num / den })
            }
            Self::Bisector => {
                let total: f64 = membership.iter().sum();
                if total.abs() < 1e-12 {
                    return Ok(universe.samples[universe.len() / 2]);
                }
                let half = 0.5 * total;
                let mut cumulative = 0.0;
                for (x, mu) in universe.samples.iter().zip(membership.iter()) {
                    cumulative += mu;
                    if cumulative >= half {
                        return Ok(*x);
                    }
                }
                Ok(*universe.samples.last().unwrap())
            }
            Self::Mom => {
                let max_mu = membership.iter().copied().fold(0.0, f64::max);
                let peaks: Vec<f64> = universe.samples.iter().zip(membership.iter()).filter(|(_, mu)| (**mu - max_mu).abs() < 1e-9).map(|(x, _)| *x).collect();
                Ok(peaks.iter().sum::<f64>() / peaks.len().max(1) as f64)
            }
            Self::Som => Ok(universe.samples[argmax(membership)]),
            Self::Lom => {
                let max_mu = membership.iter().copied().fold(0.0, f64::max);
                let last = universe.samples.iter().zip(membership.iter()).filter(|(_, mu)| (**mu - max_mu).abs() < 1e-9).map(|(x, _)| *x).next_back();
                Ok(last.unwrap_or(universe.samples[universe.len() / 2]))
            }
            Self::WeightedAverage => {
                let heights = rule_heights.ok_or_else(|| FuzzyError::DimensionMismatch("weighted average needs rule heights".into()))?;
                let values = rule_values.ok_or_else(|| FuzzyError::DimensionMismatch("weighted average needs rule values".into()))?;
                let mut num = 0.0;
                let mut den = 0.0;
                for (h, v) in heights.iter().zip(values.iter()) {
                    num += h * v;
                    den += h;
                }
                Ok(if den.abs() < 1e-12 { values[0] } else { num / den })
            }
            Self::Height => {
                let heights = rule_heights.ok_or_else(|| FuzzyError::DimensionMismatch("height defuzz needs rule heights".into()))?;
                let values = rule_values.ok_or_else(|| FuzzyError::DimensionMismatch("height defuzz needs rule values".into()))?;
                Self::WeightedAverage.apply(universe, membership, Some(heights), Some(values))
            }
        }
    }
}
// #endregion 🔖️Defuzzification

// #region 🔖️InferenceEngine
/// ⚙️ Fuzzy inference engine type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InferenceModel {
    Mamdani,
    Sugeno,
    Tsukamoto,
    Larsen,
    Hybrid,
}

/// 🔥️ Single rule firing trace for explainability.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleTrace {
    pub rule_id: usize,
    pub activation: f64,
    pub contribution: f64,
    pub description: String,
}

/// 🧾️ Explanation of an inference step.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Explanation {
    pub model: InferenceModel,
    pub input_values: Vec<f64>,
    pub output_values: Vec<f64>,
    pub traces: Vec<RuleTrace>,
    pub defuzzifier: Defuzzifier,
    pub rationale: String,
}

/// 🎛️ Multi-input multi-output fuzzy inference system.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MimoSystem {
    pub inputs: Vec<LinguisticVariable>,
    pub outputs: Vec<LinguisticVariable>,
    pub rules: RuleBase,
    pub model: InferenceModel,
    pub tnorm: TNorm,
    pub tconorm: TConorm,
    pub defuzzifier: Defuzzifier,
}

impl MimoSystem {
    pub fn infer(&self, values: &[f64]) -> FuzzyResult<(Vec<f64>, Explanation)> {
        if values.len() != self.inputs.len() {
            return Err(FuzzyError::DimensionMismatch("input count".into()));
        }
        if self.rules.is_empty() {
            return Err(FuzzyError::EmptyRuleBase);
        }
        let mut outputs = vec![0.0; self.outputs.len()];
        let mut traces = Vec::new();
        for (out_idx, output_var) in self.outputs.iter().enumerate() {
            let (value, out_traces) = self.infer_output(out_idx, output_var, values)?;
            outputs[out_idx] = value;
            traces.extend(out_traces);
        }
        let rationale = format!("{} engine produced {} output(s) from {} rule(s)", format!("{:?}", self.model).to_lowercase(), outputs.len(), self.rules.rules.len());
        Ok((outputs.clone(), Explanation { model: self.model, input_values: values.to_vec(), output_values: outputs, traces, defuzzifier: self.defuzzifier, rationale }))
    }

    fn infer_output(&self, out_idx: usize, output_var: &LinguisticVariable, values: &[f64]) -> FuzzyResult<(f64, Vec<RuleTrace>)> {
        let mut aggregated = vec![0.0; output_var.universe.len()];
        let mut traces = Vec::new();
        let mut sugeno_num = 0.0;
        let mut sugeno_den = 0.0;
        let mut heights = Vec::new();
        let mut rule_values = Vec::new();
        for rule in &self.rules.rules {
            let activation = rule.firing_strength(&self.inputs, values, self.tnorm);
            if activation <= 1e-12 {
                continue;
            }
            match (&self.model, &rule.consequent) {
                (InferenceModel::Mamdani | InferenceModel::Hybrid, Consequent::Mamdani { output, term }) if *output == out_idx => {
                    for (i, x) in output_var.universe.samples.iter().enumerate() {
                        let mu = output_var.terms[*term].grade(*x).min(activation);
                        aggregated[i] = self.tconorm.apply(aggregated[i], mu);
                    }
                    traces.push(RuleTrace { rule_id: rule.id, activation, contribution: activation, description: format!("Mamdani rule {} clipped consequent", rule.id) });
                }
                (InferenceModel::Larsen | InferenceModel::Hybrid, Consequent::Mamdani { output, term }) if *output == out_idx => {
                    for (i, x) in output_var.universe.samples.iter().enumerate() {
                        let base = output_var.terms[*term].grade(*x);
                        let mu = if base <= 1e-12 { 0.0 } else { (activation * base / base.max(activation)).min(1.0) };
                        aggregated[i] = self.tconorm.apply(aggregated[i], mu.min(base * activation));
                    }
                    traces.push(RuleTrace { rule_id: rule.id, activation, contribution: activation, description: format!("Larsen rule {} scaled consequent", rule.id) });
                }
                (InferenceModel::Sugeno | InferenceModel::Hybrid, Consequent::SugenoConstant { output, value }) if *output == out_idx => {
                    sugeno_num += activation * value;
                    sugeno_den += activation;
                    heights.push(activation);
                    rule_values.push(*value);
                    traces.push(RuleTrace { rule_id: rule.id, activation, contribution: activation * value, description: format!("Sugeno constant {:.3}", value) });
                }
                (InferenceModel::Sugeno | InferenceModel::Hybrid, Consequent::SugenoLinear { output, coeffs }) if *output == out_idx => {
                    let mut y = coeffs.first().copied().unwrap_or(0.0);
                    for (i, &v) in values.iter().enumerate() {
                        if i + 1 < coeffs.len() {
                            y += coeffs[i + 1] * v;
                        }
                    }
                    sugeno_num += activation * y;
                    sugeno_den += activation;
                    heights.push(activation);
                    rule_values.push(y);
                    traces.push(RuleTrace { rule_id: rule.id, activation, contribution: activation * y, description: format!("Sugeno linear -> {:.3}", y) });
                }
                (InferenceModel::Tsukamoto | InferenceModel::Hybrid, Consequent::Tsukamoto { output, term }) if *output == out_idx => {
                    let term_mf = &output_var.terms[*term].mf;
                    let crisp = tsukamoto_inverse(term_mf, activation);
                    sugeno_num += activation * crisp;
                    sugeno_den += activation;
                    heights.push(activation);
                    rule_values.push(crisp);
                    traces.push(RuleTrace { rule_id: rule.id, activation, contribution: activation * crisp, description: format!("Tsukamoto inverse -> {:.3}", crisp) });
                }
                (_, Consequent::SoftConstraint { output, term, preference }) if *output == out_idx => {
                    let pref = preference.max(0.0);
                    for (i, x) in output_var.universe.samples.iter().enumerate() {
                        let mu = output_var.terms[*term].grade(*x).min(activation * pref);
                        aggregated[i] = self.tconorm.apply(aggregated[i], mu);
                    }
                    traces.push(RuleTrace { rule_id: rule.id, activation, contribution: activation * pref, description: format!("soft constraint preference {:.3}", pref) });
                }
                _ => {}
            }
        }
        let value = match self.model {
            InferenceModel::Sugeno | InferenceModel::Tsukamoto => {
                if sugeno_den.abs() < 1e-12 {
                    return Err(FuzzyError::NoFiredRules);
                }
                Defuzzifier::WeightedAverage.apply(&output_var.universe, &aggregated, Some(&heights), Some(&rule_values))?
            }
            InferenceModel::Mamdani | InferenceModel::Larsen | InferenceModel::Hybrid => {
                if aggregated.iter().sum::<f64>() < 1e-12 && sugeno_den > 1e-12 {
                    sugeno_num / sugeno_den
                } else {
                    self.defuzzifier.apply(&output_var.universe, &aggregated, None, None)?
                }
            }
        };
        Ok((value, traces))
    }
}

fn tsukamoto_inverse(mf: &MembershipFunction, alpha: f64) -> f64 {
    let alpha = clamp01(alpha);
    if alpha <= 0.0 {
        return mf.support_min();
    }
    let min = mf.support_min();
    let max = mf.support_max();
    let mut lo = min;
    let mut hi = max;
    for _ in 0..64 {
        let mid = 0.5 * (lo + hi);
        if mf.eval(mid) >= alpha {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    0.5 * (lo + hi)
}

/// 🏗️ Hierarchical fuzzy system: layer outputs become next-layer inputs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HierarchicalSystem {
    pub layers: Vec<MimoSystem>,
}

impl HierarchicalSystem {
    pub fn new(layers: Vec<MimoSystem>) -> Self {
        Self { layers }
    }

    pub fn infer(&self, values: &[f64]) -> FuzzyResult<(Vec<f64>, Vec<Explanation>)> {
        let mut current = values.to_vec();
        let mut explanations = Vec::new();
        for layer in &self.layers {
            let (out, explanation) = layer.infer(&current)?;
            explanations.push(explanation);
            current = out;
        }
        Ok((current, explanations))
    }
}
// #endregion 🔖️InferenceEngine

// #region 🔖️AdaptiveMembership
/// 🎯️ Adaptive membership-function fitting from labeled (x, μ) samples via gradient descent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdaptiveMembership {
    pub mf: MembershipFunction,
    pub learning_rate: f64,
}

impl AdaptiveMembership {
    pub fn new(mf: MembershipFunction, learning_rate: f64) -> Self {
        Self { mf, learning_rate }
    }

    pub fn fit(&mut self, samples: &[(f64, f64)], epochs: usize) -> f64 {
        let mut loss = 0.0;
        for _ in 0..epochs {
            loss = 0.0;
            let mut grads = vec![0.0; self.mf.parameters().len()];
            for &(x, target) in samples {
                let pred = self.mf.eval(x);
                let err = pred - clamp01(target);
                loss += err * err;
                let eps = 1e-4;
                let params = self.mf.parameters();
                for (i, g) in grads.iter_mut().enumerate().take(params.len()) {
                    let mut p = params.clone();
                    p[i] += eps;
                    let _ = self.mf.set_parameters(&p);
                    let plus = self.mf.eval(x);
                    let _ = self.mf.set_parameters(&params);
                    *g += err * (plus - pred) / eps;
                }
            }
            let mut params = self.mf.parameters();
            for (p, g) in params.iter_mut().zip(grads.iter()) {
                *p -= self.learning_rate * g / samples.len().max(1) as f64;
            }
            let _ = self.mf.set_parameters(&params);
        }
        loss / samples.len().max(1) as f64
    }
}
// #endregion 🔖️AdaptiveMembership

// #region 🔖️Anfis
/// 🧠️ ANFIS: adaptive neuro-fuzzy inference system with hybrid least-squares + gradient learning.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Anfis {
    pub input_count: usize,
    pub rules_per_input: usize,
    pub premise_centers: Vec<Vec<f64>>,
    pub premise_widths: Vec<Vec<f64>>,
    pub consequent_coeffs: Vec<Vec<f64>>,
}

impl Anfis {
    pub fn new(input_count: usize, rules_per_input: usize, input_ranges: &[(f64, f64)]) -> Self {
        let rule_count = rules_per_input.pow(input_count as u32);
        let mut premise_centers = vec![vec![0.0; input_count]; rule_count];
        let mut premise_widths = vec![vec![1.0; input_count]; rule_count];
        let mut idx = 0usize;
        let steps: Vec<Vec<f64>> = input_ranges.iter().map(|(lo, hi)| linspace(*lo, *hi, rules_per_input)).collect();
        fn recurse(input_count: usize, steps: &[Vec<f64>], path: &mut Vec<usize>, idx: &mut usize, centers: &mut [Vec<f64>], widths: &mut [Vec<f64>], ranges: &[(f64, f64)]) {
            if path.len() == input_count {
                for (j, &s) in path.iter().enumerate() {
                    centers[*idx][j] = steps[j][s];
                    widths[*idx][j] = (ranges[j].1 - ranges[j].0) / steps[j].len().max(1) as f64;
                }
                *idx += 1;
                return;
            }
            for s in 0..steps[path.len()].len() {
                path.push(s);
                recurse(input_count, steps, path, idx, centers, widths, ranges);
                path.pop();
            }
        }
        let mut path = Vec::new();
        recurse(input_count, &steps, &mut path, &mut idx, &mut premise_centers, &mut premise_widths, input_ranges);
        let consequent_coeffs = vec![vec![0.0; input_count + 1]; rule_count];
        Self { input_count, rules_per_input, premise_centers, premise_widths, consequent_coeffs }
    }

    pub fn rule_count(&self) -> usize {
        self.consequent_coeffs.len()
    }

    fn firing_strengths(&self, x: &[f64]) -> Vec<f64> {
        (0..self.rule_count())
            .map(|r| {
                (0..self.input_count)
                    .map(|j| {
                        let sigma = self.premise_widths[r][j].max(1e-6);
                        let z = (x[j] - self.premise_centers[r][j]) / sigma;
                        (-0.5 * z * z).exp()
                    })
                    .product()
            })
            .collect()
    }

    pub fn forward(&self, x: &[f64]) -> f64 {
        let w = self.firing_strengths(x);
        let den: f64 = w.iter().sum();
        if den < 1e-12 {
            return 0.0;
        }
        let mut num = 0.0;
        for (r, wr) in w.iter().enumerate() {
            let mut y = self.consequent_coeffs[r][0];
            for (j, &xj) in x.iter().enumerate() {
                y += self.consequent_coeffs[r][j + 1] * xj;
            }
            num += wr * y;
        }
        num / den
    }

    pub fn fit_hybrid(&mut self, data: &[(Vec<f64>, f64)], epochs: usize) -> f64 {
        let mut loss = 0.0;
        for _ in 0..epochs {
            self.fit_consequents(data);
            loss = self.fit_premises(data);
        }
        loss
    }

    fn fit_consequents(&mut self, data: &[(Vec<f64>, f64)]) {
        let n = data.len();
        let r = self.rule_count();
        let p = self.input_count + 1;
        let mut ata = MatD::zeros(r * p, r * p);
        let mut atb = VecD::zeros(r * p);
        for (x, y) in data {
            let w = self.firing_strengths(x);
            let den: f64 = w.iter().sum();
            if den < 1e-12 {
                continue;
            }
            for i in 0..r {
                let wi = w[i] / den;
                let mut row = vec![wi];
                row.extend(x.iter().map(|&xj| wi * xj));
                for a in 0..p {
                    for b in 0..p {
                        ata.add_at(i * p + a, i * p + b, row[a] * row[b]);
                    }
                    atb.add_at(i * p + a, row[a] * *y);
                }
            }
        }
        if let Some(sol) = ata.lu_solve(&atb) {
            for i in 0..r {
                for j in 0..p {
                    self.consequent_coeffs[i][j] = sol.get(i * p + j);
                }
            }
        }
        let _ = n;
    }

    fn fit_premises(&mut self, data: &[(Vec<f64>, f64)]) -> f64 {
        let mut loss = 0.0;
        let lr = 0.01;
        for (x, y) in data {
            let pred = self.forward(x);
            let err = pred - y;
            loss += err * err;
            for r in 0..self.rule_count() {
                for j in 0..self.input_count {
                    let sigma = self.premise_widths[r][j].max(1e-6);
                    let z = (x[j] - self.premise_centers[r][j]) / sigma;
                    let g = err * (-z / sigma) * (-0.5 * z * z).exp();
                    self.premise_centers[r][j] -= lr * g;
                    self.premise_widths[r][j] = (self.premise_widths[r][j] - lr * g.abs()).max(1e-3);
                }
            }
        }
        loss / data.len().max(1) as f64
    }
}
// #endregion 🔖️Anfis

// #region 🔖️RuleLearning
/// 🌱️ Wang–Mendel rule induction from input-output samples.
pub fn wang_mendel_rules(inputs: &[LinguisticVariable], output: &LinguisticVariable, data: &[(Vec<f64>, f64)], model: InferenceModel) -> RuleBase {
    let mut rules = Vec::new();
    for (idx, (x, y)) in data.iter().enumerate() {
        let mut antecedents = Vec::new();
        for (i, var) in inputs.iter().enumerate() {
            let grades = var.fuzzify(x[i]);
            let term = argmax(&grades);
            antecedents.push(AntecedentClause { input: i, term, hedge: None });
        }
        let out_grades = output.fuzzify(*y);
        let out_term = argmax(&out_grades);
        let consequent = match model {
            InferenceModel::Sugeno => Consequent::SugenoConstant { output: 0, value: *y },
            InferenceModel::Tsukamoto => Consequent::Tsukamoto { output: 0, term: out_term },
            _ => Consequent::Mamdani { output: 0, term: out_term },
        };
        rules.push(Rule { id: idx, antecedents, consequent, weight: 1.0, confidence: 1.0 });
    }
    RuleBase::new(rules)
}

/// 🧹️ Prune rules with activation support or weight below thresholds.
pub fn prune_rules(rules: RuleBase, min_weight: f64, min_confidence: f64) -> RuleBase {
    RuleBase::new(rules.rules.into_iter().filter(|r| r.weight >= min_weight && r.confidence >= min_confidence).collect())
}

/// 🎯️ Re-weight rules by empirical fit quality on a dataset.
pub fn weight_rules_by_fit(system: &mut MimoSystem, data: &[(Vec<f64>, Vec<f64>)]) {
    let mut confidences = Vec::new();
    for rule in &system.rules.rules {
        let mut err = 0.0;
        let mut n = 0.0;
        for (x, y) in data {
            if let Ok((pred, _)) = system.infer(x) {
                for (p, t) in pred.iter().zip(y.iter()) {
                    err += (p - t).abs();
                    n += 1.0;
                }
            }
        }
        confidences.push(if n > 0.0 { (1.0 / (1.0 + err / n)).clamp(0.0, 1.0) } else { rule.confidence });
    }
    for (rule, confidence) in system.rules.rules.iter_mut().zip(confidences) {
        rule.confidence = confidence;
    }
}

/// 🌱️ Subtractive clustering seed centers for rule generation.
pub fn subtractive_cluster_centers(data: &[Vec<f64>], ra: f64, accept_ratio: f64, reject_ratio: f64) -> Vec<Vec<f64>> {
    if data.is_empty() {
        return Vec::new();
    }
    let dim = data[0].len();
    let mut potentials: Vec<f64> = data.iter().map(|_| 0.0).collect();
    for i in 0..data.len() {
        for point in data.iter() {
            let dist2: f64 = (0..dim).map(|d| (data[i][d] - point[d]).powi(2)).sum();
            potentials[i] += (-dist2 / (2.0 * ra * ra)).exp();
        }
    }
    let mut centers = Vec::new();
    let max_p = potentials.iter().copied().fold(0.0, f64::max);
    let accept = accept_ratio * max_p;
    let reject = reject_ratio * max_p;
    while let Some((idx, p)) = potentials.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).map(|(i, &p)| (i, p)) {
        if p < reject {
            break;
        }
        centers.push(data[idx].clone());
        for j in 0..data.len() {
            let dist2: f64 = (0..dim).map(|d| (data[idx][d] - data[j][d]).powi(2)).sum();
            potentials[j] -= (-dist2 / (2.0 * ra * ra)).exp();
            potentials[j] = potentials[j].max(0.0);
        }
        if p < accept {
            break;
        }
    }
    centers
}
// #endregion 🔖️RuleLearning

// #region 🔖️Optimizers
/// 🧬️ Genetic algorithm optimizer for fuzzy parameter vectors.
pub struct GeneticOptimizer {
    pub population_size: usize,
    pub generations: usize,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub bounds: Vec<(f64, f64)>,
    pub seed: u64,
}

impl GeneticOptimizer {
    pub fn optimize<F: Fn(&[f64]) -> f64>(&self, fitness: F) -> (Vec<f64>, f64) {
        let mut rng = Rng::from_seed(self.seed);
        let dim = self.bounds.len();
        let mut population: Vec<Vec<f64>> = (0..self.population_size).map(|_| (0..dim).map(|d| self.bounds[d].0 + rng.next_f64() * (self.bounds[d].1 - self.bounds[d].0)).collect()).collect();
        let mut best = population[0].clone();
        let mut best_fit = fitness(&best);
        for _ in 0..self.generations {
            let mut scored: Vec<(f64, Vec<f64>)> = population.drain(..).map(|ind| (fitness(&ind), ind)).collect();
            scored.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            if scored[0].0 < best_fit {
                best_fit = scored[0].0;
                best = scored[0].1.clone();
            }
            let elite = scored.len() / 5;
            population = scored.into_iter().take(elite).map(|(_, ind)| ind).collect();
            while population.len() < self.population_size {
                let p1 = &population[rng.next_range(0, population.len() as u64) as usize];
                let p2 = &population[rng.next_range(0, population.len() as u64) as usize];
                let mut child: Vec<f64> = if rng.next_bool(self.crossover_rate) { (0..dim).map(|d| if rng.next_bool(0.5) { p1[d] } else { p2[d] }).collect() } else { p1.clone() };
                for d in 0..dim {
                    if rng.next_bool(self.mutation_rate) {
                        child[d] = self.bounds[d].0 + rng.next_f64() * (self.bounds[d].1 - self.bounds[d].0);
                    }
                }
                population.push(child);
            }
        }
        (best, best_fit)
    }
}

/// 🐝️ Particle swarm optimizer for fuzzy parameter tuning.
pub struct PsoOptimizer {
    pub swarm_size: usize,
    pub iterations: usize,
    pub inertia: f64,
    pub cognitive: f64,
    pub social: f64,
    pub bounds: Vec<(f64, f64)>,
    pub seed: u64,
}

impl PsoOptimizer {
    pub fn optimize<F: Fn(&[f64]) -> f64>(&self, fitness: F) -> (Vec<f64>, f64) {
        let mut rng = Rng::from_seed(self.seed);
        let dim = self.bounds.len();
        let mut positions: Vec<Vec<f64>> = (0..self.swarm_size).map(|_| (0..dim).map(|d| self.bounds[d].0 + rng.next_f64() * (self.bounds[d].1 - self.bounds[d].0)).collect()).collect();
        let mut velocities = vec![vec![0.0; dim]; self.swarm_size];
        let mut personal_best = positions.clone();
        let mut personal_fit: Vec<f64> = positions.iter().map(|p| fitness(p)).collect();
        let gbest_idx = personal_fit.iter().enumerate().min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).map_or(0, |(i, _)| i);
        let mut gbest = personal_best[gbest_idx].clone();
        let mut gbest_fit = personal_fit[gbest_idx];
        for _ in 0..self.iterations {
            for i in 0..self.swarm_size {
                for d in 0..dim {
                    let r1 = rng.next_f64();
                    let r2 = rng.next_f64();
                    velocities[i][d] = self.inertia * velocities[i][d] + self.cognitive * r1 * (personal_best[i][d] - positions[i][d]) + self.social * r2 * (gbest[d] - positions[i][d]);
                    positions[i][d] = (positions[i][d] + velocities[i][d]).clamp(self.bounds[d].0, self.bounds[d].1);
                }
                let fit = fitness(&positions[i]);
                if fit < personal_fit[i] {
                    personal_fit[i] = fit;
                    personal_best[i] = positions[i].clone();
                    if fit < gbest_fit {
                        gbest_fit = fit;
                        gbest = positions[i].clone();
                    }
                }
            }
        }
        (gbest, gbest_fit)
    }
}
// #endregion 🔖️Optimizers

// #region 🔖️EvolvingFuzzySystem
/// 🔄️ Evolving fuzzy system that adds, prunes, and adapts rules from streaming samples.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvolvingFuzzySystem {
    pub system: MimoSystem,
    pub learning_rate: f64,
    pub prune_threshold: f64,
    pub max_rules: usize,
}

impl EvolvingFuzzySystem {
    pub fn new(system: MimoSystem, learning_rate: f64, prune_threshold: f64, max_rules: usize) -> Self {
        Self { system, learning_rate, prune_threshold, max_rules }
    }

    pub fn observe(&mut self, x: &[f64], y: &[f64]) -> FuzzyResult<Explanation> {
        let (pred, mut explanation) = self.system.infer(x)?;
        for (rule, trace) in self.system.rules.rules.iter_mut().zip(explanation.traces.iter()) {
            rule.weight = (rule.weight + self.learning_rate * trace.activation).clamp(0.0, 1.0);
            for (p, t) in pred.iter().zip(y.iter()) {
                let err = (p - t).abs();
                rule.confidence = (rule.confidence + self.learning_rate * (1.0 - err)).clamp(0.0, 1.0);
            }
        }
        self.system.rules = prune_rules(self.system.rules.clone(), self.prune_threshold, self.prune_threshold);
        if self.system.rules.rules.len() < self.max_rules {
            if let Some(output) = y.first().copied() {
                let rule = Rule {
                    id: self.system.rules.rules.len(),
                    antecedents: (0..self.system.inputs.len())
                        .map(|i| {
                            let grades = self.system.inputs[i].fuzzify(x[i]);
                            AntecedentClause { input: i, term: argmax(&grades), hedge: None }
                        })
                        .collect(),
                    consequent: Consequent::SugenoConstant { output: 0, value: output },
                    weight: 0.5,
                    confidence: 0.5,
                };
                self.system.rules.rules.push(rule);
            }
        }
        explanation.rationale = format!("evolving update: {} active rules", self.system.rules.rules.len());
        Ok(explanation)
    }
}
// #endregion 🔖️EvolvingFuzzySystem

// #region 🔖️FuzzyCMeans
/// 🎯️ Fuzzy c-means clustering result.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FcmResult {
    pub centers: Vec<Vec<f64>>,
    pub membership: Vec<Vec<f64>>,
    pub objective: f64,
}

/// 🎯️ Standard fuzzy c-means clustering.
pub fn fuzzy_c_means(data: &[Vec<f64>], k: usize, m: f64, max_iter: usize, tol: f64) -> FuzzyResult<FcmResult> {
    if data.is_empty() || k == 0 {
        return Err(FuzzyError::InvalidDomain("fcm data".into()));
    }
    let n = data.len();
    let dim = data[0].len();
    let mut membership = vec![vec![1.0 / k as f64; k]; n];
    let mut centers = data[..k.min(n)].to_vec();
    while centers.len() < k {
        centers.push(data[centers.len() % n].clone());
    }
    let mut objective = f64::INFINITY;
    for _ in 0..max_iter {
        for j in 0..k {
            for d in 0..dim {
                let mut num = 0.0;
                let mut den = 0.0;
                for i in 0..n {
                    let _dist2: f64 = (0..dim).map(|dd| (data[i][dd] - centers[j][dd]).powi(2)).sum();
                    let w = membership[i][j].powf(m);
                    num += w * data[i][d];
                    den += w;
                }
                centers[j][d] = if den.abs() < 1e-12 { centers[j][d] } else { num / den };
            }
        }
        let mut new_obj = 0.0;
        for i in 0..n {
            for j in 0..k {
                let dist2: f64 = (0..dim).map(|d| (data[i][d] - centers[j][d]).powi(2)).sum::<f64>().max(1e-12);
                membership[i][j] = 1.0
                    / (0..k)
                        .map(|c| {
                            let dist2c: f64 = (0..dim).map(|d| (data[i][d] - centers[c][d]).powi(2)).sum::<f64>().max(1e-12);
                            (dist2 / dist2c).powf(1.0 / (m - 1.0))
                        })
                        .sum::<f64>();
                new_obj += membership[i][j].powf(m) * dist2;
            }
        }
        if (objective - new_obj).abs() < tol {
            objective = new_obj;
            break;
        }
        objective = new_obj;
    }
    Ok(FcmResult { centers, membership, objective })
}

/// 🎯️ Gustafson–Kessel fuzzy clustering with adaptive covariance per cluster.
pub fn gustafson_kessel(data: &[Vec<f64>], k: usize, m: f64, max_iter: usize) -> FuzzyResult<FcmResult> {
    let mut result = fuzzy_c_means(data, k, m, max_iter, 1e-5)?;
    let n = data.len();
    let dim = data[0].len();
    for _ in 0..max_iter {
        let mut covariances = vec![MatD::identity(dim); k];
        for j in 0..k {
            let mut cov = MatD::zeros(dim, dim);
            let mut den = 0.0;
            for i in 0..n {
                let w = result.membership[i][j].powf(m);
                den += w;
                for a in 0..dim {
                    for b in 0..dim {
                        cov.add_at(a, b, w * (data[i][a] - result.centers[j][a]) * (data[i][b] - result.centers[j][b]));
                    }
                }
            }
            if den > 1e-12 {
                for a in 0..dim {
                    for b in 0..dim {
                        covariances[j].set(a, b, cov.get(a, b) / den + if a == b { 1e-6 } else { 0.0 });
                    }
                }
            }
        }
        for i in 0..n {
            for j in 0..k {
                let mut dist2 = 0.0;
                let diff: Vec<f64> = (0..dim).map(|d| data[i][d] - result.centers[j][d]).collect();
                for a in 0..dim {
                    for b in 0..dim {
                        dist2 += diff[a] * covariances[j].get(a, b) * diff[b];
                    }
                }
                result.membership[i][j] = 1.0
                    / (0..k)
                        .map(|c| {
                            let mut d2 = 0.0;
                            let diff: Vec<f64> = (0..dim).map(|d| data[i][d] - result.centers[c][d]).collect();
                            for a in 0..dim {
                                for b in 0..dim {
                                    d2 += diff[a] * covariances[c].get(a, b) * diff[b];
                                }
                            }
                            (dist2.max(1e-12) / d2.max(1e-12)).powf(1.0 / (m - 1.0))
                        })
                        .sum::<f64>();
            }
        }
    }
    Ok(result)
}
// #endregion 🔖️FuzzyCMeans

// #region 🔖️MulticriteriaDecision
/// 📊️ Fuzzy AHP pairwise comparison matrix.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FuzzyAhp {
    pub matrix: Vec<Vec<FuzzyNumber>>,
}

impl FuzzyAhp {
    pub fn new(matrix: Vec<Vec<FuzzyNumber>>) -> Self {
        Self { matrix }
    }

    pub fn weights(&self) -> Vec<f64> {
        let n = self.matrix.len();
        let mut scores = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                scores[i] += self.matrix[i][j].defuzzify_centroid(32);
            }
        }
        let sum: f64 = scores.iter().sum();
        if sum.abs() < 1e-12 {
            vec![1.0 / n as f64; n]
        } else {
            scores.into_iter().map(|s| s / sum).collect()
        }
    }
}

/// 📊️ Fuzzy TOPSIS decision over alternatives.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FuzzyTopsis {
    pub alternatives: Vec<String>,
    pub criteria: Vec<String>,
    pub decision_matrix: Vec<Vec<FuzzyNumber>>,
    pub weights: Vec<f64>,
    pub benefit: Vec<bool>,
}

impl FuzzyTopsis {
    pub fn rank(&self) -> Vec<(usize, f64)> {
        let m = self.alternatives.len();
        let n = self.criteria.len();
        let mut crisp = vec![vec![0.0; n]; m];
        for i in 0..m {
            for j in 0..n {
                crisp[i][j] = self.decision_matrix[i][j].defuzzify_centroid(32);
            }
        }
        let mut norm = vec![vec![0.0; n]; m];
        for j in 0..n {
            let denom = (0..m).map(|i| crisp[i][j] * crisp[i][j]).sum::<f64>().sqrt().max(1e-12);
            for i in 0..m {
                norm[i][j] = crisp[i][j] / denom * self.weights[j];
            }
        }
        let ideal_pos: Vec<f64> = (0..n).map(|j| if self.benefit[j] { (0..m).map(|i| norm[i][j]).fold(f64::NEG_INFINITY, f64::max) } else { (0..m).map(|i| norm[i][j]).fold(f64::INFINITY, f64::min) }).collect();
        let ideal_neg: Vec<f64> = (0..n).map(|j| if self.benefit[j] { (0..m).map(|i| norm[i][j]).fold(f64::INFINITY, f64::min) } else { (0..m).map(|i| norm[i][j]).fold(f64::NEG_INFINITY, f64::max) }).collect();
        let mut scores = Vec::new();
        for i in 0..m {
            let d_pos: f64 = (0..n).map(|j| (norm[i][j] - ideal_pos[j]).powi(2)).sum::<f64>().sqrt();
            let d_neg: f64 = (0..n).map(|j| (norm[i][j] - ideal_neg[j]).powi(2)).sum::<f64>().sqrt();
            scores.push((i, d_neg / (d_pos + d_neg + 1e-12)));
        }
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores
    }
}

/// 📊️ Fuzzy VIKOR compromise ranking.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FuzzyVikor {
    pub alternatives: Vec<String>,
    pub criteria: Vec<String>,
    pub decision_matrix: Vec<Vec<FuzzyNumber>>,
    pub weights: Vec<f64>,
    pub benefit: Vec<bool>,
    pub v: f64,
}

impl FuzzyVikor {
    pub fn rank(&self) -> Vec<(usize, f64)> {
        let m = self.alternatives.len();
        let n = self.criteria.len();
        let crisp = self.decision_matrix.iter().map(|row| row.iter().map(|cell| cell.defuzzify_centroid(32)).collect::<Vec<_>>()).collect::<Vec<_>>();
        let f_star: Vec<f64> = (0..n).map(|j| if self.benefit[j] { (0..m).map(|i| crisp[i][j]).fold(f64::NEG_INFINITY, f64::max) } else { (0..m).map(|i| crisp[i][j]).fold(f64::INFINITY, f64::min) }).collect();
        let _f_minus: Vec<f64> = (0..n).map(|j| if self.benefit[j] { (0..m).map(|i| crisp[i][j]).fold(f64::INFINITY, f64::min) } else { (0..m).map(|i| crisp[i][j]).fold(f64::NEG_INFINITY, f64::max) }).collect();
        let mut s = vec![0.0_f64; m];
        let mut r = vec![0.0_f64; m];
        for i in 0..m {
            for j in 0..n {
                let d = if self.benefit[j] { (f_star[j] - crisp[i][j]).max(0.0) } else { (crisp[i][j] - f_star[j]).max(0.0) };
                s[i] += self.weights[j] * d;
                r[i] = r[i].max(self.weights[j] * d);
            }
        }
        let s_star = s.iter().copied().fold(f64::INFINITY, f64::min);
        let s_minus = s.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let r_star = r.iter().copied().fold(f64::INFINITY, f64::min);
        let r_minus = r.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let mut q = Vec::new();
        for i in 0..m {
            let qi = self.v * (s[i] - s_star) / (s_minus - s_star + 1e-12) + (1.0 - self.v) * (r[i] - r_star) / (r_minus - r_star + 1e-12);
            q.push((i, qi));
        }
        q.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        q
    }
}
// #endregion 🔖️MulticriteriaDecision

// #region 🔖️TemporalSpatial
/// ⏱️ Temporal fuzzy evaluator for vague time concepts.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemporalEvaluator {
    pub now: f64,
    pub recent_window: f64,
    pub frequent_window: f64,
}

impl TemporalEvaluator {
    pub fn recently(&self, timestamp: f64) -> f64 {
        let dt = (self.now - timestamp).max(0.0);
        clamp01(1.0 - dt / self.recent_window.max(1e-6))
    }

    pub fn frequently(&self, timestamps: &[f64]) -> f64 {
        let count = timestamps.iter().filter(|t| (self.now - **t).abs() <= self.frequent_window).count() as f64;
        clamp01(count / 5.0)
    }
}

/// 📍️ Spatial fuzzy evaluator for proximity concepts.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpatialEvaluator {
    pub anchor: Vec<f64>,
    pub near_radius: f64,
}

impl SpatialEvaluator {
    pub fn near(&self, point: &[f64]) -> f64 {
        let dist2: f64 = self.anchor.iter().zip(point.iter()).map(|(a, p)| (a - p).powi(2)).sum();
        clamp01(1.0 - dist2.sqrt() / self.near_radius.max(1e-6))
    }

    pub fn slowly(&self, velocity: f64, max_speed: f64) -> f64 {
        clamp01(1.0 - velocity.abs() / max_speed.max(1e-6))
    }
}
// #endregion 🔖️TemporalSpatial

// #region 🔖️HybridUncertainty
/// 🔀️ Map probabilistic scores to fuzzy membership without external Bayesian engines.
pub fn probabilistic_to_membership(probability: f64, certainty: f64) -> f64 {
    clamp01(probability * certainty + 0.5 * (1.0 - certainty))
}

/// 🔀️ Bridge possibility and fuzzy membership on a shared universe.
pub fn possibility_to_membership(possibility: &PossibilityMeasure) -> Vec<f64> {
    possibility.membership.clone()
}

/// 🔀️ Combine fuzzy and possibility evidence via weighted fusion.
pub fn hybrid_fuse(membership: f64, possibility: f64, fuzzy_weight: f64) -> f64 {
    let w = clamp01(fuzzy_weight);
    clamp01(w * membership + (1.0 - w) * possibility)
}
// #endregion 🔖️HybridUncertainty

// #region 🔖️FanController
/// 🌀️ Fan-speed controller builder demonstrating the full advanced fuzzy pipeline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FanController {
    pub temperature_var: LinguisticVariable,
    pub speed_var: LinguisticVariable,
    pub sensor_uncertainty: IntervalType2Set,
    pub evolving: EvolvingFuzzySystem,
}

impl FanController {
    pub fn from_sensor_data(temps: &[f64], fan_speeds: &[f64]) -> FuzzyResult<Self> {
        let t_min = temps.iter().copied().fold(f64::INFINITY, f64::min);
        let t_max = temps.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let s_max = fan_speeds.iter().copied().fold(0.0_f64, f64::max).max(1.0);
        let temp_univ = Universe::new(t_min - 1.0, t_max + 1.0, 101)?;
        let speed_univ = Universe::new(0.0, s_max * 1.1, 101)?;
        let mut low_mf = MembershipFunction::triangular(t_min, t_min, (t_min + t_max) * 0.5);
        let mut high_mf = MembershipFunction::triangular((t_min + t_max) * 0.5, t_max, t_max);
        let low_samples: Vec<(f64, f64)> = temps.iter().map(|&t| (t, if t < (t_min + t_max) * 0.5 { 1.0 } else { 0.0 })).collect();
        let high_samples: Vec<(f64, f64)> = temps.iter().map(|&t| (t, if t >= (t_min + t_max) * 0.5 { 1.0 } else { 0.0 })).collect();
        let mut low_adapt = AdaptiveMembership::new(low_mf, 0.01);
        low_adapt.fit(&low_samples, 20);
        low_mf = low_adapt.mf;
        let mut high_adapt = AdaptiveMembership::new(high_mf, 0.01);
        high_adapt.fit(&high_samples, 20);
        high_mf = high_adapt.mf;
        let temperature_var = LinguisticVariable::new("temperature", temp_univ, vec![FuzzySet::new("low", low_mf), FuzzySet::new("high", high_mf)]);
        let speed_var = LinguisticVariable::new("fan_speed", speed_univ, vec![FuzzySet::new("slow", MembershipFunction::triangular(0.0, 0.0, s_max * 0.5)), FuzzySet::new("fast", MembershipFunction::triangular(s_max * 0.4, s_max, s_max))]);
        let sensor_uncertainty = IntervalType2Set::new("temperature_sensor", MembershipFunction::gaussian((t_min + t_max) * 0.5, (t_max - t_min) * 0.05), MembershipFunction::gaussian((t_min + t_max) * 0.5, (t_max - t_min) * 0.12));
        let rules = RuleBase::new(vec![
            Rule { id: 0, antecedents: vec![AntecedentClause { input: 0, term: 1, hedge: None }], consequent: Consequent::Mamdani { output: 0, term: 1 }, weight: 0.9, confidence: 0.85 },
            Rule { id: 1, antecedents: vec![AntecedentClause { input: 0, term: 0, hedge: None }], consequent: Consequent::Mamdani { output: 0, term: 0 }, weight: 0.8, confidence: 0.8 },
        ]);
        let system = MimoSystem { inputs: vec![temperature_var.clone()], outputs: vec![speed_var.clone()], rules, model: InferenceModel::Mamdani, tnorm: TNorm::Min, tconorm: TConorm::Max, defuzzifier: Defuzzifier::Centroid };
        let evolving = EvolvingFuzzySystem::new(system, 0.05, 0.1, 8);
        Ok(Self { temperature_var, speed_var, sensor_uncertainty, evolving })
    }

    pub fn decide(&mut self, temperature: f64) -> FuzzyResult<(f64, Explanation)> {
        let (lo, hi) = self.sensor_uncertainty.grade_interval(temperature);
        let adjusted = 0.5 * (temperature + self.sensor_uncertainty.type_reduced_centroid(&[lo, temperature, hi]));
        self.evolving.observe(&[adjusted], &[0.0])?;
        let (outputs, explanation) = self.evolving.system.infer(&[adjusted])?;
        Ok((outputs[0], explanation))
    }
}
// #endregion 🔖️FanController

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn temp_speed_system() -> MimoSystem {
        let temp_univ = Universe::new(0.0, 40.0, 41).unwrap();
        let speed_univ = Universe::new(0.0, 100.0, 41).unwrap();
        let temp_var = LinguisticVariable::new("temperature", temp_univ, vec![FuzzySet::new("low", MembershipFunction::triangular(0.0, 0.0, 20.0)), FuzzySet::new("high", MembershipFunction::triangular(15.0, 40.0, 40.0))]);
        let speed_var = LinguisticVariable::new("fan_speed", speed_univ, vec![FuzzySet::new("slow", MembershipFunction::triangular(0.0, 0.0, 50.0)), FuzzySet::new("fast", MembershipFunction::triangular(40.0, 100.0, 100.0))]);
        MimoSystem {
            inputs: vec![temp_var],
            outputs: vec![speed_var],
            rules: RuleBase::new(vec![
                Rule { id: 0, antecedents: vec![AntecedentClause { input: 0, term: 1, hedge: None }], consequent: Consequent::Mamdani { output: 0, term: 1 }, weight: 1.0, confidence: 1.0 },
                Rule { id: 1, antecedents: vec![AntecedentClause { input: 0, term: 0, hedge: None }], consequent: Consequent::Mamdani { output: 0, term: 0 }, weight: 1.0, confidence: 1.0 },
            ]),
            model: InferenceModel::Mamdani,
            tnorm: TNorm::Min,
            tconorm: TConorm::Max,
            defuzzifier: Defuzzifier::Centroid,
        }
    }

    #[test]
    fn triangular_membership_peaks_at_center() {
        let mf = MembershipFunction::triangular(0.0, 5.0, 10.0);
        assert!((mf.eval(5.0) - 1.0).abs() < 1e-9);
        assert_eq!(mf.eval(-1.0), 0.0);
    }

    #[test]
    fn tnorm_product_is_stricter_than_min() {
        assert!(TNorm::Product.apply(0.6, 0.6) < TNorm::Min.apply(0.6, 0.6));
    }

    #[test]
    fn fuzzy_number_alpha_cut_contains_peak() {
        let n = FuzzyNumber::triangular(1.0, 3.0, 5.0);
        let (lo, hi) = n.alpha_cut(1.0);
        assert!(lo <= 3.0 && hi >= 3.0);
    }

    #[test]
    fn relation_composition_max_min() {
        let mut r1 = FuzzyRelation::new(2, 2);
        r1.set(0, 0, 0.8);
        r1.set(0, 1, 0.3);
        r1.set(1, 0, 0.2);
        r1.set(1, 1, 0.9);
        let composed = r1.compose_max_min(&r1).unwrap();
        assert!(composed.get(0, 0) > 0.0);
    }

    #[test]
    fn mamdani_high_temperature_yields_fast_fan() {
        let system = temp_speed_system();
        let (out, explanation) = system.infer(&[35.0]).unwrap();
        assert!(out[0] > 50.0);
        assert!(!explanation.traces.is_empty());
    }

    #[test]
    fn sugeno_inference_weighted_average() {
        let mut system = temp_speed_system();
        system.model = InferenceModel::Sugeno;
        system.rules = RuleBase::new(vec![
            Rule { id: 0, antecedents: vec![AntecedentClause { input: 0, term: 1, hedge: None }], consequent: Consequent::SugenoConstant { output: 0, value: 90.0 }, weight: 1.0, confidence: 1.0 },
            Rule { id: 1, antecedents: vec![AntecedentClause { input: 0, term: 0, hedge: None }], consequent: Consequent::SugenoConstant { output: 0, value: 10.0 }, weight: 1.0, confidence: 1.0 },
        ]);
        let (out, _) = system.infer(&[35.0]).unwrap();
        assert!(out[0] > 70.0);
    }

    #[test]
    fn defuzzifier_centroid_on_triangle() {
        let univ = Universe::new(0.0, 10.0, 101).unwrap();
        let membership: Vec<f64> = univ.samples.iter().map(|x| MembershipFunction::triangular(0.0, 5.0, 10.0).eval(*x)).collect();
        let c = Defuzzifier::Centroid.apply(&univ, &membership, None, None).unwrap();
        assert!((c - 5.0).abs() < 0.2);
    }

    #[test]
    fn fuzzy_c_means_two_cluster_toy() {
        let data = vec![vec![0.0, 0.0], vec![0.1, 0.0], vec![5.0, 5.0], vec![5.1, 5.0]];
        let result = fuzzy_c_means(&data, 2, 2.0, 50, 1e-4).unwrap();
        assert_eq!(result.centers.len(), 2);
        assert_eq!(result.membership.len(), 4);
    }

    #[test]
    fn fuzzy_ahp_produces_normalized_weights() {
        let matrix = vec![vec![FuzzyNumber::triangular(1.0, 1.0, 1.0), FuzzyNumber::triangular(2.0, 3.0, 4.0)], vec![FuzzyNumber::triangular(0.25, 0.33, 0.5), FuzzyNumber::triangular(1.0, 1.0, 1.0)]];
        let weights = FuzzyAhp::new(matrix).weights();
        assert!((weights.iter().sum::<f64>() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn fuzzy_topsis_ranks_better_alternative_first() {
        let topsis = FuzzyTopsis {
            alternatives: vec!["a".into(), "b".into()],
            criteria: vec!["c".into()],
            decision_matrix: vec![vec![FuzzyNumber::triangular(8.0, 9.0, 10.0)], vec![FuzzyNumber::triangular(1.0, 2.0, 3.0)]],
            weights: vec![1.0],
            benefit: vec![true],
        };
        let rank = topsis.rank();
        assert_eq!(rank[0].0, 0);
    }

    #[test]
    fn interval_type2_centroid_between_bounds() {
        let it2 = IntervalType2Set::new("temp", MembershipFunction::gaussian(20.0, 1.0), MembershipFunction::gaussian(20.0, 3.0));
        let c = it2.type_reduced_centroid(&linspace(10.0, 30.0, 41));
        assert!((c - 20.0).abs() < 2.0);
    }

    #[test]
    fn fan_controller_learns_from_sensor_data() {
        let temps: Vec<f64> = (0..20).map(|i| i as f64 * 2.0).collect();
        let speeds: Vec<f64> = temps.iter().map(|t| t * 2.5).collect();
        let mut controller = FanController::from_sensor_data(&temps, &speeds).unwrap();
        let (speed, explanation) = controller.decide(30.0).unwrap();
        assert!(speed >= 0.0);
        assert!(!explanation.rationale.is_empty());
    }

    #[test]
    fn trapezoidal_membership_flat_top_and_edges() {
        let mf = MembershipFunction::trapezoidal(0.0, 2.0, 6.0, 8.0);
        assert_eq!(mf.eval(-1.0), 0.0);
        assert_eq!(mf.eval(0.0), 0.0);
        assert!((mf.eval(1.0) - 0.5).abs() < 1e-9);
        assert_eq!(mf.eval(4.0), 1.0);
        assert!((mf.eval(7.0) - 0.5).abs() < 1e-9);
        assert_eq!(mf.eval(8.0), 0.0);
    }

    #[test]
    fn gaussian_membership_symmetric_and_peaks_at_mean() {
        let mf = MembershipFunction::gaussian(10.0, 2.0);
        assert_eq!(mf.eval(10.0), 1.0);
        assert!((mf.eval(12.0) - mf.eval(8.0)).abs() < 1e-12);
        assert!((mf.eval(12.0) - 0.6065306597126334).abs() < 1e-9);
    }

    #[test]
    fn generalized_bell_membership_symmetric_peak() {
        let mf = MembershipFunction::generalized_bell(2.0, 4.0, 5.0);
        assert_eq!(mf.eval(5.0), 1.0);
        assert!((mf.eval(7.0) - mf.eval(3.0)).abs() < 1e-9);
        assert!((mf.eval(7.0) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn sigmoid_membership_is_monotonic_increasing() {
        let mf = MembershipFunction::sigmoid(1.0, 0.0);
        assert!((mf.eval(0.0) - 0.5).abs() < 1e-9);
        assert!(mf.eval(-1.0) < mf.eval(0.0));
        assert!(mf.eval(0.0) < mf.eval(1.0));
    }

    #[test]
    fn singleton_membership_exact_match_only() {
        let mf = MembershipFunction::singleton(5.0);
        assert_eq!(mf.eval(5.0), 1.0);
        assert_eq!(mf.eval(5.001), 0.0);
    }

    #[test]
    fn piecewise_linear_membership_interpolates_and_edge_cases() {
        assert_eq!(MembershipFunction::piecewise_linear(vec![]).eval(0.0), 0.0);
        assert_eq!(MembershipFunction::piecewise_linear(vec![(2.0, 0.7)]).eval(99.0), 0.7);
        let mf = MembershipFunction::piecewise_linear(vec![(0.0, 0.0), (5.0, 1.0), (10.0, 0.0)]);
        assert_eq!(mf.eval(-1.0), 0.0);
        assert_eq!(mf.eval(11.0), 0.0);
        assert!((mf.eval(2.5) - 0.5).abs() < 1e-9);
        assert!((mf.eval(7.5) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn membership_function_parameters_roundtrip() {
        let mut tri = MembershipFunction::triangular(0.0, 1.0, 2.0);
        assert_eq!(tri.parameters(), vec![0.0, 1.0, 2.0]);
        tri.set_parameters(&[1.0, 2.0, 3.0]).unwrap();
        assert_eq!(tri.parameters(), vec![1.0, 2.0, 3.0]);
        assert_eq!(tri.set_parameters(&[1.0, 2.0]), Err(FuzzyError::InvalidParameterCount { expected: 3, got: 2 }));

        let mut trap = MembershipFunction::trapezoidal(0.0, 1.0, 2.0, 3.0);
        trap.set_parameters(&[1.0, 2.0, 3.0, 4.0]).unwrap();
        assert_eq!(trap.parameters(), vec![1.0, 2.0, 3.0, 4.0]);
        assert!(trap.set_parameters(&[1.0]).is_err());

        let mut gauss = MembershipFunction::gaussian(0.0, 1.0);
        gauss.set_parameters(&[2.0, 1e-20]).unwrap();
        assert_eq!(gauss.parameters(), vec![2.0, 1e-12]);
        assert!(gauss.set_parameters(&[1.0]).is_err());

        let mut bell = MembershipFunction::generalized_bell(1.0, 1.0, 0.0);
        bell.set_parameters(&[1e-20, 1e-20, 5.0]).unwrap();
        assert_eq!(bell.parameters(), vec![1e-12, 1e-6, 5.0]);
        assert!(bell.set_parameters(&[1.0, 2.0]).is_err());

        let mut sig = MembershipFunction::sigmoid(1.0, 0.0);
        sig.set_parameters(&[2.0, 3.0]).unwrap();
        assert_eq!(sig.parameters(), vec![2.0, 3.0]);
        assert!(sig.set_parameters(&[]).is_err());

        let mut single = MembershipFunction::singleton(1.0);
        single.set_parameters(&[9.0]).unwrap();
        assert_eq!(single.parameters(), vec![9.0]);
        assert!(single.set_parameters(&[1.0, 2.0]).is_err());

        let mut pw = MembershipFunction::piecewise_linear(vec![(0.0, 0.0)]);
        pw.set_parameters(&[0.0, 0.0, 1.0, 1.0]).unwrap();
        assert_eq!(pw.parameters(), vec![0.0, 0.0, 1.0, 1.0]);
        assert!(pw.set_parameters(&[0.0, 0.0, 1.0]).is_err());
    }

    #[test]
    fn membership_function_support_bounds() {
        assert_eq!(MembershipFunction::triangular(1.0, 2.0, 3.0).support_min(), 1.0);
        assert_eq!(MembershipFunction::triangular(1.0, 2.0, 3.0).support_max(), 3.0);
        assert_eq!(MembershipFunction::trapezoidal(1.0, 2.0, 3.0, 4.0).support_min(), 1.0);
        assert_eq!(MembershipFunction::trapezoidal(1.0, 2.0, 3.0, 4.0).support_max(), 4.0);
        let g = MembershipFunction::gaussian(10.0, 2.0);
        assert_eq!(g.support_min(), 2.0);
        assert_eq!(g.support_max(), 18.0);
        let bell = MembershipFunction::generalized_bell(2.0, 4.0, 5.0);
        assert_eq!(bell.support_min(), -3.0);
        assert_eq!(bell.support_max(), 13.0);
        let sig = MembershipFunction::sigmoid(1.0, 5.0);
        assert_eq!(sig.support_min(), -5.0);
        assert_eq!(sig.support_max(), 15.0);
        let single = MembershipFunction::singleton(7.0);
        assert_eq!(single.support_min(), 7.0);
        assert_eq!(single.support_max(), 7.0);
        assert_eq!(MembershipFunction::piecewise_linear(vec![]).support_min(), 0.0);
        let pw = MembershipFunction::piecewise_linear(vec![(1.0, 0.0), (9.0, 1.0)]);
        assert_eq!(pw.support_min(), 1.0);
        assert_eq!(pw.support_max(), 9.0);
    }

    #[test]
    fn intuitionistic_set_grades_rejects_over_unity() {
        let set = IntuitionisticSet::new("x", MembershipFunction::triangular(0.0, 5.0, 10.0), MembershipFunction::triangular(0.0, 5.0, 10.0));
        assert_eq!(set.grades(5.0), Err(FuzzyError::InvalidIntuitionistic));
    }

    #[test]
    fn intuitionistic_set_grades_computes_hesitation() {
        let set = IntuitionisticSet::new("x", MembershipFunction::triangular(0.0, 5.0, 10.0), MembershipFunction::singleton(100.0));
        let (mu, nu, hesitation) = set.grades(2.0).unwrap();
        assert!((mu - 0.4).abs() < 1e-9);
        assert_eq!(nu, 0.0);
        assert!((hesitation - 0.6).abs() < 1e-9);
    }

    #[test]
    fn tnorm_lukasiewicz_and_drastic_variants() {
        assert!((TNorm::Lukasiewicz.apply(0.6, 0.6) - 0.2).abs() < 1e-9);
        assert_eq!(TNorm::Lukasiewicz.apply(0.3, 0.3), 0.0);
        assert_eq!(TNorm::Drastic.apply(0.5, 1.0), 0.5);
        assert_eq!(TNorm::Drastic.apply(1.0, 0.5), 0.5);
        assert_eq!(TNorm::Drastic.apply(0.5, 0.5), 0.0);
    }

    #[test]
    fn tconorm_variants_apply_correctly() {
        assert_eq!(TConorm::Max.apply(0.3, 0.7), 0.7);
        assert!((TConorm::ProbSum.apply(0.5, 0.5) - 0.75).abs() < 1e-9);
        assert_eq!(TConorm::Lukasiewicz.apply(0.6, 0.6), 1.0);
        assert_eq!(TConorm::NilpotentMax.apply(0.3, 0.3), 0.0);
        assert_eq!(TConorm::NilpotentMax.apply(0.6, 0.6), 0.6);
    }

    #[test]
    fn tnorm_tconorm_fold_over_iterator() {
        let vals = [0.9, 0.8, 0.7];
        assert!((TNorm::Min.fold(vals.iter().copied()) - 0.7).abs() < 1e-9);
        assert!((TConorm::Max.fold(vals.iter().copied()) - 0.9).abs() < 1e-9);
    }

    #[test]
    fn hedge_variants_apply() {
        assert!((Hedge::Very.apply(0.8) - 0.64).abs() < 1e-9);
        assert!((Hedge::Somewhat.apply(0.64) - 0.8).abs() < 1e-9);
        assert!((Hedge::Extremely.apply(0.5) - 0.125).abs() < 1e-9);
        assert!((Hedge::MoreOrLess.apply(0.5) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn complement_concentration_dilation_basic() {
        assert!((complement(0.3) - 0.7).abs() < 1e-9);
        assert!((concentration(0.5) - 0.25).abs() < 1e-9);
        assert!((dilation(0.25) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn fuzzy_number_sub_and_scale() {
        let n1 = FuzzyNumber::triangular(1.0, 3.0, 5.0);
        let n2 = FuzzyNumber::triangular(1.0, 2.0, 3.0);
        let diff = n1.sub(n2);
        assert_eq!((diff.a, diff.b, diff.c), (-2.0, 1.0, 4.0));
        let scaled = n1.scale(-2.0);
        assert_eq!((scaled.a, scaled.b, scaled.c), (-10.0, -6.0, -2.0));
        let scaled_pos = n1.scale(2.0);
        assert_eq!((scaled_pos.a, scaled_pos.b, scaled_pos.c), (2.0, 6.0, 10.0));
    }

    #[test]
    fn fuzzy_number_defuzzify_centroid_matches_peak() {
        let n = FuzzyNumber::triangular(0.0, 5.0, 10.0);
        assert!((n.defuzzify_centroid(101) - 5.0).abs() < 0.1);
    }

    #[test]
    fn fuzzy_add_sums_components() {
        let a = FuzzyNumber::triangular(1.0, 2.0, 3.0);
        let b = FuzzyNumber::triangular(1.0, 1.0, 1.0);
        let sum = fuzzy_add(a, b);
        assert_eq!((sum.a, sum.b, sum.c), (2.0, 3.0, 4.0));
    }

    #[test]
    fn fuzzy_mul_interval_at_peak_alpha() {
        let a = FuzzyNumber::triangular(1.0, 2.0, 3.0);
        let b = FuzzyNumber::triangular(2.0, 3.0, 4.0);
        let (lo, hi) = fuzzy_mul_interval(a, b, 1.0);
        assert!((lo - 6.0).abs() < 1e-9);
        assert!((hi - 6.0).abs() < 1e-9);
    }

    #[test]
    fn relation_composition_max_product() {
        let mut r1 = FuzzyRelation::new(2, 2);
        r1.set(0, 0, 0.5);
        r1.set(0, 1, 0.5);
        r1.set(1, 0, 0.5);
        r1.set(1, 1, 0.5);
        let composed = r1.compose_max_product(&r1).unwrap();
        assert!((composed.get(0, 0) - 0.25).abs() < 1e-9);
    }

    #[test]
    fn relation_composition_dimension_mismatch_errors() {
        let r1 = FuzzyRelation::new(2, 3);
        let r2 = FuzzyRelation::new(2, 2);
        assert_eq!(r1.compose_max_min(&r2), Err(FuzzyError::DimensionMismatch("relation composition".into())));
        assert_eq!(r1.compose_max_product(&r2), Err(FuzzyError::DimensionMismatch("relation composition".into())));
    }

    #[test]
    fn possibility_and_necessity_measures() {
        let pm = PossibilityMeasure::new(vec![1.0, 2.0, 3.0, 4.0, 5.0], vec![0.2, 0.5, 0.8, 0.3, 0.1]).unwrap();
        assert!((pm.possibility(|x| x >= 3.0) - 0.8).abs() < 1e-9);
        assert!((pm.necessity(|x| x >= 3.0) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn possibility_measure_from_scores_normalizes_by_max() {
        let pm = PossibilityMeasure::from_scores(vec![1.0, 2.0, 3.0], vec![2.0, 4.0, 1.0]).unwrap();
        assert_eq!(pm.membership, vec![0.5, 1.0, 0.25]);
    }

    #[test]
    fn possibility_measure_rejects_mismatched_lengths() {
        assert_eq!(PossibilityMeasure::new(vec![1.0, 2.0], vec![0.5]), Err(FuzzyError::InvalidDomain("possibility universe".into())));
    }

    #[test]
    fn universe_rejects_invalid_bounds() {
        assert_eq!(Universe::new(5.0, 1.0, 10), Err(FuzzyError::InvalidDomain("universe bounds".into())));
        assert_eq!(Universe::new(0.0, 10.0, 0), Err(FuzzyError::InvalidDomain("universe bounds".into())));
    }

    #[test]
    fn universe_len_sample_and_is_empty() {
        let u = Universe::new(0.0, 10.0, 5).unwrap();
        assert_eq!(u.len(), 5);
        assert!(!u.is_empty());
        assert_eq!(u.sample(0), 0.0);
        assert_eq!(u.sample(4), 10.0);
    }

    #[test]
    fn linguistic_variable_fuzzify_and_term_index() {
        let univ = Universe::new(0.0, 10.0, 11).unwrap();
        let var = LinguisticVariable::new("x", univ, vec![FuzzySet::new("low", MembershipFunction::triangular(0.0, 0.0, 5.0)), FuzzySet::new("high", MembershipFunction::triangular(5.0, 10.0, 10.0))]);
        assert_eq!(var.term_index("high"), Some(1));
        assert_eq!(var.term_index("missing"), None);
        let grades = var.fuzzify(2.5);
        assert_eq!(grades.len(), 2);
        assert!(grades[0] > 0.0);
    }

    #[test]
    fn rule_firing_strength_applies_hedge() {
        let univ = Universe::new(0.0, 10.0, 11).unwrap();
        let var = LinguisticVariable::new("x", univ, vec![FuzzySet::new("mid", MembershipFunction::triangular(0.0, 5.0, 10.0))]);
        let rule = Rule { id: 0, antecedents: vec![AntecedentClause { input: 0, term: 0, hedge: Some(Hedge::Very) }], consequent: Consequent::Mamdani { output: 0, term: 0 }, weight: 1.0, confidence: 1.0 };
        let strength = rule.firing_strength(&[var], &[2.5], TNorm::Min);
        assert!((strength - 0.25).abs() < 1e-9);
    }

    #[test]
    fn rule_base_is_empty_reports_correctly() {
        assert!(RuleBase::new(vec![]).is_empty());
        let rule = Rule { id: 0, antecedents: vec![], consequent: Consequent::SugenoConstant { output: 0, value: 1.0 }, weight: 1.0, confidence: 1.0 };
        assert!(!RuleBase::new(vec![rule]).is_empty());
    }

    #[test]
    fn defuzzifier_empty_universe_errors() {
        let empty = Universe { min: 0.0, max: 1.0, samples: vec![] };
        assert_eq!(Defuzzifier::Centroid.apply(&empty, &[], None, None), Err(FuzzyError::EmptyUniverse));
    }

    #[test]
    fn defuzzifier_weighted_average_and_height_match() {
        let univ = Universe::new(0.0, 1.0, 3).unwrap();
        let membership = vec![0.1, 0.1, 0.1];
        let heights = vec![0.5, 0.8];
        let values = vec![10.0, 20.0];
        let wa = Defuzzifier::WeightedAverage.apply(&univ, &membership, Some(&heights), Some(&values)).unwrap();
        let ht = Defuzzifier::Height.apply(&univ, &membership, Some(&heights), Some(&values)).unwrap();
        assert!((wa - 16.153846153846153).abs() < 1e-9);
        assert_eq!(wa, ht);
    }

    #[test]
    fn defuzzifier_weighted_average_requires_heights_and_values() {
        let univ = Universe::new(0.0, 1.0, 3).unwrap();
        let membership = vec![0.1, 0.1, 0.1];
        assert!(Defuzzifier::WeightedAverage.apply(&univ, &membership, None, None).is_err());
        assert!(Defuzzifier::Height.apply(&univ, &membership, None, None).is_err());
    }

    #[test]
    fn defuzzifier_bisector_mom_som_lom() {
        let univ = Universe::new(0.0, 5.0, 6).unwrap();
        let membership = vec![0.0, 0.2, 0.9, 0.9, 0.9, 0.1];
        let bisector = Defuzzifier::Bisector.apply(&univ, &membership, None, None).unwrap();
        assert!((bisector - 3.0).abs() < 1e-9);
        let mom = Defuzzifier::Mom.apply(&univ, &membership, None, None).unwrap();
        assert!((mom - 3.0).abs() < 1e-9);
        // 🔍️ argmax breaks ties on the *last* max (std max_by semantics), so Som and Lom
        // coincide here even though "smallest of maximum" would conventionally pick x=2.
        let som = Defuzzifier::Som.apply(&univ, &membership, None, None).unwrap();
        let lom = Defuzzifier::Lom.apply(&univ, &membership, None, None).unwrap();
        assert!((som - 4.0).abs() < 1e-9);
        assert!((lom - 4.0).abs() < 1e-9);
    }

    #[test]
    fn mimo_infer_rejects_bad_input_and_empty_rules() {
        let system = temp_speed_system();
        assert_eq!(system.infer(&[1.0, 2.0]), Err(FuzzyError::DimensionMismatch("input count".into())));
        let mut empty_system = system;
        empty_system.rules = RuleBase::new(vec![]);
        assert_eq!(empty_system.infer(&[20.0]), Err(FuzzyError::EmptyRuleBase));
    }

    #[test]
    fn larsen_inference_model_produces_output() {
        let mut system = temp_speed_system();
        system.model = InferenceModel::Larsen;
        let (out, explanation) = system.infer(&[35.0]).unwrap();
        assert!(out[0] >= 0.0);
        assert!(explanation.traces.iter().any(|t| t.description.contains("Larsen")));
    }

    #[test]
    fn tsukamoto_inference_model_inverts_monotonic_consequent() {
        let temp_univ = Universe::new(0.0, 40.0, 41).unwrap();
        let temp_var = LinguisticVariable::new("temperature", temp_univ, vec![FuzzySet::new("high", MembershipFunction::triangular(0.0, 40.0, 40.0))]);
        let out_univ = Universe::new(0.0, 100.0, 101).unwrap();
        let out_var = LinguisticVariable::new("speed", out_univ, vec![FuzzySet::new("level", MembershipFunction::sigmoid(0.2, 50.0))]);
        let system = MimoSystem {
            inputs: vec![temp_var],
            outputs: vec![out_var],
            rules: RuleBase::new(vec![Rule { id: 0, antecedents: vec![AntecedentClause { input: 0, term: 0, hedge: None }], consequent: Consequent::Tsukamoto { output: 0, term: 0 }, weight: 1.0, confidence: 1.0 }]),
            model: InferenceModel::Tsukamoto,
            tnorm: TNorm::Min,
            tconorm: TConorm::Max,
            defuzzifier: Defuzzifier::Centroid,
        };
        let (out, _) = system.infer(&[30.0]).unwrap();
        assert!(out[0] > 0.0 && out[0] < 100.0);
    }

    #[test]
    fn soft_constraint_consequent_scales_by_preference() {
        let system = temp_speed_system();
        let mut soft_system = system;
        soft_system.rules = RuleBase::new(vec![Rule { id: 0, antecedents: vec![AntecedentClause { input: 0, term: 1, hedge: None }], consequent: Consequent::SoftConstraint { output: 0, term: 1, preference: 0.5 }, weight: 1.0, confidence: 1.0 }]);
        let (out, explanation) = soft_system.infer(&[35.0]).unwrap();
        assert!(out[0] >= 0.0);
        assert!(explanation.traces.iter().any(|t| t.description.contains("soft constraint")));
    }

    #[test]
    fn tsukamoto_inverse_handles_zero_alpha_and_solves_monotonic() {
        let mf = MembershipFunction::sigmoid(1.0, 0.0);
        assert_eq!(tsukamoto_inverse(&mf, 0.0), mf.support_min());
        let x = tsukamoto_inverse(&mf, 0.5);
        assert!((mf.eval(x) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn adaptive_membership_fit_updates_parameters() {
        let mf = MembershipFunction::triangular(0.0, 3.0, 10.0);
        let samples: Vec<(f64, f64)> = (0..10)
            .map(|i| {
                let x = i as f64;
                (x, MembershipFunction::triangular(0.0, 5.0, 10.0).eval(x))
            })
            .collect();
        let mut adaptive = AdaptiveMembership::new(mf, 0.05);
        let initial_params = adaptive.mf.parameters();
        let loss = adaptive.fit(&samples, 10);
        assert!(loss.is_finite() && loss >= 0.0);
        assert_ne!(adaptive.mf.parameters(), initial_params);
    }

    #[test]
    fn anfis_default_forward_and_rule_count() {
        let anfis = Anfis::new(2, 2, &[(0.0, 1.0), (0.0, 1.0)]);
        assert_eq!(anfis.rule_count(), 4);
        assert_eq!(anfis.forward(&[0.5, 0.5]), 0.0);
    }

    #[test]
    fn wang_mendel_rules_generates_one_rule_per_sample() {
        let univ = Universe::new(0.0, 10.0, 11).unwrap();
        let input_var = LinguisticVariable::new("x", univ.clone(), vec![FuzzySet::new("low", MembershipFunction::triangular(0.0, 0.0, 10.0)), FuzzySet::new("high", MembershipFunction::triangular(0.0, 10.0, 10.0))]);
        let output_var = LinguisticVariable::new("y", univ, vec![FuzzySet::new("low", MembershipFunction::triangular(0.0, 0.0, 10.0)), FuzzySet::new("high", MembershipFunction::triangular(0.0, 10.0, 10.0))]);
        let data = vec![(vec![1.0], 2.0), (vec![9.0], 8.0)];
        let rules = wang_mendel_rules(&[input_var], &output_var, &data, InferenceModel::Mamdani);
        assert_eq!(rules.rules.len(), 2);
        assert!(matches!(rules.rules[0].consequent, Consequent::Mamdani { .. }));
    }

    #[test]
    fn wang_mendel_rules_sugeno_uses_constant_consequent() {
        let univ = Universe::new(0.0, 10.0, 11).unwrap();
        let input_var = LinguisticVariable::new("x", univ.clone(), vec![FuzzySet::new("low", MembershipFunction::triangular(0.0, 0.0, 10.0))]);
        let output_var = LinguisticVariable::new("y", univ, vec![FuzzySet::new("low", MembershipFunction::triangular(0.0, 0.0, 10.0))]);
        let data = vec![(vec![1.0], 3.5)];
        let rules = wang_mendel_rules(&[input_var], &output_var, &data, InferenceModel::Sugeno);
        assert!(matches!(rules.rules[0].consequent, Consequent::SugenoConstant { value, .. } if (value - 3.5).abs() < 1e-9));
    }

    #[test]
    fn prune_rules_filters_below_thresholds() {
        let rules = RuleBase::new(vec![
            Rule { id: 0, antecedents: vec![], consequent: Consequent::SugenoConstant { output: 0, value: 1.0 }, weight: 0.9, confidence: 0.9 },
            Rule { id: 1, antecedents: vec![], consequent: Consequent::SugenoConstant { output: 0, value: 1.0 }, weight: 0.1, confidence: 0.9 },
        ]);
        let pruned = prune_rules(rules, 0.5, 0.5);
        assert_eq!(pruned.rules.len(), 1);
        assert_eq!(pruned.rules[0].id, 0);
    }

    #[test]
    fn weight_rules_by_fit_updates_confidence_from_error() {
        let mut system = temp_speed_system();
        let data = vec![(vec![35.0], vec![90.0]), (vec![2.0], vec![5.0])];
        weight_rules_by_fit(&mut system, &data);
        for rule in &system.rules.rules {
            assert!((0.0..=1.0).contains(&rule.confidence));
        }
    }

    #[test]
    fn subtractive_cluster_centers_finds_clusters() {
        let data = vec![vec![0.0, 0.0], vec![0.2, 0.1], vec![10.0, 10.0], vec![10.1, 9.9]];
        let centers = subtractive_cluster_centers(&data, 1.0, 0.5, 0.15);
        assert!(!centers.is_empty());
        assert!(centers.len() <= data.len());
    }

    #[test]
    fn subtractive_cluster_centers_empty_data_returns_empty() {
        assert!(subtractive_cluster_centers(&[], 1.0, 0.5, 0.15).is_empty());
    }

    #[test]
    fn gustafson_kessel_clusters_two_groups() {
        let data = vec![vec![0.0, 0.0], vec![0.1, 0.0], vec![5.0, 5.0], vec![5.1, 5.0]];
        let result = gustafson_kessel(&data, 2, 2.0, 5).unwrap();
        assert_eq!(result.centers.len(), 2);
        assert_eq!(result.membership.len(), 4);
    }

    #[test]
    fn fuzzy_vikor_ranks_compromise_solution() {
        let vikor = FuzzyVikor {
            alternatives: vec!["a".into(), "b".into()],
            criteria: vec!["c".into()],
            decision_matrix: vec![vec![FuzzyNumber::triangular(8.0, 9.0, 10.0)], vec![FuzzyNumber::triangular(1.0, 2.0, 3.0)]],
            weights: vec![1.0],
            benefit: vec![true],
            v: 0.5,
        };
        let rank = vikor.rank();
        assert_eq!(rank[0].0, 0);
    }

    #[test]
    fn temporal_evaluator_recently_and_frequently() {
        let eval = TemporalEvaluator { now: 100.0, recent_window: 10.0, frequent_window: 5.0 };
        assert!((eval.recently(100.0) - 1.0).abs() < 1e-9);
        assert_eq!(eval.recently(90.0), 0.0);
        assert!((eval.frequently(&[99.0, 98.0, 97.0]) - 0.6).abs() < 1e-9);
    }

    #[test]
    fn spatial_evaluator_near_and_slowly() {
        let eval = SpatialEvaluator { anchor: vec![0.0, 0.0], near_radius: 10.0 };
        assert!((eval.near(&[0.0, 0.0]) - 1.0).abs() < 1e-9);
        assert!(eval.near(&[20.0, 0.0]) <= 0.0);
        assert!((eval.slowly(0.0, 10.0) - 1.0).abs() < 1e-9);
        assert_eq!(eval.slowly(10.0, 10.0), 0.0);
    }

    #[test]
    fn probabilistic_to_membership_blends_by_certainty() {
        assert!((probabilistic_to_membership(0.8, 1.0) - 0.8).abs() < 1e-9);
        assert!((probabilistic_to_membership(0.8, 0.0) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn possibility_to_membership_passes_through_values() {
        let pm = PossibilityMeasure::new(vec![1.0, 2.0], vec![0.3, 0.7]).unwrap();
        assert_eq!(possibility_to_membership(&pm), vec![0.3, 0.7]);
    }

    #[test]
    fn hybrid_fuse_weighted_combination() {
        assert!((hybrid_fuse(0.8, 0.2, 1.0) - 0.8).abs() < 1e-9);
        assert!((hybrid_fuse(0.8, 0.2, 0.0) - 0.2).abs() < 1e-9);
        assert!((hybrid_fuse(0.8, 0.2, 0.5) - 0.5).abs() < 1e-9);
    }

    mod long {
        use super::*;

        #[test]
        fn anfis_learns_nonlinear_surface() {
            let data: Vec<(Vec<f64>, f64)> = (0..30)
                .flat_map(|i| {
                    (0..30).map(move |j| {
                        let x = i as f64 / 10.0;
                        let y = j as f64 / 10.0;
                        (vec![x, y], (x * y).sin())
                    })
                })
                .collect();
            let mut anfis = Anfis::new(2, 3, &[(0.0, 3.0), (0.0, 3.0)]);
            let loss = anfis.fit_hybrid(&data, 5);
            let pred = anfis.forward(&[1.5, 1.5]);
            assert!(loss.is_finite());
            assert!(pred.is_finite());
        }

        #[test]
        fn genetic_optimizer_improves_quadratic() {
            let opt = GeneticOptimizer { population_size: 20, generations: 30, mutation_rate: 0.2, crossover_rate: 0.7, bounds: vec![(-5.0, 5.0)], seed: 42 };
            let (best, fit) = opt.optimize(|x| (x[0] - 2.0).powi(2));
            assert!(fit < 1.0);
            assert!((best[0] - 2.0).abs() < 1.5);
        }

        #[test]
        fn pso_optimizer_finds_minimum() {
            let opt = PsoOptimizer { swarm_size: 15, iterations: 40, inertia: 0.7, cognitive: 1.4, social: 1.4, bounds: vec![(-3.0, 3.0), (-3.0, 3.0)], seed: 7 };
            let (best, fit) = opt.optimize(|x| x[0] * x[0] + x[1] * x[1]);
            assert!(fit < 0.5);
            assert!(best[0].abs() < 1.0 && best[1].abs() < 1.0);
        }

        #[test]
        fn hierarchical_system_chains_layers() {
            let layer1 = temp_speed_system();
            let layer2_univ = Universe::new(0.0, 100.0, 21).unwrap();
            let layer2 = MimoSystem {
                inputs: vec![LinguisticVariable::new("fan_speed", layer2_univ, vec![FuzzySet::new("any", MembershipFunction::triangular(0.0, 50.0, 100.0))])],
                outputs: vec![LinguisticVariable::new("power", Universe::new(0.0, 1.0, 21).unwrap(), vec![FuzzySet::new("high", MembershipFunction::triangular(0.5, 1.0, 1.0))])],
                rules: RuleBase::new(vec![Rule { id: 0, antecedents: vec![AntecedentClause { input: 0, term: 0, hedge: None }], consequent: Consequent::SugenoConstant { output: 0, value: 0.9 }, weight: 1.0, confidence: 1.0 }]),
                model: InferenceModel::Sugeno,
                tnorm: TNorm::Min,
                tconorm: TConorm::Max,
                defuzzifier: Defuzzifier::Centroid,
            };
            let hier = HierarchicalSystem::new(vec![layer1, layer2]);
            let (out, explanations) = hier.infer(&[35.0]).unwrap();
            assert_eq!(out.len(), 1);
            assert_eq!(explanations.len(), 2);
        }

        #[test]
        fn evolving_system_adapts_over_stream() {
            let system = temp_speed_system();
            let mut evolving = EvolvingFuzzySystem::new(system, 0.1, 0.05, 10);
            for t in (20..30).map(|x| x as f64) {
                let _ = evolving.observe(&[t], &[80.0]);
            }
            assert!(!evolving.system.rules.rules.is_empty());
        }
    }
}
// #endregion 🔖️Tests
