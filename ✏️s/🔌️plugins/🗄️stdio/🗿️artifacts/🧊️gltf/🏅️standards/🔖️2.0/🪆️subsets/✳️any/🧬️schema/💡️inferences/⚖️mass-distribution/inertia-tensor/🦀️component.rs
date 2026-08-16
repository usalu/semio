//! 💡️ inertia-tensor atomic glTF inference leaf.
use super::super::{geometry_core::GltfGeometryContext, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};
use super::super::super::modules::{inference_measures::{estimate, unavailable}, measurement_contracts::*};

pub struct GltfInertiaTensorInference;

impl GltfInferenceLeaf for GltfInertiaTensorInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.inertia-tensor.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.inertia-tensor.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor { GltfInertiaTensorInference::DESCRIPTOR }

pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<Vec<f64>> {
    let moments = super::moments_of_inertia::raw(context); estimate(vec![moments.x, 0.0, 0.0, 0.0, moments.y, 0.0, 0.0, 0.0, moments.z], GltfUnit::SquareMetre, context.sample_count, Some(context.topology))
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<Vec<f64>> {
    unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.mass.inertiaTensor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.inertia-tensor.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
