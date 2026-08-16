//! 💡️ overall-size atomic glTF inference leaf.
use super::super::{GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS, geometry_core::GltfGeometryContext};
use super::super::super::modules::{inference_measures::{exact, unavailable}, measurement_contracts::*};
pub struct GltfOverallSizeInference;
impl GltfInferenceLeaf for GltfOverallSizeInference { const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.overall-size.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.overall-size.v1:geometry-v2", reads: GLTF_GEOMETRY_READS }; }
pub fn descriptor() -> GltfInferenceLeafDescriptor { GltfOverallSizeInference::DESCRIPTOR }
pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> { exact(context.diagonal, GltfUnit::Metre, context.sample_count, Some(context.topology)) }
pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> { unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.to_vec(), 0, None) }
pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.size.overallSize)
}

#[cfg(test)] mod tests { use super::*; #[test] fn descriptor_is_versioned_and_cacheable() { assert_eq!(descriptor().id, "s.stdio.gltf.inference.overall-size.v1"); assert_eq!(descriptor().algorithm_version, 1); } }

