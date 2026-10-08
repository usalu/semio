//! 📏️ Public preflight borrows actual typed owner state and admits concrete traversal scratch.
use semio_framework_os_kernel as store;
use store::sqlite_snapshot::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::fmt::Debug;

pub(crate) fn verify<S:store::ArtifactSqliteSnapshot+Clone+PartialEq+Debug>(
 short:&S,long:&S,sql:&str,rows:usize,field_bytes:usize,cancel_at:usize,
 observe:fn(&mut dyn FnMut()->Result<(),ValueError>)->(Result<(),ValueError>,usize),
){
 let unchanged=long.clone();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
  let mut output_callback=|_|true;
  let mut output_control=SqliteSnapshotControl::new(&mut output_callback,SqliteDatabaseLimits::default());
  let encoded=long.encode_sqlite_snapshot_native(encoding,&mut output_control).expect("actual owner native producer prepares the independent file ceiling outside preflight observation");
  let mut input_callback=|_|true;
  let mut input_control=SqliteSnapshotControl::new(&mut input_callback,SqliteDatabaseLimits::default());
  assert_eq!(S::decode_sqlite_snapshot_native(&encoded,&mut input_control).unwrap(),*long,"actual native specimen preserves every typed owner field before preflight observation");
  let output_bytes=match encoded{store::io::IoPayload::Binary(bytes)=>bytes.len(),store::io::IoPayload::Text(text)=>text.len()};
  assert!(output_bytes>0);
  let defaults=SqliteDatabaseLimits::default();
  let mut callback=|_|true;
  let mut control=SqliteSnapshotControl::new(&mut callback,defaults);
  let(result,requests)=observe(&mut ||long.preflight_sqlite_snapshot_encoding(encoding,&mut control));
  result.expect("actual public preflight must support the actual typed owner");
  let debit=defaults.max_allocation_bytes-control.allocation_remaining_bytes();
  assert_eq!(debit,requests,"borrowed traversal must admit exactly its full concrete scratch requests");
  let mut callback=|_|true;
  let mut shorter=SqliteSnapshotControl::new(&mut callback,defaults);
  let(result,short_requests)=observe(&mut ||short.preflight_sqlite_snapshot_encoding(encoding,&mut shorter));
  result.unwrap();
  assert_eq!(short_requests,requests,"same-shape preflight must not construct copied literal fields or encoded output");
  assert_eq!(defaults.max_allocation_bytes-shorter.allocation_remaining_bytes(),short_requests);
  let mut callback=|_|true;
  let mut exact=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits{max_allocation_bytes:requests,..defaults});
  let(result,exact_requests)=observe(&mut ||long.preflight_sqlite_snapshot_encoding(encoding,&mut exact));
  result.unwrap();assert_eq!(exact_requests,requests);assert_eq!(exact.allocation_remaining_bytes(),0);
  if requests>0{
   let maximum=requests-1;
   let mut callback=|_|true;
   let mut small=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits{max_allocation_bytes:maximum,..defaults});
   let(result,observed)=observe(&mut ||long.preflight_sqlite_snapshot_encoding(encoding,&mut small));
   let error=result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);
   let admitted=maximum-small.allocation_remaining_bytes();
   assert_eq!(observed,admitted+match &error.message { std::borrow::Cow::Borrowed(_) => 0, std::borrow::Cow::Owned(message) => message.capacity() },"failed scratch admission owns only admitted scratch and its explicit typed refusal message");
   let maximum=requests.checked_mul(2).unwrap();
   let mut callback=|_|true;
   let mut cumulative=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits{max_allocation_bytes:maximum,..defaults});
   for _ in 0..2{long.preflight_sqlite_snapshot_encoding(encoding,&mut cumulative).unwrap();}
   assert_eq!(cumulative.allocation_remaining_bytes(),0);
   assert_eq!(long.preflight_sqlite_snapshot_encoding(encoding,&mut cumulative).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);
   assert_eq!(cumulative.allocation_remaining_bytes(),0,"retiring scratch must not refund the caller ledger");
  }
  for(limits,kind)in[
   (SqliteDatabaseLimits{max_file_bytes:output_bytes-1,..defaults},ValueRefusalKind::OwnershipLimit),
   (SqliteDatabaseLimits{max_value_bytes:1,..defaults},ValueRefusalKind::OwnershipLimit),
   (SqliteDatabaseLimits{max_rows:rows-1,..defaults},ValueRefusalKind::WorkLimit),
   (SqliteDatabaseLimits{max_schema_bytes:sql.len()-1,..defaults},ValueRefusalKind::OwnershipLimit),
  ]{
   let mut callback=|_|true;
   let mut denied=SqliteSnapshotControl::new(&mut callback,limits);
   assert_eq!(long.preflight_sqlite_snapshot_encoding(encoding,&mut denied).unwrap_err().kind,kind);
  }
  let mut start=false;
  let mut callback=|_:SqliteSnapshotProgress|{start=true;false};
  let mut canceled=SqliteSnapshotControl::new(&mut callback,defaults);
  let(result,observed)=observe(&mut ||long.preflight_sqlite_snapshot_encoding(encoding,&mut canceled));
  let error=result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::Canceled);
  assert_eq!(canceled.allocation_remaining_bytes(),defaults.max_allocation_bytes);
  assert_eq!(observed,match &error.message { std::borrow::Cow::Borrowed(_) => 0, std::borrow::Cow::Owned(message) => message.capacity() },"start refusal must allocate no traversal or payload backing");
  drop(canceled);assert!(start);
  let mut interior=false;
  let mut callback=|event:SqliteSnapshotProgress|{
   let stop=event.phase==SqliteSnapshotPhase::EncodeNative&&event.total==field_bytes&&event.completed>=cancel_at&&event.completed<event.total;
   interior|=stop;!stop
  };
  let mut canceled=SqliteSnapshotControl::new(&mut callback,defaults);
  let error=long.preflight_sqlite_snapshot_encoding(encoding,&mut canceled).unwrap_err();
  assert_eq!(error.kind,ValueRefusalKind::Canceled);
  drop(canceled);assert!(interior,"borrowed long literal census must expose its actual bounded byte frontier");
  assert_eq!(*long,unchanged,"every success, refusal and cancellation must leave the full actual owner unchanged");
 }
}
