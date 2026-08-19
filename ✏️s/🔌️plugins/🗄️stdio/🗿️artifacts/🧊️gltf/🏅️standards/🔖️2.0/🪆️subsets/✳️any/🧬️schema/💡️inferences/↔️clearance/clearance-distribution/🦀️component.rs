//! 💡️ clearance-distribution atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
    mesh_topology::Topology,
};
use super::super::{
    geometry_core::{statistics, GltfGeometryContext, GltfPairGeometry},
    GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS,
};

pub struct GltfClearanceDistributionInference;

impl GltfInferenceLeaf for GltfClearanceDistributionInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.clearance-distribution.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.clearance-distribution.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfClearanceDistributionInference::DESCRIPTOR
}

pub(crate) async fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<GltfStatistics> {
    unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology))
}

pub(crate) async fn infer_pair(pair: &GltfPairGeometry, policy: &GltfAnalysisPolicy) -> GltfMeasure<GltfStatistics> {
    estimate(statistics(&[pair.minimum_distance], &policy.histogram_edges), GltfUnit::Metre, pair.sample_count, None)
}

pub(crate) async fn from_assembly(distances: &[f64], policy: &GltfAnalysisPolicy, sample_count: usize, topology: Topology) -> Option<GltfMeasure<GltfStatistics>> {
    (!distances.is_empty()).then(|| estimate(statistics(distances, &policy.histogram_edges), GltfUnit::Metre, sample_count, Some(topology)))
}

pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfStatistics> {
    unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.clearance.clearance_distribution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.clearance-distribution.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
