//! 🧱 GLTF area-volume indicators.

use super::geometric_analysis::{GltfGeometryContext, GltfPairGeometry};
use super::super::super::modules::{inference_measures::{estimate, exact, unavailable}, mesh_topology::Topology};
use super::super::super::modules::measurement_contracts::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfAreaVolumeIndicators {
    pub surface_area: GltfMeasure<f64>,
    pub total_area: GltfMeasure<f64>,
    pub exposed_area: GltfMeasure<f64>,
    pub contact_area: GltfMeasure<f64>,
    pub volume: GltfMeasure<f64>,
    pub enclosed_volume: GltfMeasure<f64>,
    pub material_volume: GltfMeasure<f64>,
    pub void_volume: GltfMeasure<f64>,
}

pub struct GltfAreaVolumeInference;

impl GltfAreaVolumeInference {
    pub(crate) fn infer_pair_contact(pair: &GltfPairGeometry) -> GltfMeasure<f64> {
        pair.contact_area
            .map(|area| if area == 0.0 { exact(area, GltfUnit::SquareMetre, pair.sample_count, None) } else { estimate(area, GltfUnit::SquareMetre, pair.sample_count, None) })
            .unwrap_or_else(|| unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), pair.sample_count, None))
    }

    pub(crate) fn infer_assembly(indicators: &mut GltfAreaVolumeIndicators, part_count: usize, contact_area: f64, contact_area_complete: bool, sample_count: usize, topology: Topology) {
        if part_count <= 1 {
            indicators.exposed_area = indicators.surface_area.clone();
            indicators.contact_area = exact(0.0, GltfUnit::SquareMetre, sample_count, Some(topology));
        } else if contact_area_complete {
            indicators.contact_area = estimate(contact_area, GltfUnit::SquareMetre, sample_count, Some(topology));
            indicators.exposed_area = estimate((indicators.surface_area.value.unwrap_or(0.0) - 2.0 * contact_area).max(0.0), GltfUnit::SquareMetre, sample_count, Some(topology));
        } else {
            indicators.contact_area = unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), sample_count, Some(topology));
            indicators.exposed_area = unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), sample_count, Some(topology));
        }
    }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfAreaVolumeInference {
    type Output = GltfAreaVolumeIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        let volume = context
            .solid
            .map(|metrics| exact(metrics.0, GltfUnit::CubicMetre, context.sample_count, Some(context.topology)))
            .unwrap_or_else(|| unavailable(GltfUnit::CubicMetre, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology)));
        let enclosed_volume = context
            .solid
            .map(|metrics| exact(metrics.1, GltfUnit::CubicMetre, context.sample_count, Some(context.topology)))
            .unwrap_or_else(|| unavailable(GltfUnit::CubicMetre, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology)));
        let void_volume = context
            .solid
            .map(|metrics| exact(metrics.2, GltfUnit::CubicMetre, context.sample_count, Some(context.topology)))
            .unwrap_or_else(|| unavailable(GltfUnit::CubicMetre, context.unavailable_volume, Vec::new(), context.sample_count, Some(context.topology)));
        Self::Output {
            surface_area: exact(context.surface_area, GltfUnit::SquareMetre, context.sample_count, Some(context.topology)),
            total_area: exact(context.surface_area, GltfUnit::SquareMetre, context.sample_count, Some(context.topology)),
            exposed_area: unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology)),
            contact_area: unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, Vec::new(), context.sample_count, Some(context.topology)),
            volume: volume.clone(),
            enclosed_volume,
            material_volume: volume,
            void_volume,
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        let area = || unavailable(GltfUnit::SquareMetre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None);
        let volume = || unavailable(GltfUnit::CubicMetre, GltfAvailability::Unavailable, diagnostic_ids.to_vec(), 0, None);
        Self::Output { surface_area: area(), total_area: area(), exposed_area: area(), contact_area: area(), volume: volume(), enclosed_volume: volume(), material_volume: volume(), void_volume: volume() }
    }
}
