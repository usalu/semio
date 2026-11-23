//! 🧾️ Original JSON callback recovery retains its actual cursor and output owner.
use super::*;
use crate::original_close::Cell;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand};
use semio_framework_value::native_encoding::{NativeEncodeRetirementRecipient,OriginalConstructionAccept,OriginalConstructionAcceptComplete,OriginalConstructionAuthority,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionRecipient,OriginalSourceTurnIdentity,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient};
use crate::original_close::{close_encode,retire};
struct Physical<'a>{grant:RetainedCloneGrant,target:usize,calls:&'a Cell<usize>,armed:&'a Cell<bool>,received:&'a Cell<RetainedCloneProgress>}
impl OriginalConstructionRecipient for Physical<'_>{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a> where 'source:'turn{quote.complete(Ok(self.grant))}
 fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a> where 'source:'turn{let progress=receipt.progress();self.calls.set(self.calls.get()+1);let posted=self.received.get().checked_add(progress).map(|total|self.received.set(total));if progress.retained_capacity_bytes==self.target&&!self.armed.replace(true){panic!("original JSON receipt callback")}receipt.complete(posted)}
 fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.received.get()}
}
struct Work(u64);
impl OriginalWorkRecipient for Work{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply> where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.0)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(Ok(()))}
 fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(Ok(()))}
}
macro_rules! scoped{($construction:expr,$work:expr,$native:ident,$body:expr)=>{$construction.with_loan(&mut (),|construction|$work.with_loan(&mut (),|work|$native.with_construction_recipient(construction,|$native|$native.with_work_recipient(work,|$native|$body)))).unwrap().unwrap().unwrap()}}
#[test]
fn original_json_frame_retains_its_cursor_through_a_receipt_callback_unwind_until_funded_close(){
 let f:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();assert_eq!(serde_json::to_string(&f["source"]).unwrap(),f["expected"].as_str().unwrap());let source=DslValue::Null;let target=semio_framework_pack_json::JsonBorrowedWriteCursor::new().normal_step_demands(&BorrowedJson(&source)).unwrap().capacity_bytes;let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<22,maximum_release_bytes:1<<22,maximum_depth:256};let calls=Cell::new(0);let received=Cell::new(RetainedCloneProgress::default());let armed=Cell::new(false);let physical=Physical{grant,target,calls:&calls,armed:&armed,received:&received};let mut wi=OriginalWorkAuthority::new(Work(f["work"].as_u64().unwrap()));let mut pi=OriginalConstructionAuthority::new(physical);let mut callback=|_|true;let mut retirement=NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new(grant.maximum_capacity_bytes,&mut callback);native.install_retirement_recipient(&mut retirement).unwrap();let mut output=None;
 {scoped!(pi,wi,native,{let mut owner=semio_framework_io_control::NativeSnapshotEncodeOwner::new(native,grant);let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||write_json(&source,&mut owner,&mut output)));assert!(result.is_err());assert!(armed.get());drop(owner);let turn=close_encode(native,grant);if let Err(failure)=turn.result{retire(semio_framework_io_control::NativeSnapshotFailure::from(failure),grant);}assert!(native.has_retirement_owner());Ok::<_,ValueError>(())});}assert!(native.has_retirement_owner());assert!(output.is_none());let original_calls=calls.get();
 {scoped!(pi,wi,native,{for _ in 0..100000{if !native.has_retirement_owner(){break}close_encode(native,grant).result.unwrap();}assert!(!native.has_retirement_owner());assert_eq!(calls.get(),original_calls);Ok::<_,ValueError>(())});}
}

#[test]
fn original_json_receipts_cover_the_published_value_and_parent_wallet(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let source=DslValue::Null;let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<22,maximum_release_bytes:1<<22,maximum_depth:256};let calls=Cell::new(0);let received=Cell::new(RetainedCloneProgress::default());let armed=Cell::new(false);let mut pi=OriginalConstructionAuthority::new(Physical{grant,target:usize::MAX,calls:&calls,armed:&armed,received:&received});let mut wi=OriginalWorkAuthority::new(Work(fixture["work"].as_u64().unwrap()));let mut callback=|_|true;let mut retirement=NativeEncodeRetirementRecipient::new();let mut native=NativeEncodeControl::new(grant.maximum_capacity_bytes,&mut callback);native.install_retirement_recipient(&mut retirement).unwrap();let mut output=None;
 scoped!(pi,wi,native,{let mut owner=semio_framework_io_control::NativeSnapshotEncodeOwner::new(native,grant);let turn=write_json(&source,&mut owner,&mut output);assert!(turn.result.is_ok());assert!(turn.work>0);assert_eq!(owner.progress(),received.get());assert!(matches!(&output,Some(Ok(semio_framework_io_schema::IoPayload::Text(text))) if text==fixture["expected"].as_str().unwrap()));drop(owner);for _ in 0..100000{if !native.has_retirement_owner(){break}close_encode(native,grant).result.unwrap();}assert!(!native.has_retirement_owner());Ok::<_,ValueError>(())});retire(output,grant);
}
