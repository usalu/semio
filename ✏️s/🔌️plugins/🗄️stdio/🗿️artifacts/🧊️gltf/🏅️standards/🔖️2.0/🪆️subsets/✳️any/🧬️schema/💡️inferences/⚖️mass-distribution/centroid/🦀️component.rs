//! 💡️ centroid atomic glTF inference leaf.
use super::super::{geometry_core::GltfGeometryContext, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};
use super::super::super::modules::{inference_measures::{estimate, exact, unavailable}, measurement_contracts::*};

pub struct GltfCentroidInference;

impl GltfInferenceLeaf for GltfCentroidInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.centroid.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.centroid.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor { GltfCentroidInference::DESCRIPTOR }

pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<GltfVec3> {
    if context.topology.watertight && context.volume > 1e-15 { exact(GltfVec3::new(context.centroid), GltfUnit::Metre, context.sample_count, Some(context.topology)) } else { estimate(GltfVec3::new(context.centroid), GltfUnit::Metre, context.sample_count, Some(context.topology)) }
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfVec3> {
    unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.mass.centroid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.centroid.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
