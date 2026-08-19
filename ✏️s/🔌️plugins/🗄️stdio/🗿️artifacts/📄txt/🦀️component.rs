//! 🎪 `stdio.txt` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::txt::schema::diff::TxtDiff;
pub use crate::artifacts::txt::schema::mutations::TxtMutation;
pub use crate::artifacts::txt::schema::snapshot::TxtSnapshot;
pub use crate::artifacts::txt::schema::TxtArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_TXT_DOCUMENT_SCHEMA: &str = "stdio.txt";

/// 🧬️ Artifact schema descriptor id.
pub const TXT_ARTIFACT_SCHEMA_ID: &str = "s.stdio.txt";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub async fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::definition_only_assembly("txt", definition)
}

//#region 🔖️ArtifactDeclaration
/// 🌳️ New tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P pilot): the whole
/// `s.stdio.txt` artifact through the declaration tree — one standard, `utf-8`, one subset,
/// `any`. See `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🦀️component.rs`'s `artifact()` doc
/// comment for the `localization: &[]` deferral rationale (identical here).
pub async fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.stdio.txt").expect("canonical stdio.txt kind"), localization: &[], standards: vec![crate::artifacts::txt::standards::v_utf_8::standard()] }
}
//#endregion 🔖️ArtifactDeclaration

pub async fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.txt".into(),
        name: "Txt".into(),
        source_format: STDIO_TXT_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_TXT_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::txt::standards::v_utf_8::subsets::any::io::io_registry as v_utf_8;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub async fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v_utf_8::entries().iter().collect()).as_slice()
    }

    pub async fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("TxtComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    pub async fn register() {
        let _ = register_composer_entries(v_utf_8::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
