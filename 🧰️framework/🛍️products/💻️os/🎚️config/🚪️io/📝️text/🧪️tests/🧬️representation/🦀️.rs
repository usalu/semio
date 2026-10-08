//! 🧪️ Neutral config wire declarations checked against independent Serde JSON.

use super::{mutations, snapshot};
use semio_framework_value::ToValue;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("neutral IO fixture")
}

#[test]
fn service_io_snapshots_match_neutral_json_and_independent_serde() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().expect("cases") {
        let input = case["snapshot"].to_string();
        let output = match case["kind"].as_str().expect("kind") {
            "opening" => snapshot::encode_opening_preferences_json(&snapshot::decode_opening_preferences_json(&input).expect("opening")),
            "ui-preferences" => snapshot::encode_ui_preferences_json(&snapshot::decode_ui_preferences_json(&input).expect("UI")),
            "identity" => snapshot::encode_identity_setting_json(&snapshot::decode_identity_setting_json(&input).expect("identity")),
            "merge-policy" => snapshot::encode_merge_policy_setting_json(&snapshot::decode_merge_policy_setting_json(&input).expect("policy")),
            "local-catalog" => snapshot::encode_local_catalog_json(&snapshot::decode_local_catalog_json(&input).expect("catalog")),
            "local-folders" => snapshot::encode_local_folder_bindings_json(&snapshot::decode_local_folder_bindings_json(&input).expect("folders")),
            other => panic!("undeclared facet {other}"),
        };
        assert_eq!(serde_json::from_str::<serde_json::Value>(&output).expect("independent JSON"), case["snapshot"]);
        eprintln!("[DEBUG] Config IO snapshot: facet={} independentSerde=true", case["kind"]);
    }
    assert!(snapshot::decode_identity_setting_json("[]").is_err());
    assert!(snapshot::decode_local_catalog_json("{\"documents\":[],\"documents\":[]}").is_err());
}

#[test]
fn service_io_mutations_match_neutral_json_and_refuse_malformed_frames() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().expect("cases") {
        let input = case["mutation"].to_string();
        let value = match case["kind"].as_str().expect("kind") {
            "opening" => mutations::decode_opening_config_mutation_json(&input).expect("opening").to_value(),
            "ui-preferences" => mutations::decode_ui_preferences_config_mutation_json(&input).expect("UI").to_value(),
            "identity" => mutations::decode_identity_config_mutation_json(&input).expect("identity").to_value(),
            "merge-policy" => mutations::decode_merge_policy_config_mutation_json(&input).expect("policy").to_value(),
            "local-catalog" => mutations::decode_local_catalog_config_mutation_json(&input).expect("catalog").to_value(),
            "local-folders" => mutations::decode_local_folders_config_mutation_json(&input).expect("folders").to_value(),
            other => panic!("undeclared facet {other}"),
        };
        let output = semio_framework_pack_json::to_json_string(&value);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&output).expect("independent JSON"), case["mutation"]);
        eprintln!("[DEBUG] Config IO mutation: facet={} independentSerde=true", case["kind"]);
    }
    assert!(mutations::decode_identity_config_mutation_json("{\"mutation\":\"physical-replacement\"}").is_err());
    assert!(mutations::decode_local_folders_config_mutation_json("[]").is_err());
}
