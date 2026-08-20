//! 📚️ Example "crate" for `s.stdio.semio.object` — the first real fixture for this COMPOSITE
//! subset (ticket UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, W2c/object). `PRIMARY_TEXT` is the genuine
//! `SemioObjectSnapshot::print_dsl` output for `snapshot::demo_object_snapshot()`
//! (`🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/📸️snapshot/🦀️component.rs`), asserted
//! byte-identical to it by that subset's own `fixture_honesty_law` (`🚪️io/🦀️component.rs`).
//!
//! `🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` hold GENUINE `print_dsl`/`encode_pack`
//! output of `demo_object_snapshot()`, captured via a temporary `debug_dump_fixture_bytes` test in
//! `📸️snapshot/🦀️component.rs` (removed after capture), verified byte-exact with `wc -c`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "crate";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Crate", "Kiste")
}
pub const ICON: &str = "cube";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn crate_source_constructs() {
        let _ = source();
    }
}
