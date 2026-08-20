//! 🧭 GLTF orientation indicators.

#[path = "face-normal-distribution/🦀️component.rs"]
pub mod face_normal_distribution;
#[path = "main-axis-direction/🦀️component.rs"]
pub mod main_axis_direction;
#[path = "orientation-consistency/🦀️component.rs"]
pub mod orientation_consistency;

use super::super::modules::measurement_contracts::*;
use super::super::modules::mesh_topology::Topology;
use super::geometry_core::{GltfGeometryContext, GltfPairGeometry};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfOrientationIndicators {
    pub main_axis_direction: GltfMeasure<GltfVec3>,
    pub face_normal_distribution: GltfMeasure<GltfStatistics>,
    pub orientation_consistency: GltfMeasure<f64>,
}

pub struct GltfOrientationInference;

impl GltfOrientationInference {
    pub(crate) async fn infer_pair(pair: &GltfPairGeometry) -> GltfMeasure<f64> {
        orientation_consistency::infer_pair(pair).await
    }

    pub(crate) async fn infer_assembly(indicators: &mut GltfOrientationIndicators, part_count: usize, sample_count: usize, topology: Topology) {
        if part_count > 1 {
            indicators.orientation_consistency = orientation_consistency::unavailable_for_assembly(sample_count, topology).await;
        }
    }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfOrientationInference {
    type Output = GltfOrientationIndicators;

    async fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output { main_axis_direction: main_axis_direction::infer(context).await, face_normal_distribution: face_normal_distribution::infer(context).await, orientation_consistency: orientation_consistency::infer(context).await }
    }

    async fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            main_axis_direction: main_axis_direction::unavailable_measure(diagnostic_ids).await,
            face_normal_distribution: face_normal_distribution::unavailable_measure(diagnostic_ids).await,
            orientation_consistency: orientation_consistency::unavailable_measure(diagnostic_ids).await,
        }
    }
}
