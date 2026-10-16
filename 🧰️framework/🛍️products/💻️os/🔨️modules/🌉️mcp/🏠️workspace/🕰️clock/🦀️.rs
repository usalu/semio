//! 🕰️ Clock of the foundation crate: Gateway vocabulary: errors, handles, audit lane, reports, local session files and delegated credentials.

/// 🕰️ Local `now_ms` — `🦀️.rs`'s own `now_ms` is private to that module; this crate's own
/// convention (`SystemTime::now` since `UNIX_EPOCH`, clamped) restated here rather than reaching for
/// a file this packet does not own.
pub fn now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_millis() as u64).unwrap_or(0)
}
