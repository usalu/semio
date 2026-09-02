//! 🧬️ Curate diff schema — sparse field delta over the artifact.

use crate::artifacts::curate::{CuratedItem, Filters, ObjectKindExtra};
use schema::ArtifactSchema;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the curate artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
/// `catalog`/`stock_extra` replace the former `stock: Option<CurateStockDelta>` — `catalog` is a
/// whole-handle replace (never incrementally patched: composed-child content changes through the
/// child's OWN history, never through this parent diff), `stock_extra` keeps the same id-keyed
/// added/removed/patched/reordered shape the old `stock` delta used.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.sourcing.curate")]
pub struct CurateDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::curate::schema::CurateArtifact>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.kit")]
    pub catalog: Option<store::ArtifactChild<SemioKitSnapshot>>,
    #[state(artifact)]
    pub stock_extra: Option<CurateStockExtraDelta>,
    #[state(artifact)]
    pub curated: Option<CurateCuratedDelta>,
    #[state(config)]
    pub filters: Option<Filters>,
    #[state(config)]
    pub locale: Option<String>,
    #[state(config)]
    pub contributions_json: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🩹 One patched stock-extra entry.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CurateObjectKindExtraPatchEntry {
    pub id: String,
    pub extra: ObjectKindExtra,
}

/// 🧩 Identified-collection delta for `stock_extra`.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CurateStockExtraDelta {
    pub added: Vec<ObjectKindExtra>,
    pub removed: Vec<String>,
    pub patched: Vec<CurateObjectKindExtraPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched curated entry.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CurateCuratedPatchEntry {
    pub object_id: String,
    pub count: Option<u32>,
}

/// 🧺 Identified-collection delta for `curated`.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CurateCuratedDelta {
    pub added: Vec<CuratedItem>,
    pub removed: Vec<String>,
    pub patched: Vec<CurateCuratedPatchEntry>,
    pub reordered: Option<Vec<String>>,
}
//#endregion 🔖️DeltaHelpers
