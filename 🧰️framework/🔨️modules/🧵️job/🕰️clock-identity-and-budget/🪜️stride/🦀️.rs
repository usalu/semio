//! 🪜️ Clock sampling retains its calibrated stride independently of any job Source.
//#region 🪜️ClockStride
/// 🪜️ See `README.md`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClockStride {
    pub(super) stride: u32,
    pub(super) remaining: u32,
    last_read_us: Option<u64>,
}

impl ClockStride {
    pub const TARGET_READ_INTERVAL_US: u64 = 64;
    pub const MAXIMUM_STRIDE: u32 = 4_096;

    pub const fn new() -> Self {
        Self { stride: 1, remaining: 0, last_read_us: None }
    }

    /// 🏁️ See `README.md`.
    pub fn begin(&mut self, start_us: Option<u64>) {
        if start_us.is_none() || matches!((start_us,self.last_read_us),(Some(now),Some(last)) if now<last) {
            self.remaining=0;
            return;
        }
        self.last_read_us = start_us;
        self.remaining = if start_us.is_some() { self.stride - 1 } else { 0 };
    }

    /// ⏱️ See `README.md`.
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

    /// 🕰️ See `README.md`.
    pub fn latest_us(&self) -> Option<u64> {
        self.last_read_us
    }

    /// 📖️ See `README.md`.
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
