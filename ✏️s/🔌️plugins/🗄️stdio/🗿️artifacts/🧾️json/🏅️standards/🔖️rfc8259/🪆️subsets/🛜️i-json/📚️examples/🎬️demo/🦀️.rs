//! 📚️ Example demo for stdio.json (rfc8259/🛜️i-json) — an RFC 7493 conformant document: object at
//! the top level, no duplicate member name, every integer inside the IEEE 754 safe range and every
//! string free of noncharacters. The ✳️any subset's own `🎬️demo` is NOT reused: an example belongs
//! to the subset whose dialect it is published under, and this one exists to be I-JSON clean.

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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
