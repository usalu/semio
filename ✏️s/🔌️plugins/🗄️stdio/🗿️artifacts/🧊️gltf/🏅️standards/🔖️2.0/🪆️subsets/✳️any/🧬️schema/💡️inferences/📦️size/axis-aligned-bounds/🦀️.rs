//! 💡️ axis-aligned-bounds atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{exact, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};
pub struct GltfAxisAlignedBoundsInference;
impl GltfInferenceLeaf for GltfAxisAlignedBoundsInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.axis-aligned-bounds.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.axis-aligned-bounds.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfAxisAlignedBoundsInference::DESCRIPTOR
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<GltfBounds3> {
    exact(context.bounds.clone(), GltfUnit::Metre, context.sample_count, Some(context.topology))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfBounds3> {
    unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(&pack::to_json_string(&indicators.size.axis_aligned_bounds))
}
#[cfg(test)]
mod canonical_vectors {
    #[derive(value_derive::FromValue)]
    struct Value {
        min: [f64; 3],
        max: [f64; 3],
        dimensions: [f64; 3],
    }
    #[derive(value_derive::FromValue)]
    struct Vector {
        value: Option<Value>,
    }
    #[derive(value_derive::FromValue)]
    struct Contract {
        vectors: Vec<Vector>,
    }
    #[semio_framework_async_macros::async_test]
    async fn shared_analytic_unavailable_and_deterministic_bounds_vectors_are_typed() {
        let contract: Contract = pack::from_json_str(include_str!("🧪️contract/🔣️.json")).unwrap();
        assert_eq!(contract.vectors[0].value.as_ref().unwrap().dimensions, [3.0, 4.0, 5.0]);
        assert_eq!(contract.vectors[0].value.as_ref().unwrap().min, [1.0, 2.0, 3.0]);
        assert_eq!(contract.vectors[0].value.as_ref().unwrap().max, [4.0, 6.0, 8.0]);
        assert!(contract.vectors[1].value.is_none());
    }
}
