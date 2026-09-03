//! 📚️ Example "walk" for `stdio.semio.animation` — the first real, non-hex-scaffold fixture for
//! this subset (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION's animation wave).
//! `PRIMARY_TEXT` is the genuine `SemioAnimationSnapshot::print_dsl` output for
//! `snapshot::demo_animation_snapshot()` (`🏅️standards/🔖️v1/🪆️subsets/✳️animation/🧬️schema/
//! 📸️snapshot/🦀️.rs`) — asserted byte-identical to it by that subset's own
//! `fixture_honesty_law` (`🎹️composer/🦀️.rs`), so this fixture can never silently drift
//! back to a fake.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "walk";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel { LocalizedLabel::native("Walk", "Walk") }
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️.dsl.semio");
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource { ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON) }

#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn walk_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }
}
