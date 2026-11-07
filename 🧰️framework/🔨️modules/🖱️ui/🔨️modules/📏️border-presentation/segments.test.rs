use super::*;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn shared_border_language_neutral_vectors() {
    let cases: Value = serde_json::from_str(include_str!("cases.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let inputs = case["input"].as_array().unwrap();
        let ids: BTreeMap<_, _> = inputs.iter().map(|s| s["id"].as_str().unwrap()).collect::<BTreeSet<_>>().into_iter().enumerate().map(|(i, id)| (id, i)).collect();
        let owners: BTreeMap<_, _> = inputs.iter().map(|s| s["owner"].as_str().unwrap()).collect::<BTreeSet<_>>().into_iter().enumerate().map(|(i, owner)| (owner, i)).collect();
        let point = |value: &Value| [value["x"].as_f64().unwrap(), value["y"].as_f64().unwrap()];
        let state = |value: &Value| match value.as_str().unwrap() {
            "normal" => SharedBorderState::Normal, "focus" => SharedBorderState::Focus,
            "active" => SharedBorderState::Active, "waiting" => SharedBorderState::Waiting,
            "loading" => SharedBorderState::Loading, "introduced" => SharedBorderState::Introduced,
            "celebrated" => SharedBorderState::Celebrated, "hover" => SharedBorderState::Hover,
            _ => panic!("Unknown fixture state"),
        };
        let segment = |value: &Value| SharedBorderSegment { id: ids[value["id"].as_str().unwrap()], owner: owners[value["owner"].as_str().unwrap()], from: point(&value["from"]), to: point(&value["to"]), state: state(&value["state"]) };
        let mut input: Vec<_> = inputs.iter().map(segment).collect();
        let expected: Vec<_> = case["expected"].as_array().unwrap().iter().map(|value| ResolvedSharedBorderSegment { segment: segment(value), contributors: value["contributors"].as_array().unwrap().iter().map(|id| ids[id.as_str().unwrap()]).collect() }).collect();
        assert_eq!(resolve_shared_border_segments(&input).unwrap(), expected, "{}", case["name"]);
        let mut cursor = SharedBorderResolutionCursor::new();
        for &edge in &input {
            let before = cursor.retained_capacity_bytes();
            let demand = cursor.next_push_allocation_byte_demand(&edge).unwrap();
            let progress = cursor.push_with_progress(edge).unwrap();
            assert_eq!(cursor.retained_capacity_bytes() - before, demand);
            assert_eq!(progress.retained_capacity_bytes, demand);
            assert!(progress.copied_bytes <= cursor.maximum_push_copy_byte_demand());
        }
        let mut incremental = Vec::new();
        loop {
            let before = cursor.retained_capacity_bytes();
            let demand = cursor.next_step_allocation_byte_demand().unwrap();
            let copied = cursor.next_step_copy_byte_demand();
            let (step, progress) = cursor.step_with_progress();
            let transfer = match &step { SharedBorderResolutionStep::Segment(edge) => edge.contributors.capacity() * std::mem::size_of::<usize>(), _ => 0 };
            assert_eq!(cursor.retained_capacity_bytes() + transfer - before, demand);
            assert_eq!(progress.retained_capacity_bytes, demand);
            assert_eq!(progress.copied_bytes, copied);
            match step {
                SharedBorderResolutionStep::Pending => {},
                SharedBorderResolutionStep::Segment(edge) => incremental.push(edge),
                SharedBorderResolutionStep::Complete => break,
            }
        }
        assert_eq!(incremental, expected, "{}", case["name"]);
        while !cursor.retire_step() {}
        assert_eq!(cursor.retained_capacity_bytes(), 0);
        input.reverse();
        assert_eq!(resolve_shared_border_segments(&input).unwrap(), expected, "{}", case["name"]);
    }
}

#[test]
fn shared_border_cancellation_retires_at_every_sweep_stage() {
    let input: Vec<_> = (0..24).map(|id| SharedBorderSegment { id, owner: id % 3, from: [id as f64 / 4.0, 0.0], to: [12.0, 0.0], state: SharedBorderState::Active }).collect();
    for stop in 0..180 {
        let mut cursor = SharedBorderResolutionCursor::new();
        for &edge in &input { cursor.push(edge).unwrap(); }
        for _ in 0..stop { cursor.step(); }
        let mut steps = 0;
        loop {
            let before = cursor.retained_capacity_bytes();
            let demand = cursor.next_retirement_release_byte_demand();
            let done = cursor.retire_step();
            assert_eq!(before - cursor.retained_capacity_bytes(), demand);
            if done { break; }
            steps += 1;
            assert!(steps < 300);
        }
        assert!(cursor.retire_step());
        assert_eq!(cursor.push(input[0]), Err(SharedBorderError::ResolutionStarted));
    }
}

#[test]
fn shared_border_rejects_invalid_input_and_ignores_points() {
    let segment = SharedBorderSegment { id: 0, owner: 0, from: [0.0, 0.0], to: [0.0, 0.0], state: SharedBorderState::Normal };
    assert!(resolve_shared_border_segments(&[segment]).unwrap().is_empty());
    assert_eq!(resolve_shared_border_segments(&[segment, segment]), Err(SharedBorderError::DuplicateId));
    assert_eq!(resolve_shared_border_segments(&[SharedBorderSegment { to: [1.0, 1.0], ..segment }]), Err(SharedBorderError::Diagonal));
    assert_eq!(resolve_shared_border_segments(&[SharedBorderSegment { to: [f64::INFINITY, 0.0], ..segment }]), Err(SharedBorderError::NonFinite));
}

#[test]
fn shared_border_ownership_summary_stays_within_fixed_worker_grants_for_many_incident_edges() {
    let grant = semio_framework_job::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65_536, maximum_capacity_bytes: 65_536, maximum_release_bytes: 262_144, maximum_depth: 64 };
    let mut cursor = SharedBorderOwnershipCursor::new();
    for id in 0..9000 {
        let segment = SharedBorderSegment { id, owner: id, from: [0.0, 0.0], to: [10.0, 0.0], state: SharedBorderState::Hover };
        assert!(cursor.next_push_allocation_byte_demand(&segment).unwrap() <= grant.maximum_capacity_bytes);
        assert!(cursor.push_with_progress(segment).unwrap().fits(grant));
    }
    let mut emitted = 0;
    loop {
        assert!(cursor.next_step_allocation_byte_demand().unwrap() <= grant.maximum_capacity_bytes);
        let (step, progress) = cursor.step_with_progress();
        assert!(progress.fits(grant));
        match step {
            SharedBorderOwnershipStep::Pending => {},
            SharedBorderOwnershipStep::Segment(edge) => { emitted += 1; assert_eq!(edge.segment.id, 0); assert_eq!(edge.last_contributor, 8999); },
            SharedBorderOwnershipStep::Complete => break,
        }
    }
    assert_eq!(emitted, 1);
    loop {
        assert!(cursor.next_retirement_release_byte_demand() <= grant.maximum_release_bytes);
        if cursor.retire_step() { break; }
    }
    assert_eq!(cursor.retained_capacity_bytes(), 0);
}
