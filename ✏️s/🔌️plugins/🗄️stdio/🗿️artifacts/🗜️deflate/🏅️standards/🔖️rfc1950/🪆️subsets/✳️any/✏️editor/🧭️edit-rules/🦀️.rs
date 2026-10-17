//! 🧭️ The details-pane vocabulary of the deflate editor: which snapshot pointer raises which concrete kind. The three stream-header
//! fields travel together in `set-compression-params`.

use semio_s_artifact_stdio_contract::editing::{Carried, EditRules, EntityRule};

/// 📚 A header field raises the params kind carrying the two it leaves unchanged; the preset dictionary and the payload have their own kinds.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/compressionMethod", "set-compression-params", "method").carrying(&[Carried { payload: "window_bits", pointer: "/windowBits" }, Carried { payload: "level_hint", pointer: "/compressionLevelHint" }]),
        EntityRule::new("/windowBits", "set-compression-params", "window_bits").carrying(&[Carried { payload: "method", pointer: "/compressionMethod" }, Carried { payload: "level_hint", pointer: "/compressionLevelHint" }]),
        EntityRule::new("/compressionLevelHint", "set-compression-params", "level_hint").carrying(&[Carried { payload: "method", pointer: "/compressionMethod" }, Carried { payload: "window_bits", pointer: "/windowBits" }]),
        EntityRule::new("/dictId", "set-preset-dictionary", "dict_id"),
        EntityRule::new("/payload", "set-payload", "payload"),
    ],
    inserts: &[],
    removes: &[],
};

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
/// 🧬️ Artifact schema descriptor id.
pub const DEFLATE_ARTIFACT_SCHEMA_ID: &str = "s.stdio.deflate";
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("../../../../../../📜️artifact-definition.json");
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems8
pub mod artifact_items_8 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}
}
//#endregion 🧩️ArtifactItems8
