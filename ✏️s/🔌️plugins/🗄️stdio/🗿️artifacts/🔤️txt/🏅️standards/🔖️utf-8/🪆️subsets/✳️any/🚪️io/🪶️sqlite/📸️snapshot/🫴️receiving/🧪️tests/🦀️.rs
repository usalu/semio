//! 🧪️ TXT semantic original receiving laws compare neutral rows and actual heap receipts.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::SqliteDatabaseLimits,os_store::serialization::ArtifactSqliteSnapshotHost};
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
fn fixture()->serde_json::Value{serde_json::from_str(fixture_text!("../../../🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
fn grant(value:&serde_json::Value)->RetainedCloneGrant{serde_json::from_value(value.clone()).unwrap()}
fn snapshot(value:&serde_json::Value)->TxtSnapshot{TxtSnapshot{schema:value["schema"].as_str().unwrap().into(),lines:value["lines"].as_array().unwrap().iter().map(|line|line.as_str().unwrap().into()).collect(),trailing_newline:value["trailingNewline"].as_bool().unwrap(),line_ending:if value["lineEnding"]=="crLf"{LineEnding::CrLf}else{LineEnding::Lf}}}
fn database(value:&serde_json::Value)->SqliteDatabase{SqliteDatabase{tables:value["tables"].as_array().unwrap().iter().map(|table|SqliteTable{name:table["name"].as_str().unwrap().into(),sql:table["sql"].as_str().unwrap().into(),rows:table["rows"].as_array().unwrap().iter().map(|row|SqliteRow{rowid:row["rowid"].as_i64().unwrap(),values:row["values"].as_array().unwrap().iter().map(|value|if let Some(value)=value.as_i64(){SqliteValue::Integer(value)}else{SqliteValue::Text(value.as_str().unwrap().into())}).collect()}).collect()}).collect()}}
fn retire<T:semio_framework_value::retirement::RetireOwned>(output:T,law:&serde_json::Value)->(usize,usize){let mut retirement=semio_framework_value::retirement::controlled::ControlledRetirement::new(output).ok().unwrap();let closing=grant(&law["closeGrant"]);let(mut born,mut freed)=(0,0);for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if retirement.terminal_is_empty(){break}let(step,heap)=observe(||retirement.step(closing).unwrap());assert!(step.progress().fits(closing));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;}assert!(retirement.terminal_is_empty());(born,freed)}
macro_rules! original{
 ($name:ident,$native:ident,$owner:ident,$recipient:path,$output:ty)=>{
  fn $name(law:&serde_json::Value,policy:RetainedCloneGrant,maximum:usize,limits:SqliteDatabaseLimits,sql_callback:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool,operation:impl FnOnce(&mut SqliteSnapshotControl<'_>,&mut store::$owner<'_,'_>)->Result<$output>)->(Result<$output>,RetainedCloneProgress,usize,usize){
   let mut native_callback=|_|true;let mut recipient=<$recipient>::new();let mut native=$native::new(maximum,&mut native_callback);native.install_retirement_recipient(&mut recipient).unwrap();let mut sql=SqliteSnapshotControl::new(sql_callback,limits);let sql_before=sql.allocation_remaining_bytes();let mut owner=store::$owner::new(&mut native,policy);
   let(result,heap)=observe(||operation(&mut sql,&mut owner));let receipt=owner.progress();assert!(receipt.fits(policy));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));if let Err(error)=&result{assert_eq!(error.retained_progress(),receipt)}drop(owner);assert_eq!(sql_before-sql.allocation_remaining_bytes(),native.owned_bytes());let(mut born,mut freed)=(heap.requested_bytes,heap.released_bytes);
   let held=native.has_retirement_owner();let(zero,heap)=observe(||native.close_retirement_recipient(grant(&law["deniedCloseGrant"])).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(zero.progress(),RetainedCloneProgress::default());assert_eq!(native.has_retirement_owner(),held);
   let closing=grant(&law["closeGrant"]);for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,heap)=observe(||native.close_retirement_recipient(closing).unwrap());assert!(step.progress().fits(closing));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;}assert!(!native.has_retirement_owner());(result,receipt,born,freed)
  }
 };
}
original!(decode,NativeDecodeControl,NativeSnapshotDecodeOwner,semio_framework_value::native_decoding::NativeDecodeRetirementRecipient,SqliteDatabase);
original!(encode,NativeEncodeControl,NativeSnapshotEncodeOwner,semio_framework_value::native_encoding::NativeEncodeRetirementRecipient,TxtSnapshot);
#[test]
fn txt_original_receiving_complete_rows_preserve_literal_empty_unicode_and_surrogate_semantics(){
 let law=fixture();let policy=grant(&law["bodyGrant"]);let maximum=law["nativeMaximumBytes"].as_u64().unwrap()as usize;
 for case in law["cases"].as_array().unwrap(){
  let source=snapshot(&case["snapshot"]);let input=database(&case["database"]);let source_pointer=source.schema.as_ptr();let input_pointer=input.tables.as_ptr();
  let(projected,_,born,freed)=decode(&law,policy,maximum,Default::default(),&mut |_|true,|sql,owner|source.to_sqlite_database_receiving(sql,owner));let projected=projected.unwrap();let restored=TxtSnapshot::from_sqlite_database(&projected,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();assert_eq!(restored,source);assert_eq!(projected.tables.len(),2);assert_eq!(projected.tables[0].rows[0].values[1],SqliteValue::Text(case["snapshot"]["schema"].as_str().unwrap().into()));assert_eq!(projected.tables[1].rows.len(),source.lines.len());let(extra,released)=retire(projected,&law);assert_eq!(born+extra,freed+released);
  let(restored,_,born,freed)=encode(&law,policy,maximum,Default::default(),&mut |_|true,|sql,owner|TxtSnapshot::from_sqlite_database_receiving(&input,sql,owner));let restored=restored.unwrap();assert_eq!(restored,source);assert_eq!(restored.to_body(),case["body"].as_str().unwrap());let(extra,released)=retire(restored,&law);assert_eq!(born+extra,freed+released);assert_eq!((source.schema.as_ptr(),input.tables.as_ptr()),(source_pointer,input_pointer));
 }
}
#[test]
fn txt_original_receiving_malformed_inputs_and_independent_grant_axes_keep_exact_original_receipts(){
 let law=fixture();let policy=grant(&law["bodyGrant"]);let maximum=law["nativeMaximumBytes"].as_u64().unwrap()as usize;
 for case in law["malformed"].as_array().unwrap(){let input=database(&case["database"]);let pointer=input.tables.as_ptr();let(result,_,born,freed)=encode(&law,policy,maximum,Default::default(),&mut |_|true,|sql,owner|TxtSnapshot::from_sqlite_database_receiving(&input,sql,owner));let error=result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::InvalidValue,"{}",case["id"]);assert_eq!(error.message,case["message"].as_str().unwrap(),"{}",case["id"]);assert!(matches!(error.message,std::borrow::Cow::Borrowed(_)));assert_eq!(born,freed);assert_eq!(input.tables.as_ptr(),pointer);}
 let source=snapshot(&law["cases"][3]["snapshot"]);let input=database(&law["cases"][3]["database"]);
 for axis in ["items","copy","capacity","release","depth","native","sqlAllocation"]{
  let mut incoming=policy;match axis{"items"=>incoming.maximum_items=0,"copy"=>incoming.maximum_copy_bytes=0,"capacity"=>incoming.maximum_capacity_bytes=0,"release"=>incoming.maximum_release_bytes=0,"depth"=>incoming.maximum_depth=0,_=>{}}let ceiling=if axis=="native"{0}else{maximum};let limits=SqliteDatabaseLimits{max_allocation_bytes:if axis=="sqlAllocation"{0}else{SqliteDatabaseLimits::default().max_allocation_bytes},..Default::default()};
  let(result,_,born,freed)=decode(&law,incoming,ceiling,limits,&mut |_|true,|sql,owner|source.to_sqlite_database_receiving(sql,owner));assert!(result.is_err());assert_eq!(born,freed);if ["items","capacity","depth","native","sqlAllocation"].contains(&axis){assert_eq!(born,0);}
  let(result,_,born,freed)=encode(&law,incoming,ceiling,limits,&mut |_|true,|sql,owner|TxtSnapshot::from_sqlite_database_receiving(&input,sql,owner));assert!(result.is_err());assert_eq!(born,freed);if ["items","capacity","depth","native","sqlAllocation"].contains(&axis){assert_eq!(born,0);}
 }
}
#[test]
fn txt_original_receiving_sql_cancellation_keeps_original_cause_and_same_native_close_observer(){
 let law=fixture();let policy=grant(&law["bodyGrant"]);let maximum=law["nativeMaximumBytes"].as_u64().unwrap()as usize;let text=law["source"]["unit"].as_str().unwrap().repeat(law["source"]["repeat"].as_u64().unwrap()as usize);let source=TxtSnapshot{schema:"original".into(),lines:vec![text],trailing_newline:true,line_ending:LineEnding::CrLf};let input=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();
 macro_rules! direction{
  ($native:ident,$owner:ident,$recipient:path,$call:expr)=>{
   let blocked=Cell::new(false);let canceled=Cell::new(false);let mut callback=|_|!blocked.get();let mut recipient=<$recipient>::new();let mut native=$native::new(maximum,&mut callback);native.install_retirement_recipient(&mut recipient).unwrap();let mut sql_callback=|event:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress|{let cut=event.total==law["source"]["bytes"].as_u64().unwrap()as usize&&event.completed>=law["source"]["cancelAfterBytes"].as_u64().unwrap()as usize;canceled.set(canceled.get()||cut);!cut};let mut sql=SqliteSnapshotControl::new(&mut sql_callback,Default::default());let sql_before=sql.allocation_remaining_bytes();let mut owner=store::$owner::new(&mut native,policy);
   let(result,heap)=observe(||($call)(&mut sql,&mut owner));let error=result.unwrap_err();let receipt=owner.progress();assert!(canceled.get());assert_eq!(error.kind,ValueRefusalKind::Canceled);assert_eq!(error.message,"relational SQLite transfer cancelled");assert!(matches!(error.message,std::borrow::Cow::Borrowed(_)));assert_eq!(error.retained_progress(),receipt);assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));drop(owner);assert_eq!(sql_before-sql.allocation_remaining_bytes(),native.owned_bytes());assert!(native.has_retirement_owner());let(mut born,mut freed)=(heap.requested_bytes,heap.released_bytes);
   let(zero,heap)=observe(||native.close_retirement_recipient(grant(&law["deniedCloseGrant"])).unwrap());assert_eq!(zero.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));blocked.set(true);let(refused,heap)=observe(||native.close_retirement_recipient(grant(&law["closeGrant"])));assert_eq!(refused.unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(native.has_retirement_owner());blocked.set(false);
   for _ in 0..law["maximumCloseTurns"].as_u64().unwrap(){if !native.has_retirement_owner(){break}let(step,heap)=observe(||native.close_retirement_recipient(grant(&law["closeGrant"])).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;}assert!(!native.has_retirement_owner());assert_eq!(born,freed);
  };
 }
 direction!(NativeDecodeControl,NativeSnapshotDecodeOwner,semio_framework_value::native_decoding::NativeDecodeRetirementRecipient,|sql:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>|source.to_sqlite_database_receiving(sql,owner));
 direction!(NativeEncodeControl,NativeSnapshotEncodeOwner,semio_framework_value::native_encoding::NativeEncodeRetirementRecipient,|sql:&mut SqliteSnapshotControl<'_>,owner:&mut store::NativeSnapshotEncodeOwner<'_,'_>|TxtSnapshot::from_sqlite_database_receiving(&input,sql,owner));
}
#[test]
fn txt_original_receiving_cumulative_same_owner_uses_prior_paid_receipts(){
 let law=fixture();let policy=grant(&law["bodyGrant"]);let maximum=law["nativeMaximumBytes"].as_u64().unwrap()as usize;let source=snapshot(&law["cases"][3]["snapshot"]);let input=database(&law["cases"][3]["database"]);let mut prior=None;let mut first=RetainedCloneProgress::default();
 let(second,receipt,born,freed)=decode(&law,policy,maximum,Default::default(),&mut |_|true,|sql,owner|{prior=Some(source.to_sqlite_database_receiving(sql,owner)?);first=owner.progress();source.to_sqlite_database_receiving(sql,owner)});assert!(first.copied_items>0);assert!(receipt.copied_items>first.copied_items);let(a,b)=retire(prior.take().unwrap(),&law);let(c,d)=retire(second.unwrap(),&law);assert_eq!(born+a+c,freed+b+d);
 let mut prior=None;let mut first=RetainedCloneProgress::default();let(second,receipt,born,freed)=encode(&law,policy,maximum,Default::default(),&mut |_|true,|sql,owner|{prior=Some(TxtSnapshot::from_sqlite_database_receiving(&input,sql,owner)?);first=owner.progress();TxtSnapshot::from_sqlite_database_receiving(&input,sql,owner)});assert!(first.copied_items>0);assert!(receipt.copied_items>first.copied_items);let(a,b)=retire(prior.take().unwrap(),&law);let(c,d)=retire(second.unwrap(),&law);assert_eq!(born+a+c,freed+b+d);
}

#[test]
fn txt_original_receiving_directional_validators_borrow_both_original_controls_without_backing(){
 let law=fixture();let source=snapshot(&law["cases"][0]["snapshot"]);let input=database(&law["cases"][0]["database"]);let policy=grant(&law["bodyGrant"]);
 macro_rules! direction{
  ($native:ident,$owner:ident,$method:ident,$message:expr)=>{
   for case in law["validation"].as_array().unwrap(){let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:case["kind"].as_str().unwrap().into(),standard:case["standard"].as_str().unwrap().into(),subset:case["subset"].as_str().unwrap().into()};let native_calls=Cell::new(0usize);let sql_calls=Cell::new(0usize);let mut native_callback=|_|{native_calls.set(native_calls.get()+1);case["nativeAccepted"].as_bool().unwrap()};let mut sql_callback=|_|{sql_calls.set(sql_calls.get()+1);case["sqlAccepted"].as_bool().unwrap()};let mut native=$native::new(0,&mut native_callback);let mut sql=SqliteSnapshotControl::new(&mut sql_callback,SqliteDatabaseLimits{max_allocation_bytes:0,..Default::default()});let mut owner=store::$owner::new(&mut native,policy);
    let(result,heap)=observe(||source.$method(&dialect,&input,&mut sql,&mut owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.progress(),RetainedCloneProgress::default());assert_eq!(native_calls.get(),1);assert_eq!(sql_calls.get(),usize::from(case["nativeAccepted"].as_bool().unwrap()));match result{Ok(outcome)=>{assert_eq!(case["expectedKind"],"clean");assert!(outcome.diagnostics.is_empty())},Err(error)=>{assert_eq!(error.cause.kind.as_str(),case["expectedKind"].as_str().unwrap());assert_eq!(error.cause.message,if !case["nativeAccepted"].as_bool().unwrap(){$message}else{case["message"].as_str().unwrap()});assert!(matches!(error.cause.message,std::borrow::Cow::Borrowed(_)));assert!(error.diagnostics.is_empty())}}drop(owner);assert_eq!(native.owned_bytes(),0);assert!(!native.has_retirement_owner());assert_eq!(sql.allocation_remaining_bytes(),0);
   }
  };
 }
 direction!(NativeDecodeControl,NativeSnapshotDecodeOwner,validate_sqlite_snapshot_subset_decoding,"native decoding canceled");
 direction!(NativeEncodeControl,NativeSnapshotEncodeOwner,validate_sqlite_snapshot_subset_encoding,"native encoding canceled");
}
