//! 🧪️ Item-family primitive laws share the same projection vectors as actual TypeScript endpoints.
use super::*;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcComplexType, IfcEntity, IfcHeader};
fn source(row: &serde_json::Value) -> IfcSnapshot {
    let names = row["names"].as_array().unwrap();
    IfcSnapshot { header: IfcHeader { file_description: vec![], file_name: vec![], file_schema: vec![] }, entities: vec![IfcEntity { id: 1, name: names[0].as_str().unwrap().into(), args: vec![], complex: names[1..].iter().map(|name| IfcComplexType { name: name.as_str().unwrap().into(), args: vec![] }).collect() }], ..Default::default() }
}
fn policy(row: &serde_json::Value) -> BodyPolicy {
    match row["policy"].as_str().unwrap() { "tessellation" => BodyPolicy::Tessellation, "swept-solid" => BodyPolicy::SweptSolid, _ => panic!("Unknown fixture policy") }
}
#[test]
fn native_body_family_neutral_projection_cases() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("family-cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let source = source(row);
        let mut cursor = BodyFamilyMembership::new(policy(row));
        assert_eq!(cursor.result(), BodyFamilyResult::Pending);
        cursor.start(0);
        assert_eq!(cursor.result(), BodyFamilyResult::Pending);
        let mut ready = false;
        for _ in 0..4096 { if cursor.advance(&source) { ready = true; break; } }
        assert!(ready);
        let expected = match row["expected"].as_str().unwrap() { "pass" => BodyFamilyResult::Resolved(true), "fail" => BodyFamilyResult::Resolved(false), "incomplete" => BodyFamilyResult::Unknown, _ => unreachable!() };
        assert_eq!(cursor.result(), expected);
    }
}
#[test]
fn native_body_family_pending_and_restart_are_distinct_from_negative_membership() {
    let row = serde_json::json!({"policy":"swept-solid","names":["IFCEXTRUDEDAREASOLIDTAPERED","IFCUNKNOWNITEM"]});
    let source = source(&row);
    let mut cursor = BodyFamilyMembership::new(BodyPolicy::SweptSolid);
    assert!(!cursor.advance(&IfcSnapshot::default()));
    cursor.start(0);
    while !cursor.advance(&source) {}
    assert_eq!(cursor.result(), BodyFamilyResult::Unknown);
    cursor.start(0);
    assert_eq!(cursor.result(), BodyFamilyResult::Pending);
    let mut moved = cursor;
    while !moved.advance(&source) {}
    assert_eq!(moved.result(), BodyFamilyResult::Unknown);
}
#[test]
fn native_body_family_steps_are_heap_free() {
    let source = source(&serde_json::json!({"names":["IFCEXTRUDEDAREASOLID","IFCREVOLVEDAREASOLIDTAPERED","IFCUNKNOWNITEM"]}));
    let mut cursor = BodyFamilyMembership::new(BodyPolicy::SweptSolid);
    cursor.start(0);
    let (ready, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| { for _ in 0..4096 { if cursor.advance(&source) { return true; } } false });
    assert!(ready);
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    assert_eq!(cursor.result(),BodyFamilyResult::Unknown);
}
