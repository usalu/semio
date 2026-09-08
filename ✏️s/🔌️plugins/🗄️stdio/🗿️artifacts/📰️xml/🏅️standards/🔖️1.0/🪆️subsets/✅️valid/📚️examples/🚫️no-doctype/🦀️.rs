//! 📚️ Negative example — well-formed XML without `<!DOCTYPE>` (fails ✳️valid hard gate).

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "no-doctype";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("No doctype (invalid for valid)", "Kein Doctype (ungültig für valid)")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🏷️.xml");
pub const EXPECTED_HARD_CODES: &[&str] = &["stdio.xml.valid.doctype-missing"];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
