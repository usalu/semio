//! 💡️ DwgInference (ac1024) — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🗂structure/`). `structure`
//! derives only modeled logical drawing counts.
//!
//! 🆔️ Schema id is `s.stdio.dwg.inference`, and it does NOT collide: its ac1018 sibling is
//! authored as `s.stdio.dwg.ac1018.inference`. The two *snapshots* do still share `s.stdio.dwg`,
//! but that pre-existing defect belongs to ticket
//! `26/08/12/FIX-STDIO-DWG-AC1018-AND-AC1024-SCHEMA-ID-COLLISION`, whose published fix renames the
//! ac1018 side — this facet pair is already in that post-fix shape, so the collision ticket never
//! has to touch either file.

use crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::structure::{compute_dwg_structure, DwgStructure};

//#region 🔖️Inference
/// 💡️ Everything inferable from an ac1024 dwg snapshot. One field per named inference under
/// `💡️inferences/` (currently: `structure`, backed by the `🗂structure/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dwg.inference")]
pub struct DwgInference {
    #[derived]
    pub structure: DwgStructure,
}

impl protocol::Inference<DwgSnapshot> for DwgInference {
    async fn infer(snapshot: &DwgSnapshot) -> Self {
        Self { structure: compute_dwg_structure(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` so the default follows the logical snapshot model.
impl Default for DwgInference {
    async fn default() -> Self {
        <Self as protocol::Inference<DwgSnapshot>>::infer(&DwgSnapshot::default())
    }
}

impl protocol::InferenceSpec<DwgSnapshot> for DwgInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.dwg.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.dwg.inference.structure", reads: &["drawing", "codepage", "version"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ `structure` is a whole-snapshot fold over the logical drawing.
impl ArtifactInferrer for crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::DwgBuilder {
    type Snapshot = DwgSnapshot;
    type Inference = DwgInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.dwg.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `dwg_artifact_schema_descriptor`'s registration.
pub async fn dwg_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.dwg.inference",
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
        let snapshot = DwgSnapshot::default();
        assert_eq!(DwgInference::infer(&snapshot), DwgInference::infer(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(DwgInference::infer(&DwgSnapshot::default()), DwgInference::default());
    }
}
//#endregion 🧪️Tests
