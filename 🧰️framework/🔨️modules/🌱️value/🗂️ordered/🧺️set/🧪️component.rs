//! 🧪️ Set wire parity, exact deduplication, retained insertion and retirement.

use super::*;

//#region 🧪️SetLaws
#[test]
fn ordered_set_wire_matches_serde_btree_oracle_and_retirement_uses_tiny_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️ordered-set.json")).unwrap();
    let values: Vec<String> = serde_json::from_value(fixture["values"].clone()).unwrap();
    let oracle: std::collections::BTreeSet<String> = values.iter().cloned().collect();
    let set: OrderedSet = values.into_iter().collect();
    assert_eq!(serde_json::to_value(&set).unwrap(), serde_json::to_value(&oracle).unwrap());
    assert_eq!(serde_json::to_value(&set).unwrap(), fixture["expectedValues"]);
    let alias = set.clone();
    assert!(std::ptr::eq(set.iter().next().unwrap(), alias.iter().next().unwrap()));
    let mut first = set.retire();
    while !matches!(first.advance(Grant { maximum_items: 1, maximum_bytes: 1 }), RetirementStep::Complete) {}
    let bytes: usize = alias.iter().map(String::len).sum();
    let mut last = alias.retire();
    assert!(matches!(last.advance(Grant { maximum_items: 1, maximum_bytes: 0 }), RetirementStep::Blocked));
    let mut released = 0;
    loop {
        match last.advance(Grant { maximum_items: 1, maximum_bytes: 1 }) {
            RetirementStep::Progress { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= 1); released += released_bytes; }
            RetirementStep::OwnedValue(()) => {}
            RetirementStep::Complete => break,
            RetirementStep::Blocked => panic!("positive set retirement grant blocked"),
        }
    }
    assert_eq!(released, bytes);
}

#[test]
fn ordered_set_insert_cursor_uses_existing_map_authority_and_keeps_array_wire() {
    let base = OrderedSet::from(["a".into()]);
    let mut cursor = base.begin_insert("🧵".repeat(1100));
    while !cursor.is_complete() { cursor.advance(Grant { maximum_items: 1, maximum_bytes: 1 }); }
    let result = OrderedSet::from_map(cursor.take_result().unwrap());
    assert_eq!(result.len(), 2);
    assert_eq!(serde_json::to_value(&result).unwrap().as_array().unwrap().len(), 2);
    cursor.begin_close();
    while !matches!(cursor.close_step(Grant { maximum_items: 1, maximum_bytes: 1 }), RetirementStep::Complete) {}
    base.retire_cold(); result.retire_cold();
}
//#endregion 🧪️SetLaws
