//! 🕰️ Clock of the trace primitives: the installable monotonic microsecond source, its exact installer and the native and browser default readers. See [`super`].

use super::*;

//#region 🕰️Clock
/// 🕰️ Host-installable monotonic-microseconds clock override, checked by [`now_us`] before falling
/// back to [`default_clock_us`]. Set at most meaningfully once, at host startup — mirrors
/// `HostAsyncRuntime::now_ms`'s "the implementation supplies the clock" seam, just via a free fn
/// instead of a trait method (this crate has no runtime/host object to hang a trait on).
static CLOCK_OVERRIDE: OnceLock<fn() -> Option<u64>> = OnceLock::new();

/// 🕰️ Installs one real monotonic-microseconds source shared by deadlines and watchdogs.
pub fn install_clock(clock: fn() -> Option<u64>) -> Result<(), fn() -> Option<u64>> {
    install_exact_clock(&CLOCK_OVERRIDE, clock)
}

pub(super) fn install_exact_clock(authority: &OnceLock<fn() -> Option<u64>>, clock: fn() -> Option<u64>) -> Result<(), fn() -> Option<u64>> {
    let existing = authority.get_or_init(|| clock);
    if std::ptr::fn_addr_eq(*existing, clock) {
        Ok(())
    } else {
        Err(clock)
    }
}

/// 🌐️ Converts fractional platform milliseconds into checked unsigned microseconds.
pub fn microseconds_from_milliseconds(milliseconds: f64) -> Option<u64> {
    let microseconds = milliseconds * 1_000.0;
    ((0.0..18_446_744_073_709_551_616.0).contains(&microseconds)).then(|| microseconds.floor() as u64)
}

/// 🕰️ Native and WASI p2 clocks preserve their actual monotonic microsecond precision.
#[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
pub(super) fn default_clock_us() -> Option<u64> {
    static START: OnceLock<std::time::Instant> = OnceLock::new();
    u64::try_from(START.get_or_init(std::time::Instant::now).elapsed().as_micros()).ok()
}

/// 🔒️ Bare Wasm must install its embedding host's real clock before instrumentation or work.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub(super) fn default_clock_us() -> Option<u64> {
    None
}

/// 🕰️ Current monotonic microsecond reading: the host's [`install_clock`] override if one was set,
/// else [`default_clock_us`] for this target.
pub fn try_now_us() -> Option<u64> {
    match CLOCK_OVERRIDE.get() {
        Some(clock) => clock(),
        None => default_clock_us(),
    }
}

/// 🕰️ Instrumentation requires the same installed real clock as interactive deadlines.
pub fn now_us() -> u64 {
    try_now_us().expect("real monotonic microsecond clock must be installed before instrumentation")
}
//#endregion 🕰️Clock
