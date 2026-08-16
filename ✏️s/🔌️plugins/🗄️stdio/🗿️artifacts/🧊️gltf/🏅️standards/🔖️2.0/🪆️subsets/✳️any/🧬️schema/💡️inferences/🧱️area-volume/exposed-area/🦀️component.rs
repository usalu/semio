//! 💡️ exposed-area atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
    mesh_topology::Topology,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfExposedAreaInference;

impl GltfInferenceLeaf for GltfExposedAreaInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.exposed-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.exposed-area.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfExposedAreaInference::DESCRIPTOR
}

pub fn from_assembly(surface_area: &GltfMeasure<f64>, part_count: usize, contact_area: f64, complete: bool, sample_count: usize, topology: Topology) -> GltfMeasure<f64> {
    if part_count <= 1 {
        return surface_area.clone();
    }
    if complete {
        return estimate((surface_area.value.unwrap_or(0.0) - 2.0 * contact_area).max(0.0), GltfUnit::SquareMetre, sample_count, Some(topology));
    }
    unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), sample_count, Some(topology))
}
pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology))
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.area_volume.exposed_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.exposed-area.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
