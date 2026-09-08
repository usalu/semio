
use super::*;
use crate::model::EntityId;

fn brick() -> Material {
    Material { id: EntityId(1), name: "Brick".into(), thickness_m: 0.1, conductivity_w_m_k: 0.72, density_kg_m3: 1920.0, specific_heat_j_kg_k: 840.0, thermal_absorptance: 0.9, solar_absorptance: 0.6, visible_absorptance: 0.6 }
}

fn insulation() -> Material {
    Material { id: EntityId(2), name: "EPS".into(), thickness_m: 0.14, conductivity_w_m_k: 0.035, density_kg_m3: 30.0, specific_heat_j_kg_k: 1400.0, thermal_absorptance: 0.9, solar_absorptance: 0.4, visible_absorptance: 0.4 }
}

#[test]
fn wall_u_value_reasonable() {
    let layers = vec![brick(), insulation()];
    let u = construction_u_value(&layers, R_FILM_INTERIOR_M2K_W, R_FILM_EXTERIOR_M2K_W);
    assert!(u > 0.15 && u < 0.35);
}

#[test]
fn resistance_adds_film_terms() {
    let layers = vec![insulation()];
    let r = effective_resistance(&layers, R_FILM_INTERIOR_M2K_W, R_FILM_EXTERIOR_M2K_W);
    assert!(r > layer_resistance_m2k_w(&insulation()));
}

#[test]
fn thermal_mass_positive() {
    let mass = construction_thermal_mass(&[brick(), insulation()]);
    assert!(mass > 10_000.0);
}
