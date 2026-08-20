//! 📚️ Example "sheet" for `stdio.semio.table` — the first real, non-scaffold fixture for this
//! subset (ticket UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, W2b/table). `PRIMARY_TEXT` is meant to be the
//! genuine `SemioTableSnapshot::print_dsl` output for `snapshot::demo_table_snapshot()`
//! (`🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/📸️snapshot/🦀️component.rs`), asserted
//! byte-identical to it by that subset's own `fixture_honesty_law` (`🚪️io/🦀️component.rs`), so
//! this fixture can never silently drift back to a fake.
//!
//! `🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` hold GENUINE `print_dsl`/`encode_pack`
//! output of `demo_table_snapshot()`, captured via a temporary `debug_dump_fixture_bytes` test in
//! `📸️snapshot/🦀️component.rs` (now removed) once this subset was mounted and compiled — verified
//! byte-exact with `wc -c`/`xxd`.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "sheet";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Sheet", "Tabelle")
}
pub const ICON: &str = "table";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn sheet_source_constructs() {
        let _ = source();
    }
}
