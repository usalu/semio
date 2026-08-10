//! 🎪 `stdio.svg` artifact — stdio reference format.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::svg::schema::snapshot::SvgSnapshot;
pub use crate::artifacts::svg::schema::SvgArtifact;
pub use crate::artifacts::svg::schema::diff::SvgDiff;
pub use crate::artifacts::svg::schema::mutations::SvgMutation;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_SVG_DOCUMENT_SCHEMA: &str = "stdio.svg";

/// 🧬️ Artifact schema descriptor id.
pub const SVG_ARTIFACT_SCHEMA_ID: &str = "s.stdio.svg";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.svg".into(),
        name: "Svg".into(),
        source_format: STDIO_SVG_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Document },
        schema: STDIO_SVG_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
