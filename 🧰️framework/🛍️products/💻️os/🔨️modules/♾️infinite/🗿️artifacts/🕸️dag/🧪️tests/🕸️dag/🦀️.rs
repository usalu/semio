//! 🧪️ DAG package schema and serialization oracle.
use crate::*;
#[test]
fn language_neutral_package_cases_match_serde_json() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../📦️package-contract/📜️cases.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let expected = &case["snapshot"];
        let value: DagSnapshot = os_pack::json::from_json_str(&serde_json::to_string(expected).unwrap()).unwrap();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&os_pack::json::to_json_string(&value)).unwrap(), *expected);
        os_store::test_support::assert_dsl_pack_equivalence(&value);
        println!("[DEBUG] DAG package case {} passed first-party codecs and serde_json oracle", case["id"]);
    }
}
