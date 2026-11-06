//! 🪜️ Adaptive monotonic clock sampling for bounded interactive turns.

//#region 🪜️ClockStride
/// 🪜️ A step's clock, read for real only every `stride` calls. A job that checks its deadline after
/// every unit of work used to read the host clock just as often; inside a Wasm guest that read is a
/// component-model host call, and measured 2026-09-24 (ticket 26/09/23 slice G6) the `🀄️wfc` genesis
/// solve made 6.7 million of them in one `inference_run`. The stride is recalibrated at every real
/// read so reads land about [`ClockStride::TARGET_READ_INTERVAL_US`] apart, which is also the most a
/// deadline is overshot by. Every [`StepContext`] a driver builds carries one: the step begins at the
/// driver's own entry reading ([`ClockStride::begin`]), so a new step costs no read of its own, and the
/// calibrated stride carries from step to step. Law: `🧫️fixtures/🪜️clock-stride-law.json`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClockStride {
    stride: u32,
    remaining: u32,
    last_read_us: Option<u64>,
}

impl ClockStride {
    pub const TARGET_READ_INTERVAL_US: u64 = 64;
    pub const MAXIMUM_STRIDE: u32 = 4_096;

    pub const fn new() -> Self {
        Self { stride: 1, remaining: 0, last_read_us: None }
    }

    /// 🏁️ Begins a step at `start_us`, the reading its driver made at entry: the step's first
    /// `stride - 1` reads answer it, and the calibrated stride is kept.
    pub fn begin(&mut self, start_us: Option<u64>) {
        if start_us.is_none() || matches!((start_us,self.last_read_us),(Some(now),Some(last)) if now<last) {
            self.remaining=0;
            return;
        }
        self.last_read_us = start_us;
        self.remaining = if start_us.is_some() { self.stride - 1 } else { 0 };
    }

    /// ⏱️ The clock as of the last real read of `clock`, which is at most one stride of calls old.
    pub fn read(&mut self, clock: fn() -> Option<u64>) -> Option<u64> {
        if self.remaining > 0 {
            self.remaining -= 1;
            return self.last_read_us;
        }
        let now = clock()?;
        if let Some(last) = self.last_read_us {
            if now<last {self.remaining=0;return None;}
            let elapsed = (now-last).max(1);
            let calibrated = u64::from(self.stride) * Self::TARGET_READ_INTERVAL_US / elapsed;
            self.stride = calibrated.clamp(1, u64::from(Self::MAXIMUM_STRIDE).min(u64::from(self.stride) * 2)) as u32;
        }
        self.last_read_us = Some(now);
        self.remaining = self.stride - 1;
        Some(now)
    }

    /// 🕰️ The last reading, without reading the clock.
    pub fn latest_us(&self) -> Option<u64> {
        self.last_read_us
    }

    /// 📖️ How many calls the next real read is apart — the law's observable.
    pub fn stride(&self) -> u32 {
        self.stride
    }
}

impl Default for ClockStride {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🪜️ClockStride
