//! 💡️ minimum-thickness atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfMinimumThicknessInference;

impl GltfInferenceLeaf for GltfMinimumThicknessInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.minimum-thickness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.minimum-thickness.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfMinimumThicknessInference::DESCRIPTOR
}

pub(crate) async fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    let distribution = super::distribution(context);
    distribution
        .minimum
        .map(|value| estimate(value, GltfUnit::Metre, super::samples(context).len(), Some(context.topology)))
        .unwrap_or_else(|| unavailable(GltfUnit::Metre, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology)))
}

pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.thickness.minimum_thickness)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.minimum-thickness.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
