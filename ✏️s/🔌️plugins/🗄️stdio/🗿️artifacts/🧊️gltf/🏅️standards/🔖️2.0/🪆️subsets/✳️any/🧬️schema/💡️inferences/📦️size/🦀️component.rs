//! 📦 GLTF size indicators.

#[path = "axis-aligned-bounds/🦀️component.rs"]
pub mod axis_aligned_bounds;
#[path = "bounding-box-dimensions/🦀️component.rs"]
pub mod bounding_box_dimensions;
#[path = "characteristic-length/🦀️component.rs"]
pub mod characteristic_length;
#[path = "footprint-area/🦀️component.rs"]
pub mod footprint_area;
#[path = "oriented-bounds/🦀️component.rs"]
pub mod oriented_bounds;
#[path = "overall-size/🦀️component.rs"]
pub mod overall_size;
#[path = "projected-area/🦀️component.rs"]
pub mod projected_area;

use super::super::modules::measurement_contracts::*;
use super::geometry_core::GltfGeometryContext;
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

    async fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output {
            overall_size: overall_size::infer(context).await,
            axis_aligned_bounds: axis_aligned_bounds::infer(context).await,
            oriented_bounds: oriented_bounds::infer(context).await,
            bounding_box_dimensions: bounding_box_dimensions::infer(context).await,
            characteristic_length: characteristic_length::infer(context).await,
            footprint_area: footprint_area::infer(context).await,
            projected_area: projected_area::infer(context).await,
        }
    }

    async fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            overall_size: overall_size::unavailable_measure(diagnostic_ids).await,
            axis_aligned_bounds: axis_aligned_bounds::unavailable_measure(diagnostic_ids).await,
            oriented_bounds: oriented_bounds::unavailable_measure(diagnostic_ids).await,
            bounding_box_dimensions: bounding_box_dimensions::unavailable_measure(diagnostic_ids).await,
            characteristic_length: characteristic_length::unavailable_measure(diagnostic_ids).await,
            footprint_area: footprint_area::unavailable_measure(diagnostic_ids).await,
            projected_area: projected_area::unavailable_measure(diagnostic_ids).await,
        }
    }
}
