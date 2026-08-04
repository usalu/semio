//! ⚖️ Pattern weight storage: precomputed `w`, `ln w`, `w·ln w` per pattern (the three terms the
//! incremental Shannon-entropy heuristic needs at O(1) per update), plus an optional exact-integer
//! parallel table for [`WeightMode::StrictInteger`] determinism.

use crate::error::ModelError;
use crate::ids::PatternId;

// #region 🔖️Mode
/// ⚖️ Whether heuristics/sampling read `f64` weights (fast, platform-stable but not
/// refactor-proof) or exact `u64` weights (slower, bit-for-bit reproducible everywhere — used by
/// every differential/golden-replay test in this crate).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum WeightMode {
    #[default]
    Real,
    StrictInteger,
}

/// ⚖️ What a weight of exactly zero means for sampling and pruning.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ZeroWeightPolicy {
    /// ⚖️ Zero weight is a validation error at compile time.
    Reject,
    /// ⚖️ Zero-weight patterns stay in the domain (may be forced) but are never sampled unless
    /// they are the only remaining candidate.
    #[default]
    SampleNeverUnlessForced,
    /// ⚖️ Zero weight is treated exactly like an explicit deny — the pattern is compiled out.
    Forbidden,
}
// #endregion 🔖️Mode

// #region 🔖️Weights
/// ⚖️ Per-pattern weight table with precomputed entropy terms.
#[derive(Clone, Debug)]
pub struct WeightTable {
    w: Vec<f64>,
    ln_w: Vec<f64>,
    w_ln_w: Vec<f64>,
    w_int: Option<Vec<u64>>,
}

impl WeightTable {
    /// ⚖️ Builds a table from raw positive-finite weights. `w_int` is populated only when every
    /// weight is already an exact non-negative integer value (the common case for hand-authored
    /// tilesets and frequency-counted extraction).
    pub fn new(weights: &[f64]) -> Result<Self, ModelError> {
        let mut w = Vec::with_capacity(weights.len());
        let mut ln_w = Vec::with_capacity(weights.len());
        let mut w_ln_w = Vec::with_capacity(weights.len());
        let mut all_integral = true;
        let mut w_int = Vec::with_capacity(weights.len());
        for (i, &value) in weights.iter().enumerate() {
            if !value.is_finite() || value < 0.0 {
                return Err(ModelError::InvalidWeight { pattern_index: i, value });
            }
            w.push(value);
            let lnv = if value > 0.0 { value.ln() } else { 0.0 };
            ln_w.push(lnv);
            w_ln_w.push(if value > 0.0 { value * lnv } else { 0.0 });
            if value.fract() == 0.0 && value >= 0.0 && value <= u64::MAX as f64 {
                w_int.push(value as u64);
            } else {
                all_integral = false;
            }
        }
        Ok(Self { w, ln_w, w_ln_w, w_int: if all_integral { Some(w_int) } else { None } })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.w.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.w.is_empty()
    }

    #[inline]
    pub fn w(&self, p: PatternId) -> f64 {
        self.w[p.index()]
    }

    #[inline]
    pub fn ln_w(&self, p: PatternId) -> f64 {
        self.ln_w[p.index()]
    }

    #[inline]
    pub fn w_ln_w(&self, p: PatternId) -> f64 {
        self.w_ln_w[p.index()]
    }

    /// ⚖️ Exact integer weight, when [`WeightTable::has_integer_weights`] is `true`.
    #[inline]
    pub fn w_int(&self, p: PatternId) -> Option<u64> {
        self.w_int.as_ref().map(|v| v[p.index()])
    }

    pub fn has_integer_weights(&self) -> bool {
        self.w_int.is_some()
    }

    /// ⚖️ `(sum_w, sum_w_ln_w)` restricted to the patterns present in `set`. O(domain size); used
    /// only to rebuild caches from scratch (initialization, periodic drift correction, debug
    /// verification) — never on the hot incremental path.
    pub fn sum_over(&self, set: &crate::bitset::PatternSet) -> (f64, f64) {
        let mut sum_w = 0.0;
        let mut sum_w_ln_w = 0.0;
        for p in set.iter_ones() {
            sum_w += self.w(p);
            sum_w_ln_w += self.w_ln_w(p);
        }
        (sum_w, sum_w_ln_w)
    }

    /// ⚖️ Exact-integer analogue of [`WeightTable::sum_over`], `None` if this table lacks integer
    /// weights.
    pub fn sum_int_over(&self, set: &crate::bitset::PatternSet) -> Option<u64> {
        self.w_int.as_ref().map(|w_int| set.iter_ones().map(|p| w_int[p.index()]).sum())
    }
}
// #endregion 🔖️Weights

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitset::PatternSet;

    #[test]
    fn rejects_negative_and_nonfinite() {
        assert!(WeightTable::new(&[1.0, -1.0]).is_err());
        assert!(WeightTable::new(&[1.0, f64::NAN]).is_err());
        assert!(WeightTable::new(&[1.0, f64::INFINITY]).is_err());
    }

    #[test]
    fn precomputes_terms() {
        let t = WeightTable::new(&[1.0, core::f64::consts::E]).unwrap();
        assert_eq!(t.w(PatternId(0)), 1.0);
        assert!((t.ln_w(PatternId(1)) - 1.0).abs() < 1e-12);
        assert!((t.w_ln_w(PatternId(1)) - core::f64::consts::E).abs() < 1e-9);
    }

    #[test]
    fn zero_weight_terms_are_zero_not_nan() {
        let t = WeightTable::new(&[0.0, 2.0]).unwrap();
        assert_eq!(t.ln_w(PatternId(0)), 0.0);
        assert_eq!(t.w_ln_w(PatternId(0)), 0.0);
    }

    #[test]
    fn integer_detection() {
        let t = WeightTable::new(&[1.0, 3.0, 5.0]).unwrap();
        assert!(t.has_integer_weights());
        assert_eq!(t.w_int(PatternId(1)), Some(3));

        let f = WeightTable::new(&[1.5, 2.0]).unwrap();
        assert!(!f.has_integer_weights());
    }

    #[test]
    fn sum_over_matches_manual() {
        let t = WeightTable::new(&[1.0, 2.0, 4.0]).unwrap();
        let mut set = PatternSet::new_empty(3);
        set.set(PatternId(0), true);
        set.set(PatternId(2), true);
        let (sw, swlw) = t.sum_over(&set);
        assert_eq!(sw, 5.0);
        assert!((swlw - (0.0 + 4.0 * 4.0f64.ln())).abs() < 1e-12);
        assert_eq!(t.sum_int_over(&set), Some(5));
    }
}
// #endregion 🔖️Tests
