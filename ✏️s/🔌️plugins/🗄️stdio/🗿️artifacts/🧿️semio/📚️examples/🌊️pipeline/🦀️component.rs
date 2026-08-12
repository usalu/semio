//! 📚️ Example "pipeline" for `stdio.semio.flow` — the first real, non-hex-scaffold fixture for
//! this subset (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION's flow pilot).
//! `PRIMARY_TEXT` is the genuine `SemioFlowSnapshot::print_dsl` output for
//! `snapshot::demo_flow_snapshot()` (`🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/📸️snapshot/
//! 🦀️component.rs`) — asserted byte-identical to it by that subset's own `fixture_honesty_law`
//! (`🎹️composer/🦀️component.rs`), so this fixture can never silently drift back to a fake.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "pipeline";
pub fn label() -> LocalizedLabel { LocalizedLabel::native("Pipeline", "Pipeline") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pipeline_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }
}
