//! ♻️ Complete erased Count calls release actual incoming, scratch, output and diagnostic backing.
use super::*;
use std::mem::size_of;
fn slots<T>(values:&Vec<T>)->usize{values.capacity().checked_mul(size_of::<T>()).unwrap()}
fn add(total:&mut usize,bytes:usize){*total=total.checked_add(bytes).unwrap();}
fn database_capacity(database:&SqliteDatabase)->usize{
 let mut bytes=slots(&database.tables);
 for table in &database.tables{
  add(&mut bytes,table.name.capacity());add(&mut bytes,table.sql.capacity());add(&mut bytes,slots(&table.rows));
  for row in &table.rows{add(&mut bytes,slots(&row.values));for value in &row.values{match value{SqliteValue::Text(value)=>add(&mut bytes,value.capacity()),SqliteValue::Blob(value)=>add(&mut bytes,value.capacity()),_=>{}}}}
 }
 bytes
}
fn strings_capacity(values:&Vec<String>)->usize{
 let mut bytes=slots(values);for value in values{add(&mut bytes,value.capacity());}bytes
}
fn diagnostic_capacity(error:&store::io_schema::IoError)->usize{
 let mut bytes=error.cause.message.capacity();add(&mut bytes,slots(&error.diagnostics));
 for diagnostic in &error.diagnostics{
  add(&mut bytes,diagnostic.code.0.capacity());add(&mut bytes,diagnostic.message.capacity());
  if let Some(expected)=&diagnostic.expected{for values in [&expected.tokens,&expected.keywords,&expected.keys]{add(&mut bytes,strings_capacity(values));}}
  for value in [&diagnostic.scope.plugin_id,&diagnostic.scope.app_id,&diagnostic.scope.instance_id,&diagnostic.scope.module,&diagnostic.scope.body_key]{if let Some(value)=value{add(&mut bytes,value.capacity());}}
 }
 bytes
}
#[derive(Clone,Copy,Debug)]
struct Sample{kind:Option<ValueRefusalKind>,diagnostic:usize,correct:bool}
fn measure(operation:impl FnOnce()->Result<bool,store::io_schema::IoError>)->(Sample,usize,usize){
 crate::test_allocation::observe_backing(||match operation(){
  Ok(correct)=>Sample{kind:None,diagnostic:0,correct},
  Err(error)=>{let sample=Sample{kind:Some(error.cause.kind),diagnostic:diagnostic_capacity(&error),correct:false};drop(error);sample}
 })
}
fn released(sample:Sample,requested:usize,actual_released:usize,incoming:usize,debit:usize){
 assert_eq!(actual_released,requested.checked_add(incoming).unwrap(),"every concrete requested and consumed incoming owner must be released");
 assert!(requested<=debit.checked_add(sample.diagnostic).unwrap(),"requests include the full returned diagnostic owner separately from caller scratch: {sample:?}, requests={requested}, debit={debit}");
}
#[test]
fn sqlite_retained_host_count_erased_requests_diagnostics_and_full_release(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🪶️sqlite/💰️release/🔣️.json")).unwrap();
 assert_eq!(fixture["requests"],"fullSystemAllocatorRequests");assert_eq!(fixture["incoming"],"validatedCompleteBorrowedCapacityCensus");
 assert_eq!(fixture["diagnostic"],"actualCompleteReturnedIoErrorBacking");
 let codec=codec();let coordinate=dialect();let defaults=SqliteDatabaseLimits::default();
 for count in counts(){
  let expected=database(count);let prebuilt=expected.clone();let incoming=database_capacity(&prebuilt);
  let (_,requested,actual_released)=crate::test_allocation::observe_backing(||drop(prebuilt));
  assert!(incoming>0);assert_eq!(requested,0);assert_eq!(actual_released,incoming,"complete independently released incoming database validates its borrowed capacity census");
  let source=Snapshot{count};
  for encoding in [SnapshotEncoding::Binary,SnapshotEncoding::Text]{
   let native=payload(&source,encoding);let expected_output=payload(&source,encoding);
   for reconstruct in [false,true]{
    let incoming=if reconstruct{incoming}else{0};
    let operation=|control:&mut SqliteSnapshotControl<'_>,input:&mut Option<SqliteDatabase>|{
     if reconstruct{(codec.import)(KIND,&coordinate,input.take().unwrap(),encoding,control).map(|output|{let correct=output.value==expected_output&&output.diagnostics.is_empty();drop(output);correct})}
     else{(codec.export)(KIND,&coordinate,&native,control).map(|output|{let correct=output.value==expected&&output.diagnostics.is_empty();drop(output);correct})}
    };
    let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,defaults);let mut input=Some(expected.clone());
    let (sample,requests,freed)=measure(||operation(&mut control,&mut input));
    let debit=defaults.max_allocation_bytes-control.allocation_remaining_bytes();
    assert_eq!(sample.kind,None);assert!(sample.correct);assert!(requests>0);assert_eq!(requests,debit);released(sample,requests,freed,incoming,debit);
    for maximum in [requests,0,requests-1]{
     let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits{max_allocation_bytes:maximum,..defaults});let mut input=Some(expected.clone());
     let (sample,requested,freed)=measure(||operation(&mut control,&mut input));let debit=maximum-control.allocation_remaining_bytes();
     released(sample,requested,freed,incoming,debit);
     if maximum==requests{assert_eq!(sample.kind,None);assert!(sample.correct);assert_eq!(requested,requests);assert_eq!(control.allocation_remaining_bytes(),0);}
     else{assert_eq!(sample.kind,Some(ValueRefusalKind::OwnershipLimit));assert!(debit<requests);if maximum==0{assert_eq!(debit,0);assert_eq!(requested,sample.diagnostic);}}
    }
    let maximum=requests.checked_mul(2).unwrap();let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits{max_allocation_bytes:maximum,..defaults});
    for _ in 0..2{let before=control.allocation_remaining_bytes();let mut input=Some(expected.clone());let (sample,requested,freed)=measure(||operation(&mut control,&mut input));let debit=before-control.allocation_remaining_bytes();assert_eq!(sample.kind,None);assert!(sample.correct);assert_eq!(requested,requests);assert_eq!(debit,requests);released(sample,requested,freed,incoming,debit);}
    assert_eq!(control.allocation_remaining_bytes(),0);
    let mut input=Some(expected.clone());let (sample,requested,freed)=measure(||operation(&mut control,&mut input));assert_eq!(sample.kind,Some(ValueRefusalKind::OwnershipLimit));released(sample,requested,freed,incoming,0);assert_eq!(control.allocation_remaining_bytes(),0);
    let phase=if reconstruct{SqliteSnapshotPhase::EncodeNative}else{SqliteSnapshotPhase::DecodeNative};
    for materialized in [false,true]{
     let mut reached=false;let mut boundary_requests=0;let mut cancel=|progress:SqliteSnapshotProgress|{
      let actual=crate::test_allocation::observed_requested_bytes().unwrap_or(0);
      let stop=progress.phase==phase&&(!materialized||(progress.completed>0&&actual>0));if stop{reached=true;boundary_requests=actual;}!stop
     };
     let mut control=SqliteSnapshotControl::new(&mut cancel,defaults);let mut input=Some(expected.clone());let (sample,requested,freed)=measure(||operation(&mut control,&mut input));
     let debit=defaults.max_allocation_bytes-control.allocation_remaining_bytes();released(sample,requested,freed,incoming,debit);assert_eq!(sample.kind,Some(ValueRefusalKind::Canceled));drop(control);assert!(reached);if materialized{assert!(boundary_requests>0);assert!(requested>sample.diagnostic);}else if !reconstruct{assert_eq!(boundary_requests,0);assert_eq!(debit,0);assert_eq!(requested,sample.diagnostic);}
    }
   }
  }
 }
}
