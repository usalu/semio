//! 💡️ Json inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::json::JsonSnapshot;
use protocol::Inference;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::JsonOutline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a json snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.json.inference")]
pub struct JsonInference {
    #[state(inferred)]
    pub outline: JsonOutline,
}

impl protocol::Inference<JsonSnapshot> for JsonInference {
    fn infer(snapshot: &JsonSnapshot) -> Self {
        Self { outline: JsonOutline::compute(snapshot) }
    }
}

/// 🪞️ Hand impl (not derived): `JsonSnapshot::default()`'s root is `Null`, a real value that
/// `JsonOutline::compute` reports on, so the derived all-zero `JsonOutline::default()` disagrees
/// with it and breaks `inference_default_law`. Defining default as "infer the default snapshot"
/// makes the two definitionally equal.
impl Default for JsonInference {
    fn default() -> Self {
        Self::infer(&JsonSnapshot::default())
    }
}

impl protocol::InferenceSpec<JsonSnapshot> for JsonInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.json.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.json.inference.outline", reads: &["value"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::JsonBuilder {
    type Snapshot = JsonSnapshot;
    type Inference = JsonInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.json.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `json_artifact_schema_descriptor`'s registration.
pub fn json_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.json.inference",
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
        let snapshot = JsonSnapshot::default();
        assert_eq!(JsonInference::infer(&snapshot), JsonInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(JsonInference::infer(&JsonSnapshot::default()), JsonInference::default());
    }
}
//#endregion 🧪️Tests
