//! 💡️ flatness atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{exact, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};
pub struct GltfFlatnessInference;
impl GltfInferenceLeaf for GltfFlatnessInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.flatness.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.flatness.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfFlatnessInference::DESCRIPTOR
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    let mut extent = context.oriented_extent;
    extent.sort_by(|left, right| right.total_cmp(left));
    exact(if extent[1] > 0.0 { extent[2] / extent[1] } else { 0.0 }, GltfUnit::Unitless, context.sample_count, Some(context.topology))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_str(&pack::to_json_string(&indicators.proportion.flatness))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(value_derive::FromValue)]
    #[value(rename_all = "camelCase")]
    struct Context {
        points: Vec<[f64; 3]>,
        triangles: Vec<[usize; 3]>,
        valid: bool,
    }

    #[derive(value_derive::FromValue)]
    struct Vector {
        context: Context,
        value: Option<f64>,
        availability: String,
    }

    #[derive(value_derive::FromValue)]
    struct Contract {
        vectors: Vec<Vector>,
    }

    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.flatness.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn shared_vectors_execute_the_rust_leaf() {
        let contract: Contract = pack::from_json_str(include_str!("🧪️contract/🔣️.json")).unwrap();
        for vector in contract.vectors {
            let result = if vector.context.valid {
                let policy = super::super::super::geometry_core::policy();
                let context = GltfGeometryContext::new(&vector.context.points, &vector.context.triangles, &policy).unwrap();
                infer(&context)
            } else {
                unavailable_measure(&["missing-position".into()])
            };
            assert_eq!(result.value, vector.value);
            assert_eq!(format!("{:?}", result.availability).to_ascii_lowercase(), vector.availability);
        }
    }
}
