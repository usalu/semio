//! 📚️ Example demo for stdio.csv.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
pub fn label() -> LocalizedLabel { LocalizedLabel::native("Demo", "Demo") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }

//#region 🔖️P2P1BinaryFixtures
/// 🎒️ Genuine `encode_pack` bytes of the demo snapshot (P2-P1 `fixture_honesty_law`).
pub const PACK_BYTES: &[u8] = include_bytes!("🖼️assets/🎒️example.pack.semio");
/// 📡️ Genuine `encode_op` bytes of a real `CsvMutation` (P2-P1 `protocol_walk_law`, Spr facet).
pub const SPR_BYTES: &[u8] = include_bytes!("🖼️assets/📡️example.spr.semio");
//#endregion 🔖️P2P1BinaryFixtures

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn demo_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }
}
