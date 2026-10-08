use super::*;
#[test]
fn neutral_baseline_facts_preserve_every_native_conformance_axis(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){
  let facts:JpgBaselineFacts=semio_framework_pack_json::from_json_str(&case["facts"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let owned=semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&facts));assert_eq!(serde_json::from_str::<serde_json::Value>(&owned).unwrap(),case["facts"]);
  let codes:Vec<String>=check_baseline_facts(&facts).into_iter().map(|diagnostic|diagnostic.code.0).collect();assert_eq!(serde_json::to_value(codes).unwrap(),case["codes"]);
 }
 println!("[DEBUG] JPEG baseline neutral facts independently parsed by serde_json");
}
#[test]
fn baseline_facts_cancel_inside_owned_component_traversal(){
 let mut facts=JpgBaselineFacts{has_frame:true,baseline_sequential:true,sample_precision:8,arithmetic_conditioning:false,dc_table_count:2,ac_table_count:2,components:Vec::new()};
 facts.components=vec![JpgSamplingFact{id:1,horizontal:1,vertical:1};2000];let mut reached=false;
 assert!(check_baseline_facts_with(&facts,&mut|position,total|{if position==256&&total==2000{reached=true;Err(ValueError::new(semio_framework_value::ValueRefusalKind::Canceled,"canceled"))}else{Ok(())}}).is_err());assert!(reached);
}
