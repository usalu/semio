//! 💡️ number-of-contacts atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, exact, unavailable},
    measurement_contracts::*,
    mesh_topology::Topology,
};
use super::super::{
    geometry_core::{GltfGeometryContext, GltfPairGeometry},
    GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS,
};

pub struct GltfNumberOfContactsInference;

impl GltfInferenceLeaf for GltfNumberOfContactsInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.number-of-contacts.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.number-of-contacts.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfNumberOfContactsInference::DESCRIPTOR
}

pub(crate) async fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<u64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology))
}

pub(crate) async fn infer_pair(pair: &GltfPairGeometry) -> GltfMeasure<bool> {
    estimate(pair.adjacent, GltfUnit::Unitless, pair.sample_count, None)
}

pub(crate) async fn from_assembly(part_count: usize, contacts: u64, sample_count: usize, topology: Topology) -> GltfMeasure<u64> {
    if part_count <= 1 {
        exact(0, GltfUnit::Unitless, sample_count, Some(topology))
    } else {
        estimate(contacts, GltfUnit::Unitless, sample_count, Some(topology))
    }
}

pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<u64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.adjacency.number_of_contacts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.number-of-contacts.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
