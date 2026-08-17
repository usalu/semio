//! 💡️ compactness-index atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{exact, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfCompactnessIndexInference;

impl GltfInferenceLeaf for GltfCompactnessIndexInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.compactness-index.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.compactness-index.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfCompactnessIndexInference::DESCRIPTOR
}

pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    from_raw(context, &super::raw(context))
}

pub(crate) fn from_raw(context: &GltfGeometryContext<'_>, raw: &super::GltfCompactnessRaw) -> GltfMeasure<f64> {
    raw.sphericity.map(|value| exact(value, GltfUnit::Unitless, context.sample_count, Some(context.topology))).unwrap_or_else(|| unavailable(GltfUnit::Unitless, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology)))
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.compactness.compactness_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.compactness-index.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
