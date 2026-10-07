//! 🧪️ The neutral cluster-property fixture retains exact atoms and agrees with serde_json.
use super::*;
use neural_engine::ColdRetire;
#[test]
fn cluster_tree_property_fixture_preserves_exact_atoms_and_matches_serde_json() {
    let fixture = include_str!("../🧫️fixtures/🔣️.json");
    let expected: serde_json::Value = serde_json::from_str(fixture).unwrap();
    let property: PropertyValue = semio_framework_pack_json::from_json_str(fixture, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let tree = cluster_tree_from_property(&property).unwrap();
    assert_eq!(tree.neurons[0].params.get("integer"), Some(&Value::Atom(Atom::Integer(9_007_199_254_740_993))));
    assert_eq!(tree.neurons[0].params.get("decimal"), Some(&Value::Atom(Atom::Decimal(42.0))));
    assert_eq!(tree.neurons[0].tree.as_ref().unwrap().neurons[0].id, "empty");
    let projected = cluster_tree_property(&tree);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&projected)).unwrap(), expected);
    let reconstructed = cluster_tree_from_property(&projected).unwrap();
    assert_eq!(reconstructed, tree);
    reconstructed.retire_cold();
    tree.retire_cold();
    <PropertyValue as FromValue>::retire_decoded(property);
    <PropertyValue as FromValue>::retire_decoded(projected);
}

#[test]
fn cluster_tree_property_refuses_integer_overflow_and_repeated_keys() {
    let fixture = include_str!("../🧫️fixtures/🔣️.json");
    let mut repeated: serde_json::Value = serde_json::from_str(fixture).unwrap();
    repeated["neurons"][0]["params"][1]["key"] = serde_json::Value::String("decimal".into());
    for invalid in [fixture.replace("9007199254740993", "9223372036854775808"), serde_json::to_string(&repeated).unwrap()] {
        let property: PropertyValue = semio_framework_pack_json::from_json_str(&invalid, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert!(cluster_tree_from_property(&property).is_err());
        <PropertyValue as FromValue>::retire_decoded(property);
    }
}
