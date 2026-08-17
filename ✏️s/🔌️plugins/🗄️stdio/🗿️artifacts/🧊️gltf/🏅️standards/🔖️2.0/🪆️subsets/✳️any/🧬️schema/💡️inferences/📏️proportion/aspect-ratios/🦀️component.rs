//! 💡️ aspect-ratios atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{exact, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};
pub struct GltfAspectRatiosInference;
impl GltfInferenceLeaf for GltfAspectRatiosInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.aspect-ratios.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.aspect-ratios.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}
pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfAspectRatiosInference::DESCRIPTOR
}
pub(crate) fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<GltfVec3> {
    let mut extent = context.oriented_extent;
    extent.sort_by(|left, right| right.total_cmp(left));
    exact(
        GltfVec3::new([if extent[1] > 0.0 { extent[0] / extent[1] } else { 0.0 }, if extent[2] > 0.0 { extent[1] / extent[2] } else { 0.0 }, if extent[2] > 0.0 { extent[0] / extent[2] } else { 0.0 }]),
        GltfUnit::Unitless,
        context.sample_count,
        Some(context.topology),
    )
}
pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<GltfVec3> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}
pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.proportion.aspect_ratios)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Context {
        points: Vec<[f64; 3]>,
        triangles: Vec<[usize; 3]>,
        valid: bool,
    }

    #[derive(Deserialize)]
    struct Vector {
        context: Context,
        value: Option<GltfVec3>,
        availability: String,
    }

    #[derive(Deserialize)]
    struct Contract {
        vectors: Vec<Vector>,
    }

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.aspect-ratios.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }

    #[test]
    fn shared_vectors_execute_the_rust_leaf() {
        let contract: Contract = serde_json::from_str(include_str!("🧪️contract/🔣️component.json")).unwrap();
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
