//! 💡️ SemioInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🏷️kind/`). `✳️any` is the envelope
//! union over all 18 domain subsets, so — unlike every domain subset's own inference (which reads
//! that subset's real geometry/graph/text shape) — the only thing honestly inferable from the
//! ENVELOPE alone (not the wrapped subset's own internals) is which subset it dispatches to: the
//! same `subset_tag`/`subset_ordinal` pair `📸️snapshot/🦀️.rs`'s own DSL header/binary-pack
//! codecs already compute from `SemioSubsetSnapshot`, reused here (not re-derived) as this
//! envelope's honest "union/dispatch shape" inference per the ticket's own naming guidance.

use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::SemioSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::kind::{compute_semio_kind, SemioKind};

//#region 🔖️Inference
/// 💡️ Everything inferable from the semio envelope snapshot. One field per named inference under
/// `💡️inferences/` (currently: `kind`, backed by the `🏷️kind/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.inference")]
pub struct SemioInference {
    #[derived]
    pub kind: SemioKind,
}

impl protocol::Inference<SemioSnapshot> for SemioInference {
    fn infer(snapshot: &SemioSnapshot) -> Self {
        Self { kind: compute_semio_kind(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `SemioSnapshot::default()`'s `subset` defaults
/// to `SemioSubsetSnapshot::Brep(..)` (the enum's first-declared variant, not a zero/unit value a
/// naive derive could reconstruct), so `Default` MUST be tied to `infer`, never derived.
impl Default for SemioInference {
    fn default() -> Self {
        <Self as protocol::Inference<SemioSnapshot>>::infer(&SemioSnapshot::default())
    }
}

impl protocol::InferenceSpec<SemioSnapshot> for SemioInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.semio.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.semio.inference.kind", reads: &["subset"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `kind` is a single O(1) tag/ordinal read off the already-decoded
/// `subset` enum discriminant, nothing to incrementally cache — the default `infer_cached`
/// passthrough is exact.
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::any::schema::SemioBuilder {
    type Snapshot = SemioSnapshot;
    type Inference = SemioInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `semio_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
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
        let snapshot = SemioSnapshot::default();
        assert_eq!(SemioInference::infer(&snapshot), SemioInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(SemioInference::infer(&SemioSnapshot::default()), SemioInference::default());
    }
}
//#endregion 🧪️Tests
