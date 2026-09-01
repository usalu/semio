//! 💡️ concavity-index atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfConcavityIndexInference;

impl GltfInferenceLeaf for GltfConcavityIndexInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.concavity-index.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.concavity-index.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfConcavityIndexInference::DESCRIPTOR
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn from_raw(context: &GltfGeometryContext<'_>, raw: &super::GltfConcavityRaw) -> GltfMeasure<f64> {
    raw.hull_volume
        .filter(|volume| *volume > 0.0)
        .map(|volume| {
            if context.solid.is_some() {
                estimate((1.0 - context.volume / volume).clamp(0.0, 1.0), GltfUnit::Unitless, context.sample_count, Some(context.topology))
            } else {
                unavailable(GltfUnit::Unitless, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology))
            }
        })
        .unwrap_or_else(|| unavailable(GltfUnit::Unitless, GltfAvailability::Degenerate, Vec::new(), context.sample_count, Some(context.topology)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(&pack::to_json_string(&indicators.concavity.concavity_index))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.concavity-index.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
