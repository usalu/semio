//! ↕️ GLTF thickness indicators.

use super::geometric_analysis::{GltfGeometryContext, statistics, thickness_samples};
use super::super::modules::{inference_measures::{estimate, unavailable}};
use super::super::modules::measurement_contracts::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfThicknessIndicators {
    pub mean_thickness: GltfMeasure<f64>,
    pub minimum_thickness: GltfMeasure<f64>,
    pub thickness_variability: GltfMeasure<f64>,
    pub thickness_distribution: GltfMeasure<GltfStatistics>,
}

pub struct GltfThicknessInference;

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfThicknessInference {
    type Output = GltfThicknessIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        let samples = if context.topology.watertight && context.topology.manifold { thickness_samples(&context.points, &context.faces, context.policy.sampling_budget as usize, context.policy.absolute_length_tolerance) } else { Vec::new() };
        let distribution = statistics(&samples, &context.policy.histogram_edges);
        let measure = |value: Option<f64>| {
            value.map(|value| estimate::<f64>(value, GltfUnit::Metre, samples.len(), Some(context.topology))).unwrap_or_else(|| unavailable::<f64>(GltfUnit::Metre, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology)))
        };
        Self::Output {
            mean_thickness: measure(distribution.mean),
            minimum_thickness: measure(distribution.minimum),
            thickness_variability: measure(distribution.standard_deviation),
            thickness_distribution: if samples.is_empty() {
                unavailable(GltfUnit::Metre, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology))
            } else {
                estimate(distribution, GltfUnit::Metre, samples.len(), Some(context.topology))
            },
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            mean_thickness: unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            minimum_thickness: unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            thickness_variability: unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            thickness_distribution: unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
        }
    }
}
