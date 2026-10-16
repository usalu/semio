use super::*;
#[test]
fn finite_resource_spec_matches_neutral_expected_output(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let limits=ResourceLimits{bytes:fixture["limits"]["bytes"].as_u64().unwrap(),dimension:fixture["limits"]["dimension"].as_u64().unwrap()as u32};
    for row in fixture["cases"].as_array().unwrap(){let d=&row["descriptor"];let spec=ResourceSpec{kind:if d["kind"]=="buffer"{ResourceKind::Buffer}else{ResourceKind::Texture},bytes:d["bytes"].as_u64().unwrap(),width:d["width"].as_u64().unwrap()as u32,height:d["height"].as_u64().unwrap()as u32,layers:d["layers"].as_u64().unwrap()as u32,mips:d["mips"].as_u64().unwrap()as u32,samples:d["samples"].as_u64().unwrap()as u32};assert_eq!(spec.validate(limits).is_ok(),row["accepted"].as_bool().unwrap(),"{}",row["id"]);}
}
