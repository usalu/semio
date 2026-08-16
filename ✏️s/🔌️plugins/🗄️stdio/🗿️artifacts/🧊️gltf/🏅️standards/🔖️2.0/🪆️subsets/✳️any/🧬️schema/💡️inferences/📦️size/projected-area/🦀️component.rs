//! 💡️ projected-area atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
    vector_operations::{cross, sub},
};
use super::super::{
    geometry_core::{statistics, GltfGeometryContext},
    GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS,
};
pub struct GltfProjectedAreaInference;
impl GltfInferenceLeaf for GltfProjectedAreaInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.projected-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.projected-area.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}
pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfProjectedAreaInference::DESCRIPTOR
}
pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<GltfStatistics> {
    let projected = [0, 1, 2].map(|axis| context.faces.iter().map(|face| 0.5 * cross(sub(context.points[face[1]], context.points[face[0]]), sub(context.points[face[2]], context.points[face[0]]))[axis].abs()).sum::<f64>());
    estimate(statistics(&projected, &context.policy.histogram_edges), GltfUnit::SquareMetre, context.sample_count, Some(context.topology))
}
pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfStatistics> {
    unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.size.projected_area)
}
