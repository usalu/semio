use super::*;
use semio_framework_value::{FromValue as _, ToValue as _};

#[test]
fn idml_intrinsic_admission_and_roundtrip_match_language_neutral_witness() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("intrinsic.json")).unwrap();
    let valid = &fixture["valid"];
    let source = valid.to_string();
    let package: IdmlPackage = semio_framework_pack_json::from_json_str(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&package)).unwrap();
    assert_eq!(&actual, valid);
    let encoded = package.encode().unwrap();
    let decoded = IdmlPackage::decode(&encoded).unwrap();
    assert_eq!(decoded.to_value(), package.to_value());
    for case in fixture["invalid"].as_array().unwrap() {
        let mut value = valid.clone();
        match case["operation"].as_str().unwrap() {
            "setVersion" => value["domVersion"] = case["value"].clone(),
            "setPath" => value["parts"][1]["path"] = case["value"].clone(),
            "duplicate" => { let row = value["parts"][1].clone(); value["parts"].as_array_mut().unwrap().push(row); }
            "setDesignmap" => value["parts"][1]["bytes"] = serde_json::json!(case["value"].as_str().unwrap().as_bytes()),
            _ => panic!("unknown invalid operation"),
        }
        let source = value.to_string();
        assert!(semio_framework_pack_json::from_json_str::<IdmlPackage>(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "{}", case["name"]);
    }
}

#[path = "native.rs"]
mod native;
