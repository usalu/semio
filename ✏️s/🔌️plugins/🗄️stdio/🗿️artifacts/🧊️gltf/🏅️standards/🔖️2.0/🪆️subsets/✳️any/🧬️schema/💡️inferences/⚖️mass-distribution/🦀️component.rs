//! ⚖️ GLTF mass-distribution indicators.

#[path = "centroid/🦀️component.rs"]
pub mod centroid;
#[path = "inertia-tensor/🦀️component.rs"]
pub mod inertia_tensor;
#[path = "moments-of-inertia/🦀️component.rs"]
pub mod moments_of_inertia;
#[path = "principal-axes/🦀️component.rs"]
pub mod principal_axes;
#[path = "principal-frame/🦀️component.rs"]
pub mod principal_frame;

use super::super::modules::measurement_contracts::*;
use super::geometry_core::GltfGeometryContext;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfMassIndicators {
    pub centroid: GltfMeasure<GltfVec3>,
    pub principal_frame: GltfMeasure<GltfPrincipalFrame>,
    pub principal_axes: GltfMeasure<Vec<GltfDirectionScore>>,
    pub moments_of_inertia: GltfMeasure<GltfVec3>,
    pub inertia_tensor: GltfMeasure<Vec<f64>>,
}

pub struct GltfMassInference;

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfMassInference {
    type Output = GltfMassIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output {
            centroid: centroid::infer(context),
            principal_frame: principal_frame::infer(context),
            principal_axes: principal_axes::infer(context),
            moments_of_inertia: moments_of_inertia::infer(context),
            inertia_tensor: inertia_tensor::infer(context),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            centroid: centroid::unavailable_measure(diagnostic_ids),
            principal_frame: principal_frame::unavailable_measure(diagnostic_ids),
            principal_axes: principal_axes::unavailable_measure(diagnostic_ids),
            moments_of_inertia: moments_of_inertia::unavailable_measure(diagnostic_ids),
            inertia_tensor: inertia_tensor::unavailable_measure(diagnostic_ids),
        }
    }
}
