//! 💡️ thickness-variability atomic glTF inference leaf.
use super::super::{geometry_core::GltfGeometryContext, GltfInferenceLeaf, GltfInferenceLeafDescriptor, GLTF_GEOMETRY_READS};
use super::super::super::modules::{inference_measures::{estimate, unavailable}, measurement_contracts::*};

pub struct GltfThicknessVariabilityInference;

impl GltfInferenceLeaf for GltfThicknessVariabilityInference {
    const DESCRIPTOR: GltfInferenceLeafDescriptor = GltfInferenceLeafDescriptor { id: "s.stdio.gltf.inference.thickness-variability.v1", algorithm_version: 1, cache_key: "s.stdio.gltf.inference.thickness-variability.v1:geometry-v2", reads: GLTF_GEOMETRY_READS };
}

pub fn descriptor() -> GltfInferenceLeafDescriptor { GltfThicknessVariabilityInference::DESCRIPTOR }

pub fn infer(context: &GltfGeometryContext<'_>) -> GltfMeasure<f64> {
    let distribution = super::distribution(context); distribution.standard_deviation.map(|value| estimate(value, GltfUnit::Metre, super::samples(context).len(), Some(context.topology))).unwrap_or_else(|| unavailable(GltfUnit::Metre, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology)))
}

pub fn unavailable_measure(ids: &[String]) -> GltfMeasure<f64> {
    unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, ids.to_vec(), 0, None)
}

pub fn encode_result(indicators: &GltfEntityIndicators) -> Result<serde_json::Value, serde_json::Error> {
    serde_json::to_value(&indicators.thickness.thicknessVariability)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_versioned_and_cacheable() {
        assert_eq!(descriptor().id, "s.stdio.gltf.inference.thickness-variability.v1");
        assert_eq!(descriptor().algorithm_version, 1);
    }
}

