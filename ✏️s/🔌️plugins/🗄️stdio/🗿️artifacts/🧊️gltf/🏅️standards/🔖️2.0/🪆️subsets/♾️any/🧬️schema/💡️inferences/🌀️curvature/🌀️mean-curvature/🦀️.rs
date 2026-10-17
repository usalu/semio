//! 💡️ mean-curvature atomic glTF inference leaf.
use crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::statistics;
use crate::standards::v2_0::subsets::any::schema::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::GltfGeometryContext;

pub struct GltfMeanCurvatureInference;

impl GltfInferenceLeaf for GltfMeanCurvatureInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.mean-curvature.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.mean-curvature.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfMeanCurvatureInference::DESCRIPTOR
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn from_raw(context: &GltfGeometryContext<'_>, raw: &crate::standards::v2_0::subsets::any::schema::inferences::curvature::GltfCurvatureRaw) -> GltfMeasure<GltfStatistics> {
    estimate(crate::standards::v2_0::subsets::any::schema::inferences::curvature::statistics(&raw.edge_curvatures, &context.policy.histogram_edges), GltfUnit::InverseMetre, raw.edge_curvatures.len(), Some(context.topology))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfStatistics> {
    unavailable(GltfUnit::InverseMetre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_result(measure: &GltfMeasure<GltfStatistics>) -> semio_framework_value::DslValue {
    semio_framework_value::ToValue::to_value(measure)
}

//#region 🧾️InferenceDescriptor
/// 🧾️ Registry descriptor of this inference leaf; its companion documents are embedded from this leaf's own folder.
pub fn inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.stdio.gltf.inference.mean-curvature.v1",
        inference: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🧾️InferenceDescriptor


