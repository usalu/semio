//! 📊️ Sample rings of the trace primitives: the bounded ring, the percentile ring, the labelled site registry and the omitted-sample counters. See [`super`].

use super::*;

//#region 🔄️BoundedRing
/// 🔄️ Generic fixed-capacity overwrite-oldest ring, the same shape as
/// `ActorMetrics::wall_us_ring`/[`PercentileRing`] generalized over `T: Copy` so it isn't duplicated
/// per event type — shared by [`Watchdog`]'s violation store and the [`TraceEvent`] store below.
pub(super) struct BoundedRing<T: Copy, const N: usize> {
    entries: [Option<T>; N],
    pos: usize,
    pub(super) len: usize,
    pub(super) total: u64,
}

impl<T: Copy, const N: usize> BoundedRing<T, N> {
    pub(super) const fn new() -> BoundedRing<T, N> {
        BoundedRing { entries: [None; N], pos: 0, len: 0, total: 0 }
    }

    pub(super) fn push(&mut self, value: T) {
        self.entries[self.pos] = Some(value);
        self.pos = (self.pos + 1) % N;
        self.len = (self.len + 1).min(N);
        self.total = self.total.saturating_add(1);
    }

    /// 📸️ Every retained entry, oldest first.
    pub(super) fn snapshot(&self) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len);
        if self.len < N {
            out.extend(self.entries[..self.len].iter().flatten().copied());
        } else {
            out.extend(self.entries[self.pos..].iter().flatten().copied());
            out.extend(self.entries[..self.pos].iter().flatten().copied());
        }
        out
    }

    /// 🔢️ Total ever pushed — unlike [`BoundedRing::snapshot`], never shrunk by eviction.
    pub(super) fn total(&self) -> u64 {
        self.total
    }

    pub(super) fn clear(&mut self) {
        *self = BoundedRing::new();
    }
}
//#endregion 🔄️BoundedRing

//#region 📈️PercentileRing
/// 📈️ Ring capacity for one site's [`PercentileRing`] — chosen to match
/// `ActorMetrics::WALL_US_RING_CAPACITY` (`🧰️framework/🔨️modules/🎭️actor/🦀️.rs`), the
/// precedent this type mirrors.
pub(super) const SAMPLE_RING_CAPACITY: usize = 64;

/// 📈️ Fixed-capacity ring of the last [`SAMPLE_RING_CAPACITY`] microsecond samples for one labelled
/// site, with p50/p95/p99 accessors. Mirrors `ActorMetrics::wall_us_ring`/`wall_us_p95`
/// (`🧰️framework/🔨️modules/🎭️actor/🦀️.rs`) — array-of-`u32` plus sort-on-read, no
/// dependency on that crate — rather than inventing a different percentile strategy.
#[derive(Clone, Debug)]
pub struct PercentileRing {
    pub(super) samples: [u32; SAMPLE_RING_CAPACITY],
    pub(super) len: u8,
    pos: u8,
}

impl PercentileRing {
    pub const fn new() -> PercentileRing {
        PercentileRing { samples: [0; SAMPLE_RING_CAPACITY], len: 0, pos: 0 }
    }

    pub fn record(&mut self, value_us: u64) {
        self.samples[self.pos as usize] = value_us.min(u32::MAX as u64) as u32;
        self.pos = (self.pos + 1) % SAMPLE_RING_CAPACITY as u8;
        self.len = (self.len + 1).min(SAMPLE_RING_CAPACITY as u8);
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // 🚫️async: E1 pure computation over already-recorded in-memory samples — same class as
    // `ActorMetrics::wall_us_p95`, which this mirrors.
    pub(super) fn percentile(&self, p: f32) -> u32 {
        if self.len == 0 {
            return 0;
        }
        let n = self.len as usize;
        let mut sorted = self.samples;
        let slice = &mut sorted[..n];
        slice.sort_unstable();
        let idx = ((n as f32) * p).floor() as usize;
        slice[idx.min(n - 1)]
    }

    pub fn p50(&self) -> u32 {
        self.percentile(0.50)
    }

    pub fn p95(&self) -> u32 {
        self.percentile(0.95)
    }

    pub fn p99(&self) -> u32 {
        self.percentile(0.99)
    }
}

impl Default for PercentileRing {
    fn default() -> PercentileRing {
        PercentileRing::new()
    }
}

const SITE_CAPACITY: usize = 256;

type SiteRegistry = [Option<(&'static str, PercentileRing)>; SITE_CAPACITY];

pub(super) static OMITTED_SITE_SAMPLES: AtomicU64 = AtomicU64::new(0);

pub(super) static OMITTED_EVENTS: AtomicU64 = AtomicU64::new(0);

pub(super) static OMITTED_VIOLATIONS: AtomicU64 = AtomicU64::new(0);

pub(super) static RECORDED_VIOLATIONS: AtomicU64 = AtomicU64::new(0);

/// 📇️ Fixed static backing admits optional site samples without a lazy allocation.
pub(super) fn site_registry() -> &'static StaticTryGate<SiteRegistry> {
    static REGISTRY: StaticTryGate<SiteRegistry> = StaticTryGate::new([const { None }; SITE_CAPACITY]);
    &REGISTRY
}

pub(super) fn record_site_sample(site: &'static str, elapsed_us: u64) {
    let Some(mut registry) = site_registry().try_lock() else {
        OMITTED_SITE_SAMPLES.fetch_add(1, Ordering::Relaxed);
        return;
    };
    let mut vacant = None;
    for (index, slot) in registry.iter_mut().enumerate() {
        match slot {
            Some((label, ring)) if std::ptr::eq(*label, site) => {
                ring.record(elapsed_us);
                return;
            }
            None if vacant.is_none() => vacant = Some(index),
            _ => {}
        }
    }
    if let Some(index) = vacant {
        let mut ring = PercentileRing::new();
        ring.record(elapsed_us);
        registry[index] = Some((site, ring));
    } else {
        OMITTED_SITE_SAMPLES.fetch_add(1, Ordering::Relaxed);
    }
}

/// 📊️ `(p50, p95, p99)` microseconds recorded for `site`, or `None` if nothing has landed there yet.
pub fn site_percentiles(site: &str) -> Option<(u32, u32, u32)> {
    let registry = site_registry().try_lock()?;
    registry.iter().flatten().find(|(label, _)| *label == site).map(|(_, ring)| (ring.p50(), ring.p95(), ring.p99()))
}

/// 📉️ Optional telemetry omissions never replace an exact callback verdict.
pub fn telemetry_omitted_counts() -> (u64, u64, u64) {
    (OMITTED_SITE_SAMPLES.load(Ordering::Relaxed), OMITTED_EVENTS.load(Ordering::Relaxed), OMITTED_VIOLATIONS.load(Ordering::Relaxed))
}
//#endregion 📈️PercentileRing
