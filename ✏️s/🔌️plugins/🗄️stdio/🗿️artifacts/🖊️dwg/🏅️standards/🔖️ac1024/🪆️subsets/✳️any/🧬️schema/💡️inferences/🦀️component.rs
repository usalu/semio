//! 💡️ DwgInference (ac1024) — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🗂structure/`). ac1024's
//! `DwgSnapshot` locates named sections/pages (D1/D2 real structural decode — see
//! `📸️snapshot/🦀️component.rs`) but never decodes any geometric entity out of them (that's
//! D3-D4, out of this ticket's scope), so a bounding-box inference (dxf's `📦bounds/`) would be
//! dishonest here; `structure` is the closest honest derived statistic — real byte/section/page
//! counts, richer than ac1018's own (this standard's snapshot genuinely has more decoded
//! structure: named sections with per-page decode/error status), not fabricated geometry.
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
    fn infer(snapshot: &DwgSnapshot) -> Self {
        Self { structure: compute_dwg_structure(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `DwgSnapshot::default()`'s `source`/`sections` ever stop being empty.
impl Default for DwgInference {
    fn default() -> Self {
        <Self as protocol::Inference<DwgSnapshot>>::infer(&DwgSnapshot::default())
    }
}

impl protocol::InferenceSpec<DwgSnapshot> for DwgInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.dwg.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec {
            id: "s.stdio.dwg.inference.structure",
            reads: &["source", "sections", "codepage", "version"],
        }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `structure` is a whole-snapshot fold over `sections`/`pages`
/// (summing page/decoded/error counts and declared sizes) plus a few O(1) field reads
/// (decoded logical content size, `codepage`, `version`); no honest per-entity incremental decomposition exists
/// (a merkle dep-chain over this flat section/page list costs more than the fold it would
/// cache) — the default `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::DwgBuilder {
    type Snapshot = DwgSnapshot;
    type Inference = DwgInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.dwg.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `dwg_artifact_schema_descriptor`'s registration.
pub fn dwg_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
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
