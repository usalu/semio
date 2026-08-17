//! 💡️ rotational-symmetry-score atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfRotationalSymmetryScoreInference;

impl GltfInferenceLeaf for GltfRotationalSymmetryScoreInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.rotational-symmetry-score.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.rotational-symmetry-score.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfRotationalSymmetryScoreInference::DESCRIPTOR
}

pub(crate) fn from_raw(context: &GltfGeometryContext<'_>, raw: &super::GltfSymmetryRaw) -> GltfMeasure<f64> {
    estimate(raw.rotation_score, GltfUnit::Unitless, context.sample_count, Some(context.topology))
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.symmetry.rotational_symmetry_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.rotational-symmetry-score.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
