//! 🕰️ Native host temporal formatter: the port the native application installs its operating-system temporal formatter through, lifted out of the scene state for the `native-platform` crate of the renderer partition (plan section 5.2).

#[cfg(not(target_arch = "wasm32"))]
use std::sync::{Arc, Mutex, OnceLock};

#[cfg(not(target_arch = "wasm32"))]
pub trait NativeHostTemporalFormatter: Send + Sync {
    fn format_temporal_values(&self, request: &ui_contract::HostTemporalFormatRequestV1) -> Result<ui_contract::HostTemporalFormatReplyV1, String>;
}

#[cfg(not(target_arch = "wasm32"))]
static NATIVE_HOST_TEMPORAL_FORMATTER: OnceLock<Mutex<Option<Arc<dyn NativeHostTemporalFormatter>>>> = OnceLock::new();

/// 🖥️ Installs the native application's operating-system temporal formatter. The renderer owns only
/// the neutral request/reply and never synthesizes locale or time-zone data.
#[cfg(not(target_arch = "wasm32"))]
pub fn install_native_host_temporal_formatter(formatter: Arc<dyn NativeHostTemporalFormatter>) {
    *NATIVE_HOST_TEMPORAL_FORMATTER.get_or_init(|| Mutex::new(None)).lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(formatter);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn native_host_temporal_formatter() -> Option<Arc<dyn NativeHostTemporalFormatter>> {
    NATIVE_HOST_TEMPORAL_FORMATTER.get_or_init(|| Mutex::new(None)).lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
}
