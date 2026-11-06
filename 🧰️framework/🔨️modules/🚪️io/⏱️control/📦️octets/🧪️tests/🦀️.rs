//! 🧪️ Portable octet cases keep exact backing and refused prefixes under original recipients.
use super::*;
use semio_framework_value::native_encoding::{NativeEncodeConstructionRecipient,NativeEncodeWorkRecipient};
struct Work{remaining:u64,actual:u64}
impl NativeEncodeWorkRecipient for Work{fn admits(&mut self,_:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.remaining)}fn committed(&mut self,actual:u64)->Result<(),ValueError>{self.remaining-=actual;self.actual+=actual;Ok(())}}
struct Recipient{grant:RetainedCloneGrant,submitted:RetainedCloneProgress,refuse_copy:bool,quoted:Option<RetainedCloneGrant>}
impl NativeEncodeConstructionRecipient for Recipient{
 fn submitted(&self)->RetainedCloneProgress{self.submitted}
 fn grant(&mut self,_:semio_framework_value::RetirementDemand)->Result<RetainedCloneGrant,ValueError>{self.quoted=Some(self.grant);Ok(self.grant)}
 fn accept(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{assert_eq!(Some(grant),self.quoted.take());self.submitted=self.submitted.checked_add(progress)?;if self.refuse_copy&&progress.retained_capacity_bytes==0{Err(ValueError::literal(ValueRefusalKind::Canceled,"original blob receipt refused"))}else{Ok(())}}
}
#[test]
fn encoded_octets_keep_original_backing_prefix_and_receipt(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let count=fixture["payloadBytes"].as_u64().unwrap()as usize;let pattern=fixture["pattern"].as_array().unwrap();let input:Vec<u8>=(0..count).map(|i|pattern[i%pattern.len()].as_u64().unwrap()as u8).collect();
 for case in fixture["cases"].as_array().unwrap(){
  let grant=RetainedCloneGrant{maximum_items:8,maximum_copy_bytes:count+size_of::<Vec<u8>>(),maximum_capacity_bytes:count,maximum_release_bytes:count,maximum_depth:4};
  let mut recipient=Recipient{grant,submitted:Default::default(),refuse_copy:case["name"]=="first-copy-receipt",quoted:None};let mut work=Work{remaining:if case["name"]=="before-allocation"{0}else{8},actual:0};
  let mut callback=|_|true;let mut native=NativeEncodeControl::new(count,&mut callback);let mut body=NativeSnapshotBodyWallet::new(grant);let mut output=Vec::new();let mut marker=NativeSnapshotEffectProgress::default();let mut unwind=None;let mut allocation_state=crate::NativeSnapshotEncodeAllocation::default();
  let mut construction_issuer=semio_framework_value::native_encoding::NativeEncodeConstructionIssuer::new();let mut work_issuer=semio_framework_value::native_encoding::NativeEncodeWorkIssuer::new();let mut construction_loan=construction_issuer.loan(&mut recipient);let mut work_loan=work_issuer.loan(&mut work);let result=native.with_construction_recipient(&mut construction_loan,|native|native.with_work_recipient(&mut work_loan,|native|Ok::<_,ValueError>(body.copy_bytes_encoded(native,&input,&mut output,&mut marker,&mut unwind,&mut allocation_state,1)))).unwrap();
  assert_eq!(result.result.is_err(),case["refused"].as_bool().unwrap());assert_eq!(output.len(),case["retained"].as_u64().unwrap()as usize);assert_eq!(output,&input[..output.len()]);assert_eq!(native.actual_work(),case["work"].as_u64().unwrap());assert_eq!(result.work,native.actual_work());if work.actual==0{assert_eq!(native.owned_bytes(),0);}assert_eq!(work.actual,native.actual_work());assert_eq!(recipient.submitted,body.progress());assert!(unwind.is_none());
  assert_eq!(output.capacity(),if native.actual_work()==0{0}else{count});assert_eq!(serde_json::to_value(&output).unwrap(),serde_json::from_slice::<serde_json::Value>(&serde_json::to_vec(&input[..output.len()]).unwrap()).unwrap());
  let close_grant=RetainedCloneGrant{maximum_items:1024,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:256};
  close(output,close_grant);close(allocation_state,close_grant);if let Err(failure)=result.result{close(failure,close_grant);}
 }
}

fn close<T:semio_framework_value::retirement::RetireOwned>(value:T,grant:RetainedCloneGrant){let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();for _ in 0..4096{if owner.terminal_is_empty(){break}let turn=owner.step(grant);turn.unwrap();}assert!(owner.terminal_is_empty());}
