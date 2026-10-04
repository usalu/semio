//! 💡️ SemioInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🏷️kind/`). `✉️base` is the envelope
//! union over all 18 domain subsets, so — unlike every domain subset's own inference (which reads
//! that subset's real geometry/graph/text shape) — the only thing honestly inferable from the
//! ENVELOPE alone (not the wrapped subset's own internals) is which subset it dispatches to: the
//! same `subset_tag`/`subset_ordinal` pair `📸️snapshot/🦀️.rs`'s own DSL header/binary-pack
//! codecs already compute from `SemioSubsetSnapshot`, reused here (not re-derived) as this
//! envelope's honest "union/dispatch shape" inference per the ticket's own naming guidance.

use crate::standards::v1::subsets::base::schema::snapshot::SemioSnapshot;
use framework_schema::ArtifactSchema;

use super::kind::compute_semio_kind;
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
    fn infer(snapshot: &SemioSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { kind: compute_semio_kind(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `SemioSnapshot::default()`'s `subset` defaults
/// to `SemioSubsetSnapshot::Brep(..)` (the enum's first-declared variant, not a zero/unit value a
/// naive derive could reconstruct), so `Default` MUST be tied to `infer`, never derived.
impl Default for SemioInference {
    fn default() -> Self {
        let snapshot = &SemioSnapshot::default();

        Self { kind: compute_semio_kind(snapshot) }
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

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `semio_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::kind::SemioKind;
//#endregion 🔁️Re-exports
