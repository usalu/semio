//! 📚️ Example "furniture" for `s.stdio.semio.kit` — the first real fixture for this SECOND
//! COMPOSITE subset (ticket UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, W2c/kit). `PRIMARY_TEXT` is the
//! genuine `SemioKitSnapshot::print_dsl` output for `snapshot::demo_kit_snapshot()`
//! (`🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/📸️snapshot/🦀️component.rs`), asserted
//! byte-identical to it by that subset's own `fixture_honesty_law` (`🚪️io/🦀️component.rs`).
//!
//! `🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` hold GENUINE `print_dsl`/`encode_pack`
//! output of `demo_kit_snapshot()`, captured via a temporary `debug_dump_fixture_bytes` test in
//! `📸️snapshot/🦀️component.rs` (removed after capture), verified byte-exact with `wc -c`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "furniture";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Furniture Kit", "Möbel-Kit")
}
pub const ICON: &str = "sofa";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn furniture_source_constructs() {
        let _ = source();
    }
}
