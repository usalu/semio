//! 💡️ genus atomic glTF inference leaf.
use super::super::{geometry_core::GltfGeometryContext, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};
use super::super::super::modules::{inference_measures::{exact, unavailable}, measurement_contracts::*};

pub struct GltfGenusInference;

impl GltfInferenceLeaf for GltfGenusInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.genus.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.genus.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor { GltfGenusInference::DESCRIPTOR }

pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<u64> {
    context.topology.genus.map(|value| exact(value, GltfUnit::Unitless, context.sample_count, Some(context.topology))).unwrap_or_else(|| unavailable(GltfUnit::Unitless, GltfAvailability::NonManifold, Vec::new(), context.sample_count, Some(context.topology)))
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<u64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.topology.genus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.genus.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
