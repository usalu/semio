//! 🧬️ Puzzle2d snapshot schema — artifact-lane fields only.



use crate::{Puzzle2dCamera, Puzzle2dEdge, Puzzle2dMeta, Puzzle2dNode, Puzzle2dTargetRegion, PUZZLE_2D_SCHEMA};
use ::semio_framework_schema::ArtifactSchema;


//#region 🔖️Snapshot
/// 📸️ Persisted puzzle2d document snapshot (persistent fields of the artifact).
///
/// 🩹️ `Serialize`/`Deserialize` are test-only (ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`): production JSON
/// import/export routes through `dsl::ToValue`/`dsl::FromValue` (see `🚪️io/📥️import`/`📤️export`'s
/// `🔣️json` leaves and `🧬️mutations`'s `🔖️ValueBridge`/`🔖️PlaySnapshot` regions), never
/// `serde_json::to_value`/`from_value` on this type directly anymore. `serde` stays derived under
/// `#[cfg(test)]` only, as the differential oracle the `🧪️tests/**` snapshot suite still checks
/// this type's wire shape against.
///
/// 🎯️ `target_regions` is the one collection skipped when empty (unlike `nodes`/`edges`): every
/// document written before target regions existed stays byte-identical, so the whole committed
/// snapshot corpus remains canonical.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(id = "puzzle.puzzle2d", layout = "lines")]
#[artifact_schema(id = "s.puzzle.puzzle2d")]
pub struct Puzzle2dSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(block)]
    #[state(artifact)]
    pub camera: Puzzle2dCamera,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub nodes: Vec<Puzzle2dNode>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub edges: Vec<Puzzle2dEdge>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    #[dsl(table)]
    #[state(artifact)]
    pub target_regions: Vec<Puzzle2dTargetRegion>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(block)]
    #[state(artifact)]
    pub meta: Puzzle2dMeta,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Puzzle2dSnapshot {
    fn default() -> Self {
        Self { schema: PUZZLE_2D_SCHEMA.to_string(), camera: Default::default(), nodes: Vec::new(), edges: Vec::new(), target_regions: Vec::new(), meta: Default::default() }
    }
}
