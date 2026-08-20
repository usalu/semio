//! 💡️ SemioTableInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📐shape/` — this subset's declared
//! column-kind vocabulary + row/column dimensions; the sibling `document`/`presentation` outline
//! shape does not apply here since table has no heading/text structure — a table's honest
//! structural summary is its dimensions and declared column kinds).

use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::shape::{compute_semio_table_shape, SemioTableShape};

//#region 🔖️Inference
/// 💡️ Everything inferable from a semio table snapshot. One field per named inference under
/// `💡️inferences/` (currently: `shape`, backed by the `📐shape/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.table.inference")]
pub struct SemioTableInference {
    #[derived]
    pub shape: SemioTableShape,
}

impl protocol::Inference<SemioTableSnapshot> for SemioTableInference {
    async fn infer(snapshot: &SemioTableSnapshot) -> Self {
        Self { shape: compute_semio_table_shape(snapshot).await }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `SemioTableSnapshot::default()` happens to be
/// all-empty today (no columns/rows), so a naive derive would happen to agree, but tying `Default`
/// to `infer` keeps the law correct even if that default ever stops being all-empty (the same
/// defensive pattern raster's `RasterInference` documents).
impl Default for SemioTableInference {
    fn default() -> Self {
        <Self as protocol::Inference<SemioTableSnapshot>>::infer(&SemioTableSnapshot::default())
    }
}

impl protocol::InferenceSpec<SemioTableSnapshot> for SemioTableInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.semio.table.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.table.inference.shape", reads: &["columns", "rows"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (a column-kind census is a single whole-snapshot fold over already-
/// flat `columns`, `rowCount` a single length read) — the default `infer_cached` passthrough
/// (`ArtifactInferrer::infer_cached`) is exact.
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::table::schema::SemioTableBuilder {
    type Snapshot = SemioTableSnapshot;
    type Inference = SemioTableInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.table.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `semio_table_artifact_schema_descriptor`'s registration.
pub async fn semio_table_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.table.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = SemioTableSnapshot::default();
        assert_eq!(SemioTableInference::infer(&snapshot), SemioTableInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioTableInference::infer(&SemioTableSnapshot::default()), SemioTableInference::default());
    }
}
//#endregion 🧪️Tests
