use super::*;
use crate::{GuestRelayAdmissionCall,GuestRelayOperationCaps,GuestRelayOperationRequest};
use semio_framework_value::RetainedCloneGrant;

struct Producer(bool);
impl OriginalNativeRetirement for Producer {
 fn close_on_source(&mut self,_:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{Ok(true)}
 fn terminal_is_empty(&self)->bool{self.0}
}

#[test]
fn issued_origin_uses_a_paid_logarithmic_search_under_the_actual_source() {
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/issued-origins.json")).unwrap();
 let number=|value:&serde_json::Value|value.as_u64().unwrap();
 for arrival in law["issue"].as_array().unwrap(){
  assert_eq!(verify_issued_epoch(arrival["previous"].as_u64(),number(&arrival["epoch"])).is_ok(),arrival["accepted"].as_bool().unwrap());
 }
 let setup:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let caps:GuestRelayOperationCaps=serde_json::from_value(setup["caps"].clone()).unwrap();
 let request:GuestRelayOperationRequest=serde_json::from_value(setup["cases"][0]["request"].clone()).unwrap();
 let driver=caps.issue(request,||Some(1000),Box::new(||false)).unwrap();
 let (mut actual,mut other)=(Some(Producer(true)),Some(Producer(true)));
 let (mut actual_lease,mut other_lease)=(None,None);
 for _ in 0..64{
  driver.dispatch_admission_step(false,|_,cx|GuestRelayAdmissionCall{execution:None,result:(||{
   if actual_lease.is_none(){cx.admit_original_native(&mut actual,&mut actual_lease)?;}
   if other_lease.is_none(){cx.admit_original_native(&mut other,&mut other_lease)?;}
   Ok(())
  })()}).unwrap();
  if actual_lease.is_some()&&other_lease.is_some(){break;}
 }
 let mut actual_lease=actual_lease.unwrap();
 let other_lease=other_lease.unwrap();
 let producer=actual_lease.recipient_identity();
 let foreign=other_lease.recipient_identity();
 let seed=(0..64).find_map(|_|driver.admit_guest_subloan_inner(&mut actual_lease,GuestRelayGuestPurpose::Infrastructure,false,None).unwrap()).unwrap();
 driver.receive_guest_subloan(seed,seed.budget().input,Default::default(),0).unwrap();
 let len=number(&law["count"]) as usize;
 let mut entries=PagedList::<IssuedOrigin,{usize::MAX}>::with_page_items(1);
 for index in 0..len {
  let epoch=number(&law["first"])+(index as u64)*number(&law["stride"]);
  let serial=seed.serial+index as u64;
  let budget=seed.budget();
  let input=semio_framework_actor::RetainedTurnInput{epoch,..budget.input};
  let mut origin=IssuedOrigin::new(GuestRelayGuestSubloan{serial,budget:semio_framework_actor::OriginalJobBudget{input,..budget},..seed},producer);
  if index+1<len{GuestEscrowBook::finish_receipt(&mut origin);}
  entries.try_push(origin).unwrap();
 }
 let mut paid_probes=0;
 let mut charged_work=0;
 for law_case in law["cases"].as_array().unwrap(){
  let epoch=number(&law_case["epoch"]);
  let purpose=law_case["purpose"].as_str().unwrap();
  let mut lookup=IssuedEpochLookup::new(len);
  let mut probes=0;
  let mut verdict=None;
  for _ in 0..=number(&law["lookupMaximum"]){
   let before=lookup;
   driver.dispatch_operation_step(false,false,|cx|{
    if probes==0{
     let work=cx.original_work_units_spent();
     let retained=cx.retained_progress();
     let denied=RetainedCloneGrant{maximum_items:0,..cx.retained_grant()};
     assert!(cx.with_retained_loan(denied,|cx|lookup.step(epoch,&entries,cx,"epoch absent",|_,_|Ok(())))?.is_none());
     assert_eq!((lookup,cx.original_work_units_spent(),cx.retained_progress()),(before,work,retained));
    }
    let start=cx.original_work_units_spent();
    let retained=cx.retained_progress();
    let result=lookup.step(epoch,&entries,cx,"epoch absent",|_,origin|match purpose{
     "receipt"=>verify_receipt_origin(origin,origin.descriptor,producer),
     "forged"=>verify_receipt_origin(origin,GuestRelayGuestSubloan{serial:origin.descriptor.serial+1,..origin.descriptor},producer),
     "foreign"=>verify_claim_origin(origin,foreign,false),
     _=>verify_claim_origin(origin,producer,purpose=="current"),
    });
    if let Some(index)=before.next_index(){
     let used=cx.original_work_units_spent()-start;
     assert_eq!(used,u128::from(entries.get_source_work_demand(index).unwrap()+1));
     assert_eq!(cx.retained_progress().copied_items-retained.copied_items,1);
     assert_eq!(cx.retained_progress().retained_capacity_bytes,retained.retained_capacity_bytes);
     charged_work+=used;
     probes+=1;
    }
    match result{Ok(Some(index))=>verdict=Some(Some(index)),Err(_)=>verdict=Some(None),Ok(None)=>{}}
    Ok(())
   }).unwrap();
   if verdict.is_some(){break;}
  }
  assert_eq!(verdict.unwrap(),law_case["origin"].as_u64().map(|index|index as usize),"{}",law_case["name"]);
  assert!(probes<=number(&law["lookupMaximum"]));
  paid_probes+=probes;
 }
 eprintln!("[DEBUG] issuer lookup entries={len} funded_probes={paid_probes} work={charged_work} full_scan_probes={}",len*law["cases"].as_array().unwrap().len());
 drop(actual_lease);
 drop(other_lease);
 for _ in 0..256{if driver.close_root_step().unwrap(){break}}
 assert!(driver.operation_terminal_is_empty().unwrap());
}
