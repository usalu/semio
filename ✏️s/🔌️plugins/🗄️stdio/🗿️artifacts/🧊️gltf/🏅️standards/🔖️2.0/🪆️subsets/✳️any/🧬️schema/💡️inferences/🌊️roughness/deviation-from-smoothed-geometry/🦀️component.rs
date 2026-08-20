//! 💡️ deviation-from-smoothed-geometry atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfDeviationFromSmoothedGeometryInference;

impl GltfInferenceLeaf for GltfDeviationFromSmoothedGeometryInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.deviation-from-smoothed-geometry.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.deviation-from-smoothed-geometry.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfDeviationFromSmoothedGeometryInference::DESCRIPTOR
}

pub(crate) async fn from_raw(context: &GltfGeometryContext<'_>, raw: &super::GltfRoughnessRaw) -> GltfMeasure<GltfStatistics> {
    estimate(super::statistics(&raw.deviations, &context.policy.histogram_edges), GltfUnit::Metre, raw.deviations.len(), Some(context.topology)).await
}

pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfStatistics> {
    unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.to_vec(), 0, None).await
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.roughness.deviation_from_smoothed_geometry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.deviation-from-smoothed-geometry.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
