//! 🕸 GLTF topology indicators.

use super::geometric_analysis::{GltfGeometryContext};
use super::super::super::modules::{inference_measures::{exact, unavailable}};
use super::super::super::modules::measurement_contracts::*;
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
        let genus = || {
            context
                .topology
                .genus
                .map(|value| exact(value, GltfUnit::Unitless, context.sample_count, Some(context.topology)))
                .unwrap_or_else(|| unavailable(GltfUnit::Unitless, GltfAvailability::NonManifold, Vec::new(), context.sample_count, Some(context.topology)))
        };
        Self::Output {
            holes: genus(),
            handles: genus(),
            boundary_loops: exact(context.topology.boundary_loops, GltfUnit::Unitless, context.sample_count, Some(context.topology)),
            euler_characteristic: exact(context.topology.chi, GltfUnit::Unitless, context.sample_count, Some(context.topology)),
            genus: genus(),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            holes: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            handles: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            boundary_loops: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            euler_characteristic: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            genus: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
        }
    }
}
