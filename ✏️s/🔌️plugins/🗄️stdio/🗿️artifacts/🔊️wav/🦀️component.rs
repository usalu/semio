//! 🎪 `stdio.wav` artifact — new-format artifact (master plan "New format artifacts" table).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot;
pub use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::WavArtifact;
pub use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::diff::WavDiff;
pub use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::mutations::WavMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_WAV_DOCUMENT_SCHEMA: &str = "stdio.wav";

/// 🧬️ Artifact schema descriptor id.
pub const WAV_ARTIFACT_SCHEMA_ID: &str = "s.stdio.wav";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.wav".into(),
        name: "Wav".into(),
        source_format: STDIO_WAV_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_WAV_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
