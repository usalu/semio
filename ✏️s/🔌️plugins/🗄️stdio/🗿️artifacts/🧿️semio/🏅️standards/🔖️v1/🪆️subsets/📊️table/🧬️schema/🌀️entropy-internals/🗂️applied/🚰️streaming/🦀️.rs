//! 🌊️ Online/streaming entropy estimation: a mergeable `StreamingEstimator` trait plus exact
//! incremental counts, a fixed sliding window, and exponentially decayed counts.

use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::x_ln_x;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::ConfidenceInterval;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::EntropyError;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Estimate;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::LogBase;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Warning;
use std::collections::VecDeque;

// #region 🔖️Trait
/// 🌊️ Mergeable online state: `update` folds one observation in, `remove` undoes one (where
/// supported), `merge` combines two independently accumulated states, `estimate` reports the
/// current entropy, `reset` clears all state, and `snapshot`/`restore` round-trip the state
/// through a plain-data representation (no serde — see [`StreamingSnapshot`]).
pub trait StreamingEstimator {
    type Item;
    fn update(&mut self, x: Self::Item);
    fn remove(&mut self, x: Self::Item) -> Result<(), EntropyError>;
    fn merge(&mut self, other: &Self) -> Result<(), EntropyError>;
    fn estimate(&self) -> Result<Estimate, EntropyError>;
    fn reset(&mut self);
    fn snapshot(&self) -> StreamingSnapshot;
    fn restore(snapshot: &StreamingSnapshot) -> Result<Self, EntropyError>
    where
        Self: Sized;
}

/// 🌊️ Plain-data snapshot of a streaming estimator's internal counts, used for `snapshot`/
/// `restore` round-tripping without depending on an external serialization crate.
#[derive(Clone, PartialEq, Debug)]
pub struct StreamingSnapshot {
    pub counts: Vec<f64>,
    pub alphabet_size: usize,
    pub base: LogBase,
    pub method: &'static str,
    pub extra: Vec<f64>,
}
// #endregion 🔖️Trait

// #region 🔖️Shared
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn plugin_entropy_from_counts(counts: &[f64], base: LogBase, method: &'static str, n_raw: usize) -> Estimate {
    let total: f64 = counts.iter().sum();
    let nats = if total > 0.0 { -counts.iter().map(|&c| x_ln_x(c / total)).sum::<f64>() } else { 0.0 };
    let mut warnings = Vec::new();
    let occupied = counts.iter().filter(|&&c| c > 0.0).count();
    if occupied * 2 < counts.len() {
        warnings.push(Warning::Undersampled { occupied_bins: occupied, total_bins: counts.len() });
    }
    Estimate { value: base.from_nats(nats), base, method, n: n_raw, n_effective: total, std_error: None, ci: None::<ConfidenceInterval>, warnings, diagnostics: vec![("alphabet_size", counts.len() as f64), ("total_weight", total)] }
}
// #endregion 🔖️Shared

// #region 🔖️StreamingCounts
/// 🌊️ Exact incremental symbol counts over a fixed `0..alphabet_size` alphabet.
pub struct StreamingCounts {
    counts: Vec<f64>,
    base: LogBase,
    n_raw: usize,
}

impl StreamingCounts {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(alphabet_size: usize, base: LogBase) -> Self {
        Self { counts: vec![0.0; alphabet_size], base, n_raw: 0 }
    }
}

impl StreamingEstimator for StreamingCounts {
    type Item = u32;

    fn update(&mut self, x: u32) {
        if (x as usize) < self.counts.len() {
            self.counts[x as usize] += 1.0;
            self.n_raw += 1;
        }
    }

    fn remove(&mut self, x: u32) -> Result<(), EntropyError> {
        let idx = x as usize;
        if idx >= self.counts.len() || self.counts[idx] < 1.0 {
            return Err(EntropyError::InvalidConfig { field: "x", reason: "no observation of this symbol to remove" });
        }
        self.counts[idx] -= 1.0;
        self.n_raw -= 1;
        Ok(())
    }

    fn merge(&mut self, other: &Self) -> Result<(), EntropyError> {
        if self.counts.len() != other.counts.len() {
            return Err(EntropyError::LengthMismatch { expected: self.counts.len(), actual: other.counts.len() });
        }
        for (a, &b) in self.counts.iter_mut().zip(other.counts.iter()) {
            *a += b;
        }
        self.n_raw += other.n_raw;
        Ok(())
    }

    fn estimate(&self) -> Result<Estimate, EntropyError> {
        if self.n_raw == 0 {
            return Err(EntropyError::EmptyInput { what: "streaming counts" });
        }
        Ok(plugin_entropy_from_counts(&self.counts, self.base, "streaming_counts", self.n_raw))
    }

    fn reset(&mut self) {
        self.counts.iter_mut().for_each(|c| *c = 0.0);
        self.n_raw = 0;
    }

    fn snapshot(&self) -> StreamingSnapshot {
        StreamingSnapshot { counts: self.counts.clone(), alphabet_size: self.counts.len(), base: self.base, method: "streaming_counts", extra: vec![self.n_raw as f64] }
    }

    fn restore(snapshot: &StreamingSnapshot) -> Result<Self, EntropyError> {
        Ok(Self { counts: snapshot.counts.clone(), base: snapshot.base, n_raw: snapshot.extra.first().copied().unwrap_or(0.0) as usize })
    }
}
// #endregion 🔖️StreamingCounts

// #region 🔖️SlidingWindow
/// 🌊️ Entropy over the most recent `capacity` observations only (older ones are evicted exactly,
/// via [`StreamingCounts::remove`]).
pub struct SlidingWindowEntropy {
    window: VecDeque<u32>,
    capacity: usize,
    counts: StreamingCounts,
}

impl SlidingWindowEntropy {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(alphabet_size: usize, capacity: usize, base: LogBase) -> Result<Self, EntropyError> {
        if capacity == 0 {
            return Err(EntropyError::InvalidConfig { field: "capacity", reason: "must be at least 1" });
        }
        Ok(Self { window: VecDeque::with_capacity(capacity), capacity, counts: StreamingCounts::new(alphabet_size, base) })
    }
}

impl StreamingEstimator for SlidingWindowEntropy {
    type Item = u32;

    fn update(&mut self, x: u32) {
        self.counts.update(x);
        self.window.push_back(x);
        if self.window.len() > self.capacity {
            if let Some(evicted) = self.window.pop_front() {
                let _ = self.counts.remove(evicted);
            }
        }
    }

    fn remove(&mut self, _x: u32) -> Result<(), EntropyError> {
        Err(EntropyError::InvalidConfig { field: "remove", reason: "SlidingWindowEntropy evicts automatically; explicit remove is unsupported" })
    }

    fn merge(&mut self, _other: &Self) -> Result<(), EntropyError> {
        Err(EntropyError::InvalidConfig { field: "merge", reason: "SlidingWindowEntropy carries order-dependent state and cannot be merged" })
    }

    fn estimate(&self) -> Result<Estimate, EntropyError> {
        self.counts.estimate()
    }

    fn reset(&mut self) {
        self.window.clear();
        self.counts.reset();
    }

    fn snapshot(&self) -> StreamingSnapshot {
        let mut snap = self.counts.snapshot();
        snap.method = "sliding_window_entropy";
        snap.extra.push(self.capacity as f64);
        for &v in &self.window {
            snap.extra.push(v as f64);
        }
        snap
    }

    fn restore(snapshot: &StreamingSnapshot) -> Result<Self, EntropyError> {
        if snapshot.extra.len() < 2 {
            return Err(EntropyError::InvalidConfig { field: "snapshot", reason: "missing capacity/window data" });
        }
        let capacity = snapshot.extra[1] as usize;
        let window: VecDeque<u32> = snapshot.extra[2..].iter().map(|&v| v as u32).collect();
        let counts = StreamingCounts { counts: snapshot.counts.clone(), base: snapshot.base, n_raw: snapshot.extra[0] as usize };
        Ok(Self { window, capacity, counts })
    }
}
// #endregion 🔖️SlidingWindow

// #region 🔖️Decayed
/// 🌊️ Exponentially forgetting counts: each `update` first multiplies every count by `decay` (in
/// `(0, 1]`) before incrementing the observed symbol, so older observations fade geometrically.
/// `remove` is semantically unsupported (there is no well-defined inverse of decay) and always
/// errors.
pub struct DecayedEntropy {
    counts: Vec<f64>,
    decay: f64,
    base: LogBase,
}

impl DecayedEntropy {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(alphabet_size: usize, decay: f64, base: LogBase) -> Result<Self, EntropyError> {
        if !(0.0 < decay && decay <= 1.0) {
            return Err(EntropyError::InvalidConfig { field: "decay", reason: "must be in (0, 1]" });
        }
        Ok(Self { counts: vec![0.0; alphabet_size], decay, base })
    }
}

impl StreamingEstimator for DecayedEntropy {
    type Item = u32;

    fn update(&mut self, x: u32) {
        for c in &mut self.counts {
            *c *= self.decay;
        }
        if (x as usize) < self.counts.len() {
            self.counts[x as usize] += 1.0;
        }
    }

    fn remove(&mut self, _x: u32) -> Result<(), EntropyError> {
        Err(EntropyError::InvalidConfig { field: "remove", reason: "DecayedEntropy has no well-defined inverse of exponential decay" })
    }

    fn merge(&mut self, other: &Self) -> Result<(), EntropyError> {
        if self.counts.len() != other.counts.len() {
            return Err(EntropyError::LengthMismatch { expected: self.counts.len(), actual: other.counts.len() });
        }
        for (a, &b) in self.counts.iter_mut().zip(other.counts.iter()) {
            *a += b;
        }
        Ok(())
    }

    fn estimate(&self) -> Result<Estimate, EntropyError> {
        let total: f64 = self.counts.iter().sum();
        if total <= 0.0 {
            return Err(EntropyError::EmptyInput { what: "decayed counts" });
        }
        Ok(plugin_entropy_from_counts(&self.counts, self.base, "decayed_entropy", self.counts.len()))
    }

    fn reset(&mut self) {
        self.counts.iter_mut().for_each(|c| *c = 0.0);
    }

    fn snapshot(&self) -> StreamingSnapshot {
        StreamingSnapshot { counts: self.counts.clone(), alphabet_size: self.counts.len(), base: self.base, method: "decayed_entropy", extra: vec![self.decay] }
    }

    fn restore(snapshot: &StreamingSnapshot) -> Result<Self, EntropyError> {
        let decay = snapshot.extra.first().copied().unwrap_or(1.0);
        Ok(Self { counts: snapshot.counts.clone(), decay, base: snapshot.base })
    }
}
// #endregion 🔖️Decayed

// #region 🔖️Tests
#[cfg(test)]
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
