//! 🕳 GLTF concavity indicators.

use super::geometric_analysis::{GltfGeometryContext, convex_hull_metrics, hull_sample, triangle_area};
use super::super::super::modules::{inference_measures::{estimate, unavailable}, vector_operations::{add, dot, mul}};
use super::super::super::modules::measurement_contracts::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfConcavityIndicators {
    pub convex_hull_gap: GltfMeasure<f64>,
    pub reentrant_area: GltfMeasure<f64>,
    pub reentrant_volume: GltfMeasure<f64>,
    pub concavity_index: GltfMeasure<f64>,
}

pub struct GltfConcavityInference;

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfConcavityInference {
    type Output = GltfConcavityIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        let hull_input = hull_sample(&context.points, context.policy.sampling_budget as usize);
        let tolerance = (context.diagonal * context.policy.relative_tolerance).max(context.policy.absolute_length_tolerance);
        let hull = convex_hull_metrics(&hull_input, tolerance);
        let reentrant_area = hull.as_ref().map(|(_, _, planes)| {
            context
                .faces
                .iter()
                .filter(|face| {
                    let centroid = mul(add(add(context.points[face[0]], context.points[face[1]]), context.points[face[2]]), 1.0 / 3.0);
                    !planes.iter().any(|(normal, offset)| (dot(*normal, centroid) - *offset).abs() <= tolerance * 4.0)
                })
                .map(|face| triangle_area(context.points[face[0]], context.points[face[1]], context.points[face[2]]))
                .sum::<f64>()
        });
        let hull_gap = |unit, ratio: bool| {
            hull.as_ref()
                .filter(|(_, volume, _)| *volume > 0.0)
                .map(|(_, volume, _)| {
                    if context.solid.is_some() {
                        let value = if ratio { (1.0 - context.volume / *volume).clamp(0.0, 1.0) } else { (*volume - context.volume).max(0.0) };
                        estimate(value, unit, context.sample_count, Some(context.topology))
                    } else {
                        unavailable(unit, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology))
                    }
                })
                .unwrap_or_else(|| unavailable(unit, GltfAvailability::Degenerate, Vec::new(), context.sample_count, Some(context.topology)))
        };
        Self::Output {
            convex_hull_gap: hull_gap(GltfUnit::CubicMetre, false),
            reentrant_area: reentrant_area
                .map(|area| estimate(area, GltfUnit::SquareMetre, context.sample_count, Some(context.topology)))
                .unwrap_or_else(|| unavailable(GltfUnit::SquareMetre, GltfAvailability::Degenerate, Vec::new(), context.sample_count, Some(context.topology))),
            reentrant_volume: hull_gap(GltfUnit::CubicMetre, false),
            concavity_index: hull_gap(GltfUnit::Unitless, true),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            convex_hull_gap: unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            reentrant_area: unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            reentrant_volume: unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            concavity_index: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
        }
    }
}
