//! 🧬️ Curation diff schema — sparse field delta over the artifact.

use crate::artifacts::curation::{CuratedItem, Filters, ObjectKindExtra};
use schema::ArtifactSchema;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the curation artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
/// `catalog`/`stock_extra` replace the former `stock: Option<CurationStockDelta>` — `catalog` is a
/// whole-handle replace (never incrementally patched: composed-child content changes through the
/// child's OWN history, never through this parent diff), `stock_extra` keeps the same id-keyed
/// added/removed/patched/reordered shape the old `stock` delta used.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.sourcing.curation")]
pub struct CurationDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::curation::schema::CurationArtifact>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.kit")]
    pub catalog: Option<store::ArtifactChild<SemioKitSnapshot>>,
    #[state(artifact)]
    pub stock_extra: Option<CurationStockExtraDelta>,
    #[state(artifact)]
    pub curated: Option<CurationCuratedDelta>,
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
pub struct CurationObjectKindExtraPatchEntry {
    pub id: String,
    pub extra: ObjectKindExtra,
}

/// 🧩 Identified-collection delta for `stock_extra`.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CurationStockExtraDelta {
    pub added: Vec<ObjectKindExtra>,
    pub removed: Vec<String>,
    pub patched: Vec<CurationObjectKindExtraPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched curated entry.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CurationCuratedPatchEntry {
    pub object_id: String,
    pub count: Option<u32>,
}

/// 🧺 Identified-collection delta for `curated`.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CurationCuratedDelta {
    pub added: Vec<CuratedItem>,
    pub removed: Vec<String>,
    pub patched: Vec<CurationCuratedPatchEntry>,
    pub reordered: Option<Vec<String>>,
}
//#endregion 🔖️DeltaHelpers
