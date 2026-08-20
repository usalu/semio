//! 📏 GLTF proportion indicators.

#[path = "aspect-ratios/🦀️component.rs"]
pub mod aspect_ratios;
#[path = "elongation/🦀️component.rs"]
pub mod elongation;
#[path = "flatness/🦀️component.rs"]
pub mod flatness;
#[path = "slenderness/🦀️component.rs"]
pub mod slenderness;

use super::super::modules::measurement_contracts::*;
use super::geometry_core::GltfGeometryContext;
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

    async fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output { aspect_ratios: aspect_ratios::infer(context).await, slenderness: slenderness::infer(context).await, flatness: flatness::infer(context).await, elongation: elongation::infer(context).await }
    }

    async fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            aspect_ratios: aspect_ratios::unavailable_measure(diagnostic_ids).await,
            slenderness: slenderness::unavailable_measure(diagnostic_ids).await,
            flatness: flatness::unavailable_measure(diagnostic_ids).await,
            elongation: elongation::unavailable_measure(diagnostic_ids).await,
        }
    }
}
