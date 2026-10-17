//! 📏 GLTF proportion indicators.

pub use semio_s_artifact_stdio_gltf_inferences_aspect_ratios::standards::v2_0::subsets::any::schema::inferences::proportion::aspect_ratios;
pub use semio_s_artifact_stdio_gltf_inferences_aspect_ratios::standards::v2_0::subsets::any::schema::inferences::proportion::elongation;
pub use semio_s_artifact_stdio_gltf_inferences_flatness::standards::v2_0::subsets::any::schema::inferences::proportion::flatness;
pub use semio_s_artifact_stdio_gltf_inferences_flatness::standards::v2_0::subsets::any::schema::inferences::proportion::slenderness;

use crate::standards::v2_0::subsets::any::schema::modules::measurement_contracts::*;
use crate::standards::v2_0::subsets::any::schema::inferences::geometry_core::GltfGeometryContext;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
        Self::Output { aspect_ratios: aspect_ratios::infer(context), slenderness: slenderness::infer(context), flatness: flatness::infer(context), elongation: elongation::infer(context) }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            aspect_ratios: aspect_ratios::unavailable_measure(diagnostic_ids),
            slenderness: slenderness::unavailable_measure(diagnostic_ids),
            flatness: flatness::unavailable_measure(diagnostic_ids),
            elongation: elongation::unavailable_measure(diagnostic_ids),
        }
    }
}
