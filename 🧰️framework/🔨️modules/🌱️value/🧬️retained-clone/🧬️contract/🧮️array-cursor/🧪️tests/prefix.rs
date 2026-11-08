//! 🧫️ The native selector consumes the same portable frontier vectors as the independent SQLite oracle.
use semio_framework_value_clone_selection::{ArrayPrefixShape,select_array_prefix};
#[test]
fn prefix_selection_matches_all_portable_refusal_and_work_vectors(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/staged.json")).unwrap();
 for row in rows.as_array().unwrap(){let n=|key|row[key].as_u64().unwrap()as usize;let chosen=select_array_prefix(ArrayPrefixShape{length:n("length"),index:n("index"),element_bytes:n("width"),index_bytes:n("wordBytes"),binding_work:5},n("body"),row["work"].as_u64().unwrap()).unwrap();assert_eq!(chosen.elements,n("count"),"{}",row["name"]);assert_eq!(chosen.copy_bytes,n("copy"),"{}",row["name"]);assert_eq!(chosen.work,row["spent"].as_u64().unwrap(),"{}",row["name"]);}
}
#[test]
fn prefix_selection_rejects_invalid_frontiers_and_work_overflow(){
 let shape=ArrayPrefixShape{length:1,index:2,element_bytes:1,index_bytes:8,binding_work:5};assert!(select_array_prefix(shape,4096,64).is_err());assert!(select_array_prefix(ArrayPrefixShape{index:0,binding_work:u64::MAX,..shape},4096,64).is_err());
}
