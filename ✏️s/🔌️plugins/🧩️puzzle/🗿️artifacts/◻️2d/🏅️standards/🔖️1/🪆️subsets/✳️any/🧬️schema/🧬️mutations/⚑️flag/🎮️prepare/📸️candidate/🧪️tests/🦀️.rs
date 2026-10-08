//! 🧪️ Native candidate ownership preserves complete snapshots and bounded cancellation.

use super::*;
use semio_framework_value::paged::PagedUtf8;
use crate::standards::v1::subsets::any::schema::mutations::ChangeNodeRoot;
use crate::test_source_custody;

fn grant(turn: usize) -> RetainedCloneGrant { match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, 128), 1 => RetainedCloneGrant::one_payload_turn(4096, 128), _ => RetainedCloneGrant::one_release_turn(4096, 128) } }

fn observed(permit: RetainedCloneGrant, action: impl FnOnce() -> Result<RetainedCloneStep, ValueError>) -> RetainedCloneStep {
    let (result, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(action);
    let step = result.unwrap();
    assert!(!allocation.overflowed && step.progress().fits(permit));
    assert!(step.progress().copied_bytes + step.progress().retained_capacity_bytes + step.progress().released_bytes <= 4096);
    assert!(allocation.requested_bytes <= step.progress().retained_capacity_bytes, "candidate allocated unadmitted capacity: {allocation:?}, {step:?}");
    assert!(allocation.released_bytes <= step.progress().released_bytes, "candidate released unadmitted layout: {allocation:?}, {step:?}");
    step
}

fn close<T:Puzzle2dFlagIntent>(cursor: &mut Puzzle2dFlagCandidateCursor<T>) {
    cursor.begin_close();
    test_source_custody::close_cursor(cursor,1_000_000,|cursor|{
        let copy=cursor.next_close_copy_byte_demand().unwrap();
        (copy,cursor.next_close_capacity_byte_demand(copy).unwrap(),cursor.next_close_release_byte_demand().unwrap(),cursor.next_close_depth_demand().unwrap())
    },|cursor,grant|cursor.close_step(grant),|cursor|cursor.terminal_is_empty());
}

fn retire(snapshot: Puzzle2dSnapshot) {
    let (result, allocation) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| ControlledRetirement::new(snapshot));
    assert_eq!(allocation.requested_bytes, 0);
    assert_eq!(allocation.released_bytes, 0);
    let mut owner = match result { Ok(owner) => owner, Err((error, _)) => panic!("{error:?}") };
    for turn in 0..1_000_000 { if owner.terminal_is_empty() { return; } let permit = grant(turn); observed(permit, || owner.step(permit)); }
    panic!("returned native snapshot did not retire every physical page");
}


fn law<T:Puzzle2dFlagIntent+Clone>(snapshot:&Puzzle2dSnapshot,payload:T,row:&serde_json::Value,expected:Option<&Puzzle2dSnapshot>){
 assert!(size_of::<Puzzle2dFlagCandidateCursor<T>>()<=4096);let original=serde_json::to_value(snapshot).unwrap();let mut source=test_source_custody::admit();let mut mutation=test_source_custody::admit();let(mut cursor,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dFlagCandidateCursor::<T>::default);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!heap.overflowed);assert_eq!(observed(RetainedCloneGrant::default(),||cursor.advance(source.borrow(snapshot),mutation.borrow(&payload),RetainedCloneGrant::default())).progress(),RetainedCloneProgress::default());
 let mut completed=false;for turn in 0..1000000{let permit=grant(turn);if matches!(observed(permit,||cursor.advance(source.borrow(snapshot),mutation.borrow(&payload),permit)),RetainedCloneStep::Complete(_)){completed=true;break}}assert!(completed);let output=cursor.take().unwrap();let status=match output.plan.disposition{Puzzle2dFlagDisposition::Changed=>"changed",Puzzle2dFlagDisposition::NoOp=>"no-op",Puzzle2dFlagDisposition::TargetMissing=>"target-missing"};assert_eq!(status,row["status"].as_str().unwrap());assert_eq!(serde_json::to_value(output.plan.index).unwrap(),row["index"]);assert_eq!(output.snapshot.as_ref().map(|value|serde_json::to_value(value).unwrap()),expected.map(|value|serde_json::to_value(value).unwrap()));assert!(cursor.take().is_none());close(&mut cursor);if let Some(snapshot)=output.snapshot{retire(snapshot)}
 for pause in [0,1,3,17,100]{let mut cursor=Puzzle2dFlagCandidateCursor::<T>::default();for turn in 0..pause{let permit=grant(turn);if matches!(observed(permit,||cursor.advance(source.borrow(snapshot),mutation.borrow(&payload),permit)),RetainedCloneStep::Complete(_)){break}}close(&mut cursor);assert!(cursor.take().is_none());}
 if expected.is_some(){let mut cursor=Puzzle2dFlagCandidateCursor::<T>::default();let mut reached=false;for turn in 0..1000000{let copying=cursor.phase==2;let permit=grant(turn);let step=observed(permit,||cursor.advance(source.borrow(snapshot),mutation.borrow(&payload),permit));assert!(!matches!(step,RetainedCloneStep::Complete(_)));if copying&&step.progress().copied_bytes>0{reached=true;break}}assert!(reached);close(&mut cursor);}
 let mut cursor=Puzzle2dFlagCandidateCursor::<T>::default();observed(grant(1),||cursor.advance(source.borrow(snapshot),mutation.borrow(&payload),grant(1)));let swapped=payload.clone();assert!(cursor.advance(source.borrow(snapshot),mutation.borrow(&swapped),grant(1)).is_err());close(&mut cursor);assert_eq!(serde_json::to_value(snapshot).unwrap(),original);test_source_custody::close(&mut source);test_source_custody::close(&mut mutation);
}
#[test]
fn history_edit_puzzle2d_native_optional_flag_candidate_preserves_full_native_owners_and_three_lane_cancel(){
 use super::super::{ChangeNodeLocked,ChangeNodeVisible,ChangeEdgeLocked,ChangeEdgeVisible};
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let candidates:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let full:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json")).unwrap();let text=|value:&serde_json::Value|{let value=value.as_str().unwrap();match value.strip_prefix("$large:"){Some(suffix)=>semio_framework_value::paged::PagedUtf8::<{usize::MAX}>::from(format!("{}{suffix}",corpus["control"]["largePrefix"].as_str().unwrap().repeat(corpus["control"]["largeRepeats"].as_u64().unwrap()as usize))),None=>value.into()}};
 for row in corpus["cases"].as_array().unwrap(){let mut snapshot:Puzzle2dSnapshot=serde_json::from_value(full["snapshot"].clone()).unwrap();let kind=row["kind"].as_str().unwrap();let locked=kind.ends_with("locked");if kind.starts_with("node"){snapshot.nodes.clear()}else{snapshot.edges.clear()}for item in row["rows"].as_array().unwrap(){let flag=item["value"].as_bool();if kind.starts_with("node"){let mut node=crate::Puzzle2dNode{id:text(&item["id"]),x:-0.0,text:Some("untouched\0😀".repeat(1300).into()),..Default::default()};if locked{node.locked=flag}else{node.visible=flag}snapshot.nodes.push(node)}else{let mut edge=crate::Puzzle2dEdge{id:text(&item["id"]),gap:-0.0,..Default::default()};if locked{edge.locked=flag}else{edge.visible=flag}snapshot.edges.push(edge)}}let after=&candidates["cases"].as_array().unwrap().iter().find(|value|value["id"]==row["id"]).unwrap()["after"];let mut expected=after.as_array().map(|_|snapshot.clone());if let(Some(after),Some(expected))=(after.as_array(),expected.as_mut()){for(index,item)in after.iter().enumerate(){let flag=item["value"].as_bool();if kind.starts_with("node"){let native=expected.nodes.get_mut(index).unwrap();assert_eq!(native.id,text(&item["id"]));if locked{native.locked=flag}else{native.visible=flag}}else{let native=expected.edges.get_mut(index).unwrap();assert_eq!(native.id,text(&item["id"]));if locked{native.locked=flag}else{native.visible=flag}}}}let id=text(&row["mutation"]["id"]);let value=row["mutation"]["value"].as_bool();match kind{"node-locked"=>law(&snapshot,ChangeNodeLocked{id,new_locked:value},row,expected.as_ref()),"node-visible"=>law(&snapshot,ChangeNodeVisible{id,new_visible:value},row,expected.as_ref()),"edge-locked"=>law(&snapshot,ChangeEdgeLocked{id,new_locked:value},row,expected.as_ref()),"edge-visible"=>law(&snapshot,ChangeEdgeVisible{id,new_visible:value},row,expected.as_ref()),_=>unreachable!()}}
 eprintln!("[DEBUG] forty actual native optional flag candidates retain full snapshot/catalog/ordinal owners, partial richclone and fivecancellation pauses with actual4096 three-lane heap receipts");
}

#[test]
fn history_edit_puzzle2d_native_root_flag_candidate_preserves_full_original_snapshot_and_three_lane_cancel(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🌟️root/🧫️fixtures/🔣️.json")).unwrap();let full:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json")).unwrap();let expand=|value:&str|match value.strip_prefix("$large:"){Some(suffix)=>format!("{}{suffix}",corpus["control"]["largePrefix"].as_str().unwrap().repeat(corpus["control"]["largeRepeats"].as_u64().unwrap()as usize)),None=>value.to_owned()};
 for row in corpus["cases"].as_array().unwrap(){let mut snapshot:Puzzle2dSnapshot=serde_json::from_value(full["snapshot"].clone()).unwrap();snapshot.nodes.clear();for item in row["rows"].as_array().unwrap(){snapshot.nodes.push(crate::Puzzle2dNode{id:expand(item["id"].as_str().unwrap()).into(),root:item["value"].as_bool(),x:-0.0,text:Some("untouched\0😀".repeat(1300).into()),..Default::default()})}let mut expected=row["after"].as_array().map(|_|snapshot.clone());if let(Some(after),Some(expected))=(row["after"].as_array(),expected.as_mut()){for(index,item)in after.iter().enumerate(){let node=expected.nodes.get_mut(index).unwrap();assert_eq!(node.id,PagedUtf8::<{usize::MAX}>::from(expand(item["id"].as_str().unwrap())));node.root=item["value"].as_bool()}}law(&snapshot,ChangeNodeRoot{id:expand(row["mutation"]["id"].as_str().unwrap()).into(),new_root:row["mutation"]["value"].as_bool()},row,expected.as_ref())}
 eprintln!("[DEBUG] ten real root flag candidates preserve every untouched full native snapshot owner and exact4096 three-lane heap/cancel conservation");
}

#[test]
fn history_edit_puzzle2d_native_required_region_flag_candidate_preserves_full_owner_and_three_lane_cancel(){
 use crate::standards::v1::subsets::any::schema::mutations::{ChangeTargetRegionHidden,ChangeTargetRegionLocked};
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🏁️region/🧫️fixtures/🔣️.json")).unwrap();let full:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json")).unwrap();let expand=|value:&str|match value.strip_prefix("$large:"){Some(suffix)=>format!("{}{suffix}",corpus["control"]["largePrefix"].as_str().unwrap().repeat(corpus["control"]["largeRepeats"].as_u64().unwrap()as usize)),None=>value.to_owned()};
 for row in corpus["cases"].as_array().unwrap(){let mut snapshot:Puzzle2dSnapshot=serde_json::from_value(full["snapshot"].clone()).unwrap();snapshot.target_regions.clear();let hidden=row["kind"]=="region-hidden";for item in row["rows"].as_array().unwrap(){let mut region=crate::Puzzle2dTargetRegion{id:expand(item["id"].as_str().unwrap()).into(),x:-0.0,label:Some("untouched\0😀".repeat(1300).into()),..Default::default()};if hidden{region.hidden=item["value"].as_bool().unwrap()}else{region.locked=item["value"].as_bool().unwrap()}snapshot.target_regions.push(region)}let mut expected=row["after"].as_array().map(|_|snapshot.clone());if let(Some(after),Some(expected))=(row["after"].as_array(),expected.as_mut()){for(index,item)in after.iter().enumerate(){let region=expected.target_regions.get_mut(index).unwrap();assert_eq!(region.id,PagedUtf8::<{usize::MAX}>::from(expand(item["id"].as_str().unwrap())));if hidden{region.hidden=item["value"].as_bool().unwrap()}else{region.locked=item["value"].as_bool().unwrap()}}}let id=expand(row["mutation"]["id"].as_str().unwrap()).into();let value=row["mutation"]["value"].as_bool().unwrap();if hidden{law(&snapshot,ChangeTargetRegionHidden{id,new_hidden:value},row,expected.as_ref())}else{law(&snapshot,ChangeTargetRegionLocked{id,new_locked:value},row,expected.as_ref())}}
 eprintln!("[DEBUG] twenty required region candidates retain untouched full native snapshot owners, exact4096 three-lane heap and cancellation");
}
