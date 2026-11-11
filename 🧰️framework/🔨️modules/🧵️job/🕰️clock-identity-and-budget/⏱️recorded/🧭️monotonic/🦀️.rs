//! 🧭️ One target-neutral monotonic time port; each target selects exactly one real source.
use semio_framework_value_control_contract::{ValueError,ValueRefusalKind};

/// 🧭️ Samples a real monotonic instant and measures elapsed whole microseconds between two samples.
pub trait MonotonicPort{
 /// 📍️ Opaque sample owned by the selected source.
 type Mark:Copy+Send+'static;
 /// 📥️ Samples the real clock or refuses when the target exposes none.
 fn mark()->Result<Self::Mark,ValueError>;
 /// 📏️ Returns the elapsed microseconds, or none when `now` precedes `origin`.
 fn elapsed_micros(origin:Self::Mark,now:Self::Mark)->Option<u128>;
}

/// 🖥️ The operating system monotonic clock of native and wasip2 targets.
#[cfg(not(all(target_arch="wasm32",target_os="unknown")))]
pub struct OperatingSystemMonotonic;
#[cfg(not(all(target_arch="wasm32",target_os="unknown")))]
impl MonotonicPort for OperatingSystemMonotonic{
 type Mark=std::time::Instant;
 fn mark()->Result<Self::Mark,ValueError>{Ok(std::time::Instant::now())}
 fn elapsed_micros(origin:Self::Mark,now:Self::Mark)->Option<u128>{now.checked_duration_since(origin).map(|elapsed|elapsed.as_micros())}
}
#[cfg(not(all(target_arch="wasm32",target_os="unknown")))]
pub type SelectedMonotonic=OperatingSystemMonotonic;

#[cfg(all(target_arch="wasm32",target_os="unknown"))]
#[path="🌐️browser/🦀️.rs"]
mod browser;
#[cfg(all(target_arch="wasm32",target_os="unknown"))]
pub use browser::BrowserMonotonic;
#[cfg(all(target_arch="wasm32",target_os="unknown"))]
pub type SelectedMonotonic=BrowserMonotonic;

pub(crate) fn unavailable(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
