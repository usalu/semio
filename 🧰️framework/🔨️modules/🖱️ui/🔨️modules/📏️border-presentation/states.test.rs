use super::*;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn state_of(value: &Value) -> SharedBorderState {
    match value.as_str().unwrap() {
        "normal" => SharedBorderState::Normal, "focus" => SharedBorderState::Focus,
        "active" => SharedBorderState::Active, "invalid" => SharedBorderState::Invalid, "waiting" => SharedBorderState::Waiting,
        "loading" => SharedBorderState::Loading, "introduced" => SharedBorderState::Introduced,
        "celebrated" => SharedBorderState::Celebrated, "hover" => SharedBorderState::Hover,
        other => panic!("Unknown fixture state {other}"),
    }
}

#[test]
fn box_state_language_neutral_vectors() {
    let cases: Value = serde_json::from_str(include_str!("states.cases.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let ids: BTreeMap<_, _> = case["boxes"].as_array().unwrap().iter().map(|b| b["id"].as_str().unwrap()).collect::<BTreeSet<_>>().into_iter().enumerate().map(|(i, id)| (id, id.parse::<u64>().unwrap_or(i as u64))).collect();
        let boxes: Vec<_> = case["boxes"].as_array().unwrap().iter().map(|b| BoxStateInput {
            id: ids[b["id"].as_str().unwrap()],
            parent: b["parent"].as_str().map(|id| ids[id]),
            interactive: b["interactive"].as_bool().unwrap(),
            disabled: b["disabled"].as_bool().unwrap(),
            marks: b["marks"].as_array().unwrap().iter().fold(BoxStateMarks::NONE, |marks, mark| marks.with(state_of(mark))),
        }).collect();
        let resolved = resolve_box_states(&boxes, case["pointer"].as_str().map(|id| ids[id]), case["focusVisible"].as_str().map(|id| ids[id])).unwrap();
        let actual: BTreeMap<_, _> = boxes.iter().zip(&resolved).map(|(b, s)| (b.id, *s)).collect();
        for (id, expected) in case["expected"].as_object().unwrap() {
            assert_eq!(actual[&ids[id.as_str()]], state_of(expected), "{name}: {id}");
        }
        assert_eq!(case["expected"].as_object().unwrap().len(), boxes.len(), "{name}");
    }
}

#[test]
fn box_state_oracle_matches_a_walk_of_the_path() {
    let boxes: Vec<_> = (0..6).map(|id| BoxStateInput { id, parent: id.checked_sub(1), interactive: id % 2 == 1, disabled: id == 3, marks: BoxStateMarks::NONE }).collect();
    for pointer in 0..6 {
        let resolved = resolve_box_states(&boxes, Some(pointer), Some(pointer)).unwrap();
        let hovered = (0..=pointer).rev().find(|&id| boxes[id].interactive && !boxes[id].disabled);
        let focused = (0..=pointer).rev().find(|&id| boxes[id].interactive);
        for id in 0..6 {
            assert_eq!(resolved[id], own_box_state(BoxStateMarks::NONE, hovered == Some(id), focused == Some(id)));
            assert!(resolved[id] == SharedBorderState::Normal || id <= pointer);
        }
    }
}

#[test]
fn box_state_rejects_invalid_trees() {
    let at = |id, parent| BoxStateInput { id, parent, interactive: true, disabled: false, marks: BoxStateMarks::NONE };
    assert_eq!(resolve_box_states(&[at(1, None), at(1, None)], None, None), Err(BoxStateError::DuplicateId));
    assert_eq!(resolve_box_states(&[BoxStateInput { interactive: false, ..at(1, Some(9)) }], Some(1), None), Err(BoxStateError::UnknownBox));
    assert_eq!(resolve_box_states(&[at(1, None)], Some(7), None), Err(BoxStateError::UnknownBox));
    assert_eq!(resolve_box_states(&[BoxStateInput { interactive: false, ..at(1, Some(2)) }, BoxStateInput { interactive: false, ..at(2, Some(1)) }], Some(1), None), Err(BoxStateError::Cycle));
    assert_eq!(box_state_allocation_byte_demand::<u64>(usize::MAX), Err(BoxStateError::CapacityOverflow));
}
