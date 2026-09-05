//! 💡️ interference-volume atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
    mesh_topology::Topology,
};
use super::super::{
    geometry_core::{GltfGeometryContext, GltfPairGeometry},
    GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS,
};

pub struct GltfInterferenceVolumeInference;

impl GltfInferenceLeaf for GltfInterferenceVolumeInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.interference-volume.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.interference-volume.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfInterferenceVolumeInference::DESCRIPTOR
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn infer_pair(pair: &GltfPairGeometry) -> GltfMeasure<f64> {
    pair.overlap.map(|(volume, samples)| estimate(volume, GltfUnit::CubicMetre, samples, None)).unwrap_or_else(|| unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, Vec::new(), pair.sample_count, None))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn from_assembly(volume: f64, complete: bool, pair_count: usize, sample_count: usize, topology: Topology) -> Option<GltfMeasure<f64>> {
    (pair_count > 0 && complete).then(|| estimate(volume, GltfUnit::CubicMetre, sample_count, Some(topology)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_result(indicators: &GltfEntityIndicators) -> dsl::DslValue {
    dsl::ToValue::to_value(&indicators.clearance.interference_volume)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.interference-volume.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
