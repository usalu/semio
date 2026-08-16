//! 💡️ total-area atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{exact, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfTotalAreaInference;

impl GltfInferenceLeaf for GltfTotalAreaInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.total-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.total-area.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfTotalAreaInference::DESCRIPTOR
}

pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    exact(context.surface_area, GltfUnit::SquareMetre, context.sample_count, Some(context.topology))
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.area_volume.total_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.total-area.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
