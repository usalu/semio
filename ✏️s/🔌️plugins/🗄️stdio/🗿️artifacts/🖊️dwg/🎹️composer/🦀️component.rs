//! 🎹️ DwgComposer (final, artifact-level) — union over every standard's composer entries.
//! Keeps ac1018 registered alongside the new ac1024 (real R2004+ D1/D2 decode, ticket 26/08/10/
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION 🖊️dwg wave) purely additively:
//! several other plugins' own composer entries target `Dialect{standard: StandardId("ac1018")}`
//! directly, so dropping ac1018's registration here would silently shrink dispatch even though
//! nothing forced it to.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
use crate::artifacts::dwg::standards::v_ac1018::composer as v_ac1018;
use crate::artifacts::dwg::standards::v_ac1024::composer as v_ac1024;

static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

pub fn entries() -> &'static [&'static ComposerEntry] {
    ENTRIES.get_or_init(|| v_ac1018::entries().iter().chain(v_ac1024::entries().iter()).collect()).as_slice()
}

pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    let entry = entries()
        .iter()
        .find(|e| e.writes == target)
        .ok_or_else(|| ComposeError { message: format!("DwgComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
    (entry.compose)(sources)
}

pub fn register() {
    register_composer_entries(v_ac1018::entries());
    register_composer_entries(v_ac1024::entries());
}
