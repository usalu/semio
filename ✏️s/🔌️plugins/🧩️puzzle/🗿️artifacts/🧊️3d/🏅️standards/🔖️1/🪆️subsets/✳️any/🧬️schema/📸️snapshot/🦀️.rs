//! 🧬️ Puzzle3d snapshot schema — artifact-lane fields only.

use crate::{Puzzle3dAttraction, Puzzle3dMeta, Puzzle3dObject, Puzzle3dReference, Puzzle3dTargetVolume, PUZZLE_3D_SCHEMA};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted puzzle3d document snapshot (persistent fields of the artifact).
///
/// 🩹️ `Serialize`/`Deserialize` are test-only (ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`): production JSON
/// import/export routes through `dsl::ToValue`/`dsl::FromValue` (see `🚪️io/📥️import`/`📤️export`'s
/// `🔣️json` leaves and `🧬️mutations`'s `🔖️ValueBridge`/`🔖️PlaySnapshot` regions), never
/// `serde_json::to_value`/`from_value` on this type directly anymore. `serde` stays derived under
/// `#[cfg(test)]` only, as the differential oracle the `🧪️tests/**` scene_snapshot suite still checks
/// this type's wire shape against.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(id = "puzzle.puzzle3d", layout = "lines")]
#[artifact_schema(id = "s.puzzle.puzzle3d")]
pub struct Puzzle3dSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[state(artifact)]
    pub domain: String,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(block)]
    #[state(artifact)]
    pub meta: Puzzle3dMeta,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub objects: Vec<Puzzle3dObject>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub attractions: Vec<Puzzle3dAttraction>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub target_volumes: Vec<Puzzle3dTargetVolume>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub references: Vec<Puzzle3dReference>,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Puzzle3dSnapshot {
    fn default() -> Self {
        Self { schema: PUZZLE_3D_SCHEMA.to_string(), domain: "architecture".to_string(), meta: Default::default(), objects: Vec::new(), attractions: Vec::new(), target_volumes: Vec::new(), references: Vec::new() }
    }
}



