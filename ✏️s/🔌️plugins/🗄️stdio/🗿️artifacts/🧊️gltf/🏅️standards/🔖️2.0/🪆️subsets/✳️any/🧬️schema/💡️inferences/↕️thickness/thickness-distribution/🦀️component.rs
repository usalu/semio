//! 💡️ thickness-distribution atomic glTF inference leaf.
use super::super::{geometry_core::GltfGeometryContext, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};
use super::super::super::modules::{inference_measures::{estimate, unavailable}, measurement_contracts::*};

pub struct GltfThicknessDistributionInference;

impl GltfInferenceLeaf for GltfThicknessDistributionInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.thickness-distribution.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.thickness-distribution.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor { GltfThicknessDistributionInference::DESCRIPTOR }

pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<GltfStatistics> {
    let samples = super::samples(context); if samples.is_empty() { unavailable(GltfUnit::Metre, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology)) } else { estimate(super::statistics(&samples, &context.policy.histogram_edges), GltfUnit::Metre, samples.len(), Some(context.topology)) }
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfStatistics> {
    unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.thickness.thicknessDistribution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.thickness-distribution.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}

