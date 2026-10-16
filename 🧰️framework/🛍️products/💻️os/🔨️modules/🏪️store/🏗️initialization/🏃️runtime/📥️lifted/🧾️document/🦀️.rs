//! 📥️ Items of `document_materialize` that belong to `os-store-initialization-runtime` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧾️document/🦀️.rs`.
use semio_framework_os_spr::HybridLogicalTimestamp;
/// 🪪️ A Store's replica clock: its `actor` is fresh platform entropy per Store instance, the
/// identity every edit and operation id this Store mints is namespaced by. Refusing to construct
/// without entropy is deliberate — a constant replica makes two sessions' first edits collide.
pub fn fresh_replica_clock() -> HybridLogicalTimestamp {
    HybridLogicalTimestamp::new(semio_framework_os_identity::entropy_u64().expect("artifact store replica identity requires platform entropy"), now_ms())
}

/// 🕰️ Captures the platform Unix milliseconds used by original Store edits and replica clocks.
pub fn now_ms() -> u64 {
    #[cfg(any(not(target_arch = "wasm32"), target_env = "p2"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as u64)
    }
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    {
        js_sys::Date::now() as u64
    }
}
