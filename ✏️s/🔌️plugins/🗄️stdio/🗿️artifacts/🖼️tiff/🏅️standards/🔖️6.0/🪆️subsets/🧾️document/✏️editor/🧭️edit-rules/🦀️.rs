//! 🧭️ The details-pane vocabulary of the tiff document editor: which snapshot pointer raises which concrete kind.
//! A tag's values raise `replace-tag` addressed by directory position and tag number; inserting or removing a tag entry raises `replace-tag` / `remove-tag`; inserting or removing a directory raises its positional kind.
//! A single sample of a block is a computed gesture answered by the editor itself as one `replace-samples` row; block geometry and tag numbers have no kind and are refused naming the path.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, ItemField, RemoveRule, Selector};

const DIRECTORY: &[Selector] = &[Selector::Index("ifdIndex")];
const TAG_OF_DIRECTORY: &[Selector] = &[Selector::Index("ifdIndex"), Selector::Field { payload: "tag", field: "tag" }];

/// 📚 Tag values by (directory position, tag number); directories and tag entries insert and remove positionally or by tag number.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[EntityRule::new("/ifds/*/entries/*/values", "replace-tag", "values").selecting(TAG_OF_DIRECTORY)],
    inserts: &[
        InsertRule::new("/ifds", "insert-ifd", "ifd").at("index"),
        InsertRule::new("/ifds/*/entries", "replace-tag", "").selecting(DIRECTORY).keyed(&[ItemField { payload: "tag", field: "tag" }, ItemField { payload: "values", field: "values" }]),
    ],
    removes: &[RemoveRule::by_index("/ifds", "remove-ifd", "index"), RemoveRule::by_key("/ifds/*/entries", "remove-tag", "tag", "tag").selecting(DIRECTORY)],
};

//#region 🧩️ArtifactItems
pub mod artifact_items {
/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_TIFF_DOCUMENT_SCHEMA: &str = "stdio.tiff";
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
/// 🧬️ Artifact schema descriptor id.
pub const TIFF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.tiff";
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("../../../../../../📜️artifact-definition.json");
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{STDIO_TIFF_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.tiff".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Tiff", "Tiff"),
        source_format: STDIO_TIFF_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_TIFF_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems4

//#region 🧩️ArtifactItems8
pub mod artifact_items_8 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}
}
//#endregion 🧩️ArtifactItems8
