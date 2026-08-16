//! 💡️ reentrant-area atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfReentrantAreaInference;

impl GltfInferenceLeaf for GltfReentrantAreaInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.reentrant-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.reentrant-area.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfReentrantAreaInference::DESCRIPTOR
}

pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    from_raw(context, &super::raw(context))
}

pub(crate) fn from_raw(context: &GltfGeometryContext<'_>, raw: &super::GltfConcavityRaw) -> GltfMeasure<f64> {
    raw.reentrant_area
        .map(|area| estimate(area, GltfUnit::SquareMetre, context.sample_count, Some(context.topology)))
        .unwrap_or_else(|| unavailable(GltfUnit::SquareMetre, GltfAvailability::Degenerate, Vec::new(), context.sample_count, Some(context.topology)))
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.concavity.reentrant_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.reentrant-area.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
