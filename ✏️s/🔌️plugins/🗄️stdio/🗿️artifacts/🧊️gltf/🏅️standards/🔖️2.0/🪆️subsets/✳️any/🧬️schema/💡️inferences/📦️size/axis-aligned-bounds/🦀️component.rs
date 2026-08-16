//! 💡️ axis-aligned-bounds atomic glTF inference leaf.
use super::super::{GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS, geometry_core::GltfGeometryContext};
use super::super::super::modules::{inference_measures::{exact, unavailable}, measurement_contracts::*};
pub struct GltfAxisAlignedBoundsInference;
impl GltfInferenceLeaf for GltfAxisAlignedBoundsInference { const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.axis-aligned-bounds.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.axis-aligned-bounds.v1:geometry-v2", reads: GLTF_GEOMETRY_READS }; }
pub fn descriptor() -> GltfInferenceLeafDescriptor { GltfAxisAlignedBoundsInference::DESCRIPTOR }
pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<GltfBounds3> { exact(context.bounds.clone(), GltfUnit::Metre, context.sample_count, Some(context.topology)) }
pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfBounds3> { unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.to_vec(), 0, None) }

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.size.axisAlignedBounds)
}
