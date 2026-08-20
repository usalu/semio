//! 💡️ principal-frame atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfPrincipalFrameInference;

impl GltfInferenceLeaf for GltfPrincipalFrameInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.principal-frame.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.principal-frame.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfPrincipalFrameInference::DESCRIPTOR
}

pub(crate) async fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<GltfPrincipalFrame> {
    estimate(context.principal_frame.clone(), GltfUnit::Unitless, context.sample_count, Some(context.topology)).await
}

pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfPrincipalFrame> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None).await
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.mass.principal_frame)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.principal-frame.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
