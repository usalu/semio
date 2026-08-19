//! 💡️ overall-size atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{exact, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};
pub struct GltfOverallSizeInference;
impl GltfInferenceLeaf for GltfOverallSizeInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.overall-size.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.overall-size.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}
pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfOverallSizeInference::DESCRIPTOR
}
pub(crate) async fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    exact(context.diagonal, GltfUnit::Metre, context.sample_count, Some(context.topology))
}
pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}
pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.size.overall_size)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.overall-size.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
#[cfg(test)]
mod canonical_vectors {
    use serde::Deserialize;
    #[derive(Deserialize)]
    struct Vector {
        value: Option<f64>,
        availability: String,
    }
    #[derive(Deserialize)]
    struct Contract {
        vectors: Vec<Vector>,
    }
    #[test]
    async fn shared_analytic_unavailable_and_deterministic_vectors_are_typed() {
        let contract: Contract = serde_json::from_str(include_str!("🧪️contract/🔣️component.json")).unwrap();
        assert_eq!(contract.vectors[0].value, Some(5.0));
        assert_eq!(contract.vectors[0].availability, "available");
        assert_eq!(contract.vectors[1].value, None);
        assert_eq!(contract.vectors[1].availability, "unavailable");
    }
}
