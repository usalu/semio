//! 📚️ Example "note" for `stdio.semio.text` — the first real, non-scaffold fixture for this
//! subset (ticket UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, W2a/text). `PRIMARY_TEXT` is the genuine
//! `SemioTextSnapshot::print_dsl` output for `snapshot::demo_text_snapshot()`
//! (`🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/📸️snapshot/🦀️component.rs`) — asserted
//! byte-identical to it by that subset's own `fixture_honesty_law` (`🚪️io/🦀️component.rs`), so
//! this fixture can never silently drift back to a fake.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "note";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Note", "Notiz")
}
pub const ICON: &str = "file-text";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn note_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }
}
