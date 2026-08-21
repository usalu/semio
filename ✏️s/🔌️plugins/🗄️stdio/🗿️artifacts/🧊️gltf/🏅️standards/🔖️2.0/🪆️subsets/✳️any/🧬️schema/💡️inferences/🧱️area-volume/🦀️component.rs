//! 🧱 GLTF area-volume indicators.

#[path = "contact-area/🦀️component.rs"]
pub mod contact_area;
#[path = "enclosed-volume/🦀️component.rs"]
pub mod enclosed_volume;
#[path = "exposed-area/🦀️component.rs"]
pub mod exposed_area;
#[path = "material-volume/🦀️component.rs"]
pub mod material_volume;
#[path = "surface-area/🦀️component.rs"]
pub mod surface_area;
#[path = "total-area/🦀️component.rs"]
pub mod total_area;
#[path = "void-volume/🦀️component.rs"]
pub mod void_volume;
#[path = "volume/🦀️component.rs"]
pub mod volume;

use super::super::modules::measurement_contracts::*;
use super::super::modules::mesh_topology::Topology;
use super::geometry_core::{GltfGeometryContext, GltfPairGeometry};
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn infer_pair_contact(pair: &GltfPairGeometry) -> GltfMeasure<f64> {
        contact_area::infer_pair(pair)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn infer_assembly(indicators: &mut GltfAreaVolumeIndicators, part_count: usize, contact_area: f64, contact_area_complete: bool, sample_count: usize, topology: Topology) {
        indicators.contact_area = contact_area::from_assembly(part_count, contact_area, contact_area_complete, sample_count, topology);
        indicators.exposed_area = exposed_area::from_assembly(&indicators.surface_area, part_count, contact_area, contact_area_complete, sample_count, topology);
    }
}

impl GltfInferenceStage<GltfGeometryContext<'_>> for GltfAreaVolumeInference {
    type Output = GltfAreaVolumeIndicators;

    fn infer(context: &GltfGeometryContext<'_>) -> Self::Output {
        Self::Output {
            surface_area: surface_area::infer(context),
            total_area: total_area::infer(context),
            exposed_area: exposed_area::infer(context),
            contact_area: contact_area::infer(context),
            volume: volume::infer(context),
            enclosed_volume: enclosed_volume::infer(context),
            material_volume: material_volume::infer(context),
            void_volume: void_volume::infer(context),
        }
    }

    fn unavailable(diagnostic_ids: &[String]) -> Self::Output {
        Self::Output {
            surface_area: surface_area::unavailable_measure(diagnostic_ids),
            total_area: total_area::unavailable_measure(diagnostic_ids),
            exposed_area: exposed_area::unavailable_measure(diagnostic_ids),
            contact_area: contact_area::unavailable_measure(diagnostic_ids),
            volume: volume::unavailable_measure(diagnostic_ids),
            enclosed_volume: enclosed_volume::unavailable_measure(diagnostic_ids),
            material_volume: material_volume::unavailable_measure(diagnostic_ids),
            void_volume: void_volume::unavailable_measure(diagnostic_ids),
        }
    }
}
