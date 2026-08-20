//! ⚪️ GLTF compactness indicators.

#[path = "compactness/🦀️component.rs"]
pub mod compactness;
#[path = "compactness-index/🦀️component.rs"]
pub mod compactness_index;
#[path = "hull-fill-ratio/🦀️component.rs"]
pub mod hull_fill_ratio;
#[path = "sphericity/🦀️component.rs"]
pub mod sphericity;
#[path = "surface-to-volume-ratio/🦀️component.rs"]
pub mod surface_to_volume_ratio;

use super::super::modules::measurement_contracts::*;
use super::geometry_core::{convex_hull_metrics, hull_sample, GltfGeometryContext};
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

pub(crate) struct GltfCompactnessRaw {
    pub(crate) ratio: Option<f64>,
    pub(crate) sphericity: Option<f64>,
    pub(crate) hull_volume: Option<f64>,
}

pub(crate) async fn raw(context: &GltfGeometryContext<'_>) -> GltfCompactnessRaw {
    let hull_input = hull_sample(&context.points, context.policy.sampling_budget as usize);
    let tolerance = (context.diagonal * context.policy.relative_tolerance).max(context.policy.absolute_length_tolerance);
    let hull_volume = convex_hull_metrics(&hull_input, tolerance).map(|(_, volume, _)| volume);
    GltfCompactnessRaw {
        ratio: (context.volume > 1e-15 && context.topology.watertight && context.topology.manifold && context.topology.oriented).then_some(context.surface_area / context.volume),
        sphericity: (context.volume > 1e-15 && context.surface_area > 0.0 && context.topology.watertight).then_some(std::f64::consts::PI.powf(1.0 / 3.0) * (6.0 * context.volume).powf(2.0 / 3.0) / context.surface_area),
        hull_volume,
    }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfCompactnessInference {
    type Output = GltfCompactnessIndicators;

    async fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        let raw = raw(context);
        Self::Output {
            compactness: compactness::from_raw(context, &raw).await,
            surface_to_volume_ratio: surface_to_volume_ratio::from_raw(context, &raw).await,
            sphericity: sphericity::from_raw(context, &raw).await,
            compactness_index: compactness_index::from_raw(context, &raw).await,
            hull_fill_ratio: hull_fill_ratio::from_raw(context, &raw).await,
        }
    }

    async fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            compactness: compactness::unavailable_measure(diagnostic_ids).await,
            surface_to_volume_ratio: surface_to_volume_ratio::unavailable_measure(diagnostic_ids).await,
            sphericity: sphericity::unavailable_measure(diagnostic_ids).await,
            compactness_index: compactness_index::unavailable_measure(diagnostic_ids).await,
            hull_fill_ratio: hull_fill_ratio::unavailable_measure(diagnostic_ids).await,
        }
    }
}
