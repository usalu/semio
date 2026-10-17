//! 📈️ Membership functions: the membership function enum and its evaluation, shaping and conversion methods. See [`super`].

use super::*;

// #region 🔖️MembershipFunction
/// 📐️ Parametric membership function with eval, parameter access, and gradient hooks for adaptation.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn triangular(a: f64, b: f64, c: f64) -> Self {
        Self::Triangular { a, b, c }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn trapezoidal(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self::Trapezoidal { a, b, c, d }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn gaussian(mean: f64, sigma: f64) -> Self {
        Self::Gaussian { mean, sigma: sigma.max(1e-12) }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn generalized_bell(a: f64, b: f64, c: f64) -> Self {
        Self::GeneralizedBell { a: a.max(1e-12), b: b.max(1e-6), c }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sigmoid(a: f64, c: f64) -> Self {
        Self::Sigmoid { a, c }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn singleton(value: f64) -> Self {
        Self::Singleton { value }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn piecewise_linear(knots: Vec<(f64, f64)>) -> Self {
        Self::PiecewiseLinear { knots }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
