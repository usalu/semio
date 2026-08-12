//! 📚️ Example "graph" for `stdio.semio.object` — the first real, non-hex-of-JSON fixture for this
//! subset (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION's object wave).
//! `PRIMARY_TEXT` is the genuine `SemioObjectSnapshot::print_dsl` output for
//! `snapshot::demo_semio_object_snapshot()` (`🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/📸️snapshot/
//! 🦀️component.rs`) — asserted byte-identical to it by that subset's own `fixture_honesty_law`
//! (`🎹️composer/🦀️component.rs`), so this fixture can never silently drift back to a fake.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "graph";
pub fn label() -> LocalizedLabel { LocalizedLabel::native("Graph", "Graph") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn graph_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }
}
