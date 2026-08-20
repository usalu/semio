//! 💡️ convex-hull-gap atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfConvexHullGapInference;

impl GltfInferenceLeaf for GltfConvexHullGapInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.convex-hull-gap.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.convex-hull-gap.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfConvexHullGapInference::DESCRIPTOR
}

pub(crate) async fn from_raw(context: &GltfGeometryContext<'_>, raw: &super::GltfConcavityRaw) -> GltfMeasure<f64> {
    raw.hull_volume
        .filter(|volume| *volume > 0.0)
        .map(|volume| {
            if context.solid.is_some() {
                estimate((volume - context.volume).max(0.0), GltfUnit::CubicMetre, context.sample_count, Some(context.topology))
            } else {
                unavailable(GltfUnit::CubicMetre, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology))
            }
        })
        .unwrap_or_else(|| unavailable(GltfUnit::CubicMetre, GltfAvailability::Degenerate, Vec::new(), context.sample_count, Some(context.topology)))
}

pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, ids.to_vec(), 0, None).await
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.concavity.convex_hull_gap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.convex-hull-gap.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
