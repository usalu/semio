//! 🧭 GLTF orientation indicators.

#[path = "face-normal-distribution/🦀️.rs"]
pub mod face_normal_distribution;
#[path = "main-axis-direction/🦀️.rs"]
pub mod main_axis_direction;
#[path = "orientation-consistency/🦀️.rs"]
pub mod orientation_consistency;

use super::super::modules::measurement_contracts::*;
use super::super::modules::mesh_topology::Topology;
use super::geometry_core::{GltfGeometryContext, GltfPairGeometry};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfOrientationIndicators {
    pub main_axis_direction: GltfMeasure<GltfVec3>,
    pub face_normal_distribution: GltfMeasure<GltfStatistics>,
    pub orientation_consistency: GltfMeasure<f64>,
}

pub struct GltfOrientationInference;

impl GltfOrientationInference {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn infer_pair(pair: &GltfPairGeometry) -> GltfMeasure<f64> {
        orientation_consistency::infer_pair(pair)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn infer_assembly(indicators: &mut GltfOrientationIndicators, part_count: usize, sample_count: usize, topology: Topology) {
        if part_count > 1 {
            indicators.orientation_consistency = orientation_consistency::unavailable_for_assembly(sample_count, topology);
        }
    }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfOrientationInference {
    type Output = GltfOrientationIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output { main_axis_direction: main_axis_direction::infer(context), face_normal_distribution: face_normal_distribution::infer(context), orientation_consistency: orientation_consistency::infer(context) }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            main_axis_direction: main_axis_direction::unavailable_measure(diagnostic_ids),
            face_normal_distribution: face_normal_distribution::unavailable_measure(diagnostic_ids),
            orientation_consistency: orientation_consistency::unavailable_measure(diagnostic_ids),
        }
    }
}
