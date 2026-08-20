//! 💡️ compactness atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{exact, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfCompactnessInference;

impl GltfInferenceLeaf for GltfCompactnessInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.compactness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.compactness.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfCompactnessInference::DESCRIPTOR
}

pub(crate) async fn from_raw(context: &GltfGeometryContext<'_>, raw: &super::GltfCompactnessRaw) -> GltfMeasure<f64> {
    raw.sphericity.map(|value| exact(value, GltfUnit::Unitless, context.sample_count, Some(context.topology))).unwrap_or_else(|| unavailable(GltfUnit::Unitless, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology))).await
}

pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None).await
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.compactness.compactness)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.compactness.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
