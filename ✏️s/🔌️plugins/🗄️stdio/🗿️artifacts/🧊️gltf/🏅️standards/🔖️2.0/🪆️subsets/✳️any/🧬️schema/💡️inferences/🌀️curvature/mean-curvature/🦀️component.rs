//! 💡️ mean-curvature atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfMeanCurvatureInference;

impl GltfInferenceLeaf for GltfMeanCurvatureInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.mean-curvature.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.mean-curvature.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfMeanCurvatureInference::DESCRIPTOR
}

pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<GltfStatistics> {
    from_raw(context, &super::raw(context))
}

pub(crate) fn from_raw(context: &GltfGeometryContext<'_>, raw: &super::GltfCurvatureRaw) -> GltfMeasure<GltfStatistics> {
    estimate(super::statistics(&raw.edge_curvatures, &context.policy.histogram_edges), GltfUnit::InverseMetre, raw.edge_curvatures.len(), Some(context.topology))
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfStatistics> {
    unavailable(GltfUnit::InverseMetre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.curvature.mean_curvature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.mean-curvature.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
