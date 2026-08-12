//! 💡️ DwgInference (ac1018) — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🗂structure/`). ac1018's
//! `DwgSnapshot` never decoded any geometric entities (`bytes` is the opaque, undecoded raw
//! payload — see `📸️snapshot/🦀️component.rs`'s own "deliberately frozen legacy shim" doc
//! comment), so a bounding-box inference (dxf's `📦bounds/`) would be dishonest here; `structure`
//! is the closest honest derived statistic — a byte/section count, not fabricated geometry.
//!
//! 🆔️ Schema id is `s.stdio.dwg.ac1018.inference`, deliberately NOT colliding with ac1024's
//! `s.stdio.dwg.inference`. ac1018's *snapshot* does still collide with ac1024's on the shared
//! `s.stdio.dwg` id, but that is a pre-existing defect owned by ticket
//! `26/08/12/FIX-STDIO-DWG-AC1018-AND-AC1024-SCHEMA-ID-COLLISION`, whose published fix is to give
//! ac1018 the `stdio.dwg.ac1018` shape. This facet is authored directly in that post-fix shape so
//! the collision ticket never has to touch it — reproducing a known defect for local symmetry
//! would be exactly the pragmatism the repo's greenfield rule forbids.

use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::structure::{compute_dwg_structure, DwgStructure};

//#region 🔖️Inference
/// 💡️ Everything inferable from an ac1018 dwg snapshot. One field per named inference under
/// `💡️inferences/` (currently: `structure`, backed by the `🗂structure/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dwg.ac1018.inference")]
pub struct DwgInference {
    #[state(inferred)]
    pub structure: DwgStructure,
}

impl protocol::Inference<DwgSnapshot> for DwgInference {
    fn infer(snapshot: &DwgSnapshot) -> Self {
        Self { structure: compute_dwg_structure(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `DwgSnapshot::default()`'s `bytes`/`section_names` ever stop being empty.
impl Default for DwgInference {
    fn default() -> Self {
        <Self as protocol::Inference<DwgSnapshot>>::infer(&DwgSnapshot::default())
    }
}

impl protocol::InferenceSpec<DwgSnapshot> for DwgInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.dwg.ac1018.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec {
            id: "s.stdio.dwg.ac1018.inference.structure",
            reads: &["bytes", "sectionNames", "codepage", "version"],
        }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `structure` is a direct O(1) field read/count over the flat
/// snapshot (`bytes.len()`, `section_names.len()`, `codepage`, `version` copied straight
/// through), no per-entity decomposition exists to cache incrementally — the default
/// `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::DwgBuilder {
    type Snapshot = DwgSnapshot;
    type Inference = DwgInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.dwg.ac1018.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `dwg_artifact_schema_descriptor`'s registration.
pub fn dwg_ac1018_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.dwg.ac1018.inference",
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
        let snapshot = DwgSnapshot::default();
        assert_eq!(DwgInference::infer(&snapshot), DwgInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(DwgInference::infer(&DwgSnapshot::default()), DwgInference::default());
    }
}
//#endregion 🧪️Tests
