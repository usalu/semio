//! 💡️ slenderness atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{exact, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};
pub struct GltfSlendernessInference;
impl GltfInferenceLeaf for GltfSlendernessInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.slenderness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.slenderness.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}
pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfSlendernessInference::DESCRIPTOR
}
pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    let mut extent = context.oriented_extent;
    extent.sort_by(|left, right| right.total_cmp(left));
    exact(if extent[1] > 0.0 { extent[0] / extent[1] } else { 0.0 }, GltfUnit::Unitless, context.sample_count, Some(context.topology))
}
pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}
pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.proportion.slenderness)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.slenderness.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
