//! 🧪️ Native cascade inverse heap, exact semantics, cancellation, and source authority laws.

use super::*;
use crate::apply_puzzle2d_mutation;
use crate::test_source_custody;
use crate::standards::v1::subsets::any::schema::{empty_puzzle2d_snapshot,mutations::{delete_node,inverse_puzzle2d_mutation}};

fn grant(turn:usize)->RetainedCloneGrant {match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096,64), 1 => RetainedCloneGrant::one_payload_turn(4096,64), _ => RetainedCloneGrant::one_release_turn(4096,64) }}
fn observed(grant:RetainedCloneGrant,action:impl FnOnce()->Result<RetainedCloneStep,ValueError>)->RetainedCloneStep {
    let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(action);
    let step=result.unwrap();
    assert!(!heap.overflowed);assert!(step.progress().fits(grant));
    assert!(step.progress().copied_bytes+step.progress().retained_capacity_bytes<=4096);
    assert!(heap.requested_bytes<=step.progress().retained_capacity_bytes,"delete inverse unadmitted allocation: {heap:?}, {step:?}");
    assert!(heap.released_bytes<=step.progress().copied_bytes,"delete inverse unadmitted release: {heap:?}, {step:?}");
    step
}
fn close(cursor:&mut Puzzle2dDeleteNodeInverseCursor) {
    cursor.begin_close();
    test_source_custody::close_cursor(cursor,2_000_000,|cursor|{
        let copy=cursor.next_close_copy_byte_demand().unwrap();
        (copy,cursor.next_close_capacity_byte_demand(copy).unwrap(),cursor.next_close_release_byte_demand().unwrap(),cursor.next_close_depth_demand().unwrap())
    },|cursor,grant|cursor.close_step(grant),|cursor|cursor.terminal_is_empty());
}
fn retire(inverse:PagedList<Puzzle2dMutation,{usize::MAX}>) {
    let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ControlledRetirement::new(inverse));
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let mut owner=match result{Ok(owner)=>owner,Err((error,_))=>panic!("{error:?}")};
    for turn in 0..2_000_000 {
        if owner.terminal_is_empty(){return;}
        let permit=grant(turn);observed(permit,||owner.step(permit));
    }
    panic!("delete inverse output retained native ownership");
}

#[test]
fn history_edit_puzzle2d_owned_delete_inverse_preserves_exact_cascade_and_granted_cancellation() {
    assert!(size_of::<Puzzle2dDeleteNodeInverseCursor>()<=4096);
    assert!(size_of::<<Puzzle2dEdge as RetainedClone>::Cursor>()<=4096);
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let text=|value:&serde_json::Value|{let value=value.as_str().unwrap();match value.strip_prefix("$large:"){Some(suffix)=>PagedUtf8::<{usize::MAX}>::from(format!("{}{suffix}",corpus["control"]["largePrefix"].as_str().unwrap().repeat(corpus["control"]["largeRepeats"].as_u64().unwrap() as usize))),None=>value.into()}};
    let mut source=test_source_custody::admit();
    let mut mutation=test_source_custody::admit();
    for case in corpus["cases"].as_array().unwrap() {
        let mut base=empty_puzzle2d_snapshot();
        for (index,node) in case["nodes"].as_array().unwrap().iter().enumerate() {
            base.nodes.push(Puzzle2dNode {id:text(&node["id"]),node_kind:Some("kind".into()),shape:Some("rectangle".into()),x:index as f64,y:-(index as f64),width:Some(5.0),height:Some(7.0),text:Some("😀\u{0}".repeat(1300).into()),handles:node["handles"].as_array().unwrap().iter().map(|id|crate::Puzzle2dHandle {id:text(id),handle_kind:Some("literal".into()),angle:3.0,color:Some("😀\u{0}é".into()),visible:Some(false),locked:Some(true),..Default::default()}).collect(),..Default::default()});
        }
        for (index,edge) in case["edges"].as_array().unwrap().iter().enumerate() {
            base.edges.push(Puzzle2dEdge {id:text(&edge["id"]),source:text(&edge["source"]),target:text(&edge["target"]),edge_kind:Some("".into()),gap:-3.0,shift:2.0,rise:1.0,rotation:4.0,turn:5.0,tilt:-6.0,x:7.0,y:-8.0,source_tip:Some("".into()),target_tip:Some("é".into()),visible:if index%3==0{Some(false)}else if index%3==1{Some(true)}else{None},locked:if index%3==0{Some(true)}else if index%3==1{Some(false)}else{None}});
        }
        let payload=DeleteNode {id:text(&case["target"])};
        let original=serde_json::to_value(&base).unwrap();let original_payload=serde_json::to_value(&payload).unwrap();
        let expected=inverse_puzzle2d_mutation(&base,&delete_node(payload.id.clone())).unwrap();
        let (mut cursor,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dDeleteNodeInverseCursor::default);
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        assert_eq!(observed(RetainedCloneGrant::default(),||cursor.advance(source.borrow(&base),mutation.borrow(&payload),RetainedCloneGrant::default())).progress(),Default::default());
        let mut complete=false;
        for turn in 0..2_000_000 {
            let permit=grant(turn);
            if matches!(observed(permit,||cursor.advance(source.borrow(&base),mutation.borrow(&payload),permit)),RetainedCloneStep::Complete(_)){complete=true;break;}
        }
        assert!(complete,"deletion inverse did not finish {}",case["id"]);
        let inverse=cursor.take().unwrap();assert!(cursor.take().is_none());
        assert_eq!(inverse.iter().collect::<Vec<_>>(),expected.iter().collect::<Vec<_>>(),"exact native semantic mutation assembly {}",case["id"]);
        close(&mut cursor);
        if case["id"]!="first-duplicate" && case["node"].as_u64().is_some() {
            let mut after=base.clone();apply_puzzle2d_mutation(&mut after,&delete_node(payload.id.clone())).unwrap();
            for step in inverse.iter().rev() {apply_puzzle2d_mutation(&mut after,step).unwrap();}
            assert_eq!(after,base,"all complete native owners restored {}",case["id"]);
        }
        retire(inverse);
        for pause in [0,1,3,11,97,257,1021] {
            let mut cancelled=Puzzle2dDeleteNodeInverseCursor::default();
            for turn in 0..pause {let permit=grant(turn);if matches!(observed(permit,||cancelled.advance(source.borrow(&base),mutation.borrow(&payload),permit)),RetainedCloneStep::Complete(_)){break;}}
            close(&mut cancelled);assert!(cancelled.take().is_none());
        }
        let mut swapped=Puzzle2dDeleteNodeInverseCursor::default();observed(grant(0),||swapped.advance(source.borrow(&base),mutation.borrow(&payload),grant(0)));
        let other=payload.clone();assert!(swapped.advance(source.borrow(&base),mutation.borrow(&other),grant(1)).is_err());close(&mut swapped);
        assert_eq!(serde_json::to_value(&base).unwrap(),original);assert_eq!(serde_json::to_value(&payload).unwrap(),original_payload);
        eprintln!("[DEBUG] Puzzle2d DeleteNode inverse {} preserves literal native Node/Edge records, original ordinals and optional flags with zero-heap constructor and admitted close/cancellation",case["id"].as_str().unwrap());
    }
    test_source_custody::close(&mut source);test_source_custody::close(&mut mutation);
}
