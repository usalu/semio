//! 🛠️ Constructors of the energy mutation fixtures: the BESTEST 600 zone, snapshot, link, space, material, glazing, gas, construction, surface, window and shading builders. See [`super`].

use crate::snapshot::reference::EnergyModelSnapshot;

/// 🏠️ One conditioned zone with the BESTEST 600 envelope's own air volume.
pub fn zone(id: u32, name: &str) -> crate::model::Zone {
    crate::model::Zone { id: crate::model::EntityId(id), name: name.to_string(), volume_m3: 129.6, multiplier: 1, conditioned: true, part_of_total_floor_area: true }
}

/// 📸️ The persisted snapshot a typed model lives in, both composed child handles minted.
pub fn snapshot(model: crate::model::Model) -> EnergyModelSnapshot {
    crate::snapshot::energy_snapshot_with_state(crate::snapshot::ENERGY_MODEL_DOCUMENT_SCHEMA, &model, None)
}

/// 🔗️ A head-pinned forward link to another artifact.
pub fn link(uri: &str, role: &str) -> store::ArtifactLink {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    store::ArtifactLink { target: semio_framework_artifact_reference::ArtifactRef::parse_uri(uri).expect("fixture link uri parses"), pin: store::LinkPin::Head, role: role.to_string() }
}

//#region 🧰️G1Constructors
/// 🪑️ One space inside a zone.
pub fn space(id: u32, name: &str, zone_id: u32) -> crate::model::Space {
    crate::model::Space { id: crate::model::EntityId(id), name: name.to_string(), zone_id: crate::model::EntityId(zone_id), floor_area_m2: 48.0 }
}

/// 🧱️ One opaque material layer — the BESTEST 600 lightweight wall's plasterboard.
pub fn material(id: u32, name: &str) -> crate::model::Material {
    crate::model::Material {
        id: crate::model::EntityId(id),
        name: name.to_string(),
        roughness: crate::model::SurfaceRoughness::MediumRough,
        thickness_m: 0.012,
        conductivity_w_m_k: 0.16,
        density_kg_m3: 950.0,
        specific_heat_j_kg_k: 840.0,
        thermal_absorptance: 0.9,
        solar_absorptance: 0.6,
        visible_absorptance: 0.6,
    }
}

/// 🪟️ One uncoated 3 mm clear pane, the outer sheet of ANSI/ASHRAE 140 §5.2's double glazing.
pub fn glazing_material(id: u32, name: &str) -> crate::model::GlazingMaterial {
    crate::model::GlazingMaterial {
        id: crate::model::EntityId(id),
        name: name.to_string(),
        thickness_m: 0.003,
        conductivity_w_m_k: 1.06,
        solar_transmittance: 0.837,
        solar_reflectance_front: 0.075,
        solar_reflectance_back: 0.075,
        visible_transmittance: 0.898,
        visible_reflectance_front: 0.081,
        visible_reflectance_back: 0.081,
        infrared_transmittance: 0.0,
        infrared_emissivity_front: 0.84,
        infrared_emissivity_back: 0.84,
    }
}

/// 🌫️ One 13 mm air gap between two panes.
pub fn gas_material(id: u32, name: &str) -> crate::model::GasMaterial {
    crate::model::GasMaterial { id: crate::model::EntityId(id), name: name.to_string(), thickness_m: 0.013, gas: crate::model::GasKind::Air }
}

/// 🧱️ One single-layer construction over `material_id`.
pub fn construction(id: u32, name: &str, material_id: u32) -> crate::model::Construction {
    crate::model::Construction { id: crate::model::EntityId(id), name: name.to_string(), layer_material_ids: vec![crate::model::EntityId(material_id)] }
}

/// 📐️ One sun- and wind-exposed 8 m × 2.7 m exterior wall on `zone_id`.
pub fn surface(id: u32, name: &str, zone_id: u32, construction_id: u32) -> crate::model::Surface {
    crate::model::Surface {
        id: crate::model::EntityId(id),
        name: name.to_string(),
        zone_id: crate::model::EntityId(zone_id),
        class: crate::model::SurfaceClass::ExteriorWall,
        vertices_m: vec![[0.0, 0.0, 0.0], [8.0, 0.0, 0.0], [8.0, 0.0, 2.7], [0.0, 0.0, 2.7]],
        construction_id: crate::model::EntityId(construction_id),
        outside_boundary_condition: crate::model::OutsideBoundary::OutdoorAir,
        sun_exposed: true,
        wind_exposed: true,
        multiplier: 1,
    }
}

/// 🪟️ One 3 m × 2 m window carrying ANSI/ASHRAE 140 §5.2's own quoted optics.
pub fn window(id: u32, name: &str, surface_id: u32) -> crate::model::Fenestration {
    crate::model::Fenestration {
        id: crate::model::EntityId(id),
        name: name.to_string(),
        surface_id: crate::model::EntityId(surface_id),
        u_value_w_m2k: 3.0,
        shgc: 0.787,
        vlt: 0.86,
        area_m2: 6.0,
        height_m: 2.0,
        sill_height_m: 0.5,
        frame_conductance_w_k: 0.0,
        divider_conductance_w_k: 0.0,
        overhang_depth_m: 0.0,
        overhang_offset_m: 0.0,
        fin_depth_m: 0.0,
        fin_offset_m: 0.0,
        glazing_construction_id: None,
        vertices_m: Vec::new(),
    }
}

/// 🌳️ One free-standing site shading surface.
pub fn shading(id: u32, name: &str) -> crate::model::ShadingSurface {
    crate::model::ShadingSurface { id: crate::model::EntityId(id), name: name.to_string(), vertices_m: vec![[0.0, -2.0, 3.0], [8.0, -2.0, 3.0], [8.0, 0.0, 3.0], [0.0, 0.0, 3.0]], transmittance_schedule_id: None }
}
//#endregion 🧰️G1Constructors
