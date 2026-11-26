use semio_framework_job::{OriginalNativeRetirement,OriginalTurnFailure};
use semio_framework_value::{RetainedCloneProgress,ValueRefusalKind};
use semio_plugin_host_wake::{GuestRelayAdmissionCall,GuestRelayGuestPurpose,GuestRelayOperationCaps,GuestRelayOperationRequest};

#[test]
fn live_host_copy_spending_can_leave_no_guest_room_beside_transport(){
 struct Producer;
 impl OriginalNativeRetirement for Producer{
  fn close_on_source(&mut self,_:&mut semio_framework_job::StepContext<'_>)->Result<bool,OriginalTurnFailure>{Ok(true)}
  fn terminal_is_empty(&self)->bool{true}
 }
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let mut caps:GuestRelayOperationCaps=serde_json::from_value(law["caps"].clone()).unwrap();
 let request:GuestRelayOperationRequest=serde_json::from_value(law["cases"][0]["request"].clone()).unwrap();
 let mut malformed=caps;
 malformed.transit.retained.maximum_copy_bytes=malformed.operation.maximum_copy_bytes;
 assert_eq!(malformed.bind_request(request).unwrap_err().kind,ValueRefusalKind::InvalidValue);
 let copy=caps.infrastructure.retained.maximum_copy_bytes;
 caps.transit.retained.maximum_copy_bytes=caps.operation.maximum_copy_bytes-caps.cleanup.retained.maximum_copy_bytes-copy;
 let driver=caps.issue(request,||Some(1000),Box::new(||false)).unwrap();
 let mut value=Some(Producer);let mut lease=None;
 for _ in 0..64{
  driver.dispatch_admission_step(false,|_,cx|GuestRelayAdmissionCall{result:cx.admit_original_native(&mut value,&mut lease).map(|_|()),execution:None}).unwrap();
  if lease.is_some(){break}
 }
 let mut lease=lease.unwrap();let mut paid=false;
 for _ in 0..64{
  if driver.dispatch_operation_step(false,false,|cx|{
   cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;
   cx.consume_fuel(1);paid=true;Ok(())
  }).unwrap(){break}
 }
 assert!(paid);
 let refusal=(0..64).find_map(|_|match driver.admit_guest_subloan(&mut lease,GuestRelayGuestPurpose::Infrastructure,144){
  Ok(None)=>None,Err(error)=>Some(error),Ok(Some(_))=>panic!("guest took protected transport currency")
 }).expect("actual Source reports the exhausted transport partition");
 assert!(driver.pending_guest_subloan(&lease).unwrap().is_none());
 let error=refusal.primary.as_ref().and_then(OriginalTurnFailure::primary_value).unwrap();
 assert_eq!(error.kind,ValueRefusalKind::WorkLimit,"{}",error.message);
 assert!(error.message.contains("transport")&&error.message.contains("copy"),"{}",error.message);
 drop(lease);
 assert!((0..256).any(|_|driver.close_root_step().unwrap()));
 assert!(driver.operation_terminal_is_empty().unwrap());
}
