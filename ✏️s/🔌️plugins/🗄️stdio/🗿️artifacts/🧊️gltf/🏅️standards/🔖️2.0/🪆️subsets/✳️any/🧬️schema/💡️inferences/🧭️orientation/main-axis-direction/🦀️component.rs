//! 💡️ main-axis-direction atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfMainAxisDirectionInference;

impl GltfInferenceLeaf for GltfMainAxisDirectionInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.main-axis-direction.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.main-axis-direction.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfMainAxisDirectionInference::DESCRIPTOR
}

pub(crate) async fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<GltfVec3> {
    estimate(context.principal_axes[0].direction, GltfUnit::Unitless, context.sample_count, Some(context.topology))
}

pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfVec3> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.orientation.main_axis_direction)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.main-axis-direction.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
