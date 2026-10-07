//! 🔤️ Native fixture hex is decoded independently; logical records refuse native hex bodies.
use crate::BmpSnapshot;
use store::ArtifactDsl;
#[test]
fn independent_hex_fixture_admission_and_logical_record_separation() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔤️native-source-hex/🔣️.json")).unwrap();let expected:Vec<u8>=serde_json::from_value(fixture["bytes"].clone()).unwrap();let pixels:Vec<u8>=serde_json::from_value(fixture["pixels"].clone()).unwrap();
 for case in fixture["cases"].as_array().unwrap() {
  let source=case["source"].as_str().unwrap();let valid=case["valid"].as_bool().unwrap();let compact:String=source.chars().filter(|ch|!ch.is_whitespace()).collect();let oracle=hex::decode(&compact);assert_eq!(oracle.is_ok(),valid);
  assert!(std::panic::catch_unwind(||BmpSnapshot::parse_dsl(source)).unwrap().is_err());
  if valid {let bytes=oracle.unwrap();assert_eq!(bytes,expected);let snapshot=crate::standards::v_v3::subsets::any::io::decode_bmp(&bytes).unwrap();assert_eq!(crate::schema::operations::bmp_rgba8_preview(&snapshot).unwrap(),pixels);assert_eq!(BmpSnapshot::parse_dsl(&snapshot.print_dsl()).unwrap(),snapshot);}
  eprintln!("[DEBUG] bmp native-fixture case={} logical-hex=refused",case["id"]);
 }
}
