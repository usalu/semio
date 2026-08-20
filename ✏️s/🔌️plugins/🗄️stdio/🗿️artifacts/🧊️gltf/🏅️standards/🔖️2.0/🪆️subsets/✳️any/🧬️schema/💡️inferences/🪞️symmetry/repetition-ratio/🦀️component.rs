//! 💡️ repetition-ratio atomic glTF inference leaf.
use super::super::super::modules::mesh_topology::Topology;
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use super::super::GltfPartInference;
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfRepetitionRatioInference;

impl GltfInferenceLeaf for GltfRepetitionRatioInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.repetition-ratio.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.repetition-ratio.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfRepetitionRatioInference::DESCRIPTOR
}

pub(crate) async fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology)).await
}

pub(crate) async fn from_assembly(parts: &[GltfPartInference], policy: &GltfAnalysisPolicy, topology: Topology) -> Option<GltfMeasure<f64>> {
    super::assembly_ratios(parts, policy).await.map(|(repetition, _)| estimate(repetition, GltfUnit::Unitless, parts.len(), Some(topology)))
}

pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None).await
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.symmetry.repetition_ratio)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.repetition-ratio.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
