//! 🧭️ The details-pane vocabulary of the valid xml editor: which snapshot pointer raises which concrete kind. A text node edit
//! (addressed by a child-index path through the document) is a computed gesture answered by the editor itself.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, ItemField};

/// 📚 Every kind keeps §2.8 and §2.9 closed: the document element is renamed together with the doctype, and the doctype is edited only through its two halves.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/doc/root/name", "rename-document-element", "name"),
        EntityRule::new("/doc/doctype/externalId", "set-external-subset", "externalId"),
        EntityRule::new("/doc/doctype/declarations", "set-internal-subset", "declarations"),
        EntityRule::new("/doc/declaration/standalone", "set-standalone", "standalone"),
    ],
    inserts: &[InsertRule::new("/doc/doctype/declarations", "declare-entity", "").at("index").keyed(&[ItemField { payload: "parameter", field: "parameter" }, ItemField { payload: "name", field: "name" }, ItemField { payload: "value", field: "value" }])],
    removes: &[],
};

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
/// 🧬️ Artifact schema descriptor id.
pub const XML_ARTIFACT_SCHEMA_ID: &str = "s.stdio.xml";
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
use crate::{STDIO_XML_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.xml".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Xml", "Xml"),
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
