//! 📚️ Example "memo" for `stdio.semio.document` — the first real, non-hex-scaffold fixture for
//! this subset (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION's document wave).
//! `PRIMARY_TEXT` is the genuine `SemioDocumentSnapshot::print_dsl` output for
//! `snapshot::demo_semio_document_snapshot()` (`🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/
//! 📸️snapshot/🦀️.rs`) — asserted byte-identical to it by that subset's own
//! `fixture_honesty_law` (`🎹️composer/🦀️.rs`), so this fixture can never silently drift
//! back to a fake.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "memo";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel { LocalizedLabel::native("Memo", "Memo") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🗒️memo/🗣️.dsl.semio");
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
