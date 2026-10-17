//! ↔️ GLTF clearance indicators.

pub use semio_s_artifact_stdio_gltf_inferences_volume::standards::v2_0::subsets::any::schema::inferences::clearance::clearance_distribution;
pub use semio_s_artifact_stdio_gltf_inferences_volume::standards::v2_0::subsets::any::schema::inferences::clearance::interference_volume;
pub use semio_s_artifact_stdio_gltf_inferences_volume::standards::v2_0::subsets::any::schema::inferences::clearance::minimum_distance_to_neighbors;
pub use semio_s_artifact_stdio_gltf_inferences_volume::standards::v2_0::subsets::any::schema::inferences::clearance::overlap_volume;

use crate::standards::v2_0::subsets::any::schema::modules::measurement_contracts::*;
use crate::standards::v2_0::subsets::any::schema::modules::mesh_topology::Topology;
use crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::{GltfGeometryContext, GltfPairGeometry};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfClearanceIndicators {
    pub minimum_distance_to_neighbors: GltfMeasure<f64>,
    pub clearance_distribution: GltfMeasure<GltfStatistics>,
    pub interference_volume: GltfMeasure<f64>,
    pub overlap_volume: GltfMeasure<f64>,
}

pub struct GltfClearanceInference;

impl GltfClearanceInference {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn infer_pair(pair: &GltfPairGeometry, policy: &GltfAnalysisPolicy) -> (GltfMeasure<f64>, GltfMeasure<GltfStatistics>, GltfMeasure<f64>, GltfMeasure<f64>) {
        (minimum_distance_to_neighbors::infer_pair(pair), clearance_distribution::infer_pair(pair, policy), interference_volume::infer_pair(pair), overlap_volume::infer_pair(pair))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn infer_assembly(indicators: &mut GltfClearanceIndicators, distances: &[f64], (overlap_volume, overlap_complete): (f64, bool), pair_count: usize, policy: &GltfAnalysisPolicy, sample_count: usize, topology: Topology) {
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

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output {
            minimum_distance_to_neighbors: minimum_distance_to_neighbors::infer(context),
            clearance_distribution: clearance_distribution::infer(context),
            interference_volume: interference_volume::infer(context),
            overlap_volume: overlap_volume::infer(context),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            minimum_distance_to_neighbors: minimum_distance_to_neighbors::unavailable_measure(diagnostic_ids),
            clearance_distribution: clearance_distribution::unavailable_measure(diagnostic_ids),
            interference_volume: interference_volume::unavailable_measure(diagnostic_ids),
            overlap_volume: overlap_volume::unavailable_measure(diagnostic_ids),
        }
    }
}
