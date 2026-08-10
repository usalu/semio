//! 🎹️ PdfComposer (final, artifact-level) — union over every standard's composer entries.
//! Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: 1.7 lands
//! alongside 1.4 (Decision #5: 1.7 folds 1.4 in by reading leniently, but stays a *separate*
//! dialect-keyed entry here rather than replacing 1.4 -- same "both standards coexist,
//! dialect-keyed" shape as gif's 87a/89a top-level composer).

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
use crate::artifacts::pdf::standards::v1_4::composer as v1_4;
use crate::artifacts::pdf::standards::v1_7::composer as v1_7;

static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [&'static ComposerEntry] {
    ENTRIES.get_or_init(|| v1_4::entries().iter().chain(v1_7::entries().iter()).collect()).as_slice()
}

pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    let entry = entries()
        .iter()
        .find(|e| e.writes == target)
        .ok_or_else(|| ComposeError { message: format!("PdfComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
    (entry.compose)(sources)
}

pub fn register() {
    register_composer_entries(v1_4::entries());
    register_composer_entries(v1_7::entries());
}
