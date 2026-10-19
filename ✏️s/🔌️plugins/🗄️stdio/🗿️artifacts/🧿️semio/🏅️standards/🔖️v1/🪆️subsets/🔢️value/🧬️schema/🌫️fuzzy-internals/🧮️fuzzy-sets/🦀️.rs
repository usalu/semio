//! 🧮️ Fuzzy sets: the type-1 fuzzy set, the interval type-2 set and the intuitionistic set. See [`super`].

use super::*;

// #region 🔖️FuzzySet
/// 🌫️ Type-1 fuzzy set: linguistic label plus membership function.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
