//! 💡️ Txt inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::txt::TxtSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::TxtOutline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a txt snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.txt.inference")]
pub struct TxtInference {
    #[derived]
    pub outline: TxtOutline,
}

impl protocol::Inference<TxtSnapshot> for TxtInference {
    async fn infer(snapshot: &TxtSnapshot) -> Self {
        Self { outline: TxtOutline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<TxtSnapshot> for TxtInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.txt.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.txt.inference.outline", reads: &["lines"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::txt::standards::v_utf_8::subsets::any::schema::TxtBuilder {
    type Snapshot = TxtSnapshot;
    type Inference = TxtInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.txt.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `txt_artifact_schema_descriptor`'s registration.
pub async fn txt_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.txt.inference",
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
    async fn inference_determinism_law() {
        let snapshot = TxtSnapshot::default();
        assert_eq!(TxtInference::infer(&snapshot), TxtInference::infer(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(TxtInference::infer(&TxtSnapshot::default()), TxtInference::default());
    }
}
//#endregion 🧪️Tests
