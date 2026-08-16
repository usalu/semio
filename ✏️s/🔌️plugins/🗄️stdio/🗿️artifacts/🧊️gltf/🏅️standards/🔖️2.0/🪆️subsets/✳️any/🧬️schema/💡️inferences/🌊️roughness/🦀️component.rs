//! 🌊 GLTF roughness indicators.

use super::geometric_analysis::{GltfGeometryContext, roughness_samples, statistics};
use super::super::modules::{inference_measures::{estimate, exact, unavailable}, vector_operations::{cross, dot, norm, normalize, sub}};
use super::super::modules::measurement_contracts::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfRoughnessIndicators {
    pub deviation_from_ideal: GltfMeasure<GltfStatistics>,
    pub deviation_from_smoothed_geometry: GltfMeasure<GltfStatistics>,
    pub normal_variation: GltfMeasure<GltfStatistics>,
    pub surface_waviness: GltfMeasure<GltfStatistics>,
    pub irregularity: GltfMeasure<f64>,
}

pub struct GltfRoughnessInference;

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfRoughnessInference {
    type Output = GltfRoughnessIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        let values = roughness_samples(&context.points, &context.faces);
        let distribution = statistics(&values, &context.policy.histogram_edges);
        let irregularity = match (distribution.mean, distribution.standard_deviation) {
            (Some(mean), Some(deviation)) if mean > 0.0 => Some(deviation / mean),
            _ => None,
        };
        let mut normal_angles = Vec::new();
        for (&(first, second), adjacent_faces) in &context.edge_faces {
            if adjacent_faces.len() != 2 || norm(sub(context.points[second], context.points[first])) == 0.0 {
                continue;
            }
            let normal = |face_index: usize| {
                let face = context.faces[face_index];
                normalize(cross(sub(context.points[face[1]], context.points[face[0]]), sub(context.points[face[2]], context.points[face[0]])))
            };
            normal_angles.push(dot(normal(adjacent_faces[0].0), normal(adjacent_faces[1].0)).clamp(-1.0, 1.0).acos());
        }
        Self::Output {
            deviation_from_ideal: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology)),
            deviation_from_smoothed_geometry: estimate(distribution.clone(), GltfUnit::Metre, values.len(), Some(context.topology)),
            normal_variation: exact(statistics(&normal_angles, &context.policy.histogram_edges), GltfUnit::Radian, normal_angles.len(), Some(context.topology)),
            surface_waviness: estimate(distribution, GltfUnit::Metre, values.len(), Some(context.topology)),
            irregularity: irregularity
                .map(|value| estimate(value, GltfUnit::Unitless, values.len(), Some(context.topology)))
                .unwrap_or_else(|| unavailable(GltfUnit::Unitless, GltfAvailability::Degenerate, Vec::new(), context.sample_count, Some(context.topology))),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        let statistics = || unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None);
        Self::Output {
            deviation_from_ideal: statistics(),
            deviation_from_smoothed_geometry: statistics(),
            normal_variation: statistics(),
            surface_waviness: statistics(),
            irregularity: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
        }
    }
}
