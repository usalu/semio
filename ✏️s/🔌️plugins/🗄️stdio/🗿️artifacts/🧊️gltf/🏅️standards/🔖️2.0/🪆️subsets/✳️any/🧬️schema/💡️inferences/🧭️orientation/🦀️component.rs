//! 🧭 GLTF orientation indicators.

use super::geometric_analysis::{GltfGeometryContext, GltfPairGeometry, statistics};
use super::super::super::modules::{inference_measures::{estimate, exact, unavailable}, mesh_topology::Topology, vector_operations::{cross, dot, normalize, sub}};
use super::super::super::modules::measurement_contracts::*;
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
    pub(crate) fn infer_pair(pair: &GltfPairGeometry) -> GltfMeasure<f64> {
        unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), pair.sample_count, None)
    }

    pub(crate) fn infer_assembly(indicators: &mut GltfOrientationIndicators, part_count: usize, sample_count: usize, topology: Topology) {
        if part_count > 1 {
            indicators.orientation_consistency = unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), sample_count, Some(topology));
        }
    }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfOrientationInference {
    type Output = GltfOrientationIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        let face_angles = context
            .faces
            .iter()
            .map(|face| {
                let normal = normalize(cross(sub(context.points[face[1]], context.points[face[0]]), sub(context.points[face[2]], context.points[face[0]])));
                dot(normal, context.principal_frame.axes[0].array()).clamp(-1.0, 1.0).acos()
            })
            .collect::<Vec<_>>();
        Self::Output {
            main_axis_direction: estimate(context.principal_axes[0].direction, GltfUnit::Unitless, context.sample_count, Some(context.topology)),
            face_normal_distribution: exact(statistics(&face_angles, &context.policy.histogram_edges), GltfUnit::Radian, context.faces.len(), Some(context.topology)),
            orientation_consistency: exact(if context.topology.oriented { 1.0 } else { 0.0 }, GltfUnit::Unitless, context.sample_count, Some(context.topology)),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            main_axis_direction: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            face_normal_distribution: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            orientation_consistency: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
        }
    }
}
