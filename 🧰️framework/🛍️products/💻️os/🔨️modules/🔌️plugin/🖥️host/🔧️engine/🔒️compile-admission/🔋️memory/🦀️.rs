//! 🔋️ Cold compiler memory admission from the current host's native counters.

use std::io;

const MIB: u64 = 1 << 20;
const COMPILE_BASE_BYTES: u64 = 256 * MIB;
const COMPILE_COMPONENT_MULTIPLIER: u64 = 48;

/// 🎟️ Refuses a cold compilation temporarily when it cannot establish sufficient host memory.
pub fn require_compile_memory(component_bytes: usize) -> io::Result<()> {
    check_compile_memory(component_bytes, host_memory())
}

pub(super) fn check_compile_memory(component_bytes: usize, sample: Option<(u64, u64)>) -> io::Result<()> {
    let required = COMPILE_BASE_BYTES.saturating_add((component_bytes as u64).saturating_mul(COMPILE_COMPONENT_MULTIPLIER));
    let Some((total, available)) = sample else {
        return Err(io::Error::new(io::ErrorKind::WouldBlock, "cold Wasmtime compilation cannot establish normal host memory headroom"));
    };
    let spare = available.saturating_sub(desktop_reserve(total));
    if spare < required {
        return Err(io::Error::new(io::ErrorKind::WouldBlock, format!("cold Wasmtime compilation estimates {} MiB with {} MiB available above the desktop reserve", required.div_ceil(MIB), spare / MIB)));
    }
    Ok(())
}

/// 🖥️ Mirrors the repo host-capacity memory floor in `🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json`.
fn desktop_reserve(total: u64) -> u64 {
    ((total / MIB / 8).clamp(1536, 4096) * MIB).min(total)
}

#[cfg(target_os = "linux")]
#[path = "🐧️/🦀️.rs"]
mod platform;
#[cfg(target_os = "macos")]
#[path = "🍎️/🦀️.rs"]
mod platform;
#[cfg(target_os = "windows")]
#[path = "🪟️/🦀️.rs"]
mod platform;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
use platform::host_memory;

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn host_memory() -> Option<(u64, u64)> {
    None
}
