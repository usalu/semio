//! 💡️ Csv inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::csv::CsvSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::CsvOutline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a csv snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.csv.inference")]
pub struct CsvInference {
    #[state(inferred)]
    pub outline: CsvOutline,
}

impl protocol::Inference<CsvSnapshot> for CsvInference {
    fn infer(snapshot: &CsvSnapshot) -> Self {
        Self { outline: CsvOutline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<CsvSnapshot> for CsvInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.csv.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.csv.inference.outline", reads: &["records", "hasHeader"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::csv::standards::v_rfc4180::subsets::any::schema::CsvBuilder {
    type Snapshot = CsvSnapshot;
    type Inference = CsvInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.csv.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `csv_artifact_schema_descriptor`'s registration.
pub fn csv_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.csv.inference",
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
        let snapshot = CsvSnapshot::default();
        assert_eq!(CsvInference::infer(&snapshot), CsvInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(CsvInference::infer(&CsvSnapshot::default()), CsvInference::default());
    }
}
//#endregion 🧪️Tests
