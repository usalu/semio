//! 🧪️ Neutral original binary path frames preserve exact keys and indices.
use super::*;
#[test]
fn semio_value_binary_paths_match_shared_frames_and_reject_truncated_children(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let original=row["path"].as_array().unwrap().iter().map(|item|match item["kind"].as_str().unwrap(){"key"=>SemioValuePathSegment::Key{key:item["key"].as_str().unwrap().into()},"index"=>SemioValuePathSegment::Index{index:usize::try_from(item["index"].as_u64().unwrap()).unwrap()},_=>panic!("shared path kind")}).collect::<Vec<_>>();
  let expected=row["bytes"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>();
  let mut bytes=Vec::new();enc_semio_path_bin(&original,&mut bytes);assert_eq!(bytes,expected,"{}",row["name"]);
  let mut reader=store::ByteReader::new(&bytes);assert_eq!(dec_semio_path_bin(&mut reader).unwrap(),original);assert_eq!(reader.remaining(),0);
 }
 for row in fixture["malformed"].as_array().unwrap(){let bytes=row.as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>();assert!(dec_semio_path_bin(&mut store::ByteReader::new(&bytes)).is_err(),"{bytes:?}");}
 println!("[DEBUG] Semio binary path native frames match shared Unicode/mixed/index corpus and reject malformed children");
}
