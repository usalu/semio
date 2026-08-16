//! 🕳 GLTF concavity indicators.

#[path = "convex-hull-gap/🦀️component.rs"]
pub mod convex_hull_gap;
#[path = "reentrant-area/🦀️component.rs"]
pub mod reentrant_area;
#[path = "reentrant-volume/🦀️component.rs"]
pub mod reentrant_volume;
#[path = "concavity-index/🦀️component.rs"]
pub mod concavity_index;

use super::geometry_core::{GltfGeometryContext, convex_hull_metrics, hull_sample, triangle_area};
use super::super::modules::{vector_operations::{add, dot, mul}};
use super::super::modules::measurement_contracts::*;
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

pub(crate) struct GltfConcavityRaw {
    pub(crate) hull_volume: Option<f64>,
    pub(crate) reentrant_area: Option<f64>,
}

pub(crate) fn raw(context: &GltfGeometryContext<'_>) -> GltfConcavityRaw {
    let hull_input = hull_sample(&context.points, context.policy.sampling_budget as usize);
    let tolerance = (context.diagonal * context.policy.relative_tolerance).max(context.policy.absolute_length_tolerance);
    let hull = convex_hull_metrics(&hull_input, tolerance);
    let reentrant_area = hull.as_ref().map(|(_, _, planes)| {
        context.faces.iter().filter(|face| {
            let centroid = mul(add(add(context.points[face[0]], context.points[face[1]]), context.points[face[2]]), 1.0 / 3.0);
            !planes.iter().any(|(normal, offset)| (dot(*normal, centroid) - *offset).abs() <= tolerance * 4.0)
        }).map(|face| triangle_area(context.points[face[0]], context.points[face[1]], context.points[face[2]])).sum::<f64>()
    });
    GltfConcavityRaw { hull_volume: hull.map(|(_, volume, _)| volume), reentrant_area }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfConcavityInference {
    type Output = GltfConcavityIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        let raw = raw(context);
        Self::Output {
            convex_hull_gap: convex_hull_gap::from_raw(context, &raw),
            reentrant_area: reentrant_area::from_raw(context, &raw),
            reentrant_volume: reentrant_volume::from_raw(context, &raw),
            concavity_index: concavity_index::from_raw(context, &raw),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            convex_hull_gap: convex_hull_gap::unavailable_measure(diagnostic_ids),
            reentrant_area: reentrant_area::unavailable_measure(diagnostic_ids),
            reentrant_volume: reentrant_volume::unavailable_measure(diagnostic_ids),
            concavity_index: concavity_index::unavailable_measure(diagnostic_ids),
        }
    }
}
