//! 🧪️ Portable presentation vectors checked with the independent JSON oracle.
use super::*;
use semio_framework_value::{FromValue,ToValue};
#[test]
fn portable_presentation_contract_matches_json_oracle(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🖼️presentation/🔣️.json")).expect("portable corpus");
 for value in corpus["ports"].as_array().expect("ports"){
  let port:IoPortSpec=semio_framework_pack_json::from_json_str(&value.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("port contract");
  let actual:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&port.to_value())).expect("independent JSON");
  for key in ["id","label","abbreviation","fullName","shape","visible"]{if value.get(key).is_some(){assert_eq!(actual[key],value[key],"{key}");}}
 }
 for value in corpus["labels"].as_array().expect("labels"){
  let (name,abbreviation)=normalize_node_display(value["name"].as_str().unwrap(),value["abbreviation"].as_str().unwrap());
  assert_eq!(serde_json::json!([name,abbreviation]),value["expected"]);
 }
 let values=corpus["expanded"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_owned()).collect::<DagExpandedPaths>();
 let actual:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&values.to_value())).unwrap();
 assert_eq!(actual,corpus["expectedExpanded"]);
 let snapshot=DagHostSnapshot::default();
 let actual:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&snapshot.to_value())).unwrap();
 assert_eq!(actual["schema"],corpus["emptySnapshot"]["schema"]);
 for key in ["x","y","zoom"]{assert_eq!(actual["camera"][key].as_f64(),corpus["emptySnapshot"]["camera"][key].as_f64());}
 assert_eq!(actual["nodes"],corpus["emptySnapshot"]["nodes"]);assert_eq!(actual["edges"],corpus["emptySnapshot"]["edges"]);
 let (nodes,edges)=dag_host_snapshot_execution_rows(&snapshot);
 assert!(nodes.is_empty()&&edges.is_empty());
 assert_eq!(dag_host_snapshot_to_wire_literal(&snapshot),corpus["emptyWire"].as_str().unwrap());

}
