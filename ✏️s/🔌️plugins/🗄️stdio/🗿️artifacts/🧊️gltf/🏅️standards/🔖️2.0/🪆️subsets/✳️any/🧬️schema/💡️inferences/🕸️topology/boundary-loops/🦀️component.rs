//! 💡️ boundary-loops atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{exact, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfBoundaryLoopsInference;

impl GltfInferenceLeaf for GltfBoundaryLoopsInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.boundary-loops.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.boundary-loops.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfBoundaryLoopsInference::DESCRIPTOR
}

pub(crate) async fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<u64> {
    exact(context.topology.boundary_loops, GltfUnit::Unitless, context.sample_count, Some(context.topology))
}

pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<u64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.topology.boundary_loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.boundary-loops.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
