//! 💡️ GltfInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly while semantic components own
//! shared measures, each indicator group, and aggregate geometric analysis.

use crate::artifacts::gltf::schema::snapshot::GltfSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::geometric_analysis::{compute_gltf_geometry, GltfGeometricInference};

//#region 🔖️Inference
/// 💡️ Complete universal geometric inference for a glTF snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gltf.inference")]
pub struct GltfInference {
    #[derived]
    pub geometric_analysis: GltfGeometricInference,
}

impl protocol::Inference<GltfSnapshot> for GltfInference {
    fn infer(snapshot: &GltfSnapshot) -> Self {
        Self { geometric_analysis: compute_gltf_geometry(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `GltfSnapshot::default()`'s `document` ever stops being empty.
impl Default for GltfInference {
    fn default() -> Self {
        <Self as protocol::Inference<GltfSnapshot>>::infer(&GltfSnapshot::default())
    }
}

impl protocol::InferenceSpec<GltfSnapshot> for GltfInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.gltf.inference"
    }
    fn schema_version() -> u32 {
        2
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        GLTF_INFERENCE_FIELDS
    }
}

pub const GLTF_INFERENCE_FIELDS: &[protocol::InferenceFieldSpec] = &[
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.geometricAnalysis.resources", reads: &["document/buffers", "buffers"] },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.geometricAnalysis.accessors", reads: &["document/accessors", "document/bufferViews", "document/buffers", "buffers"] },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.geometricAnalysis.primitives", reads: &["document/meshes", "document/accessors", "document/bufferViews", "document/buffers", "buffers"] },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.geometricAnalysis.instances", reads: &["document/scene", "document/scenes", "document/nodes", "document/meshes"] },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.geometricAnalysis.materials", reads: &["document/materials", "document/textures", "document/images", "document/samplers"] },
    protocol::InferenceFieldSpec { id: "s.stdio.gltf.inference.geometricAnalysis.relations", reads: &["document/scene", "document/scenes", "document/nodes", "document/meshes", "document/accessors", "document/bufferViews", "document/buffers", "buffers"] },
    protocol::InferenceFieldSpec {
        id: "s.stdio.gltf.inference.geometricAnalysis.aggregate",
        reads: &[
            "document/scene",
            "document/scenes",
            "document/nodes",
            "document/meshes",
            "document/accessors",
            "document/bufferViews",
            "document/buffers",
            "buffers",
            "document/materials",
            "document/textures",
            "document/images",
            "document/samplers",
            "document/skins",
            "document/animations",
        ],
    },
];

/// 🧠️ Returns stable DAG field IDs invalidated by touched authored regions. `None` means a
/// cold/unknown change set and therefore invalidates every field; an empty set invalidates none.
pub fn invalidated_gltf_inference_fields(touched: Option<&protocol::TouchedPaths>) -> Vec<&'static str> {
    GLTF_INFERENCE_FIELDS.iter().filter(|field| touched.is_none_or(|paths| paths.intersects_any(field.reads))).map(|field| field.id).collect()
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ The kernel owns deterministic region-level dependency fingerprints; framework-level
/// inference caching therefore safely treats `geometricAnalysis` as one derived field over document and buffers.
impl ArtifactInferrer for crate::artifacts::gltf::standards::v2_0::subsets::any::schema::GltfBuilder {
    type Snapshot = GltfSnapshot;
    type Inference = GltfInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.gltf.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `gltf_artifact_schema_descriptor`'s registration.
pub fn gltf_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.gltf.inference",
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
    use protocol::{DiffRegions as _, Inference};

    #[test]
    fn inference_determinism_law() {
        let snapshot = GltfSnapshot::default();
        assert_eq!(GltfInference::infer(&snapshot), GltfInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(GltfInference::infer(&GltfSnapshot::default()), GltfInference::default());
    }

    #[test]
    fn dependency_contract_covers_document_and_resolved_buffers() {
        let reads: Vec<_> = GLTF_INFERENCE_FIELDS.iter().flat_map(|field| field.reads.iter().copied()).collect();
        assert!(reads.iter().any(|path| path.starts_with("document/")));
        assert!(reads.contains(&"buffers"));
        assert_eq!(invalidated_gltf_inference_fields(None).len(), GLTF_INFERENCE_FIELDS.len());
    }

    #[test]
    fn node_transform_reuses_resource_accessor_and_primitive_stages() {
        let diff = crate::artifacts::gltf::schema::diff::GltfDiff {
            nodes: Some(crate::artifacts::gltf::schema::diff::GltfNodesDiff {
                modified: vec![crate::artifacts::gltf::schema::diff::GltfModified { index: 0, diff: crate::artifacts::gltf::schema::diff::GltfNodeDiff { translation: Some(Some([1.0, 2.0, 3.0])), ..Default::default() } }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let touched = diff.touches();
        let invalidated = invalidated_gltf_inference_fields(Some(&touched));
        assert!(!invalidated.contains(&"s.stdio.gltf.inference.geometricAnalysis.resources"));
        assert!(!invalidated.contains(&"s.stdio.gltf.inference.geometricAnalysis.accessors"));
        assert!(!invalidated.contains(&"s.stdio.gltf.inference.geometricAnalysis.primitives"));
        assert!(invalidated.contains(&"s.stdio.gltf.inference.geometricAnalysis.instances"));
        assert!(invalidated.contains(&"s.stdio.gltf.inference.geometricAnalysis.relations"));
        assert!(invalidated.contains(&"s.stdio.gltf.inference.geometricAnalysis.aggregate"));
    }
}
//#endregion 🧪️Tests
