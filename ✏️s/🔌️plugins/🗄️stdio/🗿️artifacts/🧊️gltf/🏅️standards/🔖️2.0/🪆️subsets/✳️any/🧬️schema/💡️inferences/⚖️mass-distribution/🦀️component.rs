//! ⚖️ GLTF mass-distribution indicators.

use super::geometric_analysis::{GltfGeometryContext};
use super::super::super::modules::{inference_measures::{estimate, exact, unavailable}};
use super::super::super::modules::measurement_contracts::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
        let eigenvalues = context.principal_frame.eigenvalues;
        let moments = GltfVec3::new([eigenvalues[1] + eigenvalues[2], eigenvalues[0] + eigenvalues[2], eigenvalues[0] + eigenvalues[1]]);
        let tensor = vec![moments.x, 0.0, 0.0, 0.0, moments.y, 0.0, 0.0, 0.0, moments.z];
        let centroid = if context.topology.watertight && context.volume > 1e-15 {
            exact(GltfVec3::new(context.centroid), GltfUnit::Metre, context.sample_count, Some(context.topology))
        } else {
            estimate(GltfVec3::new(context.centroid), GltfUnit::Metre, context.sample_count, Some(context.topology))
        };
        Self::Output {
            centroid,
            principal_frame: estimate(context.principal_frame.clone(), GltfUnit::Unitless, context.sample_count, Some(context.topology)),
            principal_axes: estimate(context.principal_axes.clone(), GltfUnit::Unitless, context.sample_count, Some(context.topology)),
            moments_of_inertia: estimate(moments, GltfUnit::SquareMetre, context.sample_count, Some(context.topology)),
            inertia_tensor: estimate(tensor, GltfUnit::SquareMetre, context.sample_count, Some(context.topology)),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            centroid: unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            principal_frame: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            principal_axes: unavailable(GltfUnit::Unitless, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            moments_of_inertia: unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            inertia_tensor: unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
        }
    }
}
