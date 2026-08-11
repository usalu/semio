//! 🎪 `stdio.mp4` artifact — new-format artifact (master plan "New format artifacts" table).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
pub use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::Mp4Artifact;
pub use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::diff::Mp4Diff;
pub use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::mutations::Mp4Mutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_MP4_DOCUMENT_SCHEMA: &str = "stdio.mp4";

/// 🧬️ Artifact schema descriptor id.
pub const MP4_ARTIFACT_SCHEMA_ID: &str = "s.stdio.mp4";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.mp4".into(),
        name: "Mp4".into(),
        source_format: STDIO_MP4_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_MP4_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
