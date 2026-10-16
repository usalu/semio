//! 🧭️ The details-pane vocabulary of the epw editor: which snapshot pointer raises which concrete kind.
//! A record column edit (its column index is the key's position in the record) is a computed gesture answered by the editor itself.

use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, InsertRule, RemoveRule};

/// 📚 Every header line is one entity of its own kind; records are inserted and removed by position.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[
        EntityRule::new("/location", "set-location", "location"),
        EntityRule::new("/designConditions", "set-design-conditions", "value"),
        EntityRule::new("/typicalExtremePeriods", "set-typical-extreme-periods", "value"),
        EntityRule::new("/groundTemperatures", "set-ground-temperatures", "value"),
        EntityRule::new("/holidaysDst", "set-holidays-dst", "value"),
        EntityRule::new("/comments1", "set-comments1", "value"),
        EntityRule::new("/comments2", "set-comments2", "value"),
        EntityRule::new("/dataPeriods", "set-data-periods", "dataPeriods"),
    ],
    inserts: &[InsertRule::new("/records", "insert-record", "record").at("index")],
    removes: &[RemoveRule::by_index("/records", "remove-record", "index")],
};

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
/// 🧬️ Artifact schema descriptor id.
pub const EPW_ARTIFACT_SCHEMA_ID: &str = "s.stdio.epw";
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems8
pub mod artifact_items_8 {
pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    Vec::new()
}
}
//#endregion 🧩️ArtifactItems8
