//! 💡️ void-volume atomic glTF inference leaf.
use super::super::{geometry_core::GltfGeometryContext, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};
use super::super::super::modules::{inference_measures::{exact, unavailable}, measurement_contracts::*};

pub struct GltfVoidVolumeInference;

impl GltfInferenceLeaf for GltfVoidVolumeInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.void-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.void-volume.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor { GltfVoidVolumeInference::DESCRIPTOR }

pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    context.solid.map(|metrics| exact(metrics.2, GltfUnit::CubicMetre, context.sample_count, Some(context.topology))).unwrap_or_else(|| unavailable(GltfUnit::CubicMetre, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology)))
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.area_volume.voidVolume)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.void-volume.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
