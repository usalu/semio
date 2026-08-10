//! 🎪 `stdio.xml` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::xml::schema::snapshot::XmlSnapshot;
pub use crate::artifacts::xml::schema::XmlArtifact;
pub use crate::artifacts::xml::schema::diff::XmlDiff;
pub use crate::artifacts::xml::schema::mutations::XmlMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_XML_DOCUMENT_SCHEMA: &str = "stdio.xml";

/// 🧬️ Artifact schema descriptor id.
pub const XML_ARTIFACT_SCHEMA_ID: &str = "s.stdio.xml";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.xml".into(),
        name: "Xml".into(),
        source_format: STDIO_XML_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Document },
        schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
