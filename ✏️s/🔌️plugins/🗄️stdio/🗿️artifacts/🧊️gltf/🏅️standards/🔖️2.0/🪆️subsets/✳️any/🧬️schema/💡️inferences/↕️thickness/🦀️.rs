//! ↕️ GLTF thickness indicators.

#[path = "mean-thickness/🦀️.rs"]
pub mod mean_thickness;
#[path = "minimum-thickness/🦀️.rs"]
pub mod minimum_thickness;
#[path = "thickness-distribution/🦀️.rs"]
pub mod thickness_distribution;
#[path = "thickness-variability/🦀️.rs"]
pub mod thickness_variability;

use super::super::modules::measurement_contracts::*;
use super::geometry_core::{statistics, thickness_samples, GltfGeometryContext};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfThicknessIndicators {
    pub mean_thickness: GltfMeasure<f64>,
    pub minimum_thickness: GltfMeasure<f64>,
    pub thickness_variability: GltfMeasure<f64>,
    pub thickness_distribution: GltfMeasure<GltfStatistics>,
}

pub struct GltfThicknessInference;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn samples(context: &GltfGeometryContext<'_>) -> Vec<f64> {
    if context.topology.watertight && context.topology.manifold {
        thickness_samples(&context.points, &context.faces, context.policy.sampling_budget as usize, context.policy.absolute_length_tolerance)
    } else {
        Vec::new()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn distribution(context: &GltfGeometryContext<'_>) -> GltfStatistics {
    statistics(&samples(context), &context.policy.histogram_edges)
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfThicknessInference {
    type Output = GltfThicknessIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output {
            mean_thickness: mean_thickness::infer(context),
            minimum_thickness: minimum_thickness::infer(context),
            thickness_variability: thickness_variability::infer(context),
            thickness_distribution: thickness_distribution::infer(context),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            mean_thickness: mean_thickness::unavailable_measure(diagnostic_ids),
            minimum_thickness: minimum_thickness::unavailable_measure(diagnostic_ids),
            thickness_variability: thickness_variability::unavailable_measure(diagnostic_ids),
            thickness_distribution: thickness_distribution::unavailable_measure(diagnostic_ids),
        }
    }
}
