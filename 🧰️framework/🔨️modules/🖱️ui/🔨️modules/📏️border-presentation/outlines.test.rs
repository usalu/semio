use super::states_tests::state_of;
use super::*;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

const SIDES: [&str; 4] = ["top", "right", "bottom", "left"];

fn point(value: &Value) -> [f64; 2] { [value["x"].as_f64().unwrap(), value["y"].as_f64().unwrap()] }

fn parse_boxes(case: &Value) -> (Vec<BoxOutlineInput>, BTreeMap<String, usize>, BTreeMap<String, usize>) {
    let boxes = case["boxes"].as_array().unwrap();
    let ids: BTreeMap<String, usize> = boxes.iter().map(|b| b["id"].as_str().unwrap().to_string()).collect::<BTreeSet<_>>().into_iter().enumerate().map(|(i, id)| (id, i)).collect();
    let owners: BTreeMap<String, usize> = boxes.iter().map(|b| b["owner"].as_str().unwrap().to_string()).collect::<BTreeSet<_>>().into_iter().enumerate().map(|(i, id)| (id, i)).collect();
    let input = boxes.iter().map(|b| {
        let rect = &b["rect"];
        let edges = b["edges"].as_array().unwrap();
        BoxOutlineInput {
            id: ids[b["id"].as_str().unwrap()],
            parent: b["parent"].as_str().map(|id| ids[id]),
            owner: owners[b["owner"].as_str().unwrap()],
            rect: ["left", "top", "right", "bottom"].map(|key| rect[key].as_f64().unwrap()),
            edges: SIDES.map(|side| edges.iter().any(|edge| edge == side)),
            state: state_of(&b["state"]),
        }
    }).collect();
    (input, ids, owners)
}

fn segment_id(ids: &BTreeMap<String, usize>, name: &str) -> usize {
    let (box_id, side) = name.split_once(':').unwrap();
    box_segment_id(ids[box_id], SIDES.iter().position(|candidate| *candidate == side).unwrap())
}

#[test]
fn box_outline_language_neutral_vectors() {
    let cases: Value = serde_json::from_str(include_str!("outlines.cases.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let (input, ids, owners) = parse_boxes(case);
        let expected: Vec<_> = case["expected"].as_array().unwrap().iter().map(|value| ResolvedSharedBorderSegment {
            segment: SharedBorderSegment { id: segment_id(&ids, value["id"].as_str().unwrap()), owner: owners[value["owner"].as_str().unwrap()], from: point(&value["from"]), to: point(&value["to"]), state: state_of(&value["state"]) },
            contributors: value["contributors"].as_array().unwrap().iter().map(|id| segment_id(&ids, id.as_str().unwrap())).collect(),
        }).collect();
        assert_eq!(box_outline_segments(&input).unwrap(), expected, "{name}");
        let mut reversed = input.clone();
        reversed.reverse();
        assert_eq!(box_outline_segments(&reversed).unwrap(), expected, "{name}: input order");
    }
}

fn covered(resolved: &[ResolvedSharedBorderSegment], from: [f64; 2], to: [f64; 2], at_least: SharedBorderState) -> bool {
    let vertical = from[0] == to[0];
    let (axis, fixed) = if vertical { (1, 0) } else { (0, 1) };
    let (lo, hi) = (from[axis].min(to[axis]), from[axis].max(to[axis]));
    let mut cut: Vec<f64> = vec![lo, hi];
    for edge in resolved.iter().map(|r| r.segment).filter(|s| s.from[fixed] == from[fixed] && (s.from[0] == s.to[0]) == vertical) {
        cut.extend([edge.from[axis], edge.to[axis]].into_iter().filter(|v| *v > lo && *v < hi));
    }
    cut.sort_by(f64::total_cmp);
    cut.windows(2).filter(|pair| pair[1] > pair[0]).all(|pair| {
        let middle = (pair[0] + pair[1]) / 2.0;
        resolved.iter().map(|r| r.segment).any(|s| s.from[fixed] == from[fixed] && (s.from[0] == s.to[0]) == vertical && s.from[axis].min(s.to[axis]) < middle && s.from[axis].max(s.to[axis]) > middle && s.state >= at_least)
    })
}

#[test]
fn box_outline_closure_invariant_holds_for_every_box_whose_state_closes_it() {
    let cases: Value = serde_json::from_str(include_str!("outlines.cases.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let (input, _, _) = parse_boxes(case);
        let resolved = box_outline_segments(&input).unwrap();
        for entry in input.iter().filter(|entry| border_state_closes(entry.state)) {
            let [left, top, right, bottom] = entry.rect;
            for (from, to) in [([left, top], [right, top]), ([right, top], [right, bottom]), ([left, bottom], [right, bottom]), ([left, top], [left, bottom])] {
                assert!(covered(&resolved, from, to, entry.state), "{}: box {} perimeter {:?}->{:?}", case["name"], entry.id, from, to);
            }
        }
    }
}

#[test]
fn box_outline_hover_adds_no_line_to_the_outline_without_the_hover() {
    let geometry = |boxes: &[BoxOutlineInput]| {
        let mut lines: BTreeMap<(bool, u64), Vec<(f64, f64)>> = BTreeMap::new();
        for segment in box_outline_segments(boxes).unwrap().iter().map(|r| r.segment) {
            let axis = usize::from(segment.from[0] == segment.to[0]);
            lines.entry((axis == 1, segment.from[1 - axis].to_bits())).or_default().push((segment.from[axis].min(segment.to[axis]), segment.from[axis].max(segment.to[axis])));
        }
        lines.into_iter().map(|(line, mut intervals)| {
            intervals.sort_by(|a, b| a.0.total_cmp(&b.0));
            let mut merged: Vec<(f64, f64)> = Vec::new();
            for (from, to) in intervals { match merged.last_mut() { Some(last) if from <= last.1 => last.1 = last.1.max(to), _ => merged.push((from, to)) } }
            (line, merged.into_iter().map(|(from, to)| (from.to_bits(), to.to_bits())).collect::<Vec<_>>())
        }).collect::<Vec<_>>()
    };
    let cases: Value = serde_json::from_str(include_str!("outlines.cases.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let (input, _, _) = parse_boxes(case);
        for hovered in input.iter().filter(|entry| entry.state == SharedBorderState::Hover) {
            let rest: Vec<_> = input.iter().map(|entry| if entry.id == hovered.id { BoxOutlineInput { state: SharedBorderState::Normal, ..*entry } } else { *entry }).collect();
            assert_eq!(geometry(&input), geometry(&rest), "{}: box {}", case["name"], hovered.id);
        }
    }
}

#[test]
fn box_outline_oracle_row_of_cells_emphasises_each_hovered_cell() {
    for count in 1..6usize {
        for hovered in 0..count {
            let mut boxes = vec![BoxOutlineInput { id: count, parent: None, owner: count, rect: [0.0, 0.0, 10.0 * count as f64, 10.0], edges: [true; 4], state: SharedBorderState::Normal }];
            boxes.extend((0..count).map(|cell| BoxOutlineInput { id: cell, parent: Some(count), owner: count, rect: [10.0 * cell as f64, 0.0, 10.0 * cell as f64 + 10.0, 10.0], edges: [true; 4], state: if cell == hovered { SharedBorderState::Hover } else { SharedBorderState::Normal } }));
            let resolved = box_outline_segments(&boxes).unwrap();
            let painted: f64 = resolved.iter().filter(|r| r.segment.state == SharedBorderState::Hover).map(|r| (r.segment.to[0] - r.segment.from[0]).abs() + (r.segment.to[1] - r.segment.from[1]).abs()).sum();
            assert_eq!(painted, 40.0, "cells {count}, hovered {hovered}");
            assert!(resolved.iter().filter(|r| r.segment.state == SharedBorderState::Normal).all(|r| r.segment.from != r.segment.to));
        }
    }
}

#[test]
fn box_outline_rejects_unknown_parent_and_reports_demand() {
    let orphan = BoxOutlineInput { id: 1, parent: Some(2), owner: 1, rect: [0.0, 0.0, 1.0, 1.0], edges: [true; 4], state: SharedBorderState::Normal };
    assert_eq!(box_outline_segments(&[orphan]), Err(BoxOutlineError::UnknownParent));
    let root = BoxOutlineInput { parent: None, ..orphan };
    let sources = box_outline_source_segments(&[root]).unwrap();
    assert_eq!(sources.len() * std::mem::size_of::<SharedBorderSegment>(), box_outline_allocation_byte_demand(1).unwrap());
    assert_eq!(sources.capacity(), sources.len());
}

#[test]
fn borderless_box_draws_nothing_at_rest_or_on_hover_and_a_closed_outline_when_focused_or_active() {
    let row = BoxOutlineInput { id: 0, parent: None, owner: 0, rect: [0.0, 0.0, 30.0, 10.0], edges: [false; 4], state: SharedBorderState::Normal };
    let tab = |state| BoxOutlineInput { id: 1, parent: Some(0), owner: 0, rect: [10.0, 0.0, 20.0, 10.0], edges: [false; 4], state };
    for state in [SharedBorderState::Normal, SharedBorderState::Hover] { assert_eq!(box_outline_segments(&[row, tab(state)]).unwrap(), vec![]); }
    for state in [SharedBorderState::Focus, SharedBorderState::Active] {
        let resolved = box_outline_segments(&[row, tab(state)]).unwrap();
        assert_eq!(resolved.len(), 4);
        assert!(resolved.iter().all(|r| r.segment.state == state && r.contributors == vec![r.segment.id]));
    }
}

#[test]
fn box_outline_focused_cell_keeps_its_perimeter_beside_a_selected_cell() {
    let cell = |id: usize, left: f64, state| BoxOutlineInput { id, parent: Some(2), owner: 2, rect: [left, 0.0, left + 10.0, 10.0], edges: [true; 4], state };
    for (focus, selected) in [(0, 1), (1, 0)] {
        let mut cells = [cell(0, 0.0, SharedBorderState::Normal), cell(1, 10.0, SharedBorderState::Normal)];
        cells[focus].state = SharedBorderState::Focus;
        cells[selected].state = SharedBorderState::Active;
        let group = BoxOutlineInput { id: 2, parent: None, owner: 2, rect: [0.0, 0.0, 20.0, 10.0], edges: [true; 4], state: SharedBorderState::Normal };
        let resolved = box_outline_segments(&[cells[0], cells[1], group]).unwrap();
        let [left, _, right, _] = cells[focus].rect;
        let focused: f64 = resolved.iter().filter(|r| r.segment.state == SharedBorderState::Focus).map(|r| (r.segment.to[0] - r.segment.from[0]).abs() + (r.segment.to[1] - r.segment.from[1]).abs()).sum();
        assert_eq!(focused, 40.0, "focus {focus}");
        assert!(resolved.iter().any(|r| r.segment.state == SharedBorderState::Focus && r.segment.from[0] == left && r.segment.to[0] == left || r.segment.from[0] == right));
    }
}
