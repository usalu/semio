//! 📚️ Example "memo" for `stdio.semio.document` — the first real, non-hex-scaffold fixture for
//! this subset (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION's document wave).
//! `PRIMARY_TEXT` is the genuine `SemioDocumentSnapshot::print_dsl` output for
//! `snapshot::demo_semio_document_snapshot()` (`🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/
//! 📸️snapshot/🦀️component.rs`) — asserted byte-identical to it by that subset's own
//! `fixture_honesty_law` (`🎹️composer/🦀️component.rs`), so this fixture can never silently drift
//! back to a fake.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "memo";
pub async fn label() -> LocalizedLabel { LocalizedLabel::native("Memo", "Memo") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
pub async fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }

#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn memo_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }
}
