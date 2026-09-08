
use super::*;
use crate::model::EntityId;
use crate::model::{Model, Site, Zone};

#[test]
fn sizes_zone_with_surfaces() {
    let model = Model {
        name: "Test".into(),
        site: Site { latitude_deg: 45.0, longitude_deg: 0.0, elevation_m: 100.0, time_zone_hours: 0.0, north_axis_deg: 0.0 },
        zones: vec![Zone { id: EntityId(1), name: "Z1".into(), volume_m3: 200.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true }],
        ..Default::default()
    };
    let tables = SizingManager::size(&model, &SizingConfig::default());
    assert!(!tables.zone_loads.is_empty());
}

#[test]
fn sizes_equipment_for_ideal_loads_zone() {
    let model = crate::sim::test_model_single_zone();
    let tables = SizingManager::size(&model, &SizingConfig::default());
    assert_eq!(tables.equipment.len(), 1);
    assert!(tables.equipment[0].design_load_w > 0.0);
}

#[test]
fn coincident_peak_sums_all_loads() {
    assert!((SizingManager::coincident_peak(&[1000.0, 2000.0, 500.0]) - 3500.0).abs() < 1e-9);
    assert_eq!(SizingManager::coincident_peak(&[]), 0.0);
}

#[test]
fn non_coincident_peak_takes_maximum() {
    assert!((SizingManager::non_coincident_peak(&[1000.0, 2000.0, 500.0]) - 2000.0).abs() < 1e-9);
    assert_eq!(SizingManager::non_coincident_peak(&[]), 0.0);
}
