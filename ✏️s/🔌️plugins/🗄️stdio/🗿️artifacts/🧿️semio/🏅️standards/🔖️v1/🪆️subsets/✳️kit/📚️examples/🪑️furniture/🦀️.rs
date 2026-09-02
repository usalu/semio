//! 📚️ Example "furniture" for `s.stdio.semio.kit` — the first real fixture for this SECOND
//! COMPOSITE subset (ticket UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, W2c/kit). `PRIMARY_TEXT` is the
//! genuine `SemioKitSnapshot::print_dsl` output for `snapshot::demo_kit_snapshot()`
//! (`🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/📸️snapshot/🦀️.rs`), asserted
//! byte-identical to it by that subset's own `fixture_honesty_law` (`🚪️io/🦀️.rs`).
//!
//! `🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` hold GENUINE `print_dsl`/`encode_pack`
//! output of `demo_kit_snapshot()`, captured via a temporary `debug_dump_fixture_bytes` test in
//! `📸️snapshot/🦀️.rs` (removed after capture), verified byte-exact with `wc -c`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "furniture";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Furniture Kit", "Möbel-Kit")
}
pub const ICON: &str = "sofa";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️.dsl.semio");
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn furniture_source_constructs() {
        let _ = source();
    }
}
