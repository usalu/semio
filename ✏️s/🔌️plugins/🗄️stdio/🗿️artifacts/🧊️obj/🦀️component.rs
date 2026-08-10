//! 🎪 `stdio.obj` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::obj::schema::snapshot::ObjSnapshot;
pub use crate::artifacts::obj::schema::ObjArtifact;
pub use crate::artifacts::obj::schema::diff::ObjDiff;
pub use crate::artifacts::obj::schema::mutations::ObjMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_OBJ_DOCUMENT_SCHEMA: &str = "stdio.obj";

/// 🧬️ Artifact schema descriptor id.
pub const OBJ_ARTIFACT_SCHEMA_ID: &str = "s.stdio.obj";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.obj".into(),
        name: "Obj".into(),
        source_format: STDIO_OBJ_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_OBJ_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
