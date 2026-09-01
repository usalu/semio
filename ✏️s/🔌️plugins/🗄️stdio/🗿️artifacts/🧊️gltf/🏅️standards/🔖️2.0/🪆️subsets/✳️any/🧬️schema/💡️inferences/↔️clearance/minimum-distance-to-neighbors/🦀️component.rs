//! 💡️ minimum-distance-to-neighbors atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
    mesh_topology::Topology,
};
use super::super::{
    geometry_core::{GltfGeometryContext, GltfPairGeometry},
    GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS,
};

pub struct GltfMinimumDistanceToNeighborsInference;

impl GltfInferenceLeaf for GltfMinimumDistanceToNeighborsInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.minimum-distance-to-neighbors.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.minimum-distance-to-neighbors.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfMinimumDistanceToNeighborsInference::DESCRIPTOR
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn infer_pair(pair: &GltfPairGeometry) -> GltfMeasure<f64> {
    estimate(pair.minimum_distance, GltfUnit::Metre, pair.sample_count, None)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn from_assembly(distances: &[f64], sample_count: usize, topology: Topology) -> Option<GltfMeasure<f64>> {
    (!distances.is_empty()).then(|| estimate(distances.iter().copied().fold(f64::INFINITY, f64::min), GltfUnit::Metre, sample_count, Some(topology)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(&pack::to_json_string(&indicators.clearance.minimum_distance_to_neighbors))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.minimum-distance-to-neighbors.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
