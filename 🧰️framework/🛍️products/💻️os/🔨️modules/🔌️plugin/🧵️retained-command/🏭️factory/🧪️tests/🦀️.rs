use super::*;
#[test]
fn retained_factory_quotes_exact_frame_raw_bytes_and_refuses_before_transfer(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 assert_eq!(fixture["payloadBytes"],13);assert_eq!(fixture["jobBytes"],29);
 for row in fixture["cases"].as_array().unwrap(){
  let input=row["inputBytes"].as_u64().map(|n|n as usize);
  let quote=quote::<[u8;13],[u8;29]>(input,row["checkpoint"].as_bool().unwrap().then(||row["checkpointBytes"].as_u64().unwrap_or(0)as usize),row["ownerMaximum"].as_u64().unwrap()as usize,row["resume"].as_bool().unwrap(),row["payloadMaximum"].as_u64().unwrap()as usize,row["payloadAdmitted"].as_bool().unwrap());
  if row["refusal"].is_null(){let demand=quote.unwrap();assert_eq!(demand,ToolJobAdmissionDemand{items:1,copy_bytes:0,capacity_bytes:row["capacityBytes"].as_u64().unwrap()as usize,release_bytes:13,depth:1});}
  else{assert_eq!(quote.unwrap_err().kind,ValueRefusalKind::InvalidValue);}
 }
 assert!(quote::<[u8;13],[u8;29]>(Some(usize::MAX),None,usize::MAX,false,usize::MAX,true).is_err());
 eprintln!("[DEBUG] retained factory declares exact frame plus eligible raw reservation, preserves explicit checkpoint policy and refuses overflow before source transfer");
}
