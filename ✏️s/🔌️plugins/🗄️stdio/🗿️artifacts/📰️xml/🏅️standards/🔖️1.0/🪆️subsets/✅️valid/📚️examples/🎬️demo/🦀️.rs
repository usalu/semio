//! 📚️ Example demo for stdio.xml (1.0/✅️valid) — the POSITIVE counterpart of the sibling
//! `🚫️no-doctype` negative asset: a well-formed document that also carries the `<!DOCTYPE ...>`
//! whose declared root name matches the document element, so `check_valid_conformance` reports no
//! hard issue. The ✳️any subset's own `🎬️demo` cannot serve here — it has no doctype at all.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🏷️.xml");
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
