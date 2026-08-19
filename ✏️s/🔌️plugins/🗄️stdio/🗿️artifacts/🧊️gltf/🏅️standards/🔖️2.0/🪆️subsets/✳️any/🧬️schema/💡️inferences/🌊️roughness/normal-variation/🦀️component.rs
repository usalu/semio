//! 💡️ normal-variation atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{exact, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfNormalVariationInference;

impl GltfInferenceLeaf for GltfNormalVariationInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.normal-variation.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.normal-variation.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfNormalVariationInference::DESCRIPTOR
}

pub(crate) async fn from_raw(context: &GltfGeometryContext<'_>, raw: &super::GltfRoughnessRaw) -> GltfMeasure<GltfStatistics> {
    exact(super::statistics(&raw.normal_angles, &context.policy.histogram_edges), GltfUnit::Radian, raw.normal_angles.len(), Some(context.topology))
}

pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfStatistics> {
    unavailable(GltfUnit::Radian, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.roughness.normal_variation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.normal-variation.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
