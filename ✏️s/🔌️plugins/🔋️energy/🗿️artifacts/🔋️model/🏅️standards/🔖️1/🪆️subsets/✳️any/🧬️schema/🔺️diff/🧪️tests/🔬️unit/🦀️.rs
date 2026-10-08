use super::*;
use crate::model::{EntityId, Zone};
use semio_framework_value::{FromValue, ToValue};

fn zone(id: u32, name: &str) -> Zone {
    Zone { id: EntityId(id), name: name.to_string(), volume_m3: 100.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true }
}

fn snapshot(zones: Vec<Zone>) -> EnergyModelSnapshot {
    crate::energy_snapshot_with_state(crate::ENERGY_MODEL_DOCUMENT_SCHEMA, &crate::model::Model { zones, ..crate::model::Model::default() }, None)
}

fn zones_of(zones: Rows<ZonePatch>) -> EnergyModelDiff {
    EnergyModelDiff::of(ModelPatch { zones, ..ModelPatch::default() })
}

fn rename(id: u32, name: &str) -> EnergyModelDiff {
    zones_of(Rows::modifying(ZonePatch { name: Some(name.to_string()), ..ZonePatch::of(EntityId(id)) }))
}

#[semio_framework_async_macros::async_test]
async fn empty_diff_is_a_no_operation() {
    let base = crate::schema::empty_energy_model_snapshot();
    let diff = EnergyModelDiff::default();
    assert!(DiffAlgebra::is_empty(&diff));
    assert_eq!(protocol::apply_diff(&diff, &base).expect("valid mutation diff"), base);
}

#[semio_framework_async_macros::async_test]
async fn keyed_rows_apply_in_place_and_invert_exactly() {
    let base = snapshot(vec![zone(1, "A"), zone(2, "B"), zone(3, "C")]);
    let mut diff = rename(2, "Bee");
    diff.absorb(zones_of(Rows::inserting(1, zone(9, "Nine"))));
    diff.absorb(zones_of(Rows::removing(&base.model.zones, &EntityId(3))));
    let after = protocol::apply_diff(&diff, &base).expect("the diff applies");
    assert_eq!(after.model.zones.iter().map(|zone| (zone.id.0, zone.name.as_str())).collect::<Vec<_>>(), [(1, "A"), (9, "Nine"), (2, "Bee")].to_vec());
    assert_eq!(protocol::apply_diff(&diff.inverse(&base), &after).expect("the inverse applies"), base);
}

#[semio_framework_async_macros::async_test]
async fn the_negative_diff_reads_the_base_row_by_row_and_restores_it() {
    let base = snapshot(vec![zone(1, "A"), zone(2, "B"), zone(3, "C")]);
    let mut diff = rename(1, "A2");
    diff.absorb(zones_of(Rows::inserting(3, zone(7, "G"))));
    diff.absorb(zones_of(Rows::removing(&base.model.zones, &EntityId(2))));
    protocol::os_spr::protocol_laws::assert_diff_algebra_inverse_law(&base, &diff).await;
}

#[semio_framework_async_macros::async_test]
async fn an_inserted_row_removed_again_cancels_and_a_later_patch_folds_into_an_earlier_insert() {
    let base = snapshot(vec![zone(1, "A")]);
    let mut cancelled = zones_of(Rows::inserting(1, zone(5, "Five")));
    cancelled.absorb(zones_of(Rows::<ZonePatch>::removing_where(&[zone(1, "A"), zone(5, "Five")], |row| row.id == EntityId(5))));
    assert!(DiffAlgebra::is_empty(&cancelled), "{cancelled:?}");
    let mut folded = zones_of(Rows::inserting(1, zone(5, "Five")));
    folded.absorb(rename(5, "Cinq"));
    assert_eq!(folded, zones_of(Rows::inserting(1, zone(5, "Cinq"))));
    assert_eq!(protocol::apply_diff(&folded, &base).expect("applies").model.zones[1].name, "Cinq");
}

#[semio_framework_async_macros::async_test]
async fn patches_of_one_row_coalesce_and_a_removed_row_drops_its_patches() {
    let mut twice = rename(1, "First");
    twice.absorb(zones_of(Rows::modifying(ZonePatch { multiplier: Some(4), ..ZonePatch::of(EntityId(1)) })));
    twice.absorb(rename(1, "Second"));
    assert_eq!(twice, zones_of(Rows::modifying(ZonePatch { name: Some("Second".into()), multiplier: Some(4), ..ZonePatch::of(EntityId(1)) })));
    let base = snapshot(vec![zone(1, "A"), zone(2, "B")]);
    twice.absorb(zones_of(Rows::removing(&base.model.zones, &EntityId(1))));
    assert_eq!(twice, zones_of(Rows::removing(&base.model.zones, &EntityId(1))));
}

#[semio_framework_async_macros::async_test]
async fn a_patch_naming_a_missing_row_or_a_wrong_removal_is_refused() {
    let base = snapshot(vec![zone(1, "A")]);
    assert!(protocol::apply_diff(&rename(7, "Ghost"), &base).is_err());
    let wrong = zones_of(Rows::<ZonePatch>::removing_where(&[zone(2, "B")], |_| true));
    assert!(protocol::apply_diff(&wrong, &base).is_err());
}

#[semio_framework_async_macros::async_test]
async fn list_edits_and_slots_compose_to_their_net_effect() {
    let base: Vec<u32> = vec![10, 20, 30];
    let mut edit = ListEdit::removing_at(1, 20);
    edit.absorb(ListEdit::inserting(1, 20));
    assert!(edit.unchanged(), "removing and re-inserting the same value at the same place is nothing");
    let mut moved = ListEdit::moving(&base, 0, 2);
    let mut after = base.clone();
    moved.commit_onto(&mut after).expect("moves");
    assert_eq!(after, [20, 30, 10]);
    moved.absorb(moved.inverse(&base));
    assert!(moved.unchanged());
    let mut slots = Slots::<f64, 3>::assigning(1, 5.0);
    slots.absorb(Slots::assigning(1, 6.0));
    slots.absorb(Slots::assigning(0, 1.0));
    let mut array = [0.0; 3];
    slots.commit_onto(&mut array).expect("assigns");
    assert_eq!(array, [1.0, 6.0, 0.0]);
    assert!(Slots::<f64, 3>::assigning(3, 1.0).commit_onto(&mut array).is_err());
}

#[semio_framework_async_macros::async_test]
async fn the_wire_form_round_trips_and_omits_what_did_not_change() {
    let base = snapshot(vec![zone(1, "A"), zone(2, "B")]);
    let mut diff = rename(1, "Uno");
    diff.absorb(zones_of(Rows::inserting(2, zone(3, "Tres"))));
    diff.absorb(zones_of(Rows::removing(&base.model.zones, &EntityId(2))));
    diff.absorb(EnergyModelDiff::weather(EnergyLinkSlotDelta::Detached));
    let value = diff.to_value();
    assert_eq!(EnergyModelDiff::from_value(value.clone()).expect("decodes"), diff);
    assert!(value["model"]["spaces"].is_null() && value["model"]["name"].is_null(), "only the touched collections appear: {value:?}");
    assert_eq!(EnergyModelDiff::default().to_value(), semio_framework_value::DslValue::Object(Vec::new()));
}
