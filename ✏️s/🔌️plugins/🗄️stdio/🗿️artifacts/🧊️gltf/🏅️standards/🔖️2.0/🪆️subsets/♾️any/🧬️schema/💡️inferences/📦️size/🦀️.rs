//! 📦 GLTF size indicators.

#[path = "↔️axis-aligned-bounds/🦀️.rs"]
pub mod axis_aligned_bounds;
#[path = "📐️bounding-box-dimensions/🦀️.rs"]
pub mod bounding_box_dimensions;
#[path = "🔗️characteristic-length/🦀️.rs"]
pub mod characteristic_length;
#[path = "🦶️footprint-area/🦀️.rs"]
pub mod footprint_area;
#[path = "🧭️oriented-bounds/🦀️.rs"]
pub mod oriented_bounds;
#[path = "📏️overall-size/🦀️.rs"]
pub mod overall_size;
#[path = "🎯️projected-area/🦀️.rs"]
pub mod projected_area;

use super::super::modules::measurement_contracts::*;
use super::geometry_core::GltfGeometryContext;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
        Self::Output {
            overall_size: overall_size::infer(context),
            axis_aligned_bounds: axis_aligned_bounds::infer(context),
            oriented_bounds: oriented_bounds::infer(context),
            bounding_box_dimensions: bounding_box_dimensions::infer(context),
            characteristic_length: characteristic_length::infer(context),
            footprint_area: footprint_area::infer(context),
            projected_area: projected_area::infer(context),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            overall_size: overall_size::unavailable_measure(diagnostic_ids),
            axis_aligned_bounds: axis_aligned_bounds::unavailable_measure(diagnostic_ids),
            oriented_bounds: oriented_bounds::unavailable_measure(diagnostic_ids),
            bounding_box_dimensions: bounding_box_dimensions::unavailable_measure(diagnostic_ids),
            characteristic_length: characteristic_length::unavailable_measure(diagnostic_ids),
            footprint_area: footprint_area::unavailable_measure(diagnostic_ids),
            projected_area: projected_area::unavailable_measure(diagnostic_ids),
        }
    }
}
