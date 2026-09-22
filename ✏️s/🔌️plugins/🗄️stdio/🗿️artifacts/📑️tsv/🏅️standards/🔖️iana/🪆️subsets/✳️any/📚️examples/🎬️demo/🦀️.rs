//! 📚️ Example demo for stdio.tsv — the five-row price list `🖼️assets/📊️.tsv` holds, in this
//! subset's own DSL facet.
//!
//! 🐛️ It used to be the W1b scaffold: `🗣️.dsl.semio` carried the HEX TRANSCRIPTION of those bytes
//! ("a trivial hex-encoded instance, matching gif's own demo convention"). `TsvSnapshot::parse_dsl`
//! has no preamble to strip off such a file, so it decodes the whole hex string as ONE record with
//! ONE field — and the `stdio-tsv` play pane rendered the example as a single `Column 1` cell full
//! of `6964096e616d65…` instead of a table (measured live on :6033, 2026-09-22). Round-trip laws
//! could not see it: hex text round-trips through a byte-exact split/rejoin codec exactly as well
//! as real TSV does. `demo_dsl_is_this_subsets_own_printed_table` below is the law that can.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️.dsl.semio");
/// 📊️ The authored TSV file this example IS — the authority `🗣️.dsl.semio` is printed from.
pub const RAW_TSV: &str = include_str!("🖼️assets/📊️.tsv");
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
