//! 🌊 GLTF roughness indicators.

#[path = "🎯️deviation-from-ideal/🦀️.rs"]
pub mod deviation_from_ideal;
#[path = "🧽️deviation-from-smoothed-geometry/🦀️.rs"]
pub mod deviation_from_smoothed_geometry;
#[path = "🪨️irregularity/🦀️.rs"]
pub mod irregularity;
#[path = "🧭️normal-variation/🦀️.rs"]
pub mod normal_variation;
#[path = "🌊️surface-waviness/🦀️.rs"]
pub mod surface_waviness;

use super::super::modules::measurement_contracts::*;
use super::super::modules::vector_operations::{cross, dot, norm, normalize, sub};
use super::geometry_core::{roughness_samples, statistics, GltfGeometryContext};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfRoughnessIndicators {
    pub deviation_from_ideal: GltfMeasure<GltfStatistics>,
    pub deviation_from_smoothed_geometry: GltfMeasure<GltfStatistics>,
    pub normal_variation: GltfMeasure<GltfStatistics>,
    pub surface_waviness: GltfMeasure<GltfStatistics>,
    pub irregularity: GltfMeasure<f64>,
}

pub struct GltfRoughnessInference;

pub(crate) struct GltfRoughnessRaw {
    pub(crate) deviations: Vec<f64>,
    pub(crate) normal_angles: Vec<f64>,
    pub(crate) irregularity: Option<f64>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn raw(context: &GltfGeometryContext<'_>) -> GltfRoughnessRaw {
    let deviations = roughness_samples(&context.points, &context.faces);
    let distribution = statistics(&deviations, &context.policy.histogram_edges);
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
    GltfRoughnessRaw { deviations, normal_angles, irregularity }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfRoughnessInference {
    type Output = GltfRoughnessIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        let raw = raw(context);
        Self::Output {
            deviation_from_ideal: deviation_from_ideal::infer(context),
            deviation_from_smoothed_geometry: deviation_from_smoothed_geometry::from_raw(context, &raw),
            normal_variation: normal_variation::from_raw(context, &raw),
            surface_waviness: surface_waviness::from_raw(context, &raw),
            irregularity: irregularity::from_raw(context, &raw),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            deviation_from_ideal: deviation_from_ideal::unavailable_measure(diagnostic_ids),
            deviation_from_smoothed_geometry: deviation_from_smoothed_geometry::unavailable_measure(diagnostic_ids),
            normal_variation: normal_variation::unavailable_measure(diagnostic_ids),
            surface_waviness: surface_waviness::unavailable_measure(diagnostic_ids),
            irregularity: irregularity::unavailable_measure(diagnostic_ids),
        }
    }
}
