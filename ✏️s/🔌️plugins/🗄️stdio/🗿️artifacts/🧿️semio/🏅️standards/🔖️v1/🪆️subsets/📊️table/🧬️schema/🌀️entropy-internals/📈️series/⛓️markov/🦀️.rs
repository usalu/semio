//! ⛓️ Fitted, stateful Markov-chain estimation: transition counts and conditional distributions
//! per context, the stationary distribution over contexts via power iteration, and the resulting
//! entropy rate. All internal computation happens in nats; [`LogBase`] conversion is applied only
//! at the [`MarkovChain::entropy_rate`] boundary.

use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::checked_state_count;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::neumaier_sum;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::numeric::x_ln_x;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::EntropyError;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Estimate;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::LogBase;

// #region 🔖️Context
/// ⛓️ Packs a window of `order` consecutive symbols into a single mixed-radix context id in
/// `0..alphabet_size^order`. The oldest symbol in `window` is the most significant digit, so
/// dropping it and appending a new symbol (advancing the chain by one step) is a cheap
/// `(context % (alphabet_size^(order-1))) * alphabet_size + next` update — see
/// [`MarkovChain::stationary`].
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn pack_context(window: &[u32], alphabet_size: usize) -> usize {
    window.iter().fold(0usize, |acc, &symbol| acc * alphabet_size + symbol as usize)
}
// #endregion 🔖️Context

// #region 🔖️MarkovChain
/// ⛓️ A fitted order-`order` Markov chain over an alphabet of size `alphabet_size`. Stores raw
/// per-context transition counts and their row-normalized conditional distributions
/// `P(next | context)`. A context with zero observed transitions is treated, for the purposes of
/// [`MarkovChain::stationary`], as an absorbing self-loop (probability 1 of transitioning back to
/// itself) — the simplest choice that keeps the context-transition matrix row-stochastic and
/// well-defined for power iteration without fabricating data the sequence never showed.
pub struct MarkovChain {
    alphabet_size: usize,
    order: usize,
    num_contexts: usize,
    n: usize,
    /// ⛓️ Row-major `num_contexts x alphabet_size` raw transition counts.
    counts: Vec<f64>,
    /// ⛓️ Row-major `num_contexts x alphabet_size` conditional probabilities `P(next | context)`.
    /// A context with zero total count has an all-zero row here.
    conditional: Vec<f64>,
}

impl MarkovChain {
    /// ⛓️ Row-major `num_contexts x alphabet_size` raw transition counts (diagnostic access to
    /// the fitted state, e.g. for inspecting per-context sample sizes before trusting
    /// [`MarkovChain::entropy_rate`] on sparsely observed contexts).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn raw_counts(&self) -> &[f64] {
        &self.counts
    }

    /// ⛓️ Fits an order-`order` Markov chain to `seq` (symbols in `0..alphabet_size`). Requires
    /// `order >= 1` and `seq.len() >= order + 1`; rejects sequences that are too short with
    /// [`EntropyError::InsufficientData`] and out-of-range symbols or an overflowing
    /// `alphabet_size^order` context space with [`EntropyError::InvalidConfig`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn fit(seq: &[u32], alphabet_size: usize, order: usize) -> Result<Self, EntropyError> {
        if order == 0 {
            return Err(EntropyError::InvalidConfig { field: "order", reason: "must be >= 1" });
        }
        if alphabet_size == 0 {
            return Err(EntropyError::InvalidConfig { field: "alphabet_size", reason: "must be >= 1" });
        }
        if seq.len() < order + 1 {
            return Err(EntropyError::InsufficientData { what: "markov sequence", needed: order + 1, actual: seq.len() });
        }
        if seq.iter().any(|&s| s as usize >= alphabet_size) {
            return Err(EntropyError::InvalidConfig { field: "seq", reason: "symbol index must be < alphabet_size" });
        }
        let num_contexts = checked_state_count(&vec![alphabet_size; order]).and_then(|c| usize::try_from(c).ok()).ok_or(EntropyError::InvalidConfig { field: "alphabet_size/order", reason: "alphabet_size^order overflows usize" })?;

        let mut counts = vec![0.0_f64; num_contexts * alphabet_size];
        for i in order..seq.len() {
            let context = pack_context(&seq[i - order..i], alphabet_size);
            let next = seq[i] as usize;
            counts[context * alphabet_size + next] += 1.0;
        }

        let mut conditional = vec![0.0_f64; num_contexts * alphabet_size];
        for context in 0..num_contexts {
            let row = &counts[context * alphabet_size..(context + 1) * alphabet_size];
            let total = neumaier_sum(row.iter().copied());
            if total > 0.0 {
                for next in 0..alphabet_size {
                    conditional[context * alphabet_size + next] = counts[context * alphabet_size + next] / total;
                }
            }
        }

        Ok(Self { alphabet_size, order, num_contexts, n: seq.len(), counts, conditional })
    }

    /// ⛓️ Stationary distribution over the `alphabet_size^order` contexts, found by power
    /// iteration on the context-transition matrix implied by the fitted conditional
    /// distributions (a context transitions to the context formed by dropping its oldest symbol
    /// and appending the sampled next symbol; for `order == 1` this is the ordinary
    /// symbol-to-symbol transition matrix). Iterates a uniform-start distribution until the L1
    /// change drops below `1e-12` or `10_000` iterations elapse, returning
    /// [`EntropyError::NotConverged`] on the latter.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn stationary(&self) -> Result<Vec<f64>, EntropyError> {
        let k = self.num_contexts;
        let modulus = k / self.alphabet_size;
        let mut pi = vec![1.0 / k as f64; k];
        for _ in 0..10_000 {
            let mut next_pi = vec![0.0_f64; k];
            for context in 0..k {
                let row = &self.conditional[context * self.alphabet_size..(context + 1) * self.alphabet_size];
                let row_total = neumaier_sum(row.iter().copied());
                if row_total <= 0.0 {
                    // ⛓️ Absorbing self-loop for an unobserved context (see struct docs).
                    next_pi[context] += pi[context];
                    continue;
                }
                for (next, &p) in row.iter().enumerate() {
                    if p <= 0.0 {
                        continue;
                    }
                    let new_context = (context % modulus) * self.alphabet_size + next;
                    next_pi[new_context] += pi[context] * p;
                }
            }
            let diff = neumaier_sum(pi.iter().zip(next_pi.iter()).map(|(&a, &b)| (a - b).abs()));
            pi = next_pi;
            if diff < 1e-12 {
                return Ok(pi);
            }
        }
        Err(EntropyError::NotConverged { what: "markov stationary distribution power iteration", iterations: 10_000 })
    }

    /// ⛓️ Entropy rate `-sum_context pi(context) * sum_next P(next|context) * ln P(next|context)`,
    /// computed in nats from [`MarkovChain::stationary`] and the fitted conditional
    /// distributions, then converted to `base`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn entropy_rate(&self, base: LogBase) -> Result<Estimate, EntropyError> {
        base.validate()?;
        let pi = self.stationary()?;
        let nats = neumaier_sum((0..self.num_contexts).map(|context| {
            let row = &self.conditional[context * self.alphabet_size..(context + 1) * self.alphabet_size];
            let context_entropy = -neumaier_sum(row.iter().map(|&p| x_ln_x(p)));
            pi[context] * context_entropy
        }));
        Ok(Estimate {
            value: base.from_nats(nats),
            base,
            method: "markov_entropy_rate",
            n: self.n,
            n_effective: self.n as f64,
            std_error: None,
            ci: None,
            warnings: Vec::new(),
            diagnostics: vec![("order", self.order as f64), ("alphabet_size", self.alphabet_size as f64)],
        })
    }
}
// #endregion 🔖️MarkovChain

// #region 🔖️Tests
#[cfg(test)]
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
