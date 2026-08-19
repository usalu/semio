//! 💡️ contact-area atomic glTF inference leaf.
use super::super::super::modules::{
    inference_measures::{estimate, exact, unavailable},
    measurement_contracts::*,
    mesh_topology::Topology,
};
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfContactAreaInference;

impl GltfInferenceLeaf for GltfContactAreaInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.contact-area.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.contact-area.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub async fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfContactAreaInference::DESCRIPTOR
}

pub(crate) async fn infer_pair(pair: &super::super::geometry_core::GltfPairGeometry) -> GltfMeasure<f64> {
    pair.contact_area
        .map(|area| if area == 0.0 { exact(area, GltfUnit::SquareMetre, pair.sample_count, None) } else { estimate(area, GltfUnit::SquareMetre, pair.sample_count, None) })
        .unwrap_or_else(|| unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), pair.sample_count, None))
}

pub(crate) async fn from_assembly(part_count: usize, area: f64, complete: bool, sample_count: usize, topology: Topology) -> GltfMeasure<f64> {
    if part_count <= 1 {
        return exact(0.0, GltfUnit::SquareMetre, sample_count, Some(topology));
    }
    if complete {
        return estimate(area, GltfUnit::SquareMetre, sample_count, Some(topology));
    }
    unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), sample_count, Some(topology))
}
pub(crate) async fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology))
}

pub async fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub async fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.area_volume.contact_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.contact-area.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
