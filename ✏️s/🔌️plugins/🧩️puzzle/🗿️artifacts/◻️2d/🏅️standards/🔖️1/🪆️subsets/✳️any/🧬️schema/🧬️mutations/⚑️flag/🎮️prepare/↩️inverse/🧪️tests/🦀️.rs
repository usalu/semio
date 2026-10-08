//! ⚑️ Native optional inverse owners conserve all three grant lanes and original absence.

use super::*;
use crate::standards::v1::subsets::any::schema::mutations::ChangeNodeRoot;
use semio_framework_value::retained_clone::RetainedCloneBorrowAuthority;

fn grant(turn: usize) -> RetainedCloneGrant { match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, 64), 1 => RetainedCloneGrant::one_payload_turn(4096, 64), _ => RetainedCloneGrant::one_release_turn(4096, 64) } }

fn observed(grant: RetainedCloneGrant, action: impl FnOnce() -> Result<RetainedCloneStep, ValueError>) -> RetainedCloneStep {
    let (result, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(action);
    let step = result.unwrap();
    assert!(!allocation.overflowed);
    assert!(step.progress().fits(grant));
    assert!(step.progress().copied_bytes + step.progress().retained_capacity_bytes + step.progress().released_bytes <= 4096);
    assert!(allocation.requested_bytes <= step.progress().retained_capacity_bytes, "owned inverse allocated unadmitted capacity: {allocation:?}, {step:?}");
    assert!(allocation.released_bytes <= step.progress().released_bytes, "owned inverse released unadmitted layout: {allocation:?}, {step:?}");
    step
}

fn close<T:Puzzle2dFlagIntent>(cursor: &mut Puzzle2dFlagInverseCursor<T>) {
    cursor.begin_close();
    for turn in 0..1_000_000 {
        if cursor.terminal_is_empty() { return; }
        let permit = grant(turn);
        observed(permit, || cursor.close_step(permit));
    }
    panic!("owned inverse did not retire every retained payload and source alias");
}

fn retire(inverse: PagedList<Puzzle2dMutation, {usize::MAX}>) {
    let (owner, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ControlledRetirement::new(inverse));
    assert_eq!(allocation.requested_bytes, 0);
    assert_eq!(allocation.released_bytes, 0);
    let mut owner = match owner { Ok(owner) => owner, Err((error, _)) => panic!("{error:?}") };
    for turn in 0..1_000_000 {
        if owner.terminal_is_empty() { return; }
        let permit = grant(turn);
        observed(permit, || owner.step(permit));
    }
    panic!("returned inverse did not retire its native paged owner");
}


fn law<T:Puzzle2dFlagIntent+Clone>(snapshot:&Puzzle2dSnapshot,payload:T,expected:&serde_json::Value,large:bool){
 assert!(size_of::<Puzzle2dFlagInverseCursor<T>>()<=4096);let original=serde_json::to_value(snapshot).unwrap();let source=RetainedCloneBorrowAuthority::new("flag inverse original snapshot");let mutation=RetainedCloneBorrowAuthority::new("flag inverse original typed payload");
 let(mut cursor,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dFlagInverseCursor::<T>::default);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!heap.overflowed);assert_eq!(observed(RetainedCloneGrant::default(),||cursor.advance(source.borrow(snapshot),mutation.borrow(&payload),RetainedCloneGrant::default())).progress(),RetainedCloneProgress::default());
 let mut complete=false;for turn in 0..1000000{let permit=grant(turn);if matches!(observed(permit,||cursor.advance(source.borrow(snapshot),mutation.borrow(&payload),permit)),RetainedCloneStep::Complete(_)){complete=true;break}}assert!(complete,"native flag inverse never completed");let output=cursor.take().unwrap();assert_eq!(serde_json::to_value(&output).unwrap(),*expected);assert!(cursor.take().is_none());close(&mut cursor);retire(output);
 for pause in [0,1,3,17,100]{let mut cursor=Puzzle2dFlagInverseCursor::<T>::default();for turn in 0..pause{let permit=grant(turn);if matches!(observed(permit,||cursor.advance(source.borrow(snapshot),mutation.borrow(&payload),permit)),RetainedCloneStep::Complete(_)){break}}close(&mut cursor);assert!(cursor.take().is_none());}
 if large{let mut cursor=Puzzle2dFlagInverseCursor::<T>::default();let mut copied=0;for turn in 0..1000000{let copying=cursor.phase==2;let permit=grant(turn);let step=observed(permit,||cursor.advance(source.borrow(snapshot),mutation.borrow(&payload),permit));assert!(!matches!(step,RetainedCloneStep::Complete(_)));if copying{copied+=step.progress().copied_bytes}if copied>=64{break}}assert!(copied>=64);close(&mut cursor);}
 let mut cursor=Puzzle2dFlagInverseCursor::<T>::default();observed(grant(1),||cursor.advance(source.borrow(snapshot),mutation.borrow(&payload),grant(1)));let swapped=payload.clone();assert!(cursor.advance(source.borrow(snapshot),mutation.borrow(&swapped),grant(1)).is_err());close(&mut cursor);assert_eq!(serde_json::to_value(snapshot).unwrap(),original);
}
#[test]
fn history_edit_puzzle2d_owned_optional_flag_inverse_preserves_first_native_owner_and_three_lane_heap_cancel(){
 use super::super::{ChangeNodeLocked,ChangeNodeVisible,ChangeEdgeLocked,ChangeEdgeVisible};
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let inverses:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let expand=|value:&str|match value.strip_prefix("$large:"){Some(suffix)=>format!("{}{suffix}",corpus["control"]["largePrefix"].as_str().unwrap().repeat(corpus["control"]["largeRepeats"].as_u64().unwrap()as usize)),None=>value.into()};let text=|value:&serde_json::Value|PagedUtf8::<{usize::MAX}>::from(expand(value.as_str().unwrap()));
 for row in corpus["cases"].as_array().unwrap(){let mut snapshot=Puzzle2dSnapshot::default();let kind=row["kind"].as_str().unwrap();let locked=kind.ends_with("locked");for item in row["rows"].as_array().unwrap(){let flag=item["value"].as_bool();if kind.starts_with("node"){let mut node=crate::Puzzle2dNode{id:text(&item["id"]),..Default::default()};if locked{node.locked=flag}else{node.visible=flag}snapshot.nodes.push(node)}else{let mut edge=crate::Puzzle2dEdge{id:text(&item["id"]),..Default::default()};if locked{edge.locked=flag}else{edge.visible=flag}snapshot.edges.push(edge)}}let mut expected=inverses["cases"].as_array().unwrap().iter().find(|value|value["id"]==row["id"]).unwrap()["inverse"].clone();for owner in expected.as_array_mut().unwrap(){owner["id"]=expand(owner["id"].as_str().unwrap()).into();}let id=text(&row["mutation"]["id"]);let value=row["mutation"]["value"].as_bool();let large=row["id"].as_str().unwrap().ends_with("large-prefix");match kind{"node-locked"=>law(&snapshot,ChangeNodeLocked{id,new_locked:value},&expected,large),"node-visible"=>law(&snapshot,ChangeNodeVisible{id,new_visible:value},&expected,large),"edge-locked"=>law(&snapshot,ChangeEdgeLocked{id,new_locked:value},&expected,large),"edge-visible"=>law(&snapshot,ChangeEdgeVisible{id,new_visible:value},&expected,large),_=>unreachable!()}}
 eprintln!("[DEBUG] forty typed original flag inverses conserve exact optional native state, paged output, every actual4096 allocation/release, fivepause+interiorID cancellation and source identity");
}

#[test]
fn history_edit_puzzle2d_owned_root_flag_inverse_preserves_literal_absence_and_three_lane_heap_cancel(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🌟️root/🧫️fixtures/🔣️.json")).unwrap();let expand=|value:&str|match value.strip_prefix("$large:"){Some(suffix)=>format!("{}{suffix}",corpus["control"]["largePrefix"].as_str().unwrap().repeat(corpus["control"]["largeRepeats"].as_u64().unwrap()as usize)),None=>value.to_owned()};
 for row in corpus["cases"].as_array().unwrap(){let mut snapshot=Puzzle2dSnapshot::default();for item in row["rows"].as_array().unwrap(){snapshot.nodes.push(crate::Puzzle2dNode{id:expand(item["id"].as_str().unwrap()).into(),root:item["value"].as_bool(),..Default::default()})}let mut expected=row["inverse"].clone();for item in expected.as_array_mut().unwrap(){item["id"]=expand(item["id"].as_str().unwrap()).into()}law(&snapshot,ChangeNodeRoot{id:expand(row["mutation"]["id"].as_str().unwrap()).into(),new_root:row["mutation"]["value"].as_bool()},&expected,row["id"].as_str().unwrap().ends_with("large-prefix"))}
 eprintln!("[DEBUG] ten original root flag inverse owners preserve literal optional state, exact4096 three-lane allocations/releases and five cancellation points");
}

#[test]
fn history_edit_puzzle2d_owned_required_region_flag_inverse_preserves_literal_bool_and_three_lane_heap_cancel(){
 use crate::standards::v1::subsets::any::schema::mutations::{ChangeTargetRegionHidden,ChangeTargetRegionLocked};
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🏁️region/🧫️fixtures/🔣️.json")).unwrap();let expand=|value:&str|match value.strip_prefix("$large:"){Some(suffix)=>format!("{}{suffix}",corpus["control"]["largePrefix"].as_str().unwrap().repeat(corpus["control"]["largeRepeats"].as_u64().unwrap()as usize)),None=>value.to_owned()};
 for row in corpus["cases"].as_array().unwrap(){let mut snapshot=Puzzle2dSnapshot::default();let hidden=row["kind"]=="region-hidden";for item in row["rows"].as_array().unwrap(){let mut region=crate::Puzzle2dTargetRegion{id:expand(item["id"].as_str().unwrap()).into(),..Default::default()};if hidden{region.hidden=item["value"].as_bool().unwrap()}else{region.locked=item["value"].as_bool().unwrap()}snapshot.target_regions.push(region)}let mut expected=row["inverse"].clone();for item in expected.as_array_mut().unwrap(){item["id"]=expand(item["id"].as_str().unwrap()).into()}let id=expand(row["mutation"]["id"].as_str().unwrap()).into();let value=row["mutation"]["value"].as_bool().unwrap();let large=row["id"].as_str().unwrap().ends_with("large-prefix");if hidden{law(&snapshot,ChangeTargetRegionHidden{id,new_hidden:value},&expected,large)}else{law(&snapshot,ChangeTargetRegionLocked{id,new_locked:value},&expected,large)}}
 eprintln!("[DEBUG] twenty literal required region inverses preserve true/false/no-op/missing and three-lane native heap cancellation");
}
