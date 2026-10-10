//! 🧪️ Original public mutation frames retain every shared value-path key and index.
use semio_framework_os_kernel::OpBinary;
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::{mutations::{SemioValueMutation,SemioValuePathSegment,set_value::SetValue},snapshot::SemioValue};

#[test]
fn original_semio_mutation_frames_match_shared_value_paths(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let path=row["path"].as_array().unwrap().iter().map(|part|match part["kind"].as_str().unwrap(){"key"=>SemioValuePathSegment::Key{key:part["key"].as_str().unwrap().into()},"index"=>SemioValuePathSegment::Index{index:usize::try_from(part["index"].as_u64().unwrap()).unwrap()},_=>panic!("shared path kind")}).collect();
  let original=SemioValueMutation::SetValue(SetValue{path,value:SemioValue::Null});let bytes=original.encode_op().unwrap();
  let expected=row["bytes"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>();assert_eq!(&bytes[2..bytes.len()-1],expected.as_slice());assert_eq!(SemioValueMutation::decode_op(&bytes).unwrap(),original);
 }
 let template=SemioValueMutation::SetValue(SetValue{path:Vec::new(),value:SemioValue::Null}).encode_op().unwrap();
 for row in fixture["malformed"].as_array().unwrap(){let mut bytes=template[..2].to_vec();bytes.extend(row.as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8));bytes.push(0);assert!(SemioValueMutation::decode_op(&bytes).is_err(),"{bytes:?}");}
 println!("[DEBUG] Original Semio public mutation codec round-trips four exact shared paths and refuses eight malformed children");
}
