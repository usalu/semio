//! ↔️ GLTF clearance indicators.

#[path = "clearance-distribution/🦀️component.rs"]
pub mod clearance_distribution;
#[path = "interference-volume/🦀️component.rs"]
pub mod interference_volume;
#[path = "minimum-distance-to-neighbors/🦀️component.rs"]
pub mod minimum_distance_to_neighbors;
#[path = "overlap-volume/🦀️component.rs"]
pub mod overlap_volume;

use super::super::modules::measurement_contracts::*;
use super::super::modules::mesh_topology::Topology;
use super::geometry_core::{GltfGeometryContext, GltfPairGeometry};
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
    pub(crate) async fn infer_pair(pair: &GltfPairGeometry, policy: &GltfAnalysisPolicy) -> (GltfMeasure<f64>, GltfMeasure<GltfStatistics>, GltfMeasure<f64>, GltfMeasure<f64>) {
        (minimum_distance_to_neighbors::infer_pair(pair), clearance_distribution::infer_pair(pair, policy), interference_volume::infer_pair(pair), overlap_volume::infer_pair(pair))
    }

    pub(crate) async fn infer_assembly(indicators: &mut GltfClearanceIndicators, distances: &[f64], overlap_volume: f64, overlap_complete: bool, pair_count: usize, policy: &GltfAnalysisPolicy, sample_count: usize, topology: Topology) {
        if let Some(measure) = minimum_distance_to_neighbors::from_assembly(distances, sample_count, topology) {
            indicators.minimum_distance_to_neighbors = measure;
        }
        if let Some(measure) = clearance_distribution::from_assembly(distances, policy, sample_count, topology) {
            indicators.clearance_distribution = measure;
        }
        if let Some(measure) = interference_volume::from_assembly(overlap_volume, overlap_complete, pair_count, sample_count, topology) {
            indicators.interference_volume = measure;
        }
        if let Some(measure) = overlap_volume::from_assembly(overlap_volume, overlap_complete, pair_count, sample_count, topology) {
            indicators.overlap_volume = measure;
        }
    }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfClearanceInference {
    type Output = GltfClearanceIndicators;

    async fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output {
            minimum_distance_to_neighbors: minimum_distance_to_neighbors::infer(context),
            clearance_distribution: clearance_distribution::infer(context),
            interference_volume: interference_volume::infer(context),
            overlap_volume: overlap_volume::infer(context),
        }
    }

    async fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            minimum_distance_to_neighbors: minimum_distance_to_neighbors::unavailable_measure(diagnostic_ids),
            clearance_distribution: clearance_distribution::unavailable_measure(diagnostic_ids),
            interference_volume: interference_volume::unavailable_measure(diagnostic_ids),
            overlap_volume: overlap_volume::unavailable_measure(diagnostic_ids),
        }
    }
}
