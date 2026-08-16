//! ↔️ GLTF clearance indicators.

use super::geometric_analysis::{GltfGeometryContext, GltfPairGeometry, statistics};
use super::super::modules::{inference_measures::{estimate, unavailable}, mesh_topology::Topology};
use super::super::modules::measurement_contracts::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfClearanceIndicators {
    pub minimum_distance_to_neighbors: GltfMeasure<f64>,
    pub clearance_distribution: GltfMeasure<GltfStatistics>,
    pub interference_volume: GltfMeasure<f64>,
    pub overlap_volume: GltfMeasure<f64>,
}

pub struct GltfClearanceInference;

impl GltfClearanceInference {
    pub(crate) fn infer_pair(pair: &GltfPairGeometry, policy: &GltfAnalysisPolicy) -> (GltfMeasure<f64>, GltfMeasure<GltfStatistics>, GltfMeasure<f64>, GltfMeasure<f64>) {
        let minimum_distance = estimate(pair.minimum_distance, GltfUnit::Metre, pair.sample_count, None);
        let distribution = estimate(statistics(&[pair.minimum_distance], &policy.histogram_edges), GltfUnit::Metre, pair.sample_count, None);
        let overlap = pair.overlap.map(|(volume, samples)| estimate(volume, GltfUnit::CubicMetre, samples, None)).unwrap_or_else(|| unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, Vec::new(), pair.sample_count, None));
        (minimum_distance, distribution, overlap.clone(), overlap)
    }

    pub(crate) fn infer_assembly(indicators: &mut GltfClearanceIndicators, distances: &[f64], overlap_volume: f64, overlap_complete: bool, pair_count: usize, policy: &GltfAnalysisPolicy, sample_count: usize, topology: Topology) {
        if !distances.is_empty() {
            indicators.minimum_distance_to_neighbors = estimate(distances.iter().copied().fold(f64::INFINITY, f64::min), GltfUnit::Metre, sample_count, Some(topology));
            indicators.clearance_distribution = estimate(statistics(distances, &policy.histogram_edges), GltfUnit::Metre, sample_count, Some(topology));
        }
        if pair_count > 0 && overlap_complete {
            indicators.interference_volume = estimate(overlap_volume, GltfUnit::CubicMetre, sample_count, Some(topology));
            indicators.overlap_volume = estimate(overlap_volume, GltfUnit::CubicMetre, sample_count, Some(topology));
        }
    }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfClearanceInference {
    type Output = GltfClearanceIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output {
            minimum_distance_to_neighbors: unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology)),
            clearance_distribution: unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology)),
            interference_volume: unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology)),
            overlap_volume: unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology)),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            minimum_distance_to_neighbors: unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            clearance_distribution: unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            interference_volume: unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            overlap_volume: unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
        }
    }
}
