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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfOverallSizeInference::DESCRIPTOR
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    exact(context.diagonal, GltfUnit::Metre, context.sample_count, Some(context.topology))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(&pack::to_json_string(&indicators.size.overall_size))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.overall-size.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
#[cfg(test)]
mod canonical_vectors {
    #[derive(value_derive::FromValue)]
    struct Vector {
        value: Option<f64>,
        availability: String,
    }
    #[derive(value_derive::FromValue)]
    struct Contract {
        vectors: Vec<Vector>,
    }
    #[semio_framework_async_macros::async_test]
    async fn shared_analytic_unavailable_and_deterministic_vectors_are_typed() {
        let contract: Contract = pack::from_json_str(include_str!("🧪️contract/🔣️.json")).unwrap();
        assert_eq!(contract.vectors[0].value, Some(5.0));
        assert_eq!(contract.vectors[0].availability, "available");
        assert_eq!(contract.vectors[1].value, None);
        assert_eq!(contract.vectors[1].availability, "unavailable");
    }
}
