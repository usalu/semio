//! 🔤️ Symbolization front door: maps continuous/discrete time series into finite alphabets via
//! time-delay embedding, ordinal (permutation) patterns, dispersion patterns, empirical quantile
//! binning, and fixed thresholds. Every [`Symbolizer`] implementation here feeds downstream
//! plug-in entropy estimators (`discrete.rs`, `ordinal.rs`, `regularity.rs`) a `Vec<u32>` of
//! symbol codes plus a declared [`Symbolizer::alphabet_size`].

use crate::schema::entropy_internals::numeric::{checked_state_count, neumaier_sum, normal_cdf};
use crate::schema::entropy_internals::{EntropyError, TiePolicy};

// #region 🔖️Embedding
/// 🔤️ Time-delay (Takens) embedding: state vector `i` is `[x[i], x[i+tau], ..., x[i+(dim-1)*tau]]`
/// for `i` in `0..=(x.len() - (dim-1)*tau - 1)`. The foundational primitive behind every
/// window-based symbolizer in this module (ordinal and dispersion patterns) as well as
/// phase-space reconstruction used elsewhere in the crate.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn embed(x: &[f64], dim: usize, tau: usize) -> Result<Vec<Vec<f64>>, EntropyError> {
    if dim < 1 {
        return Err(EntropyError::InvalidConfig { field: "dim", reason: "embedding dimension must be at least 1" });
    }
    if tau < 1 {
        return Err(EntropyError::InvalidConfig { field: "tau", reason: "embedding delay must be at least 1" });
    }
    let span = (dim - 1) * tau;
    let needed = span + 1;
    if x.len() < needed {
        return Err(EntropyError::InsufficientData { what: "time-delay embedding", needed, actual: x.len() });
    }
    let n_windows = x.len() - span;
    let mut out = Vec::with_capacity(n_windows);
    for i in 0..n_windows {
        let mut state = Vec::with_capacity(dim);
        for k in 0..dim {
            state.push(x[i + k * tau]);
        }
        out.push(state);
    }
    Ok(out)
}
// #endregion 🔖️Embedding

// #region 🔖️Trait
/// 🔤️ Maps a real-valued time series to a finite alphabet of symbols (`u32` codes, each
/// guaranteed to lie in `0..alphabet_size()`) — the common front door every symbolic/permutation
/// entropy estimator builds on.
pub trait Symbolizer {
    /// 🔤️ Encodes `x` into a sequence of symbol codes. Implementations that embed a window (e.g.
    /// [`OrdinalSymbolizer`], [`DispersionSymbolizer`]) emit fewer codes than `x.len()`.
    fn symbolize(&self, x: &[f64]) -> Result<Vec<u32>, EntropyError>;
    /// 🔤️ The size of the alphabet this symbolizer emits codes into.
    fn alphabet_size(&self) -> usize;
}
// #endregion 🔖️Trait

// #region 🔖️Ordinal
/// 🔤️ Saturating factorial (`n!` for small `n`, saturates to `u64::MAX` rather than wrapping or
/// panicking for `n` large enough to overflow — realistic ordinal-pattern dimensions are `<= 8`).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn factorial(n: usize) -> u64 {
    let mut acc: u64 = 1;
    for k in 2..=n as u64 {
        acc = acc.saturating_mul(k);
    }
    acc
}

/// 🔤️ Encodes one embedded window as an ordinal-pattern symbol in `0..dim!` via the standard
/// factorial number system: rank the window's values ascending (stable, so ties resolve by
/// original in-window index order), then Lehmer-code that permutation of original indices into a
/// single integer. `TiePolicy::Error` rejects any exact tie up front; `StableRank` and
/// `Jitterless` are treated identically here (both fall through to the stable-sort tie-break) —
/// a documented simplification for this first implementation, since neither adds real jitter.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn ordinal_pattern_symbol(window: &[f64], ties: TiePolicy) -> Result<u32, EntropyError> {
    let dim = window.len();
    if ties == TiePolicy::Error {
        for i in 0..dim {
            for j in (i + 1)..dim {
                if window[i] == window[j] {
                    return Err(EntropyError::DegenerateInput { what: "tied values in ordinal pattern" });
                }
            }
        }
    }
    let mut order: Vec<usize> = (0..dim).collect();
    order.sort_by(|&a, &b| window[a].total_cmp(&window[b]));
    let mut used = vec![false; dim];
    let mut rank: u64 = 0;
    for (j, &pos) in order.iter().enumerate() {
        let smaller_remaining = (0..pos).filter(|&k| !used[k]).count() as u64;
        rank += smaller_remaining * factorial(dim - 1 - j);
        used[pos] = true;
    }
    Ok(rank as u32)
}

/// 🔤️ Configuration for [`OrdinalSymbolizer`]: embedding dimension, embedding delay, and the tie
/// policy applied within each window.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct OrdinalConfig {
    pub dim: usize,
    pub tau: usize,
    pub ties: TiePolicy,
}

impl OrdinalConfig {
    /// 🔤️ Validated constructor: `dim >= 2` (a single-value "pattern" carries no order
    /// information) and `tau >= 1`. Defaults `ties` to [`TiePolicy::StableRank`].
    pub fn new(dim: usize, tau: usize) -> Result<Self, EntropyError> {
        if dim < 2 {
            return Err(EntropyError::InvalidConfig { field: "dim", reason: "ordinal patterns need dim >= 2 to carry order information" });
        }
        if tau < 1 {
            return Err(EntropyError::InvalidConfig { field: "tau", reason: "embedding delay must be at least 1" });
        }
        Ok(Self { dim, tau, ties: TiePolicy::StableRank })
    }

    /// 🔤️ Consuming setter for the tie policy.
    pub fn with_ties(mut self, ties: TiePolicy) -> Self {
        self.ties = ties;
        self
    }
}

impl Default for OrdinalConfig {
    /// 🔤️ The literature-default embedding: `dim = 3`, `tau = 1`.
    fn default() -> Self {
        Self { dim: 3, tau: 1, ties: TiePolicy::StableRank }
    }
}

/// 🔤️ Symbolizes a series into ordinal (Bandt-Pompe permutation) patterns: each embedded window
/// is encoded as the rank of its permutation among the `dim!` possible orderings.
pub struct OrdinalSymbolizer {
    cfg: OrdinalConfig,
}

impl OrdinalSymbolizer {
    /// 🔤️ Wraps an already-validated [`OrdinalConfig`].
    pub fn new(cfg: OrdinalConfig) -> Self {
        Self { cfg }
    }
}

impl Symbolizer for OrdinalSymbolizer {
    fn symbolize(&self, x: &[f64]) -> Result<Vec<u32>, EntropyError> {
        let windows = embed(x, self.cfg.dim, self.cfg.tau)?;
        windows.iter().map(|w| ordinal_pattern_symbol(w, self.cfg.ties)).collect()
    }

    fn alphabet_size(&self) -> usize {
        factorial(self.cfg.dim) as usize
    }
}
// #endregion 🔖️Ordinal

// #region 🔖️Dispersion
/// 🔤️ Symbolizes a series into dispersion patterns (Rostaghi-Azami): each raw value is mapped to
/// one of `classes` classes via the normal-CDF-based normalization used by NCDF dispersion
/// entropy, then embedded windows of classes are packed into one joint symbol.
pub struct DispersionSymbolizer {
    pub classes: usize,
    pub dim: usize,
    pub tau: usize,
}

impl DispersionSymbolizer {
    /// 🔤️ Validated constructor: `classes >= 2`, `dim >= 1`, `tau >= 1`, and `classes^dim` must
    /// fit within the `u32` symbol codomain that [`Symbolizer::symbolize`] returns (checked via
    /// [`checked_state_count`], which itself guards `usize`/`u128` overflow of the raw product).
    pub fn new(classes: usize, dim: usize, tau: usize) -> Result<Self, EntropyError> {
        if classes < 2 {
            return Err(EntropyError::InvalidConfig { field: "classes", reason: "dispersion needs at least 2 classes" });
        }
        if dim < 1 {
            return Err(EntropyError::InvalidConfig { field: "dim", reason: "embedding dimension must be at least 1" });
        }
        if tau < 1 {
            return Err(EntropyError::InvalidConfig { field: "tau", reason: "embedding delay must be at least 1" });
        }
        let dims = vec![classes; dim];
        let fits = matches!(checked_state_count(&dims), Some(count) if count <= u32::MAX as u128);
        if !fits {
            return Err(EntropyError::InvalidConfig { field: "classes/dim", reason: "classes^dim must fit within the u32 symbol range" });
        }
        Ok(Self { classes, dim, tau })
    }
}

impl Symbolizer for DispersionSymbolizer {
    fn symbolize(&self, x: &[f64]) -> Result<Vec<u32>, EntropyError> {
        if x.is_empty() {
            return Err(EntropyError::EmptyInput { what: "x" });
        }
        let n = x.len() as f64;
        let mean = neumaier_sum(x.iter().copied()) / n;
        let variance = neumaier_sum(x.iter().map(|&v| (v - mean).powi(2))) / n;
        let sd = variance.sqrt();
        if sd <= 0.0 {
            return Err(EntropyError::DegenerateInput { what: "constant series has no dispersion classes" });
        }
        let classes_f = self.classes as f64;
        let classed: Vec<f64> = x
            .iter()
            .map(|&v| {
                let c = (classes_f * normal_cdf((v - mean) / sd) - 0.5).round();
                c.clamp(0.0, classes_f - 1.0)
            })
            .collect();
        let windows = embed(&classed, self.dim, self.tau)?;
        let mut out = Vec::with_capacity(windows.len());
        for w in &windows {
            let mut symbol: usize = 0;
            for &v in w {
                symbol = symbol * self.classes + v as usize;
            }
            out.push(symbol as u32);
        }
        Ok(out)
    }

    fn alphabet_size(&self) -> usize {
        let dims = vec![self.classes; self.dim];
        checked_state_count(&dims).map_or(usize::MAX, |c| c as usize)
    }
}
// #endregion 🔖️Dispersion

// #region 🔖️Quantile
/// 🔤️ Symbolizes a series into empirical-quantile bins: bin edges are the series' own `bins - 1`
/// interior quantile breakpoints, so every bin holds (up to rounding) an equal share of samples.
pub struct QuantileSymbolizer {
    pub bins: usize,
}

impl QuantileSymbolizer {
    /// 🔤️ Validated constructor: `bins >= 2`.
    pub fn new(bins: usize) -> Result<Self, EntropyError> {
        if bins < 2 {
            return Err(EntropyError::InvalidConfig { field: "bins", reason: "must be at least 2" });
        }
        Ok(Self { bins })
    }
}

impl Symbolizer for QuantileSymbolizer {
    fn symbolize(&self, x: &[f64]) -> Result<Vec<u32>, EntropyError> {
        if x.is_empty() {
            return Err(EntropyError::EmptyInput { what: "x" });
        }
        let mut sorted = x.to_vec();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let n = sorted.len();
        let mut edges = Vec::with_capacity(self.bins - 1);
        for i in 1..self.bins {
            let frac = i as f64 / self.bins as f64;
            let idx = ((frac * n as f64) as usize).min(n - 1);
            edges.push(sorted[idx]);
        }
        Ok(x.iter().map(|&v| edges.partition_point(|&e| e <= v).min(self.bins - 1) as u32).collect())
    }

    fn alphabet_size(&self) -> usize {
        self.bins
    }
}
// #endregion 🔖️Quantile

// #region 🔖️Threshold
/// 🔤️ Symbolizes a series against a fixed, caller-supplied set of ascending threshold edges: the
/// class of `x_i` is the count of `edges` that are `<= x_i`, so the alphabet size is
/// `edges.len() + 1`.
pub struct ThresholdSymbolizer {
    pub edges: Vec<f64>,
}

impl ThresholdSymbolizer {
    /// 🔤️ Validated constructor: `edges` must be non-empty, every entry finite, and the sequence
    /// strictly ascending. Never silently sorts — an out-of-order `edges` is a caller bug.
    pub fn new(edges: Vec<f64>) -> Result<Self, EntropyError> {
        if edges.is_empty() {
            return Err(EntropyError::InvalidConfig { field: "edges", reason: "must not be empty" });
        }
        if !edges.iter().all(|e| e.is_finite()) {
            return Err(EntropyError::InvalidConfig { field: "edges", reason: "all edges must be finite" });
        }
        if !edges.windows(2).all(|w| w[0] < w[1]) {
            return Err(EntropyError::InvalidConfig { field: "edges", reason: "edges must be sorted strictly ascending" });
        }
        Ok(Self { edges })
    }
}

impl Symbolizer for ThresholdSymbolizer {
    fn symbolize(&self, x: &[f64]) -> Result<Vec<u32>, EntropyError> {
        if x.is_empty() {
            return Err(EntropyError::EmptyInput { what: "x" });
        }
        Ok(x.iter().map(|&v| self.edges.partition_point(|&e| e <= v) as u32).collect())
    }

    fn alphabet_size(&self) -> usize {
        self.edges.len() + 1
    }
}
// #endregion 🔖️Threshold

// #region 🔖️Tests
#[cfg(test)]
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
