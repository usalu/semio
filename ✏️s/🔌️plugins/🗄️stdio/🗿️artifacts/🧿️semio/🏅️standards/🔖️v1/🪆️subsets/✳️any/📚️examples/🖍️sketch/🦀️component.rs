//! 📚️ Example "sketch" for `stdio.semio.drawing` — the first real, non-hex-scaffold fixture for
//! this subset (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION's drawing wave).
//! `PRIMARY_TEXT` is the genuine `SemioDrawingSnapshot::print_dsl` output for
//! `snapshot::demo_drawing_snapshot()` (`🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/📸️snapshot/
//! 🦀️component.rs`) — asserted byte-identical to it by that subset's own `fixture_honesty_law`
//! (`🎹️composer/🦀️component.rs`), so this fixture can never silently drift back to a fake.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "sketch";
pub async fn label() -> LocalizedLabel { LocalizedLabel::native("Sketch", "Sketch") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
pub async fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    async fn sketch_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }
}
