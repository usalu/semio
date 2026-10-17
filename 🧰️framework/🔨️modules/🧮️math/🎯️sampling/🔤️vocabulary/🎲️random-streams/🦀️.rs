//! 🎲️ Random streams of sampling: the stream purpose and key, the seeded stream derivation, the random source trait with the counter and xoshiro sources and their closed enum. See [`super`].

use super::*;

// #region 🔖️Rng
/// 🎲️ Which sub-stream of randomness a draw belongs to, so unrelated concerns (selection noise vs.
/// speculative-decoding acceptance vs. diffusion noise) never share bits even within one sequence.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum StreamPurpose {
    Selection = 0,
    Gumbel = 1,
    Noise = 2,
    Speculative = 3,
    Beam = 4,
    Diffusion = 5,
}

/// 🎲️ Identifies one independent random stream by the ids that produced it — never by batch slot,
/// so a continuous-batching reorder never changes which bits a sequence draws.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StreamKey {
    pub request: u64,
    pub sequence: u64,
    pub beam: u32,
    pub candidate: u32,
    pub purpose: StreamPurpose,
}

async fn mix64(x: u64) -> u64 {
    semio_framework_geometry::random::SplitMix64::new(x).next_u64()
}

async fn stream_seed(key: StreamKey) -> u64 {
    let mut acc = mix64(key.request).await;
    acc = mix64(acc ^ mix64(key.sequence).await).await;
    acc = mix64(acc ^ mix64(key.beam as u64).await).await;
    acc = mix64(acc ^ mix64(key.candidate as u64).await).await;
    acc = mix64(acc ^ mix64(key.purpose as u64).await).await;
    acc
}

/// 🎲️ Which concrete generator produced a [`RngSnapshot`], so `restore` can reject a snapshot
/// meant for the other kind instead of silently reinterpreting its words.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RngKind {
    Counter,
    Xoshiro,
}

/// 🎲️ Portable capture of a generator's internal state, text-serializable for [`SequenceState`]
/// checkpoints.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RngSnapshot {
    pub kind: RngKind,
    pub words: [u64; 4],
}

impl RngSnapshot {
    /// 🎲️ Compact `kind:hex:hex:hex:hex` text form.
    pub async fn to_text(&self) -> String {
        let kind = match self.kind {
            RngKind::Counter => "counter",
            RngKind::Xoshiro => "xoshiro",
        };
        format!("{kind}:{:016x}:{:016x}:{:016x}:{:016x}", self.words[0], self.words[1], self.words[2], self.words[3])
    }

    /// 🎲️ Inverse of [`RngSnapshot::to_text`].
    pub async fn from_text(text: &str) -> Result<Self, SamplingError> {
        let mut parts = text.split(':');
        let kind = match parts.next() {
            Some("counter") => RngKind::Counter,
            Some("xoshiro") => RngKind::Xoshiro,
            _ => return Err(SamplingError::Corrupted { reason: "unknown rng snapshot kind" }),
        };
        let mut words = [0u64; 4];
        for w in words.iter_mut() {
            let part = parts.next().ok_or(SamplingError::Corrupted { reason: "truncated rng snapshot" })?;
            *w = u64::from_str_radix(part, 16).map_err(|_| SamplingError::Corrupted { reason: "invalid rng snapshot hex" })?;
        }
        if parts.next().is_some() {
            return Err(SamplingError::Corrupted { reason: "trailing data in rng snapshot" });
        }
        Ok(Self { kind, words })
    }
}

/// 🎲️ Object-safe source of randomness handed to samplers/warpers/search algorithms. Every
/// implementation must be splittable into independent child streams keyed by [`StreamKey`] alone
/// (never by call order), which is what keeps continuous-batching reorders and speculative
/// verification bit-reproducible.
#[dyn_enum]
pub trait RandomSource {
    async fn next_u64(&mut self) -> u64;
    /// 🎲️ Derives an independent child stream from `(self, key)` — order-independent across calls.
    async fn split(&self, key: StreamKey) -> RandomSources;
    async fn snapshot(&self) -> RngSnapshot;
    async fn restore(&mut self, snapshot: &RngSnapshot) -> Result<(), SamplingError>;

    /// 🎲️ Uniform `f64` in `[0, 1)`.
    async fn next_f64(&mut self) -> f64 {
        (self.next_u64().await >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// 🎲️ Uniform `f64` in `(0, 1]` — safe as the argument to `ln()`, unlike `next_f64`.
    async fn next_f64_open01(&mut self) -> f64 {
        (((self.next_u64().await >> 11) + 1) as f64) * (1.0 / (1u64 << 53) as f64)
    }

    /// 🎲️ Uniform `u64` in `[lo, hi)` via rejection sampling (no modulo bias).
    async fn next_range(&mut self, lo: u64, hi: u64) -> u64 {
        debug_assert!(hi >= lo, "next_range: hi must be >= lo");
        let range = hi - lo;
        if range == 0 {
            return lo;
        }
        let limit = u64::MAX - (u64::MAX % range);
        loop {
            let x = self.next_u64().await;
            if x < limit {
                return lo + x % range;
            }
        }
    }

    /// 🎲️ Standard Gumbel(0, 1) draw, `-ln(-ln(u))` for `u` in `(0, 1]`.
    async fn gumbel(&mut self) -> f64 {
        let u = self.next_f64_open01().await;
        -(-u.ln()).ln()
    }
}

/// 🎲️ Default splittable [`RandomSource`]: a counter-based generator (double [`mix64`] of
/// `key ^ mix64(counter)`, Philox-lite) chosen over a stepped generator specifically because
/// splitting never advances or depends on the parent's step count — two sequences split from the
/// same parent at different times still get independent, order-irrelevant streams.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CounterRng {
    pub(super) key: u64,
    ctr: u64,
}

impl CounterRng {
    /// 🎲️ A root stream from a plain seed, with no [`StreamKey`] semantics (tests, standalone use).
    pub async fn from_seed(seed: u64) -> Self {
        Self { key: mix64(seed).await, ctr: 0 }
    }

    /// 🎲️ A root stream combining a request-level seed with a full [`StreamKey`] in one step.
    pub async fn from_root(root_seed: u64, key: StreamKey) -> Self {
        Self { key: mix64(mix64(root_seed).await ^ stream_seed(key).await).await, ctr: 0 }
    }
}

impl RandomSource for CounterRng {
    async fn next_u64(&mut self) -> u64 {
        let ctr = self.ctr;
        self.ctr = self.ctr.wrapping_add(1);
        mix64(self.key ^ mix64(ctr).await).await
    }

    async fn split(&self, key: StreamKey) -> RandomSources {
        Self { key: mix64(self.key ^ stream_seed(key).await).await, ctr: 0 }.into()
    }

    async fn snapshot(&self) -> RngSnapshot {
        RngSnapshot { kind: RngKind::Counter, words: [self.key, self.ctr, 0, 0] }
    }

    async fn restore(&mut self, snapshot: &RngSnapshot) -> Result<(), SamplingError> {
        if snapshot.kind != RngKind::Counter {
            return Err(SamplingError::Corrupted { reason: "rng snapshot kind mismatch: expected counter" });
        }
        self.key = snapshot.words[0];
        self.ctr = snapshot.words[1];
        Ok(())
    }
}

/// 🎲️ [`RandomSource`] adapter over [`semio_framework_geometry::random::Rng`] (xoshiro256**), for callers who
/// want that generator's statistical profile instead of the default counter-based stream.
pub struct XoshiroSource(pub(super) semio_framework_geometry::random::Rng);

impl XoshiroSource {
    pub async fn from_seed(seed: u64) -> Self {
        Self(semio_framework_geometry::random::Rng::from_seed(seed))
    }
}

impl RandomSource for XoshiroSource {
    async fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    async fn split(&self, key: StreamKey) -> RandomSources {
        let state = self.0.state();
        let seed = mix64(state[0] ^ state[1] ^ stream_seed(key).await).await;
        Self::from_seed(seed).await.into()
    }

    async fn snapshot(&self) -> RngSnapshot {
        RngSnapshot { kind: RngKind::Xoshiro, words: self.0.state() }
    }

    async fn restore(&mut self, snapshot: &RngSnapshot) -> Result<(), SamplingError> {
        if snapshot.kind != RngKind::Xoshiro {
            return Err(SamplingError::Corrupted { reason: "rng snapshot kind mismatch: expected xoshiro" });
        }
        self.0 = semio_framework_geometry::random::Rng::from_state(snapshot.words);
        Ok(())
    }
}

dyn_enum_close! {
    pub enum RandomSources: RandomSource {
        Counter(CounterRng),
        Xoshiro(XoshiroSource),
    }
}
// #endregion 🔖️Rng
