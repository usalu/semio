use super::*;
use crate::diff_patch::{ListEdit, Rows, Slots};
use crate::model::{EntityId, Zone};
use semio_framework_value::{FromValue, ToValue};

fn zone(id: u32, name: &str) -> Zone {
    Zone { id: EntityId(id), name: name.to_string(), volume_m3: 100.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true }
}

fn snapshot(zones: Vec<Zone>) -> EnergyModelSnapshot {
    crate::snapshot::energy_snapshot_with_state(crate::snapshot::ENERGY_MODEL_DOCUMENT_SCHEMA, &crate::model::Model { zones, ..crate::model::Model::default() }, None)
}

fn zones_of(zones: Rows<ZonePatch>) -> EnergyModelDiff {
    EnergyModelDiff::of(ModelPatch { zones, ..ModelPatch::default() })
}

fn rename(id: u32, name: &str) -> EnergyModelDiff {
    zones_of(Rows::modifying(ZonePatch { name: Some(name.to_string()), ..ZonePatch::of(EntityId(id)) }))
}

#[semio_framework_async_macros::async_test]
async fn empty_diff_is_a_no_operation() {
    let base = crate::snapshot::EnergyModelSnapshot::default();
    let diff = EnergyModelDiff::default();
    assert!(DiffAlgebra::is_empty(&diff));
    assert_eq!(protocol::apply_diff_cold(&diff, &base).expect("valid mutation diff"), base);
}

#[semio_framework_async_macros::async_test]
async fn keyed_rows_apply_in_place_and_invert_exactly() {
    let base = snapshot(vec![zone(1, "A"), zone(2, "B"), zone(3, "C")]);
    let mut diff = rename(2, "Bee");
    diff.absorb(zones_of(Rows::inserting(1, zone(9, "Nine"))));
    let intermediate = [zone(1, "A"), zone(9, "Nine"), zone(2, "Bee"), zone(3, "C")];
    diff.absorb(zones_of(Rows::removing(&intermediate, &EntityId(3))));
    let after = protocol::apply_diff_cold(&diff, &base).expect("the diff applies");
    assert_eq!(after.model.zones.iter().map(|zone| (zone.id.0, zone.name.as_str())).collect::<Vec<_>>(), [(1, "A"), (9, "Nine"), (2, "Bee")].to_vec());
    assert_eq!(protocol::apply_diff_cold(&diff.inverse(&base), &after).expect("the inverse applies"), base);
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
    assert_eq!(protocol::apply_diff_cold(&folded, &base).expect("applies").model.zones[1].name, "Cinq");
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
    assert!(protocol::apply_diff_cold(&rename(7, "Ghost"), &base).is_err());
    let wrong = zones_of(Rows::<ZonePatch>::removing_where(&[zone(2, "B")], |_| true));
    assert!(protocol::apply_diff_cold(&wrong, &base).is_err());
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

#[test]
fn link_and_scalar_edits_preserve_untouched_projection_storage() {
    let mut base = snapshot(vec![zone(1, "A"), zone(2, "B")]);
    let zones = base.model.zones.as_ptr();
    let schema = base.schema.as_ptr();
    protocol::apply_diff_cold_into(&EnergyModelDiff::weather(EnergyLinkSlotDelta::Detached), &mut base).expect("link change");
    assert_eq!(base.model.zones.as_ptr(), zones);
    assert_eq!(base.schema.as_ptr(), schema);
    protocol::apply_diff_cold_into(&EnergyModelDiff::of(ModelPatch { name: Some("Changed".into()), ..Default::default() }), &mut base).expect("name change");
    assert_eq!(base.model.name, "Changed");
    assert_eq!(base.model.zones.as_ptr(), zones);
    assert_eq!(base.schema.as_ptr(), schema);
    eprintln!("[DEBUG] Energy link/scalar edits preserve unrelated model and snapshot storage");
}

#[test]
fn a_late_model_field_refusal_preserves_every_original_field_and_link() {
    let mut base = snapshot(vec![zone(1, "A")]);
    let original = base.clone();
    let zones = base.model.zones.as_ptr();
    let diff = EnergyModelDiff { model: ModelPatch { name: Some("Rejected".into()), ground_temperature: GroundTemperatureConfigPatch { building_surface_c: Slots::assigning(12, 7.0), ..Default::default() }, ..Default::default() }, weather_link: Some(EnergyLinkSlotDelta::Detached), ..Default::default() };
    assert!(protocol::apply_diff_cold_into(&diff, &mut base).is_err());
    assert_eq!(base, original);
    assert_eq!(base.model.zones.as_ptr(), zones);
    eprintln!("[DEBUG] Energy late field refusal leaves the original projection and links unchanged");
}

#[test]
fn duplicate_inserted_entity_keys_are_refused_atomically() {
    let mut base = snapshot(vec![zone(1, "A"), zone(2, "B")]);
    let original = base.clone();
    let zones = base.model.zones.as_ptr();
    let mut diff = rename(2, "Rejected");
    diff.absorb(zones_of(Rows::inserting(2, zone(1, "Duplicate"))));
    assert!(protocol::apply_diff_cold_into(&diff, &mut base).is_err());
    assert_eq!(base, original);
    let mut repeated = zones_of(Rows::inserting(2, zone(3, "C")));
    repeated.absorb(zones_of(Rows::inserting(3, zone(3, "Duplicate"))));
    assert!(protocol::apply_diff_cold_into(&repeated, &mut base).is_err());
    assert_eq!(base, original);
    assert_eq!(base.model.zones.as_ptr(), zones);
}

#[test]
fn replacing_an_entity_with_the_same_key_still_inverts_exactly() {
    let base = snapshot(vec![zone(1, "A"), zone(2, "B")]);
    let mut diff = zones_of(Rows::removing(&base.model.zones, &EntityId(1)));
    diff.absorb(zones_of(Rows::inserting(1, zone(1, "Replacement"))));
    let after = protocol::apply_diff_cold(&diff, &base).expect("replacement keeps keys unique");
    assert_eq!(after.model.zones, [zone(2, "B"), zone(1, "Replacement")]);
    assert_eq!(protocol::apply_diff_cold(&diff.inverse(&base), &after).expect("replacement inverse"), base);
}

#[test]
fn language_neutral_row_identity_cases_apply_and_invert() {
    let cases: Vec<semio_framework_value::DslValue> = semio_framework_pack_json::from_json_str(include_str!("identity.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("identity fixtures");
    for case in cases {
        let name = String::from_value(case["name"].clone()).expect("case name");
        let ids = Vec::<u32>::from_value(case["base"].clone()).expect("base keys");
        let removed = Vec::<u32>::from_value(case["removed"].clone()).expect("removed keys");
        let inserted = Vec::<Vec<u32>>::from_value(case["inserted"].clone()).expect("insertions");
        let accepted = bool::from_value(case["accepted"].clone()).expect("expected outcome");
        let base = snapshot(ids.iter().map(|id| zone(*id, &format!("Zone {id}"))).collect());
        let mut rows = Rows::<ZonePatch>::removing_where(&base.model.zones, |row| removed.contains(&row.id.0));
        for insertion in inserted {
            rows.absorb(Rows::inserting(insertion[0] as usize, zone(insertion[1], &format!("Inserted {}", insertion[1]))));
        }
        let diff = zones_of(rows);
        let mut after = base.clone();
        let outcome = protocol::apply_diff_cold_into(&diff, &mut after);
        assert_eq!(outcome.is_ok(), accepted, "{name}");
        if accepted {
            let expected = Vec::<u32>::from_value(case["expected"].clone()).expect("expected keys");
            assert_eq!(after.model.zones.iter().map(|row| row.id.0).collect::<Vec<_>>(), expected, "{name}");
            assert_eq!(protocol::apply_diff_cold(&diff.inverse(&base), &after).expect("inverse applies"), base, "{name}");
        } else {
            assert_eq!(after, base, "{name}");
        }
    }
}

#[test]
fn decoded_unordered_array_slots_respect_later_assignments() {
    let case: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(include_str!("slots.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("slot fixture");
    let mut decoded = Slots::<f64, 3>::from_value(case["earlier"].clone()).expect("decodes slots");
    decoded.absorb(Slots::from_value(case["later"].clone()).expect("later slots"));
    let mut values = [0.0; 3];
    decoded.commit_onto(&mut values).expect("valid slots");
    assert_eq!(values.to_vec(), Vec::<f64>::from_value(case["expected"].clone()).expect("expected slots"));
    assert_eq!(decoded.0.iter().map(|slot| slot.index).collect::<Vec<_>>(), [0, 2]);
}
