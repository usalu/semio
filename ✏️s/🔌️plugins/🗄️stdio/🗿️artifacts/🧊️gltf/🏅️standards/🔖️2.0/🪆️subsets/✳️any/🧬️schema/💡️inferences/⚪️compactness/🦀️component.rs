//! ⚪️ GLTF compactness indicators.

use super::geometric_analysis::{GltfGeometryContext, convex_hull_metrics, hull_sample};
use super::super::super::modules::{inference_measures::{estimate, exact, unavailable}};
use super::super::super::modules::measurement_contracts::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfCompactnessIndicators {
    pub compactness: GltfMeasure<f64>,
    pub surface_to_volume_ratio: GltfMeasure<f64>,
    pub sphericity: GltfMeasure<f64>,
    pub compactness_index: GltfMeasure<f64>,
    pub hull_fill_ratio: GltfMeasure<f64>,
}

pub struct GltfCompactnessInference;

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfCompactnessInference {
    type Output = GltfCompactnessIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        let ratio = (context.volume > 1e-15 && context.topology.watertight && context.topology.manifold && context.topology.oriented).then_some(context.surface_area / context.volume);
        let sphericity = (context.volume > 1e-15 && context.surface_area > 0.0 && context.topology.watertight).then_some(std::f64::consts::PI.powf(1.0 / 3.0) * (6.0 * context.volume).powf(2.0 / 3.0) / context.surface_area);
        let hull_input = hull_sample(&context.points, context.policy.sampling_budget as usize);
        let tolerance = (context.diagonal * context.policy.relative_tolerance).max(context.policy.absolute_length_tolerance);
        let hull = convex_hull_metrics(&hull_input, tolerance);
        let compact = |value: Option<f64>| {
            value
                .map(|value| exact::<f64>(value, GltfUnit::Unitless, context.sample_count, Some(context.topology)))
                .unwrap_or_else(|| unavailable::<f64>(GltfUnit::Unitless, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology)))
        };
        Self::Output {
            compactness: compact(sphericity),
            surface_to_volume_ratio: ratio
                .map(|value| exact(value, GltfUnit::InverseMetre, context.sample_count, Some(context.topology)))
                .unwrap_or_else(|| unavailable(GltfUnit::InverseMetre, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology))),
            sphericity: compact(sphericity),
            compactness_index: compact(sphericity),
            hull_fill_ratio: hull
                .as_ref()
                .filter(|(_, volume, _)| *volume > 0.0)
                .map(|(_, volume, _)| {
                    if context.solid.is_some() {
                        estimate((context.volume / *volume).clamp(0.0, 1.0), GltfUnit::Unitless, context.sample_count, Some(context.topology))
                    } else {
                        unavailable(GltfUnit::Unitless, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology))
                    }
                })
                .unwrap_or_else(|| unavailable(GltfUnit::Unitless, GltfAvailability::Degenerate, Vec::new(), context.sample_count, Some(context.topology))),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        let unitless = || unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None);
        Self::Output {
            compactness: unitless(),
            surface_to_volume_ratio: unavailable(GltfUnit::InverseMetre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            sphericity: unitless(),
            compactness_index: unitless(),
            hull_fill_ratio: unitless(),
        }
    }
}
