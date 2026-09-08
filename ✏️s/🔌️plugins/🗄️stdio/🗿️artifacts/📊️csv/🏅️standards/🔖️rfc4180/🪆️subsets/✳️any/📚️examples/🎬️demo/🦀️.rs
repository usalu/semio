//! 📚️ Example demo for stdio.csv.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️.dsl.semio");
/// 📄️ Genuine RFC 4180 bytes for the demo snapshot (`encode_csv(demo_csv_snapshot())`).
pub const NATIVE_BYTES: &[u8] = include_str!("🖼️assets/🧪️example/📊️.csv").as_bytes();
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

//#region 🔖️P2P1BinaryFixtures
/// 🎒️ Genuine `encode_pack` bytes of the demo snapshot (P2-P1 `fixture_honesty_law`).
pub const PACK_BYTES: &[u8] = include_bytes!("🖼️assets/🎒️.pack.semio");
/// 📡️ Genuine `encode_op` bytes of a real `CsvMutation` (P2-P1 `protocol_walk_law`, Spr facet).
pub const SPR_BYTES: &[u8] = include_bytes!("🖼️assets/📡️example.spr.semio");
//#endregion 🔖️P2P1BinaryFixtures

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
