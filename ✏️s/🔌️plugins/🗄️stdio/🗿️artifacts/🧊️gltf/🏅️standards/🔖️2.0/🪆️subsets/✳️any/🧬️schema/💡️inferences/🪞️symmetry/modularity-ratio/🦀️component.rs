//! 💡️ modularity-ratio atomic glTF inference leaf.
use super::super::super::modules::mesh_topology::Topology;
use super::super::super::modules::{
    inference_measures::{estimate, unavailable},
    measurement_contracts::*,
};
use super::super::GltfPartInference;
use super::super::{geometry_core::GltfGeometryContext, GltfEntityIndicators, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};

pub struct GltfModularityRatioInference;

impl GltfInferenceLeaf for GltfModularityRatioInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor =
        GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.modularity-ratio.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.modularity-ratio.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor {
    GltfModularityRatioInference::DESCRIPTOR
}

pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology))
}

pub fn from_assembly(parts: &[GltfPartInference], policy: &GltfAnalysisPolicy, topology: Topology) -> Option<GltfMeasure<f64>> {
    super::assembly_ratios(parts, policy).map(|(_, modularity)| estimate(modularity, GltfUnit::Unitless, parts.len(), Some(topology)))
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.symmetry.modularity_ratio)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.modularity-ratio.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}
