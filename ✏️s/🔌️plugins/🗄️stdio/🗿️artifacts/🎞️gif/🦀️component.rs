//! 🎪 `stdio.gif` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::gif::schema::snapshot::GifSnapshot;
pub use crate::artifacts::gif::schema::GifArtifact;
pub use crate::artifacts::gif::schema::diff::GifDiff;
pub use crate::artifacts::gif::schema::mutations::GifMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_GIF_DOCUMENT_SCHEMA: &str = "stdio.gif";

/// 🧬️ Artifact schema descriptor id.
pub const GIF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.gif";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.gif".into(),
        name: "Gif".into(),
        source_format: STDIO_GIF_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_GIF_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::gif::standards::v87a::engine::io_registry as v87a;
    use crate::artifacts::gif::standards::v89a::engine::io_registry as v89a;

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
}
//#endregion 🚪️DerivedIoRegistry
