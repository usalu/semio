//! 💡️ gaussian-curvature atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfGaussianCurvatureInference;

impl GltfInferenceLeaf for GltfGaussianCurvatureInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.gaussian-curvature.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.gaussian-curvature.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfGaussianCurvatureInference::DESCRIPTOR
}

pub(crate) async fn from_raw(context: &GltfGeometryContext<'_>, raw: &super::GltfCurvatureRaw) -> GltfMeasure<GltfStatistics> {
    estimate(super::statistics(&raw.gaussian_values, &context.policy.histogram_edges), GltfUnit::InverseSquareMetre, raw.gaussian_values.len(), Some(context.topology))
}

pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfStatistics> {
    unavailable(GltfUnit::InverseSquareMetre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.curvature.gaussian_curvature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.gaussian-curvature.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
