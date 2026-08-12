//! 💡️ SemioBrepInference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING, authored here
//! by DKM per the standing exclusion: IIF's inference fan-out explicitly excludes `✳️brep`/
//! `✳️drawing`/`✳️mesh` and defers them). Directory shape mirrors `🧬️mutations/`: this file is the
//! family-root assembly (never mod's/includes the slug dirs directly — `📦️glue.rs` is the sole
//! mounting mechanism, same as mutations); each named inference gets its own `<emoji><slug>/` child
//! (currently: `✅validation-report/`).
//!
//! `tessellation` and `mass-properties` are deliberately NOT fields here — see
//! `✅validation-report/🦀️component.rs`'s module doc comment for why (real curve/surface evaluation
//! math has no honest home at this layer yet; faking it was rejected, not merely deferred).

use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::validation_report::{BrepValidationDiagnostic, BrepValidationReport};

//#region 🔖️Inference
/// 💡️ Everything inferable from a brep snapshot. One field per named inference under
/// `💡️inferences/` (currently: `validationReport`, backed by the `✅validation-report/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.brep.inference")]
pub struct SemioBrepInference {
    #[state(inferred)]
    pub validation_report: Vec<BrepValidationDiagnostic>,
}

impl protocol::Inference<SemioBrepSnapshot> for SemioBrepInference {
    fn infer(snapshot: &SemioBrepSnapshot) -> Self {
        Self { validation_report: store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(snapshot, None).remove("document").unwrap_or_default() }
    }
}

impl protocol::InferenceSpec<SemioBrepSnapshot> for SemioBrepInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.semio.brep.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec {
            id: "s.stdio.semio.brep.inference.validationReport",
            reads: &["vertices", "edges", "loops", "faces", "shells", "solids"],
        }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::semio::standards::v1::subsets::brep::schema::SemioBrepBuilder {
    type Snapshot = SemioBrepSnapshot;
    type Inference = SemioBrepInference;

    fn infer_cached(snapshot: &Self::Snapshot, cache: &mut store::InferenceCache, session: &mut store::InferenceSession) -> Self::Inference {
        let _ = session;
        let report = store::infer_field::<SemioBrepSnapshot, BrepValidationReport>(snapshot, Some(cache)).remove("document").unwrap_or_default();
        SemioBrepInference { validation_report: report }
    }
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.semio.brep.inference`'s facet leaves into the OS-wide inference catalog.
/// The `register_artifact_inferences()` call site itself lives in the SHARED
/// `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs` (aggregates all 14 `s.stdio.semio.*` subsets'
/// `register()` calls) — out of this ticket's `✳️brep/`-only edit scope, same boundary
/// `✳️brep/🚪️io/🦀️component.rs`'s own conformance-law doc comment already notes for the composer
/// registration. Flagged under `## sharedFileRequests` in the wave report, not wired here.
pub fn semio_brep_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.semio.brep.inference",
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

    #[test]
    fn inference_determinism_law() {
        let snapshot = SemioBrepSnapshot::default();
        assert_eq!(SemioBrepInference::infer(&snapshot), SemioBrepInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(SemioBrepInference::infer(&SemioBrepSnapshot::default()), SemioBrepInference::default());
    }
}
//#endregion 🧪️Tests
