//! 🕸 GLTF topology indicators.

pub use semio_s_artifact_stdio_gltf_inferences_repetition_ratio::standards::v2_0::subsets::any::schema::inferences::topology::boundary_loops;
pub use semio_s_artifact_stdio_gltf_inferences_repetition_ratio::standards::v2_0::subsets::any::schema::inferences::topology::euler_characteristic;
pub use semio_s_artifact_stdio_gltf_inferences_repetition_ratio::standards::v2_0::subsets::any::schema::inferences::topology::genus;
pub use semio_s_artifact_stdio_gltf_inferences_repetition_ratio::standards::v2_0::subsets::any::schema::inferences::topology::handles;
pub use semio_s_artifact_stdio_gltf_inferences_adjacency::standards::v2_0::subsets::any::schema::inferences::topology::holes;

use crate::standards::v2_0::subsets::any::schema::modules::measurement_contracts::*;
use crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::GltfGeometryContext;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfTopologyIndicators {
    pub holes: GltfMeasure<u64>,
    pub handles: GltfMeasure<u64>,
    pub boundary_loops: GltfMeasure<u64>,
    pub euler_characteristic: GltfMeasure<i64>,
    pub genus: GltfMeasure<u64>,
}

pub struct GltfTopologyInference;

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfTopologyInference {
    type Output = GltfTopologyIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output { holes: holes::infer(context), handles: handles::infer(context), boundary_loops: boundary_loops::infer(context), euler_characteristic: euler_characteristic::infer(context), genus: genus::infer(context) }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            holes: holes::unavailable_measure(diagnostic_ids),
            handles: handles::unavailable_measure(diagnostic_ids),
            boundary_loops: boundary_loops::unavailable_measure(diagnostic_ids),
            euler_characteristic: euler_characteristic::unavailable_measure(diagnostic_ids),
            genus: genus::unavailable_measure(diagnostic_ids),
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️canonical-vectors/🦀️.rs"]
mod canonical_vectors;
