//! 📚️ Example demo for stdio.txt.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️.dsl.semio");
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🎒️ P2-P3: genuine `encode_pack` bytes of the demo snapshot (SEMIO binary envelope + the
/// demo body's raw UTF-8 bytes as payload) — `fixture_honesty_law` (⚙️engine/🦀️.rs)
/// asserts this round-trips both directions against a real `TxtEngine::register`-time codec
/// call, never hand-authored independently of the real encoder.
pub const PACK_BYTES: &[u8] = include_bytes!("🖼️assets/🎒️.pack.semio");


#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn demo_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }
}
