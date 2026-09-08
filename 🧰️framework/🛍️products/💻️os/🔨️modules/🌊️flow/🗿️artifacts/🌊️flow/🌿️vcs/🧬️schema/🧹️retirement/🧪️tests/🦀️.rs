//! 🧪️ Actual bounded retirement laws for every Flow direct mutation leaf.

use super::*;
use crate::os_spr::Identified;
use crate::os_store::SnapshotRetirementStep;
use super::super::{AddSynapse, AddWidget, ChangeLayout, ChangeSynapse, ChangeWidget, FlowFixture, FlowLayoutEntry, FlowMutation, MoveSynapse, MoveWidget, RemoveSynapse, RemoveWidget, ReplaceFlowFixture};

//#region 🧭️Fixtures
fn fixture() -> FlowFixture {
    let vectors = crate::os_pack::json::parse(include_str!("../../../🧬️schema/🔺️diff/🧪️tests/🧾️ownership/🔣️.json")).expect("actual retained ownership vectors");
    crate::os_dsl::FromValue::from_value(crate::os_pack::json::to_dsl_value(vectors.get("base").expect("base fixture"))).expect("actual retained Flow fixture")
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
        FlowMutation::ChangeWidget(ChangeWidget { id: widget_id.clone(), widget }),
        FlowMutation::AddSynapse(AddSynapse { index: 0, synapse: synapse.clone() }),
        FlowMutation::RemoveSynapse(RemoveSynapse { id: synapse_id.clone() }),
        FlowMutation::MoveSynapse(MoveSynapse { id: synapse_id.clone(), to_index: 0 }),
        FlowMutation::ChangeSynapse(ChangeSynapse { id: synapse_id, synapse }),
        FlowMutation::ChangeLayout(ChangeLayout { entries: vec![entry] }),
        FlowMutation::ReplaceFlowFixture(ReplaceFlowFixture { fixture }),
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
fn direct_leaf_retirement_refuses_zero_grants_then_reaches_terminal_empty() {
    for mutation in mutations() {
        let mut retirement = FlowMutationRetirementFrontier::new(mutation);
        assert!(matches!(retirement.close_step(0, 64).expect("zero item grant"), SnapshotRetirementStep::Blocked));
        assert!(matches!(retirement.close_step(1, 0).expect("zero byte grant"), SnapshotRetirementStep::Blocked));
        assert!(!retirement.terminal_is_empty());
        assert!(matches!(retirement.close_step(1, 64).expect("handoff"), SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
        assert!(matches!(retirement.close_step(0, 64).expect("inner owner zero item grant"), SnapshotRetirementStep::Blocked));
        assert!(matches!(retirement.close_step(1, 0).expect("inner owner zero byte grant"), SnapshotRetirementStep::Blocked));
        assert!(!retirement.terminal_is_empty());
        for _ in 0..4096 {
            let step = retirement.close_step(1, 64).expect("bounded retained close");
            match step {
                SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= 64);
                }
                SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    break;
                }
                SnapshotRetirementStep::Blocked => panic!("positive grant must advance retirement"),
            }
        }
        assert!(retirement.terminal_is_empty());
    }
}

#[test]
fn injected_inner_fault_preserves_transferred_payload() {
    let mut retirement = FlowMutationRetirementFrontier::new(FlowMutation::RemoveWidget(RemoveWidget { id: "cancelled".into() }));
    assert!(matches!(retirement.close_step(1, 64).expect("handoff to actual frontier"), SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }));
    let injected_fault = retirement.close_step_with_injected(1, 64, |_frontier, _items, _bytes| Err("injected nested retirement fault".into()));
    assert!(injected_fault.is_err());
    assert!(!retirement.terminal_is_empty());
    for _ in 0..4096 {
        if matches!(retirement.close_step(1, 64).expect("bounded injected-model close"), SnapshotRetirementStep::Complete) {
            break;
        }
    }
    assert!(retirement.terminal_is_empty());
}

#[test]
fn false_inner_completion_keeps_retained_payload_owned() {
    let mut retirement = FlowMutationRetirementFrontier::new(FlowMutation::RemoveWidget(RemoveWidget { id: "owned".into() }));
    assert!(matches!(retirement.close_step(1, 1).unwrap(), SnapshotRetirementStep::Pending { .. }));
    let result = retirement.close_step_with_injected(1, 1, |_, _, _| Ok(SnapshotRetirementStep::Complete));
    assert_eq!(result.unwrap_err(), "flow mutation retirement frontier reported Complete before terminal-empty");
    assert!(!retirement.terminal_is_empty());
    for _ in 0..32 {
        if matches!(retirement.close_step(1, 1).unwrap(), SnapshotRetirementStep::Complete) { break; }
    }
    assert!(retirement.terminal_is_empty());
}
//#endregion 🧪️Laws
