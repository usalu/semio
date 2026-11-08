const MAXIMUM_CLONE_WORK:u64=4096;
fn actual_step<E:std::fmt::Debug>(turn:crate::RetainedWorkOutcome<RetainedCloneStep,E>)->RetainedCloneStep{let work=turn.work;let step=turn.result.unwrap();assert!(step.progress()==Default::default()||work>0);step}
use super::*;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value_clone_source_testing::FixtureSourceBirth;
use crate::retained_clone::{RetainedCloneSource,effect};

fn close_all<M: RetainedClone>(cursor: &mut M::Cursor) {
    {let d=cursor.begin_close_demand().unwrap();let started=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};effect::begin_close_paid(cursor,started).result.unwrap();}
    for _ in 0..100_000 {
        if cursor.terminal_is_empty() { return; }
        let d=cursor.close_demands(1<<16).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(cursor.close_demands(grant.maximum_copy_bytes).unwrap(),d);
        let step = actual_step(effect::close_step(cursor,grant));
        assert!(step.progress().fits(grant));
    }
    panic!("standard collection clone close did not terminate");
}

#[test]
fn reselected_string_copy_budget_matches_exact_utf8_prefix_and_actual_maximum(){
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧺️std-collections/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for row in fixture["copySelection"].as_array().unwrap(){let text=row["source"].as_str().unwrap();let source=RetainedCloneSource::<String>::from_owner(text.to_owned());let mut cursor=String::retained_clone_cursor();for _ in 0..4{if cursor.phase==1{break}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:source.borrow().binding_copy_bytes(),maximum_capacity_bytes:text.len(),maximum_depth:1,..Default::default()};assert!(actual_step(effect::advance(&mut cursor,source.borrow(), text.len(),MAXIMUM_CLONE_WORK, grant)).progress().fits(grant));}assert_eq!(cursor.phase,1);let requested=row["requestedBytes"].as_u64().unwrap()as usize;let maximum=row["admittedBytes"].as_u64().unwrap()as usize;let original=cursor.advance_demands(source.borrow(),requested,MAXIMUM_CLONE_WORK).unwrap();let selected=cursor.advance_demands(source.borrow(),maximum,MAXIMUM_CLONE_WORK).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:selected.copy_bytes,maximum_depth:selected.depth,..Default::default()};let step=actual_step(effect::advance(&mut cursor,source.borrow(), maximum,MAXIMUM_CLONE_WORK, grant));assert!(step.progress().fits(grant));assert_eq!(step.progress().copied_bytes-size_of::<usize>(),row["expectedCopiedBytes"].as_u64().unwrap()as usize);assert!(step.progress().copied_bytes<=selected.copy_bytes);assert!(maximum<original.copy_bytes);let prefix:Vec<u8>=row["expectedPrefixBytes"].as_array().unwrap().iter().map(|b|b.as_u64().unwrap()as u8).collect();assert_eq!(cursor.bytes.as_deref().unwrap(),prefix.as_slice());assert_eq!(cursor.bytes.as_deref().unwrap(),&text.as_bytes()[..prefix.len()]);close_all::<String>(&mut cursor);}
}
