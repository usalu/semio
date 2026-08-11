//! 🎹️ IfcComposer (final, artifact-level) — union over every standard's composer entries.
//! Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES: `2x3` (buildingSMART Coordination
//! View 2.0 era) lands alongside `4` as a full second standard -- same "both standards coexist,
//! dialect-keyed" shape as gif's 87a/89a and pdf's 1.4/1.7 top-level composers.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
use crate::artifacts::ifc::standards::v4::composer as v4;
use crate::artifacts::ifc::standards::v2x3::composer as v2x3;

static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [&'static ComposerEntry] {
    ENTRIES.get_or_init(|| v4::entries().iter().chain(v2x3::entries().iter()).collect()).as_slice()
}

pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    let entry = entries()
        .iter()
        .find(|e| e.writes == target)
        .ok_or_else(|| ComposeError { message: format!("IfcComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
    (entry.compose)(sources)
}

pub fn register() {
    register_composer_entries(v4::entries());
    register_composer_entries(v2x3::entries());
}
