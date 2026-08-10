//! 🎹️ GifComposer (final, artifact-level) — union over every standard's composer entries.

use std::sync::OnceLock;
use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
use crate::artifacts::gif::standards::v87a::composer as v87a;
use crate::artifacts::gif::standards::v89a::composer as v89a;

static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

/// 🎹️ Both standards' entries, dialect-keyed (`writes.standard == "87a"` vs `"89a"`) — this is
/// how a caller reaches 89a's real multi-frame codec: `compose` below picks the entry whose
/// `writes` matches the requested `Dialect`, so 87a and 89a coexist without either shadowing
/// the other (unlike the flat schema/document-codec registries, which are standard-agnostic
/// pre-D4 and would collide — see `standards::v89a::engine::register`'s doc comment).
pub fn entries() -> &'static [&'static ComposerEntry] {
    ENTRIES.get_or_init(|| v87a::entries().iter().chain(v89a::entries().iter()).collect()).as_slice()
}

pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
    let entry = entries()
        .iter()
        .find(|e| e.writes == target)
        .ok_or_else(|| ComposeError { message: format!("GifComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
    (entry.compose)(sources)
}

pub fn register() {
    register_composer_entries(v87a::entries());
    register_composer_entries(v89a::entries());
}
