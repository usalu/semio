//! 🌀 GLTF curvature indicators.

#[path = "curvature-histogram/🦀️component.rs"]
pub mod curvature_histogram;
#[path = "gaussian-curvature/🦀️component.rs"]
pub mod gaussian_curvature;
#[path = "mean-curvature/🦀️component.rs"]
pub mod mean_curvature;
#[path = "sharp-feature-proportion/🦀️component.rs"]
pub mod sharp_feature_proportion;

use super::super::modules::measurement_contracts::*;
use super::super::modules::vector_operations::{cross, dot, norm, normalize, sub};
use super::geometry_core::{statistics, triangle_area, GltfGeometryContext};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfCurvatureIndicators {
    pub mean_curvature: GltfMeasure<GltfStatistics>,
    pub gaussian_curvature: GltfMeasure<GltfStatistics>,
    pub curvature_histogram: GltfMeasure<GltfStatistics>,
    pub sharp_feature_proportion: GltfMeasure<f64>,
}

pub struct GltfCurvatureInference;

pub(crate) struct GltfCurvatureRaw {
    pub(crate) edge_curvatures: Vec<f64>,
    pub(crate) gaussian_values: Vec<f64>,
    pub(crate) sharp_feature_proportion: f64,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn raw(context: &GltfGeometryContext<'_>) -> GltfCurvatureRaw {
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
    let mut vertex_areas = vec![0.0; context.sample_count];
    let mut angle_sums = vec![0.0; context.sample_count];
    for face in &context.faces {
        let area = triangle_area(context.points[face[0]], context.points[face[1]], context.points[face[2]]);
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
    GltfCurvatureRaw { edge_curvatures, gaussian_values, sharp_feature_proportion: if edge_length > 0.0 { sharp_length / edge_length } else { 0.0 } }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfCurvatureInference {
    type Output = GltfCurvatureIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        let raw = raw(context);
        Self::Output {
            mean_curvature: mean_curvature::from_raw(context, &raw),
            gaussian_curvature: gaussian_curvature::from_raw(context, &raw),
            curvature_histogram: curvature_histogram::from_raw(context, &raw),
            sharp_feature_proportion: sharp_feature_proportion::from_raw(context, &raw),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            mean_curvature: mean_curvature::unavailable_measure(diagnostic_ids),
            gaussian_curvature: gaussian_curvature::unavailable_measure(diagnostic_ids),
            curvature_histogram: curvature_histogram::unavailable_measure(diagnostic_ids),
            sharp_feature_proportion: sharp_feature_proportion::unavailable_measure(diagnostic_ids),
        }
    }
}
