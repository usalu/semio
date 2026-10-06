//! 💰️ Actual JSON logical input settles caller-owned backing independently of SQL values.
use super::*;
#[test]
fn sqlite_snapshot_json_owned_input_allocation_is_independent_exact_and_cumulative(){
 let neutral:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/💰️allocation/🔣️.json")).unwrap();
 let (_,mut snapshot)=fixture();
 snapshot.schema=neutral["schema"].as_str().unwrap().repeat(neutral["schemaRepeat"].as_u64().unwrap()as usize);
 let literal=neutral["literal"].as_str().unwrap();
 snapshot.value=JsonValue::Object{members:vec![JsonMember{key:literal.into(),value:JsonValue::String{value:literal.into()}},JsonMember{key:literal.into(),value:JsonValue::String{value:String::new()}}]};
 let limits=SqliteDatabaseLimits{max_allocation_bytes:neutral["maximumAllocationBytes"].as_u64().unwrap()as usize,..SqliteDatabaseLimits::default()};
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let payload=match encoding{SnapshotEncoding::Binary=>store::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),SnapshotEncoding::Text=>store::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))};
  let mut yes=|_|true;let mut control=SqliteSnapshotControl::new(&mut yes,SqliteDatabaseLimits{max_value_bytes:neutral["semanticBytes"].as_u64().unwrap()as usize,..limits});
  let restored=JsonSnapshot::decode_sqlite_snapshot_native(&payload,&mut control).expect("native input backing must not use the semantic SQL allowance");
  assert_eq!(restored,snapshot);restored.retire_sqlite_snapshot();
  let used=limits.max_allocation_bytes-control.allocation_remaining_bytes();
  assert!(used>=snapshot.schema.len(),"actual owned parser, metadata and binding backing must settle");
  let mut yes=|_|true;let mut exact=SqliteSnapshotControl::new(&mut yes,SqliteDatabaseLimits{max_allocation_bytes:used,..limits});
  let restored=JsonSnapshot::decode_sqlite_snapshot_native(&payload,&mut exact).expect("exact measured admission must decode identically");
  assert_eq!(restored,snapshot);restored.retire_sqlite_snapshot();assert_eq!(exact.allocation_remaining_bytes(),0);
  match JsonSnapshot::decode_sqlite_snapshot_native(&payload,&mut exact){Ok(candidate)=>{candidate.retire_sqlite_snapshot();panic!("a retired owned input must not reset cumulative admission");},Err(error)=>assert_eq!(error.kind,store::sqlite_snapshot::ValueRefusalKind::OwnershipLimit)}assert_eq!(exact.allocation_remaining_bytes(),0);
  for maximum in[neutral["tinyAllocationBytes"].as_u64().unwrap()as usize,used-1]{
   let mut yes=|_|true;let mut limited=SqliteSnapshotControl::new(&mut yes,SqliteDatabaseLimits{max_allocation_bytes:maximum,..limits});
   match JsonSnapshot::decode_sqlite_snapshot_native(&payload,&mut limited){Ok(candidate)=>{candidate.retire_sqlite_snapshot();panic!("{encoding:?}: insufficient caller backing must refuse before publication");},Err(error)=>assert_eq!(error.kind,store::sqlite_snapshot::ValueRefusalKind::OwnershipLimit)}
   assert!(limited.allocation_remaining_bytes()<=maximum);
  }
 }
 snapshot.retire_sqlite_snapshot();
}
