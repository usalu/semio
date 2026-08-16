//! 💡️ reflection-symmetries atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfReflectionSymmetriesInference;

impl GltfInferenceLeaf for GltfReflectionSymmetriesInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.reflection-symmetries.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.reflection-symmetries.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfReflectionSymmetriesInference::DESCRIPTOR
}

pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<Vec<GltfDirectionScore>> {
    from_raw(context, &super::raw(context))
}

pub(crate) fn from_raw(context: &GltfGeometryContext<'_>, raw: &super::GltfSymmetryRaw) -> GltfMeasure<Vec<GltfDirectionScore>> {
    estimate(raw.reflections.clone(), GltfUnit::Unitless, context.sample_count, Some(context.topology))
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<Vec<GltfDirectionScore>> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.symmetry.reflection_symmetries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.reflection-symmetries.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
