//! 🎲️ Seeded, reproducible pseudo-random generation: xoshiro256** core, distributions, and sequence samplers shared by every randomized graph algorithm.

// #region 🔖️SplitMix64
/// 🌱️ SplitMix64 seed-mixing step (Vigna's `splitmix64`): turns a `u64` seed into a well-mixed
/// stream, used only to derive [`Rng`]'s initial state words — never as the generator itself.
pub struct SplitMix64(u64);

impl SplitMix64 {
    /// 🌱️ Starts a mixing stream from a raw seed.
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// 🌱️ Advances the stream and returns the next mixed 64-bit word.
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}
// #endregion 🔖️SplitMix64

// #region 🔖️Rng
#[inline]
fn rotl(x: u64, k: u32) -> u64 {
    x.rotate_left(k)
}

/// 🎲️ xoshiro256** (Blackman & Vigna, public domain), a 256-bit-state generator with a 2^256-1
/// period and excellent statistical quality; chosen over xorshift128+ for its stronger equidistribution
/// and larger period margin, both useful once nested graph algorithms draw many correlated sub-sequences
/// from independently-seeded generators. All arithmetic is `u64` wrapping — no floats in the core step —
/// so a given seed produces the exact same bit sequence on every platform.
pub struct Rng {
    s: [u64; 4],
}

impl Rng {
    /// 🌱️ Seeds all four state words via [`SplitMix64`] so even adjacent seeds decorrelate immediately.
    pub fn from_seed(seed: u64) -> Self {
        let mut sm = SplitMix64::new(seed);
        let mut s = [0u64; 4];
        for slot in s.iter_mut() {
            *slot = sm.next_u64();
        }
        Self { s }
    }

    /// 🎲️ Next raw 64-bit word (the xoshiro256** `scramble` output), advancing the state.
    pub fn next_u64(&mut self) -> u64 {
        let result = rotl(self.s[1].wrapping_mul(5), 7).wrapping_mul(9);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = rotl(self.s[3], 45);
        result
    }

    /// 🎯️ Uniform `f64` in `[0, 1)`, built from the top 53 bits of a raw draw (the mantissa width of `f64`).
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// 🎯️ Uniform `u64` in `[lo, hi)`. Rejects draws that fall in the trailing partial bucket of
    /// `u64::MAX / range` instead of using `% range` directly: naive modulo keeps that partial bucket,
    /// which over-weights the low end of the range by a hair — rejection sampling discards it so every
    /// remaining value maps to exactly the same number of raw draws.
    pub fn next_range(&mut self, lo: u64, hi: u64) -> u64 {
        debug_assert!(hi >= lo, "next_range: hi must be >= lo");
        let range = hi - lo;
        if range == 0 {
            return lo;
        }
        let limit = u64::MAX - (u64::MAX % range);
        loop {
            let x = self.next_u64();
            if x < limit {
                return lo + x % range;
            }
        }
    }

    /// 🪙️ `true` with probability `p` (clamped semantics: `p <= 0.0` never fires, `p >= 1.0` always fires).
    pub fn next_bool(&mut self, p: f64) -> bool {
        self.next_f64() < p
    }

    /// 🔀️ In-place Fisher-Yates shuffle: uniform over all `n!` permutations.
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        let n = items.len();
        for i in (1..n).rev() {
            let j = self.next_range(0, (i + 1) as u64) as usize;
            items.swap(i, j);
        }
    }

    /// 🎯️ Picks one uniformly random element, or `None` for an empty slice.
    pub fn choose<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            return None;
        }
        let idx = self.next_range(0, items.len() as u64) as usize;
        items.get(idx)
    }

    /// 🎲️ Raw 256-bit state words, for external snapshot/restore of a generator mid-stream.
    pub fn state(&self) -> [u64; 4] {
        self.s
    }

    /// 🌱️ Rebuilds a generator from previously captured state words (inverse of [`Rng::state`]).
    /// The all-zero state is xoshiro256**'s fixed point (every subsequent draw is also zero), so
    /// callers must never pass it; debug builds catch the mistake immediately.
    pub fn from_state(s: [u64; 4]) -> Self {
        debug_assert!(s != [0u64; 4], "from_state: all-zero state is the xoshiro256** fixed point");
        Self { s }
    }

    /// 🎯️ `k` distinct indices drawn uniformly from `0..n`, via Floyd's O(k)-time, O(k)-space partial
    /// sampling algorithm — it never materializes the full `0..n` universe, which matters once `n` is a
    /// graph's node count and `k` is a small subsample. Order is not itself a uniform permutation.
    pub fn sample_without_replacement(&mut self, n: usize, k: usize) -> Vec<usize> {
        assert!(k <= n, "sample_without_replacement: k must not exceed n");
        let mut selected: std::collections::HashSet<usize> = std::collections::HashSet::with_capacity(k);
        let mut result = Vec::with_capacity(k);
        for j in (n - k)..n {
            let t = self.next_range(0, (j + 1) as u64) as usize;
            let picked = if selected.contains(&t) { j } else { t };
            selected.insert(picked);
            result.push(picked);
        }
        result
    }
}
// #endregion 🔖️Rng

// #region 🔖️AliasTable
/// ⚖️ Walker's alias method: O(n) setup, O(1) per draw, for sampling an index `0..weights.len()`
/// proportional to arbitrary non-negative weights. Needed by degree-sequence-weighted random graph
/// generators (configuration model, Chung-Lu) where millions of draws share one weight vector.
pub struct AliasTable {
    prob: Vec<f64>,
    alias: Vec<usize>,
}

impl AliasTable {
    /// ⚖️ Builds the table, normalizing `weights` internally. An empty slice or all-zero weights are
    /// degenerate (no valid probability distribution exists), so both are defined to always sample
    /// index `0` rather than panic — callers that pass a plain degree/weight vector don't need to
    /// special-case the all-zero graph before building a table.
    pub fn new(weights: &[f64]) -> Self {
        let n = weights.len();
        if n == 0 {
            return Self { prob: vec![1.0], alias: vec![0] };
        }
        let sum: f64 = weights.iter().sum();
        if sum <= 0.0 {
            let mut prob = vec![0.0; n];
            prob[0] = 1.0;
            return Self { prob, alias: vec![0; n] };
        }
        let mut scaled: Vec<f64> = weights.iter().map(|w| w / sum * n as f64).collect();
        let mut small: Vec<usize> = Vec::new();
        let mut large: Vec<usize> = Vec::new();
        for (i, &p) in scaled.iter().enumerate() {
            if p < 1.0 {
                small.push(i);
            } else {
                large.push(i);
            }
        }
        let mut prob = vec![0.0; n];
        let mut alias = vec![0usize; n];
        // `(small.pop(), large.pop())` would evaluate both pops unconditionally even when one list is
        // already empty, silently dropping an element from the non-empty side — check lengths first.
        while !small.is_empty() && !large.is_empty() {
            let s = small.pop().expect("small is non-empty per the loop condition");
            let l = large.pop().expect("large is non-empty per the loop condition");
            prob[s] = scaled[s];
            alias[s] = l;
            scaled[l] = scaled[l] + scaled[s] - 1.0;
            if scaled[l] < 1.0 {
                small.push(l);
            } else {
                large.push(l);
            }
        }
        for l in large {
            prob[l] = 1.0;
        }
        for s in small {
            prob[s] = 1.0;
        }
        Self { prob, alias }
    }

    /// ⚖️ Draws one index in O(1): pick a bucket uniformly, then coin-flip between its own item and its alias.
    pub fn sample(&self, rng: &mut Rng) -> usize {
        let n = self.prob.len();
        let i = rng.next_range(0, n as u64) as usize;
        if rng.next_f64() < self.prob[i] {
            i
        } else {
            self.alias[i]
        }
    }
}
// #endregion 🔖️AliasTable

// #region 🔖️Distributions
/// 🔔️ Standard normal via the Box-Muller transform, scaled to `(mean, std_dev)`. Draws `u1` from
/// `(0, 1]` (not `[0, 1)`) so `ln(u1)` never sees an exact zero.
pub fn normal(rng: &mut Rng, mean: f64, std_dev: f64) -> f64 {
    let u1 = 1.0 - rng.next_f64();
    let u2 = rng.next_f64();
    let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
    mean + std_dev * z0
}

/// 🎲️ Number of Bernoulli(`p`) trials up to and including the first success (support `1, 2, 3, ...`),
/// sampled by inverse-CDF transform.
pub fn geometric(rng: &mut Rng, p: f64) -> u64 {
    debug_assert!(p > 0.0 && p <= 1.0, "geometric: p must be in (0, 1]");
    if p >= 1.0 {
        return 1;
    }
    let u = 1.0 - rng.next_f64();
    (u.ln() / (1.0 - p).ln()).floor() as u64 + 1
}

/// 🎲️ Poisson(`lambda`) via Knuth's product-of-uniforms algorithm: O(lambda) draws per sample, so it
/// stays fast for moderate `lambda` (roughly up to a few dozen) but degrades for very large `lambda`,
/// where a transformed-rejection method would be preferable.
pub fn poisson(rng: &mut Rng, lambda: f64) -> u64 {
    debug_assert!(lambda >= 0.0, "poisson: lambda must be non-negative");
    let l = (-lambda).exp();
    let mut k = 0u64;
    let mut p = 1.0;
    loop {
        k += 1;
        p *= rng.next_f64();
        if p <= l {
            break;
        }
    }
    k - 1
}

/// 📐️ `n` samples from a Pareto-like power-law distribution with the given `exponent`, matching
/// NetworkX `utils.powerlaw_sequence`'s `random() ** (-1 / (exponent - 1))` inverse-transform.
pub fn powerlaw_sequence(rng: &mut Rng, n: usize, exponent: f64) -> Vec<f64> {
    debug_assert!(exponent != 1.0, "powerlaw_sequence: exponent must not be 1.0");
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let u = 1.0 - rng.next_f64();
        out.push(u.powf(-1.0 / (exponent - 1.0)));
    }
    out
}

/// 📊️ Samples a rank in `1..=n` from a Zipf distribution via rejection sampling: draws from the
/// unbounded-support Zipf algorithm (Devroye) and re-rejects any draw landing outside `1..=n`, so no
/// O(n) setup is needed even when `n` is huge. Requires `exponent > 1.0`.
pub fn zipf(rng: &mut Rng, n: usize, exponent: f64) -> u64 {
    debug_assert!(exponent > 1.0, "zipf: exponent must be > 1.0");
    debug_assert!(n >= 1, "zipf: n must be >= 1");
    let am1 = exponent - 1.0;
    let b = 2f64.powf(am1);
    loop {
        let u = 1.0 - rng.next_f64();
        let v = rng.next_f64();
        let x = u.powf(-1.0 / am1).floor();
        if x < 1.0 {
            continue;
        }
        let t = (1.0 + 1.0 / x).powf(am1);
        if v * x * (t - 1.0) / (b - 1.0) <= t / b && x <= n as f64 {
            return x as u64;
        }
    }
}

/// 🎯️ `n` draws from a discrete distribution given as a weight vector, matching NetworkX
/// `utils.discrete_sequence`; builds one [`AliasTable`] and draws from it `n` times.
pub fn discrete_sequence(rng: &mut Rng, n: usize, distribution: &[f64]) -> Vec<usize> {
    let table = AliasTable::new(distribution);
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(table.sample(rng));
    }
    out
}

/// ➕️ Running sum of `weights`, normalized so the last entry is `1.0` (a no-operation on an all-zero or
/// empty input, returned unnormalized since there is no meaningful scale to normalize to).
pub fn cumulative_distribution(weights: &[f64]) -> Vec<f64> {
    let mut cumulative = Vec::with_capacity(weights.len());
    let mut running = 0.0;
    for &w in weights {
        running += w;
        cumulative.push(running);
    }
    if let Some(&last) = cumulative.last() {
        if last > 0.0 {
            for v in cumulative.iter_mut() {
                *v /= last;
            }
        }
    }
    cumulative
}
// #endregion 🔖️Distributions

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
