//! ♻️ Cold property cleanup drains original deep native owners on a finite stack.
use crate::manifest::{PropertyBag,PropertyValue};
use semio_framework_value::FromValue;

fn original(value:&serde_json::Value)->PropertyValue {
    match value["kind"].as_str().unwrap() {
        "null"=>PropertyValue::Null,
        "bool"=>PropertyValue::Bool(value["value"].as_bool().unwrap()),
        "number"=>PropertyValue::Number(f64::from_bits(u64::from_str_radix(value["value"]["bits"].as_str().unwrap(),16).unwrap())),
        "string"=>PropertyValue::String(value["value"].as_str().unwrap().to_owned()),
        "array"=>PropertyValue::Array(value["values"].as_array().unwrap().iter().map(original).collect()),
        "object"=>PropertyValue::Object(value["values"].as_object().unwrap().iter().map(|(key,value)|(key.clone(),original(value))).collect()),
        _=>unreachable!(),
    }
}

#[test]
fn graph_cold_property_retirement_drains_original_nested_owners_on_a_finite_stack() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/♻️cold-properties/🔣️.json")).unwrap();
    let mut nodes=0;
    for value in fixture["values"].as_array().unwrap(){
        let value=original(value);let mut pending=vec![&value];
        while let Some(node)=pending.pop(){nodes+=1;match node{PropertyValue::Array(values)=>pending.extend(values),PropertyValue::Object(values)=>pending.extend(values.values()),_=>{}}}
        PropertyValue::retire_decoded(value);
    }
    assert_eq!(nodes,fixture["expectedNodes"].as_u64().unwrap());
    eprintln!("[DEBUG] cold-graph-property native-schema-nodes={nodes} original-owners-drained=true");
    for profile in fixture["deepProfiles"].as_array().unwrap(){
        let depth=profile["depth"].as_u64().unwrap() as usize;
        let alternating=profile["alternating"].as_bool().unwrap();
        std::thread::Builder::new().stack_size(fixture["stackBytes"].as_u64().unwrap() as usize).spawn(move||{
            let mut value=PropertyValue::String("original\0文字".to_owned());
            for index in 0..depth{value=if alternating&&index%2==1{PropertyValue::Object(PropertyBag::from([("original".to_owned(),value)]))}else{PropertyValue::Array(vec![value])};}
            PropertyValue::retire_decoded(value);
            eprintln!("[DEBUG] cold-graph-property native-depth={depth} alternating={alternating} original-owners-drained=true");
        }).unwrap().join().unwrap();
    }
}
