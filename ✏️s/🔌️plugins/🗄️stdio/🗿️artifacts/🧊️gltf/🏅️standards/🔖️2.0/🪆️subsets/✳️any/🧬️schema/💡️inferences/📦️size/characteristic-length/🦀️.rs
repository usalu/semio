//! 💡️ characteristic-length atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{exact, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};
pub struct GltfCharacteristicLengthInference;
impl GltfInferenceLeaf for GltfCharacteristicLengthInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.characteristic-length.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.characteristic-length.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfCharacteristicLengthInference::DESCRIPTOR
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    exact(if context.surface_area > 0.0 { context.surface_area.sqrt() } else { context.diagonal }, GltfUnit::Metre, context.sample_count, Some(context.topology))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(&pack::to_json_string(&indicators.size.characteristic_length))
}
