use super::*;
use semio_framework_value::{DslValue,NativeDecodeControl};

fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🧩️members/🔣️.json")).unwrap()}

#[test]
fn explicit_member_policy_binds_decoded_names_and_preserves_normalization(){
    let fixture=fixture();
    for case in fixture["cases"].as_array().unwrap(){
        let text=case["text"].as_str().unwrap();
        let replaced=parse(text,JsonMemberPolicy::Replace).unwrap();
        let expected:serde_json::Value=serde_json::from_str(text).unwrap();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&to_string(&replaced)).unwrap(),expected,"{}",case["id"]);
        let rejected=parse(text,JsonMemberPolicy::Reject);
        if case["duplicate"].is_null(){assert_eq!(rejected.unwrap(),replaced);}else{assert_eq!(rejected.unwrap_err(),JsonError::DuplicateMember{name:case["duplicate"]["name"].as_str().unwrap().into(),offset:case["duplicate"]["offset"].as_u64().unwrap()as usize});}
        let mut accept=|_|true;let mut control=NativeDecodeControl::new(1_000_000,&mut accept);
        let replaced:DslValue=from_json_str_controlled(text,JsonMemberPolicy::Replace,&mut control).unwrap();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&to_json_string(&replaced)).unwrap(),expected);
        let mut accept=|_|true;let mut control=NativeDecodeControl::new(1_000_000,&mut accept);
        let rejected=from_json_str_controlled::<DslValue>(text,JsonMemberPolicy::Reject,&mut control);
        assert_eq!(rejected.is_err(),!case["duplicate"].is_null());
    }
    let replaced=parse("{\"z\":1,\"a\":2,\"z\":3}",JsonMemberPolicy::Replace).unwrap();
    assert_eq!(to_string(&replaced),"{\"z\":3,\"a\":2}");
}

#[test]
fn member_comparison_cancels_inside_decoded_key_work(){
    let fixture=fixture();let comparison=&fixture["comparison"];
    let key=comparison["keyUnit"].as_str().unwrap().repeat(comparison["repetitions"].as_u64().unwrap()as usize);
    let text=format!("{{{}:null,{}:null}}",serde_json::to_string(&key).unwrap(),serde_json::to_string(&key).unwrap());
    let mut inside=false;let mut progress=|event:semio_framework_value::native_decoding::NativeDecodeProgress|{if event.total==key.len()&&event.completed>=comparison["chunkBytes"].as_u64().unwrap()as usize&&event.completed<event.total{inside=true;false}else{true}};
    let mut control=NativeDecodeControl::new(comparison["maximumOwnedBytes"].as_u64().unwrap()as usize,&mut progress);
    let result=from_json_str_controlled::<DslValue>(&text,JsonMemberPolicy::Reject,&mut control);
    assert!(result.is_err());drop(control);assert!(inside);
}

#[test]
fn object_ownership_transfers_order_and_descendants_without_copies() {
    let fixture = fixture();
    let text = fixture["ownership"]["text"].as_str().unwrap();
    let Value::Object(object) = parse(text, JsonMemberPolicy::Replace).unwrap() else { panic!("object fixture"); };
    let names: Vec<_> = object.iter().map(|(name, _)| name.as_ptr()).collect();
    let Value::Object(nested) = object.get("z").unwrap() else { panic!("nested fixture"); };
    let Value::String(value) = nested.get("name").unwrap() else { panic!("string fixture"); };
    let text_pointer = value.as_ptr();
    let entries = object.into_entries();
    assert_eq!(entries.iter().map(|(name, _)| name.as_str()).collect::<Vec<_>>(), fixture["ownership"]["keys"].as_array().unwrap().iter().map(|name| name.as_str().unwrap()).collect::<Vec<_>>());
    assert_eq!(entries.iter().map(|(name, _)| name.as_ptr()).collect::<Vec<_>>(), names);
    let Value::Object(nested) = &entries[0].1 else { panic!("nested transfer"); };
    let Value::String(value) = nested.get("name").unwrap() else { panic!("string transfer"); };
    assert_eq!(value.as_ptr(), text_pointer);
    let rebuilt = Value::Object(entries.into_iter().collect());
    assert_eq!(serde_json::from_str::<serde_json::Value>(&to_string(&rebuilt)).unwrap(), serde_json::from_str::<serde_json::Value>(text).unwrap());
}

