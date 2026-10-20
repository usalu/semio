//! 🔍️ Private String cursor prefix selection retains exact UTF-8 and admitted-maximum assertions.
use super::*;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value_clone_source_testing::FixtureSourceBirth;
use crate::retained_clone::RetainedCloneSource;

fn close_all<M: RetainedClone>(cursor: &mut M::Cursor) {
    cursor.begin_close();
    for _ in 0..100_000 {
        if cursor.terminal_is_empty() { return; }
        let d=cursor.close_demands(1<<16).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(cursor.close_demands(grant.maximum_copy_bytes).unwrap(),d);
        let step = cursor.close_step(grant).unwrap();
        assert!(step.progress().fits(grant));
    }
    panic!("standard collection clone close did not terminate");
}

#[test]
fn reselected_string_copy_budget_matches_exact_utf8_prefix_and_actual_maximum(){
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧺️std-collections/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for row in fixture["copySelection"].as_array().unwrap(){let text=row["source"].as_str().unwrap();let source=RetainedCloneSource::<String>::from_owner(text.to_owned());let mut cursor=String::retained_clone_cursor();for _ in 0..4{if cursor.phase==1{break}let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:text.len(),maximum_depth:1,..Default::default()};assert!(cursor.advance(source.borrow(), text.len(), grant).unwrap().progress().fits(grant));}assert_eq!(cursor.phase,1);let requested=row["requestedBytes"].as_u64().unwrap()as usize;let maximum=row["admittedBytes"].as_u64().unwrap()as usize;let original=cursor.advance_demands(source.borrow(),requested).unwrap();let selected=cursor.advance_demands(source.borrow(),maximum).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:maximum,maximum_depth:selected.depth,..Default::default()};let step=cursor.advance(source.borrow(), maximum, grant).unwrap();assert!(step.progress().fits(grant));assert_eq!(step.progress().copied_bytes,row["expectedCopiedBytes"].as_u64().unwrap()as usize);assert!(step.progress().copied_bytes<=selected.copy_bytes);assert!(maximum<original.copy_bytes);let prefix=row["expectedPrefix"].as_str().unwrap();assert_eq!(cursor.output.as_deref().unwrap(),prefix);let oracle=String::from_utf8(text.as_bytes()[..prefix.len()].to_vec()).unwrap();assert_eq!(cursor.output.as_deref().unwrap(),oracle);close_all::<String>(&mut cursor);}
}
