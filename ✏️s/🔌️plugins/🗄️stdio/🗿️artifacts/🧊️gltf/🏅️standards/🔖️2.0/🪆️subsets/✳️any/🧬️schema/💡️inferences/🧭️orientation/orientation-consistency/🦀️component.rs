//! 💡️ orientation-consistency atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{exact, unavailable},
    measurement_contracts::*,
    mesh_topology::Topology,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfOrientationConsistencyInference;

impl GltfInferenceLeaf for GltfOrientationConsistencyInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.orientation-consistency.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.orientation-consistency.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfOrientationConsistencyInference::DESCRIPTOR
}

pub(crate) fn infer_pair(pair: &super::super::geometry_core::GltfPairGeometry) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), pair.sample_count, None)
}

pub(crate) fn unavailable_for_assembly(sample_count: usize, topology: Topology) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), sample_count, Some(topology))
}
pub(crate) fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    exact(if context.topology.oriented { 1.0 } else { 0.0 }, GltfUnit::Unitless, context.sample_count, Some(context.topology))
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.orientation.orientation_consistency)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.orientation-consistency.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
