//! 🧭️ The details-pane vocabulary of the binary editor: every pointer edit of `/bytes` is answered by the editor as the byte splice it
//! means (a byte set, insert, remove or move; the whole list set replaces the buffer), so the table itself is empty.

use semio_s_artifact_stdio_contract::editing::EditRules;

/// 📚 No fixed pointer shape: see the editor's `snapshot_edit_special`.
pub const EDIT_RULES: EditRules = EditRules { entities: &[], inserts: &[], removes: &[] };

//#region 🧩️ArtifactItems
pub mod artifact_items {
/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_BINARY_DOCUMENT_SCHEMA: &str = "stdio.binary";
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
/// 🧬️ Artifact schema descriptor id.
pub const BINARY_ARTIFACT_SCHEMA_ID: &str = "s.stdio.binary";
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
use crate::{ARTIFACT_DEFINITION_SCHEMA};
use semio_framework_plugin::{ArtifactDefinition, PluginAssemblyError};

pub fn definition() -> Result<ArtifactDefinition, PluginAssemblyError> {
    semio_s_artifact_stdio_contract::definition_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}
}
//#endregion 🧩️ArtifactItems4

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};
use semio_framework_plugin::io::FormatDescriptor;
use semio_framework_plugin::ArtifactDefinitionError;

pub fn formats() -> Result<Vec<FormatDescriptor>, ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}
}
//#endregion 🧩️ArtifactItems5

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    Vec::new()
}
}
//#endregion 🧩️ArtifactItems6
