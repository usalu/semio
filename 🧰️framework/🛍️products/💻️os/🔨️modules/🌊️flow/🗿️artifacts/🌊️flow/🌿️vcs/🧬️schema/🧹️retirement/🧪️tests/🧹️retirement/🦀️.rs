//! 🧪️ Actual bounded retirement laws for every Flow direct mutation leaf.

use super::*;
use crate::os_spr::Identified;
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
use crate::retained::{FlowOwner,FlowRetirement};
use super::super::{AddSynapse, AddWidget, ChangeLayout, ChangeSynapse, ChangeWidget, FlowHostSnapshot, FlowLayoutEntry, FlowMutation, MoveSynapse, MoveWidget, RemoveSynapse, RemoveWidget};

//#region 🧭️Fixtures
fn fixture() -> FlowHostSnapshot {
    let vectors = semio_framework_pack_json::parse(include_str!("../../../🔺️diff/🧫️fixtures/🧾️ownership/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("actual retained ownership vectors");
    semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(vectors.get("base").expect("base fixture"))).expect("actual retained Flow fixture")
}

fn mutations() -> Vec<FlowMutation> {
    let fixture = fixture();
    let widget = fixture.widgets.first().expect("widget").clone();
    let synapse = fixture.synapses.first().expect("synapse").clone();
    let widget_id = widget.id().to_string();
    let synapse_id = synapse.id.clone();
    let entry = FlowLayoutEntry { id: widget_id.clone(), layout: fixture.layout.get(&widget_id).cloned() };
    vec![
        FlowMutation::AddWidget(AddWidget { index: 0, widget: widget.clone() }),
        FlowMutation::RemoveWidget(RemoveWidget { id: widget_id.clone() }),
        FlowMutation::MoveWidget(MoveWidget { id: widget_id.clone(), to_index: 0 }),
        FlowMutation::ChangeWidget(ChangeWidget { id: widget_id, widget }),
        FlowMutation::AddSynapse(AddSynapse { index: 0, synapse: synapse.clone() }),
        FlowMutation::RemoveSynapse(RemoveSynapse { id: synapse_id.clone() }),
        FlowMutation::MoveSynapse(MoveSynapse { id: synapse_id.clone(), to_index: 0 }),
        FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse_id, synapse }),
        FlowMutation::ChangeLayout(ChangeLayout { entries: vec![entry] }),
    ]
}
//#endregion 🧭️Fixtures

//#region 🧪️Laws
#[test]
fn retained_fixture_has_dictionary_and_set() {
    let fixture = fixture();
    assert!(matches!(fixture.widgets.first(), Some(crate::Widget::Neuron { params, .. }) if !params.is_empty()));
    assert!(matches!(fixture.widgets.get(1), Some(crate::Widget::OutputPreview { expanded, .. }) if !expanded.is_empty()));
    fixture.retire_cold();
}

#[test]
fn direct_leaf_retirement_requires_independent_grants_and_reaches_terminal_empty() {
    for (index,mutation) in mutations().into_iter().enumerate() {
        let mut retirement=FlowRetirement::from_owner(FlowOwner::Mutation(mutation));
        let mut total=RetainedCloneProgress::default();
        for turn in 0..200_000 {
            if retirement.terminal_is_empty(){break;}
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:1,maximum_capacity_bytes:retirement.next_capacity_byte_demand(1).unwrap(),maximum_release_bytes:retirement.next_release_byte_demand().unwrap(),maximum_depth:retirement.next_depth_demand().unwrap()};
            assert_eq!(retirement.step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap().progress(),RetainedCloneProgress::default());
            if grant.maximum_capacity_bytes!=0 {assert_eq!(retirement.step(RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}).unwrap().progress(),RetainedCloneProgress::default());}
            if grant.maximum_release_bytes!=0 {assert_eq!(retirement.step(RetainedCloneGrant {maximum_release_bytes:grant.maximum_release_bytes-1,..grant}).unwrap().progress(),RetainedCloneProgress::default());}
            let step=retirement.step(grant).unwrap();let progress=step.progress();assert!(progress.fits(grant));assert_ne!(progress.copied_items,0,"mutation {index} stalled at turn {turn}");total=total.checked_add(progress).unwrap();
        }
        assert!(retirement.terminal_is_empty());
        println!("[DEBUG] Flow typed mutation retirement variant={index} copied={} born={} released={}",total.copied_bytes,total.retained_capacity_bytes,total.released_bytes);
    }
}
