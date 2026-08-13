//! 🧬️ Curate diff schema — sparse field delta over the artifact.

use crate::artifacts::curate::{CuratedItem, Filters, ObjectKind};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the curate artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.sourcing.curate")]
pub struct CurateDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::curate::schema::CurateArtifact>>,
    #[state(artifact)]
    pub stock: Option<CurateStockDelta>,
    #[state(artifact)]
    pub curated: Option<CurateCuratedDelta>,
    #[state(config)]
    pub filters: Option<Filters>,
    #[state(presence)]
    pub selected_object_id: Option<Option<String>>,
    #[state(config)]
    pub locale: Option<String>,
    #[state(config)]
    pub contributions_json: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🩹 One patched stock entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurateObjectKindPatchEntry {
    pub id: String,
    pub kind: ObjectKind,
}

/// 🧩 Identified-collection delta for `stock`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CurateStockDelta {
    pub added: Vec<ObjectKind>,
    pub removed: Vec<String>,
    pub patched: Vec<CurateObjectKindPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched curated entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurateCuratedPatchEntry {
    pub object_id: String,
    pub count: Option<u32>,
}

/// 🧺 Identified-collection delta for `curated`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CurateCuratedDelta {
    pub added: Vec<CuratedItem>,
    pub removed: Vec<String>,
    pub patched: Vec<CurateCuratedPatchEntry>,
    pub reordered: Option<Vec<String>>,
}
//#endregion 🔖️DeltaHelpers
