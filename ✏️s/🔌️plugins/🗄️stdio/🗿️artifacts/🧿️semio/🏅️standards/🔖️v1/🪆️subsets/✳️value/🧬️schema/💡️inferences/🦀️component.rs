//! 💡️ SemioValueInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🌳census/` — this subset's own
//! ordered, lexeme-preserving typed value GRAPH's real variant census + graph depth, the honest
//! structural summary of a NEUTRAL semio type with no on-disk file format of its own).
//!
//! ⚠️ RENAME TRAP: this `value` subset IS the old value-tree `object`, renamed earlier in this
//! ticket — unrelated to the brand-new spatial `✳️object` subset (transform + owned brep/mesh/
//! value children).

use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::census::{compute_semio_value_census, SemioValueCensus};

//#region 🔖️Inference
/// 💡️ Everything inferable from a semio value snapshot. One field per named inference under
/// `💡️inferences/` (currently: `census`, backed by the `🌳census/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.value.inference")]
pub struct SemioValueInference {
    #[derived]
    pub census: SemioValueCensus,
}

impl protocol::Inference<SemioValueSnapshot> for SemioValueInference {
    async fn infer(snapshot: &SemioValueSnapshot) -> Self {
        Self { census: compute_semio_value_census(snapshot) }
    }
}

/// 🩹 Hand-rolled, NOT derived — `SemioValueSnapshot::default()`'s `root` is `SemioValue::Null`
/// (`SemioValue::default()`), never absent (`root` is not an `Option`), so a default snapshot
/// already contains ONE real value node. A naive `#[derive(Default)]` on `SemioValueInference`
/// would give an all-zero census, which disagrees with `infer(&SemioValueSnapshot::default())`
/// (`nullCount: 1`) — exactly the trap this ticket's own `📌️important.md` names for a non-empty
/// snapshot default.
impl Default for SemioValueInference {
    fn default() -> Self {
        <Self as protocol::Inference<SemioValueSnapshot>>::infer(&SemioValueSnapshot::default())
    }
}

impl protocol::InferenceSpec<SemioValueSnapshot> for SemioValueInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.semio.value.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.value.inference.census", reads: &["root", "nodes"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (a variant census + depth is a single whole-graph recursive fold,
/// same shape `flow`'s/`graph`'s own whole-graph topology facets reach for their own graphs) — the
/// default `infer_cached` passthrough (`ArtifactInferrer::infer_cached`) is exact.
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::value::schema::SemioValueBuilder {
    type Snapshot = SemioValueSnapshot;
    type Inference = SemioValueInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.value.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `semio_value_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_value_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.value.inference",
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
        let snapshot = SemioValueSnapshot::default();
        assert_eq!(SemioValueInference::infer(&snapshot), SemioValueInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioValueInference::infer(&SemioValueSnapshot::default()), SemioValueInference::default());
    }
}
//#endregion 🧪️Tests
