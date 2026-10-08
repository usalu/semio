use super::*;

#[test]
fn lifecycle_fixture_is_schema_first() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/📦️lifecycle/🔣️.json")).expect("retained clone preparation fixture");
    assert_eq!(fixture["cases"].as_array().expect("lifecycle cases").len(), 6);
    assert!(fixture["largeCapacity"]["stringByteLength"].as_u64().expect("large string byte length") > fixture["grant"]["maximumBytes"].as_u64().expect("per-turn byte grant"));
    assert_eq!(fixture["largeCapacity"]["expectedCode"], "retained-clone.step-grant-too-small");
    let maximum = fixture["grant"]["maximumBytes"].as_u64().unwrap() as usize;
    for grant in [RetainedCloneGrant::one_capacity_turn(maximum, 64), RetainedCloneGrant::one_payload_turn(maximum, 64), RetainedCloneGrant::one_release_turn(maximum, 64)] {
        assert_eq!(grant.maximum_capacity_bytes + grant.maximum_copy_bytes + grant.maximum_release_bytes, fixture["grant"]["combinedCapacityCopyAndReleaseMaximum"].as_u64().unwrap() as usize);
    }
}
