//! 🌀 GLTF curvature indicators.

use super::geometric_analysis::{GltfGeometryContext, statistics};
use super::super::super::modules::{inference_measures::{estimate, exact, unavailable}, vector_operations::{cross, dot, norm, normalize, sub}};
use super::super::super::modules::measurement_contracts::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfCurvatureIndicators {
    pub mean_curvature: GltfMeasure<GltfStatistics>,
    pub gaussian_curvature: GltfMeasure<GltfStatistics>,
    pub curvature_histogram: GltfMeasure<GltfStatistics>,
    pub sharp_feature_proportion: GltfMeasure<f64>,
}

pub struct GltfCurvatureInference;

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfCurvatureInference {
    type Output = GltfCurvatureIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        let mut edge_curvatures = Vec::new();
        let mut sharp_length = 0.0;
        let mut edge_length = 0.0;
        for (&(first, second), adjacent_faces) in &context.edge_faces {
            let length = norm(sub(context.points[second], context.points[first]));
            edge_length += length;
            if adjacent_faces.len() == 2 {
                let normal = |face_index: usize| {
                    let face = context.faces[face_index];
                    normalize(cross(sub(context.points[face[1]], context.points[face[0]]), sub(context.points[face[2]], context.points[face[0]])))
                };
                let angle = dot(normal(adjacent_faces[0].0), normal(adjacent_faces[1].0)).clamp(-1.0, 1.0).acos();
                if length > 0.0 {
                    edge_curvatures.push(angle / length);
                }
                if angle > context.policy.sharp_feature_angle_radians {
                    sharp_length += length;
                }
            }
        }
        let mean_curvature = statistics(&edge_curvatures, &context.policy.histogram_edges);
        let mut vertex_areas = vec![0.0; context.sample_count];
        let mut angle_sums = vec![0.0; context.sample_count];
        for face in &context.faces {
            let area = super::geometric_analysis::triangle_area(context.points[face[0]], context.points[face[1]], context.points[face[2]]);
            for corner in 0..3 {
                let vertex = face[corner];
                let first = sub(context.points[face[(corner + 1) % 3]], context.points[vertex]);
                let second = sub(context.points[face[(corner + 2) % 3]], context.points[vertex]);
                angle_sums[vertex] += dot(normalize(first), normalize(second)).clamp(-1.0, 1.0).acos();
                vertex_areas[vertex] += area / 3.0;
            }
        }
        let boundary_vertices = context.edge_faces.iter().filter(|(_, faces)| faces.len() == 1).flat_map(|((first, second), _)| [*first, *second]).collect::<BTreeSet<_>>();
        let gaussian_values = (0..context.sample_count)
            .filter_map(|index| {
                (vertex_areas[index] > 0.0).then(|| {
                    let target = if boundary_vertices.contains(&index) { std::f64::consts::PI } else { 2.0 * std::f64::consts::PI };
                    (target - angle_sums[index]) / vertex_areas[index]
                })
            })
            .collect::<Vec<_>>();
        Self::Output {
            mean_curvature: estimate(mean_curvature.clone(), GltfUnit::InverseMetre, edge_curvatures.len(), Some(context.topology)),
            gaussian_curvature: estimate(statistics(&gaussian_values, &context.policy.histogram_edges), GltfUnit::InverseSquareMetre, gaussian_values.len(), Some(context.topology)),
            curvature_histogram: estimate(mean_curvature, GltfUnit::InverseMetre, edge_curvatures.len(), Some(context.topology)),
            sharp_feature_proportion: exact(if edge_length > 0.0 { sharp_length / edge_length } else { 0.0 }, GltfUnit::Unitless, context.sample_count, Some(context.topology)),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        let statistics = || unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None);
        Self::Output { mean_curvature: statistics(), gaussian_curvature: statistics(), curvature_histogram: statistics(), sharp_feature_proportion: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None) }
    }
}
