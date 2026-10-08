//! 🧪️ Set wire parity, exact deduplication, retained insertion and retirement.

use super::*;
use super::super::Grant;

//#region 🧪️SetLaws
#[test]
fn ordered_set_wire_matches_serde_btree_oracle_and_retirement_uses_tiny_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let values: Vec<String> = serde_json::from_value(fixture["values"].clone()).unwrap();
    let oracle: std::collections::BTreeSet<String> = values.iter().cloned().collect();
    let set: OrderedSet = values.into_iter().collect();
    assert_eq!(serde_json::to_value(&set).unwrap(), serde_json::to_value(&oracle).unwrap());
    assert_eq!(serde_json::to_value(&set).unwrap(), fixture["expectedValues"]);
    let alias = set.clone();
    assert!(std::ptr::eq(set.iter().next().unwrap(), alias.iter().next().unwrap()));
    for(index,key)in oracle.iter().enumerate(){let original=set.key_at_rank(index).unwrap();let shared=alias.key_at_rank(index).unwrap();assert_eq!(original,key);assert!(std::ptr::eq(original,shared));assert_eq!(original.as_ptr(),shared.as_ptr());}
    assert!(set.key_at_rank(set.len()).is_none());assert!(set.key_at_rank(usize::MAX).is_none());
    let mut first = set.retire();
    while !matches!(first.advance(RetainedCloneGrant::one_release_turn(first.next_close_byte_demand().unwrap(),first.next_depth_demand())), RetirementStep::Complete) {}
    let bytes: usize = alias.iter().map(String::capacity).sum::<usize>()+alias.len()*(super::super::SharedOwner::<super::super::Node<()>>::allocation_bytes()+super::super::SharedOwner::<super::super::Entry<()>>::allocation_bytes()+super::super::SharedOwner::<String>::allocation_bytes()+super::super::SharedOwner::<()>::allocation_bytes());
    let mut last = alias.retire();
    assert!(matches!(last.advance(RetainedCloneGrant::one_release_turn(0,last.next_depth_demand())), RetirementStep::Blocked));
    let mut released = 0;
    loop {
        let physical=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:1,maximum_capacity_bytes:0,maximum_release_bytes:last.next_close_byte_demand().unwrap(),maximum_depth:last.next_depth_demand()};
        match last.advance(physical) {
            RetirementStep::Progress { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= physical.maximum_release_bytes); released += released_bytes; }
            RetirementStep::ProcessedBytes(bytes)=>assert!(bytes<=physical.maximum_copy_bytes),
            RetirementStep::OwnedValue(()) => {}
            RetirementStep::Complete => break,
            RetirementStep::Blocked => panic!("positive set retirement grant blocked"),
            RetirementStep::Failure(error)=>panic!("{error}"),
        }
    }
    assert_eq!(released, bytes);
    eprintln!("[DEBUG] OrderedSet original ranked keys share pointer identity, match independentBTreeSet and retain onebyte work with separate whole allocation release grants");
}

#[test]
fn ordered_set_insert_cursor_uses_existing_map_authority_and_keeps_array_wire() {
    let base = OrderedSet::from(["a".into()]);
    let mut cursor = base.begin_insert("🧵".repeat(1100));
    while !cursor.is_complete() { cursor.advance(RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:1,maximum_capacity_bytes:cursor.next_capacity_byte_demand().unwrap(),maximum_release_bytes:0,maximum_depth:cursor.next_depth_demand()}).unwrap(); }
    let result = OrderedSet::from_map(cursor.take_result().unwrap());
    assert_eq!(result.len(), 2);
    assert_eq!(serde_json::to_value(&result).unwrap().as_array().unwrap().len(), 2);
    cursor.begin_close();
    while !matches!(cursor.close_step(RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:1,maximum_capacity_bytes:0,maximum_release_bytes:cursor.next_close_byte_demand().unwrap(),maximum_depth:cursor.next_close_depth_demand()}), RetirementStep::Complete) {}
    base.retire_cold(); result.retire_cold();
}
//#endregion 🧪️SetLaws
