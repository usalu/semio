//! 🎹️ GlbComposer (final, artifact-level) — union over every standard's composer entries.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
use crate::artifacts::glb::standards::v2_0::composer as v2_0;

static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [&'static ComposerEntry] {
    ENTRIES.get_or_init(|| v2_0::entries().iter().collect()).as_slice()
}

pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    let entry = entries()
        .iter()
        .find(|e| e.writes == target)
        .ok_or_else(|| ComposeError { message: format!("GlbComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
    (entry.compose)(sources)
}

pub fn register() {
    register_composer_entries(v2_0::entries());
}
