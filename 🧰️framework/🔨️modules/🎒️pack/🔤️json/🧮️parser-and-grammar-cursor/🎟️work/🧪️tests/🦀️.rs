use super::*;
#[test]
fn grammar_selected_utf8_probe_counts_actual_work_before_error(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let bytes:Vec<u8>=row["bytes"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();let pointer=bytes.as_ptr();
  let mut parser=JsonGrammarCursor::<DslValue>::new(JsonMemberPolicy::Reject);let mut accept=|_|true;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(&mut accept);
  let result=parser.step_source(bytes.as_slice(),1,&mut control,RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_depth:1,..Default::default()},false);
  assert_eq!(parser.last_turn_work(),row["work"].as_u64().unwrap());assert_eq!(matches!(result,Err(JsonError::InvalidUtf8)),row["invalidUtf8"].as_bool().unwrap());assert_eq!(bytes.as_ptr(),pointer);assert_eq!(parser.normal_step_progress().copied_bytes,fixture["copyBytes"].as_u64().unwrap()as usize);assert_eq!(parser.normal_step_progress().retained_capacity_bytes,0);assert_eq!(parser.normal_step_progress().released_bytes,0);
  let independent=serde_json::from_slice::<serde_json::Value>(&bytes);assert_eq!(independent.is_err(),row["bytes"][0]==255);
  assert!(parser.frames.is_empty());assert!(parser.pending.is_none());assert!(parser.lexeme.is_none());assert!(parser.result.is_none());
 }
}
