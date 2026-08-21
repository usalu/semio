//! 💡️ Md inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::md::MdSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::MdOutline;

//#region 🔖️Inference
/// 💡️ Everything inferable from an md snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.md.inference")]
pub struct MdInference {
    #[derived]
    pub outline: MdOutline,
}

impl protocol::Inference<MdSnapshot> for MdInference {
    fn infer(snapshot: &MdSnapshot) -> Self {
        Self { outline: MdOutline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<MdSnapshot> for MdInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.md.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.md.inference.outline", reads: &["blocks"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::md::standards::v_commonmark::subsets::any::schema::MdBuilder {
    type Snapshot = MdSnapshot;
    type Inference = MdInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.md.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `md_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn md_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.md.inference",
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
        let snapshot = MdSnapshot::default();
        assert_eq!(MdInference::infer(&snapshot), MdInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(MdInference::infer(&MdSnapshot::default()), MdInference::default());
    }
}
//#endregion 🧪️Tests
