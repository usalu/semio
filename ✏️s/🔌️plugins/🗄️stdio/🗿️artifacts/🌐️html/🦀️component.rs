//! 🎪 `stdio.html` artifact — new-format artifact (master plan "New format artifacts" table).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;
pub use crate::artifacts::html::standards::v5::subsets::any::schema::HtmlArtifact;
pub use crate::artifacts::html::standards::v5::subsets::any::schema::diff::HtmlDiff;
pub use crate::artifacts::html::standards::v5::subsets::any::schema::mutations::HtmlMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_HTML_DOCUMENT_SCHEMA: &str = "stdio.html";

/// 🧬️ Artifact schema descriptor id.
pub const HTML_ARTIFACT_SCHEMA_ID: &str = "s.stdio.html";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.html".into(),
        name: "Html".into(),
        source_format: STDIO_HTML_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_HTML_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
