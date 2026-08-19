//! 💡️ thickness-distribution atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfThicknessDistributionInference;

impl GltfInferenceLeaf for GltfThicknessDistributionInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.thickness-distribution.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.thickness-distribution.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfThicknessDistributionInference::DESCRIPTOR
}

pub(crate) async fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<GltfStatistics> {
    let samples = super::samples(context);
    if samples.is_empty() {
        unavailable(GltfUnit::Metre, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology))
    } else {
        estimate(super::statistics(&samples, &context.policy.histogram_edges), GltfUnit::Metre, samples.len(), Some(context.topology))
    }
}

pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfStatistics> {
    unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.thickness.thickness_distribution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.thickness-distribution.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
