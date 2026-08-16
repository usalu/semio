//! 📏 GLTF proportion indicators.

use super::geometric_analysis::{GltfGeometryContext};
use super::super::modules::{inference_measures::{exact, unavailable}};
use super::super::modules::measurement_contracts::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfProportionIndicators {
    pub aspect_ratios: GltfMeasure<GltfVec3>,
    pub slenderness: GltfMeasure<f64>,
    pub flatness: GltfMeasure<f64>,
    pub elongation: GltfMeasure<f64>,
}

pub struct GltfProportionInference;

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfProportionInference {
    type Output = GltfProportionIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        let mut extent = context.oriented_extent;
        extent.sort_by(|left, right| right.total_cmp(left));
        Self::Output {
            aspect_ratios: exact(
                GltfVec3::new([if extent[1] > 0.0 { extent[0] / extent[1] } else { 0.0 }, if extent[2] > 0.0 { extent[1] / extent[2] } else { 0.0 }, if extent[2] > 0.0 { extent[0] / extent[2] } else { 0.0 }]),
                GltfUnit::Unitless,
                context.sample_count,
                Some(context.topology),
            ),
            slenderness: exact(if extent[1] > 0.0 { extent[0] / extent[1] } else { 0.0 }, GltfUnit::Unitless, context.sample_count, Some(context.topology)),
            flatness: exact(if extent[1] > 0.0 { extent[2] / extent[1] } else { 0.0 }, GltfUnit::Unitless, context.sample_count, Some(context.topology)),
            elongation: exact(if extent[0] > 0.0 { extent[1] / extent[0] } else { 0.0 }, GltfUnit::Unitless, context.sample_count, Some(context.topology)),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            aspect_ratios: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            slenderness: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            flatness: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            elongation: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
        }
    }
}
