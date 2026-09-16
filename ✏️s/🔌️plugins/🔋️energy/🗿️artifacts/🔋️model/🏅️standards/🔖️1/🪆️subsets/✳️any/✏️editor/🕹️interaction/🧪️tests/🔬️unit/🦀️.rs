use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_every_granularity_once() {
    let definition = energy_model_interaction_definition();
    assert_eq!(definition.id, ENERGY_MODEL_INTERACTION_DOMAIN);
    let mut ids: Vec<&str> = definition.granularities.iter().map(|granularity| granularity.id.as_str()).collect();
    let count = ids.len();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), count, "a granularity is declared twice");
    for expected in [ENERGY_GRANULARITY_ZONE, ENERGY_GRANULARITY_SURFACE, ENERGY_GRANULARITY_FENESTRATION, ENERGY_GRANULARITY_SHADING, ENERGY_GRANULARITY_MATERIAL, ENERGY_GRANULARITY_CONSTRUCTION] {
        assert!(ids.contains(&expected), "{expected} is not a granularity");
    }
}

#[semio_framework_async_macros::async_test]
async fn entity_kind_resolves_the_bestest_families_and_refuses_strangers() {
    let snapshot = crate::energy_snapshot_with_state(crate::ENERGY_MODEL_DOCUMENT_SCHEMA, &crate::examples::bestest_600::model(), None);
    let model = &snapshot.model;
    let zone = energy_target_id(model.zones[0].id);
    let surface = energy_target_id(model.surfaces[0].id);
    let window = energy_target_id(model.fenestrations[0].id);
    let material = energy_target_id(model.materials[0].id);
    let construction = energy_target_id(model.constructions[0].id);
    assert_eq!(energy_entity_kind(&snapshot, &zone), Some(ENERGY_GRANULARITY_ZONE));
    assert_eq!(energy_entity_kind(&snapshot, &surface), Some(ENERGY_GRANULARITY_SURFACE));
    assert_eq!(energy_entity_kind(&snapshot, &window), Some(ENERGY_GRANULARITY_FENESTRATION));
    assert_eq!(energy_entity_kind(&snapshot, &material), Some(ENERGY_GRANULARITY_MATERIAL));
    assert_eq!(energy_entity_kind(&snapshot, &construction), Some(ENERGY_GRANULARITY_CONSTRUCTION));
    assert_eq!(energy_entity_kind(&snapshot, "999999"), None);
    assert_eq!(energy_entity_kind(&snapshot, "not-an-id"), None);
}
