//! 🎹️ JpgComposer (final, artifact-level) — union over every standard's composer entries.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
use crate::artifacts::jpg::standards::v_jfif_1_01::composer as v_jfif_1_01;

static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [&'static ComposerEntry] {
    ENTRIES.get_or_init(|| v_jfif_1_01::entries().iter().collect()).as_slice()
}

pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    let entry = entries()
        .iter()
        .find(|e| e.writes == target)
        .ok_or_else(|| ComposeError { message: format!("JpgComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
    (entry.compose)(sources)
}

pub fn register() {
    register_composer_entries(v_jfif_1_01::entries());
}
