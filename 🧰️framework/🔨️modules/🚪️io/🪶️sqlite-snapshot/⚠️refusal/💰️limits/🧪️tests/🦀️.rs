use super::*;

fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn run_case(row:&serde_json::Value,database:&SqliteDatabase)->std::result::Result<(),ValueError>{
 let maximum=row["maximum"].as_u64().unwrap()as usize;let count=row["count"].as_u64().unwrap()as usize;let initial=row["initial"].as_u64().unwrap()as usize;let mut accept=|_|true;
 let limits=SqliteDatabaseLimits{max_rows:maximum,max_value_bytes:maximum,max_allocation_bytes:maximum,..Default::default()};let mut control=SqliteSnapshotControl::new(&mut accept,limits);
 match row["operation"].as_str().unwrap(){
  "rows"=>control.check_rows(count),"value"=>control.check_value_bytes(count),"allocation"=>control.admit_allocation_bytes(count),"nativeAllocation"=>control.admit_native_allocation_bytes(count),"reconstruction"=>control.admit_reconstruction_bytes(count),"scalar"=>control.admit_reconstruction_scalar_bytes(count),
  "narrowAllocation"|"narrowReconstruction"=>{if row["operation"]=="narrowAllocation"{control.admit_allocation_bytes(initial)?;}else{control.admit_reconstruction_bytes(initial)?;}let narrowed=row["narrowedMaximum"].as_u64().unwrap()as usize;let result=control.restrict_limits(SqliteDatabaseLimits{max_value_bytes:narrowed,max_allocation_bytes:narrowed,..limits});assert_eq!(control.limits().max_value_bytes,maximum);assert_eq!(control.limits().max_allocation_bytes,maximum);result},
  "databaseRows"=>control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot),
  "databaseValues"=>{let mut accept=|_|true;SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits{max_rows:1,..limits}).check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)},
  _=>panic!("unowned original control case")
 }
}

#[test]
fn sqlite_snapshot_original_fixed_limits_have_literal_typed_causes_and_zero_heap(){
 let f=fixture();let specimen=&f["database"];let database=SqliteDatabase{tables:vec![SqliteTable{name:specimen["name"].as_str().unwrap().into(),sql:specimen["sql"].as_str().unwrap().into(),rows:vec![SqliteRow{rowid:specimen["id"].as_i64().unwrap(),values:vec![SqliteValue::Integer(specimen["id"].as_i64().unwrap()),SqliteValue::Text(specimen["value"].as_str().unwrap().into())]}]}]};
 let mut failures=0;
 for row in f["cases"].as_array().unwrap(){let(result,born,released)=crate::test_allocation::observe_backing(||run_case(row,&database));assert_eq!(result.is_ok(),row["accepted"].as_bool().unwrap(),"{}",row["id"]);if let Err(error)=result{assert_eq!(error.kind.as_str(),row["expectedKind"].as_str().unwrap());assert_eq!(error.message,row["expectedMessage"].as_str().unwrap());failures+=usize::from(!matches!(error.message,std::borrow::Cow::Borrowed(_)));}eprintln!("[DEBUG] Original SQLite fixed limit {} accepted={} actualAllocated={born} actualReleased={released}",row["id"],row["accepted"]);failures+=usize::from((born,released)!=(0,0));}
 let profile=f["overflowProfiles"].as_array().unwrap().iter().find(|profile|profile["pointerBits"].as_u64().unwrap()==usize::BITS as u64).unwrap();let maximum:usize=profile["maximum"].as_str().unwrap().parse().unwrap();let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits{max_value_bytes:maximum,..Default::default()});control.admit_reconstruction_bytes(maximum).unwrap();control.admit_reconstruction_scalar_bytes(1).unwrap();let(error,born,released)=crate::test_allocation::observe_backing(||control.reconstruction_remaining_bytes().unwrap_err());assert_eq!(error.kind.as_str(),f["aggregateOverflow"]["expectedKind"].as_str().unwrap());assert_eq!(error.message,f["aggregateOverflow"]["expectedMessage"].as_str().unwrap());failures+=usize::from(!matches!(error.message,std::borrow::Cow::Borrowed(_))||(born,released)!=(0,0));eprintln!("[DEBUG] Original SQLite aggregate overflow pointerBits={} actualAllocated={born} actualReleased={released}",usize::BITS);
 let mut cancel=|_|false;let mut control=SqliteSnapshotControl::new(&mut cancel,Default::default());let(error,born,released)=crate::test_allocation::observe_backing(||control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0).unwrap_err());assert_eq!(error.message,f["cancelled"]["expectedMessage"].as_str().unwrap());assert_eq!(error.kind.as_str(),f["cancelled"]["expectedKind"].as_str().unwrap());assert_eq!((born,released),(0,0));assert_eq!(failures,0,"original fixed limit causes require no owned diagnostic backing");
}

#[test]
fn sqlite_snapshot_original_fixed_limits_preserve_forwarded_owned_cause(){
 let f=fixture();let mut original=Some(ValueError::new(ValueRefusalKind::OwnershipLimit,f["forwardedCause"]["message"].as_str().unwrap()));let pointer=original.as_ref().unwrap().message.as_ptr();let mut allocation=|request:semio_framework_value::native_encoding::NativeEncodeAllocation|{assert_eq!((request.bytes,request.owned_bytes,request.next_owned_bytes,request.maximum_bytes),(1,0,1,1));Err(original.take().unwrap())};let mut accept=|_|true;let mut control=SqliteSnapshotControl::new_forwarded(&mut accept,SqliteDatabaseLimits{max_allocation_bytes:1,..Default::default()},&mut allocation);
 let(error,born,released)=crate::test_allocation::observe_backing(||control.admit_allocation_bytes(1).unwrap_err());assert_eq!(error.kind.as_str(),f["forwardedCause"]["expectedKind"].as_str().unwrap());assert_eq!(error.message,f["forwardedCause"]["message"].as_str().unwrap());assert_eq!(error.message.as_ptr(),pointer);assert_eq!((born,released),(0,0));assert_eq!(control.forwarded_owned_bytes(),0);assert_eq!(control.allocation_remaining_bytes(),1);eprintln!("[DEBUG] Original SQLite forwarded cause originalPointer=true actualAllocated={born} actualReleased={released} unchangedSQLLedger=true");
}
