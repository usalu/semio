//! 💡️ Tsv inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::tsv::TsvSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::TsvOutline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a tsv snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.tsv.inference")]
pub struct TsvInference {
    #[state(inferred)]
    pub outline: TsvOutline,
}

impl protocol::Inference<TsvSnapshot> for TsvInference {
    fn infer(snapshot: &TsvSnapshot) -> Self {
        Self { outline: TsvOutline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<TsvSnapshot> for TsvInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.tsv.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.tsv.inference.outline", reads: &["records"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::tsv::standards::iana::subsets::any::schema::TsvBuilder {
    type Snapshot = TsvSnapshot;
    type Inference = TsvInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.tsv.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `tsv_artifact_schema_descriptor`'s registration.
pub fn tsv_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.tsv.inference",
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
        let snapshot = TsvSnapshot::default();
        assert_eq!(TsvInference::infer(&snapshot), TsvInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(TsvInference::infer(&TsvSnapshot::default()), TsvInference::default());
    }
}
//#endregion 🧪️Tests
