//! 👁️ Zip viewer (2.0/🧱️base) — the `view` mode: a single read-only window over the archive tree, the
//! read-only counterpart of the editor's `edit` mode.

use crate::viewer::zip::base::modes::view::windows::main;
use semio_framework_plugin::create_stack_layout;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::WindowLayout;

pub const ZIP_ANY_VIEW_MODE_ID: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::zip::base::create_zip_any_viewer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: ZIP_ANY_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ One window, one layout slot — mirrors the editor's own default layout shape minus the
/// mutation-shaped window kind.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn layout() -> WindowLayout {
    create_stack_layout(&[main::WINDOW_KIND_ID.into()], Some(&["Archive".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🧩️ArtifactItems
pub mod artifact_items {
/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_ZIP_DOCUMENT_SCHEMA: &str = "stdio.zip";
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
/// 🧬️ Artifact schema descriptor id.
pub const ZIP_ARTIFACT_SCHEMA_ID: &str = "s.stdio.zip";
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("../../../../../../../📜️artifact-definition.json");
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{STDIO_ZIP_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.zip".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Zip", "Zip"),
        source_format: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_ZIP_DOCUMENT_SCHEMA.into(),
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
