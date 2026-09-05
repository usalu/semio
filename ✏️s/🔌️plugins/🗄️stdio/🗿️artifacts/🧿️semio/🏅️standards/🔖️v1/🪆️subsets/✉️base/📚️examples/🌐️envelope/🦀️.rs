//! 📚️ Example "envelope" for `stdio.semio` (the `✉️base` union over all 13 domain subsets) — the
//! first real, non-hex-of-JSON fixture for this subset
//! (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION's `any` closer wave).
//! `PRIMARY_TEXT` is the genuine `SemioSnapshot::print_dsl` output for
//! `snapshot::demo_semio_snapshot()` (`🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/
//! 🦀️.rs`, itself wrapping `flow`'s own real demo snapshot) — asserted byte-identical
//! to it by that subset's own `fixture_honesty_law` (`🎹️composer/🦀️.rs`), so this fixture
//! can never silently drift back to a fake.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "envelope";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel { LocalizedLabel::native("Envelope", "Envelope") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️.dsl.semio");
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }

#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn envelope_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }
}
