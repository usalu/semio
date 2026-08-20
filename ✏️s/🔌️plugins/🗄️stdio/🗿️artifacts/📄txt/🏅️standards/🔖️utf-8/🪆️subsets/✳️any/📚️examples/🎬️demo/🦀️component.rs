//! 📚️ Example demo for stdio.txt.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON).await
}

/// 🎒️ P2-P3: genuine `encode_pack` bytes of the demo snapshot (SEMIO binary envelope + the
/// demo body's raw UTF-8 bytes as payload) — `fixture_honesty_law` (⚙️engine/🦀️component.rs)
/// asserts this round-trips both directions against a real `TxtEngine::register`-time codec
/// call, never hand-authored independently of the real encoder.
pub const PACK_BYTES: &[u8] = include_bytes!("🖼️assets/🎒️example.pack.semio");

/// 📡️ P2-P3: genuine `OpBinary::encode_op` bytes of a real `TxtMutation::InsertLine{index:1,
/// text:"x"}` — exercises the mutations facet's real binary op-frame
/// (`format u8 | ordinal varint | record body`, ../../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/
/// 🧬️schema/🧬️mutations/💾️binary/📡️component.protocol.semio).
pub const SPR_BYTES: &[u8] = include_bytes!("🖼️assets/📡️example.spr.semio");

#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn demo_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }
}
