//! 🧪️ A completed cold projection pays its final retained metadata transition separately.
#[test]
fn cold_projection_metadata_clear_respects_denial_and_reports_actual_work(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let grant=semio_framework_value::RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:row["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:row["depth"].as_u64().unwrap()as usize};
  let(progress,work,terminal)=semio_os_store_one_item_batch::paged_one_item::cold_projection_metadata_case(row["cloned"].as_bool().unwrap(),grant).unwrap();
  assert_eq!(serde_json::json!({"items":progress.copied_items,"copyBytes":progress.copied_bytes,"work":work,"terminal":terminal}),row["expected"],"{}",row["id"]);
  assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(0,0));
 }
}
