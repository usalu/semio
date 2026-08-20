//! 📚️ Example "cube" for `stdio.semio.mesh` — the first real, non-hex-scaffold fixture for this
//! subset (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION's mesh wave).
//! `PRIMARY_TEXT` is the genuine `SemioMeshSnapshot::print_dsl` output for
//! `engine::demo_mesh_snapshot()` (`🏅️standards/🔖️v1/🪆️subsets/✳️mesh/⚙️engine/
//! 🦀️component.rs`) — asserted byte-identical to it by this subset's own `fixture_honesty_law`
//! (`🚪️io/🦀️component.rs`), so this fixture can never silently drift back to a fake.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "cube";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Cube", "Cube")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON).await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn cube_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }
}
