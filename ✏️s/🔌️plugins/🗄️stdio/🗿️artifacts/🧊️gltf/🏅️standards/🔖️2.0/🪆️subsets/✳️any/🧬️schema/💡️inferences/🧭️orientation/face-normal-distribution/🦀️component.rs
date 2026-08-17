//! 💡️ face-normal-distribution atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{exact, unavailable},
    measurement_contracts::*,
    vector_operations::{cross, dot, normalize, sub},
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfFaceNormalDistributionInference;

impl GltfInferenceLeaf for GltfFaceNormalDistributionInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.face-normal-distribution.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.face-normal-distribution.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfFaceNormalDistributionInference::DESCRIPTOR
}

pub(crate) fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<GltfStatistics> {
    let face_angles = context
        .faces
        .iter()
        .map(|face| {
            let normal = normalize(cross(sub(context.points[face[1]], context.points[face[0]]), sub(context.points[face[2]], context.points[face[0]])));
            dot(normal, context.principal_frame.axes[0].array()).clamp(-1.0, 1.0).acos()
        })
        .collect::<Vec<_>>();
    exact(super::super::geometry_core::statistics(&face_angles, &context.policy.histogram_edges), GltfUnit::Radian, context.faces.len(), Some(context.topology))
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfStatistics> {
    unavailable(GltfUnit::Radian, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.orientation.face_normal_distribution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.face-normal-distribution.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
