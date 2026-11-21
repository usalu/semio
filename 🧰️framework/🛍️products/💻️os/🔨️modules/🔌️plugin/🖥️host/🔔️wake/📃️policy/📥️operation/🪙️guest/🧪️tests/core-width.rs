//! 📐️ The fixed Core wire and its receipts use guest-width grants over a wider native Source.
#![cfg(target_pointer_width="64")]

use semio_framework_actor::{OriginalJobBudget,RetainedTurnInput};
use semio_framework_job::{OriginalNativeRetirement,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
use semio_plugin_host_wake::{GuestRelayAdmissionCall,GuestRelayGuestPurpose,GuestRelayOperationCaps,GuestRelayOperationRequest};

#[test]
fn fixed_core_guest_grants_fit_wasm32_with_larger_host_source_stock(){
 struct Producer(bool);
 impl OriginalNativeRetirement for Producer{
  fn close_on_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
   if self.0{return Ok(true)}
   if cx.retirement_should_yield()||cx.retained_grant().maximum_items==0{return Ok(false)}
   cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;
   cx.consume_fuel(1);
   self.0=true;
   Ok(true)
  }
  fn terminal_is_empty(&self)->bool{self.0}
 }
 let base:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/core-width.json")).unwrap();
 let grant=|value:&serde_json::Value|{
  let [maximum_items,maximum_copy_bytes,maximum_capacity_bytes,maximum_release_bytes,maximum_depth]=serde_json::from_value::<[usize;5]>(value.clone()).unwrap();
  RetainedCloneGrant{maximum_items,maximum_copy_bytes,maximum_capacity_bytes,maximum_release_bytes,maximum_depth}
 };
 let mut caps:GuestRelayOperationCaps=serde_json::from_value(base["caps"].clone()).unwrap();
 caps.operation=grant(&law["physical"]);
 caps.infrastructure.retained=grant(&law["infrastructure"]);
 let mut request:GuestRelayOperationRequest=serde_json::from_value(base["cases"][0]["request"].clone()).unwrap();
 request.work_units=100000;
 for row in law["cases"].as_array().unwrap(){
  request.retained=grant(&row["requested"]);
  let driver=caps.issue(request,||Some(1000),Box::new(||false)).unwrap();
  let semantic=grant(&row["semantic"]);
  let header=grant(&row["header"]);
  assert_eq!(driver.original_remaining().unwrap(),caps.operation);
  assert_eq!(driver.operation_request_limits().unwrap().0.retained,semantic,"{}",row["name"]);
  let mut input=Some(Producer(false));
  let mut lease=None;
  for _ in 0..64{
   driver.dispatch_admission_step(false,|_,cx|GuestRelayAdmissionCall{result:cx.admit_original_native(&mut input,&mut lease).map(|_|()),execution:None}).unwrap();
   if lease.is_some(){break}
  }
  let mut lease=lease.unwrap();
  for (purpose,wire_size,policy,expected_header) in [
   (GuestRelayGuestPurpose::Infrastructure,200,caps.infrastructure.retained,header),
   (GuestRelayGuestPurpose::Semantic,144,semantic,RetainedCloneGrant{maximum_copy_bytes:header.maximum_copy_bytes-1,..header}),
  ]{
   let mut issued=None;
   for _ in 0..64{
    issued=driver.admit_guest_subloan(&mut lease,purpose,wire_size).unwrap();
    if issued.is_some(){break}
   }
   let issued=issued.unwrap();
   let budget=issued.budget();
   assert_eq!(budget.input.grant,expected_header);
   assert_eq!(budget.retained_policy,policy);
   let args=budget.scalar_arguments();
   assert!(args[6..16].iter().all(|extent|u32::try_from(*extent).is_ok()));
   let echoed=OriginalJobBudget::from_scalar_arguments(args).unwrap().input;
   let altered=RetainedTurnInput{grant:RetainedCloneGrant{maximum_copy_bytes:echoed.grant.maximum_copy_bytes-1,..echoed.grant},..echoed};
   let progress=RetainedCloneProgress{copied_bytes:1,..Default::default()};
   assert!(driver.receive_guest_subloan(issued,altered,progress,1).is_err());
   let physical=driver.original_remaining().unwrap();
   let witness=driver.receive_guest_subloan_witness(issued,echoed,progress,1).unwrap();
   assert!(witness.proves(issued,echoed,progress,1,lease.recipient_identity()));
   assert_eq!(driver.original_remaining().unwrap(),physical);
   assert!(physical.maximum_copy_bytes>u32::MAX as usize);
  }
  let mut cleanup=None;
  for _ in 0..64{
   cleanup=driver.admit_cleanup_guest_subloan(&mut lease,144).unwrap();
   if cleanup.is_some(){break}
  }
  let cleanup=cleanup.unwrap();
  let cleanup_budget=cleanup.budget();
  assert_eq!(cleanup_budget.retained_policy,RetainedCloneGrant{maximum_copy_bytes:caps.infrastructure.retained.maximum_copy_bytes-1,..caps.infrastructure.retained});
  assert!(cleanup_budget.scalar_arguments()[6..16].iter().all(|extent|u32::try_from(*extent).is_ok()));
  driver.receive_guest_subloan(cleanup,cleanup_budget.input,Default::default(),0).unwrap();
  drop(lease);
  for _ in 0..256{if driver.close_root_step().unwrap(){break}}
  assert!(driver.operation_terminal_is_empty().unwrap());
 }
 for row in law["invalidInfrastructure"].as_array().unwrap(){
  caps.infrastructure.retained=grant(row);
  assert!(caps.bind_request(request).is_err());
 }
}
