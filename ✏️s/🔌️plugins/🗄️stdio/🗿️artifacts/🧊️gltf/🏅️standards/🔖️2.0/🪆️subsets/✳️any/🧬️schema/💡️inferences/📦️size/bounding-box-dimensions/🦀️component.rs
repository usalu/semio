//! 💡️ bounding-box-dimensions atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{exact, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};
pub struct GltfBoundingBoxDimensionsInference;
impl GltfInferenceLeaf for GltfBoundingBoxDimensionsInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.bounding-box-dimensions.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.bounding-box-dimensions.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}
pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfBoundingBoxDimensionsInference::DESCRIPTOR
}
pub(crate) async fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<GltfVec3> {
    exact(GltfVec3::new(context.dimensions), GltfUnit::Metre, context.sample_count, Some(context.topology))
}
pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfVec3> {
    unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.size.bounding_box_dimensions)
}
