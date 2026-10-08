//! 🧬️ Puzzle2d snapshot schema — artifact-lane fields only.



use semio_framework_value::{list::PagedList, paged::PagedUtf8};

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
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(id = "puzzle.puzzle2d", layout = "lines")]
#[artifact_schema(id = "s.puzzle.puzzle2d")]
pub struct Puzzle2dSnapshot {
    #[state(artifact)]
    pub schema: PagedUtf8<{ usize::MAX }>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(block)]
    #[state(artifact)]
    pub camera: Puzzle2dCamera,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub nodes: PagedList<Puzzle2dNode, { usize::MAX }>,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(table)]
    #[state(artifact)]
    pub edges: PagedList<Puzzle2dEdge, { usize::MAX }>,
    #[value(default, skip_serializing_if = "PagedList::is_empty")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "PagedList::is_empty"))]
    #[dsl(table)]
    #[state(artifact)]
    pub target_regions: PagedList<Puzzle2dTargetRegion, { usize::MAX }>,
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
        Self { schema: PUZZLE_2D_SCHEMA.into(), camera: Default::default(), nodes: PagedList::new(), edges: PagedList::new(), target_regions: PagedList::new(), meta: Default::default() }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🧬️retained-clone/🦀️.rs"]
mod retained_clone_tests;

#[path = "🔎️lookup/🦀️.rs"]
pub mod lookup;
