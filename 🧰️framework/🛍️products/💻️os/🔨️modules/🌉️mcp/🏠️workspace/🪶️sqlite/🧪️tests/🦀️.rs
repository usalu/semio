use super::*;
use store::{ArtifactDsl as _,ArtifactSqliteSnapshot as _};
use semio_framework_value::{FromValue as _,ToValue as _,retirement::RetireOwned as _};

#[test]
fn probe_sqlite_original_first_party_tree_matches_neutral_node_entities(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 assert!(super::super::ProbeSnapshot::controlled_retirement_supported());
 for sample in fixture["cases"].as_array().unwrap(){
  let wire=sample["wire"].as_str().unwrap();
  let snapshot=super::super::ProbeSnapshot::parse_dsl(wire).unwrap();
  let oracle:serde_json::Value=serde_json::from_str(wire).unwrap();
  assert_eq!(serde_json::Value::from(snapshot.to_value()),oracle);
  let pointer=match &snapshot.0{semio_framework_value::DslValue::Object(entries)=>Some(entries.as_ptr()),_=>None};
  let moved=super::super::ProbeSnapshot::from_value(snapshot.0).unwrap();
  if let Some(pointer)=pointer{let semio_framework_value::DslValue::Object(entries)=&moved.0 else{panic!("original object owner")};assert_eq!(entries.as_ptr(),pointer);}
  let mut callback=|_|true;
  let mut control=SqliteSnapshotControl::new(&mut callback,Default::default());
  let database=moved.to_sqlite_database(&mut control).unwrap();
  let rows=database.table("probe_node").unwrap().rows.iter().map(|row|row.values.iter().map(|cell|match cell{SqliteValue::Null=>serde_json::Value::Null,SqliteValue::Integer(value)=>serde_json::json!(value),SqliteValue::Text(value)=>serde_json::json!(value),_=>panic!("declared Probe cell")}).collect::<Vec<_>>()).collect::<Vec<_>>();
  assert_eq!(serde_json::json!(rows),sample["rows"]);
  let rebuilt=super::super::ProbeSnapshot::from_sqlite_database(&database,&mut control).unwrap();
  assert_eq!(rebuilt,moved);
  assert_eq!(serde_json::from_str::<serde_json::Value>(&rebuilt.print_dsl()).unwrap(),oracle);
 }
 for wire in fixture["rejectedWire"].as_array().unwrap(){assert!(super::super::ProbeSnapshot::parse_dsl(wire.as_str().unwrap()).is_err());}
 eprintln!("[DEBUG] Probe first-party moved trees preserve neutral ordered relational nodes and exact Serde numeric semantics");
}
