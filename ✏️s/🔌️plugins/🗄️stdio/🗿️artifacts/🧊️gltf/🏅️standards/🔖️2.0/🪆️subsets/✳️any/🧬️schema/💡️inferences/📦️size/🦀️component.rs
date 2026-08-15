//! 📦 GLTF size indicators.

use super::geometry::{cross, estimate, exact, sub, GltfGeometryContext};
use super::measure::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfSizeIndicators {
    pub overall_size: GltfMeasure<f64>,
    pub axis_aligned_bounds: GltfMeasure<GltfBounds3>,
    pub oriented_bounds: GltfMeasure<GltfBounds3>,
    pub bounding_box_dimensions: GltfMeasure<GltfVec3>,
    pub characteristic_length: GltfMeasure<f64>,
    pub footprint_area: GltfMeasure<f64>,
    pub projected_area: GltfMeasure<GltfStatistics>,
}

pub struct GltfSizeInference;

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfSizeInference {
    type Output = GltfSizeIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        let projected = [0, 1, 2].map(|axis| context.faces.iter().map(|face| 0.5 * cross(sub(context.points[face[1]], context.points[face[0]]), sub(context.points[face[2]], context.points[face[0]]))[axis].abs()).sum::<f64>());
        Self::Output {
            overall_size: exact(context.diagonal, GltfUnit::Metre, context.sample_count, Some(context.topology)),
            axis_aligned_bounds: exact(context.bounds.clone(), GltfUnit::Metre, context.sample_count, Some(context.topology)),
            oriented_bounds: exact(context.oriented_bounds.clone(), GltfUnit::Metre, context.sample_count, Some(context.topology)),
            bounding_box_dimensions: exact(GltfVec3::new(context.dimensions), GltfUnit::Metre, context.sample_count, Some(context.topology)),
            characteristic_length: exact(if context.surface_area > 0.0 { context.surface_area.sqrt() } else { context.diagonal }, GltfUnit::Metre, context.sample_count, Some(context.topology)),
            footprint_area: estimate(projected[2], GltfUnit::SquareMetre, context.sample_count, Some(context.topology)),
            projected_area: estimate(super::geometry::statistics(&projected, &context.policy.histogram_edges), GltfUnit::SquareMetre, context.sample_count, Some(context.topology)),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            overall_size: super::geometry::unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            axis_aligned_bounds: super::geometry::unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            oriented_bounds: super::geometry::unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            bounding_box_dimensions: super::geometry::unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            characteristic_length: super::geometry::unavailable(GltfUnit::Metre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            footprint_area: super::geometry::unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
            projected_area: super::geometry::unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None),
        }
    }
}
