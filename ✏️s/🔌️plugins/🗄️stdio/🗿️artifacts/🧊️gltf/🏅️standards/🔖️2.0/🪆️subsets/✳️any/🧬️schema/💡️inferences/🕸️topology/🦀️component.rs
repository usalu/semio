//! 🕸 GLTF topology indicators.

#[path = "boundary-loops/🦀️component.rs"]
pub mod boundary_loops;
#[path = "euler-characteristic/🦀️component.rs"]
pub mod euler_characteristic;
#[path = "genus/🦀️component.rs"]
pub mod genus;
#[path = "handles/🦀️component.rs"]
pub mod handles;
#[path = "holes/🦀️component.rs"]
pub mod holes;

use super::super::modules::measurement_contracts::*;
use super::geometry_core::GltfGeometryContext;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
